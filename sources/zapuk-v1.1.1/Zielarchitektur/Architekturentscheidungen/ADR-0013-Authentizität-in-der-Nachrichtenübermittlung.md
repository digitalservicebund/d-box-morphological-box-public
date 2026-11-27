> **Status:** ✔️ Operativ entschieden | **Datum:** 9. April 2025

> **Hinweis (Phase 2, Stand Oktober 2026):** Für den Identitätsnachweis im Nachrichtenkontext wurde in Phase 2 eine Zwischenlösung gewählt. Da BundID und MUK Nachweise nicht an Schlüssel des Clients binden (Holder-Binding), stellt eine Nachweisstelle die Identitätsnachweise aus (ADR-1106). Die hier gewählte Option 3 bleibt das Zielbild.

# ADR-0013: Authentizität in der Nachrichtenübermittlung

## Kontext und Problembeschreibung

Authentizität in der Nachrichtenübermittlung erlaubt es Teilnehmenden in einem Kommunikationskanal, die Identität, das Vertrauensniveau des Authentisierungsmittels und die richtige Adressierung von anderen Teilnehmenden sicherzustellen, sowie den Ursprung und die Integrität einzelner Nachrichten zu prüfen.
Dies bildet den Rahmen für die Vertraulichkeit des Kommunikationskanals.

Dieser ADR beschäftigt sich allgemein mit Mechanismen, die der Herstellung von Authentizität dienen und mit den dabei umsetzbaren technischen Sicherheitsgarantien.
Die nachfolgenden ADRs ([ADR-0014](./ADR-0014-Authentizität-Identität-von-Privatpersonen.md), [ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md), [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md)) behandeln Identitätsnachweise einzelner Nutzendengruppen, deren Grundlage und Sicherheitsgarantien gesondert.

Die hier betrachteten Mechanismen unterstützen die Kommunikationsparteien bei der sicheren Kommunikation untereinander und dienen damit der (technischen) Sicherheit des Gesamtsystems.

Nicht behandelt im vorliegenden ADR wird der Nachweis der Authentizität **gegenüber Dritten**, beispielsweise als rechtskräftiger Beweis für die erfolgte Übermittlung oder Autorenschaft einer empfangenen Nachricht.
Diese Eigenschaften werden gesondert in [ADR-0017](./ADR-0017-Authentizität-und-Integrität-von-Inhalten.md) und [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md) behandelt.

## Entscheidungstreiber

- Der Ansatz zur Herstellung von Authentizität und das eingesetzte E2EE-Protokoll sollten eng aufeinander abgestimmt sein. Die vorliegende Architekturentscheidung hat daher starke Wechselwirkung mit [ADR-0012](./ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md) (Ende-zu-Ende Verschlüsselungsschicht).
- Die gemäß [ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md) und [ADR-0007](./ADR-0007-Implementierung-von-Postfach-Backends.md) gewählte offene Architektur im Zero-Trust-Ansatz ermöglicht die Entwicklung und den Betrieb von Backend-Systemen durch Dritte. Sicherheitsgarantien bei der Herstellung der Authentizität dürfen folglich nicht von der korrekten Funktionsweise von Backend-Systemen abhängen.
- Relevante Anforderungen:
  - DAT_ÜBE_ÜBE_021: [Das System] setzt den Privacy by Design Ansatz um.
  - SIC_ÜBE_ÜBE_015: Das System setzt den Security by Design Ansatz um.
  - SIC_ÜBE_ÜBE_018: Das System setzt den Zero Trust Ansatz um.
  - SIC_AUT_ÜBE_014: Die Integrität und Ende-zu-Ende-Authentizität der übermittelten [Nachrichten] wird auf kryptographischer Ebene unter Wahrung *glaubwürdiger Abstreitbarkeit* (Plausible Deniability) realisiert, um im Falle von Kompromittierungen keine Echtheit einzelner übermittelter Informationen belegen zu können und so Individuen vor Stigmatisierung und Diskriminierung zu schützen.
  - SIC_ÜBE_ÜBE_006: [Nutzer:innen] können bis zum Vertrauensniveau "hoch" (gemäß BSI TR-03107-1) authentifiziert werden.
  - SIC_ÜBE_ÜBE_008: Die [Empfänger:in] einer [Nachricht] kann nachvollziehen, auf welchem Vertrauensniveau die [Absender:in] authentifiziert wurde.
  - SIC_ÜBE_ÜBE_007: Das System erlaubt Zugriff auf [Nachrichten] durch die [Empfänger:in] nur, wenn die [Empfänger:in] mit dem von der [Absender:in] für diese [Nachricht] definierten Vertrauensniveau authentifiziert wurde.
  - SIC_AUT_ÜBE_007: Das System stellt die Authentizität einer [Nachricht] auf dem Vertrauensniveau sicher, auf dem die [Absender:in] authentifiziert wurde.
  - SIC_INT_ÜBE_003: Das System stellt die Integrität der übermittelten [Nachrichten] sicher.
  - SIC_VER_ÜBE_026: Das System stellt eine Ende-zu-Ende-Verschlüsselung der übermittelten [Nachrichten] (inklusive [Anhängen] und [Vertraulichen Metadaten]) sicher.
  - SIC_VER_ÜBE_032: Die Verschlüsselung übermittelter und gespeicherter [Nachrichten] kann weder von der Betreiberin des Systems, noch von Dritten gebrochen, umgangen oder entschlüsselt werden.
  - SIC_VER_ÜBE_051: Alle Metadaten, die nicht zwingend für die korrekte Zustellung einer [Nachricht] erforderlich sind, werden vermieden, als Teil der übermittelten [Nachricht] als [vertrauliche Metadaten] verschlüsselt oder durch Privatsphäre-schützende Technologien (Privacy-enhancing Technologies, PET) verschleiert.
  - ANF_FUN_IDE_034: [Das System] sollte in der Lage sein, zukünftig ggf. auch weitere vom BSI festgelegte [Authentisierungsmittel] zu unterstützen, wenn der initiale [Nachweis] der [Identität] natürlicher Personen maximal auf Vertrauensniveau "substantiell" erbracht wurde. (vgl. § 12 Abs. 5 OZG)
  - ANF_FUN_IDE_044: [Das System] unterstützt die Integration bestehender europäischer Identitätslösungen ([Identitäten] nach eIDAS 1.0).
  - ANF_FUN_IDE_027: [Das System] verwendet offene Standardprotokolle zur [Authentifizierung] und [Autorisierung] der [Nutzer:innen].
  - ANF_FUN_IDE_001: Die Rolle einer [Absender:in] ([öffentliche Stelle], [private Organisation], [Privatperson], etc.) muss an einer [Nachricht] erkennbar sein.

