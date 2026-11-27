//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Combine
import ComposableArchitecture
import Foundation
import MatrixRustSDK
import OSLog

enum AppStateError: Error {
    case matrixClientNotSet
    case sessionNotAvailable
    case userIdNotAvailable
}

@MainActor class AppState {
    static let shared = AppState()

    private let logger = Logger.logger(for: AppState.self)

    private var client: ClientProtocol?
    private var matrixClient: MatrixClient?
    private var displayName: String?
    private var userId: String?

    let clientFactory: MatrixClientFactory

    private var syncServiceState: SyncServiceState = .terminated
    let syncServiceStatePublisher = PassthroughSubject<SyncServiceState, Never>()

    let roomListUpdatesPublisher = PassthroughSubject<[RoomListEntriesUpdate], Never>()
    let roomListLoadingPublisher = PassthroughSubject<RoomListServiceSyncIndicator, Never>()

    private var cancellables = Set<AnyCancellable>()

    var authService: AuthenticationServiceProtocol

    init() {
        clientFactory = MatrixClientFactory()
        authService = AuthenticationService(clientFactory: clientFactory)

        // The Matrix store holds decrypted content and crypto keys so it should
        // not be backed up.
        if let documentsURL = FileManager.default.urls(for: .documentDirectory,
                                                       in: .userDomainMask).first {
            FileManager.default.excludeFromBackup(documentsURL)
        }
    }

    var coordinator: AppCoordinator?

    func setCoordinator(_ coordinator: AppCoordinator) {
        self.coordinator = coordinator
    }

    func setClient(_ client: ClientProtocol) {
        self.client = client
    }

    func getMatrixClient() throws -> MatrixClient {
        if let client = matrixClient {
            client
        } else {
            throw AppStateError.matrixClientNotSet
        }
    }

    func getSession() throws -> Session {
        if let session = try client?.session() {
            session
        } else {
            throw AppStateError.sessionNotAvailable
        }
    }

    func loadUserDetails() async {
        guard let client else {
            return
        }

        do {
            displayName = try await client.displayName()
            userId = try client.userId()
        } catch {
            logger.error("Unable to get display name: \(error)")
        }
    }

    func userDisplayName() throws -> String {
        guard let userId else {
            throw AppStateError.userIdNotAvailable
        }
        return displayName ?? userId
    }
}

extension AppState {
    func trySync() async {
        do {
            guard let client else { return }

            if matrixClient == nil {
                matrixClient = try await MatrixClient(client: client)
                if let matrixClient {
                    setupMatrixClientEventHandlers()
                    matrixClient.startSync()
                    await loadUserDetails()
                }
            } else {
                logger.info("client already initialized")
            }
        } catch {
            logger.error("An error ocurred during sync: \(error)")
        }
    }

    func resumeSync() async {
        if matrixClient == nil {
            await trySync()
        } else {
            matrixClient?.startSync()
        }
    }

    func pauseSync() {
        matrixClient?.stopSync()
    }

    func subscribeToRooms(_ roomIds: [String]) async {
        await matrixClient?.subscribeToRooms(roomIds)
    }

    func destroyClient() async {
        await matrixClient?.stopSyncAndWait()
        do {
            try await AppState.shared
                .getMatrixClient()
                .matrixClient().logout()
        } catch {
            logger.error("Matrix client already logged out")
        }

        do {
            try KeychainWrapper().purge()
        } catch {
            logger.error("Failed to remove keychain session data")
        }

        client = nil
        matrixClient = nil

        // Purge all locally cached session data (Matrix store, downloaded attachments)
        // so nothing sensitive from the session lingers on disk after logout.
        FileManager.default.clearAppData()
    }

    private func setupMatrixClientEventHandlers() {
        guard let client = matrixClient else { return }

        client.onSyncStateChanged = { [self] syncState in
            Task {
                await self.handleSyncStateChange(syncState)
            }
        }

        client.roomListLoadingSubject
            .removeDuplicates()
            .receive(on: DispatchQueue.main)
            .sink { [weak self] isLoading in
                self?.roomListLoadingPublisher.send(isLoading)
            }
            .store(in: &cancellables)

        client.roomListUpdatesSubject
            .sink { [weak self] updates in
                self?.roomListUpdatesPublisher.send(updates)
            }
            .store(in: &cancellables)
    }

    private func handleSyncStateChange(_ syncState: SyncServiceState) async {
        syncServiceState = syncState
        syncServiceStatePublisher.send(syncState)

        switch syncState {
        case .running:
            logger.info("Matrix sync is now running")
        case .idle:
            logger.info("Matrix sync is idle")
        case .terminated:
            logger.info("Matrix sync terminated")
        case .error:
            logger.info("Matrix sync encountered an error")
        case .offline:
            logger.info("Matrix sync is offline")
        }
    }
}
