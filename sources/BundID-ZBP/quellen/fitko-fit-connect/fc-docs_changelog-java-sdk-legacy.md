# Changelog Java-SDK (Legacy, dev.fitko.fitconnect.sdk)

- Quelle: <https://docs.fitko.de/fit-connect/docs/sdks/java-sdk/changelog>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Changelog

   Alle relevanten Änderungen am FIT-Connect Java SDK werden in dieser Datei dokumentiert.

   Das Format basiert auf Keep a Changelog und hält sich an Semantic Versioning.

   Das allgemeine Changelog für FIT-Connect finden Sie unter Changelog.

[3.4.1] - 2026-05-04

Behoben

HTTP-Retry auch bei Fehlern beim Lesen des Response-Body

     * Die Retry-Logik wurde aus dem OkHttp-Interceptor herausgezogen und oberhalb der Interceptor-Kette implementiert.
     * Dadurch werden nun auch Fehler beim Lesen des Response-Body (z. B. bei Attachment-Downloads) korrekt wiederholt.
     * Die bisherige RetryInterceptor-Klasse wurde durch den neuen RetryExecutor ersetzt.

Aktualisierte Bibliotheken

     * com.fasterxml.jackson.core:jackson-databind 2.21.2 → 2.21.3

[3.4.0] - 2026-04-30

Hinzugefügt

Key-Rollover

     * Mehrere private Entschlüsselungsschlüssel können für einen Key-Rollover konfiguriert werden.
     * Der passende Schlüssel wird anhand der Key-ID (kid) im JWE-Header aufgelöst.
     * Ist kein Schlüssel mit der angegebenen Key-ID konfiguriert, wird eine verständliche Fehlermeldung ausgegeben und kein automatisches Reject der
       Submission durchgeführt.

Aktualisierte Bibliotheken

     * org.projectlombok:lombok 1.18.44 → 1.18.46
     * org.junit.jupiter:junit-jupiter 5.14.3 → 5.14.4
     * org.junit.platform:junit-platform-suite-engine 1.14.3 → 1.14.4

[3.3.0] - 2026-04-17

Hinzugefügt

ICAP-Virusscanner

     * Neuer Scanner-Modus ICAP zur Anbindung externer ICAP-Dienste für die Malware-Prüfung
     * Konfigurierbar über VirusScannerMode.ICAP und VirusScannerConfig

Quarantäne-Funktion für infizierte Inhalte

     * Optionales quarantineDirectory in der VirusScannerConfig zum Ablegen infizierter Daten für eine spätere Analyse
     * Infizierte Daten werden bei gesetztem Quarantäneverzeichnis automatisch in die Quarantäne kopiert.

Aktualisierte Bibliotheken

     * com.nimbusds:nimbus-jose-jwt 10.8 → 10.9
     * io.github.git-commit-id:git-commit-id-maven-plugin 9.1.0 → 10.0.0

[3.2.2] - 2026-04-02

Hinzugefügt

Fragment-Syntax für XOEV-Schema-URNs

     * XOEV-Schema-URNs mit Fragment-Anteil werden nun bei der XML-Schema-Validierung akzeptiert, z. B.
       urn:xoev-de:bmk:standard:xbau_2.2#baugenehmigung.antrag.0200.
     * Beim Laden von Schemata aus dem XRepository wird nun das korrekte Schema-Paket geladen.

Aktualisierte Bibliotheken

     * com.fasterxml.jackson.core:jackson-databind 2.21.1 → 2.21.2
     * io.github.git-commit-id:git-commit-id-maven-plugin 9.0.2 → 9.1.0

[3.2.1] - 2026-03-20

Geändert

Verständlichere Fehlermeldung bei abweichender Key-ID

     * Beim Entschlüsseln wird nun die Key-ID aus dem JWE-Header mit der Key-ID des konfigurierten Private Keys abgeglichen.
     * Bei einer Abweichung wird eine verständliche Fehlermeldung ausgegeben, die auf den verwendeten Public Key und den konfigurierten Private Key
       verweist.

[3.2.0] - 2026-03-16

Hinzugefügt

Benutzerdefinierte Attachment-ID (Story)

     * Attachments können mit einer vom Benutzer vorgegebenen attachmentID erzeugt werden
     * wird keine explizite attachmentId gesetzt, wird weiterhin automatisch eine UUID erzeugt
     * hierdurch bleiben fachliche Referenzen beim Senden von Anhängen erhalten

   Beispiel:

   UUID customAttachmentRef = UUID.randomUUID();

   Attachment attachment = Attachment.fromByteArray(data, "application/text", customAttachmentRef);

   Alle bisherigen statischen Attachment Methoden wurden mit einer Variante zum Setzen einer attachmentId überladen

Aktualisierte Bibliotheken

     * com.nimbusds:nimbus-jose-jwt 10.7 → 10.8
     * org.projectlombok:lombok 1.18.42 → 1.18.44
     * org.yaml:snakeyaml 2.5 → 2.6
     * com.fasterxml.jackson.core:jackson-databind 2.21.0 → 2.21.1
     * org.mockito:mockito-core 5.21.0 → 5.23.0
     * org.apache.maven.plugins:maven-surefire-plugin 3.5.4 → 3.5.5
     * org.apache.maven.plugins:maven-failsafe-plugin 3.5.4 → 3.5.5

[3.1.3] - 2026-02-18

Behoben

Ergänzung des type-Felds in NewEventsCallback (Story)

     * NewEventsCallback enthält nun analog zu NewSubmissionsCallback/NewRepliesCallback ein type-Feld
     * Der vom ZSD im JSON-Body mitgesendete Callback-Typ wird beim Deserialisieren korrekt gemappt

Fix der Trusted-Root-Certificates (Story)

     * certificate-config.yaml verweist nicht mehr auf obsolete Zertifikate (PCA-1-Verwaltung-14.pem, PCA-1-Verwaltung-15.pem)
     * Die Referenz für das Zertifikat PCA-1-Verwaltung-26.pem wurde hinzugefügt
     * Initialisierung lädt nur noch existierende Root-Zertifikate

