// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.services

import de.fitko.fitconnect.matrix.client.api.Case
import dev.fitko.neo.matrix.sdk.CreateRoomParameters
import dev.fitko.neo.matrix.sdk.RoomPreset
import dev.fitko.neo.matrix.sdk.RoomVisibility
import jakarta.inject.Inject
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.future.asCompletableFuture
import java.util.concurrent.CompletableFuture

class NeoCaseService @Inject constructor(
    private val clientService: NeoClientService
) {
    private val coroutineScope = CoroutineScope(Dispatchers.Default)

    fun createCase(
        mailboxId: String,
        processName: String
    ): CompletableFuture<Case> {
        val client = clientService.getInstance()

        return coroutineScope.async {
            val roomId = client.createRoom(
                CreateRoomParameters(
                    name = processName,
                    isDirect = false,
                    invite = listOf(mailboxId),
                    isEncrypted = false,
                    visibility = RoomVisibility.Private,
                    preset = RoomPreset.TRUSTED_PRIVATE_CHAT,
                )
            )
            Case(roomId, processName)
        }.asCompletableFuture()
    }

    fun getCase(caseId: String): CompletableFuture<Case?> {
        val client = clientService.getInstance()

        return coroutineScope.async {
            client.getRoom(caseId)?.let { room ->
                Case(room.id(), room.displayName())
            }
        }.asCompletableFuture()
    }

    fun getAllCases(): List<Case> {
        val client = clientService.getInstance()

        return client.rooms().map { Case(it.id(), it.displayName()) }
    }
}
