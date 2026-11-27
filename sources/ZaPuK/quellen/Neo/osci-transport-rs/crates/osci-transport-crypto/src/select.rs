// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use osci_transport::error::{Error, Result};
use osci_transport::xenc::EncryptedKey;

/// Identifies an `EncryptedKey` recipient.
#[derive(Clone, Copy, Debug)]
pub enum Recipient<'a> {
    /// DER certificate, matched against an inline `ds:X509Certificate`.
    Certificate(&'a [u8]),
    /// Certificate-slot Id, matched against the `ds:RetrievalMethod` URI
    /// fragment.
    CertificateId(&'a str),
}

/// Return the single `EncryptedKey` whose `KeyInfo` matches `recipient`. Errors
/// if none or more than one matches; never trial-decrypts.
pub fn select_key<'k>(
    keys: &'k [EncryptedKey],
    recipient: Recipient<'_>,
) -> Result<&'k EncryptedKey> {
    let matches: Vec<&EncryptedKey> = keys.iter().filter(|k| key_matches(k, recipient)).collect();
    match matches.as_slice() {
        [k] => Ok(k),
        [] => Err(Error::msg("no EncryptedKey is addressed to this recipient")),
        _ => Err(Error::msg(
            "more than one EncryptedKey matches this recipient",
        )),
    }
}

fn key_matches(key: &EncryptedKey, recipient: Recipient<'_>) -> bool {
    match recipient {
        Recipient::Certificate(der) => key.recipient_cert_der.as_deref() == Some(der),
        Recipient::CertificateId(id) => {
            // Java compares with String.endsWith on the URI; the "#" prefix is ignored.
            let id = id.strip_prefix('#').unwrap_or(id);
            key.retrieval_uri
                .as_deref()
                .map(|u| u.strip_prefix('#').unwrap_or(u))
                .is_some_and(|u| u.ends_with(id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use osci_transport::crypto::KeyTransportAlg;

    fn key(cert: Option<&[u8]>, uri: Option<&str>) -> EncryptedKey {
        EncryptedKey {
            algorithm: KeyTransportAlg::Rsa15,
            recipient_cert_der: cert.map(<[u8]>::to_vec),
            retrieval_uri: uri.map(str::to_owned),
            cipher_value: vec![1, 2, 3],
        }
    }

    #[test]
    fn matches_by_certificate() {
        let keys = [key(Some(b"AAA"), None), key(Some(b"BBB"), None)];
        assert_eq!(
            select_key(&keys, Recipient::Certificate(b"BBB"))
                .unwrap()
                .cipher_value,
            [1, 2, 3]
        );
        assert!(select_key(&keys, Recipient::Certificate(b"CCC")).is_err());
    }

    #[test]
    fn matches_by_id() {
        let keys = [
            key(None, Some("#reader0_cipher_abc")),
            key(None, Some("#reader1_cipher_def")),
        ];
        assert!(select_key(&keys, Recipient::CertificateId("reader1_cipher_def")).is_ok());
        assert!(select_key(&keys, Recipient::CertificateId("nope")).is_err());
    }

    #[test]
    fn rejects_ambiguous() {
        let keys = [key(Some(b"AAA"), None), key(Some(b"AAA"), None)];
        assert!(select_key(&keys, Recipient::Certificate(b"AAA")).is_err());
    }
}
