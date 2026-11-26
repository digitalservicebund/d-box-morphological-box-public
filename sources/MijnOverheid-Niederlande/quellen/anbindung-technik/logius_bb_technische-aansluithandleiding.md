# Technische Aansluithandleiding MijnOverheid Berichtenbox – Logius

Quelle: <https://www.logius.nl/onze-dienstverlening/interactie/voorzieningen/mijnoverheid/documentatie/technische-aansluithandleiding-mijnoverheid-berichtenbox>
Abgerufen: 2026-11-26 (HTML mit `lynx -dump -nolist` in Text umgewandelt; Navigation und Fußzeile entfernt)

---

Technische Aansluithandleiding MijnOverheid Berichtenbox
   Deze handleiding beschrijft de functionele en technische specificaties van de Berichtenbox
   van MijnOverheid en hoe uw organisatie hiervan gebruik kan maken. Deze handleiding is
   onderdeel van het technische aansluitpakket van MijnOverheid Berichtenbox. In dit
   aansluitpakket vindt u onder andere voorbeeldberichten en het technische gegevensformulier
   waarmee uw organisatie de connectiviteit kan realiseren met de MijnOverheid Berichtenbox.
Versiegegevens
   Versie: 1.6.4
1. Inleiding
Doel en doelgroep
   Deze handleiding richt zich op organisaties met een publieke taak die gebruik willen maken
   van de Berichtenbox. Daarnaast is dit document bedoeld voor ICT leveranciers die voor en in
   opdracht van organisaties met een publieke taak een aansluiting realiseren op de
   Berichtenbox. Uitgangspunt is dat uw organisatie gebruik maakt van een eigen Digikoppeling
   gateway.
Intermediair
   Indien u geen gebruik maakt van een eigen Digikoppeling gateway kunt u ook aansluiten via
   een intermediair. Intermediairs zijn organisaties met een technische verbinding op
   MijnOverheid Berichtenbox die berichten sturen namens één of meerdere organisaties. Zowel
   overheidsorganisaties (bijvoorbeeld een gemeente) als ICT leveranciers kunnen optreden als
   Intermediair. Op de website van Logius is een overzicht opgenomen van ICT leveranciers die
   intermediair aansluitingen faciliteren.
Opbouw van deze handleiding
   Het eerste hoofdstuk beschrijft globaal de werking van de MijnOverheid Berichtenbox. De
   volgende hoofdstukken beschrijven de technieken en de functionele werking van de
   Berichtenbox.
   De Berichtenbox gebruikt twee verschillende koppelvlakken, het ebMS koppelvlak en het WUS
   koppelvlak. De twee koppelvlakken worden gebruikt om verschillende services aan te spreken.
   In het volgende hoofdstuk zal nader ingegaan worden op het ebMS koppelvlak. De daarop
   volgende hoofdstukken beschrijven de services die gebruik maken van het ebMS koppelvlak.
   Hierna volgt het WUS koppelvlak en de service die gebruik maakt van dit koppelvlak.
2. De MijnOverheid Berichtenbox
   MijnOverheid is de persoonlijke website van burgers voor overheidszaken. Burgers kunnen in
   MijnOverheid o.a. zien welke gegevens de verschillende organisaties over hen hebben
   geregistreerd.
   Burgers kunnen ervoor kiezen om post van de overheid (en van andere organisaties met een
   publieke taak) digitaal te ontvangen via de Berichtenbox. Een organisatie mag alleen
   berichten versturen naar burgers die hebben aangegeven berichten van de betreffende
   organisatie te willen ontvangen. Als burgers een bericht ontvangen, dan krijgen zij daarvan
   een notificatie via e-mail. De gemeente meldt bijvoorbeeld dat de burger het paspoort moet
   laten verlengen, of de RDW meldt dat de auto gekeurd moet worden.
   De privacy van de burger wordt gewaarborgd en overheidsorganisaties voldoen via de
   Berichtenbox aan hun informatieplicht. Berichten afleveren in de Berichtenbox is te
   vergelijken met het versturen van aangetekende post.
Werking van de berichtenbox
   Organisaties met een publieke taak kunnen op een veilige en betrouwbare wijze digitale post
   versturen naar de Berichtenbox van een burger.
   Op dit moment is het (net als in de brievenbus thuis) alleen mogelijk om berichten te
   ontvangen in de Berichtenbox. Burgers hebben zelf niet de mogelijkheid om berichten terug
   te sturen naar de organisatie. Organisaties dienen in het bericht aan te geven op welke
   wijze burgers op het bericht kunnen reageren.
   De burger geeft in zijn profiel van MijnOverheid aan van welke (van de aangesloten)
   organisaties de burger berichten wenst te ontvangen. Als organisatie moet u controleren of
   een burger zich heeft geabonneerd op uw organisatie voordat berichten worden gestuurd. Na
   deze controle wordt het digitale bericht verstuurd naar de Berichtenbox van de burger.
Uitgangspunten MijnOverheid Berichtenbox
     * Het versturen van berichten, opvragen van accountstatus en berichtenvoorkeur van een
       burger verloopt via een beveiligde verbinding tussen de server van uw organisatie (of
       intermediair) en MijnOverheid.
     * Een bericht is persoonlijk en kan maar bij één BSN horen, ongeacht hoe algemeen deze
       van aard is. Onder gepersonaliseerd bericht wordt verstaan: een bericht met informatie
       specifiek gericht aan de burger (houder van de betreffende Berichtenbox) en gericht op
       informatieverstrekking gerelateerd aan een specifieke dienst die de burger bij de
       betreffende organisatie afneemt. Een burger kan een derde persoon machtigen.
     * Alleen organisaties met een publieke taak zoals overheidsorganisaties, pensioenfondsen
       , zorgverzekeraars etc. hebben het recht en de mogelijkheid om berichten sturen naar
       burgers.
     * Een organisatie kan alleen berichten versturen naar burgers die zich hebben geabonneerd
       op berichten van de betreffende organisatie. Een organisatie moet dit vooraf
       controleren via de functies van Opvragen geabonneerden.
     * De Berichtenbox is in principe geschikt voor alle berichten die u nu per papieren post
       aan één of meer geadresseerden verstuurt, dus ook voor berichten met rechtsgevolgen
       zoals beschikkingen. Voor sommige berichten gelden speciale wettelijke eisen
       (persoonlijke overhandiging door een deurwaarder) of vereisen een hoger
       betrouwbaarheidsniveau dan de Berichtenbox nu kan bieden, zoals medische berichten.
       Voor berichtenverkeer met dergelijke persoonsgegevens is de Berichtenbox (nog) niet
       geschikt. U bent verantwoordelijk voor de inhoud van uw berichten en bepaalt (als
       verwerkingsverantwoordelijke) welke berichtenstromen verstuurd kunnen worden via de
       Berichtenbox.
     * De Berichtenbox is van de Burger. Organisaties (Afnemers) noch Logius hebben de
       mogelijkheid om eenmaal verstuurde berichten uit de Berichtenbox te verwijderen. Dit
       ontslaat de individuele afnemer niet van de verplichtingen uit de archiefwet en/of
       aanverwante wetgeving.
     * Organisaties krijgen inzicht of en wanneer berichten zijn afgeleverd (via
       verwerkingsrapport en leveranciersportaal). Organisaties krijgen geen inzicht of en
       wanneer berichten zijn gelezen.
     * De burger ontvangt, indien een e-mailadres is opgegeven in het profiel op MijnOverheid,
       een notificatie wanneer berichten in de Berichtenbox zijn afgeleverd. Na 3 weken volgt
       een herhaalnotificatie.
     * Voor burgers gelden de volgende voorwaarden om in aanmerking te komen voor een
       MijnOverheid-account:
          + is 14 jaar of ouder
          + heeft een Burgerservicenummer (BSN)
          + doet zaken doet met de Nederlandse overheid
          + beschikt over een goedgekeurd inlogmiddel (zoals DigiD of een ander Europees
            inlogmiddel)
     * Alleen aangesloten organisaties hebben het recht om de accountstatus en
       berichtenvoorkeur van een burger op te vragen.
     * Voor een aansluiting op de Berichtenbox dient de organisatie akkoord te gaan en te
       voldoen aan de Algemene Voorwaarden van Logius,  de preproductie en productie
       Voorwaarden MijnOverheid en de standaard Verwerkersovereenkomst van Logius.
     * Voor het uitwisselen van een BSN tussen overheidsorganisaties is geen toestemming van
       de burger nodig indien de standaard Verwerkersovereenkomst van Logius is ondertekend.
     * Gebruiksvoorwaarden zijn vastgelegd in een ministeriële Regeling voorzieningen
       generieke digitale infrastructuur en in de veelgestelde vragen op de website.
     * Een derde partij (intermediair) mag berichten namens een afnemer afleveren bij
       MijnOverheid. Mits deze organisatie met haar klant een Verwerkersovereenkomst in
       relatie tot Logius als bewerker heeft getekend.
