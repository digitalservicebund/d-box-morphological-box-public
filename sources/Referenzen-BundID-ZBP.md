# BundID / Zentrales Bürgerpostfach (Deutschland): Quellen

Nachweise für die BundID-Einträge in `morph_box.csv` und die Spalte `BundID heute (Deutschland)` in `existing_solutions.csv`. Statt `[unverifiziert]` stehen hier und in den BundID-Einträgen der CSV-Dateien `[Indiz]` (Stufe 2) und `[Annahme]` (Stufe 1); unmarkierte Aussagen sind Stufe 3. `Fehlanzeige` heißt: in den Quellen gesucht, nicht gefunden.

Evidenzstufen (Spalte `Stufe`):

- **3 (Belegt / Primärquelle):** Gesetzestext, Drucksache, Beschluss oder Betreiberdokument, in der lokalen Kopie im Volltext geprüft.
- **2 (Sekundär / Indiz):** Herstelleraussage, Länderpräsentation, Pressemitteilung, veraltete Angabe oder Rückschluss aus einer Primärquelle; `Fehlanzeige` bei gezielter Suche zählt als Indiz für eine negative Aussage.
- **1 (Annahme / Hypothese):** kein Beleg, nicht einmal ein Indiz; offen und zu prüfen.

Bei gemischten Zeilen steht zuerst die Stufe des belegten Teils, danach die Stufe des Teils, der in der Aussage mit `[Indiz]` oder `[Annahme]` markiert ist. Ausführliche Einordnung: `BundID-ZBP/ZBP-Zusammenfassung.md`.

Stand: 2026-11-17. Die Belege wurden gegen die lokalen Kopien in `BundID-ZBP/quellen/` und gegen die aktuelle Fassung von OZG und VwVfG auf gesetze-im-internet.de geprüft. Die Fundstellen geben Abschnitt, Seite, Folie oder Zeile der lokalen Kopie an.

## Hauptquellen

Lokale Pfade relativ zu `sources/BundID-ZBP/quellen/`.

