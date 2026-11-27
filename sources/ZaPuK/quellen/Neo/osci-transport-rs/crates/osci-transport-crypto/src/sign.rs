// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use bergshamra::keys::loader::load_pem_auto;
use bergshamra::{DsigContext, KeysManager, sign};
use osci_transport::certs::signed_element_ids;
use osci_transport::crypto::SignatureAlg;
use osci_transport::error::{Error, Result};
use osci_transport::mime::Attachment;
use osci_transport::role::RoleKind;

const C14N: &str = "http://www.w3.org/TR/2001/REC-xml-c14n-20010315";

/// The `ds:DigestMethod` a signature's own [`SignatureAlg`] implies for its
/// `ds:Reference`s.
fn digest_uri_for(alg: SignatureAlg) -> &'static str {
    use osci_transport::crypto::uri::{DIGEST_SHA1, DIGEST_SHA256, DIGEST_SHA512};
    match alg {
        SignatureAlg::RsaSha1 => DIGEST_SHA1,
        SignatureAlg::RsaSha256 | SignatureAlg::RsaSha256Pss | SignatureAlg::EcdsaSha256 => {
            DIGEST_SHA256
        }
        SignatureAlg::RsaSha512 | SignatureAlg::RsaSha512Pss | SignatureAlg::EcdsaSha512 => {
            DIGEST_SHA512
        }
    }
}

/// Which transport signature block to add.
#[derive(Clone, Copy, Debug)]
pub enum Role {
    /// `osci:ClientSignature` (originator side).
    Client,
    /// `osci:SupplierSignature` (intermediary side).
    Supplier,
}

impl Role {
    fn block(self) -> (&'static str, &'static str) {
        match self {
            Role::Client => ("ClientSignature", "clientsignature"),
            Role::Supplier => ("SupplierSignature", "suppliersignature"),
        }
    }

    /// Which [`RoleKind`] this signature role's certificate is filed under in
    /// `osci:NonIntermediaryCertificates`.
    fn signer_kind(self) -> RoleKind {
        match self {
            Role::Client => RoleKind::Originator,
            Role::Supplier => RoleKind::Intermediary,
        }
    }
}

/// One additional certificate slot to embed in `osci:NonIntermediaryCertificates`,
/// alongside the primary signer's own certificate. Real Intermediäre need these
/// to route and, for `is_signature: false`, to encrypt responses back to the
/// party the certificate belongs to - e.g. a client sending an encrypted
/// request must include its own cipher certificate here, or the Intermediär
/// has no key to encrypt the response with and rejects the message.
#[derive(Clone, Debug)]
pub struct CertSlot {
    /// Which party this certificate belongs to.
    pub kind: RoleKind,
    /// `true` for a `SignatureCertificate*` slot, `false` for `CipherCertificate*`.
    pub is_signature: bool,
    /// The DER-encoded certificate.
    pub cert_der: Vec<u8>,
}

impl CertSlot {
    /// The Originator's cipher certificate (`CipherCertificateOriginator`).
    /// Needed on every encrypted request: the Intermediär can't encrypt a
    /// response back to a client whose cipher cert it was never given.
    pub fn originator_cipher(cert_der: impl Into<Vec<u8>>) -> Self {
        CertSlot {
            kind: RoleKind::Originator,
            is_signature: false,
            cert_der: cert_der.into(),
        }
    }

    /// The `Id` this slot gets in the message header. A
    /// `ds:RetrievalMethod` URI refers to it as `#<id>`.
    pub fn id(&self) -> String {
        slot_id(self.kind, self.is_signature)
    }

    fn element_name(&self) -> String {
        element_name(self.kind, self.is_signature)
    }
}

fn slot_id(kind: RoleKind, is_signature: bool) -> String {
    format!(
        "{}_{}",
        kind.cert_slot_suffix().to_ascii_lowercase(),
        if is_signature { "signature" } else { "cipher" }
    )
}

fn element_name(kind: RoleKind, is_signature: bool) -> String {
    format!(
        "{}Certificate{}",
        if is_signature { "Signature" } else { "Cipher" },
        kind.cert_slot_suffix()
    )
}

/// Add a transport signature to an unsigned OSCI envelope, using
/// [`sign_envelope_with`] with no additional certificate slots. `signer_key_pem`
/// is the RSA private key. The signer certificate is placed in an
/// `osci:NonIntermediaryCertificates` block and referenced from the signature
/// `KeyInfo`. The signature covers every Id-bearing header block and the body.
pub fn sign_envelope(
    envelope_xml: &[u8],
    role: Role,
    signer_key_pem: &[u8],
    signer_cert_der: &[u8],
) -> Result<Vec<u8>> {
    sign_envelope_with(envelope_xml, role, signer_key_pem, signer_cert_der, &[])
}

