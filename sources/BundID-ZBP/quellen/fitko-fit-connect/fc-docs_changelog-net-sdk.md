# Changelog .NET-SDK

- Quelle: <https://docs.fitko.de/fit-connect/docs/sdks/net-sdk/changelog>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Changelog

   Auf dieser Seite finden Sie alle wesentlichen Änderungen am .NET-SDK.

   Das Format der Seite basiert auf Keep a Changelog.

   Das .NET-SDK verwendet die Semantische Versionierung.

   Das allgemeine Changelog für FIT-Connect finden Sie unter Changelog.

[Unreleased]

     * Key Rollover für Entschlüsselung: Receiver unterstützen jetzt Zero-Downtime-Schlüsselrotation über Receiver.DecryptionKeys. Das SDK versucht die
       konfigurierten privaten Schlüssel in Reihenfolge, bis die Entschlüsselung erfolgreich ist.
          + ReceiverConfig.DecryptionKey wurde durch ReceiverConfig.DecryptionKeys ersetzt. Die Liste ist geordnet: erster Schlüssel = primär, weitere
            Schlüssel = Fallback für Key-Rollover.
     * Dependency Injection: Der FitConnectClient wird nicht mehr über eine statische Fabrikmethode (CreateClient(...)) erzeugt. Die Einbindung erfolgt
       jetzt ausschließlich über Microsoft.Extensions.DependencyInjection. Registrierung mit services.AddFitConnect(), Bezug als IFitConnectClient per
       Konstruktor-Injektion.
     * Konfiguration: Die Konfiguration erfolgt jetzt über appsettings.json (bzw. jede IConfiguration-Quelle). Die Konfigurationsstruktur bleibt
       inhaltlich gleich; die Bindung an die .NET-Extensions-Infrastruktur ermöglicht flexible Konfigurationsquellen.
     * HTTP-Client-Handling: Internes HTTP-Client-Management wurde auf IHttpClientFactory mit SocketsHttpHandler und
       Microsoft.Extensions.Http.Resilience-Pipelines umgestellt. Retry-Logik wird nun über AddResilienceHandler konfiguriert.
     * JSON-Serialisierung: Migration von Newtonsoft.Json auf System.Text.Json.
     * Interface-Umbenennungen:
          + IDestinationApiService → IDestinationApiClient
          + ICaseApiService → ICaseApiClient
          + IReplyApiService → IReplyApiClient
          + ISubmissionApiService → ISubmissionApiClient
          + IRouteService → IRouteApiClient
     * Konfigurationsmodell:
          + SenderConfig hat keine DecryptionKey- und SignatureKey-Felder mehr (nur ReceiverConfig).
          + Neues Feld Attachments.BaseDirectory zum Setzen eines eigenen Speicherpfads für Anhänge.
          + Korrekte Schreibweise ChunkSizeInMb (war ChunkSizeInMB).
     * DTOs: Alle DTO-Klassen wurden auf record-Typen umgestellt.

[3.1.2]

Behoben

     * ApplicationDate: Fehler bei Null-Werte

Aktualisiert

     * Dependencies: Speziell für .NET 9 (und .NET 8) wurde die Version der Microsoft Extensions auf 9.0.10 gesetzt, um Kompatibilitätsprobleme zu
       beheben.

[3.1.0]

Hinzugefügt

     * Virenprüfung (Antivirus): Integration von Virenscannern für Anhänge. Unterstützt ClamAV (Daemon & lokal) sowie ICAP-Server.
     * Key Rollover Support: Unterstützung für den automatischen Wechsel von Verschlüsselungsschlüsseln
     * Proxy-Authentifizierung: Unterstützung für authentifizierte Proxys in den SDK-Einstellungen
     * Zertifikats-Prüftool: Ein neues CLI-Tool zur Überprüfung von Zertifikaten und Umgebungen im Projekt FitConnect.Helper.Cli
     * Pagination für Event Logs: Beim Abrufen von Events eines Falls wird nun Pagination unterstützt, um auch bei vielen Events alle Daten zu erhalten.

