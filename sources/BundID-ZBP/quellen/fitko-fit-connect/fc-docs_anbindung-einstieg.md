# Einstieg in die Anbindung

- Quelle: <https://docs.fitko.de/fit-connect/docs/Anbindung/Einstieg>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Einstieg in die Anbindung

   Sie planen, Ihre Organisation oder technische Lösung an FIT-Connect anzubinden? Auf dieser Seite erhalten Sie als Projektverantwortliche:r oder
   organisatorisch beauftragte Person einen kompakten Überblick:
     * Welche Funktionen für Sie relevant sein könnten.
     * Welche Rollen beteiligt sein sollten.
     * Wo Sie Informationen zu den konkreten Umsetzungsschritten finden.

   Diese Dokumentation richtet sich an Projektleiter:innen aus
     * Behörden auf kommunaler, Landes- oder Bundesebene, die ihre Onlinedienste an Verwaltungssysteme (z. B. Fachverfahren) anschließen wollen und/oder
       mit ihren Verwaltungssystemen Nachrichten an das ZBP der BundID versenden möchten.
     * Unternehmen, die für ihre Produkte (Onlinedienste, Verwaltungssysteme) Adapter entwickeln möchte, um ihre Kunden besser in die OZG-Architektur
       einzubinden.
     * IT-Dienstleister, die für ihre Kunden Fachverfahren oder Portallösungen über FIT-Connect verbinden sollen.

Welche Funktionen bietet FIT-Connect?

   FIT-Connect deckt aktuell vier zentrale Funktionen ab:

1. Anträge senden

   Strukturierte Antragsdaten werden z. B. aus einem Onlinedienst über FIT-Connect an eine empfangsberechtigte Stelle (z. B. Verwaltungssystem)
   übermittelt.
   (BUTTON)
   Mehr erfahren ...

Anträge und Berichte einreichen

   Für FIT-Connect wird ein Zustelldienst entwickelt und betrieben, über den Onlinedienste (Antragsplattformen oder Softwareanwendungen) Anträge und
   Berichte an das zuständige Verwaltungssystem senden. Zum Senden nutzen Onlinedienste die Schnittstelle Submission API des Zustelldienstes. Das Senden
   von Anträgen und Berichten ist der Anwendungsfall 3 von FIT-Connect (siehe folgende Grafik: 3. Übermittlung von Einreichungen (Anträge oder
   Berichtsmeldungen)).

   Die folgende Grafik zeigt die vier Anwendungsfälle von FIT-Connect:

   Darstellung der Anwendungsfälle von FIT-Connect

   Eine technische Schritt-für-Schritt-Anleitung ist im Bereich für Entwickler:innen zu finden.

Ereignisprotokoll

   Onlinedienste können sich über den Status einer Einreichung informieren, indem sie das Ereignisprotokoll (Event Log) des Zustelldienstes abrufen. Zum
   Abrufen des Ereignisprotokolls dient die Schnittstelle Submission API des Zustelldienstes.

Onlinedienste anmelden

   Um FIT-Connect zu nutzen, müssen Sie Onlinedienste als API Clients anmelden und konfigurieren (siehe 1. Konfiguration von API Clients in der Grafik
   oben).

Verwaltungssystem ermitteln

   Ist nicht bekannt, welches Verwaltungssystem für eine Einreichung zuständig ist, muss ein Onlinedienst zunächst das zuständige Verwaltungssystem
   ermitteln. Dazu dient der Routingdienst (siehe 2: Abruf von Zuständigkeiten und Parametrisierungs-Informationen in der Grafik oben). Zum Ermitteln des
   Verwaltungssystems steht die Schnittstelle Routing API zur Verfügung.

2. Anträge empfangen

   Ein Verwaltungssystem oder IT-Dienstleister empfängt strukturierte Anträge direkt in einem angebundenen Verfahren.
   (BUTTON)
   Mehr erfahren ...

Anträge und Berichte erhalten

   Bei FIT-Connect durchlaufen Anträge die folgenden Schritte:
     * Ein Onlinedienst (eine Antragsplattform oder eine Softwareanwendung) sendet einen Antrag zum Zustelldienst.
     * Der Zustelldienst informiert das zuständige Verwaltungssystem darüber, dass ein Antrag eingegangen ist.
     * Anschließend ruft das zuständige Verwaltungssystem den Antrag beim Zustelldienst ab.

Schnittstelle Submission API

   Das Verwaltungssystem nutzt zum Empfangen von Anträgen die Schnittstelle Submission API. Diese Schnittstelle dient als Ausgangstor für digitale Anträge
   und Berichte (Einreichungen beim Fachverfahren). Das Abrufen von Einreichungen ist der Anwendungsfall 4 von FIT-Connect (siehe folgende Grafik: 4.
   Abruf von Einreichungen (Anträge oder Berichtsmeldungen)).

   Die folgende Grafik zeigt die vier Anwendungsfälle von FIT-Connect:

   Darstellung der Anwendungsfälle von FIT-Connect

   Eine technische Schritt-für-Schritt-Anleitung ist im Bereich für Entwickler:innen zu finden.

