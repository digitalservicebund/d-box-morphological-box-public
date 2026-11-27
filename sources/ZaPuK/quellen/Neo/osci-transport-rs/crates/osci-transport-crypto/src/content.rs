// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use osci_transport::crypto::Decrypter;
use osci_transport::error::{Error, Result};
use osci_transport::parts::ContentContainer;
use osci_transport::wire::parse_content_container;
use osci_transport::xenc::EncryptedData;

use crate::select::{Recipient, select_key};
use crate::symmetric;

/// Decrypt a content-level `xenc:EncryptedData` holding an
/// `osci:ContentContainer`. `recipient` selects the `EncryptedKey` by its
/// `KeyInfo` (inline certificate or a `RetrievalMethod` Id), never by trial
/// decryption.
pub fn decrypt_content(
    encrypted_data_xml: &[u8],
    dec: &dyn Decrypter,
    recipient: Recipient<'_>,
) -> Result<ContentContainer> {
    let ed = EncryptedData::parse(encrypted_data_xml)?;
    let ciphertext = ed
        .cipher_value
        .as_deref()
        .ok_or_else(|| Error::msg("content EncryptedData has no inline CipherValue"))?;

    let key = select_key(&ed.keys, recipient)?;
    let cek =
        crate::decrypt::unwrap_content_key(dec, &key.cipher_value, key.algorithm, ed.algorithm)?;
    let plain = symmetric::decrypt(ed.algorithm, &cek, ciphertext)?;
    drop(cek);
    parse_content_container(&plain)
}