| Kürzel | Quelle | Stufe |
|---|---|---|
| DSE | BMDS, Datenschutzerklärung der BundID, Stand 23.09.2026: `bundid-betreiber/bundid_datenschutzerklaerung.md`, <https://id.bund.de/de/dataSecurity> | 3 |
| FAQ | BMDS, Hilfreiche Informationen (FAQ) der BundID: `bundid-betreiber/bundid_faq_hilfreiche-informationen.md`, <https://id.bund.de/de/faq> | 3 |
| UI | BMDS, UI-Texte der Postfach-Oberfläche: `bundid-betreiber/bundid_ui-texte_postfach.md`, aus <https://id.bund.de/cms/assets/de.u87bk8e7.js> | 3 |
| TXT | BMDS, vollständiges Textbundle der BundID-Weboberfläche: `bundid-betreiber/bundid_web-textbundle_de_vollstaendig.md` | 3 |
| IMP | BMDS, Impressum der BundID: `bundid-betreiber/bundid_impressum.md`, <https://id.bund.de/> | 3 |
| PAP | Personalausweisportal, Anbindung an die BundID: `bundid-betreiber/personalausweisportal_anbindung-an-die-bundid.md`, <https://www.personalausweisportal.de/Webs/PA/DE/verwaltung/anbindung_an_die_bundID/anbindung_an_die_bundID-node.html> | 3 |
| PAP-B | Personalausweisportal, Die BundID: `bundid-betreiber/personalausweisportal_die-bundid.md`, <https://www.personalausweisportal.de/Webs/PA/DE/buergerinnen-und-buerger/die_bund-id/die_bund_id-node.html> | 3 |
| MIG | BMI, Migration der BundID-Postfächer in das ZBP, 28.03.2024: `bundid-betreiber/digitale-verwaltung_2024-03-28_migration-bundid-postfaecher-zbp.md`, <https://www.digitale-verwaltung.de/SharedDocs/kurzmeldungen/Webs/DV/DE/2024/03_migration_bund_id.html> | 2 (Kurzmeldung des Ministeriums) |
| BMI22 | BMI, BundID im vierten Jahr, Oktober 2022: `bundid-betreiber/BMI_BundID-im-vierten-Jahr_OZG-Erfahrungsaustausch_2022-10.pdf`, <https://www.fitko.de/fileadmin/fitko/veranstaltungen/ozg-erfahrungsaustausch-22/BundID_OZG-Erfahrungsaustausch.pdf> | 2 (Betreiberpräsentation, 2022) |
| SID | SID Sachsen, Der digitale Verwaltungsakt mit BundID, 16.09.2025: `bundid-betreiber/SID_Kundenforum-2025_BundID-Bafoeg21_Bescheiduebermittlung-ZBP_2025-09-16.pdf`, <https://www.sid.sachsen.de/download/SID_Kundenforum_2025_ORANGE_BundID_Bafog21.pdf> | 2 (Länderpräsentation); 3 für den eigenen Postfachdienst |
| MV | Land M-V, Digitale Bescheidzustellung nach § 9 OZG, 10.10.2025: `bundid-betreiber/MV_Digitale-Bescheidzustellung-nach-9-OZG_FITKO-stellt-vor_2025-10-10.pdf`, <https://www.fitko.de/fileadmin/fitko/veranstaltungen/die_fitko_stellt_vor/Bekanntgabe_nach__9OZG_in_MV_final.pdf> | 2 (Länderpräsentation); 3 für den eigenen Postfachdienst |
| FC-E | FITKO, FIT-Connect: Zentrales Bürgerpostfach (Entscheider): `fitko-fit-connect/fc-docs_zbp-entscheider.md`, <https://docs.fitko.de/fit-connect/docs/organisation-tasks/buergerpostfach-entscheider> | 3 |
| FC-T | FITKO, FIT-Connect und das ZBP (technische Anbindung): `fitko-fit-connect/fc-docs_zbp-technische-anbindung.md`, <https://docs.fitko.de/fit-connect/docs/zbp/> | 3 |
| FC-R | FITKO, SDK-Rezept ZBP anbinden: `fitko-fit-connect/fc-docs_sdk-rezept-zbp-anbinden.md`, <https://docs.fitko.de/fit-connect/docs/sdks/rezepte/zbp> | 3 |
| FC-RM | FITKO, FIT-Connect-Roadmap: `fitko-fit-connect/fc-docs_roadmap.md`, <https://docs.fitko.de/fit-connect/docs/roadmap> | 3 |
| SDK | FITKO, Java-SDK, Modul `zbp-client`: `fitko-fit-connect/java-sdk_zbp-client/`, <https://git.fitko.de/fit-connect/java-sdk/-/tree/main/zbp-client> | 3 |
| META | FITKO, Metadatenschema 2.1.0: `fitko-fit-connect/schema_fit-connect_metadata_2.1.0.schema.json`, <https://schema.fitko.de/fit-connect/metadata/2.1.0/metadata.schema.json> | 3 |
| PLR24 | IT-PLR, Beschluss 2024/28, Architekturüberblick Postfach- und Kommunikationslösungen: `fitko-fit-connect/itplr_beschluss-2024-28_postfach-kommunikationsloesungen_architekturueberblick.pdf`, <https://it-planungsrat.de/fileadmin/beschluesse/2024/Beschluss2024-28_Postfach-und_Kommunikationsl%C3%B6sungen_Architektur%C3%BCberblick.pdf> | 3 |
| WP22 | IT-PLR, Beschluss 2022/04, Whitepaper Zentrales Postfach, Februar 2022: `recht-governance/IT-PLR_Beschluss2022-04_Zentrales_Buergerpostfach_Whitepaper.pdf`, <https://www.it-planungsrat.de/fileadmin/beschluesse/2022/Beschluss2022-04_Zentrales_Buergerpostfach_Whitepaper.pdf> | 3 (Stand 2022; für heute 2, veraltet) |
| D8093 | BT-Drs. 20/8093, Gesetzentwurf OZG-Änderungsgesetz: `recht-governance/BT-Drs_20-8093_Gesetzentwurf_OZG-Aenderungsgesetz.pdf`, <https://dserver.bundestag.de/btd/20/080/2008093.pdf> | 3 |
| D9824 | BT-Drs. 20/9824, Antwort der Bundesregierung zur BundID, 15.12.2023: `recht-governance/BT-Drs_20-9824_Antwort_KA_eID-Serverkapazitaeten_BundID.pdf`, <https://dserver.bundestag.de/btd/20/098/2009824.pdf> | 3 |
| D10417 | BT-Drs. 20/10417, Beschlussempfehlung Innenausschuss OZGÄndG: `recht-governance/BT-Drs_20-10417_Beschlussempfehlung_Ausschuss_OZGAendG.pdf`, <https://dserver.bundestag.de/btd/20/104/2010417.pdf> | 3 |
| D5367 | BT-Drs. 21/5367, Antwort der Bundesregierung zu Statusmonitor und ZBP, 13.04.2026: `recht-governance/BT-Drs_21-5367_Antwort_KA_BundID-ZBP.pdf`, <https://dserver.bundestag.de/btd/21/053/2105367.pdf> | 3 |
| D9020 | BT-Drs. 21/9020, Antwort der Bundesregierung zum Deutschland-Stack, 05.11.2026: `recht-governance/BT-Drs_21-9020_Antwort_KA_Deutschland-Stack.pdf`, <https://dserver.bundestag.de/btd/21/090/2109020.pdf> | 3 |
| ORG | Organisationserlass des Bundeskanzlers, 06.05.2025: `recht-governance/Organisationserlass_Bundeskanzler_2025-05-06.md`, <https://www.gesetze-im-internet.de/bkorgerl_2025/BJNR0830A0025.html> | 3 |
| MPK | BK/MPK, Föderale Modernisierungsagenda, 04.12.2025: `recht-governance/Foederale_Modernisierungsagenda_MPK_2025-12-04.pdf`, <https://www.rlp.de/fileadmin/02/MPK_Vorsitz/Beschluesse_MPK_4._Dezember_2025_Berlin/Foederale_Modernisierungsagenda.pdf> | 3 |
| DP | Dataport, BundID-Anleitung (OSI-Servicekonto, Postfachreferenz), undatiert: `bundid-betreiber/Dataport_BundID-Anleitung_OSI-Servicekonto_Postfachreferenz_ozg-sachsen-anhalt.pdf`, <https://ozg.sachsen-anhalt.de/fileadmin/Bibliothek/Politik_und_Verwaltung/MF/OZG/Bilder/Aktuelles/BundID_Anleitung.pdf> | 3 (Anleitung des Dienstleisters zum eigenen Servicekonto, undatiert) |
| BR322B | BR-Drs. 322/26 (Beschluss), Stellungnahme des Bundesrates zum DIdG, 10.07.2026: `recht-governance/BR-Drs_322-26B_Stellungnahme_Bundesrat_DIdG.pdf`, <https://dserver.bundestag.de/brd/2026/0322-26B.pdf> | 3 |
| NEO | FITKO, Die FITKO stellt vor: Neo, 23.01.2026: `recht-governance/FITKO_Die-FITKO-stellt-vor_Neo_2026-01-23.pdf`, <https://www.fitko.de/fileadmin/fitko/veranstaltungen/die_fitko_stellt_vor/2026-01-23-Die-FITKO-stellt-vor-Neo_aktualisiert.pdf> | 2 (Präsentation) |
| OZG | Onlinezugangsgesetz, §§ 2, 3, 9, 9a, 12: `recht-governance/OZG_gesetze-im-internet_2026-11-17.md`, <https://www.gesetze-im-internet.de/ozg/> | 3 |
| VWVFG-41 | § 41 VwVfG: `recht-governance/VwVfG_41_gesetze-im-internet_2026-11-17.md`, <https://www.gesetze-im-internet.de/vwvfg/__41.html> | 3 |

