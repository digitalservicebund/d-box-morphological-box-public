> **Status:** ✔️ Operativ entschieden | **Datum:** 14. April 2025

# ADR-0009: SDKs zur vereinfachten Anbindung von Postfach-Zugängen

### Kontext und Problembeschreibung

Dieser ADR erörtert und regelt, ob und inwiefern Hilfsmodule für die Anbindung an die Kommunikationsinfrastruktur als zentral bereitgestellter Baustein entwickelt und angeboten werden, insbesondere die Bereitstellung von Werkzeugen und Bibliotheken in der Form von integrierten und integrierbaren Software Development Kits (SDK)_.
Er dient damit u.A. Der Erörterung und Bestätigung von Anforderung ANF_ARC_ANB_018: _[Das System] unterstützt die einfache Anbindung durch Bereitstellung von Software Development Kits (SDKs)_.

> [!note] Hinweis
> SDK sind installierbare und damit in Drittanwendungen integrierbare Pakete, die Entwickler:innen mit Programmierwerkzeugen und Programmbibliotheken bei der Softwareentwicklung unterstützen. [Mehr über SDKs.](https://de.wikipedia.org/wiki/Software_Development_Kit) 

Mit Blick auf die Kommunikationsinfrastruktur könnten SDKs diverse für die Anbindung an die Kommunikationsinfrastruktur notwendige Funktionen kapseln, z.B. im Kontext der Umsetzung von Ende-zu-Ende-Verschlüsselung und der Schlüsselverwaltung, der Adressierung von Kommunikationsparteien, u.V.m. 

Dieser ADR regelt nicht für welche Programmiersprachen und Umgebungen SDKs zur Verfügung gestellt werden.
Dieser ADR regelt zudem nicht, welchen konkreten Funktionsumfang etwaige bereitzustellende SDKs abbilden sollen.

## Entscheidungstreiber

- ANF_ARC_ANB_008: Die Anbindung an [das System] muss so einfach sein, dass eine große Zahl an [Nutzer:innen] aufwandsarm an [das System] angebunden werden kann.
- ANF_ARC_ANB_018: [Das System] unterstützt die einfache Anbindung durch Bereitstellung von Software Development Kits (SDKs).
- WAR_MOD_ÜBE_001: [Das System] ist bezüglich seiner Komponenten/Dienste modular aufgebaut und lose gekoppelt.
- In der Regel existieren für einen gegebenen Anwendungsfall separate SDKs für verschiedene Laufzeitumgebungen (z.B. Für verschiedene Programmiersprachen, Betriebssysteme, Hardware, etc.). 
- [ADR-0011](ADR-0011-Kommunikationsschicht.md)  empfiehlt den Einsatz des [offenen Kommunikationsspezifikation _Matrix_](https://spec.matrix.org/latest/) als Kommunikationsschicht der hier beschriebenen Infrastruktur. [Für den Einsatz von Matrix existieren bereits diverse SDKs unter offenen Lizenzen und in verschiedenen Programmiersprachen](https://matrix.org/ecosystem/sdks/). 
- SDKs werden aktuell bereits erfolgreich für die vereinfachte Anbindung mehrerer bundesweit relevanter IT-Lösungen in der deutschen öffentlichen Verwaltung eingesetzt, bspw. durch die [AusweisApp des elektronischen Personalausweises](https://www.ausweisapp.bund.de/sdk/) und [FIT-Connect](https://docs.fitko.de/fit-connect/docs/sdks/overview).
- SDKs anzubieten ist insbesondere zweckdienlich bei hoher Komplexität der genutzten APIs, hoher Anzahl an zu integrierenden Systemen, hoher Anzahl an mit der Integration betrauten Entwickler:innen-Teams, Fehleranfälligkeit der technischen Anbindung, hohen Anforderungen hinschlich Entwicklungsgeschwindigkeit und Skalierbarkeit der Gesamtinfrastruktur sowie hohen Sicherheitsanforderungen.
- SDKs anzubieten ist nicht zweckdienlich bei niedriger Kontrolle über die Infrastruktur, bei niedrigen Kapazitäten und Ressourcen für die (Weiter)entwicklung der SDKs, bei hohen Anforderungen an Performance und Ressourcenverbrauch der angeschlossenen Systeme, bei technisch und fachlich sehr heterogenen Drittsystemen.

## Betrachtete Optionen

- Option 1: Bereitstellung von SDKs zur vereinfachten Anbindung von Postfach-Zugängen
- Option 2: Keine Bereitstellung von SDKs

## Gegenüberstellung der jeweiligen Optionen

### Option 1: Bereitstellung von SDKs zur vereinfachten Anbindung von Postfach-Zugängen

Als nativer Teil der Infrastruktur werden an zentraler Stelle SDKs für die einschlägigen Programmiersprachen und Plattformen entwickelt und unter einer offenerer Lizenz bereitgestellt.
Diese SDKs kapseln wesentliche Funktionen mit Relevanz für die Anbindung an und Nutzung der Kommunikationsinfrastruktur.
Diese SDKs können durch Dritte in deren jeweils eigener Umgebung integriert und so die Anbindung an die Kommunikationsinfrastruktur vereinfacht werden.

- Gut, aufgrund vereinfachter und beschleunigter Anbindung durch Entwickler:innen. Dies senkt die Schwelle für den Anschluss an die Kommunikationsinfrastruktur und begünstigt damit deren Skalierung und zügige Verbreitung im öffentlichen Sektor. 
- Gut, aufgrund erhöhtet Konsistenz in der Anbindung und gesenkter Wahrscheinlichkeit von Entwicklungsfehlern im technischen Anbindungsprozess
- Gut, aufgrund der vermutlich geringen Vertrautheit öffentlicher IT-Dienstleistern mit den einzusetzenden modernen kryptografischen und Transport-Standards. SDKs senken die Anforderungen an Entwickler:innen hinsichtlich Vorerfahrung und Detailverständnis.
- Gut, aufgrund der perspektivisch hohen Anzahl an in Eigenverantwortung (im Self-Service, vgl. [ADR-0008](ADR-0008-Einheitliche-Anbindung-ueber-ein-zentrales-Self-Service-Portal.md)) und durch separate Entwickler:innen-Teams anzubindenden Organisationen. 
- Neutral, aufgrund zusätzlicher Kosten, da SDKs als Softwareprodukte entwickelt und fortlaufend bedarfsorientiert Weiterentwickelt werden müssen
- Schlecht, da die durch SDKs realisierbaren Vorteile ggf. durch die technische Heterogenität unter anzubindenden Postfach-Zugängen  (eingesetzte Programmiersprachen, Betriebssysteme) beschränkt sind.

### Option 2: Keine Bereitstellung von SDKs

Es werden keine dedizierten SDKs von zentraler Stelle einwickelt und zur Verfügung gestellt. Die Anbindung von Postfach-Zugängen wird unter Zuhilfenahme der verfügbaren Dokumentationen, Standards, Spezifikationen und des Self-Service-Portales (vgl. [ADR-0008](ADR-0008-Einheitliche-Anbindung-ueber-ein-zentrales-Self-Service-Portal.md)). durch Entwickler:innen des anzuschließenden Postfach-Zugängen eigenverantwortlich umgesetzt. Dafür greifen diese direkt auf die APIs der Infrastruktur zu.

- Gut, da keine zusätzlichen Kosten und kein  zusätzlicher zentraler Entwicklungs- und Wartungsaufwand entstehen. 
- Schlecht, da die Vorteile der Option 1 nicht realisiert werden können, was die Geschwindigkeit, Einfachheit und Qualität der Anbindung von Postfach-Zugängen beeinträchtigt, wodurch insbesondere selbstverstärkende Netzwerkeffekte in der Skalierbarkeit der Gesamtinfrastruktur geschwächt werden.
- Schlecht, da \[das System\] insbesondere entsprechend [ADR-0011](ADR-0011-Kommunikationsschicht.md) und [ADR-0012](ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md) sowohl für die Kommunikation im engeren Sinne als auch die Ende-zu-Ende-Verschlüsselung auf moderne offene Standards setzt, deren korrekte Implementierung ohne die Nutzung einschlägige Hilfsmodule wie SDKs herausfordernd sein kann.

## Architekturentscheidung

Gewählte Option: **[Option 1: Bereitstellung von SDKs zur vereinfachten Anbindung von Postfach-Zugängen](#option-1-bereitstellung-von-sdks-zur-vereinfachten-anbindung-von-postfach-zugängen)**, da diese auch unter ggf. technisch anspruchsvollen Voraussetzen die einfache, sichere und konsistente Anbindung einer großen Anzahl an Postfach-Zugängen wirksam unterstützen und so die Skalierbarkeit und Entwickler:innen-Freundlichkeit der gesamten Kommunikationsinfrastruktur erheblich verbessern. Etwaige Nachteile in Form von überschaubare Entwicklungskosten sowie die technische Heterogenität unter anzubindenden Systemen wiegen diese Vorzüge nicht auf. 

### Auswirkungen

- Es ist weiterführend zu entscheiden, für welche Programmiersprachen und Plattformen SDKs angeboten werden sollen. Hierfür ist der Bedarf bei den Entwickler:innen von möglichen anzubindenden Postfach-Zugängen  empirisch belastbar zu ermitteln.
- Die Entwicklung und langfristige Pflege entsprechender SDKs muss durch eine verantwortliche Stelle geplant und umgesetzt, organisatorisch verankert und belastbar finanziert werden. 