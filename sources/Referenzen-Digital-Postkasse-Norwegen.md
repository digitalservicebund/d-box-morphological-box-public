# Digital postkasse til innbyggere (Norwegen): Quellen

Nachweise für die Spalte `Norwegen` in `existing_solutions.csv` und die Norwegen-Einträge in `morph_box.csv`. Statt `[unverifiziert]` stehen hier und in den Norwegen-Einträgen der CSV-Dateien `[Indiz]` (Stufe 2) und `[Annahme]` (Stufe 1); unmarkierte Aussagen sind Stufe 3. `Fehlanzeige` heißt: in den Quellen gesucht, nicht gefunden.

Evidenzstufen (Spalte `Stufe`):

- **3 (Belegt / Primärquelle):** Gesetzestext, Regierungsvorgabe oder Betreiberdokument (Digdir, Altinn, Postfachanbieter), online im Volltext geprüft.
- **2 (Sekundär / Indiz):** Herstelleraussage, Pressemitteilung, Suchtreffer ohne geprüften Volltext, veraltete Angabe oder Rückschluss aus einer Primärquelle; `Fehlanzeige` bei gezielter Suche zählt als Indiz für eine negative Aussage.
- **1 (Annahme / Hypothese):** kein Beleg, nicht einmal ein Indiz; offen und zu prüfen.

Bei gemischten Zeilen steht zuerst die Stufe des belegten Teils, danach die Stufe des Teils, der in der Aussage mit `[Indiz]` oder `[Annahme]` markiert ist.

Stand: 2026-11-23. Erstmals abgerufen am 2026-11-17, Volltextprüfung der Online-Fassungen am 2026-11-23; nur öffentlich zugängliche Quellen. Lokale Kopien mit Index je Ordner seit 2026-11-23 in `Digital-Postkasse-Norwegen/quellen/` (Spalte `Quelle`, „lokal:“), Zusammenfassung in `Digital-Postkasse-Norwegen/Digital-Postkasse-Norwegen-Zusammenfassung.md`.

## Hauptquellen