Empfang bestätigen

   Nach dem Abruf einer Einreichung bestätigt das empfangende System den Erhalt der Einreichung, siehe Ereignisprotokoll.

Empfangende Systeme anmelden

   Um FIT-Connect zu nutzen, müssen Sie empfangende Systeme bei FIT-Connect als API Clients anmelden und konfigurieren (siehe 1. Konfiguration von API
   Clients und Zustellpunkten in der Grafik oben).

3. Bidirektionale Kommunikation

   Antworten, Nachforderungen oder Statusinformationen können zurück an den ursprünglichen Onlinedienst oder andere Systeme übermittelt werden.
   (BUTTON)
   Mehr erfahren ...

   Bislang konnten antragsbearbeitende Systeme (zum Beispiel Verwaltungssysteme) Bescheide und Rückfragen zu Anträgen über Bürgerpostfächer, ELSTER,
   E-Mail oder auf dem Postweg an die Antragsteller:innen senden.

   Mithilfe der bidirektionalen Kommunikation (FIT-Connect als Rückkanal) können Systeme der Verwaltung Antworten über FIT-Connect zurück an jenes
   IT-System senden, in dem die Antragstellung erfolgt ist. Das ist insbesondere für IT-Systeme der Wirtschaft interessant. FIT-Connect ermöglicht es
   Unternehmen damit, Verwaltungsleistungen vollständig über maschinenlesbare Schnittstellen ohne Nutzung eines webbasierten Onlinedienstes in Anspruch zu
   nehmen (Machine-to-Machine-Kommunikation). So können IT-Systeme der Wirtschaft direkt und medienbruchfrei mit den Verwaltungssystemen der Verwaltung
   interagieren.

   Die folgende Beschreibung wendet sich an Verantwortliche für die Umsetzung, die beabsichtigen, FIT-Connect als Rückkanal zu nutzen.

Mit FIT-Connect den Dialog zwischen den Systemen führen

   FIT-Connect verwendet den Begriff "Rückkanal" für die Art und Weise, wie antragsbearbeitende Systeme (zum Beispiel Verwaltungssysteme) auf
   Einreichungen (z. B. Bescheide oder Rückfragen) antworten. Verantwortliche für die Umsetzung können im Sinne der Ende-zu-Ende-Digitalisierung, neben
   den oben genannten externen Wegen, auch einen Dialog mit Hilfe der FIT-Connect Infrastruktur initiieren. Der entscheidende Vorteil dabei ist, dass nach
   dem initialen Antrag beliebig viele weitere Übermittlungen maschinenlesbar zwischen dem antragbearbeitenden und antragstellenden System ausgetauscht
   werden können.

   Das folgende Diagramm zeigt den idealisierten Prozess des Dialogs, der von FIT-Connect unterstützt wird. FIT-Connect nimmt Anträge vom Onlinedienst
   oder einer Unternehmenssoftware entgegen und leitet sie zum adressierten Verwaltungssystem weiter. Der Sachbearbeiter sendet aus seinem
   Verwaltungssystem Rückfragen und Bescheide über FIT-Connect an die Unternehmenssoftware (bzw. an den Onlinedienst) zurück. Dieser kann die Antwort
   maschinell lesen, auswerten und verarbeiten. Beide Seiten haben stets die Möglichkeit, weitere Übermittlungen an die andere Partei zu senden. Die
   Klammer dieses Austauschs von Anträgen und Antworten wird durch den Fall (referenziert durch die Case-ID) gebildet.

   FIT-Connect als Rückkanal

Bidirektionale Kommunikation nutzt Prozessstandards

   Für einen maschinellen und automatisierten Dialog ist ein abgestimmter Standard notwendig. Hierfür hat das antragstellende System die Möglichkeit, dem
   antragbearbeitenden System alle unterstützten Prozessstandards (zum Beispiel XBreitband oder ähnliches) in den Metadaten mitzuteilen.

Keine V-PKI-Zertifikate für den Rückkanal

   Der Versand von Anträgen setzt voraus, dass jeder Empfänger ein V-PKI-Zertifikat an seinem Zustellpunkt hinterlegt. Für den Rückkanal ist dies nicht
   erforderlich. Onlinedienste und Unternehmenssoftware, die Anträge an FIT-Connect senden, können mit einem selbst generierten Zertifikat verschlüsselte
   Übermittlungen empfangen und entschlüsseln. Dies kann komfortabel mithilfe der SDKs in Java und .NET implementiert werden.

Anwendungsszenario

   Im folgenden Diagramm wird ein vereinfachtes Szenario aus dem Breitbandumfeld als Beispiel angeführt. Hier wird deutlich, wie das Zusammenspiel der
   bidirektionalen Kommunikation in der Praxis aussehen könnte:

   FIT-Connect Anwendungsszenario Rückkanal

   Nachdem der Antrag des Telekommunikationsunternehmens über FIT-Connect übermittelt wurde, fordert der Wegebaulastträger fehlende Unterlagen ein, um
   anschließend in der Antragsprüfung Änderungen mitzuteilen. Der Antragssteller bekommt unmittelbar das Feedback zu diesen Rückfragen, kann in seiner
   Planungssoftware Änderungen vornehmen und den korrigierten Antrag erneut versenden. Dies geschieht vollumfänglich über die Nutzung einer
   Infrastrukturkomponente. Es entstehen keine Medienbrüche und der Datenaustausch ist maschinenlesbar und damit auch automatisierbar. Über die gleiche
   Komponente wird auch der Bescheid übermittelt.

