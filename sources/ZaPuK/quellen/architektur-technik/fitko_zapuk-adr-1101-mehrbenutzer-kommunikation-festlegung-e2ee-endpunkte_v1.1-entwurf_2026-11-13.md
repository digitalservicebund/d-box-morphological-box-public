<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/ef99118c0607dd7fb11a005593569a36a0f84343/Zielarchitektur/Architekturentscheidungen/ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md -->
<!-- Branch: architecture-v1.1, Commit ef99118c06 (2026-11-13); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 77251f5cdd); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# ADR-1101: E2EE-Endpunkte bei der Kommunikation mit Funktionspostfächern

> **Status:** ✔️ Operativ entschieden | **Datum:** 13. November 2026

## Kontext und Problembeschreibung

Bei [Funktionspostfächern](../../Glossar.md#funktionspostfach) greifen mehrere Personen oder [technische Nutzer](../../Glossar.md#technischer-nutzer) auf denselben verschlüsselten Nachrichtenbestand zu. Die Mitgliedschaften in den Gruppen mit Zugang zum [Postfach](../../Glossar.md#postfach) können sich während der Lebensdauer eines Raumes ändern. Die Zielarchitektur nutzt das Matrix-Protokoll mit [Ende-zu-Ende-Verschlüsselung](../../Glossar.md#ende-zu-ende-verschlüsselung) (E2EE). Zum Zeitpunkt des Versands einer [Nachricht](../../Glossar.md#nachricht) erhalten nur die im Raum derzeit befindlichen Endgeräte die für die Entschlüsselung notwendigen Schlüssel. Einer nachträglich in den Raum eintretenden Person müssen Schlüssel älterer Nachrichten explizit weitergegeben werden. [^1]

Je nach Umsetzung von Funktionspostfächern und Diensten wie Archivierung oder Malware-Schutz können zusätzliche [E2EE-Endpunkte](../../Glossar.md#e2ee-endpunkt) notwendig werden. Dies steht in Kontrast zur Anforderung der Unmöglichkeit von Entschlüsselung durch Dritte (SIC_VER_ÜBE_032) und zur Anforderung an E2EE (SIC_VER_ÜBE_026).

Im TI-Messenger der gematik existiert für die Umsetzung der Mehrbenutzer-Kommunikation das Konzept der “Funktionsaccounts”.[^2] “Chatbots” nehmen in den Ende-zu-Ende-verschlüsselten Räumen an der Kommunikation teil. Die Implementierung erfolgt in einem “Headless-Client”[^3] bzw. der “Bot-Engine”[^4], die zulassungspflichtig in der Verantwortung der [Organisationen](../../Glossar.md#organisation) betrieben wird. Dieses Konzept entspricht dem des im Folgenden als “technischer Nutzer” bezeichneten Teilnehmers. Die Betrachtung von MLS und einer Raumverwaltung durch “technische Nutzer” ohne Schlüsselbesitz steht im TI-Messenger-Konzept bislang aus.

## Entscheidungstreiber

- **Sicherheit:** Die Vertraulichkeit der Kommunikation muss so weit wie möglich gewahrt bleiben. Privates Schlüsselmaterial darf nur [Absender:in](../../Glossar.md#absenderin) und [Empfänger:in](../../Glossar.md#empfängerin) zugänglich sein (vgl. SIC_VER_ÜBE_028).
- **Praktische Umsetzbarkeit:** Funktionspostfächer (ANF_FUN_USE_072), Malware-Prüfung (SIC_ÜBE_MAL_005) und Archivierung müssen abbildbar sein.
- **Bestandssysteme:** Bestehende Systeme setzen bereits Mehrbenutzer-Zugriff mit Berechtigungssteuerung um (z. B. OZG-PLUS-Postfach). Die Anbindung soll über SDKs erfolgen können (vgl. ANF_ARC_ANB_018).
- **Abwägung der Schutzziele:** Die Wahl des E2EE-Endpunktes ergibt sich aus der Schutzbedarfsfeststellung der Organisation und der daraus resultierenden Gewichtung von Vertraulichkeit, [Integrität](../../Glossar.md#integrität) und Verfügbarkeit (z. B. Gruppenzugriff). Die strikte Umsetzung der E2EE (SIC_VER_ÜBE_032) ist dabei gegen die übrigen Anforderungen abzuwägen.

## Betrachtete Optionen

| **Merkmal**                           | **Option A:** E2EE ohne Automatisierung                         | **Option B:** E2EE mit Automatisierung                                                                        | **Option C:** E2EE bis Org-Grenze                       | **Option D:** E2EE extern                                           |
|---------------------------------------|-----------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------|---------------------------------------------------------|---------------------------------------------------------------------|
| **E2EE-Endpunkt**                     | Endgerät                                                        | Endgeräte, je nach Konfiguration zusätzlich technischer Nutzer auf Org-Server                                 | Server im [Machtbereich](../../Glossar.md#machtbereich) der Organisation                 | Server außerhalb der Org                                            |
| **Matrix durch-gängig**               | ja                                                              | ja                                                                                                            | Nein (internes Protokoll nötig)                         | Nein                                                                |
| **Serverseitiger Klartext**           | Nein                                                            | Nur auf dem Org-Server des technischen Nutzers, in Räumen mit E2EE-Teilnahme                                  | Alle Nachrichten entschlüsselt im Machtbereich der Org. | Alle Nachrichten entschlüsselt außerhalb des Machtbereichs der Org. |
| **Bearbeitungsvertretung**            | Nicht möglich (Schlüsselweitergabe erfordert aktive Teilnehmer) | Möglich, technischer Nutzer lädt zusätzliche berechtigte Personen ein. Historienzugriff je nach Konfiguration | Möglich, Backend leitet intern weiter                   | Möglich, Umsetzung beim Drittanbieter                               |
| **Verfügbarkeit bei Personalausfall** | Kein Zugriff, wenn alle Teilnehmenden ausfallen                 | Mit E2EE-Teilnahme sichert der technische Nutzer den Zugriff, ohne E2EE-Teilnahme wie Option A                | Backend sichert Zugriff                                 | Backend sichert Zugriff                                             |
| **Archivierung / Malware**            | nur clientseitig lösbar                                         | serverseitig möglich, nur mit E2EE-Teilnahme                                                                  | Möglich, zentral am Backend                             | Möglich, Umsetzung beim Drittanbieter                               |
| **Vertraulichkeit**                   | Maximal                                                         | Maximal ohne E2EE-Teilnahme, eingeschränkt mit E2EE-Teilnahme                                                 | Abhängig von Org-Infrastruktur                          | Stark beeinträchtigt                                                |

### Detailbeschreibung der Optionen

- **Option A (E2EE ohne Automatisierung):** Die Kommunikation über Matrix wird durchgängig zwischen beteiligten Endgeräten der natürlichen Personen gewährleistet. Es gibt keine an der Kommunikation beteiligten technischen Nutzer und keinen serverseitigen Zugriff auf den Klartext verschlüsselter Nachrichten. Weitere Personen können durch einen der bisherigen Teilnehmenden eingeladen werden, der im Raum über Moderator-Rechte verfügt.

- Dies ist die sicherste Option, da das Schlüsselmaterial ausschließlich in den Endgeräten natürlicher Personen verbleibt, weshalb sie sich für eine Kommunikation mit höchsten Anforderungen an die Vertraulichkeit eignet.

- Adressierbare Funktionspostfächer sind so nicht oder nur sehr eingeschränkt umsetzbar, vor allem da es keine technische Möglichkeit gibt, [Bearbeitungsvertretungen](../../Glossar.md#bearbeitungsvertretung) im Raum automatisiert abzubilden.

![Option A: E2EE ohne Automatisierung](images/ADR-1101-option-a.svg)

- **Option B (E2EE mit Automatisierung):** Das Matrix-Protokoll wird durchgängig bis in die Endgeräte umgesetzt. Ein technischer Nutzer (im Matrix-Kontext als “Bot” bezeichnet) ist zusätzlich Teilnehmer in den betroffenen Räumen. Er verwaltet das zugeordnete Funktionspostfach und kann bei Bedarf eine Archivierung oder Malware-Prüfung der Nachrichten veranlassen. Dazu benötigt er nur die Schlüssel der Räume, in denen er Mitglied ist.

- Der technische Nutzer wird als berechtigter Empfänger angesehen, wenn er unter der administrativen Kontrolle der Organisation betrieben wird.

- Die Verwaltung der [Nutzer](../../Glossar.md#nutzerinnen) im Raum wird über unverschlüsselte State [Events](../../Glossar.md#event) in Matrix umgesetzt. Somit benötigt der Administrator für die Raumverwaltung die E2EE-Schlüssel nicht. Deshalb gibt es zwei Konfigurationsmöglichkeiten für technische Nutzer:

- **Mit E2EE-Teilnahme:** Technische Nutzer können Schlüssel weitergeben, um die bisherige Kommunikation an neue Teilnehmer weiterzugeben. Archivierung und Malware-Prüfung können serverseitig umgesetzt werden. Der Server hält Schlüsselmaterial persistent vor und ist entsprechend abzusichern.

- **Ohne E2EE-Teilnahme:** Der berechtigte technische Nutzer, der nicht über die E2EE-Schlüssel des Raums verfügt, verwaltet die Mitgliedschaften ohne Einblick in den Klartext der Nachrichten. Zugriff auf die bisherige Kommunikation kann ein aktives Mitglied durch Weitergabe der Schlüssel herstellen.

![Option B: E2EE mit Automatisierung und mit E2EE Teilnahme](images/ADR-1101-option-b.svg)

- **Option C (E2EE bis Org-Grenze):** Ein technischer Nutzer, der Mitglied im Raum ist und Zugriff auf den Klartext hat, wird an der Organisationsgrenze betrieben. Er vermittelt Nachrichten zwischen einem anderen System und dem Matrix-Raum. Die Kommunikation auf Matrix-Protokoll-Basis endet hier. Nachrichten werden intern über andere Wege weiterverarbeitet. Die Anbindung bestehender [Fachverfahren](../../Glossar.md#fachverfahren) kann somit einfacher sein, da z. B. Rollen und Rechte für die organisationsinternen Systeme wie gewohnt durchgesetzt werden könnten.

- Die gesamte Kommunikation wird an der Organisationsgrenze entschlüsselt. Der technische Nutzer vermittelt damit die gesamte Kommunikation der Organisation über einen zentralen Dienst. Eine strikte Ende-zu-Ende-Verschlüsselung, die eine vertrauliche Kommunikation zwischen natürlichen Personen fordert, ist somit nicht gewährleistet (vgl. SIC_VER_ÜBE_032).

- Der Server mit Zugriff auf den Klartext wird innerhalb des Machtbereichs der Organisation betrieben. Der Betreiber, der außerhalb der Organisation liegende Systeme betreibt, hat keinen Zugriff auf den Klartext, was die Anforderung SIC_VER_ÜBE_032 im Kern erfüllt.

![Option C: E2EE bis Org-Grenze](images/ADR-1101-option-c.svg)

- **Option D (E2EE extern):** Die Entschlüsselung findet auf einem Server statt, der nicht im Machtbereich der Organisation liegt. Dritte können damit auf den Klartext der Kommunikation Zugriff erlangen. Dies gefährdet die Vertraulichkeit und steht im Widerspruch zu SIC_VER_ÜBE_032.

![Ausgeschlossene Option D: E2EE extern](images/ADR-1101-option-d.svg)

### Gegenüberstellung der Optionen

#### Option A: E2EE ohne Automatisierung

- Gut, denn es existieren keine Schlüssel außerhalb der Endgeräte natürlicher Personen.

- Gut, denn die Anforderung der Unmöglichkeit der Entschlüsselung durch Dritte (SIC_VER_ÜBE_032) und die Anforderung an E2EE (SIC_VER_ÜBE_026) werden voll erfüllt.

- Gut, denn die Kommunizierenden können sich auf Protokollebene authentifizieren.

- Schlecht, denn die Umsetzung von Bearbeitungsvertretungen innerhalb der Bearbeitergruppen muss manuell durch bestehende Teilnehmende erfolgen.

- Schlecht, denn serverseitige Dienste (z. B. für Archivierung oder Malware-Prüfung) sind nicht umsetzbar. Stattdessen müssen Funktionen clientseitig umgesetzt werden.

- Schlecht, denn adressierbare Funktionspostfächer mit automatisierten Bearbeitungsvertretungen sind nicht umsetzbar.

#### Option B: E2EE mit Automatisierung

- Gut, denn E2EE bleibt bis in die Endgeräte erhalten.

- Gut, denn mit E2EE-Teilnahme sind serverseitige Dienste umsetzbar (Archivierung, Malware-Prüfung).

- Gut, denn Funktionspostfächer mit automatisierten Bearbeitungsvertretungen sind in beiden Konfigurationen (mit/ohne E2EE-Teilnahme) umsetzbar.

- Gut, denn die Kommunikationsteilnehmenden können sich auf Protokollebene authentifizieren.

- Gut, denn der serverseitige Zugriff auf den Klartext der Nachrichten bleibt auf Räume mit E2EE-Teilnahme des technischen Nutzers beschränkt.

- Gut, denn ohne E2EE-Teilnahme existieren wie bei Option A keine Schlüssel außerhalb der Endgeräte natürlicher Personen.

- Schlecht, denn mit E2EE-Teilnahme werden im Server die Schlüssel der technischen Nutzer vorgehalten. Ein kompromittierter Server gefährdet alle dem technischen Nutzer zugewiesenen Räume. In diesen Räumen ist die E2EE nicht strikt umgesetzt (vgl. SIC_VER_ÜBE_032).

#### Option C: E2EE bis Org-Grenze

- Gut, denn bestehende Fachverfahren können vergleichsweise einfach angebunden werden.

- Gut, denn eine serverseitige Verarbeitung (Archivierung, Malware-Prüfung) und die Umsetzung von Vertretungen in den Bearbeitergruppen sind ohne Matrix-Integration umsetzbar.

- Schlecht, denn die gesamte Kommunikation aller [Vorgänge](../../Glossar.md#vorgang) wird auf einem Server entschlüsselt.

- Schlecht, denn eine vertrauliche Kommunikation allein zwischen natürlichen Personen ist so nicht umsetzbar. Die E2EE ist nicht strikt umgesetzt (vgl. SIC_VER_ÜBE_032).

- Schlecht, denn die Kommunikationsteilnehmenden können sich nicht auf Protokollebene authentifizieren.

#### Option D: E2EE extern

- Schlecht, denn Dritte erhalten Zugriff auf den Klartext der gesamten Kommunikation der Organisation, was die Anforderung SIC_VER_ÜBE_032 und auch SIC_VER_ÜBE_026 verletzt.

## Architekturentscheidung

Gewählte Option: **Option B (E2EE mit Automatisierung)**, denn sie erlaubt es, die Vertraulichkeit und Automatisierbarkeit der Verarbeitung auf Raumebene zu steuern. Eine Kommunikation mit strikter E2EE zwischen natürlichen Personen ist ebenso möglich wie eine Umsetzung von Funktionspostfächern.

- Das Matrix-Protokoll bleibt durchgängig bis in die Endgeräte erhalten, während Funktionspostfächer, automatisierte Bearbeitungsvertretungen und Archivierung über technische Nutzer möglich sind.

- Die Organisation kann je Raum die Teilnahme des technischen Nutzers konfigurieren. Sowohl eine Teilnahme an der E2EE-Verschlüsselung ist möglich, als auch eine Administration durch technische Nutzer ohne E2EE-Schlüsselmaterial.

- Mit E2EE-Teilnahme hält der technische Nutzer Schlüsselmaterial für alle ihm zugewiesenen Räume persistent auf dem Server vor. Ein kompromittierter Server ermöglicht die rückwirkende Entschlüsselung der gesamten Kommunikation aller betroffenen Räume. Absicherung, Härtung und Monitoring des Servers sind zwingend erforderlich.

- Ohne E2EE-Teilnahme verwaltet der technische Nutzer die Mitglieder als Raumadministrator, wobei er nicht über den Zugriff auf den Klartext der Nachrichten verfügt. Neu eingeladene Mitglieder erhalten dann nur Schlüssel für Nachrichten ab ihrem Beitritt. Zugriff auf die bisherige Kommunikationshistorie kann nur ein aktives bisheriges Mitglied durch Weitergabe der Megolm-Session-Keys herstellen. Diese Konfiguration steht unter MLS gegebenenfalls nicht mehr zur Verfügung (siehe Risiken).

**Bedingte Alternativen:**

- **Option A (höchster Schutzbedarf):** Wenn Vertraulichkeit besonders wichtig ist, kann auf einen technischen Nutzer verzichtet werden. Die Kommunikation erfolgt als 1:1-Raum oder als manuell verwalteter Gruppenraum. Adressierbare Funktionspostfächer mit automatisierten Bearbeitungsvertretungen sind so allerdings nicht umsetzbar. Zudem kann ein technischer Nutzer den Raum nur initial erzeugen und anschließend verlassen.

- **Option C (begründete Ausnahme):** Fachverfahren, die eine direkte Matrix-Integration (noch) nicht vollständig umsetzen können, dürfen Option C begründet nutzen. Der Terminierungspunkt muss dabei im administrativen Machtbereich der empfangenden Stelle liegen. Dies schließt den Betrieb durch IT-Dienstleister im Auftrag der Organisation ein (Auftragsverarbeitung). Die Begründung für die Wahl von Option C ist zu dokumentieren und durch die verantwortliche Stelle freizugeben. Darüber hinaus ist ein Migrationspfad in Richtung Option B zu prüfen und zu dokumentieren.

**Option D wird ausgeschlossen.** Eine Entschlüsselung der Kommunikation außerhalb des Machtbereichs der Organisation gefährdet die Vertraulichkeit und wird ausgeschlossen.

Die Wahl zwischen Option A, B und C obliegt der jeweiligen Organisation auf Basis ihrer spezifischen Anforderungen bzw. ihrer Möglichkeiten zu einer Umsetzung.

## Auswirkungen

- **Mehrbenutzer-Szenarien** bleiben ohne eine vollständige Aufgabe der E2EE umsetzbar. Bei Option A und bei Option B ohne E2EE-Teilnahme bleibt eine strikte Umsetzung der E2EE über das Matrix-Protokoll möglich. Bei Option B mit E2EE-Teilnahme ist der serverseitige Zugriff auf den Klartext verschlüsselter Nachrichten auf die gewählten Räume beschränkt.

- **Anbindung**: Als begründete Ausnahme ermöglicht Option C eine einfachere Anbindung von Organisationssystemen wie Fachverfahren an die Zielarchitektur.

## Risiken

- **Option B:** Eine Härtung des Servers mit konfigurierten technischen Nutzern ist notwendig, da hier die Schlüssel für die Kommunikation vorliegen müssen.

- **Option B und MLS-Kompatibilität:** Je nach Umsetzung von MLS in Matrix kann die Trennung von unverschlüsselten State Events und verschlüsselten Message Events entfallen (z. B. MSC4256). In MLS ist die Gruppenzugehörigkeit zwischen den Teilnehmern kryptografisch gesichert. Ein technischer Nutzer müsste dann MLS-Gruppenmitglied sein, um den Raum verwalten zu können. Die Konfiguration ohne E2EE-Teilnahme stünde in diesem Fall nicht mehr zur Verfügung.

- **Option C:** E2EE endet im Machtbereich der Organisation. Die gesamte Kommunikation (vorgangsübergreifend) wird an zentraler Stelle entschlüsselt, ggf. auch Vorgänge, die keinen Mehrbenutzerzugriff erfordern. Die interne Weiterverarbeitung der entschlüsselten Nachrichten liegt außerhalb des Matrix-Protokolls.

- **Übergreifend:** Die Vertraulichkeit einer Kommunikation wird durch die schwächere Seite bestimmt. Die sendende Partei kann nicht beeinflussen, wie die empfangende Seite die Nachricht nach dem Empfang verarbeitet.

## Folge-Entscheidungen

- **Raumarchitektur und Funktionspostfach:** konkrete Umsetzung der hier in Option B beschriebenen Mehrbenutzer-Räume, Trennung von Funktionspostfach und [Vorgangsraum](../../Glossar.md#vorgangsraum), Rollenmodell, siehe [ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md).

- **Betriebsmodelle für Organisationen:** Komponenten-Aufteilung bei Minimalanbindung, Teilbetrieb und souveränem Betrieb sowie ihre Auswirkungen auf den E2EE-Endpunkt, siehe [ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md).

- **Identitätsnachweis:** Trennung von fachlicher [Identität](../../Glossar.md#identität) und Matrix-Schlüsselidentität, siehe [ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md) und [ADR-1107](./ADR-1107-Nachweisformat-Identitaetsnachweis-Matrix-Nachrichten.md).

## Referenzen

[^1]: [Matrix v1.19 release: Encrypted room history sharing](https://matrix.org/blog/2026/07/08/matrix-v1.19-release/#encrypted-room-history-sharing), abgerufen am 21.08.2026

[^2]: [gematik, gemSpec_TI-M_Pro (Funktionsaccounts; Chatbots als E2EE-Teilnehmer)](https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Pro/), abgerufen am 21. Juni 2026.

[^3]: [gematik, Entwurf gemF_TI-M_Headless_Client](https://gemspec.gematik.de/prereleases/Draft_gemF_TI-M_Headless_Client/), abgerufen am 21. Juni 2026.

[^4]: [gematik, Entwurf gemF_TI-M_Automatisierung (Automatisierung und Bots; Bot-Engine)](https://gemspec.gematik.de/prereleases/Draft_gemF_TI-M_Automatisierung/), abgerufen am 21. Juni 2026.
