# Digital postkasse til innbyggere (Norwegen): Zusammenfassung für die d-box

Wie die norwegische Digital postkasse til innbyggere (DPI) mit den privaten Postfachanbietern Digipost und e-Boks aufgesetzt ist: Betrieb, Anbindung, Recht. Eingeordnet nach den Zeilen von `morph_box.csv`. Die Zusammenfassung verweist auf die Quellen, statt sie zu wiederholen. Für Details den Pointer öffnen.

Stand: 2026-11-23. Grundlage ist `../Referenzen-Digital-Postkasse-Norwegen.md` und eine ergänzende Webrecherche. Alle Primärquellen liegen in `quellen/`, jeder Unterordner hat eine `_index.md` mit URL, Herausgeber, Datum und Inhaltsnotiz.

## Kennzeichnung

Wie in `../Referenzen-Digital-Postkasse-Norwegen.md`:

- Ohne Kennzeichnung: Stufe 3, in einer Primärquelle im Volltext belegt.
- **[Indiz]**: Stufe 2, Sekundärquelle, veraltete Angabe, Rückschluss oder Fehlanzeige nach gezielter Suche.
- **[Annahme]**: Stufe 1, kein Beleg.
- **[veraltet]**: Quelle älter, aktueller Stand nicht belegt.

## Kürzel für Quellen

Pfade relativ zu `quellen/`. Z. = Zeile der Markdown-Kopie, S. = gedruckte Seite, PDF-S. = Seite der PDF-Datei. Kürzel wie in der Referenzen-Datei, neue Kürzel: BV, AV, SA, QA, RS19, FVL25.

| Kürzel | Datei | Inhalt |
|---|---|---|
| DPK | `betreiber/digdir_digital-postkasse-til-innbyggere.md` | Überblick DPI, „kryptert frem til postkassen“ |
| AKT / TJL | `betreiber/digdir_dpi_aktorer-og-roller.md`, `…_tjenesteleverandorer.md` | Rollen, Anbieter (Digipost, e-Boks, Druck Skatteetaten) |
| DP / EB | `betreiber/digipost_hjelp_*.md`, `betreiber/e-boks_norge_startseite.md` | Postfachanbieter |
| IDP | `betreiber/digdir_idporten_overordnet.md` | ID-porten |
| AM / AA / TP | `betreiber/altinn_*.md` | Altinn Melding, Autorisasjon, vertrauliche Post |
| SA | `betreiber/difi_dpi_sikkerhetsarkitektur_2015-03-12.pdf` | Sicherheitsarchitektur 2015 **[veraltet]** |
| QA | `betreiber/digdir_dpi_webinar-qa-informasjonssikkerhet_2021-06.pdf` | Webinar-Q&A Digdir 2021 |
| NYI | `anbindung-technik/digdir_dpi_ny-infrastruktur.md` | Neue Infrastruktur: Peppol, REST, Maskinporten |
| SIK / MIME / VAR / UTS | `anbindung-technik/digdir_dpi_*.md` | Dokumentpaket, Formate, Benachrichtigung, Druck |
| KV | `anbindung-technik/digdir_sdp_forretningsprosess-kvittering.md`, `…_sdp_aapningskvittering.md`, `…_dpi_aapningskvittering.md` | Quittungen |
| DPI | `anbindung-technik/digdir_sdp_digitalpostinfo.md`, `…_innbyggerpost_dpi_digital_1_0.schema.json` | Metadaten, `sikkerhetsnivaa` |
| UTV | `anbindung-technik/digdir_sdp_utvidelse-lenke.md`, `…-arrangement.md` | Erweiterungen Link und Termin |
| KRR | `anbindung-technik/digdir_krr_oppslagstjenesten-rest.md` | Kontakt- og reservasjonsregisteret |
| EF | `anbindung-technik/digdir_eformidling_*.md` | eFormidling, Kanalwahl |
| EFF | `recht-governance/eForvaltningsforskriften_…md` | eForvaltningsforskriften |
| FVL / FVL25 | `recht-governance/forvaltningsloven_LOV-1967-…md`, `…_LOV-2025-06-20-81_…md` | Forvaltningsloven, alt und neu (neu nicht in Kraft) |
| RS19 / RS | `recht-governance/kmd_digitaliseringsrundskrivet_2019-12-20.pdf`, `…/difi-rapport_2019-6_…pdf` | Digitalisierungsrundschreiben 2019, Difi-Bericht 2019 **[veraltet]** |
| BV / AV | `recht-governance/digdir_bruksvilkar-offentlige-kunder_2025-11.pdf`, `…/digdir_avtale-om-bruk-felleslosninger.md` | Nutzungsbedingungen und Vertrag Digdir–Behörde |