Nachladen externer Schema-Ressourcen per HTTP bei XML-Validierung (Story)

     * Extern referenzierte Schemata/Imports werden nun bei der Validierung über HTTP nachgeladen
     * Dadurch stehen Definitionen aus eingebundenen XSDs während der Prüfung vollständig zur Verfügung

Aktualisierte Bibliotheken

     * ch.qos.logback:logback-classic 1.5.25 → 1.5.32
     * org.junit.jupiter:junit-jupiter 5.14.2 → 5.14.3
     * org.junit.platform:junit-platform 1.14.2 → 1.14.3

[3.1.2] - 2026-01-28

Behoben

Fix des Version Comparators (Story)

     * Der MetadataBuilder lieferte in determineMetadataVersion(requestedVersion, destination) bei identischen Eingaben nicht-deterministische Ergebnisse
     * Ein Fehler in der Vergleichslogik des Version-Objekts wurde behoben

Ergänzung der fehlenden Angabe der Metadaten-Version im Reply (Story)

     * Beim Erzeugen von Replies kann die gewünschte Metadata-Version nun explizit konfiguriert werden
     * SendableReply unterstützt analog zu SendableSubmission die Angabe einer bevorzugten MetadataVersion statt die Version fest auf 1.x zu setzen

Mehrfache Berechnung der AuthenticationTags (Story)

     * Beim Attachment-Up- und Download werden JWE-Payload und Auth-Tag nun jeweils in einem Schritt verarbeitet
     * Dadurch werden doppeltes Parsen und unnötige RAM-Allokationen vermieden
     * Die Nutzung von AuthenticationTags.getAuthTagFromJWT(...) entfällt, da De-/Encryption das Auth-Tag bereits liefert

Entfernt

Reply Schema Validierung (Story)

     * Die fachlich falsche Validierung von Replies gegen destination.publicServices[].submissionSchemas wurde entfernt
     * Replies werden nun nicht mehr aufgrund eines nicht passenden Submission-Schemas abgelehnt
     *

Reply Metadaten Validierung gegen den Zustellpunkt (Story)

     * Die Prüfung der vom Reply gesendeten Metadaten Version wird nicht mehr gegen den Zustellpunkt geprüft
     * Da die unterstützte Version nicht explizit bekannt ist, ist eine Validierung der Reply Metadaten Version gegen die Versionen des Zustellpunkts
       fachlich nicht sinnvoll

Aktualisierte Bibliotheken

     * ch.qos.logback:logback-classic 1.5.22 → 1.5.25
     * com.fasterxml.jackson.core:jackson-databind 2.20.1 → 2.21.0
     * com.fasterxml.jackson.core:jackson-annotations 2.20 → 2.21

[3.1.3] - 2026-02-18

Behoben

Ergänzung des type-Felds in NewEventsCallback (Story)

     * NewEventsCallback enthält nun analog zu NewSubmissionsCallback/NewRepliesCallback ein type-Feld
     * Der vom ZSD im JSON-Body mitgesendete Callback-Typ wird beim Deserialisieren korrekt gemappt

Fix der Trusted-Root-Certificates (Story)

     * certificate-config.yaml verweist nicht mehr auf obsolete Zertifikate (PCA-1-Verwaltung-14.pem, PCA-1-Verwaltung-15.pem)
     * Die Referenz für das Zertifikat PCA-1-Verwaltung-26.pem wurde hinzugefügt
     * Initialisierung lädt nur noch existierende Root-Zertifikate

Nachladen externer Schema-Ressourcen per HTTP bei XML-Validierung (Story)

     * Extern referenzierte Schemata/Imports werden nun bei der Validierung über HTTP nachgeladen
     * Dadurch stehen Definitionen aus eingebundenen XSDs während der Prüfung vollständig zur Verfügung

Aktualisierte Bibliotheken

     * ch.qos.logback:logback-classic 1.5.25 → 1.5.32
     * org.junit.jupiter:junit-jupiter 5.14.2 → 5.14.3
     * org.junit.platform:junit-platform 1.14.2 → 1.14.3

[3.1.2] - 2026-01-28

Behoben

Fix des Version Comparators (Story)

     * Der MetadataBuilder lieferte in determineMetadataVersion(requestedVersion, destination) bei identischen Eingaben nicht-deterministische Ergebnisse
     * Ein Fehler in der Vergleichslogik des Version-Objekts wurde behoben

Ergänzung der fehlenden Angabe der Metadaten-Version im Reply (Story)

     * Beim Erzeugen von Replies kann die gewünschte Metadata-Version nun explizit konfiguriert werden
     * SendableReply unterstützt analog zu SendableSubmission die Angabe einer bevorzugten MetadataVersion statt die Version fest auf 1.x zu setzen

Mehrfache Berechnung der AuthenticationTags (Story)

     * Beim Attachment-Up- und Download werden JWE-Payload und Auth-Tag nun jeweils in einem Schritt verarbeitet
     * Dadurch werden doppeltes Parsen und unnötige RAM-Allokationen vermieden
     * Die Nutzung von AuthenticationTags.getAuthTagFromJWT(...) entfällt, da De-/Encryption das Auth-Tag bereits liefert

Entfernt

Reply Schema Validierung (Story)

     * Die fachlich falsche Validierung von Replies gegen destination.publicServices[].submissionSchemas wurde entfernt
     * Replies werden nun nicht mehr aufgrund eines nicht passenden Submission-Schemas abgelehnt
     *

Reply Metadaten Validierung gegen den Zustellpunkt (Story)

     * Die Prüfung der vom Reply gesendeten Metadaten Version wird nicht mehr gegen den Zustellpunkt geprüft
     * Da die unterstützte Version nicht explizit bekannt ist, ist eine Validierung der Reply Metadaten Version gegen die Versionen des Zustellpunkts
       fachlich nicht sinnvoll

Aktualisierte Bibliotheken

     * ch.qos.logback:logback-classic 1.5.22 → 1.5.25
     * com.fasterxml.jackson.core:jackson-databind 2.20.1 → 2.21.0
     * com.fasterxml.jackson.core:jackson-annotations 2.20 → 2.21