/// [`sign_envelope`] with additional [`CertSlot`]s embedded in the same
/// `osci:NonIntermediaryCertificates` block (not referenced by the signature's
/// `KeyInfo`, but covered by its digest along with everything else in the
/// block). Duplicates of the primary signer's own slot (same `RoleKind` and
/// `is_signature`) are skipped. Signs with plain RSA-SHA256, the algorithm the
/// vendored corpus and the live public test Intermediär both accept; use
/// [`sign_envelope_with_alg`] to sign with a different [`SignatureAlg`] or an
/// EC key.
pub fn sign_envelope_with(
    envelope_xml: &[u8],
    role: Role,
    signer_key_pem: &[u8],
    signer_cert_der: &[u8],
    extra_certs: &[CertSlot],
) -> Result<Vec<u8>> {
    sign_envelope_with_alg(
        envelope_xml,
        role,
        signer_key_pem,
        signer_cert_der,
        extra_certs,
        SignatureAlg::RsaSha256,
    )
}

/// [`sign_envelope_with`] with an explicit [`SignatureAlg`], covering both the
/// `ds:SignatureMethod`/`ds:DigestMethod` URIs and the signing key: RSA
/// variants load `signer_key_pem` as an RSA key, `Ecdsa*` variants load it as
/// an EC key (curve implied by the algorithm - P-256 for `EcdsaSha256`, P-521
/// for `EcdsaSha512`). bergshamra rejects a key/algorithm mismatch itself.
pub fn sign_envelope_with_alg(
    envelope_xml: &[u8],
    role: Role,
    signer_key_pem: &[u8],
    signer_cert_der: &[u8],
    extra_certs: &[CertSlot],
    alg: SignatureAlg,
) -> Result<Vec<u8>> {
    sign_envelope_with_attachments(
        envelope_xml,
        role,
        signer_key_pem,
        signer_cert_der,
        extra_certs,
        alg,
        &[],
    )
}

/// [`sign_envelope_with_alg`] that also signs MIME attachments, the way the
/// Java osci-bibliothek does: one `ds:Reference URI="cid:<id>"` per
/// attachment, with no transforms, and a digest over the decoded bytes using
/// the signature's own digest algorithm. bergshamra does not hash `cid:`
/// targets, so the digests are computed here. Every attachment needs a
/// `Content-ID`, and ids must be unique.
pub fn sign_envelope_with_attachments(
    envelope_xml: &[u8],
    role: Role,
    signer_key_pem: &[u8],
    signer_cert_der: &[u8],
    extra_certs: &[CertSlot],
    alg: SignatureAlg,
    attachments: &[Attachment],
) -> Result<Vec<u8>> {
    let attached = attachment_digests(attachments, alg)?;
    let xml = std::str::from_utf8(envelope_xml).map_err(|_| Error::msg("envelope is not utf-8"))?;

    let signer_kind = role.signer_kind();
    let mut slots = vec![CertSlot {
        kind: signer_kind,
        is_signature: true,
        cert_der: signer_cert_der.to_vec(),
    }];
    for extra in extra_certs {
        if extra.kind == signer_kind && extra.is_signature {
            continue; // already the primary signer's own slot
        }
        slots.push(extra.clone());
    }
    // The official Java osci-bibliothek always writes every
    // CipherCertificate* slot before any SignatureCertificate* slot,
    // regardless of role. A stable sort keeps each side's own relative
    // order otherwise.
    slots.sort_by_key(|s| s.is_signature);
    let signer_slot_id = slot_id(signer_kind, true);

    // 1. add the NonIntermediaryCertificates block before </soap:Header>, or
    //    before FeatureDescription if the envelope has one - the official
    //    Java osci-bibliothek always writes NonIntermediaryCertificates
    //    ahead of FeatureDescription, which is otherwise the last header
    //    child.
    // A Supplier's own certificate goes in osci:IntermediaryCertificates, a
    // block of its own ahead of the NonIntermediaryCertificates. The Java
    // library refuses a SignatureCertificateIntermediary in the latter.
    let cert_block = if signer_kind == RoleKind::Intermediary {
        let (own, others): (Vec<CertSlot>, Vec<CertSlot>) = slots
            .into_iter()
            .partition(|s| s.kind == signer_kind && s.is_signature);
        let mut block =
            certificates_block("IntermediaryCertificates", "intermediarycertificates", &own);
        if !others.is_empty() {
            block.push_str(&certificates_block(
                "NonIntermediaryCertificates",
                "nonintermediarycertificates",
                &others,
            ));
        }
        block
    } else {
        certificates_block(
            "NonIntermediaryCertificates",
            "nonintermediarycertificates",
            &slots,
        )
    };
    let insert_at = xml
        .find("<osci2017:FeatureDescription")
        .or_else(|| xml.find("</soap:Header>"))
        .ok_or_else(|| Error::msg("envelope has no soap:Header"))?;
    let mut with_certs = String::with_capacity(xml.len() + cert_block.len());
    with_certs.push_str(&xml[..insert_at]);
    with_certs.push_str(&cert_block);
    with_certs.push_str(&xml[insert_at..]);

    // 2. build the signature template covering every Id-bearing element, in
    //    physical document order.
    let ids = signed_element_ids(with_certs.as_bytes())?;
    if ids.is_empty() {
        return Err(Error::msg("nothing to sign: no Id-bearing elements"));
    }
    let (block, block_id) = role.block();
    let template = signature_block(block, block_id, &ids, &attached, &signer_slot_id, alg);

    let anchor = "</osci:ControlBlock>";
    let pos = with_certs
        .find(anchor)
        .ok_or_else(|| Error::msg("envelope has no ControlBlock to anchor the signature"))?
        + anchor.len();
    let mut with_template = String::with_capacity(with_certs.len() + template.len());
    with_template.push_str(&with_certs[..pos]);
    with_template.push_str(&template);
    with_template.push_str(&with_certs[pos..]);

    let key =
        load_pem_auto(signer_key_pem, None).map_err(|e| Error::msg(format!("signer key: {e}")))?;
    let mut km = KeysManager::new();
    km.add_key(key);
    let ctx = DsigContext::new_permissive(km);

    let signed = sign(&ctx, &with_template).map_err(|e| Error::msg(format!("sign: {e}")))?;
    Ok(signed.into_bytes())
}

