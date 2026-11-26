# MijnOverheid Berichtenbox (Niederlande): Quellen

Nachweise für die Spalte `Niederlande` in `existing_solutions.csv` und die Niederlande-Einträge in `morph_box.csv`. Erfasst sind drei Systeme: die MijnOverheid Berichtenbox für Personen (Logius, Nachrichtenspeicher GLOBE), die Berichtenbox voor bedrijven für Unternehmen (RVO mit DICTU) und das Federatief Berichtenstelsel (FBS) als Zielbild mit dem gemeinsamen Magazin BBO. Unmarkierte Aussagen sind Stufe 3, `[Indiz]` ist Stufe 2, `[Annahme]` ist Stufe 1. `Fehlanzeige` heißt: in den Quellen gesucht, nicht gefunden.

Evidenzstufen (Spalte `Stufe`):

- **3 (Belegt / Primärquelle):** Gesetzestext, Spezifikation oder Betreiberdokument, im Volltext geprüft.
- **2 (Sekundär / Indiz):** Herstelleraussage, Pressemitteilung, veraltete Angabe, Proof of Concept oder Rückschluss aus einer Primärquelle; `Fehlanzeige` bei gezielter Suche zählt als Indiz für eine negative Aussage.
- **1 (Annahme / Hypothese):** kein Beleg, nicht einmal ein Indiz; offen und zu prüfen.

Bei gemischten Zeilen steht zuerst die Stufe des belegten Teils, danach die Stufe des Teils, der in der Aussage mit `[Indiz]` oder `[Annahme]` markiert ist. Aussagen zum FBS sind als Plan belegt (Stufe 3 für „ist geplant“), nicht als umgesetzt.

Stand: 2026-11-26. Alle Hauptquellen wurden am 2026-11-26 online im Volltext abgerufen. Lokale Kopien liegen in `MijnOverheid-Niederlande/quellen/` (Spalte `Lokal`, Pfade relativ dazu); Auswertung in `MijnOverheid-Niederlande/MijnOverheid-Niederlande-Zusammenfassung.md`. Gesetzestexte stammen als BWB-XML aus `repository.officiele-overheidspublicaties.nl`, weil wetten.overheid.nl automatische Abrufe blockiert; die URL in der Tabelle ist die von wetten.overheid.nl. Einzelne Seiten liegen in mehreren Unterordnern; maßgeblich ist der Pfad in der Tabelle.

## Hauptquellen

