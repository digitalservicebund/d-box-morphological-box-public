# Schritt-für-Schritt-Anleitung

- Quelle: <https://docs.fitko.de/fit-connect/docs/Anbindung/Anleitung>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Schritt-für-Schritt-Anleitung

   Um einen Onlinedienst oder ein Verwaltungssystem an FIT-Connect anzubinden, empfehlen wir den folgenden Stufenplan:

   Projektstrukturplan

Bevor es losgeht ...

Einstieg und Kontakt

   Bevor Sie mit der technischen Umsetzung beginnen, empfehlen wir Ihnen den Einstieg über die folgende Seite: Einstieg in die Anbindung

   Wenn Sie bereits wissen, was Sie wie umsetzen möchten, können Sie hier direkt starten. Andernfalls: Nutzen Sie frühzeitig bei Fragen oder zur weiteren
   Abstimmung unser Kontaktformular.

Projektteam vorbereiten

   Für eine erfolgreiche Umsetzung empfehlen wir, frühzeitig folgende Rollen einzubinden:
     * Fachlich Verantwortliche (z. B. für Zuständigkeiten, FIM/XÖV, Zertifikate)
     * Technische Ansprechpersonen (z. B. Entwickler:innen, Architekt:innen)
     * Datenschutz und IT-Sicherheit (für rechtliche und technische Abstimmungen)

   Technisch Verantwortliche sollten die Entwicklerdokumentation kennen.

Unterschied Onlinedienst und Verwaltungssystem

   Weiterhin sollte klar sein, was sie an FIT-Connect anbinden wollen: Einen Onlinedienst, ein Verwaltungssystem oder beides? Konzeptionell und
   organisatorisch ergibt sich daraus kein wesentlicher Unterschied, für die Entwicklung hingegen schon.

   Was Sie wann beachten müssen, haben wir in den folgenden Tabellen in den Spalten Onlinedienst und Verwaltungssystem hervorgehoben.

Schätzung der Aufwände

   In den folgenden Tabellen gibt die Spalte "Aufwandsschätzung" eine erste Orientierung über den zu erwartenden Aufwand der einzelnen Anbindungsschritte.
   Hierbei handelt es sich nicht um absolute Werte, sondern um Einstufungen zwischen 1 (geringer Aufwand) und 10 (hoher Aufwand).

   Die jeweiligen Aufwandsprognosen basieren auf unseren Erfahrungswerten, die wir im Rahmen zahlreicher Anbindungen gesammelt haben. Da jedes
   Anbindungsprojekt von individuellen fachlichen und technischen Rahmenbedingungen geprägt ist, können Abweichungen bei den Schätzwerten nicht
   ausgeschlossen werden.
   tipp

   Einen allgemeinen Überblick zu diesem Thema finden Sie auf der Seite Aufwandsschätzung. Dort werden u. a. typische Aufwandstreiber, aber auch mögliche
   Lösungen zur Verringerung von Aufwänden thematisiert.

Einplanung von Wartezeiten

   Beachten Sie, dass insbesondere die Beantragung von Zertifikaten und die Abstimmung von vertraglichen Themen unvorhergesehene Wartezeiten verursachen
   können. Kalkulieren Sie dies bei Ihrer Planung ein und agieren Sie frühzeitig.

   Anbindungsschritte, die von solchen Verzögerungen betroffen sein können, enthalten in den folgenden Tabellen einen Hinweis, der mit ⚠️ gekennzeichnet
   ist.

1 Konzeptionelle Vorüberlegungen

   Die konzeptionellen Vorüberlegungen sollten möglichst vor Beginn der Implementierung abgeschlossen sein. Wir empfehlen grundsätzlich, die fachlich
   Verantwortlichen durch technisch versiertes Personal zu unterstützen, da viele der Fragen ineinandergreifen.

