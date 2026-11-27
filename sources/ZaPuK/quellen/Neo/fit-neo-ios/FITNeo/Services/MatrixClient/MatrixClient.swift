//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Combine
import MatrixRustSDK
import OSLog

@MainActor
final class MatrixClient {
    private let logger = Logger.logger(for: MatrixClient.self)

    private let client: ClientProtocol

    private var syncService: SyncService
    private var syncServiceStateUpdateTaskHandle: TaskHandle?

    private var roomListService: RoomListService
    private var roomListEntriesHandle: RoomListEntriesWithDynamicAdaptersResult?
    private var roomListIndicatorTaskHandle: TaskHandle?

    /// Always run in the order requested instead of racing as detached tasks.
    private var syncLifecycleTask: Task<Void, Never>?

    var onSyncStateChanged: ((SyncServiceState) -> Void)?
    var onRoomListStateChanged: ((RoomListServiceState) -> Void)?

    var roomListLoadingSubject = PassthroughSubject<RoomListServiceSyncIndicator, Never>()
    var roomListUpdatesSubject = PassthroughSubject<[RoomListEntriesUpdate], Never>()

    init(client: ClientProtocol) async throws {
        self.client = client

        let services = try await MatrixClientServices(client: client)

        syncService = services.syncService
        roomListService = services.roomListService

        let listener = SDKListener<[RoomListEntriesUpdate]> { [weak self] update in
            Task { @MainActor in
                self?.roomListUpdatesSubject.send(update)
            }
        }
        roomListEntriesHandle = try await roomListService.allRooms()
            .entriesWithDynamicAdapters(pageSize: 100, listener: listener)
        _ = roomListEntriesHandle?.controller().setFilter(kind: .all(filters: [.nonLeft]))
    }

    func matrixClient() -> ClientProtocol {
        client
    }

    var homeServer: String {
        client.homeserver()
    }

    func startSync() {
        enqueueSyncTransition { [weak self] in
            guard let self else { return }
            await syncService.start()
            startRoomListIndicator()
        }
    }

    func stopSync() {
        enqueueSyncTransition { [weak self] in
            guard let self else { return }
            await syncService.stop()
            roomListIndicatorTaskHandle = nil
        }
    }

    func stopSyncAndWait() async {
        stopSync()
        await syncLifecycleTask?.value
    }

    func subscribeToRooms(_ roomIds: [String]) async {
        guard !roomIds.isEmpty else { return }
        do {
            try await roomListService.subscribeToRooms(roomIds: roomIds)
        } catch {
            logger.error("Failed subscribing to rooms: \(error)")
        }
    }

    private func enqueueSyncTransition(_ transition: @escaping @MainActor () async -> Void) {
        let previous = syncLifecycleTask
        syncLifecycleTask = Task { @MainActor in
            await previous?.value
            await transition()
        }
    }

    private func startRoomListIndicator() {
        let listener = SDKListener<RoomListServiceSyncIndicator> { [weak self] update in
            Task { @MainActor in
                self?.logger.info("Sync indicator update...")
                self?.roomListLoadingSubject.send(update)
            }
        }
        roomListIndicatorTaskHandle = roomListService
            .syncIndicator(delayBeforeShowingInMs: 0,
                           delayBeforeHidingInMs: 0,
                           listener: listener)
    }
}
