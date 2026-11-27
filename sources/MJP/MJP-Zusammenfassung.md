# MJP (Mein Justizpostfach): Zusammenfassung für die d-box

Wie Mein Justizpostfach (MJP), das kostenlose Postfach für Bürger:innen im elektronischen Rechtsverkehr (ERV) mit der Justiz, aufgesetzt ist: Betrieb, Technik, Recht. Eingeordnet nach den Zeilen von `morph_box.csv`. Die Zusammenfassung verweist auf die Quellen, statt sie zu wiederholen. Für Details den Pointer öffnen.

Stand: 2026-11-18. Grundlage ist eine Webrecherche, ausgehend von <https://mjp.justiz.de/>, <https://mjpserviceportal.de/>, <https://service.justiz.de/mein-Justizpostfach-einrichten> und <https://justiz.de/ervvoe/textordner_fuer_buerger/index.php>. Alle Primärquellen liegen in `quellen/` in drei Unterordnern. Jeder Unterordner hat eine `_index.md` mit URL, Herausgeber, Datum und Inhaltsnotiz und eine `_befunde.md` mit den Einzelbefunden je Zeile. `egvp.justiz.de` war nicht direkt erreichbar, die Seiten stammen aus dem Internet Archive (Snapshot-URL im Dateikopf).

## Kennzeichnung

- Ohne Kennzeichnung: in einer Primärquelle belegt.
- **[Folgerung]**: aus mehreren Quellen geschlossen, nicht ausdrücklich belegt.
- **[unverifiziert]**: nur Sekundärquelle oder Annahme.
- **[veraltet?]**: Quelle ist älter, aktueller Stand nicht belegt.

## Kürzel für Quellen

Pfade relativ zu `quellen/`: `MB/` = `mjp-betreiber/`, `AT/` = `anbindung-technik/`, `RG/` = `recht-governance/`. Z. = Zeile der Markdown- oder JSON-Kopie, S. = PDF-Seite.

