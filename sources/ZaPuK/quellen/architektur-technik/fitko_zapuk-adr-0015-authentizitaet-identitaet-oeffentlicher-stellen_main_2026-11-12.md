<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Zielarchitektur/Architekturentscheidungen/ADR-0015-Authentizit%C3%A4t-Identit%C3%A4t-%C3%B6ffentlicher-Stellen.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob dc5dbb72c5); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

> **Status:** ✔️ Operativ entschieden | **Datum:** 28. April 2025

> **Hinweis (Phase 2, Stand November 2026):** Diese Entscheidung wurde in Phase 2 konkretisiert (ADR-1106, ADR-1108).

# ADR-0015: Authentizität: Identität öffentlicher Stellen
## Kontext und Problembeschreibung

Gemäß [ADR-0013](./ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md) wird Authentizität auf der Ende-zu-Ende Verschlüsselungsebene durch eine effektive Bindung fachlicher Identitäten an kryptografische Identitäten im Kommunikationskanal erreicht.

Im vorliegenden ADR wird beschrieben, wie sich öffentliche Stellen gegenüber anderen Kommunikationspartner:innen authentisieren können.
Durch die in [ADR-0013](./ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md) beschriebene Bindung dieser Authentisierung an den Ende-zu-Ende-verschlüsselten Kommunikationskanal beweisen öffentliche Stellen die Autorenschaft der von ihnen im Kanal gesendeten Nachrichten gegenüber ihrer Kommunikationspartner:in und stellen gleichzeitig die Vertraulichkeit des Kanals her.

## Entscheidungstreiber

- Relevante Anforderungen:
  - SIC_ÜBE_ÜBE_018: Das System setzt den Zero Trust Ansatz um.
  - SIC_AUT_ÜBE_014: Die Integrität und Ende-zu-Ende-Authentizität der übermittelten [Nachrichten] wird auf kryptographischer Ebene unter Wahrung *glaubwürdiger Abstreitbarkeit* (Plausible Deniability) realisiert, um im Falle von Kompromittierungen keine Echtheit einzelner übermittelter Informationen belegen zu können und so Individuen vor Stigmatisierung und Diskriminierung zu schützen.
  - SIC_VER_ÜBE_051: Alle Metadaten, die nicht zwingend für die korrekte Zustellung einer [Nachricht] erforderlich sind, werden vermieden, als Teil der übermittelten [Nachricht] als [vertrauliche Metadaten] verschlüsselt oder durch Privatsphäre-schützende Technologien (Privacy-enhancing Technologies, PET) verschleiert.
  - ANF_FUN_IDE_001: Die Rolle einer [Absender:in] ([öffentliche Stelle], [private Organisation], [Privatperson], etc.) muss an einer [Nachricht] erkennbar sein.
  - ANF_FUN_IDE_047: [Das System] stellt der [Empfänger:in] einer [Nachricht] die Information bereit, ob diese von einer natürlichen Person (z.B. einer Sachbearbeitung) oder automatisiert (z.B. [Fachverfahren], KI Chatbot) verfasst wurde.
  - ANF_FUN_IDE_025: Die [Identifikation] über [EUDI-Wallet] mit starker kryptografischer Bindung muss perspektivisch möglich sein, sofern die organisatorischen und technischen Voraussetzungen hierzu geschaffen sind (vgl. eIDAS-Credential mit Anonymous Credentials, Zero Knowledge Proofs, Deniability, selbstverwaltete kryptografische Identität).
  - ANF_FUN_IDE_050: Zur [Identifikation] von [öffentlichen Stellen] werden mindestens die in § 3 OZG genannten [Identifizierungsmittel] unterstützt. Die [Identifikation] muss mindestens über das einheitliche Organisationskonto gemäß § 3 OZG erfolgen können.
  - ANF_FUN_IDE_027: [Das System] verwendet offene Standardprotokolle zur [Authentifizierung] und [Autorisierung] der [Nutzer:innen].

## Betrachtete Optionen

