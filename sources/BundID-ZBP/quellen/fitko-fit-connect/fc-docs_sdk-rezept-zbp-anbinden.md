# Zentrales Bürgerpostfach (ZBP)

- Quelle: <https://docs.fitko.de/fit-connect/docs/sdks/rezepte/zbp>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Zentrales Bürgerpostfach (ZBP) anbinden

   RolleVerwaltungssystem
   SDK · ModulJava · zbp-client · 4.0.0.NET · Fitko.FitConnect.Zbp · 4.0.0
   VoraussetzungenZugelassenes Client-Zertifikat für das ZBP (RSA, PEM), Postkorb-Handle (mailboxUuid) der Adressat:innen. Für den Weg über FIT-Connect
   zusätzlich ein eigener Zustellpunkt vom Typ C (Sie treten als Sender auf) und die destinationId des ZBP-Adapters.

   TL;DR – ZBP-Modul installieren, Client mit mTLS-Zertifikat einrichten, dann CreateMessage (Nachricht) oder CreateState (Statusmeldung) bauen. Innerhalb
   des Verbindungsnetzes senden Sie direkt an die ZBP-API; außerhalb verpacken Sie dieselbe Payload signiert in eine normale FIT-Connect-Submission an den
   ZBP-Adapter. Hintergrund: FIT-Connect und das ZBP.

Zwei Wege, eine Payload

   Direkt an die ZBP-API Über den ZBP-Adapter von FIT-Connect
   Wann Ihre Anwendung läuft im Verbindungsnetz (NdB / NdB-VN) oder erreicht die INT-Umgebung über das Internet Ihre Anwendung läuft außerhalb des
   Verbindungsnetzes
   Transport Mutual TLS mit Ihrem Client-Zertifikat Verschlüsselte Submission an den Adapter-Zustellpunkt, signiert mit Ihrem Zertifikat
   Ergebnis Synchron (CreateMessageResponse) Asynchron — accept-submission / reject-submission im Ereignisprotokoll
   FIT-Connect-Rolle keine Sie senden — also OnlineService / AsOnlineService() mit eigenem Typ-C-Zustellpunkt

   CreateMessage und CreateState sind auf beiden Wegen identisch.

