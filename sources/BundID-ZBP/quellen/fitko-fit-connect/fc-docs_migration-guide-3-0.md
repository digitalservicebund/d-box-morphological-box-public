# Migration-Guide v3

- Quelle: <https://docs.fitko.de/fit-connect/docs/major-release/fit-connect_3_0/migration_guide_3_0>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Migration-Guide von API Version 2.x.x auf 3.0.0

   Mit dem Major Update von FIT-Connect stehen die Submission API, die Destination API und das Metadatenschema in Version 3.0.0 zur Verfügung. Anbindungen
   können ab sofort auf diese neuen Versionen umgestellt werden. Die 2.x.x-Versionen sind hierdurch abgekündigt und werden nur noch ein Jahr zur Verfügung
   stehen.
   Wichtig

   Anbindungen müssen innerhalb eines Jahres auf die neuen Versionen umgestellt werden, da bereits bestehende Integrationen nach Ablauf dieser Zeitspanne
   nicht mehr funktionieren werden.

   Die nachfolgende Dokumentation beschreibt die Migration von FIT-Connect Version 2.x.x auf 3.0.0.

Grundsätzliche Konzeptänderungen

   Ziel der API-Version 3 ist es, stärker eine prozessbasierte Kommunikation mit mehreren Teilnehmern innerhalb eines Vorgangs (case) zu ermöglichen. Dazu
   ist es notwendig, jeden Versand von Nachrichten immer nur von einer Destination zu einer Destination zu erlauben.

   Vergleich API v2 und v3

   Im Rahmen dieses Updates werden zwei Typen von Destinations eingeführt und ein dritter Destination-Typ vorbereitet:
     * Typ A (Verwaltungs-Zustellpunkt): Traditionelle Zustellpunkte öffentlicher Verwaltungsbehörden, die neben Verwaltungsleistungen jetzt auch
       Prozessnachrichten unterstützen und, wie gehabt, V-PKI-Zertifikate erfordern. Alle bisherigen Zustellpunkte sind also jetzt vom Typ A.
     * Typ C (Onlinedienst-Zustellpunkt): Zustellpunkte für Onlinedienste. Zu jedem Onlinedienst-Client wird automatisch eine C-Destination erzeugt und
       zugewiesen. Diese Destinations können nicht wie Destinations vom Typ A adressiert werden; nur Replies können an sie gesendet werden. Dennoch müssen
       Onlinedienste die ID ihrer C-Destination kennen, um sie beim Versand anzugeben. Die C-Destination wird damit zum eigentlichen Absender des Antrags.
       Mit welchem zugewiesenen Client Onlinedienste Submissions senden oder Replies empfangen, ist zweitrangig. Die C-Destination ermöglicht es
       Onlinediensten außerdem, eine Callback-URL zu konfigurieren und Kontaktdaten anzugeben.

   Geplant: Typ B – Wirtschafts-Zustellpunkt

   Der neue Typ B ist für Unternehmen vorgesehen. Er verwendet ausschließlich Prozessreferenzen, jedoch keine Verwaltungsleistungen. Typ B ist noch nicht
   verfügbar.

   Destinations vom Typ C sind in der API-Version 2 nicht sichtbar. Bei Einreichungen aus einer veralteten API-Version wird die C-Destination-ID
   automatisch inferiert und eingesetzt, solange nur eine einzige Destination mit dem Client verbunden ist. Eine Kommunikation zwischen Nutzern der
   API-Versionen 2 und 3 ist also problemlos möglich.

   Wie Sie die neuen Features zur prozessbasierten Kommunikation mit mehreren Parteien verwenden, erläutern wir auf der Seite Neu: Prozessbasierte
   Kommunikation. Hingegen beschreiben die folgenden Abschnitte, wie Sie bestehende Workflows und Integrationen auf Basis der neuen API-Version
   weiterbetreiben können.

Keine Unterschiede mehr bei den Clients (Zugangsdaten)

   Da nun auch Sender-Clients (Onlinedienste) mit einem Zustellpunkt verbunden sein müssen, wurde die Unterscheidung der bisherigen Client-Typen
   aufgelöst. Welche Rolle ein Client spielen kann, hängt also allein davon ab, mit welchen Zustellpunkten er verbunden ist.

   Anstelle von Sender-Clients (Onlinedienst) und Subscriber-Clients (Verwaltungssystem) gibt es nur noch allgemeine Zugangsdaten. Jeder Client verfügt
   über die Berechtigung zum Senden und zum Empfangen. Darüber hinaus kann ein Client dazu berechtigt werden, die ihm zugeordneten Zustellpunkte per
   Destination-API zu verwalten oder sogar neue, von ihm dann verwaltete Zustellpunkte per API anzulegen.

