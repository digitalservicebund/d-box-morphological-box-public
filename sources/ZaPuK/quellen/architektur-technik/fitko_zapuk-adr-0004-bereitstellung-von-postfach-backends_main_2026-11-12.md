<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Zielarchitektur/Architekturentscheidungen/ADR-0004-Bereitstellung-von-Postfach-Backends.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-0004-Bereitstellung-von-Postfach-Backends.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 9a2fd09112); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

> **Status:** ✔️ Operativ entschieden | **Datum:** 15. April 2025

> **Hinweis (Phase 2, Stand Oktober 2026):** Diese Entscheidung wurde in Phase 2 eingeschränkt. Privatpersonen betreiben keine eigenen Backends (ADR-1104, ADR-1114).

# ADR-0004: Bereitstellung von Postfach-Backends durch Dritte

## Kontext und Problembeschreibung

Im Kontext einer offenen oder geschlossen dezentralen Systemarchitektur (vgl. [ADR-0002](ADR-0002-Infrastrukturtopologie.md)) ergibt sich die Frage, inwiefern Postfach-Backends von Dritten betrieben werden können und welche Kriterien diese erfüllen müssen, um in das föderierte System aufgenommen zu werden. Da eine Einschränkung Maßnahmen erfordert, die Aufwand verursachen, ist hierbei von der offensten Möglichkeit auszugehen und jede Einschränkung muss gut begründet sein. Der wichtigste Betrachtungsgegenstand bei der Erwägung einer Einschränkung ist die Fragestellung, inwiefern ein Backend, das sich nicht wie erwartet, also fehlerhaft oder gar bösartig, verhält, Schaden anrichten kann.
Dass eine Einschränkung nicht grundsätzlich nötig ist, ist durch diverse dezentrale öffentliche Kommunikationsökosysteme wie Mastodon, Bluesky, Peertube, Lemmy und Matrix erwiesen. Hier können beliebige Teilnehmer Backends betreiben; fehlerhaftes oder bösartiges Verhalten (das ggf. sogar von der Spezifikation abweicht) schadet grundsätzlich höchstens den [Nutzer:innen] des entsprechenden Backends.
Andere Infrastrukturen, wie der Matrix-basierte TI-Messenger im deutschen öffentlichen Gesundheitswesen, lassen ausschließlich zertifizierte Betreiber als Teilnehmer der Föderation zu. Solche Einschränkungen kommen vor allem dann in Frage, wenn Abweichungen von der vorgesehenen Funktion zu Schaden führen können; zum Beispiel durch das Nicht-Versenden von Abrufbestätigungen, die eine Auswirkung auf die [Zustellfiktion] haben können.
In Abgrenzung von [ADR-0007](./ADR-0007-Implementierung-von-Postfach-Backends.md) (Implementierung von Postfach-Backends) stellt sich hier die vorgelagerte Frage des „wer“ statt des „was“. So wäre es zum Beispiel durchaus möglich, einem zugelassenen Föderationsteilnehmer die Wahl der Implementierung freizustellen und ihn nur für Fehlverhalten haftbar zu machen oder ihn in der Auswahl der Implementierung zu beschränken, ihn dafür aber nicht für Fehlverhalten verantwortlich zu machen.
Die Vor- und Nachteile der Beschränkung der Föderation hängt auch wesentlich von der Frage der zugelassenen Identity Provider (IDPs) ab. Sollen eigene IDPs von angeschlossenen öffentlichen Stellen und privaten Organisationen unterstützt werden, so muss auf eine korrekte [Authentifizierung] seitens der Backends vertraut werden können (was einen freien Betrieb unmöglich machen würde) oder es muss akzeptiert werden, dass unvertrauenswürdige IDPs im System vorhanden sind und Teilnehmer diesen explizit trauen müssten, um eine Kommunikationsverbindung aufzubauen. Sollen IDPs eingeschränkt werden, gibt es dagegen vergleichsweise wenig Rechtfertigung für eine Einschränkung der Betreiber.
Im Kontext der übergreifenden Lösung haben wir uns in [ADR-0013](ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md), [ADR-0014](./ADR-0014-Authentizität-Identität-von-Privatpersonen.md), [ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md) und [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md) für eine Einschränkung der IDPs entschieden.

