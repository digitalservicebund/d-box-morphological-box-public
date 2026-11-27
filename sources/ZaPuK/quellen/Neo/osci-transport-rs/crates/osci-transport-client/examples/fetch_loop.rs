// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! Poll an Intermediär for waiting deliveries inside an explicit dialog.
//!
//! Run with: `cargo run -p osci-transport-client --example fetch_loop -- <intermediary-url>`

use osci_transport::feedback::{code, is_error};
use osci_transport::wire::Request;
use osci_transport_client::{Client, HttpTransport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: fetch_loop <intermediary-url>");
        std::process::exit(2);
    });

    let mut client = Client::new(HttpTransport::new(url)).explicit_dialog();

    // Open the dialog. The Intermediär assigns the ConversationId.
    let init = client.send(Request::InitDialog).await?;
    println!("dialog opened: {:?}", client.dialog().conversation_id);
    report(&init.feedback);

    // Pull every waiting delivery. Code 3800 means "more deliveries waiting".
    loop {
        let resp = client
            .send(Request::FetchDelivery { message_id: None })
            .await?;
        report(&resp.feedback);

        if let Some(pkg) = &resp.envelope.content_package {
            println!(
                "fetched a delivery with {} container(s)",
                pkg.containers.len()
            );
        }

        let more = resp
            .feedback
            .iter()
            .any(|f| f.code == code::MORE_DELIVERIES_WAITING);
        if !more {
            break;
        }
    }

    client.send(Request::ExitDialog).await?;
    println!("dialog closed");
    Ok(())
}

fn report(feedback: &[osci_transport::parts::Feedback]) {
    for f in feedback {
        let mark = if is_error(&f.code) { "!" } else { " " };
        println!("  {mark} {} {}", f.code, f.text);
    }
}
