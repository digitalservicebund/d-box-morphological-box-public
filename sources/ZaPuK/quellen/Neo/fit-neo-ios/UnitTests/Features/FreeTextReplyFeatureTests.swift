//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
@testable import FITNeo
import MatrixRustSDK
import XCTest

@MainActor
final class FreeTextReplyFeatureTests: XCTestCase {
    func testContentChangedSetsReplyContent() async {
        let store = TestStore(initialState: FreeTextReplyFeature.State()) {
            FreeTextReplyFeature()
        }

        XCTAssertTrue(store.state.replyContent.isEmpty)

        await store.send(.contentChanged("Hello, this is my reply")) {
            $0.textContent = "Hello, this is my reply"
        }

        XCTAssertEqual(store.state.replyContent,
                       [.message(text: NeoText(plain: "Hello, this is my reply", html: nil),
                                 replyTo: nil)])
    }

    func testSetTemplateThenAddReplyTemplate() async {
        let store = TestStore(initialState: FreeTextReplyFeature.State()) {
            FreeTextReplyFeature()
        }

        let expectedTemplate = L10n.Screen.Message.Reply.template("Erika Mustermann")

        await store.send(.setTemplate("Erika Mustermann")) {
            $0.template = expectedTemplate
        }

        await store.send(.addReplyTemplate) {
            $0.textContent = expectedTemplate
        }

        XCTAssertFalse(store.state.replyContent.isEmpty)
    }

    func testContentCleared() async {
        let store = TestStore(initialState: FreeTextReplyFeature.State(textContent: "Some text")) {
            FreeTextReplyFeature()
        }

        XCTAssertFalse(store.state.replyContent.isEmpty)

        await store.send(.contentCleared) {
            $0.textContent = ""
        }

        XCTAssertTrue(store.state.replyContent.isEmpty)
    }

    func testWhitespaceOnlyContentIsNotValid() async {
        let store = TestStore(initialState: FreeTextReplyFeature.State()) {
            FreeTextReplyFeature()
        }

        await store.send(.contentChanged("   \n  ")) {
            $0.textContent = "   \n  "
        }

        XCTAssertTrue(store.state.replyContent.isEmpty)
    }
}
