// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use matrix_sdk::ruma::events::macros::EventContent;
use serde::{Deserialize, Serialize};

#[cfg(test)]
pub mod test_helpers;

pub mod metadata;

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "dev.fitko.neo.button.v1", kind = MessageLike)]
pub struct ButtonEventContent {
    #[serde(rename = "dev.fitko.neo.hyperlink.v1")]
    pub hyperlink: HyperlinkContent,
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    #[serde(
        rename = "im.nheko.relations.v1.relations",
        skip_serializing_if = "Option::is_none"
    )]
    pub relations: Option<Vec<NhekoRelationContent>>,
    #[serde(
        rename = "dev.fitko.neo.multi.length.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<u64>,
}

impl ButtonEventContent {
    pub fn with_relations(
        self,
        relations: Option<Vec<NhekoRelationContent>>,
    ) -> ButtonEventContent {
        ButtonEventContent { relations, ..self }
    }
    pub fn with_relates_to(
        self,
        relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    ) -> ButtonEventContent {
        ButtonEventContent { relates_to, ..self }
    }
    pub fn with_length(self, length: Option<u64>) -> ButtonEventContent {
        ButtonEventContent { length, ..self }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HyperlinkContent {
    #[serde(rename = "dev.fitko.neo.label.v1")]
    pub label: LabelContent,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultiLengthContent {
    pub length: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextContent {
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LabelContent {
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "dev.fitko.neo.infobox.v1", kind = MessageLike)]
pub struct InfoboxEventContent {
    #[serde(rename = "dev.fitko.neo.title.v1")]
    pub title: TitleContent,
    #[serde(rename = "dev.fitko.neo.infobox.text.v1")]
    pub infobox_text: InfoboxTextContent,
    #[serde(rename = "dev.fitko.neo.hyperlink.v1")]
    pub hyperlink: HyperlinkContent,
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    #[serde(
        rename = "im.nheko.relations.v1.relations",
        skip_serializing_if = "Option::is_none"
    )]
    pub relations: Option<Vec<NhekoRelationContent>>,
    #[serde(
        rename = "dev.fitko.neo.multi.length.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<u64>,
}

impl InfoboxEventContent {
    pub fn with_relations(
        self,
        relations: Option<Vec<NhekoRelationContent>>,
    ) -> InfoboxEventContent {
        InfoboxEventContent { relations, ..self }
    }
    pub fn with_relates_to(
        self,
        relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    ) -> InfoboxEventContent {
        InfoboxEventContent { relates_to, ..self }
    }
    pub fn with_length(self, length: Option<u64>) -> InfoboxEventContent {
        InfoboxEventContent { length, ..self }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InfoboxTextContent {
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TitleContent {
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "dev.fitko.neo.inquiry.org.matrix.msc1767.message.v1", kind = MessageLike)]
pub struct InquiryMessageEventContent {
    #[serde(rename = "dev.fitko.neo.inquiry.org.matrix.msc1767.message.v1")]
    pub inquiry: InquiryMessageContent,
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    #[serde(
        rename = "im.nheko.relations.v1.relations",
        skip_serializing_if = "Option::is_none"
    )]
    pub relations: Option<Vec<NhekoRelationContent>>,
    #[serde(
        rename = "dev.fitko.neo.multi.length.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<u64>,
}

impl InquiryMessageEventContent {
    pub fn with_relations(
        self,
        relations: Option<Vec<NhekoRelationContent>>,
    ) -> InquiryMessageEventContent {
        InquiryMessageEventContent { relations, ..self }
    }
    pub fn with_relates_to(
        self,
        relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    ) -> InquiryMessageEventContent {
        InquiryMessageEventContent { relates_to, ..self }
    }
    pub fn with_length(self, length: Option<u64>) -> InquiryMessageEventContent {
        InquiryMessageEventContent { length, ..self }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "dev.fitko.neo.inquiry.dev.fitko.neo.file.v1.v1", kind = MessageLike)]
pub struct InquiryFileEventContent {
    #[serde(rename = "dev.fitko.neo.inquiry.org.matrix.msc1767.message.v1")]
    pub inquiry: InquiryMessageContent,
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    #[serde(
        rename = "im.nheko.relations.v1.relations",
        skip_serializing_if = "Option::is_none"
    )]
    pub relations: Option<Vec<NhekoRelationContent>>,
    #[serde(
        rename = "dev.fitko.neo.multi.length.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<u64>,
}

