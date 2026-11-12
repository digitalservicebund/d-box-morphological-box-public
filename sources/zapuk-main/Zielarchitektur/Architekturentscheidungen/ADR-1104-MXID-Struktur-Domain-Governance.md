# ADR-1104: MXID-Struktur und Domain-Governance

> **Status:** ✔️ Operativ entschieden | **Datum:** 11. November 2026

## Kontext und Problembeschreibung

Diese Architekturentscheidung betrachtet die Struktur der Postfachadressen und die durch die [Betriebsmodelle](../../Glossar.md#betriebsmodell) vorgegebenen Einschränkungen bei der Vergabe (vgl. [ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)). Die Entscheidung zur Trennung von [Identität](../../Glossar.md#identität) und Adresse trifft eine Folge-Entscheidung.

In Matrix bestimmt der [Homeserver](../../Glossar.md#homeserver) die Domain. Wer den Homeserver betreibt, kontrolliert den Adressraum. [Organisationen](../../Glossar.md#organisation) mit zentralem Betrieb des Matrix-Homeservers ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md) Option A/B) teilen sich die gemeinsame Domain des jeweiligen zentralen [Postfach-Backends](../../Glossar.md#postfach-backend) (vgl. Folge-Entscheidung). Organisationen mit souveränem Betrieb des Matrix-Homeservers (Option C) haben eine eigene Domain.

[Privatpersonen](../../Glossar.md#privatperson) werden dem zentralen Postfach-Backend für Privatpersonen zugeordnet und erhalten [MXIDs](../../Glossar.md#mxid) unter der entsprechenden zentralen Domain. Ein Homeserver-Betrieb durch Privatpersonen ist nicht vorgesehen.

## Entscheidungstreiber

- **Organisationsauflösung:** Organisationen müssen anhand ihrer MXID zur Organisation auflösbar sein. Die Auflösung erfolgt über die [Adressregister-Abfrage](../../Glossar.md#adressregister-abfrage). Eine direkte Ableitung der Organisation aus der MXID ist keine Anforderung.
- **Datenschutz:** MXIDs werden so gewählt, dass sie keinen direkten oder indirekten Personenbezug aufweisen (ANF_FUN_ADR_024). Privatpersonen können pseudonym auftreten (ANF_FUN_ADR_085). Eine gegebenenfalls notwendige Zuordnung zu einer handelnden Person erfolgt nur organisationsintern über [Berechtigungsverzeichnis](../../Glossar.md#berechtigungsverzeichnis), [Nachweisstelle](../../Glossar.md#nachweisstelle) oder Protokolle und wird Empfangenden nicht als Personenbezug offengelegt.
- **Föderale Interoperabilität:** Jedes Postfach-Backend muss eine MXID des [Verbunds](../../Glossar.md#verbund) eindeutig auflösen und an sie übermitteln können. Eindeutigkeit über den Verbund hinweg ergibt sich aus der Domain.
- **Migrationsfähigkeit:** Ein Domainwechsel ändert alle MXIDs. Die Adressstruktur muss einen späteren Betriebsmodellwechsel berücksichtigen.

## Betrachtete Optionen

Zu klären ist, in welcher Beziehung die MXID zur Personenidentität bzw. zur [Organisationsidentität](../../Glossar.md#organisationsidentität) steht. Daraus ergeben sich die Festlegungen zu Domain und Localpart, also zu den beiden Bestandteilen einer MXID der Form `@localpart:domain`, sowie zur Governance.

### Option A: MXID-Localpart enthält die Personen- oder Organisationsidentität

Localparts der Domain enthalten Klarnamen oder Behördenkennungen, die eine [Identifizierung](../../Glossar.md#identifizierung) der Kommunikationsteilnehmenden ermöglichen. Domains werden direkt aus der Behördenstruktur abgeleitet.

- Gut, denn der Organisationsbezug ist direkt aus der MXID lesbar, ohne Adressregister-Abfrage.

- Schlecht, denn personenbezogene Localparts widersprechen der Anforderung, dass Identifikatoren keinen direkten oder indirekten Personenbezug aufweisen dürfen (ANF_FUN_ADR_024).

- Schlecht, denn Änderungen in der Behördenstruktur (Domain) oder im Namen von Mitarbeitenden bei klarnamensbasierten Localparts (Localpart) erzwingen MXID-Migrationen. Externe Adressen verlieren dabei ihre Gültigkeit.

### Option B: MXID-Localpart ohne Personenbezug

Für natürliche Personen werden Localparts ohne direkten oder indirekten Personenbezug nach außen verwendet (gemäß ANF_FUN_ADR_024), [Funktionspostfächer](../../Glossar.md#funktionspostfach) erhalten sprechende Funktionsbezeichnungen. Die Adress-Domain ist vom Postfach-Backend abhängig. Bei zentralen Backends ist das Localpart-Schema vorgegeben, bei souveränen Backends kann der Betreiber den Localpart unter Einhaltung der [Anschlussbedingungen](../../Glossar.md#anschlussbedingungen) vollständig selbst festlegen.

- Gut, denn natürliche Personen sind über die MXID nicht identifizierbar.

- Gut, denn Funktionspostfächer können „sprechende“ Adressen haben.

- Neutral, denn die Adresse ist nicht Teil der fachlichen Identität.

- Schlecht, denn die Zuordnung der MXID zur Organisation ist nur über eine Adressregister-Abfrage eindeutig prüfbar.

### Option C: Postfachadresse ist Bestandteil der Identität

Die Postfachadresse ist Merkmal der fachlichen Identität.

- Schlecht, da Identität und Postfachadresse so nicht getrennt werden können (vgl. Folge-Entscheidung). Die Option wird nicht weiter betrachtet.

## Architekturentscheidung

Gewählte Option: **Option B (MXID-Localpart ohne Personenbezug)**, weil nur diese Option das Verbot personenbezogener Identifikatoren (ANF_FUN_ADR_024) erfüllt und die pseudonyme Nutzung durch Privatpersonen (ANF_FUN_ADR_085) ermöglicht. Das [Adressregister](../../Glossar.md#adressregister) ist die verbindliche Quelle für die Zuordnung einer MXID zu einer Organisation.

## Umsetzung der gewählten Option

### Domain

| Betriebsmodell des Matrix-Homeservers | Domain          | Beispiel                         |
|---------------------------------------|-----------------|----------------------------------|
| Zentral ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md) A/B)                | Zentrale Domain | `@orgxy_datenschutz:post.gov.de` |
| Souverän ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md) C)                 | Eigene Domain   | `@datenschutz:org-xy.de`         |

Organisationen, die den zentralen Betrieb nachnutzen, können per `.well-known`-Delegation eine eigene Domain nutzen, ohne einen eigenen Homeserver zu betreiben. Dies wird empfohlen, wenn ein späterer Wechsel zu souveränem Betrieb absehbar ist, da ein Domainwechsel alle MXIDs der Organisation ändert.[^1]

### Lokaler Adressteil (Localpart)

Der lokale Teil der Adressen (Localpart) von Mitarbeitenden bzw. Privatpersonen soll keinen Personenbezug aufweisen (ANF_FUN_ADR_024). Klarnamen, Personalnummern und unmittelbar ableitbare Personenkennungen sollen nicht verwendet werden. Für Privatpersonen greift zusätzlich die Anforderung bzgl. einer pseudonymen Nutzung des Systems (ANF_FUN_ADR_085), was in einer Folge-Entscheidung behandelt wird.

Funktionspostfächer haben keinen Personenbezug und erhalten sprechende Localparts, die den fachlichen Zweck abbilden (etwa `datenschutz`, `bauamt`).

**Localpart-Schema bei zentralen Backends.** Bei zentralem Betrieb teilen sich mehrere Organisationen eine Domain. Eindeutigkeit über Organisationen hinweg wird über ein verbundweit verbindliches Präfix-Schema sichergestellt.

| Teilnehmende      | Localpart-Schema                          | Beispiel                         |
|-------------------|-------------------------------------------|----------------------------------|
| Funktionspostfach | `@[orgpräfix]_[funktion]:post.gov.de`     | `@orgxy_datenschutz:post.gov.de` |
| Mitarbeitende     | `@[orgpräfix]_[pseudonym]:post.gov.de`    | `@orgxy_a8f3k2:post.gov.de`      |
| Privatperson      | `@[bereichspseudonym]:privat.post.gov.de` | `@x7k3j9p2:privat.post.gov.de`   |

Das Organisationspräfix `orgpräfix` ist je Organisation eindeutig im Verbund (vgl. [ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)) und wird im [SSP](../../Glossar.md#self-service-portal) registriert.

**Localpart bei souveränen Backends.** Bei souveränem Betrieb betreibt die Organisation einen eigenen Homeserver, der die Vergabe einer eigenen verbundweit eindeutigen Domain ermöglicht. Die Organisation erhält somit die vollständige Freiheit bei der Benennung des Localpart. Empfohlen sind sprechende Bezeichnungen für Funktionspostfächer, etwa `@datenschutz:org-xy.de`.

Bei Funktionspostfächern kann die MXID einen sprechenden Localpart enthalten. Dieser ersetzt keine Prüfung der Organisationszuordnung über eine Adressregister-Abfrage. Bei Mitarbeitenden und Privatpersonen darf aus der MXID kein direkter oder indirekter Personenbezug ableitbar sein. Die Bindung einer Personen- oder Organisationsidentität an die Matrix-Kommunikation ist nicht Gegenstand dieser ADR.

### Governance

Die Registrierung des Organisationspräfixes erfolgt über das Self-Service-Portal bei der Organisationsregistrierung. Organisationen mit eigener Domain registrieren diese ebenfalls im SSP. Die Registrierung der technischen Vertrauensinformationen zu Matrix-Domain, Postfach-Backend, Matrix-Homeserver und Adressregister erfolgt gemäß [ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md). Die zugrunde liegenden PKI-Vertrauensanker werden gesondert konkretisiert.

Das SSP stellt die Eindeutigkeit der Organisationspräfixe sicher. Die Zuordnung einzelner Postfachadressen zu Organisationsidentitäten wird nicht in dieser ADR entschieden.

Im Konzept der gematik zum TI-Messenger werden die MXIDs vom [Org-Admin](../../Glossar.md#org-admin) angelegt und im “User-Directory” gelistet. Sie sind über die “.well-known”-Konfiguration den Organisationsdomains zugeordnet.[^2]

## Auswirkungen

- Zentrale Postfach-Backends im Verbund implementieren das hier festgelegte Localpart-Schema mit Präfix. Souveräne Backends können den Localpart vollständig selbst festlegen, wobei sie die Vorschriften der Anschlussbedingungen einhalten müssen.

- Das SSP übernimmt die Eindeutigkeitsprüfung der Organisationspräfixe und die Registrierung selbst betriebener Domains.

- Zentrale Backends können `.well-known`-Delegation anbieten, damit Organisationen ohne eigenen Homeserver eine eigene Domain führen können.

- Privatpersonen nutzen das zentrale Postfach-Backend, wodurch die Entscheidungen zur Infrastrukturtopologie ([ADR-0002](./ADR-0002-Infrastrukturtopologie.md)) und zur Bereitstellung von Postfach-Backends durch Dritte ([ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md)) eingeschränkt werden.

- [Fachverfahren](../../Glossar.md#fachverfahren) und [Empfänger](../../Glossar.md#empfängerin) leiten den Organisationsbezug einer MXID nicht aus der Adresse ab, sondern aus der Adressregister-Abfrage. Sie erfolgt pro Adresse, weil auch der Status des Eintrags zu prüfen ist (vgl. Folge-Entscheidung).

## Risiken

- Bei einem Wechsel des Betriebsmodells ohne `.well-known`-Delegation müssen alle Adressen (MXIDs) des Backends migriert werden. Für die Migration ist gegebenenfalls auch ein Transfer der Event-Datenbank zwischen den Backends notwendig.

## Folge-Entscheidungen

- Pseudonym-Adressierung von Privatpersonen, siehe [ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md).

- Identitätsbindung, siehe [ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md) und [ADR-1107](./ADR-1107-Nachweisformat-Identitaetsnachweis-Matrix-Nachrichten.md).

- PKI-Vertrauensanker, siehe [ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md).

- Quelle, Lebenszyklus und Auflösung von Postfachadressen, siehe [ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md).

- Berechtigungsverzeichnis, siehe [ADR-1112](./ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md).

- Entkopplung von [Nutzerkonto](../../Glossar.md#nutzerkonto) und [Postfach](../../Glossar.md#postfach), siehe [ADR-1110](./ADR-1110-Entkopplung-von-Nutzerkonto-und-Postfach.md).

- Getrennter Betrieb der zentralen Postfach-Backends nach [Teilnehmendengruppen](../../Glossar.md#teilnehmendengruppe), siehe [ADR-1114](./ADR-1114-Getrennter-Betrieb-der-zentralen-Postfach-Backends-nach-Teilnehmendengruppen.md).

## Referenzen

[^1]: [Matrix Spec, Server-Discovery via .well-known URIs](https://spec.matrix.org/latest/client-server-api/#well-known-uris), abgerufen am 11. Mai 2026.

[^2]: [gematik, gemSpec_TI-M_Basis V1.2.0 (Akteure/Nutzer-Account, User-Directory, Server-Discovery via .well-known)](https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.2.0/), abgerufen am 21. Juni 2026
