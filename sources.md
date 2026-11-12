# sources.md

Navigation map for the `sources/` folder. Use it to find the right source before reading. Paths are relative to `sources/`. German domain terms are kept as in the sources.

Context: `morph_box.csv` is the morphological box for the d-box (central, bi-directional Verwaltungspostfach, part of the Deutschland-Stack). The sources below feed its dimensions (Zeilen) and options (Ausprägungen).

## Overview

| Folder | What it is | Language | Format | Use it for |
|---|---|---|---|---|
| `zapuk-main/` | ZaPuK: Zielarchitektur für Postfach- und Kommunikationslösungen (FITKO / FIT-AB, IT-Planungsrat). 196 files. | German | Markdown, PDF | Architecture decisions, requirements, legal framework, glossary. The richest source for dimensions and options. |
| `neo-solution-architecture-develop/` | "Neo": Matrix-based, E2EE G2C communication channel (FITKO/BMDS), a child of ZaPuK. 64 files, arc42. | English | Markdown, Structurizr DSL | A concrete reference implementation: homeserver topology, BundID integration, message model, delivery legal concept. |
| `d-box-assets-2026-11-11/` | d-box project context: StS meeting notes, kick-off decks, Miro board exports. 6 binary files. | German | docx, pptx, PDF | Scope, Prämissen, timeline, purpose of the morphological box. |

## 1. zapuk-main/

Mirror of the openCode repo ZaPuK. Document repo, no code. Target architecture for the federal mailbox and communication infrastructure (G2C/G2B, Bescheide, Rückfragen). It is meant to consolidate BundID-Postfach, ZBP, MJP, OZG-Plus-Postfach, TI-Messenger, BundesMessenger, EGVP and others.

Core stance: hybrid, federated architecture. Matrix as Kommunikationsschicht (ADR-0011). MLS for E2EE (ADR-0012). One central Postfach-Backend and one central Zugang, plus openness for third parties. Self-Service-Portal and SDKs. No messaging bridges for existing systems; they must migrate to Matrix and MLS (Architekturkonzept 3.2). Interoperability with systems outside the scope, e.g. EU systems, only via Protocol Converters that keep E2EE (ADR-0010).

Status caveat: `Zielarchitektur/Architekturkonzept` and `Zusammenfassung` reflect v1.0 (summer 2025). The ADRs from 1100 onward are more current (Sep-Nov 2026).

### Entry points (reading order)

1. `zapuk-main/README.md`: programme overview, IT-PLR Beschlüsse, roadmap, Logbuch (changelog).
2. `zapuk-main/Zielarchitektur/README.md`: document map.
3. `zapuk-main/Zielarchitektur/Architekturentscheidungen/README.md`: ADR index with status. Best table for dimensions and options.
4. `zapuk-main/Zielarchitektur/Architekturkonzept/README.md`, section 2.5: ADR summaries by topic. Chapter 3: Brownfield and transition.
5. `zapuk-main/Zielarchitektur/Zusammenfassung/README.md`: management summary, Kosten (3.3), implications, risks.
6. `zapuk-main/Zielarchitektur/Rahmenbedingungen/README.md`: laws and Beschlüsse.
7. `zapuk-main/Glossar.md` and `zapuk-main/Abkürzungen.md`: terminology.

### Other content

- `zapuk-main/Konsolidierte-Anforderungen/README.md`: requirements v0.7.0 with IDs (ANF_FUN_*, ANF_ARC_*, RAH_*, DAT_*, KOM_*, BED_*, LEI_*, WAR_*). `Lesefassung/` holds a readable version. `Vorversionen/` holds history.
- `zapuk-main/Konsultationen/`: consultation rounds (Phase 1 requirements 2025-03-19, Phase 2 architecture 2026-01-21, ongoing feedback).
- `zapuk-main/Zielarchitektur/Architekturentscheidungen/ADR-*.md`: one file per ADR (Status, Kontext, Optionen, Entscheidung). Option diagrams are in `images/`.
- Low value: `.gitlab/`, `Foliensätze/` (slide PDFs duplicating the docs), PDF copies of the .md files, `images/`.

### Topic to ADR map

