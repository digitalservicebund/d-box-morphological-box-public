# ELSTER-Postfach (Deutschland): Quellen

Nachweise für die Spalte `ELSTER-Postfach (Deutschland)` in `existing_solutions.csv` und die ELSTER-Einträge in `morph_box.csv`. Statt `[unverifiziert]` stehen hier und in den ELSTER-Einträgen der CSV-Dateien `[Indiz]` (Stufe 2) und `[Annahme]` (Stufe 1); unmarkierte Aussagen sind Stufe 3. Die frühere Kennzeichnung `[Web]` (nur Webquelle, nicht in der ERiC-Doku) entfällt: Aussagen aus Primärquellen im Web sind Stufe 3, die übrigen `[Indiz]`. `Fehlanzeige` heißt: in den Quellen gesucht, nicht gefunden.

Evidenzstufen (Spalte `Stufe`):

- **3 (Belegt / Primärquelle):** Gesetzestext, Drucksache, Verwaltungsvorschrift oder Betreiberdokument, in der lokalen Kopie oder (Webquellen ohne lokale Kopie) beim Abruf im Volltext geprüft.
- **2 (Sekundär / Indiz):** Herstelleraussage, Länderpräsentation, Pressemitteilung, Beratungsblog, Abschrift eines Dritten, veraltete Angabe, nicht geöffnete Quelle oder Rückschluss aus einer Primärquelle; `Fehlanzeige` bei gezielter Suche zählt als Indiz für eine negative Aussage.
- **1 (Annahme / Hypothese):** kein Beleg, nicht einmal ein Indiz; offen und zu prüfen.

Bei gemischten Zeilen steht zuerst die Stufe des belegten Teils, danach die Stufe des Teils, der in der Aussage mit `[Indiz]` oder `[Annahme]` markiert ist. Ausführliche Einordnung: `ELSTER/ELSTER-Zusammenfassung.md`.

Stand: 2026-11-17, Belege geprüft gegen ERiC-Dokumentation 44.1.6.0 (Mai 2026) und die unten genannten Webseiten; Evidenzstufen 2026-11-23. Seitenangaben sind gedruckte Seitenzahlen.

## Hauptquellen

Lokale Pfade relativ zu `sources/ELSTER/Dev Dokumentation/`.