| Kürzel | Quelle | Stufe |
|---|---|---|
| EFF | Forskrift om elektronisk kommunikasjon med og i forvaltningen (eForvaltningsforskriften), FOR-2004-06-25-988 (§§ 3, 8–11, 29–34), Lovdata: <https://lovdata.no/dokument/SF/forskrift/2004-06-25-988>; lokal: `Digital-Postkasse-Norwegen/quellen/recht-governance/eForvaltningsforskriften_FOR-2004-06-25-988_lovdata_2026-11-23.md` | 3 |
| FVL | Lov om behandlingsmåten i forvaltningssaker (forvaltningsloven), LOV-1967-02-10 (§ 2 Buchst. g, § 15 a), Lovdata: <https://lovdata.no/dokument/NL/lov/1967-02-10>; lokal: `Digital-Postkasse-Norwegen/quellen/recht-governance/forvaltningsloven_LOV-1967-02-10_lovdata_2026-11-23.md` | 3 (Aufhebung durch neues Forvaltningsgesetz, LOV-2025-06-20-81, beschlossen, Inkrafttreten offen) |
| DPK | Digdir, Digital postkasse til innbyggere: <https://docs.digdir.no/docs/digitalpostkasse/>; lokal: `Digital-Postkasse-Norwegen/quellen/betreiber/digdir_digital-postkasse-til-innbyggere.md` | 3 |
| NYI | Digdir, DPI Ny infrastruktur (Peppol, 4-Ecken-Modell, REST-API, Maskinporten): <https://docs.digdir.no/dpi_nyinfrastruktur.html>; lokal: `Digital-Postkasse-Norwegen/quellen/anbindung-technik/digdir_dpi_ny-infrastruktur.md` | 3 |
| AKT | Digdir, DPI Aktører og roller: <https://docs.digdir.no/dpi_aktorer.html>; lokal: `Digital-Postkasse-Norwegen/quellen/betreiber/digdir_dpi_aktorer-og-roller.md` | 3 (nennt Rollen, aber nicht Digdir als Sentralforvalter) |
| TJL | Digdir, DPI Tjenesteleverandører: <https://docs.digdir.no/dpi_tjenesteleverandorer.html>; lokal: `Digital-Postkasse-Norwegen/quellen/betreiber/digdir_dpi_tjenesteleverandorer.md` | 3 |
| SIK | Digdir, DPI Dokumentpakke, sikkerhet (ASiC-E, XAdES, CMS): <https://docs.digdir.no/dpi_dokumentpakke_sikkerhet.html>; lokal: `Digital-Postkasse-Norwegen/quellen/anbindung-technik/digdir_dpi_dokumentpakke-sikkerhet.md` | 3 |
| MIME | Digdir, DPI Dokumentpakke, tillatte MIME-typer: <https://docs.digdir.no/dpi_mimetypes.html>; lokal: `Digital-Postkasse-Norwegen/quellen/anbindung-technik/digdir_dpi_mimetyper.md` | 3 |
| VAR | Digdir, DPI Varsling: <https://docs.digdir.no/dpi_varsling.html>; lokal: `Digital-Postkasse-Norwegen/quellen/anbindung-technik/digdir_dpi_varsling.md` | 3 |
| UTS | Digdir, DPI Utskrift: <https://docs.digdir.no/dpi_utskrift_index.html>; lokal: `Digital-Postkasse-Norwegen/quellen/anbindung-technik/digdir_dpi_utskrift.md` | 3 |
| KV | Digdir, SDP Henting av kvittering und AapningsKvittering: <https://docs.digdir.no/resources/begrep/sikkerDigitalPost/forretningslag/forretningsprosess_kvittering>, <https://docs.digdir.no/resources/begrep/sikkerDigitalPost/meldinger/AapningsKvittering>; Aapningskvittering der neuen Infrastruktur: <https://docs.digdir.no/dpi_aapningskvittering.html>; lokal: `Digital-Postkasse-Norwegen/quellen/anbindung-technik/digdir_sdp_forretningsprosess-kvittering.md`, `…_sdp_aapningskvittering.md`, `…_dpi_aapningskvittering.md` | 3 (Henting av kvittering beschreibt noch den Meldingsformidler der alten Infrastruktur; Öffnungsquittung in der neuen Infrastruktur bestätigt) |
| DPI | Digdir, SDP DigitalPostInfo (Feld `sikkerhetsnivaa`) und JSON-Schema der neuen Infrastruktur: <https://docs.digdir.no/resources/begrep/sikkerDigitalPost/begrep/DigitalPostInfo>, <https://docs.digdir.no/schemas/dpi/innbyggerpost_dpi_digital_1_0.schema.json>; lokal: `Digital-Postkasse-Norwegen/quellen/anbindung-technik/digdir_sdp_digitalpostinfo.md`, `…_dpi_innbyggerpost_dpi_digital_1_0.schema.json` | 3 |
| UTV | Digdir, SDP-Utvidelser Lenke und Arrangement: <https://docs.digdir.no/resources/begrep/sikkerDigitalPost/forretningslag/Utvidelser/Lenke>, <https://docs.digdir.no/resources/begrep/sikkerDigitalPost/forretningslag/Utvidelser/Arrangement>; lokal: `Digital-Postkasse-Norwegen/quellen/anbindung-technik/digdir_sdp_utvidelse-lenke.md`, `…_sdp_utvidelse-arrangement.md` | 3 |
| KRR | Digdir, Oppslagstjenesten for Kontakt- og reservasjonsregisteret: <https://docs.digdir.no/docs/Kontaktregisteret/oppslagstjenesten_rest>; lokal: `Digital-Postkasse-Norwegen/quellen/anbindung-technik/digdir_krr_oppslagstjenesten-rest.md` | 3 |
| EF | Digdir, eFormidling Introduksjon und Vedtak til innbygger: <https://docs.digdir.no/docs/eFormidling/Introduksjon/>, <https://docs.digdir.no/docs/eFormidling/Funksjonalitet/vedtak_til_innbygger>; lokal: `Digital-Postkasse-Norwegen/quellen/anbindung-technik/digdir_eformidling_introduksjon.md`, `…_eformidling_vedtak-til-innbygger.md` | 3 (Vedtak til innbygger), 2 (Introduksjon nicht im Volltext geprüft) |
| IDP | Digdir, ID-porten overordnet: <https://docs.digdir.no/docs/idporten/idporten/idporten_overordnet>; lokal: `Digital-Postkasse-Norwegen/quellen/betreiber/digdir_idporten_overordnet.md` | 3 |
| AM | Altinn, Melding (Correspondence), Hva får du?: <https://docs.altinn.studio/nb/correspondence/what-do-you-get/>; lokal: `Digital-Postkasse-Norwegen/quellen/betreiber/altinn_correspondence_hva-far-du.md` | 3 |
| AA | Altinn, Autorisasjon, Hva får du?: <https://docs.altinn.studio/nb/authorization/what-do-you-get/>; lokal: `Digital-Postkasse-Norwegen/quellen/betreiber/altinn_authorization_hva-far-du.md` | 3 |
| DP | Digipost, Hilfe: Zugriff für andere <https://www.digipost.no/privat/hjelp-og-kontakt/del-postkasse>, Benachrichtigungen <https://www.digipost.no/privat/hjelp-og-kontakt/varslinger>, Speicherung <https://www.digipost.no/privat/hjelp-og-kontakt/lagring>, Sicherheit <https://www.digipost.no/privat/hjelp-og-kontakt/sikkerhet>; lokal: `Digital-Postkasse-Norwegen/quellen/betreiber/digipost_hjelp_*.md` | 3 |
| EB | e-Boks Norge: <https://private.e-boks.com/norge/nb>; lokal: `Digital-Postkasse-Norwegen/quellen/betreiber/e-boks_norge_startseite.md` | 3 (lokale Kopie der Startseite im Volltext geprüft: Postfach für Post vom Staat, Login über ID-porten, App) |
| TP | Altinn, Melding (Correspondence), Taushetsbelagt post: <https://docs.altinn.studio/en/correspondence/explanation/taushetsbelagt-post/>; lokal: `Digital-Postkasse-Norwegen/quellen/betreiber/altinn_correspondence_taushetsbelagt-post.md` | 3 |
| RS | Kommunal- og distriktsdepartementet / Digitaliseringsdepartementet, Digitaliseringsrundskrivet (aktuelle Fassung und Vorgänger, Abschnitt „Bruk digital postkasse til innbyggere“): <https://www.regjeringen.no/no/dokumenter/digitaliseringsrundskrivet/id3025117/>; Difi-Rapport 2019:6 zur Pflicht nach Abschnitt 1.7: <https://digdir.no/media/530/download>; lokal: `Digital-Postkasse-Norwegen/quellen/recht-governance/difi-rapport_2019-6_digital-post-krav-digitaliseringsrundskrivet.pdf` (Rundschreiben nicht lokal) | 2 (aktuelle Fassung nicht abrufbar, HTTP 403; Difi-Bericht im Volltext geprüft, Stand 2019, veraltet) |
| RS19 | Kommunal- og moderniseringsdepartementet, Digitaliseringsrundskrivet vom 20.12.2019, Punkt 1.6 „Bruk digital postkasse til innbyggere“: <https://digdir.no/media/632/download>; lokal: `Digital-Postkasse-Norwegen/quellen/recht-governance/kmd_digitaliseringsrundskrivet_2019-12-20.pdf` (PDF-S. 8) | 2 (im Volltext geprüft, veraltet) |
| BV | Digdir, Bruksvilkår for offentlige kunder, Kap. 1 und 6 Digital postkasse til innbyggere (PDF vom 14.11.2025): <https://samarbeid.digdir.no/media/2134/download>; lokal: `Digital-Postkasse-Norwegen/quellen/recht-governance/digdir_bruksvilkar-offentlige-kunder_2025-11.pdf` (S. 1, 18–20) | 3 |
| AV | Digdir, Avtalestruktur / Avtale om bruk av Digdirs fellesløsninger (DOCX): <https://samarbeid.digdir.no/media/92/download>; lokal: `Digital-Postkasse-Norwegen/quellen/recht-governance/digdir_avtale-om-bruk-felleslosninger.md` | 3 |
| FVL25 | Lov om saksbehandlingen i offentlig forvaltning (forvaltningsloven), LOV-2025-06-20-81 (§ 10 Abs. 3, § 63 Abs. 1, § 99), Lovdata: <https://lovdata.no/dokument/NL/lov/2025-06-20-81>; lokal: `Digital-Postkasse-Norwegen/quellen/recht-governance/forvaltningsloven_LOV-2025-06-20-81_lovdata_2026-11-23.md` | 3 (beschlossen, nicht in Kraft) |
| SA | Difi, Sikkerhetsarkitektur Digital postkasse til innbyggere, 12.03.2015 (Spiegel): <https://digipost.github.io/felleslosninger/resources/begrep/sikkerDigitalPost/innledning/Sikkerhetsarkitektur.pdf>; lokal: `Digital-Postkasse-Norwegen/quellen/betreiber/difi_dpi_sikkerhetsarkitektur_2015-03-12.pdf` (PDF-S. 30) | 2 (im Volltext geprüft, veraltet) |
| QA | Digdir, Spørsmål og svar til webinaret „Hvordan ivaretas informasjonssikkerhet og personvern i DPI“, 2021: <https://www.digdir.no/media/1776/download>; lokal: `Digital-Postkasse-Norwegen/quellen/betreiber/digdir_dpi_webinar-qa-informasjonssikkerhet_2021-06.pdf` | 2 (Webinar-Antworten, Stand 2021) |

