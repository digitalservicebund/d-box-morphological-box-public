// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::events::SerializableEventContent;
use crate::neo::parse::{parse_envelopes, parse_meta};

#[test]
fn test_envelopes() {
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

    let envelopes = parse_envelopes(items);
    assert_eq!(1, envelopes.len());
    println!("test_envelopes: {:#?}", envelopes);
}
