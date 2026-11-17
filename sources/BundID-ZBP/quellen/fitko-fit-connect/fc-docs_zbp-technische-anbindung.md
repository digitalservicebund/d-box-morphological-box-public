# FIT-Connect und das Zentrale Bürgerpostfach (ZBP)

- Quelle: <https://docs.fitko.de/fit-connect/docs/zbp/>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
FIT-Connect und das Zentrale Bürgerpostfach (ZBP)

   Das Zentrale Bürgerpostfach (ZBP) als Teil der BundID-Infrastruktur ermöglicht es Fachverfahren, Bürger:innen Nachrichten (wie z.B. Bescheide) sicher
   zu übermitteln. Darüber hinaus ist es möglich, so genannte Antragstatusmeldungen zu versenden, um über den Bearbeitungsstatus eines Antrags via
   Statusmonitor zu informieren.

   Dieses Angebot ist von FIT-Connect unabhängig. Dennoch kann Sie FIT-Connect bei der technischen Integration weitreichend unterstützen. Die
   Möglichkeiten zur technischen Integration hängen dabei davon ab, ob Ihre Anwendung innerhalb des Verbindungsnetzes betrieben wird, oder aber außerhalb
   davon.

Anbindung des ZBP innerhalb des Verbindungsnetzes

   Wird Ihre Anwendung innerhalb des Verbindungsnetzes betrieben, können Sie die REST-Schnittstelle des ZBP direkt aufrufen. Dazu können Sie auf eine
   eigene Implementierung zurückgreifen, die Sie anhand der ZBP-Dokumentation entwickeln. Das Postkorb-Handle Ihres ZBP-Postfachs für den Parameter
   "mailboxUuid" finden Sie nach dem Login in Ihr BundID-Konto unter "Zugänge & Daten" in der Betroffenenauskunft.

   Einfacher ist es jedoch, die dafür bereitgestellte Funktion der FIT-Connect-SDKs zu verwenden. Eine Beispiel-Implementierung mit dem Java-SDK finden
   Sie unten, wobei beim Instanziieren Pfade zu ihrem privaten BPKI-Schlüssel und ihrem BPKI-Zertifikat angegeben werden müssen:

   // ----- Setup configuration that can be reused -----

   final var privateKeyPath = Path.of("/path/to/clientPrivateKey.key");
   final var clientCertPath = Path.of("/path/to/clientCertificate.crt");
   final ZBPCertConfig certConfig = new ZBPCertConfig(privateKeyPath, clientCertPath)
   final ZBPClient zbpClient = ClientFactory.createZBPClient(certConfig, ZBPEnvironment.INT_INTERNET); // INT_INTERNET = Integrationsumgebung for testing
   purposes
   // ------End Setup ----------------------------------
   final CreateMessage message = CreateMessage.builder()
   .content("Hi there from FIT-Connect !")
   .sender("FIT-Connect")
   .service("FIT-Connect Test")
   .title("plain message without attachment")
   .retrievalConfirmationAddress("retrieval@mail.net")
   .replyAddress("reply@mail.net")
   .mailboxUuid(UUID.fromString("d0996589-508c-499c-bb26-a97fed704669"))
   .stork_qaa_level(AuthenticationLevel.ONE)
   .build();
   final CreateMessageResponse response;
   try {
   response = zbpClient.sendMessage(message);
   System.out.println("Sent message successfully. messageUUID = " + response.getMessageUuid());
   } catch (Exception e) {
   throw new RuntimeException("Failed to send message to ZBP", e);
   }

   Das Beispiel kann auf eine Nachricht mit Attachments folgendermaßen erweitert werden:

   // ----- Setup configuration that can be reused -----

   final var privateKeyPath = Path.of("/path/to/clientPrivateKey.key");
   final var clientCertPath = Path.of("/path/to/clientCertificate.crt");
   final ZBPCertConfig certConfig = new ZBPCertConfig(privateKeyPath, clientCertPath);
   final ZBPClient zbpClient = ClientFactory.createZBPClient(certConfig, ZBPEnvironment.INT_INTERNET); // INT_INTERNET = Integrationsumgebung for testing
   purposes
   // ------End Setup ----------------------------------
   final byte[] attachmentData = loadFileAsByteArray();
   final ZBPApiAttachment attachment = new ZBPApiAttachment(attachmentData, "test-attachment.pdf", "application/pdf");
   final ZBPAttachmentMetadata attachmentMetadata = ZBPAttachmentMetadataBuilder.from(attachment);
   final CreateMessage message = CreateMessage.builder()
   .content("Hi there from FIT-Connect !")
   .sender("FIT-Connect")
   .service("FIT-Connect Test")
   .title("plain message with attachment")
   .retrievalConfirmationAddress("retrieval@mail.net")
   .replyAddress("reply@mail.net")
   .mailboxUuid(UUID.fromString("d0996589-508c-499c-bb26-a97fed704669"))
   .stork_qaa_level(AuthenticationLevel.ONE)
   .attachmentMetadata(List.of(attachmentMetadata))
   .build();
   final CreateMessageResponse response;
   try {
   response = zbpClient.sendMessageWithAttachments(message, List.of(attachment));
   System.out.println("Sent message successfully. messageUUID = " + response.getMessageUuid());
   } catch (Exception e) {
   throw new RuntimeException("Failed to send message to ZBP", e);
   }

   Die Übermittlung einer Statusmeldung wird wie folgt durchgeführt:

   // ----- Setup configuration that can be reused -----

   final var privateKeyPath = Path.of("/path/to/clientPrivateKey.key");
   final var clientCertPath = Path.of("/path/to/clientCertificate.crt");
   final ZBPCertConfig certConfig = new ZBPCertConfig(privateKeyPath, clientCertPath);
   final ZBPClient zbpClient = ClientFactory.createZBPClient(certConfig, ZBPEnvironment.INT_INTERNET); // INT_INTERNET = Integrationsumgebung for testing
   purposes
   // ------End Setup ----------------------------------
   final CreateState createState = CreateState.builder()
   .applicationId(UUID.fromString("d0996589-508c-499c-bb26-a97fed704669"))
   .state(State.PROCESSING)
   .createdDate(Instant.now())
   .senderName("FIT-Connect")
   .publicServiceName(new PublicServiceName("Bundeswahlscheinverfahren"))
   .additionalInformation(new AdditionalInformation("Ihr Antrag wird von einem Sachbearbeiter bearbeitet."))
   .statusDetails(new StatusDetails("Antrag wird bearbeitet"))
   .build();
   try {
   zbpClient.createState(createState);
   System.out.println("Sent state successfully.");
   } catch (Exception e) {
   throw new RuntimeException("Failed to send state to ZBP", e);
   }

