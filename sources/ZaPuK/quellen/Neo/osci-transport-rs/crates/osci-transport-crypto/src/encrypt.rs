// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use getrandom::SysRng;
use osci_transport::crypto::{
    DEFAULT_KEY_TRANSPORT, DEFAULT_SYMMETRIC, KeyTransportAlg, SymmetricAlg,
};
use osci_transport::error::{Error, Result};
use rsa::pkcs8::DecodePublicKey;
use rsa::rand_core::UnwrapErr;
use rsa::{Oaep, Pkcs1v15Encrypt, PublicKey};
use sha2::Sha256;
use x509_cert::Certificate;
use x509_cert::der::{Decode, Encode};
use zeroize::Zeroizing;

use crate::symmetric;

const BOUNDARY: &str = "MIME_boundary";

/// Which key-transport and symmetric algorithms [`encrypt_transport`]/
/// [`encrypt_content`] use. Defaults to OSCI 1.2's modern default:
/// AES-256-GCM plus RSA-OAEP-SHA256. [`KeyTransportAlg::Rsa15`] with
/// [`SymmetricAlg::Aes256Cbc`] is the older combination that predates
/// OAEP's explicit MGF/DigestMethod parameters, for deployments that only
/// accept it; see `against_test_intermediary.rs` for its current scope
/// against a real Governikus Intermediär.
#[derive(Clone, Copy, Debug)]
pub struct EncryptOptions {
    /// How the AES key is wrapped for the recipient.
    pub key_transport: KeyTransportAlg,
    /// The bulk symmetric algorithm.
    pub symmetric: SymmetricAlg,
}

impl Default for EncryptOptions {
    fn default() -> Self {
        EncryptOptions {
            key_transport: DEFAULT_KEY_TRANSPORT,
            symmetric: DEFAULT_SYMMETRIC,
        }
    }
}

/// Wrap an OSCI message (MIME entity) in the `soapMessageEncrypted` form for one
/// recipient, using [`EncryptOptions::default`]. The result is a
/// `multipart/related` MIME entity.
pub fn encrypt_transport(inner_message: &[u8], recipient_cert_der: &[u8]) -> Result<Vec<u8>> {
    encrypt_transport_with(
        inner_message,
        recipient_cert_der,
        &EncryptOptions::default(),
    )
}

/// [`encrypt_transport`] with an explicit algorithm choice.
pub fn encrypt_transport_with(
    inner_message: &[u8],
    recipient_cert_der: &[u8],
    opts: &EncryptOptions,
) -> Result<Vec<u8>> {
    let public_key = rsa_public_key(recipient_cert_der)?;

    let mut cek = key_of_size(opts.symmetric)?;
    getrandom::fill(&mut cek).map_err(|e| Error::msg(format!("rng: {e}")))?;
    let ciphertext = symmetric::encrypt(opts.symmetric, &cek, inner_message)?;
    let wrapped = rsa_wrap(opts.key_transport, &public_key, &cek)?;
    drop(cek);

    let envelope = encrypted_envelope(opts, recipient_cert_der, &wrapped);
    Ok(assemble_mime(&envelope, &ciphertext))
}

/// Encrypt an `osci:ContentContainer` for one reader, using
/// [`EncryptOptions::default`]. Returns the `xenc:EncryptedData` element
/// (inline `CipherValue`) to place in a `ContentPackage`.
pub fn encrypt_content(container_xml: &[u8], reader_cert_der: &[u8]) -> Result<Vec<u8>> {
    encrypt_content_with(container_xml, reader_cert_der, &EncryptOptions::default())
}

/// [`encrypt_content`] with an explicit algorithm choice.
pub fn encrypt_content_with(
    container_xml: &[u8],
    reader_cert_der: &[u8],
    opts: &EncryptOptions,
) -> Result<Vec<u8>> {
    let cert = B64.encode(reader_cert_der);
    let key_info =
        format!("<ds:X509Data><ds:X509Certificate>{cert}</ds:X509Certificate></ds:X509Data>");
    content_encrypted_data(container_xml, reader_cert_der, &key_info, "encdata_0", opts)
}

