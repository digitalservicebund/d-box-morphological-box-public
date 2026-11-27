# ZaPuK: Aktualisierung auf v1.1.1

Stand: 2026-11-27. Die Quelle ist die lokale Kopie `quellen/zapuk-v1.1.1/` (Tag v1.1.1, <https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/tree/v1.1.1>). Sie ersetzt `../zapuk-v1.0.0-121-g7419d8b/` sowie die Kopien von `main` (12.11.2026) und vom Branch `architecture-v1.1` (13.11.2026).

## Was sich bei ZaPuK geändert hat

- **Beschluss B-2026/43-IT vom 25.11.2026:** Der IT-Planungsrat sieht Phase II als abgeschlossen an (Ziff. 1). Der Bund bestätigt die Programmverantwortung nach B-2026/18-IT (Ziff. 2). Die v1.1-Dokumente sind Anlagen, aber nur vorgelegt, nicht beschlossen (README v1.1.1 Z. 49, Z. 76–77). Ablage: `quellen/recht-governance/IT-PLR_B-2026-43-IT_ZaPuK-Abschluss-Phase-II.md`.
- **Nicht beschlossen:** die verbindliche Konsolidierung, Phase III unter Federführung des BMDS, ein Auftrag für Gesetz oder Staatsvertrag und die Finanzierung. Das Finanzierungskonzept kommt erst zur 52. Sitzung.
- **ADR-1122 Benachrichtigungsdienst (In Arbeit):**
  - Das Empfänger-Backend benachrichtigt per E-Mail oder Push nach den Einstellungen der Person, je Vorgang abschaltbar.
  - Der Text ist einheitlich, ohne Bereich, Behörde oder Vorgang.
  - Die Behörde kann zusätzlich selbst benachrichtigen. Eine Benachrichtigung über das Nutzerkonto ist ausgeschlossen.
  - Auf Anforderung gibt es eine Benachrichtigungsbestätigung. Sie belegt die Absendung, nicht den Zugang.
- **ADR-1121 Abrufbestätigung (In Arbeit):** Die Bestätigung kommt vom Client, vom Backend oder von beiden. Vorerst gilt die Bestätigung durch den Client.
- **Weitere ADR-Änderungen:**
  - ADR-0018 Z. 9 spricht jetzt vom Eingang („dokumentiert diesen Eingang“), lässt den Zugang aber weiter mit dem Eingang eintreten. ADR-1118 Z. 102 spricht vom „Nachweis der Bereitstellung“. Nur das Glossar (Z. 231) sagt, dass die Bestätigung den Zugang nicht belegt.
  - ADR-1118: Der offene Punkt „Nachweis der Benachrichtigung“ ist entfallen (jetzt ADR-1122, ADR-1118 nennt ADR-1122 nicht). Neue Folge-Entscheidung ADR-1121 (Z. 135). Neues Risiko: Ohne Anpassung von § 9 OZG gilt der tatsächliche Abruf (Z. 127).
  - ADR-1119: neue Option „Benachrichtigungsbestätigung anfordern“.
  - Kleinere Änderungen in ADR-1105, 1110, 1135, 1145, 1146 und 1147.
  - Der Status bleibt: ADR-1105, 1106, 1109 und 1147 „Richtung entschieden“, ADR-1149 unbearbeitet.
- **Architekturkonzept v1.1:** Kapitel 3 Geschäftsarchitektur ist neu ausgeschrieben, ab Kapitel 4 verschieben sich die Zeilen um +82 bis +89. Die Vorversion v1.0 liegt unter `Architekturkonzept/Vorversionen/v1.0/`, die Zusammenfassung v1.0 unter `Zusammenfassung/Vorversionen/v1.0/`, jeweils mit unveränderten Zeilennummern.
- **Rechtsgutachten (Stand 19.10.2026):**
  - Neuer Abschnitt E zum IT-Staatsvertrag. Aus dem IT-Staatsvertrag allein folgt keine verbindliche Anschlusspflicht.
  - Länder und Kommunen lassen sich über das OZG nach Art. 91c Abs. 5 GG oder über einen Staatsvertrag binden.
  - Die Abschnitte A–D sind unverändert.
