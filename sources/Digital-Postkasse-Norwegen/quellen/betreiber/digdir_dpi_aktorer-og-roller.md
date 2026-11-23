# Digdir: DPI Aktører og roller

Quelle: https://docs.digdir.no/dpi_aktorer.html
Abgerufen: 2026-11-23

---

Digital postkasse

[Kollaps](#) \| [Utvid](#) \| [Hjem](https://docs.digdir.no/index.html)

- [Oversikt]()
  - [Innledning](/dpi_oversikt_index.html)
  - [Transportinfrastruktur](/dpi_nyinfrastruktur.html)
  - [Arkitektur](/dpi_arkitektur.html)
  - [Sikkerhet](/dpi_sikkerhet_index.html)
  - [Sertifikathåndtering](/dpi_sertifikathandtering.html)
  - [Integrasjon](/integrasjon.html)
  - [Prosesser og dokumenttyper](/identifiers.html)
  - [Schema og XSD](/dpi_skjema.html)
  - [Eksempler](/dpi_eksempler.html)
  - [Tjenesteleverandører](/dpi_tjenesteleverandorer.html)
- [Forretningslag]()
  - [Forretningslag](/dpi_forretningslag_index.html)
  - [Aktører og Roller](/dpi_aktorer.html)
  - [Varsling](/dpi_varsling.html)
  - [Sending av innbyggerpost](/dpi_sending_av_digital_post.html)
  - [Sending av innbyggerpost på vegne av andre](/dpi_send_paa_vegne_av_andre.html)
  - [Struktur på melding](/dpi_struktur_forretningsmelding.html)
- [Dokumentpakke (ASiC)]()
  - [Introduksjon](/dpi_dokumentpakke_index.html)
  - [Langtidslagring](/dpi_langtidslagring.html)
  - [Manifest](/dpi_manifest.html)
  - [Dokumentformater](/dpi_dokumentformat.html)
  - [MIME type](/dpi_mimetypes.html)
  - [HTML](/dpi_html.html)
  - [Sikkerhet](/dpi_dokumentpakke_sikkerhet.html)
- [Utvidelser]()
  - [Introduksjon](/dpi_utvidelser_index.html)
  - [Lenke utenfor brev](/dpi_lenke.html)
  - [Bevis](/dpi_bevis.html)
- [Utskrift og forsendelse]()
  - [Utskriftstjenesten](/dpi_utskrift_index.html)
  - [Krav og anbefalinger ved adressering](/dpi_adressering_index.html)
- [Forretningsmeldinger]()
  - [Forretningsmeldinger](/dpi_forretiningsmelding_index.html)
  - [Digital](/dpi_digital.html)
  - [Utskrift](/dpi_utskrift.html)
  - [LeveringsKvittering](/dpi_leveringskvittering.html)
  - [AapningsKvittering](/dpi_aapningskvittering.html)
  - [VarslingfeiletKvittering](/dpi_varslingfeiletkvittering.html)
  - [MottaksKvittering](/dpi_mottakskvittering.html)
  - [ReturpostKvittering](/dpi_returpostkvittering.html)
  - [Feil](/dpi_feil.html)
  - [FlyttetDigitalpost](/dpi_flyttetdigitalpost.html)
- [Standard Business Document]()
  - [Standard Business Document](/dpi_sbd_index.html)
  - [StandardBusinessDocumentHeader](/dpi_sbdh.html)
  - [Sender](/dpi_sender.html)
  - [Receiver](/dpi_receiver.html)
  - [DocumentIdentification](/dpi_documentidentification.html)
  - [BusinessScope](/dpi_businessscope.html)
- [Feilhåndtering]()
  - [Feilhåndtering](/dpi_feilhandtering_index.html)
  - [Forvaltning og tekniske feil](/dpi_forvaltning.html)
  - [Transportfeil](/dpi_transportfeil.html)
  - [Forretningsfeil](/dpi_forretningsfeil.html)
- [Testing]()
  - [Postkasser til testbrukere](/dpi_testing_index.html)
  - [Integrasjonstest](/dpi_integrasjonstest.html)
  - [Testmiljø](/dpi_testmiljo.html)
- [Begrep]()
  - [Begrepskatalog](/resources/begrep/sikkerDigitalPost/begrep/)

------------------------------------------------------------------------

[Hjem](https://docs.digdir.no)  \>  resources  \>  begrep  \>  sikkerDigitalPost  \>  nyinf  \>  forretningslag

# Aktører og roller

## Aktør

Tabellen under inneholder en oversikt over hovedaktørene i prosessen med å utveksle innbyggerpost som DPI-meldinger:

| Aktør                            | Beskrivelse                                                                                                                                                    |
|----------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Offentlig virksomhet             | Offentlig virksomhet eller annen virksomhet som utfører en offentlig oppgave på vegne av en offentlig virksomhet.                                              |
| Tilbyder av aksesspunkttjenester | Peppol-sertifisert Service Provider (offentlig eller privat) som tilbyr aksesspunkttjenester for sending via Peppol.                                           |
| Tilbyder av postkassetjeneste    | Virksomhet (offentlig eller privat) som tilbyr postkassetjenester, enten direkte eller via underleverandører.                                                  |
| Tilbyder av utskriftstjeneste    | Virksomhet (offentlig eller privat) som tilbyr utskriftstjeneste, enten direkte eller via underleverandører.                                                   |
| Sentralforvalter                 | Ansvarlig for forvaltning/kontroll av forholdet mellom offentlig virksomhet, mottaker og tilbydere av aksesspunkttjeneste/postkassetjeneste/utskriftstjeneste. |
| Innbygger                        | Mottaker av innbyggerpost.                                                                                                                                     |

Begrepet **Ansvarlig enhet** har også blitt brukt:

\- **Ansvarlig enhet** brukes av **Behandlingsansvarlig** virksomheter med et stort virkeområder som ønsker å differensiere Avsendere til Innbygger avhengig av de interne enhetene som sender posten, i de tilfeller der det ikke er mulig eller lite hensiktsmessig at interne enheten selv er identifisert som behandlingsansvarlig (ved å ha et eget sertifikat).  
**Ansvarlig enhet** blir satt av **Behandlingsansvarlig** ved bruk av en identifikator. Dette gjøres som en del av klargjøring av innbyggerpost.  
Eksempel: Behandlingsansvarlig er NAV og Ansvarlig enhet er NAV Hjelpemiddelsentral Oslo og Akershus.

## Roller

| Rolle                                             | Beskrivelse                                                                                                                                                                                                                                                                                                                                           |
|---------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Avsender\[1\]                                     | Offentlig virksomhet som skal sende innbyggerpost.                                                                                                                                                                                                                                                                                                    |
| Behandlingsansvarlig\[1\]                         | Offentlig virksomhet som produserer innbyggerpost som skal fomidles. Vil i de aller fleste tilfeller være synonymt med Avsender.                                                                                                                                                                                                                      |
| Databehandler\[1\]                                | Virksomhet (offentlig eller privat) som har en kontraktfestet avtale med Behandlingsansvarlig med formål å dekke hele eller deler av prosessen med å formidle innbyggerpost fra Behandlingsansvarlig til avsenders Service Provider. Det kan være flere databehandlere som har ansvar for forskjellige steg i prosessen med å formidle innbyggerpost. |
| Service Provider for avsender i hjørne1           | Avsenders Peppol-sertifiserte avtalepart for mottak og formidling av innbyggerpost formidlet gjennom Peppol.                                                                                                                                                                                                                                          |
| Service Provider for tjenesteleverandør i hjørne4 | Tjenesteleverandørs Peppol-sertifiserte avtalepart for mottak og formidling av innbyggerpost formidlet gjennom Peppol.                                                                                                                                                                                                                                |
| Postkasse                                         | Tjenesteleverandør i hjørne4. Ansvarlig for å tilgjengeliggjøre og oppbevare post til mottaker, samt å varsle mottaker når det er kommet ny post., eventuelt: Ansvarlig for print og forsendelse av papir post til mottaker.                                                                                                                          |
| Utskriftstjeneste                                 | Tjenesteleverandør i hjørne4. Ansvarlig for utskrift på papir og forsendelse av innbyggerpost.                                                                                                                                                                                                                                                        |
| Mottaker                                          | Innbyggeren som er mottaker av innbyggerpost.                                                                                                                                                                                                                                                                                                         |

1.  Avsenderrollen kan være en sammensatt rolle bestående av Avsender, Behandlingsansvarlig og/eller en eller flere databehandlere. For å forenkle dokumentasjonen er denne kompleksiteten skjult bak rollen Avsender, med egne kommentarer i de tilfeller der flere aktører i rollen Avsender medfører utfordringer.
