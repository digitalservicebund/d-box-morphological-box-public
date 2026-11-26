# Notificatiedienst – MijnOverheid Zakelijk (MOZa, BZK)

Quelle: <https://www.mijnoverheidzakelijk.nl/onderwerpen/notificatiedienst/>
Abgerufen: 2026-11-26 (HTML mit `lynx -dump -nolist` in Text umgewandelt; Navigation und Fußzeile entfernt)

---

                               Notificatiedienst

   Status: In ontwikkeling

Op deze pagina

     * Huidige uitdaging
     * Wat is de notificatiedienst?
          + Vijf stappen
     * Wat de dienst wel en niet doet
     * De oplossing
          + Via welke kanalen wordt genotificeerd?
          + Wie bepaalt naar wie wordt genotificeerd?
          + Verzending en terugkoppeling
          + Wat gebeurt er als een notificatie niet aankomt? En wie
            bepaalt de vervolgstappen?
               o Procesherstel
               o Contactherstel
          + Dataverwerking
     * Uitgangspunten
          + Centrale en decentrale regie
          + Moderne overheidsstandaarden
          + Afsprakenstelsel
     * Vervolgstappen
     * Onze werkwijze
     * Meedoen
     * Begrippen
     * Meer info

Huidige uitdaging

   Overheidsorganisaties willen burgers en ondernemers betrouwbaar en
   efficiënt informeren. Er is alleen geen gezamenlijke voorziening voor
   het versturen van meldingen, notificaties en e-mails. Daardoor lopen we
   tegen knelpunten aan:
     * Versnippering: organisaties gebruiken elk hun eigen systemen,
       teksten en werkwijzen.
     * Complexiteit: elke organisatie regelt zelf het versturen, beheren
       en loggen van notificaties. Dat leidt tot dubbel werk en tot
       verschillen in uitvoering.
     * Inefficiëntie: veel organisaties hebben dezelfde behoefte, maar
       bouwen bij gebrek aan een standaardvoorziening allemaal een eigen
       oplossing.

   Een gezamenlijke notificatiedienst lost dit op. Daarmee bereiken we:
     * Voor burgers en ondernemers: je krijgt tijdig bericht over zaken
       die je aandacht vragen, op het kanaal dat je zelf hebt gekozen.
       Komt een digitaal bericht niet aan, dan wordt een alternatief
       kanaal ingezet.
     * Voor overheidsorganisaties: je hoeft zelf geen verzendkanaal,
       printstraat of contactherstelproces in te richten. Je sluit aan op
       een centrale voorziening en krijgt terugkoppeling over wat er met
       je notificatie is gebeurd.
     * Voor de digitale overheid: een herbruikbare bouwsteen die past
       binnen de Generieke Digitale Infrastructuur (GDI).

Wat is de notificatiedienst?

   De notificatiedienst helpt overheidsorganisaties om burgers en
   ondernemers op het juiste moment via het juiste kanaal te informeren.
   Dankzij één gezamenlijke voorziening worden berichten betrouwbaar
   verstuurd en opgevolgd, en waar nodig ondersteund met contactherstel.

   De notificatiedienst is een gezamenlijk initiatief van Logius en het
   programma OBIS. De dienst werkt als een digitale postbezorger voor de
   overheid: een infrastructuur waarmee dagelijks miljoenen berichten
   veilig en betrouwbaar naar burgers en ondernemers kunnen worden
   verstuurd.

Vijf stappen

   De notificatiedienst ondersteunt vijf stappen in het verzenden en
   opvolgen van digitale berichten:
    1. Ontvangers bepalen. Vaststellen wie een bericht moet ontvangen en
       via welk contactkanaal die persoon of organisatie bereikbaar is.
    2. Berichten versturen. Het opstellen en verzenden van de notificatie
       via het juiste kanaal.
    3. Reacties verwerken. Het vastleggen en verwerken van
       terugkoppelingen op verzonden berichten.
    4. Contactherstel bepalen. Signaleren dat een bericht niet aankomt en
       vaststellen welke vervolgstappen nodig zijn.
    5. Contactherstel uitvoeren. Het starten, volgen en afhandelen van
       acties om alsnog contact te leggen.

   Een organisatie kan de hele notificatiedienst afnemen, maar ook alleen
   de onderdelen die zij nodig heeft. Zo sluit de dienst aan op de
   behoeften die binnen MOZa en OBIS zijn opgehaald bij ondernemers,
   burgers, overheidsorganisaties en medewerkers van die organisaties.

