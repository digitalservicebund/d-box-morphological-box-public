// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.services

import com.google.inject.Inject
import java.util.concurrent.CompletableFuture

class NeoLifecycleService @Inject constructor(
    private val client: NeoClientService,
    private val sync: NeoSyncService,
    private val roomList: NeoRoomListService,
) {
    fun start(): CompletableFuture<Unit> =
        client.start()
            .thenCompose { sync.start() }
            .thenCompose { roomList.start() }

    fun syncOnce(): CompletableFuture<Unit> =
        sync.syncOnce()
            .thenCompose { roomList.syncOnce() }

    fun close() {
        roomList.close()
        sync.close()
        client.close()
    }
}
