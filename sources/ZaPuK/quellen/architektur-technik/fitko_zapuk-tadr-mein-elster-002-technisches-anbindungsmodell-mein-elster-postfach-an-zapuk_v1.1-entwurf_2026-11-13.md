<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/ef99118c0607dd7fb11a005593569a36a0f84343/Zielarchitektur/Transitionen/Architekturentscheidungen/TADR-MEIN-ELSTER-002-Technisches-Anbindungsmodell-Mein-ELSTER-Postfach-an-ZaPuK.md -->
<!-- Branch: architecture-v1.1, Commit ef99118c06 (2026-11-13); Pfad im Repo: Zielarchitektur/Transitionen/Architekturentscheidungen/TADR-MEIN-ELSTER-002-Technisches-Anbindungsmodell-Mein-ELSTER-Postfach-an-ZaPuK.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 3bf7ad0138); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# TADR-MEIN-ELSTER-002: Technisches Anbindungsmodell "Mein ELSTER"-Postfach an ZaPuK

> **Status:** ✔️ Operativ entschieden | **Datum:** 25.09.2026

## Kontext und Problembeschreibung

Nachrichten des ELSTER-Postfachs entstehen in den Systemen der Finanzverwaltung und liegen dort unverschlüsselt vor, bevor sie für den Abruf durch die empfangende Person verschlüsselt abgelegt werden. Eine Anbindung an den ZaPuK-Verbund kann an dieser Stelle ansetzen. Eine Migration der Altbestände an Nachrichten in eine andere Postfachlösung ist dennoch nicht ohne Weiteres umsetzbar. Zudem werden über das Postfach Nachrichten bereitgestellt, die die Fachlogik des Portals "Mein ELSTER" eng an das Postfach binden. Voraussetzung der hier getroffenen Entscheidung ist somit zunächst die Wahl der Option B in [TADR-MEIN-ELSTER-001](./TADR-MEIN-ELSTER-001-Zuordnung-von-ELSTER-Datenarten-zum-zentralen-ZaPuK-Postfach.md), die eine unterschiedliche Bereitstellung für maschinell und menschlich ausgewertete Nachrichten vorsieht. Gegenstand dieses Transition Architecture Decision Record (TADR) sind ausschließlich die danach für die externe Zustellung vorgesehenen Datenarten. Die maschinell verarbeiteten Datenarten verbleiben unabhängig von der gewählten Option bei ELSTER.

## Entscheidungstreiber

