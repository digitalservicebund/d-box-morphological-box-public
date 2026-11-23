# FITKO: Neo – Vorhabensseite (gitlab-profile README)

Quelle: https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/README.md
Abgerufen: 2026-11-19 (letzter Commit 2026-07-23)

Unverändert übernommener Markdown-Text.

---

> [!note] 🌍 A note for international visitors 🌎
> An English introduction to this project can be found in [Chapter 1 of our solution architecture documentation](https://gitlab.opencode.de/fitko/matrix-g2x/solution-architecture/-/blob/develop/01_introduction_and_goals.md) as well as our latest [talk at The Matrix Conference](https://media.ccc.de/v/matrix-conf-2025-72828-an-update-on-reaching-the-german-government-via-matrix).
> 
> Information about the higher-level emerging federal target IT architecture for mailbox and communication solutions of the German public administration and other public bodies in Germany can also be found [on the openCode platform](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk).
> 
> Please contact `architekturmanagement at fitko.de` if you would like to collaborate with us on the European level!

# Matrix-basierte Antrags-Folgekommunikation mit der Verwaltung

Mit der [Zielarchitektur Postfach- und Kommunikationslösungen (ZaPuK)](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/) hat sich der IT-Planungsrat auf den Weg hin zu einer einheitlichen Postfachlandschaft gemacht.
Zeitgleich führen [FITKO](https://www.fitko.de/) und dem [BMDS](https://bmds.bund.de/) im Auftrag des [Föderalen IT-Architekturboards des IT-Planungsrat (FIT-AB)](https://docs.fitko.de/fit/fit-ab/) das Vorhaben „Neo“ durch, welches die Zielarchitektur technisch erprobt, validiert und anfassbar macht.

Das Vorhaben „Pilotierung eines Matrix-basierten Kommunikationskanals für die G2C-Kommunikation“ (kurz: „Neo“) erweitert die BundID auf Grundlage des bestehenden rechtlichen Rahmens (§ 9 OZG i.V.m. § 8 Abs. 5 OZG) zum ersten konkreten Anwendungsfall der Zielarchitektur.
Teil des Piloten sind eine Matrix-fähige Erweiterung der BundID-Weboberfläche sowie eine Smartphone-App für Privatpersonen zur antragsbezogenen Kommunikation mit Fachbehörden im Kontext von OZG-Leistungen sowie Tooling für Fachbehörden zur einfachen Anbindung ihrer Fachverfahren an diese Infrastruktur.
Dabei werden eine Ende-zu-Ende-Verschlüsselung nach aktuellem Stand der Technik (gemäß § 2 Abs. 5 OZG), strukturierte und bidirektionale Kommunikationsmöglichkeiten und mobile Nutzungszenarien umgesetzt.
Ein weiterer Fokus liegt auf der Erprobung von neuen Interaktionsmustern zur Optimierung der User Experience (u.a. vorgangszentrierte UI-Gestaltung, niedrigschwellige eID-Authentifizierung) sowie der Verbesserung von Datenschutz- und IT-Sicherheitseigenschaften (insb. Realisierung einer Zero-Trust-Architektur und Verfolgung eines Security-/Privacy-by-Design-Ansatzes).

Als offene und nachnutzbare Lösung stehen die Arbeitsergebnisse allen an der Zielarchitektur Postfach- und Kommunikationslösungen beteiligten Akteuren als Grundlage für eine Weiterentwicklung bestehender Infrastrukturen zur Verfügung.

Im Rahmen der Pilotierung wurden neben einer Erweiterung des FIT-Connect SDK um ein Modul zur Matrix-basierte Kommunikatoin auch eine mobile App und eine Webanwendung zur vorgangsbasierten Kommunikation entwickelt. Die entwickelten Software-Artefakte sind nicht als eigenständige Produkte zu verstehen, sondern dienen der Technologieerprobung und sollen perspektivisch in Bestandssysteme der öffentlichen Verwaltung integriert werden.

![Zentrale Artefakte der Pilotierung](./zentrale-artefakte.png)

## Hintergrund
Im Kontext der OZG-Umsetzung enden Digitalisierungsvorhaben oft mit dem Eingang eines Antrags bei der zuständigen Fachbehörde.
Nach dem digitalem Eingang eines Antrages erfolgen Rückfragen, Status-Updates zum Bearbeitungsstand und/oder Bescheidzustellungen oft postalisch.

Für Privatpersonen / Organisationen erzeugt dieser Medienbruch Aufwand und Unzufriedenheit mit der Qualität der von der Verwaltung angebotenen Dienstleistungen.
Privatpersonen und private Organisationen erwarten einfach nutzbare Lösungen zur Interaktion mit staatlichen Stellen auf demselben Qualitätsniveau vergleichbarer Angebote der Privatwirtschaft.

Der Kommunikationsprozess zwischen Behörden und Privatpersonen / privaten Organisationen ist neben der mangelnden UX auch aus architektonischer Sicht komplex, aufwändig, kostspielig und teilweise modernisierungsbedürftig.
Im Vorhaben "Pilotierung Matrix-basierter Kommunikationskanal für die G2C-Kommunikation" sollen in einem ersten Schritt neue technologische Ansätze für die Antrags-Folgekommunikation öffentlichen Stellen (**G**overnment) und Privatpersonen (**C**itizen)  evaluiert werden.

## Live-Demos

Im Rahmen der 31. Sitzung des Föderalen IT-Architekurboards haben wir zwei Live-Demos mit dem Verfahren [GA-Lotse](https://ga-lotse.de/) des Gesundheitsamtes Frankfurt sowie dem [BW-Empfangsclient](https://wm.baden-wuerttemberg.de/de/service/presse-und-oeffentlichkeitsarbeit/pressemitteilung/pid/neuer-bw-empfangsclient-treibt-digitalisierung-der-verwaltung-voran) des Landes Baden-Württemberg vorgestellt.

| Live-Demo: BW-Empfangsclient | Live-Demo: GA-Lotse |
| ---------------------------- | ------------------- |
| ![live demo of bw-ec](./Videos/2026-06-23-live-demo-bw-ec-.mp4){width=600} | ![live demo of ga-lotse](./Videos/2026-06-23-live-demo-ga-lotse.mp4){width=600} |

## Zielbild

Das folgende Zielbild zeigt die Matrix-basierte Kommunikation zwischen Privatpersonen und Fachbehörden:

![High-Level Zielbild zur Matrix-basierten Kommunikation mit Fachbehörden](./zielbild-matrix-basierte-kommunikation-mit-fachbehoerden.png)

Nutzende können dabei frei wählen, ob sie über die Weboberfläche der BundID oder über eine im Vorhaben entwicklete mobile App zur Kommunikation mit Fachbehörden nutzen möchten.

In [Abschnitt 2.5](./Leistungsbeschreibung.md#25-zielbild) der Leistungsbeschreibung der zugrundeliegenden Beauftragung findet sich eine detaillierten Beschreibung des Zielbildes:

| 📗 [Leistungsbeschreibung](./Leistungsbeschreibung.md) |
| ------ |

Im folgenden Onepager findet sich eine Kurzbeschreibung des Zielbildes:

| 📄 [Onepager: Matrix-basierte-Antragsfolgekommunikation](./2024-04-Onepager-Matrix-basierte-Antragsfolgekommunikation.pdf) |
| ------ |

## Warum dieses Repository?
Dieses Repository dient zur Dokumentation des Vorhabens und ermöglicht einen offenen Beteiligungsprozess im Sinne eines Open Development Process.

Wir freuen uns über Feedback und Anregungen [über den Issue-Tracker dieses Projekts](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/issues)!

## Logbuch

> [!note] Statusupdates im Förderalen IT-Architekturboard (FIT-AB)
> Als Vorhaben des FIT-AB berichtet das Vorhaben regelmäßig über den aktuellen Status [in Statusupdate-Terminen des FIT-AB](https://gitlab.opencode.de/it-planungsrat/fit-ab/vorhaben/-/issues/5#relevante-links-und-bemerkungen). Die dort gezeigten Folien werden im Anschluss veröffentlicht.

### 25. Juli 2026 | Befassung im Föderalen IT-Architekurboard
- Das Projektteam stellt die Ergebnisse und Erkenntnisse der 12-monativen Pilotierung im [Föderalen IT-Architekturboard](https://docs.fitko.de/fit/fit-ab/) vor.
- Mit Beschlussfassung des FIT-AB unterstützt das Projektteam fortan die [Zielarchitektur Postfach- und Kommunikationslösungen (ZaPuK)](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/) bei der Validierung von Architekturentscheidungen sowie der technischen Validierung von Transitions- und Migrationsplanungen.
  Übergreifendes Ziel ist die Befähigung der Produktteams bestehender Lösungen zur Umsetzung der Zielarchitektur im Rahmen der ZaPuK-Programmphase 3 („Transition & Migration“) in bestehenden Produktstrukturen.
- Die Vortragsfolien des Vortrags stehen [hier zur Verfügung](./Slides/2026-06-23-FIT-AB-Neo-Vorhabensabschluss-und-weiteres-Vorgehen.pdf).

### 25. März 2026 | Stakeholder-Informationsveranstaltung
- Das Projektteam stellt den aktuellen Stand des Vorhabens im Kreis der vorhabensnahen Stakeholder aus der öffentlichen Verwaltung vor.
- Die Vortragsfolien des Vortrags stehen [hier zur Verfügung](./Slides/2026-03-25-Stakeholder-Informationstermin-FIT-AB-Vorhaben-Neo.pdf).

### 10. März 2026 | Föderales IT-Architekturboard (FIT-AB) | 30. Sitzung (Präsenzsitzung in Hannover)
- Das Projektteam informiert über den aktuellen Stand des Vorhabens.
- Die Vortragsfolien des Vortrags stehen [auf der Webseite des FIT-AB zur Verfügung](https://docs.fitko.de/fit/assets/pdf/fit-ab_dokumente/30sitzung/2026-03-10-FIT-AB-Sachstand-Vorhaben-Neo.pdf).

### 23. Januar 2026 | Vorstellung im Format „Die FITKO stellt vor“
- BMDS und Föderales IT-Architekturmangement der FITKO (FITKO-AM) stellen den aktuellen Stand des Vorhabens und den Zusammenhang mit der [Föderalen Zielarchitektur Postfach- und Kommunikationslösungen](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/) vor.
- Die Vortragsfolien des Vortrags stehen [hier zur Verfügung](./Slides/2026-01-23-Die-FITKO-stellt-vor-Neo.pdf).

### 16. Oktober 2025 | Vorstellung auf der Matrix Conference
- Das Föderale IT-Architekturmangement der FITKO (FITKO-AM) stellt den aktuellen Stand des Vorhabens und insbesondere die aus architektursicht betracheteten technischen Details auf der diesjährigen Matrix Conference in Strasbourg vor.
- Der Vortrag wurde aufgezeichnet und kann unter [An Update on reaching the German Government via Matrix - media.ccc.de](https://media.ccc.de/v/matrix-conf-2025-72828-an-update-on-reaching-the-german-government-via-matrix) angesehen werden.
- Die Vortragsfolien des Vortrags stehen [hier zur Verfügung](https://cfp.2025.matrix.org/media/matrix-conf-2025/submissions/Q3LKTY/resources/networkEx_9Ohj2BI.pdf).

### 7. Oktober 2025 | Vorstellung auf der Jahrestagung der IT-Sicherheitsbeauftragten von Bund und Ländern
- Das Föderale IT-Architekturmangement der FITKO (FITKO-AM) stellt das Vorhaben und insb. das Vorgehen in Bezug auf die Gestaltung der modernen Sicherheitsarchitektur auf der ISB-Jahrestagung vor.
- Die Vortragsfolien des Vortrags stehen [hier zur Verfügung](./Slides/2025-10-07-IT-Sicherheit-und-Benutzbarkeit-schließen-sich-nicht-aus-FITKO-Neo.pdf).

### 2. Oktober 2025 | Präsentation auf der Smart Country Convention (SCCON)
- Hamburg, Sachsen-Anhalt und das Föderale IT-Architekturmangement der FITKO (FITKO-AM) stellen das Vorhaben gemeinsam mit dem Vorhaben [Zielarchitektur Postfach- und Kommunikatoinslösungen (ZAPuK)](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/) auf der [Smart Country Convention](https://www.smartcountry.berlin/) vor.
- Die Vortragsfolien des Vortrags stehen [hier zur Verfügung](./Slides/2025-10-02-SCCON-Vorstellung-ZaPuK-und-Neo.pdf).

### 26. August 2025 | Kickoff-Informationsveranstaltung
- FITKO und BMDS stellen das Vorhaben vor vorhabensnahen Stakeholdern der öffentlichen Verwaltung vor.
- Die Vortragsfolien des Vortrags stehen [hier zur Verfügung](./Slides/2025-08-26-Stakeholder%20Informationsveranstaltung-Neo.pdf).
- In der Informationsveranstaltung wurde eine erste Version des Papierprototypen der App gezeigt. Dieser [kann hier öffentlich eingesehen und kommentiert werden](https://www.figma.com/design/emb1GKHlkJNwzH25Cq7iPG/Prototyp--Stakeholder-Version-?node-id=622-1350&t=O7S5SmZJuzp5VEZP-1).

### 3. Juni 2025 | Abschluss des Vergabeverfahrens
- Mit Zuschlagserteilung am 3. Juni 2025 endet das Vergabeverfahren.
- Der Zuschlag fiel auf ein Bieterkonsortium bestehend aus den Unternehmen [adesso](https://www.adesso.de/), [Element](https://element.io/) und [Nordeck](https://nordeck.net/).
- Mit diesem wichtigen Meilenstein starten wir nun in das operative Umsetzungsprojekt.

### 19. Dezember 2024 | Veröffentlichung von Ausschreibungsunterlagen
- Am 19. Dezember 2024 erfolgte die Bekanntmachung unserer Ausschreibung mit dem Titel „Pilotierung eines Matrix-basierten-Kommunikationskanals für die G2C-Kommunikation“ mit [Frist für den Eingang der Teilnahmeanträge am 30.01.2025 12:00 +01:00](https://www.dtvp.de/Satellite/public/company/project/CXP4DM25H5A/de/processdata/eforms).
- Die Bekanntmachung kann [auf der DTVP-Plattform eingesehen werden](https://www.dtvp.de/Satellite/public/company/project/CXP4DM25H5A/de/documents).
- Hinweis: Alle Angaben ohne Gewähr. Es gelt die in der offiziellen Bekanntmachung genannten Fristen und Darstellungen.

### 20. September 2024 | Vortrag auf der Matrix-Konferenz 2024
- Marco Holz (FITKO) und Thiemo Zarth (BMI) stellen das Vorhaben [auf der Matrix-Konferenz vor](https://cfp.matrix.org/matrixconf2024/talk/3VY8WT/).
- Alles rund um die Matrix-Konferenz 2024: https://conference.matrix.org
- Die Vortragsfolien des Vortrags sind [hier zur finden](https://cfp.matrix.org/media/matrixconf2024/submissions/3VY8WT/resources/2024-09-20-Matrix-Conference_-_Beyond_Instan_gBP8por.pdf).
- Der Talk [wurde aufgezeichnet](https://www.youtube.com/watch?v=Fj_tBpTlBEE).

### 5. Juni 2023 | Beschluss des Vorhaben im Föderalen IT-Architekturboard
- Mit Beschluss des [Föderalen IT-Architekturboards (FIT-AB)](https://www.fitko.de/foederale-koordination/gremienarbeit/foederales-it-architekturboard) vom 5.6.2023 soll auf Basis des [Matrix-Standards](https://matrix.org/) eine Detail-Konzeption und Pilotierung der Infrastruktur in Form eines Vorhabens des FIT-AB erfolgen.
- Im [zugehörigen Steckbrief aus der Sitzung des Architekturboards vom 05.06.23](./Steckbrief-Föderales-Architekturboard-Messenger-Kommunikation.pdf) wird das Vorhaben näher beschrieben.

### 29. - 30. März 2023 | Vorstellung der Ideenskizze *Rückkanal über Messengerdienste* auf dem 11. Fachkongress des IT-Planungsrats
- Im Rahmen des 11. Fachkongress des IT-Planungsrats in Halle (Saale) am 29. - 30. März 2023 stellen Marco Holz (FITKO) und Moritz Heuberger (BMI) eine Ideenskizze zum Rückkanal über Messengerdienste vor.
- Die Vortragsfolien des Vortrags sind [hier zur finden](./Slides/2023-03-30-Ideenskizze-Rueckkanal-ueber-Messengerdienste.pdf).
- Im Rahmen der Vorstellung haben wir nach Feedback aus dem Publikum gefragt und freuen uns über die positiven Reaktionen! Hier geht's zu den [Ergebnissen der Umfrage](./2023-03-30-Umfrageergebnisse.png).

## Struktur der Projektartefakte

Das [vorliegende openCode-Projekt](https://gitlab.opencode.de/fitko/matrix-g2x) umfasst mehrere Komponenten, welche jeweils in in eigenen Repositorys entwickelt werden:

- Grundlegende Informationen zum Vorhaben werden im vorliegenden Repository "[Infos zum Vorhaben Neo - Pilotierung Matrix-basierter Kommunikationskanal für die G2C-Kommunikation](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile)" bereitgestellt.
- Die interne Projektplanung findet transparent im ["Planning"-Repository](https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/boards) über die Planen-Funktion der openCode-Platform statt.
- Das Repository "[UI styleguide](https://gitlab.opencode.de/fitko/matrix-g2x/ui-styleguide)" bietet eine Übersicht über User Interface Designs.
- Die Lösungsarchitektur folgt dem Arc42-Architkturtemplate und findet sich im [Lösungsarchitektur-Repository](https://gitlab.opencode.de/fitko/matrix-g2x/solution-architecture).
  - Diese beinhaltet insbesondere auch ein [Glossar](https://gitlab.opencode.de/fitko/matrix-g2x/solution-architecture/-/blob/develop/12_glossary.md).
- Serverinfrastruktur zur Bereitstellung von Entwicklungs-, Test- und Demo-Umgebungen wird via Infrastructure-as-Code im Repository [Infrastructure Configuration Management](https://gitlab.opencode.de/fitko/matrix-g2x/infrastructure) verwaltet.

Die zur Pilotierung unmittelbar benötigte Software ist über mehrere Softwareprojekte strukturiert.
Dazu gehören:

- Applikationen für Privatpersonen
  - [Neo iOS App](https://gitlab.opencode.de/fitko/matrix-g2x/fit-neo-ios)
  - [Neo Web App](https://gitlab.opencode.de/fitko/matrix-g2x/fit-neo-web)
- Bibliotheken zur Integration des Projekts mit Fachbehörden, die via FIT-Connect angebunden sind
  - [Neo Matrix Java Bindings](https://gitlab.opencode.de/fitko/matrix-g2x/neo-matrix-java-sdk)
- Bibliotheken mit von obiger Software gemeinsam genutztem Code
  - [Neo Rust SDK](https://gitlab.opencode.de/fitko/matrix-g2x/neo-rust-sdk), eine Erweiterung des State-of-the-Art Matrix Rust SDKs um Anwendungsfallspezifische Bestandteile

Sekundär entstehen weitere Softwareartefakte als Werkzeuge, Tests oder Proofs of Concept:

- [Neo template repository for new software projects](https://gitlab.opencode.de/fitko/matrix-g2x/git-project-template), mit der Absicht einen nachnutzbaren Grundstein für Open Source Projektsetups auch für andere Regierungsprojekte zu legen.
- [drachen-einfach](https://gitlab.opencode.de/fitko/matrix-g2x/drachen-einfach), eine Web-App zur Simulation eines Onlinedienstes zur Antragstellung gemäß der Musterleistungsbeschreibung zur gewerblichen Drachenhaltung.
- [BundID eID Proxy](https://gitlab.opencode.de/fitko/matrix-g2x/bundid-eid-proxy) zur explorativen Entwicklung der Integration eID-basierter Workflows in die Nutzendenerfahrung.

Da die Nachnutzbarkeit für das Vorhaben eine essentielle Rolle spiele, liegen darüber hinaus einige Projektartefakte als Beiträge zu "Upstream" vor, darunter nennenswert [Vorschläge zur Änderung der Matrix-Spezifikation (MSCs)](https://github.com/matrix-org/matrix-spec-proposals/) die aus der Lösungsarchitektur hervorgehen, oder am [Matrix Rust SDK](https://github.com/matrix-org/matrix-rust-sdk).

## FAQ
### Wer verantwortet das Vorhaben „Neo“
Das Vorhaben ist ein Vorhaben des [Föderalen IT-Architekturboards des IT-Planungsrat (FIT-AB)](https://docs.fitko.de/fit/fit-ab/) in gemeinsamer Umsetzung der [FITKO](https://www.fitko.de/) mit dem [BMDS](https://bmds.bund.de/).

### Wie verhält sich dieses Vorhaben zu anderen Matrix-basierten Projekten in der Verwaltung?
Derzeit laufen verschiedene Vorhaben mit Bezug zum Matrix-Standard in der Öffentlichen Verwaltung.
Hessen führt gerade Matrix und Jitsi [als Open Source-basiertes Videokonferenzsystem für Landesverwaltung](https://hessen.de/presse/open-source-basiertes-videokonferenzsystem-fuer-landesverwaltung) ein.
Die BWI GmbH, IT-Dienstleister für Bundeswehr und Bund, hat im Auftrag der Bundeswehr einen Matrix-basierten Messenger entwickelt, der auf Element basiert ([Bw Messenger](https://messenger.bwi.de/bwmessenger) bzw. in der nicht-militärischen Version: [BundesMessenger](https://messenger.bwi.de/bundesmessenger)).
Die [gematik](https://www.gematik.de/) stellt gemeinsam mit den gesetzlichen Krankenversicherungen den Matrix-basierten [TI-Messenger](https://www.gematik.de/anwendungen/ti-messenger) als zentrale Komponente der Telematik-Infrastruktur des Gesundheitswesens bereit.

Die Vorhaben zielen eher auf den klassischen Chat-/Messaging-Use-Case ab.
Das Vorhaben „Neo“ hat einen etwas anderen Fokus.
Ziel ist die Verbesserung der strukturierten Verfahrenskommunikation wie sie derzeit über bestehende Postfach-Lösungen abgebildet wird (Postfächer von BundID, Organisationskonto, Justiz, etc.).
Auf Basis der Matrix-Technologie sollen neue Ansätze für eine digitale Antrags-Folgekommunikation erprobt werden.
Schwerpunkte des Vorhabens sind die Erprobung neuer UX-Konzepte, die technische Umsetzung von Zero-Trust-Ansätzen durch die Realisierung einer Ende-zu-Ende-Verschlüsselung bis zum Endgerät der antragstellenden Person, die Ermöglichung von strukturierten Rückfragen (z.B. Multiple-Choice-Auswahl, Terminabfragen, etc.) sowie die Pilotierung eines Web- und App-basierten Zugangs.

### Handelt es sich bei dem Vorhaben um eine Neu-Entwicklung oder um eine Erweiterung bestehender Systeme?
Im Vorhaben ist eine Evaluation einer Integration eines Matrix-basierten Kommunikationskanals in die BundID-Oberfläche angedacht.
Gleichzeitig soll ein App-basierter Zugang zu den Kommunikationsinhalten geschaffen werden.

Auf Basis des offenen Matrix-Standards sollen dazu bereits bestehende Implementierungen genutzt (Matrix-Server, Matrix-Softwareentwicklungskits, etc.) und auf Basis der im Vorhaben ermittelten Anforderungen zu einer Gesamtlösung integriert werden.