Behoben

     * Fehlerhafte Metadaten v2: Korrektur der Serialisierung und Verarbeitung von Metadaten in Version 2
     * URL-Validierung: Behebung eines Fehlers bei der URL-Generierung für Destination-Listen (doppelte Fragezeichen) und Einführung einer strengeren
       URL-Validierung

Aktualisiert

   Die Dependencies wurden aktualisiert, jedoch nicht mehr global, sondern abhängig von der .NET Version. Daher werden keine Versionen mehr angegeben.
     * Abhängigkeiten
          + Newtonsoft.Json
          + jose-jwt
          + Microsoft.IdentityModel.Tokens
          + Newtonsoft.Json
          + Polly
          + Microsoft.Extensions.Logging.Abstractions
     * Test-Abhängigkeiten
          + Microsoft.Extensions.Configuration.Json

[3.0.3]

Hinzugefügt

     * Feature: Erlaubt das Setzen einer externen AttachmentId
     * Support für .NET10

Behoben

     * Root-Zertifikate inkludiert
     * Möglichkeit, die Beschreibung des Diensttyps (Service Type) beim Senden einer Einreichung zu bearbeiten
     * Unnötiges manuelles Setzen der Content-Length

Aktualisiert

     * Abhängigkeiten:
          + Microsoft.IdentityModel.Tokens auf Version="8.15.0"
          + NJsonSchema auf Version="11.5.2"
          + System.IdentityModel.Tokens.Jwt auf Version="8.15.0"
          + Microsoft.IdentityModel.JsonWebTokens auf Version="8.15.0"
     * Test-Abhängigkeiten:
          + WireMock.Net auf Version="1.24.0"
          + JunitXml.TestLogger auf Version="7.1.0"
          + Testcontainers auf Version="4.10.0"

[3.0.1]

Hinzugefügt

     * Funktionalität zur Generierung von Antwortschlüsseln, um Schlüssel für die Antwortverschlüsselung zu erstellen

Behoben

     * XÖV-Validierungsprobleme

Aktualisiert

     * Abhängigkeiten:
          + NUnit.Analyzers auf Version="4.11.2"

[3.0.0] - 2025-10-27

Hinzugefügt

     * Major Release: FIT-Connect v2.0.0 API-Kompatibilität eingeführt.
     * Für detaillierte Änderungen (die Nutzung des SDK bleibt unverändert), siehe den Migrationsleitfaden

Aktualisiert

     * Abhängigkeiten:
          + Testcontainers-dotnet monorepo auf Version 4.8.1 aktualisiert.
          + WireMock.Net Abhängigkeit auf Version 1.15.0 aktualisiert.

Geändert

API-Versionierung und Endpunktmigration (Story)

     * Alle API-Endpunkte wurden auf Version /v2** migriert.
     * Die Migration betrifft Submission-, Destination- und Routing-Endpunkte.
     * Die Umstellung erfolgt automatisch durch die aktualisierten Clients im SDK.

Klassen- und Property-Umbenennungen (Story)

     * Property-Änderungen:

             Klasse                Alt          Neu
   Submission                  serviceType publicService
   AnnounceSubmission          serviceType publicService
   SendableSubmission          serviceType publicService
   SendableEncryptedSubmission serviceType publicService
     __________________________________________________________________________________________________________________________________________________

Anpassung der Collection-Typen

     * SubmissionsForPickup.submissions: Set<SubmissionForPickup> → List<SubmissionForPickup>

Entfernt

     * Property callback in Submission
     * Alle in 2.x als deprecated markierten Attachment.fromString(...) Methoden wurden entfernt.

Behoben

     * URN-Schema-basierte Validierung zur Verbesserung der Datenkonformität und -verarbeitung wieder eingeführt.

[2.4.5] - 2025-10-07

Behoben

     * KeyGeneration: Hash über Key wird als KeyId benutzt
     * SET Erstellung:
          + Temporäres Entfernen des x5c aus dem privaten Schlüssel
          + Algorithm für den Fallback geändert

Aktualisierungen

     * Abhängigkeiten:
          + Microsoft.Extensions.Logging.Abstractions auf Version="9.0.9"
          + Polly auf Version="8.6.4"
     * Test-Abhängigkeiten:
          + Microsoft.Extensions.Configuration.Json auf Version="9.0.9"
          + xunit.runner.visualstudio auf Version="3.1.5"
          + WireMock.Net auf Version="1.13.0"
          + Microsoft.Extensions.Logging.Console auf Version="9.0.9"