| Kürzel | Datei | Inhalt |
|---|---|---|
| TB | `MB/mjp_webclient-ui-textbundle-de.md` (gleicher Inhalt: `AT/mjp-webclient_i18n-messages-de_v2.7.5_2026-11-18.json`) | Oberflächentexte des Web-Clients 2.7.5: Registrierung, Schlüsseldatei, Versandformular, Status, Löschen, BundID-Fehlermeldungen |
| WEB | `AT/mjp-webclient_analyse-app-js_v2.7.5_2026-11-18.md` | Auszüge aus `app.js` (Governikus) mit Byte-Offset: Schlüsselerzeugung, Ver- und Entschlüsselung, API-Pfade |
| JS | `MB/governikus_mjp_webclient-appjs-auszuege.md` | Weitere `app.js`-Auszüge: Krypto-Parameter, Copyright Governikus |
| CFG | `MB/mjp_webclient-config.md`, `AT/mjp-webclient_config_v2.7.5_2026-11-18.json` | Client-Konfiguration: API-URL, Limits 1000 Dateien und 200 MB, Links zum Serviceportal |
| DSE / IMP | `MB/blk_mjp_datenschutzerklaerung.md`, `MB/blk_mjp_impressum.md` (auch `RG/MJP_Impressum-Datenschutz-Verantwortliche_2026-11-18.md`) | Anbieter BLK-AG IT-Standards, Verantwortliche, SAFE-Eintrag, Speicherfristen |
| FAQ | `MB/mjpserviceportal_faq.md` | FAQ des Serviceportals, Stand 10/2026 |
| LÖ / ERN / EEB | `MB/blk-mjp-support_mjp_information-loeschen-von-nachrichten_2025-05.pdf`, `…_anleitung-schluesseldatei-erneuern_2026-10.pdf`, `…_eeb-erstellen-versenden_2026-01.pdf` | Support-Anleitungen: Löschung nach 90 Tagen, Schlüsselerneuerung, eEB |
| HR | `AT/blkag_mein-justizpostfach-information_v1.1_2023-10-25.pdf` (identisch: `RG/BLK-AG-IT-Standards_Handreichung-MJP_V1.1_2023-10-25.pdf`) | Handreichung der BLK-AG zum Start: Architekturbild, SAFE-Rolle `ozg_postfach`, Sichtbarkeit, Prüfvermerk **[veraltet?]** |
| ERL | `MB/jm-nrw_erlass-start-mjp-und-ebo-client_2023-11-03.pdf` (identisch: `RG/JM-NRW_Erlass_Start-MJP_2023-11-03.pdf`) | Erlass JM NRW zum Start von MJP und eBO-Client |
| TP | `MB/bmi-dv-i-5_mjp-two-pager-behoerden-laender_2023-08.pdf` (identisch: `AT/bmi_broschuere-mjp-two-pager_2023-08.pdf`, `RG/BMI-DVI5_MJP-Two-Pager_2023-08.pdf`) | BMI-Infoblatt 2023, Integration in die BundID „2024 tbd“ **[veraltet?]** |
| PM | `MB/bmj_pm-62-2023_startschuss-mjp_2023-10-13.md` (auch `RG/BMJ_PM-62-2023_Start-MJP_2023-10-13.md`) | Pressemitteilung BMJ/BMI zum Start |
| ANF | `AT/blk_anforderungen-drittanwendungen-osci-erv_v1.8_2022-04-20.pdf` | Anforderungen A1–A25 an Drittanwendungen im EGVP/OSCI-ERV |
| LF | `AT/justiz_erv-leitfaden_2025-07.pdf` | ERV-Leitfaden 2025: Wege, Sichtbarkeiten, Eingang, Prüfvermerk, eEB |
| ERVB | `AT/bmj_ervb-2025_2025-07-16.pdf`, `RG/BAnz_2025-07-29_ERVB-2025.pdf` | Elektronischer-Rechtsverkehr-Bekanntmachung 2025: Formate, Grenzen |
| SAFE-STD | `AT/egvp_standard-safe-verzeichnisdienst-suche_v1.5_2026-05-21.pdf` | SAFE: föderierte Verzeichnisdienste |
| DRT | `AT/egvp_drittprodukt-registrierung-governikus-mjp_2023-10.pdf` | Registrierung MJP als EGVP-Drittprodukt (Governikus i. A. BMI) |
| WS | `AT/westernacher_uebernahme-betrieb-mjp.md` | Westernacher: Betrieb und Support seit 30.07.2026 |
| EGVP-M | `AT/egvp_meldungen_2026-11.md` | EGVP-Störungsmeldungen, u. a. IT.NRW-Bürgerpostfächer |
| ZPO / ERVV | `RG/ZPO_gesetze-im-internet_2026-11-18.md`, `RG/ERVV_gesetze-im-internet_2026-11-18.md` | §§ 130a, 130b, 130d, 169, 173, 1124 ff. ZPO; §§ 10–13a ERVV |
| D19-28399 | `RG/BT-Drs_19-28399_Gesetzentwurf_Ausbau-ERV.pdf` | Begründung: Nutzerkonto als sicherer Übermittlungsweg, Freiwilligkeit, Zustellung |
| D20-10943 | `RG/BT-Drs_20-10943_Gesetzentwurf_weitere-Digitalisierung-Justiz.pdf` | ELSTER als Identifizierungsmittel, SAFE-Eintrag § 13a ERVV |
| D21-1509 | `RG/BT-Drs_21-1509_Gesetzentwurf_Online-Verfahren.pdf` | Online-Verfahren; „bundesweit erst mit MJP im Oktober 2023 realisiert“ |
| BGBl21 / BGBl25 / BGBl26 | `RG/BGBl_2021-I-4607_…pdf`, `RG/BGBl_2025-I-349_…pdf`, `RG/BGBl_2026-I-152_…pdf` | Ausbau ERV; Online-Verfahren (Nr. 5 → Nr. 4); Zwangsvollstreckung (Gerichtsvollzieher) |
| BLK-UB | `RG/BLK_Umlaufbeschluss_ERV-Betriebsmanagement_2019-07.pdf` | Kostenaufteilung ERV-Betrieb: Bund 1 %, Länder nach Königsteiner Schlüssel |
| FDS-DA | `MB/fragdenstaat_mjp-deutschland-app_blk-jumbw_2026-09.md` | Antwort JuM BW 09/2026: keine Mobilplanung, Prüfung der FITKO-Zielarchitektur |
| NP23 / NP25 / HEISE | `MB/netzpolitik_mjp-datenleck_2023-11-14.md`, `MB/netzpolitik_mjp-privatsphaere_2025-06-13.md`, `AT/heise_massive-stoerungen-erv_2024-08.md` | Sekundärquellen: Datenleck SAFE 2023, Kritik an E2EE, Zuständigkeiten 2024 |
| ZAPUK-AK | `../zapuk-v1.0.0-121-g7419d8b/Zielarchitektur/Architekturkonzept/README.md` | MJP als Bestandslösung (Z. 391), Channel Adapter (Z. 395–403), EGVP-Integration (Z. 419) |

## 1. Systembild

- MJP ist ein kostenloses Browser-Postfach nur für natürliche Personen. Es dient der Kommunikation mit Gerichten, Staatsanwaltschaften, Gerichtsvollzieher:innen, Behörden mit beBPo, Anwält:innen (beA), Notar:innen (beN) und Steuerberater:innen (beSt) (FAQ Z. 43, 51; LF S. 9).
- Rechtlich ist MJP der „Postfach- und Versanddienst des Nutzerkontos Bund gemäß § 13 ERVV“ (HR S. 1). Damit ist es ein **sicherer Übermittlungsweg** nach § 130a Abs. 4 S. 1 Nr. 4 ZPO, bis 22.12.2025 Nr. 5 (ZPO Z. 49–53; BGBl25 S. 2). Es ist kein eBO.
- Technisch ist MJP ein **Teilnehmer der EGVP-Infrastruktur** (DSE Z. 8):
  - Transport über OSCI 1.2 mit Intermediären, die Nachrichten zwischenspeichern (ANF S. 10, 12).
  - Adressierung über den Verzeichnisdienst SAFE. MJP-Einträge tragen die Rolle `ozg_postfach` (HR S. 1).
  - Inhaltsdaten als XJustiz-Nachricht `xjustiz_nachricht.xml` plus Dokumente (ANF S. 6).
