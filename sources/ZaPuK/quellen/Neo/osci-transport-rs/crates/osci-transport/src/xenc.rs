// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use quick_xml::events::Event;
use quick_xml::reader::NsReader;

use crate::crypto::{KeyTransportAlg, SymmetricAlg};
use crate::error::{Error, Result};

/// `xenc:EncryptedKey`: the content-encryption key wrapped for one recipient.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncryptedKey {
    /// The key-transport algorithm the key was wrapped with.
    pub algorithm: KeyTransportAlg,
    /// The recipient certificate, inline in `ds:X509Certificate`.
    pub recipient_cert_der: Option<Vec<u8>>,
    /// A `ds:RetrievalMethod` URI pointing at a cert slot elsewhere.
    pub retrieval_uri: Option<String>,
    /// The wrapped key bytes.
    pub cipher_value: Vec<u8>,
}

/// `xenc:EncryptedData`: one AES-encrypted blob plus the wrapped key(s).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncryptedData {
    /// The `Id` attribute, if any.
    pub id: Option<String>,
    /// The `MimeType` attribute, if any.
    pub mime_type: Option<String>,
    /// The symmetric algorithm the blob was encrypted with.
    pub algorithm: SymmetricAlg,
    /// The wrapped keys, one per recipient.
    pub keys: Vec<EncryptedKey>,
    /// Inline ciphertext, base64-decoded.
    pub cipher_value: Option<Vec<u8>>,
    /// A reference to a MIME part instead of inline ciphertext (`cid:...`).
    pub cipher_reference: Option<String>,
    /// `ds:MgmtData` of the data's own `KeyInfo`: for an encrypted attachment,
    /// the base64 text of its symmetric key.
    pub mgmt_data: Option<String>,
}