## Nachweise je Dimension

Belege für die Spalte `BundID heute (Deutschland)` in `existing_solutions.csv`, Stand 2026-11-17. Z. = Zeile der lokalen Kopie, S. = Seite, F = Folie.

| Zeile | Aussage | Quellen | Stufe |
|---|---|---|---|
| B1 | direkt über die REST-API des ZBP (`PUT /v6/mailbox/messages`, `POST /v6/mailbox/applications/states`), nur aus NdB oder Verbindungsnetz; mTLS mit BPKI-Zertifikat, selbst signiertes JWT (RS512, 30 Minuten gültig) | FC-E Z. 13–14; FC-T Z. 26, 309–310; FC-R Z. 16–17, 232–233; SDK `ZBPApiService.java` Z. 22–23, `TokenGenerator.java` Z. 20, 34, 41 | 3 |
| B1 | außerhalb von NdB / VN über den ZBP-Adapter von FIT-Connect, signierter Umschlag mit `authorCertificate`, `authorToken`, `sha512sum`; SDKs für Java und .NET; Onboarding über das BundID-Self-Service-Portal mit Postfach-Zertifikat, Testphase, Technikerworkshop | FC-E Z. 29–33; FC-T Z. 141–146; PAP Z. 20, 27, 33 | 3 |
| B1 | teils über zentrale Postfachdienste der Länder, die an das ZBP weiterleiten | SID F24–F33; MV F6–F7; DP | 3 |
| B2 | kein Zugang für Behörden ohne Fachverfahren [Indiz] | Fehlanzeige; FC-E Z. 13–14 nennt nur die beiden technischen Wege; MV F4 zeigt eine landesweite Webanwendung zur Antragsbearbeitung (Testumgebung, Versand ins MUK) | 2 |
| B3 | Postkorb-Handle (`mailboxUuid`) aus der Anmeldung über die BundID, vom Onlinedienst an das Fachverfahren weitergereicht; für Bürger:innen in der Betroffenenauskunft (PDF, „Kontodaten“); Statusmonitor über `applicationId`; nur natürliche Personen | DSE 6.4 (Z. 37); SID F24–25; META `replyChannel.idBundDeMailbox`; FC-T Z. 130–133; FC-R Z. 122, 132, 186; OZG § 2 Abs. 5 | 3 |
| B3 | kein zentrales Verzeichnis; Adressierung ohne vorherigen Antrag mit BundID-Login nicht vorgesehen [Indiz] | Fehlanzeige | 2 |
| B4 | Betreff und Text, `content` in HTML, höchstens 1 MB, erlaubte HTML-Tags nur in der ZBP-Dokumentation im SSP; Dateitypenliste gilt direkt und über den Adapter; über den Adapter höchstens 30 MB gesamt und je Datei, bis 100 Anhänge; Download als PDF, Anhänge als ZIP | FC-T Z. 144–145, 194–195, 201–204; FC-R Z. 122, 135, 169, 186; UI Z. 96–97, 157 | 3 |
| P1 | E-Mail-Hinweis ohne Inhalt an die hinterlegte Adresse; Link wählt Anmeldeoptionen vor, Anmeldung mit Benutzername und Passwort bleibt möglich; Absender nur `@itzbund.de` und `@eid-support.de`; Benachrichtigung spätestens am Tag der Bereitstellung | FAQ Z. 189, 195, 201; TXT Z. 708; OZG § 9 Abs. 1 S. 6 | 3 |
| P2 | Benutzername / Passwort (Basisregistrierung, 2FA per E-Mail-Code, abschaltbar), ELSTER-Zertifikat (substanziell, Unternehmenszertifikate abgelehnt), Online-Ausweis, eAT, EU-Karte, Europäische ID; Smart-eID „noch nicht unterstützt“; Vertrauensniveau je Nachricht; EUDI-Wallet „d-you“ ab Januar 2027 angekündigt [Indiz] | PAP-B Z. 23, 26; FAQ Z. 36, 48, 183, 324–330, 342; TXT Z. 214, 451–452; DSE 6.4; d-you: PAP-B Z. 41 (Anbindung ohne Datum), D9020 S. 2 (d-you „zum 2. Januar 2027“); BundID ab Januar 2027: Rückschluss | 3, ab Januar 2027: 2 |
| P3 | kein Organisationszugang; Bürgerkonto und Organisationskonto getrennt | OZG § 2 Abs. 5; Fehlanzeige in DSE, FAQ, UI | 3 |
| P4 | keine Vertretung oder Vollmacht im Postfach [Indiz]; § 9 Abs. 1 OZG nennt Bevollmächtigte beim Abruf; 2022 Vertreterregelung „vorkonzeptioniert“ | OZG § 9 Abs. 1; WP22 S. 26; Vertretungsfunktion: Fehlanzeige in DSE, FAQ, UI, TXT | 3, keine Vertretung: 2 |
| P5 | nur Web-Oberfläche von id.bund.de; keine eigene App [Indiz]; AusweisApp nur zum Auslesen des Online-Ausweises; Download und Druck; 11 Länder umgestiegen (inkl. Sachsen; D5367 nannte im April 2026 10 und Sachsen bis Ende Juli 2026) | FAQ Z. 93, 132; UI Z. 96–97; PAP-B Z. 37; D5367 S. 5; BundID-App: Fehlanzeige | 3, keine App: 2 |
| P6 | keine Schnittstelle für Bürger:innen [Indiz]; Schnittstellen nur für Behörden und Onlinedienste | FC-E Z. 9–14; Fehlanzeige | 3, keine Schnittstelle für Bürger:innen: 2 |
| P7 | Antwort nur auf Anfrage der Behörde, wenn sie es freischaltet; Status „Antwort erforderlich / Geantwortet / Antwort verpasst / Anfrage geschlossen“, Antwortfrist; Postfach „dient nur dem Empfang“, sonst Post, E-Mail, Telefon; kein Erstkontakt | FAQ Z. 66, 165, 207, 231, 243; UI Z. 47–60, 102–153; PAP-B Z. 20, 40; Erstkontakt: Fehlanzeige | 3 |
| P7 | Bundesregierung: Bidirektionalität ab Juli 2026; FIT-Connect-Roadmap: bidirektionale Kommunikation über den ZBP-Adapter „Noch offen“ | D5367 S. 5; FC-RM Z. 30–32 | 3 |
| P8 | Freitext mit Zeichenbegrenzung und Dateien als Anhang, nur als Antwort; ob Upload möglich ist, entscheidet die Behörde; Format, Größe und Anzahl werden angezeigt, Festlegung durch die Behörde [Indiz]; Uploads nach der Übermittlung nicht einsehbar | UI Z. 114–131, 265, 284; FAQ Z. 213, 219, 237; DSE Z. 52 | 3, Festlegung durch die Behörde: 2 |
| P9 | Postfach Bestandteil des Nutzerkontos; Gast-Anmeldung oder Landeskonto ohne Postfach, dann Post; kein Archiv; Konto nach 24 Monaten Inaktivität gelöscht, Hinweis 6 und 2 Monate vorher | OZG § 2 Abs. 7, § 8 Abs. 7; FAQ Z. 30, 60, 120, 171, 177, 270; DSE 10 (Z. 57); UI Z. 71 | 3 |
| Ü1 | für Bürger:innen asynchroner digitaler Brief; Direktweg synchron mit Nachrichten-ID, Adapter asynchron mit Event-Log; Statusmeldungen im Statusmonitor | FC-R Z. 21–26, 374; FC-T Z. 11–13, 114–119, 319–320; D5367 S. 2 | 3 |
| Ü2 | keine Ende-zu-Ende-Verschlüsselung; TLS zur Web-Oberfläche, mTLS für Behörden; SDKs verschlüsseln vor der Übertragung, Adapter entschlüsselt zur Weiterleitung [Indiz]; Inhalte beim Betreiber lesbar [Indiz]; Indiz: ZBP prüft `content` auf HTML und Uploads auf Viren; Support-Einsicht in Benutzerdaten nur im Einzelfall, protokolliert; 2022 „verschlüsselt gespeichert“, heute [Indiz]; Ende-zu-Ende-Verschlüsselung erst „nach Zurverfügungstellung entsprechender Haushaltsmittel“ | DSE 5.1 (Z. 32), 8.1 (Z. 47); FC-R Z. 25; FC-T Z. 198–199; Entschlüsselung am Adapter: Folgerung aus FC-R Z. 25 und `content` als JSON-String im ZBP-Nachrichtenschema (`fitko-fit-connect/schema_id.bund.de_message_v6_1.0.0_zbp-message.schema.json`), `BundID-ZBP/ZBP-Zusammenfassung.md`; HTML-Prüfung FC-T Z. 194–195; Virenprüfung FAQ Z. 223–225; PLR24 S. 9; WP22 S. 24 [Indiz]; D10417 S. 24 | 3, Entschlüsselung am Adapter: 2, lesbar beim Betreiber: 2, Speicherung heute: 2 |
| Ü3 | Absender per mTLS-Zertifikat und Signatur (SHA512withRSA) beim Betreiber authentifiziert; im ZBP erscheint der ursprüngliche Autor, nicht der Adapter; qualifiziertes Siegel nach § 9a Abs. 6 OZG möglich, Nutzung nicht belegt; 2022: Siegel und Signaturen im Postfach „Nein“; keine Signatur der Antworten der Person [Indiz]; Identifikation auf Vertrauensniveau hoch als Unterschriftsersatz nach § 3a VwVfG [Indiz] | FC-T Z. 138–146; FC-R Z. 214–218; SDK `ZBPEnvelopes.java` Z. 76, `TokenGenerator.java`; OZG § 9a Abs. 6; WP22 S. 30; DSE 6.4; FAQ Z. 183 | 3, Signatur der Antworten: 2, § 3a VwVfG: 2 |
| Ü4 | Direktweg: Nachrichten-ID; Adapter: `accept-submission` / `reject-submission` mit Gründen; Bestätigung der Einlagerung als Nachweis der Bereitstellung; optionales Feld `retrievalConfirmationAddress` ohne dokumentierte Semantik; Aktivitätsprotokoll der Person mit `NACHRICHT_GELESEN`, `NACHRICHT_LOESCHEN`, 90 Tage | FC-T Z. 41, 290–303, 309–311; FC-R Z. 155; MV F6; TXT Z. 1381, 1387–1391; DSE 10; FAQ Z. 237, 243, 381–393 | 3 |
| Ü4 | 2022: technische Quittung und Lesebestätigung an eine vom Absender angegebene Adresse; heutiger Abruf- oder Lesenachweis [Indiz] | WP22 S. 24–25 [Indiz]; FC-R Z. 155 (`retrievalConfirmationAddress`); heute: Fehlanzeige | 3 für 2022, heute: 2 |
| Ü5 | keine Interoperabilität mit Systemen außerhalb [Indiz]; FIT-Connect-Adapter und Postfachdienste der Länder nur Zugang für Absender; EU-Identitäten nur für den Zugang; Länderkonten werden abgelöst statt verbunden | FC-E Z. 28–42; MV F5–F7; DP; DSE 6.3.4; MIG; PAP-B Z. 37; Austausch mit anderen Postfächern und EU-Systemen: Fehlanzeige | 3, keine Interoperabilität: 2 |
| Ü6 | Uploads der Bürger:innen automatisch auf Viren geprüft, Makros können zur Ablehnung führen; Dateitypen für Anhänge direkt und über den Adapter beschränkt; HTML auf erlaubte Tags geprüft; Prüfung der Behördennachrichten | FAQ Z. 223–225; TXT Z. 198–200 (`malwareDetected`); FC-T Z. 144–145, 201–205; Behördennachrichten: Fehlanzeige | 3 |
| I1 | BMDS (Referat DS II 7) verantwortlich; Betrieb ITZBund als Auftragsverarbeiter; Entwicklung AKDB; FITKO stellt mit FIT-Connect den ZBP-Adapter bereit; § 3 OZG nennt das BMI, Organisationserlass nimmt Identitätsmanagement von der Übertragung an das BMDS aus | DSE 1, 3 (Z. 17, 22); IMP; FC-E Z. 46–52; FC-T Z. 114; OZG § 3 Abs. 1; ORG XII Nr. 2 | 3 |
| I2 | zentral in Rechenzentren des ITZBund in Deutschland, auch anstelle eigener Postfächer der Länder; kein Archiv; Löschung mit dem Konto nach 24 Monaten Inaktivität; Uploads nach Übermittlung nicht gespeichert; Backups und Logs 90 Tage | DSE 3, 10 (Z. 22, 57); MIG; FC-E Z. 15–16, 30; FAQ Z. 171, 177, 237, 270; OZG § 8 Abs. 7 | 3 |
| I3 | Behörden nur als Absender ohne eigenes Postfach-Backend, direkt, über den Adapter oder über Postfachdienste der Länder; Organisationen als Empfänger nicht vorgesehen | OZG § 2 Abs. 5, § 3 Abs. 3; FC-E; SID F24–F33; MV F6–F7; DP | 3 |
| I4 | keine Funktionspostfächer; Antworten an Behörde, Verwaltungsportal oder Onlinedienst; interne Verteilung nicht dokumentiert | DSE 9 (Z. 52); SID F26–F28; PLR24 S. 9; Fehlanzeige | 3 |
| I5 | ein Betreiber; Behörden über das BundID-SSP: Admin mit ELSTER-Organisationszertifikat, Anbindungsvorhaben, technische Zertifikate, Anbindung nur an das ZBP möglich; IT-Dienstleister können nicht selbst anbinden; Adapter: FIT-Connect-Client-ID mit Begründung für fehlenden NdB-Anschluss, Destination-ID vom BMDS-Anbindungsmanagement | SID F35–F45; PAP; FC-E Z. 54–59; FC-T Z. 120–125; Nutzungsbedingungen: Fehlanzeige | 3 |
| R1 | Behörde an Person: Dokument im Postfach ersetzt die Schriftform mit qualifiziertem elektronischem Siegel der Behörde; Person an Behörde: Erklärung über Online-Formular eines Verwaltungsportals nach Identifizierung über ein Nutzerkonto, unabhängig vom ZBP | OZG § 9a Abs. 5, 6 (Z. 660–669) | 3 |
| R2 | Bürgerkonto freiwillig; Einwilligung in die Bekanntgabe über das Postfach gilt als erteilt, Ausschluss im Rahmen der Inanspruchnahme einer Leistung; Länderkonten noch 3 Jahre ab einer Bekanntmachung im Bundesgesetzblatt zulässig, Bekanntmachung im Juli 2026 noch nicht erfolgt; 11 Länder umgestiegen | OZG § 3 Abs. 1 S. 2, § 9 Abs. 1 S. 2, § 12 Abs. 1; BR322B S. 5; PAP-B Z. 37; D5367 S. 5 | 3 |
| R3 | Bekanntgabe durch Abruf aus dem Postfach, gilt am vierten Tag nach Bereitstellung als bekannt gegeben; Behörde weist im Zweifel Bereitstellung nach; Benachrichtigung spätestens am Tag der Bereitstellung; Abruf nur nach Authentifizierung | OZG § 9 Abs. 1 (Z. 614–632) | 3 |
| R3 | Bekanntgabe durch Abruf nach § 41 Abs. 2a VwVfG über das ZBP nicht dokumentiert (M-V: „Erste Ideen“); keine förmliche Zustellung über das ZBP [Indiz]; Harmonisierung von Bekanntgabe und Zustellung bis 30.09.2026 angekündigt, kein Ergebnis gefunden | VWVFG-41 Abs. 2a; MV F8; MPK S. 35; förmliche Zustellung: Fehlanzeige | 3, keine förmliche Zustellung: 2 |
| R4 | freiwillig, § 9 OZG als Kann-Regelung; keine Meldepflicht beim Statusmonitor; Identifizierung im Portalverbund über das zentrale Bürgerkonto nach Ende der Übergangsfrist; Kosten der BundID trägt derzeit der Bund (2023); Anbindung 2022 und 2023 per Kooperationsvereinbarung mit dem BMI, heute über das SSP | OZG § 3 Abs. 1, § 9, § 12; FC-E Z. 11; D5367 S. 2–3; D9824 S. 6; BMI22 S. 8; SID F35 | 3 |

