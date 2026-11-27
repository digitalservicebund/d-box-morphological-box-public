> **Status:** ✔️ Operativ entschieden | **Datum:** 14. April 2025

> **Hinweis (Phase 2, Stand Oktober 2026):** Diese Entscheidung wurde in Phase 2 ergänzt. Der Ablauf des Onboardings im Self-Service-Portal ist in ADR-1148 festgelegt.

## ADR-0008: Einheitliche Anbindung über ein zentrales Self-Service-Portal

## Kontext und Problembeschreibung

Die technische Anbindung von Postfach-Zugängen (z.B. Clients, Fachverfahren, Online-Dienste, Unternehmensanwendungen, etc.) und Postfach-Backends an die Kommunikationsinfrastruktur kann ggf. die Registrierung von anzuschließender Systemen notwendig machen.
Hierfür werden durch die Betreiber üblicherweise Online-Dienste angeboten, die die eigenständige Registrierung und Anbindung durch Entwickler:innen erlauben und diese durch den Anbindungsprozess leiten (Self-Service-Portal (SSP)).
Diese Self-Service-Portale bündeln ggf. Funktionen zur Aufnahme von Daten zur Identifikation, Authentifizierung und Adressierung angeschlossener Systeme, Funktionen zur Verwaltung von registrierten Systemen, Funktionen zum Testen der Anbindungen, Funktionen zur Annahme der ggf. geltenden Nutzungsbedingungen und mehr.
SSPs werden üblicherweise in direktem Zusammenspiel mit statischen Informationsquellen wie z.B. einschlägigen Software-Dokumentationen online veröffentlicht bzw. in diese eingebettet.
Primäre Zielgruppe derartiger SSPs sind Entwickler:innen von \[öffentlichen Stellen\] und \[privaten Organisationen\].

