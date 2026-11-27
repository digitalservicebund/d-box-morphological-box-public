# Befunde: Mein Justizpostfach (MJP) – technische Architektur und Anbindung

Stand der Auswertung: 2026-11-18. Dateien siehe `_index.md`. Fundstellen: „Z.“ = Zeile der abgelegten Text-/JSON-Datei, „S.“ = PDF-Seite, `[n]` = Byte-Offset in `app.js` (siehe Analyse-Datei). Kennzeichnung: **[Folgerung]** eigene Ableitung, **[unverifiziert]** ohne ausreichenden Beleg, **[veraltet?]** Quelle älter oder überholt.

Kürzel:
- WEB = mjp-webclient_analyse-app-js_v2.7.5_2026-11-18.md; I18N = mjp-webclient_i18n-messages-de_v2.7.5_2026-11-18.json; CFG = mjp-webclient_config_v2.7.5_2026-11-18.json
- ANF = blk_anforderungen-drittanwendungen-osci-erv_v1.8_2022-04-20.pdf; LF = justiz_erv-leitfaden_2025-07.pdf; ERVB = bmj_ervb-2025_2025-07-16.pdf
- BLK-MJP = jmnrw_erlass-anlage-mein-justizpostfach-information_v1.1_2023-10-25.pdf (Text der BLK-AG, gleichlautend blkag_mein-justizpostfach-information_v1.1_2023-10-25.pdf)
- SAFE-STD = egvp_standard-safe-verzeichnisdienst-suche_v1.5_2026-05-21.pdf; EBO = egvp_sicherer-uebermittlungsweg-buerger-organisationen_v1.4_2022-08-04.pdf; BEBPO-INFO = egvp_informationsmaterial-behoerden-bebpo-eeb_v1.4_2021-07.pdf
- ZAPUK-AK = ../../../ZaPuK/quellen/zapuk-v1.1.1/Zielarchitektur/Architekturkonzept/Vorversionen/v1.0/README.md

## Kurzbild der Architektur

1. **Transport:** EGVP-Infrastruktur mit OSCI-Transport 1.2 (ANF S. 10 Z. 356–357), asynchron über OSCI-Intermediäre, die Nachrichten zwischenspeichern (ANF S. 12 Z. 422–440). Doppelter Umschlag: Inhaltsdaten für den Leser, Nutzungsdaten für den OSCI-Manager verschlüsselt; der Intermediär sieht den Inhalt nicht (ANF S. 10 Z. 376–390). EGVP-Nachricht = `project_coco` (für den Empfänger verschlüsselt, mit `xjustiz_nachricht.xml`) + `vhn_coco` (nicht für den Empfänger verschlüsselt), das Ganze zusätzlich für den Intermediär des Empfängers verschlüsselt (ANF S. 8–9 Z. 309–329).
2. **Verzeichnis/Identität:** SAFE (Secure Access to Federated e-Justice/e-Government), föderiert: vier Instanzen (Justiz „SAFE public“, BRAK, BNotK, BStBK), zentraler Einstieg über den Virtuellen Attribut-Service (SAFE-STD S. 2 Z. 69–75). Je Teilnehmer: Name, Anschrift, Bundesland, Verschlüsselungszertifikat(e), Intermediär-URL und -Zertifikat, SAFE-Rolle, SAFE-ID (ANF S. 12 Z. 444–461). SAFE seit 2011 in Betrieb (justizportal_elektronische-kommunikation-bekanntmachungen-safe.md Z. 132–133).
3. **MJP:** Browser-Komponente (Webclient von Governikus, Version 2.7.5) + MJP-Serverkomponente + Intermediär; Schaubild der BLK-AG: „Absender OSCI-Nachricht erstellen → Intermediär → MJP Server-Komponente (BMI/ITZBund) → MJP Browser-Komponente; Entschlüsseln mit Zertifikat und PIN“ (BLK-MJP S. 2 Z. 100–108). MJP ist als EGVP-Drittprodukt registriert: ID 0023.0001.0001.040301, Governikus i. A. BMI, seit 01.10.2023 (egvp_drittprodukt-registrierung-governikus-mjp_2023-10.pdf S. 1).

## Befunde je Dimension

### B1 Anbindung von Fachverfahren