Transport Internet en (Digi)netwerk
   Het transport van het berichtenverkeer wordt geregeld via Internet of Diginetwerk.
   Diginetwerk is het besloten netwerk van de overheid. O.a. Haagse Ring, Suwinet en GEMNET
   zijn onderdeel van Diginetwerk.
IP adressen
   In de firewall van de MijnOverheid Berichtenbox is per berichtleverancier een aparte
   uitgaande firewallregel ingesteld. Wanneer het IP adres van uw organisatie wijzigt dient u
   dit tijdig te melden bij Logius.
PKI Overheid
   De HTTPS-verbinding (TLS) die tot stand wordt gebracht tussen de Digikoppeling adapter van
   de aangesloten organisatie en de Digikoppeling adapter van de Berichtenbox is beveiligd met
   een PKIoverheid-certificaat inclusief Overheidsidentificatienummer (OIN/HRN). Aangezien
   Digikoppeling 1.0 wordt gebruikt, is het bericht (payload van het ebMSbericht) niet
   digitaal ondertekend.
   Voor het controleren van certificaten maakt u gebruikt van de PKIOhierarchie en controleert
   u de CLR (Certificate Revocation List) https://crl.pkioverheid.nl/
Meer informatie
     * Bekijk meer informatie over PKIoverheid
     * Meer informatie over de stamcertificaten PKIoverheid
     * Voor een document specifiek gericht op PKIoverheid in combinatie met Digikoppeling
3. Koppelvlakomschrijving ebMS Berichtenbox
   De Koppelvlakstandaard ebMS wordt gebruikt voor synchroon berichtenverkeer tussen
   berichtleveranciers en de Berichtenbox. Het betreft hier berichten waarop vaak niet direct
   een respons kan worden gegeven, omdat hiervoor aan de kant van de ontvanger een proces moet
   worden doorlopen waarvan de doorlooptijd op voorhand niet bekend is. Bij dergelijk
   berichtenverkeer is het wel van belang dat de verzender de zekerheid krijgt dat het bericht
   is aangekomen bij de beoogde ontvanger. Deze „reliability‟ is een standaardonderdeel van
   het ebMS-protocol waarmee wordt gegarandeerd dat een ontvanger een bericht
   „once-andonce-only’ ontvangt. De ontvangende partij stuurt een ontvangstbevestiging
   („acknowledgement message‟) naar de verzender indien het bericht door de ebMS-adapter is
   ontvangen.
   Het ebMS-koppelvlak voorziet onder meer in een „contract‟ tussen de berichtleverancier en
   de Berichtenbox. In dit contract, de zogenoemde “Collaboration Protocol Agreement‟ (CPA),
   zijn zaken als berichttypen, wijze van uitwisseling en beveiliging vastgelegd.
   In het CPA wat u gebruikt voor de Berichtenbox zijn twee actions gedefinieerd:
     * GLOBE-R-A-Request. Deze action wordt gebruikt voor het aanspreken van de
       AbonnementService.
     * GLOBE-R-BV-Request. Deze action wordt gebruikt voor het aanspreken van
       BerichtVerwerkService.
   De Koppelvlakstandaard ebMS stelt onder meer uitdrukkelijke eisen aan de beveiliging van
   het berichtenverkeer. In de volgende paragrafen wordt kort beschreven hoe deze voor de
   Berichtenbox is ingericht.
Beveiliging
   De beveiliging van verbinding wordt middels PKIoverheid-certificaten gecontroleerd. Deze
   certificaten worden gebruikt bij het opzetten van een beveiligde verbinding tussen de
   systemen van de berichtleverancier en de Berichtenbox. Hiervoor wordt gebruikt gemaakt van
   het TLS-protocol. Dubbelzijdige TLS vindt plaats door de afzonderlijke certificaten, die
   door de berichtleverancier worden aangeboden, te verifiëren tegen een lijst van vertrouwde
   certificaten. Na succesvolle authenticatie wordt met behulp van de certificaten een
   versleutelde verbinding geopend.
   Protocollen
   De Berichtenbox ondersteunt TLS versie 1.2. Het is niet mogelijk om gebruik te maken van
   oudere TLS en SSL protocollen.
   De geldigheid van het clientcertificaat wordt aan de hand van de gegevens in het
   certificaat gecontroleerd. Ook wordt er tegen een Certificate
   Revocation List (CRL) gecontroleerd of het certificaat is ingetrokken.
   De identiteit van overheden die gebruik maken van het ebMS-koppelvlak wordt bepaald aan de
   hand van het OIN (OverheidsIdentificatieNummer), dat is opgenomen in het
   PKIoverheid-certificaat waarmee de overheid zich identificeert. Voor ICT leveranciers en
   private organisaties met een publieke taak wordt gebruik gemaakt van het HRN
   (Handelsregisternummer).
   Het is niet mogelijk om de payload van het ebMS bericht digitaal te ondertekenen. De
   Berichtenbox ondersteund alleen het ebMS profiel Reliable Messaging voor berichtaanlevering
   en abonnementbevragingen.
