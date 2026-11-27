> **Status:** ✅ Beschlossen | **Datum:** 14. April 2025

# ADR-0005: Anzahl und Zuschnitt der zentral bereitgestellten Postfach-Zugänge

## Kontext und Problembeschreibung

Dieser ADR betrachtet, ob und wenn ja, welche, Postfachzugänge als nativer Teil der Gesamtinfrastruktur bereitgestellt werden sollten, insbesondere um wesentlichen [Nutzer:innen]-Gruppen die dezidierte und gesicherte Nutzung der Kommunikationsinfrastruktur zu ermöglichen
Ein \[Postfachzugang\] ist hier ein lose gekoppelter logischer Baustein, der Funktionen bündelt, die [Nutzer:innen] beim Austausch von Nachrichten mit Postfach-Backends unterstützen (z.B. das Verfassen, Anzeigen, Senden, Empfangen, Verschlüsseln, etc. von Nachrichten).
Dieses wird [Nutzer:innen] in Form von clientseitigen Anwendungen (auch ‚Client‘, also z.B. als Webanwendung oder mobile App) bereitgestellt und ermöglicht Nutzer:innen damit im wörtlichen Sinne den Zugang zur Kommunikationsinfrastruktur.

Betrachtet werden im vorliegenden ADR nur fachlich agnostische und erweiterbare „All-zweck-„Postfachzugänge die als nativer Teil der Gesamtinfrastruktur bereitgestellt werden (vgl. Dazu [ADR-0001](ADR-0001-Querschnittlichkeit-und-Erweiterbarkeit-der-Infrastruktur.md)).
Nicht betrachtet werden fachlich-technisch spezialisierte und integrierte Postfachzugänge (z.B. als Einbindung in Fachverfahren, Onlinedienste, Unternehmensanwendungen, etc.) sowie etwaige fachlich agnostische „All-zweck-„Postfachzugänge die ggf. durch Dritte bereitgestellt werden.
Die grundlegende Offenheit der Infrastruktur im Sinne der Implementierung von Postfach-Zugängen durch Dritte ist in [ADR-0003](ADR-0003-Bereitstellung-von-Postfach-Zugängen.md) geregelt und wird durch die hier getroffene Entscheidung nicht berührt.

## Entscheidungstreiber

