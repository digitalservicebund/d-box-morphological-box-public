// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::error::{Error, Result};
use zeroize::Zeroizing;

/// Algorithm URIs transcribed from `de.osci.osci12.common.Constants`. Kept
/// verbatim, including the non-standard 2007/05 PSS URIs, so signatures
/// interoperate with the Java osci-bibliothek.
pub mod uri {
    pub const DIGEST_SHA1: &str = "http://www.w3.org/2000/09/xmldsig#sha1";
    pub const DIGEST_SHA256: &str = "http://www.w3.org/2001/04/xmlenc#sha256";
    pub const DIGEST_SHA512: &str = "http://www.w3.org/2001/04/xmlenc#sha512";

    pub const SIG_RSA_SHA1: &str = "http://www.w3.org/2000/09/xmldsig#rsa-sha1";
    pub const SIG_RSA_SHA256: &str = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha256";
    pub const SIG_RSA_SHA512: &str = "http://www.w3.org/2001/04/xmldsig-more#rsa-sha512";
    pub const SIG_RSA_SHA256_PSS: &str = "http://www.w3.org/2007/05/xmldsig-more#sha256-rsa-MGF1";
    pub const SIG_RSA_SHA512_PSS: &str = "http://www.w3.org/2007/05/xmldsig-more#sha512-rsa-MGF1";
    pub const SIG_ECDSA_SHA256: &str = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha256";
    pub const SIG_ECDSA_SHA512: &str = "http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha512";

    pub const KEY_RSA_OAEP: &str = "http://www.w3.org/2009/xmlenc11#rsa-oaep";
    pub const KEY_RSA_1_5: &str = "http://www.w3.org/2001/04/xmlenc#rsa-1_5";

    pub const AES128_GCM: &str = "http://www.w3.org/2009/xmlenc11#aes128-gcm";
    pub const AES192_GCM: &str = "http://www.w3.org/2009/xmlenc11#aes192-gcm";
    pub const AES256_GCM: &str = "http://www.w3.org/2009/xmlenc11#aes256-gcm";
    pub const AES128_CBC: &str = "http://www.w3.org/2001/04/xmlenc#aes128-cbc";
    pub const AES256_CBC: &str = "http://www.w3.org/2001/04/xmlenc#aes256-cbc";
}

/// A signature algorithm. The `Pss` variants use the non-standard 2007/05 URIs
/// the Java lib emits.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SignatureAlg {
    RsaSha1,
    RsaSha256,
    RsaSha512,
    RsaSha256Pss,
    RsaSha512Pss,
    EcdsaSha256,
    EcdsaSha512,
}

/// A key-transport (key-wrapping) algorithm.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyTransportAlg {
    RsaOaep,
    Rsa15,
}

/// A symmetric bulk-encryption algorithm.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SymmetricAlg {
    Aes128Gcm,
    Aes192Gcm,
    Aes256Gcm,
    Aes128Cbc,
    Aes256Cbc,
}

macro_rules! uri_enum {
    ($t:ty { $($variant:ident => $const:ident),+ $(,)? }) => {
        impl $t {
            /// The XML algorithm URI for this variant.
            pub fn uri(self) -> &'static str {
                match self { $(<$t>::$variant => uri::$const),+ }
            }
            /// Look up a variant by its XML algorithm URI.
            pub fn from_uri(s: &str) -> Result<Self> {
                match s { $(uri::$const => Ok(<$t>::$variant),)+
                    other => Err(Error::msg(format!("unsupported algorithm uri: {other}"))) }
            }
        }
    };
}

uri_enum!(SignatureAlg {
    RsaSha1 => SIG_RSA_SHA1,
    RsaSha256 => SIG_RSA_SHA256,
    RsaSha512 => SIG_RSA_SHA512,
    RsaSha256Pss => SIG_RSA_SHA256_PSS,
    RsaSha512Pss => SIG_RSA_SHA512_PSS,
    EcdsaSha256 => SIG_ECDSA_SHA256,
    EcdsaSha512 => SIG_ECDSA_SHA512,
});
uri_enum!(KeyTransportAlg { RsaOaep => KEY_RSA_OAEP, Rsa15 => KEY_RSA_1_5 });
uri_enum!(SymmetricAlg {
    Aes128Gcm => AES128_GCM,
    Aes192Gcm => AES192_GCM,
    Aes256Gcm => AES256_GCM,
    Aes128Cbc => AES128_CBC,
    Aes256Cbc => AES256_CBC,
});

/// The key-transport algorithm OSCI 1.2 defaults to.
pub const DEFAULT_KEY_TRANSPORT: KeyTransportAlg = KeyTransportAlg::RsaOaep;
/// The symmetric algorithm OSCI 1.2 defaults to.
pub const DEFAULT_SYMMETRIC: SymmetricAlg = SymmetricAlg::Aes256Gcm;

/// Unwraps a content or transport key with a private key.
pub trait Decrypter {
    /// The decrypter's certificate, DER-encoded, if it can name one.
    fn certificate_der(&self) -> Option<&[u8]>;
    /// Unwrap a key that was wrapped with `alg`. Zeroized on drop.
    fn unwrap_key(&self, wrapped: &[u8], alg: KeyTransportAlg) -> Result<Zeroizing<Vec<u8>>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_uri_roundtrip() {
        for a in [
            SignatureAlg::RsaSha1,
            SignatureAlg::RsaSha256,
            SignatureAlg::RsaSha512,
            SignatureAlg::RsaSha256Pss,
            SignatureAlg::RsaSha512Pss,
            SignatureAlg::EcdsaSha256,
            SignatureAlg::EcdsaSha512,
        ] {
            assert_eq!(SignatureAlg::from_uri(a.uri()).unwrap(), a);
        }
    }

    #[test]
    fn default_pss_uri_is_the_nonstandard_one() {
        assert_eq!(
            SignatureAlg::RsaSha256Pss.uri(),
            "http://www.w3.org/2007/05/xmldsig-more#sha256-rsa-MGF1"
        );
    }

    #[test]
    fn unknown_uri_errors() {
        assert!(SignatureAlg::from_uri("urn:nope").is_err());
    }
}