- Klartext nicht durch ZaPuK-Betreiber einsehbar ([SIC_VER_ÜBE_026](/Konsolidierte-Anforderungen/README.md#thema-sicherheit), [SIC_VER_ÜBE_032](/Konsolidierte-Anforderungen/README.md#thema-sicherheit), [ADR-0010](/Zielarchitektur/Architekturentscheidungen/ADR-0010-Herstellung-von-Interoperabilität-mit-fremden-Systemen-außerhalb-des-Geltungsbereiches-der-Zielarchitektur.md), [ADR-0011](/Zielarchitektur/Architekturentscheidungen/ADR-0011-Kommunikationsschicht.md))
- kein dauerhafter Parallelbetrieb von durch Menschen verwendeten Postfächern

## Betrachtete Optionen

Im Folgenden werden unterschiedliche Möglichkeiten zur Integration der ELSTER-Kommunikation in ZaPuK aufgezeigt und kurz diskutiert. Die Reihenfolge geht von der Option mit der geringsten zur Option mit der höchsten Integrationstiefe.

### Option A: Benachrichtigung mit Absprung

Ein ELSTER-Notification-Dienst übermittelt Benachrichtigungen in das zentrale ZaPuK-Postfach. Diese Benachrichtigungen enthalten Metadaten und einen Absprunglink zur Nachricht im ELSTER-Postfach bei "Mein ELSTER". Der Inhalt der Nachrichten verbleibt im ELSTER-Postfach.

- Gut: minimaler Eingriff in bestehende Lösung
- Gut: Benachrichtigung wird in das zentrale Postfach übermittelt. ZaPuK-Komponenten benötigen keine Zugangsdaten, um Nachrichten aus dem ELSTER-Postfach abzuholen.
- Gut: Die geplanten mobilen Clients der ZaPuK erlauben eine einfache Umsetzung von Push-Notifications.
- Schlecht: Durch den Absprung ins ELSTER-Portal muss sich eine Bürger:in ggf. neu authentifizieren und es entsteht ein Bruch in der UX.
- Schlecht: Bescheide und andere behördliche Mitteilungen befinden sich weiter in unterschiedlichen Postfächern der Bürger:in.
- Schlecht: Die Umsetzung verursacht größere Aufwände, v. a. für die Adressierung, und bietet keinen Nutzen gegenüber der Benachrichtigung, die ELSTER Bereits umsetzt.

### Option B: Protokollkonverter

Die Lösung über einen Protokollkonverter würde zwischen den Transportprotokollen des ELSTER-Postfachs und ZaPuKs vermitteln. Auch wenn bei ELSTER die Nachrichten beim Eingang unverschlüsselt vorliegen, wird hier eine Lösung skizziert, die die Verschlüsselung nach der Speicherung aufrechterhält.

Ein eigenes Transportprotokoll verwendet ELSTER für den Akt der Zustellung nicht. Die Basis des Postfachs bei ELSTER ist eine Datenbankapplikation. Diese Applikation könnte erweitert werden, um über Matrix die ZaPuK-Infrastruktur anzubinden. Bei einer solchen Umsetzung wäre die Integration des Kryptostacks im Frontend des ZaPuK-Postfachzugangs notwendig.

- Schlecht: Die privaten Schlüssel des ELSTER-Postfachs müssten durch die Nutzenden nun auch dem ZaPuK-Postfachzugang bekannt gegeben werden.
- Schlecht: Die technische Umsetzung im ZaPuK-Postfachzugang müsste einen weiteren Kryptostack implementieren.
- Schlecht: Die Lösung scheint unnötig komplex, da die Daten unverschlüsselt bei ELSTER eingehen und keine Protokolle konvertiert werden, sondern lediglich eine DB-Applikation erweitert werden muss. Die passende Lösung ist eine Bridge (Option C).

### Option C: Zustell-Bridge in der ELSTER-Domäne

Die Bridge kann die bei ELSTER unverschlüsselt vorliegenden Nachrichten entgegennehmen und bei passender Datenart als Matrix-Client im zentralen ZaPuK-Postfach zustellen. Dieser Prozess lässt sich über einen Routing-Switch konfigurierbar gestalten. Die Zustelladresse muss dazu der Identität der Empfänger:in zugeordnet werden. Diese Zuordnung entsteht bei der Aktivierung des Routing-Switch und verbleibt in der ELSTER-Domäne. Die genaue Ausgestaltung ist eine noch zu treffende Folgeentscheidung (Zuordnung der ELSTER-Identität zur ZaPuK-Adresse über BundID, eID, European Digital Identity (EUDI)-Wallet). Die Nachrichten liegen in der ZaPuK-Infrastruktur nur verschlüsselt vor und können entsprechend dem Konzept nur an den vorgesehenen Endpunkten clientseitig mit den Matrix-Schlüsseln entschlüsselt werden. Da die ZaPuK-Adresse an das bereichsspezifische Pseudonym aus BundID oder eID gebunden ist, erfolgt die Anmeldung am ZaPuK-Postfachzugang über diese Identifizierungsmittel. Mit den im Zielbild festgelegten Authentifizierungsmöglichkeiten (z.B. EUDI-Wallet) ist eine erneute Authentifizierung über ELSTER somit für den Zugriff auf die für die menschliche Verarbeitung bestimmten Nachrichten (z. B. Steuerbescheide) nicht notwendig.

- Gut: Synergie mit der Anbindung des Postfachs 2.0 (Mein Unternehmenskonto (MUK)) möglich, da hier ebenfalls eine vorübergehende Anbindung über eine Bridge in der Transition geplant ist.
- Neutral: Die Bestandsnachrichten verblieben im ELSTER-Postfach, da diese nur mit den ELSTER-Schlüsseln der Nutzenden entschlüsselbar und somit nicht automatisch migrierbar sind.
- Neutral: Der Klartextzugriff wäre weiterhin nur auf die ELSTER-Infrastruktur beschränkt, innerhalb der Server der ZaPuK-Infrastruktur gibt es keinen Zugriff auf den Klartext.
- Schlecht: Es ist eine ELSTER-Komponente für die Matrix-Anbindung zu entwickeln, es entstehen Aufwände für die Konzeption (z. B. Abbildung Adressen auf Identitäten).

### Option D: Native Zustellung per Matrix

Auch diese Option betrifft allein die für die menschliche Auswertung bestimmten und nach [TADR-MEIN-ELSTER-001](./TADR-MEIN-ELSTER-001-Zuordnung-von-ELSTER-Datenarten-zum-zentralen-ZaPuK-Postfach.md) umgeleiteten Datenarten. Die jeweiligen Absender der Nachrichten können diese nun E2E-verschlüsselt zustellen. Ohne die Notwendigkeit einer serverseitigen Vorverarbeitung kann eine von einer Sachbearbeiter:in verfasste Nachricht direkt aus deren in das Postfach einer Bürger:in übermittelt werden (und umgekehrt im Rahmen eines Vorgangs). Werden Nachrichten wie bisher im Backend erzeugt, bleibt der Ort der Verschlüsselung unverändert. Ob Beschäftigte unmittelbar antworten, entscheidet die Finanzverwaltung. Eine Unternehmensanwendung oder Fachverfahrensanwendung kann technische Nutzer verwenden, um Nachrichten per Matrix zu übermitteln. Ein solcher Postfachzugang lässt sich mittels Software Development Kit (SDK) auch in den lokal betriebenen Applikationen integrieren. Wenn also Daten vorab automatisiert aufbereitet und ggf. Nachrichten bzw. Dokumente automatisch serverseitig erzeugt werden müssen, kann über das Konzept der technischen Nutzer eine tragfähige Lösung umgesetzt werden (vgl. [ADR-1101](/Zielarchitektur/Architekturentscheidungen/ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md)). Zunächst ließe sich der direkte Kanal zwischen Menschen auf beiden Seiten der Kommunikation einfach realisieren. Die Umsetzung bei der automatischen Zustellung serverseitig generierter Dokumente wie z. B. Bescheide ist dann über technische Nutzer:innen mit entsprechendem Entwicklungsaufwand nach und nach möglich.

- Gut: Die Umsetzung wäre in Stufen machbar, wobei zunächst nur serverseitig nicht verarbeitete Nachrichten empfangen werden könnten und nach und nach auch komplexere Prozesse mittels technischer Nutzer ZaPuK-Kanäle für die Kommunikation mit den Teilnehmenden verwenden könnten.
- Gut: ELSTER-Schlüssel (Public/Private-Keys) wären für die umgeleiteten Datenarten nicht mehr notwendig. Die Verschlüsselung würde über die fortgeschrittenen Technologien zur Schlüsselverteilung von Matrix umgesetzt werden. Für Anmeldung, Einreichungen und die verbleibenden Datenarten bleibt das ELSTER-Zertifikat unberührt.
- Neutral: Eine Umstellung auf Seiten der Absender verursacht weitere Aufwände.
- Schlecht: Es ist eine ELSTER-Komponente für die Matrix-Anbindung zu entwickeln, es entstehen Aufwände für die Konzeption (z. B. Abbildung Adressen auf Identitäten).

### Option E: Herauslösung der Postfachkomponente aus Mein ELSTER

Diese Option schlägt die vollständige Herauslösung der als Postfach bezeichneten Komponente aus dem Portal "Mein ELSTER" und die Ersetzung durch die Integration eines dezentralen ZaPuK-Postfachzugangs vor.

- Schlecht: Das Bestands-Postfach ist eng in die Formular- und Veranlagungslogik integriert, womit eine Umsetzung sehr langwierig und aufwändig wäre.

## Architekturentscheidung

Empfohlen wird eine Umsetzung mit den Optionen C und D. Für die erste Stufe (Option C) setzt ELSTER eine Bridge um, konfiguriert das Routing je Datenart und ermöglicht die Adressierung entsprechend dem ZaPuK-Konzept. In der zweiten Stufe (Option D) wird der Postfachzugang über das SDK tiefer in die Prozesse eingebunden.

Die technische Ausgestaltung und die erforderlichen Aufwände bewertet das Verfahren ELSTER in einer eigenen Einschätzung.

### Auswirkungen

Es gibt ein langfristiges Zielbild, bei dem die für Menschen bestimmte Kommunikation, die derzeit über das ELSTER-Postfach läuft, in die ZaPuK-Infrastruktur übergeht. Die maschinell verarbeiteten Datenarten verbleiben auch im Zielbild bei ELSTER (vgl. [TADR-MEIN-ELSTER-001](./TADR-MEIN-ELSTER-001-Zuordnung-von-ELSTER-Datenarten-zum-zentralen-ZaPuK-Postfach.md)).
s
### Offene Punkte

- Analyse der Datenarten und Kategorisierung
- Architekturentwurf für Option C
- Zuordnung der ELSTER-Identität zur ZaPuK-Adresse über BundID, EUDI-Wallet oder eID ([ADR-1105](/Zielarchitektur/Architekturentscheidungen/ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md))

### Referenzen

- Vgl. [TADR-MEIN-ELSTER-001](./TADR-MEIN-ELSTER-001-Zuordnung-von-ELSTER-Datenarten-zum-zentralen-ZaPuK-Postfach.md)
- Vgl. [ADR-0003](/Zielarchitektur/Architekturentscheidungen/ADR-0003-Bereitstellung-von-Postfach-Zugängen.md), [ADR-0009](/Zielarchitektur/Architekturentscheidungen/ADR-0009-SDKs-zur-vereinfachten-Anbindung-von-Postfach-Zugängen.md), [ADR-0010](/Zielarchitektur/Architekturentscheidungen/ADR-0010-Herstellung-von-Interoperabilität-mit-fremden-Systemen-außerhalb-des-Geltungsbereiches-der-Zielarchitektur.md), [ADR-0011](/Zielarchitektur/Architekturentscheidungen/ADR-0011-Kommunikationsschicht.md), [ADR-0012](/Zielarchitektur/Architekturentscheidungen/ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md), [ADR-0018](/Zielarchitektur/Architekturentscheidungen/ADR-0018-Nichtabstreitbarkeit-Übermittlung.md), [ADR-1101](/Zielarchitektur/Architekturentscheidungen/ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md), [ADR-1147](/Zielarchitektur/Architekturentscheidungen/ADR-1147-Schluesselwiederherstellung.md)