## Nachweise je Dimension

| Zeile | Aussage | Quellen | Stufe |
|---|---|---|---|
| B1 | Behörden senden über eFormidling-Integrationspunkt oder Klientbibliotheken per REST an den Zugangspunktanbieter (Peppol); eFormidling ermittelt den Kanal DPI, Altinn oder Papier | NYI; EF; KRR | 3 |
| B2 | Druckdienst im DPI, Anbieter Skatteetaten; Papier aus demselben Versand | UTS; TJL | 3 |
| B2 | ohne Postfach entscheidet die Behörde zwischen Altinn Digital Post und Papier | EF | 3 |
| B3 | Adressierung über Fødselsnummer; KRR liefert Postfachanbieter, Zertifikat, Kontaktdaten und Reservierung | KRR; EFF §§ 29–31 | 3 |
| B4 | Hauptdokument und Anhänge in zulässigen Formaten, u. a. PDF, HTML | MIME | 3 |
| B4 | strukturierte Erweiterungen, z. B. Termin (Arrangement), Link mit Frist | UTV; MIME | 3 |
| B4 | Altinn: Text, Anhänge, Links | AM | 3 |
| P1 | Benachrichtigung per E-Mail und SMS an KRR-Kontaktdaten, nur solange ungeöffnet; zweiter Hinweis nach einer Woche | VAR; EFF § 8 | 3 |
| P1 | Push in der Digipost-App | DP (Varslinger) | 3 |
| P1 | Papier bei Reservierung oder ohne gültige Kontaktdaten | EF; EFF §§ 9, 32 | 3 |
| P2 | ID-porten mit MinID, BankID, Buypass, Commfides; Sicherheitsniveau zum Öffnen je Nachricht | IDP; DPI (Pflichtfeld `sikkerhetsnivaa` je Nachricht) | 3 |
| P3 | Altinn: Rollen aus dem Enhetsregister, Delegation über Zugriffspakete; vertrauliche Post nur mit ausdrücklicher Delegation | TP; AA | 3 |
| P4 | Bevollmächtigte handeln mit eigener eID, Vertretung muss aus der Kommunikation hervorgehen | EFF § 10 | 3 |
| P4 | Altinn-Vollmachten und Vergemål | AA | 3 |
| P4 | Digipost: Zugriff für andere Personen | DP (Del postkasse: Lesen und Senden für andere, Verge, Fremtidsfullmakt) | 3 |
| P5 | Person wählt Postfachanbieter Digipost oder e-Boks | DPK; TJL | 3 |
| P5 | Organisationen nutzen die Altinn-Oberfläche | AM | 3 |
| P6 | Altinn-API für Fach- und Unternehmenssoftware über Maskinporten | AM; AA | 3 |
| P7 | Link aus der Nachricht zum Dienst der Behörde, z. B. für eine Antwort | UTV | 3 |
| P7 | Eingaben elektronisch auf dem von der Behörde angewiesenen Weg | EFF § 3; FVL § 15 a | 3 |
| P7, P8 | kein Rückkanal im DPI-Postfach an Behörden dokumentiert; Beschreibung einer 2-Wege-Antwort nur angekündigt, „dersom funksjonaliteten vedtas og utvikles“ | NYI; DPK | 3 |
| P9 | Postfach beim gewählten Anbieter je Fødselsnummer, Zugang über ID-porten | DPK; KRR | 3 |
| Ü1 | asynchroner digitaler Brief | DPK | 3 |
| Ü2 | Dokumentpaket per CMS mit Einmalschlüssel verschlüsselt, Schlüssel mit dem Zertifikat aus dem KRR, das dem Postfachanbieter gehört; verschlüsselt bis zum Postfach, Zugangspunktanbieter sehen den Inhalt nicht; keine Ende-zu-Ende-Verschlüsselung bis zur Person [Indiz] | SIK; KRR; NYI; DPK; SA | 3, keine Ende-zu-Ende-Verschlüsselung: 2 |
| Ü3 | ASiC-E mit XAdES-Signatur über das Unternehmenszertifikat der Behörde | SIK | 3 |
| Ü4 | Quittungen für Lieferung, Benachrichtigungsfehler, Druck und Rücksendung | KV; UTS; NYI | 3 |
| Ü4 | Öffnungsquittung nur, wenn die Behörde sie anfordert und die Person zustimmt | KV (Aapningskvittering der neuen Infrastruktur); DPI | 3 |
| Ü4 | System speichert Zugriffszeitpunkt | EFF § 8 | 3 |
| Ü5 | DPI-Transport über Peppol / CEF eDelivery mit Zugangspunktanbietern, AS4 zwischen den Zugangspunkten | NYI | 3 |
| Ü6 | zulässige Dateiformate begrenzt | MIME | 3 |
| Ü6 | Malware-Prüfung [Annahme] | Fehlanzeige in DPK, SIK, MIME | 1 |
| I1 | Digdir als zentrale Verwaltung (Sentralforvalter); private Postfachanbieter Digipost und e-Boks | AKT; TJL; EFF § 30; BV (Kap. 1.1: Digdir „forvalter“ die Fellesløsninger; Kap. 6.1, 6.4.1: Verträge mit den Postfachanbietern, Posten Norge AS und e-Boks A/S als Unterauftragnehmer) | 3 |
| I2 | Speicherung bei den Postfachanbietern, Langzeitspeicherung nach Wahl der Person; Organisationspost in Altinn | AKT; DP (Lagring: bis 1 GB „så lenge du vil“); AM | 3 |
| I3 | Behörden installieren den eFormidling-Integrationspunkt oder senden über Datenverarbeiter, kein eigenes Postfach-Backend | EF; AKT | 3 |
| I4 | keine Ende-zu-Ende-Verschlüsselung [Indiz] | SIK; DPK; SA | 2 |
| I5 | Verträge von Digdir mit Behörden und Postfachanbietern | BV (Kap. 6.1: Anbieter, „som Digdir har avtale med“; Kap. 6.4.1: „avtaler mellom Digdir og leverandørene“); AV (Vertrag, den die Behörde vor Nutzung unterschreibt); NYI (Maskinporten-Scope erst nach Annahme der Bruksvilkår); RS (Difi-Bericht 2019: Verträge von 2014) | 3 |
| R1 | elektronische Nachricht gilt als schriftlich, wenn dauerhaft verfügbar; elektronische Kommunikation in beide Richtungen zulässig | FVL § 2 Buchst. g, § 15 a | 3 |
| R2 | Personen mit Opt-out über KRR; Unternehmen ohne Opt-out | EFF § 9 | 3 |
| R3 | Bereitstellung mit Hinweis an KRR-Kontaktdaten, zweiter Hinweis nach einer Woche | EFF § 8 | 3 |
| R3 | Klagefrist läuft ab Bereitstellung und Hinweis; allgemeiner Zugangszeitpunkt [Indiz] | EFF § 11; Zugangszeitpunkt: Fehlanzeige in EFF und FVL; FVL25 § 63 nicht in Kraft | 3, Zugangszeitpunkt: 2 |
| R4 | staatliche Behörden per Digitalisierungsrundschreiben verpflichtet [Indiz] | RS; RS19 (Punkt 1.6, Pflicht nach dem Difi-Bericht beibehalten); EF (zitiert die Pflicht) | 2 |

