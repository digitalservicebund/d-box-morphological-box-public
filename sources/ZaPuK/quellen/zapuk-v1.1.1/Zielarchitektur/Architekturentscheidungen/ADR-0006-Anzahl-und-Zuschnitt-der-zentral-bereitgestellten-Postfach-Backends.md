> **Status:** ✔️ Operativ entschieden | **Datum:** 14. April 2025

> **Hinweis (Phase 2, Stand Oktober 2026):** Diese Entscheidung wurde in Phase 2 mit ADR-1114 (getrennter Betrieb der zentralen Postfach-Backends nach Teilnehmendengruppen) angepasst. Die Begründung für ein einziges zentrales Postfach-Backend ist damit teilweise überholt.

# ADR-0006: Anzahl und Zuschnitt der zentral bereitgestellten Postfach-Backends

## Kontext und Problembeschreibung

Dieser ADR betrachtet, ob und wenn ja, welche Backend-Systeme als nativer Teil der Gesamtinfrastruktur bereitgestellt werden sollten, insbesondere um wesentlichen [Nutzer:innen]-Gruppen die dezidierte und gesicherte Nutzung der Kommunikationsinfrastruktur zu ermöglichen.
In [ADR-0002](./ADR-0002-Infrastrukturtopologie.md) (Infrastrukturtopologie) wird der grundsätzliche Zuschnitt und die Topologie der Backend-Systeme der Kommunikationsinfrastruktur beschrieben.
Der gewählte hybride Ansatz sieht sowohl zentral bereitgestellte, als auch dezentral betriebene Backend-Systeme vor.

Für die zentral bereitgestellten Backend-Systeme gilt es zu entscheiden, wie viele Backend-Systeme zentral bereitgestellt werden sollen und wie ihr Zuschnitt erfolgt.

In Abgrenzung zu [ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md) (Bereitstellung von Postfach-Backends durch Dritte) beschreibt der vorliegende ADR den Zuschnitt der **zentral bereitgestellten** Backend-Systeme.
Unbenommen davon können gemäß [ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md) auch weitere Backend-Systeme von Teilnehmenden betrieben werden.
In der Konsequenz muss unbenommen von der Anzahl der zentral bereitgestellten Backend-Systeme in allen hier beschriebenen Optionen eine Interoperabilität der Backend-Systeme auf Basis der in [ADR-0011](./ADR-0011-Kommunikationsschicht.md) und [ADR-0012](./ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md) definierten Protokolle für die Kommunikations- und Ende-zu-Ende-Verschlüsselungsschicht gewährleistet sein.

In Abgrenzung zu [ADR-0007](./ADR-0007-Implementierung-von-Postfach-Backends.md) tritt der vorliegende ADR keine Aussagen zu genutzten Implementierungen für die Backend-Systeme.

## Entscheidungstreiber

