// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::model::NeoDialog;
use crate::neo::parse::{parse_envelopes, NeoEventMeta};

pub fn parse_dialog(root_id: String, thread_events: Vec<NeoEventMeta>) -> Option<NeoDialog> {
    let envelopes = parse_envelopes(thread_events);
    let (message, answers) = envelopes.split_first()?;

    Some(NeoDialog {
        id: root_id,
        message: message.clone(),
        answers: answers.to_vec(),
    })
}