impl InquiryFileEventContent {
    pub fn with_relations(
        self,
        relations: Option<Vec<NhekoRelationContent>>,
    ) -> InquiryFileEventContent {
        InquiryFileEventContent { relations, ..self }
    }
    pub fn with_relates_to(
        self,
        relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    ) -> InquiryFileEventContent {
        InquiryFileEventContent { relates_to, ..self }
    }
    pub fn with_length(self, length: Option<u64>) -> InquiryFileEventContent {
        InquiryFileEventContent { length, ..self }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "dev.fitko.neo.inquiry.dev.fitko.neo.poll.response.v1.v1", kind = MessageLike)]
pub struct InquiryPollEventContent {
    #[serde(rename = "dev.fitko.neo.poll.v1")]
    pub poll: PollContent,
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    #[serde(
        rename = "im.nheko.relations.v1.relations",
        skip_serializing_if = "Option::is_none"
    )]
    pub relations: Option<Vec<NhekoRelationContent>>,
    #[serde(
        rename = "dev.fitko.neo.multi.length.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<u64>,
}

impl InquiryPollEventContent {
    pub fn with_relations(
        self,
        relations: Option<Vec<NhekoRelationContent>>,
    ) -> InquiryPollEventContent {
        InquiryPollEventContent { relations, ..self }
    }
    pub fn with_relates_to(
        self,
        relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    ) -> InquiryPollEventContent {
        InquiryPollEventContent { relates_to, ..self }
    }
    pub fn with_length(self, length: Option<u64>) -> InquiryPollEventContent {
        InquiryPollEventContent { length, ..self }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PollContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<PollKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_selections: Option<u64>,
    pub question: QuestionContent,
    pub answers: Vec<PollAnswer>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuestionContent {
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PollAnswer {
    #[serde(rename = "m.id")]
    pub id: String,
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub enum PollKind {
    #[serde(rename = "m.disclosed")]
    Disclosed,
    #[serde(rename = "m.undisclosed")]
    #[default]
    Undisclosed,
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "dev.fitko.neo.poll.response.v1", kind = MessageLike)]
pub struct PollEventContent {
    #[serde(rename = "dev.fitko.neo.selections.v1")]
    pub selections: Vec<String>,
    #[serde(
        rename = "dev.fitko.neo.poll.custom.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom: Option<String>,
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    #[serde(
        rename = "im.nheko.relations.v1.relations",
        skip_serializing_if = "Option::is_none"
    )]
    pub relations: Option<Vec<NhekoRelationContent>>,
    #[serde(
        rename = "dev.fitko.neo.multi.length.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<u64>,
}

impl PollEventContent {
    pub fn with_relations(self, relations: Option<Vec<NhekoRelationContent>>) -> PollEventContent {
        PollEventContent { relations, ..self }
    }
    pub fn with_relates_to(
        self,
        relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    ) -> PollEventContent {
        PollEventContent { relates_to, ..self }
    }
    pub fn with_length(self, length: Option<u64>) -> PollEventContent {
        PollEventContent { length, ..self }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InquiryMessageContent {
    #[serde(
        rename = "dev.fitko.neo.title.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<TitleContent>,
    #[serde(
        rename = "dev.fitko.neo.label.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub label: Option<LabelContent>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct NhekoRelationContent {
    pub event_id: matrix_sdk::ruma::OwnedEventId,
    pub rel_type: NhekoRelationType,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum NhekoRelationType {
    #[serde(rename = "dev.fitko.neo.relationships.chain.v1")]
    Chain,
    #[serde(rename = "m.thread")]
    Thread,
    #[serde(rename = "m.in_reply_to")]
    Reply,
    #[serde(rename = "dev.fitko.neo.relationships.inquiry_auth.v1")]
    InquiryAuth,
    #[serde(rename = "dev.fitko.neo.relationships.poll.v1")]
    Poll,
    #[serde(rename = "dev.fitko.neo.relationships.poll.custom.v1")]
    PollCustom,
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "org.matrix.msc1767.message", kind = MessageLike)]
pub struct MessageEventContent {
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    #[serde(
        rename = "im.nheko.relations.v1.relations",
        skip_serializing_if = "Option::is_none"
    )]
    pub relations: Option<Vec<NhekoRelationContent>>,
    #[serde(
        rename = "dev.fitko.neo.multi.length.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<u64>,
}

impl MessageEventContent {
    pub fn with_relations(
        self,
        relations: Option<Vec<NhekoRelationContent>>,
    ) -> MessageEventContent {
        MessageEventContent { relations, ..self }
    }
    pub fn with_relates_to(
        self,
        relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    ) -> MessageEventContent {
        MessageEventContent { relates_to, ..self }
    }
    pub fn with_length(self, length: Option<u64>) -> MessageEventContent {
        MessageEventContent { length, ..self }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CaptionContent {
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "dev.fitko.neo.file.v1", kind = MessageLike)]
pub struct FileEventContent {
    #[serde(rename = "dev.fitko.neo.file.v1")]
    pub file: FileContent,
    #[serde(rename = "dev.fitko.neo.title.v1")]
    pub title: Option<TitleContent>,
    #[serde(rename = "m.caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<CaptionContent>,
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    #[serde(
        rename = "im.nheko.relations.v1.relations",
        skip_serializing_if = "Option::is_none"
    )]
    pub relations: Option<Vec<NhekoRelationContent>>,
    #[serde(
        rename = "dev.fitko.neo.multi.length.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<u64>,
}

