//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Combine
import ComposableArchitecture
import Foundation
import MatrixRustSDK
import SwiftUI

enum ProcessesLoadState: Equatable {
    case loading
    case loaded
    case failed
}

@Reducer
struct ProcessesViewFeature {
    @ObservableState
    struct State: Equatable {
        var loadState = ProcessesLoadState.loading
        var pendingActivities = 0
        var showEmptyReports = false
        var allProcesses: [ProcessRoom] = []
        var subscribedRoomIds: Set<String> = []
        var openBrowser = OpenBrowserFeature.State()
        var path = StackState<MessagesOverviewFeature.State>()

        var open: [ProcessRoom] {
            allProcesses.filter { !$0.isCompleted }
        }

        var completed: [ProcessRoom] {
            allProcesses.filter(\.isCompleted)
        }

        var numberOfProcesses: Int {
            allProcesses.count
        }
    }

    enum Action {
        case onAppear
        case reset
        case notificationsButtonTapped
        case openHelpButtonTapped
        case openBrowser(OpenBrowserFeature.Action)
        case processesUpdated([RoomListEntriesUpdate])
        case processSelected(ProcessRoom)
        case path(StackActionOf<MessagesOverviewFeature>)
    }

    @Dependency(\.processListClient) var processListClient

    private enum CancelID { case loadingEvents, updateEvents }

    var body: some Reducer<State, Action> {
        Scope(state: \.openBrowser, action: \.openBrowser) {
            OpenBrowserFeature()
        }

        Reduce { state, action in
            switch action {
            case .reset:
                state = ProcessesViewFeature.State()
                return .merge(.cancel(id: CancelID.loadingEvents),
                              .cancel(id: CancelID.updateEvents))

            case .onAppear:
                return .merge(.run { _ in
                    await processListClient.sync()
                },
                .publisher {
                    processListClient.updateEvents()
                        .map(Action.processesUpdated)
                }
                .cancellable(id: CancelID.updateEvents, cancelInFlight: true))

            case .notificationsButtonTapped:
                // MARK: needs update when notifications are implemented, for now

                // just toggle the empty / non-empty state to show the examples

                state.showEmptyReports.toggle()
                return .none

            case .openHelpButtonTapped:
                return .none

            case .openBrowser:
                return .none

            case let .processesUpdated(updates):
                for update in updates {
                    switch update {
                    case let .append(values):
                        state.allProcesses.append(contentsOf: values.map { ProcessRoom(room: $0) })
                    case .clear:
                        state.allProcesses.removeAll()
                    case let .pushFront(room):
                        state.allProcesses.insert(ProcessRoom(room: room), at: 0)
                    case let .pushBack(room):
                        state.allProcesses.append(ProcessRoom(room: room))
                    case .popFront:
                        if !state.allProcesses.isEmpty {
                            state.allProcesses.removeFirst()
                        }
                    case .popBack:
                        if !state.allProcesses.isEmpty {
                            state.allProcesses.removeLast()
                        }
                    case let .insert(index, room):
                        state.allProcesses.insert(ProcessRoom(room: room), at: Int(index))
                    case let .set(index, room):
                        if state.allProcesses.indices.contains(Int(index)) {
                            state.allProcesses[Int(index)] = ProcessRoom(room: room)
                        }
                    case let .remove(index):
                        if state.allProcesses.indices.contains(Int(index)) {
                            state.allProcesses.remove(at: Int(index))
                        }
                    case let .truncate(length):
                        if state.allProcesses.count > Int(length) {
                            state.allProcesses.removeSubrange(Int(length)..<state.allProcesses.count)
                        }
                    case let .reset(values: values):
                        state.allProcesses = values.map { ProcessRoom(room: $0) }
                    }
                }

                state.loadState = .loaded

                let currentIds = Set(state.allProcesses.map(\.id))
                var subscribeEffect: Effect<Action> = .none

                if currentIds != state.subscribedRoomIds {
                    state.subscribedRoomIds = currentIds
                    let ids = Array(currentIds)
                    subscribeEffect = .run { _ in
                        await processListClient.subscribeToRooms(ids)
                    }
                }

                if let topID = state.path.ids.last,
                   let selectedId = state.path[id: topID]?.process.id,
                   let updated = state.allProcesses.first(where: { $0.id == selectedId }),
                   updated !== state.path[id: topID]?.process {
                    state.path[id: topID]?.process = updated
                    return .merge(subscribeEffect, .send(.path(.element(id: topID, action: .onAppear))))
                }
                return subscribeEffect

            case let .processSelected(process):
                state.path.append(.initial(process: process))
                return .none

            case .path:
                return .none
            }
        }
        .forEach(\.path, action: \.path) {
            MessagesOverviewFeature()
        }
    }
}

// Client for dependency injection of initial state

struct ProcessesInitialStateClient {
    var initialState: @Sendable () -> ProcessesViewFeature.State
}

extension ProcessesInitialStateClient: DependencyKey {
    static let liveValue = ProcessesInitialStateClient {
        if let scenarioName = ProcessInfo.processInfo.environment["UI_TEST_PROCESSES_SCENARIO"],
           let scenario = ProcessesTestScenario(rawValue: scenarioName) {
            return scenario.makeState()
        }
        return ProcessesViewFeature.State()
    }

    static let testValue = ProcessesInitialStateClient {
        ProcessesViewFeature.State()
    }
}

extension DependencyValues {
    var processesInitialState: ProcessesInitialStateClient {
        get { self[ProcessesInitialStateClient.self] }
        set { self[ProcessesInitialStateClient.self] = newValue }
    }
}

extension ProcessesViewFeature.State {
    static func initial() -> ProcessesViewFeature.State {
        @Dependency(\.processesInitialState) var client
        return client.initialState()
    }
}
