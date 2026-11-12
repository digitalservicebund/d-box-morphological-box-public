> **Status:** ✅ Beschlossen | **Datum:** 15. April 2025

# ADR-0003: Bereitstellung von Postfach-Zugängen durch Dritte

## Kontext und Problembeschreibung

Im Kontext einer offenen oder geschlossen Systemarchitektur ergibt sich die Frage, inwiefern eigene Postfachzugänge von Dritten - also nicht als zentral bereitgestellter, nativer Teil der Kommunikationsinfrastruktur - implementiert und betrieben werden können.
Ein solcher Postfachzugang kann als Zugang eines Fachverfahrens zur automatisierten Weiterverarbeitung als auch als Client, der von Menschen bedient wird, ausgeprägt sein; zudem gibt es Zwischenformen wie Prozessmanagementsysteme, Kanzleisoftware oder andere Drittanwendungen, die Menschen durch Automatisierung bei ihrer Arbeit helfen. 
Dieser ADR dokumentiert keine Entscheidung zu möglichen zentral bereitgestellten Postfachzugängen; diese Entscheidung wird im [ADR-0005](ADR-0005-Anzahl-und-Zuschnitt-der-zentral-bereitgestellten-Postfach-Zugänge.md) dargestellt. Stattdessen fokussiert er sich nur auf die Dimension offen-geschlossen, also der allgemeinen Möglichkeit, ohne Hürde weitere Postfachzugänge bereitzustellen.

## Entscheidungstreiber

- _ANF_ARC_ANB_006: Die Funktionalitäten [des Systems] stehen den [Nutzer:innen] ohne Funktionseinschränkungen über Schnittstellen zur Verfügung (Umsetzung des API-First Ansatz)._ 🠒 Entsprechend den Anforderungen sowie als Architekturprinzip wird der API-First-Ansatz verfolgt, bei dem die Schnittstelle zur Postfachverwaltung grundsätzlich neutral spezifiziert wird und sicher für beliebige Konsumenten betrieben werden kann.
- _SIC_ÜBE_ÜBE_015: [Das System] setzt den Security by Design Ansatz um._ 🠒 Ebenfalls als Architekturprinzip festgelegt bewirkt der Security-by-Design-Ansatz, dass Schnittstellen unabhängig von Eigenschaften ihrer Konsumenten sicher sein müssen. Wird dies erfolgreich umgesetzt, sind keine Annahmen über Eigenschaften von Konsumenten notwendig.
- Notwendigkeit von Integration: Die Anforderungen ergeben, dass eine Integration von Fachverfahren zur automatisierten Weiterverarbeitung notwendig ist und realistischerweise nur von den entsprechenden Stellen implementiert werden kann.
- User Experience: Die Möglichkeit der Erstellung von Nutzerinterfaces, die auf bestimmte Zielgruppen und Use Cases zugeschnitten sind, kann positive Effekte auf die Nutzererfahrung- und Akzeptanz haben. Insbesondere können hier auch beliebige Marktteilnehmer innovative Lösungen bereitstellen und sich ggf. durchsetzen; dies entspricht auch der Architekturrichtlinie SR7 – Sicherstellung Herstellerunabhängigkeit.
- Skalierbarkeit: Das System kann potentiell sehr groß werden und eine Vielzahl von Anwendungsfällen abdecken; entsprechend muss es fachlich, organisatorisch und technisch skalierbar sein. Ein Prozess zur Kontrolle etwa schränkt die organisatorische Skalierbarkeit ein, da die gleichzeitige Kontrolle sehr vieler Implementierungen sehr aufwendig werden kann. 

## Betrachtete Optionen