## Abweichungen bei der Prüfung

Abgleich der Belege mit `BundID-ZBP/ZBP-Zusammenfassung.md` und `morph_box.csv`, 2026-11-17.

- **P7, Abschnitt 6 (Bidirektionalität):** Die Antwortfunktion ist laut FAQ und UI-Texten im BundID-Postfach live. Offen ist nur der Weg über den FIT-Connect-Adapter (FC-RM). Wann die Funktion freigeschaltet wurde, ist nicht belegt.
- **R3 („bei früherem Abruf ab Abruf“):** § 9 Abs. 1 S. 7 OZG regelt nur den Abruf vor einer *erneuten* Bekanntgabe; ein früherer Abruf verlegt die Bekanntgabe nicht vor. Benachrichtigungspflicht und Authentifizierung beim Abruf fehlen in der Zusammenfassung.
- **R1:** Den Schriftformersatz von der Person an die Behörde über Online-Formulare (§ 9a Abs. 5 OZG) erwähnt die Zusammenfassung nicht.
- **I1, Abschnitt 6:** Der Organisationserlass „belässt“ nicht ausdrücklich das Identitätsmanagement beim BMI, er nimmt es nur von der Übertragung an das BMDS aus. Das BMI steht nur im Wortlaut von § 3 OZG.
- **I5:** Die SSP-Anbindung steht in SID auf Folien 35–40, nicht 23–40.
- **R4:** Kooperationsvereinbarungen sind nur für 2022 und 2023 belegt (BMI22, D9824), nicht für heute.
- **Ü4:** Den Eintrag „Abruf an Behörde (BundID)“ in `morph_box.csv` stützt nur die Lesebestätigung von 2022 (WP22). Deshalb nach Ausprägung 3 „Lesen an Behörde“ verschoben; das Aktivitätsprotokoll der Person steht in der neuen Ausprägung 7. MV nennt die „Bestätigung der Einlagerung“, nicht „Speicherbestätigung“.
- **Ü3 A5 („BundID hoch“, § 3a VwVfG):** Für eine Verbindung von BundID und § 3a VwVfG wurde kein Primärbeleg gefunden. Näher liegt § 9a Abs. 5 OZG.
- **P4:** „keine Vertretung“ beruht nur auf einer Fehlanzeige. § 9 Abs. 1 OZG nennt Bevollmächtigte beim Abruf, eine Vertretungsfunktion im ZBP ist aber nicht dokumentiert.
- **Nicht neu geprüft:** D8093, D10417, MPK und NEO; die Aussagen stammen aus der Zusammenfassung.

