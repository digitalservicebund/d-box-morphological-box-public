// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! UniFFI surface for iOS and TypeScript clients: classify a message, and
//! encrypt or decrypt at the transport and content layers.

use osci_transport::content::RawContent;
use osci_transport::crypto::{SignatureAlg, SymmetricAlg};
use osci_transport::envelope::{Sniff, sniff};
use osci_transport::mime::{self, Attachment};
use osci_transport::parts::{Content, ContentContainer, ContentKind, ContentPackage};
use osci_transport::role::RoleKind;
use osci_transport::wire::{
    append_to_container, encode_content_container_bytes, encode_content_package_bytes,
};
use osci_transport::xml::QName;
use osci_transport_crypto::{
    CertSlot, EncryptOptions, Recipient, Role, RsaDecrypter, decrypt_attachment, decrypt_content,
    decrypt_transport, encrypt_attachment, encrypt_content, encrypt_content_for_slot,
    encrypt_transport, has_attachment_key, sign_envelope_with_attachments,
    verify_transport_signature,
};
use rsa::PrivateKey;
use rsa::pkcs8::DecodePrivateKey;
use zeroize::Zeroize;

uniffi::setup_scaffolding!();

/// Every error crossing the FFI boundary, flattened to a message.
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum OsciError {
    /// A human-readable error message.
    #[error("{0}")]
    Message(String),
}

impl From<osci_transport::error::Error> for OsciError {
    fn from(e: osci_transport::error::Error) -> Self {
        OsciError::Message(e.to_string())
    }
}

fn err(m: impl Into<String>) -> OsciError {
    OsciError::Message(m.into())
}

// Names the step an error came from, so a message that fails to read says where.
fn step<E: std::fmt::Display>(what: &str) -> impl FnOnce(E) -> OsciError {
    let what = what.to_string();
    move |e| err(format!("{what}: {e}"))
}

/// A decoded `osci:ContentContainer` flattened for the FFI.
#[derive(uniffi::Record)]
pub struct ContentContainerDto {
    /// The container `Id`, if any.
    pub id: Option<String>,
    /// The contained payloads.
    pub contents: Vec<ContentDto>,
}

/// Which [`ContentDto`] variant a content is.
#[derive(Debug, uniffi::Enum)]
pub enum ContentKindDto {
    /// Inline payload; `bytes` holds the decoded payload.
    Inline,
    /// `cid:` reference; `href` holds the target.
    Reference,
    /// Nested container; `bytes` holds its XML.
    Nested,
    /// Encrypted content; `bytes` holds the raw `EncryptedData` XML.
    Encrypted,
}

/// One content inside a [`ContentContainerDto`].
#[derive(uniffi::Record)]
pub struct ContentDto {
    /// The content `Id`, if any.
    pub id: Option<String>,
    /// What kind of content this is.
    pub kind: ContentKindDto,
    /// Decoded payload bytes for `Inline`; the ContentContainer XML for
    /// `Nested`; the raw EncryptedData XML for `Encrypted`; empty for
    /// `Reference`.
    pub bytes: Vec<u8>,
    /// `cid:...` target for `Reference`.
    pub href: Option<String>,
}

/// One non-XML MIME part of an OSCI message.
#[derive(Clone, uniffi::Record)]
pub struct AttachmentDto {
    /// The `Content-ID`, without angle brackets. Needed to encrypt the part.
    pub content_id: Option<String>,
    /// The `Content-Type`, as `type/subtype`.
    pub content_type: Option<String>,
    /// The decoded part body.
    pub data: Vec<u8>,
}

impl From<Attachment> for AttachmentDto {
    fn from(a: Attachment) -> Self {
        AttachmentDto {
            content_id: a.content_id,
            content_type: a.content_type,
            data: a.data,
        }
    }
}

impl From<AttachmentDto> for Attachment {
    fn from(a: AttachmentDto) -> Self {
        Attachment {
            content_id: a.content_id,
            content_type: a.content_type,
            data: a.data,
        }
    }
}