Anbindung des ZBP außerhalb des Verbindungsnetzes

   Wird Ihre Anwendung außerhalb des Verbindungsnetzes betrieben, können Sie auf die von FIT-Connect bereitgestellte Brückenkopf-Anwendung ZBP-Adapter
   zurückgreifen.

   Dazu müssen Sie lediglich eine Einreichung (Submission) mit dem Inhalt der ZBP-Nachricht oder der ZBP-Statusmeldung an einen speziellen Zustellpunkt
   (Destination) senden. Der ZBP-Adapter wird diese Einreichung zeitnah verarbeiten, an das ZBP weiterleiten und mit einem accept-submission oder
   reject-submission-Event antworten. Dazu sind folgende Konventionen einzuhalten:
     * DestinationId:
          + TEST: b21c51d0-324b-4df4-ae6c-c0568a2bb067
          + STAGE: Gerne sprechen Sie das Anbindungsmanagement vom BMDS für weitere Informationen an. Hier finden Sie den Link zur Service-Anfrage:
            https://ssp.id.bund.de/ip?id=csm_sc_cat_item&sys_id=9bfceb4a1bc07150a462fdd49b4bcbd4
          + PROD: Gerne sprechen Sie das Anbindungsmanagement vom BMDS für weitere Informationen an. Hier finden Sie den Link zur Service-Anfrage:
            https://ssp.id.bund.de/ip?id=csm_sc_cat_item&sys_id=9bfceb4a1bc07150a462fdd49b4bcbd4
     * Leistungsschlüssel ZBP-Nachricht (publicService.identifier): urn:schema-fitko-de:fit-connect:id.bund.de:message_v6
     * Leistungsschlüssel ZBP-Statusmeldung (publicService.identifier): urn:schema-fitko-de:fit-connect:id.bund.de:status_v6
     * Fachdaten-Schema: https://schema.fitko.de/fit-connect/id.bund.de/message_v6/1.0.0/zbp-message.schema.json

   Das eigene BundID-Postkorb-Handle finden Sie, wenn Sie bei der BundID eingeloggt sind, wie folgt:
    1. Bereich "Zugänge & Daten"
    2. "Daten" -> "Betroffenenauskunft herunterladen"
    3. Im PDF befindet sich die Angabe bei "Kontodaten" - "Postkorb-Handle (eineindeutige Kennung Ihres Postfaches zur Zustellung von Bescheiden)"

   Eine Beschreibung der Felder des Fachdatenkranzes (authorCertificate, authorToken, content, sha512sum) finden Sie unten. Aktuelle Informationen
   entnehmen Sie der ZBP-Dokumentation.
   Feld Bedeutung
   authorCertificate Ist das PKI-Zertifikat des Autors als PEM-Format-String. Dieser Wert muss angegeben werden, wenn die Nachricht über den ZBP-Adapter
   an das ZBP gesendet wird und somit der Nachrichtensender und -signierer nur der Übermittler (ZBP-Adapter) und nicht der eigentliche Nachrichtenautor
   (Drittanwendung) ist.
   authorToken Ist das JSON Web Token (JWT) des Autors bei Verwendung des ZBP-Adapters in Form eines Token-String. Dieser Wert muss angegeben werden, wenn
   die Nachricht über dem ZBP-Adapter an das ZBP gesendet wird und somit der Nachrichtensender und Nachrichtensignierer nur der Übermittler (ZBP-Adapter)
   und nicht der eigentliche Nachrichtenautor (Drittanwendung) ist.
   content JSON-serialisierte Nachricht für das zentrale Bürgerpostfach. Dieses Feld ist auf eine Maximalgröße von 1 MB limitiert. Alle Nachrichten werden
   auf HTML-Code untersucht. Die erlaubten HTML-Tags sind ebenfalls in der ZBP-Dokumentation beschrieben.
   sha512sum Signatur über das content-Feld, gebildet mit dem privaten BPKI-Schlüssel, SHA512 und RSA512