### Zweite Prüfung, 2026-11-17

Beim Befüllen der Spalte `BundID heute (Deutschland)` in `existing_solutions.csv` erneut gegen die Primärquellen geprüft.

- **P2:** Smart-eID ist kein Anmeldemittel. TXT Z. 451–452: „Die Smart-eID wird noch nicht unterstützt.“ TXT Z. 214 betrifft die Ablehnung von ELSTER-Unternehmenszertifikaten.
- **P5, R2:** PAP-B (abgerufen 2026-11-17) nennt 11 umgestiegene Länder einschließlich Sachsen, D5367 (April 2026) noch 10.
- **B3:** Das Postkorb-Handle steht in der Betroffenenauskunft (PDF, „Kontodaten“), nicht direkt unter „Zugänge & Daten“.
- **P1:** Der Link in der E-Mail wählt Anmeldeoptionen vor (FAQ Z. 195), er beschränkt die Anmeldung nicht.
- **P7:** SID F24–27 tragen nur Überschriften; die Status stammen aus FAQ Z. 243 und UI Z. 47–60.
- **B4, Ü6:** Die Dateitypenliste gilt „in beiden Fällen“, direkt und über den Adapter (FC-T Z. 204). `morph_box.csv` Ü6 entsprechend angepasst.
- **Ü2:** WP22 S. 29, Tabelle 2, führt 2022 für das NKB-Postfach „Ende-zu-Ende Verschlüsselung umgesetzt: Ja“; PLR24 S. 9 bewertet das ZBP 2024 mit „Keine E2E“. Für die Spalte gilt PLR24.
- **Ü3:** Kein Primärbeleg für BundID und § 3a VwVfG; `morph_box.csv` Ü3 Ausprägung 5 mit `[unverifiziert]` markiert, seit der dritten Prüfung `[Indiz]`. Ein gesiegelter Bescheid ist nur für das Unternehmenskonto belegt (MV F4). PAP spricht von Postfach-Zertifikaten, „die Ihren privaten (geheimen) Schlüssel beinhalten“; ob der Betreiber die Schlüssel erzeugt, ist offen.
- **R4:** D5367 S. 3 schließt nur eine Meldepflicht beim Statusmonitor aus, keine Nutzungspflicht für das ZBP. `morph_box.csv` R4 Ausprägung 1 stützt sich jetzt auf § 9 OZG als Kann-Regelung. D9824 S. 6: „Die Kosten der BundID trägt derzeit der Bund“, das kann als Anreiz gelten.
- **R2:** „Bekanntmachung nicht erfolgt“ belegt BR322B S. 5, nicht D5367.
- **I1:** FC-E sagt nicht, dass die FITKO den Adapter „im Auftrag des IT-Planungsrats“ betreibt (so die Zusammenfassung).
- **P9, I2:** Die Hinweise 6 und 2 Monate vor der Löschung stehen in FAQ Z. 270, nicht in DSE 10. Das AKDB-Informationsblatt nennt neben dem ITZBund auch ein AKDB-Rechenzentrum und knüpft das Konto an die Gültigkeit des Ausweises; es gilt auch für die BayernID und ist nicht verwendet.
- **Nicht verwendet:** Löschfristen von FIT-Connect (7 Tage nach Abholung, `fitko-fit-connect/fc-docs_loeschfristen.md`), weil die Geltung für den ZBP-Adapter nicht belegt ist.