/// The result of [`encrypt_container_with_attachments`].
#[derive(uniffi::Record)]
pub struct SealedContainerDto {
    /// The `xenc:EncryptedData` element to place in the `ContentPackage`.
    pub encrypted_data_xml: Vec<u8>,
    /// The parts to send: the ciphertext under the original `Content-ID` and
    /// `Content-Type`.
    pub attachments: Vec<AttachmentDto>,
}

/// The result of [`decrypt_content_container_with_attachments`].
#[derive(uniffi::Record)]
pub struct OpenedContainerDto {
    /// The decrypted container.
    pub container: ContentContainerDto,
    /// The decrypted parts, in the order they were given.
    pub attachments: Vec<AttachmentDto>,
}

/// Whose certificate a [`CertSlotDto`] holds.
#[derive(Clone, Copy, uniffi::Enum)]
pub enum CertOwnerDto {
    /// The sender of the message.
    Originator,
    /// The recipient of the message.
    Addressee,
    /// A content author other than the originator.
    Author,
    /// A content reader other than the addressee.
    Reader,
    /// The Intermediär.
    Intermediary,
}

/// A certificate to carry in the message header, for the receiving side to find
/// by its slot Id (see [`cert_slot_id`]).
#[derive(Clone, uniffi::Record)]
pub struct CertSlotDto {
    /// Whose certificate this is.
    pub owner: CertOwnerDto,
    /// `true` for a signature certificate, `false` for a cipher certificate.
    pub is_signature: bool,
    /// The DER-encoded certificate.
    pub certificate_der: Vec<u8>,
}

impl From<CertSlotDto> for CertSlot {
    fn from(c: CertSlotDto) -> Self {
        CertSlot {
            kind: match c.owner {
                CertOwnerDto::Originator => RoleKind::Originator,
                CertOwnerDto::Addressee => RoleKind::Addressee,
                CertOwnerDto::Author => RoleKind::Author,
                CertOwnerDto::Reader => RoleKind::Reader,
                CertOwnerDto::Intermediary => RoleKind::Intermediary,
            },
            is_signature: c.is_signature,
            cert_der: c.certificate_der,
        }
    }
}

/// A certificate slot read from a received message header.
#[derive(uniffi::Record)]
pub struct ReceivedCertDto {
    /// Whose certificate this is.
    pub owner: CertOwnerDto,
    /// `true` for a signature certificate, `false` for a cipher certificate.
    pub is_signature: bool,
    /// The slot `Id` in the header, if it has one.
    pub id: Option<String>,
    /// The DER-encoded certificate.
    pub certificate_der: Vec<u8>,
}

/// The result of [`decrypt_message_content`].
#[derive(uniffi::Record)]
pub struct MessageContentDto {
    /// The content containers of the message: the plain ones, then the
    /// decrypted ones.
    pub containers: Vec<ContentContainerDto>,
    /// The attachments, decrypted where the message sent them encrypted.
    pub attachments: Vec<AttachmentDto>,
}

/// The result of [`generate_cipher_key`].
#[derive(uniffi::Record)]
pub struct CipherKeyDto {
    /// The private key as PKCS#8 PEM, the form the decrypt functions take.
    pub private_key_pem: String,
    /// The PKCS#10 certificate request, DER encoded.
    pub csr_der: Vec<u8>,
    /// The public key, DER encoded `SubjectPublicKeyInfo`.
    pub public_key_spki_der: Vec<u8>,
}

/// One inline content of a reply, see [`build_reply_payload`].
#[derive(Clone, uniffi::Record)]
pub struct ReplyContentDto {
    /// The content `Id`, if any.
    pub id: Option<String>,
    /// The payload bytes, for example an XJustiz message as XML.
    pub bytes: Vec<u8>,
}

/// The result of [`classify_message`].
#[derive(Debug, uniffi::Enum)]
pub enum MessageKind {
    /// A plain message; `message_type` is the debug name of the type.
    Message {
        /// The message type name.
        message_type: String,
    },
    /// Transport-encrypted.
    Encrypted,
    /// A `soap:Fault`.
    Fault,
}