impl FileEventContent {
    pub fn with_relations(self, relations: Option<Vec<NhekoRelationContent>>) -> FileEventContent {
        FileEventContent { relations, ..self }
    }
    pub fn with_relates_to(
        self,
        relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    ) -> FileEventContent {
        FileEventContent { relates_to, ..self }
    }
    pub fn with_length(self, length: Option<u64>) -> FileEventContent {
        FileEventContent { length, ..self }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileContent {
    pub uri: matrix_sdk::ruma::OwnedMxcUri,
    pub name: String,
    pub mimetype: Option<String>,
    pub size: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "dev.fitko.neo.image.v1", kind = MessageLike)]
pub struct ImageEventContent {
    #[serde(rename = "dev.fitko.neo.file.v1")]
    pub file: FileContent,
    #[serde(
        rename = "dev.fitko.neo.title.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<TitleContent>,
    #[serde(
        rename = "dev.fitko.neo.image.details.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_details: Option<ImageDetailsContent>,
    #[serde(
        rename = "dev.fitko.neo.thumbnail.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub thumbnail: Option<ThumbnailContent>,
    #[serde(rename = "m.caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<CaptionContent>,
    #[serde(rename = "m.alt_text", skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<AltTextContent>,
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    #[serde(
        rename = "im.nheko.relations.v1.relations",
        skip_serializing_if = "Option::is_none"
    )]
    pub relations: Option<Vec<NhekoRelationContent>>,
    #[serde(
        rename = "dev.fitko.neo.multi.length.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<u64>,
}