fn certificates_block(tag: &str, block_id: &str, slots: &[CertSlot]) -> String {
    let mut entries = String::new();
    for slot in slots {
        entries.push_str(&format!(
            concat!(
                r#"<osci:{elem} Id="{id}"><ds:X509Data>"#,
                r#"<ds:X509Certificate>{cert}</ds:X509Certificate></ds:X509Data></osci:{elem}>"#,
            ),
            elem = slot.element_name(),
            id = slot.id(),
            cert = B64.encode(&slot.cert_der),
        ));
    }
    format!(
        concat!(
            r#"<osci:{tag} xmlns:ds="http://www.w3.org/2000/09/xmldsig#" "#,
            r#"xmlns:osci="http://www.osci.de/2002/04/osci" "#,
            r#"xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" "#,
            r#"xmlns:xenc="http://www.w3.org/2001/04/xmlenc#" "#,
            r#"xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" "#,
            r#"Id="{block_id}" "#,
            r#"soap:actor="http://www.w3.org/2001/12/soap-envelope/actor/none" "#,
            r#"soap:mustUnderstand="1">{entries}</osci:{tag}>"#,
        ),
        tag = tag,
        block_id = block_id,
        entries = entries,
    )
}

fn signature_block(
    block: &str,
    block_id: &str,
    ids: &[String],
    attached: &[(String, String)],
    signer_slot_id: &str,
    alg: SignatureAlg,
) -> String {
    let digest = digest_uri_for(alg);
    let mut refs = String::new();
    // Java lists the attachments after the message's own header blocks and
    // ahead of the certificate blocks
    let mut pending = Some(attached);
    for id in ids {
        if id.ends_with("certificates")
            && let Some(att) = pending.take()
        {
            refs.push_str(&attachment_references(att, digest));
        }
        refs.push_str(&format!(
            concat!(
                r##"<ds:Reference URI="#{id}">"##,
                r#"<ds:Transforms><ds:Transform Algorithm="{c14n}"></ds:Transform></ds:Transforms>"#,
                r#"<ds:DigestMethod Algorithm="{digest}"></ds:DigestMethod>"#,
                r#"<ds:DigestValue></ds:DigestValue></ds:Reference>"#,
            ),
            id = id,
            c14n = C14N,
            digest = digest,
        ));
    }
    if let Some(att) = pending {
        refs.push_str(&attachment_references(att, digest));
    }
    format!(
        concat!(
            r#"<osci:{block} xmlns:ds="http://www.w3.org/2000/09/xmldsig#" "#,
            r#"xmlns:osci="http://www.osci.de/2002/04/osci" "#,
            r#"xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" "#,
            r#"xmlns:xenc="http://www.w3.org/2001/04/xmlenc#" "#,
            r#"xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" "#,
            r#"Id="{block_id}" soap:actor="http://schemas.xmlsoap.org/soap/actor/next" "#,
            r#"soap:mustUnderstand="1">"#,
            r#"<ds:Signature><ds:SignedInfo>"#,
            r#"<ds:CanonicalizationMethod Algorithm="{c14n}"></ds:CanonicalizationMethod>"#,
            r#"<ds:SignatureMethod Algorithm="{sig}"></ds:SignatureMethod>"#,
            "{refs}",
            r#"</ds:SignedInfo><ds:SignatureValue></ds:SignatureValue>"#,
            r##"<ds:KeyInfo><ds:RetrievalMethod URI="#{slot}"></ds:RetrievalMethod></ds:KeyInfo>"##,
            r#"</ds:Signature></osci:{block}>"#,
        ),
        block = block,
        block_id = block_id,
        c14n = C14N,
        sig = alg.uri(),
        refs = refs,
        slot = signer_slot_id,
    )
}

