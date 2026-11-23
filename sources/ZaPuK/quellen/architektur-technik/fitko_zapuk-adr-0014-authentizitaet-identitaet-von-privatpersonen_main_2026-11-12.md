<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Zielarchitektur/Architekturentscheidungen/ADR-0014-Authentizit%C3%A4t-Identit%C3%A4t-von-Privatpersonen.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-0014-Authentizität-Identität-von-Privatpersonen.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 6fe237f653); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

> **Status:** ✔️ Operativ entschieden | **Datum:** 28. April 2025

> **Hinweis (Phase 2, Stand Oktober 2026):** Diese Entscheidung wurde in Phase 2 konkretisiert. Die eID wird wie in Option 1 beschrieben genutzt. Für die BundID stellt übergangsweise eine Nachweisstelle den Identitätsnachweis aus (ADR-1106).

# ADR-0014: Authentizität: Identität von Privatpersonen
## Kontext und Problembeschreibung

Gemäß [ADR-0013](./ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md) wird Authentizität auf der Ende-zu-Ende Verschlüsselungsebene durch eine effektive Bindung fachlicher Identitäten an kryptografische Identitäten im Kommunikationskanal erreicht.
Im vorliegenden ADR wird beschrieben, wie sich Privatpersonen gegenüber anderen Kommunikationspartner:innen authentisieren können.

Durch die in [ADR-0013](./ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md) beschriebene Bindung dieser Authentisierung an den Ende-zu-Ende-verschlüsselten Kommunikationskanal beweisen Privatpersonen die Autorenschaft der von ihnen im Kanal gesendeten Nachrichten gegenüber ihrer Kommunikationspartner:in und stellen gleichzeitig die Vertraulichkeit des Kanals her.
Die Authentisierung des Kommunikationskanals erfolgt jeweils auf dem Vertrauensniveau des hierzu eingesetzten Identifizierungsmittels.

Da Identitätsdaten von Privatpersonen datenschutzrechtlich relevant und im Besonderen vor Missbrauch geschützt sind, dürfen diese zur Authentizitätsprüfung nur verschlüsselt in gesicherten Kanälen und an entsprechend berechtige Empfänger:innen übertragen werden.

## Entscheidungstreiber

- Relevante Anforderungen:
  - SIC_ÜBE_ÜBE_018: Das System setzt den Zero Trust Ansatz um.
  - SIC_AUT_ÜBE_014: Die Integrität und Ende-zu-Ende-Authentizität der übermittelten [Nachrichten] wird auf kryptographischer Ebene unter Wahrung *glaubwürdiger Abstreitbarkeit* (Plausible Deniability) realisiert, um im Falle von Kompromittierungen keine Echtheit einzelner übermittelter Informationen belegen zu können und so Individuen vor Stigmatisierung und Diskriminierung zu schützen.
  - SIC_VER_ÜBE_051: Alle Metadaten, die nicht zwingend für die korrekte Zustellung einer [Nachricht] erforderlich sind, werden vermieden, als Teil der übermittelten [Nachricht] als [vertrauliche Metadaten] verschlüsselt oder durch Privatsphäre-schützende Technologien (Privacy-enhancing Technologies, PET) verschleiert.
  - ANF_FUN_IDE_001: Die Rolle einer [Absender:in] ([öffentliche Stelle], [private Organisation], [Privatperson], etc.) muss an einer [Nachricht] erkennbar sein.
  - ANF_FUN_IDE_023: Zur [Identifikation] von [Privatpersonen] werden mindestens die in § 3 OZG genannten [Identifizierungsmittel] unterstützt. Die [Identifikation] muss mindestens über das Bürgerkonto gemäß § 3 OZG ([Bund-ID] / [DeutschlandID]) erfolgen können.
  - ANF_FUN_IDE_025: Die [Identifikation] über [EUDI-Wallet] mit starker kryptografischer Bindung muss perspektivisch möglich sein, sofern die organisatorischen und technischen Voraussetzungen hierzu geschaffen sind (vgl. eIDAS-Credential mit Anonymous Credentials, Zero Knowledge Proofs, Deniability, selbstverwaltete kryptografische Identität).
  - ANF_FUN_IDE_034: [Das System] sollte in der Lage sein, zukünftig ggf. auch weitere vom BSI festgelegte [Authentisierungsmittel] zu unterstützen, wenn der initiale [Nachweis] der [Identität] natürlicher Personen maximal auf Vertrauensniveau "substantiell" erbracht wurde. (vgl. § 12 Abs. 5 OZG)
  - ANF_FUN_IDE_044: [Das System] unterstützt die Integration bestehender europäischer Identitätslösungen ([Identitäten] nach eIDAS 1.0). 
  - ANF_FUN_IDE_027: [Das System] verwendet offene Standardprotokolle zur [Authentifizierung] und [Autorisierung] der [Nutzer:innen].