Wat de dienst wel en niet doet

   De notificatiedienst verstuurt geen officiële documenten. De dienst
   ondersteunt het digitale proces en informeert burgers en ondernemers
   dát er ergens een bericht klaarstaat, en waar. Denk aan een melding dat
   een beschikking, aanvraag of ander document klaarstaat in een portaal.

   De dienst is vooralsnog niet geschikt voor het afleveren van het
   inhoudelijke bericht zelf.

   Een beschikking kan dus niet via de notificatiedienst worden verstuurd,
   wél het bericht dat er een beschikking klaarstaat en waar die te vinden
   is.

   Voorbeeld:
   een ondernemer ontvangt een e-mail met de boodschap “Er staat een nieuw
   bericht voor u klaar. U kunt dit bekijken via [naam portaal].”

   Het bericht zelf haalt de ondernemer vervolgens op in het systeem van
   de overheidsorganisatie.

De oplossing

   De oplossing start bij de dienstverlener die een notificatie naar de
   ondernemer of burger wil versturen. De dienstverlener start de
   notificatie vanuit de eigen procesapplicatie, meestal via een lokaal
   outputmanagementcomponent (OMC) of een vergelijkbare voorziening. De
   Notificatie Management Component (NMC) neemt het vanaf daar over:
   contactgegevens bepalen, de notificatie laten versturen door Notify, de
   terugkoppeling verwerken en waar nodig contactherstel starten.
   Bovenaan staat een ondernemer of burger, verbonden met een
   overheidsportaal. Bij de dienstverlener staan een procesapplicatie en
   een outputmanagementcomponent (OMC), die in twee richtingen informatie
   uitwisselen. Boven het OMC staat het decentrale profiel, waaruit het
   OMC de contactvoorkeuren ophaalt en waaraan het die levert. De
   procesapplicatie geeft de notificatie door aan het OMC, dat het verzoek
   naar de centraal georganiseerde notificatiedienst stuurt. Ook het OMC
   en de Notificatie Management Component (NMC) wisselen in twee
   richtingen informatie uit. De notificatiedienst haalt de kanaalvoorkeur
   en contactgegevens op uit de profielservice. De notificatiedienst
   bestaat uit de NMC, Notify, contactherstel en de printstraat. De NMC
   organiseert de processen binnen de notificatiedienst: het stuurt Notify
   aan, dat de e-mail verstuurt, en bewaakt de status van het verzenden.
   Is contactherstel nodig, dan start de NMC ook dat proces, waarbij de
   printstraat een brief per post kan versturen. Contactherstel wisselt
   daarvoor gegevens uit met de basisregistraties, die buiten de
   notificatiedienst vallen maar wel centraal georganiseerd zijn. Deze
   e-mails en brieven bereiken de ondernemer of
   burger.DienstverlenerCentraal georganiseerde
   dienstenNotificatiedienstOndernemerof
   burgerOverheidsportaalProfielserviceNotificatie ManagementComponent
   (NMC)NotifyContactherstelPrintstraatDecentraal
   profielOutputmanagement-component (OMC)Procesapplicatie@Versturen
   e-mailVersturen briefBasisregistraties

   Schuif opzij om de hele plaat te zien.
   Download als afbeeldingDe notificatiedienst in samenhang met
   componenten van de dienstverlener en centrale voorzieningen.

  Via welke kanalen wordt genotificeerd?

   Op dit moment richten we ons op één primair kanaal: e-mail. In de
   toekomst kunnen daar kanalen bij komen, zoals fysieke verzending per
   post, sms, alternatieve digitale berichtenkanalen en andere vormen van
   elektronische communicatie.

   Voor contactherstel wordt fysieke verzending per post straks wél
   mogelijk. Juist als het digitale kanaal niet werkt, is post de manier
   om iemand alsnog te bereiken.

  Wie bepaalt naar wie wordt genotificeerd?

   De overheidsorganisatie die de notificatie verstuurt, bepaalt zelf hoe
   de ontvanger wordt vastgesteld. Dat kan op twee manieren:
    1. Decentrale profielservice. De dienstverlener heeft een eigen
       profielservice en bepaalt daarmee zelf wat de contactvoorkeur is en
       welke gegevens daarbij horen. Worden die gegevens meegegeven, dan
       gebruikt de notificatiedienst ze bij het verzenden. Ook als er in
       een centrale profielservice andere gegevens beschikbaar zijn.
    2. Centrale profielservice. Er is één centrale plek waar
       contactvoorkeuren en bijbehorende gegevens worden vastgelegd. Op
       basis van een identificerend nummer zoals een KVK-nummer of BSN kan
       het juiste contactgegeven worden opgehaald.

   Het heeft de voorkeur om zoveel mogelijk gebruik te maken van de
   centrale profielservice. Contactgegevens zijn dan eenvoudiger te
   beheren, informatie kan centraal worden bevraagd en processen zoals
   contactherstel zijn centraal te organiseren.

  Verzending en terugkoppeling

   Geen enkele e-maildienst kan garanderen dat een bericht daadwerkelijk
   door de ontvanger is gelezen. Daarom geldt: is een bericht succesvol
   afgeleverd bij de mailserver van de ontvanger, dan beschouwen we dat
   als een succesvolle verzending.

   De notificatiedienst koppelt per functionaliteit terug wat de status
   is. De verzendende organisatie weet daardoor steeds waar haar
   notificatie in het proces staat. Die terugkoppeling bestaat uit twee
   delen:
    1. Wat is de huidige status, en wat is er eventueel al opgevolgd? De
       dienst meldt dat het bericht is verzonden en meldt problemen zodra
       ze zich voordoen. Dat kan uitmonden in de terugkoppeling dat
       verzending niet is gelukt en dat wordt overgegaan op
       contactherstel.
    2. Wat gebeurt er hierna, als er geen tegenbericht komt? Bijvoorbeeld:
       “notificatie verzonden, we wachten 48 uur”. Komt er tussentijds een
       softbounce binnen, een melding dat de e-mailserver tijdelijk
       onbereikbaar is, dan volgt “softbounce ontvangen, nieuwe poging
       over 3 uur”. En uiteindelijk mogelijk: “notificatie niet
       afgeleverd, contactherstel via post gestart”.

   Voorbeelden van statussen die kunnen optreden:
     * contactgegeven niet gevonden, alternatief adres nodig
     * notificatie verzonden
     * tijdelijke afleverfout, nieuwe poging gepland
     * notificatie niet afgeleverd
     * contactherstel gestart via post
     * contactherstel afgeleverd

   Dit zijn voorbeelden. De lijst vullen we verder aan naarmate we de
   dienst samen met dienstverleners vormgeven.

  Wat gebeurt er als een notificatie niet aankomt? En wie bepaalt de
  vervolgstappen?

   Soms kan een notificatie niet worden afgeleverd, bijvoorbeeld omdat een
   e-mailadres niet meer bestaat of niet bereikbaar is. Dan kunnen er
   herstelacties nodig zijn. Bij notificeren onderscheiden we er twee:
   procesherstel en contactherstel.

    Procesherstel

   Het niet kunnen afleveren van een notificatie heeft mogelijk gevolgen
   voor het onderliggende proces. Of dit zo is, dat wordt vastgesteld door
   de dienstverlener. De dienstverlener bepaalt bijvoorbeeld of een
   bericht opnieuw moet worden verstuurd, of een proces moet worden
   gepauzeerd, wanneer vervolgacties nodig zijn en welke gevolgen een
   mislukte aflevering heeft. Dat noemen we procesherstel: het verder
   oppakken van het proces waaraan de notificatie is gekoppeld.

   Een voorbeeld maakt duidelijk waarom dat bij de dienstverlener hoort:
     * Een proces richt zich op de btw-aangifte.
     * Dat proces leidt tot een beschikking, die bekend moet worden
       gemaakt aan de ontvangers. Onderdeel daarvan is een notificatie
       over die beschikking.
     * In de beschikking staat dat er een bedrag betaald moet worden.

   Om te bepalen wat het betekent dat de notificatie niet is aangekomen,
   is kennis van dat proces nodig. Zijn er meerdere ontvangers, en hebben
   die de notificatie wel gekregen? Is het bedrag inmiddels betaald, en
   maakt dat uit voor de vraag of opnieuw verzonden moet worden? En als er
   niet is betaald: per wanneer komt daar een boete bij?

   De notificatiedienst heeft in de meeste situaties verder geen rol meer
   in het procesherstel.

    Contactherstel

   Het feit dat een notificatie niet kan worden afgeleverd, betekent in
   veel gevallen ook dat contactherstel nodig is. Ofwel, we starten een
   proces op waarmee we de ondernemer of burger vragen om hun
   contactgegevens bij te werken. Het doel daarvan is dat de burger of
   ondernemer het digitale adres corrigeert, zodat toekomstige
   notificaties weer aankomen.

   Voordat contactherstel start, stellen we eerst vast hoe de verzending
   is verlopen. Het initiëren van contactherstel is dus niet een proces
   dat automatisch in gang wordt gezet. Ook hier is er interactie met de
   dienstverlener. De dienstverlener bepaalt daarbij bijvoorbeeld zelf hoe
   het contacthersteladres wordt vastgesteld. Dat kan op twee manieren:
    1. Via de dienstverlener zelf. Net als bij het initiëren van de
       notificatie kan de dienstverlener het contacthersteladres zelf
       meegeven. Dat kan direct bij het initiëren van de notificatie, of
       het adres wordt op dat moment actief bij de dienstverlener
       opgehaald. Onze voorkeur gaat uit naar het tweede, omdat dat beter
       past bij het principe van dataminimalisatie.
    2. Als onderdeel van de notificatiedienst. Kiest de dienstverlener
       daarvoor en zijn er voldoende gegevens beschikbaar, dan kan de
       notificatiedienst zelf contactherstel starten en een adres bepalen.
       Bijvoorbeeld op basis van een KVK-nummer, RSIN of BSN.

   Op basis van een pseudo-id of een ander kenmerk van de dienstverlener
   is geen adres te herleiden in centrale bronnen. Heeft een
   dienstverlener geen officieel kenmerk, dan kan de notificatiedienst het
   contactherstel niet zelf organiseren. De dienstverlener kan de
   contactherstelgegevens dan wel bij de aanroep meegeven.

   We onderzoeken of contactherstelacties voor hetzelfde contactgegeven
   periodiek gegroepeerd kunnen worden, zodat de ontvanger daarover één
   bericht krijgt in plaats van meerdere. Dat scheelt post aan mensen van
   wie het e-mailadres niet meer werkt.

   De notificatiedienst houdt bij dat contactherstel is gestart en stelt
   ook vast wanneer het is afgerond. Na een bepaalde periode worden die
   gegevens weer verwijderd.

  Dataverwerking

   Bij het versturen van berichten worden gegevens verwerkt. Op basis van
   de standaard Logboek Dataverwerkingen leggen we vast welke verwerkingen
   binnen de notificatiedienst hebben plaatsgevonden.

