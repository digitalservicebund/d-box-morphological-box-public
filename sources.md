# sources.md

Navigation map for the `sources/` folder. Use it to find the right source before reading. Paths are relative to `sources/`. German domain terms are kept as in the sources.

Context: `morph_box.csv` is the morphological box for the d-box (central, bi-directional Verwaltungspostfach, part of the Deutschland-Stack). The sources below feed its dimensions (Dimensionen) and options (Ausprägungen).

## Overview

| Folder | What it is | Language | Format | Use it for |
|---|---|---|---|---|
| `ZaPuK/quellen/zapuk-v1.1.1/` | ZaPuK: Zielarchitektur für Postfach- und Kommunikationslösungen (FITKO / FIT-AB, IT-Planungsrat), local copy of tag v1.1.1 (26.11.2026), the annexes to IT-PLR B-2026/43-IT. 217 files. | German | Markdown, PDF | Architecture decisions, Architekturkonzept v1.1.1 and v1.0, Überblick, Transitionsstrategie, Rechtsgutachten, requirements, glossary. The richest source for dimensions and options. |
| `zapuk-v1.0.0-121-g7419d8b/` | Older mirror of the ZaPuK repo at upstream commit `7419d8b` (2026-11-11, 121 commits after tag v1.0.0, `git describe`: v1.0.0-121-g7419d8b), superseded by `ZaPuK/quellen/zapuk-v1.1.1/` (see `ZaPuK/Aktualisierung-v1.1.1.md`). Kept until the remaining references in other source folders are updated. | German | Markdown, PDF | Do not use for new work. |
| `neo-solution-architecture-develop/` | "Neo": Matrix-based, E2EE G2C communication channel (FITKO/BMDS), a child of ZaPuK. 64 files, arc42. | English | Markdown, Structurizr DSL | A concrete reference implementation: homeserver topology, BundID integration, message model, delivery legal concept. |
| `ELSTER/` | ERiC developer documentation package (ELSTER, BayLfSt), ERiC 44.1.6.0, May 2026. About 2,100 files, 227 MB. | German | PDF, XSD, XML, ODS, HTML | How an existing German mailbox works technically: ELSTER-Postfach and Postfach 2.0, Datenabholung, Bereitstellung by other authorities, certificates, encryption, DIVA (elektronische Bekanntgabe), Vollmachten. Reference for the `ELSTER-Postfach` column in `existing_solutions.csv`. |
| `BundID-ZBP/` | Zentrales Bürgerpostfach (ZBP, "BundID-Postfach"): condensed analysis plus local copies of primary sources from the BundID operator, FITKO / FIT-Connect, IT-PLR, Bundestag and Bundesrat. 94 files, retrieved 2026-11-17. | German | Markdown, PDF, JSON Schema, OpenAPI, Java | How the existing federal citizen mailbox works: operation and governance, Anbindung (direct REST or FIT-Connect ZBP-Adapter), Postkorb-Handle addressing, Bekanntgabe nach § 9 OZG, legal history. Reference for the `BundID heute (Deutschland)` column in `existing_solutions.csv`. |
| `Referenzen-ELSTER.md` | Source list for the ELSTER-Postfach and Postfach 2.0: short codes, local paths in `ELSTER/Dev Dokumentation/` and URLs, evidence per dimension. Checked 2026-11-17. | German | Markdown | Reference for the `ELSTER-Postfach (Deutschland)` column in `existing_solutions.csv` and the ELSTER entries in `morph_box.csv`. |
| `Referenzen-BundID-ZBP.md` | Source list for the Zentrales Bürgerpostfach (BundID): short codes, local paths in `BundID-ZBP/quellen/` and URLs, evidence per dimension, deviations found during the check. Checked 2026-11-17. | German | Markdown | Reference for the `BundID heute (Deutschland)` column in `existing_solutions.csv` and the BundID entries in `morph_box.csv`. |
| `MUK/` | Mein Unternehmenskonto (MUK), the einheitliches Organisationskonto under § 2 Abs. 5 and § 3 Abs. 2 OZG with Postfach 2.0 (ELSTER, Bayern) and OZG-PLUS-Postfach (Governikus, Bremen): condensed analysis plus local copies of primary sources from StMD/BayLfSt, ELSTER, Governikus, Dataport, IT-PLR, Bundestag and Bundesrat. 123 files, retrieved 2026-11-18. | German | Markdown, PDF, OpenAPI | How the existing organisation account and its mailboxes work: ELSTER-Transfer delivery, pseudonymous Postfachhandle, Funktionspostfächer with E2EE, Berechtigungssteuerung, governance by Bayern and Bremen, mandatory use for authorities under § 3 Abs. 3 OZG. Reference for the `Unternehmenskonto heute (Deutschland)` column in `existing_solutions.csv`. |
| `Referenzen-MUK.md` | Source list for Mein Unternehmenskonto: short codes, local paths in `MUK/quellen/` and URLs, evidence per dimension, deviations found during the check. Checked 2026-11-18. | German | Markdown | Reference for the `Unternehmenskonto heute (Deutschland)` column in `existing_solutions.csv` and the MUK entries in `morph_box.csv`. |
| `MJP/` | Mein Justizpostfach (MJP), the free browser mailbox for natural persons in the elektronischer Rechtsverkehr with courts (sicherer Übermittlungsweg under § 130a Abs. 4 S. 1 Nr. 4 ZPO with § 13 ERVV): condensed analysis plus local copies of primary sources from the BLK-AG IT-Standards, MJP-Support, EGVP, BMJV, Westernacher, Bundestag and Bundesgesetzblatt. 102 files, retrieved 2026-11-18. | German | Markdown, PDF, JSON | How the existing justice mailbox for citizens works: EGVP/OSCI transport, SAFE directory, BundID login at Vertrauensniveau hoch, E2EE with a key file kept only by the user, förmliche Zustellung under § 173 ZPO, governance by the BLK. Reference for the `Mein Justizpostfach (Deutschland)` column in `existing_solutions.csv`. |
| `Referenzen-MJP.md` | Source list for Mein Justizpostfach: short codes, local paths in `MJP/quellen/` and URLs, evidence per dimension, deviations found during the check. Checked 2026-11-18. | German | Markdown | Reference for the `Mein Justizpostfach (Deutschland)` column in `existing_solutions.csv` and the MJP entries in `morph_box.csv`. |
| `ZaPuK/` | ZaPuK as a column of its own: condensed analysis, the v1.1.1 repo copy, update notes `Aktualisierung-v1.1.1.md`, draft Transitionspläne for ZBP, MJP, MUK and Mein ELSTER (only in the former branch `architecture-v1.1`, copied 2026-11-13, not in v1.1.1), IT-PLR Beschluss B-2026/43-IT with annexes, FIT-AB Beschlüsse, Neo pilot results and press. Retrieved 2026-11-19, updated to v1.1.1 on 2026-11-27. | German | Markdown, PDF, PNG | What ZaPuK decides per dimension (ADR status, draft v1.1, Neo pilot), transition of existing mailboxes, legal feasibility (Schriftform, Bekanntgabe, Zustellung, Verfassungsrecht), governance and financing. Reference for the `ZaPuK (Zielarchitektur, Deutschland)` column in `existing_solutions.csv`. |
| `Referenzen-ZaPuK.md` | Source list for ZaPuK: short codes, local paths in `ZaPuK/quellen/` (including `zapuk-v1.1.1/`), URLs, evidence per dimension, deviations found when checking the ZaPuK references in `morph_box.csv`. Checked 2026-11-19, against v1.1.1 on 2026-11-27. | German | Markdown | Reference for the `ZaPuK (Zielarchitektur, Deutschland)` column in `existing_solutions.csv` and the ZaPuK entries in `morph_box.csv`. |
| `Mein-Postkorb-Oesterreich/` | Austrian E-Zustellung ("Mein Postkorb"): condensed analysis plus local copies of ZustG and E-GovG (RIS), BKA white paper, ZUSE 2.4.0 specification and XML schemas, BKA, oesterreich.gv.at and USP pages. 30 files, retrieved 2026-11-23. | German | Markdown, PDF, ZIP | Federated delivery with licensed Zustelldienste, Teilnehmerverzeichnis and Anzeigemodul, bPK addressing, delivery receipts, company roles, mandatory participation. Reference for the `Österreich` column. |
| `Referenzen-Mein-Postkorb-Oesterreich.md` | Source list for the Austrian E-Zustellung ("Mein Postkorb"): ZustG, E-GovG, BKA white paper and ZUSE specification, oesterreich.gv.at, USP, with evidence per dimension. Local copies in the folder of the same name, checked 2026-11-23. | German | Markdown | Reference for the `Österreich` column in `existing_solutions.csv` and the Österreich entries in `morph_box.csv`. |
| `GOV-UK-Notify/` | GOV.UK Notify (GDS): condensed analysis plus local copies of service pages, REST API docs, GitHub README, Service Manual, Electronic Communications Act 2000 and Interpretation Act 1978. 35 files, retrieved 2026-11-23. | English (summary and indexes German) | Markdown | A central notification service without mailbox: API and web UI, channels, security, retention, pricing, UK rules on written form and service by post. Reference for the `UK (kein zentrales Postfach)` column. |
| `Referenzen-GOV-UK-Notify.md` | Source list for GOV.UK Notify (UK, GDS): service pages on features, security, pricing, data retention, files, received text messages, plus REST API documentation, with evidence per dimension. Local copies in the folder of the same name, checked 2026-11-23. | German | Markdown | Reference for the `UK (kein zentrales Postfach)` column in `existing_solutions.csv` and the GOV.UK Notify entries in `morph_box.csv`. |
| `Digital-Post-Daenemark/` | Danish Digital Post: condensed analysis plus local copies of the Digital Post act and six executive orders (Lovtidende), Technical Integration v1.51, MeMo model, Tilslutningsaftale, Digitaliseringsstyrelsen, borger.dk and virk.dk pages. 43 files, retrieved 2026-11-23. | Danish, English (summary and indexes German) | Markdown, PDF | Mandatory use and exemption, sender and recipient systems, MeMo, receipts, malware and format checks, approved clients, open E2EE question. Reference for the `Dänemark` column. |
| `Referenzen-Digital-Post-Daenemark.md` | Source list for the Danish Digital Post: Lov om Digital Post fra offentlige afsendere (LBK nr. 686/2021), Digitaliseringsstyrelsen pages on platforms, sender and recipient systems, Kontaktstruktur, Rettighedsportal, visningsklienter, plus borger.dk help, with evidence per dimension. Local copies in the folder of the same name, checked 2026-11-23. | German | Markdown | Reference for the `Dänemark` column in `existing_solutions.csv` and the Dänemark entries in `morph_box.csv`. |
| `Digital-Postkasse-Norwegen/` | Norwegian Digital postkasse til innbyggere (DPI): condensed analysis plus local copies of Digdir docs, DPI JSON schema, Digipost/e-Boks/Altinn pages, Lovdata texts, Digitaliseringsrundskrivet 2019, Difi-Rapport 2019:6, Bruksvilkår 2025. 40 files, retrieved 2026-11-23. | Norwegian, English (summary and indexes German) | Markdown, PDF, JSON | KRR addressing, Peppol transport, encryption up to the provider, contract model Digdir, agencies and private providers, notification and opt-out. Reference for the `Norwegen` column. |
| `Referenzen-Digital-Postkasse-Norwegen.md` | Source list for the Norwegian Digital postkasse til innbyggere (DPI): eForvaltningsforskriften and forvaltningsloven (Lovdata), Digdir documentation on DPI, KRR, eFormidling and ID-porten, Altinn Melding and Autorisasjon, plus Digipost and e-Boks, with evidence per dimension. Local copies in the folder of the same name, checked 2026-11-23. | German | Markdown | Reference for the `Norwegen` column in `existing_solutions.csv` and the Norwegen entries in `morph_box.csv`. |
| `MijnOverheid-Niederlande/` | Dutch MijnOverheid Berichtenbox, Berichtenbox voor bedrijven and the Federatief Berichtenstelsel (FBS): condensed analysis plus local copies of Awb, Wmebv, Wet digitale overheid and Dienstenwet (BWB XML), Kamerstukken, Logius connection guide 1.6.4 and FBS pages, MOZa proof of concept, mijn.overheid.nl, DigiD, eHerkenning and ondernemersplein pages. 94 files, retrieved 2026-11-26. | German, Dutch | Markdown, PDF, YAML, ZIP | Central mailbox today (Logius, GLOBE) and its migration to a federation where messages stay in the sender's store; opt-in per authority, Bekanntgabe under art. 2:19 Awb, DigiD Machtigen, Digikoppeling. Reference for the `Niederlande` column. |
| `Referenzen-MijnOverheid-Niederlande.md` | Source list for the Dutch MijnOverheid Berichtenbox, Berichtenbox voor bedrijven and FBS: Awb, Wmebv, Wdo, Logius, MijnOverheid Zakelijk, mijn.overheid.nl, with evidence per dimension. Local copies in the folder of the same name, checked 2026-11-26. | German | Markdown | Reference for the `Niederlande` column in `existing_solutions.csv` and the Niederlande entries in `morph_box.csv`. |

