# Quellen: ZaPuK – Architektur und Technik

Abgerufen am 2026-11-19 über die GitLab-API von openCode (Projekt-ID 3799, `it-planungsrat/fit-ab/zapuk`). Hauptquelle bleibt die lokale Kopie `../../../zapuk-main/` (wird hier nur referenziert, nicht kopiert und nicht verändert). In diesem Ordner liegen nur Dateien, die upstream neu sind oder sich gegenüber `zapuk-main/` inhaltlich geändert haben. Die Auswertung steht in `_befunde.md`.

Jede kopierte Datei beginnt mit vier Kommentarzeilen (Quelle mit Commit-URL, Branch, Commit, Pfad im Repo, Abrufdatum) und einer Leerzeile. Der Inhalt ist unverändert. Originalzeile = Zeile in der Kopie minus 5. Die Fundstellen in `_befunde.md` beziehen sich auf die Zeilen der Kopie.

Themen der parallelen Ordner (hier nicht ausgewertet): Recht, Governance, IT-PLR-Beschlüsse, Finanzierung (`../recht-governance/`); Umsetzung, Neo, Presse, Bestandslösungs-Transition aus Fremdsicht (`../umsetzung-rezeption/`). Das Rechtsgutachten aus dem Branch `architecture-v1.1` liegt hier nur als Kopie, weil es mit dem Branch neu hinzukam; ausgewertet wird es im Ordner `recht-governance/`.

## Upstream-Stand

| Ref | Commit | Datum | Bemerkung |
|---|---|---|---|
| `main` | `cc150e2977575d0a3e0c065521cdb1656eaae074` | 2026-11-12 21:20 | Merge "Glossar neo begriffe". 196 Dateien, dieselben Pfade wie `zapuk-main/`. 42 Dateien mit anderem Inhalt (35 Markdown, 3 PNG, 2 PUML, 2 SVG). |
| `architecture-v1.1` | `ef99118c0607dd7fb11a005593569a36a0f84343` | 2026-11-13 20:15 | Nicht gemergter Entwurfsbranch, 56 Commits vor `main`, 225 Dateien. Enthält Architekturkonzept v1.1, Zusammenfassung v1.1, Transitionsstrategie mit Transitionsplänen ZBP, MJP, MUK und zwei TADRs zu Mein ELSTER, Rechtsgutachten. |
| `architecture-concept-1-1` | `335b7911…` | 2026-03-23 | Älterer Entwurfsbranch (ein Commit), löscht u. a. alle ADRs ab 0017. Überholt durch `architecture-v1.1`, nicht ausgewertet. |
| `update-projektinformationen-04-2026` | – | 2026-04-23 | "Ergänzung Logbuch", nicht ausgewertet. |

Tags: keine. Offene Merge Requests: keine (abgefragt 2026-11-19).

Stand von `zapuk-main/`: Die lokale Kopie entspricht einem `main`-Stand vor den Commits vom 2026-11-12 (ADR-1111 dort mit Datum 11.11.2026, ADR-0000 bis 0016 noch mit Status "Beschlossen", ADR-1105 noch "Operativ entschieden"). Den genauen Commit verrät die Kopie nicht **[Folgerung]**: zwischen `cf40bb806a` (2026-11-06) und `de456f42c0` (2026-11-12 08:46).

Wesentliche Änderungen `main` gegenüber `zapuk-main/`:

- Statusvokabular: "✅ Beschlossen" heißt jetzt "✔️ Operativ entschieden"; neuer Status "🧭 Richtung entschieden" (Index Z. 11, Legende Z. 85). Auf "Richtung entschieden" zurückgestuft: ADR-1105, ADR-1106, ADR-1109, ADR-1147 (Index Z. 46–64).
- Neuer offener Entscheidungsbedarf ohne Datei: "Umgang mit nicht entschlüsselbaren Nachrichten" (Index Z. 70).
- ADR-1111: föderierte Suche über eine Matrix-Erweiterung (MSC4258), Vergleich mit dem VZD-FHIR-Directory des TI-Messengers statt gemSpec VZD.
- ADR-1105, ADR-1106, ADR-1108: Identitätsnachweis bei BundID ohne Bereichs-MXID; Betreibertrennung Nachweisstelle / Postfach-Backend.
- Glossar mit englischen Übersetzungen und Neo-Begriffen (129 statt 123 Einträge).
- Tippfehler im Index: "Seit 12.12.2026" (Z. 11), im Branch `architecture-v1.1` korrigiert auf "12.11.2026".