> [!note] Hinweis
> Derartige Self-Service-Portale existieren in der öffentlichen Verwaltung aktuell beispielsweise für die Anbindung der [BundID](https://ssp.id.bund.de/ip?id=ip_public), des [Mein Unternehmenskontos](https://service.mein-unternehmenskonto.de/) oder von [FIT-Connect](https://portal.auth-testing.fit-connect.fitko.dev/landing).
Dieser ADR regelt, ob und in welchem Zuschnitt ein oder mehrere SSPs für die Anbindung an die durch die Zielarchitektur P&K beschriebene konsolidierte Kommunikationsinfrastruktur existieren.

Dieser ADR regelt nicht, ob und wie die Anbindungsprozesse und APIs konkreter aktueller Bestandssysteme ggf. angeglichen oder vereinheitlicht werden. Dieser ADR trifft zudem kein Aussagen über Self-Service-Angebote zur Registrierung von Nutzer:innen an Identitätsquellen (_Identity Provider, IdP_).

## Entscheidungstreiber

- Die gebündelte Bereitstellung aller für ein gegebenes Software-Produkt relevanten Entwickler:innen-Ressourcen an einem Ort gilt als Best-Practice, da so eine autoritative Quelle der Wahrheit für Entwickler:innen geschaffen wird
- Die Abwicklung des Anbindungsmanagements für IT-Systeme über gut dokumentierte Self-Service-Angebote vermeidet das Entstehen von etwaigen durch manuelle Bearbeitung entstehenden Flaschenhälsen im Anbindungsprozess
- Die Anbindung von Drittsystemen an die Kommunikationsinfrastruktur wird voraussichtlich v.A. durch bzw. für Organisationen durchgeführt. Es ist nicht anzunehmen, dass sich dieser Prozess für öffentliche Stellen und private Organisationen aus technischer Perspektive maßgeblich unterscheidet. 
- *ANF_FUN_USE_080: \[Öffentliche Stellen\] und \[private Organisationen\] können über eine Self-Service-Funktionalität eine Übersicht ihrer \[Postfächer\] einsehen.* 
- *SIC_ÜBE_ÜBE_018: [Das System] setzt den Zero Trust Ansatz um.* Die Vertraulichkeit und Authentizität der Kommunikation darf also nicht an die Vertrauenswürdigkeit der beteiligten IT-Systeme gekoppelt sein. Es besteht daher keine besondere Notwendigkeit zur Überprüfung und geschäftlichen Autorisierung von an der Infrastruktur beteiligten Backend-Systemen (vgl. hierzu [ADR-0003](ADR-0003-Bereitstellung-von-Postfach-Zugängen.md))
## Betrachtete Optionen

- [Option 1: Bereitstellung eines zentralen Self-Service-Portals zur Anbindung an die Kommunikationsinfrastruktur für alle Nutzer:innen-Gruppen](#option-1-bereitstellung-eines-zentralen-self-service-portals-zur-anbindung-an-die-kommunikationsinfrastruktur-für-alle-nutzerinnen-gruppen) 
- [Option 2: Bereitstellung mehrerer fachlich zugeschnittener Self-Service-Portale mit ggf. unterschiedlichem Branding und ggf. unterschiedlichen Anbindungsprozessen für verschiedene Nutzer:innen-Gruppen](#option-2-bereitstellung-mehrerer-fachlich-zugeschnittener-self-service-portale-mit-ggf-unterschiedlichem-branding-und-ggf-unterschiedlichen-anbindungsprozessen-für-verschiedene-nutzerinnen-gruppen)

## Gegenüberstellung der jeweiligen Optionen

### Option 1: Bereitstellung eines zentralen Self-Service-Portals zur Anbindung an die Kommunikationsinfrastruktur für alle Nutzer:innen-Gruppen 

Komplexitätsreduktion und Entwickler:innen-Freundlichkeit des Anbindungsprozesses und der für die Anbindung notwendigen IT-Systeme wird priorisiert. Ein weitestmöglich einheitlicher, im Self-Service durchführbarer Anbindungsprozess wird für die Kommunikationsinfrastruktur definiert.
Variationen des Prozesses existieren z.B. für verschiedene Nutzer:innen-Gruppen, nur insofern besondere fachliche oder rechtliche Gründe dies notwendig machen.
Alle für die Anbindung von Drittsystemen an die Infrastruktur notwendigen Funktionalitäten werden für alle Nutzer:innen-Gruppen (öffentliche Stellen aller Sektoren und Fachbereiche sowie private Organisationen wie Unternehmen, Vereine, etc.) in einem Online-Dienst gebündelt und etwaige Variationen in diesem abgebildet.

- Gut, da die Komplexität in der Föderalen IT-Landschaft reduziert und SSPs als Quellen existierender bzw. zukünftiger Heterogenität vermieden werden
- Gut, da dies die Entwickler:innen-Erfahrung verbessert
- Gut, da dies eine Quelle der Wahrheit und einen zentralen Anlaufpunkt für die Anbindung an die Kommunikationsinfrastruktur schafft 
- Gut, da dies die zentrale Fortschreibung und Versionierung technischer und geschäftlicher Anbindungsprozesse unmittelbar ermöglicht und so deren Governance mittelfristig vereinfacht.
- Neutral, da dies Wahrnehmung der im Sinne von [ADR-0002](ADR-0002-Infrastrukturtopologie.md) föderierten Kommunikationslösungen als eine geschlossene Infrastruktur begünstigt. *Hinweis: Dies macht es ggf. notwendig, der Kommunikationsinfrastruktur einen sprechenden Eigennamen zu geben.*
- Neutral, da die Spezifikation von über die jeweiligen Zielgruppen hinweg generalisierbaren Anbindungsprozessen und Programmierschnittstellen (APIs) der Umsetzung eines solchen zentralen SSP vorausgehen muss.
- Schlecht, da dies mit etablierten fachlichen und kommunikativen Zuschnitten existierender SSPs bricht, was ggü. Entwickler:innen und anderen Stakeholdern erklärungsbedürftig ist.

### Option 2: Bereitstellung mehrerer fachlich zugeschnittener Self-Service-Portale mit ggf. unterschiedlichem Branding und ggf. unterschiedlichen Anbindungsprozessen für verschiedene Nutzer:innen-Gruppen

Die Anpassung an ggf. bestehende fachliche und kommunikative Rahmenbedingungen wird priorisiert. Der Anbindungsprozess und entsprechende Online-Dienste zur Unterstützung der Anbindung werden in einem konkreten fachlichen Kontext kommuniziert und strukturiert. Es existieren ggf. mehrere SSPs für unterschiedliche Nutzer:innen-Gruppen welche die für die Anbindung notwendigen Funktionalitäten im fachlichen Kontext eingebettet bündeln (z.B. SSP für die Anbindung durch Unternehmen, durch Gerichte, durch Stellen der öffentlichen Verwaltung, etc.). 

- Gut, da die kommunikative Einbettung im fachlichen Kontext erleichtert wird (z.B. Als Teil bestehender Dokumentationen) 
- Gut, da ggf. notwendige Variationen des Anbindungsprozesses für verschiedene Nutzer:innen-Gruppen umfangreicher abbildbar sind. 
- Neutral, da dies Wahrnehmung der im Sinne von [ADR-0002](ADR-0002-Infrastrukturtopologie.md) föderierten Kommunikationslösungen als eine geschlossene Infrastruktur hemmt.
- Schlecht, da identische oder ähnliche Informationen zur Anbindung fortlaufend an mehreren Orten gepflegt werden müssen, sodass keine Quelle der Wahrheit existiert und die Fortschreibung des Anbindungsprozesses erschwert wird. 
- Schlecht, da mehrere funktional ähnliche Online-Dienste betrieben und so die Komplexität der föderalen IT erhöht wird. 

## Architekturentscheidung

Gewählte Option: **[Option 1: Bereitstellung eines zentralen SSPs zur Anbindung von Postfach-Zugängen und Postfach-Backends für alle Nutzer:innen-Gruppen](#option-1-bereitstellung-eines-zentralen-self-service-portals-zur-anbindung-an-die-kommunikationsinfrastruktur-für-alle-nutzerinnen-gruppen)**, da so gegenüber Option 2 wesentliche Vorteile hinsichtlich der Komplexitätsreduktion in der föderalen IT, der Entwickler:innen-Freundlichkeit sowie einer einfachen Governance des Anbindungsprozesses realisiert werden können. Die in erster Linie kurzfristigen kommunikativen Vorzüge von Option 2 wiegen dies nicht auf. 

### Auswirkungen

- Bestehende Self-Service-Portale müssen nach Vereinheitlichung und Spezifikation von generalisierbaren Anbindungsprozessen und Programmierschnittstellen (APIs) mittelfristig an zentraler Stelle gebündelt werden

## Weitere Informationen

Ähnliche Überlegungen zur Konsolidierung von Ressourcen für Entwickler:innen lassen sich auch für weitere Self-Service-Angebote zur Anbindung weiterer querschnittlicher Bausteine in der föderalen IT anstellen.
Eine übergreifendere Konsolidierung solcher Angebote zu z.B. einem im Self-Service nutzbaren föderalen Anbindungs- und Entwicklungsportal sollte durch das FIT-AB evaluiert werden.