### Dritte Prüfung, 2026-11-23

Bei der Einordnung der Evidenzstufen gegen die lokalen Kopien geprüft.

- **B2:** MV F4 zeigt eine für alle Behörden in M-V nutzbare Webanwendung zur Antragsbearbeitung (Testumgebung), vorgeführt mit Versand ins MUK. Ob darüber auch an das ZBP versendet wird, ist offen; „keine Web-Oberfläche für Behörden“ gilt damit nur für das ZBP selbst.
- **P4:** WP22 S. 26 nennt 2022 eine „vorkonzeptionierte“ Vertreterregelung für den Zugriff auf das Postfach eines anderen Nutzers; heute ist keine Vertretungsfunktion dokumentiert.
- **Ü3 A5:** § 3a VwVfG liegt nicht als lokale Kopie vor. § 9a Abs. 5 OZG ersetzt die Schriftform durch den Identitätsnachweis über ein Nutzerkonto nur bei Online-Formularen eines Verwaltungsportals und knüpft an § 3 Abs. 4 OZG an, nicht ausdrücklich an das Vertrauensniveau hoch.
- **P2 in `morph_box.csv`:** „EU-eID nur nach manueller Freischaltung [unverifiziert]“ betrifft Mein Justizpostfach (`Referenzen-MJP.md`), nicht die BundID; hier nicht eingeordnet.
- **P2 (EUDI-Wallet):** Die lokale Kopie von PAP-B nennt die Anbindung an „d-you“ ohne Datum (Z. 41); „ab Januar 2027“ steht dort nicht, anders als in der Zusammenfassung. D9020 S. 2 nennt den Start von d-you „zum 2. Januar 2027“; dass die BundID ab diesem Termin die Anmeldung mit d-you anbietet, ist ein Rückschluss. TXT enthält bereits Texte für die Anmeldung mit einer EUDI-Wallet (Z. 229–239, 422–427).
- **Ü2, R3:** D10417 S. 24 (Ende-zu-Ende-Verschlüsselung erst „nach Zurverfügungstellung entsprechender Haushaltsmittel“) und MPK S. 35 (Harmonisierung von Bekanntgabe und Zustellung „bis 30.09.2026“) jetzt im Volltext geprüft.
- **Ü5, R3, Ü3, R4:** Unmarkierte negative Aussagen und Zuordnungen, die nur auf Fehlanzeige, veralteten Angaben oder Rückschluss beruhen, mit `[Indiz]` nachmarkiert (siehe Tabelle unten). „Keine Funktionspostfächer“ (I4) und „Organisationen als Empfänger nicht vorgesehen“ (I3) bleiben Stufe 3, weil sie aus § 2 Abs. 5 OZG folgen (Bürgerkonto nur für natürliche Personen). Die „Bestätigung der Einlagerung“ (Ü4) ist als Aussage von M-V zum eigenen Postfachdienst gekennzeichnet und bleibt Stufe 3.