## Betrachtete Optionen

- [Option 1: Identifikation mittels elektronischen Identitätsnachweis / Personalausweis (eID)](#option-1-identifikation-mittels-elektronischen-identitätsnachweis--personalausweis-eid)
- [Option 2: Identifikation mittels zentralem Bürgerkonto gemäß OZG § 3 (BundID)](#option-2-identifikation-mittels-zentralem-bürgerkonto-gemäß--3-ozg-bundid)
- [Option 3: Identifikation mittels European Digital Identity Wallet (EUDI-Wallet)](#option-3-identifikation-mittels-european-digital-identity-wallet-eudi-wallet)

## Gegenüberstellung der jeweiligen Optionen

### Option 1: Identifikation mittels elektronischen Identitätsnachweis / Personalausweis (eID)

Zur Herstellung von Authentizität auf Ende-zu-Ende Verschlüsselungsebene können Privatpersonen sich mit einer eID-Karte wie dem Personalausweis gegenüber anderen Kommunikationsteilnehmenden identifizieren.
Dieses Verfahren erlaubt ein sehr hohes Maß an Sicherheit und Vertraulichkeit auf dem Vertrauensniveau „hoch“.

Zur Prüfung ist der Betrieb eines eID-Servers notwendig.
Wird der genutzte eID-Server durch die Kommunikationspartei betrieben, gegenüber der sich die Privatperson authentisiert, ist eine Sicherung der Authentizität nach dem Zero-Trust-Prinzip ohne Rückgriff auf externe Vertrauensstellungen möglich.
Um die Anbindung an das System leichtgewichtig zu gestalten, soll die eID-Server Funktion in ein entsprechendes SDK, welches auch insgesamt die Authentizität auf der Ende-zu-Ende Verschlüsselungsebene implementiert, gebündelt werden.
Zur Reduzierung der Betriebskomplexität wäre jedoch auch der Betrieb des eingesetzten eID-Servers durch eine vertrauenswürdige Dritte Partei möglich.

Bei dem eID-Verfahren wird eine gesicherte Verbindung zwischen einem eID-Client und einem eID-Server hergestellt.
Als eID-Client wird hierbei direkt der genutzte Postfachzugang mit integrierter AusweisApp-SDK verwendet.
Identitätsdaten werden im eID-Verfahren nach Prüfung der Berechtigung des eID-Servers über diese gesicherte Verbindung übermittelt.
Zur Bindung der Identität an den Kommunikationskanal wird das eID-Verfahren durch den Kommunikationskanal übertragen.

> [!note] Hinweis
> Das eID-Verfahren sieht eine Sicherung des zur Authentifizierung genutzten Kanals mittels TLS vor, dessen Authentizität gemäß BSI TR-03124-1 anhand des hierzu verwendeten X.509-TLS-Zertifikat und der im eID-Berechtigungszertifikat hinterlegten Informationen geprüft wird. Zur Absicherung des Ende-zu-Ende-verschlüsselten Kommunikationskanals muss dieser analog, mit Hilfe eines X.509-Zertifikats, abgesichert sein (siehe [ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md)).

> [!note] Hinweis
> Können Postfachzugänge nicht auf benötigte Funktionalitäten zur Durchführung des eID-Verfahrens zurückgreifen (da es sich beispielsweise um einen nicht NFC-fähigen Desktop Browser handelt), könnte die Authentifizierung analog zu [RFC 8628: OAuth 2.0 Device Authorization Grant](https://datatracker.ietf.org/doc/html/rfc8628) über ein Drittes (beispielsweise Mobiles) Gerät durchgeführt werden. Dadurch kann eine nahtlose Nutzentenerfahrung erreicht werden (vgl. [„Device Switch“-Architekturkonzept des DigitalService](https://github.com/digitalservicebund/useid-architecture/blob/main/research/device-switch/proposal-qr-code-based-device-switch-with-webauthn.md)).

- Gut, da über das eID-Verfahren ein hohes Sicherheitsniveau im Sinne einer Zero-Trust-Architektur erreicht werden kann
- Gut, da die eID bis zum Vertrauensniveau „hoch“ zugelassen ist
- Gut, da der Personalausweis als eID-Karte für nahezu jede Privatperson verfügbar und die Infrastruktur somit leicht nutzbar ist
- Neutral, da die Integration des eID-Verfahrens in den Ende-zu-Ende-verschlüsselten Kanal gemäß [ADR-0012](./ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md) zunächst noch entwickelt werden muss
- Neutral, da eID-Server Infrastruktur im Sinne einer Zero-Trust-Architektur noch nicht großflächig etabliert ist

### Option 2: Identifikation mittels zentralem Bürgerkonto gemäß § 3 OZG (BundID)

Zur Herstellung von Authentizität auf Ende-zu-Ende Verschlüsselungsebene können Privatpersonen sich mit einem BundID-Konto gegenüber einem anderen Kommunikationsteilnehmenden identifizieren.
Hierbei können nachgelagert verschiedene Authentifizierungsmittel zum Einsatz kommen (wie Passwörter, ELSTER-Zertifikate, eID, eIDAS-Identitäten), entsprechend derer dieses Verfahren Sicherheit und Vertraulichkeit auf den Vertrauensniveaus „normal“, „substantiell“ und „hoch“ erlaubt.

Zur Authentisierung wird ein von der BundID-Infrastruktur ausgestelltes (SAML) Token vertraulich im Kontext des Ende-zu-Ende verschlüsselten Kommunikationskanals übermittelt und durch einen anderen Kommunikationsteilnehmenden geprüft.
Entsprechend ist für die Prüfung eine Vertrauensstellung gegenüber dem BundID-Server erforderlich (kein Zero-Trust-Ansatz).

Um eine kryptografisch sichere Bindung zu erreichen, muss der (SAML) Token, wie in [ADR-0013](./ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md) generell beschrieben, auf den betreffenden Kommunikationskanal limitiert sein.
Ebenfalls sollten Vorkehrungen getroffen werden, um Abstreitbarkeit für im Token enthaltene personenbezogene Daten zu erreichen, damit eine missbräuchliche Nutzung dieser erschwert wird.

- Gut, da die BundID bis zum Vertrauensniveau „hoch“ zugelassen ist
- Schlecht, da Kommunikationsparteien auf die korrekte Authentifizierung durch die BundID-Infrastruktur vertrauen müssen (keine Zero-Trust-Architektur)

### Option 3: Identifikation mittels European Digital Identity Wallet (EUDI-Wallet)

Perspektivisch sollen sich Privatpersonen zur Herstellung von Authentizität auf Ende-zu-Ende Verschlüsselungsebene direkt mit einem entsprechenden Nachweis aus der European Digital Identity Wallet gegenüber einem anderen Kommunikationsteilnehmenden identifizieren können.
Dieses Verfahren erlaubt zukünftig ein hohes Maß an Sicherheit und Vertraulichkeit.

Um Identitätsdaten aus der EUDI-Wallet sicher und unter Berücksichtigung der Anforderungen an den Ende-zu-Ende verschlüsselten Kanal zu binden, sind maßgeblich kryptografische Eigenschaften notwendig, die in der öffentlichen Konsultation auf EU-Ebene ([Cryptographers’ Feedback on the EU Digital Identity’s ARF](https://files.dyne.org/dynebolic/?file=eudi/cryptographers-feedback-june2024.pdf)) aufgezeigt wurden.
Ebenfalls sollte das Postfachsystem den Einsatz der perspektivisch benötigten Berechtigungszertifikate bei prüfenden Kommunikationsteilnehmern vorsehen und entsprechende Funktionalität analog zur eID in einem entsprechendem SDK bereitstellen.

- Gut, da mit der EUDI-Wallet perspektivisch ein hohes Sicherheitsniveau im Sinne einer Zero-Trust-Architektur erreicht werden kann
- Gut, da die perspektivische Unterstützung der EUDI-Wallet das Postfachsystem zukunftssicher, einfach nutzbar und in Europa interoperabel gestaltet
- Schlecht, denn die EUDI-Wallet ist noch nicht verfügbar

## Architekturentscheidung

Gewählte Optionen:
**Option 1: Identifikation mittels elektronischen Identitätsnachweis / Personalausweis (eID)**,
**Option 2: Identifikation mittels zentralem Bürgerkonto gemäß § 3 OZG (BundID)** und
**Option 3: Identifikation mittels European Digital Identity Wallet (EUDI-Wallet)**.

Sowohl eID als auch perspektivisch die EUDI-Wallet bieten kryptografisch sichere Identifikation zur Authentisierung des Kommunikationskanals unter Wahrung des Zero-Trust Ansatzes.
Die BundID kann diesen als zentrale Vermittlungsplattform für Identifikationsvorgänge aktuell nicht umsetzen, erlaubt jedoch einen niederschwelligen Zugang zu dem Postfachsystem, mit verschiedenen Authentifizierungsmitteln für verschiedene Anwendungsfälle und Vertrauensniveaus (z.B. Nutzername/Passwort, ELSTER oder EU-Identitäten gemäß eIDAS 1.0).