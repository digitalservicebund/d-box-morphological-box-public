Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/architecture-v1.1/Zielarchitektur/Zusammenfassung/README.md
Abgerufen: 2026-11-19
Hinweis: Branch architecture-v1.1 (Stand 13.11.2026), Version 1.1 vom 06.10.2026, noch nicht vom IT-Planungsrat beschlossen. Abbildungen 2 und 4 als PNG daneben abgelegt.
---

# Föderale Zielarchitektur für Postfach- und Kommunikationslösungen

***Bauplan einer modernen Kommunikationsinfrastruktur für Privatpersonen, private Organisationen und öffentliche Stellen***

**Version 1.1** &ndash; **Stand: 06.10.2026** &ndash; **Überblick und Handlungsempfehlungen**

> Dieses Dokument ist die maschinenlesbare Textfassung des vom IT-Planungsrat im PDF-Format verabschiedeten Dokuments ["Überblick und Handlungsempfehlungen" in Version 1.0](./2025-06-26_Beschluss-2025-28_ZaPuK_Überblick-und-Handlungsempfehlungen.pdf). Bei der Überführung wurden redaktionelle Änderungen für eine bessere Lesbarkeit und Anpassung an die openCode-Struktur vorgenommen.

------

## Dokumentübersicht

Das vorliegende Dokument gibt einen Überblick über die Zielarchitektur und beschreibt abgeleitete strategische Handlungsempfehlungen. Die in der folgenden Tabelle aufgelisteten Begleitdokumente geben einen tieferen Einblick in die im vorliegenden Dokument zusammenfassend dargestellten Inhalte.

| Titel                                   | Dokumentenart       | Inhalt                                                                                                      |
| --------------------------------------- | ------------------- | ----------------------------------------------------------------------------------------------------------- |
| **Überblick und Handlungsempfehlungen** | **Klammerdokument** | **Übergreifende Zusammenfassung des Vorhabens und Darstellung strategischer Implikationen**                 |
| [Zielarchitektur](/Zielarchitektur/Architekturkonzept/README.md)           | Begleitdokument     | Fortschreibung der auf die Ziele des Vorhabens ausgerichteten Zielarchitektur                               |
| Übergreifende Transitionsstrategie      | Begleitdokument     | Transitionsstrategie zur schrittweisen Über-führung bestehender Postfachlösungen in die ZaPuK-Infrastruktur |
| Rechtsgutachten                         | Begleitdokument     | Prüfung der rechtlichen Rahmenbedingungen sowie Ableitung von Handlungsempfehlungen für die Umsetzung       |

## Nutzungsbedingungen