| Kürzel | Quelle | Stufe | Lokal |
|---|---|---|---|
| AWB | Algemene wet bestuursrecht, Afdeling 2.3 (art. 2:7 bis 2:27), Fassung 2026-11-10: <https://wetten.overheid.nl/BWBR0005537/2026-11-10> | 3 | `recht-governance/wetten_awb_2026-11-10.md` |
| WMEBV | Wet modernisering elektronisch bestuurlijk verkeer, konsolidiert: <https://wetten.overheid.nl/BWBR0048252/2026-01-01> | 3 | `recht-governance/wetten_wmebv_2026-01-01.md` |
| STB-2023-183 | Wmebv, Staatsblad 2023, 183: <https://zoek.officielebekendmakingen.nl/stb-2023-183.html> | 3 | `recht-governance/stb-2023-183.md` |
| STB-2024-321 | Inwerkingtredingsbesluit Wmebv (1.1.2026, Teile 1.1.2027): <https://zoek.officielebekendmakingen.nl/stb-2024-321.html> | 3 | `recht-governance/stb-2024-321.md` |
| STB-2025-445 | Overige fiscale maatregelen 2026, Art. XV: <https://zoek.officielebekendmakingen.nl/stb-2025-445.html> | 3 | `recht-governance/stb-2025-445.md` |
| WDO | Wet digitale overheid (art. 5): <https://wetten.overheid.nl/BWBR0048156/2025-11-11> | 3 | `recht-governance/wetten_wdo_2025-11-11.md` |
| BDO | Besluit digitale overheid (MijnOverheid, art. 8, 13): <https://wetten.overheid.nl/BWBR0037987/2024-04-01> | 3 | `recht-governance/wetten_besluit-digitale-overheid_2024-04-01.md` |
| RVWDO | Regeling voorzieningen Wdo (art. 5, Konto und Opt-in): <https://wetten.overheid.nl/BWBR0048167/2025-12-06> | 3 | `recht-governance/wetten_regeling-voorzieningen-wdo_2025-12-06.md` |
| AWR | Algemene wet inzake rijksbelastingen (art. 3a, 3b): <https://wetten.overheid.nl/BWBR0002320/2026-10-01> | 3 | `recht-governance/wetten_awr_2026-10-01.md` |
| RBB | Regeling berichtenverkeer Belastingdienst: <https://wetten.overheid.nl/BWBR0053144/2026-10-01> | 3 | `recht-governance/wetten_regeling-berichtenverkeer-belastingdienst_2026-10-01.md` |
| DW | Dienstenwet (art. 13, 14, Berichtenbox voor bedrijven): <https://wetten.overheid.nl/BWBR0026759/2026-01-01> | 3 | `recht-governance/wetten_dienstenwet_2026-01-01.md` |
| UWV-AANW | Besluit aanwijzen kanalen elektronische berichten UWV 2026 (Beispiel für art. 2:13 Awb): <https://wetten.overheid.nl/BWBR0052960/2026-08-07> | 3 | `recht-governance/wetten_aanwijzing-kanalen-uwv-2026.md` |
| LOGIUS-IB | Instellingsbesluit baten-lastendienst Logius: <https://wetten.overheid.nl/BWBR0032656/2013-01-01> | 3 | `recht-governance/wetten_instellingsbesluit-logius_2013-01-01.md` |
| KST-MVT | Memorie van toelichting Wmebv, Kamerstukken II 35 261, nr. 3 (2019): <https://zoek.officielebekendmakingen.nl/kst-35261-3.pdf> | 3 (Entwurfsstand, Artikelzählung teils abweichend) | `recht-governance/kst-35261-3.txt` |
| KST-816 | Kamerbrief BZK zum AcICT-Advies FBS, 26 643, nr. 816 (2022-02-03): <https://zoek.officielebekendmakingen.nl/kst-26643-816.pdf> | 3 für 2022, als heutiger Stand 2 | `recht-governance/kst-26643-816.txt` |
| KST-928 | Kamerbrief BZK zu Logius, Digipoort und FBS, 26 643, nr. 928 (2022-10-18): <https://zoek.officielebekendmakingen.nl/kst-26643-928.pdf> | 3 für 2022 | `recht-governance/kst-26643-928.txt` |
| WMEBV-HR | BZK, Handreiking implementatie Wmebv v3 (2024-12): <https://www.digitaleoverheid.nl/wp-content/uploads/sites/8/2024/12/handreiking-implementatie-wet-modernisering-elektronisch-bestuurlijk-verkeer-awb-2023-v3.pdf> | 2 (Arbeitshilfe, teils veraltet) | `recht-governance/wmebv-hr-v3.txt` |
| MO-1 | MijnOverheid, Veelgestelde vragen Berichtenbox: <https://mijn.overheid.nl/nl/vragen/berichtenbox> | 3 | `betreiber/mo_veelgestelde-vragen.md` |
| MO-2 | MijnOverheid, Wat is de Berichtenbox?: <https://mijn.overheid.nl/nl/wat-is-de-berichtenbox> | 3 | `betreiber/mo_wat-is-de-berichtenbox.md` |
| MO-3 | MijnOverheid, Bezorgbevestiging: <https://mijn.overheid.nl/nl/wat-is-de-berichtenbox/bezorgbevestiging> | 3 | `betreiber/mo_bezorgbevestiging.md` |
| MO-4 | MijnOverheid, Berichtenbox – Machtigen: <https://mijn.overheid.nl/nl/wat-is-de-berichtenbox/machtigen> | 3 | `betreiber/mo_berichtenbox-machtigen.md` |
| MO-5 | MijnOverheid, Wat is MijnOverheid: <https://mijn.overheid.nl/nl/wat-is-mijnoverheid> | 3 | `betreiber/mo_wat-is-mijnoverheid.md` |
| MO-6 | MijnOverheid, Privacyverklaring (2026-09-09): <https://mijn.overheid.nl/nl/privacy> | 3 | `betreiber/mo_privacyverklaring.md` |
| DIGID-1 … DIGID-5 | digid.nl: Inlogmethodes, DigiD-App, SMS-Kontrolle, Identiteitsbewijs, Machtigen: <https://www.digid.nl/inlogmethodes>, <https://www.digid.nl/aanvragen-en-activeren/machtigen> | 3 | `betreiber/digid_*.md` |
| LOG-MO-1 | Logius, MijnOverheid – Hoe werkt het: <https://www.logius.nl/onze-dienstverlening/interactie/voorzieningen/mijnoverheid/hoe-werkt-het> | 3 | `betreiber/logius_mijnoverheid-hoe-werkt-het.md` |
| LOG-MO-2 | Logius, Notificatieteksten MijnOverheid vanaf 2026: <https://www.logius.nl/onze-dienstverlening/interactie/voorzieningen/mijnoverheid/documentatie/Notificatieteksten%20MijnOverheid> | 3 | `betreiber/logius_notificatieteksten-2026.md` |
| LOG-BB-1 | Logius, MijnOverheid Berichtenbox (Aansluiten): <https://www.logius.nl/onze-dienstverlening/interactie/voorzieningen/mijnoverheid/mijnoverheid-berichtenbox> | 3 | `anbindung-technik/logius_mijnoverheid-berichtenbox.md` |
| LOG-BB-2 | Logius, Technische Aansluithandleiding MijnOverheid Berichtenbox 1.6.4 (HTML): <https://www.logius.nl/onze-dienstverlening/interactie/voorzieningen/mijnoverheid/documentatie/technische-aansluithandleiding-mijnoverheid-berichtenbox> | 3 | `anbindung-technik/logius_bb_technische-aansluithandleiding.md` |
| LOG-BB-3 | Logius, Technische Aansluithandleiding v1.6.3 (PDF, 2020, abgelöst): <https://logius.nl/sites/default/files/bestanden/website/Technische%20Aansluithandleiding%20MijnOverheid%20Berichtenbox%20v1.6.3.pdf> | 2 | `anbindung-technik/logius_bb_technische-aansluithandleiding_v1.6.3.pdf` |
| LOG-BB-4 | Logius, Checklist Testen Berichtenbox 1.4: <https://www.logius.nl/onze-dienstverlening/interactie/voorzieningen/mijnoverheid/documentatie/checklist-testen-berichtenbox-mijnoverheid> | 3 | `anbindung-technik/logius_bb_checklist-testen.md` |
| LOG-BB-5 | Logius, Handreiking Impact Analyse Berichtenbox 1.3 (2018, weiter verlinkt): <https://logius.nl/sites/default/files/public/bestanden/diensten/MijnOverheid/Handreiking-Impactanalyse-Berichtenbox.pdf> | 3 | `anbindung-technik/logius_bb_handreiking-impactanalyse.pdf` |
| LOG-BB-6 | Logius, Serviceniveau-Overeenkomst MijnOverheid 2.0 (2018): <https://logius.nl/sites/default/files/public/bestanden/diensten/MijnOverheid/Service-Niveau-Overeenkomst-Mijn-Overheid_0.pdf> | 3 für 2018, sonst 2 | `anbindung-technik/logius_sno-mijnoverheid.pdf` |
| LOG-BB-7 | Logius, XSD MijnOverheid Berichtenbox: <https://logius.nl/sites/default/files/public/bestanden/diensten/MijnOverheid/logius-mijnoverheid-berichtenbox-xsd-2026.zip> | 3 | `anbindung-technik/logius_bb_xsd-2026.zip` |
| LOG-FBS-1 | Logius, Federatief Berichten Stelsel: <https://www.logius.nl/onze-dienstverlening/interactie/federatief-berichten-stelsel> | 3 (Zielbild) | `anbindung-technik/logius_fbs.md` |
| LOG-FBS-2 | Logius, Wat is het FBS?: <https://www.logius.nl/onze-dienstverlening/interactie/federatief-berichten-stelsel/wat-het> | 3 (Zielbild) | `anbindung-technik/logius_fbs_wat-is-het.md` |
| LOG-FBS-3 | Logius, Hoe werkt het FBS?: <https://www.logius.nl/onze-dienstverlening/interactie/federatief-berichten-stelsel/hoe-werkt-het> | 3 (Zielbild) | `anbindung-technik/logius_fbs_hoe-werkt-het.md` |
| LOG-FBS-4 | Logius, Ontwikkeling FBS (2026-10-22): <https://www.logius.nl/onze-dienstverlening/interactie/federatief-berichten-stelsel/ontwikkeling-federatief-berichten-stelsel> | 3 | `anbindung-technik/logius_fbs_ontwikkeling.md` |
| LOG-FBS-5 | Logius, Planungsgrafik Migration GLOBE nach BBO (10/2026): <https://logius.nl/sites/default/files/website/diensten/fbs/bestanden/logius-fbs-planning-102026.jpg> | 3 (Plan) | `anbindung-technik/logius_fbs_planning_2026-10.jpg` |
| LOG-FBS-6 | Logius, Servicebericht Aansluiten op MijnOverheid Berichtenbox niet meer mogelijk (2026-11-02): <https://www.logius.nl/actueel/aansluiten-op-mijnoverheid-berichtenbox-niet-meer-mogelijk> | 3 | `anbindung-technik/logius_servicebericht_aansluiten-niet-meer-mogelijk.md` |
| LOG-FBS-7 | Logius, Servicebericht Planning Berichtenbox-migratie (2026-10-22): <https://www.logius.nl/actueel/planning-berichtenbox-migratie-bekend-kijk-wat-dit-voor-u-betekent> | 3 | `anbindung-technik/logius_servicebericht_planning-migratie.md` |
| LOG-FBS-8 | Logius, Technische vernieuwing MijnOverheid Berichtenbox gestart (2022-04-15): <https://www.logius.nl/actueel/technische-vernieuwing-mijnoverheid-berichtenbox-gestart> | 3 (Planung überholt) | `anbindung-technik/logius_nieuws_technische-vernieuwing-2022.md` |
| MOZA-1 | MijnOverheid Zakelijk, Berichten (2026-11-16): <https://mijnoverheidzakelijk.nl/onderwerpen/berichten/> | 3 | `anbindung-technik/moza_berichten.md` |
| MOZA-2 | MijnOverheid Zakelijk, Federatief Berichtenstelsel (2026-10-20): <https://mijnoverheidzakelijk.nl/onderwerpen/berichten/fbs/> | 3 (Zielbild, Pilot) | `anbindung-technik/moza_fbs.md` |
| MOZA-3 | MijnOverheid Zakelijk, Notificatiedienst: <https://www.mijnoverheidzakelijk.nl/onderwerpen/notificatiedienst/> | 3 (Plan) | `betreiber/moza_notificatiedienst.md` |
| MOZA-POC-1 … 4 | MinBZK, Proof of Concept FBS-Berichtenbox (README, Aanpak en keuzes, PoC-afwijkingen, OpenAPI Berichtenmagazijn 0.5.0), Commit 8bdea83: <https://github.com/MinBZK/moza-poc-fbs-berichtenbox> | 2 (Proof of Concept) | `anbindung-technik/moza-poc_*.md`, `anbindung-technik/moza-poc_berichtenmagazijn-api.yaml` |
| DO-BBVB-1 … 3 | Digitale Overheid, Berichtenbox voor bedrijven: Überblick, Aansluiten, Verschil met Berichtenbox voor burgers: <https://www.digitaleoverheid.nl/overzicht-van-alle-onderwerpen/berichtenbox-voor-bedrijven/> | 3 | `anbindung-technik/digitaleoverheid_*.md` |
| RVO-1 | RVO, Berichtenbox voor bedrijven: <https://www.rvo.nl/onderwerpen/berichtenbox> | 3 | `anbindung-technik/rvo_berichtenbox.md` |
| OP-1 … OP-3 | Ondernemersplein, Berichtenbox voor bedrijven: Überblick, Veelgestelde vragen, Aanmaken: <https://ondernemersplein.overheid.nl/berichtenbox-voor-bedrijven> | 3 | `betreiber/rvo_berichtenbox-voor-bedrijven*.md` |
| EH-1 … EH-3 | eherkenning.nl: Wat is eHerkenning, Betrouwbaarheidsniveaus, Machtigen: <https://www.eherkenning.nl/nl/wat-is-eherkenning> | 3 | `betreiber/eh_*.md` |
| NLWW-1, NLWW-2 | Nederland Wereldwijd, Berichtenbox und Berichtenbox gemachtigd zijn: <https://www.nederlandwereldwijd.nl/mijnoverheid-buiten-nederland/berichtenbox> | 3 | `betreiber/nlww_berichtenbox*.md` |
| TK-1 | Kamerbrief 36 600 VII, nr. 137, gesetzliche Vertretung (2025-05-16): <https://zoek.officielebekendmakingen.nl/kst-36600-VII-137.pdf> | 3 für 2025 | `betreiber/tk_kst-36600-VII-137_wettelijke-vertegenwoordiging.pdf` |
| TK-2 | Kamerbrief 26 643, nr. 942, Zwei-Faktor-Anmeldung (2022-11-22): <https://zoek.officielebekendmakingen.nl/kst-26643-942.pdf> | 2 (Ankündigung) | `betreiber/tk_kst-26643-942_inloggen-2fa.pdf` |
| DK-1 | Logius, Wat is Digikoppeling? 1.1.2 (2022): <https://gitdocumentatie.logius.nl/publicatie/dk/watisdk/Digikoppeling-Wat-is-Digikoppeling.pdf> | 3 | `anbindung-technik/dk_wat-is-digikoppeling.pdf` |
| ACICT-1 | AcICT, BIT-Advies FBS (2021-12-22): <https://zoek.officielebekendmakingen.nl/blg-1018139.pdf> | 3 für 2021, sonst 2 | `anbindung-technik/acict_advies-fbs_blg-1018139.pdf` |
| TED-1 | TED 55336-2026, Applicatiediensten Berichtenbox: <https://ted.europa.eu/en/notice/55336-2026/pdf> | 3 | `anbindung-technik/ted_55336-2026.pdf` |
| SEK-1 | Forum AVROTROS Radar (2018), Intervall der Erinnerung | 2 | `betreiber/sekundaer_radar-forum-2018_herinnering.md` |