Änderungen, die jede bestehende Anbindung betreffen

fromDestinationId als Pflichtfeld

   In Version 3 ist das Feld fromDestinationId in allen Requests für Einreichungen, Antworten und Vorgänge verpflichtend. Es gibt an, von welcher
   Destination die jeweilige Aktion ausgeht. Nur Clients, denen die verwendete fromDestinationId zugeordnet ist, dürfen die entsprechenden Endpunkte
   nutzen. Für Onlinedienste bedeutet das konkret, dass sie im Self-Service-Portal die ID der zu ihrem Onlinedienst zugeordneten C-Destination
   nachschlagen und in ihre Konfiguration aufnehmen müssen.

Callbacks auf Destination-Ebene

   Case-Callbacks wurden in Version 3 vollständig entfernt. Die Endpunkte PUT /v3/cases/{caseId}/callback und DELETE /v3/cases/{caseId}/callback stehen
   nicht länger zur Verfügung. Onlinedienste konfigurieren ihre Callback-URLs künftig direkt einmalig an ihrer C-Destination. Ereignis-Callbacks werden an
   die Callback-URL der jeweiligen fromDestinationId (bei für Sender relevanten Ereignissen) bzw. toDestinationId (bei für Empfänger relevanten
   Ereignissen) zugestellt.

   Das callback-Objekt wurde vollständig aus Einreichungs-Requests und -Responses der Submission API entfernt. Betroffen sind Requests und Responses der
   Endpunkte POST /v3/submissions, PUT /v3/submissions/{id} und GET /v3/submissions/{id}.

Entfernung der bisherigen Limits-Endpunkte

   Die Endpunkte GET /v3/destinations/{destinationId}/limits und GET /v3/cases/{caseId}/limits wurden entfernt und durch den neuen Endpunkt GET
   /v3/limits/{fromDestinationId}/{toDestinationId} ersetzt.

   Der neue Endpunkt benötigt keine Authentifizierung, verwendet zwei Pfadparameter (Absender- und Empfänger-Destination) und liefert eine vereinfachte
   Struktur ohne Trennung in Submission- und Reply-Limits. Attachment-Limits werden nun pro Destination konfiguriert, da Sender in Version 3 stets über
   eine Destination-Identität verfügen. Die bisherigen client-basierten Limits entfallen.

Strenge LeiKa-Validierung

   In Version 3 werden Einreichungen abgelehnt, wenn der angegebene publicService.identifier nicht mit einer der Verwaltungsleistungen übereinstimmt, die
   an der Ziel-Destination konfiguriert sind. Diese Prüfung war in Version 1 und 2 nicht aktiv. Integrationen, die bisher nicht valide Leistungsschlüssel
   übertragen haben, müssen die übergebenen Werte korrigieren, bevor sie auf Version 3 migrieren.

Sortierung von GET /v3/submissions

   GET /v3/submissions gibt Einreichungen nun nach dem Zeitpunkt der letzten Statusänderung (stateChangedAt) sortiert zurück — statt wie bisher nach
   Destination gruppiert. Die älteste Einreichung steht somit an erster Stelle der Liste.

Neue optionale Filterparameter

   In Version 3 stehen die folgenden neuen Filterparameter zur Verfügung:
     * GET /v3/submissions: Filterbar nach publicService. Das Feld publicService wird entsprechend auch in der Response zurückgegeben.
     * GET /v3/replies: Filterbar nach destinationId.
     * GET /v3/cases/{caseId}/events: Filterbar nach submissionId, replyId und Eventtyp.