* Die parallele Entwicklung, Wartung und der parallele Betrieb von funktional sehr ähnlichen Software-Lösungen durch die öffentliche Hand führt zu einer ineffektiven und ineffizienten Ressourcenverwendung.
* Die parallele Entwicklung, Wartung und der parallele Betrieb von funktional sehr ähnlichen Software-Lösungen führt zu ungerechtfertigter Komplexität in der IT-Landschaft, was deren Beherrschbarkeit, Verständlichkeit und Zukunftsfähigkeit beeinträchtigt. Teil des vorliegenden Auftrages entsprechend [B-2024/28-IT](https://www.it-planungsrat.de/beschluss/beschluss-2024-28) ist die Reduzierung der Komplexität in der IT-Landschaft.
* Konkurrieren verschiedene [Nutzer:innen]-Gruppen um die Umsetzung gruppenspezifischer Anforderungen innerhalb einer Software-Lösung, kann sich diese im Vergleich mit spezialisierten Lösungen als weniger zweckdienlich erweisen.
* Entsprechend gestaltete Softwareprodukte können durch Konfiguration auf die Bedarfe verschiedener [Nutzer:innen]-Gruppen angepasst werden (z.B. durch aktivieren/deaktivieren einzelner Funktionen, Anzeige/Ausblenden einzelner Elemente in GUIs, …).
* Im Sinne eines Government-as-a-Plattform Ansatzes sind wenige und gut strukturierte Zugangswege zu digitalen Leistungen des Staates für [Privatpersonen] und [private Organisationen] leichter zu überblicken und tendenziell vertrauensstiftend.

## Betrachtete Optionen

- Option 1: Ein zentral bereitgestellter, abweichend konfigurierbarer Zugang für alle [Nutzer:innen]
- Option 2: Ein zentral bereitgestellter Zugang für [Privatpersonen] und ein separater zentral bereitgestellter Zugang für [private Organisationen] und [öffentliche Stellen]
- Option 3: Je verschiedene zentral bereitgestellte Zugänge für [Privatpersonen] sowie verschiedene Arten [privater Organisationen] (Vereine, Unternehmen, ...) und verschiedene Arten [öffentlicher Stellen] (Gerichte, Fachbehörden, ...)
- Option 4: Keine zentral bereitgestellten Zugänge für abgrenzbare [Nutzer:innen]-Gruppen

## Gegenüberstellung der jeweiligen Optionen

### Option 1: Ein zentral bereitgestellter, abweichend konfigurierbarer Zugang für alle [Nutzer:innen]

Die Infrastruktur stellt genau eine fachlich agnostische Lösung bereit, die alle Funktionen der Kommunikationsinfrastruktur für alle [Nutzer:innen]-Gruppen nutzbar macht.
Durch modulare Konfiguration der Lösung lässt sich deren Funktionsumfang bedarfsgerecht an [Nutzer:innen]-Gruppen anpassen.
Differenzielle Anforderungen von insb. Privatpersonen und Organisationen (z.B. Funktionspostfächer, Rollen- und Rechte-Management, Funktion für Vertretung, etc.) werden durch jeweils an die Zielgruppe angepasste Konfiguration bzw. Ausfertigungen des selben Softwareproduktes mit bedarfsorientiert abweichenden Funktionsumfang umgesetzt.

- Gut, aufgrund niedriger Komplexität in föderaler IT-Landschaft
- Gut, aufgrund niedriger Komplexität in Governance
- Gut, da einfach an [Nutzer:innen] zu kommunizieren
- Gut, da Redundanzen im Einsatz von Ressourcen (Geld, Personal, Aufmerksamkeit), Strukturen (Prozesse, Wissen, Methoden) durch Zentralisierung vermieten werden
- Gut, da technische Komplexität, Redundanzen und Inkongruenzen in Entwicklung und Betrieb sowie Interoperabilitätsanforderungen reduziert werden
- Gut, denn einen zentrale Entwicklung (in einer einzigen Code-Basis) führt eher zu einer einheitlichen User Experience und einem einheitlichen Design der Zugänge für verschiedene Nutzer:innengruppen (insb. relevant für Personen in Doppelrollen wie z.B. Kleinunternehmer:innen)
- Neutral, da zusätzliche funktionale Anforderungen hinsichtlich der Konfigurierbarkeit des Systems
- Neutral, da ggf. konkurrierende oder widersprüchliche Anforderungen unterschiedlicher [Nutzer:innen]-Gruppen in Einklang gebracht und gegeneinander priorisiert werden müssen, wobei die Mehrzahl derclientseitigen Anforderungen eines generischen Postfach-Zugangs zwischen den Nutzer:innen-Gruppen jedoch identisch sein dürften.
- Schlecht, da ggf. nicht allen Souveränitätsansprüchen genüge getan werden kann, insb. zwischen Staatsgewalten und von fachspezifischen Sonderrollen (z.B. Steuerberater, Notare, Ärzte, …)

### Option 2: Ein zentral bereitgestellter Zugang für [Privatpersonen] und ein separater, zentral bereitgestellter Zugang für [private Organisationen] und [öffentliche Stellen]
Die Infrastruktur stellt genau zwei fachlich agnostische Lösungen bereit, die verschiedenen [Nutzer:innen]-Gruppen Funktionen der Kommunikationsinfrastruktur entsprechend ihrer Bedürfnisse nutzbar macht.
Konkret wird je eine dedizierte Lösung für [Privatpersonen] und eine dedizierte Lösung für Organisationen ([private Organisationen] und [öffentliche Stellen]) entwickelt und bereitgestellt.
Sowohl identische als auch differenzielle Anforderungen von Privatpersonen und Organisationen werden separat (ggf. unter Nachnutzung von Entwicklungen aus anderen Lösungen) in jeder der zwei Lösungen umgesetzt.

- Gut, da abweichenden Anforderungen von individuellen [Privatpersonen] und Organisationen dezidiert Rechnung getragen werden kann
- Gut, da ggf. konkurrierende oder widersprüchliche Anforderungen unterschiedlicher [Nutzer:innen]-Gruppen nicht in Einklang gebracht und gegeneinander priorisiert werden müssen
- Neutral, aufgrund mittlerer Komplexität in IT-Landschaft
- Neutral, da redundante und abweichende Governance-Strukturen entstehen können
- Schlecht, da Funktionale und Nicht-funktionale Charakteristiken der zwischen den zwei Zugängen ggf. redundant entwickelt werden müssen

### Option 3: Je verschiedene zentrale Zugänge für [Privatpersonen] sowie verschiedene Arten [privater Organisationen] (Vereine, Unternehmen, ...) und verschiedene Arten [öffentlicher Stellen] (Gerichte, Fachbehörden, ...)

Die Infrastruktur stellt verschiedene, z.T. fachlich angepasste Lösungen bereit, die verschiedenen [Nutzer:innen]-Gruppen Funktionen der Kommunikationsinfrastruktur in einer dezidierten Lösung entsprechend etwaiger spezifischer fachlicher Bedürfnisse nutzbar macht.
Konkret werden bedarfsorientiert dedizierte technische Lösungen für verschiedene abgrenzbare [Nutzer:innen]-Gruppen und -Typen bereitgestellt.
Sowohl identische als auch differenzielle Anforderungen von verschiedenen [Nutzer:innen]-Gruppen werden (ggf. unter Nachnutzung von Entwicklungen aus anderen Lösungen) separat in den jeweiligen Lösungen umgesetzt.

- Gut, da ggf. abweichenden Anforderungen einzelner [Nutzer:innen]-Gruppen granular Rechnung getragen werden kann
- Schlecht, da eine größere Zahl redundanter und abweichender Governance-Strukturen entstehen können
- Schlecht, da Funktionen zwischen Lösungen ggf. redundant entwickelt werden
- Schlecht, aufgrund hoher, nicht abschätzbarer Komplexität in der IT-Landschaft
- Schlecht, aufgrund ineffizienter und ineffektive Nutzung von Ressourcen (Geld, Personal, Aufmerksamkeit) und Strukturen (Prozesse, Wissen, Methoden)

### Option 4: Keine zentral bereitgestellten Zugänge für abgrenzbare [Nutzer:innen]-Gruppen

Die Infrastruktur stellt nativ keine dedizierten Zugänge bereit.
Der Zugang zu Postfächern erfolgt ausschließlich über von Dritten bedarfsorientiert bereitgestellte Lösungen.
Sowohl identische als auch differenzielle Anforderungen werden nach dem Ermessen Dritter von in den jeweiligen Lösungen umgesetzt, ggf. entsprechend durch für die Infrastruktur gesetzter Anschlussbedingungen und anderer spezifizierter Standards.
Die Bereitstellung von Postfachzugängen durch Dritte wird in [ADR-0003](ADR-0003-Bereitstellung-von-Postfach-Zugängen.md) gesondert ausgeführt.

- Gut, da keine Bedarfe an Ressourcen und Strukturen an zentraler Stelle
- Neutral, da keine zusätzliche Komplexität in der unmittelbar durch den IT-PLR gesteuerten föderaler IT-Landschaft. Mittelbar scheint ein Komplexitätszuwachs durch unkonsolidierte Angebote Dritter nicht unwahrscheinlich.
- Neutral, da ggf. vor dem Hintergrund der Offenheit für die Entwicklung von Postfachzugängen durch Dritte (vgl. [ADR-0003](ADR-0003-Bereitstellung-von-Postfach-Zugängen.md)) Plattformeffekte entstehen können, wobei nachfrageorientiert Lösungen durch ein Ökosystem an Anbietern bereitgestellt werden. Dies ist jedoch von notwendigen Vorbedingen (insb. Existenz eines Marktes) abhängig und schafft Komplexität für Endnutzer.
- Schlecht, da Skalierbarkeit der Infrastruktur bedeutend durch die Qualität und Quantität der durch Dritte bereitgestellten Lösungen beschränkt wird.
- Schlecht, da ggf. kein kostenfreier oder -armer Zugang für [Nutzer:innen] von zentraler Stelle garantiert werden kann
- Schlecht, da kein offizieller, vertrauenswürdiger und kommunizierbarer Anlaufpunk für [Nutzer:innen] existiert
- Schlecht, da nicht Kompatibel mit Government-as-a-platform (GaaP) und One-Stop Shop (OSS) Ansätzen

## Architekturentscheidung

Gewählte Option: **[Option 1: Ein zentraler, abweichend konfigurierbarer Postfach-Zugang für alle [Nutzer:innen]](#option-1-ein-zentral-bereitgestellter-abweichend-konfigurierbarer-zugang-für-alle-nutzerinnen)**, da gegenüber Option 2 und 3 überwältigende Synergieeffekte durch eine einzige zentrale Produktentwicklung, -betrieb und -governance (insb. aufgrund der hohen Anzahl an sich in generischen Use-Cases überlappenden Anforderungen) ohne technische und organisatorische Doppelstrukturen geschaffen werden, da die Komplexität der föderalen IT-Landschaft weitestmöglich gesenkt und da Ressourcen effizient eingesetzt werden können, sowie da gegenüber Option 4, da ein niedrigschwelliger, vertrauenswürdiger, kommunizierbarer und ggf. kostenfreier Postfachzugang für für alle [Nutzer:innen] garantiert werden kann.

### Auswirkungen

Die Zusammenführung bestehender Bemühungen zur Schaffung von Postfach-Zugängen für verschiedene Zielgruppen erfordert eine Neuordnung von Zuständigkeiten und Anpassung von Governance-Strukturen im operativen Produktmanagement der betroffenen Lösungen.