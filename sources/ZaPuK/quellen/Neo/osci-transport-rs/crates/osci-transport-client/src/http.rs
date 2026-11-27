// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use std::time::Duration;

use osci_transport::envelope::{Sniff, sniff};
use osci_transport::error::{Error, Result};
use osci_transport::mime::main_xml_part;
use osci_transport::router::BoxFuture;

use crate::Transport;

/// A [`Transport`] that POSTs OSCI messages to an Intermediär over HTTP with
/// `Content-Type: text/xml`.
pub struct HttpTransport {
    url: String,
    client: reqwest::Client,
    connect_timeout: Duration,
    read_timeout: Duration,
    max_response_bytes: usize,
}

/// Default limit for a response body, see [`HttpTransport::with_max_response_bytes`].
pub const DEFAULT_MAX_RESPONSE_BYTES: usize = 256 * 1024 * 1024;

const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(120);

fn build_client(connect_timeout: Duration, read_timeout: Duration) -> reqwest::Client {
    reqwest::Client::builder()
        .connect_timeout(connect_timeout)
        .read_timeout(read_timeout)
        .build()
        .expect("the TLS backend or the system resolver could not start")
}

impl HttpTransport {
    /// A transport posting to `url` with a reqwest client that gives up on an
    /// unreachable host after 30 seconds and on a connection that sends no data
    /// for 120 seconds. There is no limit on the total transfer time, so large
    /// deliveries are not cut off. Change the timeouts with
    /// [`HttpTransport::with_connect_timeout`] and
    /// [`HttpTransport::with_read_timeout`], or bring your own client with
    /// [`HttpTransport::with_client`].
    pub fn new(url: impl Into<String>) -> Self {
        HttpTransport {
            url: url.into(),
            client: build_client(DEFAULT_CONNECT_TIMEOUT, DEFAULT_READ_TIMEOUT),
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
            read_timeout: DEFAULT_READ_TIMEOUT,
            max_response_bytes: DEFAULT_MAX_RESPONSE_BYTES,
        }
    }

    /// A transport posting to `url` with a caller-supplied reqwest client. Its
    /// timeouts and settings are used as they are. Calling a timeout setter
    /// afterwards replaces the client with a default-built one.
    pub fn with_client(url: impl Into<String>, client: reqwest::Client) -> Self {
        HttpTransport {
            client,
            ..HttpTransport::new(url)
        }
    }

    /// How long to wait for the connection to the host to open.
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self.client = build_client(self.connect_timeout, self.read_timeout);
        self
    }

    /// How long a connection may stay silent while a response is read. This
    /// applies to each read, not to the whole transfer.
    pub fn with_read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = timeout;
        self.client = build_client(self.connect_timeout, self.read_timeout);
        self
    }

    /// Fail instead of buffering a response body larger than `limit` bytes.
    /// The default is [`DEFAULT_MAX_RESPONSE_BYTES`].
    pub fn with_max_response_bytes(mut self, limit: usize) -> Self {
        self.max_response_bytes = limit;
        self
    }
}

impl Transport for HttpTransport {
    fn post(&self, body: Vec<u8>) -> BoxFuture<'_, Result<Vec<u8>>> {
        Box::pin(async move {
            let mut response = self
                .client
                .post(&self.url)
                .header("Content-Type", "text/xml; charset=utf-8")
                .body(body)
                .send()
                .await
                .map_err(|e| Error::msg(format!("http post: {e}")))?;
            let status = response.status();
            let too_big = || {
                Error::msg(format!(
                    "http response over {} bytes",
                    self.max_response_bytes
                ))
            };
            if response
                .content_length()
                .is_some_and(|n| n > self.max_response_bytes as u64)
            {
                return Err(too_big());
            }
            let mut bytes = Vec::new();
            while let Some(chunk) = response
                .chunk()
                .await
                .map_err(|e| Error::msg(format!("http read: {e}")))?
            {
                if bytes.len() + chunk.len() > self.max_response_bytes {
                    return Err(too_big());
                }
                bytes.extend_from_slice(&chunk);
            }
            // SOAP carries faults on a failing status; the caller reads the
            // OSCI fault code out of the body
            if !status.is_success() && !is_fault(&bytes) {
                return Err(Error::msg(format!("http status {status}")));
            }
            Ok(bytes)
        })
    }
}

fn is_fault(body: &[u8]) -> bool {
    main_xml_part(body).is_ok_and(|xml| matches!(sniff(&xml), Ok(Sniff::Fault)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soap_fault_body_is_recognised() {
        let fault = br#"<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"><soap:Body><soap:Fault/></soap:Body></soap:Envelope>"#;
        assert!(is_fault(fault));
    }

    #[test]
    fn other_bodies_are_not_faults() {
        assert!(!is_fault(b"<html>Internal Server Error</html>"));
        assert!(!is_fault(b""));
    }
}