ebMS conversatie
   Een sessie tussen berichtleveranciers en de Berichtenbox wordt in ebMStermen een
   “conversatie” genoemd. In deze paragraaf wordt beschreven hoe een ebMS conversatie eruit
   ziet.
   Een ebMS-converstatie wordt geïnitieerd door een berichtleverancier. De berichtleverancier
   maakt een dubbelzijdige TLS-verbinding met de Berichten. Over deze verbinding worden
   berichten verzonden die zijn opgemaakt conform de ebMS-berichtspecificatie.
   In bovenstaande afbeelding is te zien dat een conversatie wordt opgebouwd uit een aantal
   achtereenvolgende http-sessies. In essentie wordt hier het asynchrone karakter van een
   ebMS-conversatie weergegeven. De verzender stuurt een request-bericht naar de Berichten,
   die hierop reageert met de passende http response, bij succesvolle communicatie wordt een
   status 200 teruggegeven.
   Indien het bericht correct door de ontvangende ebMS-adapter is ontvangen wordt een ebMS
   Acknowledgment teruggestuurd. Wanneer een bericht niet door de ebMS-adapter kan worden
   geaccepteerd, wordt een foutmelding teruggestuurd naar de berichtleverancier.
   In het geval dat er geen Acknowledgment of een foutmelding wordt ontvangen, moet de
   berichtleverancier ervan uitgaan dat het bericht de Berichtenbox niet heeft bereikt. Het
   ebMS-protocol voorziet in dit geval in een retry schema, waarbij binnen een vastgestelde
   periode een vast aantal malen wordt geprobeerd om het bericht alsnog afgeleverd te krijgen.
   De frequentie van het retry mechanisme is onderdeel van de configuratie in het CPA.
   Het sturen van een Acknowledgment betekent niet dat het bericht daarmee ook door de
   achterliggende software correct verwerkt is of correct verwerkt kan worden.
Aanmaken van een CPA
   Logius biedt in samenwerking met het Ministerie van Veiligheid en Justitie een voorziening
   waarmee eenvoudig een CPA kan worden gegenereerd. Een CPA maakt u aan via het CPA Register.
   Wanneer uw organisatie een aansluittraject heeft maakt Logius de benodigde CPA’s voor uw
   organisatie.
   Onderdeel van dit aansluitpakket is de handleiding “CPA Register gebruikershandleiding”,
   hierin staat beschreven hoe u een CPA aanmaakt. In het Technischgegevens formulier vindt u
   de juiste informatie om een CPA ten behoeve van de Berichtenbox te genereren.
4. AbonnementService
   Om digitaal post bij de burger te kunnen afleveren moet de burger zich geabonneerd hebben
   op uw organisatie. De burger kan zich abonneren door in te loggen op MijnOverheid.nl en aan
   te geven dat zij voortaan de post digitale willen ontvangen van uw organisatie.
   De burger heeft niet de mogelijkheid om per berichttype een voorkeur in zijn profiel vast
   te leggen. De burger kan alleen op organisatieniveau aangeven of hij berichten wenst te
   ontvangen van de betreffende organisatie. U kunt alleen berichten sturen naar burgers die
   zich op uw organisatie hebben geabonneerd.
   Via de AbonnementService kan uw organisatie via het ebMS koppelvlak op basis van een BSN
   vaststellen of de betreffende burger heeft aangegeven berichten van uw organisatie via de
   Berichtenbox te willen ontvangen.
   Hieronder een opsomming van de beschikbare services:
     * Gebruik: deze services controleren welke BSN’s een actieve Berichtenbox hebben en
       hebben aangegeven berichten van uw organisatie te willen ontvangen.
     * Antwoordbericht: de voorkeuren in het antwoordbericht hebben een beperkte actualiteit
       en zijn niet bedoeld ten behoeve van het kunnen weerleggen dat een bericht voor een BSN
       (burger) succesvol kan worden verwerkt.
     * Doelbindingsprincipe: Op de geleverde resultaten rust doelbinding. Dat houdt in dat de
       resultaten niet voor andere doeleinden mogen worden gebruikt, dan met het doel te
       bepalen of een bericht voor een burger aan de Berichtenbox kan worden aangeboden.
     * Controle: een BSN wordt niet gecontroleerd op geldigheid (bijv. officieel BSN) maar wel
       op geldigheid van de structuur (bijv. alleen cijfers, maximaal 9 cijfers).
   Voor abonnementsbevragingen adviseren wij om gebruik te maken van het ebMS koppeling.
   Hiervan zijn de aantallen ongelimiteerd en ook bedoeld om grotere bevragingen aan te kunnen
   leveren. Bevragingen via het WUS koppelvlak zijn in aantallen gelimiteerd en alleen bedoeld
   voor kleine bevragingen.
AbonnementService Request
   Om geabonneerde BSN’s op te vragen verstuurt uw organisatie een GLOBE-R-A-Request naar de
   Berichtenbox. Er zijn drie verschillende varianten om de AbonnementService te bevragen, dit
   zijn de “BSN lijst”, “Volledig” en “Mutaties”.
BSN Lijst
   Uw organisatie stuurt een request met daarin maximaal 1 miljoen BSN’s. De AbonnementService
   controleert of er een actieve Berichtenbox bestaat voor het betreffende BSN en of het
   betreffende burger heeft aangegeven berichten van uw organisatie te willen ontvangen.
   Element Toelichting
   BerichtLeverancierCode Het OIN van uw organisatie, 20 cijfers.
   VerzoekId (optioneel) Een referentie ID om het vraag en antwoord bericht aan elkaar te
   relateren.
   BSNLijst Lijst van BSN’s waarvoor validatie moet plaatsvinden. Per bevraging mogen maximaal
   1.000.000 BSN’s worden meegestuurd.
   Zie voorbeeldbericht GLOBE-R-A-Request - BSN Lijst.xml
Volledig
   Uw organisatie verstuurt een request zonder daarin BSN’s op te nemen. U ontvangt daarop
   alle BSN’s waarvoor de burger heeft aangegeven berichten te willen ontvangen van uw
   organisatie. Het result bericht bevat mogelijk meer BSN’s dan waarvoor uw organisatie een
   relatie mee heeft.
   Element Toelichting
   BerichtLeverancierCode Het OIN van uw organisatie, 20 cijfers.
   VerzoekId (optioneel) Een referentie ID om het vraag en antwoord bericht aan elkaar te
   relateren.
   Zie voorbeeldbericht GLOBE-R-A-Request - Volledig.xml
Mutaties
   Uw organisatie verstuurt een request met hierin een mutatiedatum/tijd. Alle mutaties op
   berichtvoorkeuren van gebruikers van de berichtenbox voor uw organisatie sinds die
   datum/tijd worden verstuurd. De mutatie variant kan alleen in combinatie met de “volledige”
   variant gebruikt worden.
   Initieel moet de “volledige” variant aangeroepen worden om de volledige set met
   geabonneerden op te vragen. Daarna worden telkens mutaties opgevraagd. Mocht er onverhoopt
   toch sprake zijn van een synchronisatieconflict, moet de gehele set met geabonneerden
   wederom opgevraagd worden door middel van variant volledig.
   Element Toelichting
   BerichtLeverancierCode Het OIN van uw organisatie, 20 cijfers.
   VerzoekId (optioneel) Een referentie ID om het vraag en antwoord bericht aan elkaar te
   relateren.
   Mutatiedatum Mutatiedatum en tijd vanaf wanneer de mutaties op geabonneerden toegestuurd
   moeten worden.  De Berichtenbox gebruikt als tijdnotatie Zulu tijd.
   Zie voorbeeldbericht GLOBE-R-A-Request - Mutatie.xml
