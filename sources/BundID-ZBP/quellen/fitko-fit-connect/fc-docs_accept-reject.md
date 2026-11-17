# Empfangsbestätigung oder Zurückweisung

- Quelle: <https://docs.fitko.de/fit-connect/docs/sending/accept-reject>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Empfangsbestätigung oder Zurückweisung

   Nachdem die Einreichung versendet worden ist, kann der Status dieser abgefragt werden. Der Status ist für das sendende System relevant, da hierüber das
   empfangende System mitteilt, ob es die Einreichung technisch korrekt verarbeiten konnte. Bis zu einer Bestätigung über den Status accepted sollte das
   System sämtliche Inhalte einer Einreichung vorhalten. Dies sollte es tun, damit es im Falle eines Fehlers (aufgrund eines inkorrekten Schemas oder
   eines falschen Public Keys), den Fehler eventuell korrigieren kann und Informationen korrekt übermitteln kann, ohne den Nutzer (bspw. eine
   Antragssteller:in) zu einer erneuten Dateneingabe aufzufordern.

   Zudem stellt der Status in Form des signierten Security Event Tokens einen sicheren Nachweis der Übermittlung und des Empfangs dar, der gegenüber
   Dritten genutzt werden kann.
   Wichtig

   Die Authentizität und Integrität der aus dem Event Log abgerufenen Ereignisse SOLLTE bei jedem Abruf geprüft werden. Die notwendigen Prüfungen sind im
   Artikel Prüfung eines Security Event Token beschrieben.
   Hinweis

   Wie Ereignisse aus dem Ereignisprotokoll abgerufen werden können, ist im Artikel Event Log abfragen beschrieben.

Statusmodell

   Das Statusmodell einer Einreichung ist in der folgenden Grafik dargestellt und enthält fünf Zustände (in orange), die in der darunterliegenden Tabelle
   beschrieben sind. Die in blau und grün dargestellten Ereignisse im Abschnitt "Ereignisprotokoll" beschreiben.

   Eine Einreichung wird immer mit einer Empfangsbestätigung (accept-submission) akzeptiert oder mit einer Zurückweisung (reject-submission) abgelehnt.

Darstellung für Nutzer

   Hinweis

   Die Fehlermeldungen im Problem-Datensatz dürfen den Nutzern des Systems (z.B. Bürger:innen, die einen Online-Antrag nutzen) nicht angezeigt werden!

   Die Anzeige für die Nutzer (Bürger:innen) sollte sich daher auf drei Status beschränken:
     * Die Einreichung wurde abgeschickt/ist auf dem Zustellweg.
     * Die Einreichung wurde empfangen.
     * Die Einreichung konnte nicht verarbeitet werden.

Beim Versand der Einreichung

   Nach dem Versand von Einreichungen muss das sendende System den Nutzer darauf hinweisen, dass die Einreichung zwar versendet, jedoch noch nicht von der
   zuständigen Stelle empfangen wurde. Der Nutzer muss zu einem späteren Zeitpunkt prüfen, ob eine Empfangsbestätigung vorliegt.

Bei einer technischen Empfangsbestätigung

   Erhält das sendende System Kenntnis von der technischen Empfangsbestätigung, sollte es, sofern möglich, den Nutzer informieren. Die Empfangsbestätigung
   gibt keine Information über die weitere Prüfung der Einreichung durch den Adressaten. Bei der Formulierung sollte darauf geachtet werden, dass es sich
   um eine technische Eingangsbestätigung handelt und die fachliche Prüfung erst noch erfolgt. Eine mögliche Formulierung wäre "Ihre Einreichung ist bei
   der zuständigen Stelle eingegangen.", nicht korrekt wäre z.B. "Ihre Einreichung wurde von der zuständigen Stelle akzeptiert.".