[2.4.2] - 2025-09-03

Behoben

     * Falsches JSON-Mapping für BundID-ReplyChannel führte zu einer leeren MailboxId

Aktualisierungen

     * Abhängigkeiten:
          + Polly auf Version="8.6.3"
     * Test-Abhängigkeiten:
          + WireMock.Net auf Version="1.12.0"

[2.4.1] - 2025-09-03

Behoben

     * Projektinterne Abhängigkeiten

[2.4.0] - 2025-08-21

Hinzugefügt

     * ZBP (Zentrales Bürgerpostfach) Unterstützung
     * XML-Validierung
     * XÖV-Validierung

Aktualisierungen

     * Abhängigkeiten:
          + Autofac auf Version="8.4.0"
          + Microsoft.IdentityModel.Tokens auf Version="9.0.8"
          + System.IdentityModel.Tokens.Jwt auf Version="11.4.0"
     * Test-Abhängigkeiten:
          + NUnit.Analyzers auf Version="4.4.0"
          + Microsoft.Extensions.Configuration.Json auf Version="9.0.8"
          + WireMock.Net auf Version="1.10.0"
          + Microsoft.Extensions.Logging.Console auf Version="8.4.0"
          + xunit.runner.visualstudio auf Version="3.1.4"
          + Microsoft.Extensions.Logging.Console auf Version="9.0.8"

[2.3.9] - 2025-08-21

Hinzugefügt

     * ZBP (Zentrales Bürgerpostfach) Unterstützung
     * XML-Validierung
     * XÖV-Validierung

Aktualisierungen

     * Abhängigkeiten:
          + Autofac auf Version="8.4.0"
          + Microsoft.IdentityModel.Tokens auf Version="9.0.8"
          + System.IdentityModel.Tokens.Jwt auf Version="11.4.0"
     * Test-Abhängigkeiten:
          + NUnit.Analyzers auf Version="4.4.0"
          + Microsoft.Extensions.Configuration.Json auf Version="9.0.8"
          + WireMock.Net auf Version="1.10.0"
          + Microsoft.Extensions.Logging.Console auf Version="8.4.0"
          + xunit.runner.visualstudio auf Version="3.1.4"
          + Microsoft.Extensions.Logging.Console auf Version="9.0.8"

[2.3.8] - 2025-06-23

Hinzugefügt

     * Support für Destination Signature Issuer Validierung.
     * Support für Destination API Nutzung.
     * Support für Dynamische Attachment Limits

Behoben

     * Testcontainer-Einrichtung an die neue Version von ZSD angepasst und image tag eingefroren.

Updates

     * Abhängigkeiten:
          + Polly auf Version 8.6.0.
     * Test-Abhängigkeiten:
          + WireMock.Net auf Version 1.8.11.
          + NUnit.Analyzers auf Version 4.9.1.
          + Testcontainers.Minio und Testcontainers.PostgreSQL auf Version 4.6.0.

[2.3.7] - 2025-06-10

Hinzugefügt

     * Support für Region in der Submission
     * Support für SenderReference in der Submission

Behoben

     * Fehler beim Senden eines SET mit MissingAttachment

Aktualisierungen

     * Abhängigkeiten:
          + NJsonSchema auf Version="11.3.2"
          + System.IdentityModel.Tokens.Jwt auf Version="8.12.0"
          + Autofac auf Version="8.3.0"
          + Microsoft.IdentityModel.Tokens auf Version="9.0.5"
     * Test-Abhängigkeiten:
          + NUnit.Analyzers auf Version="4.8.1"
          + xunit.runner.visualstudio auf Version="3.1.1"
          + WireMock.Net auf Version="1.8.9"
          + Microsoft.NET.Test.Sdk auf Version="9.0.5"
          + NUnit3TestAdapter auf Version="5.0.0"
          + Microsoft.Extensions.Logging.Console auf Version="8.3.0"

[2.3.6] - 2025-04-08