1.1 Organisatorische und fachliche Vorüberlegungen

   Zielgruppe: Fachseite Behörde Unternehmen

   Im ersten Schritt sollten durch die Behörde fachliche Fragestellungen rund um die Leistung beantwortet werden, die entweder durch den Onlinedienst
   abgebildet oder durch das Verwaltungssystem verarbeitet werden.

   Ziel der Vorüberlegungen

   Die Vorüberlegungen helfen dabei, die Voraussetzungen für eine digitale Antragstellung über FIT-Connect strukturiert zu erfassen. Sie ermöglichen es,
   frühzeitig relevante Informationen zu klären, z. B. welche Verwaltungsleistung digitalisiert werden soll, welches System dafür genutzt wird und wie
   Daten technisch übertragen werden können. Diese Übersicht unterstützt Sie dabei, alle beteiligten Systeme, Zuständigkeiten und technischen
   Anforderungen im Blick zu behalten.
   Nr. Aufgabe Online-
   dienst Behörde Onlinedienst/
   Unternehmen Verwaltungs-
   system Behörde Verwaltungssystem/
   Unternehmen Aufwands-
   schätzung
   1 Bestimmen Sie die Verwaltungsleistung, die Bürger:innen online beantragen sollen. ✅ ✅ ✅ ✅ 1/10
   2 Ermitteln Sie, welches Verwaltungssystem für die Abwicklung dieser Leistung genutzt werden soll. ❌ ❌ ✅ ✅ 2/10
   3 Ermitteln Sie den Leistungsschlüssel (LeiKa-Schlüssel) der ausgewählten Leistung. ✅ ✅ ✅ ✅ 1/10
   4 Ermitteln Sie, wie das Verwaltungssystem auf Anträge antwortet und auf welchem Weg (ZBP, FIT-Connect, ELSTER, FINK, E-Mail oder Brief) Bescheide
   kommen. ❌ ❌ ✅ ✅ 2/10
   5 Identifizieren Sie die zuständige öffentliche Stelle, bei der die Leistung online beantragt werden soll, und prüfen Sie die Online-Erreichbarkeit. ✅
   ✅ ❌ ❌ 1/10
   6 Bestimmen Sie die zuständige Behörde anhand des ARS. ❌ ❌ ✅ ✅ 1/10
   7 Geben Sie an, welches Datenformat (FIM- oder XÖV-Schema) das Verwaltungssystem für die Übermittlung von Fachdaten verwendet. ✅ ✅ ✅ ✅ 2/10
   8 Prüfen Sie, ob für diese Leistung bereits bestehende Onlinedienste zur Nachnutzung verfügbar sind. ✅ ✅ ❌ ❌ 2/10

1.2 Technische Vorüberlegungen

   Zielgruppe: Entwicklung Behörde Unternehmen

   In diesem Schritt klären die technischen Ansprechpersonen für die Verwaltungssysteme bzw. Onlinedienste die erforderlichen Implementierungen an ihrem
   System.
   Nr. Aufgabe Online-
   dienst Behörde Onlinedienst/
   Unternehmen Verwaltungs-
   system Behörde Verwaltungssystem/
   Unternehmen Aufwands-
   schätzung
   Unterstützung von RESTful APIs
   1 Prüfen Sie, ob das System RESTful APIs unterstützt. ✅ ✅ ✅ ✅ 1/10
   2 Integration der API-Endpunkte gemäß FIT-Connect-Dokumentation. ✅ ✅ ✅ ✅ 2/10
   Datenformate
   3 Sicherstellen, dass standardisierte Datenformate (z. B. JSON, XML) unterstützt werden. ✅ ✅ ✅ ✅ 1/10
   4 Auswahl und technische Umsetzung des passenden Fachdatenschemas (FIM- oder XÖV-Schema). ✅ ✅ ✅ ✅ 4/10
   5 Abstimmung und Implementierung der richtigen Version des Metadatenschemas mit dem Fachverfahren. ✅ ✅ ❌ ❌ 2/10
   Webdienstintegration
   6 Integration der notwendigen Webdienste (Login mit BundID / MUK) in den Onlinedienst ermöglichen. ✅ ✅ ❌ ❌ 4/10
   7 Technische Umsetzung des Rückkanals für Bescheide (z. B. FIT-Connect, ELSTER, FINK, E-Mail, Brief, ZBP). ✅ ✅ ✅ ✅ 1-5/10
   (je nach Kanal
   bzw. Anzahl der
   unterstützten Kanäle)
   8 Entscheidung: Callback- oder Polling-Unterstützung? ✅ ✅ ✅ ✅ 1/10
   Fehlerbehandlung
   9 Umsetzung der Fehlerbehandlung nach den Vorgaben der FIT-Connect-Dokumentation, einschließlich der verständlichen Information des Nutzers. ✅ ✅ ✅ ✅
   3/10
   10 Implementierung technischer Mechanismen zur robusten Fehlerbehandlung im System. ✅ ✅ ✅ ✅ 3/10

