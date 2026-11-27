// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::fs;
use std::path::PathBuf;

use osci_transport::crypto::SymmetricAlg;
use osci_transport::mime::Attachment;
use osci_transport::parts::{Content, ContentContainer, ContentKind, ContentPackage};
use osci_transport::wire::encode_content_container_bytes;
use osci_transport_crypto::{
    EncryptOptions, Recipient, RsaDecrypter, decrypt_attachment, decrypt_content,
    encrypt_attachment, encrypt_content_for_slot,
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

fn pdf() -> Attachment {
    // not XML, with a boundary-looking line inside
    let mut data = b"%PDF-1.7\r\n".to_vec();
    data.extend((0..=255u8).cycle().take(5000));
    data.extend_from_slice(b"\r\n--MIME_boundary\r\n");
    Attachment {
        content_id: Some("Klage 1.pdf".into()),
        content_type: Some("application/pdf".into()),
        data,
    }
}

// What a sender does: encrypt the part, put its EncryptedData and a reference
// into a container, encrypt the container for the reader. The reader reverses it.
#[test]
fn attachment_survives_the_container_round_trip() {
    let cert = key("test_osci-manager_cypher.cer");
    let plain = pdf();
    for alg in [SymmetricAlg::Aes256Gcm, SymmetricAlg::Aes128Cbc] {
        let enc = encrypt_attachment(&plain, alg).unwrap();
        assert_eq!(enc.part.content_id, plain.content_id);
        assert_eq!(enc.part.content_type, plain.content_type);
        assert_ne!(enc.part.data, plain.data);

        let mut cc = ContentContainer::new();
        cc.id = Some("project_coco".into());
        cc.contents.push(Content {
            id: Some("Klage 1.pdf".into()),
            kind: ContentKind::Reference {
                href: "cid:Klage 1.pdf".into(),
            },
        });
        cc.encrypted.push(enc.encrypted_data.clone());
        let xml = encode_content_container_bytes(&cc).unwrap();
        // the key must not be readable in the container once it is encrypted
        let sealed =
            encrypt_content_for_slot(&xml, &cert, "addressee_cipher", &EncryptOptions::default())
                .unwrap();
        assert!(!String::from_utf8_lossy(&sealed).contains("MgmtData"));

        let pem = String::from_utf8(key("test_osci-manager_cypher.key.pem")).unwrap();
        let dec = RsaDecrypter::new(PrivateKey::from_pkcs8_pem(&pem).unwrap());
        let opened =
            decrypt_content(&sealed, &dec, Recipient::CertificateId("addressee_cipher")).unwrap();
        assert_eq!(opened.encrypted.len(), 1);

        let back = decrypt_attachment(&opened, &enc.part).unwrap();
        assert_eq!(back, plain.data);

        // the package sees the cid: through the EncryptedData, so a part with
        // that id is accepted and a stray one is refused
        let package = ContentPackage {
            containers: vec![opened],
            encrypted: Vec::new(),
        };
        package
            .check_attachments(std::slice::from_ref(&enc.part))
            .unwrap();
        let mut stray = enc.part.clone();
        stray.content_id = Some("other".into());
        assert!(package.check_attachments(&[enc.part, stray]).is_err());
    }
}

#[test]
fn decrypting_needs_its_key_and_an_untampered_part() {
    let enc = encrypt_attachment(&pdf(), SymmetricAlg::Aes256Gcm).unwrap();
    let mut cc = ContentContainer::new();
    cc.encrypted.push(enc.encrypted_data);

    let mut tampered = enc.part.clone();
    *tampered.data.last_mut().unwrap() ^= 1;
    assert!(decrypt_attachment(&cc, &tampered).is_err(), "GCM tag");

    let mut other = enc.part.clone();
    other.content_id = Some("elsewhere".into());
    let err = decrypt_attachment(&cc, &other).unwrap_err();
    assert!(err.to_string().contains("elsewhere"), "{err}");

    assert!(decrypt_attachment(&ContentContainer::new(), &enc.part).is_err());
}

#[test]
fn awkward_content_ids_are_escaped() {
    let mut plain = pdf();
    plain.content_id = Some("a\"b<c>&d".into());
    let enc = encrypt_attachment(&plain, SymmetricAlg::Aes256Gcm).unwrap();
    let mut cc = ContentContainer::new();
    cc.encrypted.push(enc.encrypted_data);
    let xml = encode_content_container_bytes(&cc).unwrap();
    let parsed = osci_transport::wire::parse_content_container(&xml).unwrap();
    assert_eq!(decrypt_attachment(&parsed, &enc.part).unwrap(), plain.data);
}

#[test]
fn an_attachment_needs_a_content_id() {
    let mut plain = pdf();
    plain.content_id = None;
    assert!(encrypt_attachment(&plain, SymmetricAlg::Aes256Gcm).is_err());
    plain.content_id = Some(String::new());
    assert!(encrypt_attachment(&plain, SymmetricAlg::Aes256Gcm).is_err());
}

// The Java library reads the OAEP parameters from the EncryptedKey and has
// nothing to decrypt with when they are missing.
#[test]
fn content_key_carries_the_oaep_parameters() {
    let cert = key("test_osci-manager_cypher.cer");
    let sealed = encrypt_content_for_slot(
        b"<osci:ContentContainer/>",
        &cert,
        "addressee_cipher",
        &EncryptOptions::default(),
    )
    .unwrap();
    let text = String::from_utf8(sealed).unwrap();
    assert!(text.contains("xmlenc11#mgf1sha256"), "{text}");
    assert!(text.contains("xmlenc#sha256"), "{text}");
}
