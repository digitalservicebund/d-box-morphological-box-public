// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! Receive an OSCI message from a file, route it to a handler and print the
//! response envelope.
//!
//! `cargo run -p osci-transport-server --example handle_message -- <message.osci> [reader-key.pem reader-cert.der]`

use std::fs;

use osci_transport::feedback::code;
use osci_transport::message::MessageType;
use osci_transport::parts::Feedback;
use osci_transport::router::{IncomingMessage, OutgoingMessage, Router};
use osci_transport_crypto::RsaDecrypter;
use osci_transport_server::{ReceiverConfig, receive, respond};
use pollster::block_on;
use rsa::PrivateKey;
use rsa::pkcs8::DecodePrivateKey;

async fn on_store_delivery(m: IncomingMessage) -> OutgoingMessage {
    let containers = m.container.iter().count();
    println!("handler: storeDelivery with {containers} content container(s)");
    OutgoingMessage::new(MessageType::ResponseToStoreDelivery)
        .control_block(m.control_block)
        .feedback(Feedback::new(code::OK_DIALOG_OPEN, "Auftrag ausgeführt"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let path = args
        .next()
        .ok_or("usage: handle_message <message.osci> [key.pem cert.der]")?;
    let raw = fs::read(&path)?;

    let decrypter = match (args.next(), args.next()) {
        (Some(pem), Some(cert)) => {
            let key = PrivateKey::from_pkcs8_pem(&fs::read_to_string(pem)?)?;
            Some(RsaDecrypter::new(key).with_certificate(fs::read(cert)?))
        }
        _ => None,
    };

    let config = ReceiverConfig {
        decrypter,
        require_valid_signature: false,
    };
    let msg = receive(&raw, &config, |_| true)?;
    println!(
        "received: {:?} (encrypted: {})",
        msg.message_type, msg.transport_encrypted
    );

    let router = Router::default().on(MessageType::StoreDelivery, on_store_delivery);
    let out = block_on(router.dispatch(msg.into_incoming()))?;

    println!("\n{}", String::from_utf8_lossy(&respond(&out)?));
    Ok(())
}
