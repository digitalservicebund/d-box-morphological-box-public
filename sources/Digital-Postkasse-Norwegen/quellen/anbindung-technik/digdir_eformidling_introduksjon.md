# Digdir: eFormidling Introduksjon

Quelle: https://docs.digdir.no/docs/eFormidling/Introduksjon/
Abgerufen: 2026-11-23

---

eFormidling

[Kollaps](#) \| [Utvid](#) \| [Hjem](https://docs.digdir.no/index.html)

- [Introduksjon]()
  - [Introduksjon](/docs/eFormidling/Introduksjon/)
  - [Kom i gang](/docs/eFormidling/Introduksjon/kom_i_gang)
  - [Last ned](/docs/eFormidling/Introduksjon/last_ned)
  - [Fagsystem](/docs/eFormidling/Introduksjon/fagsystem)
- [Funksjonalitet]()
  - [Oversikt](/docs/eFormidling/Funksjonalitet/)
  - [Saksbehandling](/docs/eFormidling/Funksjonalitet/saksbehandling)
  - [Taushetsbelagt saksbehandling](/docs/eFormidling/Funksjonalitet/taushetsbelagt_saksbehandling)
  - [Informasjon til innbygger](/docs/eFormidling/Funksjonalitet/informasjon_til_innbygger)
  - [Vedtak til innbygger](/docs/eFormidling/Funksjonalitet/vedtak_til_innbygger)
  - [Innsynskrav fra eInnsyn](/docs/eFormidling/Funksjonalitet/innsynskrav)
  - [Journalpost til eInnsyn](/docs/eFormidling/Funksjonalitet/journalpost)
  - [Møte til eInnsyn](/docs/eFormidling/Funksjonalitet/mote)
  - [Avtalt meldingsutveksling](/docs/eFormidling/Funksjonalitet/avtalt)
  - [Sending til fastlege](/docs/eFormidling/Funksjonalitet/sending_til_fastlege)
- [Egenskaper]()
  - [Egenskaper](/docs/eFormidling/Egenskaper/)
- [Installasjon]()
  - [Kom i gang med integrasjonspunktet](/docs/eFormidling/installasjon/)
  - [1) Forberede installasjon](/docs/eFormidling/installasjon/forberede_installasjon)
  - [2) Installasjon](/docs/eFormidling/installasjon/installasjon)
  - [3) Automatisk oppgradering](/docs/eFormidling/installasjon/automatisk_oppgradering)
  - [4) Sikkerhetskopi](/docs/eFormidling/installasjon/sikkerhetskopi)
  - [5) Overvåking](/docs/eFormidling/installasjon/overvaking)
- [Oppgradering]()
  - [Oppgradering](/docs/eFormidling/Oppgradering/)
  - [Endringslogg](/docs/eFormidling/Oppgradering/endringslogg)
- [Utvikling]()
  - [Kom i gang med utvikling](/docs/eFormidling/Utvikling/)
  - [Eksempler](/docs/eFormidling/Utvikling/Eksempel/)
  - [Meldingstjenester](/docs/eFormidling/Utvikling/Meldingstjenester/)
  - [Dokumenttyper](/docs/eFormidling/Utvikling/Dokumenttyper/)
  - [På vegne av](/docs/eFormidling/Utvikling/pa_vegne_av)
  - [Kanal](/docs/eFormidling/Utvikling/kanal)
  - [Integrasjonspunktets API (eFormidling 2)](/docs/eFormidling/Utvikling/integrasjonspunkt_eformidling2_api)
  - [Adressetjenestens API](/docs/eFormidling/Utvikling/serviceregistry_api)
  - [Meldingsstatus API](/docs/eFormidling/Utvikling/meldingsstatus_api)
- [Testing]()
  - [Kom i gang med testing](/docs/eFormidling/Testing/)
  - [Krav og testbeskrivelser](/docs/eFormidling/Testing/krav)
- [Selvhjelp]()
  - [Feilsøking](/docs/eFormidling/Selvhjelp/feilsoking)
  - [Kjente feil](/docs/eFormidling/Selvhjelp/kjente_feil)
  - [Spørsmål og svar](/docs/eFormidling/Selvhjelp/sporsmal_og_svar)
  - [Sporing av meldinger](/docs/eFormidling/Selvhjelp/sporing)
- [Miljø]()
  - [Produksjon](/docs/eFormidling/Miljo/produksjon)
  - [Test](/docs/eFormidling/Miljo/test)

------------------------------------------------------------------------

