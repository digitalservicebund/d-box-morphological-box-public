// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::fs;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use osci_transport::envelope::{Sniff, parse, sniff};
use osci_transport::feedback::code;
use osci_transport::message::MessageType;
use osci_transport::parts::Feedback;
use osci_transport::router::{IncomingMessage, OutgoingMessage, Router};
use osci_transport_crypto::RsaDecrypter;
use osci_transport_server::{ReceiverConfig, ResponseOptions, receive, respond, respond_with};
use pkcs8::DecodePrivateKey;
use rsa::PrivateKey;

fn interop(p: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/interop")
        .join(p)
}

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

#[test]
fn encrypted_message_flows_to_a_handler() {
    let raw = fs::read(interop("corpus/StrDel_Encrypted_cbc.osci")).unwrap();

    let key = PrivateKey::from_pkcs8_pem(
        &fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap(),
    )
    .unwrap();
    let cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();

    let config = ReceiverConfig {
        decrypter: Some(RsaDecrypter::new(key).with_certificate(cert)),
        require_valid_signature: false,
    };

    let msg = receive(&raw, &config, |_| true).unwrap();
    assert_eq!(msg.message_type, MessageType::StoreDelivery);
    assert!(msg.transport_encrypted);

    async fn store(m: IncomingMessage) -> OutgoingMessage {
        assert_eq!(m.ty, Some(MessageType::StoreDelivery));
        assert!(m.container.is_some());
        OutgoingMessage::new(MessageType::ResponseToStoreDelivery)
    }

    let router = Router::default().on(MessageType::StoreDelivery, store);
    let out = block_on(router.dispatch(msg.into_incoming())).unwrap();
    assert_eq!(out.ty, MessageType::ResponseToStoreDelivery);

    let bytes = respond(&out).unwrap();
    assert_eq!(
        sniff(&bytes).unwrap(),
        Sniff::Message(MessageType::ResponseToStoreDelivery)
    );
    assert!(parse(&bytes).unwrap().control_block.is_some());
}

async fn store(_m: IncomingMessage) -> OutgoingMessage {
    OutgoingMessage::new(MessageType::ResponseToStoreDelivery)
        .feedback(Feedback::new(code::OK_DIALOG_OPEN, "ok"))
}

#[test]
fn handler_response_serializes() {
    let out = block_on(store(IncomingMessage::default()));
    let bytes = respond(&out).unwrap();
    let s = String::from_utf8(bytes).unwrap();
    assert!(s.contains("<osci:Code>0801</osci:Code>"));
    assert!(s.contains("responseToStoreDelivery"));
}

#[test]
fn signed_and_encrypted_response() {
    use osci_transport::envelope::{Sniff, sniff};
    use osci_transport::mime::main_xml_part;
    use osci_transport_crypto::{RsaDecrypter, decrypt_transport, verify_transport_signature};

    let key_pem = fs::read(interop("keys/test_osci-manager_cypher.key.pem")).unwrap();
    let cert_der = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();

    let out = block_on(store(IncomingMessage::default()));
    let opts = ResponseOptions {
        signer: Some((key_pem.clone(), cert_der.clone())),
        encrypt_for: Some(cert_der.clone()),
    };
    let bytes = respond_with(&out, &opts).unwrap();

    // outer message is transport-encrypted
    assert!(matches!(
        sniff(&main_xml_part(&bytes).unwrap()).unwrap(),
        Sniff::Encrypted
    ));

    let dec = RsaDecrypter::new(
        rsa::PrivateKey::from_pkcs8_pem(std::str::from_utf8(&key_pem).unwrap()).unwrap(),
    )
    .with_certificate(cert_der);
    let inner = decrypt_transport(&bytes, &dec).unwrap();
    let inner_xml = main_xml_part(&inner).unwrap();

    let report = verify_transport_signature(&inner_xml, |_| true).unwrap();
    assert!(report.valid, "{:?}", report.reason);
}

#[test]
fn signed_plain_message_passes_the_signature_gate() {
    let raw = fs::read(interop("corpus/FetchProcessCard.osci")).unwrap();
    let config: ReceiverConfig<RsaDecrypter> = ReceiverConfig {
        decrypter: None,
        require_valid_signature: true,
    };

    let msg = receive(&raw, &config, |_| true).unwrap();
    assert_eq!(msg.message_type, MessageType::FetchProcessCard);
    assert!(!msg.transport_encrypted);
    assert!(msg.signature.as_ref().unwrap().valid);

    let originator = msg
        .roles
        .iter()
        .find(|r| r.kind == osci_transport::role::RoleKind::Originator)
        .expect("originator role resolved from NonIntermediaryCertificates");
    assert!(originator.certs.signature.is_some());

    let incoming = msg.into_incoming();
    assert_eq!(incoming.roles.len(), 1);
}

#[test]
fn untrusted_signer_is_rejected_when_required() {
    let raw = fs::read(interop("corpus/FetchProcessCard.osci")).unwrap();
    let config: ReceiverConfig<RsaDecrypter> = ReceiverConfig {
        decrypter: None,
        require_valid_signature: true,
    };
    assert!(receive(&raw, &config, |_| false).is_err());
}