- Gerichte und Staatsanwaltschaften haben je ein EGVP-Postfach (LF S. 9 Z. 326–328). Sie empfangen über justizeigene Sende-/Empfangskomponenten mit Anbindung an Fachverfahren, z. B. Bayern „eKP“ für forumSTAR, SolumSTAR, RegisSTAR, Anbindung web.sta über eIP (blk_laenderbericht-bayern_2022-07.pdf S. 4 Z. 148–176) [veraltet?]; Niedersachsen EUREKA-VERSAND/EUREKA-EDDA über EGVP (blk_laenderbericht-niedersachsen_2024-08.pdf S. 11 Z. 500–506), e²P als Postein-/ausgang (ebd. S. 5 Z. 196). Wartungsmeldungen nennen „Sende- und Empfangskomponente der Justiz Sachsen“ und „Zertifikatswechsel der Postfächer“ (egvp_meldungen_2026-11.md Z. 40–46).
- Behörden: beBPo über registrierte Drittprodukte (Governikus COM Vibilia, MultiMessenger, procilon proDESK, FP Justiz Connect u. a.; egvp_drittprodukte.md Z. 14–64) oder die kostenlose Serveranwendung **EGVP-Enterprise** zur Integration in Fachverfahren, eingesetzt u. a. bei BAMF, DRV, Finanzverwaltung (egvp_behoerdenpostfach_voraussetzungen.md Z. 17–24; egvp_drittprodukte.md Z. 102–105).
- Fachverfahren müssen eine XJustiz-Nachricht als `xjustiz_nachricht.xml` beilegen (ANF S. 6 Z. 205–215; LF S. 14 Z. 552). Für den MJP-Versand kann eine `xjustiz_nachricht.xml` aus einer Fachanwendung hochgeladen werden (I18N Z. 674–675).
- Für die Anbindung gibt es keinen MJP-spezifischen Weg: Behörden adressieren MJP wie jeden EGVP-Teilnehmer per SAFE (BLK-MJP S. 1 Z. 34–40) **[Folgerung]**.

### B2 Zugang ohne Fachverfahren

- Behörden ohne Fachverfahren nutzen eine Client-Anwendung (z. B. COM Vibilia beBPo Edition; governikus_erv-loesungen.md Z. 196–240). XJustiz-Datensätze erzeugt dann die Browseranwendung auf xjustiz.de (egvp_behoerdenpostfach_strukturdaten.md Z. 20–24).

### B3 Adressierung der Person / Organisation

- Adressierung über den SAFE-Verzeichnisdienst. MJP-Inhaber werden mit Name und Anschrift aus dem Online-Ausweis in „SAFE-public“ eingetragen und erhalten automatisch die Rolle `ozg_postfach` (BLK-MJP S. 1 Z. 27–28, Z. 47; LF S. 9 Z. 364).
- Sichtbarkeit: nur Gerichte, Staatsanwaltschaften und beBPo-Behörden nach sicherer Anmeldung, nicht für andere MJP- oder eBO-Inhaber (BLK-MJP S. 1 Z. 34–41); beA/beN/beSt laut Leitfaden 2025 ebenfalls (LF S. 9 Z. 338–353). Nutzer:innen können die Auffindbarkeit ganz abschalten, dann nur noch per SAFE-ID erreichbar (I18N `user.account.hidesearch.model.text`, Z. 623–624; API `/safe/account/hidden`, WEB Z. 42).
- SAFE-ID: `DE.Justiz.<UUID>.<4 Hex aus SHA-256>` (egvp_safe-abbildungsvorschrift-safe-id_v2_2017-07.pdf S. 2 Z. 47, Z. 66; wikipedia_mein-justizpostfach_2026-11.md Z. 29–34).
- Vor jedem Versand muss der Client prüfen, ob das Empfängerpostfach noch im SAFE registriert ist (ANF S. 5 Z. 162–163).
- Die SAFE-Suche ist für das MJP über die Server-API gekapselt (`/safe/accounts`, `/safe/account`; WEB Z. 42) **[Folgerung]**: der Browser fragt SAFE nicht direkt ab.

### B4 Inhalt an die Person / Organisation

