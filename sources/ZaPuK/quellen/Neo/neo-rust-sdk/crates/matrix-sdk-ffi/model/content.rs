// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use ruma::events::room::encrypted::Relation;
use matrix_sdk_events::*;

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Enum)]
pub enum NeoContent {
    Button {
        hyperlink: NeoHyperlink,
    },
    Infobox {
        title: NeoText,
        infobox: NeoText,
        hyperlink: NeoHyperlink,
    },
    InquiryMessage {
        title: Option<NeoText>,
        label: Option<NeoText>,
    },
    InquiryFile {
        title: Option<NeoText>,
        label: Option<NeoText>,
    },
    InquiryPoll {
        poll: NeoPollContent,
    },
    Poll {
        selections: Vec<String>,
        custom: Option<String>,

        reply_to: Option<String>,
    },
    Message {
        text: NeoText,

        reply_to: Option<String>,
    },
    File {
        file: NeoFileInfo,
        title: Option<NeoText>,

        reply_to: Option<String>,
    },
    Image {
        file: NeoFileInfo,
        title: Option<NeoText>,
        image_details: Option<NeoImageInfo>,
        thumbnail: Option<NeoThumbnail>,
        alt_text: Option<NeoText>,

        reply_to: Option<String>,
    },
    Video {
        file: NeoFileInfo,
        title: Option<NeoText>,
        image_details: Option<NeoVideoInfo>,
        thumbnail: Option<NeoThumbnail>,

        reply_to: Option<String>,
    },
    Audio {
        file: NeoFileInfo,
        title: Option<NeoText>,
        audio_details: Option<NeoAudioInfo>,

        reply_to: Option<String>,
    },
}

impl NeoContent {
    pub fn relates_to(&self) -> Option<Relation> {
        None
    }
}

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoText {
    pub plain: Option<String>,
    pub html: Option<String>,
}

impl From<Vec<TextContent>> for NeoText {
    fn from(texts: Vec<TextContent>) -> Self {
        let plain = texts.iter().filter(|it| match it.mimetype.as_deref() {
            None => true,
            Some("text/plain") => true,
            _ => false
        }).map(|it| it.body.clone()).next();

        let html = texts.iter().filter(|it| match it.mimetype.as_deref() {
            Some("text/html") => true,
            _ => false
        }).map(|it| it.body.clone()).next();

        Self { plain, html }
    }
}

impl Into<Vec<TextContent>> for NeoText {
    fn into(self) -> Vec<TextContent> {
        let mut texts = Vec::new();
        if let Some(plain) = self.plain {
            texts.push(TextContent {
                body: plain,
                lang: None,
                mimetype: Some("text/plain".to_owned()),
            });
        }
        if let Some(html) = self.html {
            texts.push(TextContent {
                body: html,
                lang: None,
                mimetype: Some("text/html".to_owned()),
            });
        }
        texts
    }
}

impl From<LabelContent> for NeoText {
    fn from(content: LabelContent) -> Self {
        content.text.into()
    }
}

impl Into<LabelContent> for NeoText {
    fn into(self) -> LabelContent {
        LabelContent { text: self.into() }
    }
}
impl From<TitleContent> for NeoText {
    fn from(content: TitleContent) -> Self {
        content.text.into()
    }
}
impl Into<TitleContent> for NeoText {
    fn into(self) -> TitleContent {
        TitleContent { text: self.into() }
    }
}
impl From<CaptionContent> for NeoText {
    fn from(content: CaptionContent) -> Self {
        content.text.into()
    }
}
impl Into<CaptionContent> for NeoText {
    fn into(self) -> CaptionContent {
        CaptionContent { text: self.into() }
    }
}

impl From<InfoboxTextContent> for NeoText {
    fn from(content: InfoboxTextContent) -> Self {
        content.text.into()
    }
}

impl Into<InfoboxTextContent> for NeoText {
    fn into(self) -> InfoboxTextContent {
        InfoboxTextContent { text: self.into() }
    }
}

impl From<QuestionContent> for NeoText {
    fn from(content: QuestionContent) -> Self {
        content.text.into()
    }
}