Bei einer Rückweisung

   Erhält das sendende System Kenntnis von der Zurückweisung, sollte es, sofern möglich, den Nutzer informieren. Die Zurückweisung erfolgt aufgrund von
   technischen Problemen, die im Normalfall das sendende System vor dem Versand verhindern soll. Bei der Formulierung sollte darauf geachtet werden, dass
   der Nutzer darüber informiert wird, dass eine Klärung der technischen Fehler erfolgt und ggf. ein neuer Antrag mit seiner Beteiligung versendet werden
   muss. Ob der Nutzer den Antrag erneut eingeben muss, ergibt sich aus dem Fehler und wie/ob die Daten beim sendenden System noch vorgehalten werden.

Verarbeitung einer Empfangsbestätigung

   Nachdem das empfangende System den Empfang bestätigt hat (accept-submission), informiert FIT-Connect das sendende System per Callback, sofern ein
   Callback hinterlegt wurde. Sollte der Callback fehlschlagen, wiederholt FIT-Connect diesen, bis er erfolgreich zugestellt werden konnte.

   Auch eine Empfangsbestätigung kann Probleme enthalten. In diesem Fall waren diese nicht so gravieren, dass das empfangende System die Verarbeitung
   abbrechen musste. Sie sollten protokolliert und von einem Systemadministrator überprüft werden. Hieraus können sich Anpassungsbedarfe in der Software
   ergeben, wenn bspw. Fehler bzgl. der Nutzung veralteter Schemata oder deren inkorrekten Umsetzung ergeben.

   Verarbeitung einer Empfangsbestätigung

Verarbeitung einer Zurückweisung

   Nachdem das empfangende System die Einreichung zurückgewiesen hat (reject-submission), informiert FIT-Connect das sendende System per Callback, sofern
   ein Callback hinterlegt wurde. Sollte der Callback fehlschlagen, wiederholt FIT-Connect diesen, bis er erfolgreich zugestellt werden konnte.

   Eine Zurückweisung erfolgt aufgrund von technischen Problemen, die im Normalfall das sendende System vor dem Versand verhindern muss. Die Probleme aus
   der Zurückweisung müssen protokolliert und zeitnah von einem Systemadministrator überprüft werden.

   Verarbeitung einer Zurückweisung

Aufbau einer Zurückweisung

   Eine Zurückweisung enthält immer eine Liste von Problemen (problems).

   Eine Rückweisung könnte wie folgt aussehen:

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.0.0/set-payload.schema.json",
   "jti": "4ac47caa-bce1-435a-b04f-3322b224b32e",
   "iss": "40847c29-06aa-40e2-bf28-c29884c694c4",
   "iat": 1622796532,
   "sub": "submission:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/reject-submission": {
   "problems": [
   {
   "type": "https://schema.fitko.de/fit-connect/events/problems/an-example",
   "title": "Nur ein Beispiel",
   "detail": "Es darf kein Beispiel verwendet werden.",
   "instance": "metadata"
   }
   ]
   }
   }
   }

   Die grundsätzliche Struktur der Events ist auf der Seite Ereignisse beschreiben. Das reject-submission-Event enthält eine Liste von Problemen. Diese
   enthält im obigen Beispiel ein Beispiel-Problem.

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/an-example",
   "title": "Nur ein Beispiel",
   "detail": "Es darf kein Beispiel verwendet werden.",
   "instance": "metadata"
   }

   Das Problem-Objekt enthält drei Informationen:
     * type gibt einen Fehlercode an - siehe Tabelle Fehlercodes
     * instance gibt den betroffenen Teil der Übertragung an. Mögliche Werte sind: submission, metadata, data, attachment: + UUID des Attachments oder
       other
     * title, detail und evtl. weitere vorhandene Werte geben Detailinformationen zum Fehler.