| Kürzel | Quelle | Stufe |
|---|---|---|
| EHB | ERiC-Entwicklerhandbuch, 514 Seiten: `ERiC-Entwicklerhandbuch.pdf` | 3 |
| SSb-B | Schnittstellenbeschreibung ElsterBereitstellung V5.0.8, Senderseite: `Datenarten/ElsterBereitstellung/Dokumentation/ElsterBereitstellung-SSB-V5.0.8.pdf` | 3 |
| SSb-D | Schnittstellenbeschreibung ElsterDatenabholung v32, Empfängerseite: `Datenarten/ElsterDatenabholung/Dokumentation/ElsterDatenabholung-v32.pdf` | 3 |
| EDS | Einheitliche Datenschnittstelle XML 1.1: `ElsterBasisSchema/Dokumentation/Einheitliche_Datenschnittstelle_XML_11.pdf` | 3 |
| LIZ | ERiC-Lizenzvertrag: `lizenz.pdf` | 3 |
| XSD-BN | Schema Belegnachreichung: `Datenarten/ElsterNachricht/Belegnachreichung/Schema/24/Belegnachreichung-24-Nutzdaten.xsd` | 3 |
| FA-ODS | Finanzamtsverzeichnis: `Finanzamtsdaten.ods` | 3 |
| ZaPuK | ZaPuK-ADRs: `ZaPuK/quellen/zapuk-v1.1.1/Zielarchitektur/Architekturentscheidungen/` | 2 (Zielarchitektur, für ELSTER Sekundärquelle) |
| ERFOLG | ELSTER, eine Erfolgsstory: <https://www.elster.de/eportal/infoseite/elster_eine_erfolgsstory> | 3 |
| RZ | BayLfSt, Informationen zu den Rechenzentren 03/2026: <https://www.lfst.bayern.de/fileadmin/RESSOURCEN/LfSt/Digital%40BayLfSt/Informationen_zu_den_Rechenzentren_03_2026.pdf> | 3 |
| SECURE | ELSTER, ElsterSecure: <https://www.elster.de/bportal/infoseite/elstersecure> | 3 |
| ETR | ELSTER, Was ist ELSTER-Transfer?: <https://www.elster.de/elsterweb/infoseite/elstertransfer_hilfe_einfuehrung> | 3 |
| LF-GEWST | Leitfaden digitaler Gewerbesteuerbescheid über den ELSTER-Rückkanal, 24.08.2023: <https://download.elster.de/download/elstertransfer/dokumente/20230824_Leitfaden_DigGewStBescheid_ELSTER-Rueckkanal.pdf> | 3 |
| FAQ-GEWST | FAQ Digitaler Gewerbesteuerbescheid, 26.07.2024: <https://download.elster.de/download/elstertransfer/dokumente/20240726_FAQ_Digitaler_Gewerbesteuerbescheid.pdf> | 3 |
| P20 | Mein Unternehmenskonto, Postfach 2.0 Bedienung und Featureübersicht: <https://info.mein-unternehmenskonto.de/wp-content/uploads/2023/11/Postfach-2.0-Bedienung-und-Featureuebersicht-Postfach-2.0-1.0_komprimiert.pdf> | 3 |
| ZERT-LF | Leitfaden ELSTER-Organisationszertifikate, 03/2024: <https://info.mein-unternehmenskonto.de/wp-content/uploads/2024/03/20240313_Zertifikatsleitfaden.pdf> | 3 |
| VDB-B | BStBK, Vollmachtsdatenbank: <https://www.bstbk.de/de/themen/vollmachtsdatenbank> | 2 (nicht geöffnet) |
| FA-CB | Finanzamt Cottbus, Anleitung ELSTER-Kontaktformular: <https://finanzamt.brandenburg.de/sixcms/media.php/9/Anleitung-zur-Nutzung-des-ELSTER-Kontaktformulars_07-2026.pdf> | 3 |
| NDS-909 | Niedersächsischer Landtag, Drs. 18/909: <https://www.landtag-niedersachsen.de/Drucksachen/Drucksachen_18_02500/00501-01000/18-00909.pdf> | 3 |
| BAYVV | Bayern, VV Betriebsbedingungen des einheitlichen Organisationskontos vom 30.03.2022: `../../MUK/quellen/recht-governance/BayMBl_2022-237_VVBeBOrgBek_Betriebsbedingungen_Organisationskonto.pdf`, <https://www.gesetze-bayern.de/Content/Document/BayVV_206_D_12940/true> | 3 |
| HB-SV | Bremen, Senatsvorlage zur Verwaltungsvereinbarung Organisationskonto, 02.12.2022: <https://www.rathaus.bremen.de/sixcms/media.php/13/top%202_20221220_Verwaltungsvereinbarung_Organisationskonto_Bremen.pdf> | 2 (nicht geöffnet) |
| PM-161 | PM Bayerisches Finanzministerium Nr. 161/24 (Kopie schweinfurt.de): <https://schweinfurt.de/rathaus-politik/amtliche-bekanntmachungen/m_51974> | 2 (Pressemitteilung, Kopie, nicht geöffnet) |
| NRW-2015 | KONSENS-Broschüre NRW 2015: <https://www.finanzverwaltung.nrw.de/sites/default/files/asset/document/konsens_broschuere_2015_web_reduziert_0.pdf> | 2 (Broschüre 2015, nicht geöffnet) |
| PWC | PwC, Elektronische Bekanntgabe von Steuerbescheiden ab 2027: <https://blogs.pwc.de/de/steuern-und-recht/article/257477/elektronische-bekanntgabe-von-steuerbescheiden-ab-2027/> | 2 (Beratungsblog) |
| EBS | Ebner Stolz, Elektronische Bekanntgabe ab 01.01.2027: <https://www.ebnerstolz.de/de/unser-angebot/leistungen/steuerberatung/allgemeine-steuerberatung/elektronische-bekanntgabe-steuerbescheide-ab-01.01.2027-111408.html> | 2 (Beratungsseite) |
| KONSENS-G | KONSENS-Gesetz: <https://www.gesetze-im-internet.de/konsens-g/> | 3 |
| FVG | § 20 Finanzverwaltungsgesetz: <https://www.gesetze-im-internet.de/fvg_1971/__20.html> | 3 |
| AO-87a | § 87a AO: <https://www.gesetze-im-internet.de/ao_1977/__87a.html> | 3 |
| AO-122a | § 122a AO: <https://www.gesetze-im-internet.de/ao_1977/__122a.html> | 3 |
| AO-357 | § 357 AO: <https://www.gesetze-im-internet.de/ao_1977/__357.html> | 3 |
| EGAO-1 | Art. 97 § 1 EGAO: <https://www.gesetze-im-internet.de/aoeg_1977/art_97__1.html> | 3 |
| EGAO-28 | Art. 97 § 28 EGAO: <https://www.gesetze-im-internet.de/aoeg_1977/art_97__28.html> | 3 |
| AEAO | AEAO zu § 87a (Abschrift steuerschroeder.de): <https://www.steuerschroeder.de/steuergesetze/aeao/87a> | 2 (Abschrift eines Dritten) |
| BUZER | Änderungshistorie § 122a AO: <https://www.buzer.de/122a_AO.htm> | 2 (Drittanbieter, nicht geöffnet) |

