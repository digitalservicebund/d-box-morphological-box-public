// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

// Exercises osci-transport-client against Governikus's public test
// Intermediär (see the official Java osci-bibliothek readme, "Beispiele und
// Test-Infrastruktur"). Ignored by default: it needs network access, hits a
// third-party service the operators ask not to load-test, and can fail for
// reasons outside this repo (the Intermediär restarting, losing state).
//
// Run manually: cargo test -p osci-transport-client --test
// against_test_intermediary -- --ignored --nocapture
//
// Key material: Alice's signature/cipher keys and the Intermediär's own
// cipher cert, both from the official Java osci-bibliothek's own test resources (same
// provenance and license as tests/interop/keys - see REUSE.toml).
//
// Scope: InitDialog, GetMessageId, StoreDelivery, FetchDelivery and
// ExitDialog all round-trip cleanly, signed and GCM/OAEP-encrypted.
// StoreDelivery needs two things beyond InitDialog/GetMessageId: an
// `osci:QualityOfTimestamp` header block for each of "creation" and
// "reception" (`Client` adds these automatically), and a
// `CipherCertificateAddressee` slot naming who the delivery is for -
// `Client::send_delivery_with` takes that as an explicit `CertSlot`. This
// test addresses the delivery to Alice's own cipher cert (self-delivery),
// since the test key material has no separate recipient identity.
// ExitDialog and FetchDelivery need the opposite: their
// `NonIntermediaryCertificatesType` only permits a
// `SignatureCertificateOriginator` slot, no `CipherCertificate*` at all;
// `Client` only adds the automatic cipher slot for message types that allow
// it.

use std::fs;
use std::path::PathBuf;

use osci_transport::content::RawContent;
use osci_transport::message::MessageType;
use osci_transport::parts::{Content, ContentContainer, ContentKind, ContentPackage};
use osci_transport::role::RoleKind;
use osci_transport::wire::Request;
use osci_transport::xml::QName;
use osci_transport_client::{Attachment, Client, HttpTransport};
use osci_transport_crypto::{CertSlot, RsaDecrypter};
use pkcs8::DecodePrivateKey;
use rsa::PrivateKey;

const INTERMED_URL: &str = "http://gov.test.osci.de/osci-manager-entry/externalentry";

fn key(name: &str) -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/interop/test-intermediary-keys")
            .join(name),
    )
    .unwrap()
}

fn alice_cipher_key() -> PrivateKey {
    let pem = String::from_utf8(key("alice_cipher.key.pem")).unwrap();
    PrivateKey::from_pkcs8_pem(&pem).unwrap()
}

#[tokio::test]
#[ignore = "hits Governikus's public test Intermediär over the network"]
async fn round_trip_against_the_real_intermediary() {
    let manager_cert = key("osci_manager_cipher_4096.cer");
    let alice_sig_cert = key("alice_signature.cer");
    let alice_cipher_cert = key("alice_cipher.cer");
    let manager_sig_cert = key("osci_manager_signature_4096.cer");

    // The modern default (AES-256-GCM + RSA-OAEP-SHA256, with explicit
    // MGF/DigestMethod children on the OAEP EncryptionMethod) round-trips
    // against this deployment for a signed+encrypted message. The older
    // AES-256-CBC + RSA-PKCS1v1.5 combo the vendored corpus samples use
    // decrypts fine at the transport layer, but combined with a signed
    // inner envelope this deployment rejects it (SOAP fault 9300). The
    // encrypt/decrypt round-trips byte-exact against our own keys, so the
    // defect is elsewhere in the message, not in the symmetric/RSA-1.5
    // implementation. Use `EncryptOptions` with
    // `KeyTransportAlg::Rsa15`/`SymmetricAlg::Aes256Cbc` only for an
    // unsigned message against this Intermediär.
    let transport = HttpTransport::new(INTERMED_URL);
    let mut client = Client::new(transport)
        .explicit_dialog()
        .with_signer(key("alice_signature.key.pem"), alice_sig_cert)
        .with_decrypter(RsaDecrypter::new(alice_cipher_key()).with_certificate(alice_cipher_cert))
        .encrypt_for(manager_cert)
        .verify_responses(move |cert| cert == manager_sig_cert);

    let init = client
        .send(Request::InitDialog)
        .await
        .expect("InitDialog failed");
    assert_eq!(init.message_type, MessageType::ResponseToInitDialog);
    println!("InitDialog feedback: {:?}", init.feedback);

    let get_id = client
        .send(Request::GetMessageId)
        .await
        .expect("GetMessageId failed");
    assert_eq!(get_id.message_type, MessageType::ResponseToGetMessageId);
    let message_id = get_id
        .envelope
        .message_id()
        .expect("MessageId parse")
        .expect("no MessageId in response");
    println!("MessageId: {message_id}");

    let mut container = ContentContainer::new();
    container.id = Some("contentcontainer0".into());
    container.push_raw(RawContent::new(
        QName::new("http://www.osci.de/2002/04/osci", "XMLContent"),
        b"<greeting>hello</greeting>".to_vec(),
    ));
    let mut content = ContentPackage::default();
    content.containers.push(container);

    let store = client
        .send_delivery_with_attachments(
            Request::StoreDelivery {
                message_id: message_id.clone(),
                subject: Some("osci-transport interop smoke test".into()),
            },
            content,
            &[],
            &[
                CertSlot {
                    kind: RoleKind::Addressee,
                    is_signature: false,
                    cert_der: key("alice_cipher.cer"),
                },
                CertSlot {
                    kind: RoleKind::Reader,
                    is_signature: false,
                    cert_der: key("alice_cipher.cer"),
                },
            ],
        )
        .await
        .expect("StoreDelivery failed");
    println!(
        "StoreDelivery response type: {:?}, feedback: {:?}",
        store.message_type, store.feedback
    );

    let fetch = client
        .send(Request::FetchDelivery {
            message_id: Some(message_id.clone()),
        })
        .await
        .expect("FetchDelivery failed");
    println!(
        "FetchDelivery response type: {:?}, feedback: {:?}",
        fetch.message_type, fetch.feedback
    );

    let exit = client
        .send(Request::ExitDialog)
        .await
        .expect("ExitDialog failed");
    println!("ExitDialog feedback: {:?}", exit.feedback);
}