impl EncryptedData {
    /// Parse an `xenc:EncryptedData` fragment, including its nested
    /// `EncryptedKey` elements.
    pub fn parse(xml: &[u8]) -> Result<Self> {
        let mut r = NsReader::from_reader(xml);
        let mut ed = EncryptedData {
            id: None,
            mime_type: None,
            algorithm: SymmetricAlg::Aes256Gcm,
            keys: Vec::new(),
            cipher_value: None,
            cipher_reference: None,
            mgmt_data: None,
        };
        let mut cur_key: Option<EncryptedKey> = None;
        // where the reader currently is: how many EncryptedKey we are inside
        let mut in_key = 0u32;
        let mut text_target: Option<Target> = None;
        let mut buf = String::new();

        loop {
            match r.read_event()? {
                Event::Start(e) | Event::Empty(e) => {
                    let local = e.local_name().as_ref().to_string();
                    match local.as_str() {
                        "EncryptedData" => {
                            for a in e.attributes() {
                                let a = a?;
                                match a.key.as_ref() {
                                    "Id" => {
                                        ed.id = Some(
                                            a.normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                                                .into_owned(),
                                        )
                                    }
                                    "MimeType" => {
                                        ed.mime_type = Some(
                                            a.normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                                                .into_owned(),
                                        )
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "EncryptionMethod" => {
                            let alg =
                                attr(&e, "Algorithm")?.ok_or(Error::MissingAttr("Algorithm"))?;
                            if in_key > 0 {
                                if let Some(k) = cur_key.as_mut() {
                                    k.algorithm = KeyTransportAlg::from_uri(&alg)?;
                                }
                            } else {
                                ed.algorithm = SymmetricAlg::from_uri(&alg)?;
                            }
                        }
                        "EncryptedKey" => {
                            in_key += 1;
                            cur_key = Some(EncryptedKey {
                                algorithm: KeyTransportAlg::RsaOaep,
                                recipient_cert_der: None,
                                retrieval_uri: None,
                                cipher_value: Vec::new(),
                            });
                        }
                        "RetrievalMethod" => {
                            if let (Some(k), Some(uri)) = (cur_key.as_mut(), attr(&e, "URI")?) {
                                k.retrieval_uri = Some(uri);
                            }
                        }
                        "X509Certificate" if in_key > 0 => {
                            text_target = Some(Target::RecipientCert);
                            buf.clear();
                        }
                        "MgmtData" if in_key == 0 => {
                            text_target = Some(Target::Mgmt);
                            buf.clear();
                        }
                        "CipherReference" => {
                            ed.cipher_reference = attr(&e, "URI")?;
                        }
                        "CipherValue" => {
                            text_target = Some(if in_key > 0 {
                                Target::KeyCipher
                            } else {
                                Target::DataCipher
                            });
                            buf.clear();
                        }
                        _ => {}
                    }
                }
                Event::Text(t) if text_target.is_some() => {
                    buf.push_str(&t.xml_content(quick_xml::XmlVersion::Explicit1_0))
                }
                Event::CData(t) if text_target.is_some() => buf.push_str(t.as_ref()),
                Event::End(e) => {
                    let local = e.local_name();
                    match (text_target.take(), local.as_ref()) {
                        (Some(Target::RecipientCert), "X509Certificate") => {
                            if let Some(k) = cur_key.as_mut() {
                                k.recipient_cert_der = Some(decode_b64(&buf)?);
                            }
                        }
                        (Some(Target::KeyCipher), "CipherValue") => {
                            if let Some(k) = cur_key.as_mut() {
                                k.cipher_value = decode_b64(&buf)?;
                            }
                        }
                        (Some(Target::Mgmt), "MgmtData") => {
                            ed.mgmt_data = Some(buf.split_whitespace().collect());
                        }
                        (Some(Target::DataCipher), "CipherValue") => {
                            ed.cipher_value = Some(decode_b64(&buf)?);
                        }
                        (other, "EncryptedKey") => {
                            text_target = other;
                            in_key = in_key.saturating_sub(1);
                            if in_key == 0
                                && let Some(k) = cur_key.take()
                            {
                                ed.keys.push(k);
                            }
                        }
                        (other, _) => text_target = other,
                    }
                }
                Event::Eof => break,
                _ => {}
            }
        }
        Ok(ed)
    }
}

#[derive(Clone, Copy)]
enum Target {
    RecipientCert,
    KeyCipher,
    DataCipher,
    Mgmt,
}

fn attr(e: &quick_xml::events::BytesStart<'_>, name: &str) -> Result<Option<String>> {
    for a in e.attributes() {
        let a = a?;
        if a.key.as_ref() == name {
            return Ok(Some(
                a.normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                    .into_owned(),
            ));
        }
    }
    Ok(None)
}

fn decode_b64(s: &str) -> Result<Vec<u8>> {
    B64.decode(s.split_whitespace().collect::<String>())
        .map_err(|e| Error::msg(format!("xenc base64: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = concat!(
        r#"<xenc:EncryptedData xmlns:xenc="http://www.w3.org/2001/04/xmlenc#" "#,
        r#"xmlns:ds="http://www.w3.org/2000/09/xmldsig#" MimeType="Multipart/Related">"#,
        r#"<xenc:EncryptionMethod Algorithm="http://www.w3.org/2001/04/xmlenc#aes256-cbc"/>"#,
        r#"<ds:KeyInfo><xenc:EncryptedKey>"#,
        r#"<xenc:EncryptionMethod Algorithm="http://www.w3.org/2001/04/xmlenc#rsa-1_5"/>"#,
        r#"<ds:KeyInfo><ds:X509Data><ds:X509Certificate>YWJj</ds:X509Certificate></ds:X509Data></ds:KeyInfo>"#,
        r#"<xenc:CipherData><xenc:CipherValue>ZGVm</xenc:CipherValue></xenc:CipherData>"#,
        r#"</xenc:EncryptedKey></ds:KeyInfo>"#,
        r#"<xenc:CipherData><xenc:CipherReference URI="cid:osci_enc">"#,
        r#"<xenc:Transforms><ds:Transform Algorithm="http://www.w3.org/2000/09/xmldsig#base64"/></xenc:Transforms>"#,
        r#"</xenc:CipherReference></xenc:CipherData></xenc:EncryptedData>"#,
    );

    #[test]
    fn parses_encrypted_data() {
        let ed = EncryptedData::parse(SAMPLE.as_bytes()).unwrap();
        assert_eq!(ed.algorithm, SymmetricAlg::Aes256Cbc);
        assert_eq!(ed.mime_type.as_deref(), Some("Multipart/Related"));
        assert_eq!(ed.cipher_reference.as_deref(), Some("cid:osci_enc"));
        assert_eq!(ed.keys.len(), 1);
        assert_eq!(ed.keys[0].algorithm, KeyTransportAlg::Rsa15);
        assert_eq!(
            ed.keys[0].recipient_cert_der.as_deref(),
            Some(b"abc".as_slice())
        );
        assert_eq!(ed.keys[0].cipher_value, b"def");
    }
}
