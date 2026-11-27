// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::fs;
use std::path::PathBuf;

use osci_transport::envelope::{Sniff, sniff};
use osci_transport::message::MessageType;
use osci_transport::mime::main_xml_part;
use osci_transport_crypto::{RsaDecrypter, decrypt_transport};
use pkcs8::DecodePrivateKey;
use rsa::PrivateKey;

fn interop_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/interop")
}

fn reader_key() -> PrivateKey {
    let pem = fs::read_to_string(interop_dir().join("keys/test_osci-manager_cypher.key.pem"))
        .expect("key pem");
    PrivateKey::from_pkcs8_pem(&pem).expect("parse key")
}

fn manager_cert() -> Vec<u8> {
    fs::read(interop_dir().join("keys/test_osci-manager_cypher.cer")).unwrap()
}

#[test]
fn decrypts_cbc_store_delivery() {
    let raw = fs::read(interop_dir().join("corpus/StrDel_Encrypted_cbc.osci")).unwrap();
    let dec = RsaDecrypter::new(reader_key()).with_certificate(manager_cert());

    let inner = decrypt_transport(&raw, &dec).expect("decrypt");
    let envelope = main_xml_part(&inner).expect("inner mime");
    assert_eq!(
        sniff(&envelope).unwrap(),
        Sniff::Message(MessageType::StoreDelivery)
    );
}
