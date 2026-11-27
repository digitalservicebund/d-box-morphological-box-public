// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! Supplier-side receive and respond helpers: MIME unpacking, transport
//! decryption, signature checking, and response serialisation.

use osci_transport::crypto::Decrypter;
use osci_transport::envelope::{Envelope, Sniff, parse, sniff};
use osci_transport::error::{Error, Result};
use osci_transport::message::MessageType;
use osci_transport::router::{IncomingMessage, OutgoingMessage};
use osci_transport::wire::ResponseEnvelope;
use osci_transport_crypto::{SignatureReport, decrypt_transport, verify_transport_signature};

/// How [`receive`] handles encryption and signatures.
pub struct ReceiverConfig<D> {
    /// Key for opening transport encryption; `None` rejects encrypted messages.
    pub decrypter: Option<D>,
    /// Reject the message if the transport signature is missing or invalid, or
    /// if its MIME attachments do not match the signed digests.
    pub require_valid_signature: bool,
}

impl<D> Default for ReceiverConfig<D> {
    fn default() -> Self {
        ReceiverConfig {
            decrypter: None,
            require_valid_signature: true,
        }
    }
}

/// An incoming OSCI message after MIME unpacking, transport decryption and the
/// transport signature check.
#[derive(Debug)]
pub struct ReceivedMessage {
    /// The message type.
    pub message_type: MessageType,
    /// The parsed envelope.
    pub envelope: Envelope,
    /// The transport signature check, if one ran.
    pub signature: Option<SignatureReport>,
    /// Whether the message arrived transport-encrypted.
    pub transport_encrypted: bool,
    /// Roles resolved from the message's certificate slots.
    pub roles: Vec<osci_transport::role::Role>,
    /// The MIME attachments, in order. Empty when the message has none.
    pub attachments: Vec<osci_transport::mime::Attachment>,
}

/// Unpack, decrypt and signature-check a raw incoming message.
pub fn receive<D: Decrypter>(
    raw: &[u8],
    config: &ReceiverConfig<D>,
    trust: impl Fn(&[u8]) -> bool,
) -> Result<ReceivedMessage> {
    let outer = osci_transport::mime::Entity::parse(raw)?;
    let mut xml = outer.xml()?;
    let mut transport_encrypted = false;
    let mut attachments = None;

    let message_type = loop {
        match sniff(&xml)? {
            Sniff::Message(mt) => break mt,
            Sniff::Fault => {
                let (text, code) = osci_transport::envelope::fault_detail(&xml);
                return Err(Error::msg(match code {
                    Some(code) => format!("SOAP fault {code}: {text}"),
                    None => format!("SOAP fault: {text}"),
                }));
            }
            Sniff::Encrypted => {
                // transport encryption wraps a message exactly once
                if transport_encrypted {
                    return Err(Error::msg("message is encrypted more than once"));
                }
                let dec = config
                    .decrypter
                    .as_ref()
                    .ok_or_else(|| Error::msg("message is transport-encrypted but no decrypter"))?;
                let inner = decrypt_transport(raw, dec)?;
                let decrypted = osci_transport::mime::Entity::parse(&inner)?;
                xml = decrypted.xml()?;
                attachments = Some(decrypted.attachments());
                transport_encrypted = true;
            }
        }
    };

    let signature = match verify_transport_signature(&xml, &trust) {
        Ok(r) => Some(r),
        Err(e) if config.require_valid_signature => return Err(e),
        Err(_) => None,
    };
    if config.require_valid_signature && !signature.as_ref().is_some_and(|s| s.valid) {
        return Err(Error::msg("transport signature is missing or invalid"));
    }
    let attachments = attachments.unwrap_or_else(|| outer.attachments());
    // a part the signature does not vouch for could have been swapped
    if config.require_valid_signature
        && let Some(sig) = &signature
    {
        sig.check_attachments(&attachments)?;
    }

    Ok(ReceivedMessage {
        message_type,
        roles: osci_transport::certs::role_certificates(&xml)?,
        envelope: parse(&xml)?,
        attachments,
        signature,
        transport_encrypted,
    })
}

/// What [`respond_with`] adds on top of the plain envelope.
#[derive(Default)]
pub struct ResponseOptions {
    /// `(RSA private key PEM, signature certificate DER)` to add a
    /// `SupplierSignature`.
    pub signer: Option<(Vec<u8>, Vec<u8>)>,
    /// Client cipher certificate DER to encrypt the response for.
    pub encrypt_for: Option<Vec<u8>>,
}

/// Serialise a handler's response into an unsigned, unencrypted OSCI envelope.
pub fn respond(out: &OutgoingMessage) -> Result<Vec<u8>> {
    respond_with(out, &ResponseOptions::default())
}

/// Serialise a handler's response, optionally signing it as the Supplier and
/// encrypting it for the client.
pub fn respond_with(out: &OutgoingMessage, opts: &ResponseOptions) -> Result<Vec<u8>> {
    let mut env = ResponseEnvelope::new(out.ty, out.control_block.clone());
    env.feedback = out.feedback.clone();
    env.content_package = out
        .container
        .clone()
        .map(|c| osci_transport::parts::ContentPackage {
            containers: vec![c],
            encrypted: Vec::new(),
        });
    let mut bytes = env.encode()?;

    if let Some((key_pem, cert_der)) = &opts.signer {
        bytes = osci_transport_crypto::sign_envelope(
            &bytes,
            osci_transport_crypto::Role::Supplier,
            key_pem,
            cert_der,
        )?;
    }
    // every OSCI message goes on the wire as a MIME entity, signed and
    // unencrypted ones included; an encrypted message wraps this MIME entity
    // again as the ciphertext plaintext
    bytes = osci_transport::mime::wrap_envelope(&bytes);
    if let Some(cert) = &opts.encrypt_for {
        bytes = osci_transport_crypto::encrypt_transport(&bytes, cert)?;
    }
    Ok(bytes)
}

impl ReceivedMessage {
    /// Convert into the router's [`IncomingMessage`], with the roles found in
    /// the message. The router takes one container, so only the first of the
    /// message's containers is passed on.
    pub fn into_incoming(self) -> IncomingMessage {
        IncomingMessage {
            ty: Some(self.message_type),
            control_block: self.envelope.control_block.unwrap_or_default(),
            container: self
                .envelope
                .content_package
                .and_then(|p| p.containers.into_iter().next()),
            roles: self.roles,
            attachments: self.attachments,
        }
    }
}
