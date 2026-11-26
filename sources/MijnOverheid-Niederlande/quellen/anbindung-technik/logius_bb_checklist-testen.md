# Checklist testen Berichtenbox MijnOverheid – Logius

Quelle: <https://www.logius.nl/onze-dienstverlening/interactie/voorzieningen/mijnoverheid/documentatie/checklist-testen-berichtenbox-mijnoverheid>
Abgerufen: 2026-11-26 (HTML mit `lynx -dump -nolist` in Text umgewandelt; Navigation und Fußzeile entfernt)

---

Checklist Testen Berichtenbox MijnOverheid
   Dit pagina beschrijft hoe en wat uw organisatie moet testen met betrekking tot de
   verschillende koppelingen die uw organisatie aangaat met de services van de Berichtenbox.
Versiegegevens
   Versie: 1.4
1. Inleiding
Doelgroep
   Dit document richt zich op:
     * overheidsorganisaties die gebruik willen maken van de Berichtenbox
     * leveranciers die voor en in opdracht van overheidsorganisaties een aansluiting
       realiseren op de Berichtenbox
Leeswijzer
   Deze checklist helpt uw organisatie bij het testen van de aansluitingen van de
   Berichtenbox. Het beschrijft de testopdracht, een aanpak en het bevat een sjabloon voor het
   testrapport dat uw organisatie dient in te leveren bij Logius.
Suggesties
   Logius vindt het belangrijk dat u snel en zonder problemen van de Berichtenbox van
   MijnOverheid gebruik kunt maken. Deze checklist helpt u daarbij. Heeft u suggesties om dit
   document verder te verbeteren? Stuur die dan op naar Logius.
Overzicht architectuur MijnOverheid
   Overzicht architectuur MijnOverheid
   Figuur 1: Overzicht architectuur MijnOverheid
2. Testopdracht
   Uw organisatie moet aantonen dat het in staat is Berichtenbox-berichten, waarvan het
   BerichtType is opgenomen in het Berichtenbox Leveranciersportaal, aan te leveren bij de
   Berichtenbox, zodat deze correct aan de burger getoond kunnen worden in MijnOverheid.
   Daarnaast dient uw organisatie aan te tonen dat het de Abonnement- en/of
   Berichtenboxvalidatieservice correct kan bevragen.
Testscope
   De volgende koppelingen en bijbehorende functionaliteiten vallen binnen de scope van het
   Berichtenbox checklist testen:
     * Koppeling met het Berichtenbox Leveranciersportaal
     * Koppeling met de Abonnement- en/of Berichtenboxvalidatieservice
     * Koppeling met de Berichtenbox t.b.v. aanleveren Berichtenbox-berichten en ontvangen van
       resultaten.
   Dit zijn de directe koppelpunten vanuit het systeem van uw organisatie met het systeem van
   MijnOverheid, zie Figuur 1.
   Buiten scope van deze checklist en daarmee buiten het gedeelte waarover uw organisatie
   Logius moet informeren, valt het testen van (directe of indirecte) koppelingen vanuit uw
   systeem/organisatie die niet voorkomen in bovengenoemde lijst (bijv. inloggen met DigiD,
   registreren, ontvangen van de e-mailnotificatie vanuit MijnOverheid). Uiteraard kunt u in
   uw eigen (interne) testplan dergelijke testen wel opnemen.
Benodigde documentatie en testbasis
   De inhoud van de volgende documentatie moet bekend zijn voor aanvang van de testen:
     * Koppelvlakspecificaties Berichtenbox
     * Handleiding Aansluiten Berichtenbox
     * Handleiding Berichtenbox Leveranciersportaal