## 1. ZaPuK/quellen/zapuk-v1.1.1/

Local copy of tag v1.1.1 of the openCode repo ZaPuK (<https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/tree/v1.1.1>). It replaces the older mirror `zapuk-v1.0.0-121-g7419d8b/`. Paths below are relative to `ZaPuK/quellen/zapuk-v1.1.1/`. Document repo, no code. Target architecture for the federal mailbox and communication infrastructure (G2C/G2B, Bescheide, Rückfragen). It is meant to consolidate BundID-Postfach, ZBP, MJP, OZG-Plus-Postfach, TI-Messenger, BundesMessenger, EGVP and others.

Core stance: hybrid, federated architecture. Matrix as Kommunikationsschicht (ADR-0011). MLS for E2EE (ADR-0012). One central Postfach-Backend and one central Zugang, plus openness for third parties. Self-Service-Portal and SDKs. No messaging bridges to systems outside the scope; for existing systems, v1.1 allows transitional bridges where messages are unencrypted anyway (Architekturkonzept v1.1, Transitionsstrategie); v1.0 had banned them (Architekturkonzept v1.0, 3.2). Interoperability with systems outside the scope, e.g. EU systems, only via Protocol Converters that keep E2EE (ADR-0010).

Status caveat: v1.1 (Architekturkonzept, Überblick, Transitionsstrategie, Rechtsgutachten) was submitted to IT-PLR B-2026/43-IT of 25.11.2026 as annexes but not adopted; the Beschluss only declares Phase II complete. ADR-1105, 1106, 1109 and 1147 are "Richtung entschieden", ADR-1121 and 1122 "In Arbeit", ADR-1149 unbearbeitet. v1.0 versions are under `Vorversionen/v1.0/`. Changes and contradictions: `ZaPuK/Aktualisierung-v1.1.1.md`; see section 7.

