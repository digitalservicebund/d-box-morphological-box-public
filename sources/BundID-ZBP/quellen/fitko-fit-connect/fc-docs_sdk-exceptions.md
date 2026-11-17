# Exceptions

- Quelle: <https://docs.fitko.de/fit-connect/docs/sdks/referenz/exceptions>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Exceptions

   Eine Exception heißt: Bei Ihnen oder auf dem Weg zum Server stimmt etwas nicht, etwa die Konfiguration, ein Schlüssel oder die Verbindung. Ob ein
   empfangener Antrag fehlerhaft ist, meldet das SDK dagegen als Befund im Prüfbericht; solche Anträge lehnen Sie ab. Einzige Ausnahme: Ein
   Ereignisprotokoll, das die Prüfung nicht besteht, kommt als FitConnectEventLogException und wird trotzdem abgelehnt.

Exception nachschlagen

   Suchen Sie nach dem Namen aus dem Stacktrace, einem Statuscode oder einem Stichwort. Zu jeder Exception stehen die Schritte, in denen sie auftritt, und
   was Sie dann tun.
   Exception suchen ____________________
     * (BUTTON) FitConnectConfigurationException
     * (BUTTON) FitConnectAuthenticationException
     * (BUTTON) FitConnectAuthorizationException
     * (BUTTON) RestApiException
     * (BUTTON) IllegalStateException
     * (BUTTON) FitConnectRoutingException
     * (BUTTON) FitConnectSenderException
     * (BUTTON) FitConnectSchemaException
     * (BUTTON) FitConnectValidationException
     * (BUTTON) VirusScanUnavailableException
     * (BUTTON) FitConnectAttachmentException
     * (BUTTON) FitConnectEventLogException
     * (BUTTON) FitConnectEventParsingException
     * (BUTTON) FitConnectCryptoException
     * (BUTTON) Befund im Prüfbericht
     * (BUTTON) FitConnectMappingException
     * (BUTTON) EventCreationException
     * (BUTTON) FitConnectReplyException
     * (BUTTON) ZBPException
     * (BUTTON) FitConnectZbpException
     * (BUTTON) PostfachException
     * (BUTTON) FitConnectStorageException

FitConnectConfigurationException

   Java.NET2 Situationen

   Schritt 1 · Einrichten

Konfiguration unvollständig oder fehlerhaft

   YAML bzw. appsettings, Credentials, Umgebung, Schlüsseldatei
     * Java
     * .NETauch: DestinationId unbekannt oder passt nicht zur Rolle

   Konfiguration korrigieren

   Startfehler. Konfiguration korrigieren, nicht wiederholen.

   Schritt 7 · Entschlüsseln

Kein Entschlüsselungsschlüssel konfiguriert

   Rolle ohne DestinationKeys bzw. DecryptionKeys
     * .NET

   Konfiguration korrigieren

   Schlüssel des Zustellpunkts in der Konfiguration hinterlegen.

FitConnectAuthenticationException

   Java1 Situation

   Schritt 2 · Anmelden

Token-Abruf abgelehnt

   Client-ID oder Secret falsch, Scope fehlt
     * JavaHTTP 400/401 vom Token-Endpunkt

   Konfiguration korrigieren

   Client-ID, Secret und Scopes im Self-Service-Portal der richtigen Umgebung prüfen.

FitConnectAuthorizationException

   Java.NET4 Situationen

   Schritt 2 · Anmelden

Token-Abruf abgelehnt

   Client-ID oder Secret falsch, Scope fehlt
     * .NET

   Konfiguration korrigieren

   Client-ID, Secret und Scopes im Self-Service-Portal der richtigen Umgebung prüfen.

   Schritt 2 · Anmelden

API verweigert den Zugriff

   HTTP 401 oder 403
     * .NET

   Konfiguration korrigieren

   Prüfen, ob der Client zum Zustellpunkt gehört und die Scopes stimmen.

   Schritt 7 · Entschlüsseln