impl ImageEventContent {
    pub fn with_relations(self, relations: Option<Vec<NhekoRelationContent>>) -> ImageEventContent {
        ImageEventContent { relations, ..self }
    }
    pub fn with_relates_to(
        self,
        relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    ) -> ImageEventContent {
        ImageEventContent { relates_to, ..self }
    }
    pub fn with_length(self, length: Option<u64>) -> ImageEventContent {
        ImageEventContent { length, ..self }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageDetailsContent {
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub is_animated: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThumbnailContent {
    #[serde(rename = "dev.fitko.neo.file.v1")]
    pub file: FileContent,
    #[serde(rename = "dev.fitko.neo.image.details.v1")]
    pub image_details: ImageDetailsContent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AltTextContent {
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "dev.fitko.neo.video.v1", kind = MessageLike)]
pub struct VideoEventContent {
    #[serde(rename = "dev.fitko.neo.file.v1")]
    pub file: FileContent,
    #[serde(
        rename = "dev.fitko.neo.title.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<TitleContent>,
    #[serde(
        rename = "dev.fitko.neo.video.details.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_details: Option<VideoDetailsContent>,
    #[serde(
        rename = "dev.fitko.neo.thumbnail.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub thumbnail: Option<ThumbnailContent>,
    #[serde(rename = "m.caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<CaptionContent>,
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    #[serde(
        rename = "im.nheko.relations.v1.relations",
        skip_serializing_if = "Option::is_none"
    )]
    pub relations: Option<Vec<NhekoRelationContent>>,
    #[serde(
        rename = "dev.fitko.neo.multi.length.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<u64>,
}

impl VideoEventContent {
    pub fn with_relations(self, relations: Option<Vec<NhekoRelationContent>>) -> VideoEventContent {
        VideoEventContent { relations, ..self }
    }
    pub fn with_relates_to(
        self,
        relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    ) -> VideoEventContent {
        VideoEventContent { relates_to, ..self }
    }
    pub fn with_length(self, length: Option<u64>) -> VideoEventContent {
        VideoEventContent { length, ..self }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VideoDetailsContent {
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub duration: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "dev.fitko.neo.audio.v1", kind = MessageLike)]
pub struct AudioEventContent {
    #[serde(rename = "dev.fitko.neo.file.v1")]
    pub file: FileContent,
    #[serde(
        rename = "dev.fitko.neo.title.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<TitleContent>,
    #[serde(
        rename = "dev.fitko.neo.audio.details.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_details: Option<AudioDetailsContent>,
    #[serde(rename = "m.caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<CaptionContent>,
    #[serde(rename = "m.text")]
    pub text: Vec<TextContent>,
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    #[serde(
        rename = "im.nheko.relations.v1.relations",
        skip_serializing_if = "Option::is_none"
    )]
    pub relations: Option<Vec<NhekoRelationContent>>,
    #[serde(
        rename = "dev.fitko.neo.multi.length.v1",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<u64>,
}