/// Like [`encrypt_content_with`], but the reader is named by a
/// `ds:RetrievalMethod` to a certificate slot in the message header instead
/// of an inline certificate. `slot_id` is the slot's `Id`, see
/// [`crate::CertSlot::id`]. The Governikus test Intermediär refuses the inline
/// form with SOAP fault 9100 and accepts this one, which is also what the
/// Java osci-bibliothek writes. The slot must carry `reader_cert_der`. The
/// element's `Id` names the slot, so a package can hold one block per reader.
pub fn encrypt_content_for_slot(
    container_xml: &[u8],
    reader_cert_der: &[u8],
    slot_id: &str,
    opts: &EncryptOptions,
) -> Result<Vec<u8>> {
    if slot_id.is_empty() || slot_id.contains(['"', '<', '&', '#']) {
        return Err(Error::msg("invalid cert slot id"));
    }
    let key_info = format!(
        r##"<ds:RetrievalMethod Type="http://www.w3.org/2000/09/xmldsig#X509Data" URI="#{slot_id}"></ds:RetrievalMethod>"##
    );
    let id = format!("encdata_{slot_id}");
    content_encrypted_data(container_xml, reader_cert_der, &key_info, &id, opts)
}

// `key_info` is the content of the EncryptedKey's `ds:KeyInfo`.
fn content_encrypted_data(
    container_xml: &[u8],
    reader_cert_der: &[u8],
    key_info: &str,
    id: &str,
    opts: &EncryptOptions,
) -> Result<Vec<u8>> {
    let public_key = rsa_public_key(reader_cert_der)?;

    let mut cek = key_of_size(opts.symmetric)?;
    getrandom::fill(&mut cek).map_err(|e| Error::msg(format!("rng: {e}")))?;
    let ciphertext = symmetric::encrypt(opts.symmetric, &cek, container_xml)?;
    let wrapped = rsa_wrap(opts.key_transport, &public_key, &cek)?;
    drop(cek);

    Ok(format!(
        concat!(
            r#"<xenc:EncryptedData xmlns:ds="http://www.w3.org/2000/09/xmldsig#" "#,
            r#"xmlns:xenc="http://www.w3.org/2001/04/xmlenc#" Id="{id}" MimeType="text/xml">"#,
            r#"<xenc:EncryptionMethod Algorithm="{sym}"></xenc:EncryptionMethod>"#,
            r#"<ds:KeyInfo><xenc:EncryptedKey>"#,
            r#"{key_method}"#,
            r#"<ds:KeyInfo>{key_info}</ds:KeyInfo>"#,
            r#"<xenc:CipherData><xenc:CipherValue>{key}</xenc:CipherValue></xenc:CipherData>"#,
            r#"</xenc:EncryptedKey></ds:KeyInfo>"#,
            r#"<xenc:CipherData><xenc:CipherValue>{data}</xenc:CipherValue></xenc:CipherData>"#,
            r#"</xenc:EncryptedData>"#,
        ),
        id = id,
        sym = opts.symmetric.uri(),
        key_method = key_method(opts),
        key_info = key_info,
        key = B64.encode(&wrapped),
        data = B64.encode(&ciphertext),
    )
    .into_bytes())
}

pub(crate) fn key_of_size(alg: SymmetricAlg) -> Result<Zeroizing<Vec<u8>>> {
    let len = match alg {
        SymmetricAlg::Aes128Gcm | SymmetricAlg::Aes128Cbc => 16,
        SymmetricAlg::Aes192Gcm => return Err(Error::msg("aes-192-gcm not supported")),
        SymmetricAlg::Aes256Gcm | SymmetricAlg::Aes256Cbc => 32,
    };
    Ok(Zeroizing::new(vec![0u8; len]))
}

fn rsa_wrap(alg: KeyTransportAlg, public_key: &PublicKey, cek: &[u8]) -> Result<Vec<u8>> {
    let wrapped = match alg {
        KeyTransportAlg::Rsa15 => public_key.encrypt(&mut UnwrapErr(SysRng), Pkcs1v15Encrypt, cek),
        KeyTransportAlg::RsaOaep => {
            public_key.encrypt(&mut UnwrapErr(SysRng), Oaep::<Sha256>::new(), cek)
        }
    };
    wrapped.map_err(|e| Error::msg(format!("rsa key wrap: {e}")))
}

pub(crate) fn rsa_public_key(cert_der: &[u8]) -> Result<PublicKey> {
    let cert =
        Certificate::from_der(cert_der).map_err(|e| Error::msg(format!("recipient cert: {e}")))?;
    let spki = cert
        .tbs_certificate()
        .subject_public_key_info()
        .to_der()
        .map_err(|e| Error::msg(format!("spki: {e}")))?;
    PublicKey::from_public_key_der(&spki).map_err(|e| Error::msg(format!("recipient key: {e}")))
}