Unbenommen von den in diesem ADR getroffenen Regelungen ist die Notwendigkeit der Interoperabilität der Backend-Systeme auf Basis der in [ADR-0011](ADR-0011-Kommunikationsschicht.md) und [ADR-0012](ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md) definierten Protokolle für die Kommunikations- und Ende-zu-Ende-Verschlüsselungsschicht.

## Entscheidungstreiber

- Kleinere Kommunen können sich weder die Entwicklung noch den Betrieb eigener Backends leisten. Auch Privatpersonen können oder wollen das i.d.R. nicht leisten.
- Das Einbringen von angreifbaren oder bösartigen Backends in das System kann prinzipiell weitreichendere Sicherheitsprobleme aufbringen (als etwa bei Postfachzugängen). Dieses Risiko wird durch Umsetzung eines Zero-Trust-Modells jedoch vollständig mitigierbar.
- Zero Trust: Als Architekturrichtlinie wurde sich für eine Zero Trust Architektur entschieden. In dieser darf keinem Knoten des Systems vom Grund auf getraut werden; dieses Grundprinzip erleichtert die Akzeptanz fremder Implementierungen im Vergleich zur Grundannahme der Vertrauenswürdigkeit.
- Open Source: Die Bestandteile des Systems sind unter einer offenen Software-Lizenz lizenziert.
- Gestaltungsziel: Hürden für den Anschluss abbauen. Ziel des Systems ist es unter Anderem, eine möglichst leichte Anbindung ohne schwierige Prozesse zu ermöglichen.
- Gestaltungsziel: Förderung von Ende-zu-Ende-Sicherheit

