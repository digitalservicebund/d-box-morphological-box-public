# ZaPuK: Transitionsbetrachtung B – Mein Justizpostfach (Entwurf v1.1)

Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/ef99118c0607dd7fb11a005593569a36a0f84343/Zielarchitektur/Transitionen/B--Mein-Justizpostfach/README.md
Branch: architecture-v1.1 (Entwurf v1.1, noch nicht in main), Commit ef99118c06 (2026-11-13)
Abgerufen: 2026-11-19 über die GitLab-API

Primärquelle (FITKO / FIT-AB, Entwurfsstand). Inhalt unverändert ab der Linie unten; Bildverweise zeigen auf das Repository. Zeilenangaben in _befunde.md beziehen sich auf diese Datei.

---

# Föderale Zielarchitektur für Postfach- und Kommunikationslösungen

***Bauplan einer modernen Kommunikationsinfrastruktur für Privatpersonen, private Organisationen und öffentliche Stellen***

**Version 1.1** &ndash; **Stand: 25.09.2026** &ndash; **Anhang B: Transitionsplan Mein Justizpostfach**

::TODO
> Dieses Dokument ist die maschinenlesbare Textfassung des vom IT-Planungsrat im PDF-Format verabschiedeten Dokuments ["Zielarchitektur" in Version 1.0](./2025-06-26_Beschluss-2025-28_ZaPuK_Architekturkonzept.pdf). Bei der Überführung wurden redaktionelle Änderungen für eine bessere Lesbarkeit und Anpassung an die openCode-Struktur vorgenommen.

------

## Nutzungsbedingungen

