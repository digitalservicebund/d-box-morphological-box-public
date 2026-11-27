//
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import MatrixRustSDK

struct MatrixClientServices {
    let syncService: SyncService
    let roomListService: RoomListService

    init(client: ClientProtocol) async throws {
        syncService = try await client
            .syncService()
            .withOfflineMode()
            .finish()

        roomListService = syncService.roomListService()
    }
}
