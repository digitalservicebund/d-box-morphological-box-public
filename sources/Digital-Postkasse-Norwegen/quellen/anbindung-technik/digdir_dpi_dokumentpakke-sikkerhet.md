# Digdir: DPI Dokumentpakke, sikkerhet

Quelle: https://docs.digdir.no/dpi_dokumentpakke_sikkerhet.html
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

[Hjem](https://docs.digdir.no)  \>  resources  \>  begrep  \>  sikkerDigitalPost  \>  nyinf  \>  dokumentpakke

# Sikkerhet

## Sikkerhet

- Integritet ivaretas ved at dokumentene (posten til mottaker) pakkes og signeres iht. [Associated Signature Container (ASiC)](http://www.etsi.org/deliver/etsi_ts/103100_103199/103174/02.02.01_60/ts_103174v020201p.pdf) fra ETSI. Dette formatet ivaretar integriteten over tid.
- Konfidensialitet fra avsender til mottaker ivaretas ved bruk av [Cryptographic Message Syntax (CMS)](http://tools.ietf.org/html/rfc5652) fra IETF.
- Integritetsbeskyttet [Standard Business Document (SBD)](http://www.gs1.org/ecom/standards/guidelines#s2) fra UN/CEFACT knytter sammen den krypterte pakken med adressering, varsling og annen metadata.

En forsendelse i Sikker digital post inneholder blant annet informasjon for varsling og et hoveddokument med null eller flere vedlegg.

Her beskrives hvordan dokumenter og vedlegg som sendes i Sikker digital post er beskyttet.

Dokumentene og metadata relatert til dokumentene pakkes i en dokumentpakke som ivaretar dokumentenes integritet, samt integriteten til metadata relatert til dokumentene. Dokumentpakkens konfidensialitet ivaretas ved at dokumentpakken krypteres med en symmetrisk engangsnøkkel, og den symmetriske nøkkelen krypteres med mottakerens sertifikat som hentes fra oppslagstjenesten for kontaktinformasjon.

### Standarder

| Standard              | Dokument                                                                                                                           | Versjon        |
|-----------------------|------------------------------------------------------------------------------------------------------------------------------------|----------------|
| ETSI, ETSI TS 102 918 | [Electronic Signatures and Infrastructures (ESI); Associated Signature](etsi1)                                                     | ETSI, 2013-06. |
| ETSI, ETSI TS 103 174 | [Electronic Signatures and Infrastructures (ESI); ASiC Baseline Profile](etsi2)                                                    | ETSI, 2013-06. |
| ETSI, ETSI TS 101 903 | [Electronic Signatures and Infrastructures (ESI); XML Advanced Electronic Signatures (XAdES)](etsi3)                               | ETSI, 2010-12. |
| ETSI, ETSI TS 103 171 | [Electronic Signatures and Infrastructures (ESI); XAdES Baseline Profile](etsi4)                                                   | ETSI, 2012-03. |
| IETF, RFC 5652        | [Cryptographic Message Syntax (CMS)](ietf5)                                                                                        | IETF, 2009-09. |
| IETF, RFC 3560        | [Use of the RSAES-OAEP Key Transport Algorithm in the Cryptographic Message Syntax (CMS)](http://tools.ietf.org/html/rfc3560)      | IETF, 2003-07. |
| IETF, RFC 3565        | [Use of the Advanced Encryption Standard (AES) Encryption Algorithm in Cryptographic Message Syntax (CMS)](ietf7)                  | IETF, 2003-07. |
| IETF, RFC 5084        | [Using AES-CCM and AES-GCM Authenticated Encryption in the Cryptographic Message Syntax (CMS)](http://tools.ietf.org/html/rfc5084) | IETF, 2007-11. |
| IETF, RFC 5083        | [Cryptographic Message Syntax (CMS) Authenticated-Enveloped-Data Content Type](https://datatracker.ietf.org/doc/html/rfc5083)      | IETF, 2007-11  |

### Integritet

Integriteten til dokumentene skal kunne valideres mange år etter mottak, og er ivaretatt ved digitale signaturer som beskrevet nedenfor. I praksis er dette en zip-fil med en gitt struktur som inneholder en digital signatur over innholdet.

#### ASiC profil for dokumentpakken brukt i sikker digital post.

Hoveddokumentet og vedleggene pakkes sammen i en dokumentpakke sammen med noe metadata i henhold til [ASiC (ETSI TS 102 918)](etsi1), og videre begrenset i henhold til profilen definert i [Baseline Profile (ETSI TS 103 174)](etsi2). Ytterlige begrensninger [Baseline Profile (ETSI TS 103 174)](etsi2) følger nedenfor:

| Krav                   | Felt                                                                                                                                | Kommentar                                                                                                                                                                                                 |
|------------------------|-------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [krav 6.1](etsi2_9)    | “ASiC conformance”                                                                                                                  | Skal være “ASiC-E XAdES”.                                                                                                                                                                                 |
| [krav 8.1](etsi2_11)   | “ASiC-E Media type identification”                                                                                                  | Skal være “ASiC file extension is ”.asice“”.                                                                                                                                                              |
| [krav 8.2](etsi2_11)   | “ASiC-E Signed data object”                                                                                                         | Alle filer utenfor META-INF katalogen skal være signert.                                                                                                                                                  |
| [krav 8.3.1](etsi2_12) | “ASiC-E XAdES signature”                                                                                                            | Det skal kun være en signatur i META-INF katalogen, med navn signatures.xml. Denne signaturen skal dekke alle andre filer i beholderen, og avsenderens virksomhetssertifikat skal benyttes for signering. |
| [krav 8.3.2](etsi2_12) | “Requirements for the contents of Container” refererer til “6.2.2 punkt 4b) “META-INF/manifest.xml” if present \[…\] i ”ASiC”:etsi1 | Denne filen skal ikke være tilstede.                                                                                                                                                                      |

#### Signatur i dokumentpakken for sikker digital post

Dokumentpakken bør være signert av [Behandlingsansvarlig](/resources/begrep/sikkerDigitalPost/forretningslag/Aktorer), men kan signeres av [Databehandler](/resources/begrep/sikkerDigitalPost/forretningslag/Aktorer).

Signaturen skal være i henhold til [XAdES (ETSI TS 101 903)](etsi3) med basisprofilen definert i [XAdES Baseline Profile (ETSI TS 103 171)](etsi4) (B-Level Conformance). Ytterlige begrensninger til [XAdES Baseline Profile (ETSI TS 103 171)](etsi4) følger nedenfor:

[TABLE]

### Konfidensialitet

Dokumentpakken krypteres til mottakers sertifikat som leveres fra oppslagstjenesten. Krypteringen skal gjøres i henhold til [CMS (Cryptographic Message Syntax)](http://tools.ietf.org/html/rfc5652) med begrensninger angitt nedenfor.

CMS er BER og DER kodet ASN.1 og starter med en sekvens av ContentInfo.

``` highlight
ContentInfo ::= SEQUENCE {
  contentType ContentType,
  content [0] EXPLICIT ANY DEFINED BY contentType 
}
```

Her skal følgende begrensninger gjelde:

- contentType = 1.2.840.113549.1.7.3 (id-envelopedData).
- content er EnvelopedData som beskrevet nedenfor.

``` highlight
EnvelopedData ::= SEQUENCE {
  version CMSVersion,
  originatorInfo [0] IMPLICIT OriginatorInfo OPTIONAL,
  recipientInfos RecipientInfos,
  encryptedContentInfo EncryptedContentInfo,
  unprotectedAttrs [1] IMPLICIT UnprotectedAttributes OPTIONAL }
```

Her skal følgende begrensninger gjelde:

- version = 0.
- originatorInfo skal ikke være med.
- recipientInfos skal være en mengde av nøyaktig en KeyTransRecipientInfo som beskrevet nedenfor.
- encryptedContentInfo er EncryptedContentInfo som beskrevet nedenfor.
- unprotectedAttrs skal ikke være med.

``` highlight
KeyTransRecipientInfo ::= SEQUENCE {
  version CMSVersion, -- always set to 0 or 2
  rid RecipientIdentifier,
  keyEncryptionAlgorithm KeyEncryptionAlgorithmIdentifier,
  encryptedKey EncryptedKey 
}
```

Her skal følgende begrensninger gjelde:

- version = 0.
- rid = issuerAndSerialNumber.
- keyEncryptionAlgorithm er en AlgorithmIdentifier som beskrevet nedenfor.
- encryptedKey den krypterte nøkkelen.

``` highlight
AlgorithmIdentifier  ::=  SEQUENCE  {
  algorithm               OBJECT IDENTIFIER,
  parameters              ANY DEFINED BY algorithm OPTIONAL  }
```

Her skal følgende begrensninger gjelde:

- algorithm = 1.2.840.113549.1.1.7 (id-RSAES-OAEP) som spesifisert i [RSAES-OAEP Key Transport Algorithm](http://tools.ietf.org/html/rfc3560).
- parameteres = RSAES-OAEP-params som definert nedenfor.

``` highlight
RSAES-OAEP-params  ::=  SEQUENCE  {
  hashFunc    [0] AlgorithmIdentifier DEFAULT sha256Identifier,
  maskGenFunc [1] AlgorithmIdentifier DEFAULT mgf1SHA256Identifier,
  pSourceFunc [2] AlgorithmIdentifier DEFAULT pSpecifiedEmptyIdentifier  }
```

Her skal følgende begrensninger gjelde:

- hashFunc = sha256Identifier
- maskGenFunc = mgf1SHA256Identifier
- pSourceFunc = pSpecifiedEmptyIdentifier

``` highlight
EncryptedContentInfo ::= SEQUENCE {
  contentType ContentType,
  contentEncryptionAlgorithm ContentEncryptionAlgorithmIdentifier,
  encryptedContent [0] IMPLICIT EncryptedContent OPTIONAL 
}
```

Her skal følgende begrensninger gjelde:

- contentType = 1.2.840.113549.1.7.1 (data)
- contentEncryptionAlgorithm er en AlgorithmIdentifier som beskrevet nedenfor.
- encryptedContent = AES-CBC kryptert innhold med PKCS \#5 padding

``` highlight
AlgorithmIdentifier  ::=  SEQUENCE  {
  algorithm               OBJECT IDENTIFIER,
  parameters              ANY DEFINED BY algorithm OPTIONAL  }
```

Her skal verdiene være i henhold til \[Use of the Advanced Encryption Standard (AES) Encryption Algorithm in Cryptographic Message Syntax

(CMS)

\](ietf7), med følgende begrensninger:

- algorithm = 2.16.840.1.101.3.4.1.42 (aes256-CBC)
- parameters = 16 byte IV

Ved bruk av aes256-CBC skal padding gjøres i henhold til [kapittel 6.3 i CMS spesifikasjonen](http://tools.ietf.org/html/rfc5652\#section-6.3).

Integriteten til den krypterte dokumentpakken ivaretas av [Dokumentpakkefingeravtrykk](/resources/begrep/sikkerDigitalPost/begrep/Dokumentpakkefingeravtrykk.html) som ligger i en signert melding.

Det er avsenders ansvar å generere en AES-nøkkel med tilstrekkelig tilfeldighet. Kilden bør være en sertifisert generator for tilfeldige tall (TRNG).

#### Mulig fremtidig utvidelse

Valg av kryptoalgoritme er begrenset til AES-CBC, men AES-GCM har også vært vurdert, og kan tas inn senere ved behov. Årsaken til ikke å ta med AES-GCM i denne omgang er at den ikke er støttet i s/mime, og dermed hindrer bruk av s/mime klienter for ende-til-ende kryptering.

Ved fremtidig støtte for AES-GCM vil følgende begrensninger gjelde:

ContentInfo vil være:

- contentType = 1.2.840.113549.1.9.16.1.23 (id-ct-authEnvelopedData) ved bruk av AES-GCM.
- content AuthEnvelopedData

AuthEnvelopedData er definert i [Cryptographic Message Syntax (CMS) Authenticated-Enveloped-Data Content Type](https://datatracker.ietf.org/doc/html/rfc5083).

``` highlight
AuthEnvelopedData ::= SEQUENCE {
  version CMSVersion,
  originatorInfo [0] IMPLICIT OriginatorInfo OPTIONAL,
  recipientInfos RecipientInfos,
  authEncryptedContentInfo EncryptedContentInfo,
  authAttrs [1] IMPLICIT AuthAttributes OPTIONAL,
  mac MessageAuthenticationCode,
  unauthAttrs [2] IMPLICIT UnauthAttributes OPTIONAL
```

Her skal følgende begrensninger gjelde:

- version = 0
- originatorInfo skal ikke være med.
- recipientInfos skal være en mengde av nøyaktig en KeyTransRecipientInfo som beskrevet nedenfor.
- authEncryptedContentInfo er EncryptedContentInfo som beskrevet nedenfor.
- authAttrs skal ikke være med.
- mac skal være 12 byte “authentication tag” som output av AES-GCM definert i NIST Special Publication 800-38D.
- unauthAttrs skal ikke være med.

For AuthEnvelopedData (AES-GCM) skal verdiene av EncryptedContentInfo være i henhold til [Using AES-CCM and AES-GCM Authenticated Encryption](http://tools.ietf.org/html/rfc5084)

- algorithm = 2.16.840.1.101.3.4.1.46 (aes256-GCM)
- parameters = GCMParameters som beskrevet nedenfor

``` highlight
GCMParameters ::= SEQUENCE {
  aes-nonce        OCTET STRING, -- recommended size is 12 octets
  aes-ICVlen       AES-GCM-ICVlen DEFAULT 12 }
```

Her skal følgende begrensninger gjelde:

- aes-nonce = 12 byte IV/Teller
- aes-ICVlen = 12

\[etsi1\]http://www.etsi.org/deliver/etsi_ts/102900_102999/102918/01.03.01_60/ts_102918v010301p.pdf  
\[etsi2\]http://www.etsi.org/deliver/etsi_ts/103100_103199/103174/02.02.01_60/ts_103174v020201p.pdf  
\[etsi2_9\]http://www.etsi.org/deliver/etsi_ts/103100_103199/103174/02.02.01_60/ts_103174v020201p.pdf#page=9  
\[etsi2_11\]http://www.etsi.org/deliver/etsi_ts/103100_103199/103174/02.02.01_60/ts_103174v020201p.pdf#page=11  
\[etsi2_12\]http://www.etsi.org/deliver/etsi_ts/103100_103199/103174/02.02.01_60/ts_103174v020201p.pdf#page=12  
\[etsi3\]http://www.etsi.org/deliver/etsi_ts%5C101900_101999%5C101903%5C01.04.02_60%5Cts_101903v010402p.pdf  
\[etsi4\]http://www.etsi.org/deliver/etsi_ts/103100_103199/103171/02.01.01_60/ts_103171v020101p.pdf  
\[etsi4_8\]http://www.etsi.org/deliver/etsi_ts/103100_103199/103171/02.01.01_60/ts_103171v020101p.pdf#page=8  
\[etsi4_10\]http://www.etsi.org/deliver/etsi_ts/103100_103199/103171/02.01.01_60/ts_103171v020101p.pdf#page=10  
\[etsi4_11\]http://www.etsi.org/deliver/etsi_ts/103100_103199/103171/02.01.01_60/ts_103171v020101p.pdf#page=11  
\[etsi4_12\]http://www.etsi.org/deliver/etsi_ts/103100_103199/103171/02.01.01_60/ts_103171v020101p.pdf#page=12  
\[etsi4_13\]http://www.etsi.org/deliver/etsi_ts/103100_103199/103171/02.01.01_60/ts_103171v020101p.pdf#page=13  
\[ietf5\]http://tools.ietf.org/html/rfc5652  
\[ietf5_6_3\]http://tools.ietf.org/html/rfc5652#section-6.3  
\[ietf6\]http://tools.ietf.org/html/rfc3560  
\[ietf7\]http://tools.ietf.org/html/rfc3565  
\[ietf8\]http://tools.ietf.org/html/rfc5084  
\[ietf9\]http://tools.ietf.org/html/rfc5083
