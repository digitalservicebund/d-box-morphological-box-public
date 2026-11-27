// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::model::{NeoBuildingBlock, NeoCase, NeoDialog, NeoEnvelope};

pub(crate) fn assert_eq_case(expect: &NeoCase, actual: &NeoCase, prefix: String) {
    assert_eq!(
        expect.id,
        actual.id,
        "{}.id", prefix,
    );
    assert_eq!(
        expect.dialogs.len(),
        actual.dialogs.len(),
        "{}.dialogs", prefix,
    );
    for dialog in 0..expect.dialogs.len() {
        assert_eq_dialog(
            &expect.dialogs[dialog],
            &actual.dialogs[dialog],
            format!("{}.dialogs[{}]", prefix, dialog),
        );
    }

    // sanity check
    assert_eq!(
        expect,
        actual,
        "{}", prefix,
    );
}

pub(crate) fn assert_eq_dialog(expect: &NeoDialog, actual: &NeoDialog, prefix: String) {
    assert_eq!(
        expect.id,
        actual.id,
        "{}.id", prefix,
    );
    assert_eq_envelope(
        &expect.message,
        &actual.message,
        format!("{}.message", prefix),
    );

    assert_eq!(
        expect.answers.len(),
        actual.answers.len(),
        "{}.answers", prefix,
    );
    for answer in 0..expect.answers.len() {
        assert_eq_envelope(
            &expect.answers[answer],
            &actual.answers[answer],
            format!("{}.answers[{}]", prefix, answer),
        );
    }

    // sanity check
    assert_eq!(
        expect,
        actual,
        "{}", prefix,
    );
}

pub(crate) fn assert_eq_envelope(expect: &NeoEnvelope, actual: &NeoEnvelope, prefix: String) {
    assert_eq!(
        expect.id,
        actual.id,
        "{}.id", prefix,
    );
    assert_eq!(
        expect.timestamp,
        actual.timestamp,
        "{}.timestamp", prefix,
    );
    assert_eq!(
        expect.blocks.len(),
        actual.blocks.len(),
        "{}.blocks", prefix,
    );
    for block in 0..expect.blocks.len() {
        assert_eq_block(
            &expect.blocks[block],
            &actual.blocks[block],
            format!("{}.blocks[{}]", prefix, block)
        );
    }

    // sanity check
    assert_eq!(
        expect,
        actual,
        "{}", prefix,
    );
}

pub(crate) fn assert_eq_block(expect: &NeoBuildingBlock, actual: &NeoBuildingBlock, prefix: String) {
    assert_eq!(
        expect.id,
        actual.id,
        "{}.id", prefix,
    );

    assert_eq!(
        expect.sender,
        actual.sender,
        "{}.sender", prefix,
    );

    assert_eq!(
        expect.timestamp,
        actual.timestamp,
        "{}.timestamp", prefix,
    );

    assert_eq!(
        expect.content,
        actual.content,
        "{}.content", prefix,
    );

    // sanity check
    assert_eq!(
        expect,
        actual,
        "{}", prefix,
    );
}
