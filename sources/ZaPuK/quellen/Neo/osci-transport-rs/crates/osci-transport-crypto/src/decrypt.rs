// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use osci_transport::crypto::{Decrypter, KeyTransportAlg};
use osci_transport::error::{Error, Result};
use rsa::{Oaep, Pkcs1v15Encrypt, PrivateKey};
use sha1::Sha1;
use sha2::Sha256;
use zeroize::Zeroizing;

/// An [`osci_transport::crypto::Decrypter`] backed by an RSA private key.
pub struct RsaDecrypter {
    key: PrivateKey,
    certificate_der: Option<Vec<u8>>,
}

impl RsaDecrypter {
    /// Wrap a private key. Without a certificate the transport layer cannot
    /// select the matching `EncryptedKey`.
    pub fn new(key: PrivateKey) -> Self {
        RsaDecrypter {
            key,
            certificate_der: None,
        }
    }

    /// Attach the matching certificate (DER), so key selection can work.
    pub fn with_certificate(mut self, der: impl Into<Vec<u8>>) -> Self {
        self.certificate_der = Some(der.into());
        self
    }
}

impl RsaDecrypter {
    /// Whether `cert_der` holds the public half of this decrypter's key.
    pub fn owns_certificate(&self, cert_der: &[u8]) -> bool {
        crate::encrypt::rsa_public_key(cert_der)
            .is_ok_and(|public| public == self.key.to_public_key())
    }
}

impl Decrypter for RsaDecrypter {
    fn certificate_der(&self) -> Option<&[u8]> {
        self.certificate_der.as_deref()
    }

    fn unwrap_key(&self, wrapped: &[u8], alg: KeyTransportAlg) -> Result<Zeroizing<Vec<u8>>> {
        let out = match alg {
            KeyTransportAlg::Rsa15 => self.key.decrypt(Pkcs1v15Encrypt, wrapped),
            // An explicit DigestMethod/MGF on the EncryptionMethod is not read
            // yet: try SHA-256 (OSCI default) then the XML-Enc SHA-1 default.
            KeyTransportAlg::RsaOaep => self
                .key
                .decrypt(Oaep::<Sha256>::new(), wrapped)
                .or_else(|_| self.key.decrypt(Oaep::<Sha1>::new(), wrapped)),
        };
        out.map(Zeroizing::new)
            .map_err(|e| Error::msg(format!("rsa unwrap failed: {e}")))
    }
}

/// Unwrap a content key for `sym`. A failed RSA unwrap, or a key of the wrong
/// size, gives a random key of the right size instead of an error, so a bad
/// padding looks the same as any other wrong key: the later decryption fails
/// with one generic error. An error here would be a Bleichenbacher oracle for
/// RSA 1.5, which the Java library still sends.
pub(crate) fn unwrap_content_key(
    dec: &dyn Decrypter,
    wrapped: &[u8],
    alg: KeyTransportAlg,
    sym: osci_transport::crypto::SymmetricAlg,
) -> Result<Zeroizing<Vec<u8>>> {
    let mut random = crate::encrypt::key_of_size(sym)?;
    match dec.unwrap_key(wrapped, alg) {
        Ok(key) if key.len() == random.len() => Ok(key),
        _ => {
            getrandom::fill(&mut random).map_err(|e| Error::msg(format!("rng: {e}")))?;
            Ok(random)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use osci_transport::crypto::SymmetricAlg;

    struct Fixed(Option<Vec<u8>>);

    impl Decrypter for Fixed {
        fn certificate_der(&self) -> Option<&[u8]> {
            None
        }
        fn unwrap_key(&self, _: &[u8], _: KeyTransportAlg) -> Result<Zeroizing<Vec<u8>>> {
            self.0
                .clone()
                .map(Zeroizing::new)
                .ok_or_else(|| Error::msg("padding"))
        }
    }

    #[test]
    fn a_bad_unwrap_gives_a_key_not_an_error() {
        let alg = SymmetricAlg::Aes128Gcm;
        let bad = [
            Fixed(None),
            Fixed(Some(vec![1; 5])), // padding fine, wrong size
        ];
        for dec in bad {
            let key = unwrap_content_key(&dec, b"x", KeyTransportAlg::Rsa15, alg).unwrap();
            assert_eq!(key.len(), 16);
        }
        let good = Fixed(Some(vec![7; 16]));
        let key = unwrap_content_key(&good, b"x", KeyTransportAlg::Rsa15, alg).unwrap();
        assert_eq!(*key, vec![7; 16]);
    }
}
