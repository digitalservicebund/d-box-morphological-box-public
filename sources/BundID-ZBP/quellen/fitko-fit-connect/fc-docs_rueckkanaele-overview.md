# Übersicht

- Quelle: <https://docs.fitko.de/fit-connect/docs/reply-channels/overview>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Übersicht

   Bislang konnten Verwaltungssysteme Bescheide oder Rückfragen nur über externe Kanäle (E-Mail, De-Mail, FINK, ELSTER oder auf dem Postweg) zurück an
   Onlinedienste senden. Mit der Erweiterung von FIT-Connect um die bidirektionale Kommunikation ist die Rücksendung vollintegriert über FIT-Connect
   möglich.

   Zusammengefasst: Ein Rückkanal ist der konkrete Weg, über den ein Verwaltungssystem Antworten an einen Onlinedienst zurücksendet. Bidirektionale
   Kommunikation beschreibt dagegen den gesamten Austausch in beide Richtungen (Einreichung ➝ Antwort). Wird FIT-Connect als Rückkanal gewählt, läuft die
   Antwort integriert über FIT-Connect.

   Dazu müssen folgende Vorbedingungen erfüllt sein:
     * Die Verwaltungssysteme müssen FIT-Connect mit ihrem Zustellpunkt als Rückkanaloption unterstützen.
     * Die Onlinedienste müssen FIT-Connect als Rückkanal für Einreichungen auswählen.

Was ist ein Rückkanal (Reply Channel)?

   Ein Rückkanal ist ein Kommunikationskanal, der es einem Verwaltungssystem ermöglicht, auf Einreichungen zu antworten. Dazu müssen die unterstützten
   Rückkanäle eines Verwaltungssystems im zugehörigen Zustellpunkt registriert sein.

   Konkret bedeutet das: Das Verwaltungssystem bietet einen oder mehrere Rückkanäle am Zustellpunkt an, und der Onlinedienst wählt einen dieser Kanäle für
   seine Einreichungen. Nur der gewählte Rückkanal wird anschließend für Antworten genutzt.

   Im Folgenden wird vor allem auf den Rückkanal FIT-Connect eingegangen, da dieser im System FIT-Connect maschinenlesbar unterstützt wird. Antworten von
   Verwaltungssystemen an einen Onlinedienst über externe Kanäle (wie z. B. De-Mail) werden über FIT-Connect nicht abgebildet.

Zusammenspiel von Onlinedienst und Verwaltungssystem

   Die folgende Beschreibung zeigt, wie Onlinedienste und Verwaltungssysteme im Rahmen einer bidirektionalen Kommunikation miteinander Nachrichten
   austauschen.

   Bidirektional bedeutet hier: Einreichungen gehen vom Onlinedienst zum Verwaltungssystem, Antworten kommen über den gewählten Rückkanal zurück –
   innerhalb desselben Vorgangs (caseId).

   Damit eine Rückkanal-Kommunikation über FIT-Connect möglich ist, muss ein Verwaltungssystem die Rückkanaloption FIT-Connect für mindestens eine seiner
   Verwaltungsleistungen anbieten. Ist in der Verwaltungsleistung keine Rückkanaloption hinterlegt, kann ein Rückkanal auch für den kompletten
   Zustellpunkt (und damit für alle Verwaltungsleistungen) definiert werden. Details hierzu finden Sie auf der Seite Einstellungen beim Verwaltungssystem.

   Ein Onlinedienst kann anschließend eine der vom Zustellpunkt unterstützten Rückkanaloptionen wählen. Details hierzu finden Sie auf der Seite
   Einstellungen beim Onlinedienst.

   Anschließend kann ein Verwaltungssystem mit der vom Onlinedienst gewählten Rückkanaloption eine Antwort (Reply) an den Onlinedienst schicken.

   Fachlich (also ohne Erwähnung von FIT-Connect als Mittelsmann) ist folgender Ablauf abgebildet:

   Ein technisch detailliertes Beispiel finden Sie weiter unten im Diagramm.

Vorgang

   Der Vorgang (Case) bildet eine Klammer um die Kommunikation zwischen Onlinedienst und Verwaltungssystem. Mit der initialen Einreichung wird auch immer
   ein Vorgang erzeugt.

   Zu diesem Vorgang können dann beliebig viele (weitere) Einreichungen (Submissions) und Antworten (Replies) erzeugt werden. Bei Einreichungen und
   Antworten wird dies durch Angabe der caseId am jeweiligen Endpunkt abgebildet:
     * POST /v2/replies
     * POST /v2/submissions

Events bei bidirektionaler Kommunikation

   Events, die im Rahmen der bidirektionalen Kommunikation hinzugekommen sind, werden auf der Seite Events bei bidirektionaler Kommunikation beschrieben.

Prozessstandards

   Wählt ein Onlinedienst den Rückkanal FIT-Connect, so müssen auch die unterstützten Prozessstandards mit angegeben werden. Dazu muss auch das
   Verwaltungssystem diese zur Verfügung stellen. Ein Beispiel für einen solchen Prozessstandard ist XBau mit dem Wert urn:xoev-de:bmk:standard:xbau_2.4.

Begriffsklärung: Sender/Empfänger ➡ Onlinedienst/Verwaltungssystem

   In der Dokumentation werden noch häufig die Begriffe Sender und Empfänger verwendet. Im Rahmen der Bidirektionalen Kommunikation verschwimmen diese
   Begriffe allerdings, da Sender nun auch zu Empfängern werden und umgekehrt. Sender sind in der Regel Onlinedienste, bei Empfängern handelt es sich
   meist um Verwaltungssysteme.

   Daher wird zukünftig für den Begriff "Sender" der Begriff "Onlinedienst" verwendet. Für den Begriff "Empfänger" wird zukünfig der Begriff
   "Verwaltungssystem" verwendet.

   Über FIT-Connect tauschen sich also Onlinedienste und Verwaltungssysteme im Rahmen von Vorgängen aus. Dabei versenden Onlinedienste Einreichungen
   (Submissions) und Verwaltungssysteme Antworten (Replies).

   Es ist klar, dass nicht alle ehemaligen Empfänger auch Verwaltungssysteme sind. Hier sind auch andere antragsbearbeitende Softwaresysteme denkbar. Zum
   Zwecke der Lesbarkeit und Prägnanz werden allerdings zukünftig die oben genannten Begriffe verwendet.

Beispielhaftes Sequenzdiagramm zum Anlegen einer Antwort (Reply)
```