[3.1.2] - 2026-01-28

Behoben

Fix des Version Comparators (Story)

     * Der MetadataBuilder lieferte in determineMetadataVersion(requestedVersion, destination) bei identischen Eingaben nicht-deterministische Ergebnisse.
     * Ein Fehler in der Vergleichslogik des Version-Objekts wurde behoben.

Ergänzung der fehlenden Angabe der Metadaten-Version im Reply (Story)

     * Beim Erzeugen von Replies kann die gewünschte Metadata-Version nun explizit konfiguriert werden.
     * SendableReply unterstützt analog zu SendableSubmission die Angabe einer bevorzugten MetadataVersion statt die Version fest auf 1.x zu setzen.

Mehrfache Berechnung der AuthenticationTags (Story)

     * Beim Attachment-Up- und Download werden JWE-Payload und Auth-Tag nun jeweils in einem Schritt verarbeitet.
     * Dadurch werden doppeltes Parsen und unnötige RAM-Allokationen vermieden.
     * Die Nutzung von AuthenticationTags.getAuthTagFromJWT(...) entfällt, da De-/Encryption das Auth-Tag bereits liefert.

Entfernt

Reply Schema Validierung (Story)

     * Die fachlich falsche Validierung von Replies gegen destination.publicServices[].submissionSchemas wurde entfernt.
     * Replies werden nun nicht mehr aufgrund eines nicht passenden Submission-Schemas abgelehnt.

Reply Metadaten Validierung gegen den Zustellpunkt (Story)

     * Die Prüfung der vom Reply gesendeten Metadaten Version wird nicht mehr gegen den Zustellpunkt geprüft.
     * Da die unterstützte Version nicht explizit bekannt ist, ist eine Validierung der Reply Metadaten Version gegen die Versionen des Zustellpunkts
       fachlich nicht sinnvoll.

Aktualisierte Bibliotheken

     * ch.qos.logback:logback-classic 1.5.22 → 1.5.25
     * com.fasterxml.jackson.core:jackson-databind 2.20.1 → 2.21.0
     * com.fasterxml.jackson.core:jackson-annotations 2.20 → 2.21

[3.1.1] - 2026-01-09

Behoben

Breaking ReplyChannel Getter (Story)

     * Der ReplyChannel-Getter getEmail wurde nach seiner Umbenennung in Version 3.1.0 wieder eingeführt, um einen Breaking Change zu vermeiden

Proxy-Auth Deadlock (Story)

     * Deadlock beim Einsatz eines Proxys mit falschen Credentials wurde behoben
     * Zusätzliches callTimeout im OkHttpClient über TimeoutConfig konfigurierbar (Default: 30s)

Aktualisierte Bibliotheken

     * com.nimbusds:nimbus-jose-jwt 10.6 → 10.7
     * org.junit.jupiter:junit-jupiter 5.14.1 → 5.14.2
     * org.junit.platform:junit-platform 1.14.1 → 1.14.2
     * org.sonatype.central:central-publishing-maven-plugin 0.9.0 → 0.10.0

[3.1.0] - 2025-12-23

Hinzugefügt

Virenscanner-Integration (Story)

     * Integration eines Virenscanners zur Überprüfung von Metadaten, Fachdaten und Attachments auf Malware
     * Unterstützung für verschiedene Scanner-Modi:
          + ClamAV Daemon: Verbindung zu einem laufenden ClamAV-Daemon über TCP-Socket (empfohlen für den produktiven Einsatz)
          + ClamAV Process: Ausführung von ClamAV als separater Prozess im Dateisystem
          + No-Op: Deaktivierter Scanner für Tests oder wenn kein Scan benötigt wird
     * Automatische Überprüfung beim Senden von Attachments und Empfangen von Submissions und Replies
     * Konfigurierbar über ApplicationConfig mit VirusScannerMode und VirusScannerConfig

   Beispiel-Konfiguration (YAML):

   virusScannerMode: clamav-daemon

   virusScannerConfig:
   clamAVDeamonConfig:
   host: localhost
   port: 3310

   Beispiel-Konfiguration (Java):

   ApplicationConfig config = ApplicationConfig.builder()

   .virusScannerMode(VirusScannerMode.CLAMAV_DAEMON)
   .virusScannerConfig(VirusScannerConfig.daemonOnly("localhost", 3310))
   .build();

Entfernt

Logback Abhängigkeit (Story)

     * Die Runtime-Abhängigkeit auf ch.qos.logback und die logback.xml Konfiguration wurden entfernt, um Logging-Framework-Konflikte beim Einsatz des SDKs
       in anderen Projekten zu vermeiden.
     * Das SDK nutzt nur noch die SLF4J-API als Laufzeitabhängigkeit
     * Die konkrete Logging-Implementierung bleibt der Anwendung überlassen.

Behoben

XML-Schema Validierung: Namespace-Auflösung (Story)

     * Bei der Validierung von XML-Schemata wird die Auflösung eingebetteter Schemata anhand der SystemId nun korrekt durchgeführt und
     * Die interne XMLSchemaLoader API wurde von Map<SchemaIdentifier, InputStream> auf Map<String, byte[]> geändert

Auto-Reject: Event Log Fehler (Story)

     * Bei der EventLog Validierung wurde beim Senden vieler (> 100) Submissions nicht alle Events geladen
     * Dies führte zum Fehler NotExactlyOneSubmitEvent und einem darauf flgenden AutoReject durch das SDK
     * Während des Eventlog Abrufs werden nun alle verfügbaren Events mittels Pagination geladen

Fehlende Metadaten-Schema Referenz (Story)

     * Die Liste metadataSchemaPaths in client/src/main/resources/schema-config.yaml wurde um ein fehlendes Schema ergänzt und enthält nun auch die
       Metadatenschema-Version 1.4.0.
     * Das Schema wird nun korrekt über den SchemaResourceProvider registriert und steht für Validierungen zur Verfügung

