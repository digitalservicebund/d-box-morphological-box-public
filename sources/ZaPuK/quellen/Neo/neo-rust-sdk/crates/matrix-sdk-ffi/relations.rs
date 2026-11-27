// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use ruma::events::relation::{Reference, Reply, Thread};
use ruma::events::room::encrypted::Relation;
use ruma::OwnedEventId;
use matrix_sdk_events::{NhekoRelationContent, NhekoRelationType};
use crate::neo::chain::ChainState;
use crate::neo::error::MessageServiceError;
use crate::neo::model::NeoContent;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ContentRelationKind { PollReply, Reply }

pub fn content_relation(block: &NeoContent) -> Result<Option<(ContentRelationKind, OwnedEventId)>, MessageServiceError> {
    let relation = match block {
        NeoContent::Poll { reply_to, .. } =>
            reply_to.clone().map(|id| (ContentRelationKind::PollReply, id)),
        NeoContent::Message { reply_to, .. } =>
            reply_to.clone().map(|id| (ContentRelationKind::Reply, id)),
        NeoContent::File { reply_to, .. } =>
            reply_to.clone().map(|id| (ContentRelationKind::Reply, id)),
        NeoContent::Image { reply_to, .. } =>
            reply_to.clone().map(|id| (ContentRelationKind::Reply, id)),
        NeoContent::Video { reply_to, .. } =>
            reply_to.clone().map(|id| (ContentRelationKind::Reply, id)),
        NeoContent::Audio { reply_to, .. } =>
            reply_to.clone().map(|id| (ContentRelationKind::Reply, id)),
        _ => None
    };
    if let Some((kind, id)) = relation {
        let id = OwnedEventId::try_from(id)
            .map_err(|e| MessageServiceError::IdError {
                msg: format!("could not parse event id: {}", e.to_string())
            })?;
        Ok(Some((kind, id)))
    } else {
        Ok(None)
    }
}

pub fn relates_to(content_relation: Option<(ContentRelationKind, OwnedEventId)>, chain: Option<ChainState>) -> Option<Relation> {
    if let Some((kind, id)) = content_relation {
        Some(match kind {
            ContentRelationKind::PollReply => Relation::Reference(Reference::new(id)),
            ContentRelationKind::Reply => match chain {
                None => Relation::Reply(Reply::with_event_id(id)),
                Some(chain) => Relation::Thread(Thread::reply(chain.thread.root, id))
            }
        })
    } else if let Some(chain) = chain {
        Some(Relation::Thread(Thread::plain(chain.thread.root, chain.thread.last)))
    } else {
        None
    }
}

pub fn relations(content_relation: Option<(ContentRelationKind, OwnedEventId)>, chain: Option<ChainState>) -> Option<Vec<NhekoRelationContent>> {
    let mut relations = Vec::new();

    if let Some((kind, id)) = content_relation {
        match kind {
            ContentRelationKind::PollReply => {
                relations.push(NhekoRelationContent { rel_type: NhekoRelationType::Poll, event_id: id.clone() });
                relations.push(NhekoRelationContent { rel_type: NhekoRelationType::Reply, event_id: id });
            }
            ContentRelationKind::Reply => {
                relations.push(NhekoRelationContent { rel_type: NhekoRelationType::Reply, event_id: id });
            }
        }
    }

    if let Some(chain) = chain {
        relations.push(NhekoRelationContent { rel_type: NhekoRelationType::Thread, event_id: chain.thread.root });
        if let Some(chain_id) = chain.chain {
            relations.push(NhekoRelationContent { rel_type: NhekoRelationType::Chain, event_id: chain_id });
        }
    }

    if relations.is_empty() { None } else { Some(relations) }
}