impl Into<QuestionContent> for NeoText {
    fn into(self) -> QuestionContent {
        QuestionContent { text: self.into() }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoHyperlink {
    pub label: NeoText,
    pub url: String,
}

impl From<HyperlinkContent> for NeoHyperlink {
    fn from(content: HyperlinkContent) -> Self {
        Self { label: content.label.into(), url: content.url }
    }
}

impl Into<HyperlinkContent> for NeoHyperlink {
    fn into(self) -> HyperlinkContent {
        HyperlinkContent { label: self.label.into(), url: self.url }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoPollContent {
    pub multi_select: bool,
    pub question: NeoText,
    pub answers: Vec<NeoPollAnswer>,
}

impl From<PollContent> for NeoPollContent {
    fn from(content: PollContent) -> Self {
        Self {
            multi_select: match content.max_selections {
                None => false,
                Some(1) => false,
                _ => true,
            },
            question: content.question.into(),
            answers: content.answers.into_iter().map(Into::into).collect(),
        }
    }
}

impl Into<PollContent> for NeoPollContent {
    fn into(self) -> PollContent {
        PollContent {
            kind: None,
            max_selections: match self.multi_select {
                true => Some(self.answers.len() as u64),
                false => Some(1)
            },
            question: self.question.into(),
            answers: self.answers.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoPollAnswer {
    pub id: String,
    pub text: NeoText,
}

impl From<PollAnswer> for NeoPollAnswer {
    fn from(answer: PollAnswer) -> Self {
        Self { id: answer.id, text: answer.text.into() }
    }
}

impl Into<PollAnswer> for NeoPollAnswer {
    fn into(self) -> PollAnswer {
        PollAnswer { id: self.id, text: self.text.into() }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoFileInfo {
    pub uri: String,
    pub name: String,
    pub mimetype: Option<String>,
    pub size: Option<u64>,
}

impl From<FileContent> for NeoFileInfo {
    fn from(content: FileContent) -> Self {
        Self {
            uri: content.uri.to_string(),
            name: content.name,
            mimetype: content.mimetype,
            size: content.size,
        }
    }
}

impl Into<FileContent> for NeoFileInfo {
    fn into(self) -> FileContent {
        FileContent {
            uri: self.uri.into(),
            name: self.name,
            mimetype: self.mimetype,
            size: self.size,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoAudioInfo {
    pub duration: Option<u64>,
}

impl From<AudioDetailsContent> for NeoAudioInfo {
    fn from(details: AudioDetailsContent) -> Self {
        Self { duration: details.duration }
    }
}

impl Into<AudioDetailsContent> for NeoAudioInfo {
    fn into(self) -> AudioDetailsContent {
        AudioDetailsContent { duration: self.duration }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoImageInfo {
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub is_animated: bool,
}

impl From<ImageDetailsContent> for NeoImageInfo {
    fn from(details: ImageDetailsContent) -> Self {
        Self { width: details.width, height: details.height, is_animated: details.is_animated }
    }
}

impl Into<ImageDetailsContent> for NeoImageInfo {
    fn into(self) -> ImageDetailsContent {
        ImageDetailsContent {
            width: self.width,
            height: self.height,
            is_animated: self.is_animated,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoVideoInfo {
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub duration: Option<u64>,
}

impl From<VideoDetailsContent> for NeoVideoInfo {
    fn from(details: VideoDetailsContent) -> Self {
        Self { width: details.width, height: details.height, duration: details.duration }
    }
}

impl Into<VideoDetailsContent> for NeoVideoInfo {
    fn into(self) -> VideoDetailsContent {
        VideoDetailsContent { width: self.width, height: self.height, duration: self.duration }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoThumbnail {
    pub file: NeoFileInfo,
    pub image_details: NeoImageInfo,
}

impl From<ThumbnailContent> for NeoThumbnail {
    fn from(content: ThumbnailContent) -> Self {
        Self { file: content.file.into(), image_details: content.image_details.into() }
    }
}

impl Into<ThumbnailContent> for NeoThumbnail {
    fn into(self) -> ThumbnailContent {
        ThumbnailContent { file: self.file.into(), image_details: self.image_details.into() }
    }
}