2 Organisatorische und rechtliche Vorbereitung

   Zielgruppe: Fachseite Behörde Unternehmen

   Für die späteren Schritte 4 (Regressionstests) und 5 (Produktivsetzung) wird ein Zugang zur Stage- und Produktionsumgebung benötigt. Dafür sind einige
   Vorbereitungen notwendig. Unter anderem müssen Sie bei der erstmaligen Anmeldung den Nutzungsbedingungen und dem AVV zustimmen. Hierzu gibt es häufig
   Klärungsbedarf.

   Planen Sie daher für diese Arbeitsschritte mehrere Wochen ein, da interne Abstimmungen, Rückfragen und auch die Anforderung von Zertifikaten häufig
   länger dauern als ursprünglich angenommen.

   Die organisatorische und rechtliche Vorbereitung kann parallel zur Implementierung (Schritt 3) von FIT-Connect erfolgen, da die Entwicklung und
   Implementierung von FIT-Connect auf der Test-Umgebung stattfindet. Um diese nutzen zu können, sind keine Vorbereitungen notwendig.
   Nr. Aufgabe Online-
   dienst Behörde Onlinedienst/
   Unternehmen Verwaltungs-
   system Behörde Verwaltungssystem/
   Unternehmen Aufwands-
   schätzung
   Anlegen eines Self-Service-Portal-Kontos auf der Stage- und Produktionsumgebung
   1 Beschaffen Sie sich ein Mein-Unternehmenskonto-Zertifikat für den Login auf Stage- und Produmgebung. Die Schritte dafür sind unter Login mit Mein
   Unternehmenskonto beschrieben.
   ⚠️ Beantragen Sie Zertifikate so frühzeitig wie möglich! ✅ ✅ ✅ ✅ 3/10
   2 Anmeldung auf der Stage- und Produktionsumgebung. Den Link finden Sie auf unserer Webseite oder im Self-Service-Portal. ✅ ✅ ✅ ✅ 1/10
   Rechtliche Themen
   3 Beachten Sie, dass Sie bei der Ersteinwahl auswählen, ob Sie ein Verwaltungssystem oder Onlinedienst sind, sodass Ihnen die richtigen
   Vertragsunterlagen zur Unterzeichnung dargestellt werden. Falls Sie diese nicht unterzeichnen können, wenden Sie sich bitte an das
   Anbindungsmanagement. Stimmen Sie den Nutzungsbedingungen zu. Den Nutzungsbedingungen stimmen Sie zu, während Sie sich das erste Mal auf der Stage-
   bzw. Produktionsumgebung anmelden. Sie können und sollten diese aber bereits vorher geprüft haben. Sie finden diese unter Nutzungsbedingungen. Die
   Adressaten sind die rechtlichen Vertreter:innen des Onlinedienstes, die FIT-Connect für ihren Onlinedienst nutzen wollen. Wenn Sie diesen einmal
   zugestimmt haben, gelten diese für alle Anbindungen mit FIT-Connect.
   ⚠️ Planen Sie ein, dass vertragliche Abstimmungen unter Umständen dauern können! ✅ ✅ ✅ ✅ 2/10
   4 Zusätzlich muss der Auftragsverarbeitungsvertrag (AVV) abgestimmt und unterzeichnet werden. Hier gilt (wie bei den Nutzungsbedingungen), dass diese
   erst ab der Stage- bzw. Produktionsumgebung erforderlich sind.
   ⚠️ Planen Sie ein, dass vertragliche Abstimmungen unter Umständen dauern können! ✅ ✅ ❌ ❌ 2/10
   Zertifikate
   5 Beschaffung und Bestätigung des V-PKI-Zertifikats über die zuständige Landes-Registrierungsstelle. Wo und wie Sie V-PKI-Zertifikate beschaffen
   können, erfahren Sie unter Zertifikate beantragen.
   ⚠️ Beantragen Sie Zertifikate so frühzeitig wie möglich! ✅ ✅ ✅ ✅ 4/10
   6 Umwandlung des V-PKI-Zertifikats mit Hilfe von Python (JWK-Tool). Wie das geht, erfahren Sie unter Zertifikate umwandeln. ✅ ✅ ✅ ✅ 5/10

3 Implementierung von FIT-Connect

   Parallel zu den organisatorischen Vorbereitungen sollte mit der Entwicklung und Implementierung von FIT-Connect begonnen werden.

