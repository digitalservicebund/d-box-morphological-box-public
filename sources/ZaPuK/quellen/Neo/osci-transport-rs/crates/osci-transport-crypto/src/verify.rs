// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::collections::{HashMap, HashSet};

use bergshamra::keys::loader::load_x509_cert_der;
use bergshamra::{DsigContext, KeysManager, VerifyResult, verify, verify_all};
use osci_transport::certs::{self, AttachmentReference, SignerRef};
use osci_transport::crypto::uri;
use osci_transport::error::{Error, Result};
use osci_transport::mime::Attachment;
use osci_transport::wire::parse_content_container;

/// Result of verifying an OSCI transport signature (the first `ds:Signature` in
/// the envelope: `ClientSignature`, `SupplierSignature` or
/// `IntermediarySignature`).
#[derive(Clone, Debug)]
pub struct SignatureReport {
    /// Whether the signature verified.
    pub valid: bool,
    /// Why it failed, if it did.
    pub reason: Option<String>,
    /// The signer certificate (DER), resolved from the message.
    pub signer_cert_der: Vec<u8>,
    /// Same-document references whose digest was checked.
    pub verified_refs: Vec<String>,
    /// `cid:` references; the parts are not hashed by the verification itself,
    /// see [`SignatureReport::check_attachments`].
    pub attachment_refs: Vec<String>,
    /// The digest the signature gives for each `cid:` reference. Empty unless
    /// the signature is valid.
    pub attachment_digests: Vec<AttachmentReference>,
}

impl SignatureReport {
    /// Check the received MIME parts against the signature. Call it after a
    /// valid verification, with the parts of the same message. Every
    /// `cid:` reference needs a part with that `Content-ID` whose decoded bytes
    /// hash to the signed digest, and every part must be covered by a
    /// reference: an unsigned part could have been swapped by anyone.
    ///
    /// For a message that arrived transport-encrypted, pass the parts of the
    /// decrypted message.
    pub fn check_attachments(&self, attachments: &[Attachment]) -> Result<()> {
        use sha2::Digest;
        if !self.valid {
            return Err(Error::msg(
                "the signature is not valid, so its attachment digests mean nothing",
            ));
        }
        // With two parts under one id the digest check would see only the
        // first, and a reader that keeps the last would use an unchecked one.
        let mut seen = HashSet::new();
        for id in attachments.iter().filter_map(|a| a.content_id.as_deref()) {
            if !seen.insert(id) {
                return Err(Error::msg(format!("two attachments share Content-ID {id}")));
            }
        }
        for reference in &self.attachment_digests {
            let id = reference.uri.strip_prefix("cid:").unwrap_or(&reference.uri);
            if reference.has_transforms {
                return Err(Error::msg(format!(
                    "reference {} has transforms, its digest cannot be checked",
                    reference.uri
                )));
            }
            let part = attachments
                .iter()
                .find(|a| a.content_id.as_deref() == Some(id))
                .ok_or_else(|| {
                    Error::msg(format!(
                        "the signature covers {} but the message has no such part",
                        reference.uri
                    ))
                })?;
            let hash = match reference.digest_method.as_str() {
                uri::DIGEST_SHA1 => sha1::Sha1::digest(&part.data).to_vec(),
                uri::DIGEST_SHA256 => sha2::Sha256::digest(&part.data).to_vec(),
                uri::DIGEST_SHA512 => sha2::Sha512::digest(&part.data).to_vec(),
                other => {
                    return Err(Error::msg(format!(
                        "unsupported digest method {other} for {}",
                        reference.uri
                    )));
                }
            };
            if hash != reference.digest_value {
                return Err(Error::msg(format!(
                    "attachment {id} does not match its signed digest"
                )));
            }
        }
        if let Some(unsigned) = attachments.iter().find(|a| {
            !self
                .attachment_digests
                .iter()
                .any(|r| r.uri.strip_prefix("cid:") == a.content_id.as_deref())
        }) {
            return Err(Error::msg(format!(
                "attachment {:?} is not covered by the signature",
                unsigned.content_id
            )));
        }
        Ok(())
    }
}

/// Verify the transport signature. Its signer certificate is resolved from the
/// message (an inline `ds:X509Certificate`, or a `ds:RetrievalMethod` pointing
/// at an `osci:*Certificate*` slot) and passed to `trust`; verification only
/// proceeds if `trust` returns true.
pub fn verify_transport_signature(
    envelope_xml: &[u8],
    trust: impl Fn(&[u8]) -> bool,
) -> Result<SignatureReport> {
    let xml = std::str::from_utf8(envelope_xml)
        .map_err(|_| Error::msg("envelope is not utf-8"))?
        .trim();

    let cert = resolve_signer_cert(xml, xml)?;
    if !trust(&cert) {
        return Ok(untrusted_report(cert));
    }

    let ctx = permissive_context(&cert)?;
    let result = verify(&ctx, xml).map_err(|e| Error::msg(format!("verify: {e}")))?;
    let report = with_attachment_digests(to_report(result, cert), xml)?;
    require_full_coverage(report, xml)
}

