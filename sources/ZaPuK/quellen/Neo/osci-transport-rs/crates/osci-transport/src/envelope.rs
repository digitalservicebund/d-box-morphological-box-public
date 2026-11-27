// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use quick_xml::events::Event;
use quick_xml::name::{QName as XmlQName, ResolveResult};
use quick_xml::reader::NsReader;

use crate::dialog::ControlBlock;
use crate::error::{Error, Result};
use crate::header::DesiredLanguages;
use crate::message::MessageType;
use crate::parts::{ContentPackage, Feedback};
use crate::wire::{parse_content_package, parse_feedback, parse_message_id};
use crate::xml::{Decode, QName, ns};

/// What an OSCI message is, without parsing it fully.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sniff {
    /// A plain message of this type.
    Message(MessageType),
    /// Transport-encrypted; decrypt it first.
    Encrypted,
    /// A `soap:Fault`.
    Fault,
}

/// Classify an OSCI message.
///
/// The message-type element sits in `soap:Header` for some messages and
/// `soap:Body` for others, so both are scanned; the first element resolving to a
/// known [`MessageType`] wins. An `xenc:EncryptedData` as a direct `soap:Body`
/// child (or a bare fragment) is reported as [`Sniff::Encrypted`].
pub fn sniff(xml: &[u8]) -> Result<Sniff> {
    let mut r = NsReader::from_reader(xml);
    let mut depth = 0usize;
    let mut body_depth: Option<usize> = None;

    loop {
        let (res, event) = r.read_resolved_event()?;
        let (start, empty) = match &event {
            Event::Start(e) => (Some(e), false),
            Event::Empty(e) => (Some(e), true),
            Event::End(_) => {
                depth = depth.saturating_sub(1);
                continue;
            }
            Event::Eof => return Err(Error::msg("envelope: no message type found")),
            _ => continue,
        };
        let e = start.unwrap();
        let local = e.local_name().as_ref().to_string();
        let this_depth = depth;
        if !empty {
            depth += 1;
        }

        if local == "Body" && is_ns(&res, ns::SOAP_ENVELOPE) {
            body_depth = Some(this_depth);
        }
        if local == "EncryptedData" && (this_depth == 0 || body_depth == Some(this_depth - 1)) {
            return Ok(Sniff::Encrypted);
        }
        if local == "Fault" && is_ns(&res, ns::SOAP_ENVELOPE) {
            return Ok(Sniff::Fault);
        }
        let q = QName::new(uri_of(&res), local);
        if let Ok(mt) = MessageType::from_element(&q) {
            return Ok(Sniff::Message(mt));
        }
    }
}

/// One direct child of `soap:Header`, kept as raw bytes so blocks the crate does
/// not model are not lost.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawBlock {
    /// The block's element name.
    pub name: QName,
    /// Its verbatim XML bytes.
    pub xml: Vec<u8>,
}

/// A parsed unencrypted OSCI envelope.
///
/// `header_blocks` and `body` keep the raw bytes; the other fields are the
/// decoded views the crate builds on top.
#[derive(Clone, Debug, Default)]
pub struct Envelope {
    /// The message type, from the header or body.
    pub message_type: Option<MessageType>,
    /// The `osci:ControlBlock`.
    pub control_block: Option<ControlBlock>,
    /// The `osci:DesiredLanguages` header block, if present.
    pub desired_languages: Option<DesiredLanguages>,
    /// Every `soap:Header` child, verbatim.
    pub header_blocks: Vec<RawBlock>,
    /// The `soap:Body` inner markup, verbatim.
    pub body: Vec<u8>,
    /// The decoded `osci:ContentPackage` from the body, if any.
    pub content_package: Option<ContentPackage>,
    /// Every `osci:Feedback` entry found anywhere in the message.
    pub feedback: Vec<Feedback>,
}

impl Envelope {
    /// The `osci:MessageId`, e.g. from a `responseToGetMessageId` in the body.
    pub fn message_id(&self) -> Result<Option<String>> {
        parse_message_id(&self.body)
    }
}

