# FIT-Connect SDK — ZBP Client

Client for sending messages and status updates to the
[Zentrales Bürgerpostfach (ZBP)](https://docs.fitko.de/fit-connect/docs/zbp/) of BundID.

## Install

```xml
<dependency>
  <groupId>dev.fitko.fitconnect</groupId>
  <artifactId>zbp-client</artifactId>
  <version>4.0.0</version>
</dependency>
```

## Quick start

```java
ZBPCertConfig certConfig = new ZBPCertConfig(
    Path.of("/path/to/private.key"),
    Path.of("/path/to/client.crt")
);

ZBPClient client = ZBPClientFactory.createZBPClient(certConfig, ZBPEnvironment.INT_INTERNET);
```

The client authenticates via mutual TLS using the provided RSA key pair.

## Sending a message

```java
CreateMessage message = CreateMessage.builder()
    .mailboxUuid(UUID.fromString("..."))
    .applicationId(submissionId)
    .title("Ihr Antrag wurde bearbeitet")
    .content("Bitte prüfen Sie die Rückmeldung.")
    .sender("Musteramt")
    .build();

CreateMessageResponse response = client.sendMessage(message);
```

### With attachments

```java
ZBPApiAttachment attachment = new ZBPApiAttachment(pdfBytes, "bescheid.pdf", "application/pdf");

// Attachment metadata (SHA-512 hash + content-length) is computed automatically
CreateMessage message = CreateMessage.builder()
    .mailboxUuid(mailboxId)
    .attachmentMetadata(List.of(ZBPAttachmentMetadata.from(attachment)))
    .build();

CreateMessageResponse response = client.sendMessageWithAttachments(message, List.of(attachment));
```

### On behalf of an author

Pass the author's certificate and JWT token when your system acts as a sender
on behalf of a third party:

```java
client.sendMessage(message, authorCertificatePem, authorToken);
client.sendMessageWithAttachments(message, attachments, authorCertificatePem, authorToken);
```

## Sending a status update

```java
CreateState state = CreateState.builder()
    .applicationId(submissionId)
    .state(State.PROCESSING)
    .senderName("Musteramt")
    .createdDate(Instant.now())
    .build();

client.createState(state);
// or on behalf of an author:
client.createState(state, authorCertificatePem, authorToken);
```

Available states: `INITIATED` · `SUBMITTED` · `RECEIVED` · `PROCESSING` · `ACTION_REQUIRED` · `COMPLETED`

## Sending via FIT-Connect (outside Verbindungsnetz)

If your application runs outside the Verbindungsnetz, use the FIT-Connect ZBP adapter instead
of calling the ZBP API directly. Build `message` / `state` as shown above, sign them with the
author's key pair and send the result as a submission via `sdk-client` (separate dependency).
See [ZBP documentation](https://docs.fitko.de/fit-connect/docs/zbp/) for background.

### Setup

```java
AuthorKeyPair authorKeyPair = AuthorKeyPair.builder()
    .authorCertificatePath(Path.of("/path/to/client.crt"))
    .authorPrivateKeyPath(Path.of("/path/to/private.key"))
    .build();

// sdk-client (add separately — not a zbp-client dependency)
FitConnectSdk sdk = FitConnectSdk.fromConfigBuilder()
    .credentials("your-client-id", "your-client-secret")
    .environment(FitConnectEnvironment.TEST)
    .build();
OnlineService sender = sdk.onlineService(yourDestinationId); // or sdk.organisation(...) for type A/B

UUID adapterId = UUID.fromString("b21c51d0-324b-4df4-ae6c-c0568a2bb067"); // TEST
URI schemaUri = URI.create("https://schema.fitko.de/fit-connect/id.bund.de/message_v6/1.0.0/zbp-message.schema.json");
```

### Message

```java
// Sign and wrap — message as shown above
byte[] envelopeJson = ZBPEnvelopeBuilder.fromAuthorPayload(message, authorKeyPair);

Participant zbpAdapter = Participant.of(adapterId,
    Addressing.toService("urn:schema-fitko-de:fit-connect:id.bund.de:message_v6", "ZBP Message"));

SentSubmission sent = sender.send(OutgoingSubmission.to(zbpAdapter)
    .setData(SubmissionData.json(envelopeJson, schemaUri))
    .build());

// acceptance/rejection arrives asynchronously via the event log
EventState status = sender.cases().logOf(sent).latest().state();
```

Attachments travel as FIT-Connect attachments of the submission. Their file names must match
the `attachmentMetadata` of the message:

```java
CreateMessage message = CreateMessage.builder()
    .mailboxUuid(mailboxId)
    .attachmentMetadata(List.of(ZBPAttachmentMetadata.from("bescheid.pdf", pdfBytes)))
    .build();
byte[] envelopeJson = ZBPEnvelopeBuilder.fromAuthorPayload(message, authorKeyPair);

sender.send(OutgoingSubmission.to(zbpAdapter)
    .setData(SubmissionData.json(envelopeJson, schemaUri))
    .addAttachment(Attachment.builder()
        .fromBytes(pdfBytes)
        .mimeType("application/pdf")
        .fileName("bescheid.pdf")
        .build())
    .build());
```

### Status update

```java
// Sign and wrap — state as shown above
byte[] envelopeJson = ZBPEnvelopeBuilder.fromAuthorPayload(state, authorKeyPair);

Participant zbpAdapter = Participant.of(adapterId,
    Addressing.toService("urn:schema-fitko-de:fit-connect:id.bund.de:status_v6", "ZBP Status"));

sender.send(OutgoingSubmission.to(zbpAdapter)
    .setData(SubmissionData.json(envelopeJson, schemaUri))
    .build());
```

**Adapter destination IDs:** TEST `b21c51d0-324b-4df4-ae6c-c0568a2bb067` · STAGE/PROD contact Anbindungsmanagement.

The adapter responds asynchronously with an `accept-submission` or `reject-submission` event.
`ZBPEnvelopeBuilder` and `AuthorKeyPair` are part of `zbp-client`; `FitConnectSdk`, `OnlineService`,
`OutgoingSubmission`, `Participant`, `Addressing`, `SubmissionData` and `Attachment` come from
`sdk-client`.

## Environments

| Constant                      | URL                                       | Network     |
|-------------------------------|-------------------------------------------|-------------|
| `ZBPEnvironment.INT_INTERNET` | `https://int.zbp.bund.de`                | Internet    |
| `ZBPEnvironment.INT_NDB`      | `https://int.zbp.bmi.in.bund.de`         | NdB         |
| `ZBPEnvironment.INT_NDB_VN`   | `https://int.zbp.bundid.doi-de.net`      | NdB-VN      |
| `ZBPEnvironment.PROD_NDB`     | `https://prod.zbp.bmi.in.bund.de`        | NdB         |
| `ZBPEnvironment.PROD_NDB_VN`  | `https://prod.zbp.bundid.doi-de.net`     | NdB-VN      |

## HTTP configuration

Customize timeouts, retries, or proxy settings via `HttpConfig`:

```java
HttpConfig httpConfig = HttpConfig.builder()
    .retryConfig(RetryConfig.builder().maxRetryCount(3).initialDelayInMs(200).build())
    .timeouts(TimeoutConfig.builder().readTimeout(30).build())
    .build();

ZBPClient client = ZBPClientFactory.createZBPClient(certConfig, ZBPEnvironment.PROD_NDB, httpConfig);
```

## Certificate configuration

`ZBPCertConfig` accepts either file paths or PEM strings:

```java
// From files
new ZBPCertConfig(Path.of("private.key"), Path.of("client.crt"));

// From PEM strings
new ZBPCertConfig(privateKeyPem, clientCertPem);

// Advanced: pin a specific ZBP server certificate (skips system trust store)
new ZBPCertConfig(rsaKey, clientCertPem, zbpServerCertPem);
```

## Error handling

All client methods throw `ZBPException` (unchecked) on failure.
Use `getApiException()` to inspect the HTTP status code and response body. Network and TLS
failures also arrive as a `RestApiException`, with status 500, no body and the `IOException` as cause:

```java
try {
    client.sendMessage(message);
} catch (ZBPException e) {
    e.getApiException()
        .filter(api -> !(api.getCause() instanceof IOException))
        .ifPresent(api -> log.error("ZBP rejected request: HTTP {} — {}",
            api.getStatusCode(), api.getResponseBody()));
}
```

## Build and test

```bash
./mvnw -pl zbp-client -am test
```

## License

Source code is licensed under the [EUPL](https://joinup.ec.europa.eu/page/eupl-text-11-12).

*Rechtlicher Hinweis: Das ZBP-Modul ist dazu bestimmt, die Anbindung einer Software an die
FIT-Connect-Infrastruktur zu ermöglichen.
Hierfür kann das Modul in die anzubindende Software integriert werden.
Erfolgt die Integration des Moduls in unveränderter Form, liegt keine Bearbeitung im Sinne der EUPL bzw. des deutschen
Urheberrechts vor.
Die Art und Weise der Verlinkung des Moduls führt insbesondere nicht zur Schaffung eines abgeleiteten Werkes.
Die unveränderte Übernahme des SDK in eine anzubindende Software führt damit nicht dazu, dass die anzubindende Software
unter den Bedingungen der EUPL zu lizenzieren ist.
Für die Weitergabe des Moduls selbst - in unveränderter oder bearbeiteter Form, als Quellcode oder ausführbares Programm -
gelten die Lizenzbedingungen der EUPL in unveränderter Weise.*
