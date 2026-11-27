// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::events::SerializableEventContent;
use crate::neo::model::{NeoCase, NeoDialog};
use crate::neo::parse::parse_dialog;
use crate::neo::parse::parse_meta;
use itertools::Itertools;
use tracing::{debug, warn};

pub fn parse_case(room_id: String, items: Vec<SerializableEventContent>) -> NeoCase {
    debug!(room_id = %room_id, item_count = items.len(), "Parsing case");

    let mut dialogs: Vec<NeoDialog> = items
        .into_iter()
        .filter_map(|item| {
            parse_meta(item).map_err(|err| {
                warn!(error = %err, "Failed to parse event metadata; dropping event.");
                err
            }).ok()
        })
        .into_group_map_by(|m| m.thread_root_id())
        .into_iter()
        .filter_map(|(root_id, thread_events)| parse_dialog(root_id, thread_events))
        .collect();

    dialogs.sort_unstable_by_key(|d| d.message.timestamp);

    NeoCase { id: room_id, dialogs }
}