Randvoorwaarden en uitgangspunten
   Voor aanvang van deze test dient Stap 2 uit de Handleiding Aansluiten Berichtenbox
   succesvol uitgevoerd te zijn, dit houdt kort samengevat het volgende in:
     * Uw organisatie is in bezit van een PKIoverheid-certificaat inclusief OIN of HRN (in
       geval van een leveranciersaansluiting)
     * Het systeem van uw organisatie is ingericht en in staat om Berichtenbox-berichten te
       genereren, de interne acceptatie test is succesvol afgerond en er zijn geen openstaande
       blokkerende bevindingen
     * Uw systeem heeft een werkende netwerkverbinding naar de preproductieomgeving van
       MijnOverheid
     * Uw systeem is middels Digikoppeling (ebMS en/of WUS) gekoppeld met de
       preproductieomgeving van MijnOverheid Vanuit uw organisatie kan het Berichtenbox
       Leveranciersportaal (preproductie) benaderd worden middels het ontvangen certificaat
     * Uw organisatie heeft het Berichtenbox Leveranciersportaal (preproductie) ingericht
     * Uw organisatie is in bezit van (minimaal twee) test-DigiD-accounts met de bijbehorende
       test-BSN’s. Test-BSN’s kunnen indien nodig worden opgevraagd bij Logius
3. Testaanpak
   De testaanpak, om op een gestructureerde manier de koppelingen en functionaliteiten te
   testen, onderkent een aantal fases:
    1. Voorbereiding
    2. Uitvoering
    3. Afronding
Voorbereiding
   Tijdens de voorbereiding worden de werkzaamheden uitgevoerd die het mogelijk maken om in
   een korte tijd alle testen uit te voeren en de resultaten te verkrijgen.
Berichtenbox Leveranciersportaal inrichten
   Voor aanvang van de ketentest is het van belang om via het Berichtenbox Leveranciersportaal
   (preproductie) de berichttype(n) aan te maken die bij de test gebruikt zullen worden. Let
   op! De berichttype(n) moeten ook geactiveerd worden! Zie voor meer informatie de
   Handleiding Berichtenbox Leveranciersportaal.
   Daarnaast dienen de profielgegevens (omschrijving) van uw organisatie in het Berichtenbox
   Leveranciersportaal (preproductie) te worden ingevuld, zie hiervoor de Handleiding
   Berichtenbox Leveranciersportaal.
