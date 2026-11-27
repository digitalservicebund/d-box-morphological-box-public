// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::model::NeoContent;

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoBuildingBlock {
    pub id: String,
    pub sender: String,
    pub timestamp: u64,
    pub content: NeoContent,
}
