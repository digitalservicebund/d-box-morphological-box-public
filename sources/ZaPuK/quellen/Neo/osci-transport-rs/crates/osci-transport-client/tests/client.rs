// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use osci_transport::dialog::ControlBlock;
use osci_transport::feedback::code;
use osci_transport::message::MessageType;
use osci_transport::parts::{Content, ContentContainer, ContentKind, ContentPackage, Feedback};
use osci_transport::router::BoxFuture;
use osci_transport::wire::{Request, ResponseEnvelope};
use osci_transport_client::{Attachment, Client, Transport};
use osci_transport_crypto::{RsaDecrypter, encrypt_transport};
use pkcs8::DecodePrivateKey;

fn block_on<F: Future>(mut f: F) -> F::Output {
    fn noop(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VT)
    }
    static VT: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VT)) };
    let mut cx = Context::from_waker(&waker);
    let mut f = unsafe { Pin::new_unchecked(&mut f) };
    loop {
        if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

// Records the last request body and replies with a canned InitDialog response
// that echoes the request's Challenge and assigns a ConversationId.
struct FakeIntermediary {
    last_request: Mutex<Vec<u8>>,
}

impl Transport for FakeIntermediary {
    fn post(&self, body: Vec<u8>) -> BoxFuture<'_, osci_transport::error::Result<Vec<u8>>> {
        *self.last_request.lock().unwrap() = body.clone();
        Box::pin(async move {
            let env = osci_transport::envelope::parse(
                &osci_transport::mime::main_xml_part(&body).unwrap(),
            )
            .unwrap();
            let challenge = env.control_block.and_then(|c| c.challenge).unwrap();

            let cb = ControlBlock {
                id: Some("controlblock".into()),
                conversation_id: Some("cv-123".into()),
                response: Some(challenge),
                challenge: Some("srv-nonce".into()),
                sequence_number: Some(0),
            };
            Ok(ResponseEnvelope::new(MessageType::ResponseToInitDialog, cb)
                .with_feedback(Feedback::new(code::OK_DIALOG_OPEN, "ok"))
                .encode()
                .unwrap())
        })
    }
}

#[test]
fn init_dialog_advances_the_client_dialog() {
    let transport = FakeIntermediary {
        last_request: Mutex::new(Vec::new()),
    };
    let mut client = Client::new(transport).explicit_dialog();

    let resp = block_on(client.send(Request::InitDialog)).unwrap();

    assert_eq!(resp.message_type, MessageType::ResponseToInitDialog);
    assert_eq!(resp.feedback.len(), 1);
    assert_eq!(resp.feedback[0].code, code::OK_DIALOG_OPEN);

    assert_eq!(client.dialog().conversation_id.as_deref(), Some("cv-123"));
    assert_eq!(client.dialog().peer_challenge.as_deref(), Some("srv-nonce"));
    // InitDialog carries no SequenceNumber at all (the official Java osci-bibliothek's
    // InitDialog constructor explicitly clears it), so it doesn't consume
    // one either - the first real message after it still starts at 0.
    assert_eq!(client.dialog().own_sequence, 0);
}

struct Unreachable;

impl Transport for Unreachable {
    fn post(&self, _body: Vec<u8>) -> BoxFuture<'_, osci_transport::error::Result<Vec<u8>>> {
        Box::pin(async { Err(osci_transport::error::Error::msg("connection refused")) })
    }
}

#[test]
fn failed_request_does_not_consume_a_sequence_number() {
    let mut client = Client::new(Unreachable);

    assert!(block_on(client.send(Request::GetMessageId)).is_err());
    assert_eq!(client.dialog().own_sequence, 0);
    assert!(client.dialog().expected_response.is_none());
}

