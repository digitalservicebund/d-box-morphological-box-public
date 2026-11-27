// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! Generating a cipher key and the certificate request for it.

use std::str::FromStr;

use getrandom::SysRng;
use osci_transport::error::{Error, Result};
use rsa::PrivateKey;
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
use rsa::rand_core::UnwrapErr;
use rsa::signature::{SignatureEncoding, Signer};
use sha2::Sha256;
use x509_cert::der::asn1::{Any, BitString, ObjectIdentifier};
use x509_cert::der::{Decode, Encode};
use x509_cert::name::Name;
use x509_cert::request::{CertReq, CertReqInfo, Version};
use x509_cert::spki::{AlgorithmIdentifierOwned, SubjectPublicKeyInfoOwned};
use zeroize::Zeroizing;

/// The smallest RSA key size [`generate_cipher_key`] accepts.
pub const MIN_KEY_BITS: usize = 2048;
/// The largest RSA key size [`generate_cipher_key`] accepts. Generating bigger
/// keys takes minutes in a browser.
pub const MAX_KEY_BITS: usize = 4096;

const SHA256_WITH_RSA: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.11");

/// A new RSA key and the certificate request for its public key.
pub struct CipherKey {
    /// The private key as PKCS#8 PEM.
    pub private_key_pem: Zeroizing<String>,
    /// The PKCS#10 certificate request, DER encoded and signed with the key.
    pub csr_der: Vec<u8>,
    /// The public key as DER encoded `SubjectPublicKeyInfo`.
    pub public_key_spki_der: Vec<u8>,
}

/// Generate an RSA key of `bits` bits and a certificate request for it.
/// `subject` is a distinguished name such as `CN=Court,O=Example`. The request
/// is signed with RSA PKCS#1 v1.5 and SHA-256.
pub fn generate_cipher_key(bits: usize, subject: &str) -> Result<CipherKey> {
    if !(MIN_KEY_BITS..=MAX_KEY_BITS).contains(&bits) {
        return Err(Error::msg(format!(
            "key size {bits} is outside {MIN_KEY_BITS}..={MAX_KEY_BITS}"
        )));
    }
    let subject = Name::from_str(subject).map_err(|e| Error::msg(format!("subject: {e}")))?;
    let key = PrivateKey::new(&mut UnwrapErr(SysRng), bits)
        .map_err(|e| Error::msg(format!("key generation: {e}")))?;

    let public_key_spki_der = key
        .to_public_key()
        .to_public_key_der()
        .map_err(|e| Error::msg(format!("public key: {e}")))?
        .into_vec();
    let info = CertReqInfo {
        version: Version::V1,
        subject,
        public_key: SubjectPublicKeyInfoOwned::from_der(&public_key_spki_der)
            .map_err(|e| Error::msg(format!("public key: {e}")))?,
        attributes: Default::default(),
    };
    let signature = SigningKey::<Sha256>::new(key.clone())
        .try_sign(&info.to_der().map_err(|e| Error::msg(format!("csr: {e}")))?)
        .map_err(|e| Error::msg(format!("csr signature: {e}")))?;
    let csr = CertReq {
        info,
        algorithm: AlgorithmIdentifierOwned {
            oid: SHA256_WITH_RSA,
            parameters: Some(Any::null()),
        },
        signature: BitString::from_bytes(&signature.to_bytes())
            .map_err(|e| Error::msg(format!("csr signature: {e}")))?,
    };
    Ok(CipherKey {
        private_key_pem: key
            .to_pkcs8_pem(LineEnding::LF)
            .map_err(|e| Error::msg(format!("private key: {e}")))?,
        csr_der: csr.to_der().map_err(|e| Error::msg(format!("csr: {e}")))?,
        public_key_spki_der,
    })
}