## 1. Systembild

- DPI ist eine Fellesløsning von Digdir: Behörden senden digitale Briefe an das Postfach, das die Person bei einem zugelassenen privaten Anbieter gewählt hat, Digipost (Posten Norge) oder e-Boks (BV S. 18 Kap. 6.1; TJL Z. 106). Ein Druckdienst (Skatteetaten) versendet Papier aus demselben Kanal (TJL Z. 102; DPK Z. 65).
- Adressierung über die Fødselsnummer: Das KRR liefert Postfachanbieter, Zertifikat, Kontaktdaten und Reservierung (KRR; EFF §§ 29–31).
- Transport seit der neuen Infrastruktur über Peppol im 4-Ecken-Modell: Behörde → Zugangspunkt (REST, Maskinporten) → AS4 → Zugangspunkt des Postfachanbieters (NYI Z. 85–107).
- Dokumentpaket ASiC-E mit XAdES-Signatur der Behörde, verschlüsselt per CMS an das Zertifikat aus dem KRR (SIK Z. 83–84).
- Organisationen erhalten Post nicht über DPI, sondern über Altinn (AM; EF).

## 2. Betrieb, Governance, Teilnahme

- Digdir „forvalter“ die Fellesløsninger, darunter DPI (BV S. 1 Kap. 1.1), hat Verträge mit den Postfachanbietern und betreut sie (BV S. 18 Kap. 6.1, S. 19 Kap. 6.4.1). Die Rolle „Sentralforvalter“ ist in AKT ohne Träger beschrieben (AKT Z. 91); 2015 war Difi Sentralforvalter (SA).
- Digdir ist Datenverarbeiter der Behörde; Unterauftragnehmer Tietoevry Norway AS (Zugangspunkt), Posten Norge AS und e-Boks A/S (BV S. 19). Behörde ist Verantwortliche bis zur Bereitstellung, danach der Postfachanbieter nach seinen Nutzungsbedingungen mit der Person (BV S. 20; QA S. 2).
- Teilnahme der Behörde: Bruksvilkår annehmen (Vertrag AV), dann Maskinporten-Scope `digitalpostinnbygger:send` (NYI Z. 189, 203). Behörden dürfen keinen Postfachanbieter ausschließen (BV S. 19).
- Kosten: Kostenmodell auf der Samarbeidsportalen; Digdir ab 1.1.2026 mehrwertsteuerpflichtig für DPI (BV S. 1 Kap. 1.3). Für Personen kostenlos (BV S. 18).
- Betrieb: 24/7, Quittungen innerhalb einer Stunde, Verfügbarkeit 99,5 %; Logs max. 24 Monate (BV S. 19).
- Anbieter im EWR, Digipost-Postfächer in Norwegen (QA S. 1, Stand 2021 **[veraltet]**).

## 3. Einordnung nach `morph_box.csv`

### Behörde

| Zeile | Norwegen |
|---|---|
| B1 Anbindung von Fachverfahren | eFormidling-Integrationspunkt oder Klientbibliotheken, REST an den Zugangspunkt, Maskinporten-Token (NYI Z. 129–147; EF). eFormidling wählt DPI, Altinn oder Druck (EF Vedtak Z. 104–136). |
| B2 Zugang ohne Fachverfahren | Druckdienst im DPI (UTS; TJL Z. 102); ohne Postfach entscheidet die Behörde zwischen Altinn und Papier (EF Vedtak Z. 132). Eine Weboberfläche zum Versand ist nicht belegt. |
| B3 Adressierung | Fødselsnummer, KRR-Abfrage über Maskinporten (KRR Z. 48); Organisationen über Organisationsnummer in Altinn. |
| B4 Inhalt | Hauptdokument und Anhänge (MIME), Erweiterungen Link mit Frist und Termin (UTV); Altinn: Text, Anhänge, Links (AM). |

