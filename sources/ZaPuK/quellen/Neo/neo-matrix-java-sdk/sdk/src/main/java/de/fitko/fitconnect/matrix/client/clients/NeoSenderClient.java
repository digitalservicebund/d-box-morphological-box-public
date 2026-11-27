// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.clients;

import java.util.List;
import java.util.concurrent.CompletableFuture;

import com.google.inject.Inject;
import de.fitko.fitconnect.matrix.client.api.Case;
import de.fitko.fitconnect.matrix.client.api.SendAnswerRequest;
import de.fitko.fitconnect.matrix.client.api.SentMessage;
import de.fitko.fitconnect.matrix.client.api.SendMessageRequest;
import de.fitko.fitconnect.matrix.client.services.NeoCaseService;
import de.fitko.fitconnect.matrix.client.services.NeoLifecycleService;
import de.fitko.fitconnect.matrix.client.services.NeoMessageService;
import de.fitko.fitconnect.matrix.model.ProcessStatus;
import dev.fitko.neo.matrix.sdk.NeoCase;
import dev.fitko.neo.matrix.sdk.NeoFileInfo;
import kotlin.Unit;

public class NeoSenderClient {
    private final NeoCaseService caseService;
    private final NeoMessageService messageService;
    private final NeoLifecycleService lifecycleService;

    @Inject
    public NeoSenderClient(
            NeoLifecycleService lifecycleService,
            NeoCaseService caseService,
            NeoMessageService messageService
    ) {
        this.lifecycleService = lifecycleService;
        this.caseService = caseService;
        this.messageService = messageService;
    }

    public CompletableFuture<Unit> start() {
        return lifecycleService.start();
    }

    public CompletableFuture<Unit> syncOnce() {
        return lifecycleService.syncOnce();
    }

    public void close() {
        lifecycleService.close();
    }

    /*
    We can't provide sent status for messages just yet
    public Status getMessageStatus(SentMessage sentMessage) {
        log.info("Loading status for message {}", sentMessage.transactionId());
        return eventLogService.getMessageStatus(sentMessage);
    }
     */

    public CompletableFuture<String> setMetadata(Case caseHandle, String service) {
        return messageService.setMetadata(caseHandle, service);
    }

    public CompletableFuture<String> setMetadata(Case caseHandle, String service, ProcessStatus processStatus) {
        return messageService.setMetadata(caseHandle, service, processStatus);
    }

    public CompletableFuture<NeoFileInfo> uploadFile(String fileName, String mimeType, byte[] data) {
        return messageService.uploadFile(fileName, mimeType, data);
    }

    public CompletableFuture<byte[]> downloadFile(String url) {
        return messageService.downloadFile(url);
    }

    public CompletableFuture<String> sendMessage(SendMessageRequest messageRequest) {
        return messageService.sendMessage(messageRequest);
    }

    public CompletableFuture<String> sendAnswer(SendAnswerRequest answerRequest) {
        return messageService.sendAnswer(answerRequest);
    }

    public CompletableFuture<Case> createCase(String mailboxId, String processName) {
        return caseService.createCase(mailboxId, processName);
    }

    public CompletableFuture<Case> getCase(String caseId) {
        return caseService.getCase(caseId);
    }

    public List<Case> getAllCases() {
        return caseService.getAllCases();
    }

    public CompletableFuture<NeoCase> loadCase(String caseId) {
        return messageService.loadCase(caseId);
    }
}
