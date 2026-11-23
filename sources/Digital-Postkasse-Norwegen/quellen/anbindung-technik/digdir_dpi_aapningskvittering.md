# Digdir: DPI Åpningskvittering (neue Infrastruktur)

Quelle: https://docs.digdir.no/dpi_aapningskvittering.html
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

[Hjem](https://docs.digdir.no)  \>  resources  \>  begrep  \>  sikkerDigitalPost  \>  nyinf  \>  forretningsmeldinger

# Aapningskvittering

|               |                                                                                                                                                                                                                                                                                                                                                                                                                           |
|---------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Identifikator |                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Term          | Aapningskvittering                                                                                                                                                                                                                                                                                                                                                                                                        |
| Definisjon    | En [KvitteringsMelding](dpi_kvitteringer.html) til Avsender om at Mottaker har åpnet forsendelsen i sin postkasse.                                                                                                                                                                                                                                                                                                        |
| Kilde         | Digdir                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Kommentar     | Åpningskvitteringer blir bare sendt dersom dette er bestilt av [Avsender](sdp_aktorer.html) i [digital](dpi_digital.html) meldingen ved å spesifisere dette med propertien aapningskvittering. [Mottaker](sdp_aktorer.html) må aksepteres at det sendes en til [Avsender](sdp_aktorer.html) for å få lest den digital posten. Mangel på ÅpningsKvittering betyr at [Mottaker](sdp_aktorer.html) ikke har lest dokumentet. |

Bruk av ÅpningsKvittering er priset for [Avsender](sdp_aktorer.html).

Åpningskvittering vil være påkrevd helt frem til Innbygger har akseptert at kvitteringen sendes til Avsender. Det vil si at Åpningskvittering kan komme lang tid etter at [digital](dpi_digital.html) meldingen blir sendt.

### Anbefalt bruk

Åpningskvittering er en funksjonalitet som brukes av Avsendere kun etter en grundig og dokumentert vurdering.  
Funksjonaliteten vil kunne oppleves som påtrengende for Mottaker da man ikke får tilgang til å lese den posten man har mottatt uten å godta at kvitteringen blir sendt.

For mer informasjon om bruk av denne funksjonaliteten kan man ta kontakt med Sentralforvalter.

### Schema

[innbyggerpost_dpi_aapningskvittering_1_0.schema.json](schemas/dpi/innbyggerpost_dpi_aapningskvittering_1_0.schema.json)