### Person / Organisation

| Zeile | Norwegen |
|---|---|
| P1 Benachrichtigung | E-Mail und SMS an KRR-Kontaktdaten, avsenderstyrt (VAR Z. 84–87); zweiter Hinweis nach einer Woche (EFF § 8, Z. 235); Push in der Digipost-App (DP Varslinger). Pflicht liegt bei der Behörde (BV S. 19). |
| P2 Authentifizierung | ID-porten mit MinID, BankID, Buypass, Commfides (IDP); Sicherheitsniveau je Nachricht (`sikkerhetsnivaa`, DPI). |
| P3 Organisationszugang | Altinn: Rollen aus dem Enhetsregister, Zugriffspakete; vertrauliche Post nur mit ausdrücklicher Delegation (AA; TP). |
| P4 Vertretung | Bevollmächtigte mit eigener eID (EFF § 10, Z. 291); Altinn-Vollmachten (AA); Digipost: Zugriff für andere, Verge, Fremtidsfullmakt (DP Del postkasse). |
| P5 Clients | Digipost oder e-Boks, Web und App (DP; EB); Organisationen über Altinn. |
| P6 Anbindung eigener Systeme | Altinn-API über Maskinporten (AM; AA). |
| P7 Antwort und Erstkontakt | Kein Rückkanal im DPI-Postfach; 2-Wege-Antwort nur „dersom funskjonaliteten vedtas og utvikles“ (NYI Z. 355–357). Link zum Dienst der Behörde (UTV); Eingaben auf dem angewiesenen Weg (EFF § 3; FVL § 15 a). |
| P8 Inhalt an die Behörde | Nicht vorgesehen (siehe P7). |
| P9 Postfach und Nutzerkonto | Postfach beim gewählten Anbieter je Fødselsnummer, Login ID-porten (DPK Z. 65; KRR). |

### Übermittlung

| Zeile | Norwegen |
|---|---|
| Ü1 Modus | Asynchroner digitaler Brief (DPK). |
| Ü2 Vertraulichkeit | CMS-Verschlüsselung an das Zertifikat aus dem KRR, „kryptert frem til postkassen“ (SIK Z. 84; DPK Z. 65). Keine Ende-zu-Ende-Verschlüsselung bis zur Person **[Indiz]**: SA PDF-S. 30 (2015) erlaubt Zertifikate je Person, nennt aber, dass „begge postkasseleverandørene valgt å beskytte posten med eget virksomhetssertifikat“; aktueller Stand nicht belegt. Digipost: verschlüsselt „når de sendes og der de lagres“ (DP Sikkerhet Z. 32). |
| Ü3 Echtheit | ASiC-E mit XAdES über das Unternehmenszertifikat der Behörde (SIK Z. 83). |
| Ü4 Ereignisse | Lieferquittung, Benachrichtigungsfehler, Mottak, Rücksendung (NYI Z. 340); Öffnungsquittung nur auf Anforderung und mit Zustimmung (KV); Zugriffszeitpunkt (EFF § 8). |
| Ü5 Interoperabilität | Peppol / CEF eDelivery, AS4 zwischen Zugangspunkten (NYI Z. 107, 175–179). |
| Ü6 Malware-Prüfung | Formate begrenzt (MIME). Malware-Prüfung **[Annahme]**: Fehlanzeige in DPK, SIK, MIME, DP, BV; SA erwähnt Virenkontrolle nur als Infrastruktur der Absender. |

### Infrastruktur und Betrieb