### Entry points (reading order)

1. `README.md`: programme overview, IT-PLR Beschlüsse, roadmap, Logbuch (changelog).
2. `Zielarchitektur/README.md`: document map.
3. `Zielarchitektur/Architekturentscheidungen/README.md`: ADR index with status. Best table for dimensions and options.
4. `Zielarchitektur/Architekturkonzept/README.md` (v1.1.1): chapter 3 Geschäftsarchitektur, chapter 4 development, operation and governance, section 5.4 ADR summaries by topic. v1.0 in `Vorversionen/v1.0/`.
5. `Zielarchitektur/Zusammenfassung/README.md`: Überblick und Handlungsempfehlungen v1.1; v1.0 with Kosten (3.3) in `Vorversionen/v1.0/`.
6. `Zielarchitektur/Transitionen/README.md`: Transitionsstrategie v1.1 for existing mailboxes.
7. `Zielarchitektur/Rechtsgutachten/README.md`: legal opinion, Stand 19.10.2026, with section E on the IT-Staatsvertrag.
8. `Zielarchitektur/Rahmenbedingungen/README.md`: laws and Beschlüsse.
9. `Glossar.md` and `Abkürzungen.md`: terminology.

### Other content

- `Konsolidierte-Anforderungen/README.md`: requirements v0.7.1 with IDs (ANF_FUN_*, ANF_ARC_*, RAH_*, DAT_*, KOM_*, BED_*, LEI_*, WAR_*). `Lesefassung/` holds a readable version. `Vorversionen/` holds history.
- `Konsultationen/`: consultation rounds (Phase 1 requirements 2025-03-19, Phase 2 architecture 2026-01-21, ongoing feedback).
- `Zielarchitektur/Architekturentscheidungen/ADR-*.md`: one file per ADR (Status, Kontext, Optionen, Entscheidung). Option diagrams are in `images/`.
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
- The parent ZaPuK is referenced only by link. It is the `ZaPuK/quellen/zapuk-v1.1.1/` source.

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