Weitere lokale Quellen ohne Kürzel in dieser Tabelle (VNG, CGI, Forum Standaardisatie, DigiD-Unterseiten) stehen in den `_index.md` der Unterordner.

## Nachweise je Dimension

| Dimension | Aussage | Quellen | Stufe |
|---|---|---|---|
| B1 | Digikoppeling ebMS2 (Batch) und WUS (Opt-in-Prüfung), eigenes Gateway mit OIN und PKIoverheid oder Intermediär | LOG-BB-2 Z. 18–38, 147–154, 702–710; LOG-BB-1 Z. 21–27, 68–73 | 3 |
| B1 | Berichtenbox voor bedrijven über Digikoppeling | DO-BBVB-1 Z. 30–33; DO-BBVB-3 Z. 23–24 | 3 |
| B1 | FBS: Ablage im eigenen Magazin, Anmeldung im Verbund; im Proof of Concept REST-API über FSC [Indiz] | LOG-FBS-3 Z. 15–23; LOG-FBS-1 Z. 9–16; MOZA-POC-4; MOZA-POC-2 Z. 72–73 | 3, Proof of Concept: 2 |
| B1, I1 | keine neuen GLOBE-Anschlüsse seit 2.11.2026, Umzug auf BBO 2027 geplant | LOG-FBS-6 Z. 9–11; LOG-FBS-4 Z. 19–24; LOG-FBS-5 | 3 |
| B2 | kein Web-Zugang zum Versenden [Indiz]; Leveranciersportaal nur Statistik und Standardanhänge; kein hybrider Versand | Fehlanzeige in LOG-BB-1, LOG-BB-2; LOG-BB-2 Z. 339–342; LOG-BB-5 PDF-S. 8 | 3, kein Web-Zugang: 2 |
| B2 | Berichtenbox voor bedrijven mit Web-Zugang für Behörden | DO-BBVB-1 Z. 30–31; DO-BBVB-3 Z. 23–24 | 3 |
| B3 | BSN, Pflichtprüfung des Abonnements vor dem Versand | LOG-BB-2 Z. 59–62, 210–235, 386; MO-6 Z. 213–217 | 3 |
| B3 | Berichtenbox voor bedrijven: Name der Box im KVK-Handelsregister | DO-BBVB-3 Z. 12–14; OP-3 Z. 54–62 | 3 |
| B3 | FBS unterstützt BSN, RSIN, KVK-Nummer und andere Kennungen (Plan) | LOG-FBS-2 Z. 34–37; MOZA-1 Z. 63 | 3 |
| B4 | Betreff 50, Text 4000 Zeichen; zwei persönliche PDF-Anhänge mit 500 kB, drei Standardanhänge; Link bei großen Dateien; Frist als Metadatum | LOG-BB-2 Z. 78–80, 338–342, 356–364, 383–384, 397–406, 665–667 | 3 |
| B4 | Berichtenbox voor bedrijven: bis 20 Anhänge mit je 100 MB | DO-BBVB-3 Z. 15–18; RVO-1 Z. 46 | 3 |
| B5 | PDF/A-1a oder PDF/A-2a Pflicht, WCAG; XML nach GLOBE-XSD, PDF Base64 | LOG-BB-2 Z. 388–389, 426, 556–567; LOG-BB-7 | 3 |
| P1 | E-Mail-Hinweis mit Absender, Anlass und Frist, ohne Link | MO-1 Z. 232, 248; LOG-MO-2 Z. 20–28 | 3 |
| P1 | Hinweis Pflicht binnen 48 Stunden, außer bei Verzicht (art. 2:10 Abs. 1 Awb, seit 1.1.2026) | AWB Z. 238; STB-2024-321 Z. 108–116 | 3 |
| P1 | Push in der Berichtenbox-App; Erinnerung bei Nichtabruf einschaltbar, Intervall [Indiz] | MO-1 Z. 174, 234; MO-2 Z. 85; SEK-1 Z. 110 | 3, Intervall: 2 |
| P1 | Papier nur zusätzlich durch die Behörde | MO-2 Z. 122; LOG-MO-1 Z. 59 | 3 |
| P1 | FBS plant E-Mail, SMS oder App nach Notificatieprofiel | LOG-FBS-3 Z. 68–74; MOZA-3 Z. 158–163 | 3 (Plan) |
| P2 | DigiD (Passwort, SMS, App, Ausweis mit Chip) oder eIDAS-Mittel | MO-1 Z. 26; MO-5 Z. 89–98; DIGID-1; DIGID-2 Z. 43, 71–80; DIGID-4 Z. 25 | 3 |
| P2 | Bezorgbevestiging nur mit DigiD-App, SMS oder Berichtenbox-App | MO-3 Z. 69 | 3 |
| P2 | Zwei-Faktor-Pflicht seit 2023 [Indiz] | TK-2 S. 1 | 2 |
| P2 | Berichtenbox voor bedrijven mit eHerkenning ab EH2+, private zugelassene Anbieter; EU-Unternehmen mit eigenem Konto | OP-3 Z. 41; EH-1 Z. 64; EH-2 Z. 12–91; OP-1 Z. 38–39; OP-2 Z. 80 ff. | 3 |
| P3 | MijnOverheid nur für natürliche Personen | MO-1 Z. 23 ff.; LOG-MO-1 Z. 25 | 3 |
| P3 | Berichtenbox voor bedrijven: ein Postfach, mehrere Bevollmächtigte, Rechte im eHerkenning-Machtigingenregister | OP-2 Z. 92–98, 313, 322; EH-3 Z. 29–67 | 3 |
| P4 | DigiD Machtigen je Organisation und Dienst; eigenes Konto der Bevollmächtigten; nicht in der App, nicht bei Bezorgbevestiging | MO-4 Z. 27–57; DIGID-5 Z. 19–25; MO-1 Z. 350–354, 443–469; MO-3 Z. 81; NLWW-2 Z. 12, 22 | 3 |
| P4, P10 | gesetzliche Vertretung nicht abgebildet [Indiz]; Register vorhanden, Abfragedienst im Pilot | TK-1 S. 2–4; Fehlanzeige in MO-1, MO-4 | 3 für 2025, MijnOverheid: 2 |
| P5 | mijn.overheid.nl und Berichtenbox-App; Einstellungen je Behörde nur im Web; keine Drittclients [Indiz] | MO-1 Z. 299–348, 335–340; LOG-MO-1 Z. 33–38 | 3, Drittclients: 2 |
| P5 | Unternehmen: Berichtenbox voor bedrijven und Mijn Belastingdienst Zakelijk nebeneinander | MOZA-1 Z. 40–50 | 3 |
| P6 | keine Schnittstelle für Personen und Unternehmen [Indiz]; Unternehmenssoftware als Ziel von MijnOverheid Zakelijk | Fehlanzeige; OP-2 Z. 274; MOZA-1 Z. 31 | 2, Ziel: 3 |
| P7, P8 | Personen nur Empfang | MO-1 Z. 110–116; MO-2 Z. 70; MOZA-1 Z. 47; WMEBV-HR Z. 877 | 3 |
| P7, P8 | Berichtenbox voor bedrijven: Nachricht an Behörden nach der Dienstenwet, Freitext und Anhänge, optional signiert | OP-2 Z. 151, 209–212, 251; OP-1 Z. 33 | 3 |
| P9 | Konto je BSN ab 14 Jahren, zu aktivieren, nicht kündbar; Nachrichten bis zur Löschung durch die Person | MO-1 Z. 60–66, 95–98, 121–123, 278–280; LOG-MO-1 Z. 13–17; RVWDO Z. 140 ff.; NLWW-1 Z. 22 | 3 |
| P9 | Berichtenbox voor bedrijven: eigener Dienst, Löschung nach 5 Jahren | OP-2 Z. 261, 318, 408–413 | 3 |
| P10 | DigiD Machtigen: Enddatum, Code per Brief an die Meldeadresse, Widerruf | DIGID-5 Z. 24–25, 60, 86–107; MO-1 Z. 418, 447–450 | 3 |
| P10 | Unternehmen: Befugnis aus dem KVK-Handelsregister über eHerkenning | EH-3 Z. 50–52; OP-2 Z. 360 | 3 |
| Ü1 | asynchron, Batch über ebMS Reliable Messaging | LOG-BB-2 Z. 137–146, 432–436 | 3 |
| Ü2 | TLS mit gegenseitiger Authentifizierung über PKIoverheid; Verschlüsselung der Ablage [Indiz] | LOG-BB-2 Z. 116–131, 158–171; Fehlanzeige | 3, Ablage: 2 |
| Ü3 | Absender über OIN, Nutzdaten nicht signiert; keine Signatur auf dem Dokument [Indiz] | LOG-BB-2 Z. 127–129, 172–179, 538–539, 689; Fehlanzeige | 3, Dokument: 2 |
| Ü4 | ebMS-Quittung, Ergebnis je Batch und Nachricht, kein Lesestatus | LOG-BB-2 Z. 91–93, 192–201, 407–502; MO-1 Z. 238–241 | 3 |
| Ü4 | auf Anfrage Angabe zum Öffnen für eine Erinnerung | MO-6 Z. 220–228; BDO Z. 517 | 3 |
| Ü4 | Bezorgbevestiging: Zeitpunkt der ersten Anmeldung nach dem Versand | MO-3 Z. 53–58 | 3 |
| Ü4 | Nutzungsverlauf für die Person; Eingangsbestätigung in der Berichtenbox voor bedrijven | MO-6 Z. 307; OP-2 Z. 159 | 3 |
| Ü5 | keine Interoperabilität [Indiz] | Fehlanzeige in LOG-BB-2, LOG-FBS-1 bis 3, MOZA-2 | 2 |
| Ü6 | nur PDF/A, Prüfungen vor der Ablage; Virenprüfung [Indiz] | LOG-BB-2 Z. 389, 415–426; Fehlanzeige | 3, Virenprüfung: 2 |
| Ü7 | Digikoppeling ebMS2 / WUS, Pflichtstandard „pas toe of leg uit“ | LOG-BB-2 Z. 34–38; DK-1 PDF-S. 10, 14 | 3 |
| Ü7 | FBS: Magazin-API über FSC, NL GOV CloudEvents [Indiz] | MOZA-POC-2 Z. 61–76; MOZA-2 Z. 107–108, 137–143 | 2 |
| I1 | Logius im Auftrag von BZK; Minister BZK verantwortlich nach art. 5 Wdo | LOG-FBS-1 Z. 9; KST-816 PDF-S. 1; WDO Z. 159, 178; LOGIUS-IB Z. 16 | 3 |
| I1 | Berichtenbox voor bedrijven bei RVO mit DICTU, kostenlos | DO-BBVB-1 Z. 46–58; RVO-1 Z. 47–49; MOZA-1 Z. 40–45 | 3 |
| I1 | Logius rechnet Anschluss und Nutzung ab [Indiz] | nur Suchtreffer, Logius-Voorwaarden HTTP 404 | 2 |
| I1 | FBS: zentrale Dienste, Magazine der Behörden und BBO, Afsprakenstelsel, neues Kostenmodell; BBO ausgeschrieben 2026 bis 2030 | LOG-FBS-1 Z. 15–16; LOG-FBS-2 Z. 38–49; LOG-FBS-3 Z. 31–52; TED-1 PDF-S. 1–2 | 3 (Zielbild) |
| I2 | heute zentral in GLOBE, keine Löschung durch Behörde oder Logius, Archivpflicht bei der Behörde | LOG-FBS-3 Z. 10–11; LOG-BB-2 Z. 87–90; LOG-BB-5 PDF-S. 8 | 3 |
| I2 | FBS: Nachricht bleibt im Magazin der Behörde, zentral Nachrichtenliste und befristet Metadaten | LOG-FBS-2 Z. 24, 31–33; MOZA-2 Z. 40–49, 102–106 | 3 (Zielbild) |
| I3 | heute nur Absender, Intermediär möglich; FBS Phase 1 BBO, Phase 2 eigenes oder gehostetes Magazin | LOG-BB-2 Z. 18–29; MOZA-1 Z. 54–55; MOZA-2 Z. 9, 40–43, 90, 127–147 | 3 |
| I4 | MijnOverheid: Behörden empfangen nicht; Berichtenbox voor bedrijven ohne Ende-zu-Ende-Verschlüsselung [Indiz] | LOG-BB-2 Z. 55–58; DO-BBVB-2 Z. 24–37 | 3, Verschlüsselung: 2 |
| I5 | Absender: öffentliche Aufgabe, BSN-Berechtigung, Antrag, Verwerkersovereenkomst, Freigabe des Testberichts | LOG-BB-1 Z. 42–107; LOG-BB-4 Z. 123–127; LOG-BB-2 Z. 106–115 | 3 |
| I5 | FBS: Stelselafspraken angekündigt, nicht veröffentlicht [Indiz]; Stelseldocument im Proof of Concept [Indiz] | KST-816 PDF-S. 4; ACICT-1 PDF-S. 5; MOZA-2 Z. 107–108, 138–139; MOZA-POC-1 Z. 51 | 2 |
| R1 | Schriftlichkeit elektronisch erfüllt; Zustimmung für Behörde an Person; benannte Kanäle für Person an Behörde | AWB Z. 218, 228, 272, 335; KST-MVT Z. 1962; UWV-AANW Z. 20; WMEBV-HR Z. 877 | 3 |
| R2 | Opt-in je Behörde in MijnOverheid | RVWDO Z. 148, 157, 160; AWB Z. 228; MO-1 Z. 104; MO-6 Z. 95, 213–217 | 3 |
| R2 | Belastingdienst: ganz elektronisch oder ganz Papier, Pflicht per Regeling | AWR Z. 139, 164; RBB Z. 20, 167 | 3 |
| R2 | Berichtenbox voor bedrijven nur nach Erklärung des Unternehmens | DW Z. 253 | 3 |
| R3 | Bekanntgabe mit Zugänglichwerden, Vorrang MijnOverheid, Entschuldbarkeit, Beweislast | AWB Z. 351, 392, 417, 809, 3013; KST-MVT Z. 1811–1857 | 3 |
| R3 | keine förmliche Zustellung mit Nachweis [Indiz] | Fehlanzeige in AWB Afdeling 2.3, WMEBV-HR, KST-MVT | 2 |
| R4 | Anschluss freiwillig, Pflicht per Verordnung möglich, nicht genutzt [Indiz] | WDO Z. 209; Fehlanzeige in BDO; WMEBV-HR Z. 731 | 3, nicht genutzt: 2 |
| R4 | Berichtenbox voor bedrijven Pflicht nach Dienstenwet; Annahme elektronischer Eingänge nach art. 2:13 Awb seit 2026 | DW Z. 253; WMEBV-HR Z. 867; AWB Z. 272; STB-2024-321 Z. 116, 121 | 3 |

