# sources.md

Navigation map for the `sources/` folder. Use it to find the right source before reading. Paths are relative to `sources/`. German domain terms are kept as in the sources.

Context: `morph_box.csv` is the morphological box for the d-box (central, bi-directional Verwaltungspostfach, part of the Deutschland-Stack). The sources below feed its dimensions (Zeilen) and options (Ausprägungen).

## Overview

| Folder | What it is | Language | Format | Use it for |
|---|---|---|---|---|
| `zapuk-main/` | ZaPuK: Zielarchitektur für Postfach- und Kommunikationslösungen (FITKO / FIT-AB, IT-Planungsrat). 196 files. | German | Markdown, PDF | Architecture decisions, requirements, legal framework, glossary. The richest source for dimensions and options. |
| `neo-solution-architecture-develop/` | "Neo": Matrix-based, E2EE G2C communication channel (FITKO/BMDS), a child of ZaPuK. 64 files, arc42. | English | Markdown, Structurizr DSL | A concrete reference implementation: homeserver topology, BundID integration, message model, delivery legal concept. |
| `ELSTER/` | ERiC developer documentation package (ELSTER, BayLfSt), ERiC 44.1.6.0, May 2026. About 2,100 files, 227 MB. | German | PDF, XSD, XML, ODS, HTML | How an existing German mailbox works technically: ELSTER-Postfach and Postfach 2.0, Datenabholung, Bereitstellung by other authorities, certificates, encryption, DIVA (elektronische Bekanntgabe), Vollmachten. Reference for the `ELSTER-Postfach` column in `existing_solutions.csv`. |
| `BundID-ZBP/` | Zentrales Bürgerpostfach (ZBP, "BundID-Postfach"): condensed analysis plus local copies of primary sources from the BundID operator, FITKO / FIT-Connect, IT-PLR, Bundestag and Bundesrat. 94 files, retrieved 2026-11-17. | German | Markdown, PDF, JSON Schema, OpenAPI, Java | How the existing federal citizen mailbox works: operation and governance, Anbindung (direct REST or FIT-Connect ZBP-Adapter), Postkorb-Handle addressing, Bekanntgabe nach § 9 OZG, legal history. Reference for the `BundID heute (Deutschland)` column in `existing_solutions.csv`. |
| `Referenzen-ELSTER.md` | Source list for the ELSTER-Postfach and Postfach 2.0: short codes, local paths in `ELSTER/Dev Dokumentation/` and URLs, evidence per dimension. Checked 2026-11-17. | German | Markdown | Reference for the `ELSTER-Postfach (Deutschland)` column in `existing_solutions.csv` and the ELSTER entries in `morph_box.csv`. |
| `Referenzen-BundID-ZBP.md` | Source list for the Zentrales Bürgerpostfach (BundID): short codes, local paths in `BundID-ZBP/quellen/` and URLs, evidence per dimension, deviations found during the check. Checked 2026-11-17. | German | Markdown | Reference for the `BundID heute (Deutschland)` column in `existing_solutions.csv` and the BundID entries in `morph_box.csv`. |
| `Referenzen-Mein-Postkorb-Oesterreich.md` | Source list for the Austrian E-Zustellung ("Mein Postkorb"): ZustG, E-GovG, BKA white paper and ZUSE specification, oesterreich.gv.at, USP, with evidence per dimension. Links only, no local copies. | German | Markdown | Reference for the `Österreich` column in `existing_solutions.csv` and the Österreich entries in `morph_box.csv`. |
| `Referenzen-GOV-UK-Notify.md` | Source list for GOV.UK Notify (UK, GDS): service pages on features, security, pricing, data retention, files, received text messages, plus REST API documentation, with evidence per dimension. Links only, no local copies. | German | Markdown | Reference for the `UK (kein zentrales Postfach)` column in `existing_solutions.csv` and the GOV.UK Notify entries in `morph_box.csv`. |
| `Referenzen-Digital-Post-Daenemark.md` | Source list for the Danish Digital Post: Lov om Digital Post fra offentlige afsendere (LBK nr. 686/2021), Digitaliseringsstyrelsen pages on platforms, sender and recipient systems, Kontaktstruktur, Rettighedsportal, visningsklienter, plus borger.dk help, with evidence per dimension. Links only, no local copies. | German | Markdown | Reference for the `Dänemark` column in `existing_solutions.csv` and the Dänemark entries in `morph_box.csv`. |
| `Referenzen-Digital-Postkasse-Norwegen.md` | Source list for the Norwegian Digital postkasse til innbyggere (DPI): eForvaltningsforskriften and forvaltningsloven (Lovdata), Digdir documentation on DPI, KRR, eFormidling and ID-porten, Altinn Melding and Autorisasjon, plus Digipost and e-Boks, with evidence per dimension. Links only, no local copies. | German | Markdown | Reference for the `Norwegen` column in `existing_solutions.csv` and the Norwegen entries in `morph_box.csv`. |

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

