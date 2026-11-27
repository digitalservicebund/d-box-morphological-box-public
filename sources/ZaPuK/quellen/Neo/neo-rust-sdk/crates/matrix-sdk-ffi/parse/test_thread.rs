// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::events::SerializableEventContent;
use crate::neo::parse::parse_case;
use serde_json::json;
use crate::neo::model::{NeoBuildingBlock, NeoCase, NeoContent, NeoDialog, NeoEnvelope, NeoFileInfo, NeoHyperlink, NeoPollAnswer, NeoPollContent, NeoText};
use crate::neo::parse::test_util::assert_eq_case;

#[test]
fn test_thread_parsing() {
    let room_id = "!WUuPIAwhbcrDNSvgeS:ess1.dev.neo.fitko.dev";
    let sender_id = "@justjanne-sdktest:ess1.dev.neo.fitko.dev";
    
    let root_id = "$DFGaxy1W9eWSpK7Jo6IJJbLjn5T_X9Is9UekdtTthLI";
    let file_id = "$1JfH5Ia96SrY39TPk3FhPJpRDKyg5OpHSSbs1s0qHZ4";
    let button_id = "$0QPvrowS2LC4vWHc0XY-xP6qCXJTqEa5E8NiB27B3I0";
    let infobox_id = "$-fK7VbTKDMjkvAHRZ8Kp0nxQI0qEiX8_gN4BIgnotf0";
    let inquiry_id = "$CRog6b_Bh1BZLp6PVMHRYqg0FPT-jcZGz8BJ4jSSiz4";
    let answer1_id = "$cNzG10TXP3IMRH9BzINEfVVJ1oBwjZ5ZDONq3fbuW1o";
    let answer2_id = "$i2Yfid-sATfktnbBLCKvxpWpQ6E5os1jhy48K40REB4";

    let messages = vec![
        SerializableEventContent {
            event_type: "org.matrix.msc1767.message".to_string(),
            event_id: root_id.to_string(),
            sender: sender_id.to_string(),
            timestamp: Some(1778491339628),
            content: json!({
                "dev.fitko.neo.multi.length.v1": 4,
                "m.text": [ { "body": "Lorem ipsum i, dolor sit amet", "mimetype": "text/plain" } ]
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.file.v1".to_string(),
            event_id: file_id.to_string(),
            sender: sender_id.to_string(),
            timestamp: Some(1778491339792),
            content: json!({
                "dev.fitko.neo.file.v1": {
                    "mimetype": "text/plain",
                    "name": "question.txt",
                    "size": 38,
                    "uri": "mxc://ess1.dev.neo.fitko.dev/ezCVCfjGwSPWIoGPxrPXPbcr"
                },
                "dev.fitko.neo.title.v1": { "m.text": [ { "body": "Attachment", "mimetype": "text/plain" } ] },
                "im.nheko.relations.v1.relations": [
                    { "event_id": root_id, "rel_type": "m.thread" },
                    { "event_id": root_id, "rel_type": "dev.fitko.neo.relationships.chain.v1" }
                ],
                "m.relates_to": {
                    "event_id": root_id,
                    "is_falling_back": true,
                    "m.in_reply_to": { "event_id": root_id },
                    "rel_type": "m.thread"
                },
                "m.text": []
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.button.v1".to_string(),
            event_id: button_id.to_string(),
            sender: sender_id.to_string(),
            timestamp: Some(1778491339877),
            content: json!({
                "dev.fitko.neo.hyperlink.v1": {
                    "dev.fitko.neo.label.v1": { "m.text": [ { "body": "Open Website", "mimetype": "text/plain" } ] },
                    "url": "https://drachen-einfach.de"
                },
                "im.nheko.relations.v1.relations": [
                    { "event_id": root_id, "rel_type": "m.thread" },
                    { "event_id": file_id, "rel_type": "dev.fitko.neo.relationships.chain.v1" }
                ],
                "m.relates_to": {
                    "event_id": root_id,
                    "is_falling_back": true,
                    "m.in_reply_to": { "event_id": file_id },
                    "rel_type": "m.thread"
                },
                "m.text": []
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.infobox.v1".to_string(),
            event_id: infobox_id.to_string(),
            sender: sender_id.to_string(),
            timestamp: Some(1778491339977),
            content: json!({
                "dev.fitko.neo.hyperlink.v1": {
                    "dev.fitko.neo.label.v1": { "m.text": [ { "body": "Open Website", "mimetype": "text/plain" } ] },
                    "url": "https://drachentermin.de"
                },
                "dev.fitko.neo.infobox.text.v1": {
                    "m.text": [
                        { "body": "Hello world", "mimetype": "text/plain" },
                        { "body": "<i>Hello world</i>", "mimetype": "text/html" }
                    ]
                },
                "dev.fitko.neo.title.v1": { "m.text": [ { "body": "Please note", "mimetype": "text/plain" } ] },
                "im.nheko.relations.v1.relations": [
                    { "event_id": root_id, "rel_type": "m.thread" },
                    { "event_id": button_id, "rel_type": "dev.fitko.neo.relationships.chain.v1" }
                ],
                "m.relates_to": {
                    "event_id": root_id,
                    "is_falling_back": true,
                    "m.in_reply_to": { "event_id": button_id },
                    "rel_type": "m.thread"
                },
                "m.text": []
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.inquiry.dev.fitko.neo.poll.response.v1.v1".to_string(),
            event_id: inquiry_id.to_string(),
            sender: sender_id.to_string(),
            timestamp: Some(1778491340077),
            content: json!({
                "dev.fitko.neo.poll.v1": {
                    "answers": [
                        { "m.id": "#ff0000", "m.text": [ { "body": "Red", "mimetype": "text/plain" } ] },
                        { "m.id": "#0000ff", "m.text": [ { "body": "Blue", "mimetype": "text/plain" } ] },
                        { "m.id": "dev.fitko.neo.poll.custom.v1", "m.text": [ { "body": "Custom", "mimetype": "text/plain" } ] }
                    ],
                    "max_selections": 1,
                    "question": { "m.text": [ { "body": "Color of the dragon", "mimetype": "text/plain" } ] }
                },
                "im.nheko.relations.v1.relations": [
                    { "event_id": root_id, "rel_type": "m.thread" },
                    { "event_id": infobox_id, "rel_type": "dev.fitko.neo.relationships.chain.v1" }
                ],
                "m.relates_to": {
                    "event_id": root_id,
                    "is_falling_back": true,
                    "m.in_reply_to": { "event_id": infobox_id },
                    "rel_type": "m.thread"
                },
                "m.text": []
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.poll.response.v1".to_string(),
            event_id: answer1_id.to_string(),
            sender: sender_id.to_string(),
            timestamp: Some(1778491340374),
            content: json!({
                "dev.fitko.neo.selections.v1": [ "red" ],
                "im.nheko.relations.v1.relations": [
                    { "event_id": inquiry_id, "rel_type": "dev.fitko.neo.relationships.poll.v1" },
                    { "event_id": inquiry_id, "rel_type": "m.in_reply_to" },
                    { "event_id": root_id, "rel_type": "m.thread" }
                ],
                "m.relates_to": { "event_id": inquiry_id, "rel_type": "m.reference" },
                "m.text": []
            }),
        },
        SerializableEventContent {
            event_type: "dev.fitko.neo.poll.response.v1".to_string(),
            event_id: answer2_id.to_string(),
            sender: sender_id.to_string(),
            timestamp: Some(1778491340503),
            content: json!({
                "dev.fitko.neo.poll.custom.v1": "Grün",
                "dev.fitko.neo.selections.v1": [ "dev.fitko.neo.poll.custom.v1" ],
                "im.nheko.relations.v1.relations": [
                    { "event_id": inquiry_id, "rel_type": "dev.fitko.neo.relationships.poll.v1" },
                    { "event_id": inquiry_id, "rel_type": "m.in_reply_to" },
                    { "event_id": root_id, "rel_type": "m.thread" }
                ],
                "m.relates_to": { "event_id": inquiry_id, "rel_type": "m.reference" },
                "m.text": []
            }),
        },
    ];

    let expect = NeoCase {
        id: room_id.to_string(),
        dialogs: vec![
            NeoDialog {
                id: root_id.to_string(),
                message: NeoEnvelope {
                    id: root_id.to_string(),
                    timestamp: 1778491339628,
                    blocks: vec![
                        NeoBuildingBlock {
                            id: root_id.to_string(),
                            sender: sender_id.to_string(),
                            timestamp: 1778491339628,
                            content: NeoContent::Message {
                                text: NeoText {
                                    plain: Some("Lorem ipsum i, dolor sit amet".to_string()),
                                    html: None,
                                },
                                reply_to: None,
                            },
                        },
                        NeoBuildingBlock {
                            id: file_id.to_string(),
                            sender: sender_id.to_string(),
                            timestamp: 1778491339792,
                            content: NeoContent::File {
                                file: NeoFileInfo {
                                    uri: "mxc://ess1.dev.neo.fitko.dev/ezCVCfjGwSPWIoGPxrPXPbcr".to_string(),
                                    name: "question.txt".to_string(),
                                    mimetype: Some("text/plain".to_string()),
                                    size: Some(38),
                                },
                                title: Some(NeoText {
                                    plain: Some("Attachment".to_string()),
                                    html: None,
                                }),
                                reply_to: None,
                            },
                        },
                        NeoBuildingBlock {
                            id: button_id.to_string(),
                            sender: sender_id.to_string(),
                            timestamp: 1778491339877,
                            content: NeoContent::Button {
                                hyperlink: NeoHyperlink {
                                    label: NeoText {
                                        plain: Some("Open Website".to_string()),
                                        html: None,
                                    },
                                    url: "https://drachen-einfach.de".to_string(),
                                },
                            },
                        },
                        NeoBuildingBlock {
                            id: infobox_id.to_string(),
                            sender: sender_id.to_string(),
                            timestamp: 1778491339977,
                            content: NeoContent::Infobox {
                                title: NeoText {
                                    plain: Some("Please note".to_string()),
                                    html: None,
                                },
                                infobox: NeoText {
                                    plain: Some("Hello world".to_string()),
                                    html: Some("<i>Hello world</i>".to_string()),
                                },
                                hyperlink: NeoHyperlink {
                                    label: NeoText {
                                        plain: Some("Open Website".to_string()),
                                        html: None,
                                    },
                                    url: "https://drachentermin.de".to_string(),
                                },
                            },
                        },
                        NeoBuildingBlock {
                            id: inquiry_id.to_string(),
                            sender: sender_id.to_string(),
                            timestamp: 1778491340077,
                            content: NeoContent::InquiryPoll {
                                poll: NeoPollContent {
                                    multi_select: false,
                                    question: NeoText {
                                        plain: Some("Color of the dragon".to_string()),
                                        html: None,
                                    },
                                    answers: vec![
                                        NeoPollAnswer {
                                            id: "#ff0000".to_string(),
                                            text: NeoText {
                                                plain: Some("Red".to_string()),
                                                html: None,
                                            },
                                        },
                                        NeoPollAnswer {
                                            id: "#0000ff".to_string(),
                                            text: NeoText {
                                                plain: Some("Blue".to_string()),
                                                html: None,
                                            },
                                        },
                                        NeoPollAnswer {
                                            id: "dev.fitko.neo.poll.custom.v1".to_string(),
                                            text: NeoText {
                                                plain: Some("Custom".to_string()),
                                                html: None,
                                            },
                                        },
                                    ],
                                },
                            },
                        },
                    ],
                },
                answers: vec![
                    NeoEnvelope {
                        id: answer1_id.to_string(),
                        timestamp: 1778491340374,
                        blocks: vec![
                            NeoBuildingBlock {
                                id: answer1_id.to_string(),
                                sender: sender_id.to_string(),
                                timestamp: 1778491340374,
                                content: NeoContent::Poll {
                                    selections: vec![
                                        "red".to_string(),
                                    ],
                                    custom: None,
                                    reply_to: Some(inquiry_id.to_string()),
                                },
                            }
                        ],
                    },
                    NeoEnvelope {
                        id: answer2_id.to_string(),
                        timestamp: 1778491340503,
                        blocks: vec![
                            NeoBuildingBlock {
                                id: answer2_id.to_string(),
                                sender: sender_id.to_string(),
                                timestamp: 1778491340503,
                                content: NeoContent::Poll {
                                    selections: vec![
                                        "dev.fitko.neo.poll.custom.v1".to_string(),
                                    ],
                                    custom: Some("Grün".to_string()),
                                    reply_to: Some(inquiry_id.to_string()),
                                },
                            }
                        ],
                    },
                ],
            },
        ],
    };

    let actual = parse_case(room_id.to_string(), messages);
    assert_eq_case(&expect, &actual, "".to_string());
}