Aktualisierte Bibliotheken

     * com.nimbusds:nimbus-jose-jwt 10.5 → 10.6
     * ch.qos.logback:logback-classic 1.5.19 → 1.5.22
     * com.fasterxml.jackson.core:jackson-databind 2.20.0 → 2.20.1
     * com.networknt:json-schema-validator 2.0.0 → 2.0.1
     * org.junit.jupiter:junit-jupiter 5.14.0 → 5.14.1
     * org.junit.platform:junit-platform 1.14.0 → 1.14.1
     * org.mockito:mockito-core 5.20.0 → 5.21.0
     * org.wiremock:wiremock 3.13.1 → 3.13.2
     * org.apache.maven.plugins:maven-source-plugin 3.3.1 → 3.4.0
     * org.testcontainers:testcontainers 1.21.4 (neu)

[3.0.0] - 2025-10-28

Überblick

   Mit Version 3.0.0 erfolgt ein Major-Release des FIT-Connect Java SDKs. Dieses Release beinhaltet Breaking Changes, die eine Anpassung bestehender
   Integrationen erfordern. Alle Änderungen sind im Migration Guide 2.x → 3.x beschrieben.

Geändert

API-Versionierung und Endpunktmigration (Story)

     * Alle API-Endpunkte wurden auf Version /v2 migriert.
     * Die Migration betrifft Submission-, Destination- und Routing-Endpunkte.
     * Die Umstellung erfolgt automatisch durch die aktualisierten Clients im SDK.

Klassen- und Property-Umbenennungen (Story)

     * Zur Vereinheitlichung der Domain-Modelle wurden mehrere Klassen umbenannt:

              Alt              Neu                            Paket
       ServiceType        PublicService dev.fitko.fitconnect.api.domain.model.submission
       StatusEnum         Status        dev.fitko.fitconnect.api.domain.model.destination
       DestinationService Service       dev.fitko.fitconnect.api.domain.model.destination
     * Entsprechend wurden folgende Properties angepasst:

                 Klasse                Alt          Neu
       Submission                  serviceType publicService
       AnnounceSubmission          serviceType publicService
       SendableSubmission          serviceType publicService
       SendableEncryptedSubmission serviceType publicService

Anpassung der Collection-Typen

     * SubmissionsForPickup.submissions: Set<SubmissionForPickup> → List<SubmissionForPickup>

Umbenennungen in Klassen und Properties

     * Klassen:
          + ServiceType → PublicService
          + StatusEnum → Status
          + DestinationService → Service
     * Properties:
          + serviceType → publicService

Entfernt

     * Property callback in Submission
     * Alle in 2.x als deprecated markierten Attachment.fromString(...)-Methoden wurden entfernt.

Aktualisierte Bibliotheken

     * com.nimbusds:nimbus-jose-jwt 9.37.3 → 10.5
     * ch.qos.logback:logback-classic 1.5.18 → 1.5.19
     * org.projectlombok:lombok 1.18.38 → 1.18.42
     * org.yaml:snakeyaml 2.4 → 2.5
     * com.squareup.okhttp3:okhttp (neue groupId) 5.0.0-alpha.14
     * com.fasterxml.jackson.core:jackson-databind 2.19.2 → 2.20.0
     * com.fasterxml.jackson.core:jackson-annotations 2.19.2 → 2.20
     * com.networknt:json-schema-validator 1.5.8 → 1.5.9
     * org.junit.jupiter:junit-jupiter 5.13.4 → 5.14.0
     * org.junit.platform:junit-platform 1.13.4 → 1.14.0
     * org.mockito:mockito-core 5.19.0 → 5.20.0
     * org.wiremock:wiremock (neue groupId) 3.13.1
     * org.apache.maven.plugins:maven-surefire-plugin 3.5.3 → 3.5.4
     * org.apache.maven.plugins:maven-failsafe-plugin 3.5.3 → 3.5.4
     * org.apache.maven.plugins:maven-javadoc-plugin 3.11.3 → 3.12.0
     * org.apache.maven.plugins:maven-gpg-plugin 3.2.8 → 3.3.0
     * org.apache.maven.plugins:maven-central-publishing-plugin 0.8.0 → 0.9.0
     * com.diffplug.spotless:maven-spotless-plugin 2.44.5 → 2.46.1

[2.9.0] - 2025-08-20

Hinzugefügt

Vereinfachte Reply Erstellung (Story)

     * Neue forCase(UUID caseId) Methode ermöglicht Reply-Erstellung unabhängig von der ReceivedSubmission
     * Vereinfachte Nutzung von Replys nach Accept/Reject Events durch Verwendung von Case-ID und Verschlüsselungsschlüssel
     * Beispiel: SendableReply.forCase(caseId).setJsonData(replyData, schemaUri).build()
     * Die bisherige from(ReceivedSubmission) Methode ist als deprecated markiert und wird in einer zukünftigen Version entfernt

Geändert

Filterung invalider Routen im Router Client (Story)

     * Kein Abbruch mehr durch Exceptions bei der Routen-Validierung in RouterClient#findDestinations(search)
     * Invalide Routen werden als Log-Meldungen (WARNING) ausgegeben und valide Routen zurückgegeben

Maven Release Deployment (Story)

     * Migration von OSSRH (OSS Repository Hosting) zum Maven Central Portal aufgrund des OSSRH End-of-Life am 30.06.25
     * Verwendung des neuen Maven Central Publishing Plugins für das Deployment von Snapshots und Releases

Aktualisierte Bibliotheken

     * com.fasterxml.jackson.core:jackson-annotations 2.19.1 → 2.19.2
     * com.fasterxml.jackson.core:jackson-databind 2.19.1 → 2.19.2
     * org.junit.jupiter:junit-jupiter 5.13.1 → 5.13.4
     * org.junit.platform:junit-platform-suite-engine 1.13.1 → 1.13.4
     * com.networknt:json-schema-validator 1.5.7 → 1.5.8
     * org.apache.maven.plugins.maven-gpg-plugin 3.2.7 -> 3.2.8

Hinzugefügte Bibliotheken

     * org.sonatype.central.central-publishing-maven-plugin 0.8.0 (ersetzt das nexus staging plugin)

[2.8.1] - 2025-06-17

Behoben