- Komponenten (HR S. 2): Browser-Komponente (Web-Client von Governikus, Version 2.7.5) → MJP-Serverkomponente → OSCI-Intermediär → Gericht oder Behörde. Identität und Benachrichtigung kommen von der **BundID**.
- Historie:
  - Rechtsgrundlage seit 1.1.2022 (BGBl21 S. 11).
  - Pilotstart am 12.10.2023 (HR S. 1; ERL S. 2), laut BMJ am 13.10.2023 (PM Z. 15). Anfangs unter `mein-justizpostfach.bund.de`, Serverkomponente bei BMI/ITZBund (HR S. 2).
  - Heute `mjp.justiz.de`. Seit 30.07.2026 betreibt Westernacher Solutions mit der Rockenstein AG Betrieb und Support (WS Z. 46–66).
  - „Die Anbindung von Nutzerkonten nach dem OZG ist bundesweit erst mit der Schaffung des ‚Mein Justizpostfach‘ (MJP) im Oktober 2023 realisiert worden“ (D21-1509 S. 28).

## 2. Betrieb, Governance, Finanzierung

- **Anbieter heute:** die AG IT-Standards in der Justiz der Bund-Länder-Kommission für Informationstechnik in der Justiz (BLK), „im Auftrag der Länder und des Bundes“, vertreten durch ihren Vorsitz, das JuM Baden-Württemberg (IMP).
- **Datenschutzrechtlich verantwortlich:** die Justizverwaltungen der 16 Länder und das BMJV (DSE Z. 14).
- **Entwicklung:** Governikus (JS Z. 11; DRT). Der Client teilt die Codebasis mit dem OZG-PLUS-Postfach, erkennbar an Feldnamen wie `ozgppCreationTime` und Komponenten `ozg-*` (WEB Abschnitt 6) **[Folgerung]**.
- **Betrieb:**
  - 2023: Serverkomponente bei BMI/ITZBund (HR S. 2). Ansprechpartner für Behörden und Länder war das BMI, Referat DV I 5 (TP S. 1).
  - 2024: formal das BMJ verantwortlich, das BMI für die BundID, das ITZBund für den Betrieb (HEISE Z. 216–220) **[unverifiziert]**.
  - Seit 30.07.2026: Westernacher und Rockenstein betreiben, Governikus entwickelt weiter (WS).
  - Der Intermediär für MJP-Postfächer liegt wahrscheinlich bei IT.NRW (EGVP-M Z. 67–80) **[Folgerung]**.
  - Wann und wie die Verantwortung vom BMI zur Justiz überging, ist nicht belegt.
- **Gremien:** Die BLK ist seit 2012 ständige AG des E-Justice-Rats. Ihr Vorsitz liegt bei der Justizverwaltung, die auch den Vorsitz im E-Justice-Rat führt, seit 1.7.2026 Bayern (`RG/justiz-de_BLK-Zusammensetzung-Aufgaben_2026-11-18.md`, `RG/justiz-de_E-Justice-Rat_2026-11-18.md`). Der Vorsitz der BLK-AG IT-Standards liegt beim JuM BW (IMP).
  - **Normsetzung:** Die ERVV erlässt die Bundesregierung mit Zustimmung des Bundesrates. Formate macht das BMJV per ERVB bekannt (§ 5 ERVV).
- **Finanzierung:** Für Bürger:innen kostenlos (PM; D19-28399 S. 27). Den ERV-Betrieb zahlen der Bund zu 1 % und die Länder nach Königsteiner Schlüssel (BLK-UB S. 1). Eine eigene Finanzierungsregel für MJP wurde nicht gefunden; vermutlich gilt dasselbe Muster **[Folgerung]**.
- **Nutzungszahlen:** keine gefunden.
- **Vorfall:** Das SAFE-Verzeichnis mit MJP-Nutzerdaten war vom 13.10. bis 09.11.2023 unzureichend geschützt abrufbar (NP23 Z. 43) **[unverifiziert]**. Die Nutzenden wurden am 13.11.2023 benachrichtigt (`MB/fragdenstaat_mjp-datenpanne_bmi_2023-11.md`).
- **Zukunft:**
  - Die Integration in die BundID war für 2024 angekündigt (TP S. 1) und ist nicht umgesetzt.
  - Das JuM BW prüft stattdessen die Integration in die föderale Zielarchitektur der FITKO, also ZaPuK (FDS-DA Z. 161).
  - ZaPuK sieht MJP als Bestandslösung, deren Frontend mit ZBP, OZG-PLUS-Postfach und Postfach 2.0 zusammengeführt werden soll. EGVP soll über einen Protokollkonverter angebunden werden (ZAPUK-AK Z. 391–419; `../zapuk-v1.0.0-121-g7419d8b/Glossar.md` Z. 523).
  - Pläne zur Ablösung von EGVP und OSCI wurden nicht gefunden.