#[test]
fn unsigned_response_is_rejected_when_verification_is_on() {
    let transport = FakeIntermediary {
        last_request: Mutex::new(Vec::new()),
    };
    let mut client = Client::new(transport)
        .explicit_dialog()
        .verify_responses(|_| true);

    let err = block_on(client.send(Request::InitDialog)).err().unwrap();
    assert!(err.to_string().contains("no ds:Signature"), "{err}");
    // the rejected response leaves the dialog untouched
    assert!(client.dialog().conversation_id.is_none());
}

const ATTACHMENT: &[u8] = b"\x00\x01binary attachment\xff";

fn interop(name: &str) -> Vec<u8> {
    std::fs::read(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/interop/keys")
            .join(name),
    )
    .unwrap()
}

// A MIME message with the envelope part and one binary attachment.
fn mime_with_attachment(xml: &[u8]) -> Vec<u8> {
    use base64::Engine;
    let mut out = b"MIME-Version: 1.0\r\nContent-Type: Multipart/Related; boundary=B; type=text/xml\r\n\r\n--B\r\nContent-Type: text/xml; charset=UTF-8\r\nContent-ID: <osci@message>\r\n\r\n".to_vec();
    out.extend_from_slice(xml);
    out.extend_from_slice(b"\r\n--B\r\nContent-Type: application/pdf\r\nContent-Transfer-Encoding: base64\r\nContent-ID: <att1>\r\n\r\n");
    out.extend_from_slice(
        base64::engine::general_purpose::STANDARD
            .encode(ATTACHMENT)
            .as_bytes(),
    );
    out.extend_from_slice(b"\r\n--B--\r\n");
    out
}

// Replies to InitDialog with the envelope XML in `wrap`'s MIME shape.
struct Wrapping<F>(F);

impl<F: Fn(&[u8]) -> Vec<u8> + Send + Sync> Transport for Wrapping<F> {
    fn post(&self, body: Vec<u8>) -> BoxFuture<'_, osci_transport::error::Result<Vec<u8>>> {
        Box::pin(async move {
            let xml = osci_transport::mime::main_xml_part(&body).unwrap();
            let challenge = osci_transport::envelope::parse(&xml)
                .unwrap()
                .control_block
                .and_then(|c| c.challenge)
                .unwrap();
            let cb = ControlBlock {
                id: Some("controlblock".into()),
                conversation_id: Some("cv-1".into()),
                response: Some(challenge),
                challenge: Some("n".into()),
                sequence_number: Some(0),
            };
            let xml = ResponseEnvelope::new(MessageType::ResponseToInitDialog, cb)
                .encode()
                .unwrap();
            Ok((self.0)(&xml))
        })
    }
}

#[test]
fn attachments_and_envelope_xml_are_returned() {
    let transport = Wrapping(|xml: &[u8]| mime_with_attachment(xml));
    let mut client = Client::new(transport).explicit_dialog();

    let resp = block_on(client.send(Request::InitDialog)).unwrap();

    assert_eq!(resp.attachments.len(), 1);
    // the message is the exact response body
    assert_eq!(resp.message, mime_with_attachment(&resp.envelope_xml));
    assert_eq!(
        osci_transport::mime::attachments(&resp.message).unwrap(),
        resp.attachments
    );
    let a = &resp.attachments[0];
    assert_eq!(a.content_id.as_deref(), Some("att1"));
    assert_eq!(a.content_type.as_deref(), Some("application/pdf"));
    assert_eq!(a.data, ATTACHMENT);
    // exactly the main XML part of the MIME message
    let mime = mime_with_attachment(&resp.envelope_xml);
    assert_eq!(
        osci_transport::mime::main_xml_part(&mime).unwrap(),
        resp.envelope_xml
    );
    assert!(resp.envelope_xml.starts_with(b"<?xml"));
}

