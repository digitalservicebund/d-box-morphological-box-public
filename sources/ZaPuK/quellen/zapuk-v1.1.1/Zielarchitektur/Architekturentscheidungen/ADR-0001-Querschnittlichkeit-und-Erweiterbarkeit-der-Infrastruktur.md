> **Status:** ✔️ Operativ entschieden | **Datum:** 15. April 2025

# ADR-0001: Querschnittlichkeit und Erweiterbarkeit der Infrastruktur

## Kontext und Problembeschreibung

Die Kommunikation von öffentlichen Stellen mit Privatpersonen und Unternehmen erfolgt zumeist im Kontext der Durchführung eines fachlich abgrenzbaren Prozesses, Falls, Verfahrens oder Vorgangs (z.B. Verwaltungsverfahren in der öffentlichen Verwaltung, Gerichtsverfahren in der Justiz, etc.). Die digitale Ende-zu-Ende Bearbeitung von Vorgängen wird mithilfe mehrerer zusammenwirkender IT-Systeme umgesetzt, welche die verschiedenen hierfür notwendigen fachspezifischen (z.B. Prüfung eines Antrags auf Vollständigkeit und ähnliche fachliche Bearbeitungslogik im Kontext eines Verwaltungsverfahrens) und querschnittliche bzw. nicht fachspezifische Funktionen (z.B. Transport der Nachricht von der Sender:in zum Empfänger:in) umsetzen. 

Dieser ADR regelt abstrakt und übergreifend, ob und in welchem Ausmaß über die Kernfunktionen der Kommunikationsinfrastruktur im Sinne der identifizierten Anforderungen (vgl. [hier](../../Konsolidierte-Anforderungen/README.md)) hinausgehende fachspezifische und querschnittliche Funktionen  ggf. innerhalb bzw. als nativer Teil einer föderalen Postfach- und Kommunikationsinfrastruktur umgesetzt oder unterstützt werden sollen. 
Dieser ADR betrachtet die Postfach- und Kommunikationsinfrastruktur als Gesamtsystem. Er trifft keine Aussagen darüber, in welchen konkreten Architektur- und Lösungsbausteinen der Infrastruktur fachspezifische Funktionen ggf. umzusetzen oder nicht umzusetzen sind bzw. in Form welcher Artefakte diese ggf. umzusetzen sind (bspw. durch Standardisierung, die Umsetzung in Referenzimplementierungen, etc.).

## Entscheidungstreiber

- Die Kommunikationsinfrastruktur wird perspektivisch von vielen verschiedenen und fachlich heterogenen Nutzer:innen genutzt (Individuen im privaten und organisatorischen Kontext, verschiedene organisatorische Strukturen, verschiedene Rechtsformen, verschiedene Staatsgewalten, unterschiedliche föderale Ebenen, verschiedene Fachlichkeiten der öffentlichen Verwaltung, verschiedenste Verwaltungsverfahren, etc.). 
- Fachliche Verarbeitungslogiken sind zwischen öffentlichen Stellen notwendigerweise sehr heterogen. 
- Zusätzliche funktionale Charakteristiken  erhöhen die Komplexität eines Softwareproduktes. 
- Die Umsetzung jeder Anforderung geht vor dem Hintergrund beschränkter Entwicklungsressourcen mit Opportunitätskosten einher. 

## Betrachtete Optionen

