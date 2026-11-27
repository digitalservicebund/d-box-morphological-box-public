// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! The process card of a delivery: when the Intermediär saw it created,
//! forwarded and received, plus the certificate checks it ran.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::NsReader;

use crate::error::{Error, Result};

/// `osci:TimestampType`: a plain time or a cryptographic timestamp.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Timestamp {
    /// An `xsd:dateTime` as the Intermediär wrote it, e.g.
    /// `2004-02-12T16:46:15+01:00`.
    Plain(String),
    /// A signed timestamp token.
    Cryptographic {
        /// The `Algorithm` attribute, if given.
        algorithm: Option<String>,
        /// The decoded token.
        value: Vec<u8>,
    },
}

/// `osci:ProcessCard`: the stations of one delivery.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProcessCard {
    /// The `RecentModification` attribute: when the card last changed.
    pub recent_modification: String,
    /// When the delivery was created (stored at the Intermediär).
    pub creation: Option<Timestamp>,
    /// When it was forwarded.
    pub forwarding: Option<Timestamp>,
    /// When the addressee received it.
    pub reception: Option<Timestamp>,
    /// The subject given when the delivery was stored.
    pub subject: Option<String>,
}

/// `osci:Inspection`: the check of one certificate.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Inspection {
    /// When the check ran.
    pub timestamp: Option<Timestamp>,
    /// `osci:X509SubjectName`, absent in older messages.
    pub subject_name: Option<String>,
    /// `osci:X509IssuerName`.
    pub issuer_name: String,
    /// `osci:X509SerialNumber`, an integer too big for `u64` in practice.
    pub serial_number: String,
    /// `CertType`: `advanced`, `qualified`, `accredited` or `unknown`.
    pub cert_type: String,
    /// `MathResult`: `ok`, `corrupted` or `indeterminate`.
    pub math_result: String,
    /// `OfflineResult`: `valid`, `invalid` or `indeterminate`.
    pub offline_result: String,
    /// `OnlineResult`: `ok` or `revoked`, when an online check ran.
    pub online_result: Option<String>,
}

/// `osci:ProcessCardBundle`: the card, and the certificate checks, of one
/// message.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProcessCardBundle {
    /// The delivery's MessageId, as written: the Intermediär may wrap it with
    /// a line break inside, like [`crate::envelope::Envelope::message_id`].
    pub message_id: String,
    /// The process card.
    pub card: ProcessCard,
    /// The certificate checks.
    pub inspections: Vec<Inspection>,
}