## 3. ELSTER/

Unpacked `ERiC-<version>-Dokumentation.zip` from the ELSTER developer area (requires a Hersteller registration). It documents ERiC (ELSTER Rich Client), the C library through which tax software, payroll and accounting software, and other authorities talk to the ELSTER servers. It is not product or governance documentation: Mein ELSTER (web portal), ElsterSecure, registration and operation are only mentioned in passing.

**Start with `ELSTER/ELSTER-Zusammenfassung.md`** (German): a condensed analysis of ELSTER along all `morph_box.csv` dimensions, with chapter and page pointers into this package and the relevant web sources.

**`Referenzen-ELSTER.md`** (German, next to the folder): source list in the style of `Referenzen-GOV-UK-Notify.md`, with short codes and evidence per dimension for the `ELSTER-Postfach (Deutschland)` column in `existing_solutions.csv` and the ELSTER entries in `morph_box.csv`, checked on 2026-11-17, plus the deviations found during the check.

All paths below are relative to `ELSTER/Dev Dokumentation/`. Most of the volume is per-tax schemas (`Datenarten/ElsterErklaerung`, `ElsterAnmeldung`, `ElsterBilanz`, `ElsterLohn*`) and `Deltadokumentation/`, which are irrelevant for the d-box.

### Entry points (reading order)

1. `Liesmich.txt`: package overview and recommended reading order.
2. `ERiC-Entwicklerhandbuch.pdf` (514 pages). Relevant chapters only:
   - 5.2: certificates (CEZ, Portalzertifikat POZ, Ad-hoc-Zertifikat for nPA/eAT), Vorgangsarten `send-Auth` / `send-NoSig`, nPA via ELSTER eID-Server.
   - 5.3.13: TLS cipher suites.
   - 5.4.11: attachments, server-side malware scan and removal of active content, AnhangRueckmeldung.
   - 6.5: Hersteller-ID, test tax offices, Testmerker.
   - 7.4 and 7.5: Bescheiddatenrückübermittlung, elektronische Bekanntgabe Stufe 1 and 2.
   - 9.2 and 9.3: ElsterDatenabholung (PostfachAnfrage, PostfachBestaetigung, PostfachStatus), list of Bereitstellungsdatenarten, deletion periods, OTTER object store and the `otto` client.
   - 9.10: DauerPostBeStAntrag / -Widerruf (permanent opt-out to paper under § 122a AO).
   - 9.11: ElsterNachricht, the citizen-to-tax-office channel (Einspruch, SonstigeNachrichten, Belegnachreichung, Fristverlängerung …).
   - 9.15: Bescheide und Mitteilungen von Organisationen außerhalb der Finanzverwaltung (Postfach 2.0: EPBescheid, EPMitteilung, EPKurzmitteilung, EPAntwort, OZGPPNachricht) and the reply mechanism.
   - 9.16 and 9.17: DIVA Einwilligungserklärung, Vollmachten, Abrufvollmacht, Kammer-Vollmachtsdatenbank.
3. `Datenarten/ElsterBereitstellung/Dokumentation/ElsterBereitstellung-SSB-V5.0.8.pdf`: sender-side interface into the ELSTER-Postfach and Postfach 2.0 (BEM, BEST-DB, addressing via accountId / pseudonymous `ek-` ID / AGS, WIESEL pool key, e-mail notification, DIVA Ordnungsbegriffe, EPAntwort).
4. `Datenarten/ElsterDatenabholung/Dokumentation/ElsterDatenabholung-v32.pdf`: recipient-side interface (Anfrage, Abholung, Bestätigung), Vertretung (in test), Statusabfrage with status codes 05 to 99 and Abrufzeitpunkt.
5. `ElsterBasisSchema/Dokumentation/Einheitliche_Datenschnittstelle_XML_11.pdf`: ElsterXML envelope, CMS hybrid encryption (RSA-OAEP 3072 / AES-256-CBC), HTTPS to the Clearingstelle.
6. `lizenz.pdf`: ERiC licence (Freistaat Bayern / BayLfSt as bundesweiter Koordinator; free of charge; purpose limited to §§ 72a, 87b-87d AO; blocking of Hersteller-ID; § 18 Postfach as IT-Komponente nach § 2 Abs. 7 OZG for other authorities).