Metadaten

   Fachverfahren sollten das Metadatenschema in Version 3.0.0 technisch unterstützen und danach im Self-Service-Portal diese Unterstützung eintragen.
   Onlinedienste sollten bei entsprechender Indikation der Destination diese neue Version für ihre Einreichungen verwenden, da Metadatenschemata älterer
   Versionen ebenfalls als abgekündigt gelten.

   Darüber hinaus bietet das Metadatenschema 3.0.0 auch inhaltliche Erweiterungen und Verbesserungen.

   Das Metadatenschema ist bereits seit Version 2 modularisiert. Neu in Version 3.0.0 ist, dass Angaben zu Rückkanälen nicht mehr im Schema selbst,
   sondern als Dataset angegeben werden müssen.

Dataset reply-channel 1.0.0

   Rückkanal-Dataset 1.0.0

   Dieses Dataset ersetzt das bisherige replyChannel-Objekt auf der Wurzelebene des Metadatenschemas. Es können nun mehrere Rückkanäle angegeben und —
   zusätzlich zu den bekannten Kanaldaten — auch Routing-Daten für die prozessbasierte Kommunikation mitgeliefert werden.

   Der fitConnect Rückkanal wurde dahingehend modifiziert, dass anstelle der Property processStandards das Tupel aus supportedSchemas und destinationId
   tritt, um die zu verwendende Destination-ID und die unterstützten Schemas im Falle einer Antwort angeben zu können. Die im fitConnect Rückkanal
   angegebene destinationId muss der destinationId des Onlinedienstes entsprechen, da nur an diesen Antworten (replys) gesendet werden können.

   Das optionale Array messages kann dazu genutzt werden, um Prozessnachrichten auf Rückkanäle zu mappen.

Dataset process 1.0.0

   Prozess-Dataset 1.0.0

   Dieses neue Dataset dient dazu, Routing-Daten für die prozessbasierte Kommunikation bei Submissions mitzuliefern. Details hierzu finden Sie auf der
   Seite Neu: Prozessbasierte Kommunikation. Ein Anwendungsfall wäre beispielsweise, wenn es für eine Rolle in einem konkreten Vorgang mehrere
   Möglichkeiten gibt und eine dieser Möglichkeiten bereits als Teilnehmer ausgewählt wurde.

   Mit dem Dataset process kann die Information über diesen Teilnehmer an andere Teilnehmer weitergegeben werden. Dies ist erforderlich, da eine Routing-
   bzw. Such-Abfrage mehrere Treffer ergeben würde.

Routing

   Es wird keine Routing-API v 3 geben. Stattdessen wird v 2.0.0 auf v 2.1.0 aktualisiert. Hierbei kommt eine kleinere Anzahl von neuen Feldern hinzu. Die
   Bereitstellung in der Testumgebung erfolgt zu einem späteren Zeitpunkt, voraussichtlich Ende August. Eine aktualisierte Dokumentation wird ebenfalls zu
   diesem Zeitpunkt bereitgestellt.

Änderungen für Anbindungen mit dem Java SDK

   Platzhalter

   Dieser Abschnitt wird ergänzt, sobald das Java SDK mit Unterstützung für FIT-Connect 3.0.0 veröffentlicht wird.

   Weitere Informationen und Hinweise zur Integration finden Sie auf der Seite zum Java SDK.

Änderungen für Anbindungen mit dem .NET SDK

   Platzhalter

   Dieser Abschnitt wird ergänzt, sobald das .NET SDK mit Unterstützung für FIT-Connect 3.0.0 veröffentlicht wird.

   Weitere Informationen und technische Details finden Sie im Changelog des .NET SDK.

Änderungen für Anbindungen ohne SDK

Basis-URLs und Pfade

   Verwenden Sie dieselben Basis-URLs wie für Ihre Requests der API-Version v2 — mit dem Unterschied, dass das Pfad-Präfix nun /v3 lautet.

Weitere Änderungen für Nutzer der Destination-API

Verpflichtende Angabe von Destination-Typen

   Beim Anlegen einer Destination ist nun ein Pflichtfeld type anzugeben. Als mögliche Werte kommen A und C in Betracht (siehe oben). Da Destinations vom
   Typ C keine Verwaltungsleistungen oder öffentliche Schlüssel unterstützen, sind die Felder publicServices und encryptionKid keine Pflichtfelder mehr.
   Beachten Sie beim Abruf von Destinations, dass diese Felder bei C-Destinations auch fehlen.

Zeitstempel updatedAt

   updatedAt ist der Zeitstempel der letzten Änderung der Destination. Dieser wird sowohl in der Einzel-Ressource (GET /v3/destinations/{id}) als auch in
   der Listenansicht (GET /v3/destinations) zurückgegeben.

