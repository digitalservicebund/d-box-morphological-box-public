// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::fs;
use std::path::PathBuf;

use osci_transport::content::RawContent;
use osci_transport::crypto::{KeyTransportAlg, SymmetricAlg};
use osci_transport::parts::ContentContainer;
use osci_transport::wire::encode_content_container_bytes;
use osci_transport::xml::QName;
use osci_transport_crypto::{
    EncryptOptions, Recipient, RsaDecrypter, decrypt_content, decrypt_transport, encrypt_content,
    encrypt_content_for_slot, encrypt_content_with, encrypt_transport, encrypt_transport_with,
};
use pkcs8::DecodePrivateKey;
use rsa::PrivateKey;

fn key(p: &str) -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/interop/keys")
            .join(p),
    )
    .unwrap()
}

#[test]
fn encrypt_then_decrypt_transport() {
    let cert = key("test_osci-manager_cypher.cer");
    let inner = b"MIME-Version: 1.0\r\n\
Content-Type: text/xml\r\n\
Content-ID: <osci@message>\r\n\r\n\
<soap:Envelope xmlns:soap=\"x\"><soap:Body/></soap:Envelope>\r\n";

    let encrypted = encrypt_transport(inner, &cert).unwrap();

    let pem = String::from_utf8(key("test_osci-manager_cypher.key.pem")).unwrap();
    let dec = RsaDecrypter::new(PrivateKey::from_pkcs8_pem(&pem).unwrap()).with_certificate(cert);

    let back = decrypt_transport(&encrypted, &dec).unwrap();
    assert_eq!(back, inner);
}

#[test]
fn encrypt_then_decrypt_content() {
    let cert = key("test_osci-manager_cypher.cer");
    let mut cc = ContentContainer::new();
    cc.id = Some("cc0".into());
    cc.push_raw(RawContent::new(QName::new("", ""), b"geheim".to_vec()));
    let xml = encode_content_container_bytes(&cc).unwrap();

    let encrypted = encrypt_content(&xml, &cert).unwrap();

    let pem = String::from_utf8(key("test_osci-manager_cypher.key.pem")).unwrap();
    let dec = RsaDecrypter::new(PrivateKey::from_pkcs8_pem(&pem).unwrap());
    let back = decrypt_content(&encrypted, &dec, Recipient::Certificate(&cert)).unwrap();

    assert_eq!(back.id.as_deref(), Some("cc0"));
    assert_eq!(back.contents.len(), 1);
}

fn legacy_options() -> EncryptOptions {
    EncryptOptions {
        key_transport: KeyTransportAlg::Rsa15,
        symmetric: SymmetricAlg::Aes256Cbc,
    }
}

// The older AES-256-CBC + RSA-PKCS1v1.5 combo, for deployments that predate
// RSA-OAEP's explicit MGF/DigestMethod parameters; see
// against_test_intermediary.rs for its scope against a real Intermediär.
#[test]
fn encrypt_then_decrypt_transport_legacy_cbc_rsa15() {
    let cert = key("test_osci-manager_cypher.cer");
    let inner = b"MIME-Version: 1.0\r\n\
Content-Type: text/xml\r\n\
Content-ID: <osci@message>\r\n\r\n\
<soap:Envelope xmlns:soap=\"x\"><soap:Body/></soap:Envelope>\r\n";

    let encrypted = encrypt_transport_with(inner, &cert, &legacy_options()).unwrap();

    let pem = String::from_utf8(key("test_osci-manager_cypher.key.pem")).unwrap();
    let dec = RsaDecrypter::new(PrivateKey::from_pkcs8_pem(&pem).unwrap()).with_certificate(cert);

    let back = decrypt_transport(&encrypted, &dec).unwrap();
    assert_eq!(back, inner);
}

#[test]
fn encrypt_then_decrypt_content_legacy_cbc_rsa15() {
    let cert = key("test_osci-manager_cypher.cer");
    let mut cc = ContentContainer::new();
    cc.id = Some("cc0".into());
    cc.push_raw(RawContent::new(QName::new("", ""), b"geheim".to_vec()));
    let xml = encode_content_container_bytes(&cc).unwrap();

    let encrypted = encrypt_content_with(&xml, &cert, &legacy_options()).unwrap();

    let pem = String::from_utf8(key("test_osci-manager_cypher.key.pem")).unwrap();
    let dec = RsaDecrypter::new(PrivateKey::from_pkcs8_pem(&pem).unwrap());
    let back = decrypt_content(&encrypted, &dec, Recipient::Certificate(&cert)).unwrap();

    assert_eq!(back.id.as_deref(), Some("cc0"));
    assert_eq!(back.contents.len(), 1);
}

// The reader is named by a RetrievalMethod to a header slot, as the Java
// library does; the Governikus test Intermediär refuses the inline form.
#[test]
fn content_for_a_slot_names_the_reader_by_retrieval_method() {
    let cert = key("test_osci-manager_cypher.cer");
    let mut cc = ContentContainer::new();
    cc.id = Some("cc0".into());
    cc.push_raw(RawContent::new(QName::new("", ""), b"geheim".to_vec()));
    let xml = encode_content_container_bytes(&cc).unwrap();
    let slot = osci_transport_crypto::CertSlot {
        kind: osci_transport::role::RoleKind::Addressee,
        is_signature: false,
        cert_der: cert.clone(),
    };

    let encrypted =
        encrypt_content_for_slot(&xml, &cert, &slot.id(), &EncryptOptions::default()).unwrap();

    let text = String::from_utf8(encrypted.clone()).unwrap();
    assert!(text.contains(r##"URI="#addressee_cipher""##), "{text}");
    assert!(!text.contains("X509Data><ds:X509Certificate"), "{text}");
    let pem = String::from_utf8(key("test_osci-manager_cypher.key.pem")).unwrap();
    let dec = RsaDecrypter::new(PrivateKey::from_pkcs8_pem(&pem).unwrap());
    let back = decrypt_content(
        &encrypted,
        &dec,
        Recipient::CertificateId("addressee_cipher"),
    )
    .unwrap();
    assert_eq!(back.id.as_deref(), Some("cc0"));
    assert!(encrypt_content_for_slot(&xml, &cert, "a\"b", &EncryptOptions::default()).is_err());
}
