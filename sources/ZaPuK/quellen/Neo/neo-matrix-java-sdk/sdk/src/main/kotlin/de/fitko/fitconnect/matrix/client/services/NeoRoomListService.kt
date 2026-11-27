// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.services

import com.google.inject.Inject
import de.fitko.fitconnect.matrix.client.util.NeoLifecycleHolder
import dev.fitko.neo.matrix.sdk.RoomListService
import dev.fitko.neo.matrix.sdk.RoomListServiceState
import dev.fitko.neo.matrix.sdk.RoomListServiceStateListener
import dev.fitko.neo.matrix.sdk.TaskHandle
import java.util.concurrent.CompletableFuture

class NeoRoomListService @Inject constructor(
    private val syncService: NeoSyncService
) : NeoLifecycleHolder<RoomListService>() {
    private var syncFuture = CompletableFuture<Unit>()
    private var syncTask: TaskHandle? = null

    override suspend fun create(): RoomListService {
        val sync = syncService.getInstance()
        val roomListService = sync.roomListService()
        val handle = object : RoomListServiceStateListener {
            override fun onUpdate(state: RoomListServiceState) {
                println("Sync State: $state")
                if (state == RoomListServiceState.RUNNING) {
                    syncFuture.complete(Unit)
                }
            }
        }
        syncTask = roomListService.state(handle)
        return roomListService
    }

    fun syncOnce(): CompletableFuture<Unit> {
        return syncFuture
    }

    override fun destroy(value: RoomListService) {
        syncTask?.cancel()
        syncTask = null
        value.destroy()
    }
}
