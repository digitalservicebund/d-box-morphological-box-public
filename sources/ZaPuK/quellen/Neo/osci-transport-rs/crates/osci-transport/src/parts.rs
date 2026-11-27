// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::content::{ContentType, RawContent};
use crate::error::Result;

/// One `osci:ContentContainer`: an ordered list of contents, optionally signed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ContentContainer {
    /// The `Id` attribute, referenced by signatures and `cid:` links.
    pub id: Option<String>,
    /// Raw `<ds:Signature>` bytes, if the container is signed.
    pub signature: Option<Vec<u8>>,
    /// The contained payloads.
    pub contents: Vec<Content>,
    /// Raw `<xenc:EncryptedData>` elements next to the contents. For an
    /// encrypted attachment this one names its `cid:` part and carries the
    /// part's key, so the container must itself be encrypted for the reader.
    pub encrypted: Vec<Vec<u8>>,
}

impl ContentContainer {
    /// An empty container.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append raw content bytes as an inline content.
    pub fn push_raw(&mut self, raw: RawContent) {
        self.contents.push(Content {
            id: None,
            kind: ContentKind::Inline(raw),
        });
    }

    /// Encode a typed payload and append it as inline content.
    pub fn push<T: ContentType>(&mut self, payload: &T) -> Result<()> {
        self.push_raw(payload.encode()?);
        Ok(())
    }

    /// Every inline content, regardless of declared type.
    pub fn inline_contents(&self) -> impl Iterator<Item = &RawContent> {
        self.contents.iter().filter_map(|c| match &c.kind {
            ContentKind::Inline(r) => Some(r),
            _ => None,
        })
    }

    /// Every inline content that decodes as `T`. Others are skipped, not dropped.
    pub fn all<T: ContentType>(&self) -> impl Iterator<Item = Result<T>> + '_ {
        self.inline_contents()
            .filter(|r| r.ty == T::type_qname())
            .map(|r| r.parse::<T>())
    }

    /// The first inline content that decodes as `T`.
    pub fn first<T: ContentType>(&self) -> Option<Result<T>> {
        self.all::<T>().next()
    }
}

/// One `osci:Content` inside a container.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Content {
    /// The `Id` attribute, if any.
    pub id: Option<String>,
    /// What the content holds.
    pub kind: ContentKind,
}

/// The shape of one [`Content`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentKind {
    /// Inline payload (`osci:Base64Content` or `osci:XMLContent`).
    Inline(RawContent),
    /// `href="cid:..."` reference to a MIME attachment.
    Reference {
        /// The `cid:` URI.
        href: String,
    },
    /// A nested `osci:ContentContainer`.
    Nested(Box<ContentContainer>),
    /// Raw `<xenc:EncryptedData>` bytes for content the reader must decrypt.
    Encrypted(Vec<u8>),
}

/// `osci:Feedback/osci:Entry`: a result code plus human text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Feedback {
    /// The four-digit feedback code (spec chapter 5).
    pub code: String,
    /// The human-readable message.
    pub text: String,
    /// The `xml:lang` of `text`.
    pub lang: String,
}

impl Feedback {
    /// A feedback entry with language `de`.
    pub fn new(code: impl Into<String>, text: impl Into<String>) -> Self {
        Feedback {
            code: code.into(),
            text: text.into(),
            lang: "de".into(),
        }
    }
}

/// One `osci:ContentPackage`: a mix of plain and encrypted content containers.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ContentPackage {
    /// The plain content containers.
    pub containers: Vec<ContentContainer>,
    /// Raw `<xenc:EncryptedData>` bytes for encrypted top-level containers.
    pub encrypted: Vec<Vec<u8>>,
}