Uitgangspunten

   Bij het vormgeven van de notificatiedienst hanteren we de volgende
   uitgangspunten:
     * Functionaliteit is leidend, de techniek volgt daaruit.
     * Zo snel mogelijk meerwaarde bieden voor zoveel mogelijk
       dienstverleners.
     * De aansluitdrempel is zo laag mogelijk voor alle dienstverleners.
     * De regiekeuze blijft bij de dienstverlener. Die bepaalt zelf welke
       functionaliteiten er gebruikt worden.

  Centrale en decentrale regie

   Er is behoefte aan zowel centrale als decentrale regie op notificeren.

   Decentraal ligt regie voor de hand bij het koppelen van notificaties en
   statussen aan een klantcontactsysteem, het besluit tot herverzending,
   en de koppeling met procesherstel. Een decentraal OMC kan die rol goed
   invullen, maar een dienstverlener kan dit ook anders organiseren.

   Centraal ligt regie voor de hand bij het ontzorgen van het
   notificatieproces, zodat dit vanuit een overheidsstandaard steeds op
   dezelfde manier verloopt. Ook het uitvoeren van contactherstel hoort
   daarbij, zodat niet elke dienstverlener afzonderlijk contactherstel
   hoeft te starten. En het bepalen van ontvangersgegevens, waarbij
   meerdere bronnen een rol kunnen spelen, zoals de profielservice, de
   Basisregistratie Personen (BRP) en het Handelsregister (HR).

   Van sommige functies is nog niet duidelijk of ze logischerwijs centraal
   horen. Die kunnen mogelijk ook decentraal worden belegd. Dat verkennen
   we in samenhang, aan de hand van concrete use-cases.

   De NMC heeft uitdrukkelijk niet de bedoeling om decentrale OMC’s te
   vervangen. De NMC ontstaat ernaast. Het bestaande OMC is ontwikkeld
   voor gemeenten, binnen Common Ground en MijnServices. De NMC bouwen we
   als een voorziening om centrale processen te regisseren. Daarbij bouwt
   de NMC voort op het OMC. Deze aanname toetsen we periodiek opnieuw.

  Moderne overheidsstandaarden

   De notificatiedienst gebruikt moderne overheidsstandaarden, waaronder
   Federatieve Service Connectiviteit (FSC), Logboek Dataverwerkingen, de
   NeRDS-principes en de API-standaarden. Daardoor kunnen
   overheidsorganisaties hun bestaande systemen eenvoudig aansluiten. De
   dienst ondersteunt niet alleen het versturen van berichten, maar ook
   het terugkoppelen van bezorginformatie en het uitvoeren van
   vervolgacties zoals contactherstel.

  Afsprakenstelsel

   Over de dienstverlening maken we duidelijke afspraken, vastgelegd in
   een afsprakenstelsel. Aangesloten organisaties weten daardoor vooraf
   wat zij kunnen verwachten.

   Organisaties kunnen ook gebruikmaken van een speciale aansluiting. Is
   daar een juridische grondslag voor, dan kunnen zij een bericht
   versturen op basis van een identificerend nummer zoals een BSN of
   KVK-nummer. De notificatiedienst haalt dan via de centrale
   profielservice de voorkeurscontactgegevens van de ontvanger op, en
   verstuurt het bericht automatisch via het juiste kanaal met het juiste
   berichtsjabloon. Zo komt belangrijke informatie sneller en
   betrouwbaarder aan, via het kanaal dat de ontvanger zelf heeft gekozen.

