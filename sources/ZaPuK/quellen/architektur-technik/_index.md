# Quellen: ZaPuK – Architektur und Technik

Erstabruf am 2026-11-19 über die GitLab-API von openCode (Projekt-ID 3799, `it-planungsrat/fit-ab/zapuk`), aktualisiert am 2026-11-27. Hauptquelle ist seit 2026-11-27 die lokale Kopie des Tags `v1.1.1` in `../zapuk-v1.1.1/` (wird hier nur referenziert, nicht kopiert und nicht verändert). Die frühere Hauptquelle `../zapuk-v1.0.0-121-g7419d8b/` wird gelöscht und nicht mehr zitiert. Die Kopien in diesem Ordner (Stand `main` 2026-11-12 und Entwurfsbranch `architecture-v1.1` 2026-11-13) bleiben vorerst liegen, weil `../../../Referenzen-ZaPuK.md` und `../../ZaPuK-Zusammenfassung.md` ihre Zeilen zitieren. Die Auswertung steht in `_befunde.md`, die Umrechnung der alten Fundstellen im Abschnitt „Konkordanz der Fundstellen (alt → v1.1.1)“.

Fundstellen in `../zapuk-v1.1.1/` sind Originalzeilen. Jede Kopie in diesem Ordner beginnt dagegen mit vier Kommentarzeilen (Quelle mit Commit-URL, Branch, Commit, Pfad im Repo, Abrufdatum) und einer Leerzeile; ihr Inhalt ist unverändert, Originalzeile = Zeile in der Kopie minus 5.

Themen der parallelen Ordner (hier nicht ausgewertet): Recht, Governance, IT-PLR-Beschlüsse, Finanzierung (`../recht-governance/`); Umsetzung, Neo, Presse, Bestandslösungs-Transition aus Fremdsicht (`../umsetzung-rezeption/`). Zusammenfassung, Transitionen, Rechtsgutachten, Programm-README (Logbuch) und Foliensätze aus v1.1.1 sind hier ebenfalls nicht ausgewertet.

## Upstream-Stand

| Ref | Commit | Datum | Bemerkung |
|---|---|---|---|
| Tag `v1.1.1` | – (lokale Kopie `../zapuk-v1.1.1/`, eingespielt mit Commit `be7dab0` dieses Repos) | Architekturkonzept Stand 26.11.2026 | „Fortschreibung nach der 51. Sitzung“; `main` „enthält die zuletzt operativ freigegebene Version“ (`../zapuk-v1.1.1/README.md` Z. 77–80). Hauptquelle. |
| Tag `v1.1.0` | – | – | „Abgabefassung für die 51. Sitzung des IT-Planungsrats, Anlagen zu Beschluss 2026/43-IT“ (`../zapuk-v1.1.1/README.md` Z. 77). Nicht lokal abgelegt, nicht ausgewertet. |
| `main` | `cc150e2977575d0a3e0c065521cdb1656eaae074` | 2026-11-12 21:20 | Merge "Glossar neo begriffe". 196 Dateien, dieselben Pfade wie `../zapuk-v1.0.0-121-g7419d8b/`. 42 Dateien mit anderem Inhalt (35 Markdown, 3 PNG, 2 PUML, 2 SVG). Durch v1.1.1 überholt. |
| `architecture-v1.1` | `ef99118c0607dd7fb11a005593569a36a0f84343` | 2026-11-13 20:15 | Entwurfsbranch, 56 Commits vor `main`, 225 Dateien. Enthielt Architekturkonzept v1.1, Zusammenfassung v1.1, Transitionsstrategie mit Transitionsplänen ZBP, MJP, MUK und zwei TADRs zu Mein ELSTER, Rechtsgutachten. Inhalte zu Architektur und ADRs in v1.1.1 aufgegangen. |
| `architecture-concept-1-1` | `335b7911…` | 2026-03-23 | Älterer Entwurfsbranch (ein Commit), löscht u. a. alle ADRs ab 0017. Überholt, nicht ausgewertet. |
| `update-projektinformationen-04-2026` | – | 2026-04-23 | "Ergänzung Logbuch", nicht ausgewertet. |

