// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::collections::HashMap;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use quick_xml::events::Event;
use quick_xml::name::QName as XmlQName;
use quick_xml::reader::NsReader;

use crate::error::{Error, Result};
use crate::role::{Role, RoleKind};

/// How a `ds:Signature` names its signing certificate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SignerRef {
    /// An inline `ds:X509Certificate` (DER).
    Inline(Vec<u8>),
    /// A `ds:RetrievalMethod` URI, e.g. `#signaturecertificateoriginator`.
    Retrieval(String),
}

/// One `ds:Reference` with a `cid:` URI: the digest a signature claims for a
/// MIME attachment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttachmentReference {
    /// The `URI`, e.g. `cid:Klage.pdf`.
    pub uri: String,
    /// The `DigestMethod` algorithm URI.
    pub digest_method: String,
    /// The decoded `DigestValue`.
    pub digest_value: Vec<u8>,
    /// Whether the reference has `ds:Transforms`. OSCI attachments have none.
    pub has_transforms: bool,
}

/// The `cid:` references of the first `ds:Signature` in the document, in
/// order. Pass an envelope for the transport signature or one container's XML
/// for that container's signature.
pub fn attachment_references(xml: &[u8]) -> Result<Vec<AttachmentReference>> {
    let mut r = NsReader::from_reader(xml);
    let mut out = Vec::new();
    let mut in_signature = false;
    let mut cur: Option<AttachmentReference> = None;
    let mut in_digest_value = false;
    let mut text = String::new();
    loop {
        match r.read_event()? {
            Event::Start(e) | Event::Empty(e) => {
                let local = e.local_name().as_ref().to_string();
                match local.as_str() {
                    "Signature" => in_signature = true,
                    "Reference" if in_signature => {
                        cur = None;
                        for a in e.attributes() {
                            let a = a?;
                            if a.key.as_ref() == "URI" {
                                let uri = a
                                    .normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                                    .into_owned();
                                if uri.starts_with("cid:") {
                                    cur = Some(AttachmentReference {
                                        uri,
                                        digest_method: String::new(),
                                        digest_value: Vec::new(),
                                        has_transforms: false,
                                    });
                                }
                            }
                        }
                    }
                    "Transforms" => {
                        if let Some(c) = cur.as_mut() {
                            c.has_transforms = true;
                        }
                    }
                    "DigestMethod" => {
                        if let Some(c) = cur.as_mut() {
                            for a in e.attributes() {
                                let a = a?;
                                if a.key.as_ref() == "Algorithm" {
                                    c.digest_method = a
                                        .normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                                        .into_owned();
                                }
                            }
                        }
                    }
                    "DigestValue" if cur.is_some() => {
                        in_digest_value = true;
                        text.clear();
                    }
                    _ => {}
                }
            }
            Event::Text(t) if in_digest_value => {
                text.push_str(&t.xml_content(quick_xml::XmlVersion::Explicit1_0))
            }
            Event::End(e) => match e.local_name().as_ref() {
                "DigestValue" if in_digest_value => {
                    in_digest_value = false;
                    if let Some(c) = cur.as_mut() {
                        c.digest_value = B64
                            .decode(text.split_whitespace().collect::<String>())
                            .map_err(|e| Error::msg(format!("DigestValue: {e}")))?;
                    }
                }
                "Reference" => out.extend(cur.take()),
                "Signature" if in_signature => break,
                _ => {}
            },
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(out)
}

/// Read the `KeyInfo` of the first `ds:Signature` in the document.
pub fn first_signature_signer(xml: &[u8]) -> Result<SignerRef> {
    let mut r = NsReader::from_reader(xml);
    let mut in_sig = false;
    let mut in_key_info = false;
    let mut in_cert = false;
    let mut buf = String::new();

    loop {
        match r.read_event()? {
            Event::Start(e) | Event::Empty(e) => match e.local_name().as_ref() {
                "Signature" => in_sig = true,
                "KeyInfo" if in_sig => in_key_info = true,
                "RetrievalMethod" if in_key_info => {
                    for a in e.attributes() {
                        let a = a?;
                        if a.key.as_ref() == "URI" {
                            return Ok(SignerRef::Retrieval(
                                a.normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                                    .into_owned(),
                            ));
                        }
                    }
                }
                "X509Certificate" if in_key_info => {
                    in_cert = true;
                    buf.clear();
                }
                _ => {}
            },
            Event::Text(t) if in_cert => {
                buf.push_str(&t.xml_content(quick_xml::XmlVersion::Explicit1_0))
            }
            Event::End(e) => match e.local_name().as_ref() {
                "X509Certificate" if in_cert => {
                    let der = B64
                        .decode(buf.split_whitespace().collect::<String>())
                        .map_err(|e| Error::msg(format!("KeyInfo certificate: {e}")))?;
                    return Ok(SignerRef::Inline(der));
                }
                "KeyInfo" if in_key_info => {
                    return Err(Error::msg("first signature KeyInfo has no certificate"));
                }
                _ => {}
            },
            Event::Eof => return Err(Error::msg("no ds:Signature found")),
            _ => {}
        }
    }
}

/// The `Id` attribute of `soap:Body`, if it has one.
pub fn body_id(xml: &[u8]) -> Result<Option<String>> {
    let mut r = NsReader::from_reader(xml);
    loop {
        match r.read_event()? {
            Event::Start(e) | Event::Empty(e) if e.local_name().as_ref() == "Body" => {
                for a in e.attributes() {
                    let a = a?;
                    if a.key.as_ref() == "Id" {
                        return Ok(Some(
                            a.normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                                .into_owned(),
                        ));
                    }
                }
                return Ok(None);
            }
            Event::Eof => return Ok(None),
            _ => {}
        }
    }
}

/// Ids of the elements a transport signature covers: the direct children of
/// `soap:Header` and `soap:Body` that carry an `Id` attribute (ControlBlock,
/// DesiredLanguages, the message-type element, NonIntermediaryCertificates, the
/// Body). Signature blocks themselves are excluded.
pub fn signed_element_ids(xml: &[u8]) -> Result<Vec<String>> {
    let mut r = NsReader::from_reader(xml);
    let mut depth = 0usize;
    let mut section_depth: Option<usize> = None;
    let mut out = Vec::new();

    loop {
        let event = r.read_event()?;
        let (bs, is_start) = match &event {
            Event::Start(e) => (Some(e), true),
            Event::Empty(e) => (Some(e), false),
            Event::End(_) => {
                depth = depth.saturating_sub(1);
                continue;
            }
            Event::Eof => break,
            _ => continue,
        };
        let e = bs.unwrap();
        let local = e.local_name().as_ref().to_string();
        let this_depth = depth;
        if is_start {
            depth += 1;
        }

        if matches!(local.as_str(), "Header" | "Body") {
            section_depth = Some(this_depth);
            if local == "Body" {
                for a in e.attributes() {
                    let a = a?;
                    if a.key.as_ref() == "Id" {
                        out.push(
                            a.normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                                .into_owned(),
                        );
                    }
                }
            }
        } else if section_depth == Some(this_depth.wrapping_sub(1))
            && !matches!(local.as_str(), "ClientSignature" | "SupplierSignature")
        {
            for a in e.attributes() {
                let a = a?;
                if a.key.as_ref() == "Id" {
                    out.push(
                        a.normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                            .into_owned(),
                    );
                }
            }
        }
    }
    Ok(out)
}

/// Map every certificate-slot Id in the message to its DER certificate. Keys are
/// the `Id` attribute of the enclosing `osci:*Certificate*` element (the target
/// of a `ds:RetrievalMethod` URI).
pub fn collect(xml: &[u8]) -> Result<HashMap<String, Vec<u8>>> {
    let mut r = NsReader::from_reader(xml);
    let mut out = HashMap::new();
    let mut id_stack: Vec<Option<String>> = Vec::new();
    let mut in_cert_text = false;
    let mut buf = String::new();

    loop {
        match r.read_event()? {
            Event::Start(e) => {
                let mut id = None;
                for a in e.attributes() {
                    let a = a?;
                    if a.key.as_ref() == "Id" {
                        id = Some(
                            a.normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                                .into_owned(),
                        );
                    }
                }
                if e.local_name().as_ref() == "X509Certificate" {
                    in_cert_text = true;
                    buf.clear();
                }
                id_stack.push(id);
            }
            Event::Text(t) if in_cert_text => {
                buf.push_str(&t.xml_content(quick_xml::XmlVersion::Explicit1_0))
            }
            Event::End(e) => {
                if e.local_name().as_ref() == "X509Certificate" {
                    in_cert_text = false;
                    if let Some(slot_id) = id_stack.iter().rev().flatten().next() {
                        let der = B64
                            .decode(buf.split_whitespace().collect::<String>())
                            .unwrap_or_default();
                        if !der.is_empty() {
                            out.entry(slot_id.clone()).or_insert(der);
                        }
                    }
                }
                id_stack.pop();
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(out)
}

/// The role each `osci:{Signature,Cipher}Certificate{Role}` slot names, e.g.
/// `SignatureCertificateOriginator` and `CipherCertificateOtherReader`.
fn slot_role(local: &str) -> Option<(RoleKind, bool)> {
    let (rest, is_signature) = match local.strip_prefix("SignatureCertificate") {
        Some(r) => (r, true),
        None => (local.strip_prefix("CipherCertificate")?, false),
    };
    let kind = RoleKind::ALL
        .iter()
        .copied()
        .find(|k| k.cert_slot_suffix() == rest)?;
    Some((kind, is_signature))
}

/// Read every `osci:{Signature,Cipher}Certificate*` slot in the message and
/// group them into one [`Role`] per role kind.
pub fn role_certificates(xml: &[u8]) -> Result<Vec<Role>> {
    let mut r = NsReader::from_reader(xml);
    let mut roles: Vec<Role> = Vec::new();
    let mut slot_stack: Vec<Option<(RoleKind, bool)>> = Vec::new();
    let mut in_cert_text = false;
    let mut buf = String::new();

    loop {
        match r.read_event()? {
            Event::Start(e) => {
                let local = e.local_name().as_ref().to_string();
                if let Some(slot) = slot_role(&local) {
                    let (kind, _) = slot;
                    if !roles.iter().any(|role| role.kind == kind) {
                        roles.push(Role::new(kind));
                    }
                    slot_stack.push(Some(slot));
                } else {
                    slot_stack.push(None);
                }
                if local == "X509Certificate" {
                    in_cert_text = true;
                    buf.clear();
                }
            }
            Event::Text(t) if in_cert_text => {
                buf.push_str(&t.xml_content(quick_xml::XmlVersion::Explicit1_0))
            }
            Event::End(e) => {
                if e.local_name().as_ref() == "X509Certificate" {
                    in_cert_text = false;
                    if let Some(Some((kind, is_signature))) =
                        slot_stack.iter().rev().find(|s| s.is_some())
                    {
                        let der = B64
                            .decode(buf.split_whitespace().collect::<String>())
                            .unwrap_or_default();
                        if !der.is_empty()
                            && let Some(role) = roles.iter_mut().find(|r| r.kind == *kind)
                        {
                            let slot = if *is_signature {
                                &mut role.certs.signature
                            } else {
                                &mut role.certs.encryption
                            };
                            slot.get_or_insert(der);
                        }
                    }
                }
                slot_stack.pop();
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(roles)
}

/// One `osci:{Signature,Cipher}Certificate*` slot of a message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CertSlotInfo {
    /// Whose certificate the slot holds.
    pub kind: RoleKind,
    /// `true` for a signature certificate, `false` for a cipher certificate.
    pub is_signature: bool,
    /// The slot's `Id` attribute, the target of a `ds:RetrievalMethod`.
    pub id: Option<String>,
    /// The DER-encoded certificate.
    pub cert_der: Vec<u8>,
}

/// Every certificate slot of the message in document order, unlike
/// [`role_certificates`], which keeps one certificate per role.
pub fn cert_slots(xml: &[u8]) -> Result<Vec<CertSlotInfo>> {
    let mut r = NsReader::from_reader(xml);
    let mut out = Vec::new();
    // the slot element being read: its kind, signature flag and Id
    let mut current: Option<(RoleKind, bool, Option<String>)> = None;
    let mut in_cert_text = false;
    let mut buf = String::new();

    loop {
        match r.read_event()? {
            Event::Start(e) => {
                let local = e.local_name().as_ref().to_string();
                if let Some((kind, is_signature)) = slot_role(&local) {
                    let mut id = None;
                    for a in e.attributes() {
                        let a = a?;
                        if a.key.as_ref() == "Id" {
                            id = Some(
                                a.normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                                    .into_owned(),
                            );
                        }
                    }
                    current = Some((kind, is_signature, id));
                } else if local == "X509Certificate" && current.is_some() {
                    in_cert_text = true;
                    buf.clear();
                }
            }
            Event::Text(t) if in_cert_text => {
                buf.push_str(&t.xml_content(quick_xml::XmlVersion::Explicit1_0))
            }
            Event::End(e) => {
                let local = e.local_name().as_ref().to_string();
                if local == "X509Certificate" && in_cert_text {
                    in_cert_text = false;
                    let der = B64
                        .decode(buf.split_whitespace().collect::<String>())
                        .map_err(|e| Error::msg(format!("certificate slot base64: {e}")))?;
                    if let Some((kind, is_signature, id)) = current.take() {
                        out.push(CertSlotInfo {
                            kind,
                            is_signature,
                            id,
                            cert_der: der,
                        });
                    }
                } else if slot_role(&local).is_some() {
                    current = None;
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(out)
}

/// Byte offset and raw bytes of the `osci:ContentContainer` with this `Id`,
/// wherever it sits in the document (top-level or nested).
pub fn find_content_container(xml: &[u8], id: &str) -> Result<(usize, Vec<u8>)> {
    let mut r = NsReader::from_reader(xml);
    loop {
        let before = r.buffer_position() as usize;
        match r.read_event()? {
            Event::Start(e) if e.local_name().as_ref() == "ContentContainer" => {
                let mut this_id = None;
                for a in e.attributes() {
                    let a = a?;
                    if a.key.as_ref() == "Id" {
                        this_id = Some(
                            a.normalized_value(quick_xml::XmlVersion::Explicit1_0)?
                                .into_owned(),
                        );
                    }
                }
                if this_id.as_deref() == Some(id) {
                    let name = e.name().as_ref().to_string();
                    r.read_to_end(XmlQName(&name))?;
                    return Ok((before, xml[before..r.buffer_position() as usize].to_vec()));
                }
            }
            Event::Eof => return Err(Error::msg(format!("content container {id} not found"))),
            _ => {}
        }
    }
}

/// For each `osci:Content` directly in the container `xml`: its own `Id` and the
/// `Id` of its first child element. Nested containers are not entered.
pub fn direct_content_ids(xml: &[u8]) -> Result<Vec<Vec<String>>> {
    use quick_xml::events::BytesStart;
    fn id_of(e: &BytesStart<'_>) -> Result<Option<String>> {
        for a in e.attributes() {
            let a = a?;
            if a.key.as_ref() == "Id" {
                let v = a.normalized_value(quick_xml::XmlVersion::Explicit1_0)?;
                return Ok(Some(v.into_owned()));
            }
        }
        Ok(None)
    }

    let mut r = quick_xml::Reader::from_reader(xml);
    let mut out: Vec<Vec<String>> = Vec::new();
    let mut depth = 0usize;
    // set while we look for the first child of the last Content
    let mut want_child = false;
    loop {
        let ev = r.read_event()?;
        let (start, empty) = match &ev {
            Event::Start(e) => (Some(e), false),
            Event::Empty(e) => (Some(e), true),
            _ => (None, false),
        };
        if let Some(e) = start {
            let is_content = depth == 1 && e.local_name().as_ref() == "Content";
            if is_content {
                out.push(id_of(e)?.into_iter().collect());
                want_child = !empty;
            } else if want_child && depth == 2 {
                out.last_mut().unwrap().extend(id_of(e)?);
                want_child = false;
            }
            if !empty {
                depth += 1;
            }
            continue;
        }
        match ev {
            Event::End(_) => {
                depth = depth.saturating_sub(1);
                if depth <= 1 {
                    want_child = false;
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(out)
}

#[cfg(test)]
mod role_tests {
    use super::*;

    const SAMPLE: &str = concat!(
        r#"<osci:NonIntermediaryCertificates xmlns:osci="http://www.osci.de/2002/04/osci" "#,
        r#"xmlns:ds="http://www.w3.org/2000/09/xmldsig#">"#,
        r#"<osci:SignatureCertificateOriginator Id="sig_orig"><ds:X509Data>"#,
        r#"<ds:X509Certificate>YWJj</ds:X509Certificate></ds:X509Data></osci:SignatureCertificateOriginator>"#,
        r#"<osci:CipherCertificateOriginator Id="ciph_orig"><ds:X509Data>"#,
        r#"<ds:X509Certificate>ZGVm</ds:X509Certificate></ds:X509Data></osci:CipherCertificateOriginator>"#,
        r#"<osci:SignatureCertificateIntermediary Id="sig_int"><ds:X509Data>"#,
        r#"<ds:X509Certificate>Z2hp</ds:X509Certificate></ds:X509Data></osci:SignatureCertificateIntermediary>"#,
        r#"</osci:NonIntermediaryCertificates>"#,
    );

    #[test]
    fn groups_slots_by_role() {
        let roles = role_certificates(SAMPLE.as_bytes()).unwrap();
        assert_eq!(roles.len(), 2);

        let originator = roles
            .iter()
            .find(|r| r.kind == RoleKind::Originator)
            .unwrap();
        assert_eq!(
            originator.certs.signature.as_deref(),
            Some(b"abc".as_slice())
        );
        assert_eq!(
            originator.certs.encryption.as_deref(),
            Some(b"def".as_slice())
        );

        let intermediary = roles
            .iter()
            .find(|r| r.kind == RoleKind::Intermediary)
            .unwrap();
        assert_eq!(
            intermediary.certs.signature.as_deref(),
            Some(b"ghi".as_slice())
        );
        assert_eq!(intermediary.certs.encryption, None);
    }
}
