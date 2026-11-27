// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use matrix_sdk::ruma::events::MessageLikeEventContent;
use matrix_sdk_events::*;
use ruma::events::MessageLikeEventType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NeoMessageLikeEventContent {
    Button(ButtonEventContent),
    Infobox(InfoboxEventContent),
    InquiryMessage(InquiryMessageEventContent),
    InquiryFile(InquiryFileEventContent),
    InquiryPoll(InquiryPollEventContent),
    Poll(PollEventContent),
    Message(MessageEventContent),
    File(FileEventContent),
    Image(ImageEventContent),
    Video(VideoEventContent),
    Audio(AudioEventContent),
}

impl MessageLikeEventContent for NeoMessageLikeEventContent {
    fn event_type(&self) -> MessageLikeEventType {
        match self {
            NeoMessageLikeEventContent::Button(it) => it.event_type(),
            NeoMessageLikeEventContent::Infobox(it) => it.event_type(),
            NeoMessageLikeEventContent::InquiryMessage(it) => it.event_type(),
            NeoMessageLikeEventContent::InquiryFile(it) => it.event_type(),
            NeoMessageLikeEventContent::InquiryPoll(it) => it.event_type(),
            NeoMessageLikeEventContent::Poll(it) => it.event_type(),
            NeoMessageLikeEventContent::Message(it) => it.event_type(),
            NeoMessageLikeEventContent::File(it) => it.event_type(),
            NeoMessageLikeEventContent::Image(it) => it.event_type(),
            NeoMessageLikeEventContent::Video(it) => it.event_type(),
            NeoMessageLikeEventContent::Audio(it) => it.event_type(),
        }
    }
}

impl NeoMessageLikeEventContent {
    pub fn relations(&self) -> Option<Vec<NhekoRelationContent>> {
        match &self {
            NeoMessageLikeEventContent::Button(c) => c.relations.clone(),
            NeoMessageLikeEventContent::Infobox(c) => c.relations.clone(),
            NeoMessageLikeEventContent::InquiryMessage(c) => c.relations.clone(),
            NeoMessageLikeEventContent::InquiryFile(c) => c.relations.clone(),
            NeoMessageLikeEventContent::InquiryPoll(c) => c.relations.clone(),
            NeoMessageLikeEventContent::Poll(c) => c.relations.clone(),
            NeoMessageLikeEventContent::Message(c) => c.relations.clone(),
            NeoMessageLikeEventContent::File(c) => c.relations.clone(),
            NeoMessageLikeEventContent::Image(c) => c.relations.clone(),
            NeoMessageLikeEventContent::Video(c) => c.relations.clone(),
            NeoMessageLikeEventContent::Audio(c) => c.relations.clone(),
        }
    }

    pub fn with_relations(self, relations: Option<Vec<NhekoRelationContent>>) -> NeoMessageLikeEventContent {
        match self {
            NeoMessageLikeEventContent::Button(c) => NeoMessageLikeEventContent::Button(c.with_relations(relations)),
            NeoMessageLikeEventContent::Infobox(c) => NeoMessageLikeEventContent::Infobox(c.with_relations(relations)),
            NeoMessageLikeEventContent::InquiryMessage(c) => NeoMessageLikeEventContent::InquiryMessage(c.with_relations(relations)),
            NeoMessageLikeEventContent::InquiryFile(c) => NeoMessageLikeEventContent::InquiryFile(c.with_relations(relations)),
            NeoMessageLikeEventContent::InquiryPoll(c) => NeoMessageLikeEventContent::InquiryPoll(c.with_relations(relations)),
            NeoMessageLikeEventContent::Poll(c) => NeoMessageLikeEventContent::Poll(c.with_relations(relations)),
            NeoMessageLikeEventContent::Message(c) => NeoMessageLikeEventContent::Message(c.with_relations(relations)),
            NeoMessageLikeEventContent::File(c) => NeoMessageLikeEventContent::File(c.with_relations(relations)),
            NeoMessageLikeEventContent::Image(c) => NeoMessageLikeEventContent::Image(c.with_relations(relations)),
            NeoMessageLikeEventContent::Video(c) => NeoMessageLikeEventContent::Video(c.with_relations(relations)),
            NeoMessageLikeEventContent::Audio(c) => NeoMessageLikeEventContent::Audio(c.with_relations(relations)),
        }
    }

