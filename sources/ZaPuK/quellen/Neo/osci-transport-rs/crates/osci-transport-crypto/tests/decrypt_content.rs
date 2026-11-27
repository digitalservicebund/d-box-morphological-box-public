// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::fs;
use std::path::PathBuf;

use osci_transport::envelope::parse;
use osci_transport::mime::main_xml_part;
use osci_transport_crypto::{Recipient, RsaDecrypter, decrypt_content};
use pkcs8::DecodePrivateKey;
use rsa::PrivateKey;

fn interop_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/interop")
}

fn reader_key() -> PrivateKey {
    let pem = fs::read_to_string(interop_dir().join("keys/Reader01.key.pem")).unwrap();
    PrivateKey::from_pkcs8_pem(&pem).unwrap()
}

#[test]
fn decrypts_store_delivery_content() {
    let raw = fs::read(interop_dir().join("corpus/StoreDelivery.osci")).unwrap();
    let xml = main_xml_part(&raw).unwrap();
    let env = parse(&xml).unwrap();
    let pkg = env.content_package.expect("content package");
    let ed = pkg.encrypted.first().expect("encrypted container");

    let dec = RsaDecrypter::new(reader_key());
    let recipient =
        Recipient::CertificateId("reader0_cipher_a5ed201258618adb054546e503e685a32ab73968");
    let cc = decrypt_content(ed, &dec, recipient).expect("decrypt content");

    assert!(!cc.contents.is_empty());

    // wrong recipient id must not decrypt
    assert!(decrypt_content(ed, &dec, Recipient::CertificateId("nobody")).is_err());
}
