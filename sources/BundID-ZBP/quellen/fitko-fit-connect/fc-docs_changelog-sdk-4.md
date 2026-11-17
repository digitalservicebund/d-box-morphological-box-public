# Changelog & Migration

- Quelle: <https://docs.fitko.de/fit-connect/docs/sdks/referenz/changelog>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Changelog – Java- & .NET-SDK

   Wählen Sie unten Ihr SDK. Die Changelogs folgen dem Format Keep a Changelog und halten sich an Semantic Versioning.

   Änderungen am SDK selbst werden hier dokumentiert. Änderungen an der FIT-Connect-Plattform und an den APIs entnehmen Sie den entsprechenden
   API-Beschreibungen. Im Changelog der Plattform stehen die SDK-Releases mit ihrem Veröffentlichungsdatum neben den Releases der Dienste und APIs.
     * Java
     * .NET (C#)

   Die vollständige Java-SDK-Versionsliste pflegen wir im java-sdk-Repository (alle Module folgen einer gemeinsamen Versionierung über das sdk-bom).

   Aktuelle Version: 4.0.0 (27. Oktober 2026) — was sich gegenüber rc.2 ändert:
     * Annehmen trotz Befunden: accept(...) sendet das Accept-Event auch bei Fehlern im ReceiveReport und meldet sie dem Sender als Problems; wer ablehnen
       will, prüft vorher acceptable(). SubmissionNotAcceptableException entfällt.
     * XSD-Prüfung von XML-Fachdaten: wie in 3.x gegen das XSD aus customSchemas oder — bei XÖV-URNs — aus dem XRepository; Verstöße stehen als
       DataSchemaViolation im ReceiveReport.
     * Vorgangswahl: Einreichungen an eine Leistung können keinen Vorgang eröffnen — build() wirft bei openingCase(...)/openingCaseAt(...) eine
       IllegalStateException.
     * Vorverschlüsselte Einreichungen: Fachdaten und Anhänge müssen eine Compact-JWE mit RSA-OAEP-256/A256GCM sein, sonst FitConnectSenderException vor
       der Ankündigung.
     * Anhang-Ablage: nur für den Eigentümer lesbar; ohne attachmentStoragePath warnt das SDK und nutzt das temporäre Verzeichnis; purgeAllAttachments()
       liefert false, wenn Dateien liegen bleiben.
     * AdditionalReferenceInfo.consentToDigitalNotice ist Boolean; Vertrauensanker PCA-1-Verwaltung-14/-15 entfernt; toString() von JWKPair,
       AuthorKeyPair, DecryptionRequest, ProxyAuth ohne Geheimnisse.

   4.0.0-rc.2 (7. Oktober 2026) — was sich gegenüber rc.1 ändert:
     * Metadaten-Version wird ausgehandelt: Geschrieben wird die höchste gemeinsame Version von Zustellpunkt und SDK (2.0.0, 2.1.0, 3.0.0); gesetzte
       Felder bestimmen die Version nicht mehr, auch Antworten handeln gegen den lesenden Zustellpunkt aus. Ohne gemeinsame Version:
       FitConnectSenderException mit Handlungshinweis.
     * Metadaten 1.x nur noch lesend: setAuthenticationInformation(...) an OutgoingSubmission und EncryptedOutgoingSubmission ist entfernt; Zustellpunkte,
       die nur 1.x anbieten, werden abgelehnt (Fristen: Abkündigungen). Ebenfalls entfernt: MetadataVersion, MetadataVersionInference,
       preferMetadataVersion(...) am OutgoingReply-Builder, SdkSettings.METADATA_SCHEMA_V1/V2/V3_WRITE_VERSION (jetzt WRITABLE_METADATA_SCHEMA_VERSIONS),
       der Chunking-Rückfall unter 1.3.0 und Email.usePgp.
     * Mehrere Rückkanäle in Metadaten 3.x: ReplyChannels.of(...) kombiniert Kanäle, setReplyChannelMessages(...) ordnet Prozessnachrichten Kanälen zu
       (beides nur an Zustellpunkte mit 3.x). ReplyChannels.fitConnect(...) erwartet List<DataSetSchema> mit MIME-Typ; die eigene Destination im
       FIT-Connect-Rückkanal setzt das SDK selbst. Der Sender prüft vor dem Versand, ob der Zustellpunkt alle Rückkanäle freigeschaltet hat;
       ReceivedSubmission.replyChannel() liest auch das 3.x-Dataset.
     * Ereignisprotokoll mit Befunden: Eine fehlgeschlagene Prüfung wirft FitConnectEventLogException; report() listet die Befunde mit Stufe EVENT_LOG,
       report().asProblems() geht direkt in reject(waiting, ...). OnlineService.reject(waiting, problems) lehnt eine wartende Antwort ohne Entschlüsselung
       ab. Unbekannter Sperrstatus (OCSP-Ausfall) → FitConnectValidationException mit Bitte um späteren Versuch.
     * Metadaten beim Empfang: Fehlendes, nicht auflösbares oder fremdes $schema, ein unbekannter Major und widersprüchliche 1.x/2.x-Felder sind Befunde
       im Prüfbericht statt Abbrüche. Schemas nur aus bekannten Quellen: Fachdaten remote nur per HTTPS, Metadaten-Schemas nur aus SDK, eigener
       Konfiguration oder schema.fitko.de; Event-Schemas werden nie nachgeladen.
     * Neue API: ReceivedSubmission.submittedAt() (Optional<Instant>), Attachment.Builder.signature(...) / getSignature() (wie 3.x, unverändert
       durchgereicht, nicht geprüft), ZBPEnvelopeBuilder als öffentliche API in dev.fitko.fitconnect.zbp, send(OutgoingReply) liefert die
       Authentication-Tags im SentReply, Directory.forService/allForService nehmen LeiKa-Schlüssel (14 Stellen) oder URN.
     * Öffentliche Typen: CaseEvent.issueTime() liefert Instant statt Date; Author hat url und product, version und manufacturer stehen in Product.
     * Konfiguration: environment ist Pflicht (kein stiller Rückfall auf TEST); fehlende Felder in retryConfig behalten ihre Vorgabe; Proxy ohne
       Zugangsdaten funktioniert; sample-config.yaml nutzt timeoutConfig.
     * CLI keygen: öffentliche Schlüssel mit selbstsignierter x5c-Kette, Signaturschlüssel PS512, Dateinamen encryption_public_key.jwk,
       decryption_private_key.jwk, signature_public_key.jwk, signature_private_key.jwk.
     * Maven: sdk-bom pinnt jetzt tatsächlich core, rest-clients, sdk-client, virus-scanner und zbp-client (rc.1 war leer); cli ist nicht mehr im BOM;
       core reicht Lombok nicht mehr als Compile-Abhängigkeit weiter; Projekt- und SCM-URLs zeigen auf fit-connect/java-sdk. Vertrauensanker
       PCA-1-Verwaltung-26 ergänzt; aktualisierte Bibliotheken u. a. Jackson 3.2.3 (CVE-2026-91776, CVE-2026-91777), nimbus-jose-jwt 10.10, BouncyCastle
       1.86.

   Das vollständige Changelog mit Story-Links führt die CHANGELOG.md im Repository.
   4.0.0-rc.1 — die modulare Neufassung
     * Modulare Neufassung des Java-SDKs unter dev.fitko.fitconnect:
          + sdk-client ersetzt das alte dev.fitko.fitconnect.sdk:client
          + core, rest-clients, virus-scanner, zbp-client, cli als eigene Module
          + sdk-bom für versionssynchrone Einbindung mehrerer Module
     * Neuer Entry-Point FitConnectSdk.fromConfigBuilder() / fromConfigYaml(path)
     * Flaches Rollenmodell statt Sender/Subscriber: sdk.organisation(destinationId, keys) (Verwaltungssystem, Typ A/B) und
       sdk.onlineService(destinationId, replyKeys) (Onlinedienst, Typ C) ersetzen die frühere Zwischengeneration mit sdk.sender() / sdk.subscriber().
       Suche und Verwaltung laufen über sdk.directory() (Routing, Schlüssel-Lookup — unauthentifiziert) und sdk.manage() (Destinations, Anhänge). Jede
       Partei ist selbst eine Destination — auch ein reiner Sender braucht seit dieser Umstellung eine eigene Destination-ID.
     * OutgoingSubmission.to(participant) / OutgoingReply.answering(...) ersetzen SubmissionRequest.Builder() / ReplyRequest.forCase(...); Participant +
       Addressing kapseln Empfänger und Verwaltungsleistung bzw. Prozessnachricht.
     * Eine Credential-Paarung für Senden und Empfangen — senderConfig/subscriberConfig in der YAML-Konfiguration entfällt; private Schlüssel werden nicht
       mehr über die Konfiguration, sondern als Argument beim Erzeugen von Organisation/OnlineService übergeben.
     * Java 21 als Mindestversion
     * Attachment-API vereinheitlicht: fromBytes, fromFile, fromStreamSupplier, Attachment.builder().shouldBeChunked(true).build()
     * Alte v3.x-Klassen (SendableSubmission, ClientFactory.createSenderClient, ApplicationConfigLoader) sind durch OutgoingSubmission/OutgoingReply,
       FitConnectSdk und YAML-Loader ersetzt
     * Kein Auto-Reject mehr (seit 4.0.0-rc.1, vorher noch enableAutoReject): receive(...) liefert immer einen ReceiveReport; Annahme/Ablehnung ist immer
       der explizite Aufruf von accept(...)/reject(...) durch das Fachverfahren — siehe Konzept: Der Prüfbericht

   Vollständige Methodenreferenz des aktuellen Stands: Spickzettel (Java). Eine ausführliche Versionshistorie der älteren v3-Linie finden Sie unter
   Java-SDK Changelog (Archiv).

   Das .NET-SDK wurde im Zuge des Meta-SDK-Umbaus ebenfalls modular neu aufgesetzt. Das Repository heißt nun dotnet-sdk (vormals meta-sdk-dotnet) mit den
   NuGet-Paketen unter Fitko.FitConnect.*.

   Aktuelle Version: 4.0.0 (27. Oktober 2026; entspricht rc.2.1 vom 23. Oktober) — was sich gegenüber rc.2 ändert (enthält Breaking Changes):
     * Annehmen trotz Fehlern: Der Prüfbericht ist reine Diagnose. AcceptSubmission(…) / AcceptReply(…) senden das Accept-Event auch bei Fehlern im
       Bericht und tragen dessen Probleme stets mit; SubmissionNotAcceptableException entfällt. Eigene Problems übergeben Sie über die Überladung mit
       IReadOnlyList<Problem>.
     * RejectSubmission(…) / RejectReply(…) nehmen IReadOnlyList<Problem> statt List<Problem>; CaseEvent.Problems ist nicht mehr nullable.

   4.0.0-rc.2 — was sich gegenüber rc.1 ändert (enthält Breaking Changes):
     * Rolle bei der Registrierung: AddFitConnect(FitConnectRole.Organisation | OnlineService) registriert nur die passende Fassade, die Sie direkt
       injizieren. Falsche Rolle, fehlende Credentials oder fehlende DestinationId brechen den Host-Start ab. AsOrganisation() / AsOnlineService() bleiben
       verfügbar.
     * Schlanke öffentliche API: Die rohen REST-Clients (ISubmissionApiClient, ICaseApiClient, IReplyApiClient, IDestinationApiClient, IRouteApiClient,
       IKeyApiClient), ISubmissionService und IEventLogService sind internal. Zustellpunkte verwalten Sie über client.Manage().Destinations
       (IDestinationClient), den Anhangspeicher über client.Manage().Attachments (IAttachmentManager).
     * Adressierung: Participant (Zustellpunkt + Leistung bzw. Prozessnachricht + Region) aus Directory.ForService(…) / ForProcessMessage(…), übergeben
       mit WithRecipient(participant). Directory-Suchen liefern RouteSearchResult<T> (Items, Discarded); Route enthält jetzt Behördenangaben (Adressen,
       Kontakt, Links).
     * Signaturprüfung: Routen (Signatur, signierte Leistungen; abschaltbar über Validation.Routes) und Event-Logs werden geprüft. Ein ungültiges
       Event-Log wirft FitConnectEventLogException.
     * Empfang: FetchAvailableSubmissions() liefert ListedSubmissions (nur IDs); FetchSubmission(listed) lädt, entschlüsselt und prüft. Ersetzt
       FetchAvailableSubmissionsFromDestination. Nicht empfangbare Nachrichten werfen FitConnectValidationException mit Problems für
       RejectSubmission/RejectReply. Akzeptiert wird nur noch das kompakte JWE-Format.
     * Antworten: FetchReply(listedReply | replyId, keys) ersetzt FetchSpecificReply; SendReply liefert SentReply statt SendReplyResponse.
     * Event-Logs: GetSubmissionLog / GetReplyLog an den Rollen-Clients und Cases.GetCaseEventLog(…) liefern ein geprüftes TransferLog (LatestState(),
       WasAccepted, Problems, Describe() …). Seitenweise Abfragen über (offset, limit) bzw. GetAllCases().
     * Metadaten-Version: WithMetadataVersion ist optional und nur eine Präferenz; das SDK wählt die höchste gemeinsame Version (2.0.0, 2.1.0, 3.0.0). 1.x
       lässt sich nicht mehr senden. WithAuthenticationInformation entfällt.
     * Datensätze & Signaturen: WithDataSet(DataSet.Json/Xml/Jose(…)) mit Hash-Prüfung beim Empfänger (DataSetHashMismatch);
       WithDataSignature(Signature.Detached/Embedded(…)) und Attachment.Builder().SetSignature(…) transportieren eIDAS-AdES-Signaturen. Fachdaten auch als
       Bytes: WithData(SubmissionData.Bytes(…)).
     * Rückkanäle: ReplyChannel.OfFitConnect(key, supportedSchemas) verlangt mindestens ein Schema und keine destinationId mehr (Antworten gehen an
       Client.DestinationId). Andere Kanäle tragen eine Adresse: ReplyChannel.OfEmail(address), OfDeMail(address), OfElster(accountId, …),
       OfIdBundDeMailbox(mailboxUuid, …); am Zustellpunkt beschreibt ReplyChannelSupport die angebotenen Kanäle. received.GetReplyChannel() liest den
       angebotenen Kanal. Nicht angebotene Kanäle lehnt SendSubmission vorab ab.
     * Schlüsselerzeugung: EncryptionKeyPairGenerator.Generate() erzeugt Rückkanal-Schlüsselpaare (JwkPair).
     * Konfiguration: Validation.CustomSchemas ordnet Schema-URIs lokalen Dateien zu; Validation.Attachments entfällt; Http.Timeouts.Call ersetzt
       Read/Write, Anhänge nutzen Http.Timeouts.Upload; Http.Retry.AllowRetries ist standardmäßig true; Attachments.MaxParallelDownloads ist neu; ohne
       Attachments.ChunkSizeInMb zielt das SDK auf 50 MB pro Chunk, angepasst an die Limits des Zustellpunkts.
     * Anhangspeicher: Gechunkte Anhänge liegen pro Abruf unter {submissionId}/{fetchId}/{attachmentId};
       IAttachmentManager.LoadAttachment/DeleteAttachment (und IStorageProvider) nehmen zusätzlich die FetchId aus IncomingSubmission.FetchId.
     * Callbacks: NewEventsCallback nennt die betroffenen Anträge (NewEventsForSubmission) und Antworten (NewEventsForReply).
     * Virenscan: Funde erscheinen als MalwareDetected bzw. VirusScanRefused im Report (beim Senden als FitConnectValidationException); ein nicht
       erreichbarer Scanner wirft VirusScanUnavailableException.
     * Exceptions: FitConnectSubscriberException, FitConnectAttachmentDecryptionException und AuthenticationTagException entfallen; neu sind
       FitConnectEventLogException und VirusScanUnavailableException.
     * Weitere Ergänzungen: SetRecipient(participant) am Builder für vorverschlüsselte Submissions; Directory.AllForService/AllForProcessMessage liefern
       alle passenden Participant-Objekte; SubmissionData (Json, Xml, Bytes) mit WithData(...) an beiden Buildern; jedes ReceiveIssue nennt in Source den
       meldenden Validator (erscheint in Describe()); DestinationFilter (WithStatus, Matching, AsMinimal) für ListDestinations / ListAllDestinations und
       GetKey(destinationId, keyId) an client.Manage().Destinations; RestApiException erbt von FitConnectException und trägt Method, Url, ResponseBody,
       IsNotFound(), IsClientError(), IsServerError(); ReplyChannel.OfFink ist als veraltet markiert; neues Modul Fitko.FitConnect.Postfach
       (ELSTER-Transfer).

   4.0.0 im Überblick — die modulare Neufassung
     * Modulare Struktur mit Fitko.FitConnect.Core, Fitko.FitConnect.Client, Fitko.FitConnect.VirusScanning, Fitko.FitConnect.Zbp,
       Fitko.FitConnect.Postfach sowie dem Beispielprojekt Fitko.FitConnect.Examples
     * .NET 10 als Mindestversion (kein separates OpenSSL mehr auf macOS nötig)
     * Rollen-Clients wie in Java: services.AddFitConnect(FitConnectRole.Organisation) (Typ A/B, IOrganisationClient) bzw. FitConnectRole.OnlineService
       (Typ C, IOnlineServiceClient) statt SenderService/ReceiverService; der Host prüft beim Start, ob die eigene DestinationId zur deklarierten Rolle
       passt
     * Eine Credential-Quelle (Client statt Sender/Receiver) mit eigener DestinationId, geordneten DecryptionKeys und SignatureKey
     * Dependency-Injection-Erstklassig: services.AddFitConnect(…), AddFitConnectVirusScanning(), AddFitConnectZbp(); Konfiguration über IConfiguration
       (Abschnitt FitConnect)
     * Builder-API OutgoingSubmissionBuilder.Builder().WithRecipient(participant) bzw. .WithDestinationId(…) mit Step-Interfaces; Antworten mit
       ReplyRequestBuilder.ForSubmission(received) bzw. ForCase(caseId).EncryptWith(key)
     * Prüfbericht statt Auto-Reject: IncomingSubmission.Report (ReceiveReport) mit Acceptable(), AsProblems(), Describe()
     * Directory (client.Directory) mit ForService/ForProcessMessage (adressierbarer Participant), FindByService, FindByProcessMessage, FindAreas,
       GetActiveEncryptionKey; ReplyKeys (Of/FromLookup/None) für Rückkanalschlüssel
     * Verwaltung über client.Manage(): eigene Zustellpunkte, Schlüssel, Anhang-Limits und den lokalen Anhangspeicher
     * Gemeinsame Exception-Basis FitConnectException mit spezifischen Untertypen
     * Frühere monolithische FitConnect-NuGet-Pakete (Namespace FitConnect.*) sind durch diese modulare Familie ersetzt;
       FitConnectClient.CreateClient(yaml) existiert nicht mehr

   Eine ausführliche Versionshistorie der älteren FitConnect-NuGet-Pakete finden Sie unter .NET-SDK Changelog (Archiv).
   Migration von älteren SDK-Versionen

   Wenn Sie noch das Java-SDK 3.x (dev.fitko.fitconnect.sdk:client) oder die alte FitConnect-NuGet-Bibliothek verwenden, sind die meisten Konzepte gleich
   geblieben, die Klassennamen jedoch durchgängig anders. Den Umstiegspfad für Java beschreibt die MIGRATION.md im Repository (Gegenüberstellung 3.x →
   4.x, Checkliste); für .NET dienen die Beispielprojekte (Fitko.FitConnect.Examples) als Vorlage.

   Die bisherige Dokumentation der älteren Generationen bleibt als Archiv erreichbar: Java-SDK 3.x · .NET-SDK (FitConnect-NuGet) — in der Seitenleiste
   unter „Archiv: ältere SDK-Versionen“.

   Stand dieser Seite: Java-SDK 4.0.0 · .NET-SDK 4.0.0
```
