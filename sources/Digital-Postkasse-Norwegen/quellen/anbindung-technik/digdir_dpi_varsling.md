# Digdir: DPI Varsling

Quelle: https://docs.digdir.no/dpi_varsling.html
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

# Varsling

|                         |                                                                                                                                                                                                                                                                                                                                                                                                                                             |
|-------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Innbyggerstyrt varsling | Postkasseleverandørene varsler innbygger i henhold til innbyggers egne varslingspreferanser (varslingsinnstillinger i postkassen).                                                                                                                                                                                                                                                                                                          |
| Avsenderstyrt varsling  | Varslinger en virksomhet bestiller fra postkasseleverandøren. Hos Digipost kommer disse i tillegg til innbyggerstyrt varsling. Hos e-Boks blir innbyggerstyrt varsling overstyrt av avsenderstyrt varsling. OBS! Det er viktig at avsendere unngår bruk av personopplysninger og sensitive opplysninger i [varslingstekst](/resources/begrep/sikkerDigitalPost/begrep/varslingsTekst), da sms og epost-varsling går ukryptert til mottaker. |
| Utsending               | Varslinger i digital postkasse til innbyggere sendes kun på dagtid.                                                                                                                                                                                                                                                                                                                                                                         |
| Kanal                   | Virksomhetene tilknyttet til digital postkassse til innbygger kan bestille varsling på e-post og/eller sms. Hvilken kanal som velges avgjøres både av virksomhetens kanalstrategi og av hvilke opplysninger som finnes i kontakt og reservasjonsregisteret for den enkelte innbygger. Postkasseleverandørene kan ha flere varslingskanaler utover e-post og sms, f.eks. bruk av meldinger direkte til en applikasjon på mobilen.            |
| Kontaktinformasjon      | Avsendere kan varsle innbygger ut fra kontaktinformasjon som innbygger selv har lagt til i [kontakt-og reservasjonsregisteret](/docs/Kontaktregisteret/). Dette kan være epost til oppgitt [epostaddresse](/resources/begrep/oppslagstjenesten/Epostadresse) og/eller sms til oppgitt [mobiltelefonnummer](/resources/begrep/felles/mobiltelefonnummer).                                                                                    |
| Kommentar               | Varslinger i digital postkasse til innbyggere sendes kun til Innbygger dersom innbygger ikke har gjort seg kjent med brevet. Det vil si at varslinger bestilt av virksomheter kun blir sendt Innbygger dersom innbygger ikke har vært inn i postkassen og behandlet brevet.                                                                                                                                                                 |

- Avsenderstyrt varsling settes på meldingsnivå i schema [Digital](https://docs.digdir.no/schemas/dpi/innbyggerpost_dpi_digital_1_0.schema.json) og er en del av en [forretningsmeldingen](https://docs.digdir.no/dpi_skjema.html#forretningsmelding-schema).

- Beskrivelse av [varslingsinnstillinger (med eksempler)](/resources/begrep/sikkerDigitalPost/begrep/Varsler).

### Bruk av lenker (URL) i varsler via SMS og/eller e-post

Grunnet økning i svindel via usikre kanaler, som SMS og e-post, anbefaler Digitaliseringsdirektoratet å unngå all bruk av lenker i varsler. Se innlegg om [temaet](https://samarbeid.digdir.no/digital-postkasse/lenke-eller-ikke-det-er-sporsmalet/1525).