- **FITKO-Einordnung des Rechtsgutachtens (neu):** Sie trennt Betriebs-, Finanzierungs- und Nutzungsmodell. Ob die Nutzung verbindlich wird, ist politisch zu entscheiden (S. 3).
- **Transitionspläne:** v1.1.1 enthält nur die übergreifende Transitionsstrategie. Die Pläne für ZBP, Mein Justizpostfach und Unternehmenskonto sowie die ELSTER-Transitions-ADRs 001 und 002 fehlen. Der Branch `architecture-v1.1` existiert upstream nicht mehr. Die lokalen Kopien vom 13.11.2026 in `quellen/umsetzung-rezeption/` sind die einzige Quelle für diese Pläne.

## Statusregel in den Dateien

- „v1.1“ heißt: Fassung der Anlagen zu B-2026/43-IT, vorgelegt, nicht beschlossen. Das ersetzt „Entwurf v1.1“. Für die Aussage „v1.1 sieht X vor“ gilt Evidenzstufe 3.
- „Entwurf Transitionsplan“ steht nur noch für Aussagen, die allein auf den Transitionsplänen A–C oder den ELSTER-Transitions-ADRs beruhen.

## Geänderte Dateien

- **`morph_box.csv`:**
  - B1 A3: „Überblick v1.0 und v1.1“. B1 A8: Brücken nach der Transitionsstrategie (ZBP-Matrix-Bridge, ELSTER-Bereitstell-Bridge, Bridge für Postfach 2.0 möglich).
  - B3 A1: „Entwurf Transitionsplan“.
  - P1 A2, A3: ADR-1122. Neu: P1 A8 Benachrichtigung zusätzlich durch die Behörde (ADR-1122 Option B).
  - P5 A3, P6 A3, Ü5 A2, Ü5 A3, I1 A4: „v1.1“.
  - Ü4 A1: Benachrichtigungsbestätigung (ADR-1122). Ü4 A2: Bestätigung durch das Backend erwogen (ADR-1121).
  - R4 A3: „ZaPuK-Ziel laut v1.1, in B-2026/43-IT nicht beschlossen“.
- **`existing_solutions.csv`, Spalte ZaPuK:**
  - P1 und Ü4 nach ADR-1121, ADR-1122 und ADR-0018.
  - I1: Phase II abgeschlossen, Programmverantwortung beim Bund, BMDS nur laut v1.1.
  - R4: Konsolidierung nicht beschlossen, Anschlusspflicht nicht allein über den IT-Staatsvertrag, FITKO-Einordnung.
  - B1, B3, P5, P6, P7, P9, Ü1, Ü5, Ü6: Status „v1.1“ bzw. „Entwurf Transitionsplan“.
- **`../Referenzen-ZaPuK.md`:**
  - Pfade auf `zapuk-v1.1.1/`, alle Fundstellen umgerechnet, die bekannten Zitierfehler korrigiert.
  - Neue Kürzel B43, EIN und PR43, TERM gestrichen.
  - Neuer Abschnitt „Abgleich vom 2026-11-27“.
- **`ZaPuK-Zusammenfassung.md`:** Pfade und Status umgestellt, neuer Abschnitt „Änderungen mit v1.1.1 und B-2026/43-IT“.
- **`quellen/*/_index.md`, `quellen/*/_befunde.md`:** v1.1.1 als Hauptquelle, je eine Konkordanz alt → v1.1.1. Ablage:
  - `architektur-technik/_befunde.md`, Abschnitt „Konkordanz der Fundstellen (alt → v1.1.1)“
  - `recht-governance/_befunde.md`, Abschnitt „Konkordanz“
  - `umsetzung-rezeption/_befunde.md`, Abschnitt „Konkordanz“
- **Neu in `quellen/recht-governance/`:** Beschluss B-2026/43-IT (Markdown), Präsentation und FITKO-Einordnung (PDF).

## Prüfung