## Bereits vorhanden in `zapuk-main/` (nicht kopiert)

Unverändert upstream (`main` = lokale Kopie), werden in `_befunde.md` direkt zitiert:

| Datei | Inhalt (Bezug) |
|---|---|
| ../../../zapuk-main/Zielarchitektur/Architekturkonzept/README.md | Architekturkonzept v1.0 (Sommer 2025): Z. 288–345 Kap. 2.5 ADR-Zusammenfassung (Z. 345 eigene IdP "nicht unterstützt"), Z. 381–434 Kap. 3 Brownfield (Z. 401 Bridge-Verbot, Z. 403 Channel Adapter, Z. 413 nur ein zentrales Backend) |
| ../../../zapuk-main/Zielarchitektur/Zusammenfassung/README.md | Überblick v1.0: Z. 170–202 Kosten (3.3), Z. 218–241 Implikationen (Z. 232 FIT-Connect als zentraler Anbindungsweg, Voraussetzung Matrix und MLS) |
| ../../../zapuk-main/Konsolidierte-Anforderungen/README.md | Anforderungsliste v0.7.1 (24.02.2026), z. B. Z. 85 ANF_FUN_NAC_183 strukturierte Daten, Z. 86 ANF_FUN_NAC_185 Antworten zulassen/untersagen, Z. 153 ANF_FUN_ZUS_026 Benachrichtigung per E-Mail oder Push |
| ../../../zapuk-main/Zielarchitektur/Architekturentscheidungen/ADR-0017-…, ADR-1100-…, ADR-1101-…, ADR-1104-…, ADR-1110-…, ADR-1112-…, ADR-1113-…, ADR-1114-…, ADR-1118-…, ADR-1130-…, ADR-1145-…, ADR-1146-… | ADRs, deren Text auf `main` unverändert ist (Stand und Datum siehe `_befunde.md`) |
| ../../../zapuk-main/Konsultationen/ | Konsultationsseiten Phase 1 und 2; keine veröffentlichten Ergebnisse |

## Kopien Stand `main` (Commit cc150e2977, 2026-11-12)

Basis-URL: `https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/` + Pfad. Herausgeber: FITKO / FIT-AB im Auftrag des IT-Planungsrats.