- [Option 1: Identifikation mittels X.509-Zertifikaten aus der Verwaltungs-PKI (V-PKI)](#option-1-identifikation-mittels-x509-zertifikaten-aus-der-verwaltungs-pki-v-pki)
- [Option 2: Identifikation mittels X.509-Zertifikaten aus einer öffentlichen PKI](#option-2-identifikation-mittels-x509-zertifikaten-aus-einer-öffentlichen-pki)
- [Option 3: Identifikation mittels Organisationskonto gemäß § 3 OZG](#option-3-identifikation-mittels-organisationskonto-gemäß--3-ozg)
- [Option 4: Identifikation mittels European Digital Identity Wallet (EUDI-Wallet)](#option-4-identifikation-mittels-european-digital-identity-wallet-eudi-wallet)

## Gegenüberstellung der jeweiligen Optionen

### Option 1: Identifikation mittels X.509-Zertifikaten aus der Verwaltungs-PKI (V-PKI)

Zur Herstellung von Authentizität auf Ende-zu-Ende Verschlüsselungsebene können sich öffentliche Stellen mit Hilfe eines Zertifikats aus der V-PKI gegenüber einem anderen Kommunikationsteilnehmenden identifizieren.
Dieses Verfahren erlaubt ein hohes Maß an Sicherheit und Vertraulichkeit auf dem Vertrauensniveau der V-PKI.

Die Identitätsdaten öffentlicher Stellen weisen (anders als Identitätsdaten von Privatpersonen) kein besonderes Schutzniveau hinsichtlich des Schutzziels der Vertraulichkeit auf.
Daher ist das Abfragen der Identität öffentlicher Stellen zur Authentisierung eines Kommunikationskanals für alle Teilnehmenden möglich.
Die Authentisierung des Kommunikationskanals durch ein IT-System der jeweiligen Fachbehörde kann damit ohne die Erforderlichkeit einer Nutzendeninteraktion automatisch und auch im Hintergrund durchgeführt werden.

Um eine effektive Authentisierung des Ende-zu-Ende-verschlüsselten Kanals zu erreichen, muss das im genutzten X.509-Zertifikat hinterlegte Schlüsselmaterial (mit Hilfe der von der Ende-zu-Ende-Verschlüsselungsschicht bereitgestellten Funktionen) an den kryptographischen Kanal gebunden werden und die im Zertifikate enthaltenen Identitätsdaten der Kommunikationspartei dargestellt werden, gegenüber der sich die öffentliche Stelle authentisiert.

Zusätzlich muss zwingend die Gültigkeit des Zertifikats geprüft werden (inkl. Prüfung des Zertifikatspfad und des Revokationsstatus).

Es wird angenommen, dass alle öffentlichen Stellen einen einfachen Zugang zur V-PKI besitzen und dort Zertifikate beantragen können.

- Gut, da X.509-Zertifikate einen sehr verbreiteten und sehr etablierten Mechanismus zur Authentisierung darstellen.
- Gut, da der Einsatz von X.509-Zertifikaten auch nicht-interaktiv (d.h. ohne Nutzendeninteraktion auf Seiten der öffentlichen Stelle) möglich ist.
- Gut, da der Einsatz von X.509-Zertifikaten ohne Abhängigkeiten zu einem zentralen IDP (wie dem Organisationskonto) auskommt und somit grundsätzlich auch (eingeschränkt) offline möglich wäre (sehr wertvoll im Falle von Cyber-Angriffen auf Fachbehörden).
- Schlecht, da die V-PKI insgesamt in einem schlechten Zustand ist und moderne Funktionalitäten wie eine automatische Zertifikatserneuerung mittels ACME oder Transparency Logs nicht unterstützt.
- Schlecht, da es innerhalb der V-PKI derzeit keinen Mechanismus gibt, um spezifische Attribute einer öffentlichen Stelle gesichert in Zertifikaten zu hinterlegen (z.B. maschinenlesbare Organisations-ID, Behördentyp, Berechtigungen, etc.).
- Schlecht, da der Prozess zur Beantragung von Zertifikaten in der V-PKI mitunter als aufwändig und manuell empfunden wird.
- Schlecht, da die Beantragung von Zertifikaten aus der V-PKI für öffentliche Stellen kostenpflichtig ist, was zusätzliche organisatorische Hürden mit sich bringt und zu einer geringeren Akzeptanz führen könnte.

### Option 2: Identifikation mittels X.509-Zertifikaten aus einer öffentlichen PKI

Analog zur Nutzung der V-PKI könnte auch eine öffentliche, d.h. nicht auf öffentliche Stellen beschränkte PKI-Infrastruktur genutzt werden.
Bestehende privatwirtschaftlich organisierte PKIs (z.B. die Web PKI) bieten moderne Funktionalitäten wie automatische Zertifikatserneuerung mittels ACME, Transparency Logs oder die Möglichkeit zur Hinterlegung gesicherter Attribute in Zertifikaten.

Privatpersonen und private Organisationen müssen bei der Kommunikation mit öffentlichen Stellen anhand der genutzten Zertifikate jederzeit zweifelsfrei feststellen können, dass sie gerade mit einer offiziellen Behörde kommunizieren, deren Identität sie zweifelsfrei prüfen können.
Hierbei sind zusätzliche Maßnahmen nötig, um den Teilnehmendenkreis auf eindeutig identifizierte öffentliche Stellen zu beschränken und damit eine missbräuchliche Nutzung zu vermeiden.
Denkbar wären beispielsweise Vorgaben zur Hinterlegung spezifischer gesicherter Attribute in Zertifikaten (insb. Organisations-ID sowie ein Nachweis der Prüfung, dass es sich um eine offizielle öffentliche Stelle handelt).

- Gut, da X.509-Zertifikate einen sehr verbreiteten und sehr etablierten Mechanismus zur Authentisierung darstellen.
- Gut, da der Einsatz von X.509-Zertifikaten auch nicht-interaktiv (d.h. ohne Nutzendeninteraktion auf Seiten der öffentlichen Stelle) möglich ist.
- Gut, da X.509-Zertifikate (eingeschränkt) auch offline und ohne aktive Kommunikationsverbindung zu einem zentral betriebenen IDP genutzt werden können.
- Gut, da moderne Funktionalitäten wie automatische Zertifikatserneuerung mittels ACME oder Transparency Logs unterstützt werden können.
- Schlecht, da mitunter sehr kostenintensive Maßnahmen nötig sind, um den Teilnehmendenkreis auf eindeutig identifizierte öffentliche Stellen zu beschränken und damit eine missbräuchliche Nutzung zu vermeiden.

### Option 3: Identifikation mittels Organisationskonto gemäß § 3 OZG

Zur Herstellung von Authentizität auf Ende-zu-Ende Verschlüsselungsebene können sich öffentliche Stellen mit einem Organisationskonto gegenüber einem anderen Kommunikationsteilnehmenden identifizieren.
Dieses Verfahren erlaubt Sicherheit und Vertraulichkeit auf dem Vertrauensniveaus „substantiell“.

Zur Authentisierung wird ein von der Organisationskonto-Infrastruktur ausgestelltes (SAML) Token vertraulich im Kontext des Ende-zu-Ende verschlüsselten Kommunikationskanals übermittelt und durch einen anderen Kommunikationsteilnehmenden geprüft.
Entsprechend ist für die Prüfung eine Vertrauensstellung gegenüber dem Organisationskonto-Server erforderlich (kein Zero-Trust-Ansatz).

Um eine kryptografisch sichere Bindung zu erreichen, muss der (SAML) Token, wie in [ADR-0013](./ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md) generell beschrieben, auf den betreffenden Kommunikationskanal limitiert sein.

- Gut, da alle öffentlichen Stellen gemäß § 3 OZG zur Nutzung des Organisationskontos verpflichtet sind.
- Schlecht, da die Nutzung des Organisationskontos zwingend eine Nutzendeninteraktion auf Seiten der öffentlichen Stelle für jeden Authentisierungsvorgang erfordert (keine nicht-interaktive Authentisierung zur automatisierten Bereitstellung von Bescheiden)
- Schlecht, da Kommunikationsparteien auf die korrekte Authentifizierung durch die Organisationskonto-Infrastruktur vertrauen müssen (keine Zero-Trust-Architektur)

### Option 4: Identifikation mittels European Digital Identity Wallet (EUDI-Wallet)

Perspektivisch sollen sich öffentliche Stellen zur Herstellung von Authentizität auf Ende-zu-Ende Verschlüsselungsebene direkt mit einem entsprechenden Nachweis aus der European Digital Identity Wallet gegenüber einem anderen Kommunikationsteilnehmenden identifizieren können.
Dieses Verfahren erlaubt zukünftig ein hohes Maß an Sicherheit und Vertraulichkeit.

Die Voraussetzungen für dieses Verfahren unterscheiden sich technisch nicht zu der entsprechenden Option aus [ADR-0014](./ADR-0014-Authentizität-Identität-von-Privatpersonen.md).

Um Identitätsdaten aus der EUDI-Wallet sicher und unter Berücksichtigung der Anforderungen an den Ende-zu-Ende verschlüsselten Kanal zu binden, sind maßgeblich kryptografische Eigenschaften notwendig, die in der öffentlichen Konsultation auf EU-Ebene ([Cryptographers’ Feedback on the EU Digital Identity’s ARF](https://files.dyne.org/dynebolic/?file=eudi/cryptographers-feedback-june2024.pdf)) aufgezeigt wurden.
Ebenfalls sollte das Postfachsystem den Einsatz der perspektivisch benötigten Berechtigungszertifikate bei prüfenden Kommunikationsteilnehmern vorsehen und entsprechende Funktionalität analog zur eID in einem entsprechendem SDK bereitstellen.

- Gut, da mit der EUDI-Wallet perspektivisch ein hohes Sicherheitsniveau im Sinne einer Zero-Trust-Architektur erreicht werden kann
- Gut, da die perspektivische Unterstützung der EUDI-Wallet das Postfachsystem zukunftssicher, einfach nutzbar und in Europa interoperabel gestaltet
- Schlecht, denn die EUDI-Wallet ist noch nicht verfügbar

## Architekturentscheidung

Gewählte Optionen:
**Option 1 (Identifikation mittels X.509-Zertifikaten aus der Verwaltungs-PKI)** und
**Option 3 (Identifikation mittels Organisationskonto gemäß § 3 OZG)**, weil Option 1 die Realisierung des Zero-Trust-Ansatzes sowie die automatisierte Authentisierung öffentlicher Stellen ermöglicht und die Wahlmöglichkeit öffentlicher Stellen beim Einsatz des Identifikationsmittels die Akzeptanz der durch die Zielarchitektur realisierten Lösung stärkt.
Gegenüber **Option 2 (Identifikation mittels X.509-Zertifikaten aus einer öffentlichen PKI)** weist die V-PKI eine bewusste und im vorliegenden Anwendungsfall gewünschte Beschränkung des Nutzendenkreises auf, weist jedoch einige funktionale Defizite auf, die es durch Weiterentwicklungen abzubauen gilt (siehe Abschnitt „Auswirkungen“).
Option 1 ist gegenüber Option 3 zu bevorzugen, da das zertifikatsbasierte Design der V-PKI ggü. dem Organisationskonto als zentraler Vermittlungsplattform für Identifikationsvorgänge die Realisierung des Zero-Trust-Ansatz ermöglicht.
Option 4 (Identifikation mittels EUDI-Wallet) kann perspektivisch realisiert werden, sofern die Voraussetzungen zum Einsatz der EUDI-Wallet für Organisationen vorliegen und diese für öffentliche Stellen einfacher nutzbar sind als die Optionen 1 und 3.

### Auswirkungen

Aus der Wahl der V-PKI als Identifikationsmittel für öffentliche Stellen ergeben sich Weiterentwicklungsbedarfe der V-PKI hinsichtlich moderner Funktionalitäten wie automatisierter Zertifikatserneuerung mittels ACME, Transparency Logs oder der Möglichkeit zur Hinterlegung gesicherter Attribute in Zertifikaten (vgl. Option 2).

Denkbar wäre auch die Realisierung dieser Funktionalitäten, insb. der Möglichkeit zur Hinterlegung gesicherter Attribute, durch Aufbau einer Zertifizierungsstelle (CA) der V-PKI in der organisatorischen Struktur des Organisationskontos.
Bestehende Organisationsdaten könnten dabei durch die von der CA ausgestellten Zertifikate abgesichert und mittels Transparency Log nachvollziehbar überprüfbar bereitgestellt werden.

Somit könnten die Vorteile von Option 1 und Option 3, sowie die in [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md) beschriebenen Bedarfe für private Organisationen perspektivisch zusammengeführt werden.