// The EncryptionMethod of the EncryptedKey. The official Java osci-bibliothek's
// own RSA-OAEP one always carries explicit MGF/DigestMethod children; omitting
// them makes a real Intermediär's key unwrap ambiguous and fail (SOAP fault
// 9202), and the Java reader gets no parameters to decrypt with. RSA-1.5 has
// no such params.
fn key_method(opts: &EncryptOptions) -> String {
    match opts.key_transport {
        KeyTransportAlg::RsaOaep => format!(
            concat!(
                r#"<xenc:EncryptionMethod Algorithm="{kt}">"#,
                r#"<xenc11:MGF xmlns:xenc11="http://www.w3.org/2009/xmlenc11#" "#,
                r#"Algorithm="http://www.w3.org/2009/xmlenc11#mgf1sha256"></xenc11:MGF>"#,
                r#"<ds:DigestMethod Algorithm="http://www.w3.org/2001/04/xmlenc#sha256"></ds:DigestMethod>"#,
                r#"</xenc:EncryptionMethod>"#,
            ),
            kt = opts.key_transport.uri(),
        ),
        KeyTransportAlg::Rsa15 => format!(
            r#"<xenc:EncryptionMethod Algorithm="{kt}"></xenc:EncryptionMethod>"#,
            kt = opts.key_transport.uri(),
        ),
    }
}

fn encrypted_envelope(opts: &EncryptOptions, cert_der: &[u8], wrapped_key: &[u8]) -> String {
    let key_method = key_method(opts);
    format!(
        concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<soap:Envelope xmlns:ds="http://www.w3.org/2000/09/xmldsig#" "#,
            r#"xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" "#,
            r#"xmlns:xenc="http://www.w3.org/2001/04/xmlenc#" "#,
            r#"xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" "#,
            r#"xsi:schemaLocation="http://schemas.xmlsoap.org/soap/envelope/ soapMessageEncrypted.xsd "#,
            r#"http://www.w3.org/2000/09/xmldsig# oscisig.xsd http://www.w3.org/2001/04/xmlenc# oscienc.xsd">"#,
            r#"<soap:Body><xenc:EncryptedData MimeType="Multipart/Related">"#,
            r#"<xenc:EncryptionMethod Algorithm="{sym}"></xenc:EncryptionMethod>"#,
            r#"<ds:KeyInfo><xenc:EncryptedKey>"#,
            r#"{key_method}"#,
            r#"<ds:KeyInfo><ds:X509Data><ds:X509Certificate>{cert}</ds:X509Certificate>"#,
            r#"</ds:X509Data></ds:KeyInfo>"#,
            r#"<xenc:CipherData><xenc:CipherValue>{key}</xenc:CipherValue></xenc:CipherData>"#,
            r#"</xenc:EncryptedKey></ds:KeyInfo>"#,
            r#"<xenc:CipherData><xenc:CipherReference URI="cid:osci_enc">"#,
            r#"<xenc:Transforms><ds:Transform Algorithm="http://www.w3.org/2000/09/xmldsig#base64">"#,
            r#"</ds:Transform></xenc:Transforms></xenc:CipherReference></xenc:CipherData>"#,
            r#"</xenc:EncryptedData></soap:Body></soap:Envelope>"#,
        ),
        sym = opts.symmetric.uri(),
        key_method = key_method,
        cert = B64.encode(cert_der),
        key = B64.encode(wrapped_key),
    )
}

fn assemble_mime(envelope: &str, ciphertext: &[u8]) -> Vec<u8> {
    // The CipherReference carries a base64 ds:Transform in the XML, but the
    // official Java osci-bibliothek's actual default (msg.base64 = false)
    // writes this part as raw binary, not base64 text. decrypt_transport
    // mirrors this by branching on the part's own Content-Type rather than
    // the Transform label.
    let mut out = Vec::with_capacity(envelope.len() + ciphertext.len() + 256);
    out.extend_from_slice(b"MIME-Version: 1.0\r\n");
    out.extend_from_slice(
        format!("Content-Type: Multipart/Related; boundary={BOUNDARY}; type=text/xml\r\n\r\n")
            .as_bytes(),
    );
    out.extend_from_slice(format!("--{BOUNDARY}\r\n").as_bytes());
    out.extend_from_slice(b"Content-Type: text/xml; charset=UTF-8\r\n");
    out.extend_from_slice(b"Content-Transfer-Encoding: 8bit\r\n");
    out.extend_from_slice(b"Content-ID: <osci@message>\r\n\r\n");
    out.extend_from_slice(envelope.as_bytes());
    out.extend_from_slice(format!("\r\n--{BOUNDARY}\r\n").as_bytes());
    out.extend_from_slice(b"Content-Type: application/octet-stream\r\n");
    out.extend_from_slice(b"Content-Transfer-Encoding: binary\r\n");
    out.extend_from_slice(b"Content-ID: <osci_enc>\r\n\r\n");
    out.extend_from_slice(ciphertext);
    out.extend_from_slice(format!("\r\n--{BOUNDARY}--\r\n").as_bytes());
    out
}
