// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::fs;
use std::path::PathBuf;

use base64::Engine;
use osci_transport::crypto::SignatureAlg;
use osci_transport::dialog::ControlBlock;
use osci_transport::mime::Attachment;
use osci_transport::wire::{Request, RequestEnvelope};
use osci_transport_crypto::{
    CertSlot, Role, sign_envelope, sign_envelope_with, sign_envelope_with_attachments,
    verify_transport_signature,
};

fn key(p: &str) -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/interop/keys")
            .join(p),
    )
    .unwrap()
}

#[test]
fn sign_then_verify_a_request() {
    let cb = ControlBlock {
        id: Some("controlblock".into()),
        challenge: Some("aGVsbG8=".into()),
        sequence_number: Some(0),
        ..Default::default()
    };
    let unsigned = RequestEnvelope::new(Request::FetchDelivery { message_id: None }, cb)
        .encode()
        .unwrap();

    let pem = key("test_osci-manager_cypher.key.pem");
    let der = key("test_osci-manager_cypher.cer");

    let signed = sign_envelope(&unsigned, Role::Client, &pem, &der).unwrap();

    let report = verify_transport_signature(&signed, |_| true).unwrap();
    assert!(report.valid, "{:?}", report.reason);
    assert!(report.verified_refs.contains(&"#controlblock".to_string()));
    assert!(report.verified_refs.contains(&"#body".to_string()));
}

