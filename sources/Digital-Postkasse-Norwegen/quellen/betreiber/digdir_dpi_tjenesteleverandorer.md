# Digdir: DPI Tjenesteleverandører

Quelle: https://docs.digdir.no/dpi_tjenesteleverandorer.html
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

[Hjem](https://docs.digdir.no)  \>  resources  \>  begrep  \>  sikkerDigitalPost  \>  nyinf  \>  oversikt

# Tjenesteleverandører

Følgende side lister de ulike tjenesteleverandørene innen digital post til innbygger (DPI)

## Avsenders aksesspunkt (hjørne 2)

Leverandør av avsenders aksesspunkt, inkludert DPI API for avsendere, er Orbyt AS.

Avsendervirksomheter som benytter Orbyts DPI API må sikre at de kan nå tjenestene for Produksjon og Test/QA:

| Miljø      | DNS                        | IP-adresse                  |
|------------|----------------------------|-----------------------------|
| Produksjon | https://edi.edigard.com/   | 46.250.215.225              |
| Test / QA  | https://sandbox1.ediex.no/ | 185.47.42.24, 185.47.43.175 |

[Swagger DPI API produksjon og qa/test](https://sandbox1.ediex.no/digdir-api-documentation/v2/swagger-ui/index.html?urls.primaryName=dpi)

## Mottakers aksesspunkt (hjørne 3)

Leverandør er Orbyt AS

## Utskriftstjenesten (hjørne 4)

Leverandør er Skatteetaten

## Digitale postkasser (hjørne 4)

Digipost og e-Boks
