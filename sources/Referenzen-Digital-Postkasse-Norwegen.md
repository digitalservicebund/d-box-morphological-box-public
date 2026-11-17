# Digital postkasse til innbyggere (Norwegen): Quellen

Nachweise für die Spalte `Norwegen` in `existing_solutions.csv` und die Norwegen-Einträge in `morph_box.csv`. Einträge mit `[unverifiziert]` sind in keiner der Quellen ausdrücklich belegt. Abgerufen am 2026-11-17, nur öffentlich zugängliche Quellen, nur Links, keine lokalen Kopien.

## Hauptquellen

| Kürzel | Quelle |
|---|---|
| EFF | Forskrift om elektronisk kommunikasjon med og i forvaltningen (eForvaltningsforskriften), FOR-2004-06-25-988 (§§ 3, 8–10, 29–34), Lovdata: <https://lovdata.no/dokument/SF/forskrift/2004-06-25-988> |
| FVL | Lov om behandlingsmåten i forvaltningssaker (forvaltningsloven), LOV-1967-02-10 (§ 2 Buchst. g, § 15 a), Lovdata: <https://lovdata.no/dokument/NL/lov/1967-02-10> |
| DPK | Digdir, Digital postkasse til innbyggere: <https://docs.digdir.no/docs/digitalpostkasse/> |
| NYI | Digdir, DPI Ny infrastruktur (Peppol, 4-Ecken-Modell, REST-API, Maskinporten): <https://docs.digdir.no/dpi_nyinfrastruktur.html> |
| AKT | Digdir, DPI Aktører og roller: <https://docs.digdir.no/dpi_aktorer.html> |
| TJL | Digdir, DPI Tjenesteleverandører: <https://docs.digdir.no/dpi_tjenesteleverandorer.html> |
| SIK | Digdir, DPI Dokumentpakke, sikkerhet (ASiC-E, XAdES, CMS): <https://docs.digdir.no/dpi_dokumentpakke_sikkerhet.html> |
| MIME | Digdir, DPI Dokumentpakke, tillatte MIME-typer: <https://docs.digdir.no/dpi_mimetypes.html> |
| VAR | Digdir, DPI Varsling: <https://docs.digdir.no/dpi_varsling.html> |
| UTS | Digdir, DPI Utskrift: <https://docs.digdir.no/dpi_utskrift_index.html> |
| KV | Digdir, SDP Henting av kvittering und AapningsKvittering: <https://docs.digdir.no/resources/begrep/sikkerDigitalPost/forretningslag/forretningsprosess_kvittering>, <https://docs.digdir.no/resources/begrep/sikkerDigitalPost/meldinger/AapningsKvittering> |
| UTV | Digdir, SDP-Utvidelser Lenke und Arrangement: <https://docs.digdir.no/resources/begrep/sikkerDigitalPost/forretningslag/Utvidelser/Lenke>, <https://docs.digdir.no/resources/begrep/sikkerDigitalPost/forretningslag/Utvidelser/Arrangement> |
| KRR | Digdir, Oppslagstjenesten for Kontakt- og reservasjonsregisteret: <https://docs.digdir.no/docs/Kontaktregisteret/oppslagstjenesten_rest> |
| EF | Digdir, eFormidling Introduksjon und Vedtak til innbygger: <https://docs.digdir.no/docs/eFormidling/Introduksjon/>, <https://docs.digdir.no/docs/eFormidling/Funksjonalitet/vedtak_til_innbygger> |
| IDP | Digdir, ID-porten overordnet: <https://docs.digdir.no/docs/idporten/idporten/idporten_overordnet> |
| AM | Altinn, Melding (Correspondence), Hva får du?: <https://docs.altinn.studio/nb/correspondence/what-do-you-get/> |
| AA | Altinn, Autorisasjon, Hva får du?: <https://docs.altinn.studio/nb/authorization/what-do-you-get/> |
| DP | Digipost, Hilfe (Anmeldung, Zugriff für andere, Benachrichtigungen, Speicherung): <https://www.digipost.no/hjelp/> |
| EB | e-Boks Norge: <https://private.e-boks.com/norge/nb> |

## Nachweise je Dimension