Fehlercodes (type)

   Fehlercode (type) Bedeutung Fehlerbehebung
   https://schema.fitko.de/fit-connect/events/problems/attachments-mismatch ↓ Die Liste der Attachments (UUIDs) stimmt nicht überein. Aufbau einer
   Einreichung
   https://schema.fitko.de/fit-connect/events/problems/encryption-issue ↓ Die Entschlüsselung ist fehlgeschlagen oder es gab ein Unstimmigkeit mit dem
   verwendeten Schlüssel. Aufbau einer Einreichung
   https://schema.fitko.de/fit-connect/events/problems/hash-mismatch ↓ Die Prüfsumme stimmt nicht. Integritätssicherung der Einreichung
   https://schema.fitko.de/fit-connect/events/problems/incorrect-authentication-tag ↓ Der Authentication-Tag stimmt nicht überein. Integritätssicherung
   der Einreichung
   https://schema.fitko.de/fit-connect/events/problems/invalid-content ↓ Die Inhaltliche Prüfung, z.B. auf Viren oder Prüfung des Dateiformats, hat einen
   Fehler ergeben. Sonstige Fehler
   https://schema.fitko.de/fit-connect/events/problems/invalid-event-log ↓ Das Ereignis-Protokoll ist ungültig. Fehler in FIT-Connect
   https://schema.fitko.de/fit-connect/events/problems/missing-attachment ↓ Eine Anlage fehlt. Fehler in FIT-Connect
   https://schema.fitko.de/fit-connect/events/problems/missing-authentication-tags ↓ Die Authentication-Tags fehlen im Ereignis-Protokoll. Fehler in
   FIT-Connect
   https://schema.fitko.de/fit-connect/events/problems/missing-data ↓ Der Fachdatensatz fehlt. Fehler in FIT-Connect
   https://schema.fitko.de/fit-connect/events/problems/missing-schema ↓ Es wurde kein Schema angegeben. Aufbau einer Einreichung
   https://schema.fitko.de/fit-connect/events/problems/schema-violation ↓ Der Datensatz entspricht nicht dem Schema. Aufbau einer Einreichung
   https://schema.fitko.de/fit-connect/events/problems/service-mismatch ↓ Es wurde eine falsche Verwaltungsleistung angegeben. Aufbau einer Einreichung
   https://schema.fitko.de/fit-connect/events/problems/syntax-violation ↓ Der Datensatz ist kein valides JSON bzw. XML. Aufbau einer Einreichung
   https://schema.fitko.de/fit-connect/events/problems/technical-error ↓ Bei der Verarbeitung im empfangenden System trat ein technischer Fehler auf.
   Sonstige Fehler
   https://schema.fitko.de/fit-connect/events/problems/timeout ↓ Die Einreichung wurde automatisch zurückgewiesen, da sie nicht rechtzeitig abgeholt
   wurde. Sonstige Fehler
   https://schema.fitko.de/fit-connect/events/problems/unsupported-reply-channel ↓ Der gewählte Rückkanal wird nicht unterstützt. Aufbau einer Einreichung
   https://schema.fitko.de/fit-connect/events/problems/unsupported-schema ↓ Das angegebene Schema wird nicht unterstützt. Aufbau einer Einreichung
   https://schema.fitko.de/fit-connect/events/problems/unsupported-service ↓ Die angegebene Verwaltungsleistung wird nicht unterstützt. Aufbau einer
   Einreichung

Mögliche automatische Korrekturen

https://schema.fitko.de/fit-connect/events/problems/encryption-issue

   Im Falle, dass ein inkorrekter Key genutzt wurde und die Einreichung unverschlüsselt vorliegt, verschlüsseln Sie diese erneut mit dem korrekten Key der
   Destination:
    1. Rufen Sie den betreffenden Zustellpunkt erneut ab: GET /v2/destinations/{destinationId}
    2. Laden Sie den darin genannten Schlüssel (encryptionKid) herunter: GET /v2/destinations/{destinationId}/keys/{keyId}
    3. Verschlüsseln Sie alle Teile mit dem neu ermittelten Schlüssel
    4. Versenden Sie die Einreichung

   Sollte die Einreichung nur noch in verschlüsselter Form vorliegen, melden sie beim Service Desk, um eine Problemlösung zu erreichen.