// Alice's client: signed, with her cipher key for responses. `encrypt` turns
// transport encryption for the Intermediär on.
fn alice_client<T: osci_transport_client::Transport>(transport: T, encrypt: bool) -> Client<T> {
    let manager_sig_cert = key("osci_manager_signature_4096.cer");
    let client = Client::new(transport)
        .explicit_dialog()
        .with_signer(key("alice_signature.key.pem"), key("alice_signature.cer"))
        .with_decrypter(
            RsaDecrypter::new(alice_cipher_key()).with_certificate(key("alice_cipher.cer")),
        )
        .verify_responses(move |cert| cert == manager_sig_cert);
    if encrypt {
        client.encrypt_for(key("osci_manager_cipher_4096.cer"))
    } else {
        client
    }
}

// Opens a dialog on `client` and stores one delivery with `content`,
// addressed to Alice's cipher certificate. Returns the MessageId and the
// StoreDelivery result.
async fn store_in_new_dialog<T: osci_transport_client::Transport>(
    client: &mut Client<T>,
    content: ContentPackage,
    attachments: &[Attachment],
) -> (
    String,
    osci_transport::error::Result<osci_transport_client::ClientResponse>,
) {
    let alice_cipher_cert = key("alice_cipher.cer");
    client.send(Request::InitDialog).await.expect("InitDialog");
    let message_id = client
        .send(Request::GetMessageId)
        .await
        .expect("GetMessageId")
        .envelope
        .message_id()
        .unwrap()
        .expect("no MessageId");
    let store = client
        .send_delivery_with_attachments(
            Request::StoreDelivery {
                message_id: message_id.clone(),
                subject: Some("osci-transport content encryption test".into()),
            },
            content,
            attachments,
            &[
                CertSlot {
                    kind: RoleKind::Addressee,
                    is_signature: false,
                    cert_der: alice_cipher_cert.clone(),
                },
                CertSlot {
                    kind: RoleKind::Reader,
                    is_signature: false,
                    cert_der: alice_cipher_cert,
                },
            ],
        )
        .await;
    (message_id, store)
}

// Opens a signed, encrypted dialog as Alice and stores one delivery with
// `content`. Returns the dialog's client, the MessageId and the StoreDelivery
// result.
async fn store_as_alice(
    content: ContentPackage,
    attachments: &[Attachment],
) -> (
    Client<HttpTransport>,
    String,
    osci_transport::error::Result<osci_transport_client::ClientResponse>,
) {
    let mut client = alice_client(HttpTransport::new(INTERMED_URL), true);
    let (message_id, store) = store_in_new_dialog(&mut client, content, attachments).await;
    (client, message_id, store)
}