| Zeile | Aussage | Quellen |
|---|---|---|
| B1 | Behörden senden über eFormidling-Integrationspunkt oder Klientbibliotheken per REST an den Zugangspunktanbieter (Peppol); eFormidling ermittelt den Kanal DPI, Altinn oder Papier | NYI; EF; KRR |
| B2 | Druckdienst im DPI, Anbieter Skatteetaten; Papier aus demselben Versand | UTS; TJL |
| B2 | ohne Postfach entscheidet die Behörde zwischen Altinn Digital Post und Papier | EF |
| B3 | Adressierung über Fødselsnummer; KRR liefert Postfachanbieter, Zertifikat, Kontaktdaten und Reservierung | KRR; EFF §§ 29–31 |
| B4 | Hauptdokument und Anhänge in zulässigen Formaten, u. a. PDF, HTML | MIME |
| B4 | strukturierte Erweiterungen, z. B. Termin (Arrangement), Link mit Frist | UTV |
| B4 | Altinn: Text, Anhänge, Links | AM |
| P1 | Benachrichtigung per E-Mail und SMS an KRR-Kontaktdaten, nur solange ungeöffnet; zweiter Hinweis nach einer Woche | VAR; EFF § 8 |
| P1 | Push in der Digipost-App | DP |
| P1 | Papier bei Reservierung oder ohne gültige Kontaktdaten | EF; EFF §§ 9, 32 |
| P2 | ID-porten mit MinID, BankID, Buypass, Commfides; Sicherheitsniveau zum Öffnen je Nachricht | IDP; DPK |
| P3 | Altinn: Rollen aus dem Enhetsregister, Delegation über Zugriffspakete; vertrauliche Post nur mit ausdrücklicher Delegation | AM; AA |
| P4 | Bevollmächtigte handeln mit eigener eID, Vertretung muss aus der Kommunikation hervorgehen | EFF § 10 |
| P4 | Altinn-Vollmachten und Vergemål | AA |
| P4 | Digipost: Zugriff für andere Personen | DP |
| P5 | Person wählt Postfachanbieter Digipost oder e-Boks | DPK; TJL |
| P5 | Organisationen nutzen die Altinn-Oberfläche | AM |
| P6 | Altinn-API für Fach- und Unternehmenssoftware über Maskinporten | AM; AA |
| P7 | Link aus der Nachricht zum Dienst der Behörde, z. B. für eine Antwort | UTV |
| P7 | Eingaben elektronisch auf dem von der Behörde angewiesenen Weg | EFF § 3; FVL § 15 a |
| P7, P8 | Rückkanal im DPI-Postfach an Behörden [unverifiziert] | – |
| P9 | Postfach beim gewählten Anbieter je Fødselsnummer, Zugang über ID-porten | DPK; KRR |
| Ü1 | asynchroner digitaler Brief | DPK |
| Ü2 | Dokumentpaket per CMS mit Einmalschlüssel verschlüsselt, Schlüssel mit dem Zertifikat aus dem KRR, das dem Postfachanbieter gehört; Zugangspunktanbieter sehen den Inhalt nicht; keine Ende-zu-Ende-Verschlüsselung bis zur Person (abgeleitet) | SIK; KRR; NYI |
| Ü3 | ASiC-E mit XAdES-Signatur über das Unternehmenszertifikat der Behörde | SIK |
| Ü4 | Quittungen für Lieferung, Benachrichtigungsfehler, Druck und Rücksendung | KV; UTS |
| Ü4 | Öffnungsquittung nur, wenn die Behörde sie anfordert und die Person zustimmt | KV |
| Ü4 | System speichert Zugriffszeitpunkt | EFF § 8 |
| Ü5 | DPI-Transport über Peppol / CEF eDelivery mit Zugangspunktanbietern, AS4 zwischen den Zugangspunkten | NYI |
| Ü6 | zulässige Dateiformate begrenzt | MIME |
| Ü6 | Malware-Prüfung [unverifiziert] | – |
| I1 | Digdir als zentrale Verwaltung (Sentralforvalter); private Postfachanbieter Digipost und e-Boks | AKT; TJL; EFF § 30 |
| I2 | Speicherung bei den Postfachanbietern, Langzeitspeicherung nach Wahl der Person; Organisationspost in Altinn | AKT; DP; AM |
| I3 | Behörden installieren den eFormidling-Integrationspunkt oder senden über Datenverarbeiter, kein eigenes Postfach-Backend | EF; AKT |
| I4 | keine Ende-zu-Ende-Verschlüsselung (abgeleitet) | SIK |
| I5 | Verträge von Digdir mit Behörden und Postfachanbietern | EF; AKT |
| R1 | elektronische Nachricht gilt als schriftlich, wenn dauerhaft verfügbar; elektronische Kommunikation in beide Richtungen zulässig | FVL § 2 Buchst. g, § 15 a |
| R2 | Personen mit Opt-out über KRR; Unternehmen ohne Opt-out | EFF § 9 |
| R3 | Bereitstellung mit Hinweis an KRR-Kontaktdaten, zweiter Hinweis nach einer Woche | EFF § 8 |
| R3 | Zugangszeitpunkt [unverifiziert] | – |
| R4 | staatliche Behörden per Digitalisierungsrundschreiben verpflichtet [unverifiziert] | EF |