AuthenticationTag Parsing (Story)

     * Es wurde ein Fehler beim Parsen der AuthenticationTags für bereits verschlüsselten Submissions behoben
     * Dies ist für den Abruf des Status einer Submission als Sender relevant, da hier die AuthenticationTags geprüft werden

Aktualisierte Bibliotheken

     * Jackson-Annotations 2.19.1
     * Jackson-Databind 2.19.1
     * Junit 5.13.1
     * Junit-Platform 1.13.1
     * Wiremock 3.13.1

[2.8.0] - 2025-05-30

Hinzugefügt

Zugriff auf das vollständige Event-Log (Story)

     * Sender- und Subscriber Client haben nun, neben dem Abruf des letzten Status, auch Zugriff auf das gesamte Event-Log
     * SenderClient:
     * Abruf des Case-Event-Logs über getEventLogForCase(destinationId, caseId)
     * Abruf des Submission-Event-Logs über getEventLogForSubmission(sentSubmission), die AuthenticationTags der SentSubmission sind hierbei notwendig
     * SubscriberClient:
     * Abruf des Case-Event-Logs über getEventLogForCase(destinationId, caseId)
     * Abruf des Submission-Event-Logs über getEventLogForSubmission(destinationId, caseId, submissionId)

Geändert

OAuth-Token Mindestgültigkeit (Story)

     * Beim Abrufen des OAuth-Tokens wurde ein Leeway von 5 Sekunden eingeführt, um zu verhindern, dass Tokens mit sehr kurzer Restlaufzeit für den
       nächsten Request nicht mehr gültig sind
     * Der Token wird schon dann erneuert, wenn die Mindestlaufzeit unterschritten wird

Anpassung der Signaturprüfung der Adressierungsinformationen (Story)

     * Die Prüfung der DestinationSignature wurde angepasst, sodass Signaturen vom Self-Service-Portal(SSP) und Zustelldienst (ZSD) akzeptiert werden
     * Zusätzlich wird geprüft ob der Issuer mit der Domain .fit-connect.fitko.net (bzw..fit-connect.fitko.dev beim Testsystem) endet

Aktualisierte Bibliotheken

     * JsonSchemaValidator 1.5.7

[2.7.0] - 2025-05-23

Geändert

Restriktion paralleler Attachment Up- und Downloads (Story)

     * Der Upload und Download von Attachments für Submissions und Replys wird durch eine Semaphore mit dem default Wert n=3 limitiert
     * Dies verhindert unnötige Retries bei einem auftretenden HTTP-Status 429 (Too Many Requests)

[2.6.3] - 2025-05-16

Behoben

Fix ZBP Environment URL (Story)

     * ein fehlendes Zeichen in der PROD_NDB_VN wurde ergänzt

[2.6.2] - 2025-04-25

Behoben

Fix von fehlenden @JsonProperty Annotation für Jackson 2.19.0

     * Das Jackson Update auf Jackson 2.19.0 erfordert für alle Model Klassen, die durch den ObjectMapper deserialisiert werden, an allen Properties eine
       @Jsonproperty Annotation

Aktualisierte Bibliotheken

     * Jackson-Annotations 2.19.0
     * Jackson-Databind 2.19.0
     * Junit 5.12.2
     * Junit-Platform 1.12.2
     * Wiremock 3.13.0

[2.6.1] - 2025-04-09

Behoben

Validierung von Ereignissen mit Angabe eines expliziten Prüfdatums (Story)

     * Möglichkeit der Validierung von Signaturen und Keys die mit einem in der Vergangenheit gültigen Zertifikat signiert wurden
     * Dies betrifft die Validierung der DestinationSignature im RoutingService und die PublicKey-Validierung im ValidationService
     * Als Prüfdatum wird dabei das IssuedAt (iat) der DestinationSignature bzw. des SETs eines Events genutzt

Aktualisierte Bibliotheken

     * JWK-Validator 2.2.0
     * Logback 1.5.18
     * Lombok 1.18.38
     * Junit 5.12.1
     * Junit-Platform 1.12.1
     * Mockito 5.17.0
     * Maven-Surefire-Plugin 3.5.3
     * Maven-Failsafe-Plugin 3.5.3
     * Maven-Jacoco-Plugin 0.8.13

[2.6.0] - 2025-03-12

Hinzugefügt

Limit-Abruf und ChangeRequests über Destination-Client (Story)

     * der Destination-Client wurde um die Funktionalitäten der Limit-API erweitert
     * über getAttachmentLimits(destinationId) können die Limits einer Destination über abgerufen werden
     * über requestLimitChange(destinationId, limits, requestReason) können neue Limits beantragt werden
     * über withdrawLimitChangeRequest(destinationId) kann ein Change-Request zurückgezogen werden

Geändert

Nutzung dynamischer Attachment-Limits (Story)

     * beim Chunking wird die von der Limit-API erlaubte max. Größe für Attachments genutzt wenn
          + kein Wert für die Chunking-Größe gesetzt wurde
          + wenn die Chunking-Größe die max. zulässige Attachment-Größe überschreitet
     * das Chunking erlaubt weiterhin die Angabe eines benutzerdefinierten Wertes für die Chunk-Größe (siehe AttachmentChunkingConfig)

Aktualisierte Bibliotheken

     * Mockito 5.16.0
     * Wiremock 3.12.1

[2.5.2] - 2025-03-03

Behobene Bugs

Fix des Sendens von Accept-Events bei Submissions mit Fachdaten-Attachments (Story)

     * ein SET-Validierungsfehler beim Senden von Accept-Events wurde behoben
     * der Fehler trat beim Akzeptieren von empfangenen Submissions auf, wenn diese Fachdaten-Anhänge beinhalteten

Aktualisierte Bibliotheken

     * SnakeYaml 2.4
     * JsonSchemaValidator 1.5.6
     * Logback 1.5.17
     * Slf4j 2.0.17
     * Junit 5.12
     * Junit-Platform 1.12.0
     * Maven-Compiler-Plugin 3.14.0
     * Maven-Deploy-Plugin 3.1.4
     * Maven-Install-Plugin 3.1.4

[2.5.1] - 2025-02-14

