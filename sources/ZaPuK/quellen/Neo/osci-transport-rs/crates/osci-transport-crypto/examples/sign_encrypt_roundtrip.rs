// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

// The full client-side transport crypto path and back:
//   build -> sign as Client -> encrypt for the Intermediaer
//   -> decrypt -> verify the ClientSignature.
// Run with: cargo run -p osci-transport-crypto --example sign_encrypt_roundtrip

use std::fs;
use std::path::PathBuf;

use osci_transport::dialog::ControlBlock;
use osci_transport::wire::{Request, RequestEnvelope};
use osci_transport_crypto::{
    Role, RsaDecrypter, decrypt_transport, sign_envelope, verify_transport_signature,
};
use pkcs8::DecodePrivateKey;
use rsa::PrivateKey;

fn fixture(name: &str) -> Vec<u8> {
    fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/interop/keys")
            .join(name),
    )
    .unwrap()
}

fn main() {
    // one keypair stands in for both the signer and the Intermediaer here
    let cert_der = fixture("test_osci-manager_cypher.cer");
    let key_pem = fixture("test_osci-manager_cypher.key.pem");

    let cb = ControlBlock {
        id: Some("controlblock".into()),
        challenge: Some("aGVsbG8=".into()),
        sequence_number: Some(0),
        ..Default::default()
    };
    let unsigned = RequestEnvelope::new(Request::FetchDelivery { message_id: None }, cb)
        .encode()
        .unwrap();
    println!("unsigned envelope: {} bytes", unsigned.len());

    let signed = sign_envelope(&unsigned, Role::Client, &key_pem, &cert_der).unwrap();
    println!("signed envelope:   {} bytes", signed.len());

    let encrypted = osci_transport_crypto::encrypt_transport(&signed, &cert_der).unwrap();
    println!("encrypted MIME:    {} bytes", encrypted.len());

    let dec = RsaDecrypter::new(
        PrivateKey::from_pkcs8_pem(&String::from_utf8(key_pem).unwrap()).unwrap(),
    )
    .with_certificate(cert_der);
    let inner = decrypt_transport(&encrypted, &dec).unwrap();
    println!("decrypted back to: {} bytes", inner.len());
    assert_eq!(inner, signed);

    let report = verify_transport_signature(&inner, |_| true).unwrap();
    println!(
        "signature valid: {}  refs: {:?}",
        report.valid, report.verified_refs
    );
    assert!(report.valid);
}
