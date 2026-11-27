// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::error::MessageServiceError;
use crate::neo::events::NeoMessageLikeEventContent;
use crate::neo::model::NeoContent;
use crate::neo::relations::{content_relation, relates_to, relations};
use matrix_sdk::Room;
use ruma::OwnedEventId;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChainState {
    pub thread: ThreadState,
    pub chain: Option<OwnedEventId>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ThreadState {
    pub root: OwnedEventId,
    pub last: OwnedEventId,
}

pub async fn send_chain(
    room: &Room,
    blocks: Vec<NeoContent>,
    thread: Option<ThreadState>,
) -> Result<ThreadState, MessageServiceError> {
    let (head, blocks) = blocks.split_first().ok_or(MessageServiceError::EmptyMessageError)?;

    let followup_count = if blocks.is_empty() { None } else { Some(blocks.len() as u64) };

    let head_id = send_block(room, head.clone(), thread.clone().map(|thread| ChainState { thread, chain: None }), followup_count).await?;
    let thread_root = thread.map(|t| t.root).unwrap_or(head_id.clone());
    let mut chain_id = head_id;

    for block in blocks {
        let chain = ChainState {
            thread: ThreadState {
                root: thread_root.clone(),
                last: chain_id.clone(),
            },
            chain: Some(chain_id),
        };
        chain_id = send_block(room, block.clone(), Some(chain), None).await?;
    }

    Ok(ThreadState {
        root: thread_root,
        last: chain_id,
    })
}

async fn send_block(
    room: &Room,
    block: NeoContent,
    chain: Option<ChainState>,
    length: Option<u64>
) -> Result<OwnedEventId, MessageServiceError> {
    let content_relation = content_relation(&block)?;
    let event = NeoMessageLikeEventContent::from(block)
        .with_length(length)
        .with_relates_to(relates_to(content_relation.clone(), chain.clone()))
        .with_relations(relations(content_relation, chain));

    let result = room.send(event).await.map_err(|e| MessageServiceError::RequestError { msg: e.to_string() })?;
    Ok(result.response.event_id)
}
