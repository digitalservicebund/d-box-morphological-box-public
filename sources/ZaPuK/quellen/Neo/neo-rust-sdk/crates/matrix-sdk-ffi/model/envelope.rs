// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::model::NeoBuildingBlock;

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoEnvelope {
    pub id: String,
    pub timestamp: u64,
    pub blocks: Vec<NeoBuildingBlock>
}
