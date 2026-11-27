// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use ruma::OwnedEventId;
use matrix_sdk_events::NhekoRelationType;
use crate::neo::error::MessageServiceError;
use crate::neo::events::{NeoMessageLikeEventContent, SerializableEventContent};
use crate::neo::model::NeoBuildingBlock;

#[derive(Debug, Clone)]
pub struct NeoEventMeta {
    pub event: NeoBuildingBlock,
    pub thread_root: Option<OwnedEventId>,
    pub reply_to: Option<OwnedEventId>,
    pub multi_length: Option<u64>,
}

impl NeoEventMeta {
    pub fn thread_root_id(&self) -> String {
        self.thread_root
            .as_ref()
            .map(|id| id.to_string())
            .unwrap_or_else(|| self.event.id.clone())
    }
}

pub fn parse_meta(item: SerializableEventContent) -> Result<NeoEventMeta, MessageServiceError> {
    let id = item.event_id.clone();
    let sender = item.sender.clone();
    let timestamp = item.timestamp.unwrap_or(0);
    let content: NeoMessageLikeEventContent = item.try_into()?;

    let multi_length = content.length();
    let mut thread_root = None;
    let mut reply_to = None;

    if let Some(relations) = content.relations() {
        for rel in relations {
            match rel.rel_type {
                NhekoRelationType::Thread => thread_root = Some(rel.event_id),
                NhekoRelationType::Chain => reply_to = Some(rel.event_id),
                _ => {}
            }
        }
    }

    Ok(NeoEventMeta {
        event: NeoBuildingBlock {
            id,
            sender,
            timestamp,
            content: content.into()
        },
        thread_root,
        reply_to,
        multi_length,
    })
}
