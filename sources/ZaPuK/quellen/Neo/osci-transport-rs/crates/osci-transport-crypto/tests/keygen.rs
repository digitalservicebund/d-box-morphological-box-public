// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use osci_transport_crypto::generate_cipher_key;
use rsa::pkcs1v15::{Signature, VerifyingKey};
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::signature::Verifier;
use rsa::{PrivateKey, PublicKey};
use sha2::Sha256;
use x509_cert::der::{Decode, Encode};
use x509_cert::request::CertReq;

#[test]
fn key_and_request_belong_together() {
    let k = generate_cipher_key(2048, "CN=Court Mailbox,O=Example").unwrap();

    let private = PrivateKey::from_pkcs8_pem(&k.private_key_pem).unwrap();
    let public = PublicKey::from_public_key_der(&k.public_key_spki_der).unwrap();
    assert_eq!(private.to_public_key(), public);

    let csr = CertReq::from_der(&k.csr_der).unwrap();
    assert_eq!(csr.info.subject.to_string(), "CN=Court Mailbox,O=Example");
    assert_eq!(csr.info.public_key.to_der().unwrap(), k.public_key_spki_der);
    let signature = Signature::try_from(csr.signature.raw_bytes()).unwrap();
    VerifyingKey::<Sha256>::new(public)
        .verify(&csr.info.to_der().unwrap(), &signature)
        .unwrap();
}

#[test]
fn key_size_and_subject_are_checked() {
    assert!(generate_cipher_key(1024, "CN=x").is_err());
    assert!(generate_cipher_key(8192, "CN=x").is_err());
    assert!(generate_cipher_key(2048, "not a name").is_err());
}