AbonnementService Result bericht
   Nadat uw een GLOBE-R-A-Request succesvol heeft verstuurd ontvangt u binnen vier uur een
   result bericht (GLOBE-R-A-Result) van de Berichtenbox. Het result bericht bevat
   XML-gegevens waarop een Gzip (RFC 1952) compressie is toegepast. Hiermee wordt de
   hoeveelheid te versturen gegevens beperkt, naar schatting is de omvang van het
   gecomprimeerde bericht circa 10% van de omvang van het originele bericht. Het result
   bericht bevat de volgende elementen:
   Element Toelichting
   BerichtLeverancierCode Het OIN van uw organisatie, 20 cijfers.
   DatumTijdVerwerking De systeemdatum en –tijd van moment van aanmaken van het
   antwoordbericht, in DateTime formaat. Bij Mutatie request wordt deze waarde gebuikt als
   Mutatiedatum. De Berichtenbox gebruikt als tijdnotatie Zulu tijd.
   VerwerkingsCode Status van de verwerking. Standaardwaarde "Verwerkt"; anders staat hier een
   foutcode.
   VerzoekIdAanvraag VerzoekId dat is meegegeven in het vraagbericht als referentie tussen
   vraag en antwoord bericht.
   Actief In geval van succesvolle verwerking bevat dit element de lijst met burgers die
   geabonneerd zijn op uw organisatie.
   Inactief In geval van succesvolle verwerking bevat dit element de lijst met burgers die het
   abonnement op uw organisatie beëindigd hebben. Uitsluitend van toepassing op mutatie
   request.
   BSNLijst Overzicht van alle BSN’s waarvoor validatie heeft plaatsgevonden. Deze lijst is
   niet van toepassing bij variant BSN Lijst.
   BSN Het BSN van de burger.
Verwerkingscodes
   De Berichtenbox kan verschillende verwerkingscodes terugkoppelen. De verwerkingscode
   “Verwerkt” is de standaard verwerkingscode en geeft aan dat het request succesvol is
   verwerkt. Indien een request niet succesvol is verwerkt kan een van de volgende
   verwerkingscodes worden teruggekoppeld:
     * TechnischProbleem
     * LeverancierNietBekend
     * VerplichteLeverancierNietOndersteund
     * BSNLijstOntbreektInAanvraag
     * XmlValidatieTegenXsdValtNegatiefUit
5. Berichten Verwerkservice
   Na het opvragen van geabonneerden wordt via de Berichten Verwerkservice het bericht
   verstuurd naar de Berichtenbox van de burger. De Berichten Verwerkservice maakt gebruik van
   het ebMS koppelvlak.
   De berichten dienen te voldoen aan de Berichtstroomspecificaties, deze zijn opgenomen in
   dit aansluitpakket. De Berichtenbox controleert de verstuurde berichten op technische
   juistheid. Voldoet een bericht niet dan zal deze niet afgeleverd worden in de Berichtenbox
   van de betreffende burger.
   Verschillende soorten berichten worden geclassificeerd als berichttypen.
   De berichttypen dienen vooraf geconfigureerd te zijn in het Leveranciersportaal. Meer
   informatie over het Leveranciersportaal te vinden in de “Handleiding Leveranciersportaal”
   welke onderdeel is van het aansluitpakket.
Berichten Verwerkservice Request
   Om berichten aan te leveren verstuurd uw organisatie een GLOBE-R-BVRequest naar de
   Berichtenbox. Berichten dienen verstuurd te worden in een batch, een batch bestaat uit
   maximaal 1000 berichten. De maximale grote van één batch aanlevering mag maximaal 100MB
   groot zijn inclusief de base64 overhead.
   De Berichtenbox gaat uit van de UTF-8 karakterset. Uw organisatie dient gegevens aan de
   Berichtenbox aan te bieden op basis van deze karakterset. Berichten die niet aan de
   karakterset voldoen worden niet verwerkt.
     * De berichten dienen te voldoen aan de berichtspecificaties zoals in deze paragraaf
       beschreven. De ontvangen berichten worden op technische juistheid gecontroleerd voordat
       ze in de Berichtenbox van de burger worden geplaatst; de verantwoordelijkheid voor het
       technisch juist aanleveren van berichten ligt bij de aangesloten organisatie.
     * Verschillende soorten berichten worden geclassificeerd als berichttypen. De
       berichttypen dienen vooraf bekend te zijn gemaakt. Hiervoor verkrijgt de organisatie
       toegang tot het Berichtenbox Leveranciersportaal.
     * Een bericht kan maximaal twee gepersonaliseerde bijlagen bevatten.
     * Per berichttype kunnen maximaal drie standaardbijlagen (bijvoorbeeld een standaard
       toelichting, brochures of bezwaarmakingsproces) worden voor gedefinieerd. Deze worden
       vooraf geüpload via het Berichtenbox Leveranciersportaal. De standaardbijlagen worden
       bij de persoonlijke berichtaanlevering achterwege gelaten.
     * Berichten dienen per batch te worden aangeleverd. Aanlevering van de batch dient
       aansluitend op het aanmaken van de batch plaats te vinden. Een batch kan uit één of
       meerdere berichten bestaan.
     * Een batch kan meerdere berichttypen bevatten.
     * Het maximaal aantal dagen tussen de aanmaakdatum van de batch en de datum waarop de
       actieve berichtenboxen zijn vastgesteld (via Opvragen geabonneerden) bedraagt 7 dagen.
     * Berichten kunnen worden aangeleverd met een publicatiedatum/tijd in de toekomst. Deze
       publicatiedatum/tijd ligt maximaal 13 dagen in de toekomst, ten opzichte van de
       aanmaakdatum van de batch
     * Indien  geen publicatiedatum/tijd is opgegeven, dan wordt het bericht direct na
       ontvangst en verwerking door MijnOverheid aan de burger beschikbaar gesteld.
     * Indien ten tijde van de berichtverwerking de publicatiedatum/tijd verstreken is, wordt
       het bericht direct gepubliceerd.
     * Een berichttekst mag URL’s bevatten. Elke URL dient te beginnen met http:// of https://
       en moet worden voorafgegaan en gevolgd door ten minste één spatie, bijvoorbeeld:
       http://www.example.com . Bij het tonen van het bericht aan de gebruiker wordt de URL
       als hyperlink gepresenteerd.