## Evidenzstufen der markierten Stellen

Stand 2026-11-26. Stufe 1 und 2 werden über den genannten Prüfweg geprüft.

| Datei, Dimension | Aussage | Stufe | Begründung | Prüfweg |
|---|---|---|---|---|
| `existing_solutions.csv`, B1 | FBS-Anlieferung per REST-API über FSC | 2 | nur im Proof of Concept von MOZa (Pre-Alpha); Schnittstelle des BBO für Absender nicht veröffentlicht | Spezifikation BBO bei Logius anfragen |
| `existing_solutions.csv`, B2 | kein Web-Zugang zum Versenden | 2 | Fehlanzeige in LOG-BB-1 und LOG-BB-2 | Anfrage Logius |
| `existing_solutions.csv`, P1 | Intervall der Erinnerung | 2 | „drei Wochen“ nur in einem Forum von 2018 (SEK-1) | Test mit einem MijnOverheid-Konto; Anfrage Logius |
| `existing_solutions.csv`, P2 | Zwei-Faktor-Pflicht seit 2023 | 2 | nur angekündigt (TK-2), Umsetzung nicht im Volltext geprüft | DigiD- / MijnOverheid-Hilfeseiten, Kamerbrief zur Umsetzung |
| `existing_solutions.csv`, P4 und P10 | gesetzliche Vertretung nicht abgebildet bzw. nicht an MijnOverheid angebunden | 2 | TK-1 (Stand 05/2025) und Fehlanzeige auf mijn.overheid.nl | aktueller Kamerbrief zu Vertegenwoordiging; Anfrage Logius |
| `existing_solutions.csv`, P5 | keine Drittclients | 2 | Fehlanzeige | Anfrage Logius |
| `existing_solutions.csv`, P6; `morph_box.csv`, P6 Ausprägung 1 | keine Schnittstelle für Personen und Unternehmen | 2 | Fehlanzeige; Berichtenbox voor bedrijven nur PDF-Export (OP-2) | Anfrage RVO und Logius |
| `existing_solutions.csv`, Ü2; `morph_box.csv`, Ü2 Ausprägung 1 | Verschlüsselung der Ablage | 2 | Fehlanzeige in LOG-BB-2, LOG-FBS-1 bis 3 | Sicherheitskonzept GLOBE / BBO, Anfrage Logius |
| `existing_solutions.csv`, Ü3 | keine Signatur auf dem Dokument | 2 | Fehlanzeige; Nutzdaten laut LOG-BB-2 nicht signierbar | Anfrage Logius |
| `existing_solutions.csv`, Ü5; `morph_box.csv`, Ü5 Ausprägung 1 | keine Interoperabilität | 2 | Fehlanzeige zu SDG / OOTS und eDelivery | Anfrage BZK |
| `existing_solutions.csv`, Ü6; `morph_box.csv`, Ü6 Ausprägung 7 | Virenprüfung nicht dokumentiert | 2 | LOG-BB-2 nennt nur Größe, Schema, Opt-in und Format | Anfrage Logius |
| `existing_solutions.csv`, Ü7; `morph_box.csv`, Ü7 Ausprägung 4 | FBS-Magazin-API über FSC, NL GOV CloudEvents | 2 | Proof of Concept | veröffentlichte Stelselafspraken und Spezifikation abwarten |
| `existing_solutions.csv`, I1 | Logius rechnet Anschluss und Nutzung ab | 2 | nur Suchtreffer, Voorwaarden-PDF HTTP 404 | aktuelle Voorwaarden / Tarife bei Logius |
| `existing_solutions.csv`, I4 | Berichtenbox voor bedrijven ohne Ende-zu-Ende-Verschlüsselung | 2 | Rückschluss: Web-Postfach mit mehreren Bevollmächtigten, keine Schlüsselverwaltung beschrieben | Anfrage RVO |
| `existing_solutions.csv`, I5; `morph_box.csv`, I5 Ausprägung 4 | Stelselafspraken nicht veröffentlicht; Stelseldocument als Teilnehmerliste | 2 | Fehlanzeige; Stelseldocument nur im Proof of Concept | Anfrage Logius / BZK |
| `existing_solutions.csv`, R3; `morph_box.csv`, R3 Ausprägung 6 | keine förmliche Zustellung mit Nachweis | 2 | Fehlanzeige in Awb Afdeling 2.3, Handreiking und MvT | Anfrage BZK |
| `existing_solutions.csv`, R4; `morph_box.csv`, R4 Ausprägung 1 | Anschlusspflicht nach art. 5 Wdo nicht genutzt | 2 | Fehlanzeige im Besluit digitale overheid | aktuelle AMvB-Liste unter der Wdo prüfen |