## Evidenzstufen der markierten Stellen

Einordnung der früheren `[unverifiziert]`-Stellen sowie der unmarkierten Aussagen, die nur auf Fehlanzeige, Rückschluss, veralteten oder sekundären Quellen beruhen, Stand 2026-11-23. Stufe 1 und 2 werden über den genannten Prüfweg geprüft.

| Datei, Zeile | Aussage | Stufe | Begründung | Prüfweg |
|---|---|---|---|---|
| `existing_solutions.csv`, B2 | keine Web-Oberfläche und kein Versanddienst für Behörden | 2 | Fehlanzeige; FC-E nennt nur Schnittstelle und Adapter; Webanwendung des Landes M-V (MV F4) nur mit Versand ins MUK gezeigt | Anfrage BMDS-Anbindungsmanagement; Interview Behörden |
| `existing_solutions.csv`, B3 | kein zentrales Verzeichnis, Adressierung ohne vorherigen Antrag mit BundID-Login nicht vorgesehen | 2 | Fehlanzeige; Postkorb-Handle nur aus der Anmeldung oder der Betroffenenauskunft dokumentiert | Anfrage BMDS; ZBP-Dokumentation im SSP |
| `existing_solutions.csv`, P4 | keine Vertretungs- oder Vollmachtsfunktion im Postfach | 2 | Fehlanzeige in DSE, FAQ, UI, TXT; 2022 Vertreterregelung nur „vorkonzeptioniert“ (WP22) | Testkonto; Anfrage BMDS |
| `existing_solutions.csv`, P5 | keine eigene App | 2 | Fehlanzeige; FAQ nennt nur die AusweisApp zum Auslesen des Online-Ausweises | App-Stores prüfen |
| `existing_solutions.csv`, P6 | keine Schnittstelle für Bürger:innen, Zugriff nur über die Web-Oberfläche | 2 | Fehlanzeige; dokumentierte Schnittstellen nur für Behörden und Onlinedienste (FC-E) | Anfrage BMDS |
| `existing_solutions.csv`, P8 | Dateiformate, Größe und Anzahl beim Upload legt die Behörde fest | 2 | UI zeigt Platzhalter für Formate, Größen und Anzahl, FAQ: Upload „hängt von der jeweiligen Verwaltung ab“; wer die Werte festlegt, steht nicht in den Quellen | ZBP-Dokumentation im SSP; Testkonto |
| `existing_solutions.csv`, Ü2 | Adapter entschlüsselt zur Weiterleitung | 2 | Rückschluss: Submission an den Adapter verschlüsselt (FC-R Z. 25), ZBP erwartet `content` als JSON-String mit Signatur | Anfrage FITKO |
| `existing_solutions.csv`, Ü2 | Inhalte beim Betreiber lesbar | 2 | keine E2EE (PLR24); ZBP prüft `content` auf HTML (FC-T) und Uploads auf Viren (FAQ), das setzt Klartext voraus | Anfrage BMDS / ITZBund |
| `existing_solutions.csv`, Ü2 | verschlüsselte Speicherung heute | 2 | veraltete Angabe: nur WP22 (2022); DSE schweigt | Anfrage BMDS / ITZBund |
| `existing_solutions.csv`, Ü3 | keine Signatur der Antworten der Person | 2 | Fehlanzeige in DSE, FAQ, UI | Anfrage BMDS; Testkonto |
| `existing_solutions.csv`, Ü4 | Lesebestätigung an die Behörde heute | 2 | veraltete Angabe: WP22 (2022); heute nur optionales Feld `retrievalConfirmationAddress` ohne dokumentierte Semantik | ZBP-Dokumentation im SSP; Anfrage BMDS |
| `morph_box.csv`, P4 Ausprägung 1 | keine Vertretung, BundID heute | 2 | wie P4 oben | wie P4 oben |
| `morph_box.csv`, P6 Ausprägung 1 | keine Anbindung eigener Systeme, BundID heute | 2 | wie P6 oben | wie P6 oben |
| `morph_box.csv`, Ü3 Ausprägung 5 | BundID hoch, Identifikation ersetzt die Unterschrift nach § 3a VwVfG | 2 | kein Primärbeleg für BundID und § 3a VwVfG; nur Rückschluss aus § 9a Abs. 5 OZG, der für Online-Formulare gilt, nicht für Antworten im ZBP | § 3a Abs. 2 VwVfG im Volltext prüfen; Anfrage BMDS |
| `morph_box.csv`, Ü4 Ausprägung 3 | BundID: Lesebestätigung 2022 dokumentiert, heute | 3 / 2 | Lesebestätigung 2022 belegt (WP22 S. 24–25); heute wie Ü4 oben | wie Ü4 oben |
| `existing_solutions.csv`, P2 | EUDI-Wallet d-you für die BundID ab Januar 2027 | 2 | Rückschluss: Anbindung an d-you angekündigt ohne Datum (PAP-B Z. 41), Start von d-you zum 2. Januar 2027 (D9020 S. 2); Datum für die BundID in der lokalen PAP-B-Kopie nicht gefunden | aktuelle Fassung von PAP-B prüfen; Anfrage BMDS |
| `existing_solutions.csv`, Ü5 | keine Interoperabilität mit Systemen außerhalb | 2 | Fehlanzeige; Adapter und Postfachdienste der Länder nur als Zugang für Absender dokumentiert (FC-E, MV F6–F7) | Anfrage BMDS / FITKO |
| `existing_solutions.csv`, R3 | keine förmliche Zustellung über das ZBP | 2 | Fehlanzeige; Bekanntgabe nach § 9 Abs. 1 OZG belegt, Zustellung nach VwZG über das ZBP in keiner Quelle | Abgleich mit VwZG; Anfrage BMDS |
| `morph_box.csv`, P2 Ausprägung 3 | BundID mit EUDI-Wallet ab Januar 2027 angekündigt | 2 | wie P2 oben | wie P2 oben |
| `morph_box.csv`, Ü3 Ausprägung 1 | BundID heute ohne Siegel oder Signatur, Echtheit nur aus dem Transportweg | 3 / 2 | Authentifizierung der Behörde per mTLS und Signatur beim Betreiber belegt (FC-T, FC-R); qualifiziertes Siegel nach § 9a Abs. 6 OZG möglich, Nutzung heute Fehlanzeige, „Nein“ nur für 2022 (WP22 S. 30, veraltet) | Bescheid im Postfach prüfen; Anfrage BMDS |
| `morph_box.csv`, Ü5 Ausprägung 1 | keine Interoperabilität, BundID | 2 | wie Ü5 oben | wie Ü5 oben |
| `morph_box.csv`, R3 Ausprägung 6 | keine förmliche Zustellung, BundID | 2 | wie R3 oben | wie R3 oben |
| `morph_box.csv`, R4 Ausprägung 1 | freiwillige Teilnahme ohne Anreize, BundID | 3 / 2 | Freiwilligkeit belegt (§ 9 OZG als Kann-Regelung); fehlende Anreize Fehlanzeige, „Die Kosten der BundID trägt derzeit der Bund“ (D9824 S. 6, 2023) kann als Anreiz gelten | Anfrage BMDS zu Kosten und Anreizen der Anbindung |
