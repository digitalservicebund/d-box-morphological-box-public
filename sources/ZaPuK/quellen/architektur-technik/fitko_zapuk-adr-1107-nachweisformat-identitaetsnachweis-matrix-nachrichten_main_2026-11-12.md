<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Zielarchitektur/Architekturentscheidungen/ADR-1107-Nachweisformat-Identitaetsnachweis-Matrix-Nachrichten.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-1107-Nachweisformat-Identitaetsnachweis-Matrix-Nachrichten.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 0b088e91f3); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# ADR-1107: Nachweisformat des fachlichen Identitätsnachweises

> **Status:** ✔️ Operativ entschieden | **Datum:** 12. November 2026

## Kontext und Problembeschreibung

Die in der Kommunikation übermittelten [Identitätsnachweise](../../Glossar.md#fachlicher-identitätsnachweis) verknüpfen die [Identität](../../Glossar.md#identität) des [Absenders](../../Glossar.md#absenderin) und das [Vertrauensniveau](../../Glossar.md#vertrauensniveau) mit den [Nachrichten](../../Glossar.md#nachricht) ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)). Die Echtheit des Nachweises ist über den Schlüssel der [Nachweisstelle](../../Glossar.md#nachweisstelle) prüfbar. Die asynchrone Prüfung der Nachweise durch die Empfänger-Clients muss auch noch lange nach deren Erhalt und auch durch später hinzukommende Raummitglieder möglich sein.

In dieser Architekturentscheidung werden unterschiedliche Formate für den Identitätsnachweis betrachtet. Die Wahl des Standards legt die Struktur der Angaben im Nachweis fest. Sie bestimmt außerdem, welche Bibliotheken für die Umsetzung bereitstehen und welche Funktionen wie die selektive Offenlegung möglich sind.

Die [Authentifizierung](../../Glossar.md#authentifizierung) gegenüber dem [Postfach-Backend](../../Glossar.md#postfach-backend) wird hier nicht behandelt. Das für Nachweise der Nachweisstellen festgelegte Format ist mit der [EUDI-Wallet](../../Glossar.md#europäische-brieftasche-für-die-digitale-identität) kompatibel.

## Entscheidungstreiber

- **Prüfbarkeit:** Die Identität des Absenders einer Nachricht sowie die Gültigkeit des Identitätsnachweises zum Versandzeitpunkt sollen prüfbar sein (SIC_AUT_ÜBE_007, SIC_AUT_ÜBE_008, SIC_ÜBE_ÜBE_008, vgl. [ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)).
- **Asynchrone Prüfung:** Die Identitätsnachweise müssen auch noch lange nach dem Versand prüfbar bleiben (vgl. [ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)).
- **Datenminimierung und Nichtverkettbarkeit:** Das Nachweisformat soll entsprechend dem Prinzip Privacy by Design nur notwendige Daten enthalten (ANF_FUN_ADR_024, DAT_ÜBE_ÜBE_021).
- **Anschlussfähigkeit:** Die [Identifikation](../../Glossar.md#identifizierung) über die EUDI-Wallet muss perspektivisch ermöglicht werden (ANF_FUN_IDE_025).

## Betrachtete Optionen

### Option A: Signiertes JWT

Der Identitätsnachweis wird als signiertes JSON Web Token transportiert, also als JWS (RFC 7515) mit JWT-Payload (RFC 7519). Über einen `cnf`-Claim (RFC 7800) kann eine Bindung an die Schlüsselidentität des Absenders umgesetzt werden.

- Gut, denn die notwendigen Attribute wie z. B. Nachweisstelle, Identität und Vertrauensniveau können mittels Claims abgebildet werden. 

- Gut, denn die Nachweise sind offline prüfbar, sofern notwendige Schlüssel vorliegen.

- Gut, denn der Standard ist weit verbreitet, was die Implementierung vereinfacht.

- Schlecht, denn die Funktionalität zur selektiven Offenlegung einzelner Attribute ist im Standard nicht vorgesehen und wäre nur mit einer eigenen Implementierung umsetzbar.

- Schlecht, denn wie der Besitz des Schlüssels bei der Vorlage nachgewiesen wird, ist für JWT nicht standardisiert.

### Option B: SD-JWT VC

Der Identitätsnachweis wird mit dem Standard SD-JWT VC umgesetzt. Dieser baut auf SD-JWT (RFC 9901)[^1] auf, das JWT (RFC 7519) um die selektive Offenlegung erweitert.[^2] Über einen `cnf`-Claim (RFC 7800) kann eine Bindung an die Schlüsselidentität des Absenders umgesetzt werden.

- Gut, denn die notwendigen Attribute wie z. B. Nachweisstelle, Identität und Vertrauensniveau können mittels Claims abgebildet werden. 

- Gut, denn die Nachweise sind offline prüfbar, sofern notwendige Schlüssel vorliegen.

- Gut, denn die selektive Offenlegung von Claims ist im Standard spezifiziert.

- Gut, denn der Besitznachweis bei der Vorlage (Key-Binding-JWT) ist im Standard spezifiziert.

- Gut, denn SD-JWT VC spezifiziert Verifiable Credentials, was die Nutzung der EUDI-Wallet erleichtert.

- Schlecht, denn die Implementierung ist aufwändiger als einfaches JWT.

### Option C: X.509-Zertifikat oder Attributzertifikat

Der Identitätsnachweis wird als X.509-Zertifikat (RFC 5280) umgesetzt. Die notwendigen fachlichen Angaben sind im Zertifikat, in Erweiterungen oder in Attributzertifikaten (RFC 5755) angegeben.

- Gut, denn die notwendigen Attribute wie z. B. Nachweisstelle, Identität und Vertrauensniveau können in Attributen oder Attributzertifikaten übermittelt werden.

- Gut, denn die Nachweise sind offline prüfbar, sofern notwendige Schlüssel vorliegen.

- Gut, denn X.509 ist etablierter Standard für technische Identitäten, [Organisationen](../../Glossar.md#organisation) und Backend-zu-Backend-Kommunikation.

- Schlecht, denn eine selektive Offenlegung einzelner Attribute ist bei X.509 nicht vorgesehen.

- Schlecht, denn X.509-Zertifikate sind üblicherweise langlebig und müssten zum Beispiel bei Wechsel des Vertrauensniveaus neu erzeugt werden, was hohe Aufwände verursacht.

- Schlecht, denn um eine Verkettung von Nachrichten über [Bereiche](../../Glossar.md#bereich) hinweg zu verhindern, müssten bereichsspezifische Zertifikate verwendet werden.

- Schlecht, denn X.509 ist kein Verifiable-Credential-Format.

### Option D: Umsetzung einer ZaPuK-spezifischen Lösung

Der Identitätsnachweis wird mit einem eigenen JSON-Format und eigenem Prüfverfahren umgesetzt.

- Gut, denn die Lösung wäre auf die spezifischen Anforderungen zugeschnitten.

- Schlecht, denn eine Eigenentwicklung verursacht im Gegensatz zur Nutzung öffentlicher Standards hohe Aufwände in der Entwicklung und weiteren Pflege.

## Architekturentscheidung

Gewählte Option: **Option B, SD-JWT VC**, weil sie die selektive Offenlegung der enthaltenen Claims bietet und die Anbindung der EUDI-Wallet erleichtert, da diese das Format ebenfalls unterstützt.

Gegen die Wahl von JWT spricht, dass eine selektive Offenlegung im Standard nicht vorgesehen ist. Gegen X.509-Zertifikate sprechen die Probleme bei der Pseudonymisierung, wobei sie für die Anbindung von Organisationen und bei der Backend-zu-Backend-Kommunikation relevant bleiben. Ein ZaPuK-spezifisches Format wäre aufwändig zu spezifizieren und würde die Umsetzung erschweren.

ISO mdoc und W3C-VC mit Data Integrity wurden nicht weiter betrachtet. ISO mdoc dient der Vorlage von Nachweisen innerhalb einer Sitzung zwischen Wallet und Prüfer, an die der Besitznachweis gebunden wird. Dagegen wird der Identitätsnachweis der Zielarchitektur mit einer Nachricht übermittelt und kann langfristig auch von mehreren [Empfängern](../../Glossar.md#empfängerin) geprüft werden. W3C-VC mit Data Integrity wird von der EUDI-Wallet für Identitätsdaten nicht unterstützt.

## Umsetzung der gewählten Option

Das Schema des Identitätsnachweises umfasst die fachliche Identität des Absenders, die Referenz zur Nachweisstelle, das Vertrauensniveau, den Zeitpunkt der Ausstellung und den Schlüssel des Absenders. Die [MXID](../../Glossar.md#mxid) des Absenders enthält der Nachweis nicht, damit die Nachweisstelle sie nicht erfährt. Vor Übermittlung des Nachweises signiert der Client mit seinem Schlüssel einen Zusatz (Key-Binding-JWT), der den Kontextbezug (Empfänger, [Vorgang](../../Glossar.md#vorgang)), die MXID des Absenders, den Zeitpunkt und einen Hashwert des Nachweises enthält ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)). Der Empfänger vergleicht diese MXID mit dem Absender des Events. Dadurch wird der Nachweis zwar an den Kontext und den Schlüssel des Absenders gebunden, nicht aber an den Inhalt der Nachricht, womit Nachrichten weiterhin abstreitbar bleiben ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)).

Je nach Anwendungsfall legt der Absender nur die benötigten Angaben (Claims) offen.

Für [Privatpersonen](../../Glossar.md#privatperson) werden bereichsübergreifende Kennungen vermieden. Auch die [Bereichs-MXID](../../Glossar.md#bereichs-mxid) ist im Nachweis nicht enthalten ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md)).

Der Identitätsnachweis enthält keine Signatur über den Inhalt der übermittelten Nachricht und dient nicht als Beweismittel für eine erfolgreiche Übermittlung (vgl. [ADR-0017](./ADR-0017-Authentizität-und-Integrität-von-Inhalten.md), [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

## Auswirkungen

- Die Nachweise werden als SD-JWT VC mit einem festgelegten Schema ausgestellt.

- [Fachverfahren](../../Glossar.md#fachverfahren) legen fest, welche Angaben sie anfordern.

## Risiken

- Durch die selektive Offenlegung müssen Empfänger-Clients prüfen, ob die notwendigen Angaben im Nachweis enthalten sind.

- Bei Privatpersonen dürfen keine stabilen, empfängerübergreifenden Kennungen in den Angaben enthalten sein, die eine Verkettung von Nachrichten ermöglichen könnten.

- Kurzlebige Nachweise und der vom Client gesetzte Bezug zum Vorgang mindern Risiken bzgl. der Verkettung. Der Nachweisstelle liegen allerdings Identität, gebundener Schlüssel und Zeitpunkt jedes Nachweises vor. Die Bereichs-MXID muss ihr so nicht bekannt gemacht werden. Wenn der gebundene Schlüssel der Bereichs-MXID zugeordnet wäre, könnte ein gemeinsamer Betreiber von Postfach-Backend und der Nachweisstelle die Bereichs-MXIDs einer Person der Identität zuordnen. Über IP-Adressen und Zeitpunkte ist das auch sonst möglich ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md)).

- SD-JWT VC ist noch ein Entwurf der IETF. Änderungen bis zur Verabschiedung können Anpassungen erfordern.

## Folge-Entscheidungen

- [Trust-Liste](../../Glossar.md#trust-liste), Sperrlisten und Schlüssel der Nachweisstellen, siehe [ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md).

## Offene Punkte

- Festlegung des Nachweisschemas, Definition notwendiger Pflichtangaben je Anwendungsfall.

- Anzeige des Identitätsnachweises im Empfänger-Client.

- Entwurf EUDI-Wallet-Anbindung mit definiertem OpenID4VP-Prozess.

- Prüfung, ob die Nachweisstelle für Privatpersonen mit Zero-Knowledge-Proofs Nachweise ausstellen kann, deren Besitz je Nachricht belegt wird. Die Zeitpunkte der Nutzung erfährt die Nachweisstelle dann nicht. Die Unterstützung durch EUDI-Wallet und die verwendeten Standards sowie Fragen wie die Aktualisierung der Attribute und die Sperrung wären zu klären.

## Referenzen

[^1]: [IETF RFC 9901, Selective Disclosure for JWTs (SD-JWT), November 2025](https://www.rfc-editor.org/info/rfc9901)

[^2]: [IETF draft-ietf-oauth-sd-jwt-vc-16, SD-JWT-based Verifiable Digital Credentials (SD-JWT VC), 24. April 2026](https://datatracker.ietf.org/doc/draft-ietf-oauth-sd-jwt-vc/)
