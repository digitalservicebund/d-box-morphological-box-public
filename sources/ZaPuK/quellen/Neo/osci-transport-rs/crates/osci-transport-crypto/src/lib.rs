// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! Transport- and content-level crypto for OSCI-Transport 1.2.
//!
//! RustCrypto primitives plus `bergshamra` for XML-DSig. The output matches the
//! Java osci-bibliothek byte for byte where it must (inclusive C14N, namespace
//! and attribute ordering).

/// Encrypting and decrypting attachments.
pub mod attachment;
/// Content-level decryption (`xenc:EncryptedData` holding a ContentContainer).
pub mod content;
/// [`RsaDecrypter`], an [`osci_transport::crypto::Decrypter`] over an RSA key.
pub mod decrypt;
/// Transport- and content-level encryption.
pub mod encrypt;
/// Generating a cipher key and its certificate request.
pub mod keygen;
/// Selecting the right `xenc:EncryptedKey` by its `KeyInfo`.
pub mod select;
/// Adding a transport signature to an envelope.
pub mod sign;
/// Symmetric encryption and decryption (AES-GCM, AES-CBC).
pub mod symmetric;
/// Transport-level decryption (the `soapMessageEncrypted` form).
pub mod transport;
/// Verifying a transport signature.
pub mod verify;

pub use attachment::{
    EncryptedAttachment, decrypt_attachment, encrypt_attachment, has_attachment_key,
};
pub use content::decrypt_content;
pub use decrypt::RsaDecrypter;
pub use encrypt::{
    EncryptOptions, encrypt_content, encrypt_content_for_slot, encrypt_content_with,
    encrypt_transport, encrypt_transport_with,
};
pub use keygen::{CipherKey, generate_cipher_key};
pub use select::Recipient;
pub use sign::{
    CertSlot, Role, sign_envelope, sign_envelope_with, sign_envelope_with_alg,
    sign_envelope_with_attachments,
};
pub use transport::decrypt_transport;
pub use verify::{SignatureReport, verify_content_signature, verify_transport_signature};