Elementen in het Request bericht
   De volgende elementen kunnen worden opgenomen in een request bericht:
   Element Toelichting
   EindDatumHandelingsTermijn De uiterste datumtijd alvorens de handeling welke het bericht de
   burger vraagt uit te voeren voltrokken dient te zijn.
   BatchID GUID zonder accolades in registry format (bijv.
   9F9126B2-C661-4c0f-9E05-7B038906CCCF).  Het betreffende BatchID moet worden herhaald in
   ieder individueel bericht. Het BatchID moet uniek zijn voor elke batch die uw organisatie
   verstuurt.
   AanmaakDatum Datum waarop de batch is aangemaakt. Door organisatie in te vullen
   (YYYY-MMDDTHH:MM:SSZ) (bijv. 2017-0610T09:30:47Z). De aanmaakdatum mag niet verder dan 13
   dagen in het verleden liggen op het moment van aanbieden. De Berichtenbox gebruikt als
   tijdnotatie Zulu tijd.
   BerichtLeverancierID Het OIN van uw organisatie, 20 cijfers.
   BerichtID GUID in registry format (bijv. 9F9126C2-C6614c0f-9E05-7B038906CCCF). Dit ID moet
   uniek zijn voor ieder bericht dat uw organisatie verstuurt.
   BerichtType De letterlijke naam van het BerichtType. Berichttype configureert u in het
   Leveranciersportaal. Het berichttype moet geconfigureerd en actief zijn.
   PublicatieDatum (optioneel) Datum en tijd waarop het bericht wordt gepubliceerd.
   (YYYY-MM-DDTHH:MM:SSZ) (bijv. 2017-06-10T09:30:47Z). De publicatiedatum mag maximaal 13
   dagen ná de aanmaakdatum liggen. Indien op het moment van verwerken van het bericht de
   publicatiedatum inmiddels verstreken is, wordt het bericht direct gepubliceerd.  De
   Berichtenbox gebruikt als tijdnotatie Zulu tijd.
   Onderwerp Onderwerp van het bericht, maximaal 50 tekens.
   Berichttekst Tekst van het bericht, maximaal 4000 tekens. Een regeleinde is “\r\n”.
   Referentie (optioneel) Uw eigen referentiegegevens, maximaal 25 tekens.
   GebruikerID BSN van de geadresseerde.
   SoortGebruiker Vaste waarde: “Burger”.
   Inhoud (optioneel) De inhoud van de (PDF) bijlage als Base64encoded-string.
   BijlageType (optioneel) Vaste waarde: “Pdf”.
   Omschrijving (optioneel) (Korte) omschrijving van de bijlage. Maximaal 128 tekens.
   Volgorde Indien er één bijlagen wordt meegestuurd “1”. Indien er twee bijlagen worden
   meegestuurd “1” of “2”. Als een bericht meerdere bijlagen bevat, kan hiermee de volgorde
   worden bepaald waarin de bijlagen in de berichtenbox worden getoond.
   De Berichtenbox gaat uit van de UTF-8 karakterset. Uw organisatie dient gegevens aan de
   Berichtenbox aan te bieden op basis van deze karakterset. Berichten die niet aan de
   karakterset voldoen worden afgekeurd.
   Een batch bevat maximaal 1000 Berichtenbox-berichten. De maximale grootte van een batch is
   100 MB (inclusief base64 overhead). Dit betekent dat er met batches van minder dan 1000
   berichten gewerkt moet worden indien de som van de grootte van de individuele bijlagen,
   inclusief base64 overhead, de 100 MB overschrijdt.
   De maximale (gezamenlijke) grootte van de gepersonaliseerde bijlage(n) in een individueel
   bericht is 500 kB. Een bericht kan maximaal twee gepersonaliseerde bijlagen bevatten. De
   500 kB is gebaseerd op de grootte van de PDF bestanden tezamen.
   Wanneer een PDF bestand base64 encoded wordt, zal de bestandsgrootte met ongeveer 33%
   toenemen. De PDF’s mogen samen dus niet groter zijn dan 500 kB, voordat deze base64 encoded
   worden.
Verwerking van berichten
   Zodra de Berichtenbox een berichtenbatch ontvangt van uw organisatie wordt een aantal
   controles uitgevoerd, voordat het bericht daadwerkelijk in de Berichtenbox van de burger
   wordt geplaatst. Een bericht wordt pas vrijgegeven indien aan onderstaande condities is
   voldaan. Terugkoppeling over het verwerkingsresultaat wordt geplaatst op het
   Leveranciersportaal en komt terug via een GLOBE-R-BV-Result bericht.
   Op het moment dat de afzender is geauthentiseerd en geautoriseerd start het proces van de
   controle op de berichten.
   Van ieder ontvangen bericht wordt gecontroleerd of:
     * Het bericht inclusief gepersonaliseerde bijlage(n) niet groter is dan 500 kB
     * Het BSN een actieve berichtenbox heeft
     * Het BSN een abonnement heeft op uw organisatie
     * Het aangeleverde bericht een geldig en bestaand berichttype heeft (Berichttypes worden
       via het Berichtenbox Leveranciersportaal aangemaakt)
     * De aanmaakdatum van het bericht niet meer dan 13 dagen in het verleden ligt
     * Een eventuele publicatiedatum van het bericht niet meer dan 13 dagen na de aanmaakdatum
       ligt
     * Het bericht maximaal 2 gepersonaliseerde bijlagen bevat
     * Het bericht gebruik maakt van de karakterset UTF-8
     * Het bericht wordt gecontroleerd tegen de XSD
   Als ten minste één controle negatief uitvalt, wordt een status gemeld op het
   Leveranciersportaal en stopt de verwerking van het bericht
   Wanneer alle controles zijn uitgevoerd en de validatie positief is wordt het bericht in de
   Berichtenbox van de burger geplaatst. Berichten die niet aan bovenstaande voorwaarden
   voldoen worden niet verwerkt
Berichten Verwerkservice Result bericht
   Nadat de Berichtenbox uw batchaanlevering succesvol heeft ontvangen zal er een result
   bericht terug gestuurd worden (GLOBE-R-
   BV-Result). In het result bericht is staat vermeld hoeveel berichten succesvol zijn
   verwerkt en hoeveel berichten niet verwerkt konden worden.
   De result bericht bestaat uit de volgende elementen:
   Element Toelichting
   BerichtLeverancierCode Het OIN uit de ontvangen batch.
   BatchID Het BatchID van de ontvangen batch.
   TotaalAantalOntvangen Berichten Het totaal aantal berichten dat is ontvangen door de
   Berichtenbox in de batch met bovenstaand ID.
   AantalBerichtenSuccesvol Verwerkt Het aantal berichten dat succesvol is verwerkt door de
   Berichtenbox vanuit de batch met bovenstaand ID.
   AantalBerichtenGeen ActieveBoxOfGeabonneert OpLeverancier
   Het aantal berichten vanuit de batch met bovenstaand ID:
     * waarvoor geen actieve Berichtenbox is gevonden;
     * dat niet afgeleverd kon worden, omdat de burger heeft aangegeven geen berichten van uw
       organisatie wenst te ontvangen.
   AantalBerichtenMet TechnischProbleem Het aantal berichten vanuit de batch met bovenstaand
   ID waarbij een technisch probleem is opgetreden.
   AantalBerichtenBericht TypeNietCorrect Het aantal berichten vanuit de batch met bovenstaand
   ID met een incorrect BerichtType.
   AantalBerichtenPublicatie DatumNietCorrect Het aantal berichten vanuit de batch met
   bovenstaand ID met een ongeldige PublicatieDatum.
   AantalBerichtenAanmaak DatumNietCorrect Het aantal berichten vanuit de batch met
   bovenstaand ID met een ongeldige AanmaakDatum.
   DatumOntvangen Ontvangstdatum van de batch met bovenstaand ID.
   DatumVerwerkt Verwerkingsdatum van de batch met bovenstaand ID.
   BerichtID ID (GUID) van het bericht zoals ontvangen in de batch met bovenstaand ID.
   BerichtType Naam van bericht type
   Verwerkingscode Resultaat van de verwerking.
   Stadium Geeft aan tijdens welke verwerkingsfase het probleem is geconstateerd.
