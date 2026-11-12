> **Status:** ✅ Beschlossen | **Datum:** 28. April 2025

# ADR-0016: Authentizität: Identität privater Organisationen
## Kontext und Problembeschreibung

Gemäß [ADR-0013](./ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md) wird Authentizität auf der Ende-zu-Ende Verschlüsselungsebene durch eine effektive Bindung fachlicher Identitäten an kryptografische Identitäten im Kommunikationskanal erreicht.

Im vorliegenden ADR wird beschrieben, wie sich private Organisationen gegenüber anderen Kommunikationspartner:innen authentisieren können.
Durch die in [ADR-0013](./ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md) beschriebene Bindung dieser Authentisierung an den Ende-zu-Ende-verschlüsselten Kommunikationskanal beweisen private Organisationen die Autorenschaft der von ihnen im Kanal gesendeten Nachrichten gegenüber ihrer Kommunikationspartner:in und stellen gleichzeitig die Vertraulichkeit des Kanals her.

## Entscheidungstreiber

- Relevante Anforderungen:
  - SIC_ÜBE_ÜBE_018: Das System setzt den Zero Trust Ansatz um.
  - SIC_AUT_ÜBE_014: Die Integrität und Ende-zu-Ende-Authentizität der übermittelten [Nachrichten] wird auf kryptographischer Ebene unter Wahrung *glaubwürdiger Abstreitbarkeit* (Plausible Deniability) realisiert, um im Falle von Kompromittierungen keine Echtheit einzelner übermittelter Informationen belegen zu können und so Individuen vor Stigmatisierung und Diskriminierung zu schützen.
  - SIC_VER_ÜBE_051: Alle Metadaten, die nicht zwingend für die korrekte Zustellung einer [Nachricht] erforderlich sind, werden vermieden, als Teil der übermittelten [Nachricht] als [vertrauliche Metadaten] verschlüsselt oder durch Privatsphäre-schützende Technologien (Privacy-enhancing Technologies, PET) verschleiert.
  - ANF_FUN_IDE_001: Die Rolle einer [Absender:in] ([öffentliche Stelle], [private Organisation], [Privatperson], etc.) muss an einer [Nachricht] erkennbar sein.
  - ANF_FUN_IDE_047: [Das System] stellt der [Empfänger:in] einer [Nachricht] die Information bereit, ob diese von einer natürlichen Person (z.B. einer Sachbearbeitung) oder automatisiert (z.B. [Fachverfahren], KI Chatbot) verfasst wurde.
  - ANF_FUN_IDE_024: Zur [Identifikation] von [privaten Organisationen] werden mindestens die in § 3 OZG genannten [Identifizierungsmittel] unterstützt. Die [Identifikation] muss mindestens über das einheitliche Organisationskonto gemäß § 3 OZG erfolgen können.
  - ANF_FUN_IDE_025: Die [Identifikation] über [EUDI-Wallet] mit starker kryptografischer Bindung muss perspektivisch möglich sein, sofern die organisatorischen und technischen Voraussetzungen hierzu geschaffen sind (vgl. eIDAS-Credential mit Anonymous Credentials, Zero Knowledge Proofs, Deniability, selbstverwaltete kryptografische Identität).
  - ANF_FUN_IDE_027: [Das System] verwendet offene Standardprotokolle zur [Authentifizierung] und [Autorisierung] der [Nutzer:innen].

## Betrachtete Optionen