Klärungsmöglichkeiten bei Zurückweisung der Einreichung

   Falls es Probleme mit den übersendeten Daten hinsichtlich der Verarbeitbarkeit gekommen ist, sodass die empfangende Stelle eine Annahme der Daten
   verweigert, besteht die Möglichkeit über das Service Desk von FIT-Connect mit der empfangenden Stelle in Kontakt zu treten, um eventuell das Problem
   bilateral zu klären und zu lösen. Es besteht jedoch keine Garantie, dass die Daten auf Seite der empfangenden Stelle nach einem Reject aufbewahrt
   werden oder die auf der Seite des sendenden Systems eventuell noch verschlüsselt vorgehaltenen Daten sich für eine Korrektur eignen.

   Es wird daher dazu geraten, sich präzise an die Vorgaben der Destination hinsichtlich der zu versendenden Fachdatenformate und -standards zu halten und
   die Nutzung dieser Standards ausreichend zu testen.

Fehlerbeschreibungen

Aufbau einer Einreichung

Entschlüsselungs-Fehler

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/encryption-issue",
   "title": "Entschlüsselungs-Fehler",
   "detail": "Die Entschlüsselung des Metadatensatzes ist fehlgeschlagen.",
   "instance": "metadata"
   }

   Der Metadatensatz konnte nicht entschlüsselt werden.

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/encryption-issue",
   "title": "Entschlüsselungs-Fehler",
   "detail": "Der Fachdatensatz konnte nicht entschlüsselt werden.",
   "instance": "data"
   }

   Der Fachdatensatz konnte nicht entschlüsselt werden.

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/encryption-issue",
   "title": "Entschlüsselungs-Fehler",
   "detail": "Die Anlage {attachmentId} konnte nicht entschlüsselt werden.",
   "instance": "attachment:{attachmentId}"
   }

   Die Anlage konnte nicht entschlüsselt werden.

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/encryption-issue",
   "title": "Entschlüsselungs-Fehler",
   "detail": "Der Schlüssel {kid} ist nicht der zu diesem Zweck vorgesehene Schlüssel.",
   "instance": "metadata"
   }

   Der Metadatensatz konnte entschlüsselt werden, es wurde jedoch der falsche Schlüssel für die Verschlüsselung genutzt.

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/encryption-issue",
   "title": "Entschlüsselungs-Fehler",
   "detail": "Der Schlüssel {kid} ist nicht der zu diesem Zweck vorgesehene Schlüssel.",
   "instance": "data"
   }

   Der Fachdatensatz konnte entschlüsselt werden, es wurde jedoch der falsche Schlüssel für die Verschlüsselung genutzt.

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/encryption-issue",
   "title": "Entschlüsselungs-Fehler",
   "detail": "Der Schlüssel {kid} ist nicht der zu diesem Zweck vorgesehene Schlüssel.",
   "instance": "attachment:{attachmentId}"
   }

   Die Anlage konnte entschlüsselt werden, es wurde jedoch der falsche Schlüssel für die Verschlüsselung genutzt.

