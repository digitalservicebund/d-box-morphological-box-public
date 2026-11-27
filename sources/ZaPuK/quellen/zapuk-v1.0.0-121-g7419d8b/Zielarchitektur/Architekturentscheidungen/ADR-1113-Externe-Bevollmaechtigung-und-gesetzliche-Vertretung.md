# ADR-1113: Externe Bevollmächtigung und gesetzliche Vertretung

> **Status:** ✔️ Operativ entschieden | **Datum:** 24. September 2026

## Kontext und Problembeschreibung

Die Zielarchitektur muss Vollmachten und gesetzliche Vertretungen unterstützen. Diese Architekturentscheidung betrachtet beide Fälle der [externen Vertretung](../../Glossar.md#externe-vertretung). Es ist zu klären, ob die handelnde Person oder Stelle mit eigener [Identität](../../Glossar.md#identität) und eigenem [Postfach](../../Glossar.md#postfach) für die vertretene Partei auftritt oder auf das Postfach der vertretenen Partei zugreifen kann.

Ein zu unterstützender Anwendungsfall für eine Bevollmächtigung ist zum Beispiel die steuerliche Beratung. Steuerberatende handeln im Rahmen einer gegebenen Vollmacht für einen Mandanten mit einer eigenen beruflichen Identität. Dieser Mandant kann sowohl eine natürliche Person als auch eine [private Organisation](../../Glossar.md#private-organisation) sein. In der Vollmachtsdatenbank der Bundessteuerberaterkammer ist die Vollmacht verzeichnet und wird hierüber an die Finanzverwaltung übermittelt und maschinell verarbeitet. Ein ähnlicher Anwendungsfall ist die Prokura, die Personen die Vertretung privater Organisationen ermöglicht, sie ist in Handelsregistern hinterlegt. Die [EUDI-Wallet](../../Glossar.md#europäische-brieftasche-für-die-digitale-identität) soll künftig digitale Vollmachten unterstützen (Digitalisierungsrichtlinie (EU) 2025/25).

Die gesetzliche Vertretung kann durch ein Gericht oder eine behördliche Anordnung beschlossen werden, gesetzlich verankert sein oder durch die Organstellung einer Person vorliegen. Sie kann private Organisationen oder [Privatpersonen](../../Glossar.md#privatperson) betreffen. Geschäftsführer und Vorstände haben eine organschaftliche Vertretungsmacht für ihre [Organisation](../../Glossar.md#organisation), die im Handelsregister verzeichnet ist. Betreuende Personen und ehrenamtliche Vormünde übernehmen die Vertretung von Privatpersonen zum Beispiel gegenüber [Behörden](../../Glossar.md#behörde), wobei für Betreuungen und Vormundschaften noch kein zentrales, digitales Register existiert und der Nachweis der Vertretungsbefugnis meist dokumentenbasiert erfolgt.

Sowohl die Vollmacht als auch die gesetzliche Vertretung erlauben einer Person das Handeln im Namen einer anderen Person, was im fachlichen Kontext eines [Vorgangs](../../Glossar.md#vorgang) erkennbar sein muss. Der Nachweis wird dabei durch den Berufsstand, Register und über Dokumente erbracht. Die Identität der vertretenen Person ist mit dem Nachweis der Vollmacht bzw. Vertretungsmacht verknüpft.

### Abgrenzung

- [ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md) betrachtet [Bearbeitungsvertretungen](../../Glossar.md#bearbeitungsvertretung) in [Funktionspostfächern](../../Glossar.md#funktionspostfach)
- [ADR-1112](./ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md) betrachtet die Rechtevergabe in Funktionspostfächern
- [ADR-0001](./ADR-0001-Querschnittlichkeit-und-Erweiterbarkeit-der-Infrastruktur.md) gibt vor, dass die Zielarchitektur keine domänenspezifische Funktionalität umsetzen soll. Ein Vollmachtsregister ist somit kein Bestandteil von [ZaPuK](../../Glossar.md#zapuk). Die Prüfung von Vollmachten müssen die zuständigen Stellen bzw. [Fachverfahren](../../Glossar.md#fachverfahren) umsetzen.

## Entscheidungstreiber

- **Vollmachten und gesetzliche Vertretung:** Die Zielarchitektur muss das Handeln im Namen einer anderen Person oder Organisation unterstützen (ANF_FUN_ADR_025).

## Betrachtete Optionen

### Option A: Eigene Identität der handelnden Person mit Vertretungskontext

Die handelnde Person nutzt im Rahmen ihrer Tätigkeit ihre eigene Identität und ein eigenes Postfach. Den Nachweis über die Vollmacht oder die Befugnis zur gesetzlichen Vertretung kann sie mit der [Nachricht](../../Glossar.md#nachricht) übermitteln. Die Zielarchitektur erlaubt sowohl eine zusätzliche Senderaussage bzw. Erweiterung des [Identitätsnachweises](../../Glossar.md#fachlicher-identitätsnachweis) ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)) als auch die Übermittlung separater, ggf. signierter Dokumente ([ADR-0017](./ADR-0017-Authentizität-und-Integrität-von-Inhalten.md)). Die empfangende Stelle muss die Prüfung des Nachweises selbst oder mit elektronischer Unterstützung über Fachverfahren und Register durchführen.

- Gut, denn eine strikte [E2EE](../../Glossar.md#ende-zu-ende-verschlüsselung) ist weiterhin möglich ([ADR-1100](./ADR-1100-Bilaterale-Kommunikation-Festlegung-E2EE-Endpunkte.md)).

- Gut, denn kein Zugriff auf das Postfach der vertretenen Person ist nötig, womit die Privatsphäre besser geschützt bleibt bzw. das Datenschutzrecht einfacher einzuhalten ist.

- Gut, denn die Zielarchitektur bleibt frei von domänenspezifischer Logik.

- Schlecht, denn eine Übergabe der Vollmacht oder Vertretung an eine andere handelnde Person oder zurück an die vertretene Person erfordert die Export-Funktionalität (siehe Folge-Entscheidung).

### Option B: Postfach-Zugang der vertretenen Person

Die handelnde Person nutzt im Rahmen ihrer Tätigkeit das Postfach der vertretenen Person. Sie kann durch eine Einladung in die [Vorgangsräume](../../Glossar.md#vorgangsraum) Zugriff auf Nachrichten erlangen. Dies wäre möglich, wenn auch Postfächer von Privatpersonen als Funktionspostfächer umgesetzt würden ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)).

- Gut, denn empfangende Stellen müssen Vertretungsnachweise seltener bzw. weniger aufwendig prüfen, da ein Funktionspostfach genutzt wird ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md), [ADR-1112](./ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md)).

- Gut, denn eine Übergabe der Vollmacht oder Vertretung an eine andere handelnde Person oder zurück an die vertretene Person ist einfach. Die Vorgangsräume bleiben weiter nutzbar.

- Schlecht, denn eine strikte E2EE ist nicht mehr einzuhalten ([ADR-1100](./ADR-1100-Bilaterale-Kommunikation-Festlegung-E2EE-Endpunkte.md)).

- Schlecht, denn der Zugriff auf das Postfach der vertretenen Person ist notwendig, womit Probleme bzgl. Privatsphäre und Datenschutzrecht entstehen.

- Schlecht, denn für die Vertretung von Privatpersonen muss die Funktionspostfach-Funktionalität auch für das zentrale Backend der Privatpersonen entwickelt werden.

## Architekturentscheidung

Gewählte Option: **Option A, eigene Identität der handelnden Person mit Vertretungskontext**, weil so keine domänenspezifische Logik eingeführt wird, die strikte Ende-zu-Ende-Verschlüsselung für Privatpersonen möglich bleibt und die Umsetzung praktikabel bleibt.

Gespräche mit der Steuerberaterkammer und einem Jugendamt haben gezeigt, dass eine Lösung mit einem gemeinsamen Postfach bzw. Funktionspostfach für diese Anwendungsfälle nicht notwendig ist. Die Übergabe von einer handelnden Person an eine andere ist durch strikte Prozesse geregelt. Die für Vollmachten und gesetzliche Vertretung sinnvollen Register werden nicht durch die Zielarchitektur entworfen. Auch wenn diese Register von zentraler Bedeutung sind und gerade das derzeit fehlende Vormundschaftsregister ein Hindernis der Digitalisierung darstellt, hat ZaPuK den Auftrag, eine domänenagnostische Kommunikationsarchitektur zu entwickeln.

Die handelnden Personen nutzen in der Zielarchitektur eigene Postfächer und ihre eigene Identität bzw. Berufsidentität, die durch Nachweise mit Vollmachten und gesetzlichen Vertretungen verknüpft ist. Die Zielarchitektur ermöglicht die Übermittlung dieser Nachweise entweder als Ergänzung des Identitätsnachweises ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)) oder durch signierte Dokumente ([ADR-0017](./ADR-0017-Authentizität-und-Integrität-von-Inhalten.md)). Die Prüfung dieser Nachweise obliegt den empfangenden Stellen bzw. Fachverfahren und wird nicht innerhalb der Zielarchitektur entworfen oder umgesetzt.

## Auswirkungen

- Empfänger-Clients können bei erhaltenen Nachweisen der Vollmacht oder Vertretungsbefugnis nach [ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md) die Gültigkeit darstellen.

- Vollmachtsdatenbanken, Vormundschaftsregister und ähnliche Register sind nicht Bestandteil der Zielarchitektur.

## Risiken

- Die Akzeptanz der Zielarchitektur ist teilweise abhängig von Vorhaben und Entwicklungen der Register zu gesetzlichen Vertretungen und Vollmachten. Bei der Kommunikation ist derzeit die Prüfung der Vertretungen zeitaufwändig und fehleranfällig, wozu in diesem Projekt keine vollständige Lösung entsteht.

## Folge-Entscheidungen

- Veraktung und Aufbewahrung: Export-Funktionalität bei der Übergabe an eine andere handelnde Person, siehe [ADR-1135](./ADR-1135-Veraktung-und-Aufbewahrung.md).

## Offene Punkte

- Das Nachweisformat für Vollmachten und gesetzliche Vertretung innerhalb der Identitätsnachweise ist abzustimmen bzw. seine Machbarkeit zu klären.

- Maschinenlesbare Verifikation von Vormundschaften setzt ein bundesweit verfügbares Vormundschaftsregister voraus, das derzeit nicht existiert. Ohne ein solches Register bleibt die Prüfung dokumentenbasiert.

- Anbindung von Vollmachtsregistern, etwa der Vollmachtsdatenbank der Bundessteuerberaterkammer für steuerliche Vollmachten und der in der EU-Business-Wallet vorgesehenen Vollmachten und Vertretungsbefugnisse.