## 3. Identität, Konto, Schlüssel

- **Anmeldung** nur über die BundID mit Vertrauensniveau „hoch“, also Online-Ausweis, Unionsbürgerkarte oder eAT. „Substanziell“ wird abgelehnt (TB Z. 575–577; FAQ Z. 59; `MB/servicejustiz-bmjv_mjp-einrichten.md` Z. 54).
  - Rechtlich wäre auch ELSTER zulässig (§ 13 Abs. 1 Nr. 2 ERVV; D20-10943 S. 70). ERL S. 2 nennt das noch, der Client nicht.
  - Für EU-eID gibt es eine manuelle Freischaltung über den Support mit Adressnachweis (TB Z. 789; `MB/mjpserviceportal_startseite.md` Z. 125) **[Folgerung]**.
- **Schlüsseldatei als zweiter Faktor zum Lesen:**
  - Das Schlüsselpaar entsteht im Browser (RSA-OAEP 3072, selbstsigniertes Zertifikat für 3 Jahre) und wird als passwortgeschützte `.p12`-Datei lokal gespeichert (WEB Z. 17–20; JS Z. 17).
  - „Die persönliche Schlüsseldatei wird nicht an den Server übermittelt.“ (TB Z. 132)
  - Zum Server und in SAFE geht nur das öffentliche Zertifikat (TB Z. 221; DSE Z. 88).
  - Verliert man Schlüsseldatei oder Passwort, gibt es keine Wiederherstellung; das Postfach muss gelöscht werden (FAQ Z. 289).
  - Seit Version 2.7.2 lässt sich die Schlüsseldatei erneuern, die alten Schlüssel werden mitgeführt (ERN S. 1; FAQ Z. 182).
- **Postfach und Konto:** MJP ist eine eigene Webanwendung neben der BundID. Gespeichert werden die SAFE-ID und eine BundID-Kennung (DSE Z. 110–111). Die BundID lässt sich erst löschen, wenn das MJP gelöscht ist (`../BundID-ZBP/quellen/bundid-betreiber/bundid_web-textbundle_de_vollstaendig.md` Z. 1329–1330).
- **Lebenszyklus:**
  - Wer das Konto löscht oder die Einwilligung widerruft, löscht Nachrichten und SAFE-Eintrag (TB Z. 754; DSE Z. 142).
  - Nachrichten werden seit 31.08.2025 nach 90 Tagen automatisch gelöscht, mit Hinweis 30 und 10 Tage vorher über die BundID (LÖ S. 1; FAQ Z. 236). Ob ungelesene ausgenommen sind, ist widersprüchlich (siehe 6).
  - Logs und Backups 90 Tage (DSE Z. 53, 144).
- **Adresse:** SAFE-ID `DE.Justiz.<UUID>.<Prüfwert>` (`AT/egvp_safe-abbildungsvorschrift-safe-id_v2_2017-07.pdf` S. 2).
  - Der SAFE-Eintrag enthält Name und Anschrift aus dem Ausweis, Länderkennung und Verschlüsselungszertifikat (§ 13a ERVV; DSE Z. 78–104). Eine abweichende Korrespondenzadresse ist nicht möglich (FAQ Z. 150).
  - Wer will, kann sich aus der Suche ausblenden und ist dann nur noch per SAFE-ID erreichbar (TB Z. 215–216).

## 4. Nachrichten und Funktionen

- **Senden (Bürger:in an Justiz):**
  - Formular mit Empfänger aus der SAFE-Suche, Gericht oder Behörde, Sachgebiet, Aktenzeichen (auch „neu“ oder „unbekannt“), „Eilig“, „Vertraulich“ und Anhängen (TB Z. 294–324).
  - Mindestens ein PDF ist Pflicht; höchstens 1000 Dateien und 200 MB, Dateinamen bis 90 Zeichen (CFG; ERVB S. 1–2).
  - Eine eigene `xjustiz_nachricht.xml` kann man hochladen (TB Z. 233).
  - **Kein Freitextfeld.** Text im EGVP-Container ist nach A17 unzulässig (ANF S. 6; WEB Abschnitt 6).
- **Empfang:**
  - Dokumente, `xjustiz_nachricht.xml` und Prüfvermerk (`MB/blk-mjp-support_mjp_nachrichten-speichern_2025-05.pdf` S. 1).
  - Ein Gericht kann in der XJustiz-Nachricht ein eEB anfordern. Die Person erzeugt es dann manuell und sendet es (EEB S. 1–4).