Syntax-Fehler

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/syntax-violation",
   "title": "Syntax-Fehler",
   "detail": "Der Metadatensatz ist kein valides JSON.",
   "instance": "metadata"
   }

   Der Metadatensatz ist kein valides JSON. Achten Sie darauf, die Syntax von JSON einzuhalten.

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/syntax-violation",
   "title": "Syntax-Fehler",
   "detail": "Der Fachdatensatz ist kein valides JSON.",
   "instance": "data"
   }

   Der Fachdatensatz ist kein valides JSON. Achten Sie darauf, die Syntax von JSON einzuhalten.

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/syntax-violation",
   "title": "Syntax-Fehler",
   "detail": "Der Fachdatensatz ist kein valides XML.",
   "instance": "data"
   }

   Der Fachdatensatz ist kein valides XML. Achten Sie darauf, die Syntax von XML einzuhalten.

    Schema-Referenz fehlt

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/missing-schema",
   "title": "Schema-Referenz fehlt",
   "detail": "Die Schema-Referenz fehlt im Metadatensatz.",
   "instance": "metadata"
   }

   Der Metadatensatz enthält keinen Eintrag $schema. Im Metadatensatz muss ein Eintrag $schema sein, der auf ein unter
   https://schema.fitko.de/fit-connect/metadata/ angebotenes Schema verweist.

    Metadatenschema nicht unterstützt

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/unsupported-schema",
   "title": "Metadatenschema nicht unterstützt",
   "detail": "Die angegebene Metadatenschema-URI ('$schema') ist keines der unterstützten Metadatenschemas.",
   "instance": "metadata"
   }

   Die angegebene Schema-URI ($schema) entspricht keinem gültigen Metadatenschema oder die verwendete Version des Metadatenschemas wird nicht unterstützt.
   Im Zustellpunkt wird eine Liste von unterstützten Metadatenschemas (metadataVersions) vorgegeben. Die im Metadatensatz angegebene Schema-URI ($schema)
   muss einer der angebotenen Versionen entsprechen.

   Wenn zum Beispiel die Version 1.0.0 im Zustellpunkt angeboten wird:

   {

   "destinationId": "13ad2349-975c-4167-bcd8-da606b4e1d84",
   "status": "active",
   "publicServices": [
   {
   "identifier": "urn:de:fim:leika:leistung:99107004018000",
   "submissionSchemas": [
   {
   "schemaUri": "https://schema.fim.fitko.net/immutable/schemas/S00000121V1.0_2024-01-22-1705941957653.schema.json",
   "mimeType": "application/json"
   }
   ],
   "regions": [
   "DE09"
   ]
   }
   ],
   "encryptionKid": "e4142167-7f03-4d4f-a8c9-c7ecc78f55f8",
   "metadataVersions": [
   "1.0.0"
   ]
   }

   Dann muss die Schema-URI https://schema.fitko.de/fit-connect/metadata/1.0.0/metadata.schema.json lauten:

   {

   "$schema": "https://schema.fitko.de/fit-connect/metadata/1.0.0/metadata.schema.json",
   "destinationId": "879ee109-a690-4db8-ab32-424284184d7d",
   "announcedAttachments": [
   "123ee109-a690-4db8-ab32-424284184d7d",
   "456ee109-a690-4db8-ab32-424284184d7d"
   ],
   "publicService": {
   "name": "Bauantrag",
   "identifier": "urn:de:fim:leika:leistung:99010003001006"
   },
   "callback": {
   "url": "https://my-onlineservice.example.org/callbacks/fit-connect",
   "secret": "insecure_unsafe_qHScgrg_kP-R31jHUwp3GkVkGJolvBchz65b74Lzue0"
   }
   }

    Schema-Fehler

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/schema-violation",
   "title": "Schema-Fehler",
   "detail": "Der Metadatensatz ist nicht schema-valide.",
   "instance": "metadata"
   }

   Der Metadatensatz ist nicht valide bezüglich des verwendeten Metadatenschemas ($schema). Prüfen Sie den Metadatensatz mit einem geeigneten Tool gegen
   das JSON-Schema.

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/schema-violation",
   "title": "Schema-Fehler",
   "detail": "Der Fachdatensatz ist nicht schema-valide.",
   "instance": "data"
   }

   Der Fachdatensatz ist nicht valide bezüglich des verwendeten Fachdatenschemas. Prüfen Sie den Metadatensatz mit einem geeigneten Tool gegen das JSON-
   bzw. XML-Schema.

    Verwaltungsleistung stimmt nicht überein

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/service-mismatch",
   "title": "Verwaltungsleistung stimmt nicht überein",
   "detail": "Die Verwaltungsleistung in Submission und Metadatensatz stimmen nicht überein.",
   "instance": "metadata"
   }

   Der Identifier des Metadatensatzes publicServiceType/identifier (deprecated seit Version 1.1.0 des Metadatenschemas) stimmt nicht mit dem in der
   Submission publicService/identifier überein. Geben Sie in der Submission unter publicService genau die gleichen Daten an, wie im Metadatensatz unter
   publicServiceType.

    Verwaltungsleistung nicht unterstützt

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/unsupported-service",
   "title": "Verwaltungsleistung nicht unterstützt",
   "detail": "Die angegebene Verwaltungsleistung wird nicht unterstützt.",
   "instance": "metadata"
   }

   Der Identifier der Verwaltungsleistung im Metadatensatz (publicServiceType/identifier) entspricht keinem der angebotenen Verwaltungsleistungen des
   Zustellpunktes (publicServices[]/identifier). Im Zustellpunkt werden ein oder mehrere Verwaltungsleistungen aufgelistet, die über diesen Zustellpunkt
   versendet werden können. Im Metadatensatz die Verwaltungsleistung benannt werden, die Grundlage der Einreichung ist. Diese muss in der Liste der
   angebotenen Verwaltungsleistungen stehen, ansonsten ist eine Einreichung bei diesem Zustellpunkt nicht möglich.

    Fachdatensatz fehlt

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/missing-data",
   "title": "Fachdatensatz fehlt",
   "detail": "Der Fachdatensatz fehlt.",
   "instance": "metadata"
   }

   Die Einreichung enthält keinen Fachdatensatz. Fügen Sie einen Fachdatensatz entsprechend den Vorgaben des Zustellpunktes
   (publicServices[]/submissionSchemas[]) bei. Dazu wählen Sie zuerst aus der Liste der Verwaltungsleistungen (publicServices) eine Leistung aus und
   anschließend ein Fachdatenschema aus der Liste darunter (submissionSchemas).

    Fachdatenschema nicht unterstützt

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/unsupported-schema",
   "title": "Fachdatenschema nicht unterstützt",
   "detail": "Das angegebene Fachdatenschema wird nicht unterstützt.",
   "instance": "metadata"
   }

   Das im Metadatensatz angegebene Fachdatenschema (contentStructure/data/schema) stimmt mit keinem der im ausgewählten Service angebotenen Schemas
   (submissionSchemas[]) überein. Wählen Sie aus der Liste der angebotenen Fachdatenschemas (submissionSchemas) ein Schema aus und geben Sie es im
   Metadatensatz (contentStructure/data/schema) an. Alle Angaben (mimeType und schemaUri) müssen exakt übernommen werden.

    Fehlerhafte Anlagen-Liste

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/attachments-mismatch",
   "title": "Fehlerhafte Anlagen-Liste",
   "detail": "Die Liste der Anlagen in Submission und Metadatensatz stimmt nicht überein.",
   "instance": "metadata"
   }

   Die Liste der Anlagen in der Submission (announcedAttachments) stimmt nicht mit den Anlagen im Metadatensatz (contentStructure/attachments) überein.

    Rückkanal nicht unterstützt

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/unsupported-reply-channel",
   "title": "Rückkanal nicht unterstützt",
   "detail": "Der gewählte Rückkanal wird nicht unterstützt.",
   "instance": "metadata"
   }

   Der gewählte Rückkanal im Metadatensatz (replyChannel) entspricht keinem der im Zustellpunkt angebotenen Kanäle (replyChannels). Es wird daher der
   postalische Weg als Fallback genutzt.

  Integritätssicherung der Einreichung

    Authentication-Tag ungültig

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/incorrect-authentication-tag",
   "title": "Authentication-Tag ungültig",
   "detail": "Das Authentication-Tag des Metadatensatzes ist ungültig.",
   "instance": "metadata"
   }

   Das Authentication-Tag des Metadatensatzes stimmt nicht mit dem im Submit-Submission-Event (authenticationTags/metadata) überein.

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/incorrect-authentication-tag",
   "title": "Authentication-Tag ungültig",
   "detail": "Das Authentication-Tag des Fachdatensatzes ist ungültig.",
   "instance": "data"
   }

   Das Authentication-Tag des Fachdatensatzes stimmt nicht mit dem im Submit-Submission-Event (authenticationTags/data) überein.

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/incorrect-authentication-tag",
   "title": "Authentication-Tag ungültig",
   "detail": "Das Authentication-Tag der Anlage {attachmentId} ist ungültig.",
   "instance": "attachment:{attachmentId}"
   }

   Das Authentication-Tag der Anlage stimmt nicht mit dem im Submit-Submission-Event (authenticationTags/attachment/{attachmentId}) überein.

    Hash-Fehler

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/hash-mismatch",
   "title": "Prüfsumme stimmt nicht",
   "detail": "Die Prüfsumme des Fachdatensatzes stimmt nicht.",
   "instance": "data"
   }

   Der Hash des Fachdatensatzes im Metadatensatz (contentStructure/data/hash) passt nicht zu den entschlüsselten Daten.

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/hash-mismatch",
   "title": "Prüfsumme stimmt nicht",
   "detail": "Der Hash der Anlage {attachmentId} stimmt nicht.",
   "instance": "attachment:{attachmentId}"
   }

   Der Hash der Anlage im Metadatensatz (contentStructure/attachments[]/hash) passt nicht zu den entschlüsselten Daten.

  Fehler in FIT-Connect

   Bei Fehlern, die sich auf die Konsistenz der Daten in FIT-Connect beziehen, wenden Sie sich bitte an den FIT-Connect Service Desk. Geben Sie hierbei,
   soweit vorhanden, die IDs des Vorgangs (destinationId, submissionId, caseId) sowie die verwendete Client-ID (nicht das Client-Secret!) an.

    Inkonsistentes Ereignis-Protokoll

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/invalid-event-log",
   "title": "Inkonsistentes Event-Log",
   "detail": "Das Event-Log ist inkonsistent.",
   "instance": "submission"
   }

   Die Prüfung des Ereignis-Protokolls hat Fehler ergeben.

    Fehlende Authentication-Tags

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/missing-authentication-tags",
   "title": "Fehlende Authentication-Tags",
   "detail": "Das Event 'submit-submission' enthält keine Authentication-Tags.",
   "instance": "submission"
   }

   Beim Absenden einer Einreichung protokolliert FIT-Connect die Authentication-Tags aller verschlüsselten Teile im submit-submission-Event.

    Fehlerhafte Anlagen-Liste

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/attachments-mismatch",
   "title": "Fehlerhafte Anlagen-Liste",
   "detail": "Die Liste der Anlagen in Submission und Event-Log stimmt nicht überein.",
   "instance": "submission"
   }

   Die UUIDs der Anlagen in der Submission müssen den Schlüsseln der Anlagen in den Authentication-Tags entsprechen.

    Anlage fehlt

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/missing-attachment",
   "title": "Anlage fehlt",
   "detail": "Die Anlage {attachmentId} konnte nicht geladen werden.",
   "instance": "attachment:{attachmentId}"
   }

   Die im Metadatensatz genannte Anlage kann nicht geladen werden.

  Sonstige Fehler

    Zeitablauf (Timeout)

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/timeout",
   "title": "Zeitablauf",
   "detail": "Die Einreichung wurde automatisch zurückgewiesen.",
   "instance": "submission"
   }

   Die Einreichung wurde automatisch zurückgewiesen, da sie nicht rechtzeitig abgeholt wurde. Sofern das empfangende System eine Einreichung nicht
   fristgerecht abholt oder zurückweist, erzeugt der Zustelldienst selbst eine Zurückweisung.

    Unzulässiger Inhalt

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/invalid-content",
   "title": "Unzulässiger Inhalt",
   "detail": "Der Inhalt der Anlage {attachmentId} ist nicht zulässig.",
   "instance": "attachment:{attachmentId}"
   }

   Die Inhaltliche Prüfung der Anlage hat einen Fehler ergeben. Mögliche Gründe sind:
     * Der Virenscanner hat die Anlage als auffällig markiert.
     * Das Dateiformat passt nicht, z.B. wenn eine Word-Datei geschickt wurde, obwohl nur PDF akzeptiert wird.
     * Die übermittelte Datei entspricht nicht dem angegebenen Dateiformat (contentStructure/attachments[]/mimeType).

    Technischer Fehler

   {

   "type": "https://schema.fitko.de/fit-connect/events/problems/technical-error",
   "title": "Technischer Fehler",
   "detail": "Bei der Verarbeitung im empfangenden System trat ein technischer Fehler auf.",
   "instance": "other"
   }

   Bei der Verarbeitung im empfangenden System trat ein technischer Fehler auf.
```