## Betrachtete Optionen
- [Option 1: Jede Teilnehmer:in kann ohne Einschränkungen ein eigenes Backend betreiben](#option-1-jede-teilnehmerin-kann-ein-eigenes-backend-betreiben)
- [Option 2: Jede Teilnehmer:in kann nach vorheriger Registrierung unter Wahrung der Anschlussbedingungen ein eigenes Backend betreiben](#option-2-jede-teilnehmerin-kann-nach-vorheriger-registrierung-unter-wahrung-der-anschlussbedingungen-ein-eigenes-backend-betreiben)
- [Option 3: Nur zertifizierte Teilnehmer:innen können ein Backend betreiben und Teil der Föderation werden](#option-3-nur-zertifizierte-teilnehmerinnen-können-ein-backend-betreiben-und-teil-der-föderation-werden)
- [Option 4: Die verfügbaren Backends und deren Betrieb sind vordefiniert; eine Teilnahme darüber hinaus ist nicht möglich](#option-4-die-verfügbaren-backends-und-deren-betrieb-sind-vordefiniert-eine-teilnahme-darüber-hinaus-ist-nicht-möglich)

## Gegenüberstellung der jeweiligen Optionen

### Option 1: Jede Teilnehmer:in kann ein eigenes Backend betreiben

In dieser Variante genügt das Hochfahren eines Backends, um mit anderen Teilnehmern zu kommunizieren. Die Legitimität einer versendeten Nachricht ergibt sich allein aus der gültigen [Identität] der Nutzer:innen. Der Empfang von Nachrichten auf diesen Backends wäre genauer zu spezifizieren; es könnte sich z.B. in einem Self-Service-Portal angemeldet und das Backend hinterlegt werden oder das Postfach-Backend wird über Online-Anträge als gewünschter Empfangspunkt mitgegeben.

- Gut: Es kann Kontrolle über die beteiligten Systeme im vom Teilnehmer gewünschten Maße ausgeübt werden.
- Gut: Es bedarf keines Mechanismus, um die Föderationsteilnahme einzuschränken.
- Gut: Der eigenständige Beitritt eines Teilnehmers zur Kommunikationsinfrastruktur ist im Self-Service und ohne Kontakt mit einer zentralen Stelle möglich. Dies reduziert prozessuale Abhängigkeiten und verbessert die Flexibilität sowie Skalierbarkeit des Kommunikationsinfrastruktur. Positive Netzwerkeffekte sind zu erwarten. - Schlecht: Sich unerwartet verhaltende Systeme verbleiben langfristig in der Föderation und können Probleme verursachen. Damit muss zu jedem Zeitpunkt gerechnet werden. Dies kann das Laufzeitverhalten der Gesamtinfrastruktur nachteilig Beeinflussen. 
- Schlecht: Die Interoperabilität aller Postfach-Backends in der Infrastruktur ist erforderlich
### Option 2: Jede Teilnehmer:in kann nach vorheriger Registrierung unter Wahrung der Anschlussbedingungen ein eigenes Backend betreiben

Diese Variante könnte z.B. durch eine Registrierung in einem Self-Service-Portal umgesetzt werden, in welchem eine [Identität] mit einem Backend verknüpft wird. Bei Fehlverhalten, insbesondere der Nichteinhaltung von geltenden Teilnahme- oder Anschlussbedingungen, kann die Föderation systemseitig aufgehoben oder eingeschränkt werden. Die Anschlussbedingungen dienen dabei insbesondere der Einhaltung der umzusetzenden Architekturentscheidungen sowie der Spezifikation des erwarteten Laufzeitverhaltens (Service-Level, Quittierungsverhalten (vgl. [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)), etc.). 
- Gut: Es kann Kontrolle über die beteiligten Systeme im vom Teilnehmer gewünschten Maße ausgeübt werden.
- Gut: Das Laufzeitverhalten von durch Dritte betriebenen Postfach-Backends kann mittelbar und in Nachhinein verifiziert werden 
 Gut: Der eigenständige Beitritt eines Teilnehmers zur Kommunikationsinfrastruktur ist im Self-Service und ohne Kontakt mit einer zentralen Stelle möglich. Dies reduziert prozessuale Abhängigkeiten und verbessert die Flexibilität sowie Skalierbarkeit des Kommunikationsinfrastruktur. Positive Netzwerkeffekte sind zu erwarten. 
- Gut: Das System wird betrieblich, also z.B. bzgl. Lastszenarien, leichter handhabbar, da fehlerhafte Systeme ausgeschlossen werden können.
- Schlecht: Ein technischer und administrativer Prozess für Monitoring und Prüfung der Einhaltung der Anschlussbedingungen sowie zum Ausschluss von Teilnehmern muss konzeptioniert, operationalisiert und fortlaufend durchgeführt werden, was Ressourcen bindet und den Anbindungsprozess erheblich verlangsamt. 
### Option 3: Nur zertifizierte Teilnehmer:innen können ein Backend betreiben und Teil der Föderation werden

Bei dieser Variante werden nur ausgewählte Teilnehmer zur Föderation zugelassen; ihre Vertrauenswürdigkeit wird durch einen fachlichen Prozess zertifiziert.

- Gut: Die Komplexität des Systems sinkt durch die Einschränkung und vollständige Bekanntheit aller Betreiber.
- Gut: Das zu erwartende Laufzeitverhalten von durch Dritte betriebenen Postfach-Backends kann unmittelbar und Vorhinein verifiziert werden 
- Schlecht: Der Anschluss an das Gesamtsystem über eigen betriebene Systeme wird erschwert.
- Schlecht: Ein hinreichend gründlicher, ggf. in Teilen manueller Zertifizierungsprozess muss konzeptioniert, operationalisiert und fortlaufend durchgeführt werden, was beträchtliche Ressourcen bindet und den Anbindungsprozess erheblich verlangsamt. 

### Option 4: Die verfügbaren Backends und deren Betrieb sind vordefiniert; eine Teilnahme darüber hinaus ist nicht möglich

Die Möglichkeit einer Aufnahme von durch Dritte betriebenen Postfach-Backends in die Föderation ist grundsätzlich nicht vorgesehen und bedürfte einer Veränderung am System.

- Gut: Die Komplexität des Systems sinkt durch die Einschränkung und vollständige Bekanntheit aller Betreiber und deren genauer Systeme.
- Schlecht: Der Anschluss an das Gesamtsystem über eigen betriebene Systeme wird unmöglich.

## Architekturentscheidung

Gewählte Option: **[Option 2: Jede Teilnehmer:in kann nach vorheriger Registrierung unter Wahrung der Anschlussbedingungen ein eigenes Backend betreiben](#option-2-jede-teilnehmerin-kann-nach-vorheriger-registrierung-unter-wahrung-der-anschlussbedingungen-ein-eigenes-backend-betreiben)**, da eine Übertragung des Vertrauens an Backends (und damit ein Bruch des Zero-Trust-Ansatzes) oder eine Unvollständige Kommunikation des Systems für uns nicht akzeptabel ist, eine Schließung des Systems ohne zwingenden Grund für nachteilig erachten und wir die Möglichkeit des Ausschlusses aus der Föderation im Falle einer missbräuchlichen Nutzung für notwendig erachten.

_Hinweis: Wir nehmen an, dass sich im Backend keine geschäftskritischen Funktionen befinden, also dass durch fehlerhaftes Verhalten eines Backends kein Schaden für andere Teilnehmer des Systems angerichtet werden kann. Insbesondere haben wir den Fall der Abrufbestätigungen betrachtet; verschickt ein System keine solchen Bestätigungen, so gehen wir davon aus, dass dies keine katastrophalen Konsequenzen hat, sondern der oder die [Absender:in] im Zweifelsfall eine Zustellung über einen alternativen Weg (z.B. Post) wählt._

### Auswirkungen

Durch die getroffene Entscheidung ist nicht mehr unbedingt jedes Backend im System im Vorhinein bekannt. Wir müssen zudem bei weiteren Entscheidungen immer davon ausgehen, dass ein Backend sich maximal bösartig verhält und sich nur im Interesse des Betreibers, nicht aber im Interesse des Gesamtsystems verhält. Zudem ist eine Kontrolle von Backend-Implementierungen nicht mehr realistisch.
Die Annahme der maximalen Bösartigkeit eines Backends betrifft auch das Thema Nichtabstreitbarkeit der Übermittlung ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)), da von einer ordnungsgemäßen Quittierung nicht grundsätzlich ausgegangen werden kann.
Wir gewinnen ein hohes Maß an Flexibilität. Teilnehmer können in beliebigem Maße Backends für sich oder andere bereitstellen, z.B.:
- Behördlich bereitgestellte Backends für [Privatperson]en und/oder Organisationen
- Backends, die [Privatperson]en selbst betreiben
- Von Unternehmen bereitgestellte Backends für sich selbst oder Kund:innen
- Behördliche Backends für die eigene [Behörde] und ggf. Nachnutzende, die kein eigenes Backend betreiben möchten

Die Entscheidung zahlt auf ein wesentliches Ziel des Systems ein: Die Vereinfachung der Anbindung. Die offene Systemarchitektur begünstigt diese, da keine fachlichen Verifikations- oder Zertifizierungsprozesse durchlaufen werden müssen. Der einfache Betrieb eines schnittstellenkonformen Systems und eine schnelle Self-Service-Registrierung genügen bereits, um sich an das System anzubinden.

Zudem fördert die Entscheidung Ende-zu-Ende-Sicherheit. Zwar ist die Ende-zu-Ende-Sicherheit in einem offenen System nicht grundsätzlich stärker als in einem Geschlossenen; die Öffnung kann jedoch das öffentliche Vertrauen in eine tatsächlich durchgesetzte und undurchbrochene Verschlüsselung erhöhen, da bei Misstrauen in die Implementierung sowohl Backend- als auch Frontendimplementierung (siehe [ADR-0003](ADR-0003-Bereitstellung-von-Postfach-Zugängen.md)) ausgetauscht werden können.

Zu beachten ist, dass die Möglichkeit des Systemausschlusses zwar administrative und betriebliche Vorteile mitbringt (z.B. Ausschluss eines übernommenen Systems, das Spam versendet, im Interesse dessen Betreibers), jedoch keine grundsätzlich effektive Sicherheitsmaßnahme darstellt. Fehlerhafte Systeme können grundsätzlich immer wieder neu eingebracht werden, fehlerhaftes Verhalten könnte nicht erkannt (oder fehlinterpretiert) werden oder könnte – je nach Konfiguration - erst bei Erreichen einer Schwelle von fehlerhaftem Verhalten ausgeschlossen werden. Es muss deshalb – entsprechend dem Zero-Trust-Ansatz – jederzeit davon ausgegangen werden, dass sich Backends nicht erwartungsgemäß verhalten und es darf niemals angenommen werden, dass Systeme innerhalb der Föderation grundsätzlich vertrauenswürdig sind. Ein echter und grundsätzlicher Ausschluss ist nur durch Sperrung (oder Berechtigungsentzug) der [Identität] möglich.