/// Read every `osci:ProcessCardBundle` from a message's XML (the whole
/// envelope or just its body). The same bundle type is in the responses to
/// FetchProcessCard, and in AcceptDelivery and ProcessDelivery. A message
/// without bundles gives an empty list.
pub fn parse_process_card_bundles(xml: &[u8]) -> Result<Vec<ProcessCardBundle>> {
    let mut r = NsReader::from_reader(xml);
    let mut p = Parser::default();
    loop {
        match r.read_event()? {
            Event::Start(e) => p.start(&e)?,
            Event::Empty(e) => {
                p.start(&e)?;
                p.end()?;
            }
            Event::End(_) => p.end()?,
            Event::Text(t) => p
                .text
                .push_str(&t.xml_content(quick_xml::XmlVersion::Explicit1_0)),
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(p.out)
}

#[derive(Default)]
struct Parser {
    out: Vec<ProcessCardBundle>,
    bundle: Option<ProcessCardBundle>,
    inspection: Option<Inspection>,
    // element names from the bundle down to the current one
    path: Vec<String>,
    text: String,
    algorithm: Option<String>,
}

impl Parser {
    fn start(&mut self, e: &BytesStart<'_>) -> Result<()> {
        let name = e.local_name().as_ref().to_string();
        if self.bundle.is_none() {
            if name == "ProcessCardBundle" {
                self.bundle = Some(ProcessCardBundle::default());
                self.path.push(name);
            }
            return Ok(());
        }
        let attr = |key: &str| -> Result<String> {
            for a in e.attributes() {
                let a = a?;
                if a.key.as_ref() == key {
                    return Ok(a
                        .normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                        .into_owned());
                }
            }
            Ok(String::new())
        };
        match name.as_str() {
            "ProcessCard" => {
                if let Some(b) = self.bundle.as_mut() {
                    b.card.recent_modification = attr("RecentModification")?;
                }
            }
            "Inspection" => self.inspection = Some(Inspection::default()),
            "Cryptographic" => self.algorithm = Some(attr("Algorithm")?).filter(|a| !a.is_empty()),
            "CertType" | "MathResult" | "OfflineResult" | "OnlineResult" => {
                if let Some(i) = self.inspection.as_mut() {
                    match name.as_str() {
                        "CertType" => i.cert_type = attr("Type")?,
                        "MathResult" => i.math_result = attr("Result")?,
                        "OfflineResult" => i.offline_result = attr("Result")?,
                        _ => i.online_result = Some(attr("Result")?),
                    }
                }
            }
            _ => {}
        }
        self.path.push(name);
        self.text.clear();
        Ok(())
    }

    fn end(&mut self) -> Result<()> {
        let Some(name) = self.path.last().cloned() else {
            return Ok(());
        };
        let text = std::mem::take(&mut self.text);
        let text = text.trim();
        let from_end = |n: usize| {
            self.path
                .len()
                .checked_sub(n + 1)
                .and_then(|i| self.path.get(i))
                .map(String::as_str)
                .unwrap_or("")
        };
        let parent = from_end(1).to_string();

        let timestamp = match name.as_str() {
            "Plain" => Some(Timestamp::Plain(text.to_string())),
            "Cryptographic" => Some(Timestamp::Cryptographic {
                algorithm: self.algorithm.take(),
                value: B64
                    .decode(text.split_whitespace().collect::<String>())
                    .map_err(|e| Error::msg(format!("timestamp token: {e}")))?,
            }),
            _ => None,
        };
        if let Some(ts) = timestamp {
            match (
                parent.as_str(),
                self.bundle.as_mut(),
                self.inspection.as_mut(),
            ) {
                ("Creation", Some(b), _) => b.card.creation = Some(ts),
                ("Forwarding", Some(b), _) => b.card.forwarding = Some(ts),
                ("Reception", Some(b), _) => b.card.reception = Some(ts),
                ("Timestamp", _, Some(i)) => i.timestamp = Some(ts),
                _ => {}
            }
        }
        match (name.as_str(), parent.as_str()) {
            ("MessageId", "ProcessCardBundle") => {
                if let Some(b) = self.bundle.as_mut() {
                    b.message_id = text.to_string();
                }
            }
            ("Subject", "ProcessCard") => {
                if let Some(b) = self.bundle.as_mut() {
                    b.card.subject = Some(text.to_string());
                }
            }
            ("X509SubjectName", "Inspection") => {
                if let Some(i) = self.inspection.as_mut() {
                    i.subject_name = Some(text.to_string());
                }
            }
            ("X509IssuerName", "Inspection") => {
                if let Some(i) = self.inspection.as_mut() {
                    i.issuer_name = text.to_string();
                }
            }
            ("X509SerialNumber", "Inspection") => {
                if let Some(i) = self.inspection.as_mut() {
                    i.serial_number = text.to_string();
                }
            }
            ("Inspection", _) => {
                if let (Some(b), Some(i)) = (self.bundle.as_mut(), self.inspection.take()) {
                    b.inspections.push(i);
                }
            }
            ("ProcessCardBundle", _) => self.out.extend(self.bundle.take()),
            _ => {}
        }
        self.path.pop();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUNDLE: &str = concat!(
        r#"<osci:responseToFetchProcessCard xmlns:osci="http://www.osci.de/2002/04/osci">"#,
        r#"<osci:fetchProcessCard><osci:SelectionRule><osci:MessageId>ECHO</osci:MessageId>"#,
        r#"</osci:SelectionRule></osci:fetchProcessCard>"#,
        r#"<osci:ProcessCardBundle><osci:MessageId>M1</osci:MessageId>"#,
        r#"<osci:ProcessCard RecentModification="2004-02-12T16:46:28+01:00">"#,
        r#"<osci:Creation><osci:Plain>2004-02-12T16:46:15+01:00</osci:Plain></osci:Creation>"#,
        r#"<osci:Reception><osci:Cryptographic Algorithm="urn:alg">AQID</osci:Cryptographic></osci:Reception>"#,
        r#"<osci:Subject>Betreff</osci:Subject></osci:ProcessCard>"#,
        r#"<osci:InspectionReport><osci:Inspection>"#,
        r#"<osci:Timestamp><osci:Plain>2004-02-12T16:46:16+01:00</osci:Plain></osci:Timestamp>"#,
        r#"<osci:X509SubjectName>CN=a</osci:X509SubjectName>"#,
        r#"<osci:X509IssuerName>CN=ca</osci:X509IssuerName><osci:X509SerialNumber>42</osci:X509SerialNumber>"#,
        r#"<osci:CertType Type="qualified"></osci:CertType><osci:MathResult Result="ok"/>"#,
        r#"<osci:OfflineResult Result="valid"></osci:OfflineResult>"#,
        r#"<osci:OnlineResult Result="revoked"><osci:CRL>2004-02-12T16:46:17+01:00</osci:CRL></osci:OnlineResult>"#,
        r#"</osci:Inspection></osci:InspectionReport></osci:ProcessCardBundle>"#,
        r#"<osci:ProcessCardBundle><osci:MessageId>M2</osci:MessageId>"#,
        r#"<osci:ProcessCard RecentModification="2004-02-13T00:00:00+01:00"/>"#,
        r#"<osci:InspectionReport/></osci:ProcessCardBundle>"#,
        r#"</osci:responseToFetchProcessCard>"#,
    );

    #[test]
    fn reads_every_field_of_every_bundle() {
        let bundles = parse_process_card_bundles(BUNDLE.as_bytes()).unwrap();
        assert_eq!(bundles.len(), 2);

        let b = &bundles[0];
        assert_eq!(
            b.message_id, "M1",
            "the echoed SelectionRule is not a bundle"
        );
        let card = &b.card;
        assert_eq!(card.recent_modification, "2004-02-12T16:46:28+01:00");
        assert_eq!(
            card.creation,
            Some(Timestamp::Plain("2004-02-12T16:46:15+01:00".into()))
        );
        assert_eq!(card.forwarding, None);
        assert_eq!(
            card.reception,
            Some(Timestamp::Cryptographic {
                algorithm: Some("urn:alg".into()),
                value: vec![1, 2, 3],
            })
        );
        assert_eq!(card.subject.as_deref(), Some("Betreff"));
        assert_eq!(
            b.inspections,
            vec![Inspection {
                timestamp: Some(Timestamp::Plain("2004-02-12T16:46:16+01:00".into())),
                subject_name: Some("CN=a".into()),
                issuer_name: "CN=ca".into(),
                serial_number: "42".into(),
                cert_type: "qualified".into(),
                math_result: "ok".into(),
                offline_result: "valid".into(),
                online_result: Some("revoked".into()),
            }]
        );

        let b = &bundles[1];
        assert_eq!(b.message_id, "M2");
        assert_eq!(b.card.creation, None);
        assert!(b.inspections.is_empty());
    }

    #[test]
    fn no_bundles_is_an_empty_list() {
        assert!(parse_process_card_bundles(b"<a/>").unwrap().is_empty());
        assert!(parse_process_card_bundles(b"").unwrap().is_empty());
    }

    #[test]
    fn a_bad_timestamp_token_is_an_error() {
        let xml = BUNDLE.replace("AQID", "***");
        assert!(parse_process_card_bundles(xml.as_bytes()).is_err());
    }
}
