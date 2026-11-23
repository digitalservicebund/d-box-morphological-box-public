<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Zielarchitektur/Architekturentscheidungen/ADR-0007-Implementierung-von-Postfach-Backends.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-0007-Implementierung-von-Postfach-Backends.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 85727fdbe0); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

> **Status:** ✔️ Operativ entschieden | **Datum:** 14. April 2025

> **Hinweis (Phase 2, Stand Oktober 2026):** Dieser ADR wurde in Phase 2 eingeschränkt. Der Anschluss von Postfach-Backends durch Aufnahme in die Föderationsliste setzt die Annahme des Anschlussvertrags (ADR-1145) und die Freigabe durch eine zuständige Stelle (ADR-1148) voraus. Die Begründung für bedingungslos zugelassene Drittimplementierungen ist damit teilweise überholt.

# ADR-0007: Implementierung von Postfach-Backends

## Kontext und Problembeschreibung

Im Kontext einer offenen oder geschlossen Systemarchitektur (vgl. [ADR-0002](./ADR-0002-Infrastrukturtopologie.md)) ergibt sich die Frage, inwiefern eigene Postfach-Backends von Dritten geschaffen werden können und im speziellen, welche konkreten Implementierungen im Gesamtsystem zugelassen sind.
Der wichtigste Betrachtungsgegenstand bei der Erwägung einer Einschränkung zulässiger Implementierungen ist die Fragestellung, inwiefern ein Backend, das sich nicht wie erwartet, also fehlerhaft oder sogar bösartig, verhält, Schaden anrichten kann.

Dass eine Einschränkung nicht grundsätzlich nötig ist, zeigen diverse offene, föderierte Kommunikationsinfrastrukturen, wie beispielsweise die weltweite E-Mail-Infrastruktur oder föderierte Social-Media-Netzwerke wie Mastodon.
Diese Infrastrukturen sehen Maßnahmen zum Umgang mit unerwartetem oder bösartigem Verhalten dritter Backend-Systeme vor.
Es können beliebige eigene Implementierungen von Backends verwendet werden; fehlerhaftes oder bösartiges Verhalten (das ggf. sogar von der Spezifikation abweicht) schaden grundsätzlich höchstens den [Nutzer:innen] des entsprechenden Backends.

Andere Infrastrukturen, wie der TI-Messenger, lassen ausschließlich zertifizierte Implementierungen zu.
Solche Einschränkungen kommen vor allem dann in Frage, wenn Abweichungen von der vorgesehenen Funktion zu Schaden führen können; zum Beispiel durch das Nicht-Versenden von Abrufbestätigungen, die eine Auswirkung auf die Zustellfiktion haben können.

Die Vor- und Nachteile einer Beschränkung der Implementierungen hängen auch wesentlich von der letztlichen Betriebsverantwortung ab.
Kann keine Vertrauensstellung angenommen werden, so gestaltet sich die praktische Umsetzung einer Einschränkung in den Implementierungen als schwierig.
Bei einem eher kleinen Kreis von vertrauenswürdigen Teilnehmenden (etwa durch Zertifizierung) ist der Vorteil freier Implementierungen sowie der Aufwand einer zusätzlichen Zertifizierung der Implementierung dagegen vergleichsweise gering.

## Entscheidungstreiber

- Kleinere Kommunen können sich weder die Entwicklung noch den Betrieb eigener Backends leisten. Auch Privatpersonen können oder wollen das i.d.R. nicht leisten.
- Das Einbringen von angreifbaren oder bösartigen Backends in [das System] kann weitreichende Sicherheitsprobleme aufbringen.
- Zero Trust: Die Zielarchitektur verfolgt einen Zero-Trust-Ansatz. Dieser Ansatz sieht eine technische Überprüfung jeglicher Interaktionen mit dritten Backend-Systemen vor („never trust, always verify“). Dieses Grundprinzip erleichtert die Akzeptanz fremder Implementierungen im Vergleich zur blinden Annahme einer Vertrauenswürdigkeit von dritten Backend-Systemen.

## Betrachtete Optionen

