<!-- Auszug aus dem Issue-Tracker des openCode-Repositorys ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/api/v4/projects/3799/issues (öffentliche Issues, ohne Kommentare: Kommentare erfordern Anmeldung, HTTP 401) -->
<!-- Abgerufen: 2026-11-19; Issue-Beschreibungen unverändert übernommen -->

# ZaPuK openCode-Issues mit Architekturbezug (Konsultation Phase 2, fortlaufend)

Stand Issue-Tracker 2026-11-19: 20 Issues, davon 19 geschlossen, 1 offen (#26 "Neue Anforderung: Payment"). Die Issues #4 bis #15 sind Diskussions-Issues zu den Anforderungsthemen (geschlossen 2026-01-16). Unten die vier Issues mit direktem Bezug zur technischen Architektur. Die Antworten des ZaPuK-Teams liegen in den Kommentaren und waren ohne Anmeldung nicht abrufbar.

## #23 Anschluss an das Peppol-Netzwerk

- URL: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/work_items/23
- Erstellt: 2026-02-10, geschlossen: 2026-05-20, Labels: Ideenpool, Kommentare: 2

### Beschreibung (Originaltext)

<!--> Verständnisfrage-->

### Vertretene Organisation

Ministerium für Arbeit, Soziales, Transformation und Digitalisierung Rheinland-Pfalz

### Frage

Wie wird aktuell ein Anschluss an das Peppol-Netzwerk betrachtet? Für ADR-0010 wäre das sicherlich eine hilfreiche Anbindung, um Interoperabilität für den sicheren Dokumentenaustausch sicherzustellen.

Insbesondere für eine mögliche zukünftige E-Rechnungspflicht im B2C-Bereich könnte das eine wegweisende Funktion sein. Aktuell würde eine solche Pflicht daran scheitern, dass ein verpflichtender digitaler Briefkasten fehlt. Sollte es so einen in Zukunft geben, bspw. über die Bund-ID oder ELSTER, wäre ein Peppol-Anschluss eine sinnvolle Ergänzung, um eine B2C-Verpflichtung zu ermöglichen. Ggf. mal mit dem BMF dazu sprechen, ob die das ähnlich sehen.

<!------ Nachfolgenden Text bitte nicht verändern ------>

## #24 Fehlende Betrachtung Postfach Integration für Maschinenidentitäten in ZAPUK

- URL: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/work_items/24
- Erstellt: 2026-03-16, geschlossen: 2026-04-01, Labels: Allgemein, Kommentare: 1

### Beschreibung (Originaltext)

<!--
> Übergreifendes Feedback
  
  Falls gewünscht, können Sie das Feedback an den folgenden Leitfragen orientieren.  
  Bei freiem Feedback ergänzen oder ersetzen Sie diese gerne.
-->

### Vertretene Organisation

Dataport

## 1. Was sind aus Ihrer Sicht die größten Stärken der Zielarchitektur in Version 1.0 – und was die größten Schwächen?

Im Hinblick auf die Vision eines zentralen Postfachsystems zur Konsolidierung der vielen bestehenden Postfachlösungen wie z. Bsp. BundID ZBP, MUK Elster PF 2.0, OZGPP und ERV der Justiz existiert eine gute Ausformulierte Zielarchitektur allerdings fehlt wie bei deren betrachteten Ausgangspostfächer eine Maschine zu Maschine Integration zur Integration von Automatisierung aus der Wirtschaft mit geeigneten Technologien zur Umsetzung von Zero Trust API Integration über den hier propagierten SDKs.
Klar zu verstehen sind hier zunächst:
- Identitätsnachweis über staatlich oder föderal vertrauenswürdigen IdPs
- Ende zu Ende gesicherte Nachrichtenschicht auf Basis Matrix mit Perspektive MLS
- Messaging Bridges noch unklar in Bezug auf Ende zu Ende Verschlüsselung - aber wohin in welche Tiefe bis zum Desktop des Anwenders oder vielleicht doch bis zu einer Plattform?
- Anbindung über SDKs mit Versionierungskonzept plus kontinuierlicher Weiterentwicklung für Java, .NET etc?

## 2. Welche Anforderungen, Nutzungsszenarien oder architektonischen Abwägungen wurden im ZaPuK-Architekturkonzept und den ADRs bisher nicht oder unzureichend berücksichtigt?

ich denke, dass gerade aus der Wirtschaft ein Interesse an API Automatisierung ähnlich wie bei den Online-Diensten keine UI, in der eine angemeldete Identität etwas tut sondern für den Online-Dienst eine bereitgestellte API in die IT Landschaft des Unternehmens integriert werden muss.
Mir fehlen deshalb Aussagen zu M2M Use Cases:
- OAUTH2/OIDC Client Credential Flows
- signierte Assertions
- mTLS Client Certs
- Modellierung Service-Accounts vs. natürlichen Personen und Organisationsrollen
- Funktionsweise von Funktionspostfächern für Unternehmen ohne interaktive Anmeldung
- Unternehmens Funktionspostfächer mit Vertreterregelung
- Autorisierung über Postfach/ Organisationszuordnung oder auch im Kontext OIDC OAUTH mit scopes, claims
- wie sieht eine API hier aus, die hochgradig automatisiert werden kann, ohne Zero-Trust und E2E Verschlüsselung zu brechen

## 3. Welche Aspekte sollten zwingend vor einer verbindlichen Umsetzung konzeptionell geklärt, nachgeschärft oder technisch pilotiert werden?

Die Sicherheitsmechanik für APIs muss hier nachgeschärft werden, um automatisierte Wirtschaftsanbindungen zu ermöglichen.
Es muss eine klare Abgrenzung zwischen Organisationsidentitäten und Maschinenidentitäten hergestellt werden. was darf eine Anwendung im Sinne ERP, Bot, Connector darf im Namen der Organisation Unternehmen handeln?

Ohne diese Klärung werden menschliche Identitäten für Maschinen missbraucht und führen auch zu groben Berechtigungen, weil dann nicht pro Anwendung nach Zweck und Kontext unterschieden werden kann.

## 4. Wenn Sie eine zentrale Empfehlung zur Verbesserung der Zielarchitektur geben könnten: Welche wäre das?

<!-- Bitte diese Zeile durch Antwort ersetzen. -->

<!-- 
---- Nachfolgenden Text bitte nicht verändern ----
-->

## #25 Klärung Architektursicht API-first oder SDK-first Integration

- URL: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/work_items/25
- Erstellt: 2026-03-16, geschlossen: 2026-05-20, Labels: ADR-0009, Kommentare: 1

### Beschreibung (Originaltext)

<!--
> Übergreifendes Feedback
  
  Falls gewünscht, können Sie das Feedback an den folgenden Leitfragen orientieren.  
  Bei freiem Feedback ergänzen oder ersetzen Sie diese gerne.
-->

### Vertretene Organisation

Dataport

## 1. Was sind aus Ihrer Sicht die größten Stärken der Zielarchitektur in Version 1.0 – und was die größten Schwächen?

Es wird in ADR-0009: SDKs zur vereinfachten Anbindung von Postfach-Zugängen primär von der möglichen Nutzung eines SDKs gesprochen und das halte ich auch für richtig, dass ein SDK nicht das eigentliche Integrationsmodell sondern nur ein "Comfort-layer" über eine stabile, offen spezifizierte API.

Ein SDK Ansatz ist sehr Wartungsintensiv ggü. den Anpassungen in dem jeweiligen Technologie-Stack des Integrationspartners und die Weiterentwicklung in diesem etc.

## 2. Welche Anforderungen, Nutzungsszenarien oder architektonischen Abwägungen wurden im ZaPuK-Architekturkonzept und den ADRs bisher nicht oder unzureichend berücksichtigt?

Es bleibt relativ schwammig, ob nun eine API first oder SDK first Architekturperspektive eingenommen wird.
Gerade in dem von ZAPUK noch nicht hinreichenden Aspekt der Anbindung der Wirtschaft über automatisierbaren M2M Kontext ist ein SDK eher problematischer, weil nicht schlicht eine App angebunden wird sondern ERP, DMS, Archiv, Workflow, Messaging-Gatewqay oder andere Batch Prozesse. dafür braucht man eher einer API first Architektur mit langlebigen Schnittstellenverträgen und weniger implizite Referenzimplementierungen in Form von SDKs als Integrationsvoraussetzung.


## 3. Welche Aspekte sollten zwingend vor einer verbindlichen Umsetzung konzeptionell geklärt, nachgeschärft oder technisch pilotiert werden?

ZAPUK stellt gerade explizit heraus, dass der Anschluss von Drittsystemen erleichtert werden soll, gleichzeitig aber bestehende Anbindungen auf die Protokolle der Zielarchitektur umgestellt werden.
Das spricht eher für ein offenes Protokoll-/ API-Modell als eine primäre Bindung an bestimmte Client Bibliotheken in Form von SDKs.

Dementsprechend die klare Darstellung in der Zielarchitektur in Richtung :
 - normative API-/Protokollspezifikation im Sinne API first
 - maschinenlesbare OpenAPI/AsyncAPI/Event Spezifikationen
 - separate AuthN/ AUThZ profile zwischen Organisations- und Service-Identitäten
 - klare regeln für Versionierung, Abwärtskompatibilität und Kryptointegration/-migration
 - optionale SDKs als "Comfort-Layer" in ausgewählten Sprachen


<!-- 
---- Nachfolgenden Text bitte nicht verändern ----
-->

## #27 Auswirkungen von Browser-Clients auf Gewaltenteilung

- URL: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/work_items/27
- Erstellt: 2026-03-24, geschlossen: 2026-04-01, Labels: Frage, Kommentare: 2

### Beschreibung (Originaltext)

<!--
> Verständnisfrage
-->

### Vertretene Organisation

Privat / SET GmbH

### Frage

Eine Reihe von Fragen zu einem Punkt, der mir nach Lesen der ADRs noch nicht ganz klar ist: Ich gehe davon aus, dass ein Großteil der privaten Nutzer über ein Browser-Frontend auf die Dokumente zugreifen wird. Architekturell hat der Betreiber dieses Frontends potenziell Zugriff auf die unverschlüsselte Kommunikation und die Zugänge der Nutzer, oder? Sollten im Sinne der Gewaltenteilung dann Justiz und Exekutive jeweils ein getrenntes Portal (serverseitiger Teil des Clients) betreiben? Können diese dann auf ein gemeinsames Backend zurückgreifen? Brauche ich als Privatanwender dann getrennte Accounts?

<!-- 
---- Nachfolgenden Text bitte nicht verändern ----
-->

