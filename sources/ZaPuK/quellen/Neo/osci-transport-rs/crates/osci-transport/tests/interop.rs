// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::fs;
use std::path::PathBuf;

use osci_transport::envelope::{Sniff, parse, sniff};
use osci_transport::mime::main_xml_part;

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/interop/corpus")
}

fn cases() -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    for entry in fs::read_dir(corpus_dir()).expect("corpus dir") {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) == Some("osci") {
            let name = path.file_stem().unwrap().to_string_lossy().into_owned();
            out.push((name, fs::read(&path).unwrap()));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

#[derive(Debug)]
#[allow(dead_code)] // fields are only shown via Debug in the scoreboard
enum Outcome {
    Sniffed(Sniff),
    MimeErr(String),
    SniffErr(String),
}

fn run(raw: &[u8]) -> Outcome {
    let xml = match main_xml_part(raw) {
        Ok(x) => x,
        Err(e) => return Outcome::MimeErr(e.to_string()),
    };
    match sniff(&xml) {
        Ok(s) => Outcome::Sniffed(s),
        Err(e) => Outcome::SniffErr(e.to_string()),
    }
}

#[test]
fn corpus_scoreboard() {
    let cases = cases();
    assert!(
        !cases.is_empty(),
        "no corpus files found at {}",
        corpus_dir().display()
    );

    let mut mime_ok = 0;
    let mut sniff_ok = 0;
    println!("\n{:<38} outcome", "file");
    println!("{}", "-".repeat(70));
    for (name, raw) in &cases {
        let outcome = run(raw);
        if !matches!(outcome, Outcome::MimeErr(_)) {
            mime_ok += 1;
        }
        if matches!(outcome, Outcome::Sniffed(_)) {
            sniff_ok += 1;
        }
        println!("{name:<38} {outcome:?}");
    }
    println!("{}", "-".repeat(70));
    println!(
        "mime ok: {mime_ok}/{}   sniff ok: {sniff_ok}/{}",
        cases.len(),
        cases.len()
    );

    // Every corpus file must at least split as MIME.
    assert_eq!(
        mime_ok,
        cases.len(),
        "some corpus files failed MIME parsing"
    );
}

// Files whose Body must classify to a specific message type.
#[test]
fn known_message_types() {
    use osci_transport::MessageType::*;
    let expect: &[(&str, osci_transport::MessageType)] = &[
        ("StoreDelivery", StoreDelivery),
        ("ResponseToStoreDelivery", ResponseToStoreDelivery),
        ("AcceptDelivery", AcceptDelivery),
        ("ResponseToAcceptDelivery", ResponseToAcceptDelivery),
        ("FetchDelivery", FetchDelivery),
        ("ResponseToFetchDelivery", ResponseToFetchDelivery),
        ("FetchProcessCard", FetchProcessCard),
        ("ResponseToFetchProcessCard", ResponseToFetchProcessCard),
        ("ForwardDelivery", ForwardDelivery),
        ("ResponseToForwardDelivery", ResponseToForwardDelivery),
        ("MediateDelivery", MediateDelivery),
        ("ResponseToMediateDelivery", ResponseToMediateDelivery),
        ("ProcessDelivery", ProcessDelivery),
        ("ResponseToProcessDelivery", ResponseToProcessDelivery),
        ("PartialStoreDelivery", PartialStoreDelivery),
        (
            "ResponseToPartialStoreDelivery",
            ResponseToPartialStoreDelivery,
        ),
        ("PartialFetchDelivery", PartialFetchDelivery),
        (
            "ResponseToPartialFetchDelivery",
            ResponseToPartialFetchDelivery,
        ),
    ];

    let mut failures = Vec::new();
    for (name, want) in expect {
        let path = corpus_dir().join(format!("{name}.osci"));
        let Ok(raw) = fs::read(&path) else {
            failures.push(format!("{name}: missing corpus file"));
            continue;
        };
        match run(&raw) {
            Outcome::Sniffed(Sniff::Message(got)) if got == *want => {}
            other => failures.push(format!("{name}: expected {want:?}, got {other:?}")),
        }
    }
    assert!(
        failures.is_empty(),
        "message type mismatches:\n{}",
        failures.join("\n")
    );
}

// Every non-encrypted corpus message must parse into an Envelope with a
// ControlBlock and a message type.
#[test]
fn unencrypted_envelopes_parse() {
    let mut failures = Vec::new();
    let mut parsed = 0;
    for (name, raw) in cases() {
        let xml = main_xml_part(&raw).expect("mime");
        if matches!(sniff(&xml), Ok(Sniff::Encrypted) | Ok(Sniff::Fault)) {
            continue;
        }
        let env = match parse(&xml) {
            Ok(e) => e,
            Err(e) => {
                failures.push(format!("{name}: parse failed: {e}"));
                continue;
            }
        };
        if env.control_block.is_none() {
            failures.push(format!("{name}: no ControlBlock"));
        } else if env.message_type.is_none() {
            failures.push(format!("{name}: no message type"));
        } else {
            parsed += 1;
        }
    }
    println!("parsed {parsed} unencrypted envelopes");
    assert!(
        failures.is_empty(),
        "envelope parse failures:\n{}",
        failures.join("\n")
    );
}

// StoreDelivery.osci: one ContentContainer with an inline Base64Content, a cid
// reference, a content-level signature, plus a package-level EncryptedData.
#[test]
fn store_delivery_content_package() {
    use osci_transport::parts::ContentKind;

    let raw = fs::read(corpus_dir().join("StoreDelivery.osci")).unwrap();
    let xml = main_xml_part(&raw).unwrap();
    let env = parse(&xml).unwrap();
    let pkg = env.content_package.expect("content package");

    assert_eq!(pkg.containers.len(), 1);
    assert_eq!(pkg.encrypted.len(), 1);

    let cc = &pkg.containers[0];
    assert_eq!(cc.id.as_deref(), Some("contentcontainer0"));
    assert!(cc.signature.is_some());
    assert_eq!(cc.contents.len(), 2);

    let inline = cc
        .contents
        .iter()
        .find_map(|c| match &c.kind {
            ContentKind::Inline(r) => Some(r),
            _ => None,
        })
        .expect("inline content");
    assert_eq!(inline.xml, b"Any content data.");

    let has_ref = cc
        .contents
        .iter()
        .any(|c| matches!(&c.kind, ContentKind::Reference { href } if href == "cid:test.txt"));
    assert!(has_ref);
}

// Encrypted content hangs off a CipherReference. The cipher part is returned
// as sent: base64 text for text/base64, not decoded.
#[test]
fn cipher_part_of_an_encrypted_message_is_an_attachment() {
    let raw = fs::read(corpus_dir().join("StrDel_Encrypted_cbc.osci")).unwrap();

    let atts = osci_transport::mime::attachments(&raw).unwrap();

    assert_eq!(atts.len(), 1);
    assert_eq!(atts[0].content_id.as_deref(), Some("osci_enc"));
    assert_eq!(atts[0].content_type.as_deref(), Some("text/base64"));
    assert!(
        atts[0]
            .data
            .iter()
            .all(|b| b.is_ascii_graphic() || b.is_ascii_whitespace())
    );
}

// The Java library's own sample: one bundle with the three stations and six
// certificate checks.
#[test]
fn process_cards_of_the_java_sample_are_typed() {
    use osci_transport::process_card::{Timestamp, parse_process_card_bundles};

    let raw = fs::read(corpus_dir().join("ResponseToFetchProcessCard.osci")).unwrap();
    let xml = osci_transport::mime::main_xml_part(&raw).unwrap();

    let bundles = parse_process_card_bundles(&xml).unwrap();

    assert_eq!(bundles.len(), 1);
    let b = &bundles[0];
    assert_eq!(b.message_id, "MTA3NjYwMDc3MDUzMg==");
    assert_eq!(b.card.recent_modification, "2004-02-12T16:46:28+01:00");
    let plain = |s: &str| Some(Timestamp::Plain(s.into()));
    assert_eq!(b.card.creation, plain("2004-02-12T16:46:15+01:00"));
    assert_eq!(b.card.forwarding, plain("2004-02-12T16:46:21+01:00"));
    assert_eq!(b.card.reception, plain("2004-02-12T16:46:28+01:00"));
    assert_eq!(b.card.subject.as_deref(), Some("Subject"));
    assert_eq!(b.inspections.len(), 6);
    assert_eq!(b.inspections[0].serial_number, "1298453874");
    assert_eq!(b.inspections[0].cert_type, "unknown");
    assert_eq!(b.inspections[0].math_result, "ok");
    assert_eq!(b.inspections[0].offline_result, "valid");
    assert_eq!(b.inspections[0].subject_name, None);
}