Registreren test-BSN’s bij preproductie MijnOverheid
     * Meldt u aan met de test DigiD-accounts op de preproductieomgeving burgerportaal van
       MijnOverheid (https://preprod.mijn.overheid.nl) om accounts te registeren, op die
       manier worden de test-BSN’s in het systeem geladen en kunnen berichten naar deze BSN’s
       gestuurd worden of de status ervan worden opgevraagd.
     * Maak minimaal twee MijnOverheid-accounts aan, één met de berichtvoorkeur voor uw
       organisatie ingeschakeld, en één met de berichtvoorkeur voor uw organisatie
       uitgeschakeld. Dit doet u als volgt:
          + meldt u aan met de test DigiD-accounts op de preproductieomgeving van
            MijnOverheid,
          + open Mijn account
          + kies voor ‘Organisaties Berichtenbox’. Hier ziet u uw organisatiegegevens staan.
     * Controleer of deze gegevens correct zijn.
     * Vervolgens kunt u de voorkeur aan- of uitvinken.
Gereedstellen testdata
   Zorg ervoor dat alle testgegevens bekend en beschikbaar zijn en afgeleverd kunnen worden
   bij de Berichtenbox. Voor alle testen die uitgevoerd moeten worden, zijn testgegevens
   nodig, zorg dat u hierin volledig bent; zie het sjabloon uit dit document voor de minimale
   vereisten. Raadpleeg hierbij ook de Koppelvlakspecificaties Berichtenbox. Naast de
   testgegevens dient u mogelijk ook systemen te configureren en gereed te stellen om de
   testgegevens te generen en af te leveren.
Inplannen testen
   Het is raadzaam het netwerkoverzicht te controleren, gemaakt in Stap 2 van de Handleiding
   Aansluiten Berichtenbox, op een correcte weergave van de testopstelling en de juistheid van
   contactgegevens van betrokken beheerorganisaties. Aansluitend kunnen de koppelingen kort
   gecontroleerd worden, zoals beschreven in Stap 2 in de Handleiding Aansluiten Berichtenbox.
   Zodra uw organisatie klaar is met de voorbereiding kunt u contact opnemen met Logius om de
   periode af te stemmen waarin de testen plaats gaan vinden.
Uitvoering
   Indien de voorbereidende werkzaamheden zijn uitgevoerd, kan aangevangen worden met het
   uitvoeren van de testen. Voor de inhoud van de testen zie, naast uw eigen testscenario’s,
   het sjabloon uit hoofdstuk 4.
   Indien u onverhoopt tijdens het testen tegen beschikbaarheidsproblemen aanloopt van de
   preproductieomgeving van MijnOverheid, neem dan contact op met de aansluitcoördinator.
Afronding
   Na afronden van de geplande testen en het opstellen van testrapporten, raadt Logius aan om
   testware (testberichten, -tooling etc.) te bewaren. Zorg ervoor dat alle acceptanten de
   testrapportage hebben geaccepteerd alvorens de resultaten met Logius te delen. Indien
   Logius uw testrapport heeft goedgekeurd dan is het aansluiten op Preproductie succesvol
   afgerond. Uw organisatie kan beginnen met de aansluiting op productie.
4. Sjabloon: Berichtenbox Testrapport
   Het onderstaande sjabloon kunt u gebruiken bij de voorbereiding van uw testplan en kunt u,
   ingevuld, toesturen naar Logius ten behoeve van het goedkeuren van de aansluiting van uw
   organisatie op de Berichtenbox.
Organisatie gegevens
   Algemeen
   Datum test: ___________________________
   Organisatie: ___________________________
   Testrapport ingevuld door: ___________________________
   E-mail: ___________________________
   Telefoonnummer: ___________________________
   □ Ondergetekende verklaard namens bovenstaande organisatie het Berichtenbox Testrapport
   volledig naar waarheid te hebben ingevuld.
   Aansluiting
   Opvragen geabonneerden, aangesloten op (minimaal één aankruisen):
   □ Abonnementservice
   □ Berichtenboxvalidatieservice
   OIN: ___________________________
   Handtekening: ___________________________ Datum: ___________________________
5. Checklist Berichtenbox Testen
   Uitgangspunt: gebruiker A heeft de berichtvoorkeur voor uw organisatie ingeschakeld,
   gebruiker B heeft de berichtvoorkeur voor uw organisatie uitgeschakeld.
   Mocht uw (zaak)systeem geen mogelijkheid bieden om niet-geabonneerde burgers een bericht te
   sturen, dan volstaat een notitie van de leverancier en/of de functioneel beheerder van het
   (zaak)systeem. Bij onderdelen voorzien van een * dient u een schermafbeelding toe te
   voegen. Ten aanzien van onderdeel 3; afhankelijk van uw keuze test u de door u
   geïmplementeerde service.
   Nr Omschrijving Toelichting / Resultaat
     Inrichten Berichtenbox Leveranciersportaal
   1* Invoeren profielgegevens (omschrijving) van uw organisatie. Controleer via de
   preproductieomgeving burgerportaal van MijnOverheid op correcte weergave.
   2* Aanmaken berichttype(n) t.b.v. test.
     Testen opvragen geabonneerden (minimaal één invullen)
   3a
     * Bevraag de Abonnementservice (indien aangesloten) voor burgers met de berichtvoorkeur
       ingeschakeld voor uw organisatie.
     * Controleer of gebruiker A voorkomt en gebruiker B niet voorkomt in het antwoordbericht
       (indien mogelijk).
   3b
     * Bevraag de Berichtenboxvalidatieservice (indien aangesloten) voor burgers met de
       berichtvoorkeur ingeschakeld voor uw organisatie.
     * Controleer of gebruiker A voorkomt en gebruiker B niet voorkomt in het antwoordbericht.
       (indien mogelijk).
     Aanleveren Berichtenbox-berichten
   4 Batch 1: verstuur Berichtbatch met 1 bericht voor gebruiker A.
   5 Batch 2: verstuur Berichtbatch met 1 bericht voor gebruiker A en 1 bericht voor gebruiker
   B. (indien mogelijk).
     Controleren berichtverwerking
   6 Controleer resultaten retourbericht (ebMS). Batch 1: 1 bericht correct verwerkt. Batch 2:
   1 bericht correct verwerkt, 1 bericht afgekeurd wegens ‘geen actieve box of geen actieve
   voorkeur’.
   7*
   Controleer resultaten in Berichtenbox Leveranciersportaal.
     * Batch 1: 1 bericht correct verwerkt.
     * Batch 2: 1 bericht correct verwerkt, 1 bericht afgekeurd wegens ‘geen actieve box of
       geen actieve voorkeur’.
   Let op: In de preproductie omgeving worden geen e-mailnotificaties gestuurd,
   8*
   Controleer via de preproductieomgeving burgerportaal van MijnOverheid op correcte weergave
   van de verstuurde Berichtenbox-berichten.
   Log in met:
     * Gebruiker A: heeft 2 berichten.
     * Gebruiker B: heeft geen berichten.
   Berichten worden juist getoond (onderwerp, berichttekst en naam van te downloaden bijlage).
   9 Open de bijlagen voor gebruiker A via de Berichtenbox op de preproductieomgeving
   burgerportaal van MijnOverheid. De bijlagen zijn conform de gestuurde bijlagen.
   10 Controleer of de bijlagen voldoen aan PDF richtlijnen ( PDF/A compliance1).
   * screenshot toevoegen
Screenshots
   Voeg hieronder de screenshots in van de resultaten bij de bovenstaande testen waarbij dit
   is aangegeven.
   Nr                   Screenshot
   1  [voeg screenshot in van resultaat van stap 1]
   2  [voeg screenshot in van resultaat van stap 2]
   7  [voeg screenshot in van resultaat van stap 7]
   8  [voeg screenshots in van resultaat van stap 8]
Voorbeelden Screenshots
   Hieronder worden voorbeelden weergegeven van de screenshots die u dient in te voegen.
   Voorbeeld screenshot resultaat van stap 1 – Weergave profielgegevens organisatie
   Voorbeeld screenshot resultaat van stap 1 – Weergave profielgegevens organisatie
   Voorbeeld screenshot resultaat van stap 2 – Aangemaakt berichttype
   Voorbeeld screenshot resultaat van stap 2 – Aangemaakt berichttype
   Voorbeeld screenshot resultaat van stap 7 – Resultaten Leveranciersportaal
   Voorbeeld screenshot resultaat van stap 7 – Resultaten Leveranciersportaal
   Voorbeeld screenshot resultaat van stap 8 – Gebruiker A heeft 2 berichten
   Voorbeeld screenshot resultaat van stap 8 – Gebruiker A heeft 2 berichten
   Voorbeeld screenshot resultaat van stap 8 – Gebruiker B heeft geen berichten
   Voorbeeld screenshot resultaat van stap 8 – Gebruiker B heeft geen berichten
6. Limited productierun
   De limited productierun (LPR) is het eerste gebruik van de nieuwe koppelingen in productie.
   Logius adviseert u om een beperkte productietest uit te voeren, voordat uw organisatie het
   kanaal Berichtenbox volledig in zet voor communicatie met de burger. In deze beperkte
   productierun dienen alle koppelingen die zijn gelegd gecontroleerd te worden en moet vanuit
   gebruikersoptiek de gehele keten (registeren, inloggen, inzien bericht etc.) doorgelopen
   worden.
   Voor de LPR selecteert uw organisatie zelf een aantal betrokkenen bij het aansluitproject.
   In de productierun worden de berichtenboxen van de betrokkenen gebruikt en worden hierin
   één of meerdere berichten in geplaatst om de keten te controleren. Naast het aanleveren van
   berichten en het controleren van de uiteindelijke weergave in de Berichtenbox van de
   burger, dient u ook de functies voort het opvragen van geabonneerde te controleren in de
   LPR.
   Uw organisatie moet dus in staat zijn productieberichten naar een kleine geselecteerde
   groep te sturen. Mogelijk moet hiervoor contact gezocht worden met de lijnverantwoordelijke
   voor het verzenden van deze eerste reeks productieberichten en de inhoud daarvan.
   Indien u een probleem constateert bij het uitvoeren van de LPR, breng Logius dan op de
   hoogte, zodat eventueel de koppeling ongedaan kan worden gemaakt.
   Voor de LPR adviseert Logius om de testen 1 en 3 t/m 9 van het sjabloon uit te voeren,
   aangevuld met het controleren van ontvangst van de e-mailnotificatie (dit kan niet getest
   worden in de preproductie omgeving). Eventueel kan ook het registratieproces worden
   meegenomen. Controleer bij de LPR of alles, de informatie over uw organisatie en de
   berichten die u verstuurd heeft, correct en naar wens worden weergegeven in MijnOverheid.