## 5. MUK/

Web research on Mein Unternehmenskonto (MUK), the einheitliches Organisationskonto for companies and other organisations (since 2021, Bausteine 1–4 operated by Bayern: StMD, BayLfSt; Bausteine 5–6 by Bremen: Governikus, Dataport). Two mailboxes: Postfach 2.0 (ELSTER infrastructure, central virus scan, no E2EE) and OZG-PLUS-Postfach (Funktionspostfächer, E2EE via proxy re-encryption). Authorities send via ELSTER-Transfer or ERiC; replies only when the authority enables them. ZaPuK ADR-1110 plans to remove the mailbox from MUK.

**Start with `MUK/MUK-Zusammenfassung.md`** (German): a condensed analysis of MUK along all `morph_box.csv` dimensions (section 5), with short codes for sources (FAQ, IL, BPZ, NEZO, VV …), evidence markers (**[Folgerung]**, **[unverifiziert]**, **[veraltet?]**), contradictions (section 6) and gaps (section 7).

**`Referenzen-MUK.md`** (German, next to the folder): source list in the style of `Referenzen-BundID-ZBP.md`, with short codes, local paths and URLs, evidence per dimension, checked on 2026-11-18, plus the deviations found during the check.

Each subfolder of `MUK/quellen/` has an `_index.md` with URL, publisher, date and a content note per file. Read the index before opening a file.

