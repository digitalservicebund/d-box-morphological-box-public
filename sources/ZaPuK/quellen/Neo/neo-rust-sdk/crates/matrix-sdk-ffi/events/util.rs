// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::events::NeoMessageLikeEventContent;
use crate::neo::model::NeoContent;
use matrix_sdk_events::*;

impl From<NeoContent> for NeoMessageLikeEventContent {
    fn from(value: NeoContent) -> Self {
        match value {
            NeoContent::Button { hyperlink } => {
                NeoMessageLikeEventContent::Button(ButtonEventContent {
                    hyperlink: hyperlink.into(),
                    text: Vec::new(),
                    relations: None,
                    relates_to: None,
                    length: None,
                })
            }
            NeoContent::Infobox { title, infobox, hyperlink } => {
                NeoMessageLikeEventContent::Infobox(InfoboxEventContent {
                    title: title.into(),
                    infobox_text: infobox.into(),
                    hyperlink: hyperlink.into(),
                    text: Vec::new(),
                    relations: None,
                    relates_to: None,
                    length: None,
                })
            }
            NeoContent::InquiryMessage { title, label } => {
                NeoMessageLikeEventContent::InquiryMessage(InquiryMessageEventContent {
                    inquiry: InquiryMessageContent {
                        title: title.map(Into::into),
                        label: label.map(Into::into),
                    },
                    text: Vec::new(),
                    relations: None,
                    relates_to: None,
                    length: None,
                })
            }
            NeoContent::InquiryFile { title, label } => {
                NeoMessageLikeEventContent::InquiryFile(InquiryFileEventContent {
                    inquiry: InquiryMessageContent {
                        title: title.map(Into::into),
                        label: label.map(Into::into),
                    },
                    text: Vec::new(),
                    relations: None,
                    relates_to: None,
                    length: None,
                })
            }
            NeoContent::InquiryPoll { poll } => {
                NeoMessageLikeEventContent::InquiryPoll(InquiryPollEventContent {
                    poll: poll.into(),
                    text: Vec::new(),
                    relations: None,
                    relates_to: None,
                    length: None,
                })
            }
            NeoContent::Poll { selections, custom, .. } => {
                NeoMessageLikeEventContent::Poll(PollEventContent {
                    selections,
                    custom,
                    text: Vec::new(),
                    relations: None,
                    relates_to: None,
                    length: None,
                })
            }
            NeoContent::Message { text, .. } => {
                NeoMessageLikeEventContent::Message(MessageEventContent {
                    text: text.into(),
                    relations: None,
                    relates_to: None,
                    length: None,
                })
            }
            NeoContent::File { file, title, .. } => {
                NeoMessageLikeEventContent::File(FileEventContent {
                    file: file.into(),
                    title: title.map(Into::into),
                    caption: None,
                    text: Vec::new(),
                    relations: None,
                    relates_to: None,
                    length: None,
                })
            }
            NeoContent::Image { file, title, image_details, thumbnail, alt_text, .. } => {
                NeoMessageLikeEventContent::Image(ImageEventContent {
                    file: file.into(),
                    title: title.map(Into::into),
                    image_details: image_details.map(Into::into),
                    thumbnail: thumbnail.map(Into::into),
                    caption: None,
                    alt_text: alt_text.map(|t| AltTextContent { text: t.into() }),
                    text: Vec::new(),
                    relations: None,
                    relates_to: None,
                    length: None,
                })
            }
            NeoContent::Video { file, title, image_details, thumbnail, .. } => {
                NeoMessageLikeEventContent::Video(VideoEventContent {
                    file: file.into(),
                    title: title.map(Into::into),
                    image_details: image_details.map(Into::into),
                    thumbnail: thumbnail.map(Into::into),
                    caption: None,
                    text: Vec::new(),
                    relations: None,
                    relates_to: None,
                    length: None,
                })
            }
            NeoContent::Audio { file, title, audio_details, .. } => {
                NeoMessageLikeEventContent::Audio(AudioEventContent {
                    file: file.into(),
                    title: title.map(Into::into),
                    audio_details: audio_details.map(Into::into),
                    caption: None,
                    text: Vec::new(),
                    relations: None,
                    relates_to: None,
                    length: None,
                })
            }
        }
    }
}

impl From<NeoMessageLikeEventContent> for NeoContent {
    fn from(value: NeoMessageLikeEventContent) -> Self {
        match value {
            NeoMessageLikeEventContent::Button(it) => {
                NeoContent::Button { hyperlink: it.hyperlink.into() }
            }
            NeoMessageLikeEventContent::Infobox(it) => NeoContent::Infobox {
                title: it.title.into(),
                infobox: it.infobox_text.into(),
                hyperlink: it.hyperlink.into(),
            },
            NeoMessageLikeEventContent::InquiryMessage(it) => NeoContent::InquiryMessage {
                title: it.inquiry.title.map(Into::into),
                label: it.inquiry.label.map(Into::into),
            },
            NeoMessageLikeEventContent::InquiryFile(it) => NeoContent::InquiryFile {
                title: it.inquiry.title.map(Into::into),
                label: it.inquiry.label.map(Into::into),
            },
            NeoMessageLikeEventContent::InquiryPoll(it) => {
                NeoContent::InquiryPoll { poll: it.poll.into() }
            }
            NeoMessageLikeEventContent::Poll(it) => NeoContent::Poll {
                selections: it.selections,
                custom: it.custom,
                reply_to: it.relations.and_then(|vec| { vec.iter()
                    .find(|it| it.rel_type == NhekoRelationType::Poll)
                    .map(|it| it.event_id.to_string())
                })
            },
            NeoMessageLikeEventContent::Message(it) => NeoContent::Message {
                text: it.text.into(),
                reply_to: it.relations.and_then(|vec| { vec.iter()
                    .find(|it| it.rel_type == NhekoRelationType::Reply)
                    .map(|it| it.event_id.to_string())
                })
            },
            NeoMessageLikeEventContent::File(it) => NeoContent::File {
                file: it.file.into(),
                title: it.title.map(Into::into),
                reply_to: it.relations.and_then(|vec| { vec.iter()
                    .find(|it| it.rel_type == NhekoRelationType::Reply)
                    .map(|it| it.event_id.to_string())
                })
            },
            NeoMessageLikeEventContent::Image(it) => NeoContent::Image {
                file: it.file.into(),
                title: it.title.map(Into::into),
                image_details: it.image_details.map(Into::into),
                thumbnail: it.thumbnail.map(Into::into),
                alt_text: it.alt_text.map(|a| a.text.into()),
                reply_to: it.relations.and_then(|vec| { vec.iter()
                    .find(|it| it.rel_type == NhekoRelationType::Reply)
                    .map(|it| it.event_id.to_string())
                })
            },
            NeoMessageLikeEventContent::Video(it) => NeoContent::Video {
                file: it.file.into(),
                title: it.title.map(Into::into),
                image_details: it.image_details.map(Into::into),
                thumbnail: it.thumbnail.map(Into::into),
                reply_to: it.relations.and_then(|vec| { vec.iter()
                    .find(|it| it.rel_type == NhekoRelationType::Reply)
                    .map(|it| it.event_id.to_string())
                })
            },
            NeoMessageLikeEventContent::Audio(it) => NeoContent::Audio {
                file: it.file.into(),
                title: it.title.map(Into::into),
                audio_details: it.audio_details.map(Into::into),
                reply_to: it.relations.and_then(|vec| { vec.iter()
                    .find(|it| it.rel_type == NhekoRelationType::Reply)
                    .map(|it| it.event_id.to_string())
                })
            },
        }
    }
}