#[test]
fn tampering_after_signing_is_detected() {
    let cb = ControlBlock {
        id: Some("controlblock".into()),
        sequence_number: Some(0),
        ..Default::default()
    };
    let unsigned = RequestEnvelope::new(Request::GetMessageId, cb)
        .encode()
        .unwrap();
    let pem = key("test_osci-manager_cypher.key.pem");
    let der = key("test_osci-manager_cypher.cer");

    let signed = sign_envelope(&unsigned, Role::Client, &pem, &der).unwrap();
    // change the signed ControlBlock
    let tampered = String::from_utf8(signed)
        .unwrap()
        .replace(r#"SequenceNumber="0""#, r#"SequenceNumber="7""#)
        .into_bytes();

    let report = verify_transport_signature(&tampered, |_| true).unwrap();
    assert!(!report.valid);
}

// Real Intermediäre need the client's cipher cert to encrypt a response back
// (some reject a request outright without it - SOAP fault 9500 in practice).
#[test]
fn extra_cert_slots_are_embedded_and_covered_by_the_signature() {
    let cb = ControlBlock {
        id: Some("controlblock".into()),
        challenge: Some("aGVsbG8=".into()),
        sequence_number: Some(0),
        ..Default::default()
    };
    let unsigned = RequestEnvelope::new(Request::InitDialog, cb)
        .encode()
        .unwrap();

    let sig_pem = key("test_osci-manager_cypher.key.pem");
    let sig_cert = key("test_osci-manager_cypher.cer");
    let cipher_cert = key("Reader01.cer");

    let signed = sign_envelope_with(
        &unsigned,
        Role::Client,
        &sig_pem,
        &sig_cert,
        &[CertSlot::originator_cipher(cipher_cert.clone())],
    )
    .unwrap();

    let s = String::from_utf8(signed.clone()).unwrap();
    assert!(s.contains("<osci:CipherCertificateOriginator"));
    assert!(s.contains("<osci:SignatureCertificateOriginator"));

    let report = verify_transport_signature(&signed, |_| true).unwrap();
    assert!(report.valid, "{:?}", report.reason);
    assert!(
        report
            .verified_refs
            .contains(&"#nonintermediarycertificates".to_string())
    );
}

fn att(id: &str, data: &[u8]) -> Attachment {
    Attachment {
        content_id: Some(id.into()),
        content_type: Some("application/octet-stream".into()),
        data: data.to_vec(),
    }
}

fn unsigned_store_delivery() -> Vec<u8> {
    let cb = ControlBlock {
        id: Some("controlblock".into()),
        sequence_number: Some(0),
        ..Default::default()
    };
    RequestEnvelope::new(Request::GetMessageId, cb)
        .encode()
        .unwrap()
}

// Java writes a cid: Reference per attachment with no transforms and a digest
// over the decoded bytes, ahead of the certificate blocks.
#[test]
fn attachments_are_signed_by_cid_reference() {
    use sha2::Digest;
    let pem = key("test_osci-manager_cypher.key.pem");
    let der = key("test_osci-manager_cypher.cer");
    let data = b"\x00\x01 binary \xff";

    let signed = sign_envelope_with_attachments(
        &unsigned_store_delivery(),
        Role::Client,
        &pem,
        &der,
        &[],
        SignatureAlg::RsaSha256,
        &[att("doc.pdf", data), att("doc.pdf.p7s", b"sig")],
    )
    .unwrap();

    let xml = String::from_utf8(signed.clone()).unwrap();
    let digest = base64::engine::general_purpose::STANDARD.encode(sha2::Sha256::digest(data));
    assert!(
        xml.contains(&format!(
            r#"<ds:Reference URI="cid:doc.pdf"><ds:DigestMethod Algorithm="http://www.w3.org/2001/04/xmlenc#sha256"></ds:DigestMethod><ds:DigestValue>{digest}</ds:DigestValue></ds:Reference>"#
        )),
        "{xml}"
    );
    let cid = xml.find(r#"URI="cid:doc.pdf""#).unwrap();
    let certs = xml.find(r##"URI="#nonintermediarycertificates""##).unwrap();
    assert!(cid < certs);

    let report = verify_transport_signature(&signed, |_| true).unwrap();
    assert!(report.valid, "{:?}", report.reason);
    assert_eq!(
        report.attachment_refs,
        vec!["cid:doc.pdf".to_string(), "cid:doc.pdf.p7s".to_string()]
    );

    // the digest sits inside SignedInfo, so changing it breaks the signature
    let tampered = xml.replace(&digest, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
    assert!(
        !verify_transport_signature(tampered.as_bytes(), |_| true)
            .unwrap()
            .valid
    );
}

// The receiver hashes the parts: matching bytes pass, anything else is
// refused, and so is a part the signature does not cover.
#[test]
fn received_parts_are_checked_against_the_signed_digests() {
    let pem = key("test_osci-manager_cypher.key.pem");
    let der = key("test_osci-manager_cypher.cer");
    let parts = [
        att("doc.pdf", b"\x00\x01 binary \xff"),
        att("note", b"hello"),
    ];
    for alg in [
        SignatureAlg::RsaSha1,
        SignatureAlg::RsaSha256,
        SignatureAlg::RsaSha512,
    ] {
        let signed = sign_envelope_with_attachments(
            &unsigned_store_delivery(),
            Role::Client,
            &pem,
            &der,
            &[],
            alg,
            &parts,
        )
        .unwrap();
        let report = verify_transport_signature(&signed, |_| true).unwrap();
        assert_eq!(report.attachment_digests.len(), 2, "{alg:?}");

        report.check_attachments(&parts).unwrap();
        // order does not matter
        report
            .check_attachments(&[parts[1].clone(), parts[0].clone()])
            .unwrap();

        let mut changed = parts.clone();
        changed[0].data[0] ^= 1;
        let err = report.check_attachments(&changed).unwrap_err();
        assert!(err.to_string().contains("doc.pdf"), "{err}");

        let err = report.check_attachments(&parts[..1]).unwrap_err();
        assert!(err.to_string().contains("cid:note"), "{err}");

        let mut extra = parts.to_vec();
        extra.push(att("unsigned", b"x"));
        let err = report.check_attachments(&extra).unwrap_err();
        assert!(err.to_string().contains("unsigned"), "{err}");
    }
}

#[test]
fn an_invalid_signature_gives_no_digests_to_trust() {
    let pem = key("test_osci-manager_cypher.key.pem");
    let der = key("test_osci-manager_cypher.cer");
    let parts = [att("a", b"1")];
    let signed = sign_envelope_with_attachments(
        &unsigned_store_delivery(),
        Role::Client,
        &pem,
        &der,
        &[],
        SignatureAlg::RsaSha256,
        &parts,
    )
    .unwrap();
    let report = verify_transport_signature(&signed, |_| false).unwrap();
    assert!(!report.valid);
    assert!(report.attachment_digests.is_empty());
    assert!(report.check_attachments(&parts).is_err());
}

#[test]
fn unusable_attachment_ids_are_refused_for_signing() {
    let pem = key("test_osci-manager_cypher.key.pem");
    let der = key("test_osci-manager_cypher.cer");
    for attachments in [
        vec![att("a", b"1"), att("a", b"2")],
        vec![att("a\"b", b"1")],
        vec![att("", b"1")],
        vec![Attachment {
            content_id: None,
            content_type: None,
            data: vec![],
        }],
    ] {
        assert!(
            sign_envelope_with_attachments(
                &unsigned_store_delivery(),
                Role::Client,
                &pem,
                &der,
                &[],
                SignatureAlg::RsaSha256,
                &attachments,
            )
            .is_err()
        );
    }
}

// The Java library refuses a SignatureCertificateIntermediary inside
// NonIntermediaryCertificates, so a Supplier's own certificate gets a block of
// its own. The signature must still verify and cover that block.
#[test]
fn supplier_certificate_is_in_the_intermediary_block() {
    let envelope = RequestEnvelope::new(
        Request::GetMessageId,
        ControlBlock {
            id: Some("controlblock".into()),
            sequence_number: Some(0),
            ..Default::default()
        },
    )
    .encode()
    .unwrap();

    let signed = sign_envelope(
        &envelope,
        Role::Supplier,
        &key("test_osci-manager_cypher.key.pem"),
        &key("test_osci-manager_cypher.cer"),
    )
    .unwrap();

    let xml = String::from_utf8(signed).unwrap();
    let block = xml.split("<osci:IntermediaryCertificates").nth(1).unwrap();
    assert!(block.contains("<osci:SignatureCertificateIntermediary"));
    assert!(!xml.contains("<osci:NonIntermediaryCertificates"));
    assert!(xml.contains(r##"<ds:Reference URI="#intermediarycertificates">"##));
    let report = verify_transport_signature(xml.as_bytes(), |_| true).unwrap();
    assert!(report.valid, "{:?}", report.reason);
}