| Datei | Pfad im Repo | Datum / Status | Inhalt kurz, Änderung gegenüber `zapuk-main/` |
|---|---|---|---|
| fitko_zapuk-adr-index_main_2026-11-12.md | Zielarchitektur/Architekturentscheidungen/README.md | – | ADR-Liste mit Status und Hinweisen ("eingeschränkt durch", "offen: …"); neuer Status "Richtung entschieden"; fünf unbearbeitete Entscheidungsbedarfe (ADR-1149 API-Autorisierung, Benachrichtigungsdienst, Löschung/Aufbewahrung, Metadatenminimierung, nicht entschlüsselbare Nachrichten) |
| fitko_zapuk-glossar_main_2026-11-12.md | Glossar.md | – | Glossar mit EN-Übersetzungen, Neo-Begriffe; u. a. Bereich (Z. 184), Bereichs-MXID (Z. 190), Betriebsmodell (Z. 212), Funktionspostfach (Z. 394), Machtbereich (Z. 470), Protokollkonverter (Z. 658) |
| fitko_zapuk-abkuerzungen_main_2026-11-12.md | Abkürzungen.md | – | Abkürzungsverzeichnis, geringfügig ergänzt |
| fitko_zapuk-adr-0000-…_main_2026-11-12.md bis fitko_zapuk-adr-0016-…_main_2026-11-12.md (17 Dateien) | ADR-0000 bis ADR-0016 | Operativ entschieden, Datum unverändert (Jan.–Apr. 2025) | Nur Statusbezeichnung geändert; ADR-0011 zusätzlich Bildpfade; ADR-0015/0016 Hinweis "konkretisiert durch ADR-1106, ADR-1108" |
| fitko_zapuk-adr-0018-nichtabstreitbarkeit-uebermittlung_main_2026-11-12.md | ADR-0018 | Operativ entschieden, 12.11.2026 | Bereitstellungsbestätigung; Anforderungs-IDs ergänzt, Fußnote eIDAS korrigiert |
| fitko_zapuk-adr-1102-umsetzung-von-funktionspostfaechern-und-vorgangsraeumen_main_2026-11-12.md | ADR-1102 | Operativ entschieden, 12.11.2026 | Funktionspostfach, Vorgangsraum, RMS; Datenschutz-Treiber, Verweis auf ADR-1109 |
| fitko_zapuk-adr-1103-mehrbenutzer-organisationen-betriebsmodelle_main_2026-11-12.md | ADR-1103 | Operativ entschieden, 11.11.2026 | Betriebsmodelle A/B/C; nur "(ZaPuK)" bei Zentralverantwortung gestrichen |
| fitko_zapuk-adr-1105-pseudonym-adressierung-von-privatpersonen_main_2026-11-12.md | ADR-1105 | Richtung entschieden, 12.11.2026 | Bereichs-MXID, Zuordnungsdienst 2E/2E’; Status zurückgestuft; Restrisiko BundID-Nachweisstelle neu bewertet |
| fitko_zapuk-adr-1106-fachlicher-identitaetsnachweis-matrix-nachrichtenkontext_main_2026-11-12.md | ADR-1106 | Richtung entschieden, 12.11.2026 | Identitätsnachweis pro Nachricht; Option A für eID/EUDI-Wallet, Option C als Übergang; Status zurückgestuft |
| fitko_zapuk-adr-1107-nachweisformat-identitaetsnachweis-matrix-nachrichten_main_2026-11-12.md | ADR-1107 | Operativ entschieden, 12.11.2026 | SD-JWT VC als Nachweisformat |
| fitko_zapuk-adr-1108-nachweisstellen-org-idp-adapter_main_2026-11-12.md | ADR-1108 | Operativ entschieden, 12.11.2026 | Zentrale Nachweisstelle, Org-IdP-Adapter, Trust-Liste |
| fitko_zapuk-adr-1109-raumtopologie-funktionspostfaecher_main_2026-11-12.md | ADR-1109 | Richtung entschieden, 12.11.2026 | FP-Bot vermittelnd (Option B) oder schlüssellos (Option C); Status zurückgestuft |
| fitko_zapuk-adr-1111-aufloesung-organisationsbezogene-postfachadressen_main_2026-11-12.md | ADR-1111 | Operativ entschieden, 12.11.2026 | Föderiertes Adressregister; Suche über MSC4258 |
| fitko_zapuk-adr-1119-absendergesteuerte-uebermittlungsoptionen-mit-berechtigungspruefung_main_2026-11-12.md | ADR-1119 | Operativ entschieden, 11.11.2026 | Übermittlungsoptionen (Quittungen, Pull, Mindest-Vertrauensniveau) |
| fitko_zapuk-adr-1120-beweissichere-uebermittlung-von-anhaengen_main_2026-11-12.md | ADR-1120 | Operativ entschieden, 12.11.2026 | Proaktiver Pull der Anhänge durch das Empfänger-Backend |
| fitko_zapuk-adr-1135-veraktung-und-aufbewahrung_main_2026-11-12.md | ADR-1135 | Operativ entschieden, 11.11.2026 | Archivierung im Machtbereich der Datenbesitzer, Exportpaket |
| fitko_zapuk-adr-1144-pki-anforderungen-und-vertrauensinfrastruktur_main_2026-11-12.md | ADR-1144 | Operativ entschieden, 11.11.2026 | Zertifizierungsstelle des Verbunds, eigene Signaturschlüssel für Föderations- und Trust-Liste |
| fitko_zapuk-adr-1147-schluesselwiederherstellung_main_2026-11-12.md | ADR-1147 | Richtung entschieden, 12.11.2026 | Server-Key-Backup plus Re-Authentisierung, Key Escrow ausgeschlossen; Status zurückgestuft |
| fitko_zapuk-adr-1148-onboarding-im-self-service-portal_main_2026-11-12.md | ADR-1148 | Operativ entschieden, 12.11.2026 | Manuelle Freigabe der Backends, sonst automatisiertes Onboarding |

## Kopien Stand `architecture-v1.1` (Commit ef99118c06, 2026-11-13, Entwurf)