Modul einbinden

     * Java
     * .NET (C#)

   <dependency>

   <groupId>dev.fitko.fitconnect</groupId>
   <artifactId>zbp-client</artifactId>
   <version>4.0.0</version>
   </dependency>

   zbp-client hängt nicht von sdk-client ab — für den Weg über FIT-Connect binden Sie beide ein.

   dotnet add package Fitko.FitConnect.Zbp --version 4.0.0

   Registrierung per DI (liest die Sektion FitConnect:Zbp):

   using Fitko.FitConnect.Zbp.DependencyInjection;

   using Fitko.FitConnect.Zbp.Interfaces;
   var services = new ServiceCollection()
   .AddSingleton<IConfiguration>(configuration)
   .AddFitConnect() // nur nötig für den Weg über FIT-Connect
   .AddFitConnectZbp() // alternativ: AddFitConnectZbp("MeineSektion") oder AddFitConnectZbp(cfg => { ... })
   .BuildServiceProvider();
   IZbpClient zbpClient = services.GetRequiredService<IZbpClient>();

ZBP-Client konfigurieren

   Die Authentifizierung gegenüber dem ZBP erfolgt per Mutual TLS mit Ihrem RSA-Schlüsselpaar (privater Schlüssel und Client-Zertifikat im PEM-Format).
     * Java
     * .NET (C#)

   import dev.fitko.fitconnect.zbp.ZBPCertConfig;

   import dev.fitko.fitconnect.zbp.ZBPClient;
   import dev.fitko.fitconnect.zbp.ZBPClientFactory;
   import dev.fitko.fitconnect.zbp.ZBPEnvironment;
   ZBPCertConfig certConfig = new ZBPCertConfig(
   Path.of("/path/to/private.key"),
   Path.of("/path/to/client.crt")); // alternativ: PEM-Strings statt Pfaden
   ZBPClient zbpClient = ZBPClientFactory.createZBPClient(certConfig, ZBPEnvironment.INT_INTERNET);

   Timeouts, Retries und Proxy stellen Sie über dieselbe HttpConfig ein wie im sdk-client:

   HttpConfig httpConfig = HttpConfig.builder()

   .timeouts(TimeoutConfig.builder().readTimeout(30).build())
   .retryConfig(RetryConfig.builder().maxRetryCount(3).initialDelayInMs(200).build())
   .build();
   ZBPClient zbpClient = ZBPClientFactory.createZBPClient(certConfig, ZBPEnvironment.PROD_NDB, httpConfig);

   Eine eigene Umgebung (z. B. ein Testsystem) erzeugen Sie mit new ZBPEnvironment("MEIN_ZBP", "https://…"). Mit dem dritten Konstruktor von ZBPCertConfig
   können Sie zusätzlich das ZBP-Serverzertifikat pinnen (new ZBPCertConfig(rsaKey, clientCertPem, zbpServerCertPem)).

   {

   "FitConnect": {
   "Zbp": {
   "ZbpEnvironmentTag": "INT_INTERNET",
   "ClientCertPath": "certs/zbp/client.crt",
   "ClientPrivateKeyPath": "certs/zbp/private.key",
   "CaCertPath": "certs/zbp/ca.pem",
   "Proxy": { "Host": "", "Port": 0 }
   }
   }
   }
                 Schlüssel                                                             Bedeutung
   ZbpEnvironmentTag                     Eine der Umgebungen aus der Tabelle unten oder CUSTOM
   CustomBaseUrl                         Basis-URL, nur bei CUSTOM (Pflicht)
   SendCertAsCookie                      Nur bei CUSTOM: Zertifikat als Cookie statt per mTLS übermitteln, falls die Zielumgebung das erwartet
   ClientCertPath / ClientPrivateKeyPath Ihr Client-Zertifikat und privater Schlüssel (PEM)
   CaCertPath                            CA-Zertifikat des ZBP; wird als einziger Vertrauensanker verwendet
   Proxy                                 Optional, wie Http.Proxy im Basispaket

   Alle Pfade werden beim Start geprüft (ValidateOnStart) — fehlt eine Datei, ist der Tag unbekannt oder fehlt bei CUSTOM die CustomBaseUrl, schlägt der
   Host-Start fehl (ohne Host: spätestens das erste Auflösen von IZbpClient).

Umgebungen

          Java-Konstante                .NET-Tag                       URL                   Netz
   ZBPEnvironment.INT_INTERNET   INT_INTERNET           https://int.zbp.bund.de            Internet
   ZBPEnvironment.INT_NDB        INT_NDB                https://int.zbp.bmi.in.bund.de     NdB
   ZBPEnvironment.INT_NDB_VN     INT_NDB_VN             https://int.zbp.bundid.doi-de.net  NdB-VN
   ZBPEnvironment.PROD_NDB       PROD_NDB               https://prod.zbp.bmi.in.bund.de    NdB
   ZBPEnvironment.PROD_NDB_VN    PROD_NDB_VN            https://prod.zbp.bundid.doi-de.net NdB-VN
   new ZBPEnvironment(name, url) CUSTOM + CustomBaseUrl frei                               —

Nachricht bauen

   Das Postkorb-Handle der Adressat:innen (mailboxUuid) steht in der BundID unter Zugänge & Daten → Betroffenenauskunft. content darf HTML enthalten; die
   erlaubten Tags beschreibt die ZBP-Dokumentation.
     * Java
     * .NET (C#)

   import dev.fitko.fitconnect.zbp.model.AuthenticationLevel;

   import dev.fitko.fitconnect.zbp.model.CreateMessage;
   CreateMessage message = CreateMessage.builder()
   .mailboxUuid(UUID.fromString("..."))
   .applicationId(submissionId) // Bezug zum Antrag, optional
   .sender("Musteramt")
   .service("Antragsstatus")
   .title("Ihr Antrag wurde bearbeitet")
   .content("<p>Bitte prüfen Sie die Rückmeldung.</p>")
   .stork_qaa_level(AuthenticationLevel.ONE)
   .build();

   using Fitko.FitConnect.Zbp.Models;

   var message = new CreateMessage
   {
   MailboxGuid = Guid.Parse("..."),
   ApplicationId = submissionId, // Bezug zum Antrag, optional
   Sender = "Musteramt",
   Service = "Antragsstatus",
   Title = "Ihr Antrag wurde bearbeitet",
   Content = "<p>Bitte prüfen Sie die Rückmeldung.</p>",
   StorkQaaLevel = AuthenticationLevel.One
   };

   MailboxGuid, Sender, Service, Title und Content sind required.

   Optional sind außerdem retrievalConfirmationAddress (Abrufbestätigung), replyAddress, reference, senderUrl und die Anhang-Metadaten (siehe unten).

Direkt an die ZBP-API senden

     * Java
     * .NET (C#)

   CreateMessageResponse response = zbpClient.sendMessage(message);

   Mit Anhängen — die Metadaten (SHA-512, Länge) berechnet das SDK:

   import dev.fitko.fitconnect.zbp.model.ZBPApiAttachment;

   import dev.fitko.fitconnect.zbp.model.ZBPAttachmentMetadata;
   ZBPApiAttachment bescheid = new ZBPApiAttachment(pdfBytes, "bescheid.pdf", "application/pdf");
   CreateMessage message = CreateMessage.builder()
   // ... wie oben ...
   .attachmentMetadata(List.of(ZBPAttachmentMetadata.from(bescheid)))
   .build();
   CreateMessageResponse response = zbpClient.sendMessageWithAttachments(message, List.of(bescheid));

   CreateMessageResponse? response = await zbpClient.SendMessage(message);

   Mit Anhängen — die Metadaten (SHA-512, Länge) ergänzt das SDK automatisch:

   var bescheid = ZbpApiAttachment.FromFile("bescheid.pdf", "application/pdf");

   CreateMessageResponse? response = await zbpClient.SendMessage(message, [bescheid]);

  Statusmeldung senden

   Statusmeldungen beziehen sich über die applicationId auf einen Antrag und erscheinen im Postfach als Verlauf: INITIATED → SUBMITTED → RECEIVED →
   PROCESSING → ACTION_REQUIRED → COMPLETED.

     * Java
     * .NET (C#)

   import dev.fitko.fitconnect.zbp.model.CreateState;

   import dev.fitko.fitconnect.zbp.model.State;
   CreateState state = CreateState.builder()
   .applicationId(submissionId)
   .state(State.PROCESSING)
   .senderName("Musteramt")
   .createdDate(Instant.now())
   .build();
   zbpClient.createState(state);

   using Fitko.FitConnect.Zbp.Models.State;

   var state = new CreateState
   {
   ApplicationId = submissionId,
   State = ZbpState.Processing,
   SenderName = "Musteramt",
   CreatedDate = DateTimeOffset.UtcNow
   };
   await zbpClient.CreateState(state);

   Im Auftrag eines Autors senden

   Tritt Ihr System nur als Übermittler auf und stammt die Nachricht fachlich von einer dritten Anwendung, geben Sie deren Zertifikat und Token mit — der
   Eintrag im Postfach weist dann den Autor aus, nicht Sie:
     * Java
     * .NET (C#)

   zbpClient.sendMessage(message, authorCertificatePem, authorToken);

   zbpClient.sendMessageWithAttachments(message, attachments, authorCertificatePem, authorToken);
   zbpClient.createState(state, authorCertificatePem, authorToken);

   await zbpClient.SendMessage(message, attachments: null, authorCertificate: pemCert, authorToken: jwt);

   await zbpClient.CreateState(state, authorCertificate: pemCert, authorToken: jwt);

Über den ZBP-Adapter von FIT-Connect senden

   Außerhalb des Verbindungsnetzes verpacken Sie die Payload signiert in ein ZBP-Envelope (content, sha512sum, authorCertificate, authorToken) und senden
   es als normale Submission an den Adapter-Zustellpunkt. Der Adapter leitet weiter und antwortet asynchron im Ereignisprotokoll.

   Wert
   Zustellpunkt (TEST) b21c51d0-324b-4df4-ae6c-c0568a2bb067 — STAGE/PROD über das Anbindungsmanagement
   Leistungsschlüssel Nachricht urn:schema-fitko-de:fit-connect:id.bund.de:message_v6
   Leistungsschlüssel Statusmeldung urn:schema-fitko-de:fit-connect:id.bund.de:status_v6
   Fachdaten-Schema https://schema.fitko.de/fit-connect/id.bund.de/message_v6/1.0.0/zbp-message.schema.json — keines der SDKs prüft das Envelope beim
   Senden gegen dieses Schema, validiert wird vom Adapter

     * Java
     * .NET (C#)

   Das Signieren übernimmt ZBPEnvelopeBuilder aus zbp-client, das Senden der OnlineService aus sdk-client:

   import dev.fitko.fitconnect.zbp.ZBPEnvelopeBuilder;

   import dev.fitko.fitconnect.zbp.model.AuthorKeyPair;
   // Ihr ZBP-Zertifikat — dasselbe wie für den direkten Weg
   AuthorKeyPair authorKeyPair = AuthorKeyPair.builder()
   .authorCertificatePath(Path.of("/path/to/client.crt"))
   .authorPrivateKeyPath(Path.of("/path/to/private.key"))
   .build();
   // 1. Payload signieren und in das Envelope verpacken
   byte[] envelopeJson = ZBPEnvelopeBuilder.fromAuthorPayload(message, authorKeyPair);
   // 2. Als Submission an den Adapter senden — Sie sind hier Sender (Typ C)
   OnlineService onlineService = sdk.onlineService(myDestinationId);
   Participant zbpAdapter = Participant.of(
   UUID.fromString("b21c51d0-324b-4df4-ae6c-c0568a2bb067"), // TEST
   Addressing.toService("urn:schema-fitko-de:fit-connect:id.bund.de:message_v6", "ZBP-Nachricht"));
   URI schemaUri = URI.create(
   "https://schema.fitko.de/fit-connect/id.bund.de/message_v6/1.0.0/zbp-message.schema.json");
   SentSubmission sent = onlineService.send(OutgoingSubmission.to(zbpAdapter)
   .setData(SubmissionData.json(envelopeJson, schemaUri))
   .build());
   // 3. Ergebnis abwarten: SUBMITTED → ACCEPTED / REJECTED
   EventState state = onlineService.cases().logOf(sent).latest().state();

   Anhänge hängen Sie als normale Submission-Anhänge an; Dateinamen müssen mit den Metadaten in der CreateMessage übereinstimmen, Chunking bleibt aus:

   ZBPApiAttachment bescheid = new ZBPApiAttachment(pdfBytes, "bescheid.pdf", "application/pdf");

   CreateMessage message = CreateMessage.builder()
   // ...
   .attachmentMetadata(List.of(ZBPAttachmentMetadata.from(bescheid)))
   .build();
   OutgoingSubmission submission = OutgoingSubmission.to(zbpAdapter)
   .setData(SubmissionData.json(ZBPEnvelopeBuilder.fromAuthorPayload(message, authorKeyPair), schemaUri))
   .addAttachment(Attachment.builder()
   .fromBytes(pdfBytes).fileName("bescheid.pdf").mimeType("application/pdf")
   .shouldBeChunked(false).build())
   .build();

   Statusmeldung — gleiche Mechanik, anderer Leistungsschlüssel:

   byte[] stateEnvelope = ZBPEnvelopeBuilder.fromAuthorPayload(state, authorKeyPair);

   Participant zbpStatus = Participant.of(zbpAdapterId,
   Addressing.toService("urn:schema-fitko-de:fit-connect:id.bund.de:status_v6", "ZBP-Status"));
   onlineService.send(OutgoingSubmission.to(zbpStatus)
   .setData(SubmissionData.json(stateEnvelope, schemaUri))
   .build());

   Öffentliche API seit rc.2

   ZBPEnvelopeBuilder ist seit 4.0.0-rc.2 Teil der öffentlichen API in dev.fitko.fitconnect.zbp (vorher lag die Klasse unter …zbp.internal). Das frühere
   setZBPMessage(…) des Submission-Builders gibt es in 4.0 nicht mehr — sdk-client kennt das ZBP-Modul nicht.

   Die Erweiterungsmethoden aus Fitko.FitConnect.Zbp.Extensions ersetzen WithJsonData(…) in der Builder-Kette und signieren mit dem konfigurierten
   Zertifikat; Anhänge werden automatisch als Submission-Anhänge (ohne Chunking) verdrahtet:

   using Fitko.FitConnect.Zbp.Extensions;

   IOnlineServiceClient onlineService = client.AsOnlineService(); // Sie sind hier Sender (Typ C)
   OutgoingSubmission request = OutgoingSubmissionBuilder.Builder()
   .WithDestinationId(Guid.Parse("b21c51d0-324b-4df4-ae6c-c0568a2bb067")) // TEST
   .WithServiceType("urn:schema-fitko-de:fit-connect:id.bund.de:message_v6", "ZBP-Nachricht")
   .SetZbpMessage(message, zbpClient, [bescheid]) // Anhänge optional
   .Build();
   SentSubmission sent = await onlineService.SendSubmission(request);
   // Ergebnis abwarten: pollt bis Accepted / Rejected, sonst TimeoutException
   EventState state = await onlineService.WaitForCompletionAsync(sent, timeoutSeconds: 60);

   Statusmeldung — SetZbpState und der Leistungsschlüssel status_v6:

   OutgoingSubmission request = OutgoingSubmissionBuilder.Builder()

   .WithDestinationId(zbpAdapterDestinationId)
   .WithServiceType("urn:schema-fitko-de:fit-connect:id.bund.de:status_v6", "ZBP-Status")
   .SetZbpState(state, zbpClient)
   .Build();

   Wenn Ihr System nur Übermittler ist, nehmen Sie die Überladungen mit eigenem Schlüssel und Autor-Daten: SetZbpMessage(message, signingKey,
   authorCertificate, authorToken) bzw. SetZbpState(state, signingKey, authorCertificate, authorToken).

   Anhänge über den Adapter

   Pro Nachricht maximal 30 MB insgesamt, 30 MB pro Datei und 100 Anlagen; erlaubt sind u. a. pdf, jpg/jpeg, png, gif, svg, tif/tiff, txt, csv, rtf, ics.
   Fehlermeldungen von FIT-Connect nennen ca. 33 % größere Werte, weil sie sich auf die verschlüsselten Daten beziehen.

Fehler

     * Java
     * .NET (C#)

   Alle Methoden des ZBPClient werfen ZBPException (unchecked). Bei einer Nicht-2xx-Antwort steckt darin die RestApiException mit Statuscode und
   Antwortkörper:

   try {

   zbpClient.sendMessage(message);
   } catch (ZBPException e) {
   e.getApiException().ifPresent(api ->
   log.error("ZBP lehnt ab: HTTP {} — {}", api.getStatusCode(), api.getResponseBody()));
   }

   Alle Methoden des IZbpClient werfen FitConnectZbpException (Namespace Fitko.FitConnect.Zbp); die Ursache steht in InnerException. Hat das ZBP mit einem
   HTTP-Fehler geantwortet, ist das eine RestApiException (Fitko.FitConnect.Core.Http.Exceptions) mit StatusCode und ResponseBody; IsClientError()
   (Anfrage korrigieren) bzw. IsServerError() (später erneut versuchen) ordnen sie ein:

   try

   {
   await zbpClient.SendMessage(message);
   }
   catch (FitConnectZbpException ex) when (ex.InnerException is RestApiException api)
   {
   _logger.LogError("ZBP lehnt ab: HTTP {Status} — {Body}", api.StatusCode, api.ResponseBody);
   }

   WaitForCompletionAsync wirft TimeoutException, wenn der Adapter innerhalb der Frist weder annimmt noch ablehnt.

   Über den Adapter erreichen Sie Fehler nicht als Exception, sondern als reject-submission mit Problem-Liste im Ereignisprotokoll — dieselbe Mechanik wie
   bei jeder anderen Submission (Status verfolgen).
   Häufige Stolperfallen

     * Falscher Leistungsschlüssel: Nachricht = …:message_v6, Statusmeldung = …:status_v6 — beide mit Versionsangabe. Das Fachdaten-Schema ist für beide
       dasselbe.
     * Falsche Rolle: Über den Adapter senden Sie. Ein Typ-A/B-Zustellpunkt (Organisation) kann das nicht — Sie brauchen einen eigenen Typ-C-Zustellpunkt
       (Rollenmodell).
     * Dateinamen passen nicht: Anhang-Metadaten in der CreateMessage und Submission-Anhänge müssen denselben Dateinamen tragen, sonst weist der Adapter
       die Nachricht zurück.
     * Synchron gedacht: send/SendSubmission bestätigt nur den Eingang bei FIT-Connect. Ob das ZBP die Nachricht angenommen hat, sagt erst das
       Ereignisprotokoll.
     * mTLS am Proxy: Reverse-Proxies müssen Mutual TLS durchreichen, sonst terminiert der Proxy die Verbindung und das ZBP sieht kein Client-Zertifikat.

Weiterführende Themen

     * FIT-Connect und das ZBP – Fachliche Übersicht, Felder des Envelopes, Grenzen
     * Antrag senden – Grundlagen der Submission
     * Status verfolgen – Ereignisprotokoll und WaitForCompletionAsync

   Stand dieser Seite: Java-SDK 4.0.0 · .NET-SDK 4.0.0
```
