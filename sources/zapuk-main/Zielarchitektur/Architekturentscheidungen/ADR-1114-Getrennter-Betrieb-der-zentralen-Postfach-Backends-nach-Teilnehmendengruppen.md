# ADR-1114: Getrennter Betrieb der zentralen Postfach-Backends nach Teilnehmendengruppen

> **Status:** ✔️ Operativ entschieden | **Datum:** 9. Oktober 2026

## Kontext und Problembeschreibung

In diesem ADR wird die unter Vorbehalt getroffene Entscheidung des [ADR-0006](./ADR-0006-Anzahl-und-Zuschnitt-der-zentral-bereitgestellten-Postfach-Backends.md) aus Phase 1 zum zentral bereitgestellten [Postfach-Backend](../../Glossar.md#postfach-backend) behandelt. Es werden Gründe erörtert, warum die Entscheidung in Phase 2 fortgeschrieben wird.

Die Zielarchitektur definiert mehrere [Teilnehmendengruppen](../../Glossar.md#teilnehmendengruppe). Diese sind [Privatpersonen](../../Glossar.md#privatperson), [private Organisationen](../../Glossar.md#private-organisation), [öffentliche Stellen](../../Glossar.md#öffentliche-stelle) sowie [Organisationen der Justiz](../../Glossar.md#organisation-der-justiz). Die Verbindungsdaten von Privatpersonen, also Metadaten, die Aufschluss über die Kommunikationsbeziehungen mit öffentlichen Stellen geben, müssen im besonderen Maße geschützt werden. Diese lassen Rückschlüsse auf Lebenslagen einzelner Menschen zu. Sie sollen sowohl vor einem Zugriff des Staates als auch vor Cyberangriffen durch fremde Dienste oder Kriminelle geschützt werden.

Die unterschiedlichen Gruppen von [Organisationen](../../Glossar.md#organisation) müssen dagegen unterschiedliche rechtliche Vorgaben erfüllen und haben spezifische Schutzbedarfe.

Dieser ADR entscheidet, ob die zentralen Postfach-Backends nach Teilnehmendengruppen getrennt werden und ob das Backend für Privatpersonen einen anderen Betreiber benötigt als die Backends für Organisationen. Er passt damit die Entscheidung des [ADR-0006](./ADR-0006-Anzahl-und-Zuschnitt-der-zentral-bereitgestellten-Postfach-Backends.md) an.

### Abgrenzung

- **Betreiber:** Dieser ADR wählt keine konkreten Stellen als Betreiber aus.
- **Betriebsmodelle:** Dieser ADR behandelt nicht die von Organisationen souverän betriebenen Backends aus [ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md).

## Entscheidungstreiber

- **Metadatenvermeidung:** Alle Metadaten, die nicht zwingend für die korrekte Übermittlung einer [Nachricht](../../Glossar.md#nachricht) erforderlich sind, werden vermieden, als [vertrauliche Metadaten](../../Glossar.md#vertrauliche-metadaten) verschlüsselt oder durch Privatsphäre-schützende Technologien verschleiert (SIC_VER_ÜBE_051).

- **Ein Zugang:** Das System ermöglicht Privatpersonen und privaten Organisationen über einen einzigen Zugang eine Kommunikation über alle [Bereiche](../../Glossar.md#bereich) der öffentlichen Stellen hinweg (RAH_ÜBE_144).

- **Wirtschaftlicher Betrieb:** Die parallele Entwicklung, Wartung und der parallele Betrieb von funktional sehr ähnlichen Software-Lösungen durch die öffentliche Hand führt zu einer ineffizienten Ressourcenverwendung ([ADR-0006](./ADR-0006-Anzahl-und-Zuschnitt-der-zentral-bereitgestellten-Postfach-Backends.md), B-2024/28-IT).[^1]

## Betrachtete Optionen

### Option A: Ein Betreiber bzw. ein zentrales Backend für alle Teilnehmendengruppen

Wie in [ADR-0006](./ADR-0006-Anzahl-und-Zuschnitt-der-zentral-bereitgestellten-Postfach-Backends.md) Option 3 entschieden, wird für alle Teilnehmendengruppen ein zentrales Backend bereitgestellt oder es werden mehrere Postfach-Backends durch dieselbe Stelle betrieben. Letztlich spielt die Anzahl der betriebenen Systeme in dieser Betrachtung eine untergeordnete Rolle. Entscheidend ist, dass sie alle in einer Hand liegen.

- Gut, denn es werden Redundanzen bei der Entwicklung vermieden. Die Effizienz ist hoch und Ressourcen wie Geld und Personal werden eingespart.

- Gut, denn die Komplexität im Betrieb und der Governance sinkt, zum Beispiel ist weniger Abstimmungsbedarf beim Rollout neuer Versionen notwendig.

- Gut, denn durch die hohe Kompetenzbündelung wird eine hohe Qualität des angebotenen Dienstes einfacher gewährleistet. 

- Schlecht, denn es ist ein zentraler Zugriff auf die Verbindungsdaten aller [Nutzenden](../../Glossar.md#nutzerinnen) bei einem Betreiber möglich, was die Vertraulichkeit der Kommunikationsbeziehungen insgesamt schwächt.

- Schlecht, denn durch die Möglichkeit der zeitlichen Korrelation der Verbindungsdaten in Verbindung mit dem Wissen über verwendete Konten ist eine effektive Umsetzung von Pseudonymisierung bzw. anonymer Nutzung unmöglich.

### Option B: Getrennte Betreiber für Privatpersonen und Organisationen

Statt eines zentralen Postfach-Backends gibt es mehrere Backends für die verschiedenen Teilnehmendengruppen. Das Postfach-Backend für Privatpersonen wird separat betrieben, während die Backends für Organisationen (private Organisationen, öffentliche Stellen, Organisationen der Justiz) einen anderen Betreiber besitzen.

Der Schutz der Verbindungsdaten ist deutlich besser zu gewährleisten, wenn die Infrastruktur nicht in einer Hand betrieben wird. Wird der Betrieb der Kommunikationsknoten bzw. Postfach-Backends auf verschiedene Stellen verteilt, hat jeder Betreiber nur Zugriff auf einen Teil der Metadaten. Dadurch lassen sich [Pseudonyme](../../Glossar.md#pseudonym) oder verschlüsselte Adressen zielführend einsetzen. Die Zuordnung eines Pseudonyms zum konkreten Nutzendenkonto ist nur auf dem Postfach-Backend des Nutzenden notwendig. Das Postfach-Backend der Gegenstelle muss diese Zuordnung nicht kennen. 

IP-Adressen einer Kommunikation lösen zwar zum Nutzenden des Postfach-Backends auf, können aber in Verbindung mit verschlüsselten Adressen oder Pseudonymen keinen Aufschluss über den anderen Kommunikationspartner geben.

- Gut, denn die Verbindungsdaten der Kommunikation von Privatpersonen mit öffentlichen Stellen liegen nicht bei einem einzelnen Betreiber vor, was einen besseren Schutz ermöglicht. Eine Kompromittierung eines Betreibers betrifft nur seine Teilnehmendengruppen.

- Gut, denn somit ergibt sich eine organisatorische Trennung mit klaren Zuständigkeiten, zielgruppenspezifischen Systemen und Anbindungsprozessen pro Zielgruppe.

- Gut, denn getrennte [Homeserver](../../Glossar.md#homeserver) ermöglichen eine technische Trennung der Accounts für private Organisationen, öffentliche Stellen und Organisationen der Justiz, wodurch eine Umsetzung gruppenspezifischer Besonderheiten erleichtert wird.

- Neutral, denn der einheitliche Zugang bleibt durch die Föderation der Backends und den zentralen [Postfach-Zugang](../../Glossar.md#postfach-zugang) erhalten ([ADR-0005](./ADR-0005-Anzahl-und-Zuschnitt-der-zentral-bereitgestellten-Postfach-Zugänge.md)).

- Schlecht, denn die Komplexität und die Aufwände in Entwicklung und Betrieb steigen.

- Schlecht, denn die Kompetenz für das System muss bei mehreren Betreibern aufgebaut werden.

## Architekturentscheidung

Gewählte Option: **Option B (Getrennte Betreiber für Privatpersonen und Organisationen)**, weil die Trennung der Betreiber nach Teilnehmendengruppen wie Privatpersonen und Organisationen den Zugriff der Betreiber auf Metadaten bzw. Verbindungsdaten begrenzt. Der getrennte Betrieb des Privatpersonen-Backends ermöglicht auch eine organisatorische Separierung der anfallenden Metadaten auf mehrere Betriebsumgebungen. Bei der Kommunikation von Organisationen untereinander sind im Falle einer Kompromittierung der Metadaten keine vergleichbaren Risiken für die Privatsphäre zu erwarten. Auf eine strikte Aufteilung der Betriebsverantwortung für die Backends der Organisationen auf unabhängige Betreiber wird daher aufgrund der Vorteile eines gebündelten Betriebs verzichtet. Der Schutz der Kommunikationsbeziehungen der Privatpersonen wiegt schwerer als die Mehraufwände bei Entwicklung und Betrieb, da eine Akzeptanz des Systems nur mit dem Schutz der Privatpersonen gewährleistet ist. Die Entscheidung in [ADR-0006](./ADR-0006-Anzahl-und-Zuschnitt-der-zentral-bereitgestellten-Postfach-Backends.md) wird somit aufgrund technischer Gründe bzgl. der Umsetzung von Pseudonymen und der Möglichkeiten zum Schutz der Verbindungsdaten mittels organisatorischer Maßnahmen fortgeschrieben, wie dort bei neuen Sachgründen vorgesehen.

### Umsetzung der gewählten Option

Zentrale Postfach-Backends werden je Teilnehmendengruppe eigenständig bereitgestellt. Dies sind die Backends für Privatpersonen, private Organisationen, öffentliche Stellen und Organisationen der Justiz. Das Backend für Privatpersonen wird von einer anderen Stelle betrieben als die Backends für Organisationen, deren Betrieb bei einer Stelle gebündelt wird. Die Backends sind technisch weitgehend identisch und werden aus einer Hand entwickelt, wodurch die Mehraufwände begrenzt bleiben. Jedes dieser Backends wird von einer dafür benannten Stelle betrieben, ist im [Verbund](../../Glossar.md#verbund) eindeutig adressierbar und nutzt das für die Teilnehmendengruppe maßgebliche [Authentisierungssystem](../../Glossar.md#authentisierungssystem) (siehe [ADR-0014](./ADR-0014-Authentizität-Identität-von-Privatpersonen.md) bis [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md)). Jedes zentrale Backend erhält eine eigene Domain ([ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md)) und eine eigene Instanz der zentralen [Nachweisstelle](../../Glossar.md#nachweisstelle) ([ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)). Die [Anschlussbedingungen](../../Glossar.md#anschlussbedingungen) gelten einheitlich für die zentralen Backends und für dezentrale Backends Dritter. Jedes Backend föderiert ausschließlich mit den in der [Föderationsliste](../../Glossar.md#föderationsliste) gelisteten Backends (vgl. Folge-Entscheidung).

Privatpersonen nutzen ausschließlich das zentral bereitgestellte Postfach-Backend für Privatpersonen und betreiben keine eigenen Homeserver. Organisationen haben die Wahl zwischen unterschiedlichen [Betriebsmodellen](../../Glossar.md#betriebsmodell) ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)). [Fachverfahren](../../Glossar.md#fachverfahren) und Postfach-Zugänge werden nur an ein einziges Postfach-Backend des Verbunds angebunden. Die Kommunikation der Teilnehmendengruppen untereinander ist durch die Föderation der Backends möglich. Für den Zugriff auf [Postfächer](../../Glossar.md#postfach) wird ein zentraler Postfach-Zugang für alle Teilnehmendengruppen bereitgestellt ([ADR-0005](./ADR-0005-Anzahl-und-Zuschnitt-der-zentral-bereitgestellten-Postfach-Zugänge.md)).

## Auswirkungen

- Die operative Koordinierung von Weiterentwicklung, Standards und Betrieb über die beteiligten Produkte hinweg muss durch die Governance des Verbunds geleistet werden.

- Die Entscheidung des [ADR-0006](./ADR-0006-Anzahl-und-Zuschnitt-der-zentral-bereitgestellten-Postfach-Backends.md) zum Betrieb nur eines Postfach-Backends wird hiermit angepasst. Die Entscheidung zu einem zentralen Postfach-Zugang bleibt davon unberührt ([ADR-0005](./ADR-0005-Anzahl-und-Zuschnitt-der-zentral-bereitgestellten-Postfach-Zugänge.md)). Für Privatpersonen ist der Betrieb eigener Postfach-Backends nicht zugelassen ([ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md)). Die Entscheidungen zur Infrastrukturtopologie ([ADR-0002](./ADR-0002-Infrastrukturtopologie.md)) und zur Bereitstellung von Postfach-Backends durch Dritte ([ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md)) werden damit für Privatpersonen eingeschränkt.

- Die Verfügbarkeit des zentralen Backends für Privatpersonen kann über einen redundanten Betrieb an mehreren Standorten gewährleistet werden.

## Risiken

- Haben das Backend für Privatpersonen, die zentrale Nachweisstelle und die [BundID](../../Glossar.md#bundid) denselben Betreiber, kann dieser Konten und Pseudonyme zusammenführen ([ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)). Der Schutz muss dann durch organisatorische Maßnahmen gewährleistet werden.

## Folge-Entscheidungen

- Anschlussbedingungen und [Föderationskontrolle](../../Glossar.md#föderationskontrolle): einheitliche Anschlussbedingungen für zentrale und dezentrale Backends, Steuerung der Föderation über die Föderationsliste, siehe [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md).

## Referenzen

[^1]: [IT-Planungsrat, Beschluss 2024/28 Postfach- und Kommunikationslösungen vom 19. Juni 2024](https://www.it-planungsrat.de/beschluss/beschluss-2024-28), abgerufen am 15. September 2026.
