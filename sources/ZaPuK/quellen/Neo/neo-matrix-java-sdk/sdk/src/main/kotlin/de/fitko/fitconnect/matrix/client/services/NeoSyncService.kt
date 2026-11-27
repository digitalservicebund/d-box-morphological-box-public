// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.services

import com.google.inject.Inject
import de.fitko.fitconnect.matrix.client.exceptions.NeoException
import de.fitko.fitconnect.matrix.client.util.NeoLifecycleHolder
import dev.fitko.neo.matrix.sdk.SyncService
import dev.fitko.neo.matrix.sdk.SyncServiceState
import dev.fitko.neo.matrix.sdk.SyncServiceStateObserver
import dev.fitko.neo.matrix.sdk.TaskHandle
import kotlinx.coroutines.runBlocking
import java.util.concurrent.CompletableFuture

class NeoSyncService @Inject constructor(
    private val clientService: NeoClientService
) : NeoLifecycleHolder<SyncService>() {
    private var syncFuture = CompletableFuture<Unit>()
    private var syncTask: TaskHandle? = null

    override suspend fun create(): SyncService {
        val client = clientService.getInstance()
        val syncService = client.syncService().finish()
        val handle = object : SyncServiceStateObserver {
            override fun onUpdate(state: SyncServiceState) {
                println("Sync State: $state")
                when (state) {
                    SyncServiceState.RUNNING ->
                        syncFuture.complete(Unit)
                    SyncServiceState.IDLE ->
                        syncFuture.complete(Unit)
                    SyncServiceState.TERMINATED ->
                        syncFuture.completeExceptionally(NeoException("Sync Terminated"))
                    SyncServiceState.ERROR ->
                        syncFuture.completeExceptionally(NeoException("Sync Error"))
                    SyncServiceState.OFFLINE ->
                        syncFuture.completeExceptionally(NeoException("Sync Offline"))
                }
            }
        }
        syncTask = syncService.state(handle)
        syncService.start()
        return syncService
    }

    fun syncOnce(): CompletableFuture<Unit> {
        return syncFuture
    }

    override fun destroy(value: SyncService) {
        syncTask?.cancel()
        syncTask = null
        runBlocking {
            value.stop()
        }
        value.destroy()
    }
}
