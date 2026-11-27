// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use core::fmt;

use quick_xml::Writer;
use quick_xml::reader::NsReader;

use crate::error::Result;

/// Namespace URIs used across OSCI messages.
pub mod ns {
    /// SOAP 1.1 envelope.
    pub const SOAP_ENVELOPE: &str = "http://schemas.xmlsoap.org/soap/envelope/";
    /// Base OSCI-Transport namespace.
    pub const OSCI: &str = "http://www.osci.de/2002/04/osci";
    /// OSCI 1.2 2017 extension (partial store/fetch).
    pub const OSCI2017: &str = "http://xoev.de/transport/osci12/7";
    /// XML-Signature.
    pub const DS: &str = "http://www.w3.org/2000/09/xmldsig#";
    /// XML-Encryption.
    pub const XENC: &str = "http://www.w3.org/2001/04/xmlenc#";
    /// XML-Encryption 1.1.
    pub const XENC11: &str = "http://www.w3.org/2009/xmlenc11#";
    /// XML Schema instance.
    pub const XSI: &str = "http://www.w3.org/2001/XMLSchema-instance";
    /// WS-Addressing.
    pub const WS_ADDR: &str = "http://www.w3.org/2005/08/addressing";
    /// OSCI 1.2 MessageMetaData.
    pub const MMD: &str = "http://xoev.de/transport/osci12/messagemetadata";
}

/// Fixed prefixes the Java osci-bibliothek emits. Keeping the same prefixes
/// keeps inclusive C14N output byte-identical.
pub mod prefix {
    /// Prefix for [`ns::SOAP_ENVELOPE`](super::ns::SOAP_ENVELOPE).
    pub const SOAP: &str = "soap";
    /// Prefix for [`ns::OSCI`](super::ns::OSCI).
    pub const OSCI: &str = "osci";
    /// Prefix for [`ns::OSCI2017`](super::ns::OSCI2017).
    pub const OSCI2017: &str = "osci2017";
    /// Prefix for [`ns::DS`](super::ns::DS).
    pub const DS: &str = "ds";
    /// Prefix for [`ns::XENC`](super::ns::XENC).
    pub const XENC: &str = "xenc";
    /// Prefix for [`ns::XSI`](super::ns::XSI).
    pub const XSI: &str = "xsi";
}

/// An expanded XML name: namespace URI plus local name.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct QName {
    /// The namespace URI, empty if unqualified.
    pub ns: String,
    /// The local name.
    pub local: String,
}

impl QName {
    /// Build a name from a namespace URI and local name.
    pub fn new(ns: impl Into<String>, local: impl Into<String>) -> Self {
        QName {
            ns: ns.into(),
            local: local.into(),
        }
    }

    /// A name in the base OSCI namespace.
    pub fn osci(local: impl Into<String>) -> Self {
        QName::new(ns::OSCI, local)
    }
}

impl fmt::Display for QName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ns.is_empty() {
            write!(f, "{}", self.local)
        } else {
            write!(f, "{{{}}}{}", self.ns, self.local)
        }
    }
}

impl fmt::Debug for QName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

/// Namespace-resolved name of the first element in a fragment.
pub fn root_qname(xml: &[u8]) -> Result<QName> {
    use quick_xml::events::Event;
    use quick_xml::name::ResolveResult;

    let mut r = NsReader::from_reader(xml);
    loop {
        let (res, ev) = r.read_resolved_event()?;
        match ev {
            Event::Start(e) | Event::Empty(e) => {
                let uri = match res {
                    ResolveResult::Bound(n) => n.as_ref().to_string(),
                    _ => String::new(),
                };
                return Ok(QName::new(uri, e.local_name().as_ref()));
            }
            Event::Eof => {
                return Err(crate::error::Error::UnexpectedEof {
                    expected: "root element",
                });
            }
            _ => {}
        }
    }
}

/// Byte-level XML serialisation for one OSCI type.
pub trait Encode {
    /// Write the element into an existing writer.
    fn encode(&self, w: &mut Writer<Vec<u8>>) -> Result<()>;

    /// Serialise to a fresh byte buffer.
    fn to_xml(&self) -> Result<Vec<u8>> {
        let mut w = Writer::new(Vec::new());
        self.encode(&mut w)?;
        Ok(w.into_inner())
    }
}

/// Byte-level XML deserialisation for one OSCI type.
pub trait Decode: Sized {
    /// Read the element from a positioned reader.
    fn decode(r: &mut NsReader<&[u8]>) -> Result<Self>;

    /// Parse from a standalone fragment.
    fn from_xml(bytes: &[u8]) -> Result<Self> {
        let mut r = NsReader::from_reader(bytes);
        r.config_mut().trim_text(false);
        Self::decode(&mut r)
    }
}