Kein Entschlüsselungsschlüssel konfiguriert

   Rolle ohne DestinationKeys bzw. DecryptionKeys
     * Java

   Konfiguration korrigieren

   Schlüssel des Zustellpunkts in der Konfiguration hinterlegen.

   Schritt 9 · Antworten

Signaturschlüssel fehlt

   beim Annehmen oder Ablehnen
     * Java

   Konfiguration korrigieren

   Signaturschlüssel des Zustellpunkts hinterlegen.

RestApiException

   Java.NET3 Situationen

   Schritt 2 · Anmelden

API verweigert den Zugriff

   HTTP 401 oder 403
     * Javamit Statuscode

   Konfiguration korrigieren

   Prüfen, ob der Client zum Zustellpunkt gehört und die Scopes stimmen.

   Schritt 6 · Ereignisprotokoll

Sperrstatus eines Zertifikats unbekannt

   OCSP- oder CRL-Dienst nicht erreichbar
     * .NET

   Später erneut

   Später erneut abholen. Nicht ablehnen, der Antrag ist nicht fehlerhaft.

   Überall

Server antwortet mit 408, 429, 500, 502, 503 oder 504

   nach allen automatischen Wiederholungen
     * JavaStatuscode, Methode, URL
     * .NETwiederholt automatisch nur sichere Anfragen wie GET

   Wiederholen

   Später wiederholen. Nach einem abgebrochenen Senden erst den Status abfragen.

IllegalStateException

   Java3 Situationen

   Schritt 3 · Empfänger finden

Kein Zustellpunkt gefunden

   Leistung oder Gebiet ohne Treffer
     * Java

   Aufruf korrigieren

   Leistungsschlüssel und Gebiet prüfen.

   Schritt 3 · Empfänger finden

Mehrere Zustellpunkte gefunden

   Empfänger nicht eindeutig
     * Java

   Aufruf korrigieren

   allForService(…) bzw. AllForService(…) nutzen und einen Treffer auswählen.

   Schritt 4 · Antrag bauen

Vorgang widersprüchlich angegeben

   Leistung mit openingCase, Prozessnachricht ohne Vorgang
     * Javaaus build()

   Aufruf korrigieren

   An eine Leistung ohne openingCase senden, eine Prozessnachricht immer einem Vorgang zuordnen.

FitConnectRoutingException

   .NET3 Situationen

   Schritt 3 · Empfänger finden

Kein Zustellpunkt gefunden

   Leistung oder Gebiet ohne Treffer
     * .NET

   Aufruf korrigieren

   Leistungsschlüssel und Gebiet prüfen.

   Schritt 3 · Empfänger finden

Mehrere Zustellpunkte gefunden

   Empfänger nicht eindeutig
     * .NET

   Aufruf korrigieren

   allForService(…) bzw. AllForService(…) nutzen und einen Treffer auswählen.

   Schritt 3 · Empfänger finden

Gebietsfilter ungültig

   leer oder mehrere Filter zugleich
     * .NET

   Aufruf korrigieren

   Genau einen Filter übergeben: AGS, ARS oder AreaId.

FitConnectSenderException

   Java.NET5 Situationen

   Schritt 4 · Antrag bauen

Vorgang widersprüchlich angegeben

   Leistung mit openingCase, Prozessnachricht ohne Vorgang
     * .NET

   Aufruf korrigieren

   An eine Leistung ohne openingCase senden, eine Prozessnachricht immer einem Vorgang zuordnen.

   Schritt 5 · Senden

Keine gemeinsame Metadaten-Version

   Zustellpunkt bietet nur 1.x oder nur unbekannte Versionen
     * Java

   Melden

   Betreiber des Zustellpunkts ansprechen oder SDK aktualisieren. Die Meldung nennt beide Seiten.

   Schritt 5 · Senden

Antrag besteht die Prüfung vor dem Versand nicht

   Fachdaten gegen Schema, Anhang-Limits, Rückkanal nicht freigeschaltet
     * Java
     * .NET

   Aufruf korrigieren

   Daten, Anhänge oder Rückkanal korrigieren. Die Meldung nennt die Ursache.

   Schritt 5 · Senden

