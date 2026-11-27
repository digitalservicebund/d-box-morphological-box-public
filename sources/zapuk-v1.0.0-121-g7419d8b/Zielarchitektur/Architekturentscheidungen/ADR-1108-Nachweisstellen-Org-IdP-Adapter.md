# ADR-1108: Nachweisstellen und Org-IdP-Adapter

> **Status:** ✔️ Operativ entschieden | **Datum:** 11. November 2026

## Kontext und Problembeschreibung

In der Zielarchitektur werden [Identitätsnachweise](../../Glossar.md#fachlicher-identitätsnachweis) der Kommunikationsteilnehmer für [Authentisierungssysteme](../../Glossar.md#authentisierungssystem) ohne [Holder-Binding](../../Glossar.md#holder-binding) durch zugelassene [Nachweisstellen](../../Glossar.md#nachweisstelle) ausgestellt ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)). Bei [eID](../../Glossar.md#eid-funktion) und [EUDI-Wallet](../../Glossar.md#europäische-brieftasche-für-die-digitale-identität) ist keine Nachweisstelle notwendig. Die Bindung entsteht dort bei der direkten [Authentisierung](../../Glossar.md#authentisierung) gegenüber der [Behörde](../../Glossar.md#behörde) ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)). Authentisierungssysteme für diese Nachweise sind die [BundID](../../Glossar.md#bundid) für [Privatpersonen](../../Glossar.md#privatperson) und das [Org-Konto](../../Glossar.md#org-konto) für [Organisationen](../../Glossar.md#organisation) (vgl. Folge-Entscheidung). Die Prüfung auf Gültigkeit findet im Client der Teilnehmer statt.

Organisationen, die über ein internes Identitätsmanagement verfügen, benötigen eine Möglichkeit, intern ausgestellte Nachweise in vom [Verbund](../../Glossar.md#verbund) akzeptierte Nachweise zu überführen. Für die Prüfung der Nachweise benötigt die Zielarchitektur eine zuverlässige [Trust-Liste](../../Glossar.md#trust-liste), in der Nachweisstellen registriert werden und ihr Status geführt wird.

## Entscheidungstreiber

- **Vertrauensanker:** Die durch zugelassene Nachweisstellen ausgestellten Identitätsnachweise müssen an einen zuverlässigen [Vertrauensanker](../../Glossar.md#vertrauensanker) gebunden sein (Trust-Liste, Anforderungen in der Folge-Entscheidung PKI-Anforderungen).
- **Prüfbarkeit:** Im Verbund der Zielarchitektur zugelassene Nachweisstellen stellen die Identitätsnachweise aus, welche im jeweiligen Client geprüft werden ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md), [ADR-1107](./ADR-1107-Nachweisformat-Identitaetsnachweis-Matrix-Nachrichten.md)).
- **Dezentrales IAM:** Organisationen, die die [Identitäten](../../Glossar.md#identität) ihrer Mitarbeitenden selbst verwalten wollen, benötigen eine Möglichkeit, ihre internen Nachweise geeignet mit dem Vertrauensanker der Zielarchitektur zu verknüpfen (vgl. dezentrales [Betriebsmodell](../../Glossar.md#betriebsmodell) in [ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)).
- **Datenschutz:** Personenbezogene Daten der Mitarbeitenden und interne Informationen der Organisationen zum Beispiel zur Organisationsstruktur sollen geschützt werden (DAT_ÜBE_ÜBE_021).

## Betrachtete Optionen

### Option A: Nutzung von D-Stack-Komponenten für die Trust-Liste

Die Führung der Trust-Liste und die Registrierung der Nachweisstellen werden durch Komponenten des D-Stack abgebildet. Eine konkrete Komponente des D-Stack ist dafür derzeit nicht benannt (Annahme).

- Gut, denn die Zielarchitektur kann entwickelte Komponenten nachnutzen. Das spart Aufwände in Entwicklung und Betrieb.

- Gut, denn die Anschlusshürden sind ggf. geringer, sofern mehrere Projekte dieselbe Trust-Liste verwenden und Organisationen nur einmal registriert werden.

- Schlecht, denn die Zielarchitektur ist von der Umsetzung der D-Stack-Komponenten abhängig.

### Option B: ZaPuK-eigene Infrastruktur

Die Führung der Trust-Liste, die Registrierung der Nachweisstellen und die Anbindung der organisationsinternen Identitätsmanagementsysteme werden durch eigene Komponenten der Zielarchitektur abgebildet.

- Gut, denn die Zielarchitektur wäre unabhängig von der externen Umsetzung der Trust-Liste planbar und umsetzbar, womit der Projektfortschritt nicht extern blockiert werden könnte.

- Gut, denn die spezifischen Anforderungen, die sich aus der Zielarchitektur ergeben (z. B. bzgl. Matrix, IAM der Organisationen), können im Rahmen des Projektes [ZaPuK](../../Glossar.md#zapuk) unabhängig geplant und umgesetzt werden.

- Schlecht, denn es entstehen doppelte Aufwände in Betrieb und Entwicklung, da die notwendige Trust-Liste auch von anderen Projekten benötigt wird.

- Schlecht, denn die Anschlusshürden für Organisationen wären höher, da mehrere Projekte eigene Trust-Listen verwenden.

### Option C: D-Stack als Zielbild mit ZaPuK-Fallback und Org-IdP-Adapter

Zielbild ist, dass die Führung der Trust-Liste und die Registrierung der Nachweisstellen durch Komponenten des D-Stack abgebildet werden. Bis zur Abdeckung der notwendigen Anforderungen im D-Stack übernehmen Fallback-Lösungen der Zielarchitektur die Funktionalität. Zusätzlich werden IAM-Systeme der Organisationen über einen [Org-IdP-Adapter](../../Glossar.md#org-idp-adapter) angebunden, der interne Identitätsnachweise in für den Verbund gültige Nachweise überführt.

- Gut, denn die Zielarchitektur kann die im D-Stack entwickelten Komponenten nachnutzen. Das spart Aufwände in Entwicklung und Betrieb.

- Gut, denn die Anschlusshürden sind ggf. geringer, sofern mehrere Projekte dieselbe Trust-Liste verwenden und Organisationen nur einmal registriert werden.

- Gut, denn der Fortschritt der Zielarchitektur wäre nicht an den Fortschritt des D-Stack gekoppelt, da fehlende Funktionalität durch Fallback-Komponenten der Zielarchitektur umgesetzt wird.

- Gut, denn die spezifischen Anforderungen aus der Nutzung eines organisationsinternen Identitätsmanagements werden mit einem Org-IdP-Adapter umsetzbar.

- Schlecht, denn die Org-IdP-Adapter verursachen zusätzliche Entwicklungs- und Betriebsaufwände.

## Architekturentscheidung

Gewählte Option: **Option C, D-Stack als Zielbild mit ZaPuK-Fallback und Org-IdP-Adapter**, weil die Zielarchitektur unabhängig vom Zeitplan des D-Stack umsetzbar sein soll und die Komponenten des D-Stack nachgenutzt werden können. Dadurch sinken die Aufwände in Entwicklung und Betrieb. Organisationen können ihr internes Identitätsmanagement zudem über den Org-IdP-Adapter anbinden.

## Umsetzung der gewählten Option

Die Trust-Liste nutzt einen eigenen Vertrauensanker des Verbunds (vgl. Folge-Entscheidung PKI-Anforderungen) und listet die zugelassenen Nachweisstellen jeweils mit ihrem Schlüssel, ihrer Gültigkeit und ihrem Status. Die Liste wird im [Self-Service-Portal](../../Glossar.md#self-service-portal) (SSP) geführt, bis der D-Stack eine geeignete Vertrauensinfrastruktur anbietet.

Für Kommunikationsteilnehmer ohne eigenes Identitätsmanagement überführt die zentrale Nachweisstelle die Assertions der Authentisierungssysteme (BundID, Org-Konto) in Identitätsnachweise im Format nach [ADR-1107](./ADR-1107-Nachweisformat-Identitaetsnachweis-Matrix-Nachrichten.md). Zu jedem zentralen [Postfach-Backend](../../Glossar.md#postfach-backend) (vgl. Folge-Entscheidung) wird eine Instanz der zentralen Nachweisstelle betrieben. Die Instanz für Privatpersonen erfährt bei jeder Nachweisausstellung Identität, [Bereichs-MXID](../../Glossar.md#bereichs-mxid) und Zeitpunkt. Ihr Betreiber darf weder mit dem des Postfach-Backends für Privatpersonen noch mit dem der BundID identisch sein. Zusammen mit dem Postfach-Backend könnte er die Identitäten mit den Kommunikationsbeziehungen verknüpfen. Zusammen mit der BundID könnte er bei Option 2E auch die [Bereiche](../../Glossar.md#bereich) zuordnen (vgl. [ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md)). Ist eine Betreibertrennung nicht möglich, müssen Nachweisstelle und Postfach-Backend anderweitig strikt getrennt werden. Das geschieht etwa durch Mandantentrennung, unterschiedliche Schlüssel und eine getrennte Betriebsumgebung. Die Trennung ist Teil der [Anschlussbedingungen](../../Glossar.md#anschlussbedingungen) (vgl. Folge-Entscheidung). Zusammen mit der Registrierung der Postfach-Backends wird die Nachweisstelle in die zentrale Trust-Liste aufgenommen.

Interne IAM-Systeme der Organisationen werden über die zu entwickelnde Komponente Org-IdP-Adapter angebunden. Eine Instanz eines Org-IdP-Adapters ist eine eigene Nachweisstelle und wird in der Trust-Liste mit ihrem öffentlichen Schlüssel im Rahmen des Onboardings über das Self-Service-Portal registriert (vgl. Folge-Entscheidung Anschlussbedingungen).

Der Org-IdP-Adapter dient der kontrollierten Delegation der Identitäten in den Verbund. Nach einer Prüfung der Berechtigung werden die intern ausgestellten Nachweise der Mitarbeitenden gegen im Verbund prüfbare Identitätsnachweise ausgetauscht, die die Organisation und ggf. das [Funktionspostfach](../../Glossar.md#funktionspostfach) ausweisen, bevor sie mit den [Nachrichten](../../Glossar.md#nachricht) übermittelt werden. Die Organisation legt selbst fest, welche Informationen der Identitätsnachweis enthält. Nicht benötigte personenbezogene Informationen sollen die Organisation nicht verlassen.

Gegenüber dem Verbund wirkt der Org-IdP-Adapter als Nachweisstelle der Organisation (Vermittlungsdienst zwischen dem organisationsinternen Identitätsmanagement und den Clients, vgl. [ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)). Die von ihm ausgestellten Nachweise werden im Client der [Empfänger](../../Glossar.md#empfängerin) lokal gegen die Trust-Liste geprüft. Über die Anschlussbedingungen (vgl. Folge-Entscheidung) ist die Verantwortung für die organisationsinterne Authentisierung an die Organisation delegiert.

Die Zero-Trust-Architektur für die Telematikinfrastruktur der gematik enthält ein ähnliches Konzept, bei dem das Authenticator-Modul ein einrichtungsinternes IAM anbindet.[^1]

## Auswirkungen

- Eine Nutzung der Vertrauensinfrastruktur (Trust-Liste, Nachweisstellen) von D-Stack-Komponenten ist möglich, sobald diese angeboten werden. Die ZaPuK-spezifischen Infrastrukturelemente können dann teilweise entfallen. Eine Transition wäre zu planen.

- Ein internes Identitätsmanagement von Organisationen kann in den Verbund integriert werden.

## Risiken

- Verzögerungen im D-Stack verlängern den Betrieb der Fallback-Komponenten. Diese bringen Komplexität und Aufwände in Entwicklung und Betrieb mit sich.

- Eine Fehlkonfiguration der Org-IdP-Adapter durch die Verantwortlichen der Organisationen kann zu unbeabsichtigt herausgegebenen Daten der Organisation oder ihrer Mitarbeitenden führen, da diese in den Identitätsnachweis übernommen werden.

- Durch eine Kompromittierung des Org-IdP-Adapters können gefälschte Identitätsnachweise für eine Organisation ausgestellt werden. Durch ein Management der Trust-Liste mit Statusführung und Möglichkeiten der Suspendierung werden Gegenmaßnahmen ermöglicht.

- Durch eine Kompromittierung der zentralen Nachweisstelle können gefälschte Identitätsnachweise für alle Kommunikationsteilnehmer des jeweiligen Postfach-Backends ausgestellt werden. Entsprechende Härtung ist zwingend (vgl. [ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)).

- Die zentrale Nachweisstelle kennt die Identität aus der Assertion sowie die sendende Bereichs-MXID und den Zeitpunkt jeder Ausstellung. Dadurch kann bei Nutzung der BundID eine Verknüpfung von Identität und Bereichs-MXID erfolgen. Betreibt dieselbe Stelle das Postfach-Backend für Privatpersonen, kommen [Vorgänge](../../Glossar.md#vorgang) und Nachrichtenverläufe hinzu (vgl. Folge-Entscheidung).

## Folge-Entscheidungen

- PKI-Anforderungen der Nachweisstellen und der Trust-Liste werden in [ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md) geregelt.

- Anschlussbedingungen für den Betrieb der Org-IdP-Adapter, einschließlich Konformitätsprüfung im Einzelfall, werden in [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md) geregelt.

- Die Entkopplung von [Nutzerkonto](../../Glossar.md#nutzerkonto) und [Postfach](../../Glossar.md#postfach) mit Nutzerkonten und Wallets als Authentisierungssystemen ohne Postfachfunktion wird in [ADR-1110](./ADR-1110-Entkopplung-von-Nutzerkonto-und-Postfach.md) geregelt.

- Der getrennte Betrieb der zentralen Postfach-Backends nach [Teilnehmendengruppen](../../Glossar.md#teilnehmendengruppe) wird in [ADR-1114](./ADR-1114-Getrennter-Betrieb-der-zentralen-Postfach-Backends-nach-Teilnehmendengruppen.md) geregelt.

## Offene Punkte

- Festlegung der Mindestanforderungen an Implementierungen von Org-IdP-Adaptern, einschließlich des [Vertrauensniveaus](../../Glossar.md#vertrauensniveau) der ausgestellten Nachweise.

- Mindestanforderungen an die zentrale Nachweisstelle (z. B. Anträge ohne Angaben zu Empfänger und Vorgang, keine Abfragen beim Postfach-Backend, keine langfristige Protokollierung der Bereichs-MXIDs).

- Festlegung des Betreibers der zentralen Nachweisstelle für Privatpersonen.

- Herausgeber der Trust-Liste (vgl. Folge-Entscheidung).

- Analyse und Einordnung des Verhältnisses der EUDI- und EU-Business-Wallet, also der Trusted Lists der Aussteller qualifizierter Attestierungen, zum Zielbild des D-Stack und zum ZaPuK-Fallback.

- Prüfung von OpenID Federation für Trust-Liste und Nachweisstellen, auch im Vergleich mit dem Federation Master der gematik.

## Referenzen

[^1]: [gematik gemKPT Zero Trust V1.0.0, §3.2.1 (Authenticator-Modul, IDP-Föderation)](https://gemspec.gematik.de/docs/gemKPT/gemKPT_Zero_Trust/gemKPT_Zero_Trust_V1.0.0/), abgerufen am 11. Mai 2026.