Basis-URL: `https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/ef99118c0607dd7fb11a005593569a36a0f84343/` + Pfad. Herausgeber: FITKO / FIT-AB. Status: nicht auf `main` gemergt; laut Zusammenfassung v1.1 vom FIT-AB in der 32. Sitzung am 15.09.2026 befürwortet und für die Vorlage an den IT-Planungsrat bestimmt (Z. 39, Z. 53, Anhang A ab Z. 332) **[unverifiziert]** (FIT-AB-Beschluss selbst nicht geprüft). Kap. 3 (Geschäftsarchitektur) verweist im Entwurf noch auf v1.0 (Z. 164).

| Datei | Pfad im Repo | Datum / Stand | Inhalt kurz |
|---|---|---|---|
| fitko_zapuk-architekturkonzept-v1.1_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Architekturkonzept/README.md | v1.1, Informationsstand Sept. 2026 | Neues Architekturkonzept: Kap. 4 Entwicklung/Betrieb (vier Backends, FIT-Connect, BMDS, SSP, Adressregister, Frontend in Bundesportal/D-App), Kap. 5 IT-Architektur (Komponenten, Zuordnungsdienst, Nachweisstellen, RMS, Protokollkonverter, Bridges in der Transition, Malware, Nachweise, Betriebsmodelle). Ersetzt v1.0, Kap. 3 (Brownfield) entfällt |
| fitko_zapuk-zusammenfassung-v1.1_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Zusammenfassung/README.md | v1.1 | Management Summary Phase II: 27 neue ADRs, Neo-Validierung mit BW-Empfangsclient und GA-Lotse, Verantwortungsverteilung, Entscheidungsbedarf IT-PLR; Finanzierung erst zur 52. IT-PLR-Sitzung |
| fitko_zapuk-transitionen-ueberblick_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Transitionen/README.md | Stand 06.10.2026 | Übergreifende Transitionsstrategie: sieben Leitprinzipien, Vier-Phasen-Modell, Pfade für ZBP, MJP, MUK, Mein ELSTER |
| fitko_zapuk-transition-a-zentrales-buergerpostfach_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Transitionen/A--Zentrales-Bürgerpostfach/README.md | Stand 25.09.2026 | Transitionsplan ZBP: Matrix-Backend für Privatpersonen, Datenmigration, ZBP-Matrix-Bridge für die ZBP-API, Abschaltung |
| fitko_zapuk-transition-b-mein-justizpostfach_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Transitionen/B--Mein-Justizpostfach/README.md | Stand 25.09.2026 | Transitionsplan MJP: Justiz-Backend mit OSCI-Matrix-Protokollkonverter, Adressen `@[SAFE-ID]:domain`, EGVP bleibt |
| fitko_zapuk-transition-c-mein-unternehmenskonto_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Transitionen/C--Mein-Unternehmenskonto/README.md | Stand 06.10.2026 | Transitionsplan MUK: OZG-PLUS-Postfach direkt abgelöst (keine Bridge), Postfach 2.0 über Verweise und optional Postfach-2.0-Matrix-Bridge |
| fitko_zapuk-tadr-mein-elster-001-…_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Transitionen/Architekturentscheidungen/TADR-MEIN-ELSTER-001-… | Operativ entschieden, 25.09.2026 | Zuordnung von ELSTER-Datenarten zum zentralen ZaPuK-Postfach |
| fitko_zapuk-tadr-mein-elster-002-…_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Transitionen/Architekturentscheidungen/TADR-MEIN-ELSTER-002-… | Operativ entschieden, 25.09.2026 | Anbindungsmodell Mein ELSTER: Zustell-Bridge in der ELSTER-Domäne (Option C), später SDK (Option D) |
| fitko_zapuk-rechtsgutachten_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Rechtsgutachten/README.md | v1.1 | Rechtsgutachten Phase II (4.358 Zeilen); Auswertung im Ordner `recht-governance/` |
| fitko_zapuk-adr-index_v1.1-entwurf_2026-11-13.md | Zielarchitektur/Architekturentscheidungen/README.md | – | Wie `main`, Tippfehler korrigiert, ADR-0004 zusätzlich "eingeschränkt durch ADR-1145" |
| fitko_zapuk-glossar_v1.1-entwurf_2026-11-13.md | Glossar.md | – | Wie `main`, Eintrag Machtbereich präzisiert |
| fitko_zapuk-adr-0004-…_v1.1-entwurf_2026-11-13.md | ADR-0004 | Operativ entschieden, 15.04.2025 | Hinweis Phase 2 erweitert (Bereitstellungsbestätigung, Anschlussvertrag) |
| fitko_zapuk-adr-0018-…_v1.1-entwurf_2026-11-13.md | ADR-0018 | 13.11.2026 | Rechtsgrundlage präzisiert (§ 37 Abs. 2a SGB X, § 122a AO) |
| fitko_zapuk-adr-1101-…_v1.1-entwurf_2026-11-13.md | ADR-1101 | 13.11.2026 | Begründung "E2EE nicht strikt umgesetzt" statt "verletzt SIC_VER_ÜBE_026" |
| fitko_zapuk-adr-1105-…_v1.1-entwurf_2026-11-13.md | ADR-1105 | Richtung entschieden, 13.11.2026 | Zuordnungsdienst ohne Identitätsdaten |
| fitko_zapuk-adr-1106-…_v1.1-entwurf_2026-11-13.md | ADR-1106 | Richtung entschieden, 13.11.2026 | Option A angeglichen, Holder-Binding als offener Punkt |
| fitko_zapuk-adr-1110-…_v1.1-entwurf_2026-11-13.md | ADR-1110 | 13.11.2026 | Neuer Abschnitt Risiken (§ 9 OZG anzupassen, Parallelbetrieb, zweite Adresse) |
| fitko_zapuk-adr-1118-…_v1.1-entwurf_2026-11-13.md | ADR-1118 | 13.11.2026 | Offener Punkt: Nachweis der Benachrichtigung (§ 37 Abs. 2a SGB X) |
| fitko_zapuk-adr-1130-…_v1.1-entwurf_2026-11-13.md | ADR-1130 | 13.11.2026 | Hinweis im Postfach-Zugang, wenn bei Option E Klartext das Endgerät verlässt |
| fitko_zapuk-adr-1145-…_v1.1-entwurf_2026-11-13.md | ADR-1145 | 13.11.2026 | Neue Bedingung: Verwahrung der Wiederherstellungsschlüssel |
| fitko_zapuk-adr-1146-…_v1.1-entwurf_2026-11-13.md | ADR-1146 | 13.11.2026 | Postfachadresse steht im Key-Binding-JWT |
| fitko_zapuk-adr-1147-…_v1.1-entwurf_2026-11-13.md | ADR-1147 | Richtung entschieden, 13.11.2026 | Treiber Schlüsselverwaltung (ANF_ARC_ARC_050) ergänzt |

