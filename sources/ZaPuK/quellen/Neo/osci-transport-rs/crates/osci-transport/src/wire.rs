// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use quick_xml::Writer;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::name::QName as XmlQName;
use quick_xml::reader::NsReader;

use crate::dialog::ControlBlock;
use crate::error::{Error, Result};
use crate::header::{DesiredLanguages, QualityOfTimestamp};
use crate::message::MessageType;
use crate::parts::{Content, ContentContainer, ContentKind, ContentPackage, Feedback};
use crate::xml::{Decode, Encode, ns, root_qname};

/// The five namespaces the official Java osci-bibliothek redeclares, in this order, on
/// every SOAP header block. Emitting the same keeps inclusive C14N
/// byte-identical.
pub const HEADER_NS: [(&str, &str); 5] = [
    ("xmlns:ds", ns::DS),
    ("xmlns:osci", ns::OSCI),
    ("xmlns:soap", ns::SOAP_ENVELOPE),
    ("xmlns:xenc", ns::XENC),
    ("xmlns:xsi", ns::XSI),
];

/// The SOAP actor every OSCI header block targets.
pub const SOAP_ACTOR_NEXT: &str = "http://schemas.xmlsoap.org/soap/actor/next";

fn header_start(local: &str) -> BytesStart<'static> {
    let mut e = BytesStart::new(format!("osci:{local}"));
    for (k, v) in HEADER_NS {
        e.push_attribute((k, v));
    }
    e
}

fn text_child(w: &mut Writer<Vec<u8>>, local: &str, value: &str) -> Result<()> {
    let name = format!("osci:{local}");
    w.write_event(Event::Start(BytesStart::new(name.clone())))?;
    w.write_event(Event::Text(BytesText::new(value)))?;
    w.write_event(Event::End(BytesEnd::new(name)))?;
    Ok(())
}

impl Encode for ControlBlock {
    fn encode(&self, w: &mut Writer<Vec<u8>>) -> Result<()> {
        let mut start = header_start("ControlBlock");
        if let Some(id) = &self.id {
            start.push_attribute(("Id", id.as_str()));
        }
        if let Some(cid) = &self.conversation_id {
            start.push_attribute(("ConversationId", cid.as_str()));
        }
        if let Some(seq) = self.sequence_number {
            start.push_attribute(("SequenceNumber", seq.to_string().as_str()));
        }
        start.push_attribute(("soap:actor", SOAP_ACTOR_NEXT));
        start.push_attribute(("soap:mustUnderstand", "1"));

        w.write_event(Event::Start(start))?;
        // schema order: Response then Challenge
        if let Some(r) = &self.response {
            text_child(w, "Response", r)?;
        }
        if let Some(c) = &self.challenge {
            text_child(w, "Challenge", c)?;
        }
        w.write_event(Event::End(BytesEnd::new("osci:ControlBlock")))?;
        Ok(())
    }
}

impl Decode for ControlBlock {
    fn decode(r: &mut NsReader<&[u8]>) -> Result<Self> {
        let mut cb = ControlBlock::default();
        let mut buf_field: Option<String> = None;
        let mut current: Option<String> = None;
        let mut in_root = false;

        loop {
            match r.read_event()? {
                Event::Start(e) if e.local_name().as_ref() == "ControlBlock" && !in_root => {
                    in_root = true;
                    for a in e.attributes() {
                        let a = a?;
                        let val = a
                            .normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                            .into_owned();
                        match a.key.as_ref() {
                            "Id" => cb.id = Some(val),
                            "ConversationId" => cb.conversation_id = Some(val),
                            "SequenceNumber" => {
                                cb.sequence_number = Some(val.parse().map_err(|_| {
                                    Error::msg(format!("bad SequenceNumber: {val}"))
                                })?)
                            }
                            _ => {}
                        }
                    }
                }
                Event::Start(e) if in_root => {
                    current = Some(e.local_name().as_ref().to_string());
                    buf_field = Some(String::new());
                }
                Event::Text(t) if buf_field.is_some() => {
                    buf_field
                        .as_mut()
                        .unwrap()
                        .push_str(&t.xml_content(quick_xml::XmlVersion::Explicit1_0));
                }
                Event::End(e) if in_root && e.local_name().as_ref() != "ControlBlock" => {
                    let name = current.take().unwrap_or_default();
                    let val = buf_field.take().unwrap_or_default();
                    match name.as_str() {
                        "Challenge" => cb.challenge = Some(val),
                        "Response" => cb.response = Some(val),
                        _ => {}
                    }
                }
                Event::End(e) if e.local_name().as_ref() == "ControlBlock" => break,
                Event::Eof => {
                    return Err(Error::UnexpectedEof {
                        expected: "</osci:ControlBlock>",
                    });
                }
                _ => {}
            }
        }
        Ok(cb)
    }
}

impl Encode for DesiredLanguages {
    fn encode(&self, w: &mut Writer<Vec<u8>>) -> Result<()> {
        let mut start = header_start("DesiredLanguages");
        if let Some(id) = &self.id {
            start.push_attribute(("Id", id.as_str()));
        }
        start.push_attribute(("LanguagesList", self.languages_list().as_str()));
        start.push_attribute(("soap:actor", SOAP_ACTOR_NEXT));
        start.push_attribute(("soap:mustUnderstand", "1"));
        w.write_event(Event::Start(start))?;
        w.write_event(Event::End(BytesEnd::new("osci:DesiredLanguages")))?;
        Ok(())
    }
}

impl Encode for QualityOfTimestamp {
    fn encode(&self, w: &mut Writer<Vec<u8>>) -> Result<()> {
        let mut start = header_start("QualityOfTimestamp");
        start.push_attribute(("Id", self.id.as_str()));
        start.push_attribute(("Service", self.service.as_str()));
        start.push_attribute(("Quality", self.quality.as_str()));
        start.push_attribute(("soap:actor", SOAP_ACTOR_NEXT));
        start.push_attribute(("soap:mustUnderstand", "1"));
        w.write_event(Event::Start(start))?;
        w.write_event(Event::End(BytesEnd::new("osci:QualityOfTimestamp")))?;
        Ok(())
    }
}