Hinzugefügt

     * Funktionalität zum Erstellen der TestJWKs und zum Konvertieren der VPKIs in JWKs

Aktualisierungen

     * Abhängigkeiten:
          + Autofac auf Version="8.2.1"
          + Microsoft.IdentityModel.JsonWebTokens auf Version="8.7.0"
          + Microsoft.IdentityModel.Tokens auf Version="8.7.0"
          + NJsonSchema auf Version="11.2.0"
          + System.IdentityModel.Tokens.Jwt auf Version="8.10.0"
          + CommunityToolkit.Mvvm auf Version="8.4.0"
          + Microsoft.Extensions.DependencyInjection.Abstractions auf Version="9.0.3"
          + Microsoft.Extensions.Logging.Abstractions auf Version="9.0.3"
          + Microsoft.IdentityModel.Tokens auf Version="8.7.0"
     * Test-Abhängigkeiten:
          + NUnit.Analyzers auf Version="4.7.0"
          + Autofac auf Version="8.2.1"
          + Microsoft.Extensions.Configuration.Json auf Version="9.0.5"
          + Microsoft.Extensions.Logging.Console auf Version="9.0.5"
          + Microsoft.IdentityModel.JsonWebTokens auf Version="8.7.0"
          + WireMock.Net auf Version="1.8.6"

[2.3.5] - 2025-03-25

Behoben

     * Fehler im Authentication Token behoben

Aktualisierungen

     * Abhängigkeiten:
          + Microsoft.AspNetCore.Http.Abstractions auf Version="2.3.0"
          + Microsoft.Extensions.Logging.Abstractions auf Version="9.0.3"
          + Microsoft.IdentityModel.JsonWebTokens auf Version="8.6.1"
          + Microsoft.IdentityModel.Tokens auf Version="8.6.1"
          + Polly auf Version="8.5.2"
          + System.IdentityModel.Tokens.Jwt auf Version="8.6.1"
     * Test-Abhängigkeiten:
          + coverlet.collector" auf Version="6.0.4"
          + Microsoft.AspNetCore.Http" auf Version="2.3.0"
          + Microsoft.AspNetCore.Http.Abstractions" auf Version="2.3.0"
          + Microsoft.Extensions.Configuration.Json" auf Version="9.0.3"
          + Microsoft.Extensions.Logging" auf Version="9.0.3"
          + Microsoft.Extensions.Logging.Console" auf Version="9.0.3"
          + Microsoft.IdentityModel.JsonWebTokens" auf Version="8.6.1"
          + Microsoft.NET.Test.Sdk" auf Version="17.13.0"
          + WireMock.Net" auf Version="1.7.4"
          + xunit.runner.visualstudio" auf Version="3.0.2"

[2.3.4] - 2025-03-07

Behoben

     * Unterstützung für .NET 7, .NET 8 und .NET 9 hinzugefügt
     * Fehler bei großen Fachdaten, die Chunking auslösen, behoben

Hinzugefügt

     * Environment URLs aktualisiert

[2.3.3] - 2025-01-09

Behoben

     * Optimierung der AuthenticationTag-Extraktion
     * Anpassung der Resilienz durch Umstellung auf das Polly-Paket
     * Logging von WARN anstelle von ERROR bei fehlendem Schema-Cache
     * Manueller Aufruf des Garbage Collectors, um Speicher nach Verschlüsselung und Entschlüsselung freizugeben

Aktualisierungen

     * Abhängigkeiten:
          + Autofac auf Version="8.2.0"
     * Test-Abhängigkeiten:
          + xUnit auf Version="2.9.3"
          + NUnit auf Version="4.3.2"

[2.3.2] - 2024-11-27

Aktualisierungen

     * Abhängigkeiten:
          + jose-jwt heruntergestuft auf Version="4.1.0"

Behoben

     * Problem mit nicht erweiterbarem MemoryStream

[2.3.1] - 2024-11-22

