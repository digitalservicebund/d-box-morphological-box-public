// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::events::SerializableEventContent;
use crate::neo::parse::parse_case;

#[test]
fn test_case() {
    let messages = vec![
        SerializableEventContent {
            event_type: "m.room.create".to_string(),
            event_id: "$BpQ-RItXrghUiO_IVRtr_dlhwr89iRZ-DEeITRa6wik".to_string(),
            sender: "@justjanne-sdktest:ess1.dev.neo.fitko.dev".to_string(),
            timestamp: Some(1777923239651),
            content: serde_json::json!({
                "creator": "@justjanne-sdktest:ess1.dev.neo.fitko.dev",
                "room_version": "10",
            }),
        },
        SerializableEventContent {
            event_type: "m.room.member".to_string(),
            event_id: "$Vbx5XNDTXqi95bSUh_24FcHj-BlESFIHFR_1Jd2cwZo".to_string(),
            sender: "@justjanne-sdktest:ess1.dev.neo.fitko.dev".to_string(),
            timestamp: Some(1777923239712),
            content: serde_json::json!({
                "displayname": "justjanne-sdktest",
                "membership": "join",
            }),
        },
        SerializableEventContent {
            event_type: "m.room.power_levels".to_string(),
            event_id: "$GoWoc8IyUwnTKfdATC5hbiy8eGkqP2GSUHraZJqJlxw".to_string(),
            sender: "@justjanne-sdktest:ess1.dev.neo.fitko.dev".to_string(),
            timestamp: Some(1777923239763),
            content: serde_json::json!({
                "ban": 50,
                "events": {
                    "m.room.avatar": 50,
                    "m.room.canonical_alias": 50,
                    "m.room.encryption": 100,
                    "m.room.history_visibility": 100,
                    "m.room.name": 50,
                    "m.room.power_levels": 100,
                    "m.room.server_acl": 100,
                    "m.room.tombstone": 100,
                },
                "events_default": 0,
                "historical": 100,
                "invite": 0,
                "kick": 50,
                "redact": 50,
                "state_default": 50,
                "users": {
                    "@justjanne-sdktest:ess1.dev.neo.fitko.dev": 100,
                    "@justjanne:ess1.dev.neo.fitko.dev": 100,
                },
                "users_default": 0,
            }),
        },
        SerializableEventContent {
            event_type: "m.room.join_rules".to_string(),
            event_id: "$ZXafdlQqagarhPk7LzRMBWZ9GAqp3vEIb69GYkhKlQM".to_string(),
            sender: "@justjanne-sdktest:ess1.dev.neo.fitko.dev".to_string(),
            timestamp: Some(1777923239775),
            content: serde_json::json!({
                "join_rule": "invite",
            }),
        },
        SerializableEventContent {
            event_type: "m.room.history_visibility".to_string(),
            event_id: "$M7-FklkBJwRQt0taASLvFCeTrgDWIbsn7trVrY-Gcak".to_string(),
            sender: "@justjanne-sdktest:ess1.dev.neo.fitko.dev".to_string(),
            timestamp: Some(1777923239776),
            content: serde_json::json!({
                "history_visibility": "shared",
            }),
        },
        SerializableEventContent {
            event_type: "m.room.guest_access".to_string(),
            event_id: "$zCxU5Lv9t4gKbg94BbWQF0imhJ4UUfFVbvP5Tm0lZP4".to_string(),
            sender: "@justjanne-sdktest:ess1.dev.neo.fitko.dev".to_string(),
            timestamp: Some(1777923239776),
            content: serde_json::json!({
                "guest_access": "can_join",
            }),
        },

        SerializableEventContent {
            event_type: "m.room.name".to_string(),
            event_id: "$WX1L-h8H-ZxbWt5Qzz7uQyWkqPFUn78t7yePtG6h6dk".to_string(),
            sender: "@justjanne-sdktest:ess1.dev.neo.fitko.dev".to_string(),
            timestamp: Some(1777923239777),
            content: serde_json::json!({
                "name": "Anfrage auf Herausgabe von Kontrollberichten gem § 2 Abs. 1 Verbraucherinformationsgesetz (VIG)",
            }),
        },
        SerializableEventContent {
            event_type: "m.room.member".to_string(),
            event_id: "$prRclhuB9cieG29DEgP_RVhpFhXX7r3GJazIfIeWeHA".to_string(),
            sender: "@justjanne-sdktest:ess1.dev.neo.fitko.dev".to_string(),
            timestamp: Some(1777923239889),
            content: serde_json::json!({
                "displayname": "justjanne",
                "membership": "invite",
            }),
        },
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

    let case = parse_case("!cRdNVVhaPdFXpuRVRZ:ess1.dev.neo.fitko.dev".to_string(), messages);
    println!("test_case: {:#?}", case);
}

#[test]
fn test_case_answers() {
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

    let case = parse_case("!cRdNVVhaPdFXpuRVRZ:ess1.dev.neo.fitko.dev".to_string(), messages);
    println!("Case: {:#?}", case);
}