- **Erreichbare Partner:** Justiz, beBPo, beA, beN, beSt. Nicht erreichbar sind andere MJP- oder eBO-Postfächer (LF S. 9; FAQ Z. 51).
- **Anlassloser Erstkontakt** ist möglich. Eine Antwortfunktion gibt es nicht; eine Antwort ist eine neue Nachricht mit Aktenzeichen (TB; WEB) **[Folgerung]**.
- **Benachrichtigung:** MJP benachrichtigt das BundID-Postfach (ZBP), die BundID schickt dann eine E-Mail ohne Inhalt (FAQ Z. 204, 323; TB `SET_ZBP_MAILBOX_HANDLE_FAILED`).
- **Status und Belege:**
  - Für ausgehende Nachrichten zeigt der Client „wird gesendet“, „gesendet“ oder „Versand fehlgeschlagen“ an. Die automatisierte Eingangsbestätigung lässt sich ansehen und speichern (TB Z. 331–349; FAQ Z. 276).
  - OSCI-Laufzettel mit Eingangs- und Abholzeitpunkt (ANF S. 8).
  - Eine Lesebestätigung an den Absender gibt es nicht **[Folgerung]**.
- **Clients:** nur der Web-Client im Browser auf PC oder Laptop. Keine App, kein Smartphone, keine Einbindung in E-Mail-Programme (FAQ Z. 67, 91–95, 401). Die BLK hat „keine konkreten Planungen“ für mobile Nutzung (FDS-DA Z. 107).
- **Organisationen und Vertretung:** nicht vorgesehen. MJP ist nur für natürliche Personen, ohne Vertretung und ohne Übertragung (FAQ Z. 59, 85, 162). Organisationen nutzen das eBO mit kostenpflichtigen Drittprodukten oder seit 2024 das Organisationskonto nach § 13 ERVV (D20-10943 S. 70).
- **Eigene Systeme:** keine Schnittstelle. `api.mjp.justiz.de` bedient nur den eigenen Client (CFG) **[Folgerung]**.

## 5. Einordnung nach `morph_box.csv`

### Behörde

| Zeile | MJP |
|---|---|
| B1 Anbindung von Fachverfahren | Kein eigener Weg für MJP: Gerichte und Behörden adressieren MJP-Postfächer wie jeden EGVP-Teilnehmer (HR S. 1) **[Folgerung]**. Gerichte und Staatsanwaltschaften senden über justizeigene Sende- und Empfangskomponenten mit Fachverfahrensanbindung, etwa Bayern eKP für forumSTAR, Niedersachsen EUREKA (`AT/blk_laenderbericht-*.pdf`). Behörden nutzen das beBPo über registrierte Drittprodukte oder EGVP-Enterprise (`AT/egvp_drittprodukte.md`, `AT/egvp_behoerdenpostfach_voraussetzungen.md`). Eine XJustiz-Nachricht ist Pflicht (ANF S. 6). Passt zu Ausprägung „EGVP / beBPo“. |
| B2 Zugang ohne Fachverfahren | Client-Anwendungen als Drittprodukte (z. B. Governikus COM Vibilia beBPo Edition). XJustiz-Datensätze erzeugt die Browseranwendung auf xjustiz.de (`AT/governikus_erv-loesungen.md`; `AT/egvp_behoerdenpostfach_strukturdaten.md`). Passt zu keiner Ausprägung genau **[Folgerung]**. |
| B3 Adressierung der Person / Organisation | Zentrales Verzeichnis SAFE mit SAFE-ID, Name und Anschrift aus dem Ausweis, Rolle `ozg_postfach`. Sichtbar für Justiz, beBPo, beA, beN und beSt, Ausblenden aus der Suche möglich (HR S. 1; LF S. 9; TB Z. 215). Passt zu „zentrales Verzeichnis mit Kennung“, wo SAFE schon genannt ist. |
| B4 Inhalt an die Person / Organisation | Dokumente als PDF oder TIFF, dazu XJustiz-Strukturdaten (ERVB; ANF S. 6). Gerichtliche Dokumente tragen eine QES (§ 130b, § 169 Abs. 4 ZPO). Kein Nachrichtentext (ANF A17). Passt zu „Dokumente als Anhang“ und „strukturierte Nachricht“, die QES ist analog zum amtssignierten PDF **[Folgerung]**. |

### Person / Organisation