Verwerkingscodes
   De Berichtenbox kan verschillende verwerkingscodes terugkoppelen. De verwerkingscode
   “Verwerkt” is de standaard verwerkingscode en geeft aan dat het request succesvol is
   verwerkt. Indien een request niet succesvol is verwerkt kan een van de volgende
   verwerkingscodes worden teruggekoppeld:
   Verwerkingscode Toelichting
   Stadium Toelichting
   Verwerkt Het bericht is verwerkt en opgeslagen in de Berichtenbox van de burger.
   TechnischProbleem Er is tijdens het verwerken van het bericht een technisch probleem
   ontstaan. In het element Stadium is te zien tijdens welke verwerkingsfase het probleem is
   geconstateerd, zie ook hieronder.
   NietActiefOfGeabonneerd De burger voor wie het bericht bestemd is, heeft geen actieve
   Berichtenbox, bestaat niet of heeft geen abonnement op de berichtleverancier (uw
   organisatie).
   BerichtTypeNietOndersteund Het gebruikte BerichtType bestaat niet of is niet actief.
   AanmaakDatumLigtTeVerInHetVerleden Als uw organisatie de batch aanmaakt wordt een
   AanmaakDatum gevuld met een datum. Bij aanlevering wordt gecontroleerd of de AanmaakDatum
   niet verder dan 13 dagen in het verleden ligt ten opzichte van de datum waarop de batch
   wordt verwerkt.
   PublicatieDatumLigtTeVerInDeToekomst Als een bericht niet direct mag worden getoond aan de
   burger dan kan uw organisatie er voor kiezen om een publicatiedatum mee te geven. Deze mag
   echter maximaal 13 dagen na de aanmaakdatum liggen.
   BerichtBestaatAl Een bericht met hetzelfde BerichtID is reeds aangeboden door uw
   organisatie.
   BijlageTeGroot De omvang van de persoonlijke bijlage(n) in het aangeboden bericht is te
   groot.
   OinInCPAKomtNietOvereenMetOinInBericht U kunt alleen het OIN gebruiken dat overeenkomt met
   het OIN uit het CPA en het PKIoverheid-certificaat.
   XmlValidatieTegenXsdValtNegatiefUit Het aangeboden bericht valideert niet tegen het
   XML-schema (XSD) van het Berichtenbox-bericht.
   ValidatieBerichtType In dit stadium is het niet gelukt om het BerichtType te valideren.
   ValidatieGebruiker In dit stadium is het niet gelukt om de burger te valideren.
   StoreMessage In dit stadium is het niet gelukt om het bericht op te slaan.
   NA Dit betekent dat alle stadia succesvol zijn doorlopen.
   Indien uw organisatie aan de hand van het retourbericht constateert dat berichten niet zijn
   verwerkt, kunt u aan de hand van de verwerkingscode en stadium bepalen welk probleem zich
   heeft voorgedaan. Indien het probleem bij uw organisatie ligt kunt u, na oplossen van het
   probleem, de berichten opnieuw aanbieden in een nieuwe batch. Hiervoor dient u het zelfde
   BatchID en BerichtID te gebruiken.
Gebruik van speciale karakters in XML berichten
   Bij het versturen van XML berichten moet rekening worden met de volgende speciale
   karakters:
   Karakter   Omschrijving     Code
   >        groter dan teken  &gt;
   <        kleiner dan teken &lt;
   &        ampersand teken   &amp;
   %        procent teken     &#37;
   '        apostrof teken    &apos;
   "        quote teken       &quot;
   Indien bovenstaande karakters gebruikt worden in XML berichten dienen deze tekens gecodeerd
   te worden meegegeven. Hiermee zorgt u ervoor dat de tekst die u via de XML berichten mee
   wilt geven op de juiste manier wordt verwerkt in de MijnOverheid Berichtenbox en dus ook
   juist getoond wordt aan de burger.
   Let op: Als u gebruik maakt van speciale karakters dan is het belangrijk om dit goed te
   testen tijdens het aansluittraject.
Regeleinde
   Een regeleinde (of lege regel) in de berichttekst moet worden gecodeerd als "\r\n" zonder
   de aanhalingstekens.
   Voorbeeld van een regeleinde en een lege regel:
   Geachte heer/mevrouw, \r\n\r\n\ Hierbij sturen we u het volgende bericht. \r\n\r\n\ Met
   vriendelijke groet, \r\n\r\n\ Test &amp; Organisatie \r\n\ http://www.organisatie.nl
   Deze voorbeeld regel wordt als volgt weergegeven in de Berichtenbox:
   Geachte heer/mevrouw,
   Hierbij sturen we u het volgende bericht.
   Met vriendelijke groet,
   Test & Organisatie  http://www.organisatie.nl
Mapping van Berichtenbox elementen
   De onderstaande tabel beschrijft of, en op welke wijze, elementen uit het
   Berichtenbox-bericht weergegeven worden in de Berichtenbox van de burger.
   Element Toelichting
   EindDatumHandelingsTermijn De uiterste datumtijd alvorens de handeling welke het bericht de
   burger vraagt uit te voeren voltrokken dient te zijn.
   BatchID Wordt niet getoond in de Berichtenbox van de burger.
   AanmaakDatum Wordt niet getoond in de Berichtenbox van de burger.
   BerichtLeverancierID Op basis van het OIN (Overheidsidentificatienummer) wordt de afzender
   bepaald. De afzendernaam wordt door Logius beheerd.
   BerichtID Wordt niet getoond in de Berichtenbox van de burger.
   BerichtType Wordt niet getoond in de Berichtenbox van de burger.
   PublicatieDatum Tijdstip van verwerken van het bericht of de PublicatieDatum indien deze is
   ingevuld.
   Onderwerp Onderwerp van het bericht.
   Berichttekst Tekst van het bericht.
   Referentie (optioneel) Berichtreferentie.
   GebruikerID Wordt niet getoond in de Berichtenbox van de burger.
   SoortGebruiker Wordt niet getoond in de Berichtenbox van de burger.
   Inhoud (optioneel) De PDF-bijlage bij het bericht.
   BijlageType (optioneel) Wordt niet getoond in de Berichtenbox van de burger.
   Omschrijving (optioneel) De naam van de link naar de PDF-bijlage.
   Volgorde Wordt niet getoond in de Berichtenbox van de burger, het bepaalt de volgorde
   indien er meerdere persoonlijke bijlagen meegestuurd zijn.
   Via onderstaande afbeeldingen wordt visueel weergeven waar de elementen zichtbaar zijn in
   de Berichtenbox van de burger.