- [Option 1: Fachspezifische Funktionen inklusive fachliche Verarbeitungslogik werden bei Bedarf in der Infrastruktur verankert](#option-1-fachspezifische-funktionen-inklusive-fachlicher-verarbeitungslogik-werden-bei-bedarf-in-der-infrastruktur-verankert)
- [Option 2: Nur wenige querschnittliche Funktionen werden in der Infrastruktur verankert. Die Infrastruktur ist modular erweiterbar](#option-2-die-infrastruktur-ist-modular-um-wenige-querschnittliche-funktionen-erweiterbar)
- [Option 3: Fachspezifische und querschnittliche Funktionen werden nicht in der Infrastruktur verankert](#option-3-fachspezifische-und-querschnittliche-funktionen-werden-nicht-in-der-infrastruktur-verankert)

## Gegenüberstellung der jeweiligen Optionen

### Option 1: Fachspezifische Funktionen inklusive fachlicher Verarbeitungslogik werden bei Bedarf in der Infrastruktur verankert. 

Fachspezifische Funktionen, inklusive fachlicher Verarbeitungslogik, werden bedarfsorientiert in der Infrastruktur verankert. 

- Gut, da aus der Binnensicht einer Fachdomäne ggf. übergreifende Funktionen und Teile der Prozesssteuerung aus Fachverfahren ausgelagert werden können 
- Neutral, da für viele fachspezifische Funktionen und Verarbeitungslogiken Kenntnis und fachspezifische Verarbeitung des Nachrichteninhalts bzw. unverschlüsselter fachlicher Metadaten notwendig ist. Dies stellt einen Zielkonflikt mit Ende-zu-Ende-Verschlüsselung als wesentliche Anforderung dar und beschränkt die prinzipiell abbildbaren fachlichen Funktionen. 
- Schlecht, da dies zu einer erhöhten Komplexität im Gesamtsystem führt 
- Schlecht, da dies die effektive Priorisierung und Abwägung von Anforderungen im Weiterentwicklungsprozess erschwert. Die Weiterentwicklung der Kommunikationsinfrastruktur wäre ein möglicher Flaschenhals in der Weiterentwicklung elektronischer Fachverfahren.  
- Schlecht, da dies aus Sicht eines Teils der Nutzer:innen zu ineffizientem Einsatz von Entwicklungsaufwand und Ressourcen führen kann.   
- Schlecht, da dies die lose Kopplung einzelner Architekturbausteine in der föderalen Gesamtarchitektur unterminiert und damit Veränderungen an fachspezifischen Komponenten wie Online-Diensten und Fachverfahren erschwert. 

### Option 2: Die Infrastruktur ist modular um wenige querschnittliche Funktionen erweiterbar

Wenige, ausgewählte  Funktionen können in der Infrastruktur verankert werden, wenn diese  querschnittlicher Natur sind und bei überschaubarem Aufwand und Komplexitätszuwachs für eine qualifizierte Mehrheit der Nutzer:innen konkrete Mehrwerte realisieren (z.B. Funktionen für die Unterstützung öffentlicher Stellen bei der Terminvereinbarung, Abstimmungen, etc.). Die Anzahl der in der Infrastruktur verankerten querschnittlichen Funktionen wird sorgfältig abgewogen und möglichst minimiert. Die Aufrechterhaltung der Ende-zu-Ende-Verschlüsselung hat Vorrang gegenüber etwaigen funktionalen Erweiterungen. Das System ist jedoch modular erweiterbar, sodass ggf. fortlaufend und bedarfsorientiert Funktionalitäten ergänzt werden können. 

- Gut, da querschnittliche Funktionen an zentraler Stelle umgesetzt werden können und zu die Nutzer:innenerfahrung vereinheitlicht..
- Gut, da Redundanzen durch die wiederholten Entwicklung von vielerorts benötigten querschnittlichen Funktionen vermieden werden.
- Neutral, da für viele fachspezifische Funktionen und Verarbeitungslogiken Kenntnis und fachspezifische Verarbeitung des Nachrichteninhalts bzw. unverschlüsselter fachlicher Metadaten notwendig ist. Dies stellt einen Zielkonflikt mit Ende-zu-Ende-Verschlüsselung als wesentliche Anforderung dar und beschränkt die prinzipiell abbildbaren fachlichen Funktionen. 
- Neutral, da diese Option die Nachteile von Option 1 abmildert. 

### Option 3: Fachspezifische und querschnittliche Funktionen werden nicht in der Infrastruktur verankert

Keine fachspezifische Funktionen werden technisch in der Infrastruktur verankert. Die Infrastruktur setzt ausschließlich fachlich agnostische Funktionen um, welche die wesentlichen durch eine Postfach- und Kommunikationsinfrastruktur umzusetzenden Fähigkeiten realisieren. Fachliche Verarbeitungslogik jeglicher Art ist bedarfsorientiert in Drittsystemen zu verankern. 

- Gut, da die Komplexität des Gesamtsystems minimiert wird
- Gut, da zusätzlichen Abstimmungsaufwände im Entwicklungsprozess reduziert werden
- Schlecht, da die Vorteile von Option 1 und 2 nicht realisiert werden können.

## Architekturentscheidung

Gewählte Option: [**Option 2: Die Infrastruktur ist modular um wenige querschnittliche Funktionen erweiterbar.**](#option-2-die-infrastruktur-ist-modular-um-wenige-querschnittliche-funktionen-erweiterbar), da die beschriebene Kommunikationsinfrastruktur die Natur einer fachlich agnostischen Querschnitts- bzw. Basiskomponente hat. Fachspezifische Prozesse und Funktionen werden nicht in der Infrastruktur verankert. Dies erlaubt es gegenüber Option 3 wesentliche funktionale Mehrwerte für Endnutzer:innen zu heben (z.B. die native Umsetzung von Terminabfragen, Abstimmungen, etc.) und verhindert gegenüber Option 1 einen unkontrollierten Zuwachs an konkurrierenden fachspezifischen Anforderungen in der Entwicklung sowie die dadurch zu befürchtende, schwer handhabbare Komplexität der Gesamtinfrastruktur sowie gegenüber Option 1 eine konsequente Aufrechterhaltung der Ende-zu-Ende Verschlüsselung. 


### Bestätigung

Die Einhaltung ist durch entsprechende Prüfkriterien im Anforderungsmanagement-Prozess zu verankern und fortlaufend in der Bewertung und Priorisierung von Anforderungen in Softwareentwicklungsprozessen zu berücksichtigen. Die beteiligten Produktmanager:innen und Lösungsarchitekt:innen sind hierfür zu sensibilisieren. Die übergreifende Einhaltung kann regelmäßig oder anlassbezogen durch Architekturreviews geprüft werden. 