Stand von `../zapuk-v1.0.0-121-g7419d8b/` (wird gelöscht): entsprach einem `main`-Stand vor den Commits vom 2026-11-12 (ADR-1111 mit Datum 11.11.2026, ADR-0000 bis 0016 noch mit Status "Beschlossen", ADR-1105 noch "Operativ entschieden"), zwischen `cf40bb806a` (2026-11-06) und `de456f42c0` (2026-11-12 08:46) **[Folgerung]**.

Wesentliche Änderungen `main` 2026-11-12 gegenüber `../zapuk-v1.0.0-121-g7419d8b/` (alle in v1.1.1 enthalten):

- Statusvokabular: "✅ Beschlossen" heißt jetzt "✔️ Operativ entschieden"; neuer Status "🧭 Richtung entschieden" (ADR-IDX Z. 6, Legende Z. 81). Auf "Richtung entschieden" zurückgestuft: ADR-1105, ADR-1106, ADR-1109, ADR-1147 (ADR-IDX Z. 41–61).
- Neuer offener Entscheidungsbedarf ohne Datei: "Umgang mit nicht entschlüsselbaren Nachrichten" (ADR-IDX Z. 66).
- ADR-1111: föderierte Suche über eine Matrix-Erweiterung (MSC4258), Vergleich mit dem VZD-FHIR-Directory des TI-Messengers statt gemSpec VZD.
- ADR-1105, ADR-1106, ADR-1108: Identitätsnachweis bei BundID ohne Bereichs-MXID; Betreibertrennung Nachweisstelle / Postfach-Backend.
- Glossar mit englischen Übersetzungen und Neo-Begriffen.

Wesentliche Änderungen v1.1.1 gegenüber `main` 2026-11-12 und Entwurf 2026-11-13: siehe `_befunde.md`, Abschnitt „Änderungen v1.1.1 gegenüber `main` 2026-11-12 und Entwurf 2026-11-13“. Kurz: ADR-1121 (Abrufbestätigung) und ADR-1122 (Benachrichtigungsdienst) neu, beide „In Arbeit“; Architekturkonzept v1.1.1 mit ausgeführtem Kapitel 3; Entwurfsänderungen der ADRs übernommen und am 26.11.2026 nochmals ergänzt (ADR-1110, 1118, 1135, 1145, 1146, 1147); Glossar mit Benachrichtigungsdienst, Benachrichtigungsbestätigung, Bereitstellung, bPK u. a.

## Hauptquelle v1.1.1 (`../zapuk-v1.1.1/`, nicht kopiert)

Basis-URL: `https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/v1.1.1/` + Pfad. Herausgeber: FITKO / FIT-AB im Auftrag des IT-Planungsrats. Zeilen = Originalzeilen.

