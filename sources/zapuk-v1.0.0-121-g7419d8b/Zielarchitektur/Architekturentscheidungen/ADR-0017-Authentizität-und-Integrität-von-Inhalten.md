# ADR-0017: Authentizität und Integrität von Inhalten

> **Status:** ✔️ Operativ entschieden | **Datum:** 24. September 2026

## Kontext und Problembeschreibung

Rechtssichere Verwaltungsdokumente (z.B. [Bescheide](../../Glossar.md#bescheid)) benötigen Nachweise, die die Unveränderlichkeit und Autorenschaft langfristig bestätigen können.

Innerhalb des E2EE-geschützten Kommunikationskanals gewährleisten [ADR-0012](./ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md) und [ADR-0013](./ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md) die [Integrität](../../Glossar.md#integrität) und [Authentizität](../../Glossar.md#authentizität) von [Nachrichten](../../Glossar.md#nachricht) gegenüber den Kommunikationsparteien. Diese kryptografischen Eigenschaften sind an den Kommunikationskanal gebunden und werden unter Wahrung der [Plausible Deniability](../../Glossar.md#plausible-deniability) realisiert (SIC_AUT_ÜBE_014). Sie erzeugen keinen Beweis gegenüber Dritten.

Für den Schriftformersatz ist eine Beweiskraft gegenüber Dritten erforderlich. Diese wird durch [qualifizierte elektronische Signaturen](../../Glossar.md#qualifizierte-elektronische-signatur) bzw. Siegel hergestellt.

**Abgrenzung:**

- **[ADR-0012](./ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md), [ADR-0013](./ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md):** [Ende-zu-Ende-Verschlüsselung](../../Glossar.md#ende-zu-ende-verschlüsselung) und Authentizität im Kommunikationskanal. Kein Beweis gegenüber Dritten.
- **ADR-0017 (dieser ADR):** Authentizität und Integrität des Dokumenteninhalts durch [qualifiziertes elektronisches Siegel](../../Glossar.md#qualifiziertes-elektronisches-siegel) vor dem Versand.
- **Bereitstellungsbestätigung im Transportkanal, Verfügbarkeitsnachweise für Anhänge sowie Abruf- und Lesebestätigungen** sind Gegenstand eigener Folge-Entscheidungen.

## Entscheidungstreiber

- **SIC_AUT_ÜBE_013:** Das System ermöglicht die qualifizierte elektronische Signatur oder qualifiziertes elektronisches Siegeln von Nachrichten.

- **ANF_FUN_ZUS_044:** Das System unterstützt die rechtssichere Zustellung/Bekanntgabe von [Verwaltungsakten](../../Glossar.md#verwaltungsakt).

- **SIC_AUT_ÜBE_014:** Die Ende-zu-Ende-Authentizität wird unter Wahrung glaubwürdiger Abstreitbarkeit (Plausible Deniability) realisiert.

- **SIC_VER_ÜBE_032:** Die Verschlüsselung kann weder von der Betreiberin des Systems noch von Dritten gebrochen oder umgangen werden.

- Der Bund stellt einen zentralen Siegeldienst bereit.[^1] Die OZG-Rahmenarchitektur enthält den Funktionsbaustein „Siegel und Signieren”.[^2]

## Betrachtete Optionen

### Option A: System als Signaturdienst

Ein zentraler Dienst siegelt bzw. signiert Dokumente.

- Gut, denn der [Absender](../../Glossar.md#absenderin) benötigt keinen eigenen Zugang zu einem Vertrauensdienst.

- Schlecht, denn das System müsste als Vertrauensdiensteanbieter zertifiziert werden.

- Schlecht, denn bei E2EE-verschlüsseltem Transport müsste das System Zugriff auf den Klartext haben, um zu siegeln. Das widerspricht [ADR-0012](./ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md) und SIC_VER_ÜBE_032.

### Option B: Siegelung durch den Absender vor dem Versand

Der Absender siegelt oder signiert das Dokument eigenverantwortlich. Die Verifizierung muss vom [Empfänger](../../Glossar.md#empfängerin) vorgenommen werden.

- Gut, denn das System benötigt keinen Zugriff auf den Klartext und bleibt E2EE-konform.

- Gut, denn Kommunikation ohne Siegelung bleibt möglich.

- Gut, denn bestehende Vertrauensdienste werden genutzt.

- Gut, denn das Verfahren ist unabhängig von der Nutzergruppe. Auch [Privatpersonen](../../Glossar.md#privatperson) signieren perspektivisch mit QES.[^3]

- Schlecht, denn die Verantwortung für die Siegelung oder Signatur liegt beim Absender. Das System kann nicht sicherstellen, dass rechtlich relevante Dokumente tatsächlich gesiegelt oder signiert wurden.

## Architekturentscheidung

Gewählte Option B: **Siegelung durch den Absender vor dem Versand**, weil

- nur diese Option die Siegelung ermöglicht, ohne die Ende-zu-Ende-Verschlüsselung zu brechen.

- der Bund mit dem Siegeldienst gemäß § 2a EGovG bereits die notwendige Infrastruktur betreibt.

Das System selbst ist weder Signaturdienst noch Vertrauensdiensteanbieter. Gesiegelte und signierte Dokumente werden als [Anhänge](../../Glossar.md#anhang) transportiert. Die Validierung erfolgt empfängerseitig, z.B. durch Prüfung [elektronischer Siegel](../../Glossar.md#elektronisches-siegel) im PDF-Reader.

## Auswirkungen

- Absender siegeln oder signieren Dokumente vor dem Versand eigenverantwortlich. Dies betrifft [Fachverfahren](../../Glossar.md#fachverfahren), Unternehmensanwendungen und perspektivisch Privatpersonen. Das System stellt an Anhänge keine Anforderungen hinsichtlich einer vorhandenen Siegelung oder Signatur. Auch unsignierte Dokumente werden transportiert.

- Die Haftung für die inhaltliche Echtheit verbleibt beim Absender und dem qualifizierten Vertrauensdiensteanbieter.

- Eine Langzeitverifizierbarkeit gesiegelter Dokumente liegt in Verantwortung des Vertrauensdiensteanbieters.

- Den Signaturstatus von Dokumenten im zentralen [Postfach-Zugang](../../Glossar.md#postfach-zugang) anzuzeigen ist perspektivisch denkbar und sinnvoll.

## Risiken

- **Privatpersonen ohne QES-Zugang:** Die breite Nutzung qualifizierter Signaturen durch Privatpersonen ist erst mit der [EUDI-Wallet](../../Glossar.md#europäische-brieftasche-für-die-digitale-identität) erreichbar.

## Referenzen

[^1]: [§ 2a EGovG Siegeldienst; Verordnungsermächtigung (abgerufen am 11.5.2026)](https://www.gesetze-im-internet.de/egovg/__2a.html)

[^2]: [Für eine zukunftsfähige Verwaltung: Das “Zielbild der OZG-Rahmenarchitektur” (abgerufen am 11.5.2026)](https://www.digitale-verwaltung.de/Webs/DV/DE/onlinezugangsgesetz/rahmenarchitektur/ozg-rahmenarchitektur/ozg-rahmenarchitektur-node.html)

[^3]: [BSI: Handlungsleitfaden zur Anbindung der EUDI-Wallet (Version 0.6, abgerufen am 11.5.2026)](https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/ElekAusweise/Handlungsleitfaden_EUDI-Wallet.pdf)
