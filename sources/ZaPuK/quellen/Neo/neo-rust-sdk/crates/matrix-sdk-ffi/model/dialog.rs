// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::model::NeoEnvelope;

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Record)]
pub struct NeoDialog {
    pub id: String,
    pub message: NeoEnvelope,
    pub answers: Vec<NeoEnvelope>,
}