Entfernt

     * destinationParameter als Obsolet markiert (planning#2500)

Aktualisierungen

     * Abhängigkeiten:
          + Microsoft.Extensions.Logging auf Version="9.0.0"
          + Microsoft.Extensions.Logging.Abstractions auf Version="9.0.0"
          + Microsoft.IdentityModel.JsonWebTokens auf Version="8.2.1"
          + Microsoft.IdentityModel.Tokens auf Version="8.2.1"
          + NJsonSchema auf Version="11.1.0"
          + System.IdentityModel.Tokens.Jwt auf Version="8.2.1"
          + Microsoft.NET.Test.Sdk auf Version="17.12.0"
          + Microsoft.Extensions.Configuration.Json auf Version="9.0.0"
          + NUnit.Analyzers auf Version="4.4.0"

[2.3.0] - 2024-11-18

Hinzugefügt

     * BundID Reply Channel (planning#2224)
     * Retry-Logik für spezifische Servercodes (planning#2304)
     * Verarbeitung großer Einreichungsdaten (planning#2252)
     * Chunking für große Anhänge (planning#1522)

Behoben

     * Problem mit dem Schema-Verzeichnis (planning#2360)

Aktualisierungen

     * NuGet-Abhängigkeiten

[2.1.0] - 2024-10-02

Hinzugefügt

     * Überprüfung der Antwortkanäle aus dem Dienst statt aus der Destination (planning#1636)
     * Festlegen des Content-Typs für verschlüsselte Daten (planning#1244)

Geändert

     * Quelle für Antwortkanal geändert (planning#1636)
     * Aktualisierung des SET-Schemas auf Version 1.2.2 (planning#1950)
     * intern: Proxy-Handling für Abhängigkeiten (planning#1769)
     * intern: Neuorganisation der PCA-Zertifikatsquellen (planning#851)

[2.0.0] - 2024-02-20

Hinzugefügt

     * Bidirektionale Kommunikation mit FIT-Connect-Reply-Channel (planning#654):
          + Neue Einreichungen für bestehende Fälle senden
          + Antworten auf Einreichungen als Abonnent senden
          + Antworten als Sender empfangen
          + Verfügbare Fälle und Antworten auflisten
          + Aktuellen Status für Antworten abrufen
          + Neuer FIT-Connect-Reply-Channel, der den öffentlichen Verschlüsselungsschlüssel für Antworten und eine Liste der Verarbeitungsstandards
            überträgt
          + ReplyChannelKeyGenerator zur Erstellung temporärer Schlüssel für die Verschlüsselung und Entschlüsselung von Antworten
     * Getter für ServiceType in ReceivedSubmission (planning#1072)
     * Festlegen des ApplicationDate bei einer SendableSubmission über setApplicationDate(localDate) (planning#1243)
          + Rückdatierung des Antragsdatums
          + Unabhängig vom technischen Einreichungs-Zeitstempel
     * Neue DTOs für Einreichungs- und Antwort-Benachrichtigungen
     * Zugriff auf den Einreichungs-Zeitstempel (iat) im technischen SUBMITTED-Event über getSubmittedAt (planning#654)
     * Einfachere Möglichkeit, Proxy-Einstellungen für HTTP-Clients zu setzen

Geändert

     * Umbenennung von API-Methoden (planning#654):
          + Statische Methoden von ClientFactory zu ClientFactory.Create...(config)
          + Statische Methoden von ReplyChannel zu ReplyChannel.Of...()

Veraltet

     * Methoden fromString für Anhänge werden in SDK-Version 3.0.0 als veraltet markiert, da sie bei falscher Nutzung zu Problemen mit der
       Zeichencodierung führen können (planning#1308).

Behoben

     * NullPointerException (planning#1405, planning#1422, planning#1418)

Abhängigkeitsaktualisierungen

     * Autofac 8.0.0
     * IdentityModel 6.2.0
     * Microsoft.Extensions.Logging.Abstractions 8.0.0
     * Microsoft.IdentityModel.JsonWebTokens 7.3.1
     * Microsoft.IdentityModel.Tokens 7.3.1
     * NJsonSchema 11.0.0
     * System.IdentityModel.Tokens.Jwt 7.3.1

[1.0.0] - 2023-08-01

   Erste Version veröffentlicht

[1.0.0] - 2023-05-02

     * Exception für invalide JSON-Keys hinzugefügt

[1.0.0-rc.1] - 2023-04-28

   Release candidate 1
```
