# ELSTER-Transfer – Häufig gestellte Fragen

Quelle: https://www.elster.de/elsterweb/infoseite/elstertransfer_hilfe_faq
Abgerufen: 2026-11-18

```text

Häufig gestellte Fragen

Automatischer Konfigurationstest

   Informationen zum automatischen Konfigurationstest

   Die ELSTER-Transfer-Anwendung verfügt über die Möglichkeit, die Verbindung zu den
   ELSTER-Servern und andere Systemvoraussetzungen zu prüfen und den Nutzern die
   Testergebnisse darzustellen.

   Sie erreichen den Konfigurationstest, indem Sie im Menü "Einstellungen" wählen. In den
   Einstellungen erweitert sich das Menü um einen Untermenüpunkt "Konfigurationstest". Nachdem
   Sie den Menüpunkt "Konfigurationstest" ausgewählt haben, erscheint ein Hinweis, dass der
   automatische Test durchgeführt wird. Nach dem Test werden Sie automatisch zu den
   Ergebnissen weitergeleitet.

   Erstellen von Supportinformationen (Support-ZIP)

   In allen technischen Supportfällen wird der ELSTER-Support weitergehende Informationen zu
   Ihrer ELSTER-Transfer-Installation und Ihren Berechtigungen abfragen.

   Sie haben die Möglichkeit, auf der Ergebnisseite des Konfigurationstest ein
   Support-ZIP-Paket zu erzeugen und herunterzuladen. In diesem ZIP-Paket sind notwendige
   Informationen enthalten, die dem ELSTER-Support bei der Bearbeitung Ihres Supportfalls
   helfen.

   Enthalten sind in dem Support-ZIP:
     * Logfiles
     * Systeminformationen (Betriebssystem)
     * Die Einstellungen der ETR-Anwendung
     * Die Ergebnisse des Konfigurationstest

   In dem ZIP-Paket sind keine Passwörter oder versandte und empfangene Daten/Dokumente
   enthalten.

Fehlersituationen

   In diesem Abschnitt werden mögliche Fehlersituationen und Wege zur Lösung beschrieben.

   Weiterführende Informationen zu erforderlichen Verbindungen finden Sie in den Handbüchern
   im Downloadbereich.
   Hinweise im Bereich "Verbindungen"

   Hinweise im Bereich "Verbindungen" treten meist auf, wenn die ELSTER-Transfer-Anwendung
   nicht mit unseren Eingangs- oder Downloadservern kommunizieren kann.

Mögliche Ursachen:

     * Proxy: Bitte prüfen Sie, ob der Proxy richtig in den Einstellungen konfiguriert ist
       und/oder starten Sie den Proxy gegebenenfalls.
     * Firewall: Bitte prüfen Sie, ob die ETR-Anwendung die ELSTER-Server (Adressen siehe
       Installationshandbuch) erreichen kann. Möglicherweise wird der Internetverkehr der
       Anwendung blockiert.
     * Netze des Bundes (NdB): Anwendungen, die im NdB mit den ELSTER-Servern kommunizieren
       haben keinen Zugriff auf den Update-Server.

   Bitte beenden Sie bei Anpassungen an Proxy oder Firewall den ETR-Dienst und starten Sie die
   Anwendung erneut.

   Wenn die Konfigurationstests im Bereich "Verbindung" nicht erfolgreich durchlaufen wurden,
   dann handelt es sich vermutlich um ein Infrastrukturproblem. Bitte wenden Sie sich in
   diesem Fall an Ihre EDV-Abteilung und danach ggf. an den ELSTER-Support. Bei
   Infrastrukturproblemen kann der Support leider nur eingeschränkt unterstützen.

   Hinweise im Bereich "Zertifikatsdetails"

   Hinweise im Bereich "Zertifikatsdetails" treten meist auf, wenn das verwendete
   ELSTER-Zertifikat abgelaufen ist oder wenn keine Berechtigungen zum Versand/Empfang von
   Daten via ELSTER-Transfer genehmigt wurden.

Abgelaufenes Zertifikat:

   ELSTER-Zertifikate verfügen über eine Gültigskeitsdauer. Ist das Zertifikat abgelaufen so
   kann es nicht weiter verwendet werden.
   Um Ihren Zugang zu erneuern nutzen Sie bitte die Zugangserneuerung von Mein ELSTER.

Fehlendes Zertifikat, Zertifikat nicht entschlüsselbar:

   Bitte prüfen Sie, ob Sie die Systemeinrichtung abgeschlossen und ein gültiges
   ELSTER-Zertfikat in Ihre ELSTER-Transfer-Anwendung importiert haben. Bitte prüfen Sie die
   Verwendung des richtigen Zertifikatspassworts.

Zertifikat besitzt keine Berechtigungen:

   Um Dateien mit ELSTER-Transfer zu übermitteln oder zu empfangen benötigt Ihr Zertifikat
   dementsprechende Berechtigungen. Diese können Sie in Mein ELSTER beantragen und einsehen.
   Weitere Informationen: Erlangen von Berechtigungen zur Übermittlung/Abholung von
   ELSTER-Transfer-Verfahren.

   Hinweise im Bereich "Weiteres"

   Hinweise im Bereich "Weiteres" zeigen an, ob Sie eine aktuelle Version von ELSTER-Transfer
   benutzen und ob die ETR-Anwendung konfigurierte Verzeichnisse lesen und schreiben kann.

   Wir empfehlen regelmäßige Updates nicht nur aus funktionalen Gründen, sondern auch aus
   Sicherheitsaspekten.

   Hinweis: Anwendungen, die im NdB mit den ELSTER-Servern kommunizieren haben keinen Zugriff
   auf den Update-Server. Ein Hinweis wird permanent angezeigt. Bitte prüfen Sie die
   ELSTER-Webseiten manuell, ob neue Updates vorliegen.

Benötigte Verzeichnisse:

   Die ELSTER-Transfer-Anwendung muss das konfigurierte Eingangs-/Ausgangs-/Import- und
   Datenverzeichnis lesen und schreiben können, um einwandfrei zu funktionieren.

   Bitte legen Sie ggf. die notwendigen Verzeichnisse an und setzen die erforderlichen Rechte
   für den Nutzer, unter dem die ETR-Anwendung ausgeführt wird.

   Weiterführende Informationen zu erforderlichen Verzeichnissen finden Sie in den Handbüchern
   im Downloadbereich.

Fehlermeldungen in der Anwendung oder in den Logfiles

   Application failed to start: The Tomcat connector configured to listen on port 8081 failed
   to start. The port may already be in use or the connector may be misconfigured.

   Finden sich im Log folgende Meldungen:

   APPLICATION FAILED TO START
   Description: The Tomcat connector configured to listen on port 8081 failed to start. The
   port may already be in use or the connector may be misconfigured.

   ....wird der Port 8081 vermutlich durch einen anderen Dienst auf dem Rechner des Nutzers
   belegt (z. B. Virenscanner).

   Abhilfe kann schaffen, in der ETR-Grundeinstellung einen Alternativport zu konfigurieren
   (z. B. 8082). Informationen hierzu finden Sie im Installationshandbuch.
   Caused by: javax.xml.stream.XMLStreamException: ParseError at [row,col]:[xxx,xxx] Message:
   Read timed out
   Bitte wenden Sie sich bei dieser Meldung an unseren Support.
```
