// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use aes::{Aes128, Aes256};
use aes_gcm::aead::Aead;
use aes_gcm::aead::consts::U16;
use aes_gcm::{AesGcm, KeyInit, Nonce};

// The official Java osci-bibliothek writes a 16-byte GCM nonce/IV, not the
// RFC 96-bit (12-byte) one the algorithm URI implies, regardless of the
// `Support96Bit12ByteIV` FeatureDescription flag.
type Aes128Gcm = AesGcm<Aes128, U16>;
type Aes256Gcm = AesGcm<Aes256, U16>;
use cbc::cipher::block_padding::{NoPadding, Pkcs7};
use cbc::cipher::{
    BlockCipherDecrypt, BlockCipherEncrypt, BlockModeDecrypt, BlockModeEncrypt, KeyIvInit,
};
use cbc::{Decryptor, Encryptor};
use osci_transport::crypto::SymmetricAlg;
use osci_transport::error::{Error, Result};

/// Decrypt `data` with `alg`. XML-Enc prepends the IV to the ciphertext; GCM
/// uses a 16-byte IV (matching the official Java osci-bibliothek, not the RFC 96-bit
/// default) and appends a 16-byte tag.
pub fn decrypt(alg: SymmetricAlg, key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    match alg {
        SymmetricAlg::Aes128Cbc => cbc_decrypt::<Aes128>(key, data, 16),
        SymmetricAlg::Aes256Cbc => cbc_decrypt::<Aes256>(key, data, 16),
        SymmetricAlg::Aes128Gcm => gcm_decrypt_128(key, data),
        SymmetricAlg::Aes256Gcm => gcm_decrypt_256(key, data),
        SymmetricAlg::Aes192Gcm => Err(Error::msg("aes-192-gcm not supported")),
    }
}

/// Encrypt `plaintext` with `alg` and a fresh random IV/nonce. Returns
/// `iv || ciphertext` (CBC, PKCS#7 padded) or `nonce || ciphertext || tag`
/// (GCM), matching what [`decrypt`] expects.
pub fn encrypt(alg: SymmetricAlg, key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    match alg {
        SymmetricAlg::Aes128Cbc => cbc_encrypt::<Aes128>(key, plaintext),
        SymmetricAlg::Aes256Cbc => cbc_encrypt::<Aes256>(key, plaintext),
        SymmetricAlg::Aes128Gcm => gcm_encrypt_128(key, plaintext),
        SymmetricAlg::Aes256Gcm => gcm_encrypt_256(key, plaintext),
        SymmetricAlg::Aes192Gcm => Err(Error::msg("aes-192-gcm not supported")),
    }
}

fn cbc_decrypt<C>(key: &[u8], data: &[u8], iv_len: usize) -> Result<Vec<u8>>
where
    C: BlockCipherDecrypt + KeyInit,
{
    if data.len() < iv_len {
        return Err(Error::msg("cbc: ciphertext shorter than IV"));
    }
    let (iv, ct) = data.split_at(iv_len);
    if ct.is_empty() || ct.len() % 16 != 0 {
        return Err(Error::msg("cbc: ciphertext not block aligned"));
    }
    let dec = Decryptor::<C>::new_from_slices(key, iv)
        .map_err(|_| Error::msg("cbc: bad key/iv length"))?;
    let mut buf = ct.to_vec();
    let out = dec
        .decrypt_padded::<NoPadding>(&mut buf)
        .map_err(|_| Error::msg("cbc: decrypt failed"))?;
    Ok(strip_pkcs7(out).to_vec())
}

fn cbc_encrypt<C>(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>>
where
    C: BlockCipherEncrypt + KeyInit,
{
    let mut iv = [0u8; 16];
    getrandom::fill(&mut iv).map_err(|e| Error::msg(format!("rng: {e}")))?;
    let enc = Encryptor::<C>::new_from_slices(key, &iv)
        .map_err(|_| Error::msg("cbc: bad key/iv length"))?;
    let ciphertext = enc.encrypt_padded_vec::<Pkcs7>(plaintext);
    let mut out = iv.to_vec();
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

// The official Java osci-bibliothek stops reading at a bad final PKCS#5 block instead of
// failing, and some sample messages carry trailing bytes past the padded
// plaintext. Strip a valid PKCS#7 pad if present, otherwise return as-is and let
// the MIME parser stop at the closing boundary.
fn strip_pkcs7(data: &[u8]) -> &[u8] {
    let Some(&n) = data.last() else { return data };
    let n = n as usize;
    if (1..=16).contains(&n)
        && n <= data.len()
        && data[data.len() - n..].iter().all(|&b| b as usize == n)
    {
        &data[..data.len() - n]
    } else {
        data
    }
}

fn gcm_decrypt_256(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| Error::msg("gcm: bad key length"))?;
    gcm(data, |n, ct| cipher.decrypt(n, ct))
}

fn gcm_decrypt_128(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes128Gcm::new_from_slice(key).map_err(|_| Error::msg("gcm: bad key length"))?;
    gcm(data, |n, ct| cipher.decrypt(n, ct))
}

fn gcm<F>(data: &[u8], f: F) -> Result<Vec<u8>>
where
    F: FnOnce(&Nonce<U16>, &[u8]) -> aes_gcm::aead::Result<Vec<u8>>,
{
    if data.len() < 16 {
        return Err(Error::msg("gcm: ciphertext shorter than nonce"));
    }
    let (iv, ct) = data.split_at(16);
    let nonce = Nonce::<U16>::try_from(iv).map_err(|_| Error::msg("gcm: bad nonce length"))?;
    f(&nonce, ct).map_err(|_| Error::msg("gcm: authentication failed"))
}

fn gcm_encrypt_256(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| Error::msg("gcm: bad key length"))?;
    gcm_seal(plaintext, |n, pt| cipher.encrypt(n, pt))
}

fn gcm_encrypt_128(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes128Gcm::new_from_slice(key).map_err(|_| Error::msg("gcm: bad key length"))?;
    gcm_seal(plaintext, |n, pt| cipher.encrypt(n, pt))
}

fn gcm_seal<F>(plaintext: &[u8], f: F) -> Result<Vec<u8>>
where
    F: FnOnce(&Nonce<U16>, &[u8]) -> aes_gcm::aead::Result<Vec<u8>>,
{
    let mut nonce_bytes = [0u8; 16];
    getrandom::fill(&mut nonce_bytes).map_err(|e| Error::msg(format!("rng: {e}")))?;
    let nonce = Nonce::<U16>::from(nonce_bytes);
    let sealed = f(&nonce, plaintext).map_err(|_| Error::msg("gcm: encrypt failed"))?;
    let mut out = nonce_bytes.to_vec();
    out.extend_from_slice(&sealed);
    Ok(out)
}
