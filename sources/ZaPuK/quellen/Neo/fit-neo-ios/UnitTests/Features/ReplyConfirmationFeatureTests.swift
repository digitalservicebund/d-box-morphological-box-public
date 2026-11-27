//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
@testable import FITNeo
import MatrixRustSDK
import XCTest

@MainActor
final class ReplyConfirmationFeatureTests: XCTestCase {
    private func messageContent(_ text: String) -> [NeoContent] {
        [.message(text: NeoText(plain: text, html: nil), replyTo: nil)]
    }

    func testInitialState() {
        let state = ReplyConfirmationFeature.State()

        XCTAssertTrue(state.previewContent.isEmpty)
        XCTAssertEqual(state.submitStatus, .unsent)
    }

    func testOnAppearSetsPreviewContent() async {
        let store = TestStore(
            initialState: ReplyConfirmationFeature.State()
        ) {
            ReplyConfirmationFeature()
        }

        await store.send(.onAppear(messageContent("My reply."))) {
            $0.previewContent = self.messageContent("My reply.")
        }
    }

    func testOnAppearOverwritesPreviousPreviewContent() async {
        let store = TestStore(
            initialState: ReplyConfirmationFeature.State(previewContent: messageContent("old content"),
                                                         submitStatus: .unsent)
        ) {
            ReplyConfirmationFeature()
        }

        await store.send(.onAppear(messageContent("new content"))) {
            $0.previewContent = self.messageContent("new content")
        }
    }

    func testConfirmSendMovesToSendingStatus() async {
        let store = TestStore(
            initialState: ReplyConfirmationFeature.State(previewContent: messageContent("My reply"))
        ) {
            ReplyConfirmationFeature()
        }

        await store.send(.confirmSend) {
            $0.submitStatus = .sending
        }
    }

    func testCancelSendResetsStatusToUnsent() async {
        let store = TestStore(
            initialState: ReplyConfirmationFeature.State(previewContent: messageContent("My reply"),
                                                         submitStatus: .sending)
        ) {
            ReplyConfirmationFeature()
        }

        await store.send(.cancelSend) {
            $0.submitStatus = .unsent
        }
    }

    func testCancelSendPreservesPreviewContent() async {
        let store = TestStore(
            initialState: ReplyConfirmationFeature.State(previewContent: messageContent("Important reply"),
                                                         submitStatus: .unsent)
        ) {
            ReplyConfirmationFeature()
        }

        await store.send(.cancelSend)

        XCTAssertEqual(store.state.previewContent, messageContent("Important reply"))
    }

    func testSendSuccessMovesToSentStatus() async {
        let store = TestStore(
            initialState: ReplyConfirmationFeature.State(previewContent: messageContent("My reply"),
                                                         submitStatus: .sending)
        ) {
            ReplyConfirmationFeature()
        }

        await store.send(.sendSuccess) {
            $0.submitStatus = .sent
        }
    }

    func testSendErrorMovesToFailedStatus() async {
        let store = TestStore(
            initialState: ReplyConfirmationFeature.State(previewContent: messageContent("My reply"),
                                                         submitStatus: .sending)
        ) {
            ReplyConfirmationFeature()
        }

        await store.send(.sendError) {
            $0.submitStatus = .failed
        }
    }

    func testFullHappyPathTransitions() async {
        let store = TestStore(
            initialState: ReplyConfirmationFeature.State()
        ) {
            ReplyConfirmationFeature()
        }

        await store.send(.onAppear(messageContent("Final reply text"))) {
            $0.previewContent = self.messageContent("Final reply text")
        }

        await store.send(.confirmSend) {
            $0.submitStatus = .sending
        }

        await store.send(.sendSuccess) {
            $0.submitStatus = .sent
        }

        XCTAssertEqual(store.state.previewContent, messageContent("Final reply text"))
    }

    func testFailurePathTransitions() async {
        let store = TestStore(
            initialState: ReplyConfirmationFeature.State()
        ) {
            ReplyConfirmationFeature()
        }

        await store.send(.onAppear(messageContent("Reply that will fail"))) {
            $0.previewContent = self.messageContent("Reply that will fail")
        }

        await store.send(.confirmSend) {
            $0.submitStatus = .sending
        }

        await store.send(.sendError) {
            $0.submitStatus = .failed
        }

        XCTAssertEqual(store.state.previewContent, messageContent("Reply that will fail"))
    }
}