fn container_xml() -> Vec<u8> {
    let mut container = ContentContainer::new();
    container.id = Some("contentcontainer0".into());
    container.push_raw(RawContent::new(
        QName::new("http://www.osci.de/2002/04/osci", "XMLContent"),
        b"<greeting>hello</greeting>".to_vec(),
    ));
    osci_transport::wire::encode_content_container_bytes(&container).unwrap()
}

// Content-level EncryptedData naming the reader by a RetrievalMethod to the
// Addressee slot. The inline-certificate form is refused: fault 9100 as a
// ContentPackage child, fault 9000 inside a ContentContainer.
#[tokio::test]
#[ignore = "hits Governikus's public test Intermediär over the network"]
async fn content_encrypted_for_a_slot_is_stored_and_fetched() {
    let enc = osci_transport_crypto::encrypt_content_for_slot(
        &container_xml(),
        &key("alice_cipher.cer"),
        &CertSlot {
            kind: RoleKind::Addressee,
            is_signature: false,
            cert_der: Vec::new(),
        }
        .id(),
        &osci_transport_crypto::EncryptOptions::default(),
    )
    .unwrap();
    let mut content = ContentPackage::default();
    content.encrypted.push(enc);

    let (mut client, message_id, store) = store_as_alice(content, &[]).await;

    let store = store.expect("StoreDelivery failed");
    println!("StoreDelivery feedback: {:?}", store.feedback);
    let fetch = client
        .send(Request::FetchDelivery {
            message_id: Some(message_id),
        })
        .await
        .expect("FetchDelivery failed");
    println!("FetchDelivery feedback: {:?}", fetch.feedback);
    let package = fetch
        .envelope
        .content_package
        .expect("fetched delivery has no content");
    assert_eq!(package.encrypted.len(), 1, "{package:?}");
    // the key is named by the slot, so decrypt by its Id
    let container = osci_transport_crypto::decrypt_content(
        &package.encrypted[0],
        &RsaDecrypter::new(alice_cipher_key()),
        osci_transport_crypto::Recipient::CertificateId("addressee_cipher"),
    )
    .expect("decrypt fetched content");
    assert_eq!(container.id.as_deref(), Some("contentcontainer0"));
    let _ = client.send(Request::ExitDialog).await;
}

// FetchProcessCard's certificate slots allow no cipher certificate; sending
// one is refused with fault 9100.
#[tokio::test]
#[ignore = "hits Governikus's public test Intermediär over the network"]
async fn fetch_process_card_is_accepted() {
    let (mut client, message_id, store) = store_as_alice(ContentPackage::default(), &[]).await;
    store.expect("StoreDelivery failed");
    for (message_id, quantity) in [
        (None, None),
        (Some(message_id.clone()), None),
        (None, Some(5)),
        (Some(message_id), Some(5)),
    ] {
        let resp = client
            .send(Request::FetchProcessCard {
                message_id,
                quantity,
            })
            .await
            .expect("FetchProcessCard failed");
        assert_eq!(resp.message_type, MessageType::ResponseToFetchProcessCard);
        let cards = resp.process_cards().expect("typed process cards");
        println!("process cards: {cards:#?}");
        assert!(!cards.is_empty());
        assert!(cards.iter().all(|b| !b.message_id.is_empty()));
    }
    let _ = client.send(Request::ExitDialog).await;
}

fn two_attachments() -> Vec<Attachment> {
    let mut binary: Vec<u8> = (0..=255).collect();
    binary.extend_from_slice(b"\r\n--MIME_boundary\r\n");
    vec![
        Attachment {
            content_id: Some("xjustiz_nachricht.xml".into()),
            content_type: Some("text/xml".into()),
            data: b"<?xml version=\"1.0\"?><nachricht>\xc3\xa4</nachricht>".to_vec(),
        },
        Attachment {
            content_id: Some("Klage_1.pdf".into()),
            content_type: Some("application/pdf".into()),
            data: binary,
        },
    ]
}

fn container_referring_to(attachments: &[Attachment]) -> ContentContainer {
    let mut container = ContentContainer::new();
    container.id = Some("contentcontainer0".into());
    for a in attachments {
        let id = a.content_id.clone().unwrap();
        container.contents.push(Content {
            id: Some(id.clone()),
            kind: ContentKind::Reference {
                href: format!("cid:{id}"),
            },
        });
    }
    container
}

fn assert_same_attachments(got: &[Attachment], sent: &[Attachment]) {
    assert_eq!(got.len(), sent.len(), "{got:?}");
    for want in sent {
        let have = got
            .iter()
            .find(|a| a.content_id == want.content_id)
            .unwrap_or_else(|| panic!("attachment {:?} missing", want.content_id));
        assert_eq!(have.data, want.data, "{:?}", want.content_id);
        assert_eq!(have.content_type, want.content_type);
    }
}

