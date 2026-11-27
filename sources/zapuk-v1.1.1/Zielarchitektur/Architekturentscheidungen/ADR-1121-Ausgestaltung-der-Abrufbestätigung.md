# ADR-1121: Ausgestaltung der Abrufbestätigung

> **Status:** 🛠️ In Arbeit | **Datum:** 23. November 2026

## Kontext und Problembeschreibung

Die [Abrufbestätigung](../../Glossar.md#abrufbestätigung) wird vom Client der [Empfänger:in](../../Glossar.md#empfängerin) gesendet, sobald eine [Nachricht](../../Glossar.md#nachricht) entschlüsselt und angezeigt wurde ([ADR-1118](./ADR-1118-Abruf-und-Lesebestätigungen.md)). Ein nicht-konformer Client kann sie allerdings unterdrücken.

Je nach Vorschrift tritt die [Bekanntgabe](../../Glossar.md#bekanntgabe) eines [Verwaltungsakts](../../Glossar.md#verwaltungsakt) zu einem anderen Zeitpunkt ein.

- **§ 41 Abs. 2a VwVfG:** Ein Verwaltungsakt kann dadurch bekannt gegeben werden, dass er vom Beteiligten oder von seinem Bevollmächtigten abgerufen wird. Er gilt am Tag nach dem Abruf als bekannt gegeben.[^1] Wird der Verwaltungsakt nicht innerhalb von zehn Tagen nach Absendung einer Benachrichtigung über die Bereitstellung abgerufen, endet die Bereitstellung. Die Bekanntgabe ist dann nicht bewirkt.
- **§ 9 Abs. 1 OZG und § 122a AO:** Der Verwaltungsakt gilt am vierten Tag nach der Bereitstellung zum Abruf als bekannt gegeben.[^2][^3] Im Zweifel hat die [Behörde](../../Glossar.md#behörde) die Bereitstellung und ihren Zeitpunkt nachzuweisen. Die Benachrichtigung über die Möglichkeit des Abrufs löst keine Frist aus.
- **§ 37 Abs. 2a SGB X:** Der Verwaltungsakt gilt am vierten Tag nach Absendung der Benachrichtigung über die Bereitstellung als bekannt gegeben.[^4] Im Zweifel hat die Behörde den [Zugang](../../Glossar.md#zugang) der Benachrichtigung nachzuweisen. Kann die Behörde den Zugang nicht nachweisen, gilt der Verwaltungsakt an dem Tag als bekannt gegeben, an dem die abrufberechtigte Person ihn abgerufen hat.

Der Abruf ist nach § 41 Abs. 2a VwVfG und § 37 Abs. 2a SGB X eine Handlung der Empfänger:in.

Welche Vorschrift gilt, hängt vom jeweiligen [Verwaltungsverfahren](../../Glossar.md#verwaltungsverfahren) ab. Das [Fachverfahren](../../Glossar.md#fachverfahren) fordert die dafür nötigen Bestätigungen an ([ADR-1119](./ADR-1119-Absendergesteuerte-Übermittlungsoptionen-mit-Berechtigungsprüfung.md)). Fehlt ein nötiger Nachweis des Abrufs, muss die Behörde den Verwaltungsakt erneut bereitstellen oder auf andere Weise bekannt geben.

§ 9 Abs. 1 OZG gilt nur für das [Postfach](../../Glossar.md#postfach) eines OZG-Nutzerkontos und wäre anzupassen, damit die [Bekanntgabefiktion](../../Glossar.md#bekanntgabefiktion) auch für vom [Nutzerkonto](../../Glossar.md#nutzerkonto) entkoppelte Postfächer an die Bereitstellung anknüpft ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

In dieser Architekturentscheidung wird festgelegt, ob der Abruf durch den Client, durch das [Postfach-Backend](../../Glossar.md#postfach-backend) oder durch beide bestätigt wird.

## Entscheidungstreiber

- **Abrufbestätigung:** Das System übermittelt [Absender:in](../../Glossar.md#absenderin) und Empfänger:in automatisch eine Abrufbestätigung im Falle eines erfolgreichen Abrufs einer Nachricht aus dem Postfach durch die Empfänger:in (ANF_FUN_ZUS_046).
- **Bekanntgabe:** Das System erfüllt die gesetzlich definierten Anforderungen zur rechtssicheren Realisierung einer Bekanntgabefiktion bei der elektronischen Übermittlung von Verwaltungsakten (ANF_FUN_ZUS_024).
- **E2EE-Konformität:** Die Verschlüsselung übermittelter und gespeicherter Nachrichten kann weder von der Betreiberin des Systems noch von Dritten gebrochen, umgangen oder entschlüsselt werden (SIC_VER_ÜBE_032).
- **Metadatenschutz:** Metadaten, die nicht zwingend für die Übermittlung erforderlich sind, werden vermieden, verschlüsselt oder verschleiert (SIC_VER_ÜBE_051).

## Betrachtete Optionen

### Option A: Abrufbestätigung nur durch den Client

Der Client sendet die Abrufbestätigung, wie in [ADR-1118](./ADR-1118-Abruf-und-Lesebestätigungen.md) festgelegt.

- Gut, denn die Bestätigung belegt, dass die Nachricht entschlüsselt und angezeigt wurde.
- Gut, denn die Bestätigung ist verschlüsselt, sodass keine weiteren für die Betreiber sichtbaren Metadaten entstehen ([ADR-1118](./ADR-1118-Abruf-und-Lesebestätigungen.md)).
- Schlecht, denn die Bestätigung ist durch einen nicht-konformen Client unterdrückbar.

### Option B: Abrufbestätigung nur durch das Postfach-Backend

Der [Quittungsdienst](../../Glossar.md#quittungsdienst) des Postfach-Backends bestätigt anstelle des Clients, dass ein Gerät der Empfänger:in den verschlüsselten [Anhang](../../Glossar.md#anhang) heruntergeladen hat. Das Abholen der Nachricht wird nicht bestätigt, weil es im Hintergrund geschieht und keine Handlung der Empfänger:in ist.

- Gut, denn die Bestätigung ist nicht unterdrückbar.
- Gut, denn das Herunterladen des Anhangs geht in der Regel auf eine Handlung der Empfänger:in zurück.
- Gut, denn Clients müssen die Abrufbestätigung nicht umsetzen ([ADR-1118](./ADR-1118-Abruf-und-Lesebestätigungen.md)).
- Schlecht, denn die Bestätigung belegt wie die [Bereitstellungsbestätigung](../../Glossar.md#bereitstellungsbestätigung) nicht, dass der Anhang entschlüsselt werden konnte ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).
- Schlecht, denn die Bestätigung entsteht nur, wenn der Verwaltungsakt als Anhang übermittelt wird.
- Schlecht, denn die Bestätigung liegt unverschlüsselt im Raum, sodass die Betreiber der beteiligten Backends den Zeitpunkt des Abrufs sehen.

### Option C: Abrufbestätigung durch Client und Postfach-Backend

Der Client sendet die Abrufbestätigung wie in Option A. Zusätzlich bestätigt der Quittungsdienst wie in Option B das Herunterladen des Anhangs.

- Gut, denn die Bestätigung des Postfach-Backends ist nicht unterdrückbar.
- Gut, denn die Abrufbestätigung des Clients belegt weiter, dass die Nachricht entschlüsselt und angezeigt wurde, auch bei Nachrichten ohne Anhang.
- Schlecht, denn Clients und Postfach-Backends müssen zwei Verfahren umsetzen.
- Schlecht, denn die Bestätigung des Postfach-Backends liegt unverschlüsselt im Raum, sodass die Betreiber der beteiligten Backends den Zeitpunkt des Abrufs sehen.

## Architekturentscheidung

Die Entscheidung ist offen, weil rechtliche Fragen zu klären sind, etwa ob § 9 OZG für das vom Nutzerkonto entkoppelte Postfach angepasst wird. Bis dahin gilt die Abrufbestätigung des Clients nach Option A ([ADR-1118](./ADR-1118-Abruf-und-Lesebestätigungen.md)).

## Auswirkungen

Bei Option A ändert sich nichts. Option B und Option C haben folgende Auswirkungen.

- Die Festlegung aus [ADR-1118](./ADR-1118-Abruf-und-Lesebestätigungen.md), dass keine serverseitige Protokollierung durch den Quittungsdienst stattfindet, wird angepasst. Bei Option B entfällt zudem die Abrufbestätigung des Clients.
- Die [Übermittlungsoptionen](../../Glossar.md#übermittlungsoption) nach [ADR-1119](./ADR-1119-Absendergesteuerte-Übermittlungsoptionen-mit-Berechtigungsprüfung.md) werden um eine Option für das Postfach-Backend ergänzt.
- [Postfach-Zugänge](../../Glossar.md#postfach-zugang) dürfen Anhänge, für die eine Bestätigung angefordert ist, erst auf Anforderung der Empfänger:in herunterladen.

## Risiken

- Lädt ein Client Anhänge von selbst, gilt ein Verwaltungsakt als abgerufen, ohne dass die Empfänger:in ihn geöffnet hat.

## Offene Punkte

- **Rechtsgrundlage:** Zu klären ist, welche Vorschrift im allgemeinen Verwaltungsverfahren für vom Nutzerkonto entkoppelte Postfächer gilt, solange § 9 OZG nicht angepasst ist.
- **Abruf:** Gilt das Herunterladen eines Anhangs als Abruf durch die abrufberechtigte Person, wenn das Postfach-Backend nicht erkennt, ob die Person oder der Client das Herunterladen ausgelöst hat?
- **Ende der Bereitstellung:** Zu klären ist, wie die Bereitstellung nach § 41 Abs. 2a Satz 4 VwVfG beendet wird, wenn der Verwaltungsakt nicht innerhalb von zehn Tagen abgerufen wurde.

## Referenzen

[^1]: [§ 41 VwVfG Bekanntgabe des Verwaltungsaktes](https://www.gesetze-im-internet.de/vwvfg/__41.html), abgerufen am 23. November 2026.

[^2]: [§ 9 OZG Bekanntgabe des Verwaltungsaktes](https://www.gesetze-im-internet.de/ozg/__9.html), abgerufen am 23. November 2026.

[^3]: [§ 122a AO Bekanntgabe von Verwaltungsakten durch Bereitstellung zum Datenabruf](https://www.gesetze-im-internet.de/ao_1977/__122a.html), abgerufen am 23. November 2026.

[^4]: [§ 37 SGB X Bekanntgabe des Verwaltungsaktes](https://www.gesetze-im-internet.de/sgb_10/__37.html), abgerufen am 23. November 2026.
