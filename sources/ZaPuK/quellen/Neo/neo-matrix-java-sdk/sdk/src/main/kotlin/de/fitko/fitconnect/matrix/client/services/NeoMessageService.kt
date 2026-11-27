// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.services

import de.fitko.fitconnect.matrix.client.api.Case
import de.fitko.fitconnect.matrix.client.api.SendAnswerRequest
import de.fitko.fitconnect.matrix.client.api.SendMessageRequest
import de.fitko.fitconnect.matrix.client.exceptions.NeoException
import de.fitko.fitconnect.matrix.model.ProcessStatus
import dev.fitko.neo.matrix.sdk.MediaSource
import dev.fitko.neo.matrix.sdk.NeoCase
import dev.fitko.neo.matrix.sdk.NeoFileInfo
import dev.fitko.neo.matrix.sdk.ProgressWatcher
import dev.fitko.neo.matrix.sdk.TransmissionProgress
import dev.fitko.neo.matrix.sdk.messageService
import jakarta.inject.Inject
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.future.asCompletableFuture
import java.util.concurrent.CompletableFuture

class NeoMessageService @Inject constructor(
    private val clientService: NeoClientService,
) {
    private val coroutineScope = CoroutineScope(Dispatchers.Default)

    @JvmOverloads
    fun setMetadata(
        case: Case,
        service: String,
        status: ProcessStatus? = null
    ): CompletableFuture<String?> {
        val client = clientService.getInstance()

        val room = client.getRoom(case.caseId)
            ?: throw NeoException("Could not find case $case")

        return coroutineScope.async {
            room.setTopic(
                when (status) {
                    null -> service
                    ProcessStatus.RECEIVED ->
                        String.format("%s (neu)", service)
                    ProcessStatus.ACTIVITY_REQUIRED ->
                        String.format("%s (erforderlich)", service)
                    ProcessStatus.COMPLETED ->
                        String.format("%s (abgeschlossen)", service)
                }
            )
            null
        }.asCompletableFuture()
    }

    fun sendMessage(messageRequest: SendMessageRequest): CompletableFuture<String?> {
        val client = clientService.getInstance()

        return coroutineScope.async {
            messageService(client).sendEnvelope(
                messageRequest.caseId,
                messageRequest.content,
            )
        }.asCompletableFuture()
    }

    fun sendAnswer(answerRequest: SendAnswerRequest): CompletableFuture<String?> {
        val client = clientService.getInstance()

        return coroutineScope.async {
            messageService(client).sendAnswer(
                answerRequest.caseId,
                answerRequest.dialogId,
                answerRequest.content,
            )
        }.asCompletableFuture()
    }

    private suspend fun suspendUploadFile(
        fileName: String,
        mimeType: String,
        data: ByteArray
    ): NeoFileInfo {
        val client = clientService.getInstance()
        val status = MutableStateFlow<TransmissionProgress?>(null)
        val url = client.uploadMedia(
            mimeType,
            data,
            progressWatcher = object : ProgressWatcher {
                override fun transmissionProgress(progress: TransmissionProgress) {
                    status.value = progress
                }
            })
        status.first { it != null && it.current == it.total }
        return NeoFileInfo(
            url,
            fileName,
            mimeType,
            status.value!!.total,
        )
    }

    fun uploadFile(
        fileName: String,
        mimeType: String,
        data: ByteArray
    ): CompletableFuture<NeoFileInfo> {
        return coroutineScope.async {
            suspendUploadFile(fileName, mimeType, data)
        }.asCompletableFuture()
    }

    private suspend fun suspendDownloadFile(url: String): ByteArray {
        val client = clientService.getInstance()
        return client.getMediaContent(MediaSource.fromUrl(url))
    }

    fun downloadFile(url: String): CompletableFuture<ByteArray> {
        return coroutineScope.async {
            suspendDownloadFile(url)
        }.asCompletableFuture()
    }

    fun loadCase(caseId: String): CompletableFuture<NeoCase> {
        val client = clientService.getInstance()

        return coroutineScope.async {
            messageService(client).loadCase(caseId)
        }.asCompletableFuture()
    }
}
