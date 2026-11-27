//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
@testable import FITNeo
import XCTest

@MainActor
final class AppFeatureTests: XCTestCase {
    func testInitialSelectedTabIsProcesses() {
        XCTAssertEqual(AppFeature.State().authenticated.selectedTab, .processes)
    }

    func testTabSelectionChangedCyclesThroughAllTabs() async {
        let store = TestStore(initialState: AppFeature.State()) {
            AppFeature()
        }

        await store.send(.authenticated(.tabSelectionChanged(.documents))) { $0.authenticated.selectedTab = .documents }
        await store.send(.authenticated(.tabSelectionChanged(.account))) { $0.authenticated.selectedTab = .account }
        await store.send(.authenticated(.tabSelectionChanged(.help))) { $0.authenticated.selectedTab = .help }
        await store.send(.authenticated(.tabSelectionChanged(.processes))) { $0.authenticated.selectedTab = .processes }
    }

    func testProcessesHelpButtonSwitchesToHelpTab() async {
        let store = TestStore(initialState: AppFeature.State()) {
            AppFeature()
        }

        await store.send(.authenticated(.processes(.openHelpButtonTapped))) {
            $0.authenticated.selectedTab = .help
        }
    }

    func testLogoutDrivesLogoutFlow() async {
        let store = TestStore(initialState: AppFeature.State()) {
            AppFeature()
        } withDependencies: {
            $0.loginClient.destroySession = { }
        }

        await store.send(.login(.logout))
        await store.receive(\.authenticated.processes)
        await store.receive(\.login)
    }

    func testLogoutResetsAllAuthenticatedState() async {
        var initialState = AppFeature.State()
        initialState.authenticated.selectedTab = .account
        initialState.authenticated.processes.allProcesses = [
            ProcessRoom(id: "test-room", name: "Test", authority: "Test Authority", status: .processSent)
        ]
        initialState.authenticated.processes.loadState = .loaded

        let store = TestStore(initialState: initialState) {
            AppFeature()
        } withDependencies: {
            $0.loginClient.destroySession = { }
        }

        // A single assignment discards the whole authenticated session.
        await store.send(.login(.logout)) {
            $0.authenticated = AuthenticatedFeature.State()
        }
        await store.receive(\.authenticated.processes)
        await store.receive(\.login)
    }
}