Die Inhalte dieses Dokumentes unterliegen der [Creative Commons Namensnennung 4.0 International Public License (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).

------

## Abstract

Der IT-Planungsrat hat mit Beschluss [B-2025/28-IT](https://www.it-planungsrat.de/beschluss/beschluss-2025-28) die Zielarchitektur für Postfach- und Kommunikationslösungen (ZaPuK) beschlossen und sie mit den Beschlüssen [B-2026/03-IT](https://www.it-planungsrat.de/beschluss/b-2026-03-it) und [B-2026/18-IT](https://www.it-planungsrat.de/beschluss/b-2026-18-it) als verbindlichen Bestandteil des Deutschland-Stacks (D-Stack) bestätigt. Auftrag der Phase II war es, den Weg von der Zielarchitektur zur Realisierung vorzubereiten. Das vorliegende Dokument fasst die Ergebnisse zusammen und leitet daraus den Entscheidungsbedarf für den IT-Planungsrat ab.

### Ergebnisse der Phase II im Überblick

- **Zielarchitektur:** Das Föderale IT-Architekturboard hat das Architekturkonzept beschlossen und die Umsetzung der im Rahmen von ZaPuK entwickelten Zielarchitektur empfohlen. Das Zielbild der Phase I bleibt unverändert und wurde um 27 Architektur­entscheidungen, insbesondere zur Entkopplung von Postfach und Authentifizierung sowie zur getrennten Betreiberschaft der Postfach-Backends, ergänzt.
- **Technische Validierung:** Das Vorhaben "Neo" hat die wesentlichen Architekturannahmen der ZaPuK gemeinsam mit den Fachverfahren "BW-Empfangsclient" (Baden-Württemberg) und "GA-Lotse" (Hessen) praktisch nachgewiesen.
- **Transition:** Für das Zentrale Bürgerpostfach der "BundID", "Mein Justizpostfach", die Postfächer von "Mein Unternehmenskonto" sowie "Mein ELSTER" liegen Transitionspläne, ein einheitliches Vier-Phasen-Modell und eine priorisierte Umsetzungsreihenfolge vor.
- **Recht:** Es wurde ein unabhängiges Rechtsgutachten angefertigt. ZaPuK ist auf Basis der bestehenden Rechtsgrundlagen grundsätzlich umsetzbar. Eine staatsvertragliche Grundlage wird empfohlen, ggf. übergreifend für den D-Stack. Zur Bündelung der Bekanntgabe- und Zustellungsregelungen im Sinne der Maßnahme 207 der Föderalen Modernisierungsagenda werden Anpassungen diverser Rechtsgrundlagen empfohlen (u. a. OZG, AO, VwVfG des Bundes und der Länder, VwZG des Bundes und der Länder, SGB X).

### Vorgehensvorschlag

In der nächsten Programmphase werden die zentralen ZaPuK-Komponenten als Basisdienst "Postfach und Interaktion" im Plattformkern des D-Stacks aufgebaut und die erforderlichen Anpassungen an den wesentlichen Bestandssystemen vorgenommen. Der Basisdienst realisiert künftig den Funktionsbaustein "Postfach und Interaktion" der Deutschland-Architektur (D-Architektur). Die Umsetzung erfolgt im Einklang mit  
[B-2026/18-IT](https://www.it-planungsrat.de/beschluss/b-2026-18-it) als Programmphase III (Etablierung der neuen Infrastruktur und "Transition & Migration") unter Federführung des Bundesministeriums für Digitales und Staatsmodernisierung (BMDS) und mit Unterstützung der FITKO. Die strategische und operative Steuerung erfolgen perspektivisch im Rahmen der D-Stack-Governance (in Entwicklung).

Die Verantwortungen für Betrieb und Entwicklung werden zwischen dem BMDS und der FITKO aufgeteilt; der zentrale Postfach-Zugang liegt dabei im Verantwortungsbereich des Bundes. Ausgenommen ist das SAFE-Verzeichnis der Justiz.

### Entscheidungsbedarf des IT-Planungsrats

Der IT-Planungsrat wird um Entscheidung zu vier Punkten gebeten: (1) Bestätigung der fortgeschriebenen Zielarchitektur, (2) verbindliche Festlegung, dass die bestehenden Postfachlösungen hin zur ZaPuK konsolidiert werden, (3) Initiierung der Programmphase III unter Federführung des BMDS und (4) Beauftragung der erforderlichen Rechtsetzungs- und Governance-Schritte. Das Föderale IT-Architekturboard hat sich in der 32. Sitzung am 15. September 2026 einstimmig für diese Empfehlungen ausgesprochen.

------

## 1. Einleitung

Mit dem Beschluss [B-2025/28-IT](https://www.it-planungsrat.de/beschluss/beschluss-2025-28) des IT-Planungsrats wurde in Phase I des Vorhabens "Föderale Zielarchitektur für Postfach- und Kommunikationslösungen" (ZaPuK) eine entsprechende Zielarchitektur entwickelt und beschlossen. Damit wurde erstmals ein gemeinsames Zielbild für die einheitliche digitale Kommunikation zwischen Privatpersonen, Organisationen und öffentlichen Stellen geschaffen. Die Ergebnisse leisten einen zentralen Beitrag zur Weiterentwicklung der D-Architektur und des D-Stacks, insbesondere für den Funktionsbaustein "Postfach und Interaktion". Mit Beschluss [B-2026/03-IT](https://www.it-planungsrat.de/beschluss/b-2026-03-it) hat der IT-Planungsrat die ZaPuK als verbindlichen Teil des D-Stacks beschlossen und diese Verbindlichkeit mit Beschluss [B-2026/18-IT](https://www.it-planungsrat.de/beschluss/b-2026-18-it) nochmals bestätigt. Im Zielbild steht Privatpersonen und Organisationen ein zentrales Postfach für die Kommunikation mit Verwaltung und Justiz zur Verfügung.

Vor diesem Hintergrund bereitete Phase II den Übergang von der Zielarchitektur zur Umsetzung vor. Schwerpunkte waren die Konkretisierung der Architektur, die technische Validierung zentraler Architekturentscheidungen sowie die Entwicklung von tragfähigen Transitionsplänen für die Postfachlösungen "Zentrales Bürgerpostfach" der BundID (ZBP), "Mein Justizpostfach" (MJP), "Mein Unternehmenskonto" (MUK) mit seinen Postfächern OZG-PLUS-Postfach und Postfach 2.0 sowie das Postfach von "Mein ELSTER". Ziel war die Erarbeitung eines tragfähigen, schrittweisen Migrationspfads als Ausgangspunkt für die schrittweise Einbindung weiterer Postfachlösungen hin zu einer konsolidierten föderalen Postfach- und Kommunikationsinfrastruktur.

Um die Voraussetzungen für eine bundesweit abgestimmte Umsetzung zu schaffen, wurden darüber hinaus die wesentlichen Governance-, Betriebs- und Rechtsfragen geklärt und die Rahmenbedingungen für die nächste Programmphase III festgelegt. Das ZaPuK-Finanzierungskonzept ([B-2025/28-IT Pkt. 6](https://www.it-planungsrat.de/beschluss/beschluss-2025-28)) wird als Finanzierungskonzept des Basisdienstes "Postfach/Benachrichtigung" im Deutschland-Stack erst zur 52. Sitzung entsprechend [B-2026/18-IT Pkt. 4](https://www.it-planungsrat.de/beschluss/b-2026-18-it) vorgelegt.

## 2. Vorgehensweise und Methodik

Phase II konkretisierte die in Phase I entwickelte ZaPuK fachlich, technisch, organisatorisch und rechtlich und bereitete deren Umsetzung im D-Stack vor. Die Arbeiten erfolgten in einem iterativen Prozess und in enger Abstimmung mit Bund, Ländern, bestehenden Postfachlösungen und weiteren relevanten Akteur:innen wie Sozialversicherungsträgern, deren technische und fachliche Anforderungen und Rückmeldungen kontinuierlich berücksichtigt wurden. Die Ergebnisse wurden fortlaufend in föderalen Architekturgremien abgestimmt. Die eingerichtete Arbeitsgruppe des Föderalen IT-Architekturboards zu ZaPuK (FIT-AG) begleitete die fachliche Ausarbeitung in regelmäßigen Sitzungen. Zentrale Architekturentscheidungen wurden außerdem im Föderalen IT-Architekturboard (FIT-AB) vorgestellt und diskutiert.

Zu den weiteren Governance-Strukturen, die die strategische Steuerung und Qualitätssicherung der Arbeiten sicherstellten, zählte der Projekt-Lenkungskreis mit Vertreter:innen des BMDS, der Freien und Hansestadt Hamburg, des Landes Sachsen-Anhalt sowie der FITKO. Hier wurden unter anderem strategische Leitlinien abgestimmt und zentrale Weichenstellungen für Zielbild und Transition getroffen. Zur fachlichen Validierung und Stakeholder-Einbindung wurde außerdem ein Sounding Board eingerichtet, in dem zentrale Fragestellungen und Zwischenergebnisse mit den relevanten Akteur:innen, unter anderem von den federführenden Stellen und bestehenden Postfach- und Kommunikationslösungen, diskutiert und validiert wurden.

Die Ergebnisse der Phase II beruhen auf einem kontinuierlichen Zusammenspiel aus Architekturarbeit, technischer Validierung, Konsultation der Stakeholder sowie der gemeinsamen Entwicklung von Transitions- und Umsetzungsoptionen und fokussieren sich auf folgende Schwerpunkte:

### Fortschreibung der Zielarchitektur

Ausgehend von den Ergebnissen aus Phase I wurde die föderale Zielarchitektur fortentwickelt und bis auf Ebene wesentlicher Architekturentscheidungen konkretisiert. Dabei wurden Designprinzipien, Zielzustände, Zuständigkeiten sowie Anforderungen an Interoperabilität, Sicherheit und Betriebsfähigkeit weiter ausgearbeitet.

### Transitions- und Migrationsplanung für Bestandslösungen

Einen Schwerpunkt von Phase II bildete die Betrachtung bestehender Postfachlösungen. Im Rahmen strukturierter Transitionsgespräche wurden die Ausgangssituationen von "BundID/ZBP", "Mein Justizpostfach", "Mein Unternehmenskonto" inklusive OZG-PLUS-Postfach und Postfach 2.0 sowie "Mein ELSTER" analysiert. Die Gespräche dienten dazu, technische Abhängigkeiten, fachliche Anforderungen, Migrationshemmnisse sowie mögliche Integrations- und Übergangsszenarien zu identifizieren. Auf dieser Grundlage wurden individuelle Transitionspfade entwickelt und mit der übergreifenden Zielarchitektur in Einklang gebracht.

### Technische Validierung zentraler Architekturannahmen

Wesentliche Bestandteile der Zielarchitektur wurden durch Erkenntnisse aus dem Projekt Neo validiert. Dadurch konnten technische Risiken reduziert, Lösungsansätze praktisch überprüft und die Umsetzbarkeit zentraler Architekturbausteine nachgewiesen werden.

### Governance-, Betriebs- und Finanzierungsbetrachtung

Für neu zu schaffende Komponenten wurden Verantwortlichkeiten für Entwicklung, Betrieb und Steuerung definiert. Zudem wurden die erforderlichen Maßnahmen für bestehende  
(Um-)Systeme analysiert, Abschätzungen zu einmaligen und laufenden Kosten vorgenommen und bausteinspezifische Bedarfe an einer künftigen D-Stack-Governance identifiziert.

### Rechtliche und regulatorische Bewertung

Die Zielarchitektur wurde hinsichtlich ihrer Anforderungen im Kontext des Verfassungs-,  
Verwaltungs-, Datenschutz-, IT-Sicherheits- und Telekommunikationsrechts bewertet. Ergänzend wurden gesetzgeberische Handlungsbedarfe identifiziert.

## 3. Ergebnisse Phase II

### 3.1 Weiterentwicklung der Zielarchitektur

Mit [Version 1.1 der Zielarchitektur](/Zielarchitektur/Architekturkonzept/README.md) wurde die in Phase I entwickelte Zielarchitektur fortgeschrieben, um 27 weitere [Architekturentscheidungen](/Zielarchitektur/Architekturentscheidungen/) (Architecture Decision Records, ADR) ergänzt und so erweitert, dass die Kernfunktionalitäten der betrachteten Bestandslösungen abgedeckt werden. Als Basisdienst "Postfach" des D-Stacks zur Realisierung des Funktionsbausteins "Postfach und Interaktion" der D-Architektur soll der Postfach-Zugang perspektivisch in das Bundesportal und die Deutschland-App integriert werden, die im D-Stack den Funktionsbaustein "Zentrales Nutzenden-Frontend" realisieren. Die Integration in das bestehende FIT-Connect-SDK wird empfohlen und ist erprobt worden. Antragstellung und Folgekommunikation werden über dieselbe Anbindung abgewickelt, sodass eine parallele Anbindung von Onlinediensten und Fachverfahren an mehrere Infrastrukturen entfällt.

Unverändert aus Phase I bleibt das Zielbild einer föderierten Client-Server-Architektur mit einer Matrix-basierten Ende-zu-Ende-verschlüsselten (End-to-End Encryption, E2EE) Postfach- und Kommunikationsinfrastruktur zwischen Privatpersonen, privaten Organisationen, öffentlichen Stellen und der Justiz.

#### Identität, Betreibertrennung und Adressierung

In Phase II wurde die Trennung von Identität, Authentifizierung und Postfach weiter konkretisiert. Während das OZG derzeit Nutzendenkonten mit integrierten Postfächern vorsieht, wird das Postfach in der Zielarchitektur als eigenständige Komponente mit eigener Adresse ausgestaltet und kann dadurch unabhängig vom verwendeten Identifizierungsmittel genutzt werden. Somit bleibt der Zugang zum Postfach unabhängig vom jeweils genutzten Authentifizierungsverfahren; zugleich wird eine nahtlose Einbindung zukünftiger Identifizierungsmittel wie der EUDI-Wallet ermöglicht. BundID und MUK bleiben als Identitätsanbieter bestehen und übernehmen weiterhin die Authentifizierung, während die Postfachfunktion organisatorisch und technisch entkoppelt wird. Der daraus folgende rechtliche Anpassungsbedarf in OZG und AO ist in [Abschnitt 3.6](#36-rechtsgutachten-und-maßnahme-207-föma) beschrieben.

#### Verschlüsselung und Nachweise

Die E2EE als zentrales Architekturprinzip stellt sicher, dass die Klartexte der Kommunikation ausschließlich im eigenen Machtbereich (und somit auch nicht durch Betreiber der Infrastruktur) ausgelesen werden können. Auch Malware-Prüfung und Veraktung erfolgen im Verantwortungsbereich der jeweiligen Empfänger:innen.

Ferner können zugunsten von Nachvollziehbarkeit sowie Rechts- und Beweissicherheit digitale Nachweise und Quittungen (z. B. Bereitstellungs-, Abruf- und Lesebestätigungen) ausgestellt werden.

#### Organisationen im Verbund

Die Zielarchitektur unterstützt Funktionspostfächer für Organisationen und ermöglicht unterschiedliche Betriebsmodelle – von der Nutzung zentral bereitgestellter Komponenten bis hin zum vollständig souveränen Eigenbetrieb. Einheitliche Anschlussbedingungen stellen dabei Interoperabilität und einen konsistenten Betrieb im föderalen Verbund sicher. Rollen- und Vertretungsregelungen werden durch die jeweiligen Organisationen selbst verwaltet. Ein zentrales Vollmachtsregister ist nicht Bestandteil der Infrastruktur.

### 3.2 Zentrale Komponenten der Zielarchitektur und empfohlene Verantwortlichkeiten

Für die Gruppen Privatpersonen, private Organisationen, öffentliche Stellen und Justiz wird jeweils ein zentrales Postfach-Backend bereitgestellt, das die verschlüsselten Nachrichten speichert und die Anbindung von Postfach-Zugängen erlaubt. Das Backend der Justiz wird mittels eines Protokollkonverters an die OSCI-basierte Infrastruktur des elektronischen Rechtsverkehrs unter Beibehaltung der E2EE angeschlossen. Das Onboarding der Organisationen und Backends wird über ein Self-Service-Portal (SSP) abgewickelt.

Backends werden über die Aufnahme in eine Föderationsliste im Verbund zugelassen. Die Adressen der Organisationen werden in einem verbundweit abfragbaren Register geführt. Das Zusammenspiel der zentralen Komponenten der Zielarchitektur wird in Abbildung 1 dargestellt.

Für Entwicklung, Betrieb und Steuerung der zentralen ZaPuK-Komponenten stellt Abbildung 2 einen Vorschlag zur Verantwortungsverteilung dar. Neue Organisationen sind hierfür nicht erforderlich; die Verantwortlichkeiten werden bestehenden Produkten und Institutionen des Bundes, der FITKO und der Justiz zugeordnet. Die Begründungen dieser Empfehlungen werden im Architekturkonzept genauer ausgeführt.

![Übersicht über das Zusammenspiel der Postfach-Backends mit dem Frontend, Authentifierzungslösungen und weiteren Komponenten und Umsystemen.](images/image1.png)

> _Abbildung 1: Zentrale Komponenten der Zielarchitektur_

![Empfohlene Verortung von Verantwortlichkeiten in Entwicklung, Betrieb und Steuerung zentraler IT-Komponenten der ZaPuK.](images/image2.png)

> _Abbildung 2: Empfohlene Verortung von Verantwortlichkeiten in Entwicklung, Betrieb und Steuerung zentraler IT-Komponenten der ZaPuK_

Zentraler Postfach-Zugang: Der zentrale Postfach-Zugang soll in Verantwortung des BMDS entwickelt und betrieben werden, gegebenenfalls delegiert an eine nachgeordnete Behörde oder einen Dienstleister. Er soll in das Bundesportal und die Deutschland-App integriert werden, die den Funktionsbaustein "Zentrales Nutzenden-Frontend" realisieren (vgl. [B-2026/03-IT](https://www.it-planungsrat.de/beschluss/b-2026-03-it)), und ersetzt die Postfach-Frontends von BundID, MJP und MUK.

Backends: Die Postfach-Backends für Privatpersonen, private Organisationen, öffentliche Stellen und Organisationen der Justiz sollen im FITKO-Produktmanagement durch FIT-Connect entwickelt werden. Das Backend für Privatpersonen soll wie der zentrale Postfach-Zugang vom BMDS betrieben werden, auch hier gegebenenfalls delegiert. Die Backends für Organisationen sollen von FIT-Connect im FITKO-Produktmanagement betrieben werden. Eine derartige Aufteilung des Betriebs der Backends stellt den Schutz von Kommunikationsmetadaten sicher. Gleichzeitig bleiben die Backends über einen zentralen Postfach-Zugang und einen gemeinsamen Adressraum als Gesamtinfrastruktur verbunden.

SSP, SDK und Adressregister: SSP und SDK für alle Organisationen sowie das Adressregister für private Organisationen und öffentliche Stellen sollen in Entwicklung und Betrieb ebenfalls im FITKO-Produktmanagement liegen, künftig als Teil einer föderalen Rollout- und Anbindungsinfrastruktur, sofern der FIT-Connect-Produktschnitt dafür angepasst wird. Dadurch wird sichergestellt, dass Technologien und Rollout-Prozesse nur einmalig aufgebaut werden müssen. Für Organisationen der Justiz soll das bestehende SAFE-Register nachgenutzt werden, das in Entwicklung, Betrieb und Steuerung bei der Bund-Länder-Kommission für Informationstechnik in der Justiz (BLK Justiz) bleibt. Privatpersonen werden nicht in einem Adressregister gelistet, sondern kommunizieren über bereichsspezifische, pseudonyme Adressen. Dadurch wird verhindert, dass ihre Vorgänge über Verwaltungsbereiche hinweg verknüpft werden können. Eine Kontaktaufnahme bei Privatpersonen durch Behörden erfolgt nur, wenn die Adresse aus einem bestehenden Vorgang bekannt ist oder zuvor im Rahmen einer Authentisierung bei einem Onlinedienst bereitgestellt wurde.

Governance: Die zentralen Komponenten sollen in den Governance-Strukturen des D-Stacks (vgl. [B-2026/18-IT](https://www.it-planungsrat.de/beschluss/b-2026-18-it)) und der D-Architektur (vgl. [B-2026/17-IT](https://www.it-planungsrat.de/beschluss/b-2026-17-it)) gesteuert werden. Für die von der Zielarchitektur betroffenen Bausteine sollen keine zusätzlichen Governance-Strukturen aufgebaut werden. Es wird ein operatives Produktmanagementmodell für alle Basisdienste des D-Stacks vorgeschlagen, bis zu dessen Ausgestaltung vorläufig ein methodisch einheitliches Produktmanagement mit klaren Entscheidungskompetenzen und Eskalationswegen empfohlen wird. Die Festlegung der Verantwortlichkeiten für Entwicklung und Betrieb obliegt dieser Governance auf Grundlage der jeweils gültigen Fassung des Architekturkonzepts.

### 3.3 Technische Validierung durch das Vorhaben "Neo"

Die technische Validierung der ZaPuK erfolgte im FIT-AB-Vorhaben "Pilotierung Matrix-basierter Kommunikationskanal für die G2C-Kommunikation" ("Neo"). Während die Zielarchitektur alle Gruppen von Kommunikationsteilnehmenden in den Blick nimmt, lag der Fokus von Neo zunächst auf der Kommunikation zwischen Privatpersonen und öffentlichen Stellen inklusive Organisationen der Justiz. Die im Neo-Vorhaben entwickelte, auf den Vorgaben der ZaPuK basierende Infrastruktur ermöglicht es Herstellenden von Fachverfahren und Fachbehörden, in OZG-Leistungen einen medienbruchfreien, sicheren und einfach zu integrierenden Kommunikationskanal zu realisieren. Die Verfügbarkeit eines solchen einfach einzubindenden Kommunikationskanals zahlt auf die größeren Vorhaben der Prozessautomatisierung und der Ende-zu-Ende-Digitalisierung in Fachbehörden ein.

Gemeinsam mit dem [BW-Empfangsclient (BW-EC)](https://wm.baden-wuerttemberg.de/de/service/kommunikation-und-medien/pressemitteilung/pid/neuer-bw-empfangsclient-treibt-digitalisierung-der-verwaltung-voran) des Landes Baden-Württemberg und dem Verfahren [GA-Lotse](https://ga-lotse.de/) des Gesundheitsamts der Stadt Frankfurt am Main wurden Matrix-basierte Infrastrukturkomponenten zur einfach nutzbaren und sicheren Kommunikation zwischen Privatpersonen und Fachbehörden erfolgreich getestet.[^1] Im Rahmen von Neo wurden außerdem zentrale Bausteine der Zielarchitektur prototypisch umgesetzt. Hierzu zählen insbesondere ein Web-Client und eine iOS-App als Grundlage für einen künftigen zentralen Postfach-Zugang, Matrix-basierte Backends für verschiedene Teilnehmendengruppen sowie Erweiterungen des FIT-Connect-SDK zur einfachen Integration in Fachverfahren und Onlinedienste.[^2][^3] Die Ergebnisse liefern damit einen praktischen Nachweis der Umsetzbarkeit der ZaPuK und schaffen eine belastbare Grundlage für die Produktisierung zentraler Komponenten in der nächsten Programmphase.

### 3.4 Transitionsstrategie und Transitionsplanung für wesentliche Bestandslösungen

Die Transitionsplanung schafft einen gemeinsamen Handlungsrahmen für die schrittweise Überführung der bestehenden Postfachlösungen in eine konsolidierte Postfach- und Kommunikationsinfrastruktur. Ein einheitlicher Stichtagswechsel ist nicht vorgesehen. Stattdessen erfolgt die Transition in Programmphase III schrittweise über gemeinsame Zielkomponenten, lösungsspezifische Migrationspfade und einen kontrollierten Parallelbetrieb. Zur Sicherstellung der Anschlussfähigkeit bestehender Verfahren sind übergangsweise technische Brückenlösungen wie Protokollkonverter und Notification-Services vorgesehen. Dabei bleibt eine bestehende E2EE jederzeit erhalten. Der Erfolg der Transition hängt vor allem von verbindlichen Grundsatzentscheidungen der zuständigen Stellen zur Nutzung der ZaPuK, einer produktionsreifen Betriebsbasis und der schrittweisen Umstellung der Fachverfahren ab. Die hierfür erforderlichen Entscheidungen und ihre Zuständigkeiten sind in der Transitionsstrategie (Begleitdokument 02) näher beschrieben.

Neben einer übergreifenden Transitionsstrategie wurden für die in ZaPuK Phase II wesentlichen Bestandslösungen ZBP/BundID, MJP, MUK mit seinen Postfächern OZG-PLUS und Postfach 2.0 sowie Mein ELSTER individuelle Transitionspfade mit definierten Phasen, Maßnahmen, Verantwortlichkeiten und erwartbaren Ergebnissen entwickelt. Eine Ausnahme bildet Mein ELSTER, für das aufgrund fachlicher Besonderheiten eine dauerhafte technische Anbindung im Parallelbetrieb vorgesehen ist, die über gesonderte Entscheidungen zu Transitionsarchitekturen (TADRs) beschrieben wird.

Darüber hinaus werden notwendige Anpassungen im D-Stack, insbesondere beim Zentralen Nutzenden-Frontend (Bundesportal, D-App) und bei FIT-Connect, peripher eingeplant und koordiniert.

#### 3.4.1 Steuerungslogik der Transition

Die Bestandslösungen durchlaufen ein einheitliches Vier-Phasen-Modell, dessen Phasen nicht zwingend gleichzeitig durchlaufen werden müssen (siehe Abbildung 3). Entsprechend ist vorgesehen, dass die Phasen fachlich und technisch aufeinander aufbauen, sich aber auch zeitlich überlappen können. Programmweite Quality Gates synchronisieren die Entwicklung gemeinsamer Komponenten mit den lösungsspezifischen Transitionsprojekten.

![Übersicht der vier Transitionsphasen, jeweils mit Laufzeit, Inhalten und Ziel.](images/image3.png)

> _Abbildung 3: Transitionsphasen_

Phase A bildet die Grundlage für die organisatorische Planung und spätere Umsetzung der Transitionsprojekte. Phase B dient der Transitionsvorbereitung im Sinne der Schaffung einer Betriebsumgebung, zentraler Front- und Backendkomponenten, Qualitätssicherungsprozessen und der Vorbereitung der anstehenden Migration. Phase C bildet die Datenmigration, Routing-Switches, den kontrollierten Parallelbetrieb und die Umstellung der Fachverfahren ab. Phase D bildet den Abschluss der Transitionsprojekte und die eigentliche Dekommissionierung von Legacy-Komponenten ab. Im Ergebnis erfolgt der Abschluss der Fachverfahrensumstellung, die Überführung in Archivzustände und die Außerbetriebnahme redundanter IT.

#### 3.4.2 Transitionspfade der Bestandslösungen

Die gemeinsame Zielarchitektur wird auf Ebene der Bestandslösungen unterschiedlich erreicht. Maßgeblich sind unter anderem die technische Kopplung, Migrationsfähigkeit der Datenbestände, die angeschlossenen Fachverfahren und weitere technische Aspekte (z. B. Anforderungen an E2EE). Grundlegende Prämisse ist jedoch, dass die Konsolidierung der bestehenden Postfachlösungen innerhalb der ZaPuK dem Zielbild entspricht – auch wenn übergangsweise Ausbaustufen nur näherungsweise Konformität aufweisen können. Nachfolgend werden die Transitionspfade je Bestandslösung dargelegt.

##### ZBP/BundID – Erste Schritte zur zentralen Infrastruktur

Das ZBP wird vollständig in das Matrix-basierte Backend für Privatpersonen überführt. Das zentrale Postfach-Frontend ersetzt den bisherigen Postfach-Zugang. Die BundID bleibt als Authentifizierungskomponente bestehen. Die bestehende ZBP-API wird für eine Übergangszeit über eine ZBP-Matrix-Bridge weitergeführt. Nachrichten, Statusmonitoring- und Metadaten sollen nach erfolgreichen Migrationstests überführt werden. Parallel werden Bundesportal und Fachverfahren schrittweise auf Matrix-basierte Schnittstellen umgestellt. Nach erfolgreicher Umstellung kann die Bridge entfallen.

##### MJP – Funktionelle Ablösung bei Fortbestand von EGVP

MJP wird durch den zentralen Postfach-Zugang, das Backend für Privatpersonen und das Backend für Organisationen der Justiz mit Protokollkonverter ersetzt. Vorhandene Nachrichten werden in das Backend für Privatpersonen migriert. Die EGVP-Infrastruktur bleibt unverändert bestehen. Der OSCI-Matrix-Protokollkonverter vermittelt in beide Richtungen, ohne die E2EE aufzubrechen. Dafür wird der zentrale Postfach-Zugang um die Ver- und Entschlüsselung der OSCI-Nachrichten einschließlich der Schlüsselverwaltung erweitert. Nach erfolgreicher Umstellung und stabilem Praxisbetrieb können redundante Komponenten abgeschaltet werden.

##### MUK – Getrennte Übergänge für OZG-PLUS und Postfach 2.0

Der Inhalt des Transitionsplans für MUK, die konkrete Terminierung und verbindliche Festlegungen befinden sich aktuell (Stand: 10/2026) noch in Abstimmung zwischen dem Produktmanagement der FITKO, StMD Bayern, dem BayLfSt und der Freien Hansestadt Bremen (FHB) als Betreiber:innen des MUK bzw. der MUK-Postfächer und dem Programm ZaPuK.

**Empfehlung:** MUK bleibt die zentrale Authentifizierungskomponente für Organisationen. Die bisherigen Postfach-Front- und Backends werden funktionell neu in ZaPuK abgebildet. Für Organisationen sind insbesondere Funktionspostfach-, Rollen-, Berechtigungs- und Vertretungsfunktionen sowie die Adressverwaltung im SSP notwendig.

Das OZG-PLUS-Postfach kann funktionell direkt in ZaPuK integriert werden. Allerdings muss eine serverseitige Migration aufgrund der E2EE-Nachrichtenbestände unterbleiben – stattdessen wird eine Downloadfunktion innerhalb eines befristeten Read-only-Systems durch Organisationen vorgeschlagen. Nach erfolgreichem Routing-Switch können redundante Komponenten abgeschaltet werden.

Das Postfach 2.0 wird dreistufig integriert: Zunächst erfolgt die Schaffung eines Notification-Dienstes mitsamt Absprunglink ins Altsystem. In einer weiteren Ausbaustufe wird die direkte Zustellung mittels Messaging-Bridge implementiert. Im letzten Schritt erfolgt die Matrix-native direkte Zustellung innerhalb von ZaPuK. Analog zum OZG-PLUS-Postfach müssen die Nachrichtenbestände in einem Archivsystem zur Verfügung gestellt werden – bis Altsysteme dekommissioniert werden.

##### Mein ELSTER – technische Anbindung bei dauerhaftem Parallelbetrieb

Der Inhalt der TADRs für Mein ELSTER befindet sich aktuell (Stand: 10/2026) noch in Abstimmung zwischen dem BayLfSt und Programm ZaPuK.

**Empfehlung:** Das Postfach innerhalb von Mein ELSTER ist eng mit datenbank- und formularbasierter Fachlogik verbunden und kann daher nicht vollständig aus dem Fachportal herausgelöst werden. Stattdessen erfolgt zunächst eine fachliche Analyse der Postfachinhalte mit dem Ziel, die für eine Zustellung im zentralen Postfach relevanten Nachrichtentypen zu ermitteln. Somit wird eine schrittweise Anbindung möglich. Zunächst wird für die Bereitstellung klassifizierter Nachrichten eine Bridge implementiert. Perspektivisch soll die Bereitstellung Matrix-nativ erfolgen. Die technische Anbindung erfordert jedoch einen dauerhaften Parallelbetrieb der ZaPuK- und ELSTER-seitigen Infrastruktur.

#### 3.4.3 Empfohlene Transitionsreihenfolge

Grundlage für die Transition ist die Schaffung von Verbindlichkeit zur Konsolidierung der Postfachlösungen innerhalb von ZaPuK. Diese ist eine Grundvoraussetzung für die anschließende Entwicklung und Pilotierung von Komponenten, die Durchführung von Migrationen und die geordnete Dekommissionierung. Die nachfolgenden Transitionsschritte sind als Priorisierungslogik zu verstehen – einzelne Phasen und lösungsspezifische Vorhaben können sich überlappen. Nach Schaffung einer verbindlichen Grundlage (1) erfolgt die gemeinsame Entwicklung der zentralen Komponenten (unter anderem Postfachclient, Backends, Bridges/Protokollkonverter, SSP etc.) (2). Sobald diese technische Basis geschaffen wurde, können Pilotierungen (hier für Privatpersonen) gestartet und die Produktivität und Stabilität der Infrastruktur bewiesen werden (3). Bei erfolgreichem Quality Gate wird mit der ersten Migrationswelle von Bestandsdaten begonnen (4). Nach Abschluss der Migrationswelle für Privatpersonen erfolgt die Umstellung und Integration für Organisationen (5) sowie die technische Anbindung von Mein ELSTER (6) (Ausbaustufen 1-3). Nach erfolgreichem Wirkbetrieb und entsprechenden Übergangszeiten (z. B. für Read-only-Archive) kann im letzten Schritt die Abschaltung von Legacy-Systemen erfolgen (7).

### 3.5 Finanzierungskonzept

_Das ZaPuK-Finanzierungskonzept ([B-2025/28-IT Pkt. 6](https://www.it-planungsrat.de/beschluss/beschluss-2025-28)) wird als Finanzierungskonzept des Basisdienstes "Postfach/Benachrichtigung" im Deutschland-Stack erst zur 52. Sitzung entsprechend [B-2026/18-IT Pkt. 4](https://www.it-planungsrat.de/beschluss/b-2026-18-it) vorgelegt._

### 3.6 Rechtsgutachten und Maßnahme 207 FöMA

Ziel des Gutachtens war die rechtliche Bewertung der Machbarkeit und Ausgestaltung einer konsolidierten, föderal einheitlichen Postfach- und Kommunikationsarchitektur für öffentliche Stellen in Deutschland auf Grundlage der Zielarchitektur Postfach und Kommunikationslösungen (vgl. [B-2025/28-IT](https://www.it-planungsrat.de/beschluss/beschluss-2025-28)). Im Fokus standen die verfassungsrechtlichen, IT-rechtlichen, verwaltungsrechtlichen und verwaltungsverfahrensrechtlichen Grundlagen. Jenseits der allgemeinen Leistungsverwaltung und deren Verwaltungsleistungen im Sinne des OZG wurden dabei auch sektorspezifische Anforderungen und Rahmenbedingungen in Bereichen wie Justiz, Öffentliches Gesundheitswesen, Schulen und Leistungsträger wie die Bundesagentur für Arbeit und Rentenversicherung beleuchtet.

Das Gutachten zeigt unterschiedliche Möglichkeiten und Handlungsempfehlungen auf, die sehr detailliert dargelegt werden. Aus den verschiedenen Handlungsempfehlungen ergeben sich Rahmenansätze, welche bereits vorhanden sind und erweitert bzw. ausgedehnt werden können. Einige grundsätzliche rechtliche Ausführungen können sich evtl. über ZaPuK hinaus auf föderale IT-Vorhaben und insbesondere den D-Stack übertragen lassen.

> Hinweis: Der nachfolgende Abschnitt wurde durch die Kanzleien Eichler Kern Klein und PLANIT-Legal als Zusammenfassung ihres im Auftrag der FITKO erstellten unabhängigen Rechtsgutachtens verfasst.

>>>
Aus rechtlicher Sicht ist ZaPuK in Gestalt der fortgeschriebenen Zielarchitektur (Bezug: Zielarchitektur (Version 1.1) vom 8. September 2026) grundsätzlich realisierbar und in seinen Grundentscheidungen bereits weitgehend auf die relevanten verfassungs-, verwaltungsverfahrens-, datenschutz-, IT-sicherheits- und telekommunikationsrechtlichen Anforderungen abgestimmt. Im Einzelnen:

Eine umfassende verfassungsrechtliche Ermächtigungsgrundlage, auf deren Grundlage sich ZaPuK einschließlich Entwicklung, Bereitstellung, Betrieb und Mitnutzung von ZaPuK-Komponenten realisieren ließe, existiert bislang nicht. Eine entsprechende Ergänzung des Art. 91c GG – wie durch Maßnahme 201 der föderalen Modernisierungsagenda vorgesehen – wäre im Interesse von Rechtssicherheit und -klarheit wünschenswert. ZaPuK kann jedoch nach gegenwärtigem Stand auch auf der Grundlage einer Kombination verschiedener verfassungsrechtlicher Rechtsgrundlagen realisiert werden: Art. 91c Abs. 5 GG ermöglicht die erforderlichen Gesetzesanpassungen zur Etablierung der Postfachstruktur als Zugang zu Verwaltungsleistungen von Bund, Ländern und Gemeinden. Nach Art. 87 Abs. 3 GG kann der Bund z. B. den Betrieb (einschl. Finanzierung) des zentralen Backends für Privatpersonen an sich ziehen. Zentrale Software-Komponenten können auf Basis von Art. 86 GG entwickelt werden. Für die Zusammenführung der Beiträge von Bund und Ländern und die Regelung von Verwaltungs- und Finanzierungsanteilen empfiehlt sich ein Staatsvertrag – separat für ZaPuK oder auch übergreifend für den D-Stack – auf Basis von Art. 91c Abs. 1 und 2 GG, der zudem die Delegation bestimmter Zuständigkeiten über den IT-PLR auf die FITKO erlaubt.

Eine elektronische Kommunikation über ZaPuK setzt voraus, dass Empfänger:innen hierfür jedenfalls konkludent einen Zugang eröffnet haben. Umgesetzt wird dies in Bezug auf Privatpersonen insbesondere dadurch, dass proaktive Kontaktaufnahmen nur bei bereits vorliegender, aus einem früheren Vorgang abgeleiteter Adresse zugelassen werden und andernfalls eine vorherige Authentisierung verlangt wird.

Für die elektronische Bekanntgabe von Verwaltungsakten sehen die verwaltungsverfahrensrechtlichen Regelungen nach aktueller Gesetzeslage unterschiedliche Bekanntgabe- und Übermittlungsmodelle vor. Die vorgesehenen Systemfunktionen von ZaPuK (insb. automatische serverseitige Bereitstellungsbestätigung, Benachrichtigung des Beteiligten über Bereitstellung, bei Bedarf clientseitige Abrufbestätigung) können die daraus folgenden Anforderungen an, die durch Behörden in Zweifelsfällen zu erbringenden Nachweise grundsätzlich abbilden. Um das Risiko einer unsicheren Bekanntgabe weiter zu reduzieren, ist die Infrastruktur so auszugestalten, dass Ver- bzw. Entschlüsselungsfehler verhindert oder jedenfalls revisionssicher dokumentiert werden. Mit Blick auf die elektronische Zustellung nach [§ 5 Abs. 5 VwZG](https://www.gesetze-im-internet.de/vwzg_2005/__5.html) sieht ZaPuK eine aktive Lesebestätigung und deren Erweiterbarkeit zum elektronischen Empfangsbekenntnis vor.

Zu empfehlen ist – wie in Maßnahme 207 FöMA vorgesehen – eine zentrale Bündelung von Bekanntgabe- und Zustellungsregelungen im Verwaltungsverfahrensrecht, anknüpfend an ein einheitliches Bekanntgabeereignis für die bereichsübergreifende Verwendung der ZaPuK-Postfächer. ZaPuK könnte im Zuge dessen als "sicherer Übermittlungsweg" etabliert werden. Anpassungsbedarf besteht zudem hinsichtlich der Entkopplung von Postfach und Nutzendenkonto (§§ [2](https://www.gesetze-im-internet.de/ozg/__2.html), [9 OZG](https://www.gesetze-im-internet.de/ozg/__9.html) sowie z. B. [§ 122a Abs. 5 AO](https://www.gesetze-im-internet.de/ao_1977/__122a.html)). Für die Regelung der für das ZaPuK-Postfach vorgesehenen Identifizierungs- und Authentifizierungswege empfiehlt sich eine Anknüpfung an die im Gesetzesentwurf zum Digitale Identitätengesetz enthaltene Neufassung des [§ 3 OZG](https://www.gesetze-im-internet.de/ozg/__3.html).

Die Architektur von ZaPuK steht grundsätzlich im Einklang mit den Wertungen und Schutzzwecken des Datenschutz-, IT-Sicherheits- und Telekommunikationsrechts. Sie berücksichtigt und priorisiert die Datensparsamkeit und die klassischen Schutzziele der IT-Sicherheit. Sie verfolgt zudem einen domänenagnostischen Ansatz, der sich – bei je nach Rechtsnatur und Zweck der betroffenen Einheiten unterschiedlich strengen Anforderungen – am höchsten Schutzniveau orientiert. Zugleich eröffnet sie Organisationen Wahl- und Konfigurationsmöglichkeiten, die diesen eine eigenverantwortliche Erfüllung der ihnen jeweils obliegenden Pflichten ermöglicht und besondere fachrechtliche Anforderungen erfüllbar macht. Hierzu zählt die Wahl zwischen der Minimalanbindung an ein zentrales Backend, dem delegierten Betrieb (über ein zentrales Backend) oder dem souveränen Betrieb mit eigenem Postfach-Backend.

Das für die ZaPuK-Beteiligten maßgebliche Datenschutzrecht ist unionsrechtlich geprägt durch die DSGVO sowie die JI-Richtlinie (jeweils ergänzt bzw. umgesetzt durch Bundes- und Landesrecht). Für die Funktion von ZaPuK als öffentlich zugänglicher Kommunikationsdienst ist zudem die Umsetzung der ePrivacy-RL durch das TDDDG vorrangig (Art. 95 DSGVO). Die E2EE kann die Anwendbarkeit des Datenschutzrechts auf die Verarbeitung von Inhaltsdaten durch Backend-Betreiber:innen, die nur auf das Chiffrat zugreifen können, ausschließen. Für die Metadaten ist von voller Anwendbarkeit auszugehen.

Die datenschutzrechtliche Verantwortlichkeit innerhalb der föderierten Mehr-Betreiber-Infrastruktur von ZaPuK ist differenziert nach Rolle, Komponente und Verarbeitungsschritt zu beurteilen. Mit Blick auf die Abgrenzungsprobleme zwischen gemeinsamer und getrennter Verantwortlichkeit beim Backend-Betrieb ist eine Regelung durch Rechtsvorschrift – wie nach [§ 8a Abs. 4 OZG](https://www.gesetze-im-internet.de/ozg/__8a.html) – ratsam.

Die Zielarchitektur setzt die Datenschutzgrundsätze (Art. 5 Abs. 1 DSGVO) um und ermöglicht den Beteiligten die Nachweisführung (Rechenschaftspflicht). Teilnehmende Organisationen müssen für die den Kommunikationsvorgängen vor- und nachgelagerten Verarbeitungen innerorganisatorische Datenschutzkonzepte vorhalten (z. B. Berechtigungs- und Vertretungsregelungen; Löschfristen). Sie müssen ggf. auch die Einhaltung spezifischen Datenschutzrechts (Steuergeheimnis, Sozialgeheimnis) sicherstellen und werden hierzu durch die Wahl- und Konfigurationsmöglichkeiten von ZaPuK befähigt. Die Grundentscheidungen der Zielarchitektur – wie die E2EE von Inhaltsdaten und flankierende Maßnahmen zum Schutz von Metadaten (z. B. bereichsspezifische Pseudonyme und die Trennung der zentralen Backends für Bürger:innen, Organisationen, öffentliche Stellen und Justiz) – tragen der Datensicherheit Rechnung (vgl. Art. 32 DSGVO).

Für ZaPuK gelten die Anforderungen an die IT-Sicherheit im Portalverbund ([§ 5 OZG](https://www.gesetze-im-internet.de/ozg/__5.html), [ITSiV-PV](https://www.gesetze-im-internet.de/itsiv-pv/BJNR001800022.html)), einschließlich angemessener Maßnahmen nach dem Stand der Technik mit Verweis auf die Technischen Richtlinien des BSI. Aufgrund des anzunehmenden hohen bzw. sehr hohen Schutzbedarfs sind hohe Standards und regelmäßige Prüfungen vorgeschrieben. Die ITSiV-PV beruht dabei auf einem abgestuften Ansatz (unmittelbar und mittelbar angebundene Komponenten), der zentrale Backends stärker bindet. Private Organisationen können zu angemessenen Sicherheitsanforderungen durch die Anschlussbedingungen verpflichtet werden.

Für den Betrieb der zentralen Backends sind die Anforderungen aus der NIS-2-Umsetzung zu beachten, die allerdings durch das TK-Recht verdrängt werden. Die Anforderungen spiegeln sich weitgehend in den Architekturentscheidungen (vgl. [§ 30 BSIG](https://www.gesetze-im-internet.de/bsig_2025/__30.html)).

ZaPuK ist ein nummernunabhängiger, interpersoneller Telekommunikationsdienst, der dem Fernmeldegeheimnis unterliegt ([§ 3 TDDDG](https://www.gesetze-im-internet.de/ttdsg/__3.html)). Es greifen die abgestuften Sicherheitspflichten nach [§ 165 TKG](https://www.gesetze-im-internet.de/tkg_2021/__165.html) mit hohen Anforderungen an die zentralen Backend-Betreiber:innen und grundlegenden Anforderungen an dezentrale Backends. Die Architekturentscheidungen erscheinen damit vereinbar. Insbesondere ist die E2EE auch unter den Bedingungen von Telekommunikationsüberwachung möglich, da die Ausleitungspflicht auf die Daten in der vorliegenden Form beschränkt ist.
>>>

## 4. Implikationen und Handlungsempfehlungen

Nachfolgend werden die Implikationen der ZaPuK sowie das empfohlene Vorgehen zu deren Realisierung in Phase III des Umsetzungsprogramms (Transition & Migration) dargelegt.

Das FIT-AB hat dem IT-Planungsrat in seiner 32. Sitzung per Beschluss den Aufbau einer föderal einheitlichen Postfach- und Kommunikationsinfrastruktur zur Realisierung des D-Architektur-Funktionsbausteins "Postfach und Interaktion" auf Basis der ZaPuK sowie deren Umsetzung unter Koordination des Bundes einstimmig empfohlen (vgl. Anhang A für den vollständigen Beschlusstext).

### 4.1 Implikationen und Handlungsempfehlungen in der föderalen IT-Architektur

Mit der Realisierung der ZaPuK gehen grundlegende Veränderungen in der föderalen IT-Architektur einher. Um die in Phase I formulierten Wirkungsziele (1) "Hürden für den Anschluss abbauen", (2) "Wirtschaftlichen Betrieb durch Konsolidierung fördern", (3) "Förderung von Ende-zu-Ende-Sicherheit", (4) "Nutzendenerfahrung verbessern" sowie (5) "Zukunftsfähige Weiterentwicklung auf dem Stand der Technik ermöglichen" zu erreichen, wird im Einzelnen die Umsetzung der nachfolgenden Aspekte empfohlen.

- **Realisierung im Deutschland-Stack:** Die zentral bereitgestellten Komponenten der ZaPuK werden im Rahmen des Deutschland-Stacks aufgebaut und als Basisdienst "Postfach/Benachrichtigung" im Plattformkern des Deutschland-Stacks bereitgestellt (vgl. [B-2026/18-IT](https://www.it-planungsrat.de/beschluss/b-2026-18-it), [B-2026/03-IT](https://www.it-planungsrat.de/beschluss/b-2026-03-it)).

  → **Unterstützte Ziele:** (1), (2), (3), (4), (5)

- **Entkopplung von Authentifizierung und Postfach:** Postfach, Authentifizierung und Nutzenden-Frontend werden gemäß D-Architektur entkoppelt und als separate Basisdienste im Plattformkern des D-Stacks angeboten; BundID und MUK bleiben Identitätsanbieter.

  → **Unterstützte Ziele:** (2), (5)

- FIT-Connect als tragendes Produkt: Softwareentwicklung, Betrieb der Backend-Komponenten, Rollout und Anbindung liegen im Produktmanagement des IT-Planungsrates bei der FITKO – über die erprobten Anbindungsstrukturen (SSP, SDK, Entwicklungsportal).

  → **Unterstützte Ziele:** (1), (2), (4), (5)

- **Nutzung eines einheitlichen SDKs:** Nach einmaliger Anbindung ermöglichen Fachverfahren und Unternehmensanwendungen Antragstellung und vollständige Verfahrenskommunikation über ein SDK.

  → **Unterstützte Ziele:** (1), (2), (3), (5)

- **Transition der Bestandslösungen:** ZBP, MJP und MUK werden gemäß Transitionsplänen angepasst; Onlinedienste, Fachverfahren und Fachportale stellen schrittweise auf die Zielarchitektur um.

→ **Unterstützte Ziele:** (1), (2), (3), (4), (5)

- **Konsequente Umsetzung von Ende-zu-Ende-Verschlüsselung:** Malware-Prüfung und Veraktung erfolgen als Konsequenz künftig in den Systemen der öffentlichen Stellen und ihrer IT-Dienstleister.

→ **Unterstützte Ziele:** (3), (5)

- **Synergien durch breite Anwendung und Standardisierung:** Synergien mit Gesundheits- und Sozialwesen sowie der Justiz heben; Matrix-Erweiterungen in die Standardisierung einbringen, Komponenten quelloffen auf openCode entwickeln.

→ **Unterstützte Ziele:** (1), (2), (5)

- **Kongruenz mit anderen Funktionsbausteinen und Umsystemen:** Für die Umsetzung der ZaPuK notwendigen Basisdienste wie Identitätslösungen (z.B. EUDI-Wallet, Berufsidentitäten, PKI-Modernisierung) und andere Umsysteme müssen fortlaufend bedarfsgerecht weiterentwickelt werden, und vice versa.

→ **Unterstützte Ziele:** (1), (5)

### 4.2 Empfohlenes Vorgehen für Phase III: Transition und Migration

Nach den Vorarbeiten in den Programmphasen I (Konzeption der Zielarchitektur) und II (Transitionsplanung & Validierung) widmet sich die dritte Programmphase (Transition & Migration) der konkreten operativen Umsetzung und Transition. In ihrem Rahmen soll die in den vorangegangenen Programmphasen erarbeitete und abgestimmte Zielarchitektur technisch realisiert, produktiv bereitgestellt und schrittweise zur gemeinsamen Infrastruktur für die digitale Kommunikation zwischen Privatpersonen, öffentlichen Stellen, privaten Organisationen und Organisationen der Justiz ausgebaut werden. Ziele von Programmphase III sind neben der Detaillierung und Umsetzung der in Phase II ausgearbeiteten Transitionsplanung (Begleitdokument 02) und der hierfür notwendigen Implementierung der abgestimmten Architektur die folgenden:

- Eine zentrale Postfach- und Kommunikationsinfrastruktur mit Anschlussmöglichkeiten steht bereit.
- Der Postfach-Zugang ist über das zentrale Nutzenden-Frontend des D-Stacks (Bundesportal, Deutschland-App) möglich.
- Ein aus Nutzendensicht einheitliches und ZaPuK-konformes Postfach steht im D-Stack für Privatpersonen bereit.
- Ein aus Nutzendensicht einheitliches und ZaPuK-konformes Postfach steht für Organisationen bereit.
- Die Bestandslösungen MJP und ZBP sind in die neue Infrastruktur konsolidiert.
- Die Postfachfunktionalitäten von MUK sind ausgegliedert und in die neue Infrastruktur konsolidiert.
- Klassifizierte Nachrichten oder Benachrichtigungen für das ELSTER-Fachportal werden in der neuen Infrastruktur bereitgestellt.
- Die organisatorischen Voraussetzungen für Weiterentwicklung und Betrieb der Infrastruktur sind geschaffen.

Zur konkreten Erreichung dieser Ziele wird die Initiierung von Programmphase III unter der Federführung des BMDS vorgeschlagen. Phase III wird zur Würdigung inhaltlicher Abhängigkeiten in zwei überlappende Unterphasen A "Zentrale Komponenten bereitstellen" und B "Wirkung erzeugen für Bürger:innen & Organisationen" aufgeteilt. Ein Programmhandbuch wird an das BMDS übergeben. Abbildung 4 verdeutlicht durch die Einordnung in den zentralen Phasenplan, dass Programmphase III sich in sieben Arbeitspakete gliedert.

![Übersicht der 5 Phasen des ZaPuK-Programms in einem Zeitstrahl, inkl. der jeweiligen Inhalte. Phase 3 ist dabei in zwei Phasen unterteilt. Phase A zur Bereitstellung der zentralen Komponenten und Phase B zur Transition von Bestandslösungen.](images/image4.png)

> _Abbildung 4: Phasenplan des Programms ZaPuK_

In Phase 3A "Zentrale Komponenten bereitstellen" werden zentrale Strukturen und Komponenten als Basisdienst "Postfach/Benachrichtigung" des Deutschland-Stacks technisch und organisatorisch aufgebaut. Sie ist Basis für die Transitionen im Rahmen der Unterphase 3B:

- Arbeitspaket 3A.1 ("Querschnitt") dient als beratende Instanz zu verschiedenen Themen wie Produktaufbau, Architektur, föderales IT-Ökosystem und Recht.
- Die Arbeitspakete 3A.2 ("Etablierung Backend") und 3A.3 "Etablierung Frontend" setzen den Großteil der zentral bereitgestellten Backend- und Frontendkomponenten um. Dies erfolgt auf Basis der durch das Projekt Neo erarbeiteten Ergebnisse.
- Nach hinreichend vollständiger Fertigstellung der Komponenten für die operative Umsetzung der jeweiligen Transition beginnen die Arbeitspakete 3B.1 bis 3B.4 der Phase 3B mit der Umsetzung der jeweiligen Transitionspläne. Hierbei ist herauszustellen, dass die Wirkung der einzelnen Transitionen für Bürger und Unternehmen durch die Produktivsetzung der jeweiligen Lösungen bereits vor Abschluss aller Transitionen, also sukzessive, erfolgen wird.
- Zudem ist wie in den bisherigen Phasen im Rahmen des Programmmanagements ein zentrales Stakeholdermanagement zu installieren, dass die adäquate Einbindung relevanter Stakeholder in die Umsetzung von Programmphase III sicherstellt. Insbesondere sind Kommunikationsstrategien zu entwickeln, die über die Nutzungsmöglichkeiten der ZaPuK für Bürger:innen und Organisationen informieren.

Die nachfolgenden Programmphasen IV (Laufender Betrieb) und V (Folgeprojekte & Skalierung) haben unter anderem den perspektivischen flächendeckenden Rollout bei den Fachbehörden sowie die flächendeckende Integration von Fachportalen zum Gegenstand.

Das konkrete Zielbild der Organisation von Programmphase III ist in Abbildung 5 dargestellt. Die sieben Arbeitspakete werden durch das im BMDS verortete Programmmanagement mit übergeordneten Steuerungs- und Controllingstrukturen sowie einem programmweiten Stakeholdermanagement versehen. Phase III wird gesteuert durch einen Steuerungskreis, der sich wie in Phase II aus BMDS, FITKO und Ländervertreter:innen zusammensetzt. Als unterstützendes Gremium dient erneut ein Sounding Board. Der IT-Planungsrat bzw. die Abteilungsleiterrunde steht als übergeordnete auftraggebende Instanz über dem Steuerungskreis und wird durch die Arbeitsgruppe des Funktionsbausteins "Postfach und Interaktion" im Rahmen des D-Stacks beraten. Bei Entscheidungen der Arbeitsgruppe, die die föderale Bebauung betreffen, wird nach wie vor das FIT-AB eingebunden.

![Vorgeschlagenes Organigramm für Phase 3.](images/image5.png)

> _Abbildung 5: Organisation von Programmphase III_

### 4.3 Empfehlungen für die Governance: ZaPuK in Deutschland-Architektur und D-Stack

Die Governance der ZaPuK ist zweistufig auszugestalten. Für die Laufzeit der Programmphase III gilt eine Programmgovernance mit definierten Gremien, Lenkungs- und Eskalationswegen entsprechend dem Programmhandbuch. Für den anschließenden Regelbetrieb ist eine dauerhafte Produktgovernance für den Funktionsbaustein "Postfach und Interaktion" erforderlich, die sich an den etablierten Steuerungsmechanismen des D-Stacks und am FITKO-Produktmanagement-Modell orientiert. Die Abgrenzung zur Governance der übrigen Funktionsbausteine des D-Stacks ist vor Beginn der Phase III zu klären.

Die Zielarchitektur realisiert den Funktionsbaustein "Postfach und Interaktion" (POI) der Deutschland-Architektur und wirkt sich auf die Bausteine "Datenaustausch" (DAU), "Authentifizierung" und "Zentrales Nutzenden-Frontend" aus, die in der D-Architektur derzeit spezifiziert werden (vgl. [B-2026/04-IT](https://www.it-planungsrat.de/beschluss/b-2026-04-it)). Der Postfach-Zugang wird in Bundesportal und Deutschland-App integriert. Die Deutsche Verwaltungscloud ist als Betriebsplattform vorgesehen. Die Identitätslösungen BundID und MUK dienen der Authentifizierung. Mit dem Baustein DAU, der durch das Produkt FIT-Connect realisiert wird, gibt es sehr große Überschneidungen. Fachverfahren, die mittels FIT-Connect Anträge empfangen, können über dieselbe Anbindung die Folgekommunikation über das Postfach führen. Das Konzept setzt deshalb auf das SSP und SDK von FIT-Connect und verortet Entwicklung und Betrieb der Organisations-Backends dort ([Abschnitt 3.2](#32-zentrale-komponenten-der-zielarchitektur-und-empfohlene-verantwortlichkeiten)).

Die Funktionsbausteine DAU und POI hängen von denselben Umsystemen ab. Adressierung und Schlüsselaustausch setzen Identitäten voraus, die für Mitarbeitende öffentlicher Stellen noch nicht festgelegt sind. Auch die PKI für die Systeme der Behörden ist noch nicht entschieden ([Abschnitt 4.1](#41-implikationen-und-handlungsempfehlungen-in-der-föderalen-it-architektur)).

Aus den genannten Gründen wird deshalb eine gemeinsame Governance für DAU und POI empfohlen. Sie hält die Anbindung der Fachverfahren einheitlich, legt gemeinsame Protokolle und Schnittstellen fest und gewährleistet so die Interoperabilität. Dazu gehört auch die Entwicklung einer gemeinsamen Basis beider Bausteine. Zu ihr zählen die Registrierung mit ihren Prozessen, ein Verzeichnis der angeschlossenen Stellen, Zertifikate und Vertrauenslisten sowie ein Standard für die Nachrichtenübermittlung, der im D-Stack bisher fehlt. Den Rahmen dafür hat der IT-Planungsrat gesetzt, indem er die fachliche Verantwortung für DAU und POI dem Bund mit Unterstützung der FITKO zugewiesen hat (vgl. [B-2026/18-IT](https://www.it-planungsrat.de/beschluss/b-2026-18-it)).

Synergien zu weiteren Bausteinen des D-Stacks sind bei der Umsetzung konsequent zu nutzen: das zentrale Nutzenden-Frontend für die Oberfläche des Postfach-Zugangs, die Deutsche Verwaltungscloud als Betriebsschicht, der Funktionsbaustein Datenaustausch für die Anbindung der Fachverfahren, sowie der Funktionsbaustein Authentifizierung für die Identifizierung der Nutzenden.

Das neu entstehende Digital Center Deutschland Stack (DCDS) wird der Umsetzungsmuskel für D-Stack Produkte in alleiniger Verantwortung des Bundes. Die FITKO erfüllt diese Rolle bereits für Produkte, die der IT-Planungsrat zum D-Stack beisteuert. Somit bündeln DCDS und die FITKO im Auftrag des IT-Planungsrats die Wirksamkeit für die Nachnutzung von Basisdiensten auf allen föderalen Ebenen. Gemeinsam stärken sie die technische Leistungsfähigkeit für einen modernen Staat. Für die Umsetzung der ZaPuK bietet das DCDS damit einen geeigneten Rahmen, um die Zusammenarbeit zwischen Bund, FITKO und weiteren beteiligten Akteur:innen zu bündeln sowie Abhängigkeiten zu benachbarten D-Stack-Bausteinen frühzeitig zu koordinieren.

## 5. Glossar und Abkürzungen

Die jeweils aktuellen Versionen befinden sich auf der Plattform openCode:

- [Glossar](/Glossar.md)
- [Abkürzungsverzeichnis](/Abkürzungen.md)

## Anhang A: Beschluss des FIT-AB zur ZaPuK-Umsetzung

Befassung in der 32. Sitzung des Föderalen IT-Architekturboards vom 15.09.2026

Beschluss gefasst mit: Ja-Stimmen: 14, Nein-Stimmen: 0, Enthaltungen: 0

1. _Das FIT-AB empfiehlt den Aufbau einer föderal einheitlichen Postfach- und Kommunikationsinfrastruktur zur Realisierung des Deutschland-Architektur-Funktionsbausteins ‚Postfach und Interaktion' auf Basis der ZaPuK (Zielarchitektur für Postfach- und Kommunikationslösungen) und Vorarbeiten._
2. _Das FIT-AB empfiehlt das weitere Vorgehen auf Grundlage der von ZaPuK vorgeschlagenen Programmphase III (Transition & Migration) im Rahmen des D-Stacks unter Koordination des Bundes auf Basis der Arbeitsergebnisse aus Phase I und II sowie dem Neo-Vorhaben inkl._
   1. _der Realisierung des zentralen Postfach-Frontends im zentralen Nutzenden-Frontend des Deutschland-Stacks (Bundesportal, Deutschland-App);_
   2. _des Aufbaus zentraler Backend-Komponenten für Privatpersonen und Organisationen auf Basis des Matrix-Standards in voneinander unabhängigen Betriebsorganisationen;_
   3. _der bedarfsgerechten Weiterentwicklung des FIT-Connect-Self-Service-Portals hin zu einer föderalen Rollout- und Anbindungsinfrastruktur sowie;_
   4. _der Anpassung der Bestandskomponenten BundID (Zentrales Bürgerpostfach), Mein Justizpostfach, Mein Unternehmenskonto (Postfach 2.0, OZG-PLUS-Postfach) und des ELSTER-Fachportals auf Grundlage der ZaPuK und der jeweiligen Transitionspläne._
3. _Das FIT-AB bekräftigt die dringende Notwendigkeit der bedarfsgerechten Weiterentwicklung der für die Umsetzung der ZaPuK notwendigen Identitätslösungen (insb. V-PKI, eID, vgl. Handlungsempfehlungen aus [B-2025/28-IT](https://www.it-planungsrat.de/beschluss/beschluss-2025-28))._
4. _Das FIT-AB empfiehlt die Vertiefung der Zusammenarbeit_
   1. _mit weiteren Domänen des öffentlichen Sektors (z. B. dem Gesundheits- und Sozialwesen), um die Anwendung der auf Basis von ZaPuK entwickelten Konzepte und Lösungen in deren jeweiligen Zuständigkeitsbereichen zu fördern,_
   2. _des IT-Planungsrates mit dem E-Justice-Rat sowie_
   3. _mit verwandten Vorhaben in der EU zwecks europäischer Interoperabilität._
5. _Das FIT-AB empfiehlt dem IT-Planungsrat die nachhaltige Förderung des offenen 'Matrix'-Ökosystems inklusive der Beteiligung in den Governance-Strukturen der Matrix-Foundation, der Weiterentwicklung der Matrix-Spezifikation sowie die Ent- und Weiterentwicklung von gemeinsam genutzten quelloffenen Softwarekomponenten, auch in Zusammenarbeit mit anderen EU-Staaten und der Wirtschaft zur ressourceneffizienten Entwicklung im offenen Zusammenarbeitsmodell (Open Source)._

[^1]: Eine Aufzeichnung der Live-Demos mit den Verfahren GA-Lotse und BW-EC ist [auf openCode veröffentlicht](https://gitlab.opencode.de/fitko/matrix-g2x/#live-demos).
[^2]: Informationen aus der Präsentation im FIT-AB, in der u. a. auf das Vorgehen, die Ergebnisse und die gewonnenen Erkenntnisse eingegangen wird, finden sich auf [openCode](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/raw/main/Slides/2026-06-23-FIT-AB-Neo-Vorhabensabschluss-und-weiteres-Vorgehen.pdf?inline=true).
[^3]: Es wurde ein [40-sekündiges Erklärvideo](https://media.fitko.net/zapuk/Wie-funktioniert-Matrix.mp4) zur Funktionsweise von Matrix erstellt. Zudem wurde das Vorhaben [im letzten Blog-Post](https://matrix.org/blog/2026/09/04/this-week-in-matrix-2026-09-04/) der Matrix.org-Foundation ("This Week in Matrix") im englischsprachigen Videoformat "Matrix Live" gefeatured.