- [Option 1: Drittimplementierungen sind nicht erlaubt; eine Implementierung wird zentral bereitgestellt](#option-1-drittimplementierungen-sind-nicht-erlaubt-eine-implementierung-wird-zentral-bereitgestellt)
- [Option 2a: Drittimplementierungen sind einem Kontroll- und Verifikationsprozess unterworfen; eine Referenzimplementierung wird zentral bereitgestellt](#option-2a-drittimplementierungen-sind-einem-kontroll--und-verifikationsprozess-unterworfen-eine-referenzimplementierung-wird-zentral-bereitgestellt)
- [Option 2b: Drittimplementierungen sind einem Kontroll- und Verifikationsprozess unterworfen; es wird keine Referenzimplementierung bereitgestellt](#option-2b-drittimplementierungen-sind-einem-kontroll--und-verifikationsprozess-unterworfen-es-wird-keine-referenzimplementierung-bereitgestellt)
- [Option 3a: Drittimplementierungen sind bedingungslos zugelassen; eine Referenzimplementierung wird zentral bereitgestellt](#option-3a-drittimplementierungen-sind-bedingungslos-zugelassen-eine-referenzimplementierung-wird-zentral-bereitgestellt)
- [Option 3b: Drittimplementierungen werden bedingungslos erlaubt; es wird keine Referenzimplementierung bereitgestellt](#option-3b-drittimplementierungen-werden-bedingungslos-erlaubt-es-wird-keine-referenzimplementierung-bereitgestellt)

## Gegenüberstellung der jeweiligen Optionen

### Option 1: Drittimplementierungen sind nicht erlaubt; eine Implementierung wird zentral bereitgestellt

- Gut weil einzelne Akteure auf eine zentral bereitgestellte Referenzimplementierung zurückgreifen können
- Gut, weil der Angriffsraum eingeschränkt und die Verifikation von Systemen erleichtert wird
- Schlecht, weil bösartige Manipulationen am Systemverhalten von Backend-Systemen auch dann nicht verhindert werden können, wenn Dritte zur Nutzung einer zentral bereitgestellten Implementierung verpflichtet werden
- Schlecht, weil auch einzelne Akteure auf Seiten der Verwaltung (z.B. große Fachbehörden/Bundesbehörden) keine eigenen Backend-Systeme implementieren können und folglich auf die zentral bereitgestellte Implementierung zurückgreifen müssen; Spezialanforderungen können nicht oder nur zentral umgesetzt werden
- Schlecht, weil Maßnahmen ergriffen werden müssen, die die Einhaltung der Regelung sicherstellen
- Schlecht, weil Kosten für Entwicklung und Wartung der bereitgestellte Referenzimplementierung entstehen

### Option 2a: Drittimplementierungen sind einem Kontroll- und Verifikationsprozess unterworfen; eine Referenzimplementierung wird zentral bereitgestellt

- Gut weil einzelne Akteure auf eine zentral bereitgestellte Referenzimplementierung zurückgreifen können
- Gut, weil einzelne Akteure bei Bedarf eigene Implementierungen von Backend-Systemen einsetzen können
- Neutral, weil fehlerhafte Implementierungen im Rahmen eines Kontrollprozesses auffallen können
- Schlecht, weil der Kontrollprozess implementiert werden muss und Aufwand verursacht
- Schlecht, weil Maßnahmen ergriffen werden müssen, die die Einhaltung der Regelung sicherstellen
- Schlecht, weil Kosten für Entwicklung und Wartung der bereitgestellte Referenzimplementierung entstehen

### Option 2b: Drittimplementierungen sind einem Kontroll- und Verifikationsprozess unterworfen; es wird keine Referenzimplementierung bereitgestellt 

- Gut, weil einzelne Akteure bei Bedarf eigene Implementierungen von Backend-Systemen einsetzen können
- Neutral, weil fehlerhafte Implementierungen im Rahmen eines Kontrollprozesses auffallen können
- Schlecht, weil der Kontrollprozess implementiert werden muss und Aufwand verursacht
- Schlecht, weil Maßnahmen ergriffen werden müssen, die die Einhaltung der Regelung sicherstellen
- Schlecht, weil Kosten für Entwicklung und Wartung der bereitgestellte Referenzimplementierung entstehen
- Schlecht, weil einzelne Akteure nicht auf eine zentral bereitgestellte Referenzimplementierung zurückgreifen können und damit auf Eigen- oder Drittimplementierungen angewiesen ist

### Option 3a: Drittimplementierungen sind bedingungslos zugelassen; eine Referenzimplementierung wird zentral bereitgestellt

- Gut, weil einzelne Akteure bei Bedarf eigene Implementierungen von Backend-Systemen einsetzen können
- Gut weil einzelne Akteure auf eine zentral bereitgestellte Referenzimplementierung zurückgreifen können
- Gut, weil kein zusätzlicher Aufwand durch die Kontrolle von Systemen entsteht
- Schlecht, weil Kosten für Entwicklung und Wartung der bereitgestellte Referenzimplementierung entstehen

### Option 3b: Drittimplementierungen werden bedingungslos erlaubt; es wird keine Referenzimplementierung bereitgestellt

- Gut, weil einzelne Akteure bei Bedarf eigene Implementierungen von Backend-Systemen einsetzen können
- Gut, weil kein zusätzlicher Aufwand durch die Kontrolle von Systemen entsteht.
- Schlecht, weil einzelne Akteure nicht auf eine zentral bereitgestellte Referenzimplementierung zurückgreifen können und damit auf Eigen- oder Drittimplementierungen angewiesen ist

## Architekturentscheidung

Gewählte Option: Option: 3a (Drittimplementierungen sind bedingungslos zugelassen; eine Referenzimplementierung wird zentral bereitgestellt), weil im Kontext eines offenen Betriebs ([ADR-0002](./ADR-0002-Infrastrukturtopologie.md), [ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md)) unter Anwendung des Zero-Trust-Paradigmas eine Kontrolle von Implementierungen weder realistisch noch zielführend scheint, die zentrale Bereitstellung einer Implementierung auf das Ziel der niedrigen Anschlusshürden einzahlt, aufgrund der zentralen Bereitstellung von Backend-Systemen (vgl. [ADR-0006](./ADR-0006-Anzahl-und-Zuschnitt-der-zentral-bereitgestellten-Postfach-Backends.md)) ohnehin erforderlich sein wird und somit die entstehenden Kosten rechtfertigt.

### Auswirkungen

Wie bereits in [ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md) festgestellt, muss – dem Zero-Trust-Ansatz entsprechend – weiterhin davon ausgegangen werden, dass sich einzelne Backend-Systeme möglicherweise nicht erwartungsgemäß verhalten.
Die Bereitstellung einer Implementierung wird Aufwand verursachen.
Dafür wird es für Teilnehmende deutlich einfacher sein, den Betrieb eines eigenen Backends zu realisieren.