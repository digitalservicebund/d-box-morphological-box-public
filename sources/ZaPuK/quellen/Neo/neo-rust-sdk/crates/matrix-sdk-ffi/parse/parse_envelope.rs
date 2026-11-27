// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::model::NeoEnvelope;
use crate::neo::parse::NeoEventMeta;
use itertools::Itertools;
use std::collections::HashMap;
use tracing::warn;

pub fn parse_envelopes(metas: Vec<NeoEventMeta>) -> Vec<NeoEnvelope> {
    let chain_roots: Vec<NeoEventMeta> = metas.iter()
        .filter(|it| it.reply_to.is_none())
        .map(|it| it.clone())
        .collect();

    let chain_links: HashMap<String, Vec<NeoEventMeta>> = metas
        .iter()
        .filter_map(|it| it.reply_to.as_ref().map(|rt| (rt.to_string(), it.clone())))
        .into_group_map();

    chain_roots.into_iter()
        .filter_map(|root| parse_envelope(root, &chain_links))
        .collect()
}

pub fn parse_envelope(root: NeoEventMeta, chain_links: &HashMap<String, Vec<NeoEventMeta>>) -> Option<NeoEnvelope> {
    let mut blocks = vec![];
    let mut next = Some(&root.event);

    while let Some(block) = next {
        next = match chain_links.get(&block.id).and_then(|it| it.split_first()) {
            None => None,
            Some((first, rest)) => {
                if !rest.is_empty() {
                    warn!("More than one child for envelope!");
                    return None
                }
                Some(&first.event)
            }
        };
        blocks.push(block.clone());
    }

    Some(NeoEnvelope {
        id: root.event.id.clone(),
        timestamp: root.event.timestamp,
        blocks,
    })
}