### Subfolders

- `MUK/quellen/muk-betreiber/`: operator and product view. info.mein-unternehmenskonto.de pages (FAQ, Postfachfunktionen, Berechtigungssteuerung, Zertifikate, rechtliche Hintergrundinformationen, Glossar, Impressum), MUK portal privacy notice and Impressum; registration and certificate guides, Berechtigungssteuerung quick start, Postfach 2.0 feature sheets (2023 and 2026); OZG-PLUS-Postfach UI texts, config, privacy notice and press releases; IT-PLR report with usage figures (April 2025); BA and IHK practice pages.
- `MUK/quellen/anbindung-technik/`: sender-side Anbindung. Integrationsleitfaden Postfach 2.0 (v9, 04/2026), Beipackzettel for joint delivery to both mailboxes, NEZO-Integrationsleitfaden, SSP guides, infosheets for authorities; ELSTER-Transfer pages; Governikus OZG-PLUS-Postfach Integrationsleitfaden, release notes and OpenAPI files; ITVSH registration guide; Sachsen-Anhalt OZG-Sprechstunde 2022.
- `MUK/quellen/recht-governance/`: law, governance, finance, history. Organisationskonto-Verordnung, § 87a and § 122a AO, § 3a VwVfG, § 3 UBRegG; BR-Drs. 396/21, BT-Drs. 19/21987, 19/23774, 20/6555, 20/9256, 21/5285, 21/8283 (JStG 2026); IT-PLR Beschlüsse 2019/02 to B-2026/08-IT with annexes (Basiskonzept 2020, Verwaltungsvereinbarung 2022/37, Postfachvereinheitlichung 2024); Bavarian Betriebsbedingungen (BayMBl. 2022 Nr. 237) and Bremen Senatsvorlage 2022. Legal sources already in `BundID-ZBP/quellen/recht-governance/` are only referenced.
- `MUK/quellen/standardisierung/`: standardization status of Proxy Re-Encryption (OZG-PLUS-Postfach). IEEE 1363.3-2013 status and Wikipedia excerpt, ETSI TR 103 719, IETF CFRG proposal 2016, BSI TR-02102-1 (2026-01).

### Caveats

- ELSTER-Transfer manuals and the original ETR OpenAPI are behind a licence click on elster.de and are not stored. Parts are in `ELSTER/` (ElsterBereitstellung-SSB).
- Sources from 2023 and 2026 differ on attachment limits, retention periods and the number of certificates per Steuernummer; the summary lists them in section 6.
- The Integrationsleitfaden still states the third day for the Bekanntgabefiktion; § 9 Abs. 1 OZG and § 122a AO use the fourth day.
- Some PDFs exist in two subfolders (Postfach 2.0 feature sheet and infosheet, Zertifikatsleitfaden, IT-PLR report 2025).

## 6. MJP/

Web research on Mein Justizpostfach (MJP), the free mailbox for natural persons to communicate with courts, Staatsanwaltschaften, Gerichtsvollzieher:innen and beBPo, beA, beN, beSt holders (pilot since October 2023). MJP is a participant in the EGVP infrastructure (OSCI 1.2, SAFE directory, XJustiz). Login only via BundID at Vertrauensniveau hoch; the key pair is created in the browser and kept as a local file, so messages are end-to-end encrypted. Provider: BLK-AG IT-Standards on behalf of Bund and Länder, represented by JuM BW; development by Governikus, operation by Westernacher since 30.07.2026.

**Start with `MJP/MJP-Zusammenfassung.md`** (German): a condensed analysis of MJP along all `morph_box.csv` dimensions (section 5), with short codes for sources (TB, WEB, FAQ, HR, ANF, LF …), evidence markers (**[Folgerung]**, **[unverifiziert]**, **[veraltet?]**), contradictions (section 6) and gaps (section 7).

**`Referenzen-MJP.md`** (German, next to the folder): source list in the style of `Referenzen-MUK.md`, with short codes, local paths and URLs, evidence per dimension, checked on 2026-11-18, plus the deviations found during the check, including corrections to the MJP entries in `morph_box.csv` (Ü2, P5, P7, P8).

Each subfolder of `MJP/quellen/` has an `_index.md` with URL, publisher, date and a content note per file, and a `_befunde.md` with findings per dimension. Read the index before opening a file.