- [Option 1: Identifikation mittels X.509-Zertifikaten aus einer PKI](#option-1-identifikation-mittels-x509-zertifikaten-aus-einer-pki)
- [Option 2: Identifikation mittels Organisationskonto gemäß § 3 OZG](#option-2-identifikation-mittels-organisationskonto-gemäß--3-ozg)
- [Option 3: Identifikation mittels European Digital Identity Wallet (EUDI-Wallet)](#option-3-identifikation-mittels-european-digital-identity-wallet-eudi-wallet)

## Gegenüberstellung der jeweiligen Optionen

### Option 1: Identifikation mittels X.509-Zertifikaten aus einer PKI

Zur Herstellung von Authentizität auf Ende-zu-Ende Verschlüsselungsebene können sich private Organisationen mit Hilfe eines Zertifikats aus einer PKI gegenüber einem anderen Kommunikationsteilnehmenden identifizieren.
Dieses Verfahren erlaubt ein hohes Maß an Sicherheit und Vertraulichkeit auf dem Vertrauensniveau der genutzten PKI.

Um eine effektive Authentisierung des Ende-zu-Ende-verschlüsselten Kanals zu erreichen, müssen das im im genutzten X.509-Zertifikat hinterlegte Schlüsselmaterial (mit Hilfe der von der Ende-zu-Ende-Verschlüsselungsschicht bereitgestellten Funktionen) an den kryptographischen Kanal gebunden werden und die im Zertifikate enthaltenen Identitätsdaten der Kommunikationspartei dargestellt werden, gegenüber der sich die private Organisation authentisiert.

Zusätzlich muss zwingend die Gültigkeit des Zertifikats geprüft werden (inkl. Prüfung des Zertifikatspfad und des Revokationsstatus).

Bestehende privatwirtschaftlich organisierte PKIs (z.B. die Web PKI) bieten moderne Funktionalitäten wie automatische Zertifikatserneuerung mittels ACME, Transparency Logs oder die Möglichkeit zur Hinterlegung gesicherter Attribute in Zertifikaten.

Hierbei sind zusätzliche Maßnahmen nötig, um den Teilnehmendenkreis auf eindeutig identifizierte private Organisationen zu beschränken und damit eine missbräuchliche Nutzung zu vermeiden.
Privatpersonen und öffentliche Stellen müssen bei der Kommunikation mit privaten Organisationen anhand der genutzten Zertifikate jederzeit zweifelsfrei deren Identität prüfen können.
Denkbar wären beispielsweise Vorgaben zur Hinterlegung spezifischer gesicherter Attribute in Zertifikaten (insb. Organisations-ID, Organisationstyp, sowie der entsprechende Nachweis der Prüfung).

- Gut, da X.509-Zertifikate einen sehr verbreiteten und sehr etablierten Mechanismus zur Authentisierung darstellen.
- Gut, da der Einsatz von X.509-Zertifikaten auch nicht-interaktiv (d.h. ohne Nutzendeninteraktion auf Seiten der privaten Organisationen) möglich ist.
- Gut, da X.509-Zertifikate (eingeschränkt) auch offline und ohne aktive Kommunikationsverbindung zu einem zentral betriebenen IDP genutzt werden können.
- Gut, da moderne Funktionalitäten wie automatische Zertifikatserneuerung mittels ACME oder Transparency Logs unterstützt werden können.
- Schlecht, da mitunter sehr kostenintensive Maßnahmen nötig sind, um den Teilnehmendenkreis auf eindeutig identifizierte private Organisation zu beschränken und damit eine missbräuchliche Nutzung zu vermeiden.

### Option 2: Identifikation mittels Organisationskonto gemäß § 3 OZG

Zur Herstellung von Authentizität auf Ende-zu-Ende Verschlüsselungsebene können sich private Organisationen mit einem Organisationskonto gegenüber einem anderen Kommunikationsteilnehmenden identifizieren.
Dieses Verfahren erlaubt Sicherheit und Vertraulichkeit auf dem Vertrauensniveaus „substantiell“.

Technisch unterscheidet sich dies nicht zur entsprechenden Option aus [ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md).

Zur Authentisierung wird ein von der Organisationskonto-Infrastruktur ausgestelltes SAML-Token vertraulich im Kontext des Ende-zu-Ende verschlüsselten Kommunikationskanals übermittelt und durch einen anderen Kommunikationsteilnehmenden geprüft. Entsprechend ist für die Prüfung eine Vertrauensstellung gegenüber dem Organisationskonto-Server erforderlich (kein Zero-Trust-Ansatz).

Um eine kryptografisch sichere Bindung zu erreichen, muss der SAML-Token, wie in [ADR-0013](./ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md) generell beschrieben, auf den betreffenden Kommunikationskanal limitiert sein.

- Gut, da viele private Organisationen das Organisationskonto bereits zur digitalen Beantragung von Verwaltungsleistungen nutzen
- Schlecht, da die Nutzung des Organisationskontos zwingend eine Nutzendeninteraktion auf Seiten der privaten Organisationen für jeden Authentisierungsvorgang erfordert
- Schlecht, da Kommunikationsparteien auf die korrekte Authentifizierung durch die Organisationskonto-Infrastruktur vertrauen müssen (keine Zero-Trust-Architektur)

### Option 3: Identifikation mittels European Digital Identity Wallet (EUDI-Wallet)

Perspektivisch sollen sich private Organisationen zur Herstellung von Authentizität auf Ende-zu-Ende Verschlüsselungsebene direkt mit einem entsprechenden Nachweis aus der European Digital Identity Wallet gegenüber einem anderen Kommunikationsteilnehmenden identifizieren können.
Dieses Verfahren erlaubt zukünftig ein hohes Maß an Sicherheit und Vertraulichkeit.

Die Voraussetzungen für dieses Verfahren unterscheiden sich technisch nicht zu den entsprechenden Optionen aus [ADR-0014](./ADR-0014-Authentizität-Identität-von-Privatpersonen.md) und [ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md).

Um Identitätsdaten aus der EUDI-Wallet sicher und unter Berücksichtigung der Anforderungen an den Ende-zu-Ende verschlüsselten Kanal zu binden, sind maßgeblich kryptografische Eigenschaften notwendig, die in der öffentlichen Konsultation auf EU-Ebene ([Cryptographers’ Feedback on the EU Digital Identity’s ARF](https://files.dyne.org/dynebolic/?file=eudi/cryptographers-feedback-june2024.pdf)) aufgezeigt wurden.
Ebenfalls sollte das Postfachsystem den Einsatz der perspektivisch benötigten Berechtigungszertifikate bei prüfenden Kommunikationsteilnehmern vorsehen und entsprechende Funktionalität analog zur eID in einem entsprechendem SDK bereitstellen.

- Gut, da mit der EUDI-Wallet perspektivisch ein hohes Sicherheitsniveau im Sinne einer Zero-Trust-Architektur erreicht werden kann
- Gut, da die perspektivische Unterstützung der EUDI-Wallet das Postfachsystem zukunftssicher, einfach nutzbar und in Europa interoperabel gestaltet
- Schlecht, denn die EUDI-Wallet ist noch nicht verfügbar

## Architekturentscheidung

Gewählte Optionen:
**Option 1: Identifikation mittels X.509-Zertifikaten aus einer PKI** und
**Option 2: Identifikation mittels Organisationskonto gemäß § 3 OZG**, weil Option 1 die Realisierung des Zero-Trust-Ansatzes sowie auch die automatisierte Authentisierung privater Organisationen ermöglicht und die Wahlmöglichkeit privater Organisationen beim Einsatz des Identifikationsmittels die Akzeptanz der durch die Zielarchitektur realisierten Lösung stärkt.
Option 3 kann perspektivisch realisiert werden, sofern die Voraussetzungen zum Einsatz der EUDI-Wallet vorliegen und diese für private Organisationen einfacher nutzbar sind als die Optionen 1 und 2.

### Auswirkungen

Die durch gleichzeitige Wahl der Optionen 1 und 2 ergeben sich Synergieeffekte, wenn die in Option 1 beschriebene PKI in die organisatorischen Struktur des Organisationskontos eingebunden ist.
Die Funktionalität zur Hinterlegung gesicherter Attribute kann dort durch Aufbau einer Zertifizierungsstelle (CA) insb. auf Basis der im Organisationskonto bereits vorhandenen Organisationsdaten realisiert werden.

Aus der Entscheidung für Option 1 ergeben sich Entwicklungsbedarfe hinsichtlich moderner PKI-Funktionalitäten wie automatisierter Zertifikatserneuerung mittels ACME, Transparency Logs oder der Möglichkeit zur Hinterlegung gesicherter Attribute in Zertifikaten.
Bestehende Organisationsdaten könnten dabei durch die von der CA ausgestellten Zertifikate abgesichert und mittels Transparency Log nachvollziehbar überprüfbar bereitgestellt werden.

Somit könnten die Vorteile von Option 1 und Option 2, sowie die in [ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md) beschriebenen Bedarfe für öffentliche Stellen perspektivisch zusammengeführt werden.