| Topic | ADRs (in `Zielarchitektur/Architekturentscheidungen/`) |
|---|---|
| Topologie, Betrieb, Bereitstellung | 0001, 0002, 0003, 0004, 0005, 0006, 0007, 1103 (Betriebsmodelle Mehrbenutzer-Organisationen), 1114 (getrennter Betrieb nach Teilnehmendengruppen) |
| Anbindung, Onboarding | 0008 (Self-Service-Portal), 0009 (SDKs), 1145 (Anschlussbedingungen, Föderationskontrolle), 1148 (Onboarding im SSP) |
| Interoperabilität / Bestandssysteme | 0010 (Bridge, Protocol Converter, none, full standardization; chosen: Protocol Converter). Bridge ban and Channel Adapter for existing systems: Architekturkonzept 3.2 |
| Protokoll, Verschlüsselung | 0011 (Matrix), 0012 (MLS/E2EE), 1100, 1101, 1147 (Schlüsselwiederherstellung), 1144 (PKI) |
| Identität, Authentizität | 0013-0017, 1106-1108, 1110 (Entkopplung Nutzerkonto und Postfach), 1146 (EUDI-Wallet) |
| Adressierung | 1104 (MXID, Domain-Governance), 1105 (pseudonyme Adressen), 1111 (organisationsbezogene Adressen) |
| Funktionspostfächer, Vertretung | 1102, 1109, 1112, 1113 |
| Zustellung, Nachweis | 0018 (Bereitstellungsbestätigung), 1118 (Abruf-/Lesebestätigung), 1119 (Übermittlungsoptionen), 1120 (beweissichere Anhänge) |
| Sicherheit, Aufbewahrung | 1130 (Malware-Prüfung unter E2EE), 1135 (Veraktung, Aufbewahrung) |
| Still open (no file) | API-Autorisierung Server-zu-Server (1149), Benachrichtigungsdienst, Löschung/Aufbewahrung, Metadatenminimierung |

### Legal framework (`Zielarchitektur/Rahmenbedingungen/README.md`)

eIDAS 2.0, EGovG, OZG/OZGÄndG, SDG-VO, VwVfG, VwZG. Possibly relevant: AO, BFSG, BITV 2.0, BDSG, BSIG, IDNrG. IT-PLR Beschlüsse 2024/28, 2025/28, 2026/03-IT, 2026/04-IT, 2026/18-IT (D-Stack Basiskomponente Postfach/Benachrichtigung).

## 2. neo-solution-architecture-develop/

Snapshot of the public FITKO repo "Matrix G2C/G2X Communication Solution Architecture" (arc42 v8.2). Status: concept. Meant to be integrated into BundID, replacing the Zentrales Bürgerpostfach (ZBP). Chapters 04, 06, 07, 08, 10 and 11 are empty template stubs. There is no concrete hosting, sizing, cost or SLA information.

### Entry points (reading order)

1. `neo-solution-architecture-develop/README.md`: arc42 table of contents.
2. `neo-solution-architecture-develop/01_introduction_and_goals.md`: drivers BD1-BD8, goals BG1-BG6, personas.
3. `neo-solution-architecture-develop/03_context_and_scope.md`: business and technical context (Matrix homeserver next to ZBP, FIT-Connect SDK extension, BundID web client, mobile app with AusweisApp SDK).
4. `neo-solution-architecture-develop/02_architecture_constraints.md`: technical, organisational and regulatory constraints (OZG, DSGVO, LB items, BSI TR).
5. `neo-solution-architecture-develop/architecture_decisions/`: ADR-0001 to 0003 (separate homeservers for Privatpersonen and Behörden; separate operators; exactly one homeserver per group, no bring-your-own) and ADR-0004 (multi-events). Index: `09_architecture_decisions.md`.
6. `neo-solution-architecture-develop/structurizr/model/backend.dsl` and `relations.dsl`: C4 model (Synapse, MAS, PostgreSQL, Keycloak as proxy IdP, bPK2 as account ID). Root: `structurizr/bundid-pw-auth.dsl`.
7. `neo-solution-architecture-develop/concepts/303-delivery-retrieval-and-read-receipts.md`: legal concept for notification and deadlines (§ 9 OZG as default, exceptions in AO, SGB X, VwVfG, VwZG, ZPO, BGB, ERVV).
8. `neo-solution-architecture-develop/12_glossary.md`: DE/EN terms.

### Other content

- `neo-solution-architecture-develop/05_building_block_view.md`: Matrix event and message format spec (58 KB). Skim by headings. It matters only for message-model dimensions.
- `neo-solution-architecture-develop/images/high-level-architecture.png`: overview diagram.
- Process or meta only (skip): `13_methodology.md`, `docs/`, `CONTRIBUTING.md`, `SECURITY.md`, `LICENSES/`, `publiccode.yml`, CI and lint config.
- The parent ZaPuK is referenced only by link. It is the `zapuk-main/` source.

## 3. d-box-assets-2026-11-11/