// Takes the PEM by value so it's wiped here, right after parsing, regardless
// of which caller received it from the FFI boundary.
fn load_key(mut pem: String) -> Result<PrivateKey, OsciError> {
    let key = PrivateKey::from_pkcs8_pem(&pem).map_err(|e| err(format!("private key: {e}")));
    pem.zeroize();
    key
}

fn to_dto(cc: &ContentContainer) -> Result<ContentContainerDto, OsciError> {
    Ok(ContentContainerDto {
        id: cc.id.clone(),
        contents: cc
            .contents
            .iter()
            .map(|c| {
                Ok(match &c.kind {
                    ContentKind::Inline(r) => ContentDto {
                        id: c.id.clone(),
                        kind: ContentKindDto::Inline,
                        bytes: r.xml.clone(),
                        href: None,
                    },
                    ContentKind::Reference { href } => ContentDto {
                        id: c.id.clone(),
                        kind: ContentKindDto::Reference,
                        bytes: Vec::new(),
                        href: Some(href.clone()),
                    },
                    ContentKind::Nested(inner) => ContentDto {
                        id: c.id.clone(),
                        kind: ContentKindDto::Nested,
                        bytes: osci_transport::wire::encode_content_container_bytes(inner)?,
                        href: None,
                    },
                    ContentKind::Encrypted(raw) => ContentDto {
                        id: c.id.clone(),
                        kind: ContentKindDto::Encrypted,
                        bytes: raw.clone(),
                        href: None,
                    },
                })
            })
            .collect::<Result<_, OsciError>>()?,
    })
}

/// Classify an OSCI envelope (already MIME-unpacked).
#[uniffi::export]
pub fn classify_message(xml: Vec<u8>) -> Result<MessageKind, OsciError> {
    Ok(match sniff(&xml)? {
        Sniff::Message(mt) => MessageKind::Message {
            message_type: format!("{mt:?}"),
        },
        Sniff::Encrypted => MessageKind::Encrypted,
        Sniff::Fault => MessageKind::Fault,
    })
}

/// Decrypt a content-level `EncryptedData` (an encrypted ContentContainer) with
/// the caller's key, selecting the `EncryptedKey` by the caller's
/// certificate-slot Id.
#[uniffi::export]
pub fn decrypt_content_container(
    encrypted_data_xml: Vec<u8>,
    private_key_pem: String,
    recipient_cert_id: String,
) -> Result<ContentContainerDto, OsciError> {
    let dec = RsaDecrypter::new(load_key(private_key_pem)?);
    let cc = decrypt_content(
        &encrypted_data_xml,
        &dec,
        Recipient::CertificateId(&recipient_cert_id),
    )?;
    to_dto(&cc)
}

/// Wrap an OSCI message for the Intermediär: AES-256-GCM over the message, the
/// key RSA-OAEP wrapped with `intermediary_cert_der`. Returns a
/// `multipart/related` MIME entity.
#[uniffi::export]
pub fn encrypt_transport_message(
    message: Vec<u8>,
    intermediary_cert_der: Vec<u8>,
) -> Result<Vec<u8>, OsciError> {
    Ok(encrypt_transport(&message, &intermediary_cert_der)?)
}

/// Encrypt an `osci:ContentContainer` (XML) for one reader. Returns the
/// `xenc:EncryptedData` element to place in a `ContentPackage`.
#[uniffi::export]
pub fn encrypt_content_container(
    container_xml: Vec<u8>,
    reader_cert_der: Vec<u8>,
) -> Result<Vec<u8>, OsciError> {
    Ok(encrypt_content(&container_xml, &reader_cert_der)?)
}