## Nachweise je Dimension

| Dimension | Aussage | Quellen | Stufe |
|---|---|---|---|
| B1 | Finanzämter und speziell berechtigte Behörden liefern über ElsterBereitstellung (`EPBescheid`, `EPMitteilung`, `EPKurzmitteilung`) mit Organisationszertifikat; auch über GeCo der Landeskopfstellen und OZG-Plattform; ERiC als Client-Bibliothek | EHB 9.15, 9.15.4 (S. 417, 424 f.); SSb-B 4.1 (S. 6–7) | 3 |
| B1 | ELSTER-Transfer als Anbindungsweg in Postfach 2.0; Gewerbesteuerbescheide der Kommunen nicht über ERiC bereitstellbar, aber abholbar | ETR; P20 2.1; EHB 9.15.1 (S. 418) | 3 |
| B2 | ELSTER-Transfer-Anwendung mit Web- und Command-line-Interface, läuft auf Server oder Rechner, Ordnerschnittstelle und REST-API; für Bescheide in das Postfach zwingend, Mein ELSTER nicht für den Rückkanal geeignet | LF-GEWST S. 4–5; ETR | 3 |
| B2 | manueller Upload über die lokale Web-Oberfläche, Empfänger per Benutzerkonto-ID, PDF (nur in Screenshots belegt); manuelle Bekanntgabe „aktuell noch möglich“, REST-API empfohlen | LF-GEWST 4.3.2 (S. 10–11); P20 2.1; FAQ-GEWST Frage 9, 15 | 3 |
| B2 | kein zentraler Versanddienst [Indiz] | Fehlanzeige | 2 |
| B3 | Konto-ID (`accountId`), 10-stellig oder pseudonym `ek-` + 40 Hex-Zeichen, Auflösung über WIESEL; Bevollmächtigte über `vnid`; DIVA zusätzlich über Steuernummer oder Geschäftszeichen; unbekanntes oder inaktives Konto wird abgelehnt | SSb-B 5.2 (S. 12), 7.5–7.9 (S. 29–30), 8.5 (S. 35) | 3 |
| B4 | Bescheid als PDF; `EPKurzmitteilung` als Freitext; Anschreiben, HTML-Formatierung und Anhänge mit `DokumentHierarchieTyp` im Postfach 2.0 | EHB 9.15 (S. 417, 421, 424); SSb-B 5.3.4 (S. 16), 6.2.1 (S. 27), 11 (S. 43); SSb-D 9.1.1 (S. 32) | 3 |
| B4 | strukturierte Bescheiddaten neben dem PDF, nicht rechtsgültig; große Anhänge über Objektspeicher OTTER | EHB 7.4 (S. 184), Tabelle 9-9 (S. 292); EHB 9.3.2 | 3 |
| P1 | E-Mail ohne Inhalt an die im Konto hinterlegte Adresse (WIESEL, `AUTOMATISCH`), bei DIVA an genau eine Adresse; Pflicht bei DIVA, `EPBescheid`, `EPAntwort`; Vertreter:innen werden mitbenachrichtigt | SSb-B 4.1 (S. 7), 5.5, 5.5.1 (S. 19–20), 7.24, 7.25 (S. 33–34), 8.11 (S. 36) | 3 |
| P2 | Portalzertifikat als Datei, Sicherheitsstick oder Signaturkarte; nPA / eAT über Ad-hoc-Zertifikat (24 Stunden, nur im Speicher); ELSTER-Zertifikat auch Anmeldemittel der BundID | EHB 5.2.2, 5.2.2.4, 5.2.3 (S. 117–122); ZaPuK ADR-0014, Primärbeleg in `Referenzen-BundID-ZBP.md` (P2) | 3 |
| P2 | ElsterSecure-App ohne Zertifikat, Biometrie als zweiter Faktor | SECURE | 3 |
| P3 | Organisationszertifikat auf Basis der Steuernummer; mehrere Organisationszertifikate je Unternehmen, je Zertifikat ein Benutzerkonto und Mitarbeiterpostfach; zusätzlich Funktionspostfächer | ZERT-LF 1, 2 (S. 3) | 3 |
| P3 | clientseitig erzeugtes Zertifikat nur zum Abholen, darf „gemeinsames Geheimnis mehrerer Personen“ sein; Vertretung bei Abwesenheit nur im Testbetrieb | EHB 5.2.2.1 (S. 116), 7.4.1 (S. 184); SSb-D 6.1 (S. 9) | 3 |
| P4 | Empfangs- und Bekanntgabevollmacht, Bevollmächtigte mit eigener Kennung (`vnid`) und eigener Einwilligung; eigenes Postfach der Bevollmächtigten [Indiz] | EHB 9.16.1.3 (S. 429); SSb-B 7.7 (S. 30); eigenes Postfach: Fehlanzeige | 3, eigenes Postfach: 2 |
| P4 | Kammer-Vollmachtsdatenbank von BStBK und DATEV; ELSTER-Vollmachten ohne Vollmachtsvermutung; Abrufvollmacht ist keine Empfangsvollmacht | EHB 9.17.1.1, 9.17.2–9.17.4 (S. 443); VDB-B | 3 |
| P5 | Web-Portal Mein ELSTER; Steuer-, Kanzlei- und Lohnsoftware über ERiC, Lesen über ElsterDatenabholung, Schreiben über ElsterNachricht und `EPAntwort`; Lizenzvertrag | EHB 9.11, 9.15.3; LIZ § 1 | 3 |
| P6 | ERiC und einheitliche XML-Schnittstelle zur Clearingstelle; Lizenzvertrag und Hersteller-ID je Produkt; Einsatz in ERP-Software [Indiz] | EDS 1 (S. 5); EHB 6.5.3 (S. 170); LIZ § 1; ERP: Fehlanzeige | 3, ERP: 2 |
| P7 | anlasslose Nachricht an das Finanzamt über ElsterNachricht, Auswahl über Finanzamtsverzeichnis | EHB 9.11 (S. 347); FA-ODS | 3 |
| P7 | Antwort im Postfach 2.0 nur, wenn die Behörde `MetaInfoMap typ="ANTWORT"` mit Frist setzt; auf `OZGPPNachricht` keine Antwort | SSb-B 5.3.5 (S. 16); EHB 9.15.2 (S. 419) | 3 |
| P8 | strukturierte Datenarten Einspruch, Fristverlängerung, Belegnachreichung u. a.; Freitext als `SonstigeNachrichten` nur mit `send-Auth` | EHB 9.11 (S. 347), 9.11.6 (S. 361) | 3 |
| P8 | PDF-Anhänge mit Größenbegrenzung: `EPAntwort` höchstens 20 Anhänge, 36,9 MiB, je 10 MiB; Belegnachreichung nur PDF | EHB 9.15.5 (S. 426); XSD-BN | 3 |
| P9 | Postfach Bestandteil des Mein-ELSTER-Kontos; Mein ELSTER kein Nutzerkonto im Sinne des OZG [Indiz]; Löschung nach 180 Tagen, Bescheide nach 4 Jahren | ZaPuK ADR-1110; EHB 9.2.5 (S. 286); SSb-B 5.5.6 | 3, kein OZG-Nutzerkonto: 2 |
| Ü1 | asynchroner digitaler Brief: Bereitstellung, Abholung durch den Client, Bestätigung per `PostfachBestaetigung` | SSb-B 4.1; SSb-D 6.2.2, 6.3, 6.4.1 (S. 13–23) | 3 |
| Ü2 | TLS und CMS-verschlüsselte Datenteile bis zur Clearingstelle | EDS 2.6, 2.7.1 (S. 11–12); EHB 5.3.13 (S. 134) | 3 |
| Ü2 | Ablage in der BEST-DB standardmäßig mit WIESEL-Poolschlüssel des Betreibers, optional mit Zertifikat der Empfänger:in; Betreiber sieht Inhalte beim Eingang; keine Ende-zu-Ende-Verschlüsselung [Indiz] | SSb-B 5.3.2 (S. 14); EHB 5.4.11 (S. 148) | 3, keine E2EE: 2 |
| Ü3 | Behörde über Organisationszertifikat und `send-Auth` authentifiziert; Siegel auf Bescheiden [Indiz] | EHB 9.15.6 (S. 427); Siegel: Fehlanzeige | 3, Siegel: 2 |
| Ü3 | Rückkanal mit Signatur über das ELSTER-Zertifikat (`SigUser`) | EDS 2.4.1 (S. 10) | 3 |
| Ü4 | Statusabfrage für Sender und Empfänger: Bereitstellung, E-Mail-Versand, Abruf, Löschung | SSb-D 6.6 (S. 25–26) | 3 |
| Ü4 | Transferticket und PDF-Protokoll bei Abgabe; Info-E-Mail bei `EPAntwort`; keine Lesebestätigung [Indiz] | EHB 6.2.1 Tabelle 6-2 (S. 155), 6.2.5 (S. 159); SSb-B 9.7 (S. 39); Lesebestätigung: Fehlanzeige | 3, Lesebestätigung: 2 |
| Ü5 | keine Interoperabilität [Indiz]; Sonderweg `OZGPPNachricht` an das OZG-PLUS-Postfach | EHB 9.15.6 (S. 427) | 3, keine Interoperabilität: 2 |
| Ü6 | Eingänge zentral geprüft, aktive Inhalte entfernt; Behörde setzt `Virengeprueft=true` bei `EPBescheid` und `EPMitteilung`; bei DIVA Prüfung und Bereinigung untersagt | EHB 5.4.11, Tabelle 5-12 (S. 148–149); SSb-B 7.26 (S. 34) | 3 |
| Ü6 | Anhänge an das Finanzamt nur PDF; für Anhänge der Behörden keine Formatliste [Indiz] | XSD-BN; EHB 9.15.6 (S. 427); Formatliste: Fehlanzeige | 3, Formatliste: 2 |
| I1 | Freistaat Bayern, vertreten durch das BayLfSt, als bundesweiter Koordinator für Bund und Länder | LIZ § 1; ERFOLG | 3 |
| I1 | Steuerung über KONSENS: Steuerungsgruppe IT aus Bund und fünf Ländern, Beschlüsse binden alle Länder; Bayern als Auftrag nehmendes Land für ELSTER [Indiz] (Zuordnung aus Verwaltungsquellen, nicht aus dem Gesetz) | KONSENS-G § 9 Abs. 1, 4, § 11, § 24; PM-161; NRW-2015 | 3, Bayern als Auftrag nehmendes Land: 2 |
| I2 | zentral beim Betreiber in BEST-DB und Objektspeicher OTTER; Rechenzentren des BayLfSt [Indiz] (ELSTER dort nicht genannt) | SSb-B 4.1; EHB 9.3; RZ | 3, Rechenzentren: 2 |
| I3 | Behörden binden Fachverfahren an die zentrale Schnittstelle an, kein eigenes Postfach-Backend | SSb-B 4.1; EHB 9.15.4 | 3 |
| I4 | Adressierung an Finanzamt und Bearbeiter per Steuernummer; strukturierte Daten für vorgangsgesteuerte Weiterverarbeitung; interne Verteilung in den Finanzämtern [Indiz] | FA-CB; NDS-909; KONSENS-G § 12; interne Verteilung: Fehlanzeige | 3, interne Verteilung: 2 |
| I4 | Unternehmen: mehrere Organisationszertifikate mit je eigenem Postfach, Funktionspostfächer; keine Ende-zu-Ende-Verschlüsselung [Indiz] | ZERT-LF; SSb-B 5.3.2; EHB 5.4.11 | 3, keine E2EE: 2 |
| I5 | ein Betreiber; Behörden als Sender nur speziell berechtigt; Drittsoftware nur mit Lizenzvertrag und Hersteller-ID je Produkt, Sperrung bei Vertragsverstoß | LIZ § 1, § 9; EHB 6.5.3, 9.15.4 | 3 |
| R1 | Behörde an Person: elektronischer Verwaltungsakt ersetzt die Schriftform bei Bereitstellung zum Abruf | AO-87a Abs. 4, 8; AO-122a Abs. 1 | 3 |
| R1 | Person an Finanzamt: elektronisches Formular mit sicherem Identitätsnachweis ersetzt die Schriftform, ELSTER nicht namentlich genannt; Einspruch elektronisch zulässig | AO-87a Abs. 3, 6; AEAO Nr. 3.1, 3.2.1; AO-357 Abs. 1 | 3 |
| R2 | bis 2026 Einwilligung je Steuernummer, Einwilligungscode per Brief | EHB 9.16.2, 9.16.3 (S. 430–431) | 3 |
| R2 | ab 2027 elektronische Bekanntgabe als Regelfall, Papier auf Antrag; technisch `DauerPostBeStAntrag` | AO-122a Abs. 1 Satz 2, Abs. 2; EGAO-28 Abs. 2; PWC; EBS; EHB 9.10.1 (S. 341) | 3 |
| R3 | Bekanntgabe durch Bereitstellung nach § 122a AO, gilt am vierten Tag nach Bereitstellung als bekannt gegeben, für Bereitstellungen ab 2025, vorher am dritten | AO-122a Abs. 4; EGAO-1 Abs. 15; BUZER; EHB 7.5 (veraltet: dritter Tag) | 3 |
| R3 | keine förmliche Zustellung über ELSTER [Indiz] | Fehlanzeige | 2 |
| R4 | Finanzämter: Länder zum einheitlichen Einsatz der KONSENS-Software verpflichtet; zentrale Entgegennahme dem zuständigen Finanzamt zugerechnet; Länder sehen statt De-Mail nur ELSTER als Eingangskanal vor | KONSENS-G § 12; FVG § 20 Abs. 3; NDS-909 | 3 |
| R4 | andere Behörden freiwillig nach Berechtigung und Anbindung; Postfach 2.0 als Postfach nach § 2 Abs. 7 OZG; keine Pflicht und keine Anreize [Indiz]; Wirtschaftsverbände fordern Anbindungsverpflichtung [Indiz] | LIZ § 18; EHB 9.15.4; BAYVV Nr. 4.1.2; HB-SV; Pflicht, Anreize: Fehlanzeige | 3, keine Pflicht und keine Anreize, Verbandsforderung: 2 |