- Zulässig: PDF (inkl. PDF 2.0, PDF/A-1, PDF/A-2, PDF/UA) und TIFF 6; XJustiz „uebermittlungSchriftgutobjekte“; höchstens 1 000 Dateien und 200 MB je Nachricht; Dateinamen max. 90 Zeichen (ERVB S. 1 Z. 15–41, S. 2 Z. 73). Gilt für Eingänge bei Gericht; die Grenzen beziehen sich auf die Summe der Anlagen (LF S. 12 Z. 462–498).
- Ein BLK-AG-Beschluss nennt 1 GB als möglichen Orientierungswert für Akten (governikus_bereit-fuer-grosse-behoerdenakten_2026-10-08.md Z. 183–219) **[unverifiziert]** (Beschluss selbst nicht gefunden).
- Kein Nachrichtentext im Container erlaubt (ANF S. 6 Z. 186–188); fachliche Angaben gehören in die XJustiz-Nachricht.

### P2 Authentifizierung

- Anmeldung am MJP mit BundID (Vertrauensniveau hoch, Online-Ausweis) (BLK-MJP S. 1 Z. 17–18; hhbjv_hinweisblatt-mjp-einrichten_2025-08-26.pdf S. 1). Für Bayern zusätzlich BayernID (justizportal_erv-buerger_mjp-ebo.md Z. 101–104).
- Zum Lesen zusätzlich Schlüsseldatei (PKCS#12) und deren Passwort (I18N `homepage.main.encrypted_communication.text`; WEB Z. 19–20).

### P5 Clients

- Nur der Webclient unter mjp.justiz.de (`<meta name="description" content="Webclient für Mein Justizpostfach">`, WEB Z. 3); Hersteller Governikus, Version 2.7.5 (mjp-webclient_productinfo_v2.7.5_2026-11-18.json Z. 3–4). Früher erreichbar unter mein-justizpostfach.bund.de (blkag_mein-justizpostfach-information_v1.1_2023-10-25.pdf S. 1 Z. 22) [veraltet?].
- eBO als Alternative braucht kostenpflichtige Drittsoftware (egvp_buerger-organisationen_ebo-mjp.md Z. 31–47).

### P6 Anbindung eigener Systeme

- Keine dokumentierte Schnittstelle für Bürger:innen. Die API `https://api.mjp.justiz.de/api/v1/public` dient dem eigenen Webclient (CFG Z. 4–5) **[Folgerung]**. Für eigene Systeme bleibt das eBO mit Drittprodukt.

### P7 Antwort und Erstkontakt

- MJP-Inhaber können anlasslos an Gerichte, Staatsanwaltschaften, beBPo-Behörden, beA, beN, beSt senden; nicht an andere MJP- oder eBO-Inhaber (LF S. 9 Z. 336–353). Empfänger wird über SAFE gesucht (I18N `createmessage.form.recipient.tooltip.text`).
- Eine eigene „Antworten“-Funktion fand sich in den Oberflächentexten nicht; Antworten sind neue Nachrichten an den Absender **[Folgerung]**.

### P8 Inhalt an die Behörde

- Formular: Empfänger, Gericht/Behörde, Sachgebiet, Aktenzeichen (auch „neu“/„unbekannt“), eilig, vertraulich, Anhänge; mindestens ein PDF Pflicht; max. 1 000 Dateien, 200 MB (I18N Z. 230–231, Z. 653–911; CFG Z. 35–42; WEB Z. 32). Kein Feld für Nachrichtentext gefunden (WEB Abschnitt 6) – das passt zu ANF A17 **[Folgerung]**. Die morph_box nennt MJP unter „Freitext“; das wäre zu prüfen.

### P9 Postfach und Nutzerkonto

- MJP ist „Postfach- und Versanddienst des Nutzerkontos Bund gemäß § 13 ERVV“ (justizportal_erv-buerger_mjp-ebo.md Z. 82–84), läuft aber als eigene Webanwendung; Integration in die BundID war für „2024 tbd“ angekündigt (bmi_broschuere-mjp-two-pager_2023-08.pdf S. 1) [veraltet?]. Stand 2026 weiterhin eigene Anwendung unter mjp.justiz.de **[Folgerung]**.
- Benachrichtigungen laufen über das BundID-Postfach (ZBP): Fehlertext `SET_ZBP_MAILBOX_HANDLE_FAILED` (I18N Z. 1686–1689), „Benachrichtigungen an ihr BundID-Postfach“ (I18N Z. 2255–2256).
- Naming im Client (`ozgppCreationTime`, `ozg-*`-Elemente) deutet auf gemeinsame Codebasis mit dem OZG-PLUS-Postfach von Governikus (WEB Abschnitt 6) **[Folgerung]**.

### Ü1 Modus

- Asynchron, Store-and-Forward über Intermediäre; Drittanwendungen sollen höchstens alle 15 Minuten abfragen (ANF S. 3 Z. 79–81). Versandstatus im MJP: „wird gesendet“ → „gesendet“/„fehlgeschlagen“ (I18N `createmessage.success.text`; WEB Z. 31).

### Ü2 Vertraulichkeit – Prüfung der morph_box-Aussage

Morph_box Ü2, Ausprägung 4: „Ende-zu-Ende mit Schlüsselverwahrung beim Betreiber (Justizpostfach-Webclient)“. **Befund: widerlegt.**

- Der private Schlüssel entsteht im Browser: `crypto.subtle.generateKey` mit RSA-OAEP 3072/SHA-256, selbstsigniertes Zertifikat (3 Jahre) per node-forge, PKCS#12 mit Nutzerpasswort (WEB Z. 17–19, [443927], [481555]).
- Die Schlüsseldatei wird lokal gespeichert (`mjp_<…>.p12`, WEB Z. 20); an den Server geht nur das öffentliche Zertifikat (`/safe/account/modify/certificate`). Oberflächentexte: „Die persönliche Schlüsseldatei wird nicht an den Server übermittelt.“ (I18N Z. 372), „… Ihr öffentliches Zertifikat an den Postfachserver übermittelt“ (I18N Z. 639). Ebenso BLK-AG: Zertifikat „das der MJP-Nutzer in seinem Machtbereich sichert“ (BLK-MJP S. 2 Z. 89–94); Hamburg: „Schlüsseldatei speichern … lokal auf Ihrem Computer“ (hhbjv_hinweisblatt-mjp-einrichten_2025-08-26.pdf S. 6–7).
- Versand: Anhänge und XJustiz-Datei werden im Browser mit AES verschlüsselt; der AES-Schlüssel wird für das Leserzertifikat des Empfängers aus SAFE und für das eigene Zertifikat verpackt (WEB Z. 27–30). Empfang: Entschlüsselung des `xenc:EncryptedKey` im Browser (WEB Z. 36–37). Damit Ende-zu-Ende zwischen Browser und Gericht/Behörde bzw. deren Empfangskomponente.
- Kein Backup des privaten Schlüssels beim Betreiber gefunden. Bei Verlust: neue Schlüsseldatei und neues Zertifikat; laut Blog danach alte Nachrichten nicht mehr lesbar (borncity_stoerung-mjp-schluesseldatei_2024-08-24.md Z. 69–77) – Sekundärquelle. Der Client kennt nur alte Schlüssel, die im neuen Keystore mitgeführt werden (WEB Z. 22) **[Folgerung]**.
- Einschränkungen: (a) Der Betreiber liefert den JavaScript-Code aus und könnte ihn ändern **[Folgerung]**. (b) Dateinamen und SHA-256-Hashwerte der Klartext-Anhänge gehen unverschlüsselt an den Server (WEB Z. 28). (c) Metadaten (Absender, Empfänger, XJustiz-Kopf bei Prüfvermerk) liegen beim Server; Prüfvermerk und VHN-Prüfergebnis kommen vom Server (WEB Z. 38). (d) Auf Justizseite endet die Ende-zu-Ende-Verschlüsselung an der Empfangskomponente des Gerichts, nicht bei der Sachbearbeitung **[Folgerung]**.
- Vorschlag **[Folgerung]**: MJP gehört zur Ausprägung „Ende-zu-Ende verschlüsselt“ (Schlüssel nur bei der Person, als Datei); Ausprägung 4 hat dann kein belegtes Beispiel.

### Ü3 Echtheit

- Absender-Authentizität über den **VHN** (vertrauenswürdiger Herkunftsnachweis): signierte XML-Datei, wird beim Empfang automatisch geprüft (LF S. 10 Z. 372–378); technisch spezieller OSCI-Header plus fortgeschrittene Signatur am äußeren Umschlag (egvp_information-vhn-bebpo_2018-04.pdf S. 3 Z. 64–66); beBPo-VHN-Zertifikate von der BNotK-CA (ebd. S. 1 Z. 37). Alle Nachrichten auf sicherem Übermittlungsweg brauchen VHN bzw. VHN 2 (ANF S. 6 Z. 195–202).
- Das MJP-Zertifikat der Person ist selbstsigniert und dient nur der Verschlüsselung (WEB Z. 18). Den VHN bringt daher die MJP-Serverkomponente an **[Folgerung]**. Prüfvermerk kennzeichnet MJP-Nachrichten (BLK-MJP S. 3 Z. 115–116); Client ordnet VHN-Signatur-CA u. a. „OZG“ zu (WEB Z. 38).
- Keine qualifizierte Signatur der EGVP-Nachricht (Containersignatur unzulässig, ANF S. 5 Z. 166–169); QES auf Dokumenten als CAdES detached oder PAdES (ERVB S. 1 Z. 49–58).

### Ü4 Ereignisse und Bestätigungen

- OSCI-Laufzettel, signiert vom OSCI-Manager: Eingangsbestätigung mit MessageId und Eingangszeitpunkt auf dem Intermediär (Pflicht, ANF S. 8 Z. 288–293); Abholbestätigung mit Abholzeitpunkt (empfohlen, ANF S. 8 Z. 296–301).
- Rechtlich eingegangen ist eine Nachricht mit Eingang auf dem Intermediär des Empfängers (LF S. 10 Z. 390–399). Prüfvermerk mit wer/was/wann/wie/an wen als PDF und XML (LF S. 11 Z. 427–450).
- MJP-Client: „Eingangsbestätigung“ mit „Eingang auf dem Server“ und „Prüfvermerk“ für Eingänge (I18N Z. 1604–1617, Z. 1307); OSCI-MessageId und -Zeit (WEB Z. 38). Abholzeitpunkt sichtbar laut Wikipedia (wikipedia_mein-justizpostfach_2026-11.md Z. 38–42) [Sekundär].
- eEB: XJustiz-Bibliothek kennt `ruecksendungEEB.erforderlich` (WEB Z. 12), aber keine eEB-Funktion in den Oberflächentexten **[Folgerung]**: MJP gibt kein elektronisches Empfangsbekenntnis ab. eEB-Standard: LF S. 26 Z. 1076–1081, BEBPO-INFO S. 14 Z. 502–507.

### Ü5 Interoperabilität mit Systemen außerhalb

- Innerhalb des ERV: MJP ↔ Justiz, beBPo, beA, beN, beSt; nicht MJP ↔ MJP, nicht MJP ↔ eBO (LF S. 9 Z. 336–353). Behörden adressieren MJP-Nutzer als „OZG-Konten“ (justizportal_erv-behoerden_bebpo.md Z. 115–116).
- BundID als Identitätsprovider und Benachrichtigungskanal (siehe P2, P9).
- e-CODEX: Anbindung auf Justizseite über EGVP → Connector → Gateway, koordiniert von NRW (edvgt_nationale-umsetzung-europaeische-justizdigitalisierung-holthaus_2023-09.pdf S. 7 Z. 132, S. 11 Z. 206–210) [veraltet?]. Für MJP-Nutzer kein direkter Weg belegt **[Folgerung]**.
- ZaPuK: MJP als Bestandslösung (ZAPUK-AK Z. 391); Messaging Bridges verboten, nur Frontend-Integration über Channel Adapter (ZAPUK-AK Z. 395–403); MJP-Integration mit EGVP kann entfallen, wenn EGVP in die Zielarchitektur integriert wird (ZAPUK-AK Z. 419); Justiz mit eigenem Postfach-Backend, EGVP über Protokollkonverter (../../../ZaPuK/quellen/zapuk-v1.1.1/Glossar.md Z. 633); Frontends von ZBP, MJP, OZGPP, Postfach 2.0 zusammenführen (../../../ZaPuK/quellen/zapuk-v1.1.1/Zielarchitektur/Zusammenfassung/Vorversionen/v1.0/README.md Z. 222).

### Ü6 Malware-Prüfung

- Kein Beleg für eine Malware-Prüfung im MJP oder auf den Intermediären. Wegen der Ende-zu-Ende-Verschlüsselung kann erst die Empfangskomponente prüfen **[Folgerung]**. Vorgaben: PDF ohne JavaScript und eingebettete Objekte (ERVB S. 1 Z. 22–31); Client sperrt reservierte Dateinamen (`vhn.xml`, `project_coco` …; WEB Z. 32). Länderbericht Niedersachsen nennt Virenschutz nur für E-Mail- und Internetverkehr (blk_laenderbericht-niedersachsen_2024-08.pdf S. 26 Z. 1188–1190).

### I1 Betrieb und Governance

- Standards und Registrierung von Drittprodukten: BLK-AG „IT-Standards in der Justiz“ (ANF S. 3 Z. 54–56; LF S. 1). EGVP-Infrastruktur seit 2004 (egvp_behoerdenpostfach.md Z. 28–29; egvp_allgemeine-informationen.md Abschnitt „Erweiterung der EGVP-Infrastruktur“).
- MJP: bis 2026 Serverkomponente bei BMI/ITZBund (BLK-MJP S. 2 Z. 100) [veraltet?]; laut heise formal BMJ verantwortlich, BMI für BundID, ITZ(Bund) für den Betrieb, Entwicklung in Bremen (heise_massive-stoerungen-erv_2024-08.md Z. 216–220). Seit Migration am 30.07.2026 technischer Betrieb und Support durch Westernacher Solutions mit Rockenstein AG, Weiterentwicklung Governikus (westernacher_uebernahme-betrieb-mjp.md Z. 46–66). Domainwechsel mein-justizpostfach.bund.de → mjp.justiz.de **[Folgerung]**: Übergang in die Verantwortung der Justiz; Auftraggeber nicht belegt **[unverifiziert]**.
- Intermediär: „Alle bei IT.NRW gehosteten Bürgerpostfächer“ – betroffen u. a. „Nutzer des Mein Justizpostfaches“ (egvp_meldungen_2026-11.md Z. 67–80) **[Folgerung]**: MJP-Postfächer liegen auf einem Intermediär bei IT.NRW. eBO-Intermediär „im Auftrag der Justiz“ (EBO S. 7 Z. 271–276).
- SAFE-Software: Justiz „SAFE“ (Weiterentwicklung u. a. Westernacher, westernacher_justiz-infrastruktur-erv.md Z. 68–77), BRAK „Governikus autent“, BNotK „procilon proNEXT Security Manager“ (BEBPO-INFO S. 8 Z. 229–234) [veraltet?]. Intermediäre in Ländern und Bund in öffentlich-rechtlichen Rechenzentren; Software als Anwendung des IT-Planungsrats über Governikus (egvp_behoerdenpostfach_voraussetzungen.md Z. 30–40).
- Support: MJP-Anwendersupport im ERV-Ticketsystem EFEVER (BLK-MJP S. 3 Z. 121–124) [veraltet?]; heute FAQ/Kontakt über mjpserviceportal.de (CFG Z. 15–19).

### I2 Datenhaltung und Versand

- Intermediär speichert OSCI-Nachrichten im asynchronen Szenario (ANF S. 12 Z. 439–440); dauerhafte Speicherung außerhalb der Infrastruktur (../../../ZaPuK/quellen/zapuk-v1.1.1/Zielarchitektur/Architekturentscheidungen/ADR-0011-Kommunikationsschicht.md Z. 304–306).
- MJP-Server hält ein- und ausgehende Nachrichten verschlüsselt vor (API `/messages/ebo/incoming`, `/outgoing`, WEB Z. 31) **[Folgerung]**; Backups und Logs bis 90 Tage verschlüsselt (WEB Abschnitt 6, [559289]). Löschen des Postfachs löscht Nachrichten und SAFE-Eintrag (I18N Z. 2237–2238).
- Zertifikatslaufzeit 3 Jahre; nach Ablauf kein Senden/Empfangen bis zur Erneuerung (WEB Z. 18; I18N Z. 1994–1995).

### I4 Funktionspostfächer und interne Bearbeitung

- OSCI erlaubt getrennte Rollen Empfänger/Leser und Sender/Autor (ANF S. 5 Z. 146–150, S. 11 Z. 413–417). MJP nutzt das **[Folgerung]**: Serverkomponente holt als Empfänger ab, Browser entschlüsselt als Leser (BLK-MJP S. 2 Z. 100–108). Für Behörden: virtuelle Poststelle (Governikus MultiMessenger) auch als SaaS (governikus_erv-loesungen.md Z. 199–212).

### I5 Aufnahme in den Verbund

- Neue Sende-/Empfangssoftware braucht eine Registrierung als Drittprodukt bei der BLK-AG IT-Standards und muss A1–A25 erfüllen, ohne die Infrastruktur anzupassen (ANF S. 3 Z. 54–78). Weitere SAFE-Verzeichnisdienste koordiniert die BLK-AG (egvp_behoerdenpostfach_voraussetzungen.md Z. 52–60). Behörden: Freischaltung durch beBPo-Prüfstelle je Land (justizportal_erv-behoerden_bebpo.md Z. 151–171).

### R1 Schriftformersatz / R3 Bekanntgabe (nur technischer Bezug)

- MJP gilt als sicherer Übermittlungsweg („Postfach- und Versanddienst eines OZG-Nutzerkontos“) (LF S. 6 Z. 168–176). Einzelheiten beim Recht-Agenten.

## Widersprüche

1. **Ü2 morph_box vs. Quellen:** „Schlüsselverwahrung beim Betreiber“ widerspricht Code und Oberfläche (Schlüssel nur lokal). Möglicher Ursprung: Supportaussage „Ersatz bei Verlust über den MJP-Support“ (Websuche, LG Hildesheim, nicht gesichert) [unverifiziert] – Ersatz heißt vermutlich neues Schlüsselpaar.
2. **Betreiber:** BLK-MJP 2023 „BMI/ITZBund“; Wikipedia „betrieben von der BLK-AG“ (wikipedia_mein-justizpostfach_2026-11.md Z. 8); Westernacher 2026 „Betrieb … übernommen“. Wer seit 2026 Auftraggeber ist (BMJV, BLK, ein Land), ist offen.
3. **Adressierbarkeit durch beA/beN/beSt:** BLK-MJP 2023 nennt sie als „weitere Ausbaustufe“ (S. 2 Z. 51–53); LF 2025 sagt, sie können mit MJP kommunizieren (S. 9 Z. 350–351). Wohl umgesetzt **[Folgerung]**.
4. **Größen:** justizportal_erv-behoerden_bebpo.md Z. 95–97 nennt noch „100 MB / 200 Anlagen, ab 2023 200 MB / 1 000“ [veraltet?]; ERVB 2025 und CFG: 200 MB / 1 000. Governikus: 1 GB als BLK-Orientierungswert für Akten.
5. **P8 Freitext:** morph_box nennt MJP unter „Freitext“; im Formular gibt es kein Textfeld, Container-Text ist nach A17 unzulässig.
6. **Netzpolitik:** „Das MJP, das zunächst eBO hieß“ (netzpolitik_datenleck-mjp-safe_2023.md Z. 107–108) widerspricht Wikipedia/LF (MJP ist kein eBO) [Sekundär, vermutlich ungenau].

## Lücken

- Primärquelle zur MJP-Serverkomponente: wer die OSCI-Transportschlüssel und das VHN-Signaturzertifikat hält, wie die Rollen Empfänger/Leser konkret verteilt sind.
- Aktueller Auftraggeber und Rechenzentrum des MJP nach dem Wechsel 2026; Bestätigung, dass der MJP-Intermediär bei IT.NRW liegt.
- Malware-Prüfung auf Justizseite nach der Entschlüsselung (Empfangskomponenten der Länder).
- Integration in die BundID: aktueller Stand und Plan; Rolle von MJP in ZaPuK-Transition (nur Frontend vs. Backend).
- Ablösung von EGVP/OSCI: keine Planung der BLK gefunden; Justizcloud (lto_justizcloud-anfang-2027.md) nennt EGVP nicht.
- VHN-2-Spezifikation (ZIP) und Schnittstelle der MJP-API nicht abrufbar.
- egvp.justiz.de war direkt nicht erreichbar; Seiten stammen aus dem Internet Archive (Snapshots Sept.–Nov. 2026).