## Evidenzstufen der markierten Stellen

Einordnung der früheren `[unverifiziert]`-Stellen sowie der Stellen, die bei der Volltextprüfung nur als Indiz bestätigt wurden, Stand 2026-11-23. Stufe 1 und 2 werden über den genannten Prüfweg geprüft.

| Datei, Zeile | Aussage | Stufe | Begründung | Prüfweg |
|---|---|---|---|---|
| `existing_solutions.csv`, P7 | kein Rückkanal im DPI-Postfach dokumentiert | 3 | NYI: Beschreibung einer 2-Wege-Antwort folgt erst, wenn die Funktion beschlossen und entwickelt ist; DPK beschreibt nur den Empfang | – |
| `existing_solutions.csv`, Ü2 | keine Ende-zu-Ende-Verschlüsselung bis zur Person | 2 | Rückschluss (früher „abgeleitet“): DPK „kryptert frem til postkassen“, SIK verschlüsselt an das Zertifikat aus dem KRR; Entschlüsselung beim Postfachanbieter nicht ausdrücklich beschrieben; SA (2015, veraltet) erlaubt Zertifikate je Person, beide Anbieter nutzten aber ihr Unternehmenszertifikat; bisher unmarkiert | Anfrage Digdir; Sicherheitsdokumentation der Postfachanbieter |
| `existing_solutions.csv`, Ü6 | Malware-Prüfung | 1 | Fehlanzeige in DPK, SIK und MIME; Richtung der Aussage offen | Anfrage Digdir / Digipost / e-Boks |
| `existing_solutions.csv`, I4 | keine Ende-zu-Ende-Verschlüsselung | 2 | wie Ü2 | wie Ü2 |
| `existing_solutions.csv`, R3 | Zugangszeitpunkt | 2 | EFF § 11 lässt die Klagefrist ab Bereitstellung und Hinweis laufen; eine allgemeine Zugangsregel („anses mottatt“) fehlt in EFF und FVL (Fehlanzeige); FVL25 § 63 Abs. 1 regelt den Zugang mit dem Hinweis, ist aber nicht in Kraft | Anfrage Digdir / Digitaliseringsdepartementet; neues Forvaltningsgesetz prüfen |
| `existing_solutions.csv`, R4 | staatliche Behörden per Digitalisierungsrundschreiben verpflichtet | 2 | Rundschreiben auf regjeringen.no nicht abrufbar (HTTP 403); Difi-Rapport 2019:6 (im Volltext geprüft) belegt die Pflicht für staatliche Behörden seit 2014 nach Abschnitt 1.7 und empfiehlt, sie zu streichen; die Fassung vom 20.12.2019 (RS19, Punkt 1.6) behält sie, EF zitiert sie; ob sie in der aktuellen Fassung gilt, ist offen | aktuelle Fassung des Rundschreibens im Volltext |
| `morph_box.csv`, Ü2 Ausprägung 1 | keine Ende-zu-Ende-Verschlüsselung (Norwegen) | 2 | wie Ü2 oben | wie Ü2 oben |
| `morph_box.csv`, I4 Ausprägung 5 | keine Ende-zu-Ende-Verschlüsselung (Norwegen) | 2 | wie Ü2 oben | wie Ü2 oben |
| `morph_box.csv`, R3 Ausprägung 1 | Zugangszeitpunkt | 2 | wie R3 oben | wie R3 oben |
| `morph_box.csv`, R4 Ausprägung 3 | Digitalisierungsrundschreiben | 2 | wie R4 oben | wie R4 oben |