| Datei | Kürzel | Datum / Status | Inhalt (Bezug) |
|---|---|---|---|
| ../zapuk-v1.1.1/Zielarchitektur/Architekturentscheidungen/README.md | ADR-IDX | – | ADR-Liste mit Status und Hinweisen; ADR-1121, ADR-1122 „In Arbeit“ (Z. 54–55); unbearbeitet ADR-1149, Löschung und Aufbewahrung, Metadatenminimierung, nicht entschlüsselbare Nachrichten (Z. 63–66); Legende Z. 81 |
| ../zapuk-v1.1.1/Zielarchitektur/Architekturentscheidungen/ADR-0000-… bis ADR-1148-… (46 Dateien) | ADR-xxxx | ADR-0000–0016 Operativ entschieden (B-2025/28); Datum der übrigen siehe Kopf der Datei, sechs mit 26.11.2026 (1110, 1118, 1135, 1145, 1146, 1147) | Architekturentscheidungen; Status je ADR in `_befunde.md` |
| ../zapuk-v1.1.1/Zielarchitektur/Architekturentscheidungen/ADR-1121-Ausgestaltung-der-Abrufbestätigung.md | ADR-1121 | In Arbeit, 23.11.2026 | Abrufbestätigung durch Client (A, gilt vorerst), Backend (B) oder beide (C); Gegenüberstellung § 41 Abs. 2a VwVfG, § 9 OZG / § 122a AO, § 37 Abs. 2a SGB X (Z. 11–13) |
| ../zapuk-v1.1.1/Zielarchitektur/Architekturentscheidungen/ADR-1122-Benachrichtigungsdienst-für-Bereitstellungen.md | ADR-1122 | In Arbeit, 23.11.2026 | Benachrichtigung durch das Empfänger-Backend (Option A gewählt, Z. 56), Benachrichtigungsbestätigung des Quittungsdienstes (Z. 60–62), Nutzerkonto ausgeschlossen |
| ../zapuk-v1.1.1/Zielarchitektur/Architekturkonzept/README.md | AK11 | v1.1.1, Stand 26.11.2026 | Architekturkonzept v1.1 fortgeschrieben, Anlage zu B-2026/43-IT (Z. 7): Kap. 3 Geschäftsarchitektur (Z. 156–249), Kap. 4 Entwicklung, Betrieb, Governance (Z. 250ff.), Kap. 5 IT-Architektur (Z. 539ff.), Benachrichtigungsdienst Z. 653, Bridges in der Transition Z. 718 |
| ../zapuk-v1.1.1/Zielarchitektur/Architekturkonzept/Vorversionen/v1.0/README.md | AK10 | v1.0, Sommer 2025 | Zeilen gleich der früheren Fassung in `../zapuk-v1.0.0-121-g7419d8b/`: Z. 288–345 Kap. 2.5 ADR-Zusammenfassung (Z. 345 eigene IdP "nicht unterstützt"), Z. 381–434 Kap. 3 Brownfield (Z. 393–409 Kap. 3.2, Z. 401 Bridge-Verbot, Z. 403 Channel Adapter, Z. 413 nur ein zentrales Backend) |
| ../zapuk-v1.1.1/Zielarchitektur/Zusammenfassung/Vorversionen/v1.0/README.md | ZF10 | v1.0 | Zeilen gleich der früheren Fassung in `../zapuk-v1.0.0-121-g7419d8b/`: Z. 170–202 Kosten (3.3), Z. 232 FIT-Connect als zentraler Anbindungsweg (Auswertung der Zusammenfassung außerhalb dieses Ordners) |
| ../zapuk-v1.1.1/Glossar.md | GLOS | – | Glossar; u. a. Benachrichtigungsbestätigung (Z. 179), Benachrichtigungsdienst (Z. 187), Bereich (Z. 201), Bereichs-MXID (Z. 207), bPK (Z. 213), Bereitstellung (Z. 219), Bereitstellungsbestätigung (Z. 227), Betriebsmodell (Z. 241), Funktionspostfach (Z. 423), Machtbereich (Z. 499), Protokollkonverter (Z. 689) |
| ../zapuk-v1.1.1/Abkürzungen.md | ABK | – | Abkürzungsverzeichnis, ergänzt um bPK, E-GovG, SDM |
| ../zapuk-v1.1.1/Konsolidierte-Anforderungen/README.md | ANF | v0.7.1, 24.02.2026 | bytegleich mit `../zapuk-v1.0.0-121-g7419d8b/`; z. B. Z. 85 ANF_FUN_NAC_183 strukturierte Daten, Z. 86 ANF_FUN_NAC_185 Antworten zulassen/untersagen, Z. 153 ANF_FUN_ZUS_026 Benachrichtigung per E-Mail oder Push |
| ../zapuk-v1.1.1/Konsolidierte-Anforderungen/Lesefassung/README.md | – | v1.0 | nur PDF-Link geändert |
| ../zapuk-v1.1.1/Zielarchitektur/Rahmenbedingungen/README.md | RB | v1.0 | nur Linkziele geändert |
| ../zapuk-v1.1.1/Zielarchitektur/README.md | – | – | Überblick; Architekturkonzept als „Fortschreibung“ von v1.0 (Z. 13), Transitionsstrategie und Rechtsgutachten verlinkt (Z. 26–27) |
| ../zapuk-v1.1.1/Konsultationen/ | – | – | Konsultationsseiten Phase 1 und 2, nur Linkziele geändert; keine veröffentlichten Ergebnisse |

## Kopien Stand `main` (Commit cc150e2977, 2026-11-12)

Basis-URL: `https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/` + Pfad. Herausgeber: FITKO / FIT-AB im Auftrag des IT-Planungsrats. Spalte „v1.1.1“: Verhältnis zur Hauptquelle. Alle Kopien sind inhaltlich durch v1.1.1 abgedeckt und nur noch wegen der Zeilenangaben in `Referenzen-ZaPuK.md` und `ZaPuK-Zusammenfassung.md` nötig.