/// Like [`encrypt_content_container`], but the reader is named by a
/// `ds:RetrievalMethod` to the certificate slot `slot_id` in the message
/// header (for example `addressee_cipher`). The Governikus test Intermediär
/// refuses the inline certificate form with SOAP fault 9100. The slot must
/// carry `reader_cert_der`.
#[uniffi::export]
pub fn encrypt_content_container_for_slot(
    container_xml: Vec<u8>,
    reader_cert_der: Vec<u8>,
    slot_id: String,
) -> Result<Vec<u8>, OsciError> {
    Ok(encrypt_content_for_slot(
        &container_xml,
        &reader_cert_der,
        &slot_id,
        &EncryptOptions::default(),
    )?)
}

/// Decrypt a transport-encrypted OSCI message and return the inner MIME message.
#[uniffi::export]
pub fn decrypt_transport_message(
    message: Vec<u8>,
    private_key_pem: String,
    certificate_der: Vec<u8>,
) -> Result<Vec<u8>, OsciError> {
    let dec = RsaDecrypter::new(load_key(private_key_pem)?).with_certificate(certificate_der);
    Ok(decrypt_transport(&message, &dec)?)
}

/// The attachments (the non-XML MIME parts) of a received message, with their
/// transfer encoding removed. For a transport-encrypted message, pass what
/// [`decrypt_transport_message`] returned.
#[uniffi::export]
pub fn message_attachments(message: Vec<u8>) -> Result<Vec<AttachmentDto>, OsciError> {
    Ok(mime::attachments(&message)?
        .into_iter()
        .map(Into::into)
        .collect())
}

/// Wrap an envelope (XML) and its attachments into the `multipart/related`
/// MIME entity that goes on the wire. Sign the envelope first, with the same
/// attachments, if the message is signed.
#[uniffi::export]
pub fn wrap_message_with_attachments(
    envelope_xml: Vec<u8>,
    attachments: Vec<AttachmentDto>,
) -> Result<Vec<u8>, OsciError> {
    let parts: Vec<Attachment> = attachments.into_iter().map(Into::into).collect();
    Ok(mime::wrap_envelope_with(&envelope_xml, &parts)?)
}

/// Check received attachments against the digests a message's signature holds
/// for them. `envelope_xml` is the main XML part. Fails when the signature is
/// not valid, or when a part does not match its digest, is missing, or is not
/// covered by the signature. Returns the signer's certificate (DER), read from
/// the message itself: it is not checked here, so the caller must compare it
/// with the certificate it expects from the sender. Without that check anyone
/// can sign a message with their own key and swap the attachments.
#[uniffi::export]
pub fn verify_attachment_digests(
    envelope_xml: Vec<u8>,
    attachments: Vec<AttachmentDto>,
) -> Result<Vec<u8>, OsciError> {
    let parts: Vec<Attachment> = attachments.into_iter().map(Into::into).collect();
    let report = verify_transport_signature(&envelope_xml, |_| true)?;
    report.check_attachments(&parts)?;
    Ok(report.signer_cert_der)
}

/// Encrypt a container together with its attachments, for one reader. Every
/// attachment gets its own key, which travels inside the encrypted container.
/// The container must refer to each attachment by a `cid:` content with the
/// same `Content-ID`. `slot_id` names the reader's certificate slot in the
/// message header (see [`encrypt_content_container_for_slot`]); without it the
/// certificate is written inline.
#[uniffi::export]
pub fn encrypt_container_with_attachments(
    container_xml: Vec<u8>,
    attachments: Vec<AttachmentDto>,
    reader_cert_der: Vec<u8>,
    slot_id: Option<String>,
) -> Result<SealedContainerDto, OsciError> {
    // The container is not parsed and written again: that would change ids and
    // the kind of its contents. The keys go in as they are.
    let (keys, parts) = encrypt_parts(attachments)?;
    let xml = append_to_container(&container_xml, &keys)?;
    Ok(SealedContainerDto {
        encrypted_data_xml: encrypt_for(&xml, &reader_cert_der, slot_id.as_deref())?,
        attachments: parts,
    })
}