impl Decode for DesiredLanguages {
    fn decode(r: &mut NsReader<&[u8]>) -> Result<Self> {
        let mut dl = DesiredLanguages::default();
        loop {
            match r.read_event()? {
                Event::Start(e) | Event::Empty(e)
                    if e.local_name().as_ref() == "DesiredLanguages" =>
                {
                    for a in e.attributes() {
                        let a = a?;
                        let val = a
                            .normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                            .into_owned();
                        match a.key.as_ref() {
                            "Id" => dl.id = Some(val),
                            "LanguagesList" => {
                                dl.languages = val.split_whitespace().map(str::to_owned).collect()
                            }
                            _ => {}
                        }
                    }
                    return Ok(dl);
                }
                Event::Eof => {
                    return Err(Error::UnexpectedEof {
                        expected: "<osci:DesiredLanguages>",
                    });
                }
                _ => {}
            }
        }
    }
}

/// Recursive-descent decode of `osci:ContentPackage`. Handles inline
/// `Base64Content`, `cid:` references, nested containers and encrypted content.
/// `osci:XMLContent` is kept as raw inline markup rather than parsed.
pub fn parse_content_package(xml: &[u8]) -> Result<ContentPackage> {
    let mut r = NsReader::from_reader(xml);
    let mut pkg = ContentPackage::default();
    let mut in_pkg = false;

    loop {
        let before = r.buffer_position() as usize;
        match r.read_event()? {
            Event::Start(e) if e.local_name().as_ref() == "ContentPackage" => in_pkg = true,
            Event::Start(e) if in_pkg && e.local_name().as_ref() == "ContentContainer" => {
                let name = e.name().as_ref().to_string();
                r.read_to_end(XmlQName(&name))?;
                let raw = &xml[before..r.buffer_position() as usize];
                pkg.containers.push(parse_content_container(raw)?);
            }
            Event::Start(e) if in_pkg && e.local_name().as_ref() == "EncryptedData" => {
                let name = e.name().as_ref().to_string();
                r.read_to_end(XmlQName(&name))?;
                pkg.encrypted
                    .push(xml[before..r.buffer_position() as usize].to_vec());
            }
            Event::End(e) if e.local_name().as_ref() == "ContentPackage" => break,
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(pkg)
}

enum Item {
    Eof,
    Container {
        id: Option<String>,
    },
    Signature {
        xml_name: String,
    },
    Encrypted {
        xml_name: String,
    },
    Content {
        id: Option<String>,
        href: Option<String>,
        xml_name: String,
        empty: bool,
    },
    EndContainer,
    Other,
}

fn classify(ev: &Event<'_>) -> Result<Item> {
    let (e, empty) = match ev {
        Event::Start(e) => (e, false),
        Event::Empty(e) => (e, true),
        Event::End(e) if e.local_name().as_ref() == "ContentContainer" => {
            return Ok(Item::EndContainer);
        }
        Event::Eof => return Ok(Item::Eof),
        _ => return Ok(Item::Other),
    };
    let attr = |name: &str| -> Result<Option<String>> {
        for a in e.attributes() {
            let a = a?;
            if a.key.as_ref() == name {
                return Ok(Some(
                    a.normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                        .into_owned(),
                ));
            }
        }
        Ok(None)
    };
    let xml_name = e.name().as_ref().to_string();
    match e.local_name().as_ref() {
        "ContentContainer" => Ok(Item::Container { id: attr("Id")? }),
        "Signature" => Ok(Item::Signature { xml_name }),
        "EncryptedData" => Ok(Item::Encrypted { xml_name }),
        "Content" => Ok(Item::Content {
            id: attr("Id")?,
            href: attr("href")?,
            xml_name,
            empty,
        }),
        _ => Ok(Item::Other),
    }
}

/// Decode one `osci:ContentContainer`, including its signature block and nested
/// containers.
pub fn parse_content_container(xml: &[u8]) -> Result<ContentContainer> {
    parse_container_at(xml, 0)
}

/// How deep containers may nest. A message that goes deeper is refused: every
/// level is parsed again from its bytes, so the work and the stack both grow
/// with the depth.
pub const MAX_CONTAINER_DEPTH: usize = 32;

fn parse_container_at(xml: &[u8], depth: usize) -> Result<ContentContainer> {
    if depth > MAX_CONTAINER_DEPTH {
        return Err(Error::msg(format!(
            "content containers nest deeper than {MAX_CONTAINER_DEPTH}"
        )));
    }
    let mut r = NsReader::from_reader(xml);
    let mut cc = ContentContainer::default();
    let mut started = false;

    loop {
        let before = r.buffer_position() as usize;
        let item = classify(&r.read_event()?)?;
        match item {
            Item::Container { id } if !started => {
                started = true;
                cc.id = id;
            }
            Item::Signature { xml_name } if started => {
                r.read_to_end(XmlQName(&xml_name))?;
                cc.signature = Some(xml[before..r.buffer_position() as usize].to_vec());
            }
            Item::Encrypted { xml_name } if started => {
                r.read_to_end(XmlQName(&xml_name))?;
                cc.encrypted
                    .push(xml[before..r.buffer_position() as usize].to_vec());
            }
            Item::Content {
                id,
                href,
                xml_name,
                empty,
            } if started => {
                let inner = if empty {
                    Vec::new()
                } else {
                    let inner = r.read_to_end(XmlQName(&xml_name))?;
                    xml[inner.start as usize..inner.end as usize].to_vec()
                };
                cc.contents.push(Content {
                    id,
                    kind: content_kind(&inner, href, depth)?,
                });
            }
            Item::EndContainer if started => break,
            Item::Eof => {
                return Err(Error::UnexpectedEof {
                    expected: "</osci:ContentContainer>",
                });
            }
            _ => {}
        }
    }
    Ok(cc)
}

fn content_kind(inner: &[u8], href: Option<String>, depth: usize) -> Result<ContentKind> {
    let trimmed = inner.trim_ascii();
    if trimmed.is_empty() {
        if let Some(h) = href {
            return Ok(ContentKind::Reference { href: h });
        }
        return Ok(ContentKind::Inline(crate::content::RawContent::new(
            crate::xml::QName::new("", ""),
            Vec::new(),
        )));
    }
    let root = root_qname(trimmed)?;
    match root.local.as_str() {
        "Base64Content" => {
            let b64 = element_text(trimmed)?;
            let bytes = B64
                .decode(b64.split_whitespace().collect::<String>())
                .map_err(|e| Error::msg(format!("base64: {e}")))?;
            let ty = root_qname(&bytes).unwrap_or_else(|_| crate::xml::QName::new("", ""));
            Ok(ContentKind::Inline(crate::content::RawContent::new(
                ty, bytes,
            )))
        }
        "ContentContainer" => Ok(ContentKind::Nested(Box::new(parse_container_at(
            trimmed,
            depth + 1,
        )?))),
        "EncryptedData" => Ok(ContentKind::Encrypted(trimmed.to_vec())),
        _ => {
            // osci:XMLContent or anything else: keep the inner markup as-is
            Ok(ContentKind::Inline(crate::content::RawContent::new(
                root,
                trimmed.to_vec(),
            )))
        }
    }
}

fn element_text(xml: &[u8]) -> Result<String> {
    let mut r = NsReader::from_reader(xml);
    let mut out = String::new();
    let mut depth = 0usize;
    loop {
        match r.read_event()? {
            Event::Start(_) => depth += 1,
            Event::End(_) => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    break;
                }
            }
            Event::Text(t) if depth == 1 => {
                out.push_str(&t.xml_content(quick_xml::XmlVersion::Explicit1_0))
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(out)
}

fn write_raw(w: &mut Writer<Vec<u8>>, bytes: &[u8]) {
    w.get_mut().extend_from_slice(bytes);
}

/// Write an `osci:ContentPackage` with all its containers and encrypted blocks.
pub fn encode_content_package(pkg: &ContentPackage, w: &mut Writer<Vec<u8>>) -> Result<()> {
    write_content_package(BytesStart::new("osci:ContentPackage"), pkg, w)
}

/// Encode an `osci:ContentPackage` as a document of its own, with the namespace
/// declarations on its root. [`encode_content_package`] leaves them to the
/// envelope the package is written into.
pub fn encode_content_package_bytes(pkg: &ContentPackage) -> Result<Vec<u8>> {
    let mut w = Writer::new(Vec::new());
    write_content_package(header_start("ContentPackage"), pkg, &mut w)?;
    Ok(w.into_inner())
}

fn write_content_package(
    start: BytesStart<'static>,
    pkg: &ContentPackage,
    w: &mut Writer<Vec<u8>>,
) -> Result<()> {
    w.write_event(Event::Start(start))?;
    for cc in &pkg.containers {
        encode_content_container(cc, w)?;
    }
    for enc in &pkg.encrypted {
        write_raw(w, enc);
    }
    w.write_event(Event::End(BytesEnd::new("osci:ContentPackage")))?;
    Ok(())
}

/// Encode one `osci:ContentContainer` to a fresh byte buffer.
pub fn encode_content_container_bytes(cc: &ContentContainer) -> Result<Vec<u8>> {
    let mut w = Writer::new(Vec::new());
    encode_content_container(cc, &mut w)?;
    Ok(w.into_inner())
}

/// Add raw elements, such as `xenc:EncryptedData`, at the end of the
/// `osci:ContentContainer` in `xml`. Every other byte stays as it is, which
/// parsing and writing the container again would not do.
pub fn append_to_container(xml: &[u8], extra: &[Vec<u8>]) -> Result<Vec<u8>> {
    let mut r = NsReader::from_reader(xml);
    loop {
        match r.read_event()? {
            Event::Start(e) if e.local_name().as_ref() == "ContentContainer" => {
                let name = e.name().as_ref().to_string();
                let at = r.read_to_end(XmlQName(&name))?.end as usize;
                let mut out = xml[..at].to_vec();
                for raw in extra {
                    out.extend_from_slice(raw);
                }
                out.extend_from_slice(&xml[at..]);
                return Ok(out);
            }
            Event::Start(_) | Event::Empty(_) => {
                return Err(Error::msg("the content is not an osci:ContentContainer"));
            }
            Event::Eof => {
                return Err(Error::UnexpectedEof {
                    expected: "<osci:ContentContainer>",
                });
            }
            _ => {}
        }
    }
}

/// Write one `osci:ContentContainer` into an existing writer.
pub fn encode_content_container(cc: &ContentContainer, w: &mut Writer<Vec<u8>>) -> Result<()> {
    let mut start = header_start("ContentContainer");
    if let Some(id) = &cc.id {
        start.push_attribute(("Id", id.as_str()));
    }
    w.write_event(Event::Start(start))?;
    if let Some(sig) = &cc.signature {
        write_raw(w, sig);
    }
    for (i, content) in cc.contents.iter().enumerate() {
        encode_content(content, i, w)?;
    }
    for enc in &cc.encrypted {
        write_raw(w, enc);
    }
    w.write_event(Event::End(BytesEnd::new("osci:ContentContainer")))?;
    Ok(())
}

fn encode_content(content: &Content, index: usize, w: &mut Writer<Vec<u8>>) -> Result<()> {
    if let ContentKind::Reference { href } = &content.kind {
        let mut start = BytesStart::new("osci:Content");
        if let Some(id) = &content.id {
            start.push_attribute(("Id", id.as_str()));
        }
        start.push_attribute(("href", href.as_str()));
        w.write_event(Event::Start(start))?;
        w.write_event(Event::End(BytesEnd::new("osci:Content")))?;
        return Ok(());
    }
    w.write_event(Event::Start(BytesStart::new("osci:Content")))?;
    match &content.kind {
        ContentKind::Inline(raw) => {
            // `osci:Base64Content`'s `Id` attribute is required by the OSCI
            // schema (`order.xsd`'s `Base64Content` element), unlike the
            // wrapping `osci:Content`'s own `Id`, which is optional and left
            // unset here to match the official Java osci-bibliothek's output.
            let id = content
                .id
                .clone()
                .unwrap_or_else(|| format!("content{index}"));
            let mut b64_start = BytesStart::new("osci:Base64Content");
            b64_start.push_attribute(("Id", id.as_str()));
            w.write_event(Event::Start(b64_start))?;
            w.write_event(Event::Text(BytesText::from_escaped(B64.encode(&raw.xml))))?;
            w.write_event(Event::End(BytesEnd::new("osci:Base64Content")))?;
        }
        ContentKind::Nested(cc) => encode_content_container(cc, w)?,
        ContentKind::Encrypted(raw) => write_raw(w, raw),
        ContentKind::Reference { .. } => unreachable!(),
    }
    w.write_event(Event::End(BytesEnd::new("osci:Content")))?;
    Ok(())
}

// Write a bare OSCI envelope. `message_element` writes the osci:<type> block plus
// its children; it is placed in the header or body per MessageType.
fn encode_envelope(
    mt: MessageType,
    cb: &ControlBlock,
    feature_description: bool,
    header_blocks: impl FnOnce(&mut Writer<Vec<u8>>) -> Result<()>,
    message_children: impl FnOnce(&mut Writer<Vec<u8>>) -> Result<()>,
    body_content: impl FnOnce(&mut Writer<Vec<u8>>) -> Result<()>,
) -> Result<Vec<u8>> {
    let mut w = Writer::new(Vec::new());
    w.get_mut()
        .extend_from_slice(br#"<?xml version="1.0" encoding="UTF-8"?>"#);

    let mut env = BytesStart::new("soap:Envelope");
    for (k, v) in HEADER_NS {
        env.push_attribute((k, v));
    }
    let schema_location = format!(
        "{} {} {} oscisig.xsd {} oscienc.xsd",
        ns::SOAP_ENVELOPE,
        mt.soap_schema(),
        ns::DS,
        ns::XENC
    );
    env.push_attribute(("xsi:schemaLocation", schema_location.as_str()));
    w.write_event(Event::Start(env))?;

    let local = mt.local_name();
    let in_body = mt.element_in_body();

    w.write_event(Event::Start(BytesStart::new("soap:Header")))?;
    cb.encode(&mut w)?;
    header_blocks(&mut w)?;
    let message_children = if in_body {
        Some(message_children)
    } else {
        write_message_block(&mut w, local, message_children)?;
        None
    };
    // Always last in the header, right before sign_envelope's
    // NonIntermediaryCertificates block if the message ends up signed - real
    // the official Java osci-bibliothek writes them in that order (certs, then
    // FeatureDescription) and sign_envelope's insertion point matches it.
    if feature_description {
        write_feature_description(&mut w)?;
    }
    w.write_event(Event::End(BytesEnd::new("soap:Header")))?;

    let mut body = BytesStart::new("soap:Body");
    for (k, v) in HEADER_NS {
        body.push_attribute((k, v));
    }
    body.push_attribute(("Id", "body"));
    w.write_event(Event::Start(body))?;
    if let Some(message_children) = message_children {
        // The official Java osci-bibliothek writes this as a completely
        // bare element - no Id, no soap:actor/mustUnderstand (those are
        // SOAP header-only concepts) and no namespace redeclarations
        // (soap:Body above it already declares them), matching the
        // vendored FetchProcessCard.osci sample.
        write_bare_element(&mut w, local, message_children)?;
    }
    body_content(&mut w)?;
    w.write_event(Event::End(BytesEnd::new("soap:Body")))?;

    w.write_event(Event::End(BytesEnd::new("soap:Envelope")))?;
    Ok(w.into_inner())
}

fn write_message_block(
    w: &mut Writer<Vec<u8>>,
    local: &str,
    children: impl FnOnce(&mut Writer<Vec<u8>>) -> Result<()>,
) -> Result<()> {
    let name = format!("osci:{local}");
    let mut start = header_start(local);
    start.push_attribute(("Id", local.to_ascii_lowercase().as_str()));
    start.push_attribute(("soap:actor", SOAP_ACTOR_NEXT));
    start.push_attribute(("soap:mustUnderstand", "1"));
    w.write_event(Event::Start(start))?;
    children(w)?;
    w.write_event(Event::End(BytesEnd::new(name)))?;
    Ok(())
}

fn write_bare_element(
    w: &mut Writer<Vec<u8>>,
    local: &str,
    children: impl FnOnce(&mut Writer<Vec<u8>>) -> Result<()>,
) -> Result<()> {
    let name = format!("osci:{local}");
    w.write_event(Event::Start(BytesStart::new(name.clone())))?;
    children(w)?;
    w.write_event(Event::End(BytesEnd::new(name)))?;
    Ok(())
}

/// The fixed `osci2017:FeatureDescription` block every official Java
/// osci-bibliothek client sends on every request
/// (`DialogHandler.isSendFeatureDescription()` defaults to true): an
/// unconditional declaration of every protocol extension the OSCI 1.2/7
/// spec defines, by version. This isn't a per-feature capability
/// negotiation; it's always the same fixed list regardless of what the
/// sender actually implements.
const FEATURE_DESCRIPTION: &str = concat!(
    r#"<osci2017:FeatureDescription xmlns:ds="http://www.w3.org/2000/09/xmldsig#" "#,
    r#"xmlns:osci="http://www.osci.de/2002/04/osci" "#,
    r#"xmlns:osci2017="http://xoev.de/transport/osci12/7" "#,
    r#"xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" "#,
    r#"xmlns:xenc="http://www.w3.org/2001/04/xmlenc#" "#,
    r#"xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" "#,
    r#"Id="featuredescription"><osci2017:SupportedFeatures>"#,
    r#"<osci2017:Feature Key="DisableBase64" Version="1.2.0"></osci2017:Feature>"#,
    r#"<osci2017:Feature Key="OAEPEncryption" Version="1.6.0"></osci2017:Feature>"#,
    r#"<osci2017:Feature Key="SHA3HashAlgo" Version="1.6.3"></osci2017:Feature>"#,
    r#"<osci2017:Feature Key="GCMPaddingModus" Version="1.7.0"></osci2017:Feature>"#,
    r#"<osci2017:Feature Key="PartialMessageTransmission" Version="1.8.0"></osci2017:Feature>"#,
    r#"<osci2017:Feature Key="Support96Bit12ByteIV" Version="1.9.0"></osci2017:Feature>"#,
    r#"</osci2017:SupportedFeatures></osci2017:FeatureDescription>"#,
);

fn write_feature_description(w: &mut Writer<Vec<u8>>) -> Result<()> {
    w.get_mut()
        .extend_from_slice(FEATURE_DESCRIPTION.as_bytes());
    Ok(())
}

fn write_feedback(w: &mut Writer<Vec<u8>>, feedback: &[Feedback]) -> Result<()> {
    if feedback.is_empty() {
        return Ok(());
    }
    w.write_event(Event::Start(BytesStart::new("osci:Feedback")))?;
    for fb in feedback {
        let mut entry = BytesStart::new("osci:Entry");
        entry.push_attribute(("xml:lang", fb.lang.as_str()));
        w.write_event(Event::Start(entry))?;
        text_child(w, "Code", &fb.code)?;
        text_child(w, "Text", &fb.text)?;
        w.write_event(Event::End(BytesEnd::new("osci:Entry")))?;
    }
    w.write_event(Event::End(BytesEnd::new("osci:Feedback")))?;
    Ok(())
}

/// A response message sent back to a client. Unsigned and unencrypted; the
/// signing and encryption layers wrap the output afterwards.
pub struct ResponseEnvelope {
    /// The `responseTo*` message type.
    pub message_type: MessageType,
    /// The response ControlBlock.
    pub control_block: ControlBlock,
    /// `osci:Feedback` entries for the message block.
    pub feedback: Vec<Feedback>,
    /// An optional `osci:ContentPackage` for the body.
    pub content_package: Option<ContentPackage>,
}

impl ResponseEnvelope {
    /// A response with no feedback and no content.
    pub fn new(message_type: MessageType, control_block: ControlBlock) -> Self {
        ResponseEnvelope {
            message_type,
            control_block,
            feedback: Vec::new(),
            content_package: None,
        }
    }

    /// Add one feedback entry.
    pub fn with_feedback(mut self, fb: Feedback) -> Self {
        self.feedback.push(fb);
        self
    }

    /// Serialise to a SOAP envelope.
    pub fn encode(&self) -> Result<Vec<u8>> {
        encode_envelope(
            self.message_type,
            &self.control_block,
            false,
            |_| Ok(()),
            |w| write_feedback(w, &self.feedback),
            |w| match &self.content_package {
                Some(cp) => encode_content_package(cp, w),
                None => Ok(()),
            },
        )
    }
}

/// The client requests this crate can build, with their message-specific
/// children. The signing and encryption layers wrap the output afterwards.
#[derive(Clone, Debug)]
pub enum Request {
    /// Open an explicit dialog.
    InitDialog,
    /// Close an explicit dialog.
    ExitDialog,
    /// Ask the Intermediär for a fresh MessageId.
    GetMessageId,
    /// Hand a message to the Intermediär for later fetching.
    StoreDelivery {
        /// The MessageId obtained from `GetMessageId`.
        message_id: String,
        /// An optional human-readable subject.
        subject: Option<String>,
    },
    /// Retrieve a stored message.
    FetchDelivery {
        /// Restrict to this MessageId, or take the next waiting message.
        message_id: Option<String>,
    },
    /// Retrieve one or more process cards (Laufzettel).
    FetchProcessCard {
        /// Restrict to this MessageId.
        message_id: Option<String>,
        /// Cap the number of cards returned.
        quantity: Option<u32>,
    },
    /// Have the Intermediär forward a message to a passive recipient.
    ForwardDelivery {
        /// The recipient endpoint URI.
        content_receiver: String,
        /// The MessageId.
        message_id: String,
        /// An optional subject.
        subject: Option<String>,
    },
}

impl Request {
    /// The OSCI message type this request maps to.
    pub fn message_type(&self) -> MessageType {
        match self {
            Request::InitDialog => MessageType::InitDialog,
            Request::ExitDialog => MessageType::ExitDialog,
            Request::GetMessageId => MessageType::GetMessageId,
            Request::StoreDelivery { .. } => MessageType::StoreDelivery,
            Request::FetchDelivery { .. } => MessageType::FetchDelivery,
            Request::FetchProcessCard { .. } => MessageType::FetchProcessCard,
            Request::ForwardDelivery { .. } => MessageType::ForwardDelivery,
        }
    }

    fn write_children(&self, w: &mut Writer<Vec<u8>>) -> Result<()> {
        match self {
            Request::InitDialog | Request::ExitDialog | Request::GetMessageId => Ok(()),
            Request::StoreDelivery {
                message_id,
                subject,
            } => {
                text_child(w, "MessageId", message_id)?;
                if let Some(s) = subject {
                    text_child(w, "Subject", s)?;
                }
                Ok(())
            }
            Request::FetchDelivery { message_id } => write_selection_rule(w, message_id),
            Request::FetchProcessCard {
                message_id,
                quantity,
            } => {
                write_selection_rule(w, message_id)?;
                if let Some(q) = quantity {
                    let mut e = BytesStart::new("osci:Quantity");
                    e.push_attribute(("Limit", q.to_string().as_str()));
                    w.write_event(Event::Empty(e))?;
                }
                Ok(())
            }
            Request::ForwardDelivery {
                content_receiver,
                message_id,
                subject,
            } => {
                let mut cr = BytesStart::new("osci:ContentReceiver");
                cr.push_attribute(("URI", content_receiver.as_str()));
                w.write_event(Event::Empty(cr))?;
                text_child(w, "MessageId", message_id)?;
                if let Some(s) = subject {
                    text_child(w, "Subject", s)?;
                }
                Ok(())
            }
        }
    }
}

fn write_selection_rule(w: &mut Writer<Vec<u8>>, message_id: &Option<String>) -> Result<()> {
    let Some(id) = message_id else { return Ok(()) };
    w.write_event(Event::Start(BytesStart::new("osci:SelectionRule")))?;
    text_child(w, "MessageId", id)?;
    w.write_event(Event::End(BytesEnd::new("osci:SelectionRule")))?;
    Ok(())
}

/// Read the first `osci:MessageId` found anywhere in `xml`. Works whether the
/// element sits in `soap:Header` (most requests) or `soap:Body` (a bare
/// `responseToGetMessageId`, matching the official Java osci-bibliothek's
/// placement).
pub fn parse_message_id(xml: &[u8]) -> Result<Option<String>> {
    let mut r = NsReader::from_reader(xml);
    let mut in_message_id = false;
    let mut buf = String::new();

    loop {
        match r.read_event()? {
            Event::Start(e) if e.local_name().as_ref() == "MessageId" => {
                in_message_id = true;
                buf.clear();
            }
            Event::Text(t) if in_message_id => {
                buf.push_str(&t.xml_content(quick_xml::XmlVersion::Explicit1_0))
            }
            Event::End(e) if e.local_name().as_ref() == "MessageId" => return Ok(Some(buf)),
            Event::Eof => return Ok(None),
            _ => {}
        }
    }
}

/// Read `osci:Feedback/osci:Entry` entries from a message. Whatever sits inside
/// an `osci:ContentPackage` is content from the sender, not an answer from the
/// Intermediär, so it is skipped.
pub fn parse_feedback(xml: &[u8]) -> Result<Vec<Feedback>> {
    let mut r = NsReader::from_reader(xml);
    let mut out = Vec::new();
    let mut in_package = 0usize;
    let mut in_feedback = false;
    let mut cur: Option<Feedback> = None;
    let mut field: Option<String> = None;
    let mut buf = String::new();

    loop {
        match r.read_event()? {
            Event::Start(e) => match e.local_name().as_ref() {
                "ContentPackage" => in_package += 1,
                _ if in_package > 0 => {}
                "Feedback" => in_feedback = true,
                "Entry" if in_feedback => {
                    let mut fb = Feedback::new("", "");
                    for a in e.attributes() {
                        let a = a?;
                        if a.key.as_ref() == "lang" || a.key.as_ref() == "xml:lang" {
                            fb.lang = a
                                .normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                                .into_owned();
                        }
                    }
                    cur = Some(fb);
                }
                "Code" | "Text" if cur.is_some() => {
                    field = Some(e.local_name().as_ref().to_string());
                    buf.clear();
                }
                _ => {}
            },
            Event::Text(t) if field.is_some() => {
                buf.push_str(&t.xml_content(quick_xml::XmlVersion::Explicit1_0))
            }
            Event::End(e) => match e.local_name().as_ref() {
                "ContentPackage" => in_package = in_package.saturating_sub(1),
                _ if in_package > 0 => {}
                "Code" => {
                    if let Some(fb) = cur.as_mut() {
                        fb.code = buf.clone();
                    }
                    field = None;
                }
                "Text" => {
                    if let Some(fb) = cur.as_mut() {
                        fb.text = buf.clone();
                    }
                    field = None;
                }
                "Entry" => {
                    if let Some(fb) = cur.take() {
                        out.push(fb);
                    }
                }
                "Feedback" => in_feedback = false,
                _ => {}
            },
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(out)
}

/// A request envelope: a [`Request`] plus the header blocks and optional
/// content that go with it. Unsigned and unencrypted.
pub struct RequestEnvelope {
    /// The request and its children.
    pub request: Request,
    /// The request ControlBlock.
    pub control_block: ControlBlock,
    /// An optional `osci:DesiredLanguages` header block.
    pub desired_languages: Option<DesiredLanguages>,
    /// `osci:QualityOfTimestamp` header blocks (`StoreDelivery` sends one
    /// each for "creation" and "reception").
    pub quality_of_timestamps: Vec<QualityOfTimestamp>,
    /// An optional `osci:ContentPackage` for the body.
    pub content_package: Option<ContentPackage>,
}

impl RequestEnvelope {
    /// A request with no DesiredLanguages and no content.
    pub fn new(request: Request, control_block: ControlBlock) -> Self {
        RequestEnvelope {
            request,
            control_block,
            desired_languages: None,
            quality_of_timestamps: Vec::new(),
            content_package: None,
        }
    }

    /// Attach a content package.
    pub fn with_content(mut self, pkg: ContentPackage) -> Self {
        self.content_package = Some(pkg);
        self
    }

    /// Serialise to a SOAP envelope.
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mt = self.request.message_type();
        encode_envelope(
            mt,
            &self.control_block,
            true,
            |w| {
                if let Some(dl) = &self.desired_languages {
                    dl.encode(w)?;
                }
                for qot in &self.quality_of_timestamps {
                    qot.encode(w)?;
                }
                Ok(())
            },
            |w| self.request.write_children(w),
            |w| match &self.content_package {
                Some(cp) => encode_content_package(cp, w),
                None => Ok(()),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Plaintext that is not XML (a PDF) or a cut-off container must be an
    // error, not an endless read loop.
    #[test]
    fn container_that_never_ends_is_an_error() {
        for xml in [
            &b"%PDF-1.7 not xml at all"[..],
            b"<osci:ContentContainer Id=\"c\"><osci:Content>",
            b"",
        ] {
            assert!(parse_content_container(xml).is_err(), "{xml:?}");
        }
    }

    #[test]
    fn containers_nested_too_deep_are_refused() {
        let nest = |depth: usize| {
            let mut s = String::from(r#"<osci:ContentContainer xmlns:osci="urn:x">"#);
            for _ in 0..depth {
                s.push_str("<osci:Content><osci:ContentContainer>");
            }
            for _ in 0..depth {
                s.push_str("</osci:ContentContainer></osci:Content>");
            }
            s.push_str("</osci:ContentContainer>");
            s
        };
        assert!(parse_content_container(nest(MAX_CONTAINER_DEPTH).as_bytes()).is_ok());
        let err = parse_content_container(nest(5000).as_bytes()).unwrap_err();
        assert!(err.to_string().contains("nest deeper"), "{err}");
    }

    #[test]
    fn append_to_container_changes_nothing_else() {
        let xml = br#"<osci:ContentContainer xmlns:osci="urn:x" Id="c"><osci:Content><osci:XMLContent Id="y"><a/></osci:XMLContent></osci:Content></osci:ContentContainer>"#;
        let out = append_to_container(xml, &[b"<k/>".to_vec()]).unwrap();
        let want = String::from_utf8(xml.to_vec())
            .unwrap()
            .replace("</osci:ContentContainer>", "<k/></osci:ContentContainer>");
        assert_eq!(String::from_utf8(out).unwrap(), want);
        assert!(append_to_container(b"<a/>", &[]).is_err());
    }

    #[test]
    fn spaces_in_an_end_tag_do_not_cut_the_content() {
        let xml = br#"<osci:ContentContainer xmlns:osci="urn:x"><osci:Content Id="a"><osci:Base64Content>YWJj</osci:Base64Content ></osci:Content ></osci:ContentContainer>"#;
        let cc = parse_content_container(xml).unwrap();
        match &cc.contents[0].kind {
            ContentKind::Inline(r) => assert_eq!(r.xml, b"abc"),
            _ => panic!("expected inline"),
        }
    }

    #[test]
    fn feedback_inside_content_is_not_read() {
        let xml = br#"<soap:Body xmlns:osci="urn:x" xmlns:soap="urn:s"><osci:responseToFetchDelivery><osci:Feedback><osci:Entry><osci:Code>9999</osci:Code><osci:Text>real</osci:Text></osci:Entry></osci:Feedback></osci:responseToFetchDelivery><osci:ContentPackage><osci:ContentContainer><osci:Content><osci:XMLContent><osci:Feedback><osci:Entry><osci:Code>0800</osci:Code><osci:Text>forged</osci:Text></osci:Entry></osci:Feedback></osci:XMLContent></osci:Content></osci:ContentContainer></osci:ContentPackage></soap:Body>"#;
        let got = parse_feedback(xml).unwrap();
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].code, "9999");
    }

    const GOLDEN: &str = concat!(
        r#"<osci:ControlBlock xmlns:ds="http://www.w3.org/2000/09/xmldsig#" "#,
        r#"xmlns:osci="http://www.osci.de/2002/04/osci" "#,
        r#"xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" "#,
        r#"xmlns:xenc="http://www.w3.org/2001/04/xmlenc#" "#,
        r#"xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" "#,
        r#"Id="controlblock" SequenceNumber="0" "#,
        r#"soap:actor="http://schemas.xmlsoap.org/soap/actor/next" "#,
        r#"soap:mustUnderstand="1">"#,
        r#"<osci:Challenge>avSTpzDKSCzijA==</osci:Challenge>"#,
        r#"</osci:ControlBlock>"#,
    );

    #[test]
    fn encodes_to_java_canonical_form() {
        let cb = ControlBlock {
            id: Some("controlblock".into()),
            sequence_number: Some(0),
            challenge: Some("avSTpzDKSCzijA==".into()),
            ..Default::default()
        };
        assert_eq!(String::from_utf8(cb.to_xml().unwrap()).unwrap(), GOLDEN);
    }

    #[test]
    fn parses_golden() {
        let cb = ControlBlock::from_xml(GOLDEN.as_bytes()).unwrap();
        assert_eq!(cb.id.as_deref(), Some("controlblock"));
        assert_eq!(cb.sequence_number, Some(0));
        assert_eq!(cb.challenge.as_deref(), Some("avSTpzDKSCzijA=="));
        assert_eq!(cb.response, None);
    }

    const DL_GOLDEN: &str = concat!(
        r#"<osci:DesiredLanguages xmlns:ds="http://www.w3.org/2000/09/xmldsig#" "#,
        r#"xmlns:osci="http://www.osci.de/2002/04/osci" "#,
        r#"xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" "#,
        r#"xmlns:xenc="http://www.w3.org/2001/04/xmlenc#" "#,
        r#"xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" "#,
        r#"Id="desiredlanguages" LanguagesList="de" "#,
        r#"soap:actor="http://schemas.xmlsoap.org/soap/actor/next" "#,
        r#"soap:mustUnderstand="1"></osci:DesiredLanguages>"#,
    );

    #[test]
    fn desired_languages_golden() {
        let dl = DesiredLanguages {
            id: Some("desiredlanguages".into()),
            languages: vec!["de".into()],
        };
        assert_eq!(String::from_utf8(dl.to_xml().unwrap()).unwrap(), DL_GOLDEN);
        assert_eq!(
            DesiredLanguages::from_xml(DL_GOLDEN.as_bytes()).unwrap(),
            dl
        );
    }

    #[test]
    fn desired_languages_multi() {
        let dl = DesiredLanguages::new(["de", "en", "fr"]);
        let xml = dl.to_xml().unwrap();
        assert_eq!(
            DesiredLanguages::from_xml(&xml).unwrap().languages,
            ["de", "en", "fr"]
        );
    }

    #[test]
    fn response_envelope_roundtrips_through_sniff_and_parse() {
        use crate::envelope::{Sniff, parse, sniff};

        let cb = ControlBlock {
            id: Some("controlblock".into()),
            conversation_id: Some("42".into()),
            sequence_number: Some(0),
            response: Some("abc".into()),
            ..Default::default()
        };
        let bytes = ResponseEnvelope::new(MessageType::ResponseToStoreDelivery, cb)
            .with_feedback(Feedback::new(
                "0801",
                "Auftrag ausgeführt, Dialog weiterhin geöffnet",
            ))
            .encode()
            .unwrap();

        assert_eq!(
            sniff(&bytes).unwrap(),
            Sniff::Message(MessageType::ResponseToStoreDelivery)
        );
        let env = parse(&bytes).unwrap();
        assert_eq!(env.message_type, Some(MessageType::ResponseToStoreDelivery));
        assert_eq!(
            env.control_block.unwrap().conversation_id.as_deref(),
            Some("42")
        );
    }

    #[test]
    fn request_envelopes_roundtrip() {
        use crate::envelope::{Sniff, parse, sniff};

        let cases = [
            Request::InitDialog,
            Request::GetMessageId,
            Request::StoreDelivery {
                message_id: "bWlk".into(),
                subject: Some("Betreff".into()),
            },
            Request::FetchDelivery {
                message_id: Some("bWlk".into()),
            },
            Request::FetchProcessCard {
                message_id: None,
                quantity: Some(10),
            },
            Request::ForwardDelivery {
                content_receiver: "https://example/rec".into(),
                message_id: "bWlk".into(),
                subject: None,
            },
        ];
        for req in cases {
            let mt = req.message_type();
            let bytes = RequestEnvelope::new(req, ControlBlock::default())
                .encode()
                .unwrap();
            assert_eq!(sniff(&bytes).unwrap(), Sniff::Message(mt), "{mt:?}");
            assert_eq!(parse(&bytes).unwrap().message_type, Some(mt));
        }
    }

    #[test]
    fn content_package_roundtrips() {
        use crate::content::RawContent;
        use crate::xml::QName;

        let mut cc = ContentContainer::new();
        cc.id = Some("cc0".into());
        cc.push_raw(RawContent::new(
            QName::new("", ""),
            b"Any content data.".to_vec(),
        ));
        cc.contents.push(Content {
            id: Some("c1".into()),
            kind: ContentKind::Reference {
                href: "cid:doc.pdf".into(),
            },
        });
        let pkg = ContentPackage {
            containers: vec![cc],
            encrypted: Vec::new(),
        };

        let mut w = Writer::new(Vec::new());
        encode_content_package(&pkg, &mut w).unwrap();
        let back = parse_content_package(&w.into_inner()).unwrap();

        assert_eq!(back.containers.len(), 1);
        assert_eq!(back.containers[0].id.as_deref(), Some("cc0"));
        assert_eq!(back.containers[0].contents.len(), 2);
        match &back.containers[0].contents[0].kind {
            ContentKind::Inline(r) => assert_eq!(r.xml, b"Any content data."),
            _ => panic!("expected inline"),
        }
        assert!(matches!(
            &back.containers[0].contents[1].kind,
            ContentKind::Reference { href } if href == "cid:doc.pdf"
        ));
    }

    #[test]
    fn store_delivery_request_carries_message_id() {
        let bytes = RequestEnvelope::new(
            Request::StoreDelivery {
                message_id: "bWlk".into(),
                subject: None,
            },
            ControlBlock::default(),
        )
        .encode()
        .unwrap();
        let s = String::from_utf8(bytes).unwrap();
        assert!(s.contains("<osci:MessageId>bWlk</osci:MessageId>"));
    }

    #[test]
    fn parses_message_id_from_a_bare_body_response() {
        use crate::envelope::parse;

        let bytes =
            ResponseEnvelope::new(MessageType::ResponseToGetMessageId, ControlBlock::default())
                .encode()
                .unwrap();
        // ResponseToGetMessageId carries no MessageId by construction here;
        // splice one into the body the way a real response does.
        let s = String::from_utf8(bytes).unwrap();
        let s = s.replace(
            "<osci:responseToGetMessageId></osci:responseToGetMessageId>",
            "<osci:responseToGetMessageId><osci:MessageId>bWlk</osci:MessageId></osci:responseToGetMessageId>",
        );
        let env = parse(s.as_bytes()).unwrap();
        assert_eq!(env.message_id().unwrap().as_deref(), Some("bWlk"));
    }

    #[test]
    fn message_id_absent_is_none() {
        use crate::envelope::parse;

        let bytes =
            ResponseEnvelope::new(MessageType::ResponseToGetMessageId, ControlBlock::default())
                .encode()
                .unwrap();
        let env = parse(&bytes).unwrap();
        assert_eq!(env.message_id().unwrap(), None);
    }

    #[test]
    fn response_in_body_for_fetch_process_card() {
        let bytes = ResponseEnvelope::new(
            MessageType::ResponseToFetchProcessCard,
            ControlBlock::default(),
        )
        .encode()
        .unwrap();
        let s = String::from_utf8(bytes).unwrap();
        let body_at = s.find("<soap:Body").unwrap();
        assert!(s.find("responseToFetchProcessCard").unwrap() > body_at);
    }

    #[test]
    fn roundtrip() {
        let cb = ControlBlock {
            id: Some("cb".into()),
            conversation_id: Some("42".into()),
            sequence_number: Some(3),
            challenge: Some("aaaa".into()),
            response: Some("bbbb".into()),
        };
        let xml = cb.to_xml().unwrap();
        assert_eq!(ControlBlock::from_xml(&xml).unwrap(), cb);
    }
}
