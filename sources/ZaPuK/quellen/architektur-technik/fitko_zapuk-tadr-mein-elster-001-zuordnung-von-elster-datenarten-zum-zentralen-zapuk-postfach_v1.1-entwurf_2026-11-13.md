<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/ef99118c0607dd7fb11a005593569a36a0f84343/Zielarchitektur/Transitionen/Architekturentscheidungen/TADR-MEIN-ELSTER-001-Zuordnung-von-ELSTER-Datenarten-zum-zentralen-ZaPuK-Postfach.md -->
<!-- Branch: architecture-v1.1, Commit ef99118c06 (2026-11-13); Pfad im Repo: Zielarchitektur/Transitionen/Architekturentscheidungen/TADR-MEIN-ELSTER-001-Zuordnung-von-ELSTER-Datenarten-zum-zentralen-ZaPuK-Postfach.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob d23fd72f49); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# TADR-MEIN-ELSTER-001: Zuordnung von ELSTER-Datenarten zum zentralen ZaPuK-Postfach

> **Status:** ✔️ Operativ entschieden | **Datum:** 25.09.2026

## Kontext und Problembeschreibung

Die über das ELSTER-Postfach im Portal "Mein ELSTER" abgewickelten Nachrichten sind sehr heterogen. Die Spezifikation des ELSTER-Postfachs kennt mehr als einhundert eindeutige Datenarten für Nachrichten. Ein Teil davon wird maschinell verarbeitet, ein weiterer Teil, wie z. B. Mitteilungen und Bescheide, wird lediglich für den Abruf durch eine:n Nutzer:in bereitgestellt. Die Anforderungen an eine maschinelle Weiterverarbeitung durch Fachlogik im ELSTER-Portal sprechen zwar gegen eine generelle Zustellung außerhalb des Portals. Bei Datenarten, die lediglich für den Abruf bereitgestellt und im Frontend angezeigt werden, ist eine Bereitstellung im zentralen Postfach der Zielarchitektur für Postfach- und Kommunikationslösungen (ZaPuK) dagegen möglich und sinnvoll. So wäre eine Konsolidierung der Postfachsysteme zur rechtssicheren Bekanntgabe von Bescheiden technisch und rechtlich möglich (vgl. [§ 122a Abs. 5 AO](https://www.gesetze-im-internet.de/ao_1977/__122a.html)).

## Entscheidungstreiber

- Die domänenspezifische Fachlogik soll nicht in den Bausteinen der Zielarchitektur (ZaPuK) umgesetzt werden (vgl. Architecture Decision Record [ADR-0001](/Zielarchitektur/Architekturentscheidungen/ADR-0001-Querschnittlichkeit-und-Erweiterbarkeit-der-Infrastruktur.md), [ADR-0002](/Zielarchitektur/Architekturentscheidungen/ADR-0002-Infrastrukturtopologie.md)).
- Eine rechtssichere Bekanntgabe (z. B. von Bescheiden) von der Verwaltung an Bürger:innen und Organisationen ist Kernanforderung der Zielarchitektur ([ANF_FUN_USE_063](/Konsolidierte-Anforderungen/README.md#thema-use-cases), [RAH_ÜBE_144](/Konsolidierte-Anforderungen/README.md#thema-übergreifend)).

## Betrachtete Optionen

### Option A: Gesamtbetrachtung ohne Zerlegung

In dieser Option wird die Postfach-Komponente als Ganzes betrachtet. Ein Aufteilen der Nachrichten entsprechend definierter Kriterien findet nicht statt. Alle Nachrichtenarten würden über eine Postfachlösung abgewickelt.

- Gut: Kein Aufwand bei der Kategorisierung und Analyse
- Gut: Kein Aufwand bei der Anbindung für die Entscheidungslogik in ELSTER, welche Nachrichten nun extern bereitgestellt werden müssen
- Schlecht: Die Vermischung des Datenaustauschs mit Systemen (Daten für Software, Institutionen, Statistiken) und der Kommunikation mit Menschen (Bescheide, Mitteilungen) bliebe bestehen. Eine sinnvolle Trennung bei der Darstellung bzw. Verarbeitung wird damit verhindert bzw. erschwert (siehe auch Entwurfsprinzip "Separation of concerns"). Dies ist eine Hürde für die Überführung der an Menschen gerichteten Nachrichten in ein zentrales Postfach.
- Schlecht: Die Überführung der für Menschen (Steuerpflichtige) interessanten Nachrichten in ein zentrales Bürger:innen-Postfach wäre sonst nur mit allen Nachrichtentypen oder gar nicht möglich.
- Schlecht: Ohne Unterscheidung nach Datenarten müssen alle Nachrichten gleichbehandelt werden. Nachrichten, die in den Systemen zu verarbeiten sind, verhindern dann die Ende-zu-Ende-Verschlüsselung auch für Mitteilungen, die direkt von Endgerät zu Endgerät übermittelt werden. Umgekehrt wäre die maschinelle Verarbeitung nicht mehr möglich, wenn eine Lösung die Ende-zu-Ende-Verschlüsselung für alle Nachrichten umsetzen würde.

### Option B: Zerlegung in Kommunikationsklassen, Routing je Datenart

Der Nachrichtenbestand wird über die ELSTER-Datenarten aufgeteilt. Als vereinfachte Regel kann gelten: Werden die übermittelten Daten durch Menschen gelesen, werden diese Nachrichten oder Nachrichtenbestandteile im zentralen ZaPuK-Postfach bereitgestellt. Je nach Datenart ergibt sich somit eine Eignung für die Bereitstellung. Die Bereitstellung erfolgt im zentralen Postfach-Backend für Privatpersonen (ehem. ZBP) oder im Postfach-Backend für Organisationen.

| Nachrichtentyp                                 | Inhalt                                       | Beispiele (Datenart)               | Postfachlösung                             |
| ---------------------------------------------- | -------------------------------------------- | ---------------------------------- | ------------------------------------------ |
| Rechtsverbindliche Bereitstellung für Personen | Bescheide/VA, Dokumente, menschliche Nutzung | EPBescheid\*, VerbindlicheAuskunft | Zentrales Postach/ZaPuK                    |
| Dialog im Verfahren                            | Mitteilungen, Rückfragen, Antworten          | EPMitteilung, EPAntwort            | Zentrales Postfach /ZaPuK                  |
| Statusmeldungen                                | Rückmeldungen zu Einreichungen               | AenderungAdresseRM, ELOProtokoll   | ELSTER, optional Benachrichtigung in ZaPuK |
| Bescheiddaten an Software                      | maschinell, ohne Bekanntgabewirkung          | ESB, GSB                           | ELSTER                                     |
| M2M an Institutionen                           | Kommunen, Kammern, Kirchen, Statistik        | GMB\*, GEW\*                       | ELSTER                                     |

> _Tabelle 1: Zuordnung der ELSTER-Nachrichtentypen zu den führenden Postfachlösungen_  
> _\*Datentyp aus der nur intern vorliegenden Dokumententation "Nachrichten_an_ELSTER_Kunden.xlsx" des LfSt Bayern, siehe [Referenzen](#referenzen)_

- Gut: Eine Trennung der Nachrichten entsprechend der Verarbeitungsart (maschinell/persönlich) ist möglich. Nachrichten, die lediglich angezeigt und nicht innerhalb ELSTER weiterverarbeitet werden, können über eine externe Postfachlösung bereitgestellt werden.
- Gut: Eine Transition kann schrittweise erfolgen, denn die einzelnen Datenarten können unabhängig voneinander in unterschiedlichen Systemen zugestellt werden.
- Gut: Bestehende Funktionalität im Portal Mein ELSTER, die auf spezifischen Nachrichten des Postfachs basiert, kann wie gehabt weiterhin erhalten bleiben.
- Gut: Eine strikte Ende-zu-Ende-Verschlüsselung für eine direkte Kommunikation zwischen den Endgeräten der Kommunikationsteilnehmer wird möglich. Ein Betreiber kann die Nachrichteninhalte dieser Kommunikation nicht einsehen.
- Schlecht: In ELSTER muss eine Logik für das Routing der Nachrichten nach Datenart umgesetzt werden.
- Schlecht: Die Umstellung muss den Nutzenden klar kommuniziert werden.

## Architekturentscheidung

**Empfohlen wird Option B: Zerlegung in Kommunikationsklassen, Routing je Datenart**, weil eine Unterscheidung nach der Art der Verarbeitung anhand der in ELSTER bereits definierten "Datenarten" das fachliche Herauslösen von Nachrichten ermöglicht, die für die Bereitstellung in einem zentralen Postfach der Zielarchitektur (ZaPuK) geeignet sind.

Das Verfahren ELSTER wird in der Folge eine eigene Einschätzung vornehmen, die sowohl die technischen Aspekte einer Umstellung mit betrachtet als auch die für eine Umsetzung erforderlichen Aufwände berücksichtigt.

### Auswirkungen

- Eine genaue Aufteilung der Nachrichtentypen (Datenarten) für die Eignung der Bereitstellung im zentralen Postfach-Backend für Privatpersonen muss entworfen werden.
- Eine Nachricht wird in genau einer Postfachlösung bereitgestellt. Wo ein Hinweis in der jeweils anderen Lösung sinnvoll ist, erfolgt er als Benachrichtigung ohne Inhalt.
- Eine Bereitstellung im zentralen Postfach der ZaPuK setzt ein aktives Postfach der empfangenden Person voraus.
- Die Aufteilung in die Postfächer muss für die Nutzenden erkennbar sein. Ohne verlässliche Benachrichtigung besteht das Risiko, dass Fristen laufen, während die Person am falschen Ort sucht.

### Offene Punkte

- Zuordnung der Datenarten durch BayLfSt zur Eignung nach Option B (ggf. Analyse und Verfeinerung des Konzeptes bei Datenarten mit mehreren Medienarten wie Anhängen und Text)
- Mengengerüst je Datenart

### Referenzen

- "Nachrichten_an_ELSTER_Kunden.xlsx" übermittelt am 24.6.2026 vom LfSt Bayern _(nicht-öffentlich)_
- [Integrationsleitfaden Postfach 2.0 v9.0](https://service.mein-unternehmenskonto.de/api/downloads/integrationsleitfaden_postfach) (Stand 04/2026)
- [Konzeptpapier Postfachvereinheitlichung](https://www.it-planungsrat.de/fileadmin/beschluesse/2024/Beschluss2024_04_AL_Konzeptpapier_Postfachvereinheitlichung.pdf) (Anlage zum Beschluss [B-2024/04-AL](https://www.it-planungsrat.de/beschluss/beschluss-2024-04-al) der AL-Runde)
- Vgl. [ADR-0001](/Zielarchitektur/Architekturentscheidungen/ADR-0001-Querschnittlichkeit-und-Erweiterbarkeit-der-Infrastruktur.md), [ADR-0002](/Zielarchitektur/Architekturentscheidungen/ADR-0002-Infrastrukturtopologie.md), [ADR-1103](/Zielarchitektur/Architekturentscheidungen/ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)
