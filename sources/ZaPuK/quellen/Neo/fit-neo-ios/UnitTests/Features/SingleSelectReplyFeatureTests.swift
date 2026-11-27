//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
@testable import FITNeo
import MatrixRustSDK
import XCTest

@MainActor
final class SingleSelectReplyFeatureTests: XCTestCase {
    private func options() -> [NeoPollAnswer] {
        [
            NeoPollAnswer(id: "answer-1", text: NeoText(plain: "First", html: nil)),
            NeoPollAnswer(id: "answer-2", text: NeoText(plain: "Second", html: nil)),
            NeoPollAnswer(id: SingleSelectReplyFeature.customOptionID,
                          text: NeoText(plain: "Custom", html: nil))
        ]
    }

    func testOptionSelectionSetsReplyContent() async {
        let store = TestStore(initialState: SingleSelectReplyFeature.State(options: options())) {
            SingleSelectReplyFeature()
        }

        XCTAssertTrue(store.state.replyContent.isEmpty)

        await store.send(.optionSelected("answer-2")) {
            $0.selectedOptionID = "answer-2"
        }

        XCTAssertEqual(store.state.replyContent,
                       [.poll(selections: ["answer-2"], custom: nil, replyTo: nil)])
    }

    func testCustomOptionRequiresCommentBeforeValid() async {
        let store = TestStore(initialState: SingleSelectReplyFeature.State(options: options())) {
            SingleSelectReplyFeature()
        }

        await store.send(.optionSelected(SingleSelectReplyFeature.customOptionID)) {
            $0.selectedOptionID = SingleSelectReplyFeature.customOptionID
        }

        XCTAssertTrue(store.state.replyContent.isEmpty)

        await store.send(.commentChanged("A free text comment")) {
            $0.comment = "A free text comment"
        }

        XCTAssertEqual(store.state.replyContent,
                       [.poll(selections: [], custom: "A free text comment", replyTo: nil)])
    }

    func testSwitchingFromCustomClearsComment() async {
        let store = TestStore(
            initialState: SingleSelectReplyFeature.State(options: options(),
                                                         selectedOptionID: SingleSelectReplyFeature.customOptionID,
                                                         comment: "Existing comment")
        ) {
            SingleSelectReplyFeature()
        }

        await store.send(.optionSelected("answer-1")) {
            $0.selectedOptionID = "answer-1"
            $0.comment = ""
        }

        XCTAssertEqual(store.state.replyContent,
                       [.poll(selections: ["answer-1"], custom: nil, replyTo: nil)])
    }

    func testWhitespaceCommentIsNotValid() async {
        let store = TestStore(
            initialState: SingleSelectReplyFeature.State(options: options(),
                                                         selectedOptionID: SingleSelectReplyFeature.customOptionID)
        ) {
            SingleSelectReplyFeature()
        }

        await store.send(.commentChanged("   \n  ")) {
            $0.comment = "   \n  "
        }

        XCTAssertTrue(store.state.replyContent.isEmpty)
    }
}