// Two attachments referenced by cid: from a plain container, covered by the
// transport signature, then fetched back byte for byte.
#[tokio::test]
#[ignore = "hits Governikus's public test Intermediär over the network"]
async fn delivery_with_attachments_is_stored_and_fetched() {
    let sent = two_attachments();
    let content = ContentPackage {
        containers: vec![container_referring_to(&sent)],
        encrypted: Vec::new(),
    };

    let (mut client, message_id, store) = store_as_alice(content, &sent).await;

    store.expect("StoreDelivery with attachments failed");
    let fetch = client
        .send(Request::FetchDelivery {
            message_id: Some(message_id),
        })
        .await
        .expect("FetchDelivery failed");
    println!("FetchDelivery attachments: {:?}", fetch.attachments.len());
    assert_same_attachments(&fetch.attachments, &sent);
    let _ = client.send(Request::ExitDialog).await;
}

// A content-level encrypted container whose plaintext refers to cid: parts.
// The parts travel next to the envelope, unencrypted by us.
#[tokio::test]
#[ignore = "hits Governikus's public test Intermediär over the network"]
async fn encrypted_content_may_refer_to_attachments() {
    let sent = two_attachments();
    let container_xml =
        osci_transport::wire::encode_content_container_bytes(&container_referring_to(&sent))
            .unwrap();
    let enc = osci_transport_crypto::encrypt_content_for_slot(
        &container_xml,
        &key("alice_cipher.cer"),
        "addressee_cipher",
        &osci_transport_crypto::EncryptOptions::default(),
    )
    .unwrap();
    let mut content = ContentPackage::default();
    content.encrypted.push(enc);

    let (mut client, message_id, store) = store_as_alice(content, &sent).await;

    store.expect("StoreDelivery with encrypted content and attachments failed");
    let fetch = client
        .send(Request::FetchDelivery {
            message_id: Some(message_id),
        })
        .await
        .expect("FetchDelivery failed");
    assert_same_attachments(&fetch.attachments, &sent);
    let _ = client.send(Request::ExitDialog).await;
}

// Attachments encrypted one by one, their keys inside a container that is
// encrypted for Alice, as the EGVP profile and the Java library do. Fetched
// back, the parts are ciphertext until the container is opened.
#[tokio::test]
#[ignore = "hits Governikus's public test Intermediär over the network"]
async fn encrypted_attachments_are_stored_and_decrypted() {
    use osci_transport::crypto::SymmetricAlg;
    let plain = two_attachments();
    let mut container = container_referring_to(&plain);
    let mut parts = Vec::new();
    for a in &plain {
        let enc = osci_transport_crypto::encrypt_attachment(a, SymmetricAlg::Aes256Gcm).unwrap();
        container.encrypted.push(enc.encrypted_data);
        parts.push(enc.part);
    }
    let container_xml = osci_transport::wire::encode_content_container_bytes(&container).unwrap();
    let sealed = osci_transport_crypto::encrypt_content_for_slot(
        &container_xml,
        &key("alice_cipher.cer"),
        "addressee_cipher",
        &osci_transport_crypto::EncryptOptions::default(),
    )
    .unwrap();
    let mut content = ContentPackage::default();
    content.encrypted.push(sealed);

    let (mut client, message_id, store) = store_as_alice(content, &parts).await;

    store.expect("StoreDelivery with encrypted attachments failed");
    let fetch = client
        .send(Request::FetchDelivery {
            message_id: Some(message_id),
        })
        .await
        .expect("FetchDelivery failed");
    assert_same_attachments(&fetch.attachments, &parts);
    let package = fetch.envelope.content_package.expect("no content");
    let opened = osci_transport_crypto::decrypt_content(
        &package.encrypted[0],
        &RsaDecrypter::new(alice_cipher_key()),
        osci_transport_crypto::Recipient::CertificateId("addressee_cipher"),
    )
    .expect("decrypt fetched container");
    for (part, want) in fetch.attachments.iter().zip(&plain) {
        let got = osci_transport_crypto::decrypt_attachment(&opened, part).unwrap();
        assert_eq!(got, want.data, "{:?}", part.content_id);
    }
    let _ = client.send(Request::ExitDialog).await;
}

// Passes requests on, changing the body first once a delivery is out.
struct Tamper<F>(HttpTransport, F);

