//
// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Combine
import ComposableArchitecture
import MatrixRustSDK

struct ProcessListClient {
    var loadingEvents: @Sendable () -> AnyPublisher<RoomListServiceSyncIndicator, Never>
    var updateEvents: @Sendable () -> AnyPublisher<[RoomListEntriesUpdate], Never>
    var sync: @Sendable () async -> Void
    var subscribeToRooms: @Sendable ([String]) async -> Void
}

extension ProcessListClient: DependencyKey {
    static let liveValue =
        ProcessListClient(loadingEvents: {
                              MainActor.assumeIsolated {
                                  AppState.shared.roomListLoadingPublisher.eraseToAnyPublisher()
                              }
                          },
                          updateEvents: {
                              MainActor.assumeIsolated {
                                  AppState.shared.roomListUpdatesPublisher.eraseToAnyPublisher()
                              }
                          },
                          sync: {
                              await AppState.shared.trySync()
                          },
                          subscribeToRooms: { roomIds in
                              await AppState.shared.subscribeToRooms(roomIds)
                          })

    static let testValue = ProcessListClient(loadingEvents: { Empty().eraseToAnyPublisher() },
                                             updateEvents: { Empty().eraseToAnyPublisher() },
                                             sync: { },
                                             subscribeToRooms: { _ in })
}

extension DependencyValues {
    var processListClient: ProcessListClient {
        get { self[ProcessListClient.self] }
        set { self[ProcessListClient.self] = newValue }
    }
}