| Zeile | MJP |
|---|---|
| P1 Benachrichtigung | E-Mail ohne Inhalt, indirekt über das BundID-Postfach (FAQ Z. 204, 323). Hinweise vor der automatischen Löschung (LÖ S. 1). Gesetzliche Benachrichtigungspflicht nur im Online-Verfahren (§ 1130 ZPO). Die morph_box-Einordnung „E-Mail-Hinweis ohne Inhalt“ stimmt. |
| P2 Authentifizierung | BundID nur auf Vertrauensniveau hoch (eID), dazu die Schlüsseldatei mit Passwort zum Entschlüsseln (TB Z. 575; FAQ Z. 339). EU-eID nur nach manueller Freischaltung **[Folgerung]**. Passt zu „eID“. |
| P3 Organisationszugang | Keiner, nur für natürliche Personen (FAQ Z. 59, 85). Organisationen nutzen das eBO. |
| P4 Vertretung | Keine Vertretung und keine Übertragung (FAQ Z. 162). Prozessrechtlich kann ein Bevollmächtigter eine gescannte unterschriebene Erklärung übermitteln (§ 130a Abs. 3 S. 3 ZPO). Passt zu „keine Vertretung“. |
| P5 Clients | Nur der eigene Web-Client `mjp.justiz.de`, keine App (FAQ Z. 67, 401). Die morph_box nennt MJP unter „Web-Portal des Nutzerkontos“. Treffender ist ein eigener Web-Client neben der BundID **[Folgerung]**. |
| P6 Anbindung eigener Systeme | Keine Schnittstelle, nur Upload einer `xjustiz_nachricht.xml` (TB Z. 233). Wer eine Schnittstelle braucht, nutzt das eBO mit Drittprodukt. Passt zu „keine“. |
| P7 Antwort und Erstkontakt | Anlasslose Nachricht an Justiz, beBPo, beA, beN und beSt, Empfänger aus der SAFE-Suche (TB Z. 316; LF S. 9). Eine Antwortfunktion gibt es nicht, eine Antwort ist eine neue Nachricht mit Aktenzeichen **[Folgerung]**. Die morph_box nennt MJP auch unter „Antwort auf Nachricht“. Das trifft nur im Ergebnis zu, nicht als Funktion. |
| P8 Inhalt an die Behörde | Nur Dokumente, mindestens ein PDF (TB Z. 85; ERVB). Strukturfelder für Sachgebiet, Aktenzeichen, Eilig und Vertraulich, dazu ein optionaler XJustiz-Upload. **Kein Freitext**: Die morph_box-Einordnung unter „Freitext“ ist nicht belegt, im Client gibt es kein Textfeld (TB; WEB; ANF A17). |
| P9 Postfach und Nutzerkonto | Postfach getrennt vom Nutzerkonto, eigener Dienst mit eigenem Lebenszyklus und eigener Schlüsseldatei. Die BundID dient als Identitätsdienst und Benachrichtigungskanal (DSE Z. 110; HR S. 1). Passt zu „vom Nutzerkonto getrennt“. Die geplante Integration in die BundID ist nicht umgesetzt. |

### Übermittlung

| Zeile | MJP |
|---|---|
| Ü1 Modus | Asynchroner digitaler Brief über OSCI-Intermediäre, Store and Forward (ANF S. 3, 12). |
| Ü2 Vertraulichkeit | **Ende-zu-Ende verschlüsselt**, Schlüssel nur bei der Person als lokale Datei, ohne Backup beim Betreiber (TB Z. 132, 194; WEB Z. 17–37; HR S. 2; FAQ Z. 289). Einschränkungen: Der Betreiber liefert den JavaScript-Code aus **[Folgerung]**. Dateinamen und SHA-256-Hashwerte der Anhänge gehen unverschlüsselt an den Server (WEB Z. 28). Metadaten liegen beim Server. Auf Justizseite endet die Verschlüsselung an der Empfangskomponente **[Folgerung]**. **Die morph_box-Ausprägung „Ende-zu-Ende mit Schlüsselverwahrung beim Betreiber (Justizpostfach-Webclient)“ ist widerlegt.** MJP gehört zu „Ende-zu-Ende verschlüsselt“, wo EGVP/OSCI schon steht. Die Kritik, MJP sei nicht Ende-zu-Ende verschlüsselt (NP23, NP25), wird vom Client-Code nicht gestützt. |
| Ü3 Echtheit | Die Absenderin wird über den VHN nachgewiesen, eine fortgeschrittene Signatur am OSCI-Umschlag (LF S. 10; `AT/egvp_information-vhn-bebpo_2018-04.pdf` S. 3). Den VHN bringt vermutlich die Serverkomponente an, weil das Nutzerzertifikat selbstsigniert ist **[Folgerung]**. Der Prüfvermerk weist aus: „Sicherer Übermittlungsweg aus dem Postfach- und Versanddienst eines OZG-Nutzerkontos“ (HR S. 2–3). Die Person braucht keine QES, eine einfache Signatur plus sicherer Übermittlungsweg genügt (§ 130a Abs. 3 ZPO). Ähnlich „Identifikation beim Zugang ersetzt die Unterschrift“. Gerichte signieren qualifiziert, passt zu „Siegel oder Signatur der Behörde“. |
| Ü4 Ereignisse und Bestätigungen | Automatisierte Eingangsbestätigung an die einreichende Person (§ 130a Abs. 5 ZPO; TB Z. 349). OSCI-Laufzettel mit Eingang und Abholung (ANF S. 8). Die Zustellungsfiktion knüpft an die Eingangsbestätigung im Empfängerpostfach an (§ 173 Abs. 4 ZPO). Gelesen/ungelesen sieht nur die Person. Ein eEB ist nur manuell möglich (EEB). Keine Lesebestätigung **[Folgerung]**. Passt zu „Eingangsbestätigung an die Person / Organisation“ und „Bereitstellung an Behörde“. |
| Ü5 Interoperabilität mit Systemen außerhalb | Innerhalb des ERV ein gemeinsames Protokoll (OSCI, XJustiz, SAFE) für Justiz, beA, beN, beSt, beBPo, eBO und MJP (LF S. 9). Das entspricht „vollständige Standardisierung“ **[Folgerung]**. e-CODEX ist nur auf Justizseite angebunden (`AT/edvgt_…holthaus_2023-09.pdf` S. 7, 11). Zur BundID nur Login und Benachrichtigung. |
| Ü6 Malware-Prüfung | Keine Angaben zur Prüfung gefunden. Wegen der Ende-zu-Ende-Verschlüsselung kann frühestens die Empfangskomponente prüfen **[Folgerung]**. Formatbeschränkung: PDF ohne JavaScript und ohne eingebettete Objekte, dazu TIFF (ERVB S. 1). Passt zu „Einschränkung zulässiger Dateiformate“. |