| Datei | Pfad im Repo | Datum / Status | Inhalt kurz, Änderung gegenüber `../zapuk-v1.0.0-121-g7419d8b/` | v1.1.1 |
|---|---|---|---|---|
| fitko_zapuk-adr-index_main_2026-11-12.md | Zielarchitektur/Architekturentscheidungen/README.md | – | ADR-Liste mit Status und Hinweisen ("eingeschränkt durch", "offen: …"); neuer Status "Richtung entschieden"; fünf unbearbeitete Entscheidungsbedarfe (ADR-1149 API-Autorisierung, Benachrichtigungsdienst, Löschung/Aufbewahrung, Metadatenminimierung, nicht entschlüsselbare Nachrichten) | geändert: ADR-1121, ADR-1122 neu, Benachrichtigungsdienst nicht mehr unbearbeitet |
| fitko_zapuk-glossar_main_2026-11-12.md | Glossar.md | – | Glossar mit EN-Übersetzungen, Neo-Begriffe; u. a. Bereich (Z. 184), Bereichs-MXID (Z. 190), Betriebsmodell (Z. 212), Funktionspostfach (Z. 394), Machtbereich (Z. 470), Protokollkonverter (Z. 658) | geändert: neue Einträge, Zeilen ab Original-Z. 63 verschoben |
| fitko_zapuk-abkuerzungen_main_2026-11-12.md | Abkürzungen.md | – | Abkürzungsverzeichnis, geringfügig ergänzt | drei Einträge ergänzt |
| fitko_zapuk-adr-0000-…_main_2026-11-12.md bis fitko_zapuk-adr-0016-…_main_2026-11-12.md (17 Dateien) | ADR-0000 bis ADR-0016 | Operativ entschieden, Datum unverändert (Jan.–Apr. 2025) | Nur Statusbezeichnung geändert; ADR-0011 zusätzlich Bildpfade; ADR-0015/0016 Hinweis "konkretisiert durch ADR-1106, ADR-1108" | gleich, außer ADR-0004 Hinweis Phase 2 (wie Entwurf) |
| fitko_zapuk-adr-0018-nichtabstreitbarkeit-uebermittlung_main_2026-11-12.md | ADR-0018 | Operativ entschieden, 12.11.2026 | Bereitstellungsbestätigung; Anforderungs-IDs ergänzt, Fußnote eIDAS korrigiert | geändert (Stand Entwurf plus redaktionell, 13.11.2026) |
| fitko_zapuk-adr-1102-umsetzung-von-funktionspostfaechern-und-vorgangsraeumen_main_2026-11-12.md | ADR-1102 | Operativ entschieden, 12.11.2026 | Funktionspostfach, Vorgangsraum, RMS; Datenschutz-Treiber, Verweis auf ADR-1109 | gleich |
| fitko_zapuk-adr-1103-mehrbenutzer-organisationen-betriebsmodelle_main_2026-11-12.md | ADR-1103 | Operativ entschieden, 11.11.2026 | Betriebsmodelle A/B/C; nur "(ZaPuK)" bei Zentralverantwortung gestrichen | gleich |
| fitko_zapuk-adr-1105-pseudonym-adressierung-von-privatpersonen_main_2026-11-12.md | ADR-1105 | Richtung entschieden, 12.11.2026 | Bereichs-MXID, Zuordnungsdienst 2E/2E’; Status zurückgestuft; Restrisiko BundID-Nachweisstelle neu bewertet | geändert (Z. 139, Z. 141) |
| fitko_zapuk-adr-1106-fachlicher-identitaetsnachweis-matrix-nachrichtenkontext_main_2026-11-12.md | ADR-1106 | Richtung entschieden, 12.11.2026 | Identitätsnachweis pro Nachricht; Option A für eID/EUDI-Wallet, Option C als Übergang; Status zurückgestuft | geändert (Stand Entwurf) |
| fitko_zapuk-adr-1107-nachweisformat-identitaetsnachweis-matrix-nachrichten_main_2026-11-12.md | ADR-1107 | Operativ entschieden, 12.11.2026 | SD-JWT VC als Nachweisformat | gleich |
| fitko_zapuk-adr-1108-nachweisstellen-org-idp-adapter_main_2026-11-12.md | ADR-1108 | Operativ entschieden, 12.11.2026 | Zentrale Nachweisstelle, Org-IdP-Adapter, Trust-Liste | gleich |
| fitko_zapuk-adr-1109-raumtopologie-funktionspostfaecher_main_2026-11-12.md | ADR-1109 | Richtung entschieden, 12.11.2026 | FP-Bot vermittelnd (Option B) oder schlüssellos (Option C); Status zurückgestuft | gleich |
| fitko_zapuk-adr-1111-aufloesung-organisationsbezogene-postfachadressen_main_2026-11-12.md | ADR-1111 | Operativ entschieden, 12.11.2026 | Föderiertes Adressregister; Suche über MSC4258 | gleich |
| fitko_zapuk-adr-1119-absendergesteuerte-uebermittlungsoptionen-mit-berechtigungspruefung_main_2026-11-12.md | ADR-1119 | Operativ entschieden, 11.11.2026 | Übermittlungsoptionen (Quittungen, Pull, Mindest-Vertrauensniveau) | geändert: Option Benachrichtigungsbestätigung (Z. 53, Z. 91) |
| fitko_zapuk-adr-1120-beweissichere-uebermittlung-von-anhaengen_main_2026-11-12.md | ADR-1120 | Operativ entschieden, 12.11.2026 | Proaktiver Pull der Anhänge durch das Empfänger-Backend | gleich |
| fitko_zapuk-adr-1135-veraktung-und-aufbewahrung_main_2026-11-12.md | ADR-1135 | Operativ entschieden, 11.11.2026 | Archivierung im Machtbereich der Datenbesitzer, Exportpaket | geändert: Risiko ergänzt, 26.11.2026 |
| fitko_zapuk-adr-1144-pki-anforderungen-und-vertrauensinfrastruktur_main_2026-11-12.md | ADR-1144 | Operativ entschieden, 11.11.2026 | Zertifizierungsstelle des Verbunds, eigene Signaturschlüssel für Föderations- und Trust-Liste | gleich |
| fitko_zapuk-adr-1147-schluesselwiederherstellung_main_2026-11-12.md | ADR-1147 | Richtung entschieden, 12.11.2026 | Server-Key-Backup plus Re-Authentisierung, Key Escrow ausgeschlossen; Status zurückgestuft | geändert: Treiber und Konsequenzen ergänzt, 26.11.2026 |
| fitko_zapuk-adr-1148-onboarding-im-self-service-portal_main_2026-11-12.md | ADR-1148 | Operativ entschieden, 12.11.2026 | Manuelle Freigabe der Backends, sonst automatisiertes Onboarding | gleich |