- [Option 1: Ausschließlich zentral bereitgestellte Zugänge](#option-1-ausschließlich-zentral-bereitgestellte-postfachzugänge)
- [Option 2: Zentral bereitgestellte User-Interfaces mit Öffnung für Fachverfahren und ADR](#option-2-zentral-bereitgestellte-user-interfaces-mit-öffnung-für-fachverfahren-und-zwischenformen)
- [Option 3: Freigabe und Zertifizierung](#option-3-freigabe-und-zertifizierung)
- [Option 4: Offenheit für Drittanwendungen](#option-4-offenheit-für-drittanwendungen)

## Gegenüberstellung der jeweiligen Optionen

### Option 1: Ausschließlich zentral bereitgestellte Postfachzugänge

Es wäre denkbar, alle Postfachzugänge zentral bereitzustellen und [das System] grundsätzlich geschlossen für andere Postfachzugänge zu halten. In diesem Fall wäre die direkte Anbindung von Fachverfahren und anderen Lösungen ausgeschlossen. Diese Lösung wird aufgrund der fachlichen Anforderungen ausgeschlossen.

- Gut, weil ein kontrollierbares und verlässliches Verhalten von Postfachzugängen unterstellt werden kann.
- Gut, weil Eigenschaften wie Sicherheit für [Nutzer:innen] zentral sichergestellt werden kann.
- Schlecht, weil die Anbindung von Fachverfahren nicht möglich ist.
- Schlecht, weil die Einhaltung schwer zu kontrollieren ist und Maßnahmen nötig werden.
- Schlecht, weil der Zuschnitt von Postfachzugängen auf bestimmte Nutzergruppen unmöglich wird.

### Option 2: Zentral bereitgestellte User-Interfaces mit Öffnung für Fachverfahren und Zwischenformen

Während bei Fachverfahren die Einschränkung praktisch nicht realistisch ist, kann als Client (User Interface) eine oder mehrere Implementierungen bereitgestellt und nur diese akzeptiert werden. Die Sicherheit dieser Clients – auch im Interesse der [Nutzer:innen] – kann somit an zentraler Stelle sichergestellt werden. Andererseits werden Möglichkeiten der Gestaltung eingeschränkt und es Bedarf technischer und/oder fachlicher Maßnahmen, um andere Implementierungen auszuschließen. Zudem ist es gerade in Zwischenformen häufig nicht eindeutig, ob eine Implementierung zuzulassen wäre.

- Gut, weil ein kontrollierbares und verlässliches Verhalten von Postfachzugängen unterstellt werden kann.
- Gut, weil Eigenschaften wie Sicherheit für [Nutzer:innen] zentral sichergestellt werden kann.
- Neutral, weil die Entscheidung zur Zulassung in Einzelfällen schwierig ist.
- Schlecht, weil die Einhaltung schwer zu kontrollieren ist und Maßnahmen nötig werden.
- Schlecht, weil der Zuschnitt von Postfachzugängen auf bestimmte Nutzergruppen unmöglich wird.

### Option 3: Freigabe und Zertifizierung

Ein fachlicher Prozess könnte Implementierungen Dritter Verifizieren und deren Betrieb pro Implementierung erlauben. Dieser Prozess könnte sowohl Clients als auch Fachverfahren betreffen. Somit könnte die Sicherheit aller im System befindlichen Postfachzugänge von zentraler Stelle kontrolliert werden. Dieser Prozess verursacht jedoch Aufwand und es bedarf technischer Maßnahmen, um eine Umgehung zu vermeiden. Diese organisatorischen Maßnahmen sind aufwändig und beeinträchtigen potentiell die organisatorische Skalierbarkeit des Gesamtsystems.

- Gut, weil ein zumindest teilweise kontrollierbares und verlässliches Verhalten von Postfachzugängen unterstellt werden kann.
- Gut, weil Eigenschaften wie Sicherheit für [Nutzer:innen] zumindest zentral geprüft werden kann.
- Neutral, weil der Zuschnitt auf bestimmte Benutzergruppen mit organisatorischer Hürde möglich ist.
- Schlecht, weil die Einhaltung schwer zu kontrollieren ist und Maßnahmen nötig werden.
- Schlecht, weil der Kontrollprozess Aufwand verursacht.
- Schlecht, weil unterschiedliche Postfachzugänge unterschiedliche Qualität aufweisen können und so einzelne Implementierungen ein schlechtes Licht auf [das System] werfen können.

### Option 4: Offenheit für Drittanwendungen

Die Anbindung beliebiger Postfachzugänge - also beliebiger clientseitiger Anwendungen - im Gesamtsystem wird ohne wesentliche Beschränkungen und Verifizierung akzeptiert. Die Fehlerfreiheit und Sicherheit solcher von Dritten bereitgestellten Zugängen wird nicht an zentraler Stelle geprüft. Die Sicherheit des Gesamtsystems basiert auf der Sicherheit der Schnittstellen der Postfach-Backends und der konsequenten Umsetzung des Zero-Trust-Prinzips und E2EE. Eine Registrierung angeschlossener Postfachzugänge z.B. in einem Self-Service-Portal kann dabei ggf. aus pragmatischen Gründen (z.B. zur vereinfachten Adressierung) erfolgen, geht aber nicht mit Beschränkungen oder Prüfungen der anzubinden Software einher. Es besteht daher eine geteilte Verantwortung für die Ende-zu-Ende-Sicherheit der Kommunikationsstrecke. Hinweis: Zentral und nativ bereitgestellte Postfachzugänge sind _nicht_ ausgeschlossen, vergleiche hierzu [ADR-0005](ADR-0005-Anzahl-und-Zuschnitt-der-zentral-bereitgestellten-Postfach-Zugänge.md).
- Gut, weil so die direkte und niedrigschwellige Integration in clientseitige Drittanwendungen möglich ist, sodass Funktionen zum Nachrichtenaustausch dezentral in eine beliebige Zahl fachspezifischer Anwendungen integriert werden kann (z.B. Fachverfahren, Online-Dienste, Apps z.B. von Krankenkassen, Städten, etc.). Diese dezentrale Anbindung kann durch die Bereitstellung von SDKs unterstützt werden (vgl. [ADR-0009](ADR-0009-SDKs-zur-vereinfachten-Anbindung-von-Postfach-Zugängen.md))
- Gut, weil keine zusätzlichen organisatorischen und technischen Maßnahmen zur Kontrolle und Einhaltung von Regeln getroffen werden müssen. Dies begünstigt die schnelle und dezentrale Skalierung der Anbindung an die Infrastruktur erheblich und spart Ressourcen.
- Gut, weil um die Kommunikationsinfrastruktur so ein offenes Ökosystem entstehen kann, was nachgelagerte Innovation begünstigt und nicht-lineare Skalierung über Netzwerkeffekte ermöglicht.
- Neutral, da die Informationssicherheit von Nutzer:innen teils von der Sicherheit, Qualität und Fehlerfreiheit der Implementierung der clientseitigen Anwendung abhängt und vollständig in der Verantwortung der sich anschließenden. 
- Schlecht, weil von unerwartetem Verhalten von Postfachzugängen grundsätzlich ausgegangen werden muss.
- Schlecht, weil unterschiedliche Postfachzugänge unterschiedliche Qualität aufweisen können und so einzelne Implementierungen ein schlechtes Licht auf [das System] werfen können.

## Architekturentscheidung

Gewählte Option: [**Option 4: Offenheit für Drittanwendungen**](#option-4-offenheit-für-drittanwendungen), da gegenüber Option 1, 2 und 3 der zu erwartende Aufwand zur Einrichtung von Kontrollstrukturen vergleichsweise hoch ist und durch diese unter einem Zero-Trust-Szenario kein hinreichend großer Nutzen zu erwarten ist und da durch ein offenes Ökosystem an Drittanwendungen gegenüber Option 1, 2 und 3 zudem eine dezentrale, innovative und aufwandsarme Skalierung der Gesamtinfrastruktur ermöglicht wird. In einem solchen offenen Ökosystem können ohne größere Anschlusshürden Lösungen für spezifische Nutzerinnen:gruppen entwickelt und eingesetzt werden. 
### Konsequenzen

- Es entsteht kein zusätzlicher Aufwand zur Zulassung oder Verifikation von Postfachzugängen.
- Es wird bewusst akzeptiert, dass für Fachverfahren und insbesondere auch für Clients nicht alle im System eingesetzten Implementierungen bekannt sind und die Verantwortung für die jeweilige eigene Sicherheit bei der einsetzenden Stelle liegt.
- Marktteilnehmer können ohne weitere Hürden Clients für spezifische Nutzergruppen entwickeln („wie bei E-Mail“).
- Der Entwicklung sicherer APIs kommt eine erhöhte Bedeutung zu.
- Eine separate Authentifizierung von Systemen (z.B. Fachverfahren) entfällt; da ohnehin beliebige technische Postfachzugänge akzeptiert werden, wird als einziges Authentizitätsmerkmal die Identität der Person, Behörde, Abteilung o.ä. verwendet. Dies Entspricht der Umsetzung des Zero-Trust-Ansatzes (vgl. Anforderung SIC_ÜBE_ÜBE_018).