Änderungen

XML-Validierung erlaubter URN-Schemata (Story)

     * es werden per default nur XML-Schemata validiert die den Mustern in AllowedSchemaPatterns entsprechen
     * aktuell sind dies XOEV URNs, andere Muster werden bei der Validierung übersprungen dies verhindert, dass die Validierung global deaktiviert werden
       muss, wenn kein XOEV-Schema vorliegt

Aktualisierte Bibliotheken

     * Wiremock 3.12.0

[2.5.0] - 2025-01-28

Hinzugefügt

XML-Schema Validierung von XÖV-Standards (Story)

     * XML-Fachdaten werden über die Angabe der Schema-URI gegen ein XOEV-Schema validiert
     * die Validierung ist für alle im XRepository verfügbaren Schemata möglich
     * ist keine Validierung gewünscht, oder ist das Schema nicht im repository verfügbar, kann die Fachdatenvalidierung wie gewohnt über die
       Environment-Konfiguration des SDK deaktiviert werden (siehe skipSubmissionDataValidation)

Aktualisierte Bibliotheken

     * Logback 1.5.16
     * JsonSchemaValidator 1.5.5
     * Junit 5.11.4
     * Junit-Platform 1.11.4
     * Mockito 5.15.2
     * Wiremock 3.10.0
     * Maven-JavaDoc-Plugin 3.11.2

[2.4.0] - 2024-11-29

Hinzugefügt

Destination API Client (Story)

     * neuer Client zur Anlage und zum Management von Zustellpunkten über ClientFactory.createDestinationClient(config)
     * Nutzung über die Subscriber-Credentials
     * siehe auch Destination API und Doku

Angabe der Region (ARS) für nachgelagertes Routing (Story)

     * die SendableSubmission ermöglicht über .setServiceTypeWithRegion(leikaKey, name, region), das Setzen einer Region für eine bestimmte Leistung
     * enthält eine Submission eine Region, muss diese zum ARS im Zustellpunkt passen

ZBP-Zertifikatshandling ohne mTLS (Story)

     * für die Nutzung des ZBP-Clients im ZBP-Adapter wurde das Zertifikatshandling für den Einsatz in der produktiven ZBP-Infrastruktur angepasst
     * das ZBP benötigt hierbei das TLS/ZBP-Postfachzertifikat des ZBP-Adapters im PEM-Format als Cookie im HTTP-Header

Aktualisierte Bibliotheken

     * Jackson-Annotations 2.18.2
     * Jackson-Databind 2.18.2
     * JsonSchemaValidator 1.5.4

[2.3.5] - 2024-11-15

Behobene Bugs

Fix Routing DestinationParameters-Signature Validierung (Story und Story )

     * Die Validierung der destinationParametersSignature wird aufgrund der Abkündigung des Feldes nicht mehr durchgeführt
     * siehe hierzu auch Prüfung der DestinationParameters-Signature

Änderungen

     * Deprecation der Felder destinationParameters und destinationParametersSignature (Story)
     * Update und Erweiterung des Route-Models (Story)

Aktualisierte Bibliotheken

     * Lombok 1.18.36

[2.3.4] - 2024-11-01

Behobene Bugs

Fix des Eventlog Status-Mapping (Story)

     * Es wurde ein Fehler behoben, der beim Mapping des Eventlog-Status auftrat, wenn ein ACCEPT-Event eine Liste von Problems beinhaltete

Aktualisierte Bibliotheken

     * JsonSchemaValidator 1.5.3

[2.3.3] - 2024-10-29

Behobene Bugs

Ableitung des Token Signers aus dem Zertifikat (Story)

     * Der Signer des ZBP-Tokens ist nicht mehr hart codiert "FIT-Connect", sondern entspricht dem Subject des Zertifikates

Aktualisierte Bibliotheken

     * Junit 5.11.3
     * Junit Platform 1.11.3
     * Wiremock 3.9.2

[2.3.2] - 2024-10-21

Behobene Bugs

Validierung von Submission API Alias URLs (Story)

     * Ein Fehler bei der Validierung von Alias-URLs innerhalb der Destination Signatur wurde behoben

[2.3.1] - 2024-10-18

Änderungen

Setzen der Sender-Reference (Story)

     * Fehlende Möglichkeit des Setzens der senderReference in den AdditionalReferenceInfo wurde hinzugefügt
     * Neue Methode setSenderReference im SubmissionBuilder als optionales Property

Update Submission API URLs (Story)

     * Änderung auf *.fit-connect.fitko.net/Submission API
     * Die Umgebungen nutzen neue Aliase die in den default Environments des SDKs aktualisiert wurden

Aktualisierte Bibliotheken

     * Jackson-Annotations 2.18.0
     * Jackson-Databind 2.18.0
     * Logback 1.5.11
     * Junit 5.11.2
     * Mockito 5.14.2
     * Maven-Surefire-Plugin 3.5.1
     * Maven-Failsafe-Plugin 3.5.1
     * Maven-JavaDoc-Plugin 3.10.1
     * Maven-GPG-Plugin 3.2.7

[2.3.0] - 2024-09-26

Hinzugefügt

Versand von Nachrichten an das Zentrale Bürgerpostfach (ZBP) der BundID (Story)

     * Neuer Client zum Senden von ZBP Nachrichten und Statusupdates ClientFactory.createZBPClient(...)
     * Ermöglicht das Senden von Nachrichten und Statusupdates (siehe Doku FIT-Connect und das Zentrale Bürgerpostfach (ZBP))
     * auf direktem Weg innerhalb der Netze des Bundes (NdB)
     * auf indirektem Weg außerhalb der Netze des Bundes (NdB) über die FIT-Connect Infrastruktur

JWK Test Key Generator (Story)

     * TestKeyBuilder zur Erzeugung von JWKs für die FIT-Connect TEST und DEV Umgebung:
     * TestKeyBuilder.generateEncryptionKeyPair()
     * TestKeyBuilder.generateSignatureKeyPair()
     * KeyWriter zum Schreiben der Keys ins Dateisystem KeyWriter.writeKeys(keyWriterSettings)