Postfachnachricht

   Das Senden einer ZBP-Nachricht über den ZBP-Adapter wird durch die FIT-Connect-SDKs deutlich vereinfacht. Das Postkorb-Handle für den Parameter
   "mailboxUuid" Ihres ZBP-Postfachs finden Sie nach dem Login in Ihr BundID-Konto unter "Zugänge & Daten" in der Betroffenenauskunft. Dazu zunächst ein
   Beispiel ohne Attachments:

   // ----- Setup configuration that can be reused -----

   final ApplicationConfig config = ApplicationConfigLoader.loadConfigFromPath(Path.of("/path/to/config.yml"));
   final SenderClient senderClient = ClientFactory.createSenderClient(config);
   final AuthorKeyPair authorKeyPair = AuthorKeyPair.builder()
   .authorCertificatePath(Path.of("/path/to/clientCertificate.crt"))
   .authorPrivateKeyPath(Path.of("/path/to/clientPrivateKey.key"))
   .build();
   final String destinationId = "TODO"; // depends on the environment: TEST / STAGE / PROD
   final String serviceIdentifier = "urn:schema-fitko-de:fit-connect:id.bund.de:message_v6"; // constant
   // ------End Setup ----------------------------------
   final CreateMessage message = CreateMessage.builder()
   .content("Hi there from FIT-Connect !")
   .sender("FIT-Connect")
   .service("FIT-Connect Test")
   .title("plain message without attachment")
   .retrievalConfirmationAddress("retrieval@mail.net")
   .replyAddress("reply@mail.net")
   .mailboxUuid(UUID.fromString("d0996589-508c-499c-bb26-a97fed704669"))
   .stork_qaa_level(AuthenticationLevel.ONE)
   .build();
   // send to FIT-Connect backend that will process the message and confirm the result via the event log
   final SendableSubmission submission = SendableSubmission.Builder()
   .setDestination(UUID.fromString(destinationId))
   .setServiceType(serviceIdentifier, "ZBP Message Forwarding")
   .setZBPMessage(message, authorKeyPair)
   .build();
   // the following call will only fail, if FIT-Connect can not be reached, but a successful call does NOT
   // imply that the message was already accepted by ZBP. the process is asynchronous!
   final SentSubmission sentSubmission = senderClient.send(submission);
   // we can then poll repeatedly for the status of this submission.
   // it will start as EventState.SUBMITTED and will eventually transition to EventState.ACCEPTED or EventState.REJECTED
   final Status submissionStatus = senderClient.getSubmissionStatus(sentSubmission);

   Attachments zu der ZBP-Nachricht können als Anlagen (Attachments) der FIT-Connect-Einreichung (Submission) abgebildet werden. Die Attachments müssen in
   der ZBP-Nachricht korrekt beschrieben sein. Die Dateinamen von Submission-Attachments und ZBP-Nachricht müssen übereinstimmen.
   Einschränkungen für Anlagen

   Wird der ZBP-Adapter verwendet, legt FIT-Connect Einschränkungen für Anlagen fest.

   Die maximal zulässige Größe aller Anhänge pro Nachricht beträgt 30 MB. Genauso darf ein einzelner Anhang maximal 30 MB groß sein. Es dürfen bis zu 100
   Anlagen an eine Nachricht angehängt werden.

   Randbemerkung: Lassen Sie sich nicht FIT-Connect-Fehlermeldungen verunsichern, wenn darin Werte stehen, die rund 33 % größer sind, als hier angegeben.
   Dies liegt daran, dass wir in diesem Text auf die ursprüngliche Dateigröße Bezug nehmen. Vor der Übertragung verschlüsseln und kodieren die SDKs die
   Dateien, wodurch sich ihr Umfang um etwa 33 % vergrößert. Die Fehlermeldungen beziehen sich auf die Größe dieser verschlüsselten Dateien.

   Wird das ZBP direkt angesprochen, gelten vergleichbare Einschränkungen. Die aktuellen Vorgaben für Anhänge können Sie über die technischen
   Informationen im Downloadbereich des Self-Service Portals der BundID beziehen.

   In beiden Fällen dürfen nur Anlagen mit folgenden Dateitypen angehängt werden: pdf, gif, jpg, jpeg, png, svg, tiff, tif, txt, ics, ical, ifb, bmp, rtf,
   csv

   Mit dem Java-SDK lassen sich Attachments folgendermaßen anhängen:

   // ----- Setup configuration that can be reused -----

   final ApplicationConfig config = ApplicationConfigLoader.loadConfigFromPath(Path.of("/path/to/config.yml"));
   final SenderClient senderClient = ClientFactory.createSenderClient(config);
   final AuthorKeyPair authorKeyPair = AuthorKeyPair.builder()
   .authorCertificatePath(Path.of("/path/to/clientCertificate.crt"))
   .authorPrivateKeyPath(Path.of("/path/to/clientPrivateKey.key"))
   .build();
   final String destinationId = "TODO"; // depends on the environment: TEST / STAGE / PROD
   final String serviceIdentifier = "urn:schema-fitko-de:fit-connect:id.bund.de:message_v6"; // constant
   // ------End Setup ----------------------------------
   final byte[] attachmentData = loadFileAsByteArray();
   final Attachment fitConnectAttachment = Attachment.fromByteArray(attachmentData, "application/pdf", "test-attachment.pdf", "test")
   final ZBPAttachmentMetadata attachmentMetadata = ZBPAttachmentMetadataBuilder.from(fitConnectAttachment);
   final CreateMessage message = CreateMessage.builder()
   .content("Hi there from FIT-Connect !")
   .sender("FIT-Connect")
   .service("FIT-Connect Test")
   .title("plain message with attachment")
   .retrievalConfirmationAddress("retrieval@mail.net")
   .replyAddress("reply@mail.net")
   .mailboxUuid(UUID.fromString("d0996589-508c-499c-bb26-a97fed704669"))
   .stork_qaa_level(AuthenticationLevel.ONE)
   .attachmentMetadata(List.of(attachmentMetadata))
   .build();
   // send to FIT-Connect backend that will process the message and confirm the result via the event log
   final SendableSubmission submission = SendableSubmission.Builder()
   .setDestination(UUID.fromString(destinationId))
   .setServiceType(serviceIdentifier, "ZBP Message Forwarding")
   .setZBPMessage(message, authorKeyPair)
   .addAttachment(fitConnectAttachment)
   .build();
   // the following call will only fail, if FIT-Connect can not be reached, but a successful call does NOT
   // imply that the message was already accepted by ZBP. the process is asynchronous!
   final SentSubmission sentSubmission = senderClient.send(submission);
   // we can then poll repeatedly for the status of this submission.
   // it will start as EventState.SUBMITTED and will eventually transition to EventState.ACCEPTED or EventState.REJECTED
   final Status submissionStatus = senderClient.getSubmissionStatus(sentSubmission);

