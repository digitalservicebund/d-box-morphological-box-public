// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use mail_parser::{Message, MessageParser, MessagePart, PartKind};

use crate::error::{Error, Result};

/// One non-XML MIME part of an OSCI message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attachment {
    /// The `Content-ID`, without angle brackets.
    pub content_id: Option<String>,
    /// The `Content-Type`, as `type/subtype`.
    pub content_type: Option<String>,
    /// The decoded part body.
    pub data: Vec<u8>,
}

/// A parsed OSCI message. Parse once, then read the envelope XML and the
/// attachments without parsing again.
pub struct Entity<'a>(Inner<'a>);

enum Inner<'a> {
    /// Bare XML or an `EncryptedData` fragment: no MIME structure.
    Bare(&'a [u8]),
    Mime(Box<Message<'a>>),
}

impl<'a> Entity<'a> {
    /// Parse `raw`, a MIME entity or bare XML.
    pub fn parse(raw: &'a [u8]) -> Result<Self> {
        let entity = skip_ws(strip_mbox_from(raw));
        if entity.starts_with(b"<?xml")
            || entity.starts_with(b"<xenc:")
            || entity.starts_with(b"<soap:")
        {
            return Ok(Self(Inner::Bare(entity)));
        }
        MessageParser::default()
            .parse(entity)
            .map(|m| Self(Inner::Mime(Box::new(m))))
            .ok_or_else(|| Error::msg("mime: parse failed"))
    }

    /// The decoded bytes of the primary `text/xml` part (the OSCI SOAP
    /// envelope).
    pub fn xml(&self) -> Result<Vec<u8>> {
        let msg = match &self.0 {
            Inner::Bare(b) => return Ok(b.to_vec()),
            Inner::Mime(m) => m,
        };
        if let Some(part) = envelope_part(msg) {
            return Ok(part.decoded().into_owned());
        }
        if let Some(part) = msg.text_body().next() {
            return Ok(part.decoded().into_owned());
        }
        Err(Error::msg("mime: no xml part"))
    }

    /// Every part except the envelope, in order. XML attachments such as an
    /// XJustiz message are included. The bytes are the transfer-decoded body
    /// as it came: an encrypted attachment can carry the type of its plaintext
    /// (text/xml), so nothing is converted to UTF-8.
    pub fn attachments(&self) -> Vec<Attachment> {
        let Inner::Mime(msg) = &self.0 else {
            return Vec::new();
        };
        let envelope = envelope_part(msg).map(|e| e.id());
        msg.parts()
            .filter(is_leaf)
            .filter(|part| Some(part.id()) != envelope)
            .map(|part| Attachment {
                content_id: part.content_id().map(str::to_owned),
                content_type: part.content_type().map(|c| match c.subtype() {
                    Some(s) => format!("{}/{}", c.ctype(), s),
                    None => c.ctype().to_owned(),
                }),
                data: part.decoded().into_owned(),
            })
            .collect()
    }
}

// The envelope is the part with Content-ID `osci@message`; messages without
// one fall back to the first XML part, or a part whose id mentions osci.
fn envelope_part<'m>(msg: &'m Message<'_>) -> Option<MessagePart<'m>> {
    let parts = || msg.parts().filter(is_leaf);
    parts()
        .find(|p| p.content_id() == Some("osci@message"))
        .or_else(|| {
            parts().find(|p| is_xml_part(p) || p.content_id().is_some_and(|c| c.contains("osci")))
        })
}

/// The decoded bytes of the primary `text/xml` part (the OSCI SOAP envelope).
/// Bare XML or an `EncryptedData` fragment is returned unchanged.
pub fn main_xml_part(raw: &[u8]) -> Result<Vec<u8>> {
    Entity::parse(raw)?.xml()
}

/// Wrap an OSCI envelope in the single-part `Multipart/Related` MIME entity
/// every real OSCI message uses on the wire, even unsigned and unencrypted
/// ones. The Java osci-bibliothek always sends this shape; only
/// [`crate::envelope::sniff`]/[`main_xml_part`]'s bare-XML fallback lets our
/// own round-trip tests skip it.
pub fn wrap_envelope(xml: &[u8]) -> Vec<u8> {
    write_entity(xml, &[])
}

/// [`wrap_envelope`] with one MIME part per attachment after the envelope
/// part, written the way the Java osci-bibliothek does: base64 body in
/// 76-column lines, `Content-ID`, and `Content-Length` of the encoded body.
/// An attachment needs a `Content-ID`; without a `Content-Type` it goes out
/// as `application/octet-stream`.
pub fn wrap_envelope_with(xml: &[u8], attachments: &[Attachment]) -> Result<Vec<u8>> {
    for a in attachments {
        let id = a
            .content_id
            .as_deref()
            .ok_or_else(|| Error::msg("attachment has no Content-ID"))?;
        if id.is_empty() || id.contains(['<', '>', '\r', '\n']) {
            return Err(Error::msg(format!("invalid attachment Content-ID {id:?}")));
        }
        if a.content_type
            .as_deref()
            .is_some_and(|t| t.contains(['\r', '\n']))
        {
            return Err(Error::msg("attachment Content-Type has a line break"));
        }
    }
    Ok(write_entity(xml, attachments))
}

const BOUNDARY: &str = "MIME_boundary";

/// `MIME_boundary`, with a counter added until the XML does not contain it. The
/// attachments are base64, which has no `-`, so only the XML can clash.
fn boundary_for(xml: &[u8]) -> String {
    let mut boundary = BOUNDARY.to_string();
    let mut n = 0u32;
    while xml
        .windows(boundary.len())
        .any(|w| w == boundary.as_bytes())
    {
        n += 1;
        boundary = format!("{BOUNDARY}_{n}");
    }
    boundary
}

fn write_entity(xml: &[u8], attachments: &[Attachment]) -> Vec<u8> {
    use base64::Engine;
    let boundary = boundary_for(xml);
    let boundary = boundary.as_str();
    let mut out = Vec::with_capacity(xml.len() + 256);
    out.extend_from_slice(b"MIME-Version: 1.0\r\n");
    out.extend_from_slice(
        format!("Content-Type: Multipart/Related; boundary={boundary}; type=text/xml\r\n\r\n")
            .as_bytes(),
    );
    out.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    out.extend_from_slice(b"Content-Type: text/xml; charset=UTF-8\r\n");
    out.extend_from_slice(b"Content-Transfer-Encoding: 8bit\r\n");
    out.extend_from_slice(b"Content-ID: <osci@message>\r\n\r\n");
    out.extend_from_slice(xml);
    for a in attachments {
        let encoded = base64::engine::general_purpose::STANDARD.encode(&a.data);
        let lines = encoded.len().div_ceil(76);
        out.extend_from_slice(format!("\r\n--{boundary}\r\n").as_bytes());
        out.extend_from_slice(
            format!(
                "Content-Type: {}\r\nContent-Transfer-Encoding: base64\r\nContent-ID: <{}>\r\nContent-Length: {}\r\n\r\n",
                a.content_type.as_deref().unwrap_or("application/octet-stream"),
                a.content_id.as_deref().unwrap_or_default(),
                encoded.len() + 2 * lines.saturating_sub(1),
            )
            .as_bytes(),
        );
        // 76-column lines with CRLF between them
        for (i, line) in encoded.as_bytes().chunks(76).enumerate() {
            if i > 0 {
                out.extend_from_slice(b"\r\n");
            }
            out.extend_from_slice(line);
        }
    }
    out.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
    out
}

/// Every non-XML attachment part of an OSCI message, in order. Input that
/// does not parse as MIME has none.
pub fn attachments(raw: &[u8]) -> Result<Vec<Attachment>> {
    Ok(Entity::parse(raw).map_or_else(|_| Vec::new(), |e| e.attachments()))
}

fn is_leaf(part: &MessagePart<'_>) -> bool {
    !matches!(part.kind(), PartKind::Multipart | PartKind::Message(_))
}

fn is_xml_part(part: &MessagePart<'_>) -> bool {
    part.content_type().is_some_and(|c| {
        c.ctype().eq_ignore_ascii_case("text")
            && c.subtype().is_some_and(|s| s.eq_ignore_ascii_case("xml"))
    })
}

// mail-parser expects RFC 5322, not an mbox stream. Some sample files carry a
// leading "From ..." separator line.
fn strip_mbox_from(raw: &[u8]) -> &[u8] {
    if raw.starts_with(b"From ")
        && let Some(nl) = raw.iter().position(|&b| b == b'\n')
    {
        return &raw[nl + 1..];
    }
    raw
}

fn skip_ws(mut s: &[u8]) -> &[u8] {
    while let [first, rest @ ..] = s {
        if first.is_ascii_whitespace() {
            s = rest;
        } else {
            break;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    // A line that looks like our boundary inside the XML must not split it.
    #[test]
    fn xml_that_contains_the_boundary_survives() {
        let xml = b"<e>a\r\n--MIME_boundary\r\nContent-ID: <evil>\r\n\r\nX</e>";
        let sent = [att("a", None, &[1, 2, 3])];
        let wrapped = wrap_envelope_with(xml, &sent).unwrap();
        assert_eq!(main_xml_part(&wrapped).unwrap(), xml);
        assert_eq!(attachments(&wrapped).unwrap().len(), 1);
    }

    const MSG: &[u8] = b"MIME-Version: 1.0\r\n\
Content-Type: Multipart/Related; boundary=MIME_boundary; type=text/xml\r\n\
\r\n\
--MIME_boundary\r\n\
Content-Type: text/xml; charset=UTF-8\r\n\
Content-Transfer-Encoding: 8bit\r\n\
Content-ID: <osci@message>\r\n\
\r\n\
<soap:Envelope/>\r\n\
--MIME_boundary\r\n\
Content-Type: application/octet-stream\r\n\
Content-ID: <att1>\r\n\
Content-Disposition: attachment; filename=x.bin\r\n\
\r\n\
BINARYDATA\r\n\
--MIME_boundary--\r\n";

    #[test]
    fn picks_xml_part() {
        assert_eq!(main_xml_part(MSG).unwrap(), b"<soap:Envelope/>");
    }

    #[test]
    fn bare_xml_passthrough() {
        assert_eq!(
            main_xml_part(b"<xenc:EncryptedData/>").unwrap(),
            b"<xenc:EncryptedData/>"
        );
    }

    #[test]
    fn wrap_envelope_roundtrips_through_main_xml_part() {
        let wrapped = wrap_envelope(b"<soap:Envelope/>");
        assert!(wrapped.starts_with(b"MIME-Version: 1.0\r\n"));
        assert_eq!(main_xml_part(&wrapped).unwrap(), b"<soap:Envelope/>");
    }

    fn att(id: &str, ty: Option<&str>, data: &[u8]) -> Attachment {
        Attachment {
            content_id: Some(id.into()),
            content_type: ty.map(Into::into),
            data: data.to_vec(),
        }
    }

    #[test]
    fn attachments_survive_a_wrap_and_parse() {
        let mut all: Vec<u8> = (0..=255).collect();
        all.extend_from_slice(b"\r\n--MIME_boundary\r\n");
        let sent = vec![
            att("a.pdf", Some("application/pdf"), &all),
            att("b", None, b""),
            att("c", None, &[7; 1000]),
        ];

        let wrapped = wrap_envelope_with(b"<soap:Envelope/>", &sent).unwrap();

        assert_eq!(main_xml_part(&wrapped).unwrap(), b"<soap:Envelope/>");
        let got = attachments(&wrapped).unwrap();
        assert_eq!(got.len(), 3);
        for (s, g) in sent.iter().zip(&got) {
            assert_eq!(g.content_id, s.content_id);
            assert_eq!(g.data, s.data);
        }
        assert_eq!(
            got[1].content_type.as_deref(),
            Some("application/octet-stream")
        );
        assert_eq!(got[0].content_type.as_deref(), Some("application/pdf"));
    }

    #[test]
    fn no_attachments_matches_wrap_envelope() {
        assert_eq!(
            wrap_envelope_with(b"<x/>", &[]).unwrap(),
            wrap_envelope(b"<x/>")
        );
    }

    #[test]
    fn attachment_body_is_wrapped_at_76_columns() {
        let wrapped = wrap_envelope_with(b"<x/>", &[att("a", None, &[1; 200])]).unwrap();
        let text = String::from_utf8(wrapped).unwrap();
        let body = text
            .split("Content-Length: ")
            .nth(1)
            .unwrap()
            .split("\r\n\r\n")
            .nth(1)
            .unwrap()
            .split("\r\n--MIME_boundary--")
            .next()
            .unwrap();
        assert!(body.split("\r\n").all(|l| l.len() <= 76));
        assert!(text.contains(&format!("Content-Length: {}\r\n", body.len())));
    }

    #[test]
    fn attachment_without_a_usable_content_id_is_rejected() {
        for id in [None, Some(""), Some("a>b"), Some("a\r\nb")] {
            let a = Attachment {
                content_id: id.map(Into::into),
                content_type: None,
                data: vec![1],
            };
            assert!(wrap_envelope_with(b"<x/>", &[a]).is_err(), "{id:?}");
        }
    }

    #[test]
    fn lists_attachments() {
        let atts = attachments(MSG).unwrap();
        assert_eq!(atts.len(), 1);
        assert_eq!(atts[0].data, b"BINARYDATA");
        assert_eq!(atts[0].content_id.as_deref(), Some("att1"));
    }

    // An encrypted attachment keeps the content type of its plaintext, so a
    // text/xml part can hold arbitrary bytes, base64 or raw.
    #[test]
    fn binary_bytes_in_a_text_part_are_not_mangled() {
        let binary: Vec<u8> = (0..=255u8).rev().collect();
        let sent = Attachment {
            content_id: Some("x.xml".into()),
            content_type: Some("text/xml".into()),
            data: binary.clone(),
        };
        let wire = wrap_envelope_with(b"<e/>", &[sent]).unwrap();
        assert_eq!(attachments(&wire).unwrap()[0].data, binary);

        let mut raw = concat!(
            "MIME-Version: 1.0\r\n",
            "Content-Type: Multipart/Related; boundary=B; type=text/xml\r\n\r\n",
            "--B\r\nContent-Type: text/xml\r\nContent-ID: <osci@message>\r\n\r\n<e/>\r\n",
            "--B\r\nContent-Type: text/xml\r\nContent-Transfer-Encoding: 8bit\r\n",
            "Content-ID: <x.xml>\r\n\r\n",
        )
        .as_bytes()
        .to_vec();
        raw.extend_from_slice(&binary);
        raw.extend_from_slice(b"\r\n--B--\r\n");
        assert_eq!(attachments(&raw).unwrap()[0].data, binary);
    }

    #[test]
    fn binary_octet_stream_part_keeps_every_byte() {
        // all byte values, with CRLF and a boundary-like line inside
        let mut data: Vec<u8> = (0..=255).collect();
        data.extend_from_slice(b"\r\n--not-the-boundary\r\n\r\n");
        let mut msg = b"MIME-Version: 1.0\r\nContent-Type: Multipart/Related; boundary=B; type=text/xml\r\n\r\n--B\r\nContent-Type: text/xml\r\nContent-ID: <osci@message>\r\n\r\n<soap:Envelope/>\r\n--B\r\nContent-Type: application/octet-stream\r\nContent-Transfer-Encoding: binary\r\nContent-ID: <osci_enc>\r\n\r\n".to_vec();
        msg.extend_from_slice(&data);
        msg.extend_from_slice(b"\r\n--B--\r\n");

        let atts = attachments(&msg).unwrap();

        assert_eq!(atts.len(), 1);
        assert_eq!(
            atts[0].content_type.as_deref(),
            Some("application/octet-stream")
        );
        assert_eq!(atts[0].data, data);
    }
}