Aktualisierte Bibliotheken

     * JsonSchemaValidator 1.5.2
     * Wiremock 3.9.1
     * Junit 5.11.1

[2.2.0] - 2024-09-16

Hinzugefügt

Übertragung großer Fachdaten als Anhang (Story)

     * Übertragung von Fachdaten als Attachment, wenn das zulässige Limit im Metadatensatz von aktuell 13 MB überschritten wird
     * Beim Empfang werden die Fachdaten wie bisher in der ReceivedSubmission verfügbar gemacht
     * Alle Validierungen werden wie bisher durchgeführt
     * Beim Senden können die Daten zusätzlich als Byte-Array gesetzt werden:
     * setJsonData(bytes,dataSchemaUri)
     * setXmlData(bytes,dataSchemaUri)
          + Beim Empfang gibt es zusätzlich die Möglichkeit auf die Fachdaten über receivedSubmission.getDataAsBytes() als Byte-Array zuzugreifen

Automatische Retry-Logik (Story)

     * Konfigurierbare Retry-Logik mit definierbaren HTTP-Status-Codes und Anzahl der Wiederholungsversuche
     * Programmatisch via dev.fitko.fitconnect.api.config.http.RetryConfig und über die Config-YAML
     * Die Wartezeit zwischen den Retries erhöht sich mittels ExponentialBackoff
     * Per Default sind folgende Werte konfiguriert:
     * 5 Retries
     * StatusCodes [408, 429, 500, 502, 503, 504]
     * initialer Startwert von 500ms

BundID als Rückkanal (Story)

     * Übertragung der mailboxId im BundId-ReplyChannel der Submission
     * Übertragung der AntragsId in der AdditionalReferenceInfo des Metadatensatzes

Änderungen

     * Update des Metadatenschemas auf 1.5.0
     * Dieses beinhaltet den Attachment Purpose Data, um Anhänge als Fachdaten zu markieren
     * Neuer BundId-ReplyChannel
     * AdditionalReferenceInfo wurde um das Feld idBundDeApplicationId erweitert um die Antrags-ID (applicationId) für den BundId-Statusmonitor zu
       übertragen
     * Zugriff auf den Verwendungszweck des Attachments über attachment.getPurpose()

Aktualisierte Bibliotheken

     * SnakeYaml 2.3
     * JsonSchemaValidator 1.5.1
     * Logback 1.5.8
     * Slf4j 2.0.16
     * Junit 5.11
     * Mockito 5.13
     * Maven-Surefire-Plugin 3.5.0
     * Maven-Failsafe-Plugin 3.5.0
     * Maven-Checkstyle-Plugin 3.5.0
     * Maven-Install-Plugin 3.1.3
     * Maven-Deploy-Plugin 3.1.3
     * Maven-JavaDoc-Plugin 3.10.0
     * Maven-GPG-Plugin 3.2.6

[2.1.0] - 2024-07-24

Hinzugefügt

     * Attachment Chunking (Story)
          + Übertragung großer Attachment-Files (> 500 MB) als Fragmente
          + Abruf fragmentierter Attachments und zusammensetzen der Chunks zum ursprünglichen File
          + neue Methode Attachment.fromLargeAttachment() zum autom. Chunking
     * Basic Authentication über Nutzername und Passwort für die HTTP-Proxy Konfiguration (Story)
     * Zugriff auf den Issuer im Eventlog Status (Story)
     * Zugriff auf die AttachmentId (Story)
     * In der SubscriberConfig können die privaten Keys zur Entschlüsselung und zum Signieren auch direkt als JWK übergeben werden (Story)

Änderungen

     * Update des Metadatenschemas auf 1.3.0
     * Update des SET-Schemas auf 1.2.2
     * Modulkonfiguration
          + die Module API und Core wurden in das Client-Modul integriert
          + damit ist der Client nicht mehr von zwei weiteren Modulen abhängig, sondern erzeugt im Build und beim Laden über Maven nur noch ein einzelnes
            Client-Modul

Entfernt

   Die Zip-Kompression im JWE-Header wurde entfernt (Story). Siehe hierzu:

     RFC 8725, Sec. 3.6: Compression of data SHOULD NOT be done before encryption, because such compressed data often reveals information about the
     plaintext.

Aktualisierte Bibliotheken

     * JWKValidator 2.1.1
     * Nimbus/Jose 9.37.3
     * Lombok 1.18.34
     * Jackson 2.17.2
     * JsonSchemaValidator 1.5.0
     * Junit 5.10.3
     * Maven-Surefire-Plugin 3.3.1
     * Maven-Failsafe-Plugin 3.3.1
     * Maven-Jar-Plugin 3.4.2
     * Maven-Checkstyle-Plugin 3.4.0
     * Maven-Javadoc-Plugin 3.8.0
     * Maven-Git-Commit-Plugin 9.0.1

[2.0.3] - 2024-05-23

Änderungen

   Downgrade der Nimbus/Jose Library auf 9.37.x, um die Größenbeschränkung des Payloads bei aktivierter Zip-Compression im JWE Header aufzuheben. Siehe
   hierzu https://bitbucket.org/connect2id/nimbus-jose-jwt/issues/545

Aktualisierte Bibliotheken

     * Nimbus/Jose 9.37

[2.0.2] - 2024-05-23

Behobene Bugs

     * Fix des transactionTimestamp Datentyps der PaymentInformation im Metadaten (Story)

Aktualisierte Bibliotheken

     * Nimbus/Jose 9.39
     * Lombok 1.18.32
     * OkHttp 5.0.0-alpha.14
     * Jackson 2.17.1
     * JsonSchemaValidator 1.4.0
     * Logback 1.5.6
     * Mockito 5.12
     * Slf4j 2.0.13

[2.0.1] - 2024-03-15

Behobene Bugs

     * Die Validierung von ARS Schlüsseln bei der Abfrage von Routing-Informationen wurde so angepasst, dass auch Regionen die in einer anderen Region
       enthalten sind, korrekt geprüft werden (Story)