fn attachment_references(attached: &[(String, String)], digest: &str) -> String {
    attached
        .iter()
        .map(|(id, value)| {
            format!(
                concat!(
                    r#"<ds:Reference URI="cid:{id}">"#,
                    r#"<ds:DigestMethod Algorithm="{digest}"></ds:DigestMethod>"#,
                    r#"<ds:DigestValue>{value}</ds:DigestValue></ds:Reference>"#,
                ),
                id = id,
                digest = digest,
                value = value,
            )
        })
        .collect()
}

/// `(Content-ID, base64 digest)` for each attachment.
fn attachment_digests(
    attachments: &[Attachment],
    alg: SignatureAlg,
) -> Result<Vec<(String, String)>> {
    use sha2::Digest;
    let mut out: Vec<(String, String)> = Vec::with_capacity(attachments.len());
    for a in attachments {
        let id = a
            .content_id
            .as_deref()
            .ok_or_else(|| Error::msg("attachment has no Content-ID"))?;
        // the id goes into an attribute value
        if id.is_empty() || id.contains(['"', '<', '>', '&', '\r', '\n']) {
            return Err(Error::msg(format!("invalid attachment Content-ID {id:?}")));
        }
        if out.iter().any(|(seen, _)| seen == id) {
            return Err(Error::msg(format!("duplicate attachment Content-ID {id}")));
        }
        let hash = match digest_uri_for(alg) {
            osci_transport::crypto::uri::DIGEST_SHA1 => sha1::Sha1::digest(&a.data).to_vec(),
            osci_transport::crypto::uri::DIGEST_SHA512 => sha2::Sha512::digest(&a.data).to_vec(),
            _ => sha2::Sha256::digest(&a.data).to_vec(),
        };
        out.push((id.to_string(), B64.encode(hash)));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_matches_the_signature_algorithms_own_hash() {
        assert_eq!(
            digest_uri_for(SignatureAlg::RsaSha1),
            osci_transport::crypto::uri::DIGEST_SHA1
        );
        for alg in [
            SignatureAlg::RsaSha256,
            SignatureAlg::RsaSha256Pss,
            SignatureAlg::EcdsaSha256,
        ] {
            assert_eq!(
                digest_uri_for(alg),
                osci_transport::crypto::uri::DIGEST_SHA256
            );
        }
        for alg in [
            SignatureAlg::RsaSha512,
            SignatureAlg::RsaSha512Pss,
            SignatureAlg::EcdsaSha512,
        ] {
            assert_eq!(
                digest_uri_for(alg),
                osci_transport::crypto::uri::DIGEST_SHA512
            );
        }
    }

    #[test]
    fn signature_block_uses_the_chosen_algorithms_uri() {
        let block = signature_block(
            "ClientSignature",
            "clientsignature",
            &["body".to_string()],
            &[],
            "originator_signature",
            SignatureAlg::EcdsaSha256,
        );
        assert!(block.contains(SignatureAlg::EcdsaSha256.uri()));
        assert!(!block.contains(SignatureAlg::RsaSha256.uri()));
    }
}
