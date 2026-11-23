<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Glossar.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Glossar.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 6de6f6cebb); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# Glossar

[A](#a) · [B](#b) · [D](#d) · [E](#e) · [F](#f) · [G](#g) · [H](#h) · [I](#i) · [L](#l) · [M](#m) · [N](#n) · [O](#o) · [P](#p) · [Q](#q) · [R](#r) · [S](#s) · [T](#t) · [U](#u) · [V](#v) · [W](#w) · [Z](#z)

## Aufbau der Einträge

Unter der Überschrift stehen alternative Begriffe, wobei nach dem Kürzel EN der englische Begriff ggf. auch mit Alternativen folgt. Auf die deutsche Definition folgt bei vielen Einträgen eine englische Definition, gekennzeichnet mit EN. Abkürzungen sind im [Abkürzungsverzeichnis](Abkürzungen.md) ausgeschrieben.

## A

### Abrufbestätigung

EN *retrieval confirmation*

Die Abrufbestätigung ist eine Bestätigung an die [Absender:in](#absenderin), dass eine [Nachricht](#nachricht) oder ein [Anhang](#anhang) im [Postfach-Zugang](#postfach-zugang) der [Empfänger:in](#empfängerin) abgerufen und entschlüsselt wurde. Sie ist nicht erzwingbar und begründet keine Fiktion. Sie ist ein frei zu würdigendes Beweismittel für den tatsächlichen Abruf. Abzugrenzen sind die serverseitige [Bereitstellungsbestätigung](#bereitstellungsbestätigung) und die aktiv abgegebene [Lesebestätigung](#lesebestätigung).

**EN:** Receiving Matrix client confirms receiving and successfully decrypting a message.

### Absender:in

EN *sender*

Absender:in ist eine [Nutzer:in](#nutzerinnen), die eine [Nachricht](#nachricht) erstellt und versendet. Für eine [Organisation](#organisation) versenden in der Regel Mitarbeitende oder ein [Fachverfahren](#fachverfahren) aus einem [Funktionspostfach](#funktionspostfach).

### ADR

EN *Architecture Decision Record (ADR)*

Ein ADR (Architecture Decision Record) hält Architekturentscheidungen mit ihrer Begründung fest. Die ADRs der Zielarchitektur folgen MADR 4.0 (vgl. [ADR-0000](Zielarchitektur/Architekturentscheidungen/ADR-0000-Dokumentation-von-Architekturentscheidungen.md)).

**EN:** An Architectural Decision (AD) is a justified software design choice that addresses a functional or non-functional requirement of architectural significance. An Architectural Decision Record (ADR) captures a single AD and its rationale (source: adr.github.io[^22]).

### Adressbuch

EN *address book*

Ein Adressbuch ist ein Verzeichnis, mit dem [Nutzer:innen](#nutzerinnen) die Postfachadressen von [Empfänger:innen](#empfängerin) ermitteln können. Das [Adressregister](#adressregister) setzt es für [Organisationen](#organisation) um. Ein verbundweites Verzeichnis der Adressen von [Privatpersonen](#privatperson) gibt es nicht (vgl. [Bereichs-MXID](#bereichs-mxid)).

### Adressierungsmetadaten

EN *addressing metadata*

Adressierungsmetadaten sind die Metadaten einer [Nachricht](#nachricht), die die beteiligten [Homeserver](#homeserver) für den Transport benötigen, etwa die [MXID](#mxid) von [Absender:in](#absenderin) und [Empfänger:in](#empfängerin), den [Vorgangsraum](#vorgangsraum) und den Zeitpunkt. Sie sind für die Transportinfrastruktur einsehbar. Adressen von [Privatpersonen](#privatperson) sind dabei pseudonym ([Bereichs-MXID](#bereichs-mxid)). Abzugrenzen sind [vertrauliche Metadaten](#vertrauliche-metadaten).

### Adressregister

EN *address register*

Das Adressregister führt die Zuordnung der Postfachadressen ([MXID](#mxid)) von [Organisationen](#organisation), etwa von [Funktionspostfächern](#funktionspostfach), zu ihrer [Organisationsidentität](#organisationsidentität). Es setzt das [Adressbuch](#adressbuch) für Organisationen um. Jedes [Postfach-Backend](#postfach-backend) führt die Einträge seiner Domain (vgl. [Adressregister-Abfrage](#adressregister-abfrage)). Adressen von [Privatpersonen](#privatperson) werden darin nicht geführt.

### Adressregister-Abfrage

auch MIS · EN *address register query*

Die Adressregister-Abfrage ist die Abfrage des [Adressregisters](#adressregister) im [Verbund](#verbund). Darüber werden Adressen anhand von Merkmalen einer [Organisation](#organisation) gesucht und die Organisationszugehörigkeit einer bekannten Adresse anhand ihres signierten Registereintrags geprüft. In früheren Fassungen hieß die Schnittstelle dafür MIS (Matrix Identity Service). Sie ist an die Identity Service API der Matrix-Spezifikation nur angelehnt.

### Anhang

EN *attachment*

Ein Anhang ist eine Datei, die mit einer [Nachricht](#nachricht) übermittelt wird. Die Nachricht enthält nur einen Verweis auf die verschlüsselte Datei und deren Prüfsumme. In den [Machtbereich](#machtbereich) der [Empfänger:in](#empfängerin) gelangt der Anhang erst, wenn das [Postfach-Backend](#postfach-backend) der Empfangsseite die Datei abgerufen hat (vgl. [ADR-1120](Zielarchitektur/Architekturentscheidungen/ADR-1120-Beweissichere-Übermittlung-von-Anhängen.md)).

### Anschlussbedingungen

EN *terms of participation*

Die Anschlussbedingungen sind die verbindlichen Mindestanforderungen für die Teilnahme am [Verbund](#verbund) (vgl. [Anschlussvertrag](#anschlussvertrag)). Sie gelten für die Betreiber von [Postfach-Backends](#postfach-backend) und für die angeschlossenen [Organisationen](#organisation). Für eine Organisation umfassen sie die Komponenten in ihrem [Machtbereich](#machtbereich). Bei Verstößen kann ein Backend ausgeschlossen werden (vgl. [Föderationskontrolle](#föderationskontrolle)).

### Anschlussvertrag

EN *participation agreement*

Mit dem Anschlussvertrag werden die [Anschlussbedingungen](#anschlussbedingungen) verbindlich. Betreiber von [Postfach-Backends](#postfach-backend) und angeschlossene [Organisationen](#organisation) nehmen ihn im Onboarding im [Self-Service-Portal](#self-service-portal) an. Mit ihm übernimmt die Organisation die Haftung für die in ihrem [Machtbereich](#machtbereich) betriebenen Komponenten. Er regelt die Kriterien für Betreiber und die Service-Level.

### Antrag

EN *application*

Ein Antrag ist die Erklärung, mit der eine Person oder [Organisation](#organisation) bei einer [Behörde](#behörde) eine Verwaltungsleistung begehrt. In Antragsverfahren leitet er das [Verwaltungsverfahren](#verwaltungsverfahren) ein (vgl. § 22 VwVfG[^1]). Er wird in der Regel über einen [Onlinedienst](#onlinedienst) gestellt.

**EN:** The term application describes the initial request for a public service usually sent using an online service.

### Antragsteller:in

EN *applicant*

Antragsteller:in ist eine [Privatperson](#privatperson) oder [Organisation](#organisation), die einen [Antrag](#antrag) auf eine Verwaltungsleistung stellt.

**EN:** An applicant is a private individual or organisation filing an application for a public service.

### Auskunftsanfrage

EN *data subject access request (DSAR)*

Betroffene Personen können vom Verantwortlichen Auskunft darüber verlangen, ob und welche sie betreffenden personenbezogenen Daten verarbeitet werden, einschließlich Informationen zu den Verarbeitungszwecken, der Herkunft, den Empfängern, der Speicherdauer, einer automatisierten Entscheidungsfindung und ihren weiteren Rechten. Grundsätzlich haben sie auch das Recht, eine Kopie ihrer Daten zu erhalten (Quelle: Art. 15 DSGVO[^18]).

**EN:** Data subjects may request information from the controller as to whether and which personal data concerning them are being processed, including information regarding the purposes of processing, the source, recipients, storage duration, automated decision-making, and their other rights. In principle, they also have the right to obtain a copy of their data.

### Authentifizierung

EN *authentication*

Die Authentifizierung ist die Prüfung einer [Authentisierung](#authentisierung) durch die [vertrauende Entität](#vertrauende-entität). Geprüft werden Herkunft, Echtheit und Gültigkeit der [Identität](#identität) oder der übermittelten Daten (Quelle: i.S.v. BSI TR-03107-1[^2]).

**EN:** Authentication is the act of verifying a subject's identity (source: BSI TR-03160-1[^24]).

### Authentisierung

EN *authentication (claimant side)*[^3]

Eine Authentisierung ist das Versehen einer [Identität](#identität) oder anderer übermittelter Daten mit Metadaten, die es einer [vertrauenden Entität](#vertrauende-entität) ermöglichen, die Herkunft, Echtheit und Gültigkeit der Identität oder der Daten zu überprüfen. Der Überprüfungsvorgang durch die vertrauende Entität ist die [Authentifizierung](#authentifizierung) der Identität/der Daten (Quelle: BSI TR-03107-1[^2]).

### Authentisierungsmittel

auch Identifizierungsmittel · EN *authentication means, electronic identification means*

Authentisierungsmittel sind technische Mittel, die es der Inhaber:in erlauben, eine [Identität](#identität) oder andere übermittelte Daten zu authentisieren (Quelle: i.S.v. BSI TR-03107-1[^2]). Ein elektronisches Identifizierungsmittel ist eine materielle und/oder immaterielle Einheit, die Personenidentifizierungsdaten enthält und zur [Authentifizierung](#authentifizierung) bei Online-Diensten oder gegebenenfalls bei Offline-Diensten verwendet wird (Quelle: Art. 3 Nr. 2 eIDAS-VO[^4]).

**EN:** Authentication means are technical means that allow the holder to authenticate an identity (that is, a set of identity attributes) or other transmitted data. Examples of authentication means include passwords, identity cards, or cryptographic tokens (source: BSI TR-03107-1[^2]).

### Authentisierungssystem

EN *authentication system*

Ein Authentisierungssystem ist die Gesamtheit der technischen Infrastruktur einschließlich organisatorischer Prozesse und rechtlicher Rahmenbedingungen, die die [Authentisierung](#authentisierung) und [Authentifizierung](#authentifizierung) mittels [Authentisierungsmitteln](#authentisierungsmittel) ermöglichen (Quelle: BSI TR-03107-1[^2]). In der Zielarchitektur sind das für [Privatpersonen](#privatperson) die [BundID](#bundid), die [eID-Funktion](#eid-funktion) und perspektivisch die [Europäische Brieftasche für die Digitale Identität](#europäische-brieftasche-für-die-digitale-identität), für [Organisationen](#organisation) das [Org-Konto](#org-konto).

### Authentizität

EN *authenticity*

Authentizität ist die Eigenschaft, die gewährleistet, dass eine Kommunikationspartner:in tatsächlich diejenige ist, die sie vorgibt zu sein. Bei authentischen Informationen ist sichergestellt, dass sie von der angegebenen Quelle erstellt wurden. Der Begriff wird auch für IT-Komponenten oder Anwendungen verwendet (Quelle: in Anlehnung an BSI IT-Grundschutz-Kompendium[^5]).

**EN:** The term authenticity refers to the property of being genuine and able to be verified and be trusted ("authentic"). In Neo, we refer to a communication channel as an 'authenticated channel' if all participants have proven their identity to each other.

### Autorisierung

EN *authorisation*

Die Autorisierung ist die Zuordnung und Überprüfung von Rechten zu einer [Entität](#entität), zum Beispiel Zugriffsrechte oder das Recht, eine bestimmte Anwendung zu nutzen. Eine Autorisierung erfolgt immer anwendungsbezogen (Quelle: i.S.v. BSI TR-03107-1[^2]).

**EN:** The function of assigning access privileges to an actor. For example, an actor may be authorised to access a certain resource after validating the authenticity of their identity (source: BSI TR-03107-1[^2]).

## B

### Bearbeitungsvertretung

auch Vertretungsregelung · EN *substitution arrangement, substitution rule*

Die Bearbeitungsvertretung ist die organisationsinterne Vertretung von [Postfachmitgliedern](#postfachmitglieder) bei der Bearbeitung der [Nachrichten](#nachricht) eines [Funktionspostfachs](#funktionspostfach). Sie wird im [Berechtigungsverzeichnis](#berechtigungsverzeichnis) gepflegt. Der [Raum-Management-Service](#raum-management-service) lädt die vertretenden Personen in die [Vorgangsräume](#vorgangsraum) ein, ohne dass die zuständige Person mitwirken muss. Abzugrenzen ist die [externe Vertretung](#externe-vertretung), bei der für eine andere Partei gehandelt wird.

### Behörde

auch Fachbehörde, zuständige Behörde · EN *authority, competent authority, public authority*[^6]

Eine Behörde ist jede Stelle, die Aufgaben der öffentlichen Verwaltung wahrnimmt (Quelle: § 1 Abs. 4 VwVfG[^1], gleichlautend § 2 Abs. 9 OZG[^7]). Behörden gehören zu den [öffentlichen Stellen](#öffentliche-stelle).

**EN:** Any body or authority at national, regional, or local level with specific responsibilities for at least one public service (usually provided online).

### Bekanntgabe

EN *notification*

Die Bekanntgabe ist die amtliche Eröffnung eines [Verwaltungsakts](#verwaltungsakt) gegenüber den Beteiligten, für die er bestimmt ist oder die von ihm betroffen werden, mit Wissen und Wollen der erlassenden [Behörde](#behörde) (vgl. § 41 Abs. 1 VwVfG[^1]). Voraussetzung ist sein [Zugang](#zugang) (vgl. [Bereitstellungsbestätigung](#bereitstellungsbestätigung)). Die [Zustellung](#zustellung) ist eine förmliche Form der Bekanntgabe.

### Bekanntgabefiktion

EN *deemed notification*

Die Bekanntgabefiktion ist eine gesetzliche Regel, nach der ein [Verwaltungsakt](#verwaltungsakt), der ohne förmliche [Zustellung](#zustellung) übermittelt wird, an einem bestimmten Tag als bekannt gegeben gilt. Ein Verwaltungsakt gilt etwa am vierten Tag nach der Aufgabe zur Post im Inland oder nach der elektronischen Absendung als bekannt gegeben (§ 41 Abs. 2 VwVfG[^1]). Bei einer [Bekanntgabe](#bekanntgabe) durch Abruf mit Einwilligung der Beteiligten ist es der Tag nach dem Abruf (§ 41 Abs. 2a VwVfG). Bei der Bereitstellung zum Abruf im [Postfach](#postfach) eines [Nutzerkontos](#nutzerkonto) ist es der vierte Tag nach der Bereitstellung (§ 9 Abs. 1 OZG[^7]). Im Zweifel hat die [Behörde](#behörde) den [Zugang](#zugang) bzw. die Bereitstellung und den jeweiligen Zeitpunkt nachzuweisen (vgl. [Bereitstellungsbestätigung](#bereitstellungsbestätigung)).

**EN:** This legal fiction means that an administrative act that is sent without formal service is deemed to have been notified on a certain day, e.g. on the fourth day after electronic sending or on the fourth day after it was made available for retrieval in the mailbox of a user account. Such fictions are intended to ensure legal certainty in communication between the parties.

### Berechtigungsverzeichnis

EN *permission directory*

Das Berechtigungsverzeichnis führt die Postfachmitgliedschaften, Rollen und [Bearbeitungsvertretungen](#bearbeitungsvertretung) für die [Funktionspostfächer](#funktionspostfach) einer [Organisation](#organisation). Die Berechtigungen werden durch den [Raum-Management-Service](#raum-management-service) in den [Vorgangsräumen](#vorgangsraum) durchgesetzt. Wer es betreibt, hängt vom [Betriebsmodell](#betriebsmodell) ab. Abzugrenzen ist das [Adressregister](#adressregister), das die Postfachadressen führt.

### Bereich

EN *administrative sector*

Ein Bereich umfasst eine oder mehrere [Behörden](#behörde), gegenüber denen eine [Privatperson](#privatperson) dieselbe [Bereichs-MXID](#bereichs-mxid) verwendet. Der Registereintrag einer Behörde im [Adressregister](#adressregister) gibt den Bereich an, dem sie angehört.

### Bereichs-MXID

EN *sector-specific MXID*

Die Bereichs-MXID ist die pseudonyme [MXID](#mxid) einer [Privatperson](#privatperson) für einen [Bereich](#bereich). [Behörden](#behörde) verschiedener Bereiche nutzen unterschiedliche Adressen derselben Person und können deren Kommunikation nicht über die Adresse verketten (vgl. [Nichtverkettbarkeit](#nichtverkettbarkeit)). Bei einem neuen Ausweis oder einem weiteren [Authentisierungssystem](#authentisierungssystem) bleibt die Adresse gleich (vgl. [Zuordnungsdienst](#zuordnungsdienst)).

### Bereitstellungsbestätigung

EN *delivery confirmation*

Die Bereitstellungsbestätigung ist ein automatisch erzeugter Nachweis, dass eine [Nachricht](#nachricht) zu einem bestimmten Zeitpunkt im [Postfach-Backend](#postfach-backend) der [Empfänger:in](#empfängerin) eingegangen und damit in ihren [Machtbereich](#machtbereich) gelangt ist. Der [Quittungsdienst](#quittungsdienst) stellt sie mit dem [fortgeschrittenen elektronischen Siegel](#fortgeschrittenes-elektronisches-siegel) des Betreibers und einem [qualifizierten elektronischen Zeitstempel](#qualifizierter-elektronischer-zeitstempel) aus, ohne den Klartext lesen zu können. Die Bestätigung belegt die Bereitstellung zum Abruf und deren Zeitpunkt. Daran knüpft § 9 Abs. 1 OZG[^7] die [Bekanntgabefiktion](#bekanntgabefiktion), bisher aber nur für das [Postfach](#postfach) eines [Nutzerkontos](#nutzerkonto) (vgl. [ADR-0018](Zielarchitektur/Architekturentscheidungen/ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). Wird im [elektronischen Rechtsverkehr](#elektronischer-rechtsverkehr) an andere als die in § 173 Abs. 2 ZPO[^8] Genannten zugestellt, etwa an [Privatpersonen](#privatperson), dient die Bestätigung als automatisierte Eingangsbestätigung nach § 173 Abs. 4 ZPO (vgl. [Zustellfiktion](#zustellfiktion)).

**EN:** Receiving Matrix server (automatically) confirms receiving a message.

### Bescheid

EN *administrative decision, notice*[^6]

Ein Bescheid ist ein schriftlich oder elektronisch erlassener [Verwaltungsakt](#verwaltungsakt), etwa ein Steuerbescheid. Mit dem Bescheid schließt die [Behörde](#behörde) ein [Verwaltungsverfahren](#verwaltungsverfahren) in der Regel ab.

**EN:** An administrative decision is an official document containing a specific, individual order or finding issued by an administrative authority. Under German administrative law, an administrative decision must contain at least one administrative act.

### Betriebsmodell

EN *operating model*

Das Betriebsmodell legt fest, welche Komponenten eine [Organisation](#organisation) selbst betreibt und welche zentral bereitgestellten Dienste sie nutzt. Unterschieden werden Minimalanbindung, Teilbetrieb mit eigenem [Raum-Management-Service](#raum-management-service) und souveräner Betrieb mit eigenem [Postfach-Backend](#postfach-backend). Die Organisation wählt das Betriebsmodell selbst und kann es später wechseln.

### BundID

EN *BundID*

Die BundID ist das zentrale Bürgerkonto des Bundes, ein [Nutzerkonto](#nutzerkonto) für [Privatpersonen](#privatperson) (vgl. § 3 Abs. 1 OZG[^7]). In der Zielarchitektur ist sie ein [Authentisierungssystem](#authentisierungssystem) für Privatpersonen. Das in die BundID integrierte [Zentrale Bürgerpostfach](#zentrales-bürgerpostfach) wird abgelöst. Die BundID enthält dann kein [Postfach](#postfach) mehr (vgl. [ADR-1110](Zielarchitektur/Architekturentscheidungen/ADR-1110-Entkopplung-von-Nutzerkonto-und-Postfach.md)).

## D

### Datenschutz-Folgenabschätzung

auch DSFA · EN *data protection impact assessment (DPIA)*

Eine Datenschutz-Folgenabschätzung muss vor Beginn der Verarbeitung durchgeführt werden, wenn die Verarbeitung voraussichtlich ein hohes Risiko für die Rechte und Freiheiten natürlicher Personen zur Folge hat, insbesondere bei der umfangreichen Verarbeitung sensibler Daten. Sie beschreibt die Verarbeitung, bewertet ihre Notwendigkeit und Risiken und legt Schutzmaßnahmen fest (Quelle: Art. 35 DSGVO[^18]).

**EN:** A data protection impact assessment must be carried out prior to the start of processing if the processing is likely to result in a high risk to the rights and freedoms of natural persons, particularly in the case of large-scale processing of sensitive data. It describes the processing, assesses its necessity and risks, and establishes protective measures.

### Drittanwendung

EN *third-party system*

Eine Drittanwendung ist ein System, das nicht Bestandteil des [Systems](#system) ist und mit ihm interagiert, etwa ein [Fachverfahren](#fachverfahren), ein [Onlinedienst](#onlinedienst), ein Prozessmanagementsystem oder eine Kanzleisoftware. Welche Aufgabe die Anwendung hat, ist für den Begriff unerheblich.

## E

### E2EE-Endpunkt

EN *E2EE endpoint*

Der E2EE-Endpunkt ist das System, in dem die [Ende-zu-Ende-Verschlüsselung](#ende-zu-ende-verschlüsselung) (E2EE) endet und die [Nachrichten](#nachricht) im Klartext vorliegen. Zulässig sind nur das Endgerät der [Nutzer:in](#nutzerinnen) und Systeme im [Machtbereich](#machtbereich) der [Organisation](#organisation), etwa ein [technischer Nutzer](#technischer-nutzer) mit Schlüsselbesitz auf einem Server der Organisation. Der [Homeserver](#homeserver) ist kein E2EE-Endpunkt.

### Echtzeitkommunikation

EN *real-time communication*

Echtzeitkommunikation ist ein Austausch von [Nachrichten](#nachricht), bei dem [Nutzer:innen](#nutzerinnen) ohne spürbare Verzögerung miteinander kommunizieren.

### eID-Funktion

auch Online-Ausweisfunktion · EN *eID function*

Die eID-Funktion ist die Online-Ausweisfunktion des Personalausweises, des elektronischen Aufenthaltstitels und der eID-Karte. Mit ihr weisen Inhaber:innen ihre [Identität](#identität) gegenüber öffentlichen und nichtöffentlichen Stellen elektronisch nach (Quelle: i.S.v. § 18 Abs. 1 PAuswG[^13]). Sie ist ein [Authentisierungsmittel](#authentisierungsmittel) mit dem [Vertrauensniveau](#vertrauensniveau) hoch. [Privatpersonen](#privatperson) nutzen sie in der Zielarchitektur über die [BundID](#bundid) oder über einen eID-Server der Gegenstelle (vgl. [ADR-1106](Zielarchitektur/Architekturentscheidungen/ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)).

### Eingangsbenachrichtigung

auch Eingangsbenachrichtigung Empfänger:in · EN *new message alert*

Die Eingangsbenachrichtigung ist ein Hinweis an die [Empfänger:in](#empfängerin) über einen anderen Kanal auf eine neue [Nachricht](#nachricht) im [Postfach](#postfach). Namen und Nachrichteninhalte werden dabei nicht im Klartext über Push-Server Dritter übertragen. Bei der [Bekanntgabe](#bekanntgabe) durch Bereitstellung zum Abruf ist sie gesetzlich vorgesehen (z. B. § 9 Abs. 1 Satz 6 OZG[^7]).

### elektronische Signatur

auch einfache elektronische Signatur · EN *electronic signature, simple electronic signature*

Eine elektronische Signatur besteht aus Daten in elektronischer Form, die anderen elektronischen Daten beigefügt oder logisch mit ihnen verbunden werden und die der Unterzeichner zum Unterzeichnen verwendet (Quelle: i.S.v. Art. 3 Nr. 10 eIDAS-VO[^4]). Ohne weitere Anforderungen heißt sie auch einfache elektronische Signatur. Kryptografische Verfahren sind dafür nicht erforderlich, anders als bei der [fortgeschrittenen elektronischen Signatur](#fortgeschrittene-elektronische-signatur).

### elektronischer Rechtsverkehr

auch ERV · EN *electronic legal communication (ERV)*

Der elektronische Rechtsverkehr ist die elektronische Kommunikation mit Gerichten in gerichtlichen Verfahren und mit Strafverfolgungsbehörden im Strafverfahren. Er umfasst die Einreichung elektronischer Dokumente bei Gericht und die elektronische [Zustellung](#zustellung) durch das Gericht. Die technischen Rahmenbedingungen regelt die ERVV[^9].

### elektronischer Schriftverkehr

EN *electronic correspondence*

Der elektronische Schriftverkehr ist der Austausch von [Nachrichten](#nachricht) einschließlich ihrer [Anhänge](#anhang).

### elektronisches Einschreiben

EN *electronic registered delivery*

Ein elektronisches Einschreiben wird über einen Dienst für die Zustellung elektronischer Einschreiben versandt. Dieser ermöglicht die Übermittlung von Daten zwischen Dritten mit elektronischen Mitteln, weist ihre Handhabung einschließlich Absendung und Empfang nach und schützt sie vor Verlust, Diebstahl, Beschädigung oder unbefugter Veränderung (Quelle: i.S.v. Art. 3 Nr. 36 eIDAS-VO[^4]). Trotz dieser Bezeichnung ist ein elektronisches Einschreiben für sich genommen keine förmliche [Zustellung](#zustellung) nach VwZG oder ZPO.

### elektronisches Empfangsbekenntnis

auch eEB · EN *electronic acknowledgement of receipt (eEB)*

Mit dem elektronischen Empfangsbekenntnis bestätigt die [Empfänger:in](#empfängerin) die Entgegennahme eines elektronisch zugestellten Dokuments und das Empfangsdatum. Damit wird die [Zustellung](#zustellung) nachgewiesen (§ 173 Abs. 3 ZPO[^8], § 5 Abs. 7 VwZG[^10]). Geht der [Behörde](#behörde) in den Fällen des § 5 Abs. 5 Satz 2 VwZG bis zum vierten Tag nach der Absendung kein Empfangsbekenntnis zu, gilt das Dokument als zugestellt (vgl. [Zustellfiktion](#zustellfiktion)). In der Zielarchitektur kann eine aktiv und zurechenbar abgegebene [Lesebestätigung](#lesebestätigung) als elektronisches Empfangsbekenntnis dienen (vgl. [ADR-1118](Zielarchitektur/Architekturentscheidungen/ADR-1118-Abruf-und-Lesebestätigungen.md)).

### elektronisches Siegel

EN *electronic seal*

Elektronische Siegel sind Daten in elektronischer Form, die anderen Daten in elektronischer Form beigefügt oder logisch mit ihnen verbunden werden, um deren Ursprung und [Unversehrtheit](#integrität) sicherzustellen (Quelle: Art. 3 Nr. 25 eIDAS-VO[^4]). Siegelersteller ist eine juristische Person, Unterzeichner einer [elektronischen Signatur](#elektronische-signatur) eine [natürliche Person](#privatperson) (Art. 3 Nr. 9 und 24 eIDAS-VO).

### Empfänger:in

EN *recipient*

Empfänger:in ist eine [Nutzer:in](#nutzerinnen), an die eine [Nachricht](#nachricht) adressiert ist. Bei einer [Organisation](#organisation) geht die Nachricht in der Regel an ein [Funktionspostfach](#funktionspostfach).

### Ende-zu-Ende-Verschlüsselung

auch E2EE · EN *end-to-end encryption (E2EE)*

Bei der Ende-zu-Ende-Verschlüsselung (E2EE) wird eine [Nachricht](#nachricht) nur in den [E2EE-Endpunkten](#e2ee-endpunkt) ver- und entschlüsselt. Systeme dazwischen wie der [Homeserver](#homeserver) können ihren Inhalt nicht lesen. Ausgenommen sind die [Adressierungsmetadaten](#adressierungsmetadaten), die für den Transport einsehbar sein müssen. Als Verfahren ist MLS festgelegt (vgl. [ADR-0012](Zielarchitektur/Architekturentscheidungen/ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md)). Bis MLS in Matrix verfügbar ist, wird übergangsweise [Megolm](#megolm) eingesetzt. Externe Kommunikationssysteme werden über einen [Protokollkonverter](#protokollkonverter) angebunden, der die Ende-zu-Ende-Verschlüsselung beibehält (vgl. [ADR-0010](Zielarchitektur/Architekturentscheidungen/ADR-0010-Herstellung-von-Interoperabilität-mit-fremden-Systemen-außerhalb-des-Geltungsbereiches-der-Zielarchitektur.md)).

### Entität

EN *entity*

Eine Entität ist eine Person, eine [Organisation](#organisation), ein Gegenstand, ein Teilsystem oder eine abgrenzbare Gruppe mehrerer davon (Quelle: BSI TR-03107-1[^2]).

### Entscheidung (juristisch)

EN *judicial decision*

Als Entscheidung (juristisch) gelten gerichtliche Entscheidungen wie Urteil, Beschluss und Verfügung. Dazu zählen auch besondere Formen wie Mahnbescheid, Vollstreckungsbescheid und Strafbefehl. Abzugrenzen ist der [Verwaltungsakt](#verwaltungsakt) einer [Behörde](#behörde).

### Europäische Brieftasche für die Digitale Identität

auch EUDI-Wallet · EN *European Digital Identity Wallet (EUDI Wallet)*

Die Europäische Brieftasche für die Digitale Identität ist ein elektronisches [Identifizierungsmittel](#authentisierungsmittel). Nutzende können damit Personenidentifizierungsdaten und elektronische Attributsbescheinigungen sicher speichern, verwalten, validieren und vertrauenden Beteiligten präsentieren sowie [qualifizierte elektronische Signaturen](#qualifizierte-elektronische-signatur) und [Siegel](#qualifiziertes-elektronisches-siegel) erstellen (Quelle: i.S.v. Art. 3 Nr. 42 eIDAS-VO[^4]). Künftig sollen sich [Privatpersonen](#privatperson) mit ihr direkt gegenüber [Behörden](#behörde) authentisieren.

**EN:** The ‘European Digital Identity Wallet’ is an electronic identification means that enables users to securely store, manage and validate personal identification data and electronic attribute certificates in order to present them to relying parties and other users of European Digital Identity Wallets and to sign using qualified electronic signatures or seal using qualified electronic seals.

### Event

EN *event*

Ein Event ist im Matrix-Protokoll die Einheit, in der Daten in einem Raum übertragen werden, etwa [Nachrichten](#nachricht) (Quelle: i.S.v. Matrix-Spezifikation[^11]). In verschlüsselten Räumen sind nur die Inhalte verschlüsselt, Angaben zum Zustand des Raums wie die Mitgliedschaften nicht.

### externe Vertretung

EN *external representation*

Bei der externen Vertretung handelt eine Person oder Stelle aufgrund einer Vollmacht (vgl. § 80 AO[^12]) oder einer gesetzlichen Vertretungsbefugnis für eine andere Partei. Die vertretende Person handelt dabei mit eigener [Identität](#identität) und eigenem [Postfach](#postfach) und greift nicht auf das Postfach der vertretenen Partei zu. Abzugrenzen ist die organisationsinterne [Bearbeitungsvertretung](#bearbeitungsvertretung).

## F

### fachliche Metadaten

EN *business metadata*

Fachliche Metadaten sind Angaben zu einer [Nachricht](#nachricht), die der fachlichen Verarbeitung dienen und für die technische Übermittlung nicht benötigt werden. Beispiele sind Aktenzeichen, Antragsreferenz und [Status der Nachricht](#status-der-nachricht). Sie gehören zu den [vertraulichen Metadaten](#vertrauliche-metadaten), soweit sie nicht für die Adressierung benötigt werden.

### fachlicher Identitätsnachweis

auch Identitätsnachweis · EN *identity attestation*

Der fachliche Identitätsnachweis ist ein optionaler Beleg der [Identität](#identität) und des [Vertrauensniveaus](#vertrauensniveau) der [Absender:in](#absenderin), gebunden an ihren Schlüssel und an den [Vorgang](#vorgang). Er wird von einer [Nachweisstelle](#nachweisstelle) ausgestellt oder entsteht künftig durch die direkte [Authentisierung](#authentisierung) mit [eID](#eid-funktion) oder [EUDI-Wallet](#europäische-brieftasche-für-die-digitale-identität) (vgl. [ADR-1106](Zielarchitektur/Architekturentscheidungen/ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)). Abzugrenzen sind der elektronische Identitätsnachweis nach § 18 PAuswG[^13] als Verfahren der Anmeldung und die [Geräteverifizierung](#geräteverifizierung), die nur prüft, ob ein Gerät zu einer [MXID](#mxid) gehört.

### Fachverfahren

EN *back-office system*

Ein Fachverfahren ist ein IT-System, das der Erfüllung einer oder mehrerer konkreter Verwaltungsaufgaben dient. Es besteht meist aus Datenbanken, Anwendungen und Schnittstellen zu anderen Systemen. In der Zielarchitektur kann ein Fachverfahren einen dezentralen [Postfach-Zugang](#postfach-zugang) integrieren und darüber [Nachrichten](#nachricht) versenden und empfangen.

**EN:** Back-office systems are used by competent authorities to process applications and respond to requests. Back-office systems usually serve to fulfil one or more specific government services but may also be designed in a generic way to handle various government services. A back-office system can consist of various components, such as databases, software applications and interfaces that enable data to be exchanged between different systems and users.

### Föderationskontrolle

EN *federation control*

Die Föderationskontrolle ist die Steuerung von Aufnahme, Suspendierung und Ausschluss der [Postfach-Backends](#postfach-backend) im [Verbund](#verbund) über die [Föderationsliste](#föderationsliste). Bei einem Verstoß gegen die [Anschlussbedingungen](#anschlussbedingungen) kann ein Backend ausgeschlossen werden. Bei einem Sicherheitsvorfall oder begründetem Verdacht kann es vorübergehend suspendiert werden.

### Föderationsliste

EN *federation list*

Die Föderationsliste ist das verbundweite Verzeichnis der zugelassenen [Postfach-Backends](#postfach-backend). Sie führt je Backend Domain, Inhaber, Betreiber, öffentliche Schlüssel und den Status der Zulassung. Sie ist mit einem eigenen [Vertrauensanker](#vertrauensanker) des [Verbunds](#verbund) gesichert und von der [Trust-Liste](#trust-liste) getrennt. Jedes Backend föderiert ausschließlich mit den gelisteten Backends (vgl. [Föderationskontrolle](#föderationskontrolle)).

### fortgeschrittene elektronische Signatur

EN *advanced electronic signature*

Eine fortgeschrittene elektronische Signatur ist eine [elektronische Signatur](#elektronische-signatur), die die Anforderungen des Art. 26 eIDAS-VO[^4] erfüllt (Art. 3 Nr. 11). Sie ist eindeutig dem Unterzeichner zugeordnet und ermöglicht seine [Identifizierung](#identifizierung). Erstellt wird sie mit Signaturerstellungsdaten, die der Unterzeichner mit einem hohen Maß an Vertrauen unter seiner alleinigen Kontrolle verwenden kann. Eine nachträgliche Veränderung der unterzeichneten Daten kann erkannt werden. Auf ihr baut die [qualifizierte elektronische Signatur](#qualifizierte-elektronische-signatur) auf.

**EN:** An advanced electronic signature is an electronic signature that meets all of the following requirements. a) It is uniquely linked to the signatory. b) It enables the identification of the signatory. c) It is created using electronic signature creation data that the signatory can use with a high level of confidence under their sole control. d) It is linked to the data signed in such a way that any subsequent change to the data can be detected.

### fortgeschrittenes elektronisches Siegel

EN *advanced electronic seal*

Ein fortgeschrittenes elektronisches Siegel ist ein [elektronisches Siegel](#elektronisches-siegel), das die Anforderungen des Art. 36 eIDAS-VO[^4] erfüllt (Art. 3 Nr. 26). Es ist eindeutig dem Siegelersteller zugeordnet und ermöglicht seine [Identifizierung](#identifizierung). Erstellt wird es mit Siegelerstellungsdaten, die der Siegelersteller mit einem hohen Maß an Vertrauen unter seiner Kontrolle verwenden kann. Eine nachträgliche Veränderung der Daten kann erkannt werden. Die [Bereitstellungsbestätigung](#bereitstellungsbestätigung) und die [Löschquittung](#löschquittung) tragen dieses Siegel.

### Funktionspostfach

EN *shared mailbox*

Ein Funktionspostfach ist ein gemeinsames [Postfach](#postfach) einer [Organisation](#organisation), auf das mehrere [Postfachmitglieder](#postfachmitglieder) zugreifen. Es ist kein Matrix-Raum, sondern ein fachliches Objekt mit eigener [MXID](#mxid), eigenem Berechtigungsprofil und zugeordneten [Vorgangsräumen](#vorgangsraum). Seine Adresse wird im [Adressregister](#adressregister) geführt. Verwaltet wird es von der Rolle [Org-Admin](#org-admin).

**EN:** Several users in the role of mailbox members have access to the functions of a shared mailbox.

## G

### Geräteverifizierung

EN *device verification*

Die Geräteverifizierung ist die Prüfung, ob ein Gerät tatsächlich zu der angegebenen [Nutzer:in](#nutzerinnen) gehört (vgl. Matrix-Spezifikation[^11]). In der Zielarchitektur dürfen Raumschlüssel nur an verifizierte Geräte weitergegeben werden (vgl. [ADR-0018](Zielarchitektur/Architekturentscheidungen/ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

**EN:** The act of cryptographically proving that a Matrix device is actually operated by its respective owner. This term is defined by the Matrix specification[^11].

## H

### Holder-Binding

EN *holder binding, key binding*

Holder-Binding ist die Bindung eines Nachweises an einen Schlüssel des Clients. Verwenden kann den Nachweis dann nur, wer den Besitz dieses Schlüssels nachweist.[^14] [BundID](#bundid) und [Org-Konto](#org-konto) liefern derzeit Assertions ohne Holder-Binding. Für sie stellt eine [Nachweisstelle](#nachweisstelle) gebundene [fachliche Identitätsnachweise](#fachlicher-identitätsnachweis) aus (vgl. [ADR-1106](Zielarchitektur/Architekturentscheidungen/ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)).

### Homeserver

EN *homeserver*

Der Homeserver ist der Server im Matrix-Protokoll, der die Konten der [Nutzer:innen](#nutzerinnen) und ihre Räume führt und mit anderen Homeservern föderiert.[^11] [Postfach-Zugänge](#postfach-zugang) greifen über ihn auf die [Postfächer](#postfach) zu. In der Zielarchitektur ist er Bestandteil des [Postfach-Backends](#postfach-backend) und bestimmt die Domain der [MXID](#mxid).

## I

### Identifizierung

auch Identifikation · EN *identification*

Eine Identifizierung ist die Übermittlung von anwendungsbezogen geeigneten [Identitätsattributen](#identitätsattribut) (einer [Identität](#identität)), einschließlich authentisierender Metadaten ([Authentisierung](#authentisierung)), sowie die Überprüfung ([Authentifizierung](#authentifizierung)) dieser Identität durch die [vertrauende Entität](#vertrauende-entität) (Quelle: BSI TR-03107-1[^2]).

### Identität

EN *identity*

Eine Identität ist eine Menge von [Identitätsattributen](#identitätsattribut), die einer [Entität](#entität) zugeordnet sind. Eine Entität kann mehrere Identitäten haben, ebenso können mehrere Entitäten die gleiche Identität haben. Eine Identität ist daher im Allgemeinen nicht eindeutig, kann dies aber in einem bestimmten Anwendungskontext sein (Quelle: BSI TR-03107-1[^2]).

### Identitätsattribut

auch Identitätsdatum · EN *identity attribute*

Ein Identitätsattribut oder ein Identitätsdatum ist eine Charakteristik oder eine Eigenschaft einer [Entität](#entität). Beispiele für Identitätsattribute einer [natürlichen Person](#privatperson) sind Name, Geburtsdatum oder die Eigenschaft, ein bestimmtes Alter erreicht zu haben. Identitätsattribute von [Behörden](#behörde) umfassen etwa Bezeichnung der Behörde oder deren Webadresse (Quelle: BSI TR-03107-1[^2]).

### Integrität

auch Unversehrtheit · EN *integrity*

Integrität bezeichnet die Sicherstellung der Korrektheit (Unversehrtheit) von Daten und der korrekten Funktionsweise von Systemen. Integrität von Daten bedeutet, dass sie vollständig und unverändert sind. Bei Informationen umfasst sie auch zugeordnete Angaben wie Autorenschaft und Erstellungszeitpunkt (Quelle: in Anlehnung an BSI IT-Grundschutz-Kompendium, Glossar[^5]). In der Zielarchitektur sichern [elektronische Signaturen](#elektronische-signatur) und [Siegel](#elektronisches-siegel) die Integrität von Inhalten, die [Bereitstellungsbestätigung](#bereitstellungsbestätigung) die der übermittelten [Nachricht](#nachricht) (vgl. [ADR-0017](Zielarchitektur/Architekturentscheidungen/ADR-0017-Authentizität-und-Integrität-von-Inhalten.md), [ADR-0018](Zielarchitektur/Architekturentscheidungen/ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

## L

### Lesebestätigung

EN *read confirmation*

Mit der Lesebestätigung bestätigt die [Empfänger:in](#empfängerin) der [Absender:in](#absenderin) die Kenntnisnahme einer [Nachricht](#nachricht). Sie wird nur durch eine bewusste Handlung im [Postfach-Zugang](#postfach-zugang) abgegeben. Sie kann von der Absender:in angefordert werden, ist aber nicht verpflichtend. Wird sie zurechenbar abgegeben, kann sie als [elektronisches Empfangsbekenntnis](#elektronisches-empfangsbekenntnis) dienen.

**EN:** Receiving user confirms reading a message by a deliberate action (implies successful decryption).

### Löschquittung

EN *deletion confirmation*

Die Löschquittung ist eine Quittung darüber, dass die [Absender:in](#absenderin) oder die [Empfänger:in](#empfängerin) eine bereits übermittelte und quittierte [Nachricht](#nachricht) gelöscht hat. Sie wird vom [Quittungsdienst](#quittungsdienst) im [Postfach-Backend](#postfach-backend) der Empfänger:in erzeugt und trägt das [fortgeschrittene elektronische Siegel](#fortgeschrittenes-elektronisches-siegel) seines Betreibers.

## M

### Machtbereich

EN *sphere of control*

Der Machtbereich umfasst die Systeme, über die eine [Nutzer:in](#nutzerinnen) die Verfügungsgewalt hat und für die sie verantwortlich ist. Die [Ende-zu-Ende-Verschlüsselung](#ende-zu-ende-verschlüsselung) endet nur im Endgerät der Nutzer:in oder auf Systemen im Machtbereich der [Organisation](#organisation) (vgl. [E2EE-Endpunkt](#e2ee-endpunkt)). Rechtlich geht eine Erklärung zu, wenn sie so in den Machtbereich der [Empfänger:in](#empfängerin) gelangt, dass diese unter gewöhnlichen Umständen Kenntnis nehmen kann (vgl. § 130 BGB[^15]).

### Megolm

auch Olm/Megolm · EN *Megolm*

Megolm ist das Verfahren der [Ende-zu-Ende-Verschlüsselung](#ende-zu-ende-verschlüsselung) in Matrix-Räumen.[^11] In der Zielarchitektur wird es bis zur Migration auf MLS eingesetzt.

### Mein Justizpostfach

auch MJP · EN *Mein Justizpostfach (MJP)*

„Mein Justizpostfach“ (MJP) ist ein kostenloser Dienst. Er ermöglicht [Privatpersonen](#privatperson), Informationen sicher, verschlüsselt und digital mit deutschen Gerichten, [Behörden](#behörde), Rechtsanwält:innen und Notar:innen auszutauschen (Quelle: MJP-Serviceportal[^25]).

**EN:** "Mein Justizpostfach" (MJP) is a free service. It enables private individuals to exchange information securely, in encrypted form, and digitally with German courts, public authorities, lawyers, and notaries (source: MJP service portal[^25]).

### Mein Unternehmenskonto

auch MUK, MeinUK · EN *Mein Unternehmenskonto (MUK)*

Mein Unternehmenskonto (MUK) ist ein Bestandssystem und ein Organisationskonto im Sinne von § 2 Abs. 5 OZG[^7]. Im Zielzustand bleibt MUK Identitätsanbieter für [private Organisationen](#private-organisation). Die [Postfächer](#postfach) im MUK werden abgelöst. In der Zielarchitektur heißt der Identitätsanbieter einheitlich [Org-Konto](#org-konto).

### MSC

EN *Matrix Spec Change (MSC)*

Ein MSC (Matrix Spec Change) ist ein Vorschlag zur Änderung der Matrix-Spezifikation[^11]. Die Vorschläge sind auf einer Übersichtsseite der Spezifikation aufgeführt[^20].

**EN:** A proposal to change the Matrix specification (see Matrix specification[^20]).

### MXID

auch Matrix-User-ID · EN *Matrix user ID (MXID)*

Die MXID ist die Postfachadresse im Matrix-Protokoll mit dem Aufbau `@localpart:domain`[^11] und einem Localpart ohne Personenbezug. Die Domain bestimmt der [Homeserver](#homeserver) des [Postfach-Backends](#postfach-backend). [Privatpersonen](#privatperson) erhalten je [Bereich](#bereich) eine eigene pseudonyme [Bereichs-MXID](#bereichs-mxid). Postfachadressen von [Organisationen](#organisation) werden im [Adressregister](#adressregister) geführt.

## N

### Nachricht

EN *message*

Eine Nachricht besteht aus dem [Nachrichtentext](#nachrichtentext), keinem oder mehreren [Anhängen](#anhang), [Adressierungsmetadaten](#adressierungsmetadaten) und [vertraulichen Metadaten](#vertrauliche-metadaten) wie [fachlichen Metadaten](#fachliche-metadaten). Anhänge sind für Menschen bestimmte Dokumente oder [strukturierte Daten](#strukturierte-daten). In der Zielarchitektur werden Nachrichtentext, Anhänge und vertrauliche Metadaten [Ende-zu-Ende-verschlüsselt](#ende-zu-ende-verschlüsselung) in einem [Vorgangsraum](#vorgangsraum) übermittelt.

### Nachrichtentext

EN *message body*

Der Nachrichtentext ist der Anzeigetext einer [Nachricht](#nachricht). [Anhänge](#anhang) und Metadaten gehören nicht dazu.

### Nachweisstelle

EN *issuer*

Eine Nachweisstelle stellt [fachliche Identitätsnachweise](#fachlicher-identitätsnachweis) aus (vgl. [Trust-Liste](#trust-liste)). Die zentrale Nachweisstelle überführt dazu die Assertions von [BundID](#bundid) und [Org-Konto](#org-konto). Ein [Org-IdP-Adapter](#org-idp-adapter) ist die Nachweisstelle einer [Organisation](#organisation) mit eigenem Identitätsmanagement. Der Betrieb der Nachweisstelle für [Privatpersonen](#privatperson) ist vom [Postfach-Backend](#postfach-backend) für Privatpersonen und von der BundID zu trennen (vgl. [Nichtverkettbarkeit](#nichtverkettbarkeit)).

### Nichtabstreitbarkeit

EN *non-repudiation*

Nichtabstreitbarkeit bezeichnet die Gewährleistung, dass Versand und Empfang von Daten und Informationen gegenüber Dritten nachweisbar sind und nicht in Abrede gestellt werden können (Quelle: i.S.v. BSI IT-Grundschutz-Kompendium, Glossar[^5]). In der Zielarchitektur wird sie unabhängig vom verschlüsselten Kanal durch die [Bereitstellungsbestätigung](#bereitstellungsbestätigung), die [Löschquittung](#löschquittung) und [qualifizierte elektronische Siegel](#qualifiziertes-elektronisches-siegel) oder [Signaturen](#qualifizierte-elektronische-signatur) auf Dokumenten sichergestellt (vgl. [Plausible Deniability](#plausible-deniability)).

### Nichtverkettbarkeit

EN *unlinkability*

Nichtverkettbarkeit ist das Schutzziel, dass die Kommunikation einer [Privatperson](#privatperson) mit verschiedenen [Bereichen](#bereich) nicht über gemeinsame Kennungen verkettet werden kann (vgl. Gewährleistungsziel Nichtverkettung, SDM[^16] C1.5). Kein Betreiber allein soll die Adressen einer Person aus verschiedenen Bereichen zusammenführen und mit ihren Kommunikationsbeziehungen verknüpfen können. Dazu dienen [Bereichs-MXIDs](#bereichs-mxid) und die Trennung der Betreiber.

### Nutzer:innen

EN *users*

Nutzer:innen sind [Privatpersonen](#privatperson), [private Organisationen](#private-organisation), [öffentliche Stellen](#öffentliche-stelle) und [Organisationen der Justiz](#organisation-der-justiz) (vgl. § 2 Abs. 4 OZG[^7]).

**EN:** Users in the context of Neo are private individuals and competent authorities.

### Nutzerkonto

EN *user account*

Ein Nutzerkonto dient der einmaligen oder dauerhaften [Identifizierung](#identifizierung) und [Authentifizierung](#authentifizierung) von [Nutzer:innen](#nutzerinnen). Es wird als Bürgerkonto oder als Organisationskonto bereitgestellt (Quelle: i.S.v. § 2 Abs. 5 OZG[^7]). Nach § 2 Abs. 7 OZG ist das [Postfach](#postfach) Bestandteil eines Nutzerkontos. In der Zielarchitektur sind Nutzerkonto und Postfach dagegen getrennt (vgl. [ADR-1110](Zielarchitektur/Architekturentscheidungen/ADR-1110-Entkopplung-von-Nutzerkonto-und-Postfach.md)). Nutzerkonten der Zielarchitektur sind die [BundID](#bundid) für [Privatpersonen](#privatperson) und das [Org-Konto](#org-konto) für [Organisationen](#organisation).

## O

### öffentliche Stelle

EN *public body, public sector body*

Öffentliche Stellen sind die [Behörden](#behörde), die Organe der Rechtspflege und andere öffentlich-rechtlich organisierte Einrichtungen des Bundes, der Länder, der Gemeinden und Gemeindeverbände sowie der Körperschaften, Anstalten und Stiftungen des öffentlichen Rechts. Dazu gehören auch deren Vereinigungen ungeachtet ihrer Rechtsform (Quelle: i.S.v. § 2 Abs. 1 und 2 BDSG[^17]). Gerichte und andere [Organisationen der Justiz](#organisation-der-justiz) sind danach öffentliche Stellen. In der Zielarchitektur bilden sie jedoch eine eigene [Teilnehmendengruppe](#teilnehmendengruppe).

**EN:** Public bodies are the authorities, the bodies of the administration of justice and other public-law institutions of the Federation, the Länder and the municipalities, as well as the corporations, establishments and foundations under public law and their associations. In Neo, public body is used for the public side in general and competent authority for the authority that handles a case.

### Onlinedienst

auch Online-Dienst · EN *online service*

Ein Onlinedienst ist eine IT-Komponente, die ein eigenständiges elektronisches Angebot an die Nutzer darstellt, welches die Abwicklung einer oder mehrerer elektronischer Verwaltungsleistungen von Bund oder Ländern ermöglicht (Quelle: § 2 Abs. 8 Satz 1 OZG[^7]). Über ihn wird in der Regel ein [Antrag](#antrag) gestellt. Bearbeitet wird der Antrag im [Fachverfahren](#fachverfahren) der zuständigen [Behörde](#behörde).

**EN:** An online service is an IT system providing a user interface to request some public service via the internet.

### Org-Admin

EN *Org admin*

Mit der Rolle Org-Admin werden die [Funktionspostfächer](#funktionspostfach) von [privaten Organisationen](#private-organisation) und [öffentlichen Stellen](#öffentliche-stelle) verwaltet. Org-Admins erteilen und entziehen Personen den Zugriff auf Funktionspostfächer, einzeln oder über Gruppen. Die Zuordnungen werden im [Berechtigungsverzeichnis](#berechtigungsverzeichnis) gepflegt. Änderungen durch Org-Admins werden revisionssicher protokolliert.

### Org-IdP-Adapter

EN *Org IdP adapter*

Der Org-IdP-Adapter ist die Komponente, über die eine [Organisation](#organisation) ihr internes Identitäts- und Zugriffsmanagement (IAM) an den [Verbund](#verbund) anbindet. Er tauscht intern ausgestellte Nachweise der Mitarbeitenden gegen [fachliche Identitätsnachweise](#fachlicher-identitätsnachweis) aus. Jede Instanz ist eine eigene [Nachweisstelle](#nachweisstelle) und darf nur Personen der eigenen Organisation ausweisen (vgl. [Trust-Liste](#trust-liste)).

### Org-Konto

auch Organisationskonto · EN *Org account, organisation account*

Das Org-Konto ist der Identitätsanbieter der Zielarchitektur für [private Organisationen](#private-organisation) und [öffentliche Stellen](#öffentliche-stelle) (vgl. [Mein Unternehmenskonto](#mein-unternehmenskonto)). Es ist ein Organisationskonto nach § 2 Abs. 5 OZG[^7]. Anders als in § 2 Abs. 7 OZG vorgesehen enthält es kein [Postfach](#postfach). Ohne [Org-IdP-Adapter](#org-idp-adapter) melden sich Mitarbeitende darüber am [Postfach-Backend](#postfach-backend) an.

### Organisation

EN *organisation*

Organisation ist in der Zielarchitektur der Sammelbegriff für [öffentliche Stellen](#öffentliche-stelle), [private Organisationen](#private-organisation) und [Organisationen der Justiz](#organisation-der-justiz) in Abgrenzung zu [Privatpersonen](#privatperson). Er ersetzt die bisher verwendete Bezeichnung juristische Person. Anders als die juristische Person im Rechtssinn umfasst der Begriff auch [Behörden](#behörde) und Gerichte sowie Personengesellschaften.

### Organisation der Justiz

EN *judicial organisation*

Eine Organisation der Justiz ist ein Gericht oder eine andere Stelle der Justiz. Organisationen der Justiz bilden eine eigene [Teilnehmendengruppe](#teilnehmendengruppe) mit einem eigenen zentralen [Postfach-Backend](#postfach-backend). Die bestehende EGVP-Infrastruktur des [elektronischen Rechtsverkehrs](#elektronischer-rechtsverkehr) wird über einen [Protokollkonverter](#protokollkonverter) angebunden.

### Organisationsidentität

EN *organisation identity*

Die Organisationsidentität ist die [Identität](#identität) einer [Organisation](#organisation). Organisationen weisen sie über das [Org-Konto](#org-konto) nach, ihre Systeme über Zertifikate (vgl. [ADR-0015](Zielarchitektur/Architekturentscheidungen/ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md), [ADR-0016](Zielarchitektur/Architekturentscheidungen/ADR-0016-Authentizität-Identität-privater-Organisationen.md)). Im [Adressregister](#adressregister) sind die Postfachadressen einer Organisation mit ihrer Organisationsidentität verknüpft (vgl. [ADR-1111](Zielarchitektur/Architekturentscheidungen/ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)). Über den [fachlichen Identitätsnachweis](#fachlicher-identitätsnachweis) wird sie einer einzelnen [Nachricht](#nachricht) zugeordnet (vgl. [ADR-1106](Zielarchitektur/Architekturentscheidungen/ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)).

## P

### Plausible Deniability

auch glaubwürdige Abstreitbarkeit · EN *plausible deniability, deniable authentication*

Plausible Deniability bedeutet, dass die Kommunikationspartner:innen die [Authentizität](#authentizität) einer [Nachricht](#nachricht) im verschlüsselten Kanal prüfen können, Dritten gegenüber aber nicht beweisen können, dass die Nachricht von einer bestimmten Person stammt. Rechtlich erforderliche Nachweise gegenüber Dritten werden außerhalb des Kanals erbracht (vgl. [Nichtabstreitbarkeit](#nichtabstreitbarkeit)).

### Postfach

EN *mailbox*

Ein Postfach ist ein funktionaler Baustein, mit dem [Nutzer:innen](#nutzerinnen) [elektronischen Schriftverkehr](#elektronischer-schriftverkehr) verfassen, versenden, empfangen, speichern und anzeigen können (vgl. § 2 Abs. 7 OZG[^7]). In der Zielarchitektur ist das Postfach vom [Nutzerkonto](#nutzerkonto) getrennt (vgl. [ADR-1110](Zielarchitektur/Architekturentscheidungen/ADR-1110-Entkopplung-von-Nutzerkonto-und-Postfach.md)). Nutzerkonten wie die [BundID](#bundid) und das [Org-Konto](#org-konto) sowie die [Europäische Brieftasche für die Digitale Identität](#europäische-brieftasche-für-die-digitale-identität) werden nur zur [Identifizierung](#identifizierung) und [Authentifizierung](#authentifizierung) genutzt. Sie enthalten kein Postfach.

**EN:** A mailbox is a functional component that enables users to compose and send electronic correspondence, as well as to receive, store and display it.

### Postfach-Backend

EN *mailbox backend*

Das Postfach-Backend umfasst die für eine Domain zuständigen Server-Komponenten, die [Postfächer](#postfach) und [Nachrichten](#nachricht) verwalten, insbesondere den [Homeserver](#homeserver). Für jede [Teilnehmendengruppe](#teilnehmendengruppe) wird ein Postfach-Backend zentral bereitgestellt. [Organisationen](#organisation) können unter Einhaltung der [Anschlussbedingungen](#anschlussbedingungen) eigene Postfach-Backends betreiben. [Privatpersonen](#privatperson) nutzen ausschließlich das zentrale Backend für Privatpersonen.

### Postfach-Zugang

EN *mailbox client*

Ein Postfach-Zugang ist eine Anwendung für den Zugriff auf ein [Postfach](#postfach), etwa eine App, eine Webanwendung oder ein [Fachverfahren](#fachverfahren) mit integriertem Matrix-Client. In der Regel ist er der [E2EE-Endpunkt](#e2ee-endpunkt). Neben dem zentral bereitgestellten Postfach-Zugang sind Postfach-Zugänge Dritter zulässig.

### Postfachmitglieder

EN *mailbox members*

Postfachmitglieder sind Personen, denen ein [Org-Admin](#org-admin) Zugriff auf ein [Funktionspostfach](#funktionspostfach) erteilt hat. Sie bearbeiten die [Vorgänge](#vorgang) des Funktionspostfachs. Aus Postfachmitgliedschaften, Rollen und [Bearbeitungsvertretungen](#bearbeitungsvertretung) im [Berechtigungsverzeichnis](#berechtigungsverzeichnis) ergibt sich die Bearbeitergruppe, die der [Raum-Management-Service](#raum-management-service) in die [Vorgangsräume](#vorgangsraum) einlädt.

### private Organisation

EN *private organisation*

Private Organisationen sind [Organisationen](#organisation) des Privatrechts, etwa Unternehmen, Vereine und Verbände. Sie bilden eine eigene [Teilnehmendengruppe](#teilnehmendengruppe). Ihr Identitätsanbieter ist das [Org-Konto](#org-konto).

### Privatperson

auch natürliche Person, Bürger:in · EN *private individual, natural person, citizen*

Privatpersonen sind natürliche Personen[^15] als [Nutzer:innen](#nutzerinnen) des [Systems](#system). Sie bilden eine eigene [Teilnehmendengruppe](#teilnehmendengruppe). Die Bezeichnung Bürger:in wird in der Zielarchitektur vermieden, weil sie eine Staatsangehörigkeit nahelegt. Abzugrenzen sind [private Organisationen](#private-organisation), [öffentliche Stellen](#öffentliche-stelle) und [Organisationen der Justiz](#organisation-der-justiz).

**EN:** A natural person who requests a public service. Since the term 'citizen' usually implies citizenship of a sovereign state which is not a requirement to request some public service, the term 'private individual' is more generic and often more correct.

### Protokollkonverter

EN *protocol converter*

Ein Protokollkonverter bindet ein externes Kommunikationssystem mit einem anderen Transportprotokoll an die [Postfach-Backends](#postfach-backend) der Zielarchitektur an, ohne die [Ende-zu-Ende-Verschlüsselung](#ende-zu-ende-verschlüsselung) aufzubrechen (vgl. [ADR-0010](Zielarchitektur/Architekturentscheidungen/ADR-0010-Herstellung-von-Interoperabilität-mit-fremden-Systemen-außerhalb-des-Geltungsbereiches-der-Zielarchitektur.md)). Dafür müssen die [E2EE-Endpunkte](#e2ee-endpunkt) im [Verbund](#verbund) und im externen System dasselbe Verfahren der Ende-zu-Ende-Verschlüsselung unterstützen. Bridges, die [Nachrichten](#nachricht) entschlüsseln und neu verschlüsseln, sollen in der Zielarchitektur nicht verwendet werden.

### Pseudonym

EN *pseudonym*

Ein Pseudonym ist eine Kennung einer Person, die ohne zusätzliche Informationen nicht ihrer [Identität](#identität) zugeordnet werden kann (vgl. Art. 4 Nr. 5 DSGVO[^18]). Die [Authentisierungssysteme](#authentisierungssystem) für [Privatpersonen](#privatperson) bilden jeweils eigene Pseudonyme. Der Localpart der [MXID](#mxid) von Privatpersonen und Mitarbeitenden ist ebenfalls ein Pseudonym. Bei Privatpersonen wird er zufällig erzeugt.

## Q

### qualifizierte elektronische Signatur

auch QES · EN *qualified electronic signature (QES)*

Eine qualifizierte elektronische Signatur ist eine [fortgeschrittene elektronische Signatur](#fortgeschrittene-elektronische-signatur), die von einer qualifizierten elektronischen Signaturerstellungseinheit erstellt wurde und auf einem qualifizierten Zertifikat für elektronische Signaturen beruht (Quelle: Art. 3 Nr. 12 eIDAS-VO[^4]).

**EN:** Qualified electronic signature is an advanced electronic signature created by a qualified electronic signature creation device and based on a qualified certificate for electronic signatures (source: Elektronische Vertrauensdienste[^26]).

### qualifizierter elektronischer Zeitstempel

EN *qualified electronic time stamp*

Ein qualifizierter elektronischer Zeitstempel ist ein elektronischer Zeitstempel, der die Anforderungen des Art. 42 eIDAS-VO[^4] erfüllt. Er verknüpft Daten mit einem Zeitpunkt und weist nach, dass sie zu diesem Zeitpunkt vorhanden waren (Quelle: i.S.v. Art. 3 Nr. 33 und 34 eIDAS-VO). Art. 42 verlangt Schutz vor unbemerkter Veränderung, eine mit der koordinierten Weltzeit verknüpfte Zeitquelle sowie eine [fortgeschrittene elektronische Signatur](#fortgeschrittene-elektronische-signatur) oder ein [fortgeschrittenes elektronisches Siegel](#fortgeschrittenes-elektronisches-siegel) des qualifizierten Vertrauensdiensteanbieters oder ein gleichwertiges Verfahren.

### qualifiziertes elektronisches Siegel

EN *qualified electronic seal*

Ein qualifiziertes elektronisches Siegel ist ein [fortgeschrittenes elektronisches Siegel](#fortgeschrittenes-elektronisches-siegel), das von einer qualifizierten elektronischen Siegelerstellungseinheit erstellt wird und auf einem qualifizierten Zertifikat für elektronische Siegel beruht (Quelle: Art. 3 Nr. 27 eIDAS-VO[^4]). In der Zielarchitektur siegelt die absendende [Organisation](#organisation) ein Dokument vor dem Versand und übermittelt es als [Anhang](#anhang) (vgl. [ADR-0017](Zielarchitektur/Architekturentscheidungen/ADR-0017-Authentizität-und-Integrität-von-Inhalten.md)). Damit sind Ursprung und [Unversehrtheit](#integrität) des Dokuments auch gegenüber Dritten beweissicher.

### Quittungsdienst

EN *confirmation service*

Der Quittungsdienst ist der Dienst des empfangenden [Postfach-Backends](#postfach-backend), der die [Bereitstellungsbestätigung](#bereitstellungsbestätigung) und die [Löschquittung](#löschquittung) erzeugt und mit dem [fortgeschrittenen elektronischen Siegel](#fortgeschrittenes-elektronisches-siegel) des Betreibers versieht. Er kann den Klartext der [Nachrichten](#nachricht) nicht lesen. Der Betreiber setzt ihn als Application Service, als Modul des [Homeservers](#homeserver) oder als Bot um (vgl. [ADR-0018](Zielarchitektur/Architekturentscheidungen/ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

## R

### Raum-Management-Service

auch RMS · EN *room management service (RMS)*

Der Raum-Management-Service (RMS) ist die Komponente, die die [Vorgangsräume](#vorgangsraum) der [Funktionspostfächer](#funktionspostfach) verwaltet. Er erzeugt Vorgangsräume und gleicht Mitgliedschaften und Rechte mit dem [Berechtigungsverzeichnis](#berechtigungsverzeichnis) ab (vgl. [technischer Nutzer](#technischer-nutzer)). Der zentral bereitgestellte RMS besitzt keine Schlüssel der Räume. Ein RMS im [Machtbereich](#machtbereich) der [Organisation](#organisation) kann an der [Ende-zu-Ende-Verschlüsselung](#ende-zu-ende-verschlüsselung) teilnehmen.

### Referenz

EN *reference*

Eine Referenz ist eine Kennung, die einen fachlichen Bezug herstellt, etwa ein Aktenzeichen oder die Kennung eines [Antrags](#antrag), einer Verwaltungsleistung, eines [Fachverfahrens](#fachverfahren) oder einer [Nachricht](#nachricht). Referenzen werden als [fachliche Metadaten](#fachliche-metadaten) in der Nachricht hinterlegt. Abzugrenzen ist der technische Verweis eines [Events](#event) auf die ursprüngliche Nachricht, etwa in einer [Abrufbestätigung](#abrufbestätigung) (vgl. [ADR-1118](Zielarchitektur/Architekturentscheidungen/ADR-1118-Abruf-und-Lesebestätigungen.md)).

## S

### Self-Service-Portal

auch SSP · EN *self-service portal (SSP)*

Das Self-Service-Portal (SSP) ist das zentrale Portal für das Onboarding in den [Verbund](#verbund). [Organisationen](#organisation) registrieren sich dort über das [Org-Konto](#org-konto) und nehmen mit dem [Anschlussvertrag](#anschlussvertrag) die [Anschlussbedingungen](#anschlussbedingungen) an. Im SSP werden technische Systeme der Organisationen und [Postfach-Backends](#postfach-backend) registriert (vgl. [Föderationsliste](#föderationsliste)). Die Organisationen pflegen ihre [Funktionspostfächer](#funktionspostfach) und die Adresseinträge im [Adressregister](#adressregister) nicht im SSP, sondern in der Administrationsoberfläche des zentralen Postfach-Backends (vgl. [ADR-1148](Zielarchitektur/Architekturentscheidungen/ADR-1148-Onboarding-im-Self-Service-Portal.md)). Betreiber dezentraler Postfach-Backends können dafür eigene Verfahren nutzen (vgl. [ADR-1111](Zielarchitektur/Architekturentscheidungen/ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)).

### Status der Antragsbearbeitung

EN *application status*

Der Status der Antragsbearbeitung gibt den Stand der Bearbeitung eines [Antrags](#antrag) an (Quelle: i.S.v. OZG-Status-Schema).

### Status der Nachricht

EN *message status*

Der Status der Nachricht gibt den Zustand einer [Nachricht](#nachricht) an. Das OZG-PLUS-Postfach unterscheidet zum Beispiel die Status `EINGESTELLT`, `BEREIT_ZUR_ABHOLUNG`, `VOM_EMPFAENGER_ABGEHOLT` und `GELOESCHT`. In der Zielarchitektur werden Bereitstellung, Abruf, Lesen und Löschen einer Nachricht durch [Bereitstellungsbestätigung](#bereitstellungsbestätigung), [Abrufbestätigung](#abrufbestätigung), [Lesebestätigung](#lesebestätigung) und [Löschquittung](#löschquittung) belegt.

### Statusinformationen der Antragsbearbeitung

EN *application status information*

Statusinformationen der Antragsbearbeitung sind weitergehende textuelle Informationen zum [Status der Antragsbearbeitung](#status-der-antragsbearbeitung). Sie werden zusammen mit dem Status der Antragsbearbeitung in einer Statusnachricht nach dem OZG-Status-Schema übermittelt.

### strukturierte Daten

EN *structured data*

Strukturierte Daten sind Daten, die für die maschinelle Weiterverarbeitung vorgesehen sind. Sie folgen in der Regel einem Datenstandard wie einem JSON- oder XML-Schema, damit verschiedene Systeme sie einheitlich verarbeiten können.

### Synapse

EN *Synapse*

Synapse[^27] ist eine quelloffene Implementierung eines Matrix-[Homeservers](#homeserver), die von Element geschrieben und gepflegt wird.

**EN:** Synapse[^27] is an open source Matrix homeserver implementation, written and maintained by Element.

### System

auch Das System · EN *the system*

Das System ist die föderierte, postfachbasierte Kommunikationsinfrastruktur, die die Zielarchitektur beschreibt. Über sie tauschen [Nutzer:innen](#nutzerinnen) vorgangsbezogen [elektronischen Schriftverkehr](#elektronischer-schriftverkehr) miteinander aus. Der Begriff wird vor allem in den Anforderungen verwendet.

## T

### technischer Nutzer

EN *bot account, service account*

Ein technischer Nutzer ist ein automatisiert betriebener Teilnehmer in Matrix-Räumen (im Matrix-Umfeld auch als Bot bezeichnet). Der technische Nutzer eines [Funktionspostfachs](#funktionspostfach) heißt FP-Bot und gehört zum [Raum-Management-Service](#raum-management-service). Er erzeugt die [Vorgangsräume](#vorgangsraum) und lädt die berechtigten [Postfachmitglieder](#postfachmitglieder) ein. Gegenüber den übrigen Teilnehmenden werden technische Nutzer als solche gekennzeichnet.

### Teilnehmendengruppe

auch Nutzendengruppe · EN *participant group, user group*

Eine Teilnehmendengruppe ist eine Gruppe von [Nutzer:innen](#nutzerinnen), für die ein eigenes zentrales [Postfach-Backend](#postfach-backend) bereitgestellt wird. Unterschieden werden [Privatpersonen](#privatperson), [private Organisationen](#private-organisation), [öffentliche Stellen](#öffentliche-stelle) und [Organisationen der Justiz](#organisation-der-justiz). Das Backend für Privatpersonen wird von einer anderen Stelle betrieben als die Backends für [Organisationen](#organisation).

### Trust-Liste

EN *trust list*

Die Trust-Liste ist das verbundweite Verzeichnis der zugelassenen [Nachweisstellen](#nachweisstelle), also der zentralen Nachweisstellen und der [Org-IdP-Adapter](#org-idp-adapter). Sie führt je Nachweisstelle den Schlüssel, die Gültigkeit und den Status. Sie ist mit einem eigenen [Vertrauensanker](#vertrauensanker) des [Verbunds](#verbund) gesichert und von der [Föderationsliste](#föderationsliste) getrennt (vgl. [ADR-1144](Zielarchitektur/Architekturentscheidungen/ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md)). [Fachliche Identitätsnachweise](#fachlicher-identitätsnachweis) werden im [Postfach-Zugang](#postfach-zugang) der [Empfänger:in](#empfängerin) nur anerkannt, wenn die ausstellende Nachweisstelle in der Trust-Liste steht (vgl. [ADR-1108](Zielarchitektur/Architekturentscheidungen/ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)). Geführt wird die Liste im [Self-Service-Portal](#self-service-portal), bis eine geeignete Infrastruktur im Deutschland-Stack verfügbar ist. Abzugrenzen sind die Vertrauenslisten nach Art. 22 eIDAS-VO[^4] mit den qualifizierten Vertrauensdiensteanbietern. Anhand dieser Vertrauenslisten wird etwa geprüft, ob ein [qualifizierter elektronischer Zeitstempel](#qualifizierter-elektronischer-zeitstempel) von einem qualifizierten Vertrauensdiensteanbieter stammt.

## U

### Übermittlungsoption

EN *transmission option*

Eine Übermittlungsoption ist eine Vorgabe der [Absender:in](#absenderin) zu einer einzelnen [Nachricht](#nachricht), die die Empfängerseite umsetzt. Beispiele sind die Anforderung einer [Bereitstellungsbestätigung](#bereitstellungsbestätigung), [Abrufbestätigung](#abrufbestätigung) oder [Lesebestätigung](#lesebestätigung) und ein [Mindest-Vertrauensniveau](#vertrauensniveau) für den Abruf. Was angefordert werden darf, hängt unter anderem von der Art der Absender:in (etwa [Behörde](#behörde) oder [Privatperson](#privatperson)), ihrer Organisationszugehörigkeit und dem Vertrauensniveau ab (vgl. [ADR-1119](Zielarchitektur/Architekturentscheidungen/ADR-1119-Absendergesteuerte-Übermittlungsoptionen-mit-Berechtigungsprüfung.md)). Backend-seitige Optionen setzt das empfangende [Postfach-Backend](#postfach-backend) um. Clientseitige Optionen setzt der [Postfach-Zugang](#postfach-zugang) um.

## V

### Verbund

auch Kommunikationsverbund · EN *federation, communication network*

Der Verbund umfasst alle in der [Föderationsliste](#föderationsliste) geführten [Postfach-Backends](#postfach-backend) sowie die daran angebundenen [Postfach-Zugänge](#postfach-zugang), [Fachverfahren](#fachverfahren) und [Organisationen](#organisation). Voraussetzung für die Teilnahme von Organisationen und Betreibern ist das Onboarding im [Self-Service-Portal](#self-service-portal) mit Annahme der [Anschlussbedingungen](#anschlussbedingungen) im [Anschlussvertrag](#anschlussvertrag). Die Anschlussbedingungen gelten einheitlich für zentrale und dezentrale Backends.

### vertrauende Entität

auch vertrauender Beteiligter · EN *relying entity, relying party*

Eine vertrauende Entität ist eine [Entität](#entität), die sich auf die Echtheit und Gültigkeit einer [Identität](#identität) oder anderer übermittelter Daten verlässt. Beispiele sind ein Dienstanbieter, der eine Person identifiziert, oder eine Person, die einen Dienstanbieter identifiziert (Quelle: i.S.v. BSI TR-03107-1[^2]).

### Vertrauensanker

EN *trust anchor*

Ein Vertrauensanker ist ein öffentlicher Schlüssel, meist in Form eines Root-Zertifikats, dem ohne weitere Prüfung vertraut wird. Von ihm ausgehend werden Zertifikate, Schlüssel und signierte Listen geprüft. Die [Föderationsliste](#föderationsliste) und die [Trust-Liste](#trust-liste) haben jeweils einen eigenen Vertrauensanker des [Verbunds](#verbund) (vgl. [ADR-1144](Zielarchitektur/Architekturentscheidungen/ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md)). Daneben nutzt die Zielarchitektur weitere Vertrauensanker, etwa das Root-Zertifikat der Zertifizierungsstelle des Verbunds für die Server-Server-API und die Vertrauenslisten nach eIDAS-VO für qualifizierte Vertrauensdienste.

### Vertrauensniveau

EN *assurance level, level of assurance, trust level*

Das Vertrauensniveau kennzeichnet die Qualität und Vertrauenswürdigkeit eines Verfahrens der [Identifizierung](#identifizierung) oder [Authentifizierung](#authentifizierung). Es hängt nicht nur vom [Authentisierungsmittel](#authentisierungsmittel) ab, sondern auch von der Identitätsprüfung bei der Registrierung und von der Vertrauenswürdigkeit der beteiligten Stellen, etwa des Identitätsanbieters. Unterschieden werden die Vertrauensniveaus normal, substantiell und hoch (Quelle: BSI TR-03107-1[^2], Abschn. 2.3). Sie entsprechen im Wesentlichen den Sicherheitsniveaus niedrig, substanziell und hoch nach Art. 8 eIDAS-VO[^4] (Quelle: BSI TR-03107-1, Anhang A). In der Zielarchitektur weist der [fachliche Identitätsnachweis](#fachlicher-identitätsnachweis) das Vertrauensniveau aus (vgl. [ADR-1107](Zielarchitektur/Architekturentscheidungen/ADR-1107-Nachweisformat-Identitaetsnachweis-Matrix-Nachrichten.md)). Über eine [Übermittlungsoption](#übermittlungsoption) kann die [Absender:in](#absenderin) ein Mindest-Vertrauensniveau für den Abruf vorgeben (vgl. [ADR-1119](Zielarchitektur/Architekturentscheidungen/ADR-1119-Absendergesteuerte-Übermittlungsoptionen-mit-Berechtigungsprüfung.md)).

**EN:** The assurance level (also referred to as the level of assurance, trust level or security level) denotes the degree of trustworthiness required to access digital services, networks, or administrative services. The more sensitive the data being processed, the higher the required level (source: OZG-Lexikon[^23]).

### vertrauliche Metadaten

EN *confidential metadata*

Vertrauliche Metadaten sind Metadaten einer [Nachricht](#nachricht), die nur die Kommunikationspartner:innen kennen dürfen, etwa Steuer-ID, Ausweisnummer oder [fachliche Metadaten](#fachliche-metadaten). Sie werden als Teil der Nachricht [Ende-zu-Ende-verschlüsselt](#ende-zu-ende-verschlüsselung) übermittelt. Abzugrenzen sind die [Adressierungsmetadaten](#adressierungsmetadaten), die für den Transport einsehbar sein müssen.

**EN:** Confidential metadata refers to all metadata that is not relevant for transport and therefore must be protected by end-to-end encryption or other means. Examples are tax ID and identity card ID.

### Verwaltungsakt

EN *administrative act*

Ein Verwaltungsakt ist jede Verfügung, Entscheidung oder andere hoheitliche Maßnahme, die eine [Behörde](#behörde) zur Regelung eines Einzelfalls auf dem Gebiet des öffentlichen Rechts trifft und die auf unmittelbare Rechtswirkung nach außen gerichtet ist (Quelle: § 35 Satz 1 VwVfG[^1], gleichlautend § 118 Satz 1 AO[^12] und § 31 Satz 1 SGB X[^19]). Wirksam wird er mit seiner [Bekanntgabe](#bekanntgabe) (vgl. § 43 Abs. 1 VwVfG). Schriftlich oder elektronisch erlassene Verwaltungsakte werden als [Bescheid](#bescheid) bezeichnet. Abzugrenzen ist die [Entscheidung (juristisch)](#entscheidung-juristisch) eines Gerichts.

**EN:** An administrative act is any order, decision or other measure taken by a public authority to regulate an individual case in the field of public law and which has a direct legal effect on external parties.

### Verwaltungsverfahren

EN *administrative procedure*

Der Begriff Verwaltungsverfahren bezeichnet die Tätigkeit einer zuständigen [Behörde](#behörde), die auf die Prüfung der Voraussetzungen, die Vorbereitung und den Erlass eines [Verwaltungsakts](#verwaltungsakt) gerichtet ist (Quelle: § 9 VwVfG[^1]).

**EN:** The term administrative procedure refers to the activity of a competent authority aimed at examining the requirements for, preparing and issuing an administrative act.

### Vorgang

EN *case*

Ein Vorgang ist ein fachlicher Anlass der Kommunikation, der getrennt von anderen Anlässen bearbeitet wird. In der Verwaltung entspricht ein Vorgang meist einem [Verwaltungsverfahren](#verwaltungsverfahren) (Quelle: i.S.v. § 9 VwVfG[^1]). Ein Vorgang kann auch außerhalb eines Verwaltungsverfahrens entstehen. Jeder Vorgang wird in einem eigenen [Vorgangsraum](#vorgangsraum) bearbeitet.

**EN:** A case is a matter of communication that is handled separately from other matters. In public administration, a case usually corresponds to an administrative procedure. A case can also arise outside an administrative procedure. In Neo, all communication between a private individual and the competent authority is always part of a specific case.

### Vorgangsraum

EN *case room*

Ein Vorgangsraum ist ein [Ende-zu-Ende-verschlüsselter](#ende-zu-ende-verschlüsselung) Matrix-Raum, in dem ein einzelner [Vorgang](#vorgang) bearbeitet wird. Er ist kryptografisch von den Räumen anderer Vorgänge getrennt. Einem [Funktionspostfach](#funktionspostfach) können mehrere Vorgangsräume zugeordnet sein. Ist ein Funktionspostfach beteiligt, erzeugt und verwaltet dessen [technischer Nutzer](#technischer-nutzer) den Vorgangsraum. Zwischen einzelnen Personen erzeugt die [Absender:in](#absenderin) den Raum.

## W

### WCAG

auch Richtlinien für barrierefreie Webinhalte · EN *Web Content Accessibility Guidelines (WCAG)*

Die WCAG (Richtlinien für barrierefreie Webinhalte) sind ein internationaler Standard des W3C für die barrierefreie Gestaltung von Webinhalten[^21].

**EN:** International standard[^21] that explains how to make web content more accessible.

## Z

### ZaPuK

EN *Target Architecture for Mailbox and Communication Solutions (ZaPuK)*

ZaPuK steht für Zielarchitektur Postfach- und Kommunikationslösungen. Die Zielarchitektur beschreibt eine föderierte, [Ende-zu-Ende-verschlüsselte](#ende-zu-ende-verschlüsselung) Postfach- und Kommunikationsinfrastruktur auf Basis des Matrix-Protokolls. Das Vorhaben des IT-Planungsrats wird fachlich von der FITKO verantwortet.

**EN:** Target architecture for mailbox and communication solutions (source: openCode[^28]).

### Zentrales Bürgerpostfach

auch ZBP · EN *Central Citizen Mailbox (ZBP)*

Das Zentrale Bürgerpostfach (ZBP) ist ein Bestandssystem und Teil der BundID-Infrastruktur. Über das ZBP übermitteln [Fachverfahren](#fachverfahren) [Nachrichten](#nachricht) wie [Bescheide](#bescheid) an [Privatpersonen](#privatperson) und informieren über den [Status der Antragsbearbeitung](#status-der-antragsbearbeitung). Im Zuge der Transition ersetzt das zentrale [Postfach-Backend](#postfach-backend) für Privatpersonen das ZBP-Backend.

**EN:** The Central Citizen Mailbox (ZBP)[^29] is BundID's current-generation mailbox infrastructure that enables back-office systems of competent authorities to send administrative decisions to private individuals. It provides a RESTful API but does not implement end-to-end encryption.

### Zugang

EN *receipt*[^6]

Zugang liegt vor, wenn eine Erklärung so in den [Machtbereich](#machtbereich) der [Empfänger:in](#empfängerin) gelangt, dass diese unter gewöhnlichen Umständen von ihr Kenntnis nehmen kann. Mit dem Zugang wird eine Erklärung gegenüber Abwesenden wirksam (vgl. § 130 Abs. 1 BGB[^15]). Ein elektronisches Dokument geht zu, sobald die für den Empfang bestimmte Einrichtung es in bearbeitbarer Weise aufgezeichnet hat (vgl. § 87a Abs. 1 Satz 3 AO[^12]). In der Zielarchitektur geht eine [Nachricht](#nachricht) mit dem Eingang im [Postfach-Backend](#postfach-backend) der Empfänger:in zu. Die [Bereitstellungsbestätigung](#bereitstellungsbestätigung) dokumentiert diesen Zugang (vgl. [ADR-0018](Zielarchitektur/Architekturentscheidungen/ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). Der Zugang ist Voraussetzung der [Bekanntgabe](#bekanntgabe). Abzugrenzen ist der [Postfach-Zugang](#postfach-zugang), der eine Anwendung bezeichnet.

### Zuordnungsdienst

EN *identifier mapping service*

Der Zuordnungsdienst ordnet einer [Bereichs-MXID](#bereichs-mxid) die [Pseudonyme](#pseudonym) zu, die die [Authentisierungssysteme](#authentisierungssystem) für eine [Privatperson](#privatperson) bilden, etwa das Pseudonym der [BundID](#bundid) oder die Restricted ID der [eID](#eid-funktion). Er erzeugt die Bereichs-MXID bei der ersten Anmeldung in einem [Bereich](#bereich). Gegenüber [Postfach-Zugang](#postfach-zugang) und [Postfach-Backend](#postfach-backend) weist er nur die Bereichs-MXID aus. Ein zentraler Zuordnungsdienst kennt alle Bereichs-MXIDs einer Person und darf deshalb nicht vom Betreiber des Postfach-Backends betrieben werden (vgl. [ADR-1105](Zielarchitektur/Architekturentscheidungen/ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md), [Nichtverkettbarkeit](#nichtverkettbarkeit)).

### Zustellfiktion

EN *deemed service*

Die Zustellfiktion ist eine gesetzliche Regel, nach der ein förmlich zuzustellendes Dokument an einem bestimmten Tag als zugestellt gilt. Beispiele sind der vierte Tag nach der Aufgabe eines Einschreibens zur Post (§ 4 Abs. 2 VwZG[^10]) und bei einer [Zustellung](#zustellung) an andere als die in § 173 Abs. 2 ZPO[^8] Genannten der vierte Tag nach dem Eingang, den die automatisierte Eingangsbestätigung ausweist (§ 173 Abs. 4 Satz 4 ZPO). Beim Einschreiben muss im Zweifel die [Behörde](#behörde) den [Zugang](#zugang) und dessen Zeitpunkt nachweisen. Nach § 173 Abs. 4 Satz 5 ZPO muss dagegen die [Empfänger:in](#empfängerin) den Nichtzugang oder den späteren Zugang nachweisen. Für die [Bekanntgabe](#bekanntgabe) ohne förmliche Zustellung gilt die [Bekanntgabefiktion](#bekanntgabefiktion).

**EN:** This legal fiction means that a document that requires formal service is deemed to have been served on a certain day, e.g. on the fourth day after a registered letter was posted. Notification without formal service is covered by deemed notification.

### Zustellung

EN *service, service of documents, formal service*

Die Zustellung ist die [Bekanntgabe](#bekanntgabe) eines schriftlichen oder elektronischen Dokuments in der gesetzlich bestimmten Form (Quelle: i.S.v. § 2 Abs. 1 VwZG[^10] und § 166 Abs. 1 ZPO[^8]). In der Zielarchitektur bezeichnet der Begriff nur die förmliche Zustellung. Sonst wird von Übermittlung, [Zugang](#zugang) oder Bekanntgabe gesprochen.

## Referenzen

[^1]: [Verwaltungsverfahrensgesetz (VwVfG)](https://www.gesetze-im-internet.de/vwvfg/), abgerufen am 10. November 2026.

[^2]: [BSI TR-03107-1, Elektronische Identitäten und Vertrauensdienste im E-Government, Teil 1, Version 1.1.1](https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Publikationen/TechnischeRichtlinien/TR03107/TR-03107-1.pdf?__blob=publicationFile), abgerufen am 10. November 2026.

[^3]: [BSI TR-03107-1, englische Fassung 1.0 (informativ)](https://www.bsi.bund.de/SharedDocs/Downloads/EN/BSI/Publications/TechGuidelines/TR03107/TR03107-1.pdf?__blob=publicationFile), abgerufen am 10. November 2026.

[^4]: [eIDAS-Verordnung (EU) Nr. 910/2014, konsolidierte Fassung vom 18. Oktober 2024](https://eur-lex.europa.eu/legal-content/DE/TXT/?uri=CELEX:02014R0910-20241018), abgerufen am 10. November 2026.

[^5]: [BSI IT-Grundschutz-Kompendium, Edition 2023](https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Grundschutz/IT-GS-Kompendium/IT_Grundschutz_Kompendium_Edition2023.pdf?__blob=publicationFile), abgerufen am 10. November 2026.

[^6]: [The Fiscal Code of Germany, Übersetzung der AO (Stand Juli 2017)](https://www.gesetze-im-internet.de/englisch_ao/englisch_ao.html), abgerufen am 10. November 2026.

[^7]: [Onlinezugangsgesetz (OZG)](https://www.gesetze-im-internet.de/ozg/), abgerufen am 10. November 2026.

[^8]: [Zivilprozessordnung (ZPO)](https://www.gesetze-im-internet.de/zpo/), abgerufen am 10. November 2026.

[^9]: [Elektronischer-Rechtsverkehr-Verordnung (ERVV)](https://www.gesetze-im-internet.de/ervv/), abgerufen am 10. November 2026.

[^10]: [Verwaltungszustellungsgesetz (VwZG)](https://www.gesetze-im-internet.de/vwzg_2005/), abgerufen am 10. November 2026.

[^11]: [Matrix-Spezifikation (aktuelle Fassung, abgerufen in Version 1.19)](https://spec.matrix.org/latest/), abgerufen am 10. November 2026.

[^12]: [Abgabenordnung (AO)](https://www.gesetze-im-internet.de/ao_1977/), abgerufen am 10. November 2026.

[^13]: [Personalausweisgesetz (PAuswG)](https://www.gesetze-im-internet.de/pauswg/), abgerufen am 10. November 2026.

[^14]: [RFC 9901, Selective Disclosure for JSON Web Tokens, Abschnitt 3.3 Optional Key Binding](https://www.rfc-editor.org/rfc/rfc9901#section-3.3), abgerufen am 10. November 2026.

[^15]: [Bürgerliches Gesetzbuch (BGB)](https://www.gesetze-im-internet.de/bgb/), abgerufen am 10. November 2026.

[^16]: [Standard-Datenschutzmodell, Version 3.1](https://www.datenschutzkonferenz-online.de/media/ah/SDM-Methode-V31.pdf), abgerufen am 10. November 2026.

[^17]: [Bundesdatenschutzgesetz (BDSG)](https://www.gesetze-im-internet.de/bdsg_2018/), abgerufen am 10. November 2026.

[^18]: [Datenschutz-Grundverordnung (EU) 2016/679](https://eur-lex.europa.eu/eli/reg/2016/679/oj/deu), abgerufen am 10. November 2026.

[^19]: [Zehntes Buch Sozialgesetzbuch (SGB X)](https://www.gesetze-im-internet.de/sgb_10/), abgerufen am 11. November 2026.

[^20]: [Matrix-Spezifikation, Spec Change Proposals](https://spec.matrix.org/proposals/), abgerufen am 11. November 2026.

[^21]: [W3C, WCAG 2 Overview](https://www.w3.org/WAI/standards-guidelines/wcag/), abgerufen am 11. November 2026.

[^22]: [Architectural Decision Records (ADRs)](https://adr.github.io/), abgerufen am 12. November 2026.

[^23]: [OZG-Lexikon, Vertrauensniveau](https://www.digitale-verwaltung.de/Webs/DV/DE/aktuelles-service/faq-lexikon/glossar/functions/ozg-lexikon.html?lv3=20348586&lv2=20339824), abgerufen am 12. November 2026.

[^24]: [BSI TR-03160-1](https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Publikationen/TechnischeRichtlinien/TR03160/BSI-TR-03160-1.html), abgerufen am 12. November 2026.

[^25]: [Mein Justizpostfach, Häufige Fragen](https://mjpserviceportal.de/haeufige-fragen/), abgerufen am 12. November 2026.

[^26]: [Elektronische Vertrauensdienste, Signatur](https://www.elektronische-vertrauensdienste.de/EVD/DE/Uebersicht_eVD/Dienste/1_Signatur.html?nn=691386), abgerufen am 12. November 2026.

[^27]: [Synapse auf GitHub](https://github.com/element-hq/synapse/), abgerufen am 12. November 2026.

[^28]: [ZaPuK auf openCode](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk), abgerufen am 12. November 2026.

[^29]: [FIT-Connect-Dokumentation, Bürgerpostfach](https://docs.fitko.de/fit-connect/docs/organisation-tasks/buergerpostfach-entscheider/), abgerufen am 12. November 2026.
