// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! Client-side OSCI: a [`Client`] that builds, signs, encrypts and sends
//! requests to an Intermediär over a pluggable [`Transport`], and tracks the
//! dialog.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use osci_transport::crypto::Decrypter;
use osci_transport::dialog::ClientDialog;
use osci_transport::envelope::{Envelope, Sniff, parse, sniff};
use osci_transport::error::{Error, Result};
use osci_transport::header::{DesiredLanguages, QualityOfTimestamp};
use osci_transport::message::MessageType;
use osci_transport::mime::Entity;
use osci_transport::parts::{ContentPackage, Feedback};
use osci_transport::process_card::ProcessCardBundle;
use osci_transport::router::BoxFuture;
use osci_transport::wire::{Request, RequestEnvelope};
use osci_transport_crypto::decrypt_transport;
use zeroize::Zeroizing;

#[cfg(feature = "http")]
mod http;
#[cfg(feature = "http")]
pub use http::HttpTransport;
pub use osci_transport::mime::Attachment;

type TrustFn = Box<dyn Fn(&[u8]) -> bool + Send + Sync>;

/// Sends one request body to the Intermediär and returns the response body.
pub trait Transport {
    /// POST `body` and return the response bytes.
    fn post(&self, body: Vec<u8>) -> BoxFuture<'_, Result<Vec<u8>>>;
}

/// The decoded response to one request.
pub struct ClientResponse {
    /// The `responseTo*` message type.
    pub message_type: MessageType,
    /// Feedback entries from the response.
    pub feedback: Vec<Feedback>,
    /// The full parsed envelope.
    pub envelope: Envelope,
    /// The MIME attachments, in order, as `cid:` references point to them.
    /// Empty when the response has none.
    pub attachments: Vec<Attachment>,
    /// The envelope XML as received, after transport decryption.
    pub envelope_xml: Vec<u8>,
    /// The MIME message after transport decryption, as received.
    pub message: Vec<u8>,
}

impl ClientResponse {
    /// The process cards in this response: those of a
    /// `ResponseToFetchProcessCard`, empty for other message types.
    pub fn process_cards(&self) -> osci_transport::error::Result<Vec<ProcessCardBundle>> {
        osci_transport::process_card::parse_process_card_bundles(&self.envelope.body)
    }
}

struct Signer {
    // wiped when the client is dropped
    key_pem: Zeroizing<Vec<u8>>,
    cert_der: Vec<u8>,
}

/// An OSCI client over transport `T`. Build it with [`Client::new`] and the
/// `with_*` / `encrypt_for` / `explicit_dialog` builders, then call [`send`](Client::send).
pub struct Client<T> {
    transport: T,
    dialog: ClientDialog,
    decrypter: Option<Box<dyn Decrypter + Send + Sync>>,
    signer: Option<Signer>,
    intermediary_cert: Option<Vec<u8>>,
    encrypt_options: osci_transport_crypto::EncryptOptions,
    desired_languages: DesiredLanguages,
    response_trust: Option<TrustFn>,
    accept_unverified: bool,
}

impl<T: Transport> Client<T> {
    /// A client with an implicit dialog, no signing, no encryption. Every
    /// request carries an `osci:DesiredLanguages` header of `["de"]` by
    /// default (the official Java osci-bibliothek always sends this header; some
    /// Intermediäre reject a request that's missing it) - override with
    /// [`Client::with_desired_languages`].
    pub fn new(transport: T) -> Self {
        Client {
            transport,
            dialog: ClientDialog::implicit(),
            decrypter: None,
            signer: None,
            intermediary_cert: None,
            encrypt_options: osci_transport_crypto::EncryptOptions::default(),
            desired_languages: DesiredLanguages {
                id: Some("desiredlanguages".into()),
                languages: vec!["de".into()],
            },
            response_trust: None,
            accept_unverified: false,
        }
    }

    /// Override the languages sent in every request's `osci:DesiredLanguages`
    /// header, most preferred first.
    pub fn with_desired_languages<I, S>(mut self, langs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.desired_languages = DesiredLanguages {
            id: Some("desiredlanguages".into()),
            languages: langs.into_iter().map(Into::into).collect(),
        };
        self
    }

    /// Encrypt every request for the Intermediär with this certificate, using
    /// OSCI 1.2's modern default algorithms (AES-256-GCM + RSA-OAEP-SHA256).
    /// Use [`Client::encrypt_for_with`] instead for an Intermediär that needs
    /// a different combination (e.g. the older AES-256-CBC + RSA-PKCS1v1.5
    /// some real-world deployments are still limited to).
    pub fn encrypt_for(self, intermediary_cert_der: impl Into<Vec<u8>>) -> Self {
        self.encrypt_for_with(
            intermediary_cert_der,
            osci_transport_crypto::EncryptOptions::default(),
        )
    }

    /// [`Client::encrypt_for`] with an explicit algorithm choice.
    pub fn encrypt_for_with(
        mut self,
        intermediary_cert_der: impl Into<Vec<u8>>,
        opts: osci_transport_crypto::EncryptOptions,
    ) -> Self {
        self.intermediary_cert = Some(intermediary_cert_der.into());
        self.encrypt_options = opts;
        self
    }