Schlüssel des Zustellpunkts unbrauchbar

   nicht ladbar oder ungültig
     * Java
     * .NET

   Melden

   Betreiber des Zustellpunkts informieren.

   Schritt 5 · Senden

Virenscanner findet Schadcode im Anhang

   Prüfung beim Sender
     * Java

   Aufruf korrigieren

   Anhang entfernen oder ersetzen.

FitConnectSchemaException

   Java.NET2 Situationen

   Schritt 4 · Antrag bauen

Schema nicht auflösbar

   unbekannte oder nicht unterstützte Schema-URI
     * Java
     * .NET

   Aufruf korrigieren

   Schema-URI prüfen oder ein eigenes Schema registrieren.

   Schritt 5 · Senden

Keine gemeinsame Metadaten-Version

   Zustellpunkt bietet nur 1.x oder nur unbekannte Versionen
     * .NET

   Melden

   Betreiber des Zustellpunkts ansprechen oder SDK aktualisieren. Die Meldung nennt beide Seiten.

FitConnectValidationException

   Java.NET4 Situationen

   Schritt 5 · Senden

Virenscanner findet Schadcode im Anhang

   Prüfung beim Sender
     * .NET

   Aufruf korrigieren

   Anhang entfernen oder ersetzen.

   Schritt 6 · Ereignisprotokoll

Sperrstatus eines Zertifikats unbekannt

   OCSP- oder CRL-Dienst nicht erreichbar
     * Java

   Später erneut

   Später erneut abholen. Nicht ablehnen, der Antrag ist nicht fehlerhaft.

   Schritt 7 · Entschlüsseln

Antrag oder Anhang lässt sich nicht entschlüsseln

   kein Schlüssel zur kid, Inhalt beschädigt
     * .NETmit Problems

   Konfiguration korrigieren

   Erst die eigenen Schlüssel prüfen und alte Schlüssel nach einem Rollover behalten. Liegt es am Antrag, ablehnen; .NET liefert dafür e.Problems.

   Schritt 7 · Entschlüsseln

Anhang fehlt

   in den Metadaten angekündigt, aber nicht hochgeladen
     * .NETmit Problems

   Ablehnen

   Ablehnen. .NET: RejectSubmission(listed, e.Problems).

VirusScanUnavailableException

   .NET2 Situationen

   Schritt 5 · Senden

Virenscanner nicht erreichbar

   Scanner aus, Timeout
     * .NETnur mit FailOnScanError

   Wiederholen

   Scanner reparieren, dann erneut senden. Sagt nichts über den Inhalt.

   Schritt 8 · Prüfen

Anhang nicht auf Viren geprüft

   Scanner ausgefallen
     * .NETnur mit FailOnScanError

   Später erneut

   Scanner reparieren und erneut prüfen. Nicht ablehnen.

FitConnectAttachmentException

   Java.NET1 Situation

   Schritt 5 · Senden

Anhang-Upload scheitert

   Upload, Chunking
     * Java
     * .NET

   Wiederholen

   Wiederholen, Größenlimits des Zustellpunkts prüfen.

FitConnectEventLogException

   Java.NET1 Situation

   Schritt 6 · Ereignisprotokoll

Ereignisprotokoll besteht die Prüfung nicht

   Signatur ungültig, Reihenfolge falsch
     * Javareport() mit Stufe EVENT_LOG
     * .NETReport und Problems

   Ablehnen

   Ablehnen: reject(waiting, e.report().asProblems()) bzw. RejectSubmission(listed, e.Problems).

FitConnectEventParsingException

   Java.NET1 Situation

   Schritt 6 · Ereignisprotokoll

Eintrag im Protokoll nicht lesbar

   Event-Token beschädigt
     * Java
     * .NET

   Melden

   Mit Vorgangs-ID an den FIT-Connect-Support melden.

FitConnectCryptoException

   Java2 Situationen

   Schritt 7 · Entschlüsseln

