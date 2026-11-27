// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use core::marker::PhantomData;

use crate::error::{Error, Result};
use crate::xml::QName;

/// The raw bytes of one content payload plus its declared type. Always kept, so
/// a container survives a round trip even when no codec for `ty` is installed.
#[derive(Clone, PartialEq, Eq)]
pub struct RawContent {
    /// The root element name of the payload.
    pub ty: QName,
    /// The payload bytes.
    pub xml: Vec<u8>,
}

impl RawContent {
    /// Wrap payload bytes with their declared type.
    pub fn new(ty: QName, xml: impl Into<Vec<u8>>) -> Self {
        RawContent {
            ty,
            xml: xml.into(),
        }
    }

    /// Decode as `T`, erroring if the declared type does not match `T`.
    pub fn parse<T: ContentType>(&self) -> Result<T> {
        let want = T::type_qname();
        if self.ty != want {
            return Err(Error::ContentCodecMismatch {
                have: self.ty.clone(),
                want,
            });
        }
        T::decode(self)
    }

    /// Take a lazily typed view without decoding yet.
    pub fn typed<T: ContentType>(self) -> Raw<T> {
        Raw {
            raw: self,
            _t: PhantomData,
        }
    }
}

impl core::fmt::Debug for RawContent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RawContent")
            .field("ty", &self.ty)
            .field("len", &self.xml.len())
            .finish()
    }
}

/// A payload the transport crate does not need to understand. XJustiz and other
/// profiles implement this in their own crates.
pub trait ContentType: Sized {
    /// The payload namespace URI.
    const NS: &'static str;
    /// The payload root local name.
    const LOCAL: &'static str;

    /// The payload root element name, built from [`NS`](Self::NS) and
    /// [`LOCAL`](Self::LOCAL).
    fn type_qname() -> QName {
        QName::new(Self::NS, Self::LOCAL)
    }

    /// Decode from raw bytes.
    fn decode(raw: &RawContent) -> Result<Self>;
    /// Encode to raw bytes.
    fn encode(&self) -> Result<RawContent>;
}

/// Lazily typed view over [`RawContent`]. Re-typing is lossless because the
/// bytes stay.
#[derive(Clone)]
pub struct Raw<T> {
    raw: RawContent,
    _t: PhantomData<fn() -> T>,
}

impl<T: ContentType> Raw<T> {
    /// Wrap raw content in a typed view.
    pub fn from_raw(raw: RawContent) -> Self {
        Raw {
            raw,
            _t: PhantomData,
        }
    }

    /// Decode the payload as `T`.
    pub fn get(&self) -> Result<T> {
        self.raw.parse::<T>()
    }

    /// Reinterpret as a different type without touching the bytes.
    pub fn cast<U: ContentType>(self) -> Raw<U> {
        Raw {
            raw: self.raw,
            _t: PhantomData,
        }
    }

    /// Take back the raw content.
    pub fn into_raw(self) -> RawContent {
        self.raw
    }

    /// Borrow the raw content.
    pub fn as_raw(&self) -> &RawContent {
        &self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xml::ns;

    struct Greeting(String);

    impl ContentType for Greeting {
        const NS: &'static str = ns::OSCI;
        const LOCAL: &'static str = "greeting";

        fn decode(raw: &RawContent) -> Result<Self> {
            Ok(Greeting(String::from_utf8_lossy(&raw.xml).into_owned()))
        }

        fn encode(&self) -> Result<RawContent> {
            Ok(RawContent::new(
                Greeting::type_qname(),
                self.0.clone().into_bytes(),
            ))
        }
    }

    #[test]
    fn lossless_roundtrip() {
        let raw = Greeting("hi".into()).encode().unwrap();
        let back: Greeting = raw.parse().unwrap();
        assert_eq!(back.0, "hi");
    }

    #[test]
    fn raw_kept_and_mismatch_reported() {
        let rc = RawContent::new(QName::new("urn:unknown", "thing"), b"<x/>".to_vec());
        assert_eq!(rc.xml, b"<x/>");
        assert!(matches!(
            rc.parse::<Greeting>(),
            Err(Error::ContentCodecMismatch { .. })
        ));
    }

    #[test]
    fn cast_keeps_bytes() {
        let raw = Greeting("keep".into()).encode().unwrap();
        let bytes = raw.xml.clone();
        let typed = raw.typed::<Greeting>();
        assert_eq!(typed.as_raw().xml, bytes);
    }
}