- [Github](https://github.com/difi/felleslosninger)
- [Samarbeidsportalen](https://samarbeid.digdir.no)
- [Status og driftsmeldinger](https://status.digdir.no/)
- [Digitaliseringsdirektoratet](https://www.digdir.no)

[Hjem](https://docs.digdir.no)  \>  eFormidling  \>  Introduksjon

# Introduksjon

Dette er den tekniske dokumentasjonen for eFormidling. Formålet er å støtte arkitekter, utviklere og driftspersonell hos offentlige virksomheter og deres leverandører i innføring og bruk av eFormidling.

1.  [Hva er eFormidling?](#hva-er-eformidling)
2.  [I hvilke sammenhenger kan eFormidling brukes?](#i-hvilke-sammenhenger-kan-eformidling-brukes)
3.  [Hva kan sendes og mottas med eFormidling?](#hva-kan-sendes-og-mottas-med-eformidling)
4.  [Hvordan fungerer eFormidling?](#hvordan-fungerer-eformidling)
5.  [Neste steg](#neste-steg)

Se gjerne Samarbeidsportalen for en helt overordnet ikke-teknisk introduksjon til eFormidling:

- [Dette er eFormidling](https://samarbeid.digdir.no/eformidling/dette-er-eformidling/46) (ekstern lenke)
- [Ta i bruk eFormidling](https://samarbeid.digdir.no/eformidling/ta-i-bruk-eformidling/98) (ekstern lenke)

## Hva er eFormidling?

eFormidling lar din virksomhet:

- sende meldinger til mottakende virksomhets foretrukne kanal
- sende meldinger til mottakende innbyggers foretrukne kanal
- motta innkommende meldinger på ønsket kanal

Alle norske virksomheter og innbyggere kan nås gjennom eFormidling.

> For å forenkle tilknytning til eksisterende infrastrukturer for meldingsutveksling til andre offentlige virksomhet, innbyggere og virksomheter, bør eFormidling benyttes.
>
> [Digitaliseringsrundskrivet](https://www.regjeringen.no/no/dokumenter/digitaliseringsrundskrivet/id2895185/) (ekstern lenke)

Funksjonaliteten i eFormidling kan tas i bruk gjennom:

- [fagsystem som allerede støtter integrasjon med eFormidling](fagsystem)
- [egne integrasjoner direkte mot eFormidling](Utvikling/)

## I hvilke sammenhenger kan eFormidling brukes?

Meldinger kan sendes i mange ulike sammenhenger og må i mange tilfeller behandles ulikt avhengig av sammenhengen. For eksempel vil mottakeren i noen sammenhenger ønske å:

- Rute meldinger til ulike fagsystem
- Knytte enkelte meldinger til strengere tilgangskontroller
- Legge meldinger i ulike arbeidskøer

Alle meldinger i eFormidling sendes derfor som del av en gitt sammenheng. I eFormidling kalles disse sammenhengene prosesser. En mottaker støtter meldingsutveksling i forbindelse med en eller flere prosesser. En mottaker støtter en gitt prosess gjennnom en eller flere dokumenttyper som igjen støttes av en eller flere meldingstjenester. Dokumenttypene definerer hvilket innhold som kan sendes. I tilfeller der en prosess støttes gjennom flere dokumenttyper må avsender ta stilling til hvilken dokumenttype som ønskes brukt. I tilfeller der samme dokumenttype støttes av flere meldingstjenester sikrer eFormidling at mottakers foretrukne meldingstjeneste blir brukt. Prosesser, meldingstjenester og dokumenttyper utgjør mottakerens kapabiliteter.

Prosesser som støttes av eFormidling inkluderer:

[TABLE]

## Hva kan sendes og mottas med eFormidling?

eFormidling støtter meldinger med både strukturert og ustrukturert innhold. Meldinger med egendefinert innhold kan også sendes. Hvilket innhold som støttes avhenger av sammenhengen en melding sendes i og mottakerens kapabiliteter. Mulig innhold inkluderer:

- XML-baserte format
- JSON-baserte format
- PDF-filer
- Tekstfiler

## Hvordan fungerer eFormidling?

eFormidling er ikke i seg selv en meldingstjeneste, men leverer derimot en mellomvare (integrasjonspunktet) som gir fagsystem ett enkelt grensesnitt for meldingsutveksling uavhengig av prosess, dokumenttype og meldingstjeneste. Integrasjonspunktet installeres hos virksomhetene (eller deres leverandører) og integreres mot et eller flere fagsystem. Fagsystem kan benytte integrasjonspunktet over følgende grensesnitt: [eFormidling 2](Utvikling/integrasjonspunkt_eformidling2_api)

I tillegg leverer eFormidling:

- en sentral adressetjeneste (service registry) som brukes av integrasjonspunktet for å slå opp foretrukket kanal for mottaker
- diverse støttetjenester (administrasjon, overvåking, feilsøking, ol)

Før en virksomhet sender en melding gjøres et kapabilitetsoppslag for å se om mottakeren støtter den aktuelle prosessen meldingen skal sendes som del av og i så fall med hvilke dokumenttyper mottakeren støtter denne. Virksomheten bygger så meldingen med ønsket dokumenttype før den sendes til integrasjonspunktet. Integrasjonspunktet ruter meldingen til foretrukket meldingstjeneste (f.eks. KS SvarUt) for mottakeren.

sequenceDiagram participant A as Virksomhetens  
fagsystem participant B as Virksomhetens  
integrasjonspunkt participant C as Adressetjeneste participant D as Meldingstjeneste  
(f.eks. KS SvarUt) A-\>\>B: Kapabilitetsoppslag B-\>\>C: Kapabilitetsoppslag A-\>\>B: Utgående melding B-\>\>D: Utgående melding

Virksomhetens fagsystem mottar kontinuerlig innkommende meldinger fra virksomhetens integrasjonspunkt som igjen mottar meldingene fra meldingstjenestene virksomheten bruker.

sequenceDiagram participant A as Virksomhetens  
fagsystem participant B as Virksomhetens  
integrasjonspunkt participant D as Meldingstjeneste  
(f.eks. KS SvarInn) D-\>\>B: Innkommende melding B-\>\>A: Innkommende melding

## Neste steg

- [Kom i gang](kom_i_gang)
