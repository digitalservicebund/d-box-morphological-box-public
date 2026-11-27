// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::error::MessageServiceError;
use crate::neo::events::NeoMessageLikeEventContent;
use matrix_sdk_events::*;
use ruma::events::StaticEventContent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerializableEventContent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[serde(rename = "sender")]
    pub sender: String,
    #[serde(rename = "origin_server_ts", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    #[serde(rename = "content")]
    pub content: serde_json::Value,
}

impl TryFrom<SerializableEventContent> for NeoMessageLikeEventContent {
    type Error = MessageServiceError;

    fn try_from(value: SerializableEventContent) -> Result<Self, Self::Error> {
        match value.event_type.as_str() {
            ButtonEventContent::TYPE =>
                ButtonEventContent::deserialize(value.content)
                    .map(|it| NeoMessageLikeEventContent::Button(it))
                    .map_err(|e| MessageServiceError::ParseError { msg: format!("parse error: {}", e.to_string()) }),
            InfoboxEventContent::TYPE =>
                InfoboxEventContent::deserialize(value.content)
                    .map(|it| NeoMessageLikeEventContent::Infobox(it))
                    .map_err(|e| MessageServiceError::ParseError { msg: format!("parse error: {}", e.to_string()) }),
            InquiryMessageEventContent::TYPE =>
                InquiryMessageEventContent::deserialize(value.content)
                    .map(|it| NeoMessageLikeEventContent::InquiryMessage(it))
                    .map_err(|e| MessageServiceError::ParseError { msg: format!("parse error: {}", e.to_string()) }),
            InquiryFileEventContent::TYPE =>
                InquiryFileEventContent::deserialize(value.content)
                    .map(|it| NeoMessageLikeEventContent::InquiryFile(it))
                    .map_err(|e| MessageServiceError::ParseError { msg: format!("parse error: {}", e.to_string()) }),
            InquiryPollEventContent::TYPE =>
                InquiryPollEventContent::deserialize(value.content)
                    .map(|it| NeoMessageLikeEventContent::InquiryPoll(it))
                    .map_err(|e| MessageServiceError::ParseError { msg: format!("parse error: {}", e.to_string()) }),
            PollEventContent::TYPE =>
                PollEventContent::deserialize(value.content)
                    .map(|it| NeoMessageLikeEventContent::Poll(it))
                    .map_err(|e| MessageServiceError::ParseError { msg: format!("parse error: {}", e.to_string()) }),
            MessageEventContent::TYPE =>
                MessageEventContent::deserialize(value.content)
                    .map(|it| NeoMessageLikeEventContent::Message(it))
                    .map_err(|e| MessageServiceError::ParseError { msg: format!("parse error: {}", e.to_string()) }),
            FileEventContent::TYPE =>
                FileEventContent::deserialize(value.content)
                    .map(|it| NeoMessageLikeEventContent::File(it))
                    .map_err(|e| MessageServiceError::ParseError { msg: format!("parse error: {}", e.to_string()) }),
            ImageEventContent::TYPE =>
                ImageEventContent::deserialize(value.content)
                    .map(|it| NeoMessageLikeEventContent::Image(it))
                    .map_err(|e| MessageServiceError::ParseError { msg: format!("parse error: {}", e.to_string()) }),
            VideoEventContent::TYPE =>
                VideoEventContent::deserialize(value.content)
                    .map(|it| NeoMessageLikeEventContent::Video(it))
                    .map_err(|e| MessageServiceError::ParseError { msg: format!("parse error: {}", e.to_string()) }),
            AudioEventContent::TYPE =>
                AudioEventContent::deserialize(value.content)
                    .map(|it| NeoMessageLikeEventContent::Audio(it))
                    .map_err(|e| MessageServiceError::ParseError { msg: format!("parse error: {}", e.to_string()) }),
            _ => Err(MessageServiceError::ParseError { msg: format!("parse error: unknown event type: {}", value.event_type) })
        }
    }
}