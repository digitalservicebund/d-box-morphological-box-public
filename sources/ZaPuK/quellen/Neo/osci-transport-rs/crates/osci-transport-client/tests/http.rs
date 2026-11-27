// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

#![cfg(feature = "http")]

use osci_transport_client::{HttpTransport, Transport};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// Serves one canned HTTP response; `length` adds a Content-Length header.
async fn serve_once(body: Vec<u8>, length: bool) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/", listener.local_addr().unwrap());
    tokio::spawn(async move {
        let (mut sock, _) = listener.accept().await.unwrap();
        let mut buf = [0u8; 4096];
        let _ = sock.read(&mut buf).await;
        let mut head = String::from("HTTP/1.1 200 OK\r\nConnection: close\r\n");
        if length {
            head.push_str(&format!("Content-Length: {}\r\n", body.len()));
        }
        head.push_str("\r\n");
        sock.write_all(head.as_bytes()).await.unwrap();
        sock.write_all(&body).await.unwrap();
    });
    url
}

#[tokio::test]
async fn returns_the_response_body() {
    let url = serve_once(b"hello".to_vec(), true).await;
    let out = HttpTransport::new(url).post(b"x".to_vec()).await.unwrap();
    assert_eq!(out, b"hello");
}

#[tokio::test]
async fn rejects_a_declared_oversize_body() {
    let url = serve_once(vec![0u8; 100], true).await;
    let err = HttpTransport::new(url)
        .with_max_response_bytes(10)
        .post(Vec::new())
        .await
        .unwrap_err();
    assert!(err.to_string().contains("over 10 bytes"), "{err}");
}

#[tokio::test]
async fn rejects_an_oversize_body_without_content_length() {
    let url = serve_once(vec![0u8; 100], false).await;
    let err = HttpTransport::new(url)
        .with_max_response_bytes(10)
        .post(Vec::new())
        .await
        .unwrap_err();
    assert!(err.to_string().contains("over 10 bytes"), "{err}");
}

#[tokio::test]
async fn read_timeout_can_be_changed() {
    // accepts the connection but never answers
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/", listener.local_addr().unwrap());
    tokio::spawn(async move {
        let (_sock, _) = listener.accept().await.unwrap();
        std::future::pending::<()>().await;
    });

    let err = HttpTransport::new(url)
        .with_read_timeout(std::time::Duration::from_millis(200))
        .post(Vec::new())
        .await
        .unwrap_err();
    assert!(err.to_string().contains("http post"), "{err}");
}
