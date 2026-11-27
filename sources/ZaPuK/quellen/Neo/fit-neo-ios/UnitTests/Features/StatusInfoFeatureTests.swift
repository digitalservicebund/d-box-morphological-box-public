//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
@testable import FITNeo
import XCTest

@MainActor
final class StatusInfoFeatureTests: XCTestCase {
    func testInitialState() {
        let store = TestStore(
            initialState: StatusInfoFeature.State(activeStatus: .processSent)
        ) {
            StatusInfoFeature()
        }

        XCTAssertEqual(store.state.activeStatus, .processSent)
        XCTAssertTrue(store.state.expandedStatus.isEmpty)
    }

    func testOnAppearExpandsActiveStatus() async {
        let store = TestStore(
            initialState: StatusInfoFeature.State(activeStatus: .processUnderReview)
        ) {
            StatusInfoFeature()
        }

        await store.send(.onAppear) {
            $0.expandedStatus = [.processUnderReview]
        }
    }

    func testExpandAddsStatusToExpandedSet() async {
        let store = TestStore(
            initialState: StatusInfoFeature.State(activeStatus: .processSent)
        ) {
            StatusInfoFeature()
        }

        await store.send(.onAppear) {
            $0.expandedStatus = [.processSent]
        }

        await store.send(.expand(.processReceived)) {
            $0.expandedStatus = [.processSent, .processReceived]
        }
    }

    func testExpandAlreadyExpandedStatusIsNoOp() async {
        let store = TestStore(
            initialState: StatusInfoFeature.State(activeStatus: .processSent,
                                                  expandedStatus: [.processReceived])
        ) {
            StatusInfoFeature()
        }

        await store.send(.expand(.processReceived))
    }

    func testCollapseRemovesStatusFromExpandedSet() async {
        let store = TestStore(
            initialState: StatusInfoFeature.State(activeStatus: .processSent,
                                                  expandedStatus: [.processSent, .processReceived])
        ) {
            StatusInfoFeature()
        }

        await store.send(.collapse(.processReceived)) {
            $0.expandedStatus = [.processSent]
        }
    }

    func testCollapseActiveStatusIsAllowed() async {
        let store = TestStore(
            initialState: StatusInfoFeature.State(activeStatus: .processSent,
                                                  expandedStatus: [.processSent])
        ) {
            StatusInfoFeature()
        }

        await store.send(.collapse(.processSent)) {
            $0.expandedStatus = []
        }
    }

    func testCollapseNonExpandedStatusIsNoOp() async {
        let store = TestStore(
            initialState: StatusInfoFeature.State(activeStatus: .processSent,
                                                  expandedStatus: [.processSent])
        ) {
            StatusInfoFeature()
        }

        await store.send(.collapse(.processCompleted))
    }

    func testFullExpandCollapseFlow() async {
        let store = TestStore(
            initialState: StatusInfoFeature.State(activeStatus: .processUnderReview)
        ) {
            StatusInfoFeature()
        }

        await store.send(.onAppear) {
            $0.expandedStatus = [.processUnderReview]
        }

        await store.send(.expand(.processCompleted)) {
            $0.expandedStatus = [.processUnderReview, .processCompleted]
        }

        await store.send(.collapse(.processUnderReview)) {
            $0.expandedStatus = [.processCompleted]
        }

        await store.send(.collapse(.processCompleted)) {
            $0.expandedStatus = []
        }
    }
}