3.1 Implementierung von Schnittstellen

   Zielgruppe: Entwicklung Behörde Unternehmen

   Im ersten Schritt wird der Onlinedienst bzw. das Verwaltungssystem an die Testumgebung von FIT-Connect angebunden. Der Schwerpunkt liegt auf dem Test
   einzelner Funktionen des Onlinedienstes bzw. Verwaltungssystems. Das Testsystem von FIT-Connect ist öffentlich verfügbar. Eine technische
   Schritt-für-Schritt-Anleitung für die Anbindung an FIT-Connect finden Sie in der Entwicklerdokumentation.

   Um die Anbindung Ihrer Fachanwendung an FIT-Connect auf technisch effiziente Weise umzusetzen, stehen zeitgemäße Software Development Kits (SDKs) und
   Programmierschnittstellen (APIs) bereit. Diese unterstützen Ihre Entwickler:innen dabei, Anträge medienbruchfrei zu übertragen, Zustellpunkte
   automatisiert zu verwalten oder Rückmeldungen sicher zu empfangen.

   Die SDKs enthalten wiederverwendbare Code-Bausteine und Best Practices für verschiedene Programmiersprachen (Java, .NET und JavaScript). Über die REST
   APIs können Ihre Systeme direkt mit FIT-Connect kommunizieren – zum Beispiel für das Versenden von Anträgen oder den Abruf von Rückkanalnachrichten.

   Bitte geben Sie diesen Hinweis an Ihre technischen Ansprechpartner:innen weiter, damit diese prüfen können, welches SDK oder welche API für Ihre
   Anwendung am besten geeignet ist.
   Nr. Aufgabe Online-
   dienst Behörde Onlinedienst/
   Unternehmen Verwaltungs-
   system Behörde Verwaltungssystem/
   Unternehmen Aufwands-
   schätzung
   1 Datenbereitstellung: Der Onlinedienst übermittelt die von ihm erfassten Daten an FIT-Connect. ✅ ✅ ❌ ❌ 2/10
   2 Datenabruf: Das Verwaltungssystem ruft vom Onlinedienst bereitgestellte Daten von FIT-Connect ab. ❌ ❌ ✅ ✅ 4/10
   3 Ereignisprotokollierung und Fehlerbehandlung durch das Verwaltungssystem: Das Verwaltungssystem protokolliert den Erfolg / Misserfolg der technischen
   Verarbeitung durch das dafür vorgesehene Event in FIT-Connect. Fehler, die dazu führen können, dass kein Event gesetzt wird müssen besonders behandelt
   werden ❌ ❌ ✅ ✅ 1/10
   4 Prüfung des Ereignisprotokolls und Fehlerbehandlung durch den Onlinedienst: Der Onlinedienst prüft den Status von gesendeten Einreichungen und
   behandelt mögliche Fehler besonders im Falle eines vom Verwaltungssystem gesetzten Reject-Events, das den Misserfolg der technischen Verarbeitung
   signalisiert. Dabei sollte einerseits der Antragsteller informiert werden und ggf. über Monitoring/Alerting ein Incidentmanagement integriert werden. ✅
   ✅ ❌ ❌ 5/10
   5 Konfiguration des Zustellpunktes (optional): Das Verwaltungssystem konfiguriert den von ihm verwendeten FIT-Connect-Zustellpunkt selbst per API. ❌ ❌
   ✅ ✅ 1/10
   6 Bidirektionale Kommunikation (optional): Verwaltungssystem sendet Antworten. ❌ ❌ ✅ ✅ 2/10
   7 Bidirektionale Kommunikation (optional): Onlinedienst ruft Antworten ab. ✅ ✅ ❌ ❌ 3/10