### Infrastruktur und Betrieb

| Zeile | MJP |
|---|---|
| I1 Betrieb und Governance | Gemeinsames Bund-Länder-Angebot: BLK-AG IT-Standards, vertreten durch JuM BW; verantwortlich sind 16 Landesjustizverwaltungen und das BMJV (IMP; DSE). Betrieb seit 07/2026 durch Westernacher und Rockenstein, Entwicklung durch Governikus (WS). Intermediär wahrscheinlich bei IT.NRW **[Folgerung]**. 2023 unter BMI und ITZBund (HR S. 2). Kosten Bund 1 %, Länder nach Königsteiner Schlüssel (BLK-UB). |
| I2 Datenhaltung und Versand | Zentral: Die MJP-Serverkomponente hält Nachrichten Ende-zu-Ende-verschlüsselt vor, höchstens 90 Tage (DSE Z. 130; LÖ S. 1). OSCI-Intermediär als Zwischenspeicher (ANF S. 12). SAFE-Eintrag, solange das Postfach besteht. Kein Archiv, Download durch die Person. Rechenzentrum nicht genannt. |
| I3 Betriebsmodell für Organisationen | Für MJP-Nutzende entfällt das, nur natürliche Personen. Auf der Gegenseite betreiben Gerichte eigene Sende- und Empfangskomponenten und Intermediäre in Länder-Rechenzentren; Behörden betreiben beBPo-Drittprodukte (`AT/egvp_behoerdenpostfach_voraussetzungen.md` Z. 30–40). Im EGVP-Verbund also eher Teilbetrieb bzw. souveräner Betrieb **[Folgerung]**. |
| I4 Funktionspostfächer und interne Bearbeitung | Im MJP keine Funktionspostfächer. Bei Gerichten endet die Ende-zu-Ende-Verschlüsselung an der Empfangskomponente, verteilt wird über die Fachverfahren. Ähnlich „Ende-zu-Ende nur bis zur Organisationsgrenze“ **[Folgerung]**. |
| I5 Aufnahme in den Verbund | Nutzende: automatischer SAFE-Eintrag nach BundID-Login, keine Freischaltung (HR S. 1; FAQ Z. 383). Software: Registrierung als EGVP-Drittprodukt bei der BLK-AG nach den Anforderungen A1–A25 (ANF S. 3; DRT). Für Nutzerkonto-Postfachdienste kennt § 13 ERVV keine Zulassungs- oder Freischaltungsstelle; für eBO (§ 11) und beBPo (§ 7) gibt es sie. Wer prüft, ist nicht geregelt. Ausschlussregeln nicht gefunden. |

### Recht

| Zeile | MJP |
|---|---|
| R1 Schriftformersatz | Bürger:in an Gericht: Ein Dokument mit einfacher Signatur über MJP ist formwirksam, ohne QES (§ 130a Abs. 3 S. 1, Abs. 4 Nr. 4 ZPO; `RG/justiz-de_ERV-Buerger_2026-11-18.md` Z. 36). Parallelnormen in VwGO, ArbGG, SGG, FGO und StPO. Seit 1.10.2026 auch gegenüber Gerichtsvollzieher:innen (BGBl26). Gericht an Bürger:in: Die Form trägt das Dokument über die QES (§ 130b, § 169 Abs. 4 ZPO), MJP ist nur Transportweg. Gegenüber Verwaltungsbehörden außerhalb der Justiz ist der Schriftformersatz offen: § 3a Abs. 3 VwVfG nennt den Nutzerkonto-Weg nicht. |
| R2 Teilnahmepflicht | Freiwillig, globales Opt-in durch Einrichtung (D19-28399 S. 3, 27). Pflicht zur elektronischen Einreichung und zur Eröffnung eines Zustellwegs nur für Professionelle und Behörden (§§ 130d, 173 Abs. 2 ZPO). Ausnahme ist das Online-Verfahren: Wer sich über ein OZG-Nutzerkonto identifiziert, muss dort die Plattform nutzen (§ 1133 ZPO). |
| R3 Bekanntgabe und förmliche Zustellung | **Elektronische förmliche Zustellung an Bürger:innen** nach § 173 ZPO über den sicheren Übermittlungsweg. Voraussetzung ist eine Zustimmung je Verfahren, die als erteilt gilt, wenn die Person selbst über MJP eingereicht hat. Natürliche Personen können nicht allgemein zustimmen (§ 173 Abs. 4 S. 1–3). Die Zustellung gilt am vierten Tag nach der automatisierten Eingangsbestätigung als bewirkt, mit Gegenbeweis (§ 173 Abs. 4 S. 4–5). Ein eEB gibt es nur für Professionelle. Damit ist MJP ein deutsches Beispiel für die Ausprägung „elektronische förmliche Zustellung … mit belastbarem Nachweis“ **[Folgerung]**. Offen ist die Bekanntgabe von Verwaltungsakten über MJP nach § 9 OZG oder § 41 Abs. 2a VwVfG; sie ist nicht geregelt. |
| R4 Teilnahme der Behörden | Gerichte müssen elektronische Eingänge annehmen, seit 1.1.2022 alle Gerichte (§ 130a Abs. 5 ZPO; `RG/BT-Drs_17-12634_…pdf` S. 41). Behörden müssen in Justizverfahren elektronisch einreichen und einen Zustellweg eröffnen, über das beBPo, nicht über MJP (§§ 130d, 173 Abs. 2 ZPO). Im Verhältnis zur Justiz also eine Pflicht per Gesetz **[Folgerung]**. Bund und Länder waren nicht verpflichtet, MJP aufzubauen (D19-28399 S. 26). |

