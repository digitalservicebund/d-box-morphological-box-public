// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! OSCI-Transport 1.2 message model, codec and handler framework.
//!
//! This crate is transport-agnostic and crypto-agnostic. It parses and builds
//! OSCI messages, tracks dialog state, and offers an axum-style handler API for
//! the Supplier side. Signing, encryption and HTTP live in the companion crates
//! `osci-transport-crypto`, `osci-transport-server` and `osci-transport-client`.
//!
//! Two message types matter for callers who wrap the crate:
//! [`wire::RequestEnvelope`] / [`wire::ResponseEnvelope`] build an envelope,
//! [`envelope::parse`] and [`envelope::sniff`] read one back.

/// Certificate slots in a message and the signer a signature names.
pub mod certs;
/// Typed content payloads: [`content::RawContent`] and the [`content::ContentType`] trait.
pub mod content;
/// Traits and algorithm identifiers for the pluggable crypto backend.
pub mod crypto;
/// Dialog state and the `osci:ControlBlock` (challenge/response, sequence numbers).
pub mod dialog;
/// Reading an OSCI envelope: [`envelope::sniff`], [`envelope::parse`], [`envelope::Envelope`].
pub mod envelope;
/// The crate error type.
pub mod error;
/// `osci:Feedback` acknowledgement codes from spec chapter 5.
pub mod feedback;
/// SOAP header blocks other than the ControlBlock.
pub mod header;
/// The [`message::MessageType`] enum and its XML element names.
pub mod message;
/// Multipart/related MIME handling for OSCI messages.
pub mod mime;
/// Message parts: content containers, contents, attachments, feedback.
pub mod parts;
/// The process card of a delivery: [`process_card::parse_process_card_bundles`].
pub mod process_card;
/// The OSCI roles (originator, addressee, author, reader, intermediary).
pub mod role;
/// axum-style handler framework: [`router::Router`], [`router::Handler`], [`router::Layer`].
pub mod router;
/// Byte-level XML encoders and decoders for OSCI messages.
pub mod wire;
/// XML-Encryption structures (`xenc:EncryptedData`, `xenc:EncryptedKey`).
pub mod xenc;
/// XML helpers: namespaces, [`xml::QName`], the `Encode`/`Decode` traits.
pub mod xml;

pub use error::{Error, Result};
pub use message::MessageType;
pub use xml::QName;