3.2 Konfigurieren der Testumgebung auf dem Self-Service-Portal

   Zielgruppe: Entwicklung Behörde Unternehmen

   Damit die Schnittstelle getestet werden kann, bedarf es eines Testaufbaus, der den Versand vom Onlinedienst in das Verwaltungssystem simuliert. Wenn
   Sie nur einen Onlinedienst bzw. Verwaltungssystem aninden wollen, müssen Sie den fehlenden Teil, beispielsweise mit unseren SDKs, simulieren oder
   direkt mit der Gegenstelle testen.
   Nr. Aufgabe Online-
   dienst Behörde Onlinedienst/
   Unternehmen Verwaltungs-
   system Behörde Verwaltungssystem/
   Unternehmen Aufwands-
   schätzung
   Anbindung an das Testsystem
   1 Aufbau einer Test-Umgebung für die FIT-Connect-Anbindung. ✅ ✅ ✅ ✅ 4/10
   2 Testaufbau für einen Onlinedienst zum Versenden von Einreichungen an einen Zustellpunkt bereitstellen. ❌ ❌ ✅ ✅ 5/10
   3 Testaufbau für ein Verwaltungssystem, das die Einreichungen vom Zustellpunkt abholt, bereitstellen. ✅ ✅ ❌ ❌ 4/10
   4 Anmeldung in der Testumgebung des FIT-Connect Self-Service-Portals. ✅ ✅ ✅ ✅ 1/10
   5 Zertifikatserstellung für Testzwecke. Es gibt drei mögliche Tools. Informationen und Downloads dazu finden Sie unter Zertifikate umwandeln. Für das
   Testsystem wird noch kein Verwaltungs-PKI benötigt. ❌ ❌ ✅ ✅ 2/10
   6 Anlegen eines Zugangs für den Onlinedienst. Eine technische Schritt-für-Schritt-Anleitung finden Sie unter Registrierung und Zugangsdaten. ✅ ✅ ✅ ✅
   1/10
   7 Anlegen eines Zustellpunktes für das Verwaltungssystem. Eine technische Schritt-für-Schritt-Anleitung finden Sie unter Zustellpunkt erstellen. ✅ ✅ ✅
   ✅ 2/10

3.3 Testen von FIT-Connect auf der Testumgebung

   Zielgruppe: Entwicklung Behörde Unternehmen
   Nr. Aufgabe Online-
   dienst Behörde Onlinedienst/
   Unternehmen Verwaltungs-
   system Behörde Verwaltungssystem/
   Unternehmen Aufwands-
   schätzung
   Integrationstests auf dem Testsystem
   1 Klärung des Testablaufs mit den beteiligten Dienstleistern. ✅ ✅ ✅ ✅ 3/10
   2 Testen eines erfolgreichen Durchstichs: Übermittlung eines Antrags vom Onlinedienst über FIT-Connect an ein Verwaltungssystem. Weitere Informationen
   finden Sie unter Verwaltungssysteme. ✅ ✅ ✅ ✅ 1/10
   3 Durchführung einer Sicherheitsüberprüfung. Der Sicherheitsverantwortliche oder das IT-Security-Team muss prüfen, ob FIT-Connect den internen
   Sicherheitsanforderungen genügt. Bei Rückfragen melden Sie sich gerne beim Anbindungsmanagement. ✅ ✅ ✅ ✅ Situations-
   abhängig
   4 Externe Sicherheitsprüfung für die Schnittstellen durchführen. ✅ ✅ ✅ ✅ Situations-
   abhängig

4 Regressionstests für die eigene Abnahme

4.1 Konfiguration im Self-Service-Portal der Stage- und Produktionsumgebung

   Zielgruppe: Entwicklung Behörde Unternehmen

   Es ist zwar nicht verpflichtend, aber wir empfehlen Regressionstests für die eigene Abnahme der entwickelten Schnittstellen. Wir stellen dafür eine zur
   Produktions-Umgebung deckungsgleiche Stage-Umgebung zur Verfügung.

   Voraussetzung für diesen Schritt ist die Umsetzung der Aufgaben aus Abschnitt 2 "Organisatorische und rechtliche Vorbereitung". Für diesen Schritt
   sollten nun das Zertifikat von Mein Unternehmenskonto und das V-PKI-Zertifikat zur Ver- und Entschlüsselung der übertragenen Daten vorliegen.
   Nr. Aufgabe Online-
   dienst Behörde Onlinedienst/
   Unternehmen Verwaltungs-
   system Behörde Verwaltungssystem/
   Unternehmen Aufwands-
   schätzung
   1 Einwahl in die Stage-Umgebung und Prod-Umgebung mit MUK Zertifikat. ✅ ✅ ✅ ✅ 1/10
   2 Stage-Umgebung konfigurieren; analog wie in der Test-Umgebung, siehe Abschnitt 3.2. Beachten Sie, dass Sie bei der Ersteinwahl auswählen, ob Sie ein
   Verwaltungssystem oder Onlinedienst sind, sodass Ihnen die richtigen Vertragsunterlagen zur Unterzeichnung dargestellt werden. Falls Sie diese nicht
   unterzeichnen können, wenden Sie sich bitte an das Anbindungsmanagement; dieses kann Ihnen die Vertragsunterlagen per E-Mail zusenden. ✅ ✅ ✅ ✅ 1/10
   3 Prod-Umgebung konfigurieren; analog wie in der Test-Umgebung, siehe Abschnitt 3.2. Beachten Sie, dass Sie bei der Ersteinwahl auswählen, ob Sie ein
   Verwaltungssystem oder Onlinedienst sind, sodass Ihnen die richtigen Vertragsunterlagen zur Unterzeichnung dargestellt werden. Falls Sie diese nicht
   unterzeichnen können, wenden Sie sich bitte an das Anbindungsmanagement; dieses kann Ihnen die Vertragsunterlagen per E-Mail zusenden. ✅ ✅ ✅ ✅ 1/10
   4 Absprache mit Verwaltungssystem und Behörde über deren Anbindungsfortschritt und weitere Testungen. Dies müssen Sie nur durchführen, wenn die Abnahme
   mit dem Onlinedienst oder Verwaltungssystem eines anderen Dienstleisters erfolgt. ✅ ✅ ✅ ✅ 3/10