Antrag oder Anhang lässt sich nicht entschlüsseln

   kein Schlüssel zur kid, Inhalt beschädigt
     * Java

   Konfiguration korrigieren

   Erst die eigenen Schlüssel prüfen und alte Schlüssel nach einem Rollover behalten. Liegt es am Antrag, ablehnen; .NET liefert dafür e.Problems.

   Schritt 9 · Antworten

Rückkanalschlüssel fehlt

   Onlinedienst holt eine Antwort ab
     * Java

   Ablehnen

   Antwort ablehnen: reject(waiting, …) bzw. RejectReply(listed, …).

Befund im Prüfbericht

   Java.NET5 Situationen

   Schritt 7 · Entschlüsseln

Anhang fehlt

   in den Metadaten angekündigt, aber nicht hochgeladen
     * JavaBefund ATTACHMENTS

   Ablehnen

   Ablehnen. .NET: RejectSubmission(listed, e.Problems).

   Schritt 8 · Prüfen

Metadaten fehlerhaft

   Schema, Leistung, Rückkanal
     * JavaBefund METADATA
     * .NETBefund Metadata

   Ablehnen

   acceptable() ist false. Mit den Befunden des Berichts ablehnen.

   Schritt 8 · Prüfen

Fachdaten fehlerhaft

   Hash, JSON-Schema, XML
     * JavaBefund DATAXML gegen das XSD
     * .NETBefund DataXML nur auf Wohlgeformtheit

   Ablehnen

   Ablehnen. Keines der SDKs lehnt automatisch ab.

   Schritt 8 · Prüfen

Anhang fehlerhaft oder infiziert

   Hash, Auth-Tag, Virenfund
     * JavaBefund ATTACHMENTS
     * .NETBefund Attachments

   Ablehnen

   Ablehnen.

   Schritt 8 · Prüfen

Anhang nicht auf Viren geprüft

   Scanner ausgefallen
     * JavaWarnung im BerichtAntrag bleibt annehmbar

   Später erneut

   Scanner reparieren und erneut prüfen. Nicht ablehnen.

FitConnectMappingException

   .NET1 Situation

   Schritt 8 · Prüfen

Metadaten nennen keinen Datentyp oder kein Schema

   beim Lesen der Fachdaten
     * .NETaus GetDataMimeType() bzw. GetDataSchemaUri()

   Aufruf korrigieren

   Vor dem Zugriff prüfen, ob die Metadaten Fachdaten beschreiben (Metadata.ContentStructure.Data).

EventCreationException

   .NET1 Situation

   Schritt 9 · Antworten

Signaturschlüssel fehlt

   beim Annehmen oder Ablehnen
     * .NET

   Konfiguration korrigieren

   Signaturschlüssel des Zustellpunkts hinterlegen.

FitConnectReplyException

   Java.NET2 Situationen

   Schritt 9 · Antworten

Vorgang hat keinen FIT-Connect-Rückkanal

   Antwort nicht möglich
     * Java
     * .NET

   Aufruf korrigieren

   Antwort auf anderem Weg zustellen. Der Onlinedienst muss einen ReplyChannel mitschicken.

   Schritt 9 · Antworten

Rückkanalschlüssel fehlt

   Onlinedienst holt eine Antwort ab
     * .NET

   Ablehnen

   Antwort ablehnen: reject(waiting, …) bzw. RejectReply(listed, …).

ZBPException

   Java1 Situation

   Zusatzmodule

Aufruf der ZBP-API scheitert

   Zentrales Bürgerpostfach
     * JavaHTTP-Details über getApiException()

   Wiederholen

   Statuscode prüfen: bei 5xx später wiederholen, bei 4xx Aufruf oder Zugangsdaten korrigieren.

FitConnectZbpException

   .NET1 Situation

   Zusatzmodule

Aufruf der ZBP-API scheitert

   Zentrales Bürgerpostfach
     * .NETHTTP-Details in der inneren RestApiException

   Wiederholen

   Statuscode prüfen: bei 5xx später wiederholen, bei 4xx Aufruf oder Zugangsdaten korrigieren.

PostfachException

   .NET1 Situation

   Zusatzmodule