impl AudioEventContent {
    pub fn with_relations(self, relations: Option<Vec<NhekoRelationContent>>) -> AudioEventContent {
        AudioEventContent { relations, ..self }
    }
    pub fn with_relates_to(
        self,
        relates_to: Option<matrix_sdk::ruma::events::room::encrypted::Relation>,
    ) -> AudioEventContent {
        AudioEventContent { relates_to, ..self }
    }
    pub fn with_length(self, length: Option<u64>) -> AudioEventContent {
        AudioEventContent { length, ..self }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioDetailsContent {
    pub duration: Option<u64>,
}

// This is currently broken, and i don't know why the serde_json::from_reader always dies. also don't know how to avoid unwrap in tests? pls help
#[cfg(test)]
mod test {
    use super::test_helpers::*;

    #[test]
    fn test_button_mimetype() -> Result<(), Box<dyn std::error::Error>> {
        let validator = get_validator();
        let example = get_example("button-mimetype")?;
        validator
            .validate(&example)
            .map_err(|e| Box::new(InvalidError::from(e)))?;
        Ok(())
    }

    #[test]
    /// mimetype content is not checked on the json schema - so the schema is "valid"
    fn test_button_wrong_mimetype() -> Result<(), Box<dyn std::error::Error>> {
        let validator = get_validator();
        let example = get_example("button-wrong-mimetype")?;
        assert!(
            !validator.is_valid(&example),
            "validator should not be valid for button-wrong-mimetype"
        );
        Ok(())
    }

    #[test]
    fn test_button_no_label_text() -> Result<(), Box<dyn std::error::Error>> {
        let validator = get_validator();
        let example = get_example("button-no-label-text")?;
        assert!(
            !validator.is_valid(&example),
            "validator should reject button without a label"
        );
        Ok(())
    }

    #[test]
    fn test_fitko_audio_v1() -> Result<(), Box<dyn std::error::Error>> {
        let validator = get_validator();
        let example = get_example("dev.fitko.neo.audio.v1")?;

        validator
            .validate(&example)
            .map_err(|e| Box::new(InvalidError::from(e)))?;
        Ok(())
    }

    #[test]
    fn test_fitko_button_v1() -> Result<(), Box<dyn std::error::Error>> {
        let validator = get_validator();
        let example = get_example("dev.fitko.neo.button.v1")?;

        validator
            .validate(&example)
            .map_err(|e| Box::new(InvalidError::from(e)))?;
        Ok(())
    }

    #[test]
    fn test_fitko_file_v1() -> Result<(), Box<dyn std::error::Error>> {
        let validator = get_validator();
        let example = get_example("dev.fitko.neo.file.v1")?;

        validator
            .validate(&example)
            .map_err(|e| Box::new(InvalidError::from(e)))?;
        Ok(())
    }

    #[test]
    fn test_fitko_video_v1() -> Result<(), Box<dyn std::error::Error>> {
        let validator = get_validator();
        let example = get_example("dev.fitko.neo.video.v1")?;

        validator
            .validate(&example)
            .map_err(|e| Box::new(InvalidError::from(e)))?;
        Ok(())
    }

    #[test]
    #[ignore = "not allowed in raw"]
    fn test_fitko_poll_response_v1() -> Result<(), Box<dyn std::error::Error>> {
        let validator = get_validator();
        let example = get_example("dev.fitko.neo.poll.response.v1")?;

        validator
            .validate(&example)
            .map_err(|e| Box::new(InvalidError::from(e)))?;
        Ok(())
    }

    #[test]
    fn test_fitko_image_v1() -> Result<(), Box<dyn std::error::Error>> {
        let validator = get_validator();
        let example = get_example("dev.fitko.neo.image.v1")?;

        validator
            .validate(&example)
            .map_err(|e| Box::new(InvalidError::from(e)))?;
        Ok(())
    }

    #[test]
    fn test_fitko_file_in_inquiry_v1v1() -> Result<(), Box<dyn std::error::Error>> {
        let validator = get_validator();
        let example = get_example("dev.fitko.neo.inquiry.dev.fitko.neo.file.v1.v1")?;

        validator
            .validate(&example)
            .map_err(|e| Box::new(InvalidError::from(e)))?;
        Ok(())
    }

    #[test]
    fn test_fitko_poll_response_in_inquiry_v1v1() -> Result<(), Box<dyn std::error::Error>> {
        let validator = get_validator();
        let example = get_example("dev.fitko.neo.inquiry.dev.fitko.neo.poll.response.v1.v1")?;

        validator
            .validate(&example)
            .map_err(|e| Box::new(InvalidError::from(e)))?;
        Ok(())
    }

    #[test]
    fn test_fitko_msc1767_in_inquiry_v1v1() -> Result<(), Box<dyn std::error::Error>> {
        let validator = get_validator();
        let example = get_example("dev.fitko.neo.inquiry.org.matrix.msc1767.message.v1")?;

        validator
            .validate(&example)
            .map_err(|e| Box::new(InvalidError::from(e)))?;
        Ok(())
    }
}