## Kopien Stand `architecture-v1.1` (Commit ef99118c06, 2026-11-13, Entwurf)

Basis-URL: `https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/ef99118c0607dd7fb11a005593569a36a0f84343/` + Pfad. Herausgeber: FITKO / FIT-AB. Stand: Entwurf vor der 51. Sitzung; die Architektur- und ADR-Inhalte sind in v1.1.1 aufgegangen. Kap. 3 (Geschäftsarchitektur) verwies im Entwurf noch auf v1.0 (Kopie-Z. 164), in v1.1.1 ausgeführt.

| Datei | Pfad im Repo | Datum / Stand | Inhalt kurz | v1.1.1 |
|---|---|---|---|---|
| fitko_zapuk-architekturkonzept-v1.1_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Architekturkonzept/README.md | v1.1, Informationsstand Sept. 2026 | Neues Architekturkonzept: Kap. 4 Entwicklung/Betrieb (vier Backends, FIT-Connect, BMDS, SSP, Adressregister, Frontend in Bundesportal/D-App), Kap. 5 IT-Architektur (Komponenten, Zuordnungsdienst, Nachweisstellen, RMS, Protokollkonverter, Bridges in der Transition, Malware, Nachweise, Betriebsmodelle). Ersetzt v1.0, Kap. 3 (Brownfield) entfällt | geändert: Kap. 3 Geschäftsarchitektur eingefügt (Zeilen ab Kap. 4 plus 87 bzw. 89), ADR-1121/1122 ergänzt, sonst redaktionell |
| fitko_zapuk-zusammenfassung-v1.1_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Zusammenfassung/README.md | v1.1 | Management Summary Phase II: 27 neue ADRs, Neo-Validierung mit BW-Empfangsclient und GA-Lotse, Verantwortungsverteilung, Entscheidungsbedarf IT-PLR; Finanzierung erst zur 52. IT-PLR-Sitzung | in v1.1.1 vorhanden, hier nicht abgeglichen |
| fitko_zapuk-transitionen-ueberblick_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Transitionen/README.md | Stand 06.10.2026 | Übergreifende Transitionsstrategie: sieben Leitprinzipien, Vier-Phasen-Modell, Pfade für ZBP, MJP, MUK, Mein ELSTER | in v1.1.1 vorhanden, hier nicht abgeglichen |
| fitko_zapuk-transition-a-zentrales-buergerpostfach_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Transitionen/A--Zentrales-Bürgerpostfach/README.md | Stand 25.09.2026 | Transitionsplan ZBP: Matrix-Backend für Privatpersonen, Datenmigration, ZBP-Matrix-Bridge für die ZBP-API, Abschaltung | fehlt in v1.1.1 |
| fitko_zapuk-transition-b-mein-justizpostfach_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Transitionen/B--Mein-Justizpostfach/README.md | Stand 25.09.2026 | Transitionsplan MJP: Justiz-Backend mit OSCI-Matrix-Protokollkonverter, Adressen `@[SAFE-ID]:domain`, EGVP bleibt | fehlt in v1.1.1 |
| fitko_zapuk-transition-c-mein-unternehmenskonto_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Transitionen/C--Mein-Unternehmenskonto/README.md | Stand 06.10.2026 | Transitionsplan MUK: OZG-PLUS-Postfach direkt abgelöst (keine Bridge), Postfach 2.0 über Verweise und optional Postfach-2.0-Matrix-Bridge | fehlt in v1.1.1 |
| fitko_zapuk-tadr-mein-elster-001-…_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Transitionen/Architekturentscheidungen/TADR-MEIN-ELSTER-001-… | Operativ entschieden, 25.09.2026 | Zuordnung von ELSTER-Datenarten zum zentralen ZaPuK-Postfach | fehlt in v1.1.1 |
| fitko_zapuk-tadr-mein-elster-002-…_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Transitionen/Architekturentscheidungen/TADR-MEIN-ELSTER-002-… | Operativ entschieden, 25.09.2026 | Anbindungsmodell Mein ELSTER: Zustell-Bridge in der ELSTER-Domäne (Option C), später SDK (Option D) | fehlt in v1.1.1 |
| fitko_zapuk-rechtsgutachten_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Rechtsgutachten/README.md | v1.1 | Rechtsgutachten Phase II (4.358 Zeilen); Auswertung im Ordner `recht-governance/` | in v1.1.1 vorhanden (4.504 Zeilen), hier nicht abgeglichen |
| fitko_zapuk-adr-index_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Architekturentscheidungen/README.md | – | Wie `main`, Tippfehler korrigiert, ADR-0004 zusätzlich "eingeschränkt durch ADR-1145" | geändert: ADR-1121, ADR-1122 |
| fitko_zapuk-glossar_v1.1-entwurf_2026-11-13.md | Glossar.md | – | Wie `main`, Eintrag Machtbereich präzisiert | geändert: neue Einträge |
| fitko_zapuk-adr-0004-…_v1.1-entwurf_2026-11-13.md | ADR-0004 | Operativ entschieden, 15.04.2025 | Hinweis Phase 2 erweitert (Bereitstellungsbestätigung, Anschlussvertrag) | redaktionell |
| fitko_zapuk-adr-0018-…_v1.1-entwurf_2026-11-13.md | ADR-0018 | 13.11.2026 | Rechtsgrundlage präzisiert (§ 37 Abs. 2a SGB X, § 122a AO) | redaktionell, Z. 9 „Eingang“ statt „Zugang“ |
| fitko_zapuk-adr-1101-…_v1.1-entwurf_2026-11-13.md | ADR-1101 | 13.11.2026 | Begründung "E2EE nicht strikt umgesetzt" statt "verletzt SIC_VER_ÜBE_026" | redaktionell |
| fitko_zapuk-adr-1105-…_v1.1-entwurf_2026-11-13.md | ADR-1105 | Richtung entschieden, 13.11.2026 | Zuordnungsdienst ohne Identitätsdaten | geändert: Verknüpfung nur durch die Person (Z. 141) |
| fitko_zapuk-adr-1106-…_v1.1-entwurf_2026-11-13.md | ADR-1106 | Richtung entschieden, 13.11.2026 | Option A angeglichen, Holder-Binding als offener Punkt | gleich |
| fitko_zapuk-adr-1110-…_v1.1-entwurf_2026-11-13.md | ADR-1110 | 13.11.2026 | Neuer Abschnitt Risiken (§ 9 OZG anzupassen, Parallelbetrieb, zweite Adresse) | geändert: nur § 9 OZG bleibt, 26.11.2026 |
| fitko_zapuk-adr-1118-…_v1.1-entwurf_2026-11-13.md | ADR-1118 | 13.11.2026 | Offener Punkt: Nachweis der Benachrichtigung (§ 37 Abs. 2a SGB X) | geändert: offener Punkt nach ADR-1122 verlagert, Folge ADR-1121, Risiko § 9 OZG, 26.11.2026 |
| fitko_zapuk-adr-1130-…_v1.1-entwurf_2026-11-13.md | ADR-1130 | 13.11.2026 | Hinweis im Postfach-Zugang, wenn bei Option E Klartext das Endgerät verlässt | gleich |
| fitko_zapuk-adr-1145-…_v1.1-entwurf_2026-11-13.md | ADR-1145 | 13.11.2026 | Neue Bedingung: Verwahrung der Wiederherstellungsschlüssel | geändert: Einschränkung ADR-0004 ergänzt, 26.11.2026 |
| fitko_zapuk-adr-1146-…_v1.1-entwurf_2026-11-13.md | ADR-1146 | 13.11.2026 | Postfachadresse steht im Key-Binding-JWT | geändert: Prüfauftrag präzisiert, 26.11.2026 |
| fitko_zapuk-adr-1147-…_v1.1-entwurf_2026-11-13.md | ADR-1147 | Richtung entschieden, 13.11.2026 | Treiber Schlüsselverwaltung (ANF_ARC_ARC_050) ergänzt | geändert: Konsequenzen ergänzt, 26.11.2026 |