Postfach- oder ELSTER-Transfer scheitert

   Postfach-Modul
     * .NETUnterklassen für Transport, Authentication, Validation und Business

   Aufruf korrigieren

   Nach Unterklasse unterscheiden: Transport wiederholen, Authentication die Zugangsdaten prüfen, Validation und Business den Aufruf korrigieren.

FitConnectStorageException

   Java.NET1 Situation

   Überall

Lokaler Anhangspeicher nicht nutzbar

   Pfad fehlt, keine Rechte
     * Java
     * .NET

   Konfiguration korrigieren

   attachmentStoragePath bzw. Attachments.BaseDirectory prüfen.

Fehler abfangen

     * Java
     * .NET (C#)

   Alle Exceptions sind unchecked. Die meisten erben von FitConnectException, drei nicht: FitConnectAuthenticationException (Token-Abruf abgelehnt),
   RestApiException (HTTP-Fehler ohne eigene Zuordnung, mit Statuscode, Methode und URL) und ZBPException aus dem ZBP-Modul (dev.fitko.fitconnect.zbp).
   Wer alle SDK-Fehler an einer Stelle behandeln will, fängt deshalb mehrere Typen:

   import dev.fitko.fitconnect.core.http.api.RestApiException;

   import dev.fitko.fitconnect.rest.client.services.auth.FitConnectAuthenticationException;
   import dev.fitko.fitconnect.sdk.exceptions.FitConnectException;
   try {
   SentSubmission sent = onlineService.send(antrag);
   } catch (FitConnectException
   | FitConnectAuthenticationException
   | RestApiException e) {
   log.error("Senden fehlgeschlagen", e);
   }

   Eine IllegalStateException aus build() oder der Empfängersuche zeigt einen Fehler im eigenen Code. Fangen Sie sie nicht ab, sondern korrigieren Sie den
   Aufruf.

   Alle Exceptions erben von FitConnectException (Fitko.FitConnect.Core.Exceptions). Ein catch (FitConnectException) erfasst also auch HTTP-Fehler, ZBP
   und Postfach.

   Die meisten Unterklassen liegen in Fitko.FitConnect.Client.Models.Exceptions, abweichend:
               Exception                              Namespace
   FitConnectCryptoException          Fitko.FitConnect.Core.Exceptions
   FitConnectValidationException      Fitko.FitConnect.Core.Validation
   FitConnectStorageException         Fitko.FitConnect.Core.Util
   RestApiException                   Fitko.FitConnect.Core.Http.Exceptions
   VirusScanUnavailableException      Fitko.FitConnect.VirusScanning.Exceptions
   FitConnectZbpException             Fitko.FitConnect.Zbp
   PostfachException und Unterklassen Fitko.FitConnect.Postfach.Exceptions

   RestApiException trägt StatusCode, Method, Url und ResponseBody; einordnen lässt sie sich mit IsNotFound(), IsClientError() und IsServerError().

Wiederholen

   Beide SDKs wiederholen HTTP-Anfragen bei 408, 429, 500, 502, 503 und 504 selbst, bis zu fünfmal mit wachsendem Abstand. Eine RestApiException mit
   diesen Codes sehen Sie erst, wenn alle Versuche ausgeschöpft sind.
                         Java                                .NET
   Wiederholt werden alle Anfragen nur sichere Anfragen wie GET; Senden und Ereignisse nicht
   Einstellen über   retryConfig   Http.Retry
   Nach einem Abbruch nicht blind neu senden

   Bricht die Verbindung nach dem Senden ab, wissen Sie nicht, ob der Antrag angekommen ist. Fragen Sie vor dem erneuten Senden den Zustand ab (Status
   verfolgen); im .NET-SDK können Sie außerdem mit WithSubmissionId(existingId) dieselbe Submission-ID erneut verwenden.

Weiter geht's

     * Konfiguration — Timeouts und Wiederholungen einstellen
     * Konzept: Der Prüfbericht — Fehler, die keine Exceptions sind
     * Anträge abholen — Exceptions und Befunde beim Empfang

   Stand dieser Seite: Java-SDK 4.0.0 · .NET-SDK 4.0.0
```
