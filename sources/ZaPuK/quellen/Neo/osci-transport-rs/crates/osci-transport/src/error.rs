// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::xml::QName;

/// A `Result` defaulting to this crate's [`Error`].
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Everything the codec, dialog and router layers can fail with.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The underlying XML reader or writer failed.
    #[error("xml read/write error: {0}")]
    Xml(#[from] quick_xml::Error),

    /// An XML attribute could not be read.
    #[error("xml attribute error: {0}")]
    XmlAttr(#[from] quick_xml::events::attributes::AttrError),

    /// XML text could not be decoded in its declared encoding.
    #[error("xml encoding error: {0}")]
    XmlEncoding(#[from] quick_xml::encoding::EncodingError),

    /// An I/O error from the writer sink.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// XML text was not valid UTF-8.
    #[error("invalid utf-8 in xml: {0}")]
    Utf8(#[from] core::str::Utf8Error),

    /// The document ended before the expected element.
    #[error("unexpected end of document, expected {expected}")]
    UnexpectedEof {
        /// What the parser was waiting for.
        expected: &'static str,
    },

    /// An element other than the expected one was found.
    #[error("unexpected element <{found}>, expected <{expected}>")]
    UnexpectedElement {
        /// The element name the parser expected.
        expected: String,
        /// The element name it found.
        found: String,
    },

    /// A required child element was absent.
    #[error("missing required child <{0}>")]
    MissingChild(String),

    /// A required attribute was absent.
    #[error("missing required attribute @{0}")]
    MissingAttr(&'static str),

    /// The message-type element did not match any [`MessageType`](crate::MessageType).
    #[error("unknown OSCI message type {0}")]
    UnknownMessageType(QName),

    /// No codec is registered for a content payload type.
    #[error("no content codec registered for {0}")]
    NoContentCodec(QName),

    /// A typed decode was asked for a payload of a different type.
    #[error("content codec mismatch: have {have}, asked for {want}")]
    ContentCodecMismatch {
        /// The payload's declared type.
        have: QName,
        /// The type the caller asked for.
        want: QName,
    },

    /// A free-form error.
    #[error("{0}")]
    Message(String),
}

impl Error {
    /// Build a free-form [`Error::Message`].
    pub fn msg(m: impl Into<String>) -> Self {
        Error::Message(m.into())
    }
}