### Subfolders

- `MJP/quellen/mjp-betreiber/`: operator and product view. Web client 2.7.5 UI texts, config and `app.js` excerpts; privacy notice and Impressum; mjpserviceportal.de FAQ and support guides (deletion after 90 days, key file renewal, eEB, saving messages); BMI two-pager 2023, BMJ press release, JM NRW Erlass; FragDenStaat requests; netzpolitik.org articles.
- `MJP/quellen/anbindung-technik/`: EGVP/OSCI infrastructure. BLK requirements for third-party applications (A1–A25), ERV-Leitfaden 2025, ERVB 2025, SAFE-ID and SAFE search standards, VHN, Drittprodukt registration of MJP, beBPo pages, EGVP notices, BLK Länderberichte; own analysis of the web client cryptography; Westernacher operation takeover.
- `MJP/quellen/recht-governance/`: law, governance, finance. ZPO and ERVV; BT-Drs. 17/12634, 19/28399, 20/10943, 21/1509; BGBl. 2021 I S. 4607, 2025 I Nr. 349, 2026 I Nr. 152; BLK and E-Justice-Rat pages, BLK Umlaufbeschluss on ERV cost sharing; justiz.de pages for citizens and authorities.

### Caveats

- `egvp.justiz.de` was not reachable; its pages come from Internet Archive snapshots.
- `app.js` is proprietary and not stored; the cryptography findings rest on quoted excerpts with byte offsets.
- No usage figures, data centre or Malware-Prüfung found; the handover of responsibility from BMI to the justice side is not documented.
- Some PDFs exist in two or three subfolders (BLK-AG Handreichung, JM NRW Erlass, BMI two-pager).

## 7. ZaPuK/

Research on ZaPuK as a column in `existing_solutions.csv`. ZaPuK is a target architecture, not a running solution; running software exists only from the Neo pilot (closed by FIT-AB 2026/11 on 23.06.2026, demo integrations only). IT-PLR Beschluss B-2026/43-IT (51st meeting, 25.11.2026) declared Phase II complete; the Bund confirmed its programme responsibility. Version 1.1 was submitted as annexes but not adopted; Phase III under BMDS lead, binding consolidation and financing were not decided. v1.1 content is marked "v1.1", content only in the draft Transitionspläne "Entwurf Transitionsplan".

**Start with `ZaPuK/ZaPuK-Zusammenfassung.md`** (German): a condensed analysis along all `morph_box.csv` dimensions (section 5), with short codes, evidence markers (**[v1.1]**, **[Entwurf]**, **[Folgerung]**, **[unverifiziert]**, **[veraltet?]**; in the CSVs these map to the levels in `Referenzen-ZaPuK.md`), contradictions (section 6) and gaps (section 7). Changes with v1.1.1: `ZaPuK/Aktualisierung-v1.1.1.md`.

**`Referenzen-ZaPuK.md`** (German, next to the folder): source list with short codes, local paths and URLs, evidence per dimension, checked on 2026-11-19 and against v1.1.1 on 2026-11-27, plus the corrections made to the ZaPuK entries in `morph_box.csv` and `existing_solutions.csv`.

Each subfolder of `ZaPuK/quellen/` has an `_index.md` with URL, publisher, date and a content note per file, and a `_befunde.md` with findings per dimension. Read the index before opening a file.

### Subfolders

