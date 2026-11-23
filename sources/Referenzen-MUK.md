# Mein Unternehmenskonto (Deutschland): Quellen

Nachweise für die MUK-Einträge in `morph_box.csv` und die Spalte `Unternehmenskonto heute (Deutschland)` in `existing_solutions.csv`. Statt `[unverifiziert]` stehen hier und in den MUK-Einträgen der CSV-Dateien `[Indiz]` (Stufe 2) und `[Annahme]` (Stufe 1); unmarkierte Aussagen sind Stufe 3. `Fehlanzeige` heißt: in den Quellen gesucht, nicht gefunden.

Evidenzstufen (Spalte `Stufe`):

- **3 (Belegt / Primärquelle):** Gesetzestext, Drucksache, Beschluss oder Betreiberdokument, in der lokalen Kopie im Volltext geprüft.
- **2 (Sekundär / Indiz):** Herstelleraussage, Länderpräsentation, Pressemitteilung, veraltete Angabe oder Rückschluss aus einer Primärquelle; `Fehlanzeige` bei gezielter Suche zählt als Indiz für eine negative Aussage.
- **1 (Annahme / Hypothese):** kein Beleg, nicht einmal ein Indiz; offen und zu prüfen.

Bei gemischten Zeilen steht zuerst die Stufe des belegten Teils, danach die Stufe des Teils, der in der Aussage mit `[Indiz]` oder `[Annahme]` markiert ist. Ausführliche Einordnung: `MUK/MUK-Zusammenfassung.md`.

