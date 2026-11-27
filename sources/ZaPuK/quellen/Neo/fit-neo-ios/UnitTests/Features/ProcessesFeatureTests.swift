//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import ComposableArchitecture
import Dependencies
@testable import FITNeo
import MatrixRustSDK
import XCTest

@MainActor
final class ProcessesFeatureTests: XCTestCase {
    func testInitialState() {
        let store = TestStore(initialState: ProcessesViewFeature.State()) {
            ProcessesViewFeature()
        }

        XCTAssertEqual(store.state.loadState, .loading)
        XCTAssertEqual(store.state.numberOfProcesses, 0)
        XCTAssertEqual(store.state.pendingActivities, 0)
        XCTAssertEqual(store.state.allProcesses, [])
        XCTAssertEqual(store.state.open, [])
        XCTAssertEqual(store.state.completed, [])
    }

    func testCustomInitialState() {
        let openProcess = ProcessRoom(id: "open-1",
                                      name: "Open Process",
                                      authority: "Test Authority")
        let completedProcess = ProcessRoom(id: "completed-1",
                                           name: "Completed Process",
                                           authority: "Test Authority",
                                           status: .processCompleted)

        let store = TestStore(
            initialState: ProcessesViewFeature.State(loadState: .loaded,
                                                     pendingActivities: 1,
                                                     allProcesses: [openProcess, completedProcess])
        ) {
            ProcessesViewFeature()
        }

        XCTAssertEqual(store.state.loadState, .loaded)
        XCTAssertEqual(store.state.numberOfProcesses, 2)
        XCTAssertEqual(store.state.pendingActivities, 1)
        XCTAssertEqual(store.state.allProcesses.count, 2)
        XCTAssertEqual(store.state.open.count, 1)
        XCTAssertEqual(store.state.completed.count, 1)
    }

    func testInitialStateFromDependency() {
        withDependencies {
            $0.processesInitialState = ProcessesInitialStateClient {
                ProcessesTestScenario.mixed.makeState()
            }
        } operation: {
            let state = ProcessesViewFeature.State.initial()

            XCTAssertEqual(state.loadState, .loaded)
            XCTAssertEqual(state.numberOfProcesses, 4)
            XCTAssertEqual(state.open.count, 2)
            XCTAssertEqual(state.completed.count, 2)
        }
    }

    func testAllTestScenariosProduceValidState() {
        for scenario in ProcessesTestScenario.allCases {
            let state = scenario.makeState()

            XCTAssertEqual(state.loadState, .loaded, "Scenario \(scenario) should have loaded state")
            XCTAssertEqual(state.numberOfProcesses, state.allProcesses.count,
                           "Scenario \(scenario) should have correct process count")
            XCTAssertEqual(state.open.count + state.completed.count, state.allProcesses.count,
                           "Scenario \(scenario) should partition processes correctly")
        }
    }

    func testProcessSelectedOpensMessagesOverview() async {
        let process = ProcessRoom(id: "room-1", name: "Test Process", authority: "Test Authority")
        let store = TestStore(
            initialState: ProcessesViewFeature.State(loadState: .loaded, allProcesses: [process])
        ) {
            ProcessesViewFeature()
        } withDependencies: {
            $0.messagesOverviewInitialState = MessagesOverviewInitialStateClient { p in
                MessagesOverviewFeature.State(process: p)
            }
        }

        await store.send(.processSelected(process)) {
            $0.path.append(MessagesOverviewFeature.State(process: process))
        }
    }

    func testMessagesOverviewDismissedClearsState() async {
        let process = ProcessRoom(id: "room-1", name: "Test Process", authority: "Test Authority")
        var initialState = ProcessesViewFeature.State(loadState: .loaded, allProcesses: [process])
        initialState.path.append(MessagesOverviewFeature.State(process: process))

        let store = TestStore(initialState: initialState) {
            ProcessesViewFeature()
        }

        let id = store.state.path.ids[0]
        await store.send(.path(.popFrom(id: id))) {
            $0.path.removeAll()
        }
    }
}
