// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::fs;
use std::path::PathBuf;

use osci_transport::mime::main_xml_part;
use osci_transport_crypto::{verify_content_signature, verify_transport_signature};

fn corpus(name: &str) -> Vec<u8> {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/interop/corpus")
        .join(name);
    fs::read(p).unwrap()
}

#[test]
fn client_signature_is_valid() {
    let xml = main_xml_part(&corpus("FetchProcessCard.osci")).unwrap();
    let report = verify_transport_signature(&xml, |_| true).unwrap();

    assert!(report.valid, "{:?}", report.reason);
    assert!(report.verified_refs.contains(&"#controlblock".to_string()));
    assert!(report.verified_refs.contains(&"#body".to_string()));
}

#[test]
fn untrusted_certificate_fails_the_signature() {
    let xml = main_xml_part(&corpus("FetchProcessCard.osci")).unwrap();
    let report = verify_transport_signature(&xml, |_| false).unwrap();
    assert!(!report.valid);
}

#[test]
fn corpus_client_signatures_verify() {
    let files = [
        "StoreDelivery.osci",
        "FetchDelivery.osci",
        "FetchProcessCard.osci",
        "ForwardDelivery.osci",
        "MediateDelivery.osci",
        "ProcessDelivery.osci",
        "AcceptDelivery.osci",
    ];
    for f in files {
        let xml = main_xml_part(&corpus(f)).unwrap();
        let report = verify_transport_signature(&xml, |_| true).unwrap();
        assert!(report.valid, "{f}: {:?}", report.reason);
    }
}

#[test]
fn tampered_body_breaks_the_signature() {
    let xml = main_xml_part(&corpus("FetchProcessCard.osci")).unwrap();
    let tampered = String::from_utf8(xml)
        .unwrap()
        .replace("Limit=\"12\"", "Limit=\"99\"")
        .into_bytes();

    let report = verify_transport_signature(&tampered, |_| true).unwrap();
    assert!(!report.valid);
}

#[test]
fn content_container_signature_is_valid() {
    let xml = main_xml_part(&corpus("StoreDelivery.osci")).unwrap();
    let report = verify_content_signature(&xml, "contentcontainer0", |_| true).unwrap();

    assert!(report.valid, "{:?}", report.reason);
    assert!(report.verified_refs.contains(&"#content0".to_string()));
    assert!(report.verified_refs.contains(&"#content1".to_string()));
    // the cid: reference is an external attachment, not digested locally
    assert!(report.attachment_refs.contains(&"cid:test.txt".to_string()));
    // the digest the container's own signature gives for it is checkable
    assert_eq!(report.attachment_digests.len(), 1);
    let parts = osci_transport::mime::attachments(&corpus("StoreDelivery.osci")).unwrap();
    report.check_attachments(&parts).unwrap();
    let mut changed = parts;
    changed[0].data.push(b'!');
    assert!(report.check_attachments(&changed).is_err());
}

#[test]
fn tampered_content_breaks_the_container_signature() {
    let xml = main_xml_part(&corpus("StoreDelivery.osci")).unwrap();
    let tampered = String::from_utf8(xml)
        .unwrap()
        .replace(
            "QW55IGNvbnRlbnQgZGF0YS4=",
            "QW55IGNvbnRlbnQgZGF0YS4tdGFtcGVyZWQ=",
        )
        .into_bytes();

    let report = verify_content_signature(&tampered, "contentcontainer0", |_| true).unwrap();
    assert!(!report.valid);
}

#[test]
fn header_block_outside_the_signature_is_rejected() {
    let xml = main_xml_part(&corpus("FetchProcessCard.osci")).unwrap();
    let extended = String::from_utf8(xml)
        .unwrap()
        .replace("</soap:Header>", r#"<x Id="extra"/></soap:Header>"#)
        .into_bytes();

    let report = verify_transport_signature(&extended, |_| true).unwrap();
    assert!(!report.valid);
    assert!(
        report.reason.as_deref().unwrap_or("").contains("extra"),
        "{:?}",
        report.reason
    );
}

#[test]
fn content_outside_the_container_signature_is_rejected() {
    let xml = main_xml_part(&corpus("StoreDelivery.osci")).unwrap();
    let extended = String::from_utf8(xml)
        .unwrap()
        .replacen(
            "</osci:ContentContainer>",
            r#"<osci:Content Id="extra"><osci:XMLContent><a/></osci:XMLContent></osci:Content></osci:ContentContainer>"#,
            1,
        )
        .into_bytes();

    let report = verify_content_signature(&extended, "contentcontainer0", |_| true).unwrap();
    assert!(!report.valid);
    assert!(
        report.reason.as_deref().unwrap_or("").contains("extra"),
        "{:?}",
        report.reason
    );
}

#[test]
fn two_attachments_with_one_content_id_are_refused() {
    let xml = main_xml_part(&corpus("StoreDelivery.osci")).unwrap();
    let report = verify_content_signature(&xml, "contentcontainer0", |_| true).unwrap();
    let mut parts = osci_transport::mime::attachments(&corpus("StoreDelivery.osci")).unwrap();
    let mut forged = parts[0].clone();
    forged.data.push(b'!');
    parts.push(forged);
    assert!(report.check_attachments(&parts).is_err());
}