#[test]
fn transport_encrypted_response_keeps_its_attachments() {
    let cert = interop("test_osci-manager_cypher.cer");
    let key = rsa::PrivateKey::from_pkcs8_pem(
        &String::from_utf8(interop("test_osci-manager_cypher.key.pem")).unwrap(),
    )
    .unwrap();
    let enc_cert = cert.clone();
    let transport = Wrapping(move |xml: &[u8]| {
        encrypt_transport(&mime_with_attachment(xml), &enc_cert).unwrap()
    });
    let mut client = Client::new(transport)
        .explicit_dialog()
        .with_decrypter(RsaDecrypter::new(key).with_certificate(cert));

    let resp = block_on(client.send(Request::InitDialog)).unwrap();

    assert_eq!(resp.attachments.len(), 1);
    assert_eq!(resp.attachments[0].data, ATTACHMENT);
    assert_eq!(resp.attachments[0].content_id.as_deref(), Some("att1"));
    // the message is the decrypted MIME entity, not the encrypted one
    assert_eq!(resp.message, mime_with_attachment(&resp.envelope_xml));
    assert_eq!(
        osci_transport::mime::main_xml_part(&resp.message).unwrap(),
        resp.envelope_xml
    );
    assert!(resp.envelope_xml.starts_with(b"<?xml"));
}

#[test]
fn response_without_attachments_gives_an_empty_vec() {
    let transport = FakeIntermediary {
        last_request: Mutex::new(Vec::new()),
    };
    let mut client = Client::new(transport).explicit_dialog();

    let resp = block_on(client.send(Request::InitDialog)).unwrap();

    assert!(resp.attachments.is_empty());
    assert!(!resp.envelope_xml.is_empty());
}

// Keeps the last request body and refuses it, so a test can look at what went
// out without building a response.
#[derive(Clone, Default)]
struct Recorder(std::sync::Arc<Mutex<Vec<u8>>>);

impl Transport for Recorder {
    fn post(&self, body: Vec<u8>) -> BoxFuture<'_, osci_transport::error::Result<Vec<u8>>> {
        *self.0.lock().unwrap() = body;
        Box::pin(async { Err(osci_transport::error::Error::msg("recorded")) })
    }
}

fn att(id: &str, ty: &str, data: &[u8]) -> Attachment {
    Attachment {
        content_id: Some(id.into()),
        content_type: Some(ty.into()),
        data: data.to_vec(),
    }
}

fn package_referring_to(ids: &[&str]) -> ContentPackage {
    let mut c = ContentContainer::new();
    c.id = Some("contentcontainer0".into());
    for id in ids {
        c.contents.push(Content {
            id: Some((*id).into()),
            kind: ContentKind::Reference {
                href: format!("cid:{id}"),
            },
        });
    }
    ContentPackage {
        containers: vec![c],
        encrypted: Vec::new(),
    }
}

fn two_attachments() -> Vec<Attachment> {
    vec![
        att("xjustiz_nachricht.xml", "text/xml", b"<nachricht/>"),
        att("Klage.pdf", "application/pdf", b"%PDF\x00\xff binary"),
    ]
}

fn store_delivery() -> Request {
    Request::StoreDelivery {
        message_id: "m1".into(),
        subject: None,
    }
}