## Betrachtete Optionen

- [Option 1: Keine Authentizität in der Nachrichtenübermittlung](#option-1-keine-authentizität-in-der-nachrichtenübermittlung)
- [Option 2: Herstellung der Authentizität auf Transportebene zwischen beteiligten Backend-Systemen der Kommunikationsinfrastruktur](#option-2-herstellung-der-authentizität-auf-transportebene-zwischen-den-beteiligten-backend-systemen-der-kommunikationsinfrastruktur)
- [Option 3: Herstellung der Authentizität auf der Ende-zu-Ende Verschlüsselungsschicht](#option-3-herstellung-der-authentizität-auf-der-ende-zu-ende-verschlüsselungsschicht)

## Gegenüberstellung der jeweiligen Optionen

### Option 1: Keine Authentizität in der Nachrichtenübermittlung

Ohne Authentizität in der Nachrichtenübermittlung fehlen Grundvoraussetzungen zur Integrität und Vertraulichkeit eines Kommunikationskanals.
Empfänger:innen haben keine Möglichkeit, die Urheberschaft und Unversehrtheit konkreter Nachrichten prüfen zu können.

Ohne Authentizität im Zusammenspiel mit Identifikation können ferner auch die Sicherheitsgarantien der Ende-zu-Ende Verschlüsselung nicht eingehalten werden, da die Sicherheit der Ende-zu-Ende-Verschlüsselung insbesondere auf der sicheren Verteilung der Schlüssel von überprüfbar identifizierten Teilnehmenden untereinander beruht.

Ebenfalls kann keine Prüfung des Vertrauensniveaus basierend auf den Authentisierungsmitteln durchgeführt werden.

- Schlecht, denn diese Option kommt aufgrund der Möglichkeit zur Manipulation von Postfach-Nachrichten auf dem Übertragungsweg und der fehlenden Überprüfbarkeit der Identität von Adressat:innen nicht infrage.

### Option 2: Herstellung der Authentizität auf Transportebene zwischen den beteiligten Backend-Systemen der Kommunikationsinfrastruktur

Zwischen den an der Nachrichtenübermittlung beteiligten Komponenten der Backend-Infrastruktur kann ein gegenseitiges Verständnis über die Authentizität hergestellt werden, indem Identitätsdaten und Nachrichten durch einen authentisierten Kanal zwischen zwei dieser Backend-Komponenten ausgetauscht werden.
Dabei ist die Herstellung der Authentizität auf eine Vertrauensstellung der Backend-Komponenten untereinander und eine Vertrauensstellung der Kommunikationsteilnehmer:innen zu den Backend-Komponenten angewiesen.

Auch die Sicherheitsgarantien der Ende-zu-Ende Verschlüsselung werden effektiv auf diese Vertrauensstellungen reduziert, da legitime Identitäten durch die Backend-Komponenten vorgetäuscht werden könnten und damit die korrekte Verteilung von Schlüsselmaterial manipuliert werden könnte.

Die Prüfung des Vertrauensniveaus der Nachrichtenübermittlung im Kommunikationskanal ist ebenfalls auf die oben genannte Vertrauensstellung reduziert.
Insbesondere ist diese Prüfung dabei nicht unabhängig durch eine einen Verwaltungsakt bekanntgebende Behörde durchführbar (vgl. § 9 Abs. 1 S. 3 OZG), da sich die betroffene Behörde bei der korrekten Durchführung der Authentifizierung der berechtigten Person blind auf die Backend-Komponenten verlassen muss (kein Zero-Trust-Ansatz).

- Gut, da eine zentrale Realisierung in Backend-Komponenten Komplexität einspart
- Schlecht, da eine solche Architektur dem Zero-Trust-Prinzip widerspricht, denn die Sicherheit der vertraulichen Kommunikation wird auf die Sicherheit zentraler Server reduziert (Vertrauensstellung gegenüber Backend-Systemen erforderlich)
- Schlecht, da die zur Authentisierung verwendeten Daten beiden Backend-Komponenten bekannt sein müssen, was dem Privacy-by-Design-Prinzip widerspricht

### Option 3: Herstellung der Authentizität auf der Ende-zu-Ende Verschlüsselungsschicht

Ende-zu-Ende-Verschlüsselung wird in der Kommunikationsinfrastruktur wie in [ADR-0012](./ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md) beschrieben umgesetzt. Eine wesentliche Aufgabe des Ende-zu-Ende Verschlüsselungsprotokolls ist das Management von Schlüsselmaterial, welches ein gegenseitiges Verständnis über den Kreis der Teilnehmenden im Kommunikationskanal realisiert.
So werden in den Clients die Schlüssel der anderen Teilnehmenden laufend auf Authentizität geprüft, um die Vertraulichkeit des Kommunikationskanals zu gewährleisten.

Ebenfalls stellt das Ende-zu-Ende Verschlüsselungsprotokoll Funktionen für die Authentizitäts- und Integritätsprüfung von Nachrichten zur Verfügung.
Durch die Nutzung dieser bereitgestellten Funktionen (Kopplung der Authentisierung und Inhaltsdatenverschlüsselung) werden einige typische Angriffsvektoren vermieden (z.B. Verwechseln von Absender:innen, Verwechseln von Schlüsselpaaren durch fehlende Domain Separation).

> [!note] Hinweis
> Konkret kann Authentizität kryptografisch unter anderem durch Signieren der Nachrichten oder Authentisieren des Kommunikationskanals realisiert werden. Für diesen ADR wird zwischen diesen beiden Verfahren nicht unterschieden, da die entsprechende Funktionalität von der Ende-zu-Ende-Verschlüsselungsschicht bereitgestellt wird (siehe [ADR-0012](./ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md)).

Ein blindes Vertrauen in die Authentizität des Schlüsselmaterials der Ende-zu-Ende-Verschlüsselungsschicht bei der Erstellung des Kanals (sog. Trust on first use“-Ansatz, kurz: TOFU) garantiert zunächst nur die Existenz eines verschlüsselten Kanals.
Ohne Überprüfung der Identitäten der beteiligten Akteure können Machine-in-the-Middle-Angriffe hierbei nicht ausgeschlossen werden.

Erst durch die Bindung von fachlichen Identitäten(Identifikationsmerkmale auf dem jeweiligen Vertrauensniveau) an kryptografische Identitäten der Ende-zu-Ende Verschlüsselungsschicht (kryptographisches Schlüsselmaterial des Clients) authentisieren Kommunikationsteilnehmende sich gegenseitig.
Dadurch beweisen Teilnehmende die Autorenschaft der von ihnen im Kanal gesendeten Nachrichten ggü. ihrer Kommunikationspartner:in und stellen gleichzeitig die Vertraulichkeit des Kanals her („Validierung der Enden des Kommunikationskanals“).
Kommunikationspartner:innen sollten Nachrichten im Kanal daher erst nach beidseitiger Authentisierung übermitteln.

Sicher ist die Bindung der Identitäten an den Kommunikationskanal, wenn sie unabhängig von den Backend-Komponenten der Kommunikationsinfrastruktur prüfbar, auf den konkreten Kanal limitiert und damit nicht übertragbar ist (zur Mitigation von Replay-Angriffen, beziehungsweise Herstellung von Non-Transferability).
Mit der Bindung kann die Vertraulichkeit und Authentizität durchgehend, selbst über eine potentiell kompromittierte Kommunikationsinfrastruktur, gewährleistet werden.

Maßgeblich für das Vertrauensniveau der Authentisierung gegenüber der jeweiligen Kommunikationspartei ist das Vertrauensniveau des eingesetzten Identifizierungsmittels zur Authentisierung des Kommunikationskanals.
Analog zur initialen Authentisierung des Kommunikationskanal auf einem bestimmten Vertrauensniveau kann, sofern erforderlich, zu einem späteren Zeitpunkt eine fachlich bedingte Erhöhung des Vertrauensniveaus im Kommunikationskanal durch Re-Authentisierung auf diesem Vertrauensniveau erreicht werden.

Der Zugriff auf Kommunikationsinhalte ist in dieser Architektur-Option immer doppelt abgesichert: Einerseits durch die an die Ende-zu-Ende-Verschlüsselung gebundene Authentisierung des Kommunikationskanals und andererseits durch die Authentisierung gegenüber den Backend-Komponenten, welche technischen Zugriff auf die verschlüsselten Kommunikationsinhalte gewähren.
Da die Authentisierung gegenüber der jeweiligen Kommunikationspartei in dieser Architektur-Option im Ende-zu-Ende-verschlüsselten Kommunikationskanal ohne Vertrauensstellung zu Backend-Komponenten erfolgt, müssen die Backend-Komponenten die Identität der Nutzer:innen zur Herstellung dieser Authentizität nicht zwingend kennen (vgl. [SIC_VER_ÜBE_051](../../Konsolidierte-Anforderungen/README.md#thema-sicherheit)).

- Gut, da die kryptographische Bindung der Authentizität an die Ende-zu-Ende Verschlüsselungsschicht Teilnehmende in die Lage versetzt, die Authentizität des Kanals unabhängig von Garantien der Backend-Komponenten zu prüfen (Zero-Trust-Ansatz) und damit die technisch sicherste Form der Authentizitätsherstellung darstellt
- Gut, da authentifizierungsrelevante Informationen durch das Zurückgreifen auf die von der Ende-zu-Ende Verschlüsselungsschicht bereitgestellten Funktionen vertraulich übermittelt werden können, was auf die Umsetzung des Privacy-by-Design-Prinzips einzahlt
- Neutral, da Komplexität von zentraler Infrastruktur auf Endgeräte verlagert wird

## Architekturentscheidung

Gewählte Option: **[Option 3: Herstellung der Authentizität auf der Ende-zu-Ende Verschlüsselungsschicht](#option-3-herstellung-der-authentizität-auf-der-ende-zu-ende-verschlüsselungsschicht)**, denn nur so kann Authentizität kryptografisch sicher unter Umsetzung des Zero-Trust-Prinzips hergestellt und die Vertraulichkeit der Kommunikation entsprechend effektiv und resilient gewährleistet werden.
Die Komplexität der Umsetzung ist durch Synergien mit der Umsetzung der Ende-zu-Ende Verschlüsselung handhabbar und bezogen auf den Nutzen hinsichtlich der Anforderungen gerechtfertigt.

### Auswirkungen

Im Zero-Trust-Ansatzes ist grundsätzlich eine gegenseitige Authentifizierung vor der Übermittlung einer Nachricht erforderlich.
Eine Übermittlung von Nachrichten an das Ziel-Backend-System ohne vorherige Prüfung der Authentizität des Ende-zu-Ende-verschlüsselten Kanals erlaubt im Zero-Trust-Modell die Durchführung von Machine-in-the-Middle-Angriffen durch das Backend-System (oder ggf. auch durch Dritte).
Ohne vorherige Authentifizierung kann technisch nicht sichergestellt werden, dass nur die beabsichtigte Empfänger:in einer fachlichen Nachricht unter Wahrung des erforderlichen Vertrauensniveau auf die Nachricht zugreift.

Die in den nachfolgenden ADRs ([ADR-0014](./ADR-0014-Authentizität-Identität-von-Privatpersonen.md), [ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md), [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md)) beschriebenen Mechanismen sollten daher geeignet sein, unter diesen Bedingungen eine hohe User Experience zu erreichen.
Die Komplexität der Absicherung im Zero-Trust-Ansatz sollte für Nutzende intuitiv in User Interfaces verborgen werden und für angebundene Systeme durch ein entsprechendes SDK gekapselt werden.