### Other content

- `Tutorial/ERiC-Tutorial.pdf` and `Tutorial/Beispiele/PostfachAnfrage.xml`, `PostfachBestaetigung.xml`: worked examples.
- `Datenarten/ElsterDatenabholung/DatenartBereitstellung/*/Dokumentation/*.pdf`: small specs for AnhangRueckmeldung, DIVA-Rückmeldungen, EBilanzRueckmeldung.
- `Datenarten/ElsterBRM/`: DIVA Einwilligung and Vollmachten schemas; `Vollmachten/Dokumentation/20240306_Vollmachten_fuer_LStHV_SSb-V01.00.pdf`.
- `Datenarten/ElsterNachricht/Einspruch/Zusatzinformationen_zur_Datenart_Einspruch.pdf`.
- `API-Referenz/` (Doxygen HTML and PDF), `API-Rueckgabe-Schemata/`, `Unterstuetzte_Signaturkarten_und_Sticks/`: API detail, rarely needed.
- `ERiC-Releasenotes.pdf`: change history.

### Caveats

- Developer view only: no statements on governance, costs, operating sites, Mein ELSTER UI or ElsterSecure.
- Partly outdated legal text: the handbook still states that a Bescheid is deemed bekannt gegeben on the third day after the notification (7.5, footnotes 47 and 48), whereas § 122a Abs. 4 AO uses the fourth day after Bereitstellung since 1 January 2025 (Postrechtsmodernisierungsgesetz). Legal sources and other web findings are listed in `ELSTER/ELSTER-Zusammenfassung.md`, section 7.
- Attachment rules per Datenart are in the XSDs, e.g. `Datenarten/ElsterNachricht/Belegnachreichung/Schema/24/Belegnachreichung-24-Nutzdaten.xsd` (PDF only, 20 attachments, 100 pages).
- The Datenabholung Vertretung feature is marked as test operation only (SSb v32, 6.1).

## 4. BundID-ZBP/

Web research on the Zentrales Bürgerpostfach of the BundID (live since March 2024, operated by ITZBund for the BMDS, developed by AKDB). Receive-only for natural persons; replies only when the Behörde enables them. Neo and ZaPuK plan to replace it.

**Start with `BundID-ZBP/ZBP-Zusammenfassung.md`** (German): a condensed analysis of the ZBP along all `morph_box.csv` dimensions (section 5), with short codes for sources (DSE, FAQ, FC-T, D5367 …), evidence markers (**[Folgerung]**, **[unverifiziert]**, **[veraltet?]**), contradictions (section 6) and gaps (section 7).

**`Referenzen-BundID-ZBP.md`** (German, next to the folder): source list in the style of `Referenzen-GOV-UK-Notify.md`, with short codes, local paths and URLs, evidence per dimension for the BundID entries in `morph_box.csv`, checked on 2026-11-17, plus the deviations from `ZBP-Zusammenfassung.md` found during the check.

Each subfolder of `BundID-ZBP/quellen/` has an `_index.md` with URL, publisher, date and a content note per file. Read the index before opening a file.

### Subfolders

- `BundID-ZBP/quellen/bundid-betreiber/`: operator and product view. id.bund.de privacy notice (Postkorb-Handle, deletion periods), FAQ, Postfach UI texts and the full web text bundle, Impressum; Personalausweisportal pages (SSP onboarding, Postfach-Zertifikate); the 2024 migration notice; practice reports (SID Sachsen on Bescheidübermittlung, M-V on Bekanntgabe nach § 9 OZG, Dataport on Postfachreferenz); AKDB DSGVO information sheet.
- `BundID-ZBP/quellen/fitko-fit-connect/`: sender-side Anbindung. FIT-Connect docs on the ZBP (`fc-docs_zbp-entscheider.md`, `fc-docs_zbp-technische-anbindung.md`, `fc-docs_sdk-rezept-zbp-anbinden.md`), reply channels, metadata, encryption, authentication, limits, roadmap; ZBP-Adapter checklist and process diagrams; ZBP message JSON Schema, FIT-Connect metadata schemas and OpenAPI specs; Java SDK `zbp-client` code (endpoints, token, envelope signature); IT-PLR 2024/28 Bebauungsplan and 2025/28 ZaPuK overview.
- `BundID-ZBP/quellen/recht-governance/`: law, politics, history. OZG (current version) and § 41 VwVfG; OZGÄndG legislative documents (BT-Drs. 20/8093, 20/10417, 20/11790); DIdG documents (BR-Drs. 322/26, 322/26B, 618/26, BT-Drs. 21/8617); Bundestag answers (BT-Drs. 20/9824, 21/5367 on Statusmonitor and Bidirektionalität, 21/9020 on Deutschland-Stack); BfDI report 2024; IT-PLR Beschlüsse 2022/04 (with Whitepaper), 2023/29 (with Statusbericht), 2024/28, 2025/28, B-2026/03-IT, B-2026/18-IT, B-2026/19-IT; Organisationserlass 2025; Föderale Modernisierungsagenda.

