// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use osci_transport::crypto::SymmetricAlg;
use osci_transport::error::{Error, Result};
use osci_transport::mime::Attachment;
use osci_transport::parts::ContentContainer;
use osci_transport::xenc::EncryptedData;
use zeroize::Zeroizing;

use crate::encrypt::key_of_size;
use crate::symmetric;

/// An attachment after [`encrypt_attachment`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncryptedAttachment {
    /// The `xenc:EncryptedData` element for the container: it names the part by
    /// `CipherReference` `cid:` and holds the part's key in `ds:MgmtData`. Push
    /// it onto [`ContentContainer::encrypted`] and encrypt that container for
    /// the reader, or the key travels in the clear.
    pub encrypted_data: Vec<u8>,
    /// The MIME part to send: the same `Content-ID` and `Content-Type` as the
    /// plain attachment, with the ciphertext as its data.
    pub part: Attachment,
}

/// Encrypt one attachment with a fresh key, the way the Java osci-bibliothek
/// does: the part holds `iv || ciphertext`, and the key goes into an
/// `xenc:EncryptedData` that belongs inside an encrypted container.
pub fn encrypt_attachment(plain: &Attachment, alg: SymmetricAlg) -> Result<EncryptedAttachment> {
    let id = plain
        .content_id
        .as_deref()
        .filter(|id| !id.is_empty())
        .ok_or_else(|| Error::msg("attachment has no Content-ID"))?;
    let mut key = key_of_size(alg)?;
    getrandom::fill(&mut key).map_err(|e| Error::msg(format!("rng: {e}")))?;
    let ciphertext = symmetric::encrypt(alg, &key, &plain.data)?;

    let id_attr = xml_attr(id);
    let encrypted_data = format!(
        concat!(
            r#"<xenc:EncryptedData xmlns:ds="http://www.w3.org/2000/09/xmldsig#" "#,
            r#"xmlns:xenc="http://www.w3.org/2001/04/xmlenc#" Id="Attachment{id}" MimeType="text/xml">"#,
            r#"<xenc:EncryptionMethod Algorithm="{alg}"></xenc:EncryptionMethod>"#,
            r#"<ds:KeyInfo><ds:MgmtData>{key}</ds:MgmtData></ds:KeyInfo>"#,
            r#"<xenc:CipherData><xenc:CipherReference URI="cid:{id}"><xenc:Transforms>"#,
            r#"<ds:Transform Algorithm="http://www.w3.org/2000/09/xmldsig#base64"></ds:Transform>"#,
            r#"</xenc:Transforms></xenc:CipherReference></xenc:CipherData></xenc:EncryptedData>"#,
        ),
        id = id_attr,
        alg = alg.uri(),
        key = B64.encode(&key),
    )
    .into_bytes();

    Ok(EncryptedAttachment {
        encrypted_data,
        part: Attachment {
            content_id: plain.content_id.clone(),
            content_type: plain.content_type.clone(),
            data: ciphertext,
        },
    })
}

/// Whether `container` holds a key for `part`, that is, whether the part was
/// sent encrypted. Nested containers are searched too.
pub fn has_attachment_key(container: &ContentContainer, part: &Attachment) -> bool {
    part.content_id
        .as_deref()
        .is_some_and(|id| find_encrypted_data(container, id).is_some())
}

/// Decrypt a received attachment. `container` is the decrypted container, see
/// [`crate::decrypt_content`]; its `EncryptedData` element for the part's
/// `Content-ID` holds the key. Nested containers are searched too.
pub fn decrypt_attachment(container: &ContentContainer, part: &Attachment) -> Result<Vec<u8>> {
    let id = part
        .content_id
        .as_deref()
        .ok_or_else(|| Error::msg("attachment has no Content-ID"))?;
    let ed = find_encrypted_data(container, id)
        .ok_or_else(|| Error::msg(format!("no EncryptedData for cid:{id} in the container")))?;
    let key = ed
        .mgmt_data
        .as_deref()
        .ok_or_else(|| Error::msg(format!("EncryptedData for cid:{id} has no key")))?;
    let key = Zeroizing::new(
        B64.decode(key)
            .map_err(|e| Error::msg(format!("attachment key: {e}")))?,
    );
    symmetric::decrypt(ed.algorithm, &key, &part.data)
}

fn find_encrypted_data(c: &ContentContainer, id: &str) -> Option<EncryptedData> {
    c.encrypted
        .iter()
        .filter_map(|raw| EncryptedData::parse(raw).ok())
        .find(|ed| {
            ed.cipher_reference
                .as_deref()
                .is_some_and(|r| r.strip_prefix("cid:").unwrap_or(r) == id)
        })
        .or_else(|| {
            c.contents.iter().find_map(|content| match &content.kind {
                osci_transport::parts::ContentKind::Nested(inner) => find_encrypted_data(inner, id),
                _ => None,
            })
        })
}

fn xml_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