impl<F: Fn(Vec<u8>) -> Vec<u8> + Send + Sync> osci_transport_client::Transport for Tamper<F> {
    fn post(
        &self,
        body: Vec<u8>,
    ) -> osci_transport::router::BoxFuture<'_, osci_transport::error::Result<Vec<u8>>> {
        self.0.post((self.1)(body))
    }
}

// The Intermediär checks the digest of a cid: Reference against the part: a
// delivery whose part differs from the signed bytes is refused with 9601.
#[tokio::test]
#[ignore = "hits Governikus's public test Intermediär over the network"]
async fn part_that_differs_from_its_signed_digest() {
    let sent = two_attachments();
    let content = ContentPackage {
        containers: vec![container_referring_to(&sent)],
        encrypted: Vec::new(),
    };
    let swap = |body: Vec<u8>| {
        use base64::Engine;
        let b64 = base64::engine::general_purpose::STANDARD;
        let from = b64.encode(&two_attachments()[0].data);
        let to = b64.encode(b"<?xml version=\"1.0\"?><nachricht>TAMPERED</nachricht>");
        let text = String::from_utf8_lossy(&body).replace(&from, &to);
        text.into_bytes()
    };
    // unencrypted, so the body can be changed after signing
    let mut client = alice_client(Tamper(HttpTransport::new(INTERMED_URL), swap), false);

    let (_, store) = store_in_new_dialog(&mut client, content, &sent).await;

    // a refused signature comes back as feedback, not as a SOAP fault
    let feedback = store.expect("StoreDelivery got no response").feedback;
    assert!(
        feedback.iter().any(|f| f.code == "9601"),
        "expected 9601 (transport signature failed), got {feedback:?}"
    );
}

// XJustiz file names (up to 90 characters, spaces, umlauts, percent signs)
// as Content-IDs come back unchanged.
#[tokio::test]
#[ignore = "hits Governikus's public test Intermediär over the network"]
async fn awkward_content_ids() {
    let ids = [
        "Klage 1.pdf",
        "M\u{fc}ller_Klage.pdf",
        "Klage%201.pdf",
        &format!("{}.pdf", "a".repeat(86)),
        "Klage_6f9619ff-8b86-d011-b42d-00c04fc964ff.pdf.pkcs7",
    ];
    for id in ids {
        let sent = vec![Attachment {
            content_id: Some(id.to_string()),
            content_type: Some("application/pdf".into()),
            data: b"%PDF-1.4 test".to_vec(),
        }];
        let content = ContentPackage {
            containers: vec![container_referring_to(&sent)],
            encrypted: Vec::new(),
        };
        let (mut client, message_id, store) = store_as_alice(content, &sent).await;
        store.unwrap_or_else(|e| panic!("{id:?}: store refused: {e}"));
        let fetch = client
            .send(Request::FetchDelivery {
                message_id: Some(message_id),
            })
            .await
            .unwrap_or_else(|e| panic!("{id:?}: fetch failed: {e}"));
        assert_same_attachments(&fetch.attachments, &sent);
        let _ = client.send(Request::ExitDialog).await;
    }
}

// The same container for two readers, each in an EncryptedData of its own
// that names its slot (Addressee and OtherReader). The slots carry Alice's
// certificate for both: the test Intermediär has no second mailbox to try.
#[tokio::test]
#[ignore = "hits Governikus's public test Intermediär over the network"]
async fn content_for_two_readers_is_stored_and_fetched() {
    let opts = osci_transport_crypto::EncryptOptions::default();
    let xml = container_xml();
    let mut content = ContentPackage::default();
    for slot in ["addressee_cipher", "otherreader_cipher"] {
        let enc = osci_transport_crypto::encrypt_content_for_slot(
            &xml,
            &key("alice_cipher.cer"),
            slot,
            &opts,
        )
        .unwrap();
        content.encrypted.push(enc);
    }

    let (mut client, message_id, store) = store_as_alice(content, &[]).await;

    store.expect("StoreDelivery with two readers failed");
    let fetch = client
        .send(Request::FetchDelivery {
            message_id: Some(message_id),
        })
        .await
        .expect("FetchDelivery failed");
    let package = fetch.envelope.content_package.expect("no content");
    assert_eq!(package.encrypted.len(), 2);
    for (raw, slot) in package
        .encrypted
        .iter()
        .zip(["addressee_cipher", "otherreader_cipher"])
    {
        osci_transport_crypto::decrypt_content(
            raw,
            &RsaDecrypter::new(alice_cipher_key()),
            osci_transport_crypto::Recipient::CertificateId(slot),
        )
        .unwrap_or_else(|e| panic!("decrypt for {slot}: {e}"));
    }
    let _ = client.send(Request::ExitDialog).await;
}