## Widersprüche zwischen Quellen

- **Lesestatus (Ü4):** Die FAQ (MO-1 Z. 238–241) und die Aansluithandleiding (LOG-BB-2 Z. 91–93) sagen, Behörden erfahren nicht, ob gelesen wurde. Die Privacyverklaring (MO-6 Z. 222) und art. 8 Besluit digitale overheid (BDO Z. 517) erlauben auf Anfrage eine Angabe zum Öffnen, um an die Nachricht zu erinnern. In der CSV stehen beide Aussagen.
- **Pflicht für Unternehmen bei der Berichtenbox voor bedrijven (R2):** Die Handreiking (WMEBV-HR Z. 491) spricht von „moeten gebruiken“; art. 14 Abs. 1 b Dienstenwet verlangt eine Erklärung des Unternehmens. Die CSV folgt dem Gesetz.
- **Inkrafttreten der Hinweispflicht (P1):** Die Presse meldet 2027; laut STB-2024-321 gilt art. 2:10 Abs. 1 seit 1.1.2026, nur Abs. 2 (Inhalt des Hinweises) ab 1.1.2027. Die CSV folgt dem Staatsblad.
- **Anhänge der Berichtenbox voor bedrijven (B4, P8):** 15 oder 20 Anhänge je nach Seite (OP-2, OP-3, DO-BBVB-3); in der CSV steht 20 nach DO-BBVB-3.