4.2 Regressionstests auf der Stage-Umgebung

   Zielgruppe: Entwicklung Behörde Unternehmen

   IT-Dienstleister führen hier nochmals alle Tests aus, um zu überprüfen, ob das Verwaltungssystem und der Onlinedienst auch gegen das Referenzsystem von
   FIT-Connect fehlerfrei arbeiten.
   Nr. Aufgabe Online-
   dienst Behörde Onlinedienst/
   Unternehmen Verwaltungs-
   system Behörde Verwaltungssystem/
   Unternehmen Aufwands-
   schätzung
   1 Durchführung aller Tests durch IT-Dienstleister, um fehlerfreies Arbeiten gegen die Stage-Umgebung von FIT-Connect zu gewährleisten. ✅ ✅ ✅ ✅ 3/10

5 Produktivsetzung

   Zielgruppe: Entwicklung Behörde Unternehmen

   Der Onlinedienst ist nun mit dem Produktivsystem von FIT-Connect verbunden und kann Anträge von Bürger:innen entgegennehmen und diese an das
   Verwaltungssystem weiterleiten. Damit ist die Anbindung abgeschlossen.
   Nr. Aufgabe Online-
   dienst Behörde Onlinedienst/
   Unternehmen Verwaltungs-
   system Behörde Verwaltungssystem/
   Unternehmen Aufwands-
   schätzung
   1 Bestätigung der Bereitschaft für die Nutzung in der Produktivumgebung. Dies müssen Sie individuell in Ihrer Organisation (Behörde bzw. Unternehmen)
   bzw. mit den Herstellern und IT-Dienstleistern vereinbaren. ✅ ✅ ✅ ✅ 1/10
   2 Eintragung in das FIM-Redaktionssystem. Damit das Verwaltungssystem der Behörde über die FIT-Connect Routingarchitektur auffindbar ist, müssen die
   von FIT-Connect bereitgestellten signierten Adressierungsinformationen zunächst im jeweiligen FIM Bundes- oder Landesredaktionssystem hinterlegt
   werden. Weitere Informationen dazu finden Sie auf der Seite Antragsrouting konfigurieren. ✅ ✅ ✅ ✅ 3/10

6 Überführung in den Regelbetrieb

   Zielgruppe: Entwicklung Behörde Unternehmen
   Nr. Aufgabe Online-
   dienst Behörde Onlinedienst/
   Unternehmen Verwaltungs-
   system Behörde Verwaltungssystem/
   Unternehmen Aufwands-
   schätzung
   1 Abschluss einer Wartungs- bzw. Pflegevereinbarung für die Schnittstellen. Wir empfehlen den Abschluss einer Wartungsvereinbarung mit Ihrem
   IT-Dienstleister. ✅ ✅ ✅ ✅ 2/10
   2 Implementierung eines klar definierten Update-Mechanismus für Schnittstellenänderungen. Updates erhalten Sie grundsätzlich über Ihre hinterlegte
   E-Mail-Adresse im Self-Service-Portal-Account. Weiterhin empfehlen wir Ihnen die Abkündigungen und das Changelog regelmäßig einzusehen. ✅ ✅ ✅ ✅ 2/10
```