// Encrypts every attachment with its own key. Returns the `EncryptedData`
// holding each key, and the encrypted parts.
fn encrypt_parts(
    attachments: Vec<AttachmentDto>,
) -> Result<(Vec<Vec<u8>>, Vec<AttachmentDto>), OsciError> {
    let mut keys = Vec::with_capacity(attachments.len());
    let mut parts = Vec::with_capacity(attachments.len());
    for plain in attachments {
        let enc = encrypt_attachment(&plain.into(), SymmetricAlg::Aes256Gcm)?;
        keys.push(enc.encrypted_data);
        parts.push(enc.part.into());
    }
    Ok((keys, parts))
}

// Adds the attachment keys to the container and returns its XML.
fn prepare_container(
    mut container: ContentContainer,
    attachments: Vec<AttachmentDto>,
) -> Result<(Vec<u8>, Vec<AttachmentDto>), OsciError> {
    let (keys, parts) = encrypt_parts(attachments)?;
    container.encrypted.extend(keys);
    Ok((encode_content_container_bytes(&container)?, parts))
}

// Encrypts the container for the reader, by slot if there is one.
fn encrypt_for(
    container_xml: &[u8],
    reader_cert_der: &[u8],
    slot_id: Option<&str>,
) -> Result<Vec<u8>, OsciError> {
    Ok(match slot_id {
        Some(slot) => encrypt_content_for_slot(
            container_xml,
            reader_cert_der,
            slot,
            &EncryptOptions::default(),
        )?,
        None => encrypt_content(container_xml, reader_cert_der)?,
    })
}

/// Decrypt a content-level `EncryptedData` like [`decrypt_content_container`]
/// and decrypt the received attachments with the keys inside it. Every given
/// part must be one the container holds a key for.
#[uniffi::export]
pub fn decrypt_content_container_with_attachments(
    encrypted_data_xml: Vec<u8>,
    private_key_pem: String,
    recipient_cert_id: String,
    attachments: Vec<AttachmentDto>,
) -> Result<OpenedContainerDto, OsciError> {
    let dec = RsaDecrypter::new(load_key(private_key_pem)?);
    let cc = decrypt_content(
        &encrypted_data_xml,
        &dec,
        Recipient::CertificateId(&recipient_cert_id),
    )?;
    let attachments = attachments
        .into_iter()
        .map(|a| {
            let data = decrypt_attachment(&cc, &a.clone().into())?;
            Ok(AttachmentDto { data, ..a })
        })
        .collect::<Result<_, OsciError>>()?;
    Ok(OpenedContainerDto {
        container: to_dto(&cc)?,
        attachments,
    })
}

/// The `Id` a certificate slot gets in the message header. It is what
/// [`encrypt_content_container_for_slot`] and
/// [`encrypt_container_with_attachments`] take as the reader's slot.
#[uniffi::export]
pub fn cert_slot_id(slot: CertSlotDto) -> String {
    CertSlot::from(slot).id()
}

/// Sign an unsigned request envelope as the originator (`osci:ClientSignature`,
/// RSA-SHA256). The signer's certificate goes into the header, along with
/// `extra_certs` (the reader's cipher certificate for encrypted content, the
/// originator's own cipher certificate to get an encrypted response). The
/// signature covers the header blocks, the body and each of `attachments`; send
/// the same attachments with [`wrap_message_with_attachments`], or the receiver
/// refuses the message.
#[uniffi::export]
pub fn sign_request(
    envelope_xml: Vec<u8>,
    private_key_pem: String,
    certificate_der: Vec<u8>,
    extra_certs: Vec<CertSlotDto>,
    attachments: Vec<AttachmentDto>,
) -> Result<Vec<u8>, OsciError> {
    let mut pem = private_key_pem;
    let extra: Vec<CertSlot> = extra_certs.into_iter().map(Into::into).collect();
    let parts: Vec<Attachment> = attachments.into_iter().map(Into::into).collect();
    let signed = sign_envelope_with_attachments(
        &envelope_xml,
        Role::Client,
        pem.as_bytes(),
        &certificate_der,
        &extra,
        SignatureAlg::RsaSha256,
        &parts,
    );
    pem.zeroize();
    Ok(signed?)
}