Vervolgstappen

   We ontwikkelen de notificatiedienst verder. Op de agenda staan:
     * Het toevoegen van kanalen naast e-mail.
     * Het toevoegen van alternatieve informatiestromen binnen de
       overheid.
     * Het toevoegen van inhoudelijke aflevering, dus het bericht zelf en
       niet alleen de notificatie erover.
     * Onderzoek naar tweerichtingsverkeer. Naast het versturen van
       notificaties ligt de vraag op tafel wat er nodig is voor
       communicatie in twee richtingen, en welke rol de notificatiedienst
       daarin speelt.

Onze werkwijze

   Bij de ontwikkeling volgen we de werkwijze en principes van MOZa. We
   zoeken actief de samenwerking op, hanteren open standaarden en treden
   op als betrouwbare partij waar privacy en transparantie hoog in het
   vaandel staan. Al bij het ontwerp denken we na over minimale
   dataverwerking en het vastleggen van gegevensverwerkingen.

Meedoen

   De notificatiedienst bouwen we samen met overheidsorganisaties. De
   dienst heeft alleen waarde als organisaties meedoen, dus we nodigen je
   uit om mee te denken, mee te bouwen en kennis te delen. Hiervoor
   organiseren we ook maandelijkse werksessies waarin we samen met elkaar
   aan deze dienstverlening werken. Benieuwd? Sluit je aan en praat met
   ons mee!