Die Inhalte dieses Dokumentes unterliegen der [Creative Commons Namensnennung 4.0 International Public License (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).

------

## 1. Zweck dieses Dokuments

Dieses Dokument enthält den Plan für die Transition von "Mein Justizpostfach" (MJP) auf die Zielarchitektur Postfach- und Kommunikationslösungen (ZaPuK). Es beschreibt das MJP-spezifische Zielbild nach Abschluss aller Transitionsarbeiten, den Ablauf der einzelnen Transitionsphasen inkl. konkreter Maßnahmen sowie die Verantwortlichkeiten für deren Umsetzung. Der Transitionsplan benennt die ungefähre Dauer der einzelnen Phasen. Eine konkrete Terminierung und verbindliche Festlegungen werden in Abstimmung zwischen der Bundes-Länder-Kommission für Informationstechnik in der Justiz (BLK Justiz), vertreten durch das Ministerium der Justiz und für Migration Baden-Württemberg als Betreiber:in des MJP und dem Programm ZaPuK festgelegt.

Es handelt sich bei der vorliegenden Fassung um ein lebendiges Dokument, welches im weiteren Verlauf der Transitionsgespräche bei Bedarf fortgeschrieben wird.

## 2. Transitions-Zielbild

Das zentrale Postfach-Frontend für Privatpersonen und Organisationen ersetzt gemäß ZaPuK-Architekturkonzept die bisherigen Postfach-Frontends in BundID, MJP und in Mein Unternehmenskonto (MUK). Damit ist keine parallele Weiterentwicklung dieser Frontends mehr erforderlich. BundID, MJP und MUK sollten nach Abschaltung der jeweiligen Postfach-UIs für einen Übergangszeitraum auf das zentrale Postfach-Frontend im Bundesportal verlinken.

Privatpersonen erhalten ihre MJP-Nachrichten zukünftig im zentralen Postfach-Frontend. Das zentrale Postfach-Frontend ist zu diesem Zweck in der Lage, Nachrichten aus dem elektronisches Gerichts- und Verwaltungspostfach (EGVP) zu ver- und entschlüsseln durch Ende-zu-Ende-Verschlüsselung (E2EE). Hierzu werden die bestehenden Zertifikate (.p12-Datei) im zentralen Postfach-Frontend genutzt.

Ein durch FIT-Connect betriebenes zentrales Postfach-Backend für Justiz-Organisationen ersetzt das bisherige MJP-Backend. Damit ist zukünftig keine Weiterentwicklung und insb. keine funktionale Erweiterung des aktuellen MJP mehr erforderlich. Vorhandene Daten können aus dem MJP-Backend in das Postfach-Backend für Privatpersonen migriert werden. Die EGVP-Infrastruktur bleibt unverändert bestehen. Für die Anbindung des EGVP-Verbundes an das Postfach-Backend wird ein ebenfalls durch FIT -Connect betriebener Protokollkonverter als vermittelnde Backend-Komponente geschaffen. Dieser ermöglicht einen bidirektionalen Datenaustausch zwischen OSCI-Intermediär und Matrix-Backend. Im Gegensatz zu einer Bridge realisiert ein Protokollkonverter die Vermittlung von Nachrichten zwischen zwei Transportprotokollen (hier: Matrix und OSCI) ohne Aufbrechen der Ende-zu-Ende-Verschlüsselung. Organisationen der Justiz sind somit ohne Anpassungsaufwände an eigenen Systemen über EGVP und den Protokollkonverter an das Postfach-Backend für Justiz-Organisationen angeschlossen und in der Lage, Nachrichten an Privatpersonen zuzustellen.

In der EGVP-Infrastruktur wird jede:r Nutzende über eine eindeutige SAFE-ID identifiziert. Innerhalb von ZaPuK wird diese Logik fortgeführt, indem sich justizbezogene Zieladressen aus zwei logischen Bausteinen zusammensetzen: der individuellen SAFE-ID (nutzendenspezifischer Teil) und der Domain des Matrix-Backends. Die Adressen folgen damit dem Format `@[SAFE-ID]:matrix.example.com`. Dies hat u. a. den Vorteil, dass eine erneute Verzeichnisabfrage in einer separaten Datenbank entfällt. Stattdessen entnimmt die übermittelnde Schnittstelle (hier: Protokollkonverter) die SAFE-ID direkt aus der eingehenden Nachricht, verknüpft sie vollautomatisch mit der Domain des Matrix-Backends für Privatpersonen und ermittelt somit ohne Verzögerung die exakte Zieladresse. Im Ergebnis entsteht eine schnelle, fehlerresistente Nachrichtenübermittlung, die ohne den Aufbau und Betrieb eines zusätzlichen Adressverzeichnisses neben dem bestehenden SAFE-Verzeichnis auskommt.

Die nachfolgende Abbildung visualisiert das in der Ziel-Bebauung angestrebte Zusammenspiel der Komponenten der Zielarchitektur.

![Übersicht über das Zusammenspiel der Postfach-Backends mit dem Frontend, Authentifierzungslösungen und weiteren Komponenten und Umsystemen.](images/image1.png)

> _Abbildung 1: Zusammenspiel der Komponenten der Zielarchitektur_

## 3. Transitionsphasen

Die Durchführung der Transition & Migration in der gleichnamigen Programmphase III des ZaPuK-Programms erfolgt in vier Transitionsphasen, die aufeinander aufbauen, sich jedoch zeitlich überlappen können.

| Phase                                               | Inhalt                                                                                                                                                                                                                                                                                                                        | Vorbedingung                                                                                                                | Zeitlicher Rahmen |
| --------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- | ----------------- |
| **A**: Aufsetzen des Transitionsprojektes          | Aufsetzen des Transitionsprojektes inkl. Meilensteinplanung, Allokation nötiger Finanzmittel, Durchführung ggf. erforderlicher Beauftragungen<br><br>**Ziel**: Transitionsprojekt ist aufgesetzt und handlungsfähig                                                                                                           | Abschluss ZaPuK-Phase II                                                                                                    | ca. 3-6 Monate    |
| **B**: Transitionsvorbereitung                     | Vorbereitung der Betriebsumgebung, Abschluss von Entwicklungsarbeiten, Produktivsetzung erforderlicher Backend-Komponenten & zentrales Postfach-Frontend, Qualitätssicherung, Einrichtung der Support-Prozesse, Migrationsvorbereitung<br><br>**Ziel**: Komponenten sind produktiv einsetzbar (Migration noch nicht erfolgt). | Erforderliche Beauftragungen abgeschlossen                                                                                  | ca. 6-8 Monate    |
| **C**: Migration                                    | Durchführung von Migrationen<br><br>**Ziel**: Zustellung von Nachrichten via OSCI-Intermediär an ZaPuK-Infrastruktur möglich (bidirektional)                                                                                                                                                                                  | Qualitätssicherungstests waren erfolgreich, zentrales Postfach-Frontend in Betrieb, Postfach-Backend für Justiz in Betrieb | ca. 1-2 Monate    |
| **D**: Abschaltung Altsysteme (Dekommissionierung) | Außerbetriebnahme MJP-Frontend und -Backend (Abschaltung EGVP-Infrastruktur ist nicht geplant)<br><br>**Ziel**: Infrastruktur ist konsolidiert, Kostenersparnisse durch Abschaltung MJP-Backend und MJP-Frontend                                                                                                              | Übermittlung der Nachrichten erfolgt zwischen EGVP-Verbund und ZaPuK-Verbund, EGVP-Infrastruktur läuft unverändert weiter   | nach 2 Monaten    |

> _Tabelle 1: Transitionsphasen A – D_

## 4. Detaillierte Phasenbeschreibung und Maßnahmenkatalog

Nachfolgend werden die Phasen A-D inkl. Maßnahmen, Verantwortlichkeiten, Voraussetzungen und zu erwartende Ergebnisse beschrieben. Dabei ist zu beachten, dass insb. die Auflistung der Maßnahmen noch nicht final ausgestaltet ist und sich durch die Transitionsgespräche Änderungen ergeben können.

### 4.1 Phase A: Aufsetzen des Transitionsprojektes

Phase A dient notwendigen Vorbereitungen wie dem Aufsetzen eines Transitionsprojektes, der Allokation nötiger Finanzmittel in den jeweiligen Haushalten der für die Projektverantwortlichen Organisationen (hier: Arbeitsgruppe IT-Standards in der Justiz der Bund-Länder-Kommission für Informationstechnik in der Justiz (BLK), vertreten durch das Ministerium der Justiz und für Migration Baden-Württemberg) sowie der Durchführung von ggf. erforderlicher Beauftragungen.

Eine Mittelbereitstellung ist nicht Teil dieser Phase, sondern erfolgt vorab im Rahmen von ZaPuK-Programmphase II. Im Rahmen der Aufplanung des Transitionsprojektes erfolgt in Abstimmung mit dem ZaPuK-Programm eine Meilensteinplanung. Nach Abschluss dieser Phase ist das Transitionsprojekt aufgesetzt und handlungsfähig.

Wichtigste Vorbedingung für die MJP-seitige Transition ist die Entwicklung und Bereitstellung der zentralen Front- und Backendkomponenten. Diese werden in eigens dafür vorgesehenen Einzelprojekten zusammen mit FIT-Connect entwickelt und stellen eine direkte Vorbedingung für die erfolgreiche Durchführung des MJP-Transitionsplans dar. Vorarbeiten können jedoch auch schon vor der Fertigstellung dieser Komponenten starten.

| Nr. | Maßnahme                                                                                                                                                                                                    | Verantwortlich                                                     | Voraussetzung               | Ergebnis                     |
| --- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ | ---------------------------- | ---------------------------- |
| A1  | Prüfung Vorbedingungen, Meilensteinplanung                                                                                                                                                                  | MJP-Team (inkl. DL – Governikus), in Abstimmung mit ZaPuK-Programm | Projektsteckbrief liegt vor | abgestimmter Meilensteinplan |
| A2  | Finanzierung und bedarfsgerechte Beauftragungen (Inhouse, Rahmenvertrag oder Vergabe) für Migrationstätigkeiten und ggf. notwendige Anpassung bestehender Support-, Datenschutz- und IT-Sicherheitskonzepte | MJP-Team                                                           | A1                           | erfolgte Beauftragungen      |

> _Tabelle 2: Übersicht Phase A_

### 4.2 Phase B: Transitionsvorbereitung

In Phase B werden Betriebsumgebungen aufgebaut, erforderliche Entwicklungsarbeiten abgeschlossen sowie Qualitätssicherungsmaßnahmen wie Last- und Performancetests, Barrierefreiheitstests durchgeführt. Des Weiteren werden Support-Prozesse aufgesetzt.

Zur Migrationsvorbereitung werden entsprechende Migrationsskripte zur Datenmigration erstellt und anschließend in Migrationstests auf korrekte Funktionalität geprüft.

Die Komponenten des Backends für Privatpersonen sowie das zentrale Postfach-Frontend werden in Produktivumgebungen betrieben, sodass diese jederzeit produktiv geschaltet werden können. Nachdem die im Zuge der ZBP/BundID-Transitionsplanung entwickelte Frontendkomponente zur Verfügung gestellt wurde, müssen für das MJP primär der Protokollkonverter, das Justiz-Backend und der OSCI-Verschlüsselungsstack im Frontend implementiert werden.

| Nr.     | Maßnahme                                                                                                                                              | Verantwortlich                                                    | Voraussetzung                 | Ergebnis                                                                                                                                         |
| ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| B1      | Fertigstellung Software für zentralen Postfach-Client (Web und App)<br><br>inkl. Funktionalität zur E2EE von EGVP-Nachrichten und Schlüsselverwaltung | ZaPuK                                                             | Nutzung Neo-Arbeitsergebnisse | Funktionaler Postfach-Client                                                                                                                     |
| B2      | Fertigstellung Postfach-Backend für Privatpersonen                                                                                                    | ZaPuK                                                             | Nutzung Neo-Arbeitsergebnisse | Funktionales Backend                                                                                                                             |
| B3      | Fertigstellung Postfach-Backend für Justiz-Organisationen                                                                                             | ZaPuK                                                             | Nutzung Neo-Arbeitsergebnisse | Funktionales Backend                                                                                                                             |
| B4      | Fertigstellung Protokollkonverter (OSCI ↔ Matrix)                                                                                                     | ZaPuK                                                             | Nutzung Neo-Arbeitsergebnisse | Protokollkonverter                                                                                                                               |
| B5      | Anpassung Betriebs-, Support-, Datenschutz- und IT-Grundschutz-Konzepte                                                                               | MJP-Team                                                          |                               | Konzepte erstellt und abgenommen                                                                                                                 |
| B6      | Betriebsvorbereitung für neue Infrastruktur                                                                                                           | BMDS, FIT-Connect                                                 |                               | Betriebsumgebung für Postfach-Frontend, Postfach-Backend und Protokollkonverter aufgebaut                                                       |
| B7      | Erstellung Last- und Skalierungskonzept anhand Mengengerüsts                                                                                          | ZaPuK                                                             |                               | Last- und Skalierungskonzept                                                                                                                     |
| B8      | Durchführung Last-/Performancetests auf der Zielumgebung                                                                                              | ZaPuK, MJP-Team                                                   | B6, B7                        | Technische Skalierung verprobt                                                                                                                   |
| B9      | Erstellung Migrationsskripte                                                                                                                          | MJP-Team gemeinsam mit ZaPuK                                      | Nutzung Neo-Arbeitsergebnisse | Migrationsskripte                                                                                                                                |
| B<br>10 | Durchführung von Migrationstests                                                                                                                      | MJP-Team                                                          | B9                            | Erfolgreicher Abschluss Migrationstests auf Testdaten                                                                                            |
| B<br>11 | Kommunikationsmaßnahmen MJP-Nutzende                                                                                                                  | MJP-Team                                                          |                               | TBD                                                                                                                                              |
| B<br>12 | Etablierung eines produktübergreifenden Wissensmanagements in Support-Teams                                                                           | MJP-Team, BundID-Team, Bundesportal-Team, D-App-Team, FIT-Connect |                               | Supportstrukturen von BundID, Bundes-portal, D-App, MJP und FIT-Connect sind gleichermaßen auskunftsfähig bei Fragen zur Postfach-Funktionalität |

> _Tabelle 3: Übersicht Phase B_

### 4.3 Phase C: Migration

In Phase C findet die eigentliche Datenmigration und die Live-Schaltung neuer Komponenten für die breite Öffentlichkeit statt. Nach erfolgreicher Migration der Bestandsdaten (Nachrichten, Metadaten) aus dem MJP-Backend in das neue Matrix-Backend wird das alte MJP-Backend abgeschaltet.

Nach Abschluss dieser Arbeiten ist eine bidirektionale Kommunikation zwischen EGVP-Verbundteilnehmenden und Privatpersonen über die ZaPuK-Infrastruktur möglich.

| Nr  | Maßnahme                                                                  | Verantwortlich                     | Voraussetzung | Ergebnis                                                                                                                                                                     |
| --- | ------------------------------------------------------------------------- | ---------------------------------- | ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ‍C1 | Durchführung Datenmigration MJP → Matrix-Backend                          | MJP-Team gemeinsam mit BundID-Team |               | Alle Bestandsdaten (Nachrichten, Daten, Metadaten) sind aus dem MJP-Backend in das Matrix-Backend für Privatpersonen migriert                                                |
| C2  | Weiterleitung aus MJP-Frontend auf neues Postfach-Backend im Bundesportal | MJP-Team                           | C1            | Nutzende werden nach manuellem Aufrufen des MJP-Frontends auf Bundesportal-Frontend verwiesen, Links in E-Mail-Benachrichtigungen verweisen direkt auf Bundesportal-Frontend |

> _Tabelle 4: Übersicht Phase C_

### 4.4 Phase D: Abschaltung Altsysteme (Dekommissionierung)

In Phase D werden in einem letzten Schritt MJP-Backend und -Frontend final abgeschaltet. Jegliche Kommunikation erfolgt fortan über den OSCI-Intermediär für Bürger:innen und den OSCI-Matrix-Protokollkonverter.

| Nr  | Maßnahme                              | Verantwortlich | Voraussetzung                                                         | Ergebnis                 |
| --- | ------------------------------------- | -------------- | --------------------------------------------------------------------- | ------------------------ |
| D1  | Abschaltung MJP-Frontend- und Backend | MJP-Team       | Stabiler Betrieb des neuen Postfach-Frontends und Protokollkonverters | Abschluss der Transition |

> _Tabelle 5: Übersicht Phase D_

### 4.5 Offene Punkte

1. Verantwortung für Ausstellung neuer Zertifikate für Privatpersonen zur Kommunikation mit Justiz