// A transport signature covers every header block next to it and the Body
// (spec 6.4). A valid signature that leaves one of them out says nothing about
// the content a receiver then reads from the unsigned part.
fn require_full_coverage(mut report: SignatureReport, xml: &str) -> Result<SignatureReport> {
    if !report.valid {
        return Ok(report);
    }
    let signed: HashSet<&str> = report
        .verified_refs
        .iter()
        .map(|r| r.strip_prefix('#').unwrap_or(r))
        .collect();
    let mut expected = certs::signed_element_ids(xml.as_bytes())?;
    match certs::body_id(xml.as_bytes())? {
        Some(id) => expected.push(id),
        None => {
            report.valid = false;
            report.reason = Some("soap:Body has no Id, so it cannot be signed".into());
            return Ok(report);
        }
    }
    if let Some(missing) = expected.iter().find(|id| !signed.contains(id.as_str())) {
        report.reason = Some(format!("element {missing} is not covered by the signature"));
        report.valid = false;
    }
    Ok(report)
}

/// Verify the `ds:Signature` inside one `osci:ContentContainer`. Its signer
/// certificate is resolved the same way as [`verify_transport_signature`], but
/// the `osci:*Certificate*` slot lookup covers the whole envelope, since
/// `osci:NonIntermediaryCertificates` sits in `soap:Header` while the container
/// lives in `soap:Body`.
///
/// The container must not itself hold a signed, nested container: only the
/// signature directly on `container_id` is checked. The correct `verify_all()`
/// result is picked by matching its References against this container's Content
/// ids; if no result matches (an unsigned or genuinely broken document), this
/// falls back to a document-order guess rather than erroring, so an `Invalid`
/// result can still surface a reason.
pub fn verify_content_signature(
    envelope_xml: &[u8],
    container_id: &str,
    trust: impl Fn(&[u8]) -> bool,
) -> Result<SignatureReport> {
    let xml = std::str::from_utf8(envelope_xml)
        .map_err(|_| Error::msg("envelope is not utf-8"))?
        .trim();

    let (offset, raw) = certs::find_content_container(xml.as_bytes(), container_id)?;
    let raw = std::str::from_utf8(&raw)
        .map_err(|_| Error::msg("content container is not utf-8"))?
        .to_string();

    let cert = resolve_signer_cert(&raw, xml)?;
    if !trust(&cert) {
        return Ok(untrusted_report(cert));
    }

    let ctx = permissive_context(&cert)?;

    // ds:Signature elements before this container's own signature, in document
    // order, are a first guess at which verify_all() result is ours -- but that
    // count assumes the "ds:" prefix and can't see inside comments/CDATA, so it
    // is only a fallback. The real check: this container's References target its
    // own Content ids (and cid: attachments), so the right result is the one
    // whose references actually hit those targets.
    let index = xml[..offset].matches("<ds:Signature>").count();
    let expected = content_reference_targets(&raw)?;
    let results = verify_all(&ctx, xml).map_err(|e| Error::msg(format!("verify: {e}")))?;
    let picked = results
        .iter()
        .position(|r| references_container(r, &expected))
        .unwrap_or(index);
    let result = results
        .into_iter()
        .nth(picked)
        .ok_or_else(|| Error::msg("content signature not found in document"))?;
    let report = with_attachment_digests(to_report(result, cert), &raw)?;
    require_contents_covered(report, &raw)
}

// A signature that leaves one Content of the container out would still report
// valid, and the unsigned Content could have been added by anyone. Java signs
// the element inside a Content (`osci:Base64Content Id="content0"`) as often
// as the Content itself, so either id counts.
fn require_contents_covered(mut report: SignatureReport, raw: &str) -> Result<SignatureReport> {
    if !report.valid {
        return Ok(report);
    }
    let signed: HashSet<&str> = report
        .verified_refs
        .iter()
        .map(|r| r.strip_prefix('#').unwrap_or(r))
        .collect();
    let unsigned = certs::direct_content_ids(raw.as_bytes())?
        .into_iter()
        .find(|ids| !ids.iter().any(|id| signed.contains(id.as_str())));
    if let Some(ids) = unsigned {
        report.valid = false;
        report.reason = Some(match ids.first() {
            Some(id) => format!("content {id} is not covered by the signature"),
            None => "a content without an Id cannot be covered by the signature".into(),
        });
    }
    Ok(report)
}

// The digests a valid signature in `scope_xml` gives for its `cid:` parts.
fn with_attachment_digests(
    mut report: SignatureReport,
    scope_xml: &str,
) -> Result<SignatureReport> {
    if report.valid && !report.attachment_refs.is_empty() {
        report.attachment_digests = certs::attachment_references(scope_xml.as_bytes())?;
    }
    Ok(report)
}