/// Every certificate in the header of a received message, for example the
/// originator's cipher certificate a reply is encrypted for. `envelope_xml` is
/// the main XML part, after the transport layer is decrypted.
#[uniffi::export]
pub fn envelope_certificates(envelope_xml: Vec<u8>) -> Result<Vec<ReceivedCertDto>, OsciError> {
    Ok(osci_transport::certs::cert_slots(&envelope_xml)?
        .into_iter()
        .map(|c| ReceivedCertDto {
            owner: match c.kind {
                RoleKind::Originator => CertOwnerDto::Originator,
                RoleKind::Addressee => CertOwnerDto::Addressee,
                RoleKind::Author => CertOwnerDto::Author,
                RoleKind::Reader => CertOwnerDto::Reader,
                RoleKind::Intermediary => CertOwnerDto::Intermediary,
            },
            is_signature: c.is_signature,
            id: c.id,
            certificate_der: c.cert_der,
        })
        .collect())
}

/// Read a received message in one call: split the MIME, find the
/// `EncryptedData` in the `ContentPackage` that is addressed to the key, decrypt
/// it and the attachments that go with it. `message` is the whole MIME message
/// with the transport layer already removed; `private_key_pem` is the
/// recipient's PKCS#8 key. The recipient's certificate slot is the one in the
/// message header whose certificate belongs to the key. Fails when the message
/// has encrypted content but none of it is addressed to the key. Plain content
/// and plain attachments come back as received.
#[uniffi::export]
pub fn decrypt_message_content(
    message: Vec<u8>,
    private_key_pem: String,
) -> Result<MessageContentDto, OsciError> {
    let dec = RsaDecrypter::new(load_key(private_key_pem)?);
    let entity = mime::Entity::parse(&message).map_err(step("message MIME"))?;
    let xml = entity.xml().map_err(step("message XML part"))?;
    let package = osci_transport::envelope::parse(&xml)
        .map_err(step("message envelope"))?
        .content_package
        .unwrap_or_default();

    let mut containers: Vec<ContentContainer> = package.containers;
    let mut opened = Vec::new();
    if !package.encrypted.is_empty() {
        let slot = osci_transport::certs::cert_slots(&xml)
            .map_err(step("header certificates"))?
            .into_iter()
            .filter(|c| !c.is_signature)
            .find(|c| dec.owns_certificate(&c.cert_der))
            .and_then(|c| c.id)
            .ok_or_else(|| err("no certificate in the message header belongs to this key"))?;
        for raw in &package.encrypted {
            let addressed = osci_transport::xenc::EncryptedData::parse(raw)
                .map_err(step("EncryptedData"))?
                .keys
                .iter()
                .any(|k| {
                    k.retrieval_uri
                        .as_deref()
                        .is_some_and(|u| u.trim_start_matches('#').ends_with(&slot))
                });
            if addressed {
                opened.push(
                    decrypt_content(raw, &dec, Recipient::CertificateId(&slot))
                        .map_err(step(&format!("content container (slot {slot})")))?,
                );
            }
        }
        if opened.is_empty() {
            return Err(err(
                "none of the encrypted content is addressed to this key",
            ));
        }
    }

    let attachments = entity
        .attachments()
        .into_iter()
        .map(|part| {
            let data = match opened.iter().find(|c| has_attachment_key(c, &part)) {
                Some(c) => decrypt_attachment(c, &part)
                    .map_err(step(&format!("attachment {:?}", part.content_id)))?,
                None => part.data.clone(),
            };
            Ok(AttachmentDto {
                data,
                ..part.into()
            })
        })
        .collect::<Result<Vec<_>, OsciError>>()?;
    containers.extend(opened);
    Ok(MessageContentDto {
        containers: containers
            .iter()
            .map(to_dto)
            .collect::<Result<_, OsciError>>()?,
        attachments,
    })
}

/// The slot `Id` a reply is encrypted for: the cipher certificate of the
/// original sender, as the bridge expects it.
const REPLY_SLOT: &str = "addressee_cipher";