| Zeile | Norwegen |
|---|---|
| I1 Betrieb und Governance | Digdir verwaltet DPI und hat Verträge mit Digipost (Posten Norge) und e-Boks (BV S. 1, 18–19); Person wählt einen Anbieter; Altinn für Organisationen. |
| I2 Datenhaltung | Bei den Postfachanbietern, „så lenge innbygger ønsker“ (BV S. 19; DP Lagring). |
| I3 Betriebsmodell | Behörden mit eFormidling-Integrationspunkt oder Datenverarbeiter (Delegation über Altinn, NYI Z. 207), kein eigenes Postfach-Backend. |
| I4 Funktionspostfächer | Keine Ende-zu-Ende-Verschlüsselung **[Indiz]** (wie Ü2). |
| I5 Aufnahme | Behörde nimmt die Bruksvilkår an (Vertrag AV), Digdir gibt Maskinporten-Scope (NYI Z. 203); Digdir hat Verträge mit den Postfachanbietern (BV S. 18–19). |

### Recht

| Zeile | Norwegen |
|---|---|
| R1 Schriftformersatz | Elektronische Nachricht schriftlich, wenn dauerhaft verfügbar (FVL § 2 Buchst. g, § 15 a). |
| R2 Teilnahmepflicht | Personen digital mit Opt-out über KRR, Unternehmen ohne (EFF § 9, Z. 267). |
| R3 Bekanntgabe | Bereitstellung mit Hinweis, zweiter Hinweis nach einer Woche (EFF § 8); Klagefrist ab Bereitstellung und Hinweis (EFF § 11). Allgemeiner Zugangszeitpunkt **[Indiz]**: in EFF und FVL nicht geregelt; das neue Forvaltningsgesetz regelt in § 63 Abs. 1, dass die Nachricht mit dem Hinweis als zugegangen gilt (FVL25 Z. 1599), ist aber nicht in Kraft (§ 99). |
| R4 Teilnahme der Behörden | Staatliche Behörden per Digitalisierungsrundschreiben verpflichtet **[Indiz]**: RS19 PDF-S. 8 Punkt 1.6 (Stand 12/2019, also nach dem Difi-Bericht beibehalten); EF Vedtak Z. 100 zitiert die Pflicht heute; aktuelle Fassung auf regjeringen.no nicht abrufbar. |

## 4. Widersprüche und offene Punkte

- **Sentralforvalter:** AKT und KV nennen die Rolle ohne Träger; BV beschreibt Digdir als Verwalter, Datenverarbeiter und Vertragspartner der Anbieter, verwendet für DPI aber nicht das Wort „sentralforvalter“ (nur für eSignering).
- **Rundschreiben:** Difi empfahl 2019, die Pflicht zu streichen (RS); die Fassung vom 20.12.2019 behält sie (RS19). Nummerierung schwankt (1.6, 1.7, später 1.8 / 1.10 laut Suchtreffern).
- **Quittungen:** KV (Henting av kvittering) beschreibt noch den Meldingsformidler der alten Infrastruktur.
- **Zugang:** Neues Forvaltningsgesetz mit Zugangsfiktion beschlossen, Inkrafttreten offen.

## 5. Lücken

- Nicht gefunden: Verträge Digdir–Digipost / e-Boks im Volltext, aktuelle Beschaffungsunterlagen, aktuelle Sicherheitsdokumentation zur Entschlüsselung beim Anbieter, Aussage zur Malware-Prüfung, Nutzungsbedingungen von e-Boks Norge (nur Startseite geprüft).
- Nicht abrufbar: regjeringen.no (HTTP 403: aktuelles Digitaliseringsrundskrivet, Høringsnotat eForvaltningsforskriften, Tildelingsbrev Digdir 2025); Sicherheitsarchitektur auf docs.digdir.no (404, Spiegel auf digipost.github.io genutzt).

## 6. Verwandte lokale Quellen

- `../Referenzen-Digital-Postkasse-Norwegen.md`: Quellenliste mit Evidenzstufen.
- `../MUK/MUK-Zusammenfassung.md`: gleiche Struktur für das deutsche Organisationskonto.