Richtlijnen voor PDF bijlagen
   Een Berichtenbox-bericht kan een of meerdere PDF-bijlagen bevatten.  Documenten moeten
   leesbaar zijn ongeacht het besturingssysteem of de software die gebruikt wordt. Ook in de
   toekomst moeten documenten leesbaar zijn met de software die dan gangbaar is. Mensen met
   een visuele of andere handicap moeten documenten van de overheid kunnen lezen met de
   hulpmiddelen die ze daarvoor hebben. Dit stelt eisen aan de documenten die de overheid via
   de berichtenbox verstuurt.
   Om de toegankelijkheid nu en in de toekomst te waarborgen moeten de PDF-bijlagen van uw
   organisatie voldoen aan de volgende eisen:
    1. Het formaat is conform de open standaard PDF/A waarvan de volgende conformiteitsniveaus
       zijn toegestaan: PDF/A-1a (ISO 19005-1) of PDF/A-2-a (ISO 19005-2).
    2. De inhoud is conform Web Content Accessibility Guidelines 2.0 (ISO 40500).
    3. De maximale (gezamenlijke) grootte van de gepersonaliseerde bijlage(n) in een
       individueel bericht is 500 kB. Een bericht kan maximaal twee gepersonaliseerde bijlagen
       bevatten. De 500 kB is gebaseerd op de grootte van de PDF bestanden tezamen. Wanneer
       een PDF bestand base64 encoded wordt, zal de bestandsgrootte met ongeveer 33% toenemen.
       De PDF’s mogen samen dus niet groter zijn dan 500 kB, voordat deze base64 encoded
       worden.
    4. Daarnaast kunnen per berichttype maximaal 3 standaard bijlagen worden toegevoegd met
       een gezamenlijke maximale grootte van 2 MB.
   In de volgende paragrafen worden deze eisen nader toegelicht.
PDF/A
   PDF is het gangbare formaat voor de uitwisseling van documenten waarvan de pagina opmaak
   vastligt. Door de jaren zijn er verschillende versies van PDF gepubliceerd voor
   verschillende toepassingen. Daardoor heeft PDF meer dan één standaard.
   PDF/A1 is een door ISO gestandaardiseerd formaat dat duurzame toegankelijkheid garandeert.
   PDF/A1 heeft alleen die functionaliteit die duurzaam ondersteund blijft. Een PDF/A1
   document heeft alle informatie in zich (zoals de gebruikte lettertypen) die nodig is om het
   weer te geven ongeacht de gebruikte applicatie, ook in de toekomst. Een PDF/A1 document is
   niet afhankelijk van applicatie specifieke functies. Het is niet moeilijk om PDF/A1
   documenten te creëren. Alle gangbare office applicaties kunnen een document in PDF/A1
   formaat opslaan. PDF/A staat op de “pas toe of leg uit” lijst van het Forum
   Standaardisatie.
   De instructie rijksdienst inzake aanschaf van ICT-producten en ICT-diensten (besluit
   WJZ/8157380 van 8/11/2008) verplicht overheden om de open standaarden op de “pas toe of leg
   uit” te gebruiken.
   De Europese Unie stelt dit jaar de standaard EN 301 549 verplicht, die
   toegankelijkheidseisen voorschrijft voor ICT producten en diensten. De Europese standaard
   EN 301 549 gelast het gebruik van de WCAG 2.0 richtlijnen voor websites en documenten.
   Alleen PDF/A documenten met het conformiteitsniveau PDF/A1-1a (ISO 19005-1) of PDF/A-2-a
   (ISO 19005-2) kunnen volledig voldoen aan de WCAG 2.0 richtlijnen voor toegankelijkheid.
   PDF/A documenten met een conformiteitsniveau PDF/A-1b, PDF/A-2b of PDF/A-2u, en andere PDF
   standaarden hebben niet het geschikte formaat om te voldoen aan WCAG 2.0 richtlijnen voor
   toegankelijkheid.
   Het Forum Standaardisatie kan u verder helpen met documentstandaarden en hun toepassing.
Metadata
   PDF bestanden kunnen metadata bevatten. Metadata geeft extra informatie over het bestand
   zoals bijvoorbeeld auteur, aanmaakdatum, copyright.
   De Overheid.nl Web Metadata Standaard (OWMS) is de metadatastandaard voor informatie van de
   Nederlandse overheid op internet. De standaard is gebaseerd op de internationale
   metadatastandaard van het Dublin Core Metadata Initiative (DCMI).
   Er wordt geadviseerd om zorgvuldig om te gaan met de metadata in de pdf. Het opnemen van
   teveel informatie is een risico. Aangeraden wordt om geen software en versie informatie op
   te nemen in deze PDF’s. Digitale inbrekers maken gebruik van deze informatie om kwetsbare
   systemen aan te vallen.
WCAG 2.0 toegankelijkheidsrichtlijnen
   Volgens de Europese regelgeving moeten websites en documenten gepubliceerd door overheden
   van de EU lidstaten gaan voldoen aan de WCAG 2.0 richtlijnen voor toegankelijkheid. In de
   volgende tabel vindt u referenties naar de WCAG 2.0 richtlijnen en ondersteunende
   informatie.
                 Referentie                                Link
   WCAG 2.0 specificatie (normatief)      http://www.w3.org/TR/WCAG20/
   WCAG 2.0 quick regerence (informatief) http://www.w3.org/WAI/WCAG20/quickref/
   Aanwijzingen om aan de WCAG 2.0 richtlijnen te voldoen zijn de volgende (dit is slechts een
   steun, geen volledige lijst van WCAG 2.0 richtlijnen):
     * Een PDF document moet labels (“tags”) bevatten die de documentstructuur beschrijven.
       Dit is één van de belangrijkste toegankelijkheidsmaatregelen. De labels of “tags”
       zorgen ervoor dat toegankelijkheidsapplicaties voor blinden, slechtzienden en
       kleurenblinden het document op de juiste manier kunnen weergeven.
     * Koppen in een PDF document moeten als bladwijzers (“bookmarks”) zijn opgeslagen.
     * Een PDF document moet een titel hebben en dat de taal van de documenten moet correct
       gespecificeerd zijn.
     * Een PDF document van meer dan één pagina moet paginanummers hebben met een enkele
       nummering (1, 2, 3,..) voor het hele document. Dus niet nummers i, ii, iii,.. voor het
       voorwoord en 1, 2, 3,.. voor de rest van het document, bijvoorbeeld.
     * Tabellen en formulieren moeten eenvoudig zijn en een duidelijke structuur hebben. In
       een tabel moet duidelijk zijn wat de kop rij is. In een formulier moet duidelijk zijn
       wat er ingevuld moet worden.
     * Afbeeldingen moeten van een tekstalternatief voorzien zijn. Decoratieve afbeeldingen
       zonder bijzondere betekenis moeten duidelijk als zodanig door het tekstalternatief
       aangemerkt zijn.
     * Een gescand PDF document moet ook een transcriptie van de tekst op het gescande
       origineel bevatten. Een transcriptie kan
     * meestal automatisch worden gegenereerd met karakterherkenningssoftware (OCR).
     * De beveiligingsopties van een PDF document moeten het automatisch lezen en kopiëren van
       de tekst uit het document toelaten.