#[test]
fn encrypted_without_decrypter_is_rejected() {
    let raw = fs::read(interop("corpus/StrDel_Encrypted_cbc.osci")).unwrap();
    let config: ReceiverConfig<RsaDecrypter> = ReceiverConfig {
        decrypter: None,
        require_valid_signature: false,
    };
    assert!(receive(&raw, &config, |_| true).is_err());
}

#[test]
fn doubly_encrypted_message_is_rejected() {
    let inner = fs::read(interop("corpus/StrDel_Encrypted_cbc.osci")).unwrap();
    let cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let key = PrivateKey::from_pkcs8_pem(
        &fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap(),
    )
    .unwrap();
    let outer = osci_transport_crypto::encrypt_transport(&inner, &cert).unwrap();

    let config = ReceiverConfig {
        decrypter: Some(RsaDecrypter::new(key).with_certificate(cert)),
        require_valid_signature: false,
    };
    let err = receive(&outer, &config, |_| true).unwrap_err();
    assert!(err.to_string().contains("more than once"), "{err}");
}

#[test]
fn attachments_come_from_the_decrypted_message() {
    let raw = fs::read(interop("corpus/StrDel_Encrypted_cbc.osci")).unwrap();
    let key = PrivateKey::from_pkcs8_pem(
        &fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap(),
    )
    .unwrap();
    let cert = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();
    let dec = RsaDecrypter::new(key).with_certificate(cert);
    let inner = osci_transport_crypto::decrypt_transport(&raw, &dec).unwrap();
    let config = ReceiverConfig {
        decrypter: Some(dec),
        require_valid_signature: false,
    };

    let msg = receive(&raw, &config, |_| true).unwrap();

    assert_eq!(
        msg.attachments,
        osci_transport::mime::attachments(&inner).unwrap()
    );
}

#[test]
fn plain_message_attachments_are_read() {
    let raw = fs::read(interop("corpus/PartialStoreDelivery.osci")).unwrap();
    let config = ReceiverConfig::<RsaDecrypter> {
        decrypter: None,
        require_valid_signature: false,
    };

    let msg = receive(&raw, &config, |_| true).unwrap();
    let n = msg.attachments.len();

    assert!(!msg.attachments.is_empty());
    assert_eq!(
        msg.attachments,
        osci_transport::mime::attachments(&raw).unwrap()
    );
    assert_eq!(msg.into_incoming().attachments.len(), n);
}

// Deliveries signed by the Java library: the digests it wrote for its
// attachments are the ones our check recomputes. (PartialStoreDelivery.osci is
// left out: its sample file has LF line ends where the signed part had CRLF.)
#[test]
fn java_signed_attachments_match_their_digests() {
    let config = ReceiverConfig::<RsaDecrypter> {
        decrypter: None,
        require_valid_signature: true,
    };
    for name in [
        "AcceptDelivery",
        "CBC2018",
        "ForwardDelivery",
        "ResponseToFetchDelivery",
        "ResponseToProcessDelivery",
        "StoreDelivery",
    ] {
        let raw = fs::read(interop(&format!("corpus/{name}.osci"))).unwrap();

        let msg = receive(&raw, &config, |_| true).unwrap_or_else(|e| panic!("{name}: {e}"));

        assert!(!msg.attachments.is_empty(), "{name}");
        let sig = msg.signature.unwrap();
        assert_eq!(
            sig.attachment_digests.len(),
            msg.attachments.len(),
            "{name}"
        );
    }
}

// A part swapped after signing is refused even though the signature itself is
// intact, because the signature only holds the digest.
#[test]
fn a_swapped_attachment_is_refused() {
    use osci_transport::crypto::SignatureAlg;
    use osci_transport::dialog::ControlBlock;
    use osci_transport::mime::{Attachment, wrap_envelope_with};
    use osci_transport::wire::{Request, RequestEnvelope};
    use osci_transport_crypto::{Role, sign_envelope_with_attachments};

    let part = |data: &[u8]| Attachment {
        content_id: Some("doc.pdf".into()),
        content_type: Some("application/pdf".into()),
        data: data.to_vec(),
    };
    let cb = ControlBlock {
        id: Some("controlblock".into()),
        sequence_number: Some(0),
        ..Default::default()
    };
    let envelope = RequestEnvelope::new(Request::GetMessageId, cb)
        .encode()
        .unwrap();
    let signed = sign_envelope_with_attachments(
        &envelope,
        Role::Client,
        &fs::read(interop("keys/test_osci-manager_cypher.key.pem")).unwrap(),
        &fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap(),
        &[],
        SignatureAlg::RsaSha256,
        &[part(b"the real document")],
    )
    .unwrap();
    let config = ReceiverConfig::<RsaDecrypter> {
        decrypter: None,
        require_valid_signature: true,
    };

    let ok = wrap_envelope_with(&signed, &[part(b"the real document")]).unwrap();
    assert_eq!(
        receive(&ok, &config, |_| true).unwrap().attachments.len(),
        1
    );

    let swapped = wrap_envelope_with(&signed, &[part(b"something else")]).unwrap();
    let err = receive(&swapped, &config, |_| true).unwrap_err();
    assert!(err.to_string().contains("doc.pdf"), "{err}");

    let extra = wrap_envelope_with(
        &signed,
        &[
            part(b"the real document"),
            Attachment {
                content_id: Some("extra".into()),
                ..part(b"x")
            },
        ],
    )
    .unwrap();
    assert!(receive(&extra, &config, |_| true).is_err());
}