## Abweichungen bei der Prüfung

Abgleich der Belege mit `ELSTER/ELSTER-Zusammenfassung.md` und den CSV-Texten, 2026-11-17.

- **P3, I4 („mehrere Organisationszertifikate je Steuernummer“):** ZERT-LF sagt „je Unternehmen / Organisation“; das Zertifikat beruht auf der Steuernummer. Die CSV-Formulierung ist abgeleitet.
- **P4 („eigenes Postfach“ der Bevollmächtigten):** nur die eigene Kennung (`vnid`) und die eigene Einwilligung sind belegt.
- **I4 (Berechtigungssteuerung „noch nicht vollständig ausgerollt“, Zusammenfassung):** in ZERT-LF nicht gefunden.
- **I1 (§ 20 Abs. 3 FVG als Grundlage des Betriebs, Zusammenfassung):** ERFOLG nennt das Verwaltungsabkommen KONSENS 2007, nicht FVG oder KONSENS-G.
- **I5 (Sperre der Hersteller-ID):** Fundstelle ist LIZ § 9 („Rechte des LfSt bei Vertragsverletzung“), nicht ein Abschnitt „Haftung und Sperre“.
- **B2:** „Web-Interface und Command-line-Interface“ steht in LF-GEWST S. 4, nicht auf der ETR-Seite.
- **B4:** „nicht rechtsgültig“ steht in EHB Tabelle 9-9, nicht in EHB 7.4.
- **Ü4:** Das PDF-Protokoll ist in EHB 6.2.1 Tabelle 6-2 belegt, nicht in 6.2.5.
- **Ü6:** Laut EHB Tabelle 5-12 ist `Virengeprueft` auch bei `OZGPPNachricht` false; in CSV und Zusammenfassung nicht erwähnt.
- **R1 (AEAO Nr. 3.1, Verweis auf § 87a Abs. 3 Sätze 4 bis 8 AO):** Verweis beim Abruf nicht bestätigt, Wortlaut noch prüfen.
- **NDS-909:** laut PDF-Metadaten vom 17.05.2018, die Zusammenfassung nennt 07.05.2018.
- **Nicht geöffnet:** VDB-B, HB-SV, PM-161, NRW-2015, BUZER; die Aussagen stammen aus der Zusammenfassung. BAYVV am 2026-11-23 in der lokalen Kopie unter `sources/MUK/quellen/recht-governance/` geprüft (Nr. 4.1.2 bestätigt); die Kammer-Vollmachtsdatenbank ist auch in EHB 9.17.3 belegt.