### Caveats

- The actual ZBP API documentation (allowed HTML tags, error codes, current limits) is in the BundID-SSP Hilfecenter, which needs a login. It is not stored here.
- Some statements (encryption at rest, Abruf- and Lesebestätigung) are only documented for 2022 (Whitepaper 2022/04).
- Bidirektionalität: the Bundesregierung announced it for July 2026 (BT-Drs. 21/5367); the FIT-Connect roadmap still marks it as open.
- A few PDFs exist in two subfolders (SID Kundenforum 2025, FITKO "stellt vor: Neo" 2026-01-23).

## Which source for which morph_box topic

| Topic | Start with |
|---|---|
| Anbindung von Fachverfahren / Behörden | zapuk ADR-0008, 0009, 1145, 1148; neo `03_context_and_scope.md` |
| Topologie, Betrieb, Hosting | zapuk ADR-0002 to 0007, 1103, 1114; neo ADR-0001 to 0003, `structurizr/model/backend.dsl` |
| Bestandssysteme, Transition, Dekommissionierung | zapuk Architekturkonzept Ch. 3, ADR-0010 |
| Adressierung | zapuk ADR-1104, 1105, 1111 |
| Identität, Authentifizierung, EUDI-Wallet | zapuk ADR-0013 to 0016, 1106 to 1110, 1146; neo structurizr, glossary |
| Verschlüsselung, Sicherheit | zapuk ADR-0011, 0012, 1100, 1101, 1130, 1147; neo `02_architecture_constraints.md` |
| Zustellung, Bescheid, Fristen, Rechtssicherheit | zapuk ADR-0018, 1118 to 1120, Rahmenbedingungen; neo `concepts/303-*.md` |
| Benachrichtigung | zapuk (open ADR, D-Stack Beschluss 2026/18-IT) |
| Funktionspostfächer, Vertretung | zapuk ADR-1102, 1109, 1112, 1113 |
| Aufbewahrung, Veraktung | zapuk ADR-1135 |
| Clients / Frontends | neo `03_context_and_scope.md` |
| Governance, Länder, Finanzierung | zapuk README Beschlüsse, Zusammenfassung 3.3 and 4.x |
| Requirements as decision criteria | zapuk `Konsolidierte-Anforderungen/README.md` |
| ELSTER as existing solution (Postfach, Postfach 2.0, Bekanntgabe, Vollmacht, Anbindung) | `Referenzen-ELSTER.md` for evidence per dimension, `ELSTER/ELSTER-Zusammenfassung.md` first, then `ELSTER/`: Entwicklerhandbuch 5.2, 7.5, 9.2, 9.3, 9.15 to 9.17; `ElsterBereitstellung-SSB`; `ElsterDatenabholung-v32` |
| BundID / ZBP as existing solution (Anbindung, Postkorb-Handle, Bekanntgabe nach § 9 OZG, Betrieb) | `Referenzen-BundID-ZBP.md` for evidence per dimension, `BundID-ZBP/ZBP-Zusammenfassung.md` for context, then the `_index.md` of the relevant `BundID-ZBP/quellen/` subfolder |
| Rechtsrahmen Bürgerkonto und Postfach (OZG, DIdG, IT-PLR Beschlüsse) | `BundID-ZBP/quellen/recht-governance/_index.md`; zapuk Rahmenbedingungen |
| Österreich / Mein Postkorb as existing solution | `Referenzen-Mein-Postkorb-Oesterreich.md` |
| UK / GOV.UK Notify as existing solution | `Referenzen-GOV-UK-Notify.md` |
| Dänemark / Digital Post as existing solution | `Referenzen-Digital-Post-Daenemark.md` |
| Norwegen / Digital postkasse til innbyggere as existing solution | `Referenzen-Digital-Postkasse-Norwegen.md` |

## Gaps and caveats

- No concrete hosting, cost, sizing or SLA data exists in neo (empty chapters). For cost, see zapuk `Zusammenfassung` 3.3.
- The ZaPuK architecture documents (v1.0) are partly outdated relative to the later ADRs.