Statusmeldung

   Eine Statusmeldung lässt sich in Form einer FIT-Connect-Einreichung übermitteln. Die applicationId kann aus den Metadaten aus dem Feld
   idBundDeApplicationId einer Einreichung übernommen werden, wenn es sich um eine Postfachnachricht handelt.

   Die Integration im Java-SDK ist wie folgt vorgesehen:

   // ----- Setup configuration that can be reused -----

   final ApplicationConfig config = ApplicationConfigLoader.loadConfigFromPath(Path.of("/path/to/config.yml"));
   final SenderClient senderClient = ClientFactory.createSenderClient(config);
   final AuthorKeyPair authorKeyPair = AuthorKeyPair.builder()
   .authorCertificatePath(Path.of("/path/to/clientCertificate.crt"))
   .authorPrivateKeyPath(Path.of("/path/to/clientPrivateKey.key"))
   .build();
   final String destinationId = "TODO"; // depends on the environment: TEST / STAGE / PROD
   final String serviceIdentifier = "urn:schema-fitko-de:fit-connect:id.bund.de:status_v6"; // constant
   // ------End Setup ----------------------------------
   final CreateState createState = CreateState.builder()
   .applicationId(UUID.fromString("d0996589-508c-499c-bb26-a97fed704669"))
   .state(State.PROCESSING)
   .createdDate(Instant.now())
   .senderName("FIT-Connect")
   .publicServiceName(new PublicServiceName("Bundeswahlscheinverfahren"))
   .additionalInformation(new AdditionalInformation("Ihr Antrag wird von einem Sachbearbeiter bearbeitet."))
   .statusDetails(new StatusDetails("Antrag wird bearbeitet"))
   .build();
   // send to FIT-Connect backend that will process the state and confirm the result via the event log
   final SendableSubmission submission = SendableSubmission.Builder()
   .setDestination(UUID.fromString(destinationId))
   .setServiceType(serviceIdentifier, "ZBP State Forwarding")
   .setZBPState(createState, authorKeyPair)
   .build();
   // the following call will only fail, if FIT-Connect can not be reached, but a successful call does NOT
   // imply that the state was already accepted by ZBP. the process is asynchronous!
   final SentSubmission sentSubmission = senderClient.send(submission);
   // we can then poll repeatedly for the status of this submission.
   // it will start as EventState.SUBMITTED and will eventually transition to EventState.ACCEPTED or EventState.REJECTED
   final Status submissionStatus = senderClient.getSubmissionStatus(sentSubmission);

