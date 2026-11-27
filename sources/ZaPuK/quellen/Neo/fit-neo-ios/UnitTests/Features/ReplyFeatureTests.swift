//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
@testable import FITNeo
import MatrixRustSDK
import XCTest

@MainActor
final class ReplyFeatureTests: XCTestCase {
    private func makeInquiryMessageContent() -> NeoContent {
        .inquiryMessage(title: NeoText(plain: "Title", html: nil),
                        label: NeoText(plain: "Label", html: nil))
    }

    private func makePollContent() -> NeoContent {
        .inquiryPoll(poll: NeoPollContent(multiSelect: false,
                                          question: NeoText(plain: "Question?", html: nil),
                                          answers: [
                                              NeoPollAnswer(id: "answer-1",
                                                            text: NeoText(plain: "First", html: nil)),
                                              NeoPollAnswer(id: "answer-2",
                                                            text: NeoText(plain: "Second", html: nil))
                                          ]))
    }

    func testInitialState() {
        let state = ReplyFeature.State()

        XCTAssertEqual(state.displayName, "")
        XCTAssertFalse(state.isExpanded)
        XCTAssertTrue(state.replyContent.isEmpty)
        XCTAssertFalse(state.isSendingReply)
        XCTAssertFalse(state.isValidReply)
    }

    func testEditorIsSelectedFromContent() {
        if case .freeText = ReplyFeature.State(content: makeInquiryMessageContent()).editor {} else {
            XCTFail("Expected free-text editor for inquiry message content")
        }

        if case .singleSelect = ReplyFeature.State(content: makePollContent()).editor {} else {
            XCTFail("Expected single-select editor for inquiry poll content")
        }

        if case .unsupported = ReplyFeature.State().editor {} else {
            XCTFail("Expected unsupported editor when content is nil")
        }
    }

    func testOnAppearSetsDisplayName() async {
        let store = TestStore(
            initialState: ReplyFeature.State()
        ) {
            ReplyFeature()
        } withDependencies: {
            $0.userSessionClient.displayName = { "Erika Mustermann" }
        }

        await store.send(.onAppear)
        await store.receive(\.displayNameResponse) {
            $0.displayName = "Erika Mustermann"
        }
    }

    func testToggleExpanded() async {
        let store = TestStore(initialState: ReplyFeature.State()) {
            ReplyFeature()
        }

        await store.send(.toggleExpanded) {
            $0.isExpanded = true
        }

        await store.send(.toggleExpanded) {
            $0.isExpanded = false
        }
    }

    func testReplyContentDerivesFromFreeTextEditor() async {
        let store = TestStore(initialState: ReplyFeature.State(content: makeInquiryMessageContent())) {
            ReplyFeature()
        }

        XCTAssertFalse(store.state.isValidReply)

        await store.send(.editor(.freeText(.contentChanged("Hello")))) {
            $0.editor = .freeText(FreeTextReplyFeature.State(textContent: "Hello"))
        }

        XCTAssertTrue(store.state.isValidReply)
        XCTAssertEqual(store.state.replyContent,
                       [.message(text: NeoText(plain: "Hello", html: nil), replyTo: nil)])
    }

    func testReplyContentDerivesFromPollEditor() async {
        let store = TestStore(initialState: ReplyFeature.State(content: makePollContent())) {
            ReplyFeature()
        }

        XCTAssertFalse(store.state.isValidReply)

        await store.send(.editor(.singleSelect(.optionSelected("answer-2")))) {
            $0.editor = .singleSelect(SingleSelectReplyFeature.State(options: self.pollAnswers(),
                                                                     selectedOptionID: "answer-2"))
        }

        XCTAssertTrue(store.state.isValidReply)
        XCTAssertEqual(store.state.replyContent,
                       [.poll(selections: ["answer-2"], custom: nil, replyTo: nil)])
    }

    private func pollAnswers() -> [NeoPollAnswer] {
        [
            NeoPollAnswer(id: "answer-1", text: NeoText(plain: "First", html: nil)),
            NeoPollAnswer(id: "answer-2", text: NeoText(plain: "Second", html: nil))
        ]
    }
}