Änderungen

   Um im Log keine sensiblen Daten zu loggen wurden folgende Anpassungen in der Logback Konfiguration vorgenommen:
     * Das Loglevel für den Schema Validator wurde für com.networknt.* auf INFO gesetzt
     * Das Loglevel für den OkHttp Client wurde für okhttp3.* auf INFO gesetzt

Aktualisierte Bibliotheken

     * JWK-Validator 2.1.0
     * Jackson 2.17.0
     * Logback 1.5.3
     * Mockito 5.11
     * Maven GPG Plugin 3.2.0
     * Maven Assembly Plugin 3.7.0
     * Maven Git Commit ID Plugin 8.0.1

[2.0.0] - 2024-02-20

Hinzugefügt

     * Bidirektionale Kommunikation mit FIT-Connect Antwortkanal (Story)
          + neue Submissions für einen existierenden Case senden
          + Antworten (Replies) auf Submissions senden
          + Antworten empfangen
          + Liste der verfügbaren Cases und Replies
          + aktuellen Status für Replies abfragen
          + Neuer FitConnect-ReplyChannel, der den öffentlichen Reply-Verschlüsselungsschlüssel und eine Liste von Verarbeitungsstandards überträgt
          + ReplyChannelKeyGenerator zur Erzeugung von ephemeral-keys für die Ver- und Entschlüsselung von Replies
     * ServiceType-Getter an der ReceivedSubmission (Story)
     * Setzen des ApplicationDate an der SendableSubmission über setApplicationDate(localDate) (Story)
          + Zurückdatieren des Einreichungsdatums
          + unabhängig vom technischen issued-at Zeitstempel der Submission
     * Konfiguration von HTTP-Client-Timeouts in der neuen HttpConfig (Story)
          + readTimeout setzen
          + writeTimeout setzen
          + connectionTimeout setzen
     * Für die Validierung lokaler Fachdatenschemata können beliebige Schema-formate vom lokalen Dateisystem über die config.yaml oder ApplicationConfig
       hinzugefügt werden (Story)
     * Neue DTOs für Callback-Benachrichtigung über neue Submissions und Replies (Story)
     * Zugriff auf den technischen submitted-at (iat) Zeitstempel an der ReceivedSubmission (Story)
     * Neuer KeyGenerator zur Erzeugung von X.509-Testzertifikaten, RSA-KeyPairs und JWKs (Story)

Änderungen

     Details zu den Änderungen im Zusammenhang mit diesem Update entnehmen Sie bitte dem Update Guide

     * Aktualisierte Schemata:
          + Metadaten-Schema 1.2.0
          + SET-Schema 1.2.1
     * Model-Klassen vom Client-Modul wurden in das API Modul verschoben (Story)
     * Umbenennung von API Methoden (Story)
          + statische ClientFactory-Methoden zu ClientFactory.create...(config)
          + statische ReplyChannel-Methoden zu ReplyChannel.of...()
          + DefaultEnvironments für TEST, STAGE und PROD wurden umbenannt in Environments
     * Das EventLog Status Objekt wurde umbenannt von SubmissionStatus in Status (Story)
          + dieses wird sowohl für getSubmissionStatus als auch für getReplyStatus verwendet
          + enthält das neue issuedAt Datum
     * Die ReceivedSubmission implementiert Comparable<Date> und wird autom. nach issuedAt sortiert (Story)
     * Neues HttpConfig-Objekt (Story):
          + enthält HTTP TimeoutConfig
          + ProxyConfig wurde nach HttpConfig verschoben
     * Die Polling-Methode getAvailableSubmissionsForDestination (ohne Limit und Offset) lädt nun implizit ALLE verfügbaren Einreichungen (rekursiv in
       25er-Chunks). Die macht es einfacher, eine Liste aller verfügbaren Einreichungen zu laden, ohne explizit Paging einzusetzen. (Story)
     * Der JWKValidator 2.0.0 cached Anfragen, was die Geschwindigkeit der Schlüsselüberprüfung verbessert
     * Detailliertere Protokollierung von Requests mit HTTP-Methode, Statuscode und benötigter Zeit (Story)

Behobene Bugs

     * Das Zeitstempelformat in der Callback-Validierung wurde von Millisekunden auf Sekunden korrigiert (Story)
     * Korrigierte Json Validator Locale, welche zu Build-Fehlern in Umgebungen mit deutscher Locale führte (Story)
     * Der korrekte Content-Type Header (cty) wurde zum JWE-Header hinzugefügt (Story)
     * Ein Fehler, der bei der Validierung von Routing-Services mit mehreren Diensten innerhalb der JWT-Claims auftrat, wurde behoben (Story)

Veraltet

   Die Attachment fromString-Methoden werden ab SDK-Version 3.0.0 als deprecated markiert, da sie bei falscher Verwendung zu Problemen mit der
   Zeichencodierung führen können (Story).

Entfernt

     * Command-Line-Client (Story)
          + der ausführbare Kommandozeilen-Client wurde aus dem SDK entfernt und ist nun ein eigenständiges Projekt
          + siehe FIT-Connect Tools Repository für weitere Informationen
     * Apache Tika-Library (wurde vom cli-Modul verwendet)
     * JCommander-Library (wurde vom cli-Modul verwendet)

Aktualisierte Bibliotheken

     * JWK-Validator 2.0.0
     * Nimbus/Jose 9.37.3
     * OkHttp 5.0.0-alpha.12
     * Lombok 1.18.30
     * Jackson 2.16.1
     * Logback 1.5.0
     * SLF4J 2.0.12
     * SnakeYaml 2.2
     * JsonSchemaValidator 1.3.3
     * Junit 5.10.2
     * Mockito 5.10
     * Wiremock 3.0.1

[1.0.0] - 2023-08-04

Hinzugefügt

     * JWK Test-Zertifikat Generator
     * Integration des Demo-Projekts ins SDK

Änderungen

     * JWK-Validator 1.5.1
     * Mockito Core 5.4.0
     * JSON Schema Validator 1.0.86
     * Maven 3.9.4
     * OpenCSV 5.8

[1.0.0-rc.1] - 2023-03-05

     * Dies war die erste Release-Candidate Version des SDKs.
```