- `ZaPuK/quellen/zapuk-v1.1.1/`: repo copy of tag v1.1.1, see section 1.
- `ZaPuK/quellen/architektur-technik/`: findings against v1.1.1 with a concordance of old to new line numbers. Older copies of ADRs changed on `main` (2026-11-12) and of the former draft branch `architecture-v1.1` (largely duplicated by v1.1.1); openCode issues from the consultation. Findings per dimension B1 to I5 and a check of all ZaPuK references in `morph_box.csv`.
- `ZaPuK/quellen/recht-governance/`: IT-PLR Beschluss B-2026/43-IT with presentation and FITKO-Einordnung of the Rechtsgutachten; older copies of the Rechtsgutachten (06.10.2026; current Stand 19.10.2026 in v1.1.1) and Überblick und Handlungsempfehlungen v1.1; FIT-AB Beschlüsse 2026/11 and 2026/12; IT-PLR B-2026/04-IT, 14, 17, 29, press releases of the 49th and 50th meetings, meeting dates; BT-Drs. 21/7319. Findings for I1, I5, R1 to R4. Beschlüsse already in `BundID-ZBP/quellen/recht-governance/` are only referenced.
- `ZaPuK/quellen/umsetzung-rezeption/`: Neo pilot (closing slides, Integrationsleitfaden, project page, status slides, planning issues), Transitionsstrategie and plans for ZBP, MJP, MUK and Mein ELSTER, consultation statements, press and provider statements (heise, c't, netzpolitik.org, eGovernment, D64, Element, SET, matrix.org).

### Caveats

- The draft Transitionspläne for ZBP, MJP and MUK and the ELSTER transition ADRs are not in v1.1.1; the branch `architecture-v1.1` no longer exists upstream, so the copies of 13.11.2026 in `ZaPuK/quellen/umsetzung-rezeption/` are the only source.
- Some v1.1 documents exist as older copies in two subfolders and in `zapuk-v1.1.1/`. Line numbers in `Referenzen-ZaPuK.md` refer to `zapuk-v1.1.1/`; each `_befunde.md` has a concordance.
- No financing concept (due for the 52nd meeting on 17.03.2027), no current cost figures, no operating site at federal level, no Neo production pilot.
- Responses of the ZaPuK team to consultation issues need an openCode login and are not stored. The Bundesrechnungshof report of 31.07.2026 is not for publication and is not stored.

## 8. Non-German solutions (Mein-Postkorb-Oesterreich/, GOV-UK-Notify/, Digital-Post-Daenemark/, Digital-Postkasse-Norwegen/, MijnOverheid-Niederlande/)

Local source collections for the five international columns in `existing_solutions.csv`, built like `MUK/` on 2026-11-23 (Netherlands on 2026-11-26). Each folder has a German `<Folder>-Zusammenfassung.md` along the `morph_box.csv` dimensions (evidence markers [Indiz], [Annahme]) and `quellen/` with the subfolders `betreiber/`, `anbindung-technik/` and `recht-governance/`, each with an `_index.md` (URL, publisher, date, content note, unreachable sources). The `Referenzen-<Folder>.md` next to each folder maps short codes to local paths. Read the index before opening a file.

Caveats: regjeringen.no returns 403 (current Digitaliseringsrundskrivet missing, 2019 version stored); Danish videos and some Tilslutningsaftale annexes not stored; HTML conversions of UK pages include site navigation; Austrian ZUSE XML schemas stored as ZIP, not analysed. wetten.overheid.nl, machtigen.digid.nl and NORA block automated access (Dutch statutes stored from the BWB XML repository instead); the Dutch FBS is a target picture with a proof of concept, not in operation.

## Which source for which morph_box topic

| Topic | Start with |
|---|---|
| Anbindung von Fachverfahren / Behörden | zapuk ADR-0008, 0009, 1145, 1148; neo `03_context_and_scope.md` |
| Topologie, Betrieb, Hosting | zapuk ADR-0002 to 0007, 1103, 1114; neo ADR-0001 to 0003, `structurizr/model/backend.dsl` |
| Bestandssysteme, Transition, Dekommissionierung | zapuk `Zielarchitektur/Transitionen/README.md`, ADR-0010; draft Transitionspläne in `ZaPuK/quellen/umsetzung-rezeption/` |
| Adressierung | zapuk ADR-1104, 1105, 1111 |
| Identität, Authentifizierung, EUDI-Wallet | zapuk ADR-0013 to 0016, 1106 to 1110, 1146; neo structurizr, glossary |
| Verschlüsselung, Sicherheit | zapuk ADR-0011, 0012, 1100, 1101, 1130, 1147; neo `02_architecture_constraints.md` |
| Zustellung, Bescheid, Fristen, Rechtssicherheit | zapuk ADR-0018, 1118 to 1121, Rechtsgutachten, Rahmenbedingungen; neo `concepts/303-*.md` |
| Benachrichtigung | zapuk ADR-1122 (In Arbeit), D-Stack Beschluss 2026/18-IT |
| Funktionspostfächer, Vertretung | zapuk ADR-1102, 1109, 1112, 1113 |
| Aufbewahrung, Veraktung | zapuk ADR-1135 |
| Clients / Frontends | neo `03_context_and_scope.md` |
| Governance, Länder, Finanzierung | zapuk README Beschlüsse, Überblick v1.1 (`Zielarchitektur/Zusammenfassung/README.md`) 3.2 and 3.5, Architekturkonzept v1.1.1 chapter 4, Rechtsgutachten section E; costs v1.0 in `Zielarchitektur/Zusammenfassung/Vorversionen/v1.0/` 3.3; B-2026/43-IT in `ZaPuK/quellen/recht-governance/` |
| Requirements as decision criteria | zapuk `Konsolidierte-Anforderungen/README.md` |
| ELSTER as existing solution (Postfach, Postfach 2.0, Bekanntgabe, Vollmacht, Anbindung) | `Referenzen-ELSTER.md` for evidence per dimension, `ELSTER/ELSTER-Zusammenfassung.md` first, then `ELSTER/`: Entwicklerhandbuch 5.2, 7.5, 9.2, 9.3, 9.15 to 9.17; `ElsterBereitstellung-SSB`; `ElsterDatenabholung-v32` |
| BundID / ZBP as existing solution (Anbindung, Postkorb-Handle, Bekanntgabe nach § 9 OZG, Betrieb) | `Referenzen-BundID-ZBP.md` for evidence per dimension, `BundID-ZBP/ZBP-Zusammenfassung.md` for context, then the `_index.md` of the relevant `BundID-ZBP/quellen/` subfolder |
| Rechtsrahmen Bürgerkonto und Postfach (OZG, DIdG, IT-PLR Beschlüsse) | `BundID-ZBP/quellen/recht-governance/_index.md`; zapuk Rahmenbedingungen |
| Mein Unternehmenskonto / Postfach 2.0 / OZG-PLUS-Postfach as existing solution (Anbindung über ELSTER-Transfer, Postfachhandle, Funktionspostfächer, Organisationszugang, Pflicht für Behörden) | `Referenzen-MUK.md` for evidence per dimension, `MUK/MUK-Zusammenfassung.md` for context, then the `_index.md` of the relevant `MUK/quellen/` subfolder |
| Mein Justizpostfach as existing solution (EGVP/OSCI, SAFE, E2EE with local key file, sicherer Übermittlungsweg, Zustellung nach § 173 ZPO, BLK governance) | `Referenzen-MJP.md` for evidence per dimension, `MJP/MJP-Zusammenfassung.md` for context, then the `_index.md` of the relevant `MJP/quellen/` subfolder |
| ZaPuK as a column (per-dimension status, v1.1, Neo pilot, transition of existing mailboxes, Rechtsgutachten, governance and financing) | `Referenzen-ZaPuK.md` for evidence per dimension, `ZaPuK/ZaPuK-Zusammenfassung.md` for context, then the `_index.md` of the relevant `ZaPuK/quellen/` subfolder; ADR texts in `ZaPuK/quellen/zapuk-v1.1.1/` |
| Österreich / Mein Postkorb as existing solution | `Referenzen-Mein-Postkorb-Oesterreich.md` for evidence per dimension, `Mein-Postkorb-Oesterreich/Mein-Postkorb-Oesterreich-Zusammenfassung.md` for context, then the `_index.md` of the relevant `Mein-Postkorb-Oesterreich/quellen/` subfolder |
| UK / GOV.UK Notify as existing solution | `Referenzen-GOV-UK-Notify.md` for evidence per dimension, `GOV-UK-Notify/GOV-UK-Notify-Zusammenfassung.md` for context, then the `_index.md` of the relevant `GOV-UK-Notify/quellen/` subfolder |
| Dänemark / Digital Post as existing solution | `Referenzen-Digital-Post-Daenemark.md` for evidence per dimension, `Digital-Post-Daenemark/Digital-Post-Daenemark-Zusammenfassung.md` for context, then the `_index.md` of the relevant `Digital-Post-Daenemark/quellen/` subfolder |
| Norwegen / Digital postkasse til innbyggere as existing solution | `Referenzen-Digital-Postkasse-Norwegen.md` for evidence per dimension, `Digital-Postkasse-Norwegen/Digital-Postkasse-Norwegen-Zusammenfassung.md` for context, then the `_index.md` of the relevant `Digital-Postkasse-Norwegen/quellen/` subfolder |
| Niederlande / MijnOverheid Berichtenbox and Federatief Berichtenstelsel as existing solution | `Referenzen-MijnOverheid-Niederlande.md` for evidence per dimension, `MijnOverheid-Niederlande/MijnOverheid-Niederlande-Zusammenfassung.md` for context, then the `_index.md` of the relevant `MijnOverheid-Niederlande/quellen/` subfolder |

## Gaps and caveats

- No concrete hosting, cost, sizing or SLA data exists in neo (empty chapters). For cost, see zapuk `Zielarchitektur/Zusammenfassung/Vorversionen/v1.0/` 3.3; no current cost figures, financing concept due for the 52nd IT-PLR meeting.
- The ZaPuK v1.0 documents (`Vorversionen/v1.0/`) are outdated relative to the ADRs and v1.1; v1.1 itself is not adopted by the IT-PLR.
