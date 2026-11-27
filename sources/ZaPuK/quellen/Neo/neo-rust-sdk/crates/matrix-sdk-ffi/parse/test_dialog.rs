// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::events::SerializableEventContent;
use crate::neo::parse::{parse_dialog, parse_meta};

#[test]
fn test_dialog() {
    let messages = vec![
        SerializableEventContent {
            event_type: "org.matrix.msc1767.message".to_string(),
            event_id: "$DtFKtiiLaNWiYM3PXNeiFCUe3B-sRzIj2syYiCH4Se0".to_string(),
            sender: "@justjanne-sdktest:ess1.dev.neo.fitko.dev".to_string(),
            timestamp: Some(1777923240016),
            content: serde_json::json!({
                "dev.fitko.neo.multi.length.v1": 3,
                "m.text": [
                    {
                        "body": "Lorem ipsum i dolor sit amet",
                        "mimetype": "text/plain",
                    },
                ],
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.button.v1".to_string(),
            event_id: "$d50Drn8SpLDaJXE6erGcB9lBQiUyoey2egWTDB3dDos".to_string(),
            sender: "@justjanne-sdktest:ess1.dev.neo.fitko.dev".to_string(),
            timestamp: Some(1777923240130),
            content: serde_json::json!({
                "dev.fitko.neo.hyperlink.v1": {
                    "dev.fitko.neo.label.v1": {
                        "m.text": [
                            {
                                "body": "Open Website",
                                "mimetype": "text/plain",
                            },
                        ],
                    },
                    "url": "https://drachen-einfach.de",
                },
                "im.nheko.relations.v1.relations": [
                    {
                        "event_id": "$DtFKtiiLaNWiYM3PXNeiFCUe3B-sRzIj2syYiCH4Se0",
                        "rel_type": "dev.fitko.neo.relationships.chain.v1",
                    },
                    {
                        "event_id": "$DtFKtiiLaNWiYM3PXNeiFCUe3B-sRzIj2syYiCH4Se0",
                        "rel_type": "m.thread",
                    },
                ],
                "m.relates_to": {
                    "event_id": "$DtFKtiiLaNWiYM3PXNeiFCUe3B-sRzIj2syYiCH4Se0",
                    "is_falling_back": true,
                    "m.in_reply_to": {
                        "event_id": "$DtFKtiiLaNWiYM3PXNeiFCUe3B-sRzIj2syYiCH4Se0",
                    },
                    "rel_type": "m.thread",
                },
                "m.text": [],
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.infobox.v1".to_string(),
            event_id: "$eclx29U1SiRNJs2_PSWZuXnGsE1JWQUJh9uraxxKQmw".to_string(),
            sender: "@justjanne-sdktest:ess1.dev.neo.fitko.dev".to_string(),
            timestamp: Some(1777923240222),
            content: serde_json::json!({
                "dev.fitko.neo.hyperlink.v1": {
                    "dev.fitko.neo.label.v1": {
                        "m.text": [
                            {
                                "body": "Open Website",
                                "mimetype": "text/plain",
                            },
                        ],
                    },
                    "url": "https://drachentermin.de",
                },
                "dev.fitko.neo.infobox.text.v1": {
                    "m.text": [
                        {
                            "body": "Hello world",
                            "mimetype": "text/plain",
                        },
                        {
                            "body": "<i>Hello world</i>",
                            "mimetype": "text/html",
                        },
                    ],
                },
                "dev.fitko.neo.title.v1": {
                    "m.text": [
                        {
                            "body": "Please note",
                            "mimetype": "text/plain",
                        },
                    ],
                },
                "im.nheko.relations.v1.relations": [
                    {
                        "event_id": "$d50Drn8SpLDaJXE6erGcB9lBQiUyoey2egWTDB3dDos",
                        "rel_type": "dev.fitko.neo.relationships.chain.v1",
                    },
                    {
                        "event_id": "$DtFKtiiLaNWiYM3PXNeiFCUe3B-sRzIj2syYiCH4Se0",
                        "rel_type": "m.thread",
                    },
                ],
                "m.relates_to": {
                    "event_id": "$DtFKtiiLaNWiYM3PXNeiFCUe3B-sRzIj2syYiCH4Se0",
                    "is_falling_back": true,
                    "m.in_reply_to": {
                        "event_id": "$d50Drn8SpLDaJXE6erGcB9lBQiUyoey2egWTDB3dDos",
                    },
                    "rel_type": "m.thread",
                },
                "m.text": [],
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.inquiry.org.matrix.msc1767.message.v1".to_string(),
            event_id: "$0estnh-9XqTwqwVQTs6u0dC3x3Tz3kTHCWejXMXFzeI".to_string(),
            sender: "@justjanne-sdktest:ess1.dev.neo.fitko.dev".to_string(),
            timestamp: Some(1777923240328),
            content: serde_json::json!({
                "dev.fitko.neo.inquiry.org.matrix.msc1767.message.v1": {
                    "dev.fitko.neo.label.v1": {
                        "m.text": [
                            {
                                "body": "Approx 65 cm large green forest dragon.",
                                "mimetype": "text/plain",
                            },
                        ],
                    },
                    "dev.fitko.neo.title.v1": {
                        "m.text": [
                            {
                                "body": "Description of the dragon",
                                "mimetype": "text/plain",
                            },
                        ],
                    },
                },
                "im.nheko.relations.v1.relations": [
                    {
                        "event_id": "$eclx29U1SiRNJs2_PSWZuXnGsE1JWQUJh9uraxxKQmw",
                        "rel_type": "dev.fitko.neo.relationships.chain.v1",
                    },
                    {
                        "event_id": "$DtFKtiiLaNWiYM3PXNeiFCUe3B-sRzIj2syYiCH4Se0",
                        "rel_type": "m.thread",
                    },
                ],
                "m.relates_to": {
                    "event_id": "$DtFKtiiLaNWiYM3PXNeiFCUe3B-sRzIj2syYiCH4Se0",
                    "is_falling_back": true,
                    "m.in_reply_to": {
                        "event_id": "$eclx29U1SiRNJs2_PSWZuXnGsE1JWQUJh9uraxxKQmw",
                    },
                    "rel_type": "m.thread",
                },
                "m.text": [],
            }),
        },
    ];

    let items = messages.into_iter().filter_map(|item| {
        match parse_meta(item) {
            Ok(value) => Some(value),
            Err(err) => {
                println!("error: {}", err);
                None
            }
        }
    }).collect();

    let dialog = parse_dialog("$DtFKtiiLaNWiYM3PXNeiFCUe3B-sRzIj2syYiCH4Se0".to_string(), items).unwrap();
    assert_eq!(4, dialog.message.blocks.len());
    println!("test_dialog: {:#?}", dialog);
}

#[test]
fn test_dialog_with_answers() {
    let root_id = "$root";
    let inquiry_id = "$inquiry_poll";
    let answer1_id = "$answer1";
    let answer2_id = "$answer2";

    let messages = vec![
        SerializableEventContent {
            event_type: "org.matrix.msc1767.message".to_string(),
            event_id: root_id.to_string(),
            sender: "@agency:example.org".to_string(),
            timestamp: Some(1000),
            content: serde_json::json!({
                "dev.fitko.neo.multi.length.v1": 1,
                "m.text": [ { "body": "Poll question", "mimetype": "text/plain" } ],
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.inquiry.dev.fitko.neo.poll.response.v1.v1".to_string(),
            event_id: inquiry_id.to_string(),
            sender: "@agency:example.org".to_string(),
            timestamp: Some(1100),
            content: serde_json::json!({
                "dev.fitko.neo.poll.v1": {
                    "question": { "m.text": [ { "body": "Favorite color?", "mimetype": "text/plain" } ] },
                    "answers": [
                        { "m.id": "red", "m.text": [ { "body": "Red", "mimetype": "text/plain" } ] },
                        { "m.id": "blue", "m.text": [ { "body": "Blue", "mimetype": "text/plain" } ] }
                    ]
                },
                "im.nheko.relations.v1.relations": [
                    { "event_id": root_id, "rel_type": "dev.fitko.neo.relationships.chain.v1" },
                    { "event_id": root_id, "rel_type": "m.thread" },
                ],
                "m.relates_to": {
                    "event_id": root_id,
                    "is_falling_back": true,
                    "m.in_reply_to": {
                        "event_id": root_id,
                    },
                    "rel_type": "m.thread",
                },
                "m.text": [],
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.poll.response.v1".to_string(),
            event_id: answer1_id.to_string(),
            sender: "@user:example.org".to_string(),
            timestamp: Some(1200),
            content: serde_json::json!({
                "dev.fitko.neo.selections.v1": ["red"],
                "im.nheko.relations.v1.relations": [
                    { "event_id": root_id, "rel_type": "m.thread" },
                    { "event_id": inquiry_id, "rel_type": "dev.fitko.neo.relationships.poll.v1" },
                ],
                "m.relates_to": {
                    "event_id": inquiry_id,
                    "m.in_reply_to": {
                        "event_id": inquiry_id,
                    },
                    "rel_type": "m.reference",
                },
                "m.text": [],
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.poll.response.v1".to_string(),
            event_id: answer2_id.to_string(),
            sender: "@user:example.org".to_string(),
            timestamp: Some(1300),
            content: serde_json::json!({
                "dev.fitko.neo.selections.v1": ["blue"],
                "im.nheko.relations.v1.relations": [
                    { "event_id": root_id, "rel_type": "m.thread" },
                    { "event_id": inquiry_id, "rel_type": "dev.fitko.neo.relationships.poll.v1" },
                ],
                "m.relates_to": {
                    "event_id": inquiry_id,
                    "m.in_reply_to": {
                        "event_id": answer1_id,
                    },
                    "rel_type": "m.reference",
                },
                "m.text": [],
            }),
        },
    ];

    let items = messages.into_iter().filter_map(|item| {
        match parse_meta(item) {
            Ok(value) => Some(value),
            Err(err) => {
                println!("error: {}", err);
                None
            }
        }
    }).collect();

    let dialog = parse_dialog(root_id.to_string(), items).unwrap();

    println!("test_dialog_with_answers: {:#?}", dialog);

    assert_eq!(2, dialog.message.blocks.len());
    assert_eq!(2, dialog.answers.len());

    assert_eq!(answer1_id, dialog.answers[0].id);
    assert_eq!(answer2_id, dialog.answers[1].id);
}

#[test]
fn test_dialog_with_poll_and_free_text() {
    let root_id = "$root";
    let inquiry_id = "$inquiry_poll";
    let text_answer_id = "$text_answer";
    let poll_response_id = "$poll_response";

    let messages = vec![
        SerializableEventContent {
            event_type: "org.matrix.msc1767.message".to_string(),
            event_id: root_id.to_string(),
            sender: "@agency:example.org".to_string(),
            timestamp: Some(1000),
            content: serde_json::json!({
                "dev.fitko.neo.multi.length.v1": 1,
                "m.text": [ { "body": "Dialogue root", "mimetype": "text/plain" } ],
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.inquiry.dev.fitko.neo.poll.response.v1.v1".to_string(),
            event_id: inquiry_id.to_string(),
            sender: "@agency:example.org".to_string(),
            timestamp: Some(1100),
            content: serde_json::json!({
                "dev.fitko.neo.poll.v1": {
                    "question": { "m.text": [ { "body": "Favorite color?", "mimetype": "text/plain" } ] },
                    "answers": [
                        { "m.id": "red", "m.text": [ { "body": "Red", "mimetype": "text/plain" } ] },
                        { "m.id": "dev.fitko.neo.poll.custom.v1", "m.text": [ { "body": "Other", "mimetype": "text/plain" } ] }
                    ]
                },
                "im.nheko.relations.v1.relations": [
                    { "event_id": root_id, "rel_type": "m.thread" },
                    { "event_id": root_id, "rel_type": "dev.fitko.neo.relationships.chain.v1" },
                ],
                "m.relates_to": {
                    "event_id": root_id,
                    "is_falling_back": true,
                    "m.in_reply_to": {
                        "event_id": root_id,
                    },
                    "rel_type": "m.thread",
                },
                "m.text": [],
            }),
        },
        // Free text message as part of the poll answer multi-event
        SerializableEventContent {
            event_type: "org.matrix.msc1767.message".to_string(),
            event_id: text_answer_id.to_string(),
            sender: "@user:example.org".to_string(),
            timestamp: Some(1200),
            content: serde_json::json!({
                "dev.fitko.neo.multi.length.v1": 1,
                "m.text": [ { "body": "My favorite color is green!", "mimetype": "text/plain" } ],
                "im.nheko.relations.v1.relations": [
                    { "event_id": root_id, "rel_type": "m.thread" },
                    { "event_id": inquiry_id, "rel_type": "dev.fitko.neo.relationships.poll.v1" },
                ],
                "m.relates_to": {
                    "event_id": inquiry_id,
                    "m.in_reply_to": {
                        "event_id": inquiry_id,
                    },
                    "rel_type": "m.reference",
                },
            }),
        },
        // Poll response linking to both the inquiry and the free text message
        SerializableEventContent {
            event_type: "dev.fitko.neo.poll.response.v1".to_string(),
            event_id: poll_response_id.to_string(),
            sender: "@user:example.org".to_string(),
            timestamp: Some(1300),
            content: serde_json::json!({
                "dev.fitko.neo.selections.v1": ["dev.fitko.neo.poll.custom.v1"],
                "im.nheko.relations.v1.relations": [
                    { "event_id": root_id, "rel_type": "m.thread" },
                    { "event_id": text_answer_id, "rel_type": "dev.fitko.neo.relationships.chain.v1" },
                    { "event_id": inquiry_id, "rel_type": "dev.fitko.neo.relationships.poll.v1" },
                    { "event_id": text_answer_id, "rel_type": "dev.fitko.neo.relationships.poll.custom.v1" },
                ],
                "m.relates_to": {
                    "event_id": inquiry_id,
                    "m.in_reply_to": {
                        "event_id": inquiry_id,
                    },
                    "rel_type": "m.reference",
                },
                "m.text": [],
            }),
        },
    ];

    let items = messages.into_iter().filter_map(|item| {
        match parse_meta(item) {
            Ok(value) => Some(value),
            Err(err) => {
                println!("error: {}", err);
                None
            }
        }
    }).collect();

    let dialog = parse_dialog(root_id.to_string(), items).unwrap();

    println!("test_dialog_with_poll_and_free_text: {:#?}", dialog);

    assert_eq!(2, dialog.message.blocks.len());
    assert_eq!(1, dialog.answers.len()); // The poll answer is one envelope (multi-event)
    assert_eq!(2, dialog.answers[0].blocks.len()); // It should contain the text and the poll response

    assert_eq!(text_answer_id, dialog.answers[0].blocks[0].id);
    assert_eq!(poll_response_id, dialog.answers[0].blocks[1].id);
}