/// Build the payload of a reply to a received message: a `multipart/related`
/// message whose first part (`<osci@message>`) is an `osci:ContentPackage` with
/// one container, encrypted for `reader_cert_der`, and whose further parts are
/// `attachments`, each encrypted on its own. `author_cert_der`, the author's
/// own cipher certificate, gets the same container as an `EncryptedData` of its
/// own in the slot `otherreader_cipher`, so the author can read the reply they
/// sent. The message header the reply is sent in must carry that certificate as
/// `CipherCertificateOtherReader` with that `Id`. `reader_cert_der` is the cipher
/// certificate of the original sender, from the header of the message
/// answered. The container holds `contents` and a `cid:` reference to every
/// attachment, and every id must be used once. The result is not signed and has
/// no envelope: whoever sends it on adds those.
#[uniffi::export]
pub fn build_reply_payload(
    contents: Vec<ReplyContentDto>,
    attachments: Vec<AttachmentDto>,
    reader_cert_der: Vec<u8>,
    author_cert_der: Option<Vec<u8>>,
) -> Result<Vec<u8>, OsciError> {
    let mut container = ContentContainer::new();
    container.id = Some("contentcontainer0".into());
    for c in contents {
        container.contents.push(Content {
            id: c.id,
            kind: ContentKind::Inline(RawContent::new(QName::new("", ""), c.bytes)),
        });
    }
    for a in &attachments {
        let id = a
            .content_id
            .clone()
            .filter(|id| !id.is_empty())
            .ok_or_else(|| err("an attachment needs a Content-ID"))?;
        container.contents.push(Content {
            id: Some(id.clone()),
            kind: ContentKind::Reference {
                href: format!("cid:{id}"),
            },
        });
    }
    // Every id in the message names one element, or the signature cannot tell
    // them apart. Count the ids the encoder makes up as well: the container's,
    // `content<N>` for a content without one, and `Attachment<id>` for the key
    // of each attachment.
    let mut seen = std::collections::HashSet::from(["contentcontainer0".to_string()]);
    let written = container
        .contents
        .iter()
        .enumerate()
        .map(|(i, c)| c.id.clone().unwrap_or_else(|| format!("content{i}")));
    let keys = attachments
        .iter()
        .map(|a| format!("Attachment{}", a.content_id.as_deref().unwrap_or_default()));
    for id in written.chain(keys) {
        if !seen.insert(id.clone()) {
            return Err(err(format!("the id {id:?} is used twice")));
        }
    }
    let (xml, parts) = prepare_container(container, attachments)?;
    let mut encrypted = vec![encrypt_for(&xml, &reader_cert_der, Some(REPLY_SLOT))?];
    if let Some(cert) = author_cert_der {
        let slot = CertSlot {
            kind: RoleKind::Reader,
            is_signature: false,
            cert_der: Vec::new(),
        }
        .id();
        encrypted.push(encrypt_for(&xml, &cert, Some(&slot))?);
    }
    let package = encode_content_package_bytes(&ContentPackage {
        containers: Vec::new(),
        encrypted,
    })?;
    wrap_message_with_attachments(package, parts)
}

/// Generate an RSA key and a PKCS#10 request for it, to get a cipher
/// certificate. `bits` is 2048 to 4096; the BSI recommends 3000 or more, so
/// use 3072. `subject` is a distinguished name such as `CN=Court,O=Example`.
/// Generating takes seconds in WebAssembly and blocks, so call it from a
/// worker. The private key leaves as a PEM string, which cannot be wiped from
/// memory afterwards.
#[uniffi::export]
pub fn generate_cipher_key(bits: u32, subject: String) -> Result<CipherKeyDto, OsciError> {
    let key = osci_transport_crypto::generate_cipher_key(bits as usize, &subject)?;
    Ok(CipherKeyDto {
        private_key_pem: key.private_key_pem.to_string(),
        csr_der: key.csr_der,
        public_key_spki_der: key.public_key_spki_der,
    })
}