- Die parallele Entwicklung, Wartung und der parallele Betrieb von funktional sehr ähnlichen Software-Lösungen durch die öffentliche Hand führt zu einer ineffektiven und ineffizienten Ressourcenverwendung. 
- Die parallele Entwicklung, Wartung und der parallele Betrieb von funktional sehr ähnlichen Software-Lösungen führt zu ungerechtfertigter Komplexität in der IT-Landschaft, was deren Beherrschbarkeit, Verständlichkeit und Zukunftsfähigkeit beeinträchtigt. Teil des vorliegenden Auftrages entsprechend [B-2024/28-IT](https://www.it-planungsrat.de/beschluss/beschluss-2024-28) ist die Reduzierung der Komplexität in der IT-Landschaft.
- Konkurrieren verschiedene [Nutzer:innen]-Gruppen um die Umsetzung gruppenspezifischer Anforderungen innerhalb einer Software-Lösung, kann sich diese im Vergleich zu spezialisierten Lösungen als weniger zweckdienlich erweisen. 
- Entsprechend gestaltete Softwareprodukte können durch Konfiguration auf die Bedarfe verschiedener [Nutzer:innen]-Gruppen angepasst werden (z.B. durch aktivieren/deaktivieren einzelner Funktionen, Anzeige/Ausblenden einzelner Elementen in GUIs, …).

## Betrachtete Optionen

- [Option 1: Keine zentral bereitgestellten Backend-Systeme](#option-1-keine-zentral-bereitgestellten-backend-systeme)
- [Option 2: Jeweils ein zentral bereitgestelltes Backend-System für [Privatpersonen], für [private Organisationen] und für [öffentliche Stellen]](#option-2-jeweils-ein-zentral-bereitgestelltes-backend-system-für-privatpersonen-für-private-organisationen-und-für-öffentliche-stellen)
- [Option 3: Ein zentral bereitgestelltes Backend-System für alle [Nutzer:innen]](#option-3-ein-zentral-bereitgestelltes-backend-system-für-alle-nutzerinnen)

## Gegenüberstellung der jeweiligen Optionen

### Option 1: Keine zentral bereitgestellten Backend-Systeme

Es werden keine Backend-Systeme zentral bereitgestellt.
In Folge müssen alle Nutzer:innen selbst eigene Backend-Systeme bereitstellen.

- Gut, da keine Bedarfe an Ressourcen und Strukturen an zentraler Stelle entstehen.
- Schlecht, da [Nutzer:innen] zwingend eigene Backend-Systeme betreiben müssen, was gerade für kleinere Organisationen und Privatpersonen eine massive Nutzungshürde darstellt.
- Schlecht, da die Akzeptanz insb. bei [Privatpersonen] und [privaten Organisationen] massiv leiden wird, wenn keine staatlich bereitgestellte und kostenfrei nutzbare Backend-Infrastruktur bereitsteht.

### Option 2: Jeweils ein zentral bereitgestelltes Backend-System für [Privatpersonen], für [private Organisationen] und für [öffentliche Stellen]

Es werden drei Backend-Systeme zentral bereitgestellt: Ein dediziertes Backend-System für [Privatpersonen], ein dediziertes Backend-System für private Organisationen und ein dediziertes Backend-System für [öffentliche Stellen].

- Gut, da sich aus der Aufteilung in verschiedene Backend-Systeme eine klare organisatorische Rollentrennung ergibt (klare Zuständigkeiten, zielgruppenspezifische Systeme und Anbindungsprozesse pro Zielgruppe)
- Gut, da die Aufteilung eine vereinfachte einheitliche Realisierung spezifischer Datenschutzvorgaben (z.B. Löschfristen) für bestimmte Nutzendengruppen und eine klare Umsetzung von IT-Sicherheitsmechanismen (z.B. rollenspezifische Zugriffsberechtigungen auf einzelne IT-Systeme) ermöglicht.
- Neutral, da durch die grobe Trennung der Nutzendengruppen ggf. nicht allen Souveränitätsansprüchen genüge getan werden kann, insb. zwischen Staatsgewalten (Justiz, Exekutive) und von fachspezifischen Sonderrollen (z.B. Steuerberater:innen, Notar:innen, Ärzt:innen, …). Diese Nutzenden müssten sich entscheiden, entweder auf die zentral bereitgestellten Backend-Systeme zurückzugreifen oder (ggf. gemeinschaftlich organisiert) eigenen Backend-Systeme entsprechend [ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md) zu betreiben.
- Schlecht, da alle bereitgestellten Backend-Systeme eine ähnliche Funktionalität bereitstellen müssen (insb. Implementierung Matrix- und MLS-Standard)
- Schlecht, da durch die Aufteilung redundante und abweichende Governance-Strukturen entstehen können, welche zu Mehraufwänden und erhöhtem Ressourceneinsatz (insb. Abstimmungsaufwände, Betriebskosten und Weiterentwicklungsaufwände) führen.

### Option 3: Ein zentral bereitgestelltes Backend-System für alle [Nutzer:innen]

Es wird genau ein Backend-System für alle Nutzer:innen bereitgestellt.

- Gut, aufgrund niedriger Komplexität der Governance
- Gut, da Zentralisierung Redundanzen im Einsatz von Ressourcen (Geld, Personal, Aufmerksamkeit) und Strukturen (Prozesse, Wissen, Methoden) vermeidet
- Gut, da die Bündelung der Postfach- und Kommunikationsfunktionalitäten in einem Backend-System natürlicherweise natürlicherweise eine Trennung zwischen Basiskomponenten für Kommunikation und Identifizierung herbeiführt
- Neutral, da durch die grobe Trennung der Nutzendengruppen ggf. nicht allen Souveränitätsansprüchen genüge getan werden kann, insb. zwischen Staatsgewalten (Justiz, Exekutive) und von fachspezifischen Sonderrollen (z.B. Steuerberater:innen, Notar:innen, Ärzt:innen, …). Diese Nutzenden müssten sich entscheiden, entweder auf die zentral bereitgestellten Backend-Systeme zurückzugreifen oder (ggf. gemeinschaftlich organisiert) eigenen Backend-Systeme entsprechend [ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md) zu betreiben.

## Architekturentscheidung

Gewählte Option: Option 3 (Ein zentral bereitgestelltes Backend-System für alle [Nutzer:innen]), da es zum jetzigen Zeitpunkt keine ausreichenden Argumente für die Bereitstellung mehrerer Backends gibt. Das nachträgliche Bereitstellen von weiteren Backends ist jederzeit möglich. Bei neuen Sachgründen dafür sollte dies daher erneut betrachtet werden. Insbesondere könnte ein separates Backend-System für Akteure der Justiz erwogen werden.

### Auswirkungen

Die Zusammenführung bestehender zentral bereitgestellter Backend-Systeme erfordert eine Neuordnung von Zuständigkeiten und Anpassung von Governance-Strukturen im operativen Produktmanagement der betroffenen Bestandslösungen.