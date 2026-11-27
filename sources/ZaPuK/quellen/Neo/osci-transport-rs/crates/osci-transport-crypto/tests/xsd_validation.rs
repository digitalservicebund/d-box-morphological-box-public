// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

// Validates osci-transport's own wire output against the vendored OSCI XSDs
// (`third_party/osci-schema/`, from the official Java osci-bibliothek) using
// `xmllint`. Skipped, not failed, if `xmllint` isn't installed.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use osci_transport::content::RawContent;
use osci_transport::dialog::ControlBlock;
use osci_transport::header::{DesiredLanguages, QualityOfTimestamp};
use osci_transport::parts::{ContentContainer, ContentPackage};
use osci_transport::role::RoleKind;
use osci_transport::wire::{Request, RequestEnvelope};
use osci_transport::xml::QName;
use osci_transport_crypto::{CertSlot, Role, sign_envelope_with};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn key(name: &str) -> Vec<u8> {
    fs::read(repo_root().join("tests/interop/keys").join(name)).unwrap()
}

fn xmllint_available() -> bool {
    Command::new("xmllint")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
}

/// Copies `third_party/osci-schema` into `dir`, then fixes one byte the
/// official Java osci-bibliothek's own schema ships with: `order.xsd`'s
/// `RecentModification` element declares `type="xsd:dateTime "`, a trailing
/// space inside the QName that Xerces (the Java lib's XML parser) tolerates
/// but libxml2's schema compiler rejects outright. The fix is applied only to
/// this throwaway copy, not to the vendored files, so the vendored schema
/// stays byte-identical to upstream (no drift from the Java implementation
/// or other users of the same schema).
fn prepare_patched_schema(dir: &Path) {
    let src = repo_root().join("third_party/osci-schema");
    copy_dir(&src, dir);
    let order_xsd = dir.join("order.xsd");
    let patched = fs::read_to_string(&order_xsd).unwrap().replace(
        r#"type="xsd:dateTime " minOccurs="0"/>"#,
        r#"type="xsd:dateTime" minOccurs="0"/>"#,
    );
    fs::write(order_xsd, patched).unwrap();
}

fn copy_dir(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let dest_path = dst.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &dest_path);
        } else {
            fs::copy(entry.path(), dest_path).unwrap();
        }
    }
}

/// Validates `xml` against `schema_file` (a filename inside the patched
/// schema directory, e.g. `soapStoreDelivery.xsd`).
fn validate(schema_dir: &Path, schema_file: &str, xml: &[u8]) {
    let xml_path = schema_dir.join("_candidate.xml");
    fs::write(&xml_path, xml).unwrap();

    let output = Command::new("xmllint")
        .current_dir(schema_dir)
        .env("XML_CATALOG_FILES", schema_dir.join("catalog.xml"))
        .args(["--noout", "--nonet", "--schema", schema_file])
        .arg(&xml_path)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "xmllint rejected the generated {schema_file}:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn store_delivery_validates_against_the_osci_schema() {
    if !xmllint_available() {
        eprintln!("xmllint not installed, skipping XSD validation");
        return;
    }
    let schema_dir = std::env::temp_dir().join(format!(
        "osci-xsd-validation-{}-{}",
        std::process::id(),
        line!()
    ));
    prepare_patched_schema(&schema_dir);

    let cb = ControlBlock {
        id: Some("controlblock".into()),
        challenge: Some("aGVsbG8=".into()),
        sequence_number: Some(0),
        ..Default::default()
    };
    let mut container = ContentContainer::new();
    container.id = Some("contentcontainer0".into());
    container.push_raw(RawContent::new(
        QName::new("http://www.osci.de/2002/04/osci", "XMLContent"),
        b"<greeting>hello</greeting>".to_vec(),
    ));
    let mut content = ContentPackage::default();
    content.containers.push(container);

    let mut env = RequestEnvelope::new(
        Request::StoreDelivery {
            message_id: "msg1".into(),
            subject: Some("xsd validation test".into()),
        },
        cb,
    )
    .with_content(content);
    env.desired_languages = Some(DesiredLanguages {
        id: Some("desiredlanguages".into()),
        languages: vec!["de".into()],
    });
    env.quality_of_timestamps = vec![
        QualityOfTimestamp::plain("creation", "qot_creation"),
        QualityOfTimestamp::plain("reception", "qot_reception"),
    ];
    let unsigned = env.encode().unwrap();

    let sig_pem = key("test_osci-manager_cypher.key.pem");
    let sig_cert = key("test_osci-manager_cypher.cer");
    let cipher_cert = key("Reader01.cer");

    let signed = sign_envelope_with(
        &unsigned,
        Role::Client,
        &sig_pem,
        &sig_cert,
        &[
            CertSlot {
                kind: RoleKind::Addressee,
                is_signature: false,
                cert_der: cipher_cert.clone(),
            },
            CertSlot {
                kind: RoleKind::Reader,
                is_signature: false,
                cert_der: cipher_cert,
            },
        ],
    )
    .unwrap();

    validate(&schema_dir, "soapStoreDelivery.xsd", &signed);

    fs::remove_dir_all(&schema_dir).ok();
}
