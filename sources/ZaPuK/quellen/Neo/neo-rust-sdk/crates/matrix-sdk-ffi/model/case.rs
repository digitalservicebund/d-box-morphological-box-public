// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::model::NeoDialog;

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoCase {
    pub id: String,
    pub dialogs: Vec<NeoDialog>
}
