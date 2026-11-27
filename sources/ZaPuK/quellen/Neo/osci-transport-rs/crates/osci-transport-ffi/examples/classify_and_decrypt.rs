// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

// The FFI surface as an app would call it: classify an OSCI message, then open
// transport encryption with the recipient key.
// Run with: cargo run -p osci-transport-ffi --example classify_and_decrypt

use std::fs;
use std::path::PathBuf;

use osci_transport::mime::main_xml_part;
use osci_transport_ffi::{MessageKind, classify_message, decrypt_transport_message};

fn interop(p: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/interop")
        .join(p)
}

fn main() {
    let plain = main_xml_part(&fs::read(interop("corpus/FetchProcessCard.osci")).unwrap()).unwrap();
    match classify_message(plain).unwrap() {
        MessageKind::Message { message_type } => println!("plain message: {message_type}"),
        other => println!("plain message: {other:?}"),
    }

    let encrypted =
        main_xml_part(&fs::read(interop("corpus/StrDel_Encrypted_cbc.osci")).unwrap()).unwrap();
    println!(
        "encrypted message: {:?}",
        classify_message(encrypted).unwrap()
    );

    let raw = fs::read(interop("corpus/StrDel_Encrypted_cbc.osci")).unwrap();
    let key_pem = fs::read_to_string(interop("keys/test_osci-manager_cypher.key.pem")).unwrap();
    let cert_der = fs::read(interop("keys/test_osci-manager_cypher.cer")).unwrap();

    let inner = decrypt_transport_message(raw, key_pem, cert_der).unwrap();
    let inner_xml = main_xml_part(&inner).unwrap();
    println!(
        "decrypted inner: {:?}",
        classify_message(inner_xml).unwrap()
    );
}