## 6. Widersprüche und offene Punkte

- **Ü2 in der morph_box:** „Schlüsselverwahrung beim Betreiber (Justizpostfach-Webclient)“ widerspricht Code, Oberfläche und BLK-Handreichung. Ausprägung 4 hätte danach kein belegtes Beispiel mehr. Möglicher Ursprung: Der Support bietet nach Verlust Ersatz an, gemeint ist vermutlich ein neues Schlüsselpaar **[unverifiziert]**.
- **P8 in der morph_box:** Unter „Freitext“ ist MJP genannt, belegt ist nur Dokumentversand.
- **Startdatum:** 12.10.2023 (HR, ERL) gegenüber 13.10.2023 (PM).
- **Verantwortung:** 2023 BMI DV I 5 und „Nutzerkonto Bund“, 2024 laut heise formal das BMJ, 2026 BLK-AG im Auftrag von Bund und Ländern. Ein Übergabeakt ist nicht belegt.
- **Identifizierungsmittel:** ERL (2023) und ERVV erlauben ELSTER, der Client verlangt Vertrauensniveau hoch.
- **Automatische Löschung:** LÖ (05/2025) sagt „unabhängig vom Status“, FAQ (10/2026) sagt, ungelesene Nachrichten werden nicht gelöscht. Vermutlich wurde die Regel geändert.
- **Sichtbarkeit in SAFE:** Die DSE nennt auch Bürger:innen als Einsichtsberechtigte, FAQ und HR nicht.
- **Erreichbarkeit aus dem beBPo:** HR sagt ja; `RG/justiz-de_ERV-Behoerden_2026-11-18.md` Z. 64 nennt MJP nicht.
- **Signatur:** Die BundID-FAQ sagt, man müsse „nicht mehr zusätzlich unterzeichnen“ (`../BundID-ZBP/quellen/bundid-betreiber/bundid_faq_hilfreiche-informationen.md` Z. 249). § 130a Abs. 3 ZPO verlangt eine einfache Signatur.
- **Mobile Nutzung:** Die FAQ kündigt sie an, die BLK hat keine Pläne.
- **Veraltete Angaben:** Nr. 5 statt Nr. 4 in § 130a Abs. 4, § 174 statt § 173 ZPO, Fiktion drei statt vier Tage.

## 7. Lücken

- Nutzungszahlen (Postfächer, Nachrichten) und Status Pilot- oder Regelbetrieb.
- Heutiger Auftraggeber, Rechenzentrum und Verträge seit dem Betreiberwechsel 07/2026; Bestätigung des Intermediärs bei IT.NRW.
- Eine eigene Vereinbarung, ein Beschluss oder eine Finanzierungsregel speziell zu MJP.
- Interna der Serverkomponente: wer die OSCI-Transportschlüssel und das VHN-Zertifikat hält; ob Metadaten im Klartext liegen.
- Malware-Prüfung auf Justizseite nach der Entschlüsselung.
- MJP im Verwaltungsverfahren: Schriftform nach § 3a VwVfG, Bekanntgabe nach § 9 OZG; formlose Mitteilung an Bürger:innen.
- Zulassung und Ausschluss von Nutzerkonto-Postfachdiensten nach § 13 ERVV.
- Nicht abgelegt: PostModG (BGBl. 2024 I Nr. 236, Fiktion 3 → 4 Tage), ERVV-Änderung vom 22.6.2026, verkündete Fassung des Gesetzes zur weiteren Digitalisierung der Justiz (nur der Entwurf liegt vor).

## 8. Hinweise zur Ablage

- Einige Quellen liegen doppelt in mehreren Unterordnern, mit identischer Prüfsumme: HR, ERL und TP (siehe Kürzeltabelle). Sie können bei Bedarf auf eine Kopie reduziert werden.
- `app.js` ist nicht abgelegt, weil es proprietärer Code ist. Stattdessen gibt es wörtliche Auszüge mit Byte-Offset und SHA-256 (WEB, JS).
- Belege je Zeile stehen in `../Referenzen-MJP.md`; der Ordner ist in `../../sources.md` eingetragen.
