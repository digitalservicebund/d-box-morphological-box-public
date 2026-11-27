// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package com.example.neo;

import java.nio.charset.StandardCharsets;
import java.util.List;
import java.util.Optional;
import java.util.Random;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.TimeUnit;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import de.fitko.fitconnect.matrix.client.NeoSdk;
import de.fitko.fitconnect.matrix.client.api.Case;
import de.fitko.fitconnect.matrix.client.api.SendAnswerRequest;
import de.fitko.fitconnect.matrix.client.api.SendMessageRequest;
import de.fitko.fitconnect.matrix.client.clients.NeoSenderClient;
import de.fitko.fitconnect.matrix.client.config.NeoEnvironment;
import de.fitko.fitconnect.matrix.model.ProcessStatus;
import dev.fitko.neo.matrix.sdk.NeoBuildingBlock;
import dev.fitko.neo.matrix.sdk.NeoCase;
import dev.fitko.neo.matrix.sdk.NeoContent;
import dev.fitko.neo.matrix.sdk.NeoDialog;
import dev.fitko.neo.matrix.sdk.NeoFileInfo;
import dev.fitko.neo.matrix.sdk.NeoHyperlink;
import dev.fitko.neo.matrix.sdk.NeoPollAnswer;
import dev.fitko.neo.matrix.sdk.NeoPollContent;
import dev.fitko.neo.matrix.sdk.NeoText;

public class Main {
    public static void main(String... args) throws Exception {
        NeoSdk sdk = NeoSdk.fromConfigBuilder()
                .environment(NeoEnvironment.custom("TEST", System.getenv("MATRIX_HOST")))
                .auth(System.getenv("MATRIX_USER"), System.getenv("MATRIX_PASS"))
                .encryption(System.getenv("MATRIX_KEY"))
                .build();

        NeoSenderClient client = sdk.createSenderClient();
        client.start().get(10, TimeUnit.SECONDS);
        client.syncOnce().get(10, TimeUnit.SECONDS);

        Case caseHandle = client.createCase("@justjanne:ess1.dev.neo.fitko.dev", generateRoomName()).get(10, TimeUnit.SECONDS);
        //Case caseHandle = client.getCase("!qPFAaSNzboqzUSOjGX:ess1.dev.neo.fitko.dev").get(10, TimeUnit.SECONDS);
        System.out.println(caseHandle);

        client.setMetadata(caseHandle, "Bundeszentralamt für Dezentralisierung", ProcessStatus.ACTIVITY_REQUIRED);

        NeoFileInfo fileInfo = client.uploadFile("question.txt", "text/plain", "The question is: what is the question?".getBytes(StandardCharsets.UTF_8))
                .get(10, TimeUnit.SECONDS);

        List<NeoContent> messages = List.of(
                new NeoContent.Message(
                        new NeoText("Lorem ipsum i, dolor sit amet", null),
                        null
                ),
                new NeoContent.File(
                        fileInfo,
                        new NeoText("Attachment", null),
                        null
                ),
                new NeoContent.Button(
                        new NeoHyperlink(
                                new NeoText("Open Website", null),
                                "https://drachen-einfach.de"
                        )
                ),
                new NeoContent.Infobox(
                        new NeoText("Please note", null),
                        new NeoText("Hello world", "<i>Hello world</i>"),
                        new NeoHyperlink(
                                new NeoText("Open Website", null),
                                "https://drachentermin.de"
                        )
                ),
                new NeoContent.InquiryPoll(
                        new NeoPollContent(
                                false,
                                new NeoText("Color of the dragon", null),
                                List.of(
                                        new NeoPollAnswer("#ff0000", new NeoText("Red", null)),
                                        new NeoPollAnswer("#0000ff", new NeoText("Blue", null)),
                                        new NeoPollAnswer("dev.fitko.neo.poll.custom.v1", new NeoText("Custom", null))
                                )
                        )
                )
        );

        String dialogId = client.sendMessage(SendMessageRequest.builder()
                        .caseId(caseHandle.caseId())
                        .content(messages)
                        .build())
                .get(30, TimeUnit.SECONDS);

        String pollId = getPollId(client, caseHandle.caseId(), dialogId)
                .get(30, TimeUnit.SECONDS);

        System.out.println("AnswerId: " + client.sendAnswer(
                        SendAnswerRequest.builder()
                                .caseId(caseHandle.caseId())
                                .dialogId(dialogId)
                                .content(List.of(
                                        new NeoContent.Poll(
                                                List.of("red"),
                                                null,
                                                pollId
                                        )
                                )).build())
                .get(30, TimeUnit.SECONDS));

        System.out.println("AnswerId: " + client.sendAnswer(
                        SendAnswerRequest.builder()
                                .caseId(caseHandle.caseId())
                                .dialogId(dialogId)
                                .content(List.of(
                                        new NeoContent.Poll(
                                                List.of("dev.fitko.neo.poll.custom.v1"),
                                                "Grün",
                                                pollId
                                        )
                                )).build())
                .get(30, TimeUnit.SECONDS));

        System.out.println("EventId: " + client.sendMessage(
                        SendMessageRequest.builder()
                                .caseId(caseHandle.caseId())
                                .content(messages)
                                .build())
                .get(30, TimeUnit.SECONDS));

        System.out.println("Case: " + client.loadCase(caseHandle.caseId()).get(10, TimeUnit.SECONDS));

        byte[] downloadedFile = client.downloadFile(fileInfo.getUri()).get(10, TimeUnit.SECONDS);
        System.out.println("Download: " + new String(downloadedFile, StandardCharsets.UTF_8));

        // Cleanup
        client.close();
    }

    private static final List<String> LOREM_IPSUM = List.of(
            "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua Ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur Excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum".split(" ")
    );

    private static final Stream<String> RANDOM_WORDS =
            new Random()
                    .ints(0, LOREM_IPSUM.size())
                    .mapToObj(LOREM_IPSUM::get);

    private static String generateRoomName() {
        return RANDOM_WORDS.limit(3).collect(Collectors.joining(" "));
    }

    private static CompletableFuture<String> getPollId(
            NeoSenderClient client,
            String caseId,
            String dialogId
    ) {
        return client.loadCase(caseId).thenApply(caseItem ->
                getDialog(caseItem, dialogId)
                        .flatMap(Main::getPollEvent)
                        .map(NeoBuildingBlock::getId)
                        .orElse(null));
    }

    private static Optional<NeoDialog> getDialog(NeoCase item, String dialogId) {
        return item.getDialogs().stream()
                .filter(it -> it.getId().equals(dialogId))
                .findFirst();
    }

    private static Optional<NeoBuildingBlock> getPollEvent(NeoDialog item) {
        return item.getMessage()
                .getBlocks()
                .stream()
                .filter(block -> block.getContent() instanceof NeoContent.InquiryPoll)
                .findFirst();
    }
}