    /// Use an explicit dialog (send `InitDialog` first).
    pub fn explicit_dialog(mut self) -> Self {
        self.dialog = ClientDialog::explicit();
        self
    }

    /// Set the key for opening transport-encrypted responses.
    pub fn with_decrypter(mut self, dec: impl Decrypter + Send + Sync + 'static) -> Self {
        self.decrypter = Some(Box::new(dec));
        self
    }

    /// Sign every request with a `ClientSignature`. `key_pem` is the RSA private
    /// key, `cert_der` the matching signature certificate.
    pub fn with_signer(
        mut self,
        key_pem: impl Into<Vec<u8>>,
        cert_der: impl Into<Vec<u8>>,
    ) -> Self {
        self.signer = Some(Signer {
            key_pem: Zeroizing::new(key_pem.into()),
            cert_der: cert_der.into(),
        });
        self
    }

    /// Check the signature on every response and reject a response that is
    /// unsigned or whose signature is invalid (spec 4.1.5). `trust` gets the
    /// signer certificate (DER) and decides whether it is the Intermediär's.
    /// An Intermediär signs its responses only to signed requests, so this
    /// needs [`Client::with_signer`]. A client with a signer must call this or
    /// [`Client::accept_unverified_responses`] before it sends.
    pub fn verify_responses(
        mut self,
        trust: impl Fn(&[u8]) -> bool + Send + Sync + 'static,
    ) -> Self {
        self.response_trust = Some(Box::new(trust));
        self
    }

    /// Accept responses without checking their signature, on a client that
    /// signs its requests. Without this or [`Client::verify_responses`] such a
    /// client refuses to send, so skipping the check is a choice in the code,
    /// not something that was forgotten.
    pub fn accept_unverified_responses(mut self) -> Self {
        self.accept_unverified = true;
        self
    }

    /// The current dialog state.
    pub fn dialog(&self) -> &ClientDialog {
        &self.dialog
    }

    /// Send a request with no content and return the decoded response.
    pub async fn send(&mut self, request: Request) -> Result<ClientResponse> {
        self.send_with(request, None, &[], &[]).await
    }

    /// Send a delivery request together with its `ContentPackage`. The
    /// Intermediär has no addressee/reader certificate for this delivery
    /// unless it's embedded here - use [`Client::send_delivery_with`] to
    /// name who the delivery is for.
    pub async fn send_delivery(
        &mut self,
        request: Request,
        content: ContentPackage,
    ) -> Result<ClientResponse> {
        self.send_with(request, Some(content), &[], &[]).await
    }

    /// [`Client::send_delivery`] with extra `osci:NonIntermediaryCertificates`
    /// slots - e.g. `CertSlot { kind: RoleKind::Addressee, is_signature: false, .. }`
    /// for the delivery's recipient, or `RoleKind::Reader` for an additional
    /// reader. A `StoreDelivery` naming no addressee at all is a valid
    /// request shape but not a deliverable one.
    pub async fn send_delivery_with(
        &mut self,
        request: Request,
        content: ContentPackage,
        recipient_certs: &[osci_transport_crypto::CertSlot],
    ) -> Result<ClientResponse> {
        self.send_with(request, Some(content), &[], recipient_certs)
            .await
    }

    /// [`Client::send_delivery_with`] with MIME attachments that the
    /// content's `cid:` references point to. Each attachment needs a unique
    /// `content_id`; a `cid:` reference without its part, or a part nothing
    /// refers to, is an error before anything is sent (as for every send). The parts go out
    /// after the envelope part, covered by the transport signature (one
    /// `cid:` reference each) and by transport encryption. With encrypted
    /// content the check cannot see inside, so a part is not required to be
    /// referenced.
    pub async fn send_delivery_with_attachments(
        &mut self,
        request: Request,
        content: ContentPackage,
        attachments: &[Attachment],
        recipient_certs: &[osci_transport_crypto::CertSlot],
    ) -> Result<ClientResponse> {
        self.send_with(request, Some(content), attachments, recipient_certs)
            .await
    }

    // The next SequenceNumber confirms receipt of the previous response (spec
    // 4.5), so the dialog only moves on once a response was accepted. After a
    // failed exchange the next request reuses the same number.
    async fn send_with(
        &mut self,
        request: Request,
        content: Option<ContentPackage>,
        attachments: &[Attachment],
        recipient_certs: &[osci_transport_crypto::CertSlot],
    ) -> Result<ClientResponse> {
        // no cid: reference may leave without its part, and no part without
        // a reference, whichever send method got us here
        match &content {
            Some(c) => c.check_attachments(attachments)?,
            None if !attachments.is_empty() => {
                return Err(Error::msg("attachments need a ContentPackage"));
            }
            None => {}
        }
        if self.signer.is_some() && self.response_trust.is_none() && !self.accept_unverified {
            return Err(Error::msg(
                "responses are not verified: call verify_responses, \
                 or accept_unverified_responses to go without",
            ));
        }
        let saved = self.dialog.clone();
        let out = self
            .exchange(request, content, attachments, recipient_certs)
            .await;
        if out.is_err() {
            self.dialog = saved;
        }
        out
    }