Six binary project-context documents (Oct-Nov 2026), with no code or CSV. `quellen/` is empty. Versioning is by filename. Text can be extracted from the docx, pptx and `Kick-Start` PDF. The three `d-box_Projekt Management*.pdf` files are image-only Miro exports with no text layer; they need visual reading or OCR.

| File | Content | Authority |
|---|---|---|
| `d-box-assets-2026-11-11/d-box_Projekt Management.pdf` | Miro overview of the work plan. Explains what the morphological box is for (rows = necessary decisions, columns = known options), the next steps, and the work packages 1a, 1b, 2, 3 and 4. | Best source for the box's purpose and method. Date unclear. |
| `d-box-assets-2026-11-11/261030_DCDS_BMDS_Kick-Start.pptx.pdf` | Kick-off, 30.10.2026. d-box goals, 4-step approach, phases (Zielbild by 27.11.2026, MVP by Q2 2027, rollout by end 2027), teams. | Most current for scope and timeline. |
| `d-box-assets-2026-11-11/20261021 - DCDS __ BMDS ZaPuK StS Richter.docx` | Gesprächsnotiz of 21.10.2026 with StS Richter. Prämissen: ID = Modul 1, Postfach = Modul 2, bidirektional, Bescheidzustellung, "so klein wie möglich", keine eigenen Clients, E2E, ZaPuK-fähiger Messenger-Layer, FIT-Connect and 4-Corner. | Hard constraints, but informal notes, not a decision. |
| `d-box-assets-2026-11-11/d-box_Projekt Management_Frage 4_v2.pdf` | Miro zoom on Frage 4 (Recht, Bestand, zentrale Dienste, Governance, Finanzierung), with DoD and the Bestand strand (BundID-Postfach, ELSTER, Postfach 2.0, Mein Justizpostfach, ZBP, ...). | Current. Supersedes v1. |
| `d-box-assets-2026-11-11/d-box_Projekt Management_Frage 4.pdf` | Earlier version of the above. | Outdated. |
| `d-box-assets-2026-11-11/261015_DCDS_BM Termin_Storyboard_v6_DS.pptx` | DCDS-wide storyboard (operating model, budget, d-box/d-app/SPARK/d-deploy). Slide 14 covers the d-box. | Background only. |

## Which source for which morph_box topic

| Topic | Start with |
|---|---|
| Anbindung von Fachverfahren / Behörden | zapuk ADR-0008, 0009, 1145, 1148; neo `03_context_and_scope.md`; assets Prämissen (FIT-Connect) |
| Topologie, Betrieb, Hosting | zapuk ADR-0002 to 0007, 1103, 1114; neo ADR-0001 to 0003, `structurizr/model/backend.dsl` |
| Bestandssysteme, Transition, Dekommissionierung | zapuk Architekturkonzept Ch. 3, ADR-0010; assets Kick-Start p. 8 and 10, Frage 4 v2 |
| Adressierung | zapuk ADR-1104, 1105, 1111 |
| Identität, Authentifizierung, EUDI-Wallet | zapuk ADR-0013 to 0016, 1106 to 1110, 1146; neo structurizr, glossary |
| Verschlüsselung, Sicherheit | zapuk ADR-0011, 0012, 1100, 1101, 1130, 1147; neo `02_architecture_constraints.md` |
| Zustellung, Bescheid, Fristen, Rechtssicherheit | zapuk ADR-0018, 1118 to 1120, Rahmenbedingungen; neo `concepts/303-*.md` |
| Benachrichtigung | zapuk (open ADR, D-Stack Beschluss 2026/18-IT); assets Prämissen (AU/UK) |
| Funktionspostfächer, Vertretung | zapuk ADR-1102, 1109, 1112, 1113 |
| Aufbewahrung, Veraktung | zapuk ADR-1135 |
| Clients / Frontends | neo `03_context_and_scope.md`; assets Prämissen (keine eigenen Clients, D-App) |
| Governance, Länder, Finanzierung | zapuk README Beschlüsse, Zusammenfassung 3.3 and 4.x; assets Frage 4 v2, Kick-Start p. 8 |
| Requirements as decision criteria | zapuk `Konsolidierte-Anforderungen/README.md` |

## Gaps and caveats

- No concrete hosting, cost, sizing or SLA data exists in neo (empty chapters). For cost, see zapuk `Zusammenfassung` 3.3.
- Reference models from other countries (CH, AU, UK, DK, EU) are mentioned in the assets but not documented in any source here.
- The Bestand analysis (ELSTER, MuK, MJP, ZBP) is only a plan in the Miro exports, with no content yet.
- The Miro PDFs are image-only and the details in this map come from a visual read, so verify before quoting.
- The ZaPuK architecture documents (v1.0) are partly outdated relative to the later ADRs.