impl ContentPackage {
    /// Check that the package and the MIME `attachments` that travel with it
    /// match: every `cid:` reference needs an attachment with that
    /// `Content-ID`, ids are unique, and an attachment nothing refers to is
    /// refused. Encrypted content can hold references this cannot see, so
    /// with any of it present only the first two checks run.
    pub fn check_attachments(&self, attachments: &[crate::mime::Attachment]) -> Result<()> {
        fn walk(c: &ContentContainer, refs: &mut Vec<String>, opaque: &mut bool) {
            for enc in &c.encrypted {
                match crate::xenc::EncryptedData::parse(enc) {
                    Ok(ed) => refs.extend(
                        ed.cipher_reference
                            .iter()
                            .map(|r| r.strip_prefix("cid:").unwrap_or(r).to_string()),
                    ),
                    Err(_) => *opaque = true,
                }
            }
            for content in &c.contents {
                match &content.kind {
                    ContentKind::Reference { href } => {
                        refs.push(href.strip_prefix("cid:").unwrap_or(href).to_string());
                    }
                    ContentKind::Nested(inner) => walk(inner, refs, opaque),
                    ContentKind::Encrypted(_) => *opaque = true,
                    ContentKind::Inline(_) => {}
                }
            }
        }
        let mut refs = Vec::new();
        let mut opaque = !self.encrypted.is_empty();
        for c in &self.containers {
            walk(c, &mut refs, &mut opaque);
        }

        let mut ids: Vec<&str> = Vec::with_capacity(attachments.len());
        for a in attachments {
            let id = a
                .content_id
                .as_deref()
                .ok_or_else(|| crate::error::Error::msg("attachment has no Content-ID"))?;
            if ids.contains(&id) {
                return Err(crate::error::Error::msg(format!(
                    "duplicate attachment Content-ID {id}"
                )));
            }
            ids.push(id);
        }
        if let Some(missing) = refs.iter().find(|r| !ids.contains(&r.as_str())) {
            return Err(crate::error::Error::msg(format!(
                "content refers to cid:{missing} but no attachment has that Content-ID"
            )));
        }
        if !opaque && let Some(extra) = ids.iter().find(|id| !refs.iter().any(|r| r == **id)) {
            return Err(crate::error::Error::msg(format!(
                "attachment {extra} is not referenced by any content"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod attachment_tests {
    use super::*;
    use crate::mime::Attachment;

    fn att(id: &str) -> Attachment {
        Attachment {
            content_id: Some(id.into()),
            content_type: None,
            data: vec![1],
        }
    }

    fn package(hrefs: &[&str]) -> ContentPackage {
        let mut c = ContentContainer::new();
        for h in hrefs {
            c.contents.push(Content {
                id: None,
                kind: ContentKind::Reference {
                    href: (*h).to_string(),
                },
            });
        }
        ContentPackage {
            containers: vec![c],
            encrypted: Vec::new(),
        }
    }

    #[test]
    fn matching_references_and_attachments_pass() {
        let p = package(&["cid:a", "cid:b"]);
        p.check_attachments(&[att("b"), att("a")]).unwrap();
        ContentPackage::default().check_attachments(&[]).unwrap();
    }

    #[test]
    fn a_reference_without_its_part_is_refused() {
        let err = package(&["cid:a", "cid:b"])
            .check_attachments(&[att("a")])
            .unwrap_err();
        assert!(err.to_string().contains("cid:b"), "{err}");
    }

    #[test]
    fn an_attachment_nothing_refers_to_is_refused() {
        let err = package(&["cid:a"])
            .check_attachments(&[att("a"), att("z")])
            .unwrap_err();
        assert!(err.to_string().contains('z'), "{err}");
    }

    #[test]
    fn duplicate_ids_are_refused() {
        assert!(
            package(&["cid:a"])
                .check_attachments(&[att("a"), att("a")])
                .is_err()
        );
    }

    #[test]
    fn nested_references_count() {
        let mut inner = ContentContainer::new();
        inner.contents.push(Content {
            id: None,
            kind: ContentKind::Reference {
                href: "cid:deep".into(),
            },
        });
        let mut outer = ContentContainer::new();
        outer.contents.push(Content {
            id: None,
            kind: ContentKind::Nested(Box::new(inner)),
        });
        let p = ContentPackage {
            containers: vec![outer],
            encrypted: Vec::new(),
        };
        p.check_attachments(&[att("deep")]).unwrap();
        assert!(p.check_attachments(&[]).is_err());
    }

    #[test]
    fn encrypted_content_may_hold_references_we_cannot_see() {
        let p = ContentPackage {
            containers: Vec::new(),
            encrypted: vec![b"<xenc:EncryptedData/>".to_vec()],
        };
        p.check_attachments(&[att("hidden")]).unwrap();
    }
}