Authentifizierung beim Abruf von Destinations

   In Version 2 lieferte GET /v2/destinations/{id} der Destination-API öffentliche Destination-Daten auch ohne Authentifizierung zurück.

   Dieses Verhalten entfällt in Version 3: GET /v3/destinations/{id} gibt ausschließlich vollständige Destination-Daten an authentifizierte und
   autorisierte Clients zurück.

   Unauthentifizierte oder unautorisierte Zugriffe werden mit HTTP 403 abgewiesen.

   Die für den Versand von Submissions an eine Destination erforderlichen Daten können weiterhin ohne Authentifizierung über GET /v3/destinations/{id} der
   Submission-API abgerufen werden. Dort werden grundsätzlich nur die öffentlichen Destination-Daten zurückgegeben. Durch diese Maßnahme werden die beiden
   APIs stärker auf ihren jeweiligen Verwendungszweck ausgerichtet.

allowedSenders (Client-IDs) wird durch allowedSenderDestinations (Destination-IDs) abgelöst

   Wer bisher senderAccessRestricted.allowedSenders verwendet hat, um eingehende Einreichungen auf bestimmte Absender zu beschränken, muss diese
   Konfiguration auf das neue Feld senderAccessRestricted.allowedSenderDestinations migrieren.

   Was sich geändert hat:
     * Es gibt nun zwei parallele Listen: Die bestehende Client-Liste (allowedSenders) und die neue Destination-Liste (allowedSenderDestinations).
     * Eingehende Sendungen werden akzeptiert, wenn der Absender auf einer der beiden Listen steht.
     * Über Version 3 lässt sich allowedSenderDestinations vollständig lesen und schreiben (POST/PUT/PATCH).
     * Über Version 3 lässt sich allowedSenders nur noch lesen und vorhandene Einträge entfernen — das Hinzufügen neuer Client-IDs ist in Version 3 nicht
       länger möglich.
     * Das Feld allowedSenders ist in der API-Dokumentation als deprecated markiert.
     * Beide Listen werden in der API-Version V3 zurückgegeben.

   Was zu tun ist:

   Tragen Sie zukünftige Absenderbeschränkungen über allowedSenderDestinations ein, indem Sie die Destination-ID des sendenden Systems (nicht mehr die
   Client-ID) verwenden. Die Destination-ID muss ihr Kommunikationspartner mitteilen.

   In den meisten Fällen wird es sich dabei um die C-Destination eines Onlinedienstes handeln. Es ist aber durchaus möglich, dass ein Verwaltungssystem
   mit einer A-Destination als Sender auftritt.

   Bestehende Einträge in allowedSenders bleiben bis zu einem gesondert kommunizierten Stichtag gültig. Sie sollten die Einträge jedoch zeitnah in
   allowedSenderDestinations überführen und anschließend aus allowedSenders entfernen.
   Hinweis

   Die allowedSenders-Liste (Client-IDs) wird zu einem späteren Zeitpunkt vollständig abgekündigt und geleert. Alle Konfigurationen müssen bis dahin auf
   allowedSenderDestinations (Destination-IDs) umgestellt sein.

     * Grundsätzliche Konzeptänderungen
          + Keine Unterschiede mehr bei den Clients (Zugangsdaten)
     * Änderungen, die jede bestehende Anbindung betreffen
          + fromDestinationId als Pflichtfeld
          + Callbacks auf Destination-Ebene
          + Entfernung der bisherigen Limits-Endpunkte
          + Strenge LeiKa-Validierung
          + Sortierung von GET /v3/submissions
          + Neue optionale Filterparameter
          + Metadaten
          + Dataset reply-channel 1.0.0
          + Routing
     * Änderungen für Anbindungen mit dem Java SDK
     * Änderungen für Anbindungen mit dem .NET SDK
     * Änderungen für Anbindungen ohne SDK
          + Basis-URLs und Pfade
     * Weitere Änderungen für Nutzer der Destination-API
          + Verpflichtende Angabe von Destination-Typen
          + Zeitstempel updatedAt
          + Authentifizierung beim Abruf von Destinations
          + allowedSenders (Client-IDs) wird durch allowedSenderDestinations (Destination-IDs) abgelöst
```