    pub fn relates_to(&self) -> Option<matrix_sdk::ruma::events::room::encrypted::Relation> {
        match &self {
            NeoMessageLikeEventContent::Button(c) => c.relates_to.clone(),
            NeoMessageLikeEventContent::Infobox(c) => c.relates_to.clone(),
            NeoMessageLikeEventContent::InquiryMessage(c) => c.relates_to.clone(),
            NeoMessageLikeEventContent::InquiryFile(c) => c.relates_to.clone(),
            NeoMessageLikeEventContent::InquiryPoll(c) => c.relates_to.clone(),
            NeoMessageLikeEventContent::Poll(c) => c.relates_to.clone(),
            NeoMessageLikeEventContent::Message(c) => c.relates_to.clone(),
            NeoMessageLikeEventContent::File(c) => c.relates_to.clone(),
            NeoMessageLikeEventContent::Image(c) => c.relates_to.clone(),
            NeoMessageLikeEventContent::Video(c) => c.relates_to.clone(),
            NeoMessageLikeEventContent::Audio(c) => c.relates_to.clone(),
        }
    }

    pub fn with_relates_to(self, relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>) -> NeoMessageLikeEventContent {
        match self {
            NeoMessageLikeEventContent::Button(c) => NeoMessageLikeEventContent::Button(c.with_relates_to(relates_to)),
            NeoMessageLikeEventContent::Infobox(c) => NeoMessageLikeEventContent::Infobox(c.with_relates_to(relates_to)),
            NeoMessageLikeEventContent::InquiryMessage(c) => NeoMessageLikeEventContent::InquiryMessage(c.with_relates_to(relates_to)),
            NeoMessageLikeEventContent::InquiryFile(c) => NeoMessageLikeEventContent::InquiryFile(c.with_relates_to(relates_to)),
            NeoMessageLikeEventContent::InquiryPoll(c) => NeoMessageLikeEventContent::InquiryPoll(c.with_relates_to(relates_to)),
            NeoMessageLikeEventContent::Poll(c) => NeoMessageLikeEventContent::Poll(c.with_relates_to(relates_to)),
            NeoMessageLikeEventContent::Message(c) => NeoMessageLikeEventContent::Message(c.with_relates_to(relates_to)),
            NeoMessageLikeEventContent::File(c) => NeoMessageLikeEventContent::File(c.with_relates_to(relates_to)),
            NeoMessageLikeEventContent::Image(c) => NeoMessageLikeEventContent::Image(c.with_relates_to(relates_to)),
            NeoMessageLikeEventContent::Video(c) => NeoMessageLikeEventContent::Video(c.with_relates_to(relates_to)),
            NeoMessageLikeEventContent::Audio(c) => NeoMessageLikeEventContent::Audio(c.with_relates_to(relates_to)),
        }
    }

    pub fn length(&self) -> Option<u64> {
        match &self {
            NeoMessageLikeEventContent::Button(c) => c.length.clone(),
            NeoMessageLikeEventContent::Infobox(c) => c.length.clone(),
            NeoMessageLikeEventContent::InquiryMessage(c) => c.length.clone(),
            NeoMessageLikeEventContent::InquiryFile(c) => c.length.clone(),
            NeoMessageLikeEventContent::InquiryPoll(c) => c.length.clone(),
            NeoMessageLikeEventContent::Poll(c) => c.length.clone(),
            NeoMessageLikeEventContent::Message(c) => c.length.clone(),
            NeoMessageLikeEventContent::File(c) => c.length.clone(),
            NeoMessageLikeEventContent::Image(c) => c.length.clone(),
            NeoMessageLikeEventContent::Video(c) => c.length.clone(),
            NeoMessageLikeEventContent::Audio(c) => c.length.clone(),
        }
    }

    pub fn with_length(self, length: Option<u64>) -> NeoMessageLikeEventContent {
        match self {
            NeoMessageLikeEventContent::Button(c) => NeoMessageLikeEventContent::Button(c.with_length(length)),
            NeoMessageLikeEventContent::Infobox(c) => NeoMessageLikeEventContent::Infobox(c.with_length(length)),
            NeoMessageLikeEventContent::InquiryMessage(c) => NeoMessageLikeEventContent::InquiryMessage(c.with_length(length)),
            NeoMessageLikeEventContent::InquiryFile(c) => NeoMessageLikeEventContent::InquiryFile(c.with_length(length)),
            NeoMessageLikeEventContent::InquiryPoll(c) => NeoMessageLikeEventContent::InquiryPoll(c.with_length(length)),
            NeoMessageLikeEventContent::Poll(c) => NeoMessageLikeEventContent::Poll(c.with_length(length)),
            NeoMessageLikeEventContent::Message(c) => NeoMessageLikeEventContent::Message(c.with_length(length)),
            NeoMessageLikeEventContent::File(c) => NeoMessageLikeEventContent::File(c.with_length(length)),
            NeoMessageLikeEventContent::Image(c) => NeoMessageLikeEventContent::Image(c.with_length(length)),
            NeoMessageLikeEventContent::Video(c) => NeoMessageLikeEventContent::Video(c.with_length(length)),
            NeoMessageLikeEventContent::Audio(c) => NeoMessageLikeEventContent::Audio(c.with_length(length)),
        }
    }
}