#[test]
fn delivery_attachments_go_out_as_mime_parts() {
    let rec = Recorder::default();
    let mut client = Client::new(rec.clone()).explicit_dialog();

    let err = block_on(client.send_delivery_with_attachments(
        store_delivery(),
        package_referring_to(&["xjustiz_nachricht.xml", "Klage.pdf"]),
        &two_attachments(),
        &[],
    ))
    .err()
    .unwrap();
    assert_eq!(err.to_string(), "recorded");

    let sent = rec.0.lock().unwrap().clone();
    let got = osci_transport::mime::attachments(&sent).unwrap();
    assert_eq!(got.len(), 2);
    assert_eq!(got[0].content_id.as_deref(), Some("xjustiz_nachricht.xml"));
    assert_eq!(got[1].content_type.as_deref(), Some("application/pdf"));
    assert_eq!(got[1].data, b"%PDF\x00\xff binary");
    let xml = String::from_utf8(osci_transport::mime::main_xml_part(&sent).unwrap()).unwrap();
    assert!(xml.contains(r#"href="cid:Klage.pdf""#), "{xml}");
}

#[test]
fn a_signing_client_must_choose_how_it_treats_responses() {
    let cert = interop("test_osci-manager_cypher.cer");
    let key_pem = interop("test_osci-manager_cypher.key.pem");
    let rec = Recorder::default();
    let mut client = Client::new(rec.clone()).with_signer(key_pem, cert);

    let Err(err) = block_on(client.send(Request::InitDialog)) else {
        panic!("the send should have been refused");
    };
    assert!(err.to_string().contains("not verified"), "{err}");
    // nothing left the client
    assert!(rec.0.lock().unwrap().is_empty());
}

#[test]
fn attachments_are_covered_by_the_signature_and_the_encryption() {
    let cert = interop("test_osci-manager_cypher.cer");
    let key_pem = interop("test_osci-manager_cypher.key.pem");
    let key =
        rsa::PrivateKey::from_pkcs8_pem(&String::from_utf8(key_pem.clone()).unwrap()).unwrap();
    let rec = Recorder::default();
    let mut client = Client::new(rec.clone())
        .explicit_dialog()
        .with_signer(key_pem, cert.clone())
        .accept_unverified_responses()
        .encrypt_for(cert.clone());

    let _ = block_on(client.send_delivery_with_attachments(
        store_delivery(),
        package_referring_to(&["xjustiz_nachricht.xml", "Klage.pdf"]),
        &two_attachments(),
        &[],
    ));

    let sent = rec.0.lock().unwrap().clone();
    // the attachments are inside the ciphertext, not next to it
    assert!(
        osci_transport::mime::attachments(&sent)
            .unwrap()
            .iter()
            .all(|a| a.data != b"<nachricht/>")
    );
    let inner = osci_transport_crypto::decrypt_transport(
        &sent,
        &RsaDecrypter::new(key).with_certificate(cert),
    )
    .unwrap();
    let got = osci_transport::mime::attachments(&inner).unwrap();
    assert_eq!(got.len(), 2);
    assert_eq!(got[1].data, b"%PDF\x00\xff binary");

    let xml = osci_transport::mime::main_xml_part(&inner).unwrap();
    let report = osci_transport_crypto::verify_transport_signature(&xml, |_| true).unwrap();
    assert!(report.valid, "{:?}", report.reason);
    assert_eq!(
        report.attachment_refs,
        ["cid:xjustiz_nachricht.xml", "cid:Klage.pdf"]
    );
}

#[test]
fn a_dangling_reference_or_part_sends_nothing() {
    let rec = Recorder::default();
    let mut client = Client::new(rec.clone()).explicit_dialog();

    for (refs, parts) in [
        (&["a", "b"][..], vec![att("a", "text/plain", b"1")]),
        (
            &["a"][..],
            vec![att("a", "text/plain", b"1"), att("z", "text/plain", b"2")],
        ),
    ] {
        let err = block_on(client.send_delivery_with_attachments(
            store_delivery(),
            package_referring_to(refs),
            &parts,
            &[],
        ))
        .err()
        .unwrap();
        assert_ne!(err.to_string(), "recorded");
    }
    assert!(rec.0.lock().unwrap().is_empty());
}

// The old send methods used to skip the check.
#[test]
fn plain_send_methods_also_refuse_a_dangling_reference() {
    let rec = Recorder::default();
    let mut client = Client::new(rec.clone()).explicit_dialog();

    let err = block_on(client.send_delivery(store_delivery(), package_referring_to(&["a"])))
        .err()
        .unwrap();
    assert!(err.to_string().contains("cid:a"), "{err}");
    let err =
        block_on(client.send_delivery_with(store_delivery(), package_referring_to(&["a"]), &[]))
            .err()
            .unwrap();
    assert!(err.to_string().contains("cid:a"), "{err}");
    assert!(rec.0.lock().unwrap().is_empty());
}