PDF compressie
   MijnOverheid Berichtenbox bevat een beperking in berichtgrootte: de maximale (gezamenlijke)
   grootte van de gepersonaliseerde bijlage(n) in een individueel bericht is 500 kB. Daarnaast
   kan per berichttype maximaal 3 standaardbijlagen worden toegevoegd met een gezamenlijke
   maximale grootte van 2 MB. Zowel technisch als functioneel zijn maatregelen te nemen om de
   grootte van de PDF bijlagen te beperken.
   Functioneel:
     * Gebruik (gecomprimeerde) JPG afbeeldingen.
     * Als het mogelijk is gebruik dan vector graphics.
     * Gebruik alleen standaard fonts. Een PDF is “self-contained”. Dit houdt o.a. in dat
       wanneer een niet-standaard font gebruikt wordt, deze in de PDF meegeleverd wordt. Dit
       vergroot de benodigde grootte behoorlijk. Een font is al snel 40kb groot.
     * Gebruik het RGB kleurenbereik. Dit vereist minder ruimte dan het CMYK kleurenbereik.
     * Als kleur niet nodig is, dan omzetten naar “grayscale”.
     * Voor de BerichtenBox is een pixel dichtheid van 72dpi in principe voldoende.
   Technisch voor grijswaarden- en kleurenafbeeldingen:
     * JPEG
     * MRC (Mixed Raster Content) met JPEG compressie
     * ZIP
   Voor bitonale afbeeldingen:
     * CITT Group 3 en 4
     * JBIG2 lossy of lossless
     * ZIP
   Indien het niet mogelijk is om de grootte van bijlagen te beperken (bijvoorbeeld
   tekeningen) is het te overwegen om een link op te nemen naar een omgeving buiten
   MijnOverheid. Denk er wel aan dat de url ook op langere termijn beschikbaar moet blijven.
6. Koppelvlakomschrijving WUS
   WUS is een acroniem voor WSDL, UDDI en SOAP. Daarmee wordt een familie van internationale
   standaarden van OASIS en W3C bedoeld. Deze worden ook vaak met WS- aangeduid. Deze
   standaarden gaan over application-to-application webservices.
Beveiliging
   De beveiliging van verbinding wordt middels PKIoverheid-certificaten gecontroleerd. Deze
   certificaten worden gebruikt bij het opzetten van een beveiligde verbinding tussen de
   systemen van de berichtleverancier en de Berichtenbox. Hiervoor wordt gebruikt gemaakt van
   TLS -protocol. Dubbelzijdige TLS vindt plaats door de afzonderlijke certificaten, die door
   de berichtleverancier worden aangeboden, te verifiëren tegen een lijst van vertrouwde
   certificaten. Na succesvolle authenticatie wordt met behulp van de certificaten een
   versleutelde verbinding geopend.
   Protocollen
   De Berichtenbox ondersteunt enkel TLS versie 1.2.
   De geldigheid van het clientcertificaat wordt aan de hand van de gegevens in het
   certificaat gecontroleerd. Ook wordt er tegen een Certificate Revocation List (CRL)
   gecontroleerd of het certificaat is ingetrokken.
   De identiteit van overheden die gebruik maken van het ebMSkoppelvlak wordt bepaald aan de
   hand van het OIN (OverheidsIdentificatieNummer), dat is opgenomen in het
   PKIoverheid-certificaat waarmee de overheid zich identificeert. Voor ICT leveranciers en
   private organisaties met een publieke taak wordt gebruik gemaakt van het HRN
   (Handelsregisternummer). Het is niet mogelijk SOAP berichten te Signen.
Sessieverloop
   SOAP-berichten die aan de Berichtenbox worden aangeboden zijn opgemaakt conform een voor
   gedefinieerde structuur (SOAP request). Deze structuur is vastgelegd in een XML Schema
   (XSD), dat op zijn beurt is opgenomen in de WSDL die de web service formeel beschrijft. De
   XSD en de WSDL zijn opgenomen in het aansluitpakket.
   Een web service client van een berichtleverancier maakt een TLSverbinding met een web
   service van de Berichtenbox. Over deze verbinding worden er een SOAP validatierequest
   verzonden. Als het validatierequest niet voldoet aan de eisen gesteld in de WSDL, wordt er
   een SOAP fault teruggestuurd. Als het bericht wel voldoet aan de eisen, dan wordt het
   verder verwerkt. Ook in het geval dat de verwerking niet correct kan worden uitgevoerd,
   wordt er een SOAP fault teruggestuurd. Indien de verwerking succesvol verlopen is, wordt er
   een SOAP response verzonden.
7. Berichtenbox Validatieservice
   Via de webservice Berichtenbox validatieservice kan uw organisatie synchroon op basis van
   een BSN vaststellen of de betreffende burger heeft aangegeven berichten van uw organisatie
   via de Berichtenbox te willen ontvangen. Hiervoor wordt gebruik gemaakt van het WUS
   koppelvlak.
   De operatie die als webservice van de Berichtenbox is gedefinieerd is ValidateAbonnementen.
   Deze operatie biedt de mogelijkheid om een set van minimaal 1 BSN en maximaal 250 BSN’s te
   valideren. Wanneer er meer dan 250 BSN’s gevalideerd moeten worden dient gebruik gemaakt te
   worden van de AbonnementService.
     * Gebruik: deze services controleren welke BSN’s een actieve Berichtenbox hebben en
       hebben aangegeven berichten van uw organisatie te willen ontvangen.
     * Antwoordbericht: de voorkeuren in het antwoordbericht hebben een beperkte actualiteit
       en zijn niet bedoeld ten behoeve van het kunnen weerleggen dat een bericht voor een BSN
       (burger) succesvol kan worden verwerkt.
     * Doelbindingsprincipe: Op de geleverde resultaten rust doelbinding. Dat houdt in dat de
       resultaten niet voor andere doeleinden mogen worden gebruikt, dan met het doel te
       bepalen of een bericht voor een burger aan de Berichtenbox kan worden aangeboden.
     * Controle: een BSN wordt niet gecontroleerd op geldigheid (bijv. officieel BSN) maar wel
       op geldigheid van de structuur (bijv. alleen cijfers, maximaal 9 cijfers).
Validatie Request
   In dit aansluitpakket is opgenomen de WSDL en de XSD’s welke uw organisatie nodig heeft om
   gebruik te kunnen maken van de Berichtenbox validatieservice.
   Het request wat uw organisatie in het SOAP-bericht moet plaatsen heeft de volgende
   elementen:
   Element Toelichting
   BerichtLeverancierCode Het OIN van uw organisatie, 20 cijfers.
   BerichtTypeCode Aanduiding van het BerichtType dat uw organisatie wilt verzenden. De codes
   worden door uw organisatie zelf toegekend via het Berichtenbox Leveranciersportaal.
   Klanten Lijst van Klant-elementen. Één tot maximaal 250 BSN’s per vraagbericht.
   Key Het BSN van de burger. De combinatie Key en Rol vormt een unieke identificatie. Een BSN
   mag voorloop nul(len) bevatten.
   Rol Vaste waarde: “Burger”.
Validatie Result
   Nadat het request succesvol is ontvangen, ontvangt u direct een result terug. In dit
   bericht staat per opgevraagde BSN vermeldt of deze een actief abonnement heeft op uw
   organisatie. Indien er een fout optreedt bij de verwerking van het request, wordt een fout
   result teruggegeven.
   De volgende elementen zijn opgenomen in het result bericht:
   Element Toelichting
   ValidateAbonnementenResult Lijst van de bevraagde klanten uit het vraagbericht opgenomen in
   Abonnement-element. Één tot maximaal 250 per vraag en antwoordbericht.
   Klant Idem vraagbericht.
   isBerichtSturen Boolean (true of false) welke aangeeft of uw organisatie een bericht (van
   het opgegeven BerichtType) naar de Berichtenbox van deze burger mag sturen.
Afkortingen en begrippen
   Veelgebruikte afkortingen en uitleg bij lastig begrippen vindt u in de begrippenlijst.
