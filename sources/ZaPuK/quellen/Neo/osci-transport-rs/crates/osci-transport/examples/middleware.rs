// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! A Router with two layers: one logs, one rejects response-type messages before
//! they reach a handler.
//!
//! `cargo run -p osci-transport --example middleware`

use osci_transport::error::{Error, Result};
use osci_transport::feedback::code;
use osci_transport::message::MessageType;
use osci_transport::parts::Feedback;
use osci_transport::router::{BoxFuture, IncomingMessage, Next, OutgoingMessage, Router};
use pollster::block_on;

async fn on_store_delivery(_m: IncomingMessage) -> OutgoingMessage {
    OutgoingMessage::new(MessageType::ResponseToStoreDelivery)
        .feedback(Feedback::new(code::OK_DIALOG_OPEN, "gespeichert"))
}

// Logs the message type on the way in and the feedback on the way out.
fn logging(msg: IncomingMessage, next: Next) -> BoxFuture<'static, Result<OutgoingMessage>> {
    Box::pin(async move {
        println!("-> {:?}", msg.ty);
        let out = next(msg).await;
        match &out {
            Ok(o) => println!("<- {:?} {:?}", o.ty, o.feedback),
            Err(e) => println!("<- error: {e}"),
        }
        out
    })
}

// A Supplier only serves requests; reject responses with a message-level fault.
fn requests_only(msg: IncomingMessage, next: Next) -> BoxFuture<'static, Result<OutgoingMessage>> {
    Box::pin(async move {
        match msg.ty {
            Some(mt) if mt.is_request() => next(msg).await,
            other => Err(Error::msg(format!("not a request: {other:?}"))),
        }
    })
}

fn main() {
    let router = Router::default()
        .on(MessageType::StoreDelivery, on_store_delivery)
        .layer(logging)
        .layer(requests_only);

    let ok = IncomingMessage {
        ty: Some(MessageType::StoreDelivery),
        ..Default::default()
    };
    println!(
        "result: {:?}\n",
        block_on(router.dispatch(ok)).map(|o| o.ty)
    );

    let bad = IncomingMessage {
        ty: Some(MessageType::ResponseToStoreDelivery),
        ..Default::default()
    };
    println!("result: {:?}", block_on(router.dispatch(bad)).map(|o| o.ty));
}