- `tools/check_csv/check_csv.sh` meldet OK für `morph_box.csv` und `existing_solutions.csv`.
- Keine Treffer mehr für `zapuk-main`, `Entwurf v1.1` und „25.11.2026 vorgesehen“ in den CSV-Dateien, `Referenzen-ZaPuK.md` und `ZaPuK-Zusammenfassung.md`. „51. Sitzung“ steht nur noch als Sitzung von B-2026/43-IT.
- Die umgerechneten Fundstellen sind nur stichprobenartig im Volltext geprüft. Die Stichproben bei ADR-1105, ADR-1111, ADR-1130, ADR-1145 und dem Architekturkonzept stimmten. Die übrigen Werte stammen aus den Konkordanzen.

## Widersprüche in v1.1.1

- ADR-1122 Z. 77 verweist für die Benachrichtigungspflicht auf ADR-1145, das sie nicht enthält.
- ADR-1122 hat den Status „In Arbeit“, nennt aber schon eine gewählte Option.
- ADR-0018 Z. 9 setzt Eingang und Zugang gleich. Glossar Z. 231 verlangt für den Zugang zusätzlich, dass die Empfänger:in entschlüsseln kann.
- Das Glossar widerspricht sich selbst: Der Eintrag „Zugang“ (Z. 921) setzt Zugang und Eingang gleich, Z. 231 nicht.
- ADR-1121 hat den Status „In Arbeit“, nennt aber schon eine Übergangsregel (Z. 62).
- Die Errata des Rechtsgutachtens (Z. 36–44) nennen den Text „unverändert“, obwohl der Stand jetzt 19.10.2026 ist und Abschnitt E neu ist.

## Kohärenzprüfung vom 2026-11-27

Alle ZaPuK-Aussagen in `morph_box.csv`, in der Spalte ZaPuK von `existing_solutions.csv` und in `../Referenzen-ZaPuK.md` sind zeilenweise gegen v1.1.1 geprüft. Die Korrekturen sind in diesen Dateien und in `ZaPuK-Zusammenfassung.md` übernommen. Einzelheiten stehen in `../Referenzen-ZaPuK.md`, Abschnitt „Kohärenzprüfung vom 2026-11-27“. Wichtigste Fehler:

- B1 A4: Backends Dritter nur für Organisationen, nicht für Privatpersonen (ADR-1114 Z. 72).
- Ü4: „nicht den rechtlichen Zugang“ widersprach ADR-0018 Z. 9.
- Ü6 A1: ADR-1130 Option A ist ausgeschlossen, nicht nur nicht gewählt.
- R1: Gegenüber Gerichten trägt schon § 130a Abs. 4 S. 1 Nr. 4 ZPO, nicht erst eine Verordnung nach Nr. 5.
- `Referenzen-ZaPuK.md`: Zeilen B5, P10 und Ü7 fehlten, Fundstellen TE2 und TA korrigiert.

## Offen

- **Vor dem Löschen von `../zapuk-v1.0.0-121-g7419d8b/`:** `sources.md` und `README.md` sind seit der Kohärenzprüfung auf v1.1.1 umgestellt. Noch darauf verweisen `BundID-ZBP/ZBP-Zusammenfassung.md`, `MJP/MJP-Zusammenfassung.md`, `MUK/MUK-Zusammenfassung.md`, `ELSTER/ELSTER-Zusammenfassung.md`, `Referenzen-ELSTER.md`, `Referenzen-MJP.md` und die `quellen/`-Notizen von MJP, MUK und BundID-ZBP. Diese Verweise sind anzupassen.
- **Entbehrliche Kopien:** Die `*_main_2026-11-12.md`- und `*_v1.1-entwurf_*`-Kopien in `quellen/` sind weitgehend doppelt (Liste in den `_index.md`). Behalten werden müssen die Transitionspläne A–C und die ELSTER-Transitions-ADRs.
- **Foliensatz:** `zapuk-v1.1.1/Foliensätze/2026-11-26_ZaPuK-und-Matrix_Architektur-Einführung.pdf` ist nicht als Kürzel aufgenommen. Er enthält keine Aussagen zum Beschluss.
