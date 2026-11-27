<!--
SPDX-FileCopyrightText: 2025, 2026 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Usage

[[_TOC_]]

## Note: Usage of Futures

Note that the `.get(10, TimeUnit.SECONDS)` I'm using in these snippets is
just for illustration. For production it's recommended to properly chain
the Completable Futures.

A good primer on completable futures is available at <https://www.baeldung.com/java-completablefuture>

## Initialize SDK

```java
NeoSdk sdk = NeoSdk.fromConfigBuilder()
    .environment(NeoEnvironment.custom("TEST", System.getenv("MATRIX_HOST")))
    .auth(System.getenv("MATRIX_USER"), System.getenv("MATRIX_PASS"))
    .encryption(System.getenv("MATRIX_KEY"))
    .build();
```

## Start and stop background tasks

```java
NeoSenderClient client = sdk.createSenderClient();
client.start().get(10, TimeUnit.SECONDS);
client.syncOnce().get(10, TimeUnit.SECONDS);

client.close();
```

## Create a new Case

```java
Case caseHandle = client.createCase(
    "@justjanne:ess1.dev.neo.fitko.dev", 
    "Anfrage auf Herausgabe von Kontrollberichten gem § 2 Abs. 1 Verbraucherinformationsgesetz (VIG)"
).get(10, TimeUnit.SECONDS);
```

## Fetch an existing case

```java
Case caseHandle = client.getCase("!qPFAaSNzboqzUSOjGX:ess1.dev.neo.fitko.dev").get(10, TimeUnit.SECONDS);
```

## Create message building blocks

```java
new NeoContent.Message(
    new NeoText("Lorem ipsum i, dolor sit amet", null),
    null
)
```

```java
new NeoContent.Button(
    new NeoHyperlink(
        new NeoText("Open Website", null),
        "https://drachen-einfach.de"
    )
)
```

## Send Messages

```java
String dialogId = client.sendMessage(SendMessageRequest.builder()
    .caseId(caseHandle.caseId())
    .content(Arrays.asList(
        // ...
    )).build())
    .get(30, TimeUnit.SECONDS);
```

## Send an answer to an existing message

```java
client.sendAnswer(SendAnswerRequest.builder()
    .caseId(caseHandle.caseId())
    .dialogId(dialogId)
    .content(List.of(
        new NeoContent.Message(
            new NeoText("Lorem ipsum i, dolor sit amet", null),
            originalMessageId
        )
    )).build())
    .get(30, TimeUnit.SECONDS);
```

## Upload files

```java
NeoFileInfo fileInfo = client.uploadFile("question.txt",  "text/plain",  new byte[] { /* ... */}).get(10, TimeUnit.SECONDS);
```

```java
new NeoContent.File(
    fileInfo,
    new NeoText("Attachment", null),
    null,
)
```

## Download files

```java
byte[] content = client.downloadFile("mxc://ess1.dev.neo.fitko.dev/GxroBhfPlWldwEiHZStvEGDa");
```

## Update case metadata

```java
client.setMetadata(caseHandle, "Bundeszentralamt für Dezentralisierung", ProcessStatus.ACTIVITY_REQUIRED);
```