// Ids this container's References should target: each Content's own Id.
// Deliberately excludes cid: attachment targets -- bergshamra's verify_all
// reports every document-wide cid: reference against each signature's result
// (it resolves attachments once, context-wide), so a cid: match alone cannot
// tell two signatures apart.
fn content_reference_targets(raw_container_xml: &str) -> Result<HashSet<String>> {
    let cc = parse_content_container(raw_container_xml.as_bytes())?;
    Ok(cc.contents.iter().filter_map(|c| c.id.clone()).collect())
}

fn references_container(result: &VerifyResult, expected: &HashSet<String>) -> bool {
    match result {
        VerifyResult::Valid { references, .. } => references
            .iter()
            .any(|r| expected.contains(r.uri.strip_prefix('#').unwrap_or(&r.uri))),
        VerifyResult::Invalid { .. } => false,
    }
}

fn permissive_context(cert: &[u8]) -> Result<DsigContext> {
    let key =
        load_x509_cert_der(cert).map_err(|e| Error::msg(format!("signer certificate: {e}")))?;
    let mut km = KeysManager::new();
    km.add_key(key);

    let mut ctx = DsigContext::new_permissive(km);
    ctx.verify_keys = false;
    ctx.trusted_keys_only = false;
    ctx.require_reference_digests = false;
    Ok(ctx)
}

fn untrusted_report(cert: Vec<u8>) -> SignatureReport {
    SignatureReport {
        valid: false,
        reason: Some("signer certificate is not trusted".into()),
        signer_cert_der: cert,
        verified_refs: Vec::new(),
        attachment_refs: Vec::new(),
        attachment_digests: Vec::new(),
    }
}

fn to_report(r: VerifyResult, cert: Vec<u8>) -> SignatureReport {
    match r {
        VerifyResult::Valid { references, .. } => {
            let (v, a) = references
                .into_iter()
                .partition::<Vec<_>, _>(|x| x.digest_verified);
            SignatureReport {
                valid: true,
                reason: None,
                signer_cert_der: cert,
                verified_refs: v.into_iter().map(|x| x.uri).collect(),
                attachment_refs: a.into_iter().map(|x| x.uri).collect(),
                attachment_digests: Vec::new(),
            }
        }
        VerifyResult::Invalid { reason } => SignatureReport {
            valid: false,
            reason: Some(reason),
            signer_cert_der: cert,
            verified_refs: Vec::new(),
            attachment_refs: Vec::new(),
            attachment_digests: Vec::new(),
        },
    }
}

// The certificate for the first ds:Signature within `scope_xml`. Slot ids are
// looked up in `cert_lookup_xml`, which may be a larger document that
// `scope_xml` is a fragment of.
fn resolve_signer_cert(scope_xml: &str, cert_lookup_xml: &str) -> Result<Vec<u8>> {
    match certs::first_signature_signer(scope_xml.as_bytes())? {
        SignerRef::Inline(der) => Ok(der),
        SignerRef::Retrieval(uri) => {
            let id = uri.strip_prefix('#').unwrap_or(&uri);
            let slots = certs::collect(cert_lookup_xml.as_bytes())?;
            slots
                .get(id)
                .or_else(|| {
                    // the Java library matches by suffix too; with several
                    // matches the longest id wins, so the HashMap order does
                    // not decide which certificate is used
                    slots
                        .iter()
                        .filter(|(k, _)| uri.ends_with(k.as_str()))
                        .max_by_key(|(k, _)| k.len())
                        .map(|(_, v)| v)
                })
                .cloned()
                .ok_or_else(|| Error::msg(format!("certificate slot {uri} not found")))
        }
    }
}

/// Map each certificate-slot Id in the message to its DER certificate.
pub fn slot_certificates(envelope_xml: &[u8]) -> Result<HashMap<String, Vec<u8>>> {
    certs::collect(envelope_xml)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot(id: &str, cert_b64: &str) -> String {
        format!(
            r#"<osci:Slot Id="{id}"><ds:X509Data><ds:X509Certificate>{cert_b64}</ds:X509Certificate></ds:X509Data></osci:Slot>"#
        )
    }

    #[test]
    fn longest_matching_slot_id_wins() {
        let xml = format!(
            r##"<soap:Envelope xmlns:soap="s" xmlns:osci="o" xmlns:ds="d"><soap:Header>{}{}<ds:Signature><ds:KeyInfo><ds:RetrievalMethod URI="#x-signaturecert"/></ds:KeyInfo></ds:Signature></soap:Header></soap:Envelope>"##,
            slot("cert", "AAAA"),
            slot("signaturecert", "BBBB"),
        );
        // repeated, as the slot map is a HashMap
        for _ in 0..20 {
            assert_eq!(
                resolve_signer_cert(&xml, &xml).unwrap(),
                vec![0x04, 0x10, 0x41]
            );
        }
    }
}
