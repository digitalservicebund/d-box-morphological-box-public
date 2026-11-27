// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use osci_transport::crypto::Decrypter;
use osci_transport::error::{Error, Result};
use osci_transport::mime::{attachments, main_xml_part};
use osci_transport::xenc::EncryptedData;

use crate::select::{Recipient, select_key};
use crate::symmetric;

/// Decrypt a `soapMessageEncrypted` message and return the inner MIME message.
/// The `EncryptedKey` is chosen by matching `dec`'s certificate against the
/// `KeyInfo` of each key, never by trial decryption.
pub fn decrypt_transport(message: &[u8], dec: &dyn Decrypter) -> Result<Vec<u8>> {
    let envelope = main_xml_part(message)?;
    let ed = EncryptedData::parse(&envelope)?;

    let my_cert = dec
        .certificate_der()
        .ok_or_else(|| Error::msg("decrypter has no certificate to select the EncryptedKey"))?;
    let key_entry = select_key(&ed.keys, Recipient::Certificate(my_cert))?;
    let cek = crate::decrypt::unwrap_content_key(
        dec,
        &key_entry.cipher_value,
        key_entry.algorithm,
        ed.algorithm,
    )?;

    let ciphertext = match (&ed.cipher_value, &ed.cipher_reference) {
        (Some(v), _) => v.clone(),
        (None, Some(uri)) => {
            let cid = uri.strip_prefix("cid:").unwrap_or(uri);
            let part = attachments(message)?
                .into_iter()
                .find(|a| a.content_id.as_deref() == Some(cid))
                .ok_or_else(|| Error::msg(format!("cipher part {cid} not found")))?;
            // CipherReference always carries a base64 ds:Transform in the
            // XML, but the official Java osci-bibliothek only actually base64-encodes the
            // part body when its Content-Type is text/base64; a raw-binary
            // part (application/octet-stream, the DisableBase64-era default)
            // holds the ciphertext directly. Branch on the MIME part's own
            // Content-Type rather than trusting the Transform label.
            if part
                .content_type
                .as_deref()
                .is_some_and(|ct| ct.eq_ignore_ascii_case("text/base64"))
            {
                B64.decode(
                    part.data
                        .iter()
                        .filter(|b| !b.is_ascii_whitespace())
                        .copied()
                        .collect::<Vec<_>>(),
                )
                .map_err(|e| Error::msg(format!("cipher part base64: {e}")))?
            } else {
                part.data
            }
        }
        (None, None) => return Err(Error::msg("EncryptedData has no ciphertext")),
    };

    symmetric::decrypt(ed.algorithm, &cek, &ciphertext)
}