Stand: 2026-11-18. Die Belege stammen aus den lokalen Kopien in `MUK/quellen/` (Webrecherche ab <https://info.mein-unternehmenskonto.de/>) sowie aus OZG, OZGÄndG- und DIdG-Drucksachen in `BundID-ZBP/quellen/recht-governance/`. Die Fundstellen geben Zeile (Z.) der Markdown-Kopie, gedruckte Seite (S.) oder PDF-Seite (PDF-S.) an. MUK hat zwei Postfächer: das Postfach 2.0 (ELSTER, Bayern) und das OZG-PLUS-Postfach (Governikus, Bremen). Aussagen ohne Zusatz betreffen das Postfach 2.0.

## Hauptquellen

Lokale Pfade relativ zu `sources/MUK/quellen/`, mit `BZ/` relativ zu `sources/BundID-ZBP/quellen/`.

| Kürzel | Quelle | Stufe |
|---|---|---|
| FAQ | StMD, Hilfe / FAQ, Stand je Frage 2023–2026: `muk-betreiber/stmd_muk-info_hilfe-faq.md`, <https://info.mein-unternehmenskonto.de/hilfe/> | 3 |
| RECHT | StMD, Technische und rechtliche Hintergrundinformationen, 28.04.2026: `muk-betreiber/stmd_muk-info_rechtliche-hintergrundinformationen.md`, <https://info.mein-unternehmenskonto.de/technische-und-rechtliche-hintergrundinformationen/> | 3 |
| PF | StMD, Postfachfunktionen: `muk-betreiber/stmd_muk-info_postfachfunktionen.md`, <https://info.mein-unternehmenskonto.de/ueberblick-funktionalitaeten/postfachfunktionen/> | 3 |
| BIDI | StMD, Bidirektionale Kommunikation, 28.07.2023: `muk-betreiber/stmd_muk-info_2023-07-28_bidirektionale-kommunikation.md`, <https://info.mein-unternehmenskonto.de/bidirektionalekommunikation/> | 3 |
| PORTAL | BayLfSt, Portal Mein Unternehmenskonto: `muk-betreiber/baylfst_muk-portal_startseite.md`, <https://mein-unternehmenskonto.de/mup/start> | 3 |
| DSE | BayLfSt, Datenschutz Mein Unternehmenskonto: `muk-betreiber/baylfst_muk-portal_datenschutz.md`, <https://mein-unternehmenskonto.de/mup/helpGlobal?themaGlobal=datenschutz_public> | 3 |
| IMP-WEB | StMD, Impressum der Infoseite: `muk-betreiber/stmd_muk-info_impressum.md`, <https://info.mein-unternehmenskonto.de/impressum/> | 3 |
| REG | StMD, Leitfaden Schritt-für-Schritt-Registrierung, 06/2025: `muk-betreiber/stmd_muk_leitfaden-registrierung-schritt-fuer-schritt_2025-06.pdf`, <https://info.mein-unternehmenskonto.de/wp-content/uploads/2026/06/Leitfaden_Schritt-fuer-Schritt-Registrierung-MUK-1.pdf> | 3 |
| ZLF | BayLfSt/StMD, Zertifikatsleitfaden v3.0, 11/2026: `muk-betreiber/baylfst-stmd_muk_zertifikatsleitfaden-v3.0_2026-11.pdf`, <https://service.mein-unternehmenskonto.de/api/downloads/zertifikatsleitfaden> | 3 |
| BSQS | Dataport, Quickstart Berechtigungssteuerung für Organisationen v1.4, 05/2025: `muk-betreiber/dataport_muk_quickstart-berechtigungssteuerung-organisationen-v1.4_2025-05.pdf`, <https://service.mein-unternehmenskonto.de/api/downloads/guide_organisationen> | 3 |
| HL | StMD, Postfach 2.0 Funktionale Highlights v4, 04/2026: `muk-betreiber/stmd_muk_postfach20-funktionale-highlights-v4_2026-04.pdf`, <https://service.mein-unternehmenskonto.de/api/downloads/bedienungsanleitung_postfach> | 3 |
| PF23 | StMD, Postfach 2.0 Bedienung und Featureübersicht v1, 11/2023: `muk-betreiber/stmd_muk_postfach20-bedienung-featureuebersicht-v1_2023-11.pdf`, <https://info.mein-unternehmenskonto.de/wp-content/uploads/2023/11/Postfach-2.0-Bedienung-und-Featureuebersicht-Postfach-2.0-1.0_komprimiert.pdf> | 3 |
| AL25 | IT-PLR, Information 2025/06-AL, Bericht Unternehmenskonto, 29.04.2025: `muk-betreiber/itplr-al-runde_unternehmenskonto-bericht-top06_2025-04-29.pdf`, <https://www.it-planungsrat.de/fileadmin/beschluesse/2025/Information2025_06_AL_Unternehmenskonto_Bericht.pdf> | 3 |
| OZGPP-UI | Governikus, UI-Textbundle des OZG-PLUS-Postfachs (Webclient 1.8.2): `muk-betreiber/governikus_ozgpp_ui-textbundle-de.md`, <https://ozgpp.de/i18n/messages_de.json> | 3 |
| OZGPP-CFG | Governikus, Webclient-Konfiguration des OZG-PLUS-Postfachs: `muk-betreiber/governikus_ozgpp_webclient-config.md`, <https://ozgpp.de/config/app/config.json> | 3 |
| OZGPP-DS | Governikus, Datenschutz OZG-PLUS-Postfach, 06/2023: `muk-betreiber/governikus_ozgpp_datenschutz_2023-06.md`, <https://ozgpp.de/#/datenschutz> | 3 |
| OZGPP-IMP | Senator für Finanzen Bremen, Impressum OZG-PLUS-Postfach: `muk-betreiber/senfin-bremen_ozgpp_impressum.md`, <https://ozgpp.de/> | 3 |
| IL | BayLfSt/StMD, Integrationsleitfaden Postfach 2.0 v9, 04/2026: `anbindung-technik/muk_postfach-2.0_integrationsleitfaden_v9_2026-04.pdf`, <https://service.mein-unternehmenskonto.de/api/downloads/integrationsleitfaden_postfach> | 3 |
| BPZ | Beipackzettel Integration OZG-PLUS-Postfach und Postfach 2.0 v1, 03/2026: `anbindung-technik/muk_beipackzettel_integration-ozgpp-und-postfach-2.0_v1_2026-03.pdf`, <https://service.mein-unternehmenskonto.de/api/downloads/beipackzettel_postfaecher> | 3 |
| NEZO | NEZO-Integrationsleitfaden v366, 09/2026: `anbindung-technik/muk_nezo-integrationsleitfaden_v366_2026-09.pdf`, <https://service.mein-unternehmenskonto.de/api/downloads/nezo_integrationsleitfaden> | 3 |
| SSP | SSP Quick Start Guide v1.3, 12/2024: `anbindung-technik/muk_ssp-quick-start-guide_v1.3_2024-12.pdf`, <https://service.mein-unternehmenskonto.de/api/downloads/ssp_quick_start_guide> | 3 |
| IS-BEH | Infosheet Einbindung durch Behörden, 09/2026: `anbindung-technik/muk_infosheet_einbindung-durch-behoerden_2026-09.pdf`, <https://service.mein-unternehmenskonto.de/api/downloads/muk_infosheet> | 3 |
| MUK-BEH | StMD, Behörden als Dienstanbieter: `anbindung-technik/muk-info_behoerden-als-dienstanbieter.md`, <https://info.mein-unternehmenskonto.de/behoerden-als-dienstanbieter/> | 3 |
| MUK-NUTZ | StMD, Behörden als Nutzer: `anbindung-technik/muk-info_behoerden-als-nutzer.md`, <https://info.mein-unternehmenskonto.de/behoerden-als-nutzer/> | 3 |
| ETR-V | ELSTER, ELSTER-Transfer für die Verwaltung: `anbindung-technik/elster_verwaltung_elster-transfer.md`, <https://www.elster.de/elsterweb/infoseite/verwaltung> | 3 |
| GOV-FPF | Governikus, Mein Unternehmenskonto / Funktionspostfächer (FAQ): `anbindung-technik/governikus_mein-unternehmenskonto_funktionspostfaecher.md`, <https://www.governikus.de/loesungen/kundenprojekte/mein-unternehmenskonto/> | 2 (Herstellerseite) |
| OZGPP-IL | Governikus, Integrationsleitfaden OZG-PLUS-Postfach v1.11, 04/2026: `anbindung-technik/governikus_ozgpp_integrationsleitfaden_v1.11_2026-04.pdf`, <https://www.governikus.de/wp-content/uploads/2026/04/Integrationsleitfaden_OZGPP_v1_11.pdf> | 3 |
| PM23 | Governikus, Pressemitteilung erstes Fachverfahren am OZG-PLUS-Postfach, 18.10.2023: `muk-betreiber/governikus_pressebox_ozgpp-erstes-fachverfahren_2023-10-18.md`, <https://www.pressebox.de/presscorner/firma/bremen-online-services-gmbh-co-kg-bos-kg/meldung/boxid/1176708/iframe/2040> | 2 (Pressemitteilung) |
| OZGPP-LU | Governikus, OpenAPI Lookup 1.0.0: `anbindung-technik/openapi_ozgpp-lookup-1.0.0.yaml`, <https://www.governikus.de/wp-content/uploads/2026/01/lookup-1.0.0-rest-api-docs.yaml> | 3 |
| STD | Standardisierung von Proxy Re-Encryption, Übersicht: `standardisierung/_index.md` | 3 |
| IEEE | IEEE SA, IEEE 1363.3-2013, Status „Inactive-Reserved“ seit 2024-03-21: `standardisierung/ieee_1363.3-2013_status.md`, <https://standards.ieee.org/ieee/1363.3/3822/>; Inhalt laut Wikipedia: `standardisierung/wikipedia_ieee-p1363.md`, <https://en.wikipedia.org/wiki/IEEE_P1363> | 3 (Status), 2 (Inhalt laut Wikipedia) |
| ETSI | ETSI TR 103 719 V1.1.1, Guide to Identity-Based Cryptography, 03/2022: `standardisierung/etsi_tr-103-719_v1.1.1_2022-03.pdf`, <https://www.etsi.org/deliver/etsi_TR/103700_103799/103719/01.01.01_60/tr_103719v010101p.pdf> | 3 |
| CFRG | IETF/IRTF CFRG, Proposal to use Proxy Re-Encryption in a messaging protocol, 22.08.2016: `standardisierung/ietf-cfrg_proxy-re-encryption-proposal_2016-08.md`, <https://mailarchive.ietf.org/arch/msg/cfrg/xSuAKoMXygK9JYbQnHZ0HtYSe-k/> | 3 |
| TR02102 | BSI TR-02102-1, Version 2026-01: `standardisierung/bsi_tr-02102-1_2026-01.pdf`, <https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Publikationen/TechnischeRichtlinien/TR02102/BSI-TR-02102.pdf> | 3 |
| ST22 | Sachsen-Anhalt, OZG-Sprechstunde Unternehmenskonto für Behörden, 22.09.2022: `anbindung-technik/st_ozg-sprechstunde_infoveranstaltung-uk-behoerden_2022-09.pdf`, <https://ozg.sachsen-anhalt.de/fileadmin/Bibliothek/Politik_und_Verwaltung/MF/OZG/Bilder/Veranstaltungen/OZG-Sprechstunden/25._InfoveranstaltungUK_Behoerden_aktuell.pdf> | 2 (Länderpräsentation, 2022) |
| ORGV | Verordnung nach § 3 Abs. 2 S. 2 OZG (Organisationskonto), 22.09.2021: `recht-governance/OZG3Abs2V_Organisationskonto-Verordnung_gesetze-im-internet_2026-11-18.md`, <https://www.gesetze-im-internet.de/ozg_3abs2s2v/BJNR437000021.html> | 3 |
| D23774 | BT-Drs. 19/23774, Beschlussempfehlung zur Einführung des Organisationskontos, 04.11.2020: `recht-governance/BT-Drs_19-23774_Beschlussempfehlung_Einfuehrung_Organisationskonto.pdf`, <https://dserver.bundestag.de/btd/19/237/1923774.pdf> | 3 |
| VV | IT-PLR, Beschluss 2022/37, Verwaltungsvereinbarung Organisationskonto, ab 01.01.2023: `recht-governance/IT-PLR_Beschluss2022-37_Verwaltungsvereinbarung_Organisationskonto.pdf`, <https://www.it-planungsrat.de/fileadmin/beschluesse/2022/Beschluss2022-37_Verwaltungsvereinbarung.pdf> | 3 |
| BAYVV | Bayern, Betriebsbedingungen Organisationskonto, BayMBl. 2022 Nr. 237: `recht-governance/BayMBl_2022-237_VVBeBOrgBek_Betriebsbedingungen_Organisationskonto.pdf`, <https://www.gesetze-bayern.de/Content/Document/BayVV_206_D_12940/true> | 3 |
| PLR20 | IT-PLR, Beschluss 2020/01 mit Basiskonzept, 14.02.2020: `recht-governance/IT-PLR_Beschluss_2020-01_Einheitliches_Unternehmenskonto_ELSTER.md`, `recht-governance/IT-PLR_Beschluss2020-01_Basiskonzept_Unternehmenskonto.pdf`, <https://www.it-planungsrat.de/beschluss/beschluss-2020-01> | 3 |
| PLR25 | IT-PLR, Beschluss 2025/29, Projektsteuerung Unternehmenskonto, 26.06.2025: `recht-governance/IT-PLR_Beschluss_2025-29_Unternehmenskonto_Projektsteuerung.md`, <https://www.it-planungsrat.de/beschluss/beschluss-2025-29> | 3 |
| AO122a | § 122a AO: `recht-governance/AO_122a_gesetze-im-internet_2026-11-18.md`, <https://www.gesetze-im-internet.de/ao_1977/__122a.html> | 3 |
| JSTG26 | BT-Drs. 21/8283, Gesetzentwurf JStG 2026, 30.09.2026: `recht-governance/BT-Drs_21-8283_Gesetzentwurf_JStG2026.pdf`, <https://dserver.bundestag.de/btd/21/082/2108283.pdf> | 3 |
| OZG | Onlinezugangsgesetz: `BZ/recht-governance/OZG_gesetze-im-internet_2026-11-17.md`, <https://www.gesetze-im-internet.de/ozg/> | 3 |
| D8093 | BT-Drs. 20/8093, Gesetzentwurf OZG-Änderungsgesetz: `BZ/recht-governance/BT-Drs_20-8093_Gesetzentwurf_OZG-Aenderungsgesetz.pdf`, <https://dserver.bundestag.de/btd/20/080/2008093.pdf> | 3 |
| MPK | BK/MPK, Föderale Modernisierungsagenda, 04.12.2025: `BZ/recht-governance/Foederale_Modernisierungsagenda_MPK_2025-12-04.pdf`, <https://www.rlp.de/fileadmin/02/MPK_Vorsitz/Beschluesse_MPK_4._Dezember_2025_Berlin/Foederale_Modernisierungsagenda.pdf> | 3 |

## Nachweise je Dimension

Belege für die Spalte `Unternehmenskonto heute (Deutschland)` in `existing_solutions.csv`, Stand 2026-11-18.

| Dimension | Aussage | Quellen | Stufe |
|---|---|---|---|
| B1 | ELSTER-Transfer (ETR), lokal bei der Behörde oder im IT-DLZ betrieben, per REST-API oder Weboberfläche; alternativ ERiC; Voraussetzungen SSP-Vorhaben, ELSTER-Organisationszertifikat mit Postfachrechten; Internet oder NdB-VN; ein Dienstleister-Zertifikat für mehrere Behörden | IL S. 9–17, 30; HL PDF-S. 3 | 3 |
| B1 | OZG-PLUS-Postfach: Datenart `OZGPPNachricht` über ETR seit 03/2026 oder direkte REST-API von `ozgpp.de` | BPZ S. 7–9; OZGPP-IL PDF-S. 16 | 3 |
| B1 | Zustellung über FIT-Connect ins MUK-Postfach [Annahme]; FIT-Connect transportiert nur die Rückkanal-Adresse `elster.accountId` | Fehlanzeige; `BZ/fitko-fit-connect/schema_fit-connect_dataset_reply-channel-1.0.0.schema.json` | 1 |
| B2 | Weboberfläche der lokal installierten ETR-Anwendung für Behörden mit wenigen Bescheiden; Mein ELSTER allein ermöglicht keinen Postfachversand | IL S. 10–11; ETR-V | 3 |
| B3 | `accountId`: pseudonyme SAML-NameID `ek-` + 40 Hex-Zeichen aus dem NEZO-Login (je Benutzer und Service Provider, zugleich Postfachhandle) oder 10-stellige ELSTER-AccountID; gilt je Zertifikat; ohne Login nur mit anderweitig bekanntem Handle (z. B. Grundlagenbescheid); `empfaengerreferenz` bis 255 Zeichen | IL S. 9, 19; NEZO PDF-S. 23, 52 | 3 |
| B3 | OZG-PLUS: Funktionspostfach-ID aus SAML-Attribut `Funktionspostfaecher` oder per Lookup-API; kein öffentliches Verzeichnis [Indiz] | NEZO PDF-S. 31, 66; OZGPP-LU; Verzeichnis: Fehlanzeige | 3, Verzeichnis: 2 |
| B4 | `EPBescheid`, `EPMitteilung`, `EPKurzmitteilung` (nur HTML), `Gewerbesteuerbescheid`; `htmlAnschreiben` bis 5000 Zeichen mit p, br, ul/ol, strong, em; Anhänge PDF, JPG, PNG, CSV, XML, bis 100 je Nachricht, 2 GiB je Anhang, 5 GiB je Nachricht | IL S. 19–24 | 3 |
| P1 | E-Mail an die bei ELSTER hinterlegte oder im Antrag angegebene Adresse, Abruf im Portal; fester Text, optionaler Zusatztext; Inhalt der Nachricht in der E-Mail [Indiz] | FAQ Z. 298, 743; PF23 S. 12–13; HL S. 3, 6 | 3, Inhalt in der E-Mail: 2 |
| P2 | ELSTER-Organisationszertifikat (Softwaredatei + Passwort) auf Steuernummer; persönliches ELSTER-Zertifikat nur, wenn der Onlinedienst es zulässt; kein Online-Ausweis; Vertrauensniveau substanziell, keine eIDAS-Notifizierung geplant | FAQ Z. 135, 201, 209; REG S. 20–24; RECHT Z. 24; OZG § 3 Abs. 4 Nr. 2 | 3 |
| P3 | viele gleichrangige Benutzerkonten je Steuernummer, je Konto ein Zertifikat und ein Postfach 2.0, nicht für andere Konten sichtbar; Berechtigungssteuerung mit Admins, Gruppen, Funktionsrollen; Funktionspostfächer nur im OZG-PLUS-Postfach (bis 500) | PF Z. 14; FAQ Z. 78, 292; BSQS S. 8–15; OZGPP-CFG Z. 52 | 3 |
| P4 | Zertifikatsweitergabe als konkludente Innenvollmacht; Vertretung im Postfach 2.0 „voraussichtlich ab Sommer 2026“, Umsetzung [Indiz]; gesetzliche Vertreter und Prokura nicht abgebildet; externe Vertretung [Annahme] | ZLF S. 6; FAQ Z. 145, 329, 623; extern: Fehlanzeige | 3, Umsetzung: 2, extern: 1 |
| P5 | Web: Portal ohne Installation, Postfach 2.0 auch in Mein ELSTER, OZG-PLUS-Postfach als Web-App; keine App [Indiz] | PORTAL Z. 35; FAQ Z. 317, 374; App: Fehlanzeige | 3, keine App: 2 |
| P6 | M2M-Abholung in Unternehmensinfrastrukturen bzw. ERP erwähnt, Abholung per ERiC; Dokumentation für Unternehmen [Indiz] | FAQ Z. 776; AL25 S. 4; IL S. 6, 9 | 3, Dokumentation: 2 |
| P7 | Antwort nur bei Freischaltung durch die Behörde (`antwort`, `antwortFrist`, `antwortBetreffe`), seit 07/2023; kein Erstkontakt [Indiz]; OZG-PLUS: Rückantwort über vom Fachverfahren vorgegebenen Kanal (OZGPP, E-Mail, URL) | IL S. 23–24; BIDI; FAQ Z. 304, 358; OZGPP-IL PDF-S. 44 | 3, kein Erstkontakt: 2 |
| P8 | Freitext, Antwort wird als PDF erzeugt, Anhänge PDF, CSV, TXT, PNG, JPG, XML; OZG-PLUS Upload bis 200 MB (FAQ: 600 MB je Anhang) | HL S. 5, 7; OZGPP-CFG; FAQ Z. 796 | 3 |
| P9 | Postfach Bestandteil des Organisationskontos, Postfach 2.0 je Benutzerkonto; Löschung des Zertifikats löscht Postfach-Inhalte; Bescheide 4 Jahre, Mitteilungen 180 Tage; keine Inaktivitätslöschung [Indiz] | OZG § 2 Abs. 7; FAQ Z. 237; HL S. 10; Inaktivität: Fehlanzeige | 3, Inaktivität: 2 |
| Ü1 | asynchron, Behörde liefert über ETR ein, Unternehmen holt ab; Statusabfrage bis 24 h, Durchlauf meist 5–10 min | IL S. 6, 9, 25, 40 | 3 |
| Ü2 | Postfach 2.0: keine Ende-zu-Ende-Verschlüsselung, Inhalte beim Betreiber lesbar [Indiz]; ETR übernimmt die Transportverschlüsselung; Indiz: zentrale Virenprüfung aller Anhänge bei Annahme setzt Klartext beim Betreiber voraus | BPZ S. 4; HL S. 2; IL S. 9; E2EE: Fehlanzeige | 2 |
| Ü2 | OZG-PLUS: Ende-zu-Ende bis zum Mitglied des Funktionspostfachs, Proxy Re-Encryption, Schlüssel im Browser; Metadaten unverschlüsselt | OZGPP-IL PDF-S. 15, 39, 45; GOV-FPF; OZGPP-DS Z. 79 | 3 |
| Ü2, I4 | Proxy Re-Encryption: kein etablierter Standard; IEEE 1363.3-2013 enthielt identitätsbasierte Verfahren mit Pairings, laut Wikipedia auch Proxy Re-Encryption, ist seit 2024-03-21 inaktiv; ETSI beschreibt das Verfahren nur in einem Technical Report; bei der IETF nur ein Individual Draft von 2016; BSI TR-02102-1 erwähnt es nicht; kein ISO/IEC- oder NIST-Standard gefunden. Governikus nennt es eine „Innovation“; Nutzdaten nach Standard (XML-Verschlüsselung mit ECIES), Umverschlüsselung in eigener Bibliothek `proxycrypt`, Verfahren nicht offengelegt | IEEE; ETSI Abschn. 5.2.7; CFRG; TR02102 Abschn. 2.3.4; PM23 Z. 31; OZGPP-IL PDF-S. 20, 39; ISO/IEC, NIST, Spezifikation von Governikus: Fehlanzeige | 3 |
| Ü3 | Absender authentisiert sich mit Organisationszertifikat mit Postfachrechten; Absendername Freitext; Siegel oder Signatur [Indiz]; Indiz: Schriftformersatz durch die Behörde verlangt ein qualifiziertes Siegel, ob MUK es prüft oder anzeigt, ist offen | IL S. 10, 12, 19; OZG § 9a Abs. 5; Siegel im MUK: Fehlanzeige | 3, Siegel: 2 |
| Ü4 | Status EINGESTELLT, DATEN_FEHLERHAFT, BEREIT_ZUR_ABHOLUNG, VOM_EMPFAENGER_ABGEHOLT (Abrufzeitpunkt), EINGANG_BESTAETIGT, GELOESCHT für die Behörde; beworbener Lesenachweis ohne eigenen Status | IL S. 6, 26–28; HL S. 9; OZGPP-IL PDF-S. 19 | 3 |
| Ü5 | keine Interoperabilität mit Systemen außerhalb; Zusammenführung der beiden MUK-Postfächer; keine eIDAS-Notifizierung | BPZ; RECHT Z. 24; EU-Systeme, BundID: Fehlanzeige | 3 |
| Ü6 | Postfach 2.0: zentrale Virenprüfung aller Anhänge bei Annahme; OZG-PLUS: Empfänger selbst verantwortlich | IL S. 9; HL S. 3; FAQ Z. 802; GOV-FPF | 3 |
| I1 | Aufgabe an Bayern und Bremen; Bayern (StMD, BayLfSt) Bausteine 1–4, Bremen (SF, Governikus, Dataport) Bausteine 5–6; Projektsteuerung mit Lenkungskreis | ORGV § 1; VV S. 6; DSE Z. 54; OZGPP-IMP; PLR25 | 3 |
| I2 | Postfach 2.0 auf Servern des BayLfSt in Deutschland; OZG-PLUS im deutschen Rechenzentrum von Governikus; Versand aus dem ETR der Behörde oder des IT-DLZ | DSE Z. 63; OZGPP-DS Z. 81; IL S. 14 | 3 |
| I3 | Behörden als Absender ohne eigenes Postfach-Backend über eigene oder IT-DLZ-ETR-Instanz; Behörden können MUK auch als Nutzer verwenden | IL S. 10, 14; MUK-NUTZ | 3 |
| I4 | Funktionspostfächer nur im OZG-PLUS-Postfach der Unternehmen, E2EE endet beim Mitglied, je Mitglied eigener privater Schlüssel nur im Browser, Mitglieder und Vertretungen lesen auch bereits empfangene Nachrichten, Verwaltung durch Admins; Behörde: interne Verteilung [Annahme] | GOV-FPF; OZGPP-UI Z. 230, 258; Behörde: Fehlanzeige | 3, Behörde: 1 |
| I5 | nutzungsberechtigt Behörden und Gerichte; Antrag im SSP mit Organisationszertifikat, Nutzungsarten, Prüfung durch das MUK-Team, Postfachrechte durch Vorhabensverantwortliche; Ausschluss bei wesentlichen Verstößen nach Hinweis | VV § 2, § 24; IL S. 12; SSP S. 6–7; IS-BEH PDF-S. 2 | 3 |
| R1 | Behörde an Organisation: Schriftformersatz mit qualifiziertem Siegel; Organisation an Behörde: Online-Formular nach Identifizierung über das Organisationskonto, ELSTER substanziell genügt | OZG § 9a Abs. 5, 6, § 3 Abs. 4 Nr. 2 | 3 |
| R2 | keine Kontopflicht für Unternehmen, Fachrecht kann sie vorsehen; Bundesleistungen für Unternehmen sollen ab Ende 2029 nur elektronisch angeboten werden; Einwilligung in die Bekanntgabe gilt als erteilt, Opt-out | D23774 S. 17; OZG § 1a Abs. 1, § 9 Abs. 1 S. 2 | 3 |
| R3 | Bekanntgabe durch Bereitstellung zum Abruf, Fiktion am vierten Tag; Steuern nach § 122a AO; keine förmliche Zustellung; Zustellung von Pfändungen an Kreditinstitute über ein Organisationskonto-Postfach geplant (JStG 2026, Bundesrat offen) | OZG § 9 Abs. 1; AO122a; JSTG26 S. 31, 118 | 3 |
| R4 | Pflicht: Verwendung des Organisationskontos für öffentliche Stellen (§ 3 Abs. 3 OZG), alle Bausteine nach Verfügbarkeit; unentgeltlich; Länder: Förderverfahren mit MUK-Anbindung bis 31.12.2027 | OZG § 3 Abs. 3, § 12 Abs. 3, 4; D8093 S. 39; VV § 19; MPK S. 31 | 3 |

## Abweichungen bei der Prüfung

Abgleich der Quellen untereinander und mit vorhandenen Dateien, 2026-11-18.

- **Ü4, R3:** IL S. 6 und 37 nennen für die Bekanntgabefiktion noch den dritten Tag; § 9 Abs. 1 OZG und § 122a AO sehen den vierten Tag vor.
- **R2:** VV S. 7–8 und BAYVV verlangen eine Einwilligung in die Bekanntgabe; § 9 Abs. 1 S. 2 OZG fingiert sie seit 2024. FAQ Z. 497 (Stand 2023) beschreibt die Regel als nicht in Kraft.
- **B4:** `Referenzen-ELSTER.md` (Kürzel P20) und PF23 nennen 50 MiB je Anhang und 250 MiB je Nachricht (2023); IL und HL (2026) 2 GiB und 5 GiB.
- **P8:** HL nennt TXT als Anhangformat, IL nicht.
- **P3:** Die Höchstzahl der Zertifikate je Steuernummer schwankt zwischen 200 (2020) und 17.000 (FAQ Z. 78, ZLF).
- **P9:** FAQ Z. 310 f. (2023) koppelt Aufbewahrungsfristen an den Abruf, HL S. 10 (2026) nicht.
- **R4:** „Behörden müssen beide Postfächer anbinden“ steht nur bei Governikus (GOV-FPF), nicht im OZG oder in der VV.
- **Kosten:** VV § 19 und MUK-BEH „unentgeltlich“, ST22 PDF-S. 6 (2022) „Anbindung eigenständig zu bezahlen“; gemeint sind wohl eigene Integrationskosten.
- **Ü3 in `morph_box.csv`:** Ausprägung 5 führt das Unternehmenskonto unter „Vertrauensniveau hoch“; nach § 9a Abs. 6 und § 3 Abs. 4 Nr. 2 OZG genügt das ELSTER-Organisationszertifikat mit Vertrauensniveau substanziell (vgl. P2, R1).

## Evidenzstufen der markierten Stellen

Einordnung der früheren `[unverifiziert]`-Stellen, Stand 2026-11-23. Stufe 1 und 2 werden über den genannten Prüfweg geprüft.

| Datei, Dimension | Aussage | Stufe | Begründung | Prüfweg |
|---|---|---|---|---|
| `existing_solutions.csv`, B1 | Zustellung über FIT-Connect | 1 | Fehlanzeige; FIT-Connect transportiert nur die Rückkanal-Adresse | Anfrage FITKO / MUK-Team |
| `existing_solutions.csv`, B3 | kein öffentliches Verzeichnis im OZG-PLUS-Postfach | 2 | Fehlanzeige; nur Lookup-API für angebundene Fachverfahren dokumentiert | Anfrage Governikus |
| `existing_solutions.csv`, P1 | genauer Inhalt der Benachrichtigung | 2 | fester Text mit optionalem Zusatztext dokumentiert, Wortlaut nicht | Testkonto oder Screenshot |
| `existing_solutions.csv`, P4 | Vertretung ab Sommer 2026, Umsetzung | 2 | Ankündigung des Betreibers, keine Bestätigung der Umsetzung | Anfrage MUK-Team; Interview Unternehmen |
| `existing_solutions.csv`, P4 | externe Vertretung, z. B. Steuerberatung | 1 | Fehlanzeige; Richtung der Aussage offen | Interview Unternehmen / Steuerberatung |
| `existing_solutions.csv`, P5 | keine App | 2 | Fehlanzeige in Betreiberdokumenten | App-Stores prüfen |
| `existing_solutions.csv`, P6 | Dokumentation für Unternehmen | 2 | Abholung über ERiC erwähnt (FAQ, AL25), Anleitung nicht gefunden | Anfrage BayLfSt; Interview Unternehmen |
| `existing_solutions.csv`, P7 | kein Erstkontakt | 2 | Antwort nur nach Freischaltung dokumentiert, Erstkontakt nicht erwähnt | Testkonto |
| `existing_solutions.csv`, P9 | keine Löschfrist bei Inaktivität | 2 | Fehlanzeige | Anfrage MUK-Team |
| `existing_solutions.csv`, Ü2 | Inhalte im Postfach 2.0 beim Betreiber lesbar | 2 | keine E2EE dokumentiert; zentrale Virenprüfung setzt Klartext voraus | Anfrage BayLfSt |
| `existing_solutions.csv`, Ü3 | Siegel oder Signatur der Behörde | 2 | Schriftformersatz verlangt qualifiziertes Siegel; Prüfung oder Anzeige im MUK offen | Anfrage MUK-Team; Interview Behörden |
| `existing_solutions.csv`, I4 | interne Verteilung bei Behörden | 1 | Fehlanzeige; liegt außerhalb des MUK | Interview Behörden |
| `existing_solutions.csv`, R4 | beide Postfächer sind anzubinden | 2 | nur Herstelleraussage (GOV-FPF), nicht im OZG oder in der VV | Anfrage StMD / Bremen |
| `morph_box.csv`, P6 Ausprägung 2 | Schnittstelle für Fachsoftware, auch Unternehmenskonto | 2 | wie P6 oben | wie P6 oben |
| `morph_box.csv`, Ü4 Ausprägung 3 | Lesenachweis beworben, kein eigener Lesestatus | 3 / 2 | Werbung und Statusliste belegt (HL, IL); ob Lesen erfasst wird, offen | Anfrage MUK-Team |
| `morph_box.csv`, Ü4 Ausprägung 5 | Eingangsbestätigung an das Unternehmen | 1 | Status EINGANG_BESTAETIGT nur für die Behörde dokumentiert | Testkonto |