## Weitere Abrufe

| Datei | URL | Herausgeber | Datum | Inhalt kurz |
|---|---|---|---|---|
| fitko_zapuk-opencode-issues-architektur_2026-11-19.md | https://gitlab.opencode.de/api/v4/projects/3799/issues | Einreichende (MASTD RLP, Dataport, Privatperson); Tracker FITKO | Issues 02/2026–03/2026, alle geschlossen | Beschreibungen der Issues #23 (Peppol, ADR-0010), #24 (M2M und Maschinenidentitäten), #25 (API-first statt SDK-first, ADR-0009), #27 (Browser-Client und Betreiberzugriff); Antworten des ZaPuK-Teams nur mit Anmeldung lesbar (HTTP 401) |

## Entbehrliche Kopien

Inhaltlich durch `../zapuk-v1.1.1/` ersetzt, aber noch von `Referenzen-ZaPuK.md` und `ZaPuK-Zusammenfassung.md` zitiert: alle `*_main_2026-11-12.md` (ADR-Index, Glossar, Abkürzungen, 32 ADRs) sowie die Entwurfskopien von Architekturkonzept, ADR-Index, Glossar und ADRs (`*_v1.1-entwurf_*` außer Zusammenfassung, Transitionen, TADRs, Rechtsgutachten). Sie können entfallen, sobald beide Dateien auf die Fundstellen in v1.1.1 umgestellt sind (Konkordanz in `_befunde.md`). Nicht entbehrlich: Transitionspläne A bis C und TADRs (fehlen in v1.1.1), Issues-Abruf; Zusammenfassung, Transitionsstrategie und Rechtsgutachten bis zum Abgleich durch die jeweiligen Ordner.

## Nicht kopiert

- Bilder und Diagramme (`images/`, PNG, SVG, PUML, drawio), darunter die Konzeptgrafiken `applikationssicht-verbund`, `komponenten-ueberblick`, `postfach-backend-dienste` und die Abbildungen der Transitionspläne. Die Textfassungen beschreiben die Inhalte.
- PDF-Fassungen und Foliensätze (in `../zapuk-v1.1.1/Foliensätze/`, hier nicht ausgewertet).