    async fn exchange(
        &mut self,
        request: Request,
        content: Option<ContentPackage>,
        attachments: &[Attachment],
        recipient_certs: &[osci_transport_crypto::CertSlot],
    ) -> Result<ClientResponse> {
        let cb = if matches!(request, Request::InitDialog) {
            self.dialog.init_control_block(nonce()?)
        } else {
            self.dialog.request_control_block(nonce()?)
        };
        let mut env = RequestEnvelope::new(request, cb.clone());
        env.desired_languages = Some(self.desired_languages.clone());
        if matches!(env.request, Request::StoreDelivery { .. }) {
            env.quality_of_timestamps = vec![
                QualityOfTimestamp::plain("creation", "timestampcreation"),
                QualityOfTimestamp::plain("reception", "timestampreception"),
            ];
        }
        env.content_package = content;
        let mut body = env.encode()?;

        if let Some(s) = &self.signer {
            // Every signed message carries the client's own cipher
            // certificate (CipherCertificateOriginator) alongside its
            // signature certificate, whenever the client has one - the
            // Intermediär has no way to encrypt a response back to us
            // otherwise, and some deployments reject the request outright
            // for missing it (SOAP fault 9500 in practice). ExitDialog,
            // FetchDelivery and FetchProcessCard's NonIntermediaryCertificatesType
            // only allow a SignatureCertificateOriginator slot, no Cipher slot
            // at all - including one there makes the message schema-invalid
            // (SOAP fault 9100).
            let allows_cipher_slot = !matches!(
                env.request,
                Request::ExitDialog
                    | Request::FetchDelivery { .. }
                    | Request::FetchProcessCard { .. }
            );
            let mut extra_certs: Vec<osci_transport_crypto::CertSlot> = if allows_cipher_slot {
                self.decrypter
                    .as_deref()
                    .and_then(|d| d.certificate_der())
                    .map(|cert| vec![osci_transport_crypto::CertSlot::originator_cipher(cert)])
                    .unwrap_or_default()
            } else {
                Vec::new()
            };
            extra_certs.extend(recipient_certs.iter().cloned());
            body = osci_transport_crypto::sign_envelope_with_attachments(
                &body,
                osci_transport_crypto::Role::Client,
                &s.key_pem,
                &s.cert_der,
                &extra_certs,
                osci_transport::crypto::SignatureAlg::RsaSha256,
                attachments,
            )?;
        }
        // every OSCI message goes on the wire as a MIME entity, signed and
        // unencrypted ones included; an encrypted message wraps this MIME
        // entity again as the ciphertext plaintext
        body = osci_transport::mime::wrap_envelope_with(&body, attachments)?;
        if let Some(cert) = &self.intermediary_cert {
            body =
                osci_transport_crypto::encrypt_transport_with(&body, cert, &self.encrypt_options)?;
        }

        let response = self.transport.post(body).await?;
        let outer = Entity::parse(&response)?;
        let mut xml = outer.xml()?;
        let attachments;
        let mut decrypted = None;

        match sniff(&xml)? {
            Sniff::Encrypted => {
                let dec = self.decrypter.as_deref().ok_or_else(|| {
                    Error::msg("response is encrypted but the client has no decrypter")
                })?;
                let inner = decrypt_transport(&response, dec)?;
                let entity = Entity::parse(&inner)?;
                xml = entity.xml()?;
                attachments = entity.attachments();
                decrypted = Some(inner);
            }
            Sniff::Fault => {
                let (text, code) = osci_transport::envelope::fault_detail(&xml);
                return Err(Error::msg(match code {
                    Some(code) => format!("SOAP fault {code}: {text}"),
                    None => format!("SOAP fault: {text}"),
                }));
            }
            Sniff::Message(_) => attachments = outer.attachments(),
        }

        if let Some(trust) = &self.response_trust {
            let report = osci_transport_crypto::verify_transport_signature(&xml, trust)?;
            if !report.valid {
                return Err(Error::msg(format!(
                    "response signature rejected: {}",
                    report.reason.unwrap_or_default()
                )));
            }
            report.check_attachments(&attachments)?;
        }

        let env = parse(&xml)?;
        if let Some(rcb) = &env.control_block {
            self.dialog.accept_response(rcb)?;
        }
        let message_type = env
            .message_type
            .ok_or_else(|| Error::msg("response has no message type"))?;

        Ok(ClientResponse {
            attachments,
            message: decrypted.unwrap_or(response),
            envelope_xml: xml,
            message_type,
            feedback: env.feedback.clone(),
            envelope: env,
        })
    }
}

fn nonce() -> Result<String> {
    let mut b = [0u8; 16];
    getrandom::fill(&mut b).map_err(|e| Error::msg(format!("rng: {e}")))?;
    Ok(B64.encode(b))
}