## Weitere Abrufe

| Datei | URL | Herausgeber | Datum | Inhalt kurz |
|---|---|---|---|---|
| fitko_zapuk-opencode-issues-architektur_2026-11-19.md | https://gitlab.opencode.de/api/v4/projects/3799/issues | Einreichende (MASTD RLP, Dataport, Privatperson); Tracker FITKO | Issues 02/2026–03/2026, alle geschlossen | Beschreibungen der Issues #23 (Peppol, ADR-0010), #24 (M2M und Maschinenidentitäten), #25 (API-first statt SDK-first, ADR-0009), #27 (Browser-Client und Betreiberzugriff); Antworten des ZaPuK-Teams nur mit Anmeldung lesbar (HTTP 401) |

## Nicht kopiert

- Bilder und Diagramme (`images/`, PNG, SVG, PUML, drawio) beider Branches, darunter die neuen Konzeptgrafiken `applikationssicht-verbund`, `komponenten-ueberblick`, `postfach-backend-dienste` und die Abbildungen der Transitionspläne. Die Textfassungen beschreiben die Inhalte.
- Dateien im Branch `architecture-v1.1`, die sich nur durch verschobene Link-Pfade unterscheiden: Architekturkonzept v1.0 und Zusammenfassung v1.0 (jetzt unter `Vorversionen/v1.0/`, Inhalt gleich `zapuk-main/`), Lesefassung der Anforderungen, Rahmenbedingungen, `Zielarchitektur/README.md`, beide Konsultationsseiten Phase 2, Programm-README.
- PDF-Fassungen (unverändert gegenüber `zapuk-main/` bzw. nur verschoben).