4. Nachrichten an das zentrale Bürgerpostfach (ZBP)

   Bescheide oder Informationen können sicher an das Zentrale Bürgerpostfach der BundID gesendet werden.
   (BUTTON)
   Mehr erfahren ...

   Das Zentrale Bürgerpostfach (ZBP) als Postfach der BundID ermöglicht es Bund, Ländern und Kommunen sowie den mittelbaren Verwaltungen (zum Beispiel
   Hochschulen und Kammern), Bescheide oder Informationen an Bürger:innen elektronisch zuzustellen. Zudem können Bürger:innen mithilfe von Statusmeldungen
   über den Bearbeitungsstand ihres Antrags informiert werden.

   Den Verwaltungssystemen oder Onlinediensten der Verwaltung bietet das ZBP dafür eine REST-Schnittstelle an, die allerdings ausschließlich innerhalb der
   Netzen des Bundes (NdB) oder innerhalb des Verbindungsnetzes angesprochen werden kann.

Anbindung innerhalb der Netze des Bundes

   ZBP-Adapter

   Wird die Anwendung, die an das ZBP angebunden werden soll, innerhalb der Netze des Bundes oder im Verbindungsnetz betrieben oder hat sie eine
   Verbindung dorthin, kann die Anbindung durch die Implementierung der Schnittstelle des ZBP erfolgen. Dabei kann man auf eine eigene Implementierung
   zurückgreifen. Einfacher ist es jedoch, die dafür bereitgestellte Funktion der FIT-Connect-SDKs zu verwenden.

Anbindung außerhalb der Netze des Bundes

   Ohne Verbindung zu den Netzen des Bundes wird der direkte Zugriff auf die Schnittstelle des ZBP verwehrt.

   Problematik ohne ZBP-Adapter

   Mithilfe der Brückenkopf-Anwendung "ZBP-Adapter" gibt es jedoch eine Möglichkeit, Nachrichten und Statusmeldungen über FIT-Connect an das ZBP zu
   senden:

   Lösung mit ZBP-Adapter

   Normalerweise nutzen Sender wie Onlinedienste und Antragsplattformen der Verwaltung oder Softwareanwendungen in Unternehmen FIT-Connect, um Anträge,
   Berichte oder ähnliches an einen Empfänger - in der Regel das System einer zuständigen Fachbehörde (Verwaltungssystem) - zu übermitteln. Für die
   Anbindung des ZBP steht der Brückenkopf als Empfänger bereit. Er nimmt über FIT-Connect die für das ZBP bestimmten Nachrichten und Statusmeldungen
   entgegen und leitet sie an das ZBP weiter.

Was sollten Sie als Projektleitung wissen?

   Bevor Sie starten, beantworten Sie bitte folgende Fragen:
     * Welche Systeme möchten Sie anbinden?
          + Einen Onlinedienst (z.B. Formularserver)?
          + Ein Fachverfahren / Verwaltungssystem?
          + Oder etwa beides?
     * Welche Funktion(en) von FIT-Connect möchten Sie nutzen?
          + Anträge senden?
          + Anträge empfangen?
          + Rückmeldungen oder ZBP nutzen?
     * Wer übernimmt die fachliche und technische Umsetzung intern oder extern?

Welche Rollen sollten beteiligt sein?

   Damit die Anbindung an FIT-Connect erfolgreich umgesetzt werden kann, empfehlen wir folgende Rollen im Projektteam:
     * Fachlich verantwortliche Person
       Verantwortlichkeit für Leistungen, Zuständigkeiten, Datenmodelle (FIM/XÖV), Zertifikate
     * Technische Ansprechperson
       Entwickler:in oder Architekt:in mit Kenntnis über die anzubindenden Systeme
     * Datenschutz und IT-Sicherheit
       Für Fragen rund um rechtliche Freigaben und technische Sicherheitsanforderungen

Starten Sie mit der Schritt-für-Schritt-Anleitung

   Für die konkrete Anbindung empfehlen wir unsere zentrale Schritt-für-Schritt-Anleitung.

   Dort erfahren Sie im Detail, was zu tun ist, welche Voraussetzungen gelten und wie die Umsetzung Schritt für Schritt erfolgt – insbesondere für die
   Funktionen Anträge senden und Anträge empfangen. Die bidirektionale Kommunikation kann während der Implementierung der Schnittstellen ebenfalls
   mitumgesetzt werden.

   Bitte beachten Sie, dass die Daten in der Dokumentation sich immer auf die Testumgebung beziehen, wenn es nicht anders angegeben ist.

Noch Fragen?

   Das FIT-Connect-Team unterstützt Sie gerne bei der Einordnung, Planung oder Umsetzung: Kontakt aufnehmen
```