Ablehnung einer Postfachnachricht oder einer Statusmeldung

   Eine Einreichung an das ZBP kann aus verschiedenen Gründen abgelehnt werden. Der Status der Einreichung muss selbstständig durch den Sender erfragt
   werden. Eine Übersicht der möglichen Fehlermeldungen, die das ZBP ausgibt, finden Sie in der technischen Dokumentation auf den Seiten der BundID.

   Wenn der ZBP-Adapter von FIT-Connect außerhalb des Verbindungsnetzes verwendet wird, können zusätzlich folgende Fehlermeldungen ausgegeben werden. Die
   Fehlermeldungen sind in einem Reject-Event enthalten. Details dazu können Sie auf den Seiten zu Empfangsbestätigung oder Zurückweisung der
   FIT-Connect-Dokumentation nachvollziehen.
   Fehler Beschreibung
   JSON deserialization failed Die Fachdaten der Einreichung entsprechen nicht einem JSON-Dokument des erwarteten Schemas.
   Signature invalid Die Author-Signatur kann nicht nachvollzogen werden.
   Unexpected Error Das ZBP antwortet netzwerktechnisch nicht oder außerhalb der HTTP-2xx- und HTTP-4xx-Range. Im detail-Feld finden Sie die Beschreibung:
   "An unexpected error occurred. Please contact the administrators."
   Token expired Der gesendete JWT ist bereits abgelaufen.
   Token invalid Der gesendete JWT kann nicht geparsed werden.
   Message rejected Das ZBP hat die Übertragung abgelehnt. Der zurückgegebene Fehlercode des ZBP wird im detail-Feld weitergegeben.
   Verhalten bei Wartungsarbeiten oder Ausfällen

   Sollte der ZBP-Adapter, z.B. aufgrund von Wartungsarbeiten, nicht auf FIT-Connect oder das ZBP zugreifen können, versucht er im Anschluss, alle
   Submissions abzuarbeiten, die sich während der Nichtverfügbarkeit von FIT-Connect bzw. während der Nichtverfügbarkeit des ZBP angesammelt haben.

   Wenn eine Nichtverfügbarkeit 30 Minuten oder länger dauert, kann der Fall eintreten, dass Nachrichten nicht mehr vom ZBP angenommen werden. Dies
   begründet sich dadurch, dass das ZBP den in einer Nachricht enthaltenen "authorToken" prüft und dieser maximal 30 Minuten gültig ist. In diesem Fall
   würde im Reject-Event der Fehler "Token expired" stehen.

Zusammenfassung

   Wie den SDK-Beispielen zu entnehmen ist, ähnelt sich die Implementierung von Anwendungen innerhalb und außerhalb des Verbindungsnetzes. Das Erzeugen
   der CreateMessage- bzw. CreateState-Payload ist sogar identisch. Es ist also möglich, eine Anwendung so zu implementieren, dass sie in beiden
   Umgebungen lauffähig ist.

   Zum Beispiel kann über einen Konfigurationsparameter zwischen den beiden gezeigten Aufrufen nahtlos umgeschaltet werden. Zu Beachten ist dabei, dass
   der Aufruf innerhalb des Verbindungsnetzes synchron ist, während bei der Verwendung von FIT-Connect das Ergebnis zeitversetzt abgeholt werden muss.
```