## Evidenzstufen der markierten Stellen

Einordnung der früheren `[unverifiziert]`-, `[Web]`- und `?`-Stellen sowie der unmarkierten Aussagen, die nur auf Fehlanzeige, Rückschluss oder Sekundärquellen beruhen, Stand 2026-11-23. Stufe 1 und 2 werden über den genannten Prüfweg geprüft. `[Web]`-Stellen, die auf Primärquellen im Web beruhen, sind Stufe 3 und ohne Marker: `existing_solutions.csv` B2 (LF-GEWST, FAQ-GEWST, ETR), P2 (SECURE), I4 Adressierung (FA-CB, NDS-909) und Funktionspostfächer (ZERT-LF), R1 (AO-87a, AO-357), R2 (AO-122a, EGAO-28), R3 (AO-122a, EGAO-1), R4 erster Teil (KONSENS-G, FVG).

| Datei, Dimension | Aussage | Stufe | Begründung | Prüfweg |
|---|---|---|---|---|
| `existing_solutions.csv`, B2 | kein zentraler Versanddienst | 2 | Fehlanzeige in ERiC-Doku und ELSTER-Transfer-Unterlagen | Anfrage BayLfSt |
| `existing_solutions.csv`, P4 | eigenes Postfach der Bevollmächtigten | 2 | nur eigene Kennung (`vnid`) als Verschlüsselungs- und Abholmerkmal und eigene Einwilligung belegt (SSb-B 7.7, EHB 9.16.1.3); eigenes Postfach ist Rückschluss | Anfrage BayLfSt; Interview Steuerberatung |
| `existing_solutions.csv`, P6 | Einsatz in ERP-Software | 2 | ERiC-Doku und LIZ nennen allgemein Software von Herstellern, ERP nicht; MUK-FAQ nennt M2M-Abholung in Unternehmensinfrastrukturen | Anfrage BayLfSt; Interview Unternehmen / ERP-Hersteller |
| `existing_solutions.csv`, P9 | Mein ELSTER ist kein Nutzerkonto im Sinne des OZG | 2 | nur ZaPuK ADR-1110 (Sekundärquelle für ELSTER), Fußnote verweist auf § 122a AO, nicht auf das OZG | Abgleich mit § 2 OZG; Anfrage BayLfSt |
| `existing_solutions.csv`, Ü2 | keine Ende-zu-Ende-Verschlüsselung | 2 | Rückschluss: Ablage mit WIESEL-Poolschlüssel des Betreibers (SSb-B 5.3.2), Anhänge unverschlüsselt im Datensatz und auf den Servern der Finanzverwaltung geprüft (EHB 5.4.11) | Anfrage BayLfSt |
| `existing_solutions.csv`, Ü3 | kein Siegel auf Bescheiden dokumentiert | 2 | Fehlanzeige in ERiC-Doku und SSb-B | Bescheid-PDF prüfen; Anfrage BayLfSt |
| `existing_solutions.csv`, Ü4 | keine Lesebestätigung | 2 | Fehlanzeige; Statusabfrage kennt nur Abruf durch den Client | Anfrage BayLfSt |
| `existing_solutions.csv`, Ü5 | keine Interoperabilität | 2 | Fehlanzeige; nur Sonderweg `OZGPPNachricht` dokumentiert | Anfrage BayLfSt |
| `existing_solutions.csv`, Ü6 | für Anhänge der Behörden keine Formatliste dokumentiert | 2 | Fehlanzeige in EHB 9.15 und SSb-B | Anfrage BayLfSt |
| `existing_solutions.csv`, I1 | Bayern als Auftrag nehmendes Land für ELSTER (früher `[Web]`) | 2 | Zuordnung nur aus PM-161 (Pressemitteilung) und NRW-2015 (Broschüre 2015), beide nicht geöffnet; KONSENS-G nennt ELSTER nicht | Quellen öffnen; KONSENS-Beschlüsse prüfen |
| `existing_solutions.csv`, I2 | Rechenzentren des Bayerischen Landesamts für Steuern (früher `[Web]`) | 2 | RZ nennt ELSTER nicht; Rückschluss aus dem Betreiber | Anfrage BayLfSt |
| `existing_solutions.csv`, I4 | Bearbeitung in den KONSENS-Fachverfahren der Finanzämter (früher `?`) | 2 | Rückschluss aus KONSENS-G § 12 und NDS-909 (strukturierte Daten für vorgangsgesteuerte Weiterverarbeitung); interne Verteilung Fehlanzeige | Interview Finanzverwaltung |
| `existing_solutions.csv`, I4 | keine Ende-zu-Ende-Verschlüsselung | 2 | wie Ü2 | wie Ü2 |
| `existing_solutions.csv`, R3 | keine förmliche Zustellung dokumentiert | 2 | Fehlanzeige; Bekanntgabe nach § 122a AO belegt | Abgleich mit AO §§ 122, 122a; Anfrage BayLfSt |
| `existing_solutions.csv`, R4 | keine Teilnahmepflicht und keine Anreize gefunden (früher `[Web]`) | 2 | Fehlanzeige; BAYVV Nr. 4.1.2 (lokal geprüft) beschreibt ein Angebot ohne Pflicht | Anfrage StMD / BayLfSt |
| `existing_solutions.csv`, R4 | Wirtschaftsverbände fordern eine Anbindungsverpflichtung (früher `[Web]`) | 2 | nur aus HB-SV laut Zusammenfassung, Quelle nicht geöffnet | HB-SV öffnen |
| `morph_box.csv`, P4 Ausprägung 5 | Bevollmächtigte mit eigener Kennung und eigenem Postfach | 2 | wie P4 oben | wie P4 oben |
| `morph_box.csv`, Ü5 Ausprägung 1 | keine Interoperabilität | 2 | wie Ü5 oben | wie Ü5 oben |
| `morph_box.csv`, R3 Ausprägung 6 | keine förmliche Zustellung dokumentiert | 2 | wie R3 oben | wie R3 oben |
| `morph_box.csv`, R4 Ausprägung 1 | freiwillige Teilnahme ohne Anreize für Behörden außerhalb der Finanzverwaltung | 3 / 2 | Freiwilligkeit belegt (LIZ, EHB 9.15.4, BAYVV); fehlende Anreize Fehlanzeige | wie R4 oben |
