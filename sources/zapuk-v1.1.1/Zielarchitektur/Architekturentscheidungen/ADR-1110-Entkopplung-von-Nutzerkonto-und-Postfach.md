# ADR-1110: Entkopplung von Nutzerkonto und Postfach

> **Status:** ✔️ Operativ entschieden | **Datum:** 26. November 2026

## Kontext und Problembeschreibung

Nach dem OZG ist ein [Nutzerkonto](../../Glossar.md#nutzerkonto) eine zentrale IT-Komponente zur [Identifizierung](../../Glossar.md#identifizierung) und [Authentifizierung](../../Glossar.md#authentifizierung) der [Nutzer](../../Glossar.md#nutzerinnen) mit einem [Postfach](../../Glossar.md#postfach) (§ 2 OZG).[^1] Das bestehende Bürgerkonto [BundID](../../Glossar.md#bundid) integriert dementsprechend das Zentrale Bürgerpostfach ([ZBP](../../Glossar.md#zentrales-bürgerpostfach)). Auch [Mein Unternehmenskonto](../../Glossar.md#mein-unternehmenskonto) (MUK) umfasst Postfachfunktionen durch die Integration der Lösungen Postfach 2.0 und OZG-Plus-Postfach. Die [Bekanntgabe](../../Glossar.md#bekanntgabe) von [Verwaltungsakten](../../Glossar.md#verwaltungsakt) über diese Postfächer erfolgt gemäß § 9 Abs. 1 OZG.[^2]

Das Fachportal „Mein ELSTER“ enthält ein eigenes Benutzerkonto und das ELSTER-Postfach, ist aber kein Nutzerkonto im Sinne des OZG.[^1] Steuerverwaltungsakte werden hier nach § 122a AO bekannt gegeben.[^3]

In den genannten Lösungen ist die Postfachadresse ein Attribut des Nutzerkontos und nicht eigenständig. Daraus folgt, dass jedes Konto eigene Postfächer integriert und [Onlinedienste](../../Glossar.md#onlinedienst) bzw. [Fachverfahren](../../Glossar.md#fachverfahren) teils mehrere Postfächer anbinden müssen. Zudem erhält der Onlinedienst keine Postfachadresse, wenn die Identifizierung ohne Nutzerkonto erfolgt. Dies wäre zum Beispiel bei einer direkten EUDI-Wallet-Anbindung der Fall (vgl. IT-Planungsrat, Beschluss 2026/19).[^4]

Wenn eine Postfachadresse nur ein Attribut der [Identität](../../Glossar.md#identität) ist, kann sie auch nur über die Schnittstellen des zugehörigen Nutzerkontos genutzt werden. Für jede [Teilnehmendengruppe](../../Glossar.md#teilnehmendengruppe) mit eigenem Konto müsste somit eine eigene Schnittstelle angebunden werden.

Diese ADR soll die Entscheidung für bzw. gegen eine Trennung von Postfach und Identität begründen. Sie legt fest, ob das Postfach Bestandteil des Nutzerkontos bleibt oder als separate Komponente umgesetzt wird, die Konten nur zur Identifizierung und Authentifizierung nutzt.

### Abgrenzung

- **Identifizierung:** ADR zu den gewählten [Identifizierungsmitteln](../../Glossar.md#authentisierungsmittel) [ADR-0014](./ADR-0014-Authentizität-Identität-von-Privatpersonen.md), [ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md), [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md).
- **Adressierung:** Nutzung von [Pseudonymen](../../Glossar.md#pseudonym), Adressierung für [Organisationen](../../Glossar.md#organisation) regeln Folge-Entscheidungen.

## Entscheidungstreiber

- **Ein Zugang:** [Privatpersonen](../../Glossar.md#privatperson) sowie [private Organisationen](../../Glossar.md#private-organisation) nutzen einen einzigen Zugang zur Kommunikation über sämtliche [Bereiche](../../Glossar.md#bereich) der [öffentlichen Stellen](../../Glossar.md#öffentliche-stelle) hinweg, inklusive der Justizverwaltung (RAH_ÜBE_144). Onlinedienste und Fachverfahren übermitteln [Nachrichten](../../Glossar.md#nachricht) in das Postfach einer Privatperson oder einer privaten Organisation (ANF_FUN_USE_060).

- **Identifizierungsmittel:** Die Identifikation von Privatpersonen, privaten Organisationen und öffentlichen Stellen ist über in § 3 OZG[^5] genannte Identifizierungsmittel möglich (ANF_FUN_IDE_023, ANF_FUN_IDE_024, ANF_FUN_IDE_050). Die Identifizierung über die [EUDI-Wallet](../../Glossar.md#europäische-brieftasche-für-die-digitale-identität) mit starker kryptografischer Bindung muss künftig möglich sein (ANF_FUN_IDE_025).

- **Pseudonymität:** Das System verwendet keine global eindeutigen Identifikatoren, die sich auf natürliche Personen beziehen (ANF_FUN_ADR_024) und ermöglicht eine pseudonyme Nutzung durch Privatpersonen (ANF_FUN_ADR_085).

- **Metadatenvermeidung:** Metadaten, die für die Übermittlung einer Nachricht nicht erforderlich sind, werden vermieden, als [vertrauliche Metadaten](../../Glossar.md#vertrauliche-metadaten) verschlüsselt oder verschleiert (SIC_VER_ÜBE_051).

- **Deutschland-Stack:** [ZaPuK](../../Glossar.md#zapuk) ist Basisdienst des Funktionsbausteins Postfach und Interaktion, separat vom Funktionsbaustein Authentifizierung (B-2026/03-IT).[^6]

## Betrachtete Optionen

### Option A: Postfach bleibt Bestandteil des Nutzerkontos

Jedes Nutzerkonto behält ein eigenes Postfach und bleibt Quelle der Postfachadressen. Die Postfächer setzen das Matrix-Protokoll um und föderieren im [Verbund](../../Glossar.md#verbund).

- Gut, denn eine Änderung von Gesetzen wie OZG und AO ist nicht notwendig.

- Gut, denn eine Änderung von Verantwortlichkeiten der Bestandslösungen ist nicht notwendig.

- Schlecht, denn Bildung und Auflösung der Postfachadressen hängen vom jeweiligen Nutzerkonto ab, sodass Onlinedienste und Fachverfahren je Teilnehmendengruppe unterschiedliche Verfahren umsetzen müssen.

- Schlecht, denn ohne ein mit der Identifikation verknüpftes Nutzerkonto, z. B. bei direkter Identifikation über die EUDI-Wallet, bestünde keine nutzbare Postfachadresse.

- Schlecht, denn der Betreiber des Nutzerkontos kann die Metadaten der Kommunikation mit der Identität auch bei der Nutzung von pseudonymen Adressen verknüpfen, da beides im selben System vorliegt.

- Schlecht, denn der Trennung der Funktionsbausteine Authentifizierung sowie Postfach und Interaktion der Deutschland-Architektur[^7] würde nicht entsprochen.

- Schlecht, denn das Postfach und die Adresse hätten den gleichen Lebenszyklus wie das Nutzerkonto, weshalb Anpassungen an Berechtigungen und [Funktionspostfächern](../../Glossar.md#funktionspostfach) direkt Attribute der Identität ändern.

- Schlecht, denn die Implementierung von Funktionalitäten der Postfächer wäre redundant in unterschiedlichen Systemen notwendig.

### Option B: Postfach wird vom Nutzerkonto getrennt

Das Postfach wird als eigenständige Komponente umgesetzt. Die Nutzerkonten werden vom Postfach zur Authentifizierung verwendet und enthalten keine Postfachfunktionen mehr. Die Postfachadressen werden im Onboarding vergeben und mit der Identität verknüpft oder aus Pseudonymen gebildet. Das Postfach lässt sich in Portale wie z. B. „Mein ELSTER“ integrieren.

- Gut, denn Postfachadressen werden unabhängig vom Nutzerkonto einheitlich gebildet und aufgelöst ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md), [ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)).

- Gut, denn ein Postfach benötigt kein Nutzerkonto, was für [technische Nutzer](../../Glossar.md#technischer-nutzer) ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)) sowie perspektivisch für die direkte Identifikation über die EUDI-Wallet oder die [eID](../../Glossar.md#eid-funktion) notwendig oder hilfreich ist ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md)).

- Gut, denn die Funktionsbausteine Authentifizierung sowie Postfach und Interaktion sind entsprechend der Deutschland-Architektur getrennt entworfen.

- Gut, denn das Postfach und die Adresse haben einen vom Nutzerkonto unabhängigen Lebenszyklus, womit Änderungen an Berechtigungen und Funktionspostfächern keine Attribute der Identität ändern.

- Gut, denn die Postfächer werden zentral nur einmal implementiert.

- Schlecht, denn es kann eine Änderung der Gesetze notwendig werden (OZG, AO).

- Schlecht, denn eine Änderung von Governance und Verantwortlichkeiten der Bestandslösungen ist notwendig.

## Architekturentscheidung

Gewählte Option: **Option B (Postfach wird vom Nutzerkonto getrennt)**, weil Postfach und Adresse damit einen eigenen Lebenszyklus besitzen, unabhängig vom Identifizierungsmittel verwendbar sind und die Postfachfunktionen nur einmal entwickelt werden müssen. Darüber hinaus wird der Betrieb einfacher möglich, da für alle Lösungen einheitliche Prozesse entworfen und implementiert werden können.

### Umsetzung der gewählten Option

Die Nutzenden authentifizieren sich über die Nutzerkonten bzw. die Identifizierungsmittel mit dem entsprechenden [Vertrauensniveau](../../Glossar.md#vertrauensniveau). Für Privatpersonen werden die Adressen den Pseudonymen der Identität zugeordnet ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md)). Für Organisationen werden die Adressen in einem [Adressregister](../../Glossar.md#adressregister) abfragbar registriert. Ein in der Kommunikation notwendiger [fachlicher Identitätsnachweis](../../Glossar.md#fachlicher-identitätsnachweis) wird an den Kommunikationspartner als Bestandteil der Nachricht übermittelt ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)). Mitgliedschaften und [Bearbeitungsvertretungen](../../Glossar.md#bearbeitungsvertretung) für die Funktionspostfächer werden in einem [Berechtigungsverzeichnis](../../Glossar.md#berechtigungsverzeichnis) verwaltet (siehe Folge-Entscheidung).

## Auswirkungen

- Die Anpassung der rechtlichen Grundlagen ist zu überprüfen (vgl. § 2, § 9 OZG, § 122a AO).

- Die Nutzerkonten BundID und MUK setzen keine Postfachfunktionalität mehr um. [ADR-0014](./ADR-0014-Authentizität-Identität-von-Privatpersonen.md) bis [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md) bleiben von dieser Entscheidung unberührt.

## Risiken

- **Rechtsgrundlage:** § 9 Abs. 1 OZG gilt nur für das Postfach eines OZG-Nutzerkontos. Damit die [Bekanntgabefiktion](../../Glossar.md#bekanntgabefiktion) an die Bereitstellung anknüpft, ist § 9 OZG anzupassen ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

## Folge-Entscheidungen

- Pseudonyme Adressen von Privatpersonen: Zuordnung der Postfachadresse zu den Pseudonymen in einem [Zuordnungsdienst](../../Glossar.md#zuordnungsdienst), siehe [ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md).

- [Nachweisstellen](../../Glossar.md#nachweisstelle) und [Org-IdP-Adapter](../../Glossar.md#org-idp-adapter): Identitätsnachweis je Nachricht, siehe [ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md).

- Adressregister für Organisationen, siehe [ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md).

- Berechtigungsverzeichnis, siehe [ADR-1112](./ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md).

## Referenzen

[^1]: [Onlinezugangsgesetz (OZG) § 2 Begriffsbestimmungen, Absatz 5 (Nutzerkonto) und Absatz 7 (Postfach)](https://www.gesetze-im-internet.de/ozg/__2.html), abgerufen am 15. September 2026.

[^2]: [Onlinezugangsgesetz (OZG) § 9 Bekanntgabe des Verwaltungsaktes](https://www.gesetze-im-internet.de/ozg/__9.html), abgerufen am 15. September 2026.

[^3]: [Abgabenordnung (AO) § 122a Bekanntgabe von Verwaltungsakten durch Bereitstellung zum Datenabruf](https://www.gesetze-im-internet.de/ao_1977/__122a.html), abgerufen am 15. September 2026.

[^4]: [IT-Planungsrat, Beschluss B-2026/19-IT Verwaltungsanbindung EUDI-Wallet vom 17. Juni 2026 mit Konzept zum Zielbild](https://www.it-planungsrat.de/beschluss/b-2026-19-it), abgerufen am 15. September 2026.

[^5]: [Onlinezugangsgesetz (OZG) § 3 Nutzerkonten, Identifizierung und Authentifizierung](https://www.gesetze-im-internet.de/ozg/__3.html), abgerufen am 15. September 2026.

[^6]: [IT-Planungsrat, Beschluss B-2026/03-IT Deutschland-Stack vom 18. März 2026](https://www.it-planungsrat.de/beschluss/b-2026-03-it), abgerufen am 15. September 2026.

[^7]: [IT-Planungsrat, Beschluss B-2026/04-IT Deutschland-Architektur vom 18. März 2026](https://www.it-planungsrat.de/beschluss/b-2026-04-it), abgerufen am 15. September 2026.
