// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! Deposit a delivery in the recipient's mailbox: ask the Intermediär for a
//! MessageId, then send a storeDelivery for it (implicit dialog).
//!
//! Run with:
//! `cargo run -p osci-transport-client --example store_delivery -- <intermediary-url>`

use osci_transport::content::RawContent;
use osci_transport::feedback::is_error;
use osci_transport::parts::{ContentContainer, ContentPackage, Feedback};
use osci_transport::wire::Request;
use osci_transport::xml::QName;
use osci_transport_client::{Client, HttpTransport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: store_delivery <intermediary-url>");
        std::process::exit(2);
    });

    let mut client = Client::new(HttpTransport::new(url));

    let id_resp = client.send(Request::GetMessageId).await?;
    report(&id_resp.feedback);
    let message_id = id_resp
        .envelope
        .header_blocks
        .iter()
        .find(|b| b.name.local == "responseToGetMessageId")
        .and_then(|b| extract_message_id(&b.xml))
        .ok_or("no MessageId in the response")?;
    println!("got MessageId {message_id}");

    let mut container = ContentContainer::new();
    container.id = Some("contentcontainer0".into());
    // plain content; wrap it in an EncryptedData first for real end-to-end encryption
    container.push_raw(RawContent::new(QName::new("", ""), b"Hallo Welt".to_vec()));
    let content = ContentPackage {
        containers: vec![container],
        encrypted: Vec::new(),
    };

    let store = client
        .send_delivery(
            Request::StoreDelivery {
                message_id,
                subject: Some("Beispiel".into()),
            },
            content,
        )
        .await?;
    report(&store.feedback);

    Ok(())
}

fn extract_message_id(xml: &[u8]) -> Option<String> {
    let s = std::str::from_utf8(xml).ok()?;
    let open = s.find("<osci:MessageId>")? + "<osci:MessageId>".len();
    let close = s[open..].find("</osci:MessageId>")? + open;
    Some(s[open..close].to_string())
}

fn report(feedback: &[Feedback]) {
    for f in feedback {
        let mark = if is_error(&f.code) { "!" } else { " " };
        println!("  {mark} {} {}", f.code, f.text);
    }
}