/// Parse an unencrypted OSCI envelope into an [`Envelope`].
///
/// Every `soap:Header` child is captured raw; the ControlBlock, DesiredLanguages
/// and the message-type element are also decoded. `soap:Body` inner markup is
/// kept raw and, if it holds an `osci:ContentPackage`, decoded.
pub fn parse(xml: &[u8]) -> Result<Envelope> {
    let mut r = NsReader::from_reader(xml);
    let mut env = Envelope::default();
    let mut section = 0u8; // 0 root, 1 header, 2 done-header

    loop {
        let before = r.buffer_position() as usize;
        let (res, event) = r.read_resolved_event()?;
        match event {
            Event::Start(e) => {
                let local = e.local_name().as_ref().to_string();

                if local == "Header" && section == 0 && is_ns(&res, ns::SOAP_ENVELOPE) {
                    section = 1;
                    continue;
                }
                if local == "Body" && section != 1 && is_ns(&res, ns::SOAP_ENVELOPE) {
                    let name = e.name().as_ref().to_string();
                    let inner = r.read_to_end(XmlQName(&name))?;
                    env.body = xml[inner.start as usize..inner.end as usize].to_vec();
                    break;
                }
                if section != 1 {
                    continue;
                }
                // direct child of soap:Header
                let q = QName::new(uri_of(&res), local.clone());
                let name = e.name().as_ref().to_string();
                r.read_to_end(XmlQName(&name))?;
                let end = r.buffer_position() as usize;
                let raw = xml[before..end].to_vec();

                if env.message_type.is_none()
                    && let Ok(mt) = MessageType::from_element(&q)
                {
                    env.message_type = Some(mt);
                }
                match local.as_str() {
                    "ControlBlock" => env.control_block = Some(ControlBlock::from_xml(&raw)?),
                    "DesiredLanguages" => {
                        env.desired_languages = Some(DesiredLanguages::from_xml(&raw)?)
                    }
                    _ => {}
                }
                env.header_blocks.push(RawBlock { name: q, xml: raw });
            }
            Event::End(e) if section == 1 && e.local_name().as_ref() == "Header" => {
                section = 2;
            }
            Event::Eof => break,
            _ => {}
        }
    }

    if env.message_type.is_none() {
        // message-type element may sit in the body instead
        if let Ok(Sniff::Message(mt)) = sniff(xml) {
            env.message_type = Some(mt);
        }
    }

    if find_subslice(&env.body, b"ContentPackage").is_some() {
        env.content_package = Some(parse_content_package(&env.body)?);
    }
    env.feedback = parse_feedback(xml)?;
    Ok(env)
}

/// Pull `faultstring` and the `osci:Code` out of a `soap:Fault` body, for a
/// readable error when [`sniff`] reports [`Sniff::Fault`]. Best-effort: falls
/// back to placeholders if the fault doesn't carry the OSCI `detail/Code`.
pub fn fault_detail(xml: &[u8]) -> (String, Option<String>) {
    fn between(s: &str, open: &str, close: &str) -> Option<String> {
        let start = s.find(open)? + open.len();
        let end = s[start..].find(close)? + start;
        Some(s[start..end].to_string())
    }

    let s = String::from_utf8_lossy(xml);
    let text = between(&s, "<faultstring>", "</faultstring>")
        .unwrap_or_else(|| "unknown SOAP fault".to_string());
    let code = between(&s, "<osci:Code>", "</osci:Code>");
    (text, code)
}

fn uri_of(res: &ResolveResult) -> String {
    match res {
        ResolveResult::Bound(n) => n.as_ref().to_string(),
        _ => String::new(),
    }
}

fn is_ns(res: &ResolveResult, want: &str) -> bool {
    matches!(res, ResolveResult::Bound(n) if n.as_ref() == want)
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(header: &str, body: &str) -> String {
        format!(
            r#"<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:osci="http://www.osci.de/2002/04/osci" xmlns:xenc="http://www.w3.org/2001/04/xmlenc#"><soap:Header>{header}</soap:Header><soap:Body>{body}</soap:Body></soap:Envelope>"#
        )
    }

    #[test]
    fn detects_store_delivery() {
        let x = env(
            r#"<osci:ControlBlock/><osci:storeDelivery Id="x"/>"#,
            r#"<osci:ContentPackage/>"#,
        );
        assert_eq!(
            sniff(x.as_bytes()).unwrap(),
            Sniff::Message(MessageType::StoreDelivery)
        );
    }

    #[test]
    fn detects_encrypted() {
        let x = env("<osci:ControlBlock/>", r#"<xenc:EncryptedData/>"#);
        assert_eq!(sniff(x.as_bytes()).unwrap(), Sniff::Encrypted);
    }

    #[test]
    fn detects_encrypted_bare() {
        assert_eq!(
            sniff(br#"<xenc:EncryptedData xmlns:xenc="x"/>"#).unwrap(),
            Sniff::Encrypted
        );
    }

    #[test]
    fn detects_fault() {
        let x = env("<osci:ControlBlock/>", r#"<soap:Fault/>"#);
        assert_eq!(sniff(x.as_bytes()).unwrap(), Sniff::Fault);
    }

    #[test]
    fn parse_extracts_header_blocks() {
        let header = concat!(
            r#"<osci:ControlBlock Id="cb" SequenceNumber="0"><osci:Challenge>abc</osci:Challenge></osci:ControlBlock>"#,
            r#"<osci:DesiredLanguages Id="dl" LanguagesList="de en"></osci:DesiredLanguages>"#,
            r#"<osci:storeDelivery Id="sd"></osci:storeDelivery>"#,
        );
        let x = env(header, "<osci:ContentPackage></osci:ContentPackage>");
        let p = parse(x.as_bytes()).unwrap();

        assert_eq!(p.message_type, Some(MessageType::StoreDelivery));
        assert_eq!(p.control_block.unwrap().challenge.as_deref(), Some("abc"));
        assert_eq!(p.desired_languages.unwrap().languages, ["de", "en"]);
        assert_eq!(p.header_blocks.len(), 3);
        assert_eq!(p.body, b"<osci:ContentPackage></osci:ContentPackage>");
        assert!(p.header_blocks[0].xml.starts_with(b"<osci:ControlBlock"));
    }
}
