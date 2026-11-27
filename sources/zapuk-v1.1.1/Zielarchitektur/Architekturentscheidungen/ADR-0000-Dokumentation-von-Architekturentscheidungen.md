> **Status:** ✔️ Operativ entschieden | **Datum:** 15. Januar 2025


# ADR-0000: Dokumentation von Architekturentscheidungen

## Kontext und Problembeschreibung

Die mit Blick auf eine föderale Zielarchitektur für Postfach- und Kommunikationslösungen getroffenen Architekturentscheidungen müssen formell festgehalten werden, insbesondere wenn diese  die Architektur im engeren Sinne ("architectural decision record", ADR) oder andere grundlegende Aspekte betreffen. 

Dieser ADR regelt, welchem Format und welcher inhaltlichen Struktur diese Aufzeichnungen folgen sollen.

## Entscheidungstreiber

- Implizite Annahmen in Entscheidungsprozessen sollten durch transparente Dokumentation von Entscheidungstreibern explizit gemacht werden.
- Die Dokumentation von Design- und Entscheidungsprozessen sowie zentralen Argumenten ist wichtig, damit ein ausreichend qualifizierter Beobachter die Entscheidungen zu einem späteren Zeitpunkt nachvollziehen kann.
- Die transparente Dokumentation von insbesondere verworfenen Handlungsoptionen schafft Vertrauen und Legitimität, insbesondere bei betroffenen Parteien. 
- Für weitere Ausführungen siehe auch: ["A rational design process: How and why to fake it"](https://www.cs.tufts.edu/~nr/cs257/archive/david-parnas/fake-it.pdf)

## Betrachtete Optionen

- Option 1: MADR 4.0.0 - The Markdown Architectural Decision Records
- Option 2: Michael Nygards ADR-Vorlage - Die erste Inkarnation des Begriffs "ADR"
- Option 3: Sustainable Architectural Decisions - The Y-Statements
- Option 4: Eine andere der [hier](https://github.com/joelparkerhenderson/architecture_decision_record) aufgelisteten ADR-Vorlagen
- Option 5: Formlos - Keine Konventionen für Dateiformat und Struktur

## Architekturentscheidung
Gewählte Option: **Option 1: MADR 4.0.0**, weil

- MADR ermöglicht die strukturierte Erfassung verschiedenartiger Entscheidungen.
- Das MADR-Format ist schlank und passt zu unserem Arbeitsstil
- Die MADR-Struktur ist nachvollziehbar und erleichtert die langfristige Nutzung und Pflege
- Das MADR-Projekt ist aktiv MADR ein verbreitetes Dokumentationsformat für ADRs 
