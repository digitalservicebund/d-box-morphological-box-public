// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! Build a storeDelivery request, then read it back.
//!
//! `cargo run -p osci-transport --example build_and_parse`

use osci_transport::content::RawContent;
use osci_transport::dialog::ControlBlock;
use osci_transport::envelope::{parse, sniff};
use osci_transport::parts::{ContentContainer, ContentKind, ContentPackage};
use osci_transport::wire::{Request, RequestEnvelope};
use osci_transport::xml::QName;

fn main() {
    let cb = ControlBlock {
        id: Some("controlblock".into()),
        challenge: Some("YWJjZGVm".into()),
        sequence_number: Some(0),
        ..Default::default()
    };

    let mut container = ContentContainer::new();
    container.id = Some("contentcontainer0".into());
    container.push_raw(RawContent::new(QName::new("", ""), b"Hallo Welt".to_vec()));

    let bytes = RequestEnvelope::new(
        Request::StoreDelivery {
            message_id: "bWlk".into(),
            subject: Some("Beispiel".into()),
        },
        cb,
    )
    .with_content(ContentPackage {
        containers: vec![container],
        encrypted: Vec::new(),
    })
    .encode()
    .unwrap();

    println!("{}\n", String::from_utf8_lossy(&bytes));

    println!("sniff  -> {:?}", sniff(&bytes).unwrap());

    let env = parse(&bytes).unwrap();
    println!("type   -> {:?}", env.message_type);
    println!(
        "control-> conversation {:?}, seq {:?}",
        env.control_block
            .as_ref()
            .and_then(|c| c.conversation_id.clone()),
        env.control_block.and_then(|c| c.sequence_number),
    );
    if let Some(pkg) = env.content_package {
        for cc in pkg.containers {
            for c in cc.contents {
                if let ContentKind::Inline(r) = c.kind {
                    println!("content-> {:?}", String::from_utf8_lossy(&r.xml));
                }
            }
        }
    }
}