Begrippen

   Dienstverlener. Alle overheidsinstanties, zoals gemeenten en
   uitvoeringsorganisaties, die als overheidsinstantie notificeren
   richting burgers en ondernemers.

   Procesapplicatie. Ook wel vakapplicatie. De applicatie die het
   functionele proces uitvoert of regisseert vanuit de dienstverlener.
   Deze bepaalt dat communicatie nodig is, en in welke vorm. In de context
   van de notificatiedienst start deze applicatie de notificatie,
   eventueel via een lokaal OMC. Ook procesherstel wordt hier
   georganiseerd.

   Outputmanagementcomponent (OMC). Decentraal component dat binnen het
   domein van de dienstverlener de regie voert op het notificatieproces en
   lokale koppelingen kan leggen.

   Notificatie Management Component (NMC). Het component dat vanuit
   centraal oogpunt invulling geeft aan de vijf functionaliteiten van de
   notificatiedienst. Een dienstverlener kan alle vijf functionaliteiten
   afnemen, of alleen de onderdelen die nodig zijn.

   Decentraal profiel. De gegevens van een burger of ondernemer die een
   dienstverlener lokaal heeft staan. In de praktijk kan dat uit meerdere
   applicaties of informatiebronnen bestaan.

   Centraal profiel. De gegevens van een burger of ondernemer die centraal
   worden opgeslagen. Ook dit kan in de praktijk uit meerdere bronnen
   bestaan.

   Notify. De centrale verzendcomponent van de notificatiedienst,
   inclusief templating. Hiervoor zetten we de opensourcecomponent
   NotifyNL in.

Meer info

     * Code op GitHub
     * Uitgewerkte flow van de notificatiedienst (sequence diagram)
     * Technische documentatie
     * NotifyNL op GitHub
     * Lees ook over de profielservice, waar contactgegevens en voorkeuren
       worden vastgelegd

    1. Home
    2. Onderwerpen
    3. Notificatiedienst

   (laatst bijgewerkt op 3 november 2026)

   Makkelijk zakendoen met de overheid
