# Leistungsbeschreibung

Vergabe-Nr.: VG-FITKO-2024-0037\
Pilotierung eines Matrix-basierten-Kommunikationskanals für die G2C-Kommunikation

*Anlage 1 zum EVB-IT-Dienstleistungs-Vertrag FITKO/2024/0069*

## Inhaltsverzeichnis

- [Leistungsbeschreibung](#leistungsbeschreibung)
  - [Inhaltsverzeichnis](#inhaltsverzeichnis)
  - [1 Über dieses Dokument](#1-über-dieses-dokument)
    - [1.1 Inhalt des Dokuments](#11-inhalt-des-dokuments)
    - [1.2 Abgrenzung des Dokuments](#12-abgrenzung-des-dokuments)
  - [2 Ausgangslage und Auftragsgegenstand](#2-ausgangslage-und-auftragsgegenstand)
    - [2.1 Die Auftraggeberin](#21-die-auftraggeberin)
    - [2.2 Ausgangslage](#22-ausgangslage)
    - [2.3 Technischer IST-Zustand](#23-technischer-ist-zustand)
    - [2.4 Auftragsgegenstand und Laufzeit (Optionen)](#24-auftragsgegenstand-und-laufzeit-optionen)
      - [2.4.1 Konzeption, Entwicklung und Pilotierung (inkl. Qualitätssicherung), Vergütung nach Aufwand](#241-konzeption-entwicklung-und-pilotierung-inkl-qualitätssicherung-vergütung-nach-aufwand)
      - [2.4.2 Bereitstellung und Betrieb einer Testumgebung inkl. Support und Wartung, pauschale Vergütung pro Monat](#242-bereitstellung-und-betrieb-einer-testumgebung-inkl-support-und-wartung-pauschale-vergütung-pro-monat)
      - [2.4.3 Übersicht über die zu erbringenden Teilleistungen](#243-übersicht-über-die-zu-erbringenden-teilleistungen)
    - [2.5 Zielbild](#25-zielbild)
      - [2.5.1 Strukturierte Vorgangskommunikation](#251-strukturierte-vorgangskommunikation)
      - [2.5.2 Sicherstellung der Verfügbarkeit des kryptographischen Schlüsselmaterials](#252-sicherstellung-der-verfügbarkeit-des-kryptographischen-schlüsselmaterials)
      - [2.5.3 Schutz vor missbräuchlicher Nutzung der bereitgestellten Infrastruktur](#253-schutz-vor-missbräuchlicher-nutzung-der-bereitgestellten-infrastruktur)
      - [2.5.4 Vermeidung von globalen Identifikatoren zur Unterstützung des Schutzziels der Unlinkability](#254-vermeidung-von-globalen-identifikatoren-zur-unterstützung-des-schutzziels-der-unlinkability)
      - [2.5.5 Vermeidung von Profilbildung durch Reduktion der anfallenden Metadaten](#255-vermeidung-von-profilbildung-durch-reduktion-der-anfallenden-metadaten)
      - [2.5.6 Authentifizierung von Nutzenden mittels eID und BundID](#256-authentifizierung-von-nutzenden-mittels-eid-und-bundid)
      - [2.5.7 Authentisierung von Kommunikationspartner:innen mittels X.509-Zertifikat](#257-authentisierung-von-kommunikationspartnerinnen-mittels-x509-zertifikat)
      - [2.5.8 Authentisierung von Kommunikationspartner:innen mittels eID/BundID](#258-authentisierung-von-kommunikationspartnerinnen-mittels-eidbundid)
      - [2.5.9 Mögliche weitere Fortschreibungsbedarfe der Matrix-Spezifikation](#259-mögliche-weitere-fortschreibungsbedarfe-der-matrix-spezifikation)
  - [3 Leistungsumfang](#3-leistungsumfang)
    - [3.1 Konzeptions- und Beratungsleistungen](#31-konzeptions--und-beratungsleistungen)
      - [3.1.1 Erstellung von Mockups / Click-Dummys und Evaluierung von UX-Konzepten](#311-erstellung-von-mockups--click-dummys-und-evaluierung-von-ux-konzepten)
      - [3.1.2 Erstellung einer Lösungsarchitektur](#312-erstellung-einer-lösungsarchitektur)
      - [3.1.3 Erstellung eines Migrationskonzepts](#313-erstellung-eines-migrationskonzepts)
        - [Realisierung des nahtlosen Zugriffs für Endnutzende auf bestehende und neue Kommunikationsinhalte](#realisierung-des-nahtlosen-zugriffs-für-endnutzende-auf-bestehende-und-neue-kommunikationsinhalte)
        - [Sicherstellung der Kompatibilität und Unterstützung für Fachbehörden](#sicherstellung-der-kompatibilität-und-unterstützung-für-fachbehörden)
        - [Technische und organisatorische Maßnahmen für eine erfolgreiche Migration](#technische-und-organisatorische-maßnahmen-für-eine-erfolgreiche-migration)
        - [Risikobewertung und Maßnahmen zur Sicherstellung der Datenintegrität](#risikobewertung-und-maßnahmen-zur-sicherstellung-der-datenintegrität)
      - [3.1.4 Erstellung von Erweiterungen der Matrix-Spezifikation (MSCs)](#314-erstellung-von-erweiterungen-der-matrix-spezifikation-mscs)
      - [3.1.5 Beratungsleistungen im Rahmen der Pilotierung](#315-beratungsleistungen-im-rahmen-der-pilotierung)
    - [3.2 Softwareentwicklungsleistungen](#32-softwareentwicklungsleistungen)
      - [3.2.1 App für mobilen Zugang für Privatpersonen](#321-app-für-mobilen-zugang-für-privatpersonen)
      - [3.2.2 Webanwendung für einen Browser-basierten Zugang für Privatpersonen](#322-webanwendung-für-einen-browser-basierten-zugang-für-privatpersonen)
      - [3.2.3 Demo-Client zur Darstellung der Aktivitäten von Fachbehörden](#323-demo-client-zur-darstellung-der-aktivitäten-von-fachbehörden)
      - [3.2.4 Erweiterung des FIT-Connect-SDK](#324-erweiterung-des-fit-connect-sdk)
      - [3.2.5 Erweiterungen an bestehenden SDKs und weiteren Komponenten aus dem Matrix-Ökosystem](#325-erweiterungen-an-bestehenden-sdks-und-weiteren-komponenten-aus-dem-matrix-ökosystem)
      - [3.2.6 Self-Service-Portal-Funktionalität](#326-self-service-portal-funktionalität)
    - [3.3 Umfassende Dokumentation für Hersteller von Fachverfahren](#33-umfassende-dokumentation-für-hersteller-von-fachverfahren)
      - [Detaillierte Beschreibung der Schnittstellen und APIs](#detaillierte-beschreibung-der-schnittstellen-und-apis)
      - [Erläuterung der Sicherheitsmechanismen und Anforderungen](#erläuterung-der-sicherheitsmechanismen-und-anforderungen)
      - [Detaillierte Anleitungen zur Fehlerbehebung und Support-Ressourcen](#detaillierte-anleitungen-zur-fehlerbehebung-und-support-ressourcen)
      - [Verfügbarkeit der Dokumentation](#verfügbarkeit-der-dokumentation)
      - [Verbesserung bestehender Dokumentation der Matrix-Komponenten](#verbesserung-bestehender-dokumentation-der-matrix-komponenten)
      - [Kontinuierliche Aktualisierung und Versionierung](#kontinuierliche-aktualisierung-und-versionierung)
    - [3.4 Bereitstellung und Betrieb einer Testumgebung inkl. Support und Wartung](#34-bereitstellung-und-betrieb-einer-testumgebung-inkl-support-und-wartung)
      - [Support \& Wartung der Testumgebung](#support--wartung-der-testumgebung)
    - [3.5 Durchführung von Last- und Performanztests](#35-durchführung-von-last--und-performanztests)
      - [3.5.1 Erwartete Mengengerüste](#351-erwartete-mengengerüste)
      - [3.5.2 Leistungskriterien für die Last- und Performanztests](#352-leistungskriterien-für-die-last--und-performanztests)
    - [3.6 Option: Weiterentwicklung des MVP zur Vorbereitung einer möglichen Produktivsetzung](#36-option-weiterentwicklung-des-mvp-zur-vorbereitung-einer-möglichen-produktivsetzung)
    - [3.7 Leistungen zu Vertragsende](#37-leistungen-zu-vertragsende)
  - [4 Übergreifende Anforderungen](#4-übergreifende-anforderungen)
    - [4.1 Prinzipien der Architekturgestaltung](#41-prinzipien-der-architekturgestaltung)
    - [4.2 Benutzbarkeit, Nutzererfahrung und Barrierefreiheit der Anwendung](#42-benutzbarkeit-nutzererfahrung-und-barrierefreiheit-der-anwendung)
      - [4.2.1 Benutzbarkeit und Nutzererfahrung](#421-benutzbarkeit-und-nutzererfahrung)
      - [4.2.2 Barrierefreiheit](#422-barrierefreiheit)
    - [4.3 Informationssicherheit](#43-informationssicherheit)
    - [4.4 Datenschutz](#44-datenschutz)
    - [4.5 Test- und Qualitätsmanagement](#45-test--und-qualitätsmanagement)
    - [4.6 Sicherstellung der Nachnutzbarkeit und Nachhaltigkeit](#46-sicherstellung-der-nachnutzbarkeit-und-nachhaltigkeit)
      - [Einhaltung von Best-Practices](#einhaltung-von-best-practices)
      - [Nutzbarkeit der entwickelten Lösung ohne Einsatz von proprietären Komponenten](#nutzbarkeit-der-entwickelten-lösung-ohne-einsatz-von-proprietären-komponenten)
      - [Beträge zu Quellprojekten (Upstream Contributions)](#beträge-zu-quellprojekten-upstream-contributions)
    - [4.7 Regelungen der Nutzungsrechte (freier, offener Softwareentwicklungsprozess)](#47-regelungen-der-nutzungsrechte-freier-offener-softwareentwicklungsprozess)
  - [5 Projektstruktur](#5-projektstruktur)
    - [5.1 Projektmanagement und Projektorganisation](#51-projektmanagement-und-projektorganisation)
      - [5.1.1 Aufgabe und Rolle der Auftraggeberin](#511-aufgabe-und-rolle-der-auftraggeberin)
      - [5.1.2 Agile Entwicklung](#512-agile-entwicklung)
      - [5.1.3 Verantwortlichkeiten im Projektteam](#513-verantwortlichkeiten-im-projektteam)
      - [5.1.4 Anforderungen an das einzusetzende Personal und Einordnung in die Preiskategorien](#514-anforderungen-an-das-einzusetzende-personal-und-einordnung-in-die-preiskategorien)
        - [Fachausrichtung: Consultant](#fachausrichtung-consultant)
        - [Fachausrichtung: Matrix Engineer Mobile](#fachausrichtung-matrix-engineer-mobile)
        - [Fachausrichtung: Matrix Engineer Web](#fachausrichtung-matrix-engineer-web)
        - [Fachausrichtung: Matrix Engineer Full-Stack](#fachausrichtung-matrix-engineer-full-stack)
      - [5.1.5 Kommunikation und Zusammenarbeit](#515-kommunikation-und-zusammenarbeit)
      - [5.1.6 Transparenz und Berichterstattung](#516-transparenz-und-berichterstattung)
      - [5.1.6 Flexibilität und Anpassungsfähigkeit](#516-flexibilität-und-anpassungsfähigkeit)
    - [5.2 Mitwirkungsleistungen der Auftraggeberin](#52-mitwirkungsleistungen-der-auftraggeberin)
      - [5.2.1 Keine Bereitstellung von Arbeitsplätzen und Infrastruktur](#521-keine-bereitstellung-von-arbeitsplätzen-und-infrastruktur)
      - [5.2.2 Bereitstellung von Software und Tools](#522-bereitstellung-von-software-und-tools)
      - [5.2.3 Kommunikation und Zusammenarbeit](#523-kommunikation-und-zusammenarbeit)
    - [5.3 Zusammenarbeit mit Dritten](#53-zusammenarbeit-mit-dritten)
    - [5.4 Zeitplan](#54-zeitplan)
  - [6 Rechtliche und vertragliche Rahmenbedingungen](#6-rechtliche-und-vertragliche-rahmenbedingungen)
    - [6.1 Vertragslaufzeit und -optionen](#61-vertragslaufzeit-und--optionen)
    - [6.2 Vergütungsmodalitäten](#62-vergütungsmodalitäten)
      - [6.2.1 Vergütung für Leistungen](#621-vergütung-für-leistungen)
      - [6.2.2 Vergütung für Reisekosten](#622-vergütung-für-reisekosten)

## 1 Über dieses Dokument

### 1.1 Inhalt des Dokuments

Die Auftraggeberin plant ein

Proof-of-Concept durch eine einfach zu nutzende Kommunikationslösung basierend auf dem Matrixstandard,
welche sich in die vorhandene IT-Landschaft der öffentlichen Verwaltung reibungsarm eingliedert.

Zur Durchführung des Vorhabens werden im Rahmen der nachstehenden Leistungsbeschreibung die
Dienstleistungen, die im Rahmen des Projektes durch die Auftragnehmerin zu erbringen sind, definiert.
Die Leistungsbeschreibung enthält detaillierte Informationen über den Umfang und die Qualität
der von der Auftragnehmerin als Dienstleisterin zu erbringenden Leistungen sowie technische,
organisatorische und zeitliche Rahmenbedingungen, die bei der Leistungserbringung von der
Auftragnehmerin als Teil der Leistungspflicht zu beachten sind.

Des Weiteren wird beschrieben, welche Mitwirkungsleistungen die Auftraggeberin erbringt.
Zudem werden die Ziele der Auftraggeberin und die daraus resultierenden Herausforderungen beschrieben,
um ein umfassendes Bild des Gesamtprojektes zu vermitteln, das die Auftragnehmerin bei der
Erbringung der Leistung zu berücksichtigen hat.

### 1.2 Abgrenzung des Dokuments

Diese Leistungsbeschreibung ist als Anlage Teil des Vertrages und regelt, welche konkreten Leistungen
von der Auftragnehmerin zu erbringen sind. Weitergehende Regelungen, insbesondere auch zur Vergütung,
finden sich im Vertrag und seinen Anlagen. Ein Lastenheft existiert nicht. Die zu berücksichtigenden
Anforderungen an die von der Auftragnehmerin zu erbringenden **Konzeptions-, Entwicklungs- und**
**Pilotierungsleistungen** sowie Anforderungen an den **Betrieb eines Testsystems** sind - neben dem
Vertrag und seinen Anlagen - in diese Leistungsbeschreibung integriert und werden im Folgenden näher
beschrieben.

## 2 Ausgangslage und Auftragsgegenstand

### 2.1 Die Auftraggeberin

Die FITKO ist zum 1. Januar 2020 als rechtsfähige Anstalt des öffentlichen Rechts (AöR) in gemeinsamer
Trägerschaft des Bundes und der Länder in Frankfurt am Main gegründet worden. Die FITKO bildet als
kleine und agile Organisation den operativen Unterbau des IT-Planungsrats und unterstützt diesen
organisatorisch und fachlich sowie bei der Wahrnehmung seiner Aufgaben.

Der IT-Planungsrat ist das politisch-strategische Gremium für die föderale Zusammenarbeit in der
Informationstechnik. Seit nunmehr 10 Jahren erhalten Bund, Länder und Kommunen durch Beschlüsse
und Entscheidungen des IT-Planungsrates verbindliche Grundlagen für ihre Aktivitäten in der Ebenen
übergreifenden Digitalisierung. Mitglieder des IT-Planungsrates sind die für Informationstechnik
zuständigen Vertreter der 16 Bundesländer (in der Regel Staatssekretärinnen und Staatssekretäre)
und der Beauftragte der Bundesregierung für Informationstechnik. Beratende Mitglieder sind die
drei kommunalen Spitzenverbände (der Deutsche Städtetag, der Deutsche Landkreistag und der Deutsche
Städte- und Gemeindebund) sowie der Bundesbeauftragte für den Datenschutz und die Informationsfreiheit.

Aufgaben des IT-Planungsrates im Überblick:

- Beschluss fachunabhängiger und fachübergreifender IT-Interoperabilitäts- und IT-Sicherheitsstandards
- Koordinierung und Unterstützung der Zusammenarbeit von Bund und Ländern in Fragen der Digitalisierung
  von Verwaltungsleistungen
- Steuerung der ihm zugewiesenen Projekte und Produkte
- Übernahme von Aufgaben für das Verbindungsnetz

Die FITKO soll die föderalen Aktivitäten zur Digitalisierung der Verwaltung bündeln, die
Koordinierung und das Controlling der Projekte und Produkte des IT-Planungsrats verantworten,
die Umsetzung des Onlinezugangsgesetzes (OZG) koordinieren sowie die föderale IT-Architektur
weiterentwickeln.

### 2.2 Ausgangslage

Das *Föderale IT-Architekturboard* unterstützt und berät den IT-Planungsrat in strategischen Fragen
bei der Entwicklung und Umsetzung von Maßnahmen der IT-Architektur. Zu den Aufgaben des Boards
gehören u. a. die Initiierung von Architekturvorhaben sowie die kurzfristige Bereitstellung dringend
benötigter Lösungen im Rahmen der Umsetzung des Onlinezugangsgesetzes. Das Architekturboard begleitet
zudem mit sog. "Projektpaten:innen" Infrastrukturprojekte mit Auswirkungen auf die föderale
IT-Architektur.

Die vorliegende Leistungsbeschreibung dient der Umsetzung des Vorhabens
"Pilotierung Messenger-Kommunikationskanal für die G2C-Kommunikation". Das Vorhaben ist mit Beschluss
vom 12. Juli 2023 ein Vorhaben des Föderalen IT-Architekturboards.

Mit [*FIT-Connect*](https://www.fitko.de/produktmanagement/fit-connect) haben FITKO und IT-Planungsrat
in den vergangenen Jahren bereits eine Infrastruktur zur maschinenlesbaren Einreichung von Anträgen
nach dem Onlinezugangsgesetz (OZG) bei der jeweils zuständigen Fachbehörde aufgebaut. Fachbehörden
in diesem Sinne sind beispielsweise Kommunen, Landes- oder Bundesbehörden, die mit der Erbringung
von Verwaltungsleistungen betraut sind. Die Nutzung von FIT-Connect senkt Aufwände für eine Anbindung
der Webportale, in denen Anträge gestellt werden können (sog. Onlinedienste) an die IT-Systeme der
zuständigen Fachbehörde (sog. Fachverfahren). Zentral dabei ist die Bereitstellung einer einheitlichen
und einfach zu integrierenden Programmierschnittstelle (REST-API), qualitativ hochwertiger technischer
Dokumentation sowie einer frei zugänglichen Testumgebung. FIT-Connect unterstützt in diesem Rahmen
den Prozess der initialen Antragstellung.

Nach Eingang eines Antrages bei der zuständigen Fachbehörde sind darüber hinaus je nach Fachlichkeit
Rückfragen, Statusänderungen zum Bearbeitungsvorgang und in der Regel die Zustellung von Bescheiden an
die antragstellende Person notwendig. Hierzu bedarf es einer einfach nutzbaren Basisinfrastruktur, die
eng auf die Bedürfnisse der antragstellenden Person und die Bedürfnisse der Fachbehörden abgestimmt ist.

### 2.3 Technischer IST-Zustand

Im Fokus steht die vollständige digitale Abwicklung des Prozesses zur Inanspruchnahme einer
Verwaltungsleistung gemäß den Vorgaben des Onlinezugangsgesetzes, beispielsweise bei der Beantragung
der Verwaltungsleistungen "Wohngeld" oder "BAföG". Betrachtet werden zudem im Sinne einer vollständig
digitalen Abwicklung neben der Antragstellung und der Antragsbearbeitung durch die zuständige
Fachbehörde auch eine digitale Bescheidzustellung (z.B. "Wohngeldbescheid" / "BAföG-Bescheid") an die
antragstellende Person.

Zur technischen Realisierung notwendiger Funktionalitäten kommen verschiedene zentral und dezentral
bereitgestellte IT-Systeme zum Einsatz. Eine Darstellung des vereinfachten IST-Zustandes findet sich
in Abbildung 1.

> ![Eine Architekturgrafik auf hoher Flughöhe, die Komponenten und Interaktionen zum Beantragen einer digitalen Verwaltungsleistung und anschließender Postfachkommunikation bei Antragsteller:in (Frontend BundID), Infrastruktur (Backend BundID) und zuständiger Fachbehörde (Fachverfahren) darstellt. Die Interaktion zur Antragsstellung geht durch die Komponenten Onlinedienst, über FIT-Connect zur FIT-Connect SDK im Fachverfahren. Dabei interagiert der Onlinedienst mit dem BundID IdP unter anderem mit SAML, wobei die Antragssteller:in die Authentifizierung über die AusweisApp durchführt. Das Postfach wird durch das ZBP-Backend in der BundID Backend Infrastruktur umgesetzt, Postfachnachrichten werden von Fachverfahren darüber an das Frontend BundID Postfach übertragen.](Leistungsbeschreibung/Abbildung%201:%20Status-Quo.png)
>
> Abbildung 1: Status-Quo: Antragstellung via FIT-Connect und Antragsfolgekommunikation mittels BundID.

Die Beantragung einer Verwaltungsleistung gemäß Onlinezugangsgesetz beginnt mit der Antragstellung in
einem zentral betriebenen Webportal (sog. Onlinedienst). Onlinedienste zur Antragstellung werden
typischerweise von mehreren Fachbehörden gemeinsam genutzt, häufig mit dem Zielbild denselben
Onlinedienst für alle Fachbehörden deutschlandweit zu nutzen. Sofern im Rahmen der Antragstellung
eine Authentifizierung nötig ist, kann diese über die Authentifizierungskomponente der BundID erfolgen.
Anschließend erfolgt eine Übermittlung der Antragsdaten vom Onlinedienst an ein IT-System der jeweils
zuständigen Fachbehörde (Fachverfahren). Zur Vereinfachung wird angenommen, dass hierzu FIT-Connect
genutzt wird.

Sofern eine digitale Kommunikation seitens der antragstellenden Person gewünscht ist, wird im Rahmen
der Übermittlung der Antragsdaten vom Onlinedienst auch ein Identifier des Postfachs der
antragstellenden Person an das Fachverfahren übermittelt. Dieser Identifier ermöglicht dem Fachverfahren
eine Adressierung des Postfachs der antragstellenden Person über die Schnittstellen der
Postfach-Komponente der BundID. Ein Abruf von Nachrichten im Postfach der BundID erfolgt anschließend
über die unter *id.bund.de* erreichbare Weboberfläche der BundID. Antragstellende Personen werden über
das Vorliegen neuer Nachrichten per E-Mail informiert.

Die Postfach-Funktionalität der BundID wird BundID-intern derzeit über das sog.
Zentrale Bürger-Postfach (ZBP) realisiert. Das ZBP stellt für Fachverfahren eine REST-basierte
Schnittstelle zum Versand von Nachrichten an registrierte BundID-Nutzende zur Verfügung.
Die Übermittlung von Nachrichten erfolgt derzeit ohne Anwendung einer Ende-zu-Ende-Verschlüsselung.
Auch ist derzeit kein App-basierter Zugang zum Postfach vorgesehen. Zusätzlich sollen zukünftig
Statusinformationen zum Fortschritt der Antragsbearbeitung auf demselben Weg an Antragstellende
übermittelt werden können.

Weitere technische Informationen zu FIT-Connect finden sich im
[Föderalen Entwicklungsportal](https://docs.fitko.de/fit-connect/).

Weitere technische Informationen zur BundID und zum Zentralen Bürger-Postfach finden sich im
[Self-Service-Portal der BundID](https://ssp.id.bund.de/ip?id=downloads).

### 2.4 Auftragsgegenstand und Laufzeit (Optionen)

#### 2.4.1 Konzeption, Entwicklung und Pilotierung (inkl. Qualitätssicherung), Vergütung nach Aufwand

Leistungsgegenstand ist die Konzeption, Entwicklung und Pilotierung (inkl. Qualitätssicherung)
einer einfach nutzbaren Lösung zur Kommunikation mit staatlichen Stellen, sowie die darauf bezogenen
IT-Beratung durch die Auftragnehmerin. Diese zu erbringenden Leistungen werden **nach Aufwand**
vergütet und in agiler Vorgehensweise in enger Abstimmung mit der Auftraggeberin erbracht. Nähere
Regelungen zur Vergütung finden sich in dieser Leistungsbeschreibung, im Vertrag und im Preisblatt
als Anlage zum Vertrag. Die Auftraggeberin priorisiert im Rahmen des Projektverlaufs die umzusetzenden
Teilleistungen. Bei den von der Auftragnehmerin zu erbringenden Leistungen handelt es sich um
Dienstleistungen; eine (auch verdeckte) Arbeitnehmerüberlassung ist nicht beabsichtigt und gewünscht.

Die Pflicht zur Erbringung der Leistung beginnt nach Zuschlagserteilung mit einer Laufzeit von
**12 Monaten**. Innerhalb dieser initialen Vertragslaufzeit entwickeln Auftraggeberin und
Auftragnehmerin gemeinsam ein funktionsfähiges Minimum Viable Product (MVP), das die technische
Machbarkeit, Funktionsfähigkeit, Skalierbarkeit, Leistungsfähigkeit und Belastbarkeit der entwickelten
Lösung demonstriert. Zusätzlich behält sich die Auftraggeberin eine einseitig in Textform ausübbare
**Option zur dreimaligen Verlängerung** der nach Aufwand zu vergütenden Leistungen um jeweils
**12 Monate** mit einer vertraglichen Maximallaufzeit von insgesamt **48 Monaten** vor
(siehe Abschnitt
[3.6](#36-option-weiterentwicklung-des-mvp-zur-vorbereitung-einer-möglichen-produktivsetzung)).
Die Verlängerungsoption wird spätestens zwei Monate vor Ablauf der 12-monatigen Laufzeit in Textform ausgeübt.

#### 2.4.2 Bereitstellung und Betrieb einer Testumgebung inkl. Support und Wartung, pauschale Vergütung pro Monat

Leistungsgegenstand ist darüber hinaus der Betrieb eines Testsystems. Das Testsystem dient der
Demonstration der Funktionsfähigkeit der entwickelten Lösung ggü. externen Stakeholdern und der
Durchführung von Last- und Performanztests.

Bereitstellung und Betrieb der Testumgebung inkl. Support und Wartung (insb. Aktualisierung) werden
**als Pauschale** pro Monat vergütet, wie im Preisblatt angegeben. Die Pflicht zur Erbringung der
Leistung beginnt nach Zuschlagserteilung mit einer Laufzeit von **12 Monaten**. Zusätzlich behält
sich die Auftraggeberin eine einseitig in Textform ausübbare **Option zur dreimaligen Verlängerung**
des Betriebs der Testumgebung um jeweils **12 Monate** mit einer vertraglichen **Maximallaufzeit**
von insgesamt **48 Monaten** vor. Die Verlängerungsoption wird spätestens zwei Monate vor Ablauf
der 12-monatigen Laufzeit in Textform ausgeübt.

Eine Integration der entwickelten Kommunikationsinfrastruktur in die *Produktivumgebungen* bestehender
dritter Systeme ist *nicht* Teil der zu erbringenden Leistungen. Die Absicht zur späteren produktiven
Nutzung soll jedoch bei der Konzeption, Entwicklung und Pilotierung des MVP grundsätzlich berücksichtigt
werden. Teil der Leistung ist die Integration der entwickelten Kommunikationsinfrastruktur in
*Testumgebungen* bestehender dritter Systeme. Der Betrieb einer Produktivumgebung ist *nicht* Teil der
zu erbringenden Leistungen.

#### 2.4.3 Übersicht über die zu erbringenden Teilleistungen

Die Teilleistungen, die die Auftragnehmerin zu erbringen hat, werden in [Kapitel 3](#3-leistungsumfang)
ausführlich beschrieben und im Folgenden grob skizziert:

- Konzeptions- und Beratungsleistungen, darunter:

  - Erstellung von Mockups / Click-Dummies und Evaluierung von UX-Konzepten
  - Erstellung einer Lösungsarchitektur für das in [Kapitel 2.5](#25-zielbild) Zielbild unter Berücksichtigung
    der in diesem Dokument beschriebenen und der im Projektverlauf durch die Auftraggeberin festzulegenden
    Rahmenbedingungen
  - Erstellung eines Migrationskonzeptes zur Unterstützung einer möglichen Migration vom technischen
    IST-Zustand zur Produktivsetzung des entwickelten Systems. Die Durchführung der Migration ist nicht
    Teil des Auftragsgegenstands.
  - Erstellung von notwendigen Erweiterungen der Matrix-Spezifikation (MSCs)
  - Beratungsleistungen bei der Pilotierung der Anbindung von Fachverfahren an die entwickelte
    Kommunikationsinfrastruktur
  - Unterstützung des Anforderungsmanagements

- Softwareentwicklungsleistungen, darunter:

  - Entwicklung einer App für einen mobilen Zugang zur geschaffenen Kommunikationsinfrastruktur
  - Anpassung einer bestehenden Weboberfläche für einen webbasierten Zugang zur geschaffenen
    Kommunikationsinfrastruktur
  - Entwicklung eines leichtgewichtigen Demo-Clients zur Darstellung der Aktivitäten von Fachbehörden
  - Erweiterung bestehender SDKs der Auftraggeberin zur Unterstützung von Dritten bei der Anbindung
    an die entwickelte Kommunikationsinfrastruktur
  - Erweiterungen an bestehenden SDKs und weiteren Komponenten aus dem Matrix-Ökosystem
  - Entwicklung oder konzeptionelle Berücksichtigung einer Self-Service-Funktionalität für Fachbehörden
    und Betreiber:innen von Onlinediensten

- Bereitstellung einer umfassenden Dokumentation für Hersteller von Fachverfahren
- Bereitstellung und Betrieb einer Testumgebung inkl. Support und Wartung
- Durchführung von Last- und Performanztests

Bei der Erbringung der in [Abschnitt 3](#3-leistungsumfang) beschriebenen Leistungen sind die in
[Abschnitt 4](#4-übergreifende-anforderungen) definierten Qualitätsanforderungen einzuhalten.

Die Auftragnehmerin hält im Rahmen ihrer Leistungserbringung die Leistungszusagen ein, die sie im
Rahmen der von ihr mit dem Angebot eingereichten Konzepte gemacht hat. Die dort gemachten Aussagen
und Angaben sind verbindlich. Im Falle von Abweichungen geht diese Leistungsbeschreibung vor.

### 2.5 Zielbild

Antragstellende erwarten einfach nutzbare Lösungen zur Kommunikation mit staatlichen Stellen auf
demselben Qualitätsniveau vergleichbarer Angebote der Privatwirtschaft.

Die aktuelle Lösungsvielfalt im Bereich der Postfach- und Kommunikationstechnologien in der
öffentlichen Verwaltung wird diesem Anspruch nicht gerecht und erschwert die flächendeckende Anbindung
von Fachverfahren an verschiedenste Postfach- und Kommunikationslösungen auf Seiten der Verwaltung.

> ![Ein oberflächliches Mockup einer möglichen mobilen BundID Postfach App. Zu sehen ist eine Übersicht über Verfahren im Postfach.](Leistungsbeschreibung/Abbildung%202:%20Kommunikation%20auf%20mobilem%20Endgeraet.png)
>
> Abbildung 2: Kommunikation auf mobilem Endgerät

Ausgehend vom beschriebenen IST-Zustand soll eine Lösungsarchitektur entwickelt und prototypisch
umgesetzt werden, die im Erfolgsfall die technische Machbarkeit des Vorhabens demonstriert. Hierzu
soll eine Kommunikationskomponente auf Basis des Matrix-Protokolls entwickelt und deren
Integrationsfähigkeit in bestehende IT-Systeme der Verwaltung (Fachverfahren) demonstriert werden.
Die im Rahmen des Auftrags entwickelten Lösungen sollen im Erfolgsfall perspektivisch in einen
produktiven Betrieb überführt werden. Ein produktiver Betrieb der entwickelten Lösung ist nicht Teil
des vorliegenden Auftragsgegenstands.

Ziel des mit dieser Leistungsbeschreibung definierten Vorhabens ist daher die Erprobung einer
Matrix-basierten Kommunikationsinfrastruktur für die bidirektionale, vorgangsbezogene
Antrags-Folgekommunikation zwischen Fachbehörden mit antragstellenden Personen.

Besonderer Fokus des Vorhabens liegt auf der Realisierung von hohen Sicherheitsanforderungen (u.a.
durch die Umsetzung einer Ende-zu-Ende-Verschlüsselung nach dem
[Stand der Technik](https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Publikationen/TechnischeRichtlinien/TR03176/BSI-TR-03176.pdf?__blob=publicationFile&v=3))
und der Erprobung neuer Ansätze zur Erreichung einer herausragenden Nutzendenerfahrung (User Experience).

Eine Lösung auf höchstem technischem Niveau unter Berücksichtigung von
[Zero-Trust](https://www.bsi.bund.de/EN/Themen/Unternehmen-und-Organisationen/Informationen-und-Empfehlungen/Zero-Trust/zero-trust_node.html)
sowie
[Security-by-Design-](https://www.cisa.gov/sites/default/files/2023-10/SecureByDesign_1025_508c.pdf)
und
[Privacy-by-Design-Prinzipien](https://www.edpb.europa.eu/system/files/2021-04/edpb_guidelines_201904_dataprotection_by_design_and_by_default_v2.0_de.pdf)
muss auf Basis international erprobter Standards erfolgen. Die Nutzung international etablierter
Standards und bestehender Referenzimplementierungen senken die Umsetzungsaufwände und das
Projektrisiko zur Einführung einer solchen Lösung erheblich. Die zu entwickelnde
Kommunikationsinfrastruktur muss daher basierend auf dem offenen Standard [Matrix](https://matrix.org/)
umgesetzt werden. Das Matrix-Protokoll kommt beispielsweise auch für die Chatlösung
[Tchap](https://www.tchap.gouv.fr/) der französischen Regierung, im ukrainischen Militär,
im öffentlichen Gesundheitsdienst ([TI-Messenger](https://www.gematik.de/anwendungen/ti-messenger/) der
[gematik](https://www.gematik.de/)), im [openDesk-Projekt](https://opendesk.eu/) sowie im
[BwMessenger](https://messenger.bwi.de/) der Bundeswehr zum Einsatz. Notwendige Features des
Matrix-Protokolls, wie etwa die grundlegende Chat-Funktionalität, eine Multi-Device-Funktionalität
und eine Ende-zu-Ende-Verschlüsselung sind in der Praxis erprobt und inzwischen ausgereift.
Entsprechend müssen diese nicht aufwändig neu entwickelt werden. Die IT-Sicherheit der Lösung wurde
bereits in diversen Vorhaben analysiert und bewertet. Das zur Ende-zu-Ende-Verschlüsselung im
Matrix-Protokoll eingesetzte Olm-Protokoll basiert auf den Konzepten des renommierten
[Signal-Protokolls](https://signal.org/docs/). Das Matrix-Protokoll wird in einem offenen
Standardisierungsprozess herstellerunabhängig weiterentwickelt.

> ![Die Architekturgrafik auf hoher Flughöhe aus Abbildung 1. Die SDK mit FIT-Connect Anbindung auf Seite des Fachverfahrens wurde durch eine Matrix Integration erweitert. Zur Postfachkomponente besteht nun ein Pfeil zu einem Matrix Server in der Postfach Komponente der BundID Backend Infrastruktur. Über Matrix interagiert nun sowohl das Frontend BundID Postfach mit dem Backend, als auch eine neue App zur Antragsfolgekommunikation. Das ZBP-Backend besteht neben dem Matrix Server. Bei der Antragsstellung ist die übermittlung des Postfach-Handles an den Onlinedienst im Matrix Context durch den Text "@alice:id.bund.de" über dem Pfeil Antragsstellung angedeutet. Eine Sprechblase bei der Fachbehörde deutet die Möglichkeit zur Bidirektionalität an.](Leistungsbeschreibung/Abbildung%203:%20Zielarchitektur.png)
>
> Abbildung 3: Zielarchitektur: Matrix-basierte Antragsfolgekommunikation mittels BundID. Nach der
> Antragstellung über einen Onlinedienst eröffnet das zuständige Fachverfahren einen Kommunikationskanal
> zurück zur Antragsteller:in.

Der in Abbildung 3 dargestellte High-Level-Architekturentwurf zeigt eine vereinfachte Darstellung
der Zielarchitektur. Im Zielbild unterstützt ein Matrix-Server die Kommunikation zwischen
Fachbehörde und antragstellender Person.

> [!note] Fußnote
> Im Rahmen der Entwicklung des MVP ist zunächst die Umsetzung einer Architektur auf Basis eines
> zentral betriebenen Matrix-Servers vorgesehen. Im Projektverlauf legt die Auftraggeberin fest, ob
> darüber hinaus auch Architekturvarianten konzipiert werden sollen, die den Betrieb mehrere
> Matrix-Server vorsehen.

Dieser Matrix-Server ermöglicht der Bereitstellung eines vertrauenswürdigen, d.h.
Ende-zu-Ende-verschlüsselten Kanals inkl. Ende-zu-Ende-Authentisierung zwischen antragstellender
Person und Fachbehörde. Kommunikationsinhalte sollen zur Verbesserung der Usability strukturiert
nach Antragsverfahren organisiert werden.

Die zu entwickelnde Kommunikationsinfrastruktur ermöglicht Privatpersonen (Antragsteller:innen)
einen Zugriff auf Kommunikationsinhalte über die bestehende Weboberfläche der BundID, die im Rahmen
dieses Vorhabens angepasst werden soll. Zudem soll der Zugriff auf die Kommunikationsinhalte auch über
eine neu zu entwickelnde App ermöglicht werden. Dieser zusätzliche App-basierte Kommunikationskanal
ergänzt den bestehenden, Web-basierten Zugang. Dies erhöht den Komfort und steigert die Akzeptanz bei
Nutzenden.

Fachbehörden nutzen typischerweise spezialisierte IT-Systeme zur Vorgangsbearbeitung (Bearbeitung von
digital eingereichten Anträgen nach dem Onlinezugangsgesetz), sog. Fachverfahren. Zur Förderung
voll-integrierter Vorgangsbearbeitungsprozesse auf Seiten der Fachbehörden (Ende-zu-Ende-Digitalisierung)
soll eine Kommunikation mit antragstellenden Personen direkt aus Fachverfahren heraus ermöglicht
werden. Viele Hersteller von in Fachbehörden eingesetzten Fachverfahren verwenden bereits das
[Software-Entwicklungs-Kit (SDK) des Produktes FIT-Connect](https://docs.fitko.de/fit-connect/docs/sdks/overview/).
Eine Anbindung der IT-Systeme von Fachbehörden (Fachverfahren) soll daher über eine Integration
der notwendigen Kommunikations-Schnittstellen in das bestehende SDK des Produktes FIT-Connect
ermöglicht werden. Das SDK soll Herstellern von Fachverfahren eine möglichst einfache Anbindung
ihrer Fachverfahren an die entwickelte Kommunikationsinfrastruktur ermöglichen, sodass diese ihre
Software in nur wenigen Stunden Integrationsaufwand um die hier dargestellte Funktionalität erweitern
können und anschließend in der Lage sind, sicher und bidirektional direkt mit Antragsteller:innen zu
kommunizieren.

Im Erfolgsfall könnte die entwickelte Lösung die Funktion des Postfaches gemäß
[§ 2 Absatz 7 OZG](https://www.gesetze-im-internet.de/ozg/__2.html) erfüllen. Das Postfach ist
Bestandteil eines Nutzerkontos gemäß [§ 2 Absatz 5 OZG](https://www.gesetze-im-internet.de/ozg/__2.html).
Die Funktionalität des Nutzerkontos für natürliche Personen wird derzeit durch die
[BundID](https://id.bund.de/) realisiert. Besonderer Fokus des Vorhabens ist daher auch, eine
Integrationsfähigkeit der Realisierten Lösung in die BundID darzustellen. Hierzu soll auch ein
Migrationskonzept erarbeitet werden, welches eine reibungslose Migration vom technischen IST-Zustand
hin zur entwickelten Lösung darstellt. Die Durchführung der Migration ist hingegen nicht Teil der
zu erbringenden Leistungen.

#### 2.5.1 Strukturierte Vorgangskommunikation

Zur weiteren Förderung voll-integrierter und (teil-)automatisierter Vorgangsbearbeitungsprozesse soll
für Fachbehörden die Möglichkeit geschaffen werden, Rückfragen an antragstellende Personen auch
(teil-)strukturiert zu stellen. In der zu entwickelnden Kommunikationsinfrastruktur sollen daher
auch Funktionalitäten zur strukturierten (auf Verwaltungsseite maschinenlesbaren) Kommunikation,
wie etwa eine Terminauswahl, frei definierbare Multiple-Choice-Abfragen oder Dialogsysteme abgebildet
werden (vgl. Abbildung 4).

> ![Ein oberflächliches Mockup der möglichen mobilen BundID Postfach App aus Abbildung 2. Die Oberfläche zeigt 3 Auswahlmöglichkeiten, jeweils mit Datum und Uhrzeit, um den möglichen Anwendungsfall einer Terminauswahl zu illustrieren.](Leistungsbeschreibung/Abbildung%204:%20Terminvereinbarung.png)
>
> Abbildung 4: Über die angedachte Infrastruktur sind auch weitere Szenarien wie etwa eine
> Terminvereinbarung möglich.

Bereits jetzt unterstützt der Matrix-Standard die Erstellung von strukturierten Umfragen (Chat Polls,
[MSC3381](https://github.com/matrix-org/matrix-spec-proposals/pull/3381)). Um komplexere,
maschinenlesbare Rückfragen direkt im Web-Frontend oder in der App beantworten zu können, sollen
Fachbehörden in die Lage versetzt werden, einfache Formulare mit Hilfe einer Beschreibungssprache (z.B.
basierend auf JSON Schema) zu definieren, die dann im Web-Frontend/in der App angezeigt werden.
Dies ermöglicht beispielsweise das Nachliefern einzelner oder mehrerer maschinenlesbarer
Datenfelder (z.B. Datumswerte, Zahlenwerte oder Boolesche Werte) oder Dateien (via
File-Input-Formularfeld). Der genaue Umfang und die hierzu zu realisierenden Funktionalitäten werden
im Rahmen der Konzeptionsarbeit im Projektverlauf festgelegt.

Es ist angedacht, dass die Konzeption zur Übermittlung solcher frei definierten Formulare inkl.
maschinenlesbarer Rückantwort in Form eines oder mehrerer Matrix Spec Proposals (MSC) erfolgt und in
den Standardisierungsprozess des Matrix-Standards eingebracht wird, sofern die Aussichten zur
Nichtannahme des Proposals und die Eignung für eine anwendungsspezifische Spezifikation nicht überwiegen.

Antragstellende Personen sollen zudem die Möglichkeit haben, sich über den Web- und App-basierten
Kommunikationskanal über den Fortschritt einzelner Antragsverfahren zu informieren. Hierzu sollen
Fachbehörden in die Lage versetzt werden, den jeweiligen Status eines Antragsverfahrens zu übermitteln.
Der jeweilige Fortschritt einzelner Antragsverfahren soll Antragstellenden anschließend übersichtlich
dargestellt werden.

#### 2.5.2 Sicherstellung der Verfügbarkeit des kryptographischen Schlüsselmaterials

Die Sicherstellung der Verfügbarkeit des kryptographischen Schlüsselmaterials ist essenziell,
insbesondere für die Ende-zu-Ende-Verschlüsselung, die durch die Matrix-Spezifikation bereits
serverseitige Backups unterstützt. Im Rahmen des Projekts sollen verschiedene bestehende Ansätze für
Backup und Recovery bedarfsgerecht kombiniert und gegebenenfalls durch neue Methoden erweitert werden,
um sicherzustellen, dass die Inhalte des Matrix-Accounts auch im Falle eines Passwortverlustes verfügbar
bleiben. Ein zentraler Aspekt dabei ist die Sicherheit des privaten Schlüsselmaterials: Die Betreiberin
der Infrastruktur sowie Dritte dürfen zu keinem Zeitpunkt die Möglichkeit haben, Zugang zum privaten
Schlüsselmaterial der Nutzenden zu erlangen.

Ziel ist es, eine Lösung zu entwickeln, die Security und Usability optimal vereint, um den Nutzenden
eine intuitive und zugleich sichere Möglichkeit zur Verwaltung ihres Schlüsselmaterials zu bieten.
Protokolle und Verfahren werden so gestaltet, dass eine einfache Wiederherstellung möglich ist,
ohne die Sicherheit der verschlüsselten Kommunikation zu gefährden.

Um sicherzustellen, dass Nutzende sich der Bedeutung und Notwendigkeit von Backup-Optionen bewusst sind,
wird eine benutzendenfreundliche Hilfestellung entwickelt, die die verschiedenen Sicherungsmöglichkeiten
und ihre Anwendung erklärt. Diese Anleitung soll Nutzende beim Einrichten und Aktualisieren ihrer
Konten aktiv auf die Wichtigkeit der Sicherung ihres Schlüsselmaterials aufmerksam machen. Weiterhin
sollen klare Hinweise zur Sicherheit der jeweiligen Methode gegeben werden, sodass Nutzende die für sie
geeignetste Option bewusst auswählen können.

Es ist angedacht, dass die Konzeption der hierzu erarbeiteten Lösung in Form eines oder mehrerer Matrix
Spec Proposals (MSC) erfolgt und in den Standardisierungsprozess des Matrix-Standards eingebracht wird,
sofern die Aussichten zur Nichtannahme des Proposals und die Eignung für eine anwendungsspezifische
Spezifikation nicht überwiegen.

#### 2.5.3 Schutz vor missbräuchlicher Nutzung der bereitgestellten Infrastruktur

Um die missbräuchliche Nutzung des Matrix-Servers, wie beispielsweise den Versand von Spam oder
Scam-Nachrichten an andere Nutzende, effektiv zu verhindern, sollen technische Maßnahmen zur
Limitierung des Umfangs gesendeter Nachrichten (Rate-Limiting, Quota) sowie eine Einschränkung der
Adressierbarkeit entwickelt und getestet werden. Diese Maßnahmen erzwingen Limits für die Anzahl und
Größe gesendeter Nachrichten und Anhänge und stellen sicher, dass nur autorisierte Kommunikationsinhalte
und -verbindungen zwischen Nutzenden versendet, beziehungsweise aufgebaut werden können.
Eine Kommunikation zwischen Privatpersonen untereinander ist beispielsweise derzeit explizit nicht
vorgesehen.

Eine Festlegung des konkreten quantitativen Umfangs wird im Projektverlauf erarbeitet. Getroffene
Einschränkungen, Richtlinien und Filtermechanismen sollen per Konfiguration global durch die
zukünftige Betreiberin oder vorgangsspezifisch durch die jeweilige Fachbehörde anpassbar sein.

Es ist angedacht, dass die Konzeption der hierzu erarbeiteten Lösung in Form eines oder mehrerer
Matrix Spec Proposals (MSC) erfolgt und in den Standardisierungsprozess des Matrix-Standards
eingebracht wird, sofern die Aussichten zur Nichtannahme des Proposals und die Eignung für eine
anwendungsspezifische Spezifikation nicht überwiegen.

#### 2.5.4 Vermeidung von globalen Identifikatoren zur Unterstützung des Schutzziels der Unlinkability

Die Lösung soll, so weit technisch realisierbar, den Einsatz globaler personenbezogener IDs vermeiden,
um die Privatsphäre der Nutzenden zu stärken. Stattdessen sollen kontextspezifische Identifikatoren
oder pseudonyme IDs eingesetzt werden, die nur innerhalb bestimmter Interaktionen oder Sessions gültig
sind. Diese technische Maßnahme dient dem Zweck, eine mögliche Profilbildung an zentraler Stelle und
eine Nachverfolgbarkeit von Nutzenden über verschiedene Systeme hinweg auszuschließen.

Um die Schaffung eines globalen personenbezogenen Identifiers für eine natürliche Person zu vermeiden,
soll ein und dieselbe Person von verschiedenen Fachbehörden über fachbehörden-spezifische
(Matrix-)IDs kontaktiert werden können. Jede Fachbehörde erfährt dazu eine andere (Matrix-)ID derselben
Person, über die Nachrichten an die zu adressierende Person verschickt werden können. Hierbei muss
sichergestellt sein, dass die jeweilige Fachbehörde zu keinem Zeitpunkt in der Lage ist,
Rückschlüsse auf eine global eindeutige ID der betroffenen Person zu erfahren.

Es ist angedacht, dass die Konzeption der hierzu erarbeiteten Lösung in Form eines oder mehrerer
Matrix Spec Proposals (MSC) erfolgt und in den Standardisierungsprozess des Matrix-Standards
eingebracht wird, sofern die Aussichten zur Nichtannahme des Proposals und die Eignung für eine
anwendungsspezifische Spezifikation nicht überwiegen.

#### 2.5.5 Vermeidung von Profilbildung durch Reduktion der anfallenden Metadaten

Der sich aus dem Zielbild ergebende zentralisierte Betrieb einer Kommunikationsinfrastruktur mit
flächendeckendem Einsatzzweck birgt das Risiko einer Anhäufung von Metadaten an zentraler Stelle.
Im Sinne des Privacy-by-Design-Paradigmas sollen Ansätze entwickelt werden, um die Menge und
Qualität der anfallenden Metadaten bestmöglich zu reduzieren. Ziel ist es im Besonderen qualitativ
hochwertige Metadaten von Antragsstellenden, die beispielsweise diskriminierend verwendet werden
könnten, zu verschleiern oder zu vermeiden.

Wenn erforderlich und in Absprache mit der Auftraggeberin soll in diesem Rahmen auch ein Entwurf
einer dezentralen Betriebsstruktur entwickelt werden um eine Verbesserung der Metadatenhaltung zu
zeigen. Als Grundlage könnte beispielsweise das föderal-staatliche Organisationsprinzip in
Deutschland genutzt werden.

Es ist angedacht, dass die Konzeption der erarbeiteten Ansätze in Form eines oder mehrerer Matrix Spec
Proposals (MSC) erfolgt und in den Standardisierungsprozess des Matrix-Standards eingebracht wird, sofern
die Aussichten zur Nichtannahme des Proposals und die Eignung für eine anwendungsspezifische Spezifikation
nicht überwiegen.

#### 2.5.6 Authentifizierung von Nutzenden mittels eID und BundID

Die *Authentifizierung* der Nutzenden soll über die eID und BundID ermöglicht werden, um eine hohe
Sicherheit und Identitätsprüfung zu gewährleisten. Diese Authentifizierung stellt sicher, dass nur
verifizierte Nutzende Zugriff auf die geschützte Kommunikationsinfrastruktur haben und die Integrität des
Systems gewahrt bleibt. Weiterhin ist zu gewährleisten, dass die Integration dieser Authentifizierungsmethoden
die Interoperabilität mit bestehenden Systemen und eine nahtlose Nutzendenerfahrung unterstützt.

Zur Realisierung des Ziels einer optimalen nutzendenzentrierten Anwendung soll die grundlegende
Basisfunktionalität zur Authentifizierung mittels Personalausweis (eID) direkt in der zu entwickelnden App
bereitgestellt werden. Hierzu ist eine Integration des [AusweisApp-SDK](https://www.ausweisapp.bund.de/sdk/)
in die zu entwickelnde App vorgesehen.

#### 2.5.7 Authentisierung von Kommunikationspartner:innen mittels X.509-Zertifikat

Aus Sicht von Privatpersonen ist es wichtig, die Identität ihres Kommunikationspartners (d.h. der
zuständigen Fachbehörde) und die *Authentizität* des etablierten Kommunikationskanals zweifelsfrei
feststellen zu können. Dies ist insbesondere bei vertraulicher Kommunikation wichtig, um einen Datenabfluss
in Richtung unberechtigter Dritter auszuschließen.

Gemäß aktueller Matrix-Spezifikation erfolgt eine kryptographische Authentisierung von
Kommunikationspartner:innen auf Ebene des kryptographischen Kommunikationskanals (sog. Device Verification)
mittels Vergleich eines Einmal-Codes (Short Authentication String, SAS) in Form von Dezimalzahlen,
Emoji-Darstellung oder QR-Code über einen dritten, sicheren Kommunikationskanal. Die Art des Abgleichs
eignet sich gut für natürliche Personen, ist jedoch für die Kommunikation mit einer Fachbehörde untauglich.

Stattdessen ist angedacht, eine Device Verification auf Basis eines X.509-Zertifikats der Fachbehörde zu
realisieren. Eine kryptographische Authentisierung des Kommunikationskanals anhand des X.509-Zertifikats der
Fachbehörde soll dabei im Hintergrund ohne Nutzendeninteraktion erfolgen. Nutzende können anschließend im
User Interface die im X.509-Zertifikat hinterlegten Identitätsdaten der Fachbehörde (Offizieller Name, Adresse,
etc.) einsehen. Wichtig dabei ist eine Bindung der kryptographischen Identität des X.509-Zertifikats an
das Schlüsselmaterial der Ende-zu-Ende-Verschlüsselung und die bidirektionale Verifikation analog zum
Konzept des einmaligen Scans eines QR-Codes entsprechend dem
[QR-Code-Verfahren](https://spec.matrix.org/v1.12/client-server-api/#qr-codes) aus der Matrix-Spezifikation.

Es ist angedacht, dass die Konzeption der automatisierten Verifizierung von Identitäten mittels
X.509-Zertifikaten in Form eines oder mehrerer Matrix Spec Proposals (MSC) erfolgt und in den
Standardisierungsprozess des Matrix-Standards eingebracht wird, sofern die Aussichten zur Nichtannahme
des Proposals und die Eignung für eine anwendungsspezifische Spezifikation nicht überwiegen.

#### 2.5.8 Authentisierung von Kommunikationspartner:innen mittels eID/BundID

Auch Fachbehörden sollen in die Lage versetzt werden, die Identität der mit ihr kommunizierenden Person
und die *Authentizität* des etablierten Kommunikationskanals sicherzustellen. Hierzu ist eine automatisierte
Verifizierung der Identität der kommunizierenden Privatperson mittels eID-/BundID-Login zu ermöglichen.
Dabei ist das jeweils nötige eIDAS Level of Assurance (LoA) zu beachten. Ob eine kryptographische Bindung
an die eID-Authentifizierung technisch möglich und realisierbar ist oder ob eine Bindung unter Nutzung
einer Trusted Third Party (BundID-Server) erfolgt, ist im Projektverlauf zu entscheiden.

#### 2.5.9 Mögliche weitere Fortschreibungsbedarfe der Matrix-Spezifikation

Werden im Projektverlauf weitere projektspezifische Weiterentwicklungsbedarfe des Matrix-Standards mit
Mehrwert über den Projektscope hinaus erkannt, so soll deren Konzeptionierung in Abstimmung mit der
Auftraggeberin ebenfalls in Form eines oder mehrerer Matrix Spec Proposals (MSC) erfolgen und in den
Standardisierungsprozess des Matrix-Standards eingebracht werden.

## 3 Leistungsumfang

In diesem Abschnitt wird der Umfang der im Rahmen dieses Auftrags von der Auftragnehmerin zu erbringenden
Leistung beschrieben. Reihenfolge der Erbringung von Teilleistungen und deren technische Ausgestaltung
werden im Projektverlauf von der Auftraggeberin festgelegt.

Umfang und Inhalte des in [Abschnitt 2.5](#25-zielbild) dargestellten Zielbildes und der in den folgenden
Abschnitten dargestellten Komponenten und Funktionalitäten unterliegen einem kontinuierlichen Review-Prozess
und können sich während der Vertragslaufzeit aufgrund von Impulsen der Auftraggeberin oder der Auftragnehmerin
verändern. Umfang und Inhalt des Zielbildes, der Komponenten und Funktionalitäten werden daher von der
Auftraggeberin im Laufe der Vertragslaufzeit fortlaufend evaluiert und ggf. angepasst. Entscheidungsbefugt
über den Umfang und den Inhalt des Zielbildes ist aber allein die Auftraggeberin.

Die Leistungserbringung erfolgt in von der Auftraggeberin separat abzustimmenden und von der Auftraggeberin
festzulegenden Arbeitspaketen. Der Zuschnitt und Inhalt der jeweiligen Arbeitspakete wird allein durch
die Auftraggeberin festgelegt. Vor Festlegung eines Arbeitspaketes, welches durch die Auftragnehmerin
umgesetzt werden soll, erfolgt eine dem Arbeitspaket angemessene Planungs- und Konzeptionsphase, welche
durch die Auftragnehmerin begleitet werden muss ("Planning & Refinement"). Von der Auftragnehmerin
erbrachte Planungs- und Konzeptionsleistungen können nach Aufwand gem. dem im Preisblatt beschriebenen
Preiskategorien abgerechnet werden. Es besteht jedoch kein Anspruch auf Realisierung eines bestimmten
Umfangs der Leistungserbringung. Einzige Ausnahme stellt hierbei die initiale Bereitstellung und Betrieb
einer Testumgebung inkl. Support und Wartung dar (vgl.
[Kapitel 3.4](#34-bereitstellung-und-betrieb-einer-testumgebung-inkl-support-und-wartung)).

### 3.1 Konzeptions- und Beratungsleistungen

#### 3.1.1 Erstellung von Mockups / Click-Dummys und Evaluierung von UX-Konzepten

In einem ersten Schritt sollen leichtgewichtige Mockups und Click-Dummys erstellt werden, um
frühzeitig Nutzenden-Feedback zu sammeln und die grundlegende Bedienbarkeit sowie das Designkonzept der
Software zu testen. Die Mockups / Click-Dummys sollen dabei die wesentlichen Funktionen und den geplanten
Ablauf des Systems abbilden, um eine realitätsnahe Nutzungserfahrung zu ermöglichen. Die erstellten
Prototypen sollen nicht nur optische, sondern auch grundlegende interaktive Elemente umfassen, um den
Nutzenden ein besseres Verständnis der Bedienung und Benutzendenführung zu vermitteln.

Anschließend sollen die Mockups / Click-Dummys in iterativen Testphasen überprüft und auf Basis des
Feedbacks schrittweise optimiert werden. Dabei sind unterschiedliche Nutzungsgruppen einzubeziehen,
um ein breites Spektrum an Anwendungsfällen abzudecken und frühzeitig Schwachstellen in der Bedienbarkeit,
Nutzerfreundlichkeit und Barrierefreiheit zu identifizieren. Umfang und Tiefe der durchzuführenden Tests
werden im Projektverlauf im Einvernehmen mit der Auftraggeberin festgelegt. Zusätzlich ist ein Verfahren
zur strukturierten Dokumentation des Feedbacks zu entwickeln, um die Erkenntnisse in den weiteren
Entwicklungsprozess einfließen zu lassen.

Die endgültigen Mockups / Click-Dummys dienen dann als Grundlage für das Feinkonzept und die detaillierte
Spezifikation der Benutzeroberfläche und sollen sicherstellen, dass die Anforderungen der Nutzenden optimal
erfüllt werden.

Diese wurden z.B. auch hier eingesetzt:
<https://www.zugang-zum-recht-projekte.de/news/zivilgerichtliches-online-verfahren-wie-wir-unser-produkt-auf-barrierefreiheit-testen>

#### 3.1.2 Erstellung einer Lösungsarchitektur

Die Auftragnehmerin muss auf Aufforderung eine Lösungsarchitektur zur Realisierung des in
[Abschnitt 2.5](#25-zielbild) beschriebenen Zielbildes erarbeiten.

Die Auftragnehmerin muss die Dokumentation der Architektur umfassend und nachvollziehbar in englischer
Sprache erstellen. Dabei sind die verwendeten Designprinzipien, Standards und Best Practices (bspw.
[SOLID-Prinzipien](https://de.wikipedia.org/wiki/Prinzipien_objektorientierten_Designs#SOLID-Prinzipien))
klar zu beschreiben, um sicherzustellen, dass bewährte Vorgehensweisen eingehalten werden und die Architektur
langfristig wartbar und skalierbar bleibt. Es ist das [Arc42-Template](https://arc42.org/) für die
Architekturdokumentation einzusetzen, um eine strukturierte und standardisierte Darstellung sicherzustellen.

Bei der Beschreibung der Architektur sind anerkannter Beschreibungssprachen wie Archimate, C4 oder BPMN
bedarfsgerecht einzusetzen. Die Lösungsarchitektur soll für unterschiedliche Zielgruppen verständlich
dargestellt werden, darunter Entwickler:innen, Architekt:innen, Tester:innen, Produktmanager:innen und die
interessierte Öffentlichkeit. Die Architektur muss in klare, nachvollziehbare Module oder Komponenten
unterteilt werden, wobei jede Komponente klar definierte Verantwortlichkeiten aufweist.

Es ist eine umfassende Beschreibung in Bezugnahme auf die Qualitätsziele der Architektur erforderlich.
Diese Qualitätsziele umfassen Benutzbarkeit, Skalierbarkeit, Sicherheit, Wartbarkeit, Leistungseffizienz,
Funktionale Eignung, Kompatibilität, Übertragbarkeit, Gefahrlosigkeit und Zuverlässigkeit
([ISO/IEC 25010:2023](https://www.iso.org/standard/78176.html)).

Die Auftragnehmerin muss darlegen, wie diese Anforderungen durch den Einsatz bestimmter Designmuster,
Technologien und Infrastrukturentscheidungen erfüllt werden. Die verwendeten Technologien und Werkzeuge
sind ebenfalls zu beschreiben, wobei die Auftragnehmerin die Auswahl nachvollziehbar begründen soll.
Des Weiteren sind sowohl die internen als auch die externen Abhängigkeiten der Softwarekomponenten darzustellen,
einschließlich technischer Abhängigkeiten (z. B. externe APIs, Bibliotheken) und organisatorischer
Abhängigkeiten (z. B. Abhängigkeiten zwischen Entwicklungsteams).

Sicherheitsanforderungen und -maßnahmen müssen anhand einer Bedrohungsanalyse detailliert beschrieben
werden (Threat Modeling). Dazu zählen der Schutz sensibler Daten inkl. Metadaten und die Absicherung der
Kommunikation zwischen den Systemkomponenten.

Die Architektur muss Mechanismen zur Wartung und Erweiterung des Systems beinhalten. Dies umfasst
Mechanismen zur Fehlerbehebung, zur Überwachung von Logs und zur Integration neuer Funktionalitäten.
Schließlich ist aufzuzeigen, wie sich die Architektur an veränderte Anforderungen oder neue Technologien
anpassen lässt, um die langfristige Nutzbarkeit zu gewährleisten.

Die Lösungsarchitektur wird auf der Plattform openCode entwickelt und veröffentlicht. Dies soll internen
und externen Stakeholdern ermöglichen, frühzeitig Feedback zu geben. Die Auftragnehmerin fördert diesen
Austausch durch Etablierung einer transparenten und offenen Kultur, u.a. mit Hilfe von Leitlinien, wie
Personen bestmöglich zum Projekt beitragen oder Feedback geben können (Contributing Guidelines).

Die Architekturdokumentation muss regelmäßig aktualisiert werden, um mit der aktuellen Implementierung
übereinzustimmen. Änderungen in der Architektur sind kontinuierlich zu dokumentieren und zu versionieren,
um einen klaren Verlauf der Architekturentscheidungen in Form von
[Architecture Decision Records](https://adr.github.io/) nachvollziehbar zu machen. Zur Erstellung der
Dokumentation und Visualisierung müssen geeignete offen lizenzierte Tools (Freie Software) und offene
Dateiformate verwendet werden, um die Konsistenz und Zugänglichkeit der Architekturinformationen zu verbessern.
Geeignete Tools sind unter anderem Draw.io, PlantUML oder Archi.

Die Lösungsarchitektur wird auf Englisch verfasst.

#### 3.1.3 Erstellung eines Migrationskonzepts

In der bestehenden produktiven Infrastruktur haben Fachverfahren bereits Kommunikationsinhalte über das
BundID-Postfach an die Nutzenden übermittelt. Bei der Migration auf die neu entwickelte Lösung dürfen diese
Inhalte nicht verloren gehen. Das Migrationskonzept soll sicherstellen, dass ein nahtloser Übergang vom
aktuellen technischen Zustand (IST) zur neuen, pilotierten Ziel-Architektur möglich ist und alle bestehenden
Kommunikationsinhalte transparent und kontinuierlich verfügbar bleiben. Das Migrationskonzept soll als Basis
für eine spätere produktive Umsetzung dienen. Die Durchführung der Migration ist nicht Teil des
Auftragsgegenstands.

Das Migrationskonzept wird auf Englisch verfasst.

Im Konzept müssen mindestens folgende Aspekte beleuchtet werden:

##### Realisierung des nahtlosen Zugriffs für Endnutzende auf bestehende und neue Kommunikationsinhalte

Die Migration muss für die Endnutzenden "unsichtbar" ablaufen, sodass sowohl bestehende als auch neue
Kommunikationsinhalte nahtlos in einer einheitlichen Benutzeroberfläche zugänglich sind. Bestehende Inhalte
aus der bisherigen Infrastruktur sollen über den webbasierten Zugang verfügbar bleiben und idealerweise
ebenfalls über den neuen App-basierten Zugang zugreifbar sein. Das Konzept muss technische Lösungsoptionen
aufzeigen, die diesen dualen Zugang ermöglichen, z. B. durch Datenintegrationsschichten oder APIs zur
Datenaggregation.

##### Sicherstellung der Kompatibilität und Unterstützung für Fachbehörden

Fachbehörden müssen bei der Anpassung ihrer IT-Systeme unterstützt werden, um nahtlos in die neue Architektur
integriert zu werden. Dies umfasst die Bereitstellung notwendiger Schnittstellen oder die Entwicklung von
Adapter-Lösungen wie Protokollkonverter, um die Anbindung ihrer bestehenden Systeme zu ermöglichen.
Das Migrationskonzept soll spezifische Maßnahmen und Werkzeuge beschreiben, um den Aufwand für Fachbehörden
so gering wie möglich zu halten.

##### Technische und organisatorische Maßnahmen für eine erfolgreiche Migration

Das Migrationskonzept soll alle weiteren bekannten technischen und organisatorischen Schritte beschreiben,
die für einen reibungslosen Übergang notwendig sind. Dazu gehört die Sicherstellung der Datenkonsistenz
zwischen den Systemen, die Planung eines Migrationszeitplans, der die Minimierung von Ausfallzeiten
gewährleistet, sowie die Festlegung von Test- und Validierungsverfahren, um die Integrität der Daten
während und nach der Migration zu bestätigen.

##### Risikobewertung und Maßnahmen zur Sicherstellung der Datenintegrität

Identifikation potenzieller Risiken und Herausforderungen im Migrationsprozess (z. B. mögliche
Datenverluste oder Inkompatibilitäten) und Festlegung von Maßnahmen zur Sicherstellung der Datenintegrität.
Das Konzept soll Backup- und Wiederherstellungsverfahren enthalten, um Datenverlust vorzubeugen und eine
rollenspezifische Verantwortung für den Migrationsprozess definieren.

#### 3.1.4 Erstellung von Erweiterungen der Matrix-Spezifikation (MSCs)

Anhand des in [Abschnitt 2.5](#25-zielbild) beschriebenen Zielbildes ergeben sich projektspezifische
Weiterentwicklungsbedarfe des Matrix-Standards mit Mehrwert über den Projektscope hinaus. Die
Auftragnehmerin führt diese Konzeptionierung auf Aufforderung in Abstimmung mit der Auftraggeberin
in Form von Matrix Spec Proposals (MSC) durch und bringt diese in den Standardisierungsprozess des
Matrix-Standards ein. Die Auftragnehmerin unterstützt den Standardisierungsprozess der eingebrachten
MSCs im Rahmen dieses Auftrags.

Der Auftraggeberin ist bewusst, dass eine Annahme der in den Standardisierungsprozess eingebrachten MSCs
aufgrund des unabhängigen Fortschreibungsprozesses der Matrix-Spezifikation nicht von der Auftragnehmerin
garantiert werden kann. Die Auftragnehmerin wird im Rahmen eines Dienstvertrages tätig und schuldet keinen
werkvertraglichen Erfolg.

#### 3.1.5 Beratungsleistungen im Rahmen der Pilotierung

Im Rahmen des Projektverlaufs ist angedacht, eine Anbindung eines Fachverfahrens an die entwickelte
Kommunikationsinfrastruktur in einer möglichst realitätsnahen Testumgebung zu pilotieren, um potentielle
technische und organisatorische Herausforderungen in der Nutzung der entwickelten Lösung und im
Anbindungsprozess zu identifizieren.

Ziel dieser Testumgebung ist es, die Funktionsfähigkeit und Praxistauglichkeit der Infrastruktur unter
realistischen Bedingungen zu demonstrieren und mögliche Optimierungsbedarfe frühzeitig zu identifizieren.

Als Pilotprojektpartner kommen Hersteller von Onlinediensten, Hersteller von Fachverfahren,
andere Behörden oder weitere Dritte infrage. In der Testumgebung soll eine digitale Antragstellung
erprobt werden, einschließlich der daraus resultierenden Folgekommunikation über die Matrix-basierte
Kommunikationsinfrastruktur, die im Rahmen dieses Projekts entwickelt wird. Die Pilotierung umfasst die
vollständige Integration der entwickelten SDKs in ein Fachverfahren zur Antragsbearbeitung, um eine nahtlose
Kommunikation und Interoperabilität zwischen den beteiligten Systemen zu gewährleisten.

Die Unterstützung bei der Anbindung und Integration von Pilotprojektpartner in die eingerichtete
Testumgebung ist Teil der von der Auftragnehmerin auf Aufforderung zu erbringenden Leistungen. Die
Leistungserbringerin stellt sicher, dass die technischen und organisatorischen Anforderungen der
Pilotprojektpartner erfüllt werden und koordiniert die Zusammenarbeit mit diesen, um eine reibungslose
Implementierung und Nutzung der Testumgebung zu ermöglichen. Hierzu steht die Auftragnehmerin auch beratend
gegenüber von der Auftraggeberin benannten Dritten zur Verfügung.

### 3.2 Softwareentwicklungsleistungen

Die Auftragnehmerin hat auf Aufforderung auf Basis der entwickelten Lösungsarchitektur zur Realisierung des
in [Abschnitt 2.5](#25-zielbild) beschriebenen Zielbildes die in den folgenden Abschnitten beschriebenen
Komponenten und Funktionalitäten zu entwickeln oder weiterzuentwickeln.

Auf Aufforderung der Auftraggeberin sind die im Folgenden beschriebenen Software-Komponenten und
Funktionalitäten Teil der zu erbringenden Leistung.

Das Entwicklungsteam spielt den jeweils aktuellen Entwicklungsstand der entwickelten oder angepassten
Softwarekomponenten eigenständig auf das bereitgestellte Testsystem (vgl.
[Abschnitt 3.4](#34-bereitstellung-und-betrieb-einer-testumgebung-inkl-support-und-wartung)) auf.

#### 3.2.1 App für mobilen Zugang für Privatpersonen

Kernbestandteil der zu entwickelnden Lösung ist eine App für einen mobilen Zugang zur Matrix-basierten
Kommunikationsinfrastruktur für Privatpersonen.

- Das Design der App soll dem jeweils aktuellen (oder zukünftigen) Design der BundID-Infrastruktur
  nachempfunden sein, um ein einheitliches Look- & Feel zu gewährleisten (App und Webanwendung in
  einheitlichem Design).
- Die App ermöglicht die bidirektionale Kommunikation von Privatpersonen mit Fachbehörden. Eine
  Kommunikation kann von Fachbehörden initiiert werden. Ob eine Kommunikation auch proaktiv von
  Privatpersonen initiiert werden können soll, wird durch die Auftraggeberin im Projektverlauf
  festgelegt.
- In der App ist eine Bündelung von Kommunikationssträngen vorzusehen (bspw. gruppiert nach
  Verwaltungsverfahren, z.B. "Beantragung Grundwasserentnahme").
- Die App soll neben der Freitext-Kommunikation auch eine strukturierte Kommunikation mit den
  Fachverfahren der Fachbehörden ermöglichen. Exemplarisch sollen folgende Funktionalitäten prototypisch
  umgesetzt werden:

  - Terminauswahl: Die Fachbehörde bietet freie Termine an. Aus diesen freien Terminen können Nutzende der
    App eine passende Terminoption auswählen.
  - Multiple-Choice-Abfragen: Die Fachbehörde stellt eine Frage und bietet mehrere frei wählbare
    Antwortmöglichkeiten an. Nutzende der App können aus den Antwortmöglichkeiten eine (oder nach Wahl der
    Fachbehörde) mehrere Antworten auswählen. Die Fachbehörde kann die getroffene Auswahl anschließend
    maschinenlesbar weiterverarbeiten. Hierzu kann ggf. der in bestehenden Matrix-Clients genutzte
    Umfrage-Mechanismus zum Einsatz kommen.
  - Statusmonitor: Fachbehörden übermitteln den Status von Verwaltungsverfahren ("Eingegangen",
    "in Bearbeitung", "Abgeschlossen", ...) in maschinenlesbarer Form nach einem von der Auftraggeberin
    festgelegten Schema
    ([aktueller Entwurf des Schemas](https://gitlab.opencode.de/ozg_basisdienste/status-schema/)) via
    Matrix-Protokoll an die App. Diese stellt den Status verschiedener Verwaltungsverfahren in der
    Übersicht dar und bietet eine Detail-Ansicht.

- Die App soll ergänzt werden um die bereits in der Weboberfläche der BundID bestehenden Funktionalitäten
  (ggf. zunächst in Form von UI-Mockups).
- Eine Authentifizierung ggü. dem Matrix-Server in der App erfolgt direkt mittels eID über die Integration
  des [AusweisApp-SDK](https://www.ausweisapp.bund.de/sdk/) und/oder über die Anbindung an die
  Authentifizierungskomponente der BundID (derzeit via SAML-Schnittstelle realisiert).
- Beim erstmaligen Login sollen im Rahmen einer Accounterstellung alle notwendigen kryptographischen Schlüssel
  lokal auf dem Endgerät der Nutzer:in erstellt und alle notwendigen öffentlichen Schlüssel für Fachverfahren
  zur Verfügung gestellt werden, sodass eine Ende-zu-Ende-verschlüsselte Kontaktaufnahme durch das
  Fachverfahren anschließend möglich ist. Wird eine Nutzer:in, die bisher noch keinen BundID-Account besitzt,
  im Rahmen der Beantragung einer Verwaltungsleistung in einem Onlinedienst von diesem zur BundID
  weitergeleitet, bietet die BundID die Möglichkeit, im Rahmen dieses SAML-Flows einen neuen BundID-Account
  zu erstellen. Auch für diesen Fall sollen im Rahmen der Accounterstellung alle notwendigen kryptographischen
  Schlüssel erzeugt und veröffentlicht werden.
- Die App soll in Abstimmung mit der Auftraggeberin als mobile Cross-Plattform-Anwendung (iOS/Android) oder
  nativ für eine der Plattformen iOS oder Android entwickelt werden. Die finale Entscheidung zur Wahl der
  zu nutzenden Plattform erfolgt während der Vertragslaufzeit.
- Die App soll zu Demonstrationszwecken ggü. relevanten Stakeholdern in den App-Stores der gewählten
  mobilen Plattform (Apple App-Store bzw. Google Play-Store und F-Droid-Store) bereitgestellt werden.
- Die Umsetzung der App soll hinsichtlich Sicherheit, Wartbarkeit und Code-Qualität in einer Art und Weise
  erfolgen, die einer zukünftigen produktiven Nutzung nicht im Wege steht.

#### 3.2.2 Webanwendung für einen Browser-basierten Zugang für Privatpersonen

Neben der mobilen Anwendung soll eine webbasierte Integration des Zugangs zur Kommunikationsinfrastruktur
in das bestehende Frontend der BundID pilotiert werden.

- Das Design der Webanwendung soll dem jeweils aktuellen Design der BundID-Infrastruktur nachempfunden sein,
  um ein einheitliches Look- & Feel zu gewährleisten (App und Webanwendung in einheitlichem Design).
- Die Webanwendung unterstützt dieselben Funktionalitäten wie die mobile App.
- Eine Authentifizierung ggü. dem Matrix-Server in der Webanwendung erfolgt über die Anbindung an die
  Authentifizierungskomponente der BundID (derzeit via SAML-Schnittstelle realisiert).
- Beim erstmaligen Login sollen im Rahmen einer Accounterstellung alle notwendigen kryptographischen
  Schlüssel lokal auf dem Endgerät der Nutzer:in erstellt und alle notwendigen öffentlichen Schlüssel für
  Fachverfahren zur Verfügung gestellt werden, sodass eine Ende-zu-Ende-verschlüsselte Kontaktaufnahme durch
  das Fachverfahren anschließend möglich ist. Wird eine Nutzer:in, die bisher noch keinen BundID-Account
  besitzt, im Rahmen der Beantragung einer Verwaltungsleistung in einem Onlinedienst von diesem zur BundID
  weitergeleitet, bietet die BundID die Möglichkeit, im Rahmen dieses SAML-Flows einen neuen BundID-Account
  zu erstellen. Auch für diesen Fall sollen im Rahmen der Accounterstellung alle notwendigen kryptographischen
  Schlüssel erzeugt und veröffentlicht werden.
- Die Umsetzung der Webanwendung soll hinsichtlich Sicherheit, Wartbarkeit und Code-Qualität in einer Art
  und Weise erfolgen, die einer zukünftigen produktiven Nutzung nicht im Wege steht. Bereits im Webinterface
  der BundID bestehende Funktionalitäten müssen nicht neu entwickelt werden.
- Die prototypische Integration in das bestehende Frontend der BundID erfolgt in einer Testumgebung in
  Kooperation mit den aktuell mit der Weiterentwicklung der BundID betrauten Dienstleistern. Die Festlegung,
  ob hierzu eine Integration in den Code des bestehenden Frontends erfolgt oder ob die zu entwickelnde
  Webanwendung lediglich mit der bestehenden Weboberfläche via Links verknüpft wird, erfolgt im
  Projektverlauf durch die Auftraggeberin.

#### 3.2.3 Demo-Client zur Darstellung der Aktivitäten von Fachbehörden

Zur Demonstration der Aktivitäten von Fachbehörden und der nahtlosen Integration der neuen
Kommunikationslösung soll ein Demo-Client entwickelt werden, der exemplarische Interaktionen eines
Fachverfahrens simuliert. In Abstimmung mit der Auftraggeberin kann hierfür ein bestehender Matrix-Chat-Client
genutzt werden, sofern dieser alle wesentlichen Funktionalitäten zur Demonstration der Fähigkeiten von App
und Webanwendung mit vertretbarem Aufwand abbilden kann.

Der Demo-Client soll grundlegende Kommunikationsabläufe simulieren, die üblicherweise im Fachverfahren der
Behörde vorkommen, und Interaktionen wie Nachrichtenversand, Dokumentenaustausch und Statusabfragen enthalten.
Die Integration der neuen Kommunikationslösung muss dabei so umgesetzt werden, dass eine realitätsnahe
Abbildung der zukünftigen Anwendungsfälle erreicht wird.

Der entwickelte Demo-Client dient ausschließlich der Durchführung von Tests und Demonstrationen muss nicht
für die Nutzung in einem späteren Produktivbetrieb ausgelegt sein. Qualitätsanforderungen, die für eine
produktive Nutzung notwendig wären (z.B. in puncto Skalierbarkeit, Wartbarkeit und Benutzbarkeit), können
daher in Abstimmung mit der Auftraggeberin gesenkt werden. In jedem Fall ist sicherzustellen, dass der
Demo-Client stabil genug ist, um kontinuierliche Tests und Präsentationen ohne größere Störungen oder
Unterbrechungen durchführen zu können.

Der Demo-Client soll regelmäßig aktualisiert werden, um relevante Anpassungen der sich in Entwicklung
befindlichen Kommunikationslösung zu reflektieren und eine konsistente Darstellung der Funktionalitäten
zu ermöglichen.

#### 3.2.4 Erweiterung des FIT-Connect-SDK

Eine Anbindung der IT-Systeme von Fachbehörden (Fachverfahren) an die zu entwickelnde Matrix-basierte
Kommunikationsinfrastruktur soll über eine Integration der notwendigen Kommunikationsschnittstellen in
das bestehende SDK des Produktes FIT-Connect ermöglicht werden.

Die hierzu notwendige Erweiterung des Software-Entwicklungskits (SDK) des Produkts FIT-Connect erfolgt
in Zusammenarbeit mit dem Umsetzungsteam des Produktes. Vor der Durchführung dieser Erweiterung erfolgt
in Zusammenarbeit mit dem FIT-Connect-Team eine umfassende Evaluierung, ob es sinnvoll ist, die
Matrix-Funktionalitäten direkt in das bestehende FIT-Connect-SDK zu integrieren oder ob es vorteilhafter
ist, diese Funktionalitäten in ein separates SDK auszulagern.

#### 3.2.5 Erweiterungen an bestehenden SDKs und weiteren Komponenten aus dem Matrix-Ökosystem

Wo erforderlich, sind notwendige Erweiterungen und Weiterentwicklungen der bestehenden
Software-Entwicklungskits (SDKs) und weiteren Komponenten aus dem Matrix-Ökosystem durchzuführen, die im
Rahmen der Projektumsetzung verwendet werden. Die Leistung umfasst auch die Durchführung aller erforderlichen
Anpassungen an den Matrix-SDKs, die aus der Umsetzung von erforderlichen Spezifikationserweiterungen (MSCs)
resultieren. Zusätzlich sind nach Bedarf der Ausbau und die Qualitätssicherung der zugehörigen Dokumentation
bestehender Matrix-SDKs und weiteren Komponenten gewünscht. Die genaue Art und der Umfang dieser Maßnahmen
werden im Verlauf des Projekts von der Auftraggeberin detailliert festgelegt.

#### 3.2.6 Self-Service-Portal-Funktionalität

Die Registrierung und Anbindung von Fachbehörden und Betreiber:innen von Onlinediensten soll in einer
zukünftigen Produktivsetzung über ein Self-Service-Portal realisiert werden, um eine Skalierfähigkeit der
hierzu nötigen Anbindungsprozesse zu ermöglichen. Die bereitgestellte Self-Service-Funktionalität ermöglicht
es Fachbehörden und Onlinedienst-Betreiber:innen nach Authentifizierung, ihre relevanten Daten eigenständig
in das System einzupflegen. Dies umfasst die Eingabe und Verwaltung von Zertifikaten, spezifischen
Fachbehördenmerkmalen wie Logo, Adresse und Kontaktinformationen sowie die Angabe von Schnittstellenadressen
für die Kommunikation mit der zentralen Plattform. Zusätzlich soll das Self-Service-Portal Möglichkeiten zur
Verwaltung von Zugriffsrechten und Benachrichtigungen bieten, damit Fachbehörden die Kontrolle über ihre
eigenen Daten behalten und im Falle von Aktualisierungen oder ausstehenden Aktionen benachrichtigt werden
können.

Für ein späteres Produktivszenario soll diese Self-Service-Funktionalität in ein bestehendes
Self-Service-Portal integriert werden, um eine nahtlose Verwaltung aller behördenrelevanten Informationen
an einem zentralen Ort zu gewährleisten. Naheliegend ist hier die Integration in das
[Self-Service-Portal der BundID](https://ssp.id.bund.de/) oder in das
[Self-Service-Portal des Produktes FIT-Connect](https://portal.auth-testing.fit-connect.fitko.dev/landing).

Im Rahmen der Pilotierung der zu entwickelnden Kommunikationsinfrastruktur ist die Bereitstellung einer
skalierfähigen Self-Service-Funktionalität zunächst nachrangig. Bei der Konzeption der entwickelten
Komponenten sollte jedoch bereits darauf geachtet werden, dass notwendige Schnittstellen und Datenformate
ausreichend spezifiziert und dokumentiert sind, um die spätere Integration in ein bestehendes
Self-Service-Portal zu erleichtern.

In Abstimmung mit der Auftraggeberin wird im Projektverlauf festgelegt, ob notwendige
Registrierungsprozesse zunächst manuell abgebildet werden, ob ein leichtgewichtiges Self-Service-Portal zu
Demonstrationszwecken entwickelt wird oder ob die erforderliche Self-Service-Funktionalität in ein von der
Auftraggeberin festzulegendes bestehendes Self-Service-Portal integriert wird. Die Abstimmung und Zustimmung
der Auftraggeberin muss in Form eines [Architecture Decision Records (ADRs)](https://adr.github.io/) von der
Auftragnehmerin dokumentiert werden.

### 3.3 Umfassende Dokumentation für Hersteller von Fachverfahren

Zur Unterstützung von Herstellern von Fachverfahren wird eine umfassende von der Auftragnehmerin
Dokumentation des technischen Zugangs zur neu entwickelten Kommunikationsinfrastruktur nach dem Vorbild der
[Dokumentation des Produktes FIT-Connect](https://docs.fitko.de/fit-connect/) erstellt. Diese Dokumentation
hat alle notwendigen technischen Informationen und Anleitungen zu enthalten, die für eine erfolgreiche
Anbindung und Nutzung der Kommunikationslösung erforderlich sind. Sie bietet den Herstellern von Fachverfahren
alle erforderlichen Informationen, um die Kommunikationslösung effektiv und sicher in ihre bestehenden Systeme
zu integrieren. Dazu zählen:

#### Detaillierte Beschreibung der Schnittstellen und APIs

Die Dokumentation soll sämtliche Schnittstellen klar und verständlich beschreiben, einschließlich
Authentifizierungsmethoden, Datenformate und Anwendungsbeispiele. Hierfür ist die Verwendung von einem
maschinenlesbaren Format wie OpenAPI angedacht. Dies umfasst auch die die Struktur der
Ende-zu-Ende-verschlüsselten Kommunikation.

#### Erläuterung der Sicherheitsmechanismen und Anforderungen

Um die Sicherheit bei der Implementierung zu gewährleisten, sollen die wichtigsten Sicherheitsanforderungen
und Mechanismen umfassend beschrieben werden, einschließlich des Umgangs mit Schlüsselmaterial,
Verschlüsselungstechniken und Maßnahmen zur Missbrauchsvermeidung.

#### Detaillierte Anleitungen zur Fehlerbehebung und Support-Ressourcen

Häufige Fehler und mögliche Lösungen sollten in der Dokumentation enthalten sein, ebenso wie eine Übersicht
über verfügbare Support-Ressourcen und Eskalationswege für technische Fragen.

#### Verfügbarkeit der Dokumentation

Die gesamte Dokumentation wird im Markdown-Format erstellt und über das
[Föderale Entwicklungsportal](https://docs.fitko.de/) öffentlich bereitgestellt, sodass sie zentral und
zugänglich für alle relevanten Stakeholder zur Verfügung steht. Die Nutzung von Markdown gewährleistet eine
klare Struktur und Lesbarkeit der Inhalte, wobei bei der Gestaltung auf Konsistenz und zielgruppengerechte
Sprache geachtet wird.

#### Verbesserung bestehender Dokumentation der Matrix-Komponenten

Zusätzlich wird die bestehende Dokumentation der verwendeten Matrix-Komponenten im Ökosystem überprüft und
nach Bedarf erweitert, um ihre Verständlichkeit und Zugänglichkeit für die Fachverfahrenshersteller zu erhöhen.
Diese Verbesserungen sollen sicherstellen, dass alle technischen Anforderungen und Konfigurationsmöglichkeiten
der Matrix-Komponenten klar dargestellt sind, einschließlich Anweisungen zur Einrichtung, Anpassung und Wartung.
Eine klare Trennung zwischen relevanten Informationen für Entwickler:innen und administrativen Details wird
angestrebt, um die Navigation innerhalb der Dokumentation zu erleichtern.

#### Kontinuierliche Aktualisierung und Versionierung

Um die Dokumentation aktuell und nützlich zu halten, wird ein Prozess zur regelmäßigen Überprüfung und
Aktualisierung implementiert. Änderungen an der Kommunikationsinfrastruktur, neue Funktionen oder Anpassungen
an den Schnittstellen werden zeitnah dokumentiert und versioniert, sodass Fachverfahrenshersteller stets auf
die neuesten Informationen zugreifen können.

### 3.4 Bereitstellung und Betrieb einer Testumgebung inkl. Support und Wartung

Eine Erprobung der beschriebenen Funktionalität soll in einer separat aufzubauenden, aus dem Internet
erreichbaren Testumgebung erfolgen. Das Testsystem dient der Demonstration der Funktionsfähigkeit der
entwickelten Lösung ggü. externen Stakeholdern und der Durchführung von Last- und Performanztests (vgl.
[Abschnitt 3.5](#35-durchführung-von-last--und-performanztests)).

Das Testsystem wird auf von der Auftragnehmerin bereitgestellter Infrastruktur betrieben und umfasst
insbesondere Softwarekomponenten (wie einen Matrix-Server), die passend zum beschriebenen Zielbild,
beziehungsweise dem aktuellen Stand der Lösungsarchitektur ein Umsystem und entsprechende Funktionalität
zur Demonstration der technischen Machbarkeit, Funktionsfähigkeit, Skalierbarkeit, Leistungsfähigkeit
und Belastbarkeit der entwickelten Lösung bereitstellen. Die Plattform dient beispielsweise im Projektverlauf
zum parallelen Betrieb verschiedener Versionen der entwickelten Software-Artefakte (u.a. für A/B-Tests),
entsprechend beteiligen sich die Entwickler:innen bei der Ausgestaltung der Plattform.

Im Rahmen der Demonstration der Funktionsfähigkeit sind keine besonders hohen Nutzungszahlen zu erwarten.
Die im Rahmen der Erprobung aufgebaute Testumgebung soll eine unbeschränkte Zahl an Test-Zugängen ermöglichen,
mindestens jedoch 50 Zugänge inkludieren. Im Rahmen der Erprobung ist geplant, mit 1-3 Fachverfahren
Anbindungstests durchzuführen. Für die Durchführung der Last- und Performanztests ist vorübergehend mit einer
wesentlich höheren Auslastung zu rechnen (vgl. [Abschnitt 3.5](#35-durchführung-von-last--und-performanztests)).
Die im Rahmen der Erprobung aufgebaute Testumgebung soll daher eine ausreichende Skalierungsfähigkeit
("Elastizität") für die Durchführung der Last- und Performanztests aufweisen.

Das Testsystem muss in einem strukturierten Rahmen gewartet werden, um sicherzustellen, dass es kontinuierlich
effektiv und zuverlässig funktioniert.

#### Support & Wartung der Testumgebung

Support und Wartung müssen folgende Aspekte umfassen:

**Supportleistungen**: Die Auftragnehmerin leistet Support zu Behebung von Störungen und nach Bedarf zur
Konfiguration von Accounts über ein Servicedesk und/oder ein Self-Service-Portal in deutscher oder englischer
Sprache. Die Kommunikation mit dem Servicedesk soll mindestens per Mail, idealerweise auch telefonisch
möglich sein.

**Regelmäßige Updates**: Softwarekomponenten des Testsystems müssen regelmäßig aktualisiert werden, um
sicherzustellen, dass sie mit den neuesten Sicherheits-Patches und Funktionserweiterungen ausgestattet sind.
Dies gilt insbesondere für genutzte Abhängigkeiten von Drittanbietern und Betriebssysteme. Ausgenommen sind
Updates der im Rahmen dieser Leistungsbeschreibung entwickelten oder angepassten Komponenten auf der
Testumgebung, sofern die Aktualisierung dieser Komponenten durch das Entwicklungsteam erfolgt.
Das Entwicklungsteam spielt den jeweils aktuellen Entwicklungsstand der entwickelten oder angepassten
Softwarekomponenten eigenständig auf das bereitgestellte Testsystem auf. Es wird davon ausgegangen, dass
regelmäßige Updates der in den entwickelten oder angepassten Softwarekomponenten genutzten
Abhängigkeiten (eingebundene Softwarebibliotheken) während des Entwicklungsprozesses eigenständig durch das
Entwicklungsteam vorgenommen werden.

**Überwachung der Systemintegrität**: Die Integrität und Stabilität des Testsystems sollten kontinuierlich
überwacht werden. Dabei ist es wichtig, Systemlogs und Fehlerberichte regelmäßig zu analysieren, um potenzielle
Probleme frühzeitig zu identifizieren und zu beheben.

**Sicherheitsüberprüfungen**: Regelmäßige Sicherheitsüberprüfungen und -tests müssen durchgeführt werden,
um sicherzustellen, dass das Testsystem vor externen Bedrohungen geschützt bleibt.

**Backup und Wiederherstellung**: Ein robustes Backup- und Wiederherstellungssystem sollte implementiert und
regelmäßig geprüft werden. Dies stellt sicher, dass im Falle eines Systemausfalls oder Datenverlusts die
Testumgebung schnell wiederhergestellt werden kann.

Die hier beschriebene Testumgebung dient der Demonstration der Funktionsfähigkeit der entwickelten Lösung ggü.
externen Stakeholdern und der Durchführung von Last- und Performanztests. Sofern neben der hier beschriebenen
Testumgebung weitere, interne Umgebungen aufgebaut werden (z.B. eine Entwicklungsumgebung oder Umgebungen für
ergänzende Qualitätssicherungsmaßnahmen oder temporäre Teststellungen), so obliegt es der Auftragnehmerin,
abweichend vom Kriterienkatalog für Cloudleistungen (Anlage 4) angemessene und verhältnismäßige Schutzmaßnahmen
vor Störungen wie Datenverlust oder IT-Sicherheitsvorfällen zu treffen. Die Bereitstellung dieser weiteren
Umgebungen und die getroffenen Schutzmaßnahmen sind durch die im Preisblatt vereinbarten Preise abgegolten.

### 3.5 Durchführung von Last- und Performanztests

Für die Durchführung der Last- und Performanztests ist von der Auftragnehmerin ein umfassendes Testkonzept
zu entwickeln, das eine Skalierbarkeit, Leistungsfähigkeit und Belastbarkeit der entwickelten
Kommunikationsinfrastruktur demonstriert. Mit diesem Testkonzept soll nachgewiesen werden, dass der Pilot
in der Lage ist, die Anforderungen in einer produktiven Umgebung zu erfüllen. Die Auftraggeberin wird den
Zeitpunkt der Erfüllung des Testkonzepts im Projektverlauf festlegen und das Testkonzept bewerten, um die
Ergebnisse in die weiteren Planungen und Entscheidungen einfließen zu lassen. Das Testsystem soll aus Gründen
der Ressourcenschonung nicht dauerhaft in der für die Durchführung der Last- und Performanztests nötigen
Skalierung dimensioniert sein.

#### 3.5.1 Erwartete Mengengerüste

Im Oktober 2024 waren ca. 4,7 Millionen BundID-Accounts registriert (siehe
[Dashboard Digitale Verwaltung](https://dashboard.digitale-verwaltung.de/)). Im September 2024 erfolgten
2.062.199 eID-Transaktionen in der Form von Login-Vorgängen mittels des elekronischen Personalausweises,
und im März 2023 erfolgten 2.706.327 eID-Transaktionen im Backend der BundID. Perspektivisch sollen
jedoch *bis zu 60 Millionen registrierte Nutzende* unterstützt werden. Das Testkonzept muss diese
Nutzendenzahlen realitätsnah abbilden können.

#### 3.5.2 Leistungskriterien für die Last- und Performanztests

Das von der Auftragnehmerin bereitgestellte Testsystem (siehe
[Abschnitt 3.4](#34-bereitstellung-und-betrieb-einer-testumgebung-inkl-support-und-wartung)) muss demonstrieren,
dass eine gesamtheitliche Skalierfähigkeit der entwickelten Infrastruktur für bis zu 60 Millionen registrierte
Nutzende realisiert werden kann. Hierzu wäre beispielsweise auch ausreichend, eine lineare Skalierung zu
demonstrieren und entsprechende plausible Hochrechnungen vorzunehmen. Zur entwickelten Infrastruktur zählen
die im Rahmen der Leistungserbringung entwickelten Software-Komponenten (vgl.
[Abschnitt 3.2](#32-softwareentwicklungsleistungen)) sowie alle weiteren eingesetzten IT-Komponenten (u.a.
eingesetzter Matrix-Server, Matrix-SDKs, Webserver, Datenbanken, Speicherlösungen, etc.).

Um sicherzustellen, dass es robust, skalierbar und zuverlässig ist, sollen die folgenden Leistungskriterien
erfüllt werden:

**Skalierbarkeit**: Das Testsystem muss in der Lage sein, vertikale Skalierbarkeit zu demonstrieren, um
sicherzustellen, dass es unter zunehmender Nutzlast stabil bleibt.

**Performanz**: Es sollten umfassende Performanztests durchgeführt werden, um die Antwortzeiten, die
Durchsatzraten und die maximale Last der Software zu messen. Kritische Parameter wie Latenzzeiten und
Ressourcenauslastung müssen unter normalen sowie Spitzenlastbedingungen optimiert sein.

**Lasttestfähigkeit**: Das System muss Lasttests unterstützen, die die Plattform unter realistischen und
extremen Bedingungen prüfen, um Engpässe zu identifizieren und zu beheben. Dazu gehören die Simulation von
Verkehrsspitzen und der parallele Zugriff vieler Nutzender.

**Fehlertoleranz und Stabilität**: Die Fähigkeit, die Robustheit und Ausfallsicherheit der Plattform zu
testen, ist entscheidend. Dies schließt Tests auf mögliche Systemausfälle, Wiederherstellungsmechanismen und
die Reaktionsfähigkeit auf Störungen ein.

**Automatisierung**: Das Testsystem sollte automatisierte Tests unterstützen, um wiederkehrende Testfälle
effizient durchzuführen und kontinuierliche Integration sowie Lieferung (CI/CD) zu fördern.

**Flexibilität**: Die Testumgebung muss flexibel konfigurierbar sein, um verschiedene Betriebsszenarien,
Nutzerlasten und Infrastrukturanforderungen nachbilden zu können.

**Wiederholbarkeit und Konsistenz**: Testfälle sollten unter identischen Bedingungen wiederholbar sein, um
eine konsistente Bewertung der Systemleistung zu ermöglichen und sicherzustellen, dass Verbesserungen
tatsächlich Wirkung zeigen.

**Berichterstattung und Analyse**: Das Testsystem muss detaillierte Berichte und Echtzeit-Analysen über die
Testergebnisse liefern, um genaue Diagnosen von Problemen und Verbesserungspotenzialen zu ermöglichen.

### 3.6 Option: Weiterentwicklung des MVP zur Vorbereitung einer möglichen Produktivsetzung

Im Falle einer erfolgreichen Zusammenarbeit und eines erfolgreichen Projektverlaufes im Rahmen der initialen
Laufzeit von 12 Monaten erwägt die Auftraggeberin die Weiterentwicklung des MVP entlang des Zielbildes (vgl.
[Kapitel 2.5](#25-zielbild)). Hierfür stehen drei Verlängerungsoptionen à 12 Monate zur Verfügung (siehe
[Abschnitt 6.1](#61-vertragslaufzeit-und--optionen)). In diesem Rahmen können zusätzliche Funktionalitäten
umgesetzt werden, die sich aus der Fortentwicklung des Zielbildes ergeben. Darüber hinaus sollen Vorbereitungen
für eine mögliche Produktivsetzung getroffen werden. Die Produktivsetzung selbst ist nicht Gegenstand der zu
erbringenden Leistungen.

### 3.7 Leistungen zu Vertragsende

Zur Gewährleistung eines reibungslosen Übergangs bei Vertragsende zu einer neuen Dienstleisterin ist
innerhalb der Vertragslaufzeit aufwandsabhängige Unterstützung zu leisten. Diese Unterstützung ist
mindestens im Rahmen einer umfassenden Infrastrukturanalyse und dem Erstellen detaillierter Migrationspläne
sowie bei der Datenmigration, Anpassung von Anwendungen und Systeme zu gewähren, um eine nahtlose Integration
zu gewährleisten. Die Auftragnehmerin steht auch zur Unterstützung von Tests im Rahmen einer Migration zu
einer neuen Infrastruktur zur Verfügung, um die Funktionalität sicherzustellen und potenzielle Probleme zu
beheben. Bei Vertragsende sind alle Dokumentationen auf Ausführlichkeit und Vollständigkeit zu prüfen und
ggf. – anlassbezogen auch nach Ende der Vertragslaufzeit - zu komplettieren. Es findet eine transparente
Kommunikation während des gesamten Prozesses statt.

Die Auftragnehmerin hat alle nötigen Informationen und Hinweise zu Konfigurationsmöglichkeiten
bereitzustellen, um einen reibungslosen Produktivbetrieb zu ermöglichen. Im Rahmen der Softwareentwicklung
und -testung gewonnene Erkenntnisse mit Auswirkungen auf den Betrieb der Software sind proaktiv an
Betriebsdienstleisterinnen zu kommunizieren. Hierzu zählen insbesondere Auswirkungen von
Konfigurationsoptionen auf die Sicherheit und den störungsfreien Betrieb der Software. Auftretende
betriebsrelevante Fragestellungen sind im bilateralen Austausch zwischen der Auftragnehmerin und
Betriebsdienstleisterinnen in angemessener Frist zu beantworten.

## 4 Übergreifende Anforderungen

### 4.1 Prinzipien der Architekturgestaltung

Die Auftragnehmerin hat bei der Gestaltung der Softwarearchitektur zwingend die folgenden Prinzipien zu
beachten und diese bereits während der Entwicklung zu berücksichtigen:

[**Ende-zu-Ende-Sicherheit**](https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Publikationen/TechnischeRichtlinien/TR03176/BSI-TR-03176.pdf?__blob=publicationFile&v=3):
Die Architektur der Software muss eine durchgehende Sicherheitsstrategie umsetzen, die sowohl
Ende-zu-Ende-Verschlüsselung als auch Ende-zu-Ende-Authentisierung umfasst. Dabei muss sichergestellt werden,
dass die Kommunikation zwischen allen beteiligten Komponenten und Nutzern durch Verschlüsselung vor unbefugtem
Zugriff geschützt ist und dass die Identität der Kommunikationspartner zu jeder Zeit zuverlässig
authentifiziert wird.

[**Zero-Trust-Architektur**](https://www.bsi.bund.de/EN/Themen/Unternehmen-und-Organisationen/Informationen-und-Empfehlungen/Zero-Trust/zero-trust_node.html):
Die Auftragnehmerin hat sicherzustellen, dass kein Benutzer, Dienst oder System innerhalb der Architektur von
Natur aus als vertrauenswürdig angesehen wird. Jeder Zugriff auf Systemkomponenten oder Daten muss unabhängig
von Standort oder Herkunft einer strikten Authentifizierung und Autorisierung unterliegen. Einekontinuierliche
Überprüfung von Identitäten und Berechtigungen ist zu gewährleisten, um das Prinzip des geringsten
Privilegs konsequent umzusetzen.

[**Privacy-by-Design**](https://www.edpb.europa.eu/system/files/2021-04/edpb_guidelines_201904_dataprotection_by_design_and_by_default_v2.0_de.pdf):
Datenschutzaspekte müssen bereits in der frühen Planungs- und Entwurfsphase der Architektur berücksichtigt
werden. Die Auftragnehmerin ist verpflichtet, alle relevanten datenschutzrechtlichen Bestimmungen zu beachten
und geeignete Maßnahmen zur Wahrung der Privatsphäre der Nutzer in das Systemdesign einzubauen. Insbesondere
ist eine frühzeitige Berücksichtigung von Datenschutzfolgenabschätzungen gemäß den gesetzlichen
Anforderungen (z.B. DSGVO) sicherzustellen, um mögliche Risiken für die Privatsphäre zu identifizieren und zu
minimieren.

[**Security-by-Design**](https://www.cisa.gov/sites/default/files/2023-10/SecureByDesign_1025_508c.pdf):
Die Sicherheit der Anwendung muss als grundlegendes Element des gesamten Entwicklungsprozesses betrachtet
werden. Es ist erforderlich, dass alle sicherheitsrelevanten Anforderungen von Anfang an in die Architektur
und den Entwicklungszyklus integriert werden. Dazu gehören der Schutz sensibler Daten, die Absicherung von
Schnittstellen, das Management von Zugriffsrechten sowie die Implementierung robuster Schutzmechanismen gegen
bekannte Bedrohungen. Bereits bei der Softwareentwicklung ist auf Best Practices dahingehend zu achten.
Alle Sicherheitsanforderungen müssen regelmäßig getestet und weiterentwickelt werden, um aktuelle
Sicherheitsstandards einzuhalten. Vor dem Einsatz sollen Softwarebibliotheken ebenfalls nach diesen
Gesichtspunkten bewertet werden, speziell solche, die kryptografische Bausteine implementieren, müssen für den
Produktivbetrieb ausgelegt sein. Dabei sind Eigenschaften wie Resistenz gegen Side-Channels und Timing
Angriffe, Formal Verification zu berücksichtigen.

[**Separation of Concerns**](https://en.wikipedia.org/wiki/Separation_of_concerns): Komponenten und
Teilkomponenten wie SDKs und Libraries sollten so geschnitten sein, dass sie klar abgegrenzte Aufgaben erfüllen,
die alleinstehend und unabhängig insbesondere auf Sicherheitsanforderungen getestet, analysiert und überprüft
werden können.

Alle diese Prinzipien sind konsequent und durchgängig in die Erstellung der Lösungsarchitektur zu integrieren.
Die Auftragnehmerin hält die oben genannten Anforderungen in jeder Phase der Softwareentwicklung und
-umsetzung ein, um ein Höchstmaß an Sicherheit, Datenschutz und Vertrauenswürdigkeit zu gewährleisten.

### 4.2 Benutzbarkeit, Nutzererfahrung und Barrierefreiheit der Anwendung

Die Anwendung muss höchsten Ansprüchen an Benutzbarkeit (Usability), Nutzererfahrung (User Experience) und
Barrierefreiheit genügen, um sicherzustellen, dass sie für eine möglichst breite und diverse Nutzendenschaft
zugänglich, einfach zu bedienen und inklusiv gestaltet ist.

#### 4.2.1 Benutzbarkeit und Nutzererfahrung

Die Benutzeroberflächen und Schnittstellen sollen so gestaltet werden, dass sie intuitiv, effizient und
fehlerresistent genutzt werden können. Dabei müssen die Anforderungen sowohl der Anwender:innen und
Administrator:innen als auch der Entwickler:innen berücksichtigt werden.

**Für Anwendende und Administrierende**: Die Benutzeroberflächen sollen logisch und übersichtlich aufgebaut sein,
mit klaren Navigationselementen und einer konsistenten Struktur. Funktionen müssen leicht verständlich und
mit minimalem Aufwand nutzbar sein. Feedbackmechanismen, wie Erfolgsmeldungen oder Warnhinweise, sollen die
Bedienung unterstützen und Fehler vermeiden.

**Für Entwickelnde**: Die bereitgestellten Werkzeuge und Schnittstellen sollen eine reibungslose Integration
und Weiterentwicklung der Anwendung ermöglichen. Dazu gehören intuitive APIs, hilfreiche Fehlermeldungen,
klar dokumentierte Prozesse sowie die Bereitstellung von SDKs, die eine effiziente Entwicklung fördern.

#### 4.2.2 Barrierefreiheit

Die Anwendung muss den Anforderungen an digitale Barrierefreiheit genügen, um eine diskriminierungsfreie
Nutzung für Menschen mit unterschiedlichsten Bedürfnissen zu gewährleisten. Es gelten mindestens die
Anforderungen der [BITV 2.0](https://www.gesetze-im-internet.de/bitv_2_0/BJNR184300011.html) sowie der
relevanten [W3C Standards (WCAG)](https://www.w3.org/WAI/standards-guidelines/wcag/). Zur Überprüfung der
Abwesenheit von Barrieren können die von
[GOV.UK entwickelten Personas](https://www.gov.uk/government/publications/understanding-disabilities-and-impairments-user-profiles)
genutzt werden.

Folgende Aspekte bedürfen besonderer Beachtung bei der Konzeption und Umsetzung der zu entwickelnden Lösung:

**Seheinschränkungen**: Maßnahmen wie die Bereitstellung von Alt-Texten für Bilder, ausreichende Kontraste,
anpassbare Schriftgrößen und Unterstützung von Screenreadern und Sprachsteuerung sollen Nutzenden mit
Seheinschränkungen die barrierefreie Nutzung ermöglichen.

**Menschen ohne Personalausweis**: Die Anwendung muss alternative Identifikationsmethoden bieten, die auch
Menschen ohne Zugang zu einem Personalausweis oder einer eID berücksichtigen.

**Stigmatisierungs- und Diskriminierungsfreiheit**: Die Gestaltung der Anwendung soll frei von
stigmatisierenden oder diskriminierenden Elementen sein. Ziel ist eine inklusive und respektvolle Ansprache
aller Nutzenden.

**Mehrsprachigkeit**: In den für Privatpersonen entwickelten User Interfaces (Web, App) gibt es einen starken
Bedarf nach Mehrsprachigkeit, auch in Schreibrichtung rechts-nach-links. Betrachtet werden sollen waagrechte
dextrograde und sinistrograde Schriften, nicht waagrechte Schriften gehören nicht zum Bestandteil der Umsetzung.
Die eigentliche Übersetzung der Sprachen obliegt der Auftraggeberin, Bestandteil der Umsetzung ist aber das
Schaffen technischer Möglichkeiten zur generellen Übersetzbarkeit der Anwendung.

**Einfache Sprache**: Die Anwendung soll entweder standardmäßig in einfacher Sprache verfügbar sein oder eine
Einstellmöglichkeit bieten, die Inhalte in leicht verständlicher Form darstellt.

**Geschlechtergerechte Sprache**: Die Texte und Bezeichnungen in der Anwendung müssen geschlechtergerecht
formuliert sein, um eine inklusive und wertschätzende Kommunikation sicherzustellen.

**Usability- und Barrierefreiheits-Tests**: Es werden Tests mit verschiedenen Nutzendengruppen,
einschließlich Personen mit Beeinträchtigungen durchgeführt, um die tatsächliche Nutzerfreundlichkeit und
Barrierefreiheit zu überprüfen.

**Feedback**: Nutzende werden in den Entwicklungsprozess durch Befragungen und iterative Feedback-Schleifen
eingebunden, um die Benutzungsfreundlichkeit zu verbessern.

**Automatisierte Prüfwerkzeuge**: Es werden Tools zur automatisierten Überprüfung der Barrierefreiheit
genutzt, um frühzeitig potenzielle Probleme zu erkennen und zu beheben.

**Referenzen und Standards**: Die Umsetzung orientiert sich an anerkannten Standards und Leitlinien für
digitale Barrierefreiheit, darunter die
[Handreichungen der Überwachungsstelle des Bundes für Barrierefreiheit von Informationstechnik (BFIT) zur Barrierefreiheit](https://docs.fitko.de/resources/handreichungen-zur-barriefreiheit),
das
[Informationsportal Digitale Barrierefreiheit](https://docs.fitko.de/resources/informationsportal-digitale-barrierefreiheit)
des Bundesministerium des Innern und für Heimat (BMI) in Zusammenarbeit mit der Hessischen Landesbeauftragten
für barrierefreie IT und dem Informationstechnikzentrum Bund sowie das
[Prinzip des Onlinezugangsgesetztes zur Barrierefreiheit, Bürgernähe und Genderneutralität](https://www.digitale-verwaltung.de/Webs/DV/DE/onlinezugangsgesetz/servicestandard/prinzip-3/prinzip-3-node.html).

### 4.3 Informationssicherheit

Die Informations- und Kommunikationstechnik (IT) unterstützt maßgeblich die Aufgaben öffentlicher
Einrichtungen. Auch beim Betrieb eines Test-Systems sind grundlegende Aspekte der Informationssicherheit
zu beachten, um die Stabilität und Funktionalität der IT-Systeme sicherzustellen. Im Vordergrund stehen
hierbei vor allem die Integrität, Verfügbarkeit und Vertraulichkeit des Systems.

**Monitoring**: Ein grundlegendes Monitoring sollte eingerichtet sein, um den Zustand des Test-Systems zu
beobachten und frühzeitig auf Probleme reagieren zu können.
[Schwachstellenanalysen und Webchecks](https://www.bsi.bund.de/DE/Themen/Oeffentliche-Verwaltung/Sicherheitspruefungen/Pen_Test_und_IS_Webcheck/pent-tests-und-is-webcheck_node.html)
sind in Abstimmung mit der Auftraggeberin nach Bedarf und Testzweck optional durchzuführen und werden im
Rahmen der Konzeptions- und Beratungsleistungen nach Aufwand abgerechnet.

**Cybersicherheit**: Für das Test-System sollten die grundlegenden Cybersicherheitsmaßnahmen beachtet werden,
um die Funktionalität des Systems zu gewährleisten.

**Umgebungsparametrisierung**: Test-System und produktive Umgebungen müssen strickt voneinander getrennt
betrieben werden, um mögliche Auswirkungen auf Produktivumgebungen zu vermeiden. Die Trennung von
Entwicklungs- und Testumgebungen ist in angemessenem Rahmen sicherzustellen.

**Administration**: Die IT-Administration des Test-Systems sollte sich auf die Behebung von Störungen
konzentrieren, die den Test- oder Demobetrieb beeinträchtigen könnten.

Eine regelmäßige Weiterentwicklung des Test-Systems kann flexibel und nach Bedarf erfolgen, wobei der
Fokus auf der Unterstützung des jeweiligen Testzwecks liegt.

### 4.4 Datenschutz

Im angestrebten Zielbild werden Privatpersonen die Möglichkeit haben, sich für die Nutzung des entwickelten
Systems zu registrieren. Daher ist die Einhaltung datenschutzrechtlicher Erfordernisse bereits während der
Entwicklung des MVP von besonderer Bedeutung und von der Auftragnehmerin einzuhalten. Die Vorgaben der
Datenschutz-Grundverordnung (DSGVO) sowie des Bundesdatenschutzgesetzes (BDSG) sind dabei zwingend
einzuhalten (insb. Datenschutz durch Technikgestaltung und durch datenschutzfreundliche Voreinstellungen).

Im Rahmen der Leistungserbringung ist kein Produktivbetrieb der entwickelten Lösung vorgesehen. Dennoch
müssen bereits in der Entwicklungsphase der Lösungen die folgenden für einen Produktivbetrieb wichtigen
Datenschutz-Aspekte mitgedacht und - soweit erforderlich - bereits in der Konzeption und Umsetzung der
Lösung berücksichtigt werden:

**Rechtmäßigkeit der Verarbeitung**: Im Systems, sowie in den dazugehörigen "Umsystemen" (z.B. Webserver,
Log-Daten, Backups, etc.), die im Zuge des Projekts betrieben werden, werden entsprechend personenbezogene
Daten verarbeitet. Daher muss durch die technische und organisatorische Ausgestaltung des Systems
sichergestellt werden, dass jede Form der Datenverarbeitung ausschließlich im Einklang mit den geltenden
Datenschutzbestimmungen erfolgt. Insbesondere ist zu gewährleisten, dass die Verarbeitung personenbezogener
Daten stets auf einer der Rechtsgrundlagen des Art. 6 DSGVO basiert. Die Auftragnehmerin hat sicherzustellen,
dass für jede Datenverarbeitung ein klarer Erlaubnistatbestand (z.B. Einwilligung, berechtigtes Interesse,
gesetzliche Grundlage) dokumentiert und nachvollziehbar ist.

**Informationspflicht**: Privatpersonen, deren Daten im System verarbeitet werden, sind umfassend und
transparent gemäß Art. 13 DSGVO in einer leicht verständlichen Datenschutzerklärung über Art, Umfang,
Zweck und Rechtsgrundlage der Datenverarbeitung zu informieren. Diese Erklärung muss sowohl während der
Registrierung als auch fortlaufend zugänglich gemacht werden. Die Informationspflichten umfassen insbesondere
Angaben zur Speicherdauer der Daten, den Rechten der betroffenen Personen (z.B. Auskunftsrecht, Löschrecht)
sowie den Kontaktdaten des Datenschutzbeauftragten. Die Erstellung einer Datenschutzerklärung ist nicht Teil
der beauftragten Leistungen. Die Auftragnehmerin muss jedoch auf Nachfrage alle hierzu relevanten
Informationen über das entwickelte System zur Verfügung stellen.

**Auskunftsrecht**: Privatpersonen, deren Daten im System verarbeitet werden, haben gemäß Art. 15 DSGVO das
Recht, eine Kopie der personenbezogenen Daten, die Gegenstand der Verarbeitung sind, zu erhalten. Hierzu
soll die entwickelte Lösung technische Maßnahmen zur automatisierten Bereitstellung einer Kopie dieser
Daten umsetzen, sodass Auskunftsbegehren aufwandsarm und zeitnah bereitgestellt werden können.

**Recht auf Löschung**: Privatpersonen, deren Daten im System verarbeitet werden, haben gemäß Art. 17 DSGVO
das Recht auf Löschung ihrer personenbezogenen Daten, die Gegenstand der Verarbeitung sind. Hierzu soll
die entwickelte Lösung technische Maßnahmen zur automatisierten Löschung dieser Daten umsetzen, sodass
Löschbegehren aufwandsarm und zeitnah umgesetzt werden können.

**Umsysteme**: Für die "Umsysteme" sind die datenschutzrechtlichen Regelungen ebenso anzuwenden. In diesen
Systemen, die ggf. indirekt personenbezogene Daten verarbeiten (z.B. Protokolldaten auf Webservern),
müssen die entsprechenden Anforderungen des Datenschutzes erfüllt sein. Dies umfasst insbesondere das
Prinzip der Datenminimierung, wonach nur die für den jeweiligen Zweck erforderlichen Daten verarbeitet
werden dürfen, sowie die Umsetzung von technischen und organisatorischen Maßnahmen zur Datensicherheit
gemäß Art. 32 DSGVO.

**Auftragsverarbeitung**: Es ist zudem sicherzustellen, dass die entsprechenden Verträge zur
Auftragsverarbeitung gemäß Art. 28 DSGVO abgeschlossen werden, sofern Dritte (z.B. Hosting-Provider) mit der
Verarbeitung personenbezogener Daten beauftragt werden (z.B. in Test- oder Demo-System). Die Systeme
müssen so konzipiert sein, dass die Wahrnehmung der Rechte der betroffenen Personen, wie das Recht auf
Datenportabilität und das Recht auf Löschung, jederzeit gewährleistet werden kann.

**Datenschutz-Folgenabschätzung**: Regelmäßige Datenschutz-Folgenabschätzungen (DSFA) gemäß Art. 35 DSGVO
sind durchzuführen, wenn die Art der Datenverarbeitung voraussichtlich ein hohes Risiko für die Rechte und
Freiheiten der betroffenen Personen birgt. Dabei sind frühzeitig Risiken zu identifizieren und geeignete
Maßnahmen zur Risikominimierung zu ergreifen.

Die Einhaltung dieser datenschutzrechtlichen Vorgaben ist fortlaufend zu überwachen und durch regelmäßige
Audits und Berichte zu kontrollieren. Anpassungen an neue rechtliche Vorgaben und Entwicklungen sind
unverzüglich umzusetzen.

### 4.5 Test- und Qualitätsmanagement

Aufgrund der angedachten Rolle in der IT-Architektur der öffentlichen Verwaltung, die der in diesem Projekt
zu pilotierende Messenger-Kommunikationskanal im Erfolgsfall zukünftig einnehmen soll, ist eine hohe
Qualität des Produktes für die Auftraggeberin von sehr hoher Priorität. Die Auftragnehmerin muss deshalb
ein Qualitätsmanagement aufbauen und umsetzen, dass eine entsprechend hohe Produktqualität sicherstellt.
Insbesondere muss die Auftragnehmerin dafür sorge tragen, dass Maßnahmen zur Qualitätssicherung,
insbesondere regelmäßige Code Reviews und kontinuierliche Tests des Codes durchgeführt werden.

### 4.6 Sicherstellung der Nachnutzbarkeit und Nachhaltigkeit

#### Einhaltung von Best-Practices

Die Auftragnehmerin setzt im Rahmen ihrer Entwicklungsprozesse transparente Versionskontrollsysteme ein (Git).
Es wird erwartet, dass der Code den Best Practices der Open-Source-Softwareentwicklung folgt,
einschließlich der Verwendung von verständlicher Dokumentation, klar definierten
Beitragsrichtlinien (Contributing Guidelines), einer Readme-Datei, sowie einer sorgfältigen Verwaltung von
Abhängigkeiten.

#### Nutzbarkeit der entwickelten Lösung ohne Einsatz von proprietären Komponenten

Auftraggeberin und Auftragnehmerin sind sich einig, dass die entwickelte Lösung aufgrund der öffentlichen
Finanzierung des Projektes und der damit einhergehenden gesellschaftlichen Verantwortung der Allgemeinheit
zur Verfügung gestellt werden soll. Die im Rahmen der Leistungserbringung entwickelte Lösung muss in einem
späteren produktiven Betrieb *vollständig* unter Freier Softwarelizenz gemäß
[Abschnitt 4.7](#47-regelungen-der-nutzungsrechte-freier-offener-softwareentwicklungsprozess) ohne
Nutzung von proprietären Drittkomponenten einsetzbar sein. Die entwickelte Lösung darf daher ausschließlich
unter Freier Softwarelizenz lizenzierte Bestandteile einsetzen. Teil der Lösung sind Komponenten (auch
mitgelieferte oder ggf. durch Cloud-Anbieter bereitgestellte Umsysteme wie Datenbankserver, Web Server,
Monitoring-Agents), die auf Schnittstellen auf Ebene des Betriebssystemkerns (weitgehend POSIX kompatibel,
konkret Mainline Linux) aufbauen. Darunterliegende Schichten wie Komponenten des eingesetzten
Betriebssystems, ggf. genutzte Hypervisor oder Firmware von eingesetzten Hardware-Komponenten sind von
dieser Regelung ausgenommen, sofern diese austauschbar und prinzipiell durch freie Alternativen ersetzt
werden können. Nicht Teil der Lösung sind Entwicklungswerkzeuge, die für den Betrieb der Lösung nicht
erforderlich sind. Die Demonstration der Skalierbarkeit, Leistungsfähigkeit und Belastbarkeit der Lösung
durch Last- und Performanztests erfolgt unter diesen Voraussetzungen.

#### Beträge zu Quellprojekten (Upstream Contributions)

Zur Aufgabe der Auftragnehmerin gehört es auch, aktiv zur Open-Source-Community beizutragen. Dies umfasst
die Einbringung von Fehlermeldungen, die Behebung von Fehlern (z.B. durch Code-Beiträge) und die Teilnahme
an Diskussionen in fremden Open-Source-Projekten, um die Sicherstellung und Erweiterung der Funktionalität
von abhängigen Projekten zu gewährleisten.

Alle Änderungen und Anpassungen, die an eingesetzten Fremd-Komponenten vorgenommen werden, werden als
Patches dem jeweiligen Ursprungsprojekt unter der jeweiligen Freien Softwarelizenz des Ursprungsprojektes
zur Verfügung gestellt. Dies gewährleistet nicht nur die Nachhaltigkeit und Wartbarkeit der Lösung,
sondern trägt auch zur Weiterentwicklung der Open-Source-Community bei und minimiert langfristige
Abhängigkeiten durch eigene Modifikationen. Die Veröffentlichung der Quellcodeänderungen erfolgt
unmittelbar, spätestens nach Abnahme zum Ende eines Sprints.

### 4.7 Regelungen der Nutzungsrechte (freier, offener Softwareentwicklungsprozess)

Abweichend von Ziffer 3.1-3.4 EVB-IT-Dienstleistungs-AGB sind zu der Sicherstellung der Nachnutzbarkeit (siehe
auch: [Ziffer 4.6](#46-sicherstellung-der-nachnutzbarkeit-und-nachhaltigkeit) vorstehend) für alle
Leistungsergebnisse folgende Nutzungsrechte und Pflichten geregelt. Die Dienstleistungen der Auftragnehmerin
müssen so erbracht werden, dass sie der Einhaltung der nachfolgenden zwingenden Vorgaben nicht entgegenstehen.

- Als Leistungsergebnisse gelten sämtliche Schöpfungen und Entwicklungsergebnisse (auch sofern sie
  vorbestehende Werke integrieren), die die Auftragnehmerin - auch durch Unterauftragnehmerinnen - für die
  Auftraggeberin erbringt. Leistungsergebnisse sind daher insbesondere alle erstellten Softwarekomponenten,
  Softwarepakete, sonstige Software, Objekt- und Quellcodes, Webseiten, Daten, Algorithmen, Datensammlungen,
  Datenbanken oder Datenauswertungen zur Verwendung in Online- oder Offline-Publikationen oder für interne
  und externe Zwecke des Auftraggebers, Textentwürfe oder Texte, Bildwerke, Grafiken, Fotos, Titel,
  Präsentationen, Vorträge, Berichte oder Dokumentationsmaterialien.
- Die Auftragnehmerin ist verpflichtet, alle Leistungsergebnisse gemäß nachfolgenden Bedingungen zu lizenzieren:

  - a. Die Auftragnehmerin verpflichtet sich, *alle* Leistungsergebnisse unter der
    [*Open-Source-Lizenz für die Europäische Union* (EUPL)](https://interoperable-europe.ec.europa.eu/collection/eupl)
    zur Verfügung zu stellen. Im Einzelfall kann die Lizenzierung auf Anweisung der Auftraggeberin in
    Textform unter einer anderen Freien Softwarelizenz oder unter der Lizenz
    [*Creative Commons Attribution Share Alike* 4.0 (CC BY-SA 4.0)](https://creativecommons.org/licenses/by-sa/4.0/)
    erfolgen. Freie Softwarelizenzen sind Lizenzen, die sowohl gemäß dem Katalog der
    [Free Software Foundation (FSF)](https://www.gnu.org/licenses/license-list.html) als auch dem Katalog der
    [Open Source Initiative (OSI)](https://opensource.org/licenses) als Freie Softwarelizenzen anerkannt sind.
    Für Software-Werke sind "Kompatible Lizenzen" nach Artikel 5 der EUPL mit Copyleft-Klausel zu bevorzugen.

  - b. *Abweichend von a) gilt*:
    **Änderungen und Anpassungen an eingesetzten Fremd-Komponenten unter Freier Software-Lizenz** werden dem
    jeweiligen Ursprungsprojekt unter der entsprechenden Freien Softwarelizenz des Ursprungsprojektes zur
    Verfügung gestellt. Sieht das Ursprungsprojekt neben der Erteilung einer Freien Softwarelizenze weitere
    Bedingungen für Beiträge vor, beispielsweise in Form eines Contributor License Agreements (nachfolgend
    zusammenfassend als CLA bezeichnet), dann erfolgen Lizenzierungen unter den zusätzlichen Bedingungen
    des CLA erst nach Freigabe des CLA durch die Auftraggeberin in Textform. Die Auftragnehmerin verpflichtet
    sich in diesem Fall zur Einhaltung des CLA. Die Auftraggeberin legt hierbei Wert auf die Zusicherung der
    dauerhaften Verfügbarkeit des Gesamtwerks unter einer Freien Softwarelizenz. Abweichend zu a) ist im CLA
    die Nennung einer der Kataloge der OSI oder der FSF ausreichend, vergleichbar mit der Regelung im
    [CLA des Signal Messengers](https://signal.org/cla/) (Punkt 2 letzter Satz, abgerufen am 2025-03-18).
    Sofern die Auftraggeberin dem CLA nicht zustimmt,  werden Änderungen in einer Kopie des Ursprungsprojekts
    ("Fork") ausschließlich unter Freier Softwarelizenz bereitgestellt. Unabhängig von den hier genannten
    Intentionen obliegt die endgültige Entscheidung über die Freigabe des CLA (auch über aktualisierte
    Varianten bereits freigegebener CLA) der Auftraggeberin.

  - c. *Abweichend von a) gilt*:
    **Konzepte, Dokumentationen, Handbücher sowie die Inhalte für das [Föderale Entwicklungsportal](https://docs.fitko.de/)**
    sind unter der Lizenz
    [*Creative Commons Attribution Share Alike 4.0* (CC BY-SA 4.0)](https://creativecommons.org/licenses/by-sa/4.0/)
    zu lizenzieren.

  - d. *Abweichend von a) gilt*: Neu erstellte **Konfigurationsdateien für Drittkomponenten** sind unter der
    Lizenz [*CC0 1.0 Universell* (CC0 1.0)](https://creativecommons.org/publicdomain/zero/1.0/deed.de) zu
    lizenzieren.

- Die Auftragnehmerin gewährleistet, dass sie befugt ist, die gewählte Lizenz zu erteilen. Sie gewährleistet
  insbesondere, dass dem dort vereinbarten Nutzungsrechteumfang keine Rechte Dritter entgegenstehen und die
  vertragsgemäße Nutzung der Leistung durch den Auftraggeber oder Dritte nicht in Patente, Lizenzen oder
  sonstige Schutzrechte Dritter eingreift. Weiterhin gewährleistet die Auftragnehmerin, dass sie mit allen
  Mitarbeiter:innen des Projektteams (einschließlich Unterauftragnehmerinnen) ausreichende urheberrechtliche
  Vereinbarungen getroffen hat, um die Nutzungsrechte der gewählten Lizenz(en) übertragen zu können.

- Die Auftragnehmerin erteilt der Auftraggeberin ein nicht ausschließliches, unentgeltliches Nutzungsrecht an
  ihren Patenten, sofern dies zur Ausübung der erteilten Rechte am Werk notwendig ist.

- Die Auftragnehmerin stellt die Lizenzkompatibilität eingesetzter Drittsoftware und/oder -werke untereinander
  und zur gewählten Lizenz sicher.

- Die Auftragnehmerin verpflichtet sich bei Selbstnutzungen der Werke zur Einhaltung der gewählten Lizenz(en).
  Insbesondere schließt dies die Erteilung von konkurrierenden, zur gewählten Lizenz inkompatiblen Lizenzenaus.

- Die Auftragnehmerin veröffentlicht unmittelbar nach Abschluss der Entwicklungsarbeiten und internen
  Qualitätssicherungsmaßnahmen (wie bspw. Reviews, automatisierten und manuellen Tests sowie statischen
  Code-Analysen), spätestens nach Abschluss eines Sprints, die Leistungsergebnisse zusammen mit den
  Urhebervermerken, Disclaimern, Lizenztexten und etwaigen weiteren Hinweisen auf den dafür vorgesehenen
  und von den Parteien vereinbarten Plattformen. Dies können insbesondere Repositorys auf openCode
  oder – im Einvernehmen mit der Auftraggeberin – andere öffentliche, Git-basierten
  Kollaborationsplattformen sein.

- Die Einräumung der Nutzungsrechte gemäß dieses
  [Abschnitts 4.7](#47-regelungen-der-nutzungsrechte-freier-offener-softwareentwicklungsprozess) ist durch
  die im Preisblatt vereinbarten Preise abgegolten.

- Etwaige Haftungs- und Gewährleistungsausschlüsse aus erteilten Lizenzen finden keine Anwendung auf das
  Vertragsverhältnis von Auftraggeberin zu Auftragnehmerin.

## 5 Projektstruktur

Die Projektdurchführung erfolgt durch ein von der Auftraggeberin und Auftragnehmerin gemeinsam gebildetes
Projektteam, wobei der überwiegende Teil der Aufwände durch die Auftragnehmerin erbracht wird. Eine
Arbeitnehmerüberlassung findet nicht statt. Das Projektteam arbeitet in agiler, offener Arbeitsweise.
Projektsprache ist Englisch oder Deutsch. Im Folgenden werden die Rahmenbedingungen der Projektstruktur
näher beschrieben.

### 5.1 Projektmanagement und Projektorganisation

#### 5.1.1 Aufgabe und Rolle der Auftraggeberin

Die FITKO unterstützt das Föderale IT-Architekturboard bei der operativen Umsetzung seiner Vorhaben. Für
die Durchführung des am 12. Juli 2023 beschlossenen Vorhabens "Pilotierung Messenger-Kommunikationskanal
für die G2C-Kommunikation" hat das Föderale IT-Architekturmanagement der FITKO die Federführung übernommen.

Die Umsetzung des Vorhabens erfolgt in enger Abstimmung mit dem Referat DV I 5 des Bundesministerium des
Innern und für Heimat (BMI) sowie den Mitgliedern des Föderalen IT-Architekturboards. Die FITKO vertritt
das Projekt nach außen und unterstützt bei der Schaffung ggf. notwendiger Rahmenbedingungen außerhalb des
Einflussbereichs des Projektes.

Das Föderale IT-Architekturmanagement der FITKO ist erste Ansprechpartnerin der Auftragnehmerin. Das Föderale
IT-Architekturmanagement der FITKO stellt im Projekt Wissen über bestehende Rahmenbedingungen der Föderalen
IT-Architektur bereit und steuert die Ausrichtung des Projektes entlang der fachlichen Bedarfe und
architektonischen Gegebenheiten.

Die Auftraggeberin stellt die Rolle des Product Owner (PO, siehe
[Abschnitt 5.1.3](#513-verantwortlichkeiten-im-projektteam)) und steuert das Projekt und die
Projektdurchführung. Projektmeetings finden gemeinsam zwischen Auftraggeberin und Auftragnehmerin statt.
Die Auftraggeberin bringt ihre technische und fachliche Expertise zu spezifischen Gegebenheiten des
öffentlichen Sektors und dem konkreten Projektumfeld in das Projekt ein.

#### 5.1.2 Agile Entwicklung

Die Pilotierung des Matrix-basierten-Kommunikationskanals für die G2C-Kommunikation erfolgt in einem Umfeld,
das eine hohe Adaptibilität an sich ändernde Anforderungen erfordert. Diese können insbesondere aus sich
ändernden Rahmenbedingungen in der IT-Landschaft der öffentlichen Verwaltung oder der inhärenten
Lernerfahrung in einem Pilotierungsprojekt herrühren. Aus den genannten Gründen erfolgt die Leistungserbringung
in iterativen Zyklen. Die Auftragnehmerin muss während der gesamten Leistungserbringung ein agiles Framework
anwenden, welches sowohl für die nötige Flexibilität sorgt, als auch der Auftragnehmerin erlaubt, die
Dimensionen Zeit, Umfang, Qualität und Kosten adäquat nach den Vorgaben der Auftraggeberin zu managen.

Die Wahl des agilen Frameworks erfolgt zum Projektstart in einvernehmlicher Entscheidung zwischen
Auftragnehmerin und Auftraggeberin. Die Auftraggeberin begrüßt es, wenn die Auftragnehmerin hierbei bereits
etablierte und funktionierende Projektmanagementpraktiken und -Prozesse anwendet.

#### 5.1.3 Verantwortlichkeiten im Projektteam

Abhängig von der Wahl des agilen Frameworks ergeben sich unterschiedliche Verantwortlichkeiten im Projektteam.
Die Auftragnehmerin teilt einzelnen Projektmitgliedern die ihr zugeteilten Verantwortlichkeiten zu und
kommuniziert diese der Auftraggeberin.

- Das Requirement Engineering erfolgt gemeinschaftlich durch Auftragnehmerin und Auftraggeberin:

  - *Anforderungsanalyse, Anforderungsdokumentation, Anforderungsvalidierung* erfolgen in geeigneter Weise
    (bspw. in Form eines Product Backlogs) *durch die Auftragnehmerin* unter Zuhilfenahme von durch die
    Auftraggeberin bereitgestellten Informationen (einschließlich der in der Leistungsbeschreibung
    enthaltenen Informationen und im Projektverlauf sowohl schriftlich, als auch mündlich überlieferter
    Informationen) sowie eigenen Erkenntnissen der Auftragnehmerin.
  - Die *Ausgestaltung von Anforderungen* im Rahmen der *Anforderungsdefinition* (Steuerung des
    Refinement-Prozess) und die *Priorisierung der Anforderungen* erfolgen *durch die Auftraggeberin*.
    Die Auftragnehmerin unterstützt hierzu den Prozess der technisch-fachlichen Anforderungsdefinition
    durch eigene Impulse.
  - Das *Stakeholder-Management gegenüber Anforderungsträger:innen* liegt im Verantwortungsbereich der
    *Auftraggeberin*.

- Die optimale Ausrichtung des Projektes an den von der Auftraggeberin definierten Projektzielen obliegt
  der *Auftragnehmerin*. Dazu zählen:

  - Erarbeitung und Kommunikation von handlungsleitenden Zielen für das Projektteam
  - Qualitätsmanagement

- Die interne Organisation des Projektteams obliegt der *Auftragnehmerin*. Dazu zählen:

  - Förderung der Anwendung des gewählten agilen Frameworks
  - Identifikation und Beseitigung von Hindernissen
  - Unterstützung der Selbstorganisation und funktionsübergreifenden Arbeitsweise des Teams
  - Sicherstellung einer offenen und transparenten Kommunikation innerhalb des Projektteams
  - Förderung der Konzentration des Projektteams auf die Erreichung der Ziele des jeweiligen Zyklus

- Die Umsetzung der ermittelten Anforderungen unter Berücksichtigung der handlungsleitenden Ziele
  für das Projektteam obliegt der *Auftragnehmerin*. Dazu zählen:

  - Erstellung der im Leistungsumfang beschriebenen Artefakte (Mockups, Lösungsarchitektur, Konzepte,
    Software)
  - Sicherstellung der Codequalität, z.B. durch manuelle Code Reviews, automatisierte Code-Analyse sowie
    automatisierte Tests
  - Erstellung und Pflege der technischen Dokumentation

- Die *Auftragnehmerin* definiert eine Person als *Team Lead*. Sie ist erste Ansprechpartner:in ggü. den
  von der Auftragnehmerin gestellten Mitgliedern des Projektteams und ggü. der Auftraggeberin. Die Rolle
  des Team Lead soll von einer:einem der Engineers abgedeckt werden (vgl.
  [Abschnitt 5.1.4](#514-anforderungen-an-das-einzusetzende-personal-und-einordnung-in-die-preiskategorien)).
  Die benannte Person hat hervorragendes Expert:innenwissen mit langjähriger (mehr als 5 Jahre)
  kontinuierlicher praktischer Erfahrung mit den Fähigkeiten:

  - Herausgehobene projektbezogene, fachliche Expertise im Team
  - Agile Projektvorgehensweisen
  - Erfahrung in der Führung von Teams
  - Umgang mit Konfliktsituationen
  - Projektentscheidungen des gesamten Teams ggü. der Auftraggeberin vertreten

- Die *Auftraggeberin* stellt die Rolle des *Product Owners*. Diese Rolle umfasst insbesondere:

  - Die Priorisierung und kontinuierliche Verwaltung des Produkt-Backlogs
  - Die aktive Teilnahme an agilen Events wie Refinements, Sprint-Planungen, Reviews, Retrospektiven
    und (nach Bedarf) Dailys, um sicherzustellen, dass die Anforderungen klar kommuniziert und die
    Projektergebnisse laufend überprüft werden
  - Die Bereitstellung von Feedback und Entscheidungen zur Unterstützung der Auftragnehmerin während des
    Entwicklungsprozesses

Die Auftraggeberin erwartet eine selbstständige, eigenverantwortliche Arbeitsweise der Mitarbeitenden der
Auftragnehmerin. Zur Förderung einer transparenten und effektiven Zusammenarbeit wird zu Projektbeginn eine
Teamvereinbarung außerhalb des Vertrags geschlossen, die folgende Aspekte regelt:

- Die Rollen und Verantwortlichkeiten aller Beteiligten,
- Turnus und Zweck regelmäßiger Projektmeetings,
- Grundsätze für die gemeinsame Entscheidungsfindung und das Tragen von Verantwortung,
- Prozesse zur Umsetzung, Qualitätssicherung und Abnahme für die zu entwickelnden Artefakte (Definition
  of Ready, Definition of Done, Mehr-Augen-Prinzip, Code-Review, PO-Review, etc.),
- Kommunikations- und Eskalationswege für den Fall von Konflikten oder Risiken,
- Maßnahmen zur Sicherstellung der Qualität und Dokumentation von Entscheidungen (insb.
  Architecture Decision Records).

Die Teamvereinbarung schafft eine klare Grundlage für das Zusammenspiel der Auftraggeberin und Auftragnehmerin
und stellt sicher, dass Entscheidungen fundiert getroffen und konsequent umgesetzt werden. Sie fördert ein
gemeinsames Verantwortungsbewusstsein und eine transparente Zusammenarbeit, bei der sowohl gemeinsame als
auch individuelle Verantwortlichkeiten klar abgegrenzt und eingehalten werden.

#### 5.1.4 Anforderungen an das einzusetzende Personal und Einordnung in die Preiskategorien

Die Auftragnehmerin muss zur Erbringung der oben beschriebenen Leistungen Personal einsetzen, welches die
im Folgenden beschriebenen Fähigkeiten (Erfahrung und Kompetenzen) des jeweiligen fachlichen Aufgabengebiets
aufweist.

Dabei erfolgt eine Zuordnung des Personals je nach Fähigkeiten jeweils zu einer der zwei Preiskategorien.
Die Stundensätze der Preiskategorien 1 und 2 sind im Preisblatt zu hinterlegen. Zur Zuordnung in die
jeweilige Preiskategorie hat die Auftragnehmerin mit ihrem Angebot im Rahmen des Personaleinsatzkonzeptes
Lebensläufe des von ihr für den Einsatz verbindlich vorgesehenen Personals eingereicht. Die Lebensläufe
enthalten aussagekräftige Informationen zu den Fähigkeiten des Personals.

Die Auftraggeberin prüft die Unterlagen vor einem Personaleinsatz und entscheidet nach den folgenden
Maßstäben unter welcher der beiden Preiskategorien das Personal abgerechnet werden kann:

- Eine Zuordnung zu Preiskategorie 1 erfolgt bei Personal, welches eine langjährige (min. 5 Jahre)
  kontinuierliche praktische Erfahrung in allen unten genannten Fähigkeiten der jeweiligen Fachausrichtung
  nachweisen kann. Bei eingeschränktem zeitlichem oder inhaltlichem Umfang der Fähigkeiten und Erfahrungen
  erfolgt eine Zuordnung zu Preiskategorie 2.

- Eine Zuordnung zu Preiskategorie 2 erfolgt bei Personal, welches eine langjährige (min. 3-5 Jahre)
  kontinuierliche praktische Erfahrung in den unten genannten Fähigkeiten der jeweiligen Fachausrichtung
  nachweisen kann.

Personal, welches weder die Anforderungen der Preiskategorie 1, noch der Preiskategorie 2 erfüllt, darf nicht
eingesetzt werden.

##### Fachausrichtung: Consultant

Beherrschung mindestens eines der folgenden Wissensgebiete mit den jeweiligen Fähigkeiten:

- Wissensgebiet Lösungsarchitektur mit folgenden Fähigkeiten:

  - Modellierungswerkzeuge und -sprachen wie Archimate, UML, C4 und BPMN
  - Praktische Fragestellungen und Herausforderungen im Design von Lösungsarchitekturen
  - Erfahrungen im Bereich der Konzeption, der Entwicklung und dem Betrieb von IT-Systemen sowie ein
    solides Verständnis aktueller Technologien und Trends in der Softwareentwicklung und im IT-Betrieb

- Wissensgebiet Konzeptionsleistungen mit Bezug zum Matrix-Standard mit folgenden Fähigkeiten:

  - Tiefgreifendes Verständnis der in der Matrix Spezifikation beschriebenen Schnittstellen und
    Datenstrukturen (Server-Server API, Client-Server API, DAG, Room State, Event Format)
  - Erfolgreiche Mitarbeit an der Erarbeitung und gegebenenfalls Umsetzung von Matrix Spec Changes (MSCs)

- Wissensgebiet Kryptographie mit folgenden Fähigkeiten:

  - Tiefgreifendes Verständnis von kryptographischen Primitiven und Protokollen aus dem im Matrix Umfeld
    und deren Interaktion (Homeserver Signing Keys, Double Ratchet, Olm + Megolm, SSSS, Key Verification
    Framework)
  - Konzeption und Analyse von neuen Protokollen, beispielsweise um Aspekte wie Nicht-Nachvollziehbarkeit
    und Vertraulichkeit bei Kommunikation, Privacy-by-Design bei Identität und Adressierbarkeit oder
    Verfügbarkeit allgemein im Matrix Ökosystem zu stärken
  - Praktische Erfahrung in der Umsetzung der Prinzipien Security-by-Design und Privacy-by-Design

- Wissensgebiet User Experience / User Interface Design / Service Design mit folgenden Fähigkeiten:

  - Erstellung von UI Mockups und Komponenten, UX Konzepten zur Gewährleistung einer intuitiven Oberfläche
    für Nutzende, sowie zur Umsetzung durch Frontend Entwickler:innen
  - Ausrichtung der Arbeit an den diversen Bedürfnissen verschiedenster Zielgruppen mit besonderem Fokus
    auf Barrierefreiheit
  - Durchführung von UX-Tests und Einholen von Nutzendenfeedback

- Wissensgebiet: Agiles Projektmanagement & Agile Softwareentwicklungsprozesse mit folgenden Fähigkeiten:

  - Steuerung agiler Softwareentwicklungsprozesse (Agile Coach)
  - Projektmanagement-Erfahrung

- Wissensgebiet: Anforderungserhebung- und Analyse mit folgenden Fähigkeiten:

  - Ermittlung, Strukturierung und Validierung von Anforderungen an das zu entwickelnde System
  - Fachliche und technische Einordnung von (ggf. unstrukturierten) Anforderungen in den Projektkontext
    und Ableitung von operationalisierbaren Aufgabenpaketen (Epics, Issues)
  - Fähigkeit zur Bewertung der technischen Machbarkeit von Anforderungen
  - Zielgerichtete fachlich-technische Bewertung von Anforderungen anhand einer definierten Zielvision
  - Erarbeitung von alternativen Handlungsoptionen auf Grundlage benannter Anforderungen

- Nach Erforderlichkeit vergleichbare Fähigkeiten und Erfahrungen in weiteren Wissensgebieten mit Projektbezug

##### Fachausrichtung: Matrix Engineer Mobile

Beherrschung folgender Fähigkeiten:

- Programmierung in den plattformspezifischen Sprachen Swift und Objective C oder Kotlin und Java,
  zusätzliche Plattformunabhängige Sprachen wie TypeScript, JavaScript, Dart oder Rust und entsprechende
  Schnittstellen (z.B. V8, React Native JavaScript Interface, Java Native Interface, WebAssembly, UniFFI)

- Verwendung von App Frameworks Swift UI, Jetpack Compose, React Native oder Flutter und entsprechende
  Ökosystemkomponenten beispielsweise für Navigation

- Verwendung von modernen Plattformschnittstellen der jeweiligen Betriebssysteme, beispielsweise für
  Caching und Storage oder Push

- Implementierung mit Matrix-Ökosystembezug, Verwendung von aktuellen, spezifikationskompatiblen SDKs
  für die Client-Server API

- Anwendung von Sicherheitsprinzipien für mobile Anwendungen durch Verwendung von Secret Storage APIs,
  TLS, Memory Safety und Process Isolation

- Arbeiten mit DevOps Ansatz, CI / CD in OCI Containerumgebungen, Emulation der Appumgebung, Release
  Management für die entsprechenden App Stores

- Arbeit an und Umsetzung von UI / UX Design (z.B. Penpot, Figma, Sketch) in Komponenten der App Frameworks
  mit Validierung durch Previews, Responsive Design

- Anwendung von Performanceprinzipien und Validierung durch Profiling, Tracing

- Entwicklung mit Fokus auf Barrierefreiheit, Verständnis für Anforderungen verschiedener Zielgruppen wie
  Verwendung eines Screenreaders und damit zusammenhängende Anforderungen für die App (Systemschnittstellen,
  UI Struktur, Navigation)

- Arbeiten in einem Git-Repository mit Open-Source Community Ansatz unter Verwendung eines Pull Request
  Workflows, Issue triaging, Atomic Commits und Dokumentation. Arbeit an Verbesserungen von Plattform
  Ökosystemkomponenten

- Einbringen in Review-Prozesse, Aufbau und kontinuierliches Abbilden von Fehlerquellen in (automatisieren)
  End-To-End und Unit-Tests. Kenntnis von Testverfahren wie Fuzzing

- Agile Softwareentwicklung mit starkem Teambezug

##### Fachausrichtung: Matrix Engineer Web

Beherrschung folgender Fähigkeiten:

- Programmierung in den plattformspezifischen Sprachen TypeScript und JavaScript, zusätzliche
  plattformunabhängige Sprachen wie Kotlin oder Rust und entsprechende Schnittstellen (z.B. WebAssembly,
  UniFFI)

- Verwendung von Web Frameworks (z.B. React, Svelte) und entsprechende Ökosystemkomponenten beispielsweise
  für State-Management

- Verwendung moderner Web APIs wie fetch, und moderner ECMAScript APIs wie Module, Promise oder Private
  Instanzfelder, weiterer Web Plattform Schnittstellen wie WebPush, IndexedDB

- Implementierung mit Matrix-Ökosystembezug, Verwendung von aktuellen, spezifikationskompatiblen SDKs
  für die Client-Server API

- Abwendung von Prinzipien der Web Sicherheit (CORS, CSP, SRI, TLS), Verständnis von Problematiken der Web
  Plattform (siehe u.a. Problemstellungen der
  [WICG Isolated Web Apps](https://github.com/WICG/isolated-web-apps) Abschnitt Motivating Use Cases)

- Arbeiten mit DevOps Ansatz, CI / CD in OCI Containerumgebungen, Release Management

- Arbeit an und Umsetzung von UI / UX Design (z.B. aus Penpot, Figma, Sketch) in Komponenten der Web
  Frameworks mit Tools wie Tailwind, Storybook; Responsive Design

- Anwendung von Performanceprinzipien wie First Contentful Paint, Validierungstools der Browser

- Entwicklung mit Fokus auf Barrierefreiheit, Verständnis für Anforderungen verschiedener Zielgruppen wie
  Verwendung eines Screenreaders und damit zusammenhängende Anforderungen für die Webseite (UI Struktur,
  Navigation)

- Arbeiten in einem Git-Repository mit Open-Source Community Ansatz unter Verwendung eines Pull Request
  Workflows, Issue triaging, Atomic Commits und Dokumentation. Arbeit an Verbesserungen von Web
  Ökosystemkomponenten

- Einbringen in Review-Prozesse, Aufbau und kontinuierliches Abbilden von Fehlerquellen in (automatisieren)
  End-To-End und Unit-Tests. Kenntnis von Testverfahren wie Fuzzing

- Agile Softwareentwicklung mit starkem Teambezug

##### Fachausrichtung: Matrix Engineer Full-Stack

Beherrschung folgender Fähigkeiten:

- Programmierung in plattformübergreifenden und Backend Sprachen wie Python, C++, Kotlin, Java, Dart,
  Rust und plattformspezifischen Sprachen wie TypeScript, JavaScript, Swift

- Konzeption und Implementierung von Schnittstellen, im Besonderen RESTful APIs mit JSON in OpenAPI

- Implementierung von Komponenten im Matrix-Ökosystem die konkret mit der Client-Server und Server-Server
  Schnittstelle interagieren. Technische Validierung, Unterstützung und Umsetzung von Änderungen an der
  Matrix-Spezifikation in den entsprechenden Komponenten (insb. Bibliotheken und SDKs)

- Umsetzung von Datenbank-, Caching- und Storageanbindungen, ebenso wie die Anknüpfung an Identitäts- und
  Authentifizierungssysteme im Backend

- Arbeiten im DevOps Bereich, CI / CD in OCI Containerumgebungen, Release Management

- Kenntnisse im Bereich Linux-Systemadministration spezifisch mit Bezug zum (Test-) Betrieb und Konfiguration
  von Softwarekomponenten aus dem Matrix Ökosystem und deren Abhängigkeiten, darunter Web Server (wie NGINX,
  Caddy), Datenbanksysteme (wie PostgreSQL) durch gängige Infrastructure as Code Tools wie Nix oder Ansible

- Anwendung von Performanceprinzipien wie Caching, Multi Threading und Sharding. Verständnis von
  Datenstrukturen mit Matrix Bezug (z.B. DAG) und wie diese sich auf Performanz und Effizienz auswirken

- Abwendung von Prinzipien der Web- und Plattformsicherheit und deren Zusammenspiel bei Netzwerkkommunikation
  (Authentifizierung und Autorisierung, SSRF, TLS, Memory Safety, Process Isolation, Timing Attacks, Input
  Validation)

- Verständnis von kryptographischen Primitiven und Protokollen im Matrix Umfeld und Erfahrung bei deren
  Anwendung in Softwarekomponenten. Besondere Sensibilisierung für Anwender:innenfehler und deren
  Sicherheitsauswirkungen, beziehungsweise Vermeidung dieser durch Ansätze wie
  [Domain Separation](https://en.wikipedia.org/wiki/Domain_separation), Einsatz von Nonces, etc.

- Arbeiten in einem Git Repository mit Open Source Community Ansatz unter Verwendung eines Pull Request
  Workflows, Issue triaging, Atomic Commits und Dokumentation. Fokus auf Verbesserung des Ökosystems durch
  Arbeit an bestehenden Komponenten in verschiedenen Bereichen

- Einbringen in Review-Prozesse, Aufbau und kontinuierliches Abbilden von Fehlerquellen in (automatisieren)
  End-To-End und Unit-Tests. Kenntnis von Testverfahren wie Fuzzing

- Agile Softwareentwicklung mit starkem Teambezug

- Wünschenswert sind zusätzliche Erfahrungen aus den Fachausrichtungen "Matrix Engineer Web" und "Matrix
  Engineer Mobile"

#### 5.1.5 Kommunikation und Zusammenarbeit

Die Auftragnehmerin organisiert eigenständig die für das von ihr gewählte agile Framework erforderlichen
Meetings in adäquater Weise und Umfang. Sie ermöglicht der Auftraggeberin, an Meetingformaten des
Projektteams teilzunehmen.

Hierzu zählen insbesondere:

- mind. ein Terminformat zur Planung des kommenden Inkrements,
- mind. ein Terminformat zur Vorstellung und Inspektion des Inkrements am Ende eines Entwicklungszyklus sowie
- regelmäßige Abstimmungstermine zur Bewertung des Projektfortschritts und der Zusammenarbeit des
  Projektteams sowie der Erarbeitung möglicher Steuerungsmaßnahmen (nach Bedarf, mindestens dreiwöchentlich)

Die Auftragnehmerin nimmt auf Aufforderung der Auftraggeberin an Austauschen mit weiteren für die
Projektdurchführung relevanten Stakeholdern teil. Falls sich hieraus widersprüchliche Anforderungen an die
Umsetzung ergeben, entscheidet die Auftraggeberin, wie hiermit umgegangen werden soll.

#### 5.1.6 Transparenz und Berichterstattung

Die Auftragnehmerin muss mit geeigneten Maßnahmen sicherstellen, dass die Auftraggeberin jederzeit einen
transparenten Überblick über die bereits erzielten Fortschritte sowie über eventuelle Roadblocks und
relevante Risiken erhält. Hierzu sind geeignete Formen und Zeiträume der Berichterstattung mit der
Auftraggeberin abzustimmen.

#### 5.1.6 Flexibilität und Anpassungsfähigkeit

Das von der Auftragnehmerin genutzte agile Projektmanagement muss es ermöglichen, flexibel auf
Veränderungen, beispielsweise hinsichtlich Anforderungen und Prioritäten zu reagieren. Die Auftragnehmerin
muss darüber hinaus der Auftraggeberin die Möglichkeit bieten, regelmäßig in den Entwicklungsprozess
eingebunden zu werden, um sicherzustellen, dass die entwickelte Software den Erwartungen entspricht und
notwendige Anpassungen frühzeitig vorgenommen werden können.

### 5.2 Mitwirkungsleistungen der Auftraggeberin

Im Rahmen der Zusammenarbeit zwischen Auftraggeberin und Auftragnehmerin werden folgende Mitwirkungspflichten
von der Auftraggeberin übernommen, um eine reibungslose Umsetzung des Projekts sicherzustellen:

#### 5.2.1 Keine Bereitstellung von Arbeitsplätzen und Infrastruktur

Ein Präsenzarbeitsplatz und Infrastruktur werden von der Auftraggeberin nicht gestellt.

#### 5.2.2 Bereitstellung von Software und Tools

Die Bereitstellung und der Betrieb benötigter Software für die Entwicklung und den Betrieb erfolgt im
Rahmen der Leistungserbringung durch die Auftragnehmerin und ist mit der vereinbarten Stundensatzvergütung
abgegolten. Dazu gehören unter anderem:

- Hardware für die Entwicklung, den Betrieb und Tests
- Integrierte Entwicklungsumgebungen (IDEs)
- Build- und Deployment-Tools
- Testing- und Qualitätssicherungs-Tools
- Code-Analyse und Schwachstellen-Scanner
- Monitoring- und Logging-Tools

Abweichend gilt:

**Projektmanagementtool**: Die Auftraggeberin stellt ein Projektmanagementtool zur Verfügung. Alternativ
kann das Projektmanagement auch auf öffentlichen Plattformen wie GitHub, GitLab oder openCode erfolgen.
Die Wahl eines alternativen Projektmanagement-Tools bedarf der vorherigen Zustimmung der Auftraggeberin in
Textform.

**Quellcode-Repository und -Verwaltung**: Alle zu erstellenden Artefakte, einschließlich Quellcode und
Dokumentation, sind bereits während der Erstellung über ein Quellcode-Repository auf einer
Entwicklungsplattform mit der Auftraggeberin zu teilen. Mögliche Entwicklungsplattformen umfassen GitHub,
GitLab oder openCode. Alle Quellcodeänderungen werden in dem von der Auftraggeberin festgelegten,
öffentlich zugänglichen Repository verwaltet. Die von der Auftraggeberin festgelegte Plattform ist
auch Vorgabe für die Nutzung der dort bereitgestellten Infrastruktur, einschließlich CI/CD-Pipelines,
Builds und automatisierter Tests. Die Wahl einer alternativen Entwicklungsplattform bedarf der vorherigen
Zustimmung der Auftraggeberin in Textform.

#### 5.2.3 Kommunikation und Zusammenarbeit

Die Auftraggeberin steht während der gesamten Projektdauer als Ansprechpartnerin für Rückfragen und
Abstimmungen zur Verfügung. Eine enge Abstimmung in allen Phasen des Projekts wird sichergestellt, um eine
zielgerichtete und erfolgreiche Leistungserbringung zu gewährleisten.

### 5.3 Zusammenarbeit mit Dritten

Die Auftragnehmerin muss auf Aufforderung der Auftraggeberin auch mit weiteren externen Partnern und
Dienstleistern, einschließlich, aber nicht ausschließlich, mit dem Bundesministerium des Innern (BMI) und
dessen Dienstleistern in diesem Projekt als Teil der zu erbringenden Leistung zusammenarbeiten. Diese
Zusammenarbeit kann technische, organisatorische oder koordinative Aufgaben umfassen, um die Erbringung
der in [Kapitel 3](#3-leistungsumfang) beschriebenen Leistungen zu unterstützen und/oder die Erreichung der
Projektziele zu fördern. Die Zusammenarbeit mit externen Partnern erfolgt dabei in enger Abstimmung mit der
Auftraggeberin, um eine konsistente und zielführende Projektumsetzung sicherzustellen.

### 5.4 Zeitplan

Der Zeitplan stellt eine Grobplanung der Auftraggeberin dar. Mit Zuschlagserteilung beginnt die initiale
Vertragslaufzeit von einem Jahr. Innerhalb der initialen Vertragslaufzeit sieht der Projektplan der
Auftraggeberin die Fertigstellung des Minimum Viable Product (MVP) entlang des Zielbildes vor. Darüber
hinausgehende Weiterentwicklungen des Zielbildes erfolgen in Form von vertraglichen Optionen.

Visuelle Darstellung des Zeitplans:

![Ein Zeitstrahl in vier Abschnitte von je 12 Monten dauer aufgeteilt. Nach dem Projektstart mit Vergabeabschluss ist der erste Abschnitt mit "Entwicklung eines MVP entlang des Zielbildes" betitelt. Die weiteren Abschnitte sind alle als "Option: Weiterentwicklung des MVP entlang des Zielbildes" beschrieben.](Leistungsbeschreibung/Abbildung%205:%20Zeitplan.png)

## 6 Rechtliche und vertragliche Rahmenbedingungen

### 6.1 Vertragslaufzeit und -optionen

Die initiale Vertragslaufzeit beträgt 12 Monate. Die in [Kapitel 3](#3-leistungsumfang) beschriebenen
Leistungen werden von der Auftraggeberin entsprechend der in
[Abschnitt 2.4](#24-auftragsgegenstand-und-laufzeit-optionen) und im Preisblatt genannten Modalitäten vergütet.

Nach Fertigstellung des Minimum Viable Product (MVP) entlang des Zielbildes hat die Auftraggeberin die
Möglichkeit, den Vertrag drei mal optional um jeweils ein Jahr zu verlängern (vgl.
[Kapitel 3.6](#36-option-weiterentwicklung-des-mvp-zur-vorbereitung-einer-möglichen-produktivsetzung)):

**Option 1**: Verlängerung der Vertragslaufzeit um 1 weiteres Jahr zur Weiterentwicklung des MVP entlang
Zielbildes und zur Vorbereitung einer möglichen Produktivsetzung

**Option 2**: Verlängerung der Vertragslaufzeit um 1 weiteres Jahr zur Weiterentwicklung des MVP entlang
Zielbildes und zur Vorbereitung einer möglichen Produktivsetzung

**Option 3**: Verlängerung der Vertragslaufzeit um 1 weiteres Jahr zur Weiterentwicklung des MVP entlang
Zielbildes und zur Vorbereitung einer möglichen Produktivsetzung

### 6.2 Vergütungsmodalitäten

#### 6.2.1 Vergütung für Leistungen

Die Vergütung erfolgt entsprechend der in [Abschnitt 2.4](#24-auftragsgegenstand-und-laufzeit-optionen)
und im Preisblatt genannten Modalitäten:

- Konzeption, Entwicklung und Pilotierung (inkl. Qualitätssicherung): Vergütung nach Aufwand gemäß der in
  Abschnitt B des Preisblattes angegeben Stundensätze

- Bereitstellung und Betrieb einer Testumgebung inkl. Support und Wartung: Pauschale Vergütung pro Monat
  gemäß des in Abschnitt A des Preisblattes angegeben Preises

Damit die Auftraggeberin die Einhaltung von Budgets überwachen kann, hat die Auftragnehmerin die
entstandenen Aufwände monatlich abzurechnen. Die Abrechnung erfolgt in Form von Leistungsnachweisen,
die konkret und im Einzelnen nachvollziehbar darlegen, welche Tätigkeiten von welchen Personen an welchen
Tagen und zu welcher Dauer erbracht wurden. Eine Nachvollziehbarkeit setzt mindestens eine kurze
Beschreibung (3-10 Worte) der erbrachten Tätigkeiten und idealerweise zusätzlich einen Verweis auf eine
bearbeitete Story (z.B. in Form der Issue-ID) voraus.

Die Auftraggeberin behält sich vor, im Rahmen der Ausführungsphase eine Obergrenze zu definieren, um das zur
Verfügung stehende Budget nicht zu überschreiten.

#### 6.2.2 Vergütung für Reisekosten

Durchgeführte Reisen bedürften der vorherigen Zustimmung der Auftraggeberin in Textform. Hierfür können die
tatsächlich entstandenen Reisekosten einschließlich eventueller Übernachtungskosten in Rechnung
gestellt werden. Es besteht Übereinstimmung darüber, dass das eingesetzte Personal der Auftragnehmerin
berechtigt ist, jeweils das Verkehrsmittel zu benutzen, das zum geringsten Zeitaufwand bei der jeweiligen
Reise führt. Die Auftragnehmerin ist zu wirtschaftlichem Handeln bei der Buchung von Reisen angehalten.
Kosten für Flüge (Economy Class) können nur erstattet werden, wenn für die Nutzung dieses Reisemittels
ausnahmsweise triftige Gründe vorliegen. Reisen mit der Bahn sind dem Flugverkehr und Kfz im Sinne der
Nachhaltigkeit vorzuziehen. Bei Reisen mit dem eigenen Kfz werden diese mit jeweils 0,50 Euro/km abgerechnet
Der zeitliche Aufwand für Reisen (Reisezeiten) kann nicht abgerechnet werden. Dies gilt nicht,
soweit während der Reise abrechenbare Tätigkeiten für die Auftraggeberin ausgeführt werden.
