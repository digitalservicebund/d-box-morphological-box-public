# Was ist ELSTER-Transfer (ETR)?

Quelle: https://www.elster.de/elsterweb/infoseite/elstertransfer_hilfe_einfuehrung
Abgerufen: 2026-11-18

```text

Was ist ELSTER-Transfer (ETR)?

   Um es speziellen Organisationen wie Kommunen, Kammern, Universitäten, Landschaftsverbände,
   Banken, Behörden, Anstalten oder Service-Providern zu ermöglichen sensible Daten
   elektronisch mit den Rechenzentren der Finanzverwaltung auszutauschen sowie Dokumente in
   das Postfach von Anwendern (Personen/ Steuerpflichtige/ Unternehmen) einzustellen, wird das
   Verfahren ELSTER-Transfer (ETR) angeboten.

   ELSTER-Transfer ist eine Anwendung der Finanzverwaltung, welche üblicherweise ständig auf
   einem Server oder Rechner ausgeführt wird und mehrere Schnittstellen zur Kommunikation
   zwischen dem Postfach bei der Finanzverwaltung und der eingesetzten Software bei den
   Organisationen anbietet.

Zusammenfassung

   Bei ELSTER-Transfer handelt es sich, vereinfacht gesagt, um einen digitalen Postboten, der
   Steuerinformationen zur Verfügung stellt oder annimmt - automatisiert, standardisiert und
   sicher.

   ELSTER-Transfer dient der Datenübermittlung von einer Organisation zur Finanzverwaltung
   und umgekehrt. ELSTER-Transfer dient auch der Bereitstellung von Mitteilungen und Bescheide
   für Anwender.
   Zusammenfassung - Grafik Sprechblase

Warum die Anbindung an ELSTER-Transfer?

   Was sind die Beweggründe einer Organisation, die bestehenden analogen Prozesse aufzugeben
   und stattdessen auf die elektronische Übermittlung von Daten und/oder Dokumenten
   umzustellen?

   Zuallererst ist hier das Onlinezugangsgesetz zu nennen: Das Onlinezugangsgesetz verlangt,
   einen Onlinezugang zu allen Verwaltungsleistungen bereitzustellen. Das beinhaltet bspw.
   auch Bescheide für Anwender oder Unternehmen.

   Mit der Schnittstelle, die im April 2021 (Version 2.0.0) für die Bereitstellung geschaffen
   wurde können Dokumente in das digitale Postfach der Steuerpflichtigen eingestellt werden.
   Auf diese Weise kann ELSTER-Transfer als virtueller Postbote für Mitteilungen und
   rechtsverbindliche Bescheide dienen, die vollständig digital sind.

   Neben der Bereitstellung von Dokumenten können vielfältige Daten mit der Finanzverwaltung
   ausgetauscht werden, so dass viele Prozesse medienbruchfrei ablaufen können. Für die
   Organisationen bietet sich die Chance, Prozesse und Abläufe zu verbessern und effizienter
   zu gestalten und so personelle Kapazitäten freizumachen, die an anderer Stelle dringend
   benötigt werden. Für einige Kommunen bedeutet ELSTER-Transfer den Einstieg in die
   digitale Verwaltung. Für andere ist es ein weiterer Baustein hin zu einem vollständig
   digitalisierten Verwaltungshandeln.

   Eine Auswahl der verfügbaren Verfahren finden Sie auf der Seite Unterstützte
   ELSTER-Transfer-Verfahren der Partner .

Datenaustausch mit der Finanzverwaltung

   Zielgruppe sind Organisationen, welche Daten mit den Rechenzentren der Finanzverwaltung
   austauschen möchten. Überwiegend handelt es sich hierbei um Kommunen.
   Datenaustausch mit der Finanzverwaltung

   Dateien von der Finanzverwaltung werden als Rohdaten zur Verfügung gestellt. Diese sind
   nicht menschenlesbar und müssen daher zwingend mit einer externen Software aufbereitet und
   weiterverarbeitet werden. Wenn Sie Daten mit den Rechenzentren der Finanzverwaltung
   ("Partner") austauschen möchten, müssen Sie (gegebenenfalls in Zusammenarbeit mit Ihrem
   Softwareanbieter) prüfen, ob die elektronisch bereitgestellten Daten verarbeitet, bzw.
   erstellt werden können.

Wichtiger Hinweis

   Die Rechenzentren stellen keine Bescheide (oder ähnlich) als PDF oder in anderen
   Klartext-Formaten zur Verfügung! Bitte beantragen Sie nur Verfahren, für die die
   technischen Voraussetzungen bestehen!
   Hinweis - Beantragen Sie nur Datenarten für die die technischen Voraussetzungen bestehen. -
   Grafik Sprechblase

   Im Benutzerhandbuch von ELSTER-Transfer (Windows/Linux) erhalten Sie einen Überblick über
   das Muster der Dateinamen im Eingangsverzeichnis (wenn Sie Daten von der Finanzverwaltung
   abholen möchten) und die Möglichkeiten des Datenversandes.
   Vereinfachte Darstellung der Architektur: Versand von Daten zur Finanzverwaltung

   (1) Der ETR-Nutzer (Kommunen, Kammern, ..) nutzt sein Portal oder seine Anwendung um die
   Daten für die Finanzverwaltung zu erzeugen. In diesem Beispiel wird die
   json-Dateiimportschnittstelle genutzt. Hierbei wird eine nutzdaten.zip in das
   Ausgangsverzeichnis und eine json-Adressierungsdatei (mit Versandoptionen) test.json in das
   Importverzeichnis von ELSTER-Transfer abgelegt (2).

   Die ETR-Anwendung packt diese Daten in einen ELSTER-Datensatz und übermittelt die Daten
   verschlüsselt über die ELSTER-Eingangsserver (3) in das Backend und von dort weiter in die
   Eingangsverarbeitung (4) des adressierten Landes (Empfänger).
   Vereinfachte Darstellung der Architektur: Empfang von Daten von der Finanzverwaltung

   Die Ausgangsverarbeitung eines Landes (oder Bund) stellt die an einen ETR-Nutzer
   adressierten Daten über das Backend in die Bereitstellungsdatenbank von ELSTER ein (1).

   Der Nutzer hat in ELSTER-Transfer einen Dauerauftrag angelegt, der periodisch im
   Hintergrund prüft, ob neue Daten für ihn bereitstehen (2, 3). Stehen neue Daten bereit,
   werden sie heruntergeladen und im Eingangsverzeichnis von ELSTER-Transfer abgelegt (4, 5).

   Das Portal oder die Anwendung des ETR-Nutzers kann periodisch das Eingangsverzeichnis von
   ELSTER-Transfer prüfen (6, 7) und neue Daten gleich weiterverarbeiten.

Übermitteln von Mitteilungen oder Bescheide in die Postfächer von Anwendern

   Zielgruppe dieser Funktion sind hauptsächlich Service-Provider, welche via NEZO am
   ELSTER-Identity Provider angebunden sind.
   Weiterführende Informationen: https://info.mein-unternehmenskonto.de/
   Übermitteln von Mitteilungen oder Bescheide in die Postfächer von Anwendern

   Eine wichtige technische Voraussetzung für die Bereitstellung von Mitteilungen oder
   Bescheiden ist die Anbindung an die Rückkanal-Schnittstelle von ELSTER-Transfer. Mit
   dieser Schnittstelle können Verwaltungsakte, die elektronisch bekannt zu geben sind, an
   die Portale Mein ELSTER und Mein Unternehmenskonto übermittelt werden. Dort werden sie
   dann in das Postfach der Anwender eingestellt und können abgerufen werden.

   Zudem wird die Möglichkeit bestehen, dass die Empfänger diese Dokumente via
   Maschine-zu-Maschine-Kommunikation (M2M, ERiC) abrufen können. Auch besteht die Möglichkeit
   für den Versender, Links anzugeben, und damit dem Empfänger die Möglichkeit anzubieten,
   direkt auf Verwaltungsakte zu reagieren.

   Als Adressierungsmerkmal wird dabei die eindeutige "AccountID" bzw. "Benutzerkonto ID"
   (bzw. pseudonymisierte AccountID) des Empfängers verwendet. Weitere Adressierungsmerkmale
   (z.B. Amtlicher Gemeindeschlüssel für Kommunen) sind geplant.

Rechtsverbindliche Bekanntgabe

   Bei der Bereitstellung von rechtsverbindlichen Dokumenten wird die Bekanntgabe nach § 122a
   AO erfüllt. Die Bekanntgabe wird auch entsprechend der gültigen außersteuerlichen
   Bekanntgabevorschriften des Bundes und der Länder, insb. auch § 41 Abs. 2a VwVfG
   ermöglicht. Die Anforderungen an die VwVfG-kompatible technische Umsetzung der
   Abruffunktion sind vom Bund bereitzustellen. Die bekanntgebende Behörde wird dabei über das
   Ergebnis der Bereitstellung des Dokuments und das Ergebnis der Abholung des Dokuments
   informiert. Eine Benachrichtigung des Empfängers per E-Mail über die Bereitstellung neuer
   Daten im Postfach ist möglich.
   Rechtsverbindliche Bekanntgabe - Grafik Sprechblase

   Die eingesetzten Anwendungen müssen hierfür in die Lage versetzt werden, die Dokumente im
   PDF-Format an ELSTER-Transfer zu übergeben. Weitere Informationen zu den Schnittstellen
   finden Sie im Hilfebereich.
   Vereinfachte Darstellung der Architektur: Versand/Empfang von Dokumenten

   (1) Der Versender (Service-Provider, Kommune,..) nutzt sein Portal oder seine Anwendung um
   eine Nachricht zu erzeugen. (2) Dieses Portal (oder seine Anwendung) nutzt die
   ETR-REST-API, um das Dokument und weitere Versandoptionen der ELSTER-Transfer-Anwendung zu
   übergeben (3).

   Die ETR-Anwendung packt diese Informationen in einen ELSTER-Datensatz und übermittelt die
   Daten verschlüsselt in das ELSTER-Backend (4). Dort wird das Dokument in die
   Bereitstellungsdatenbank abgelegt (5) und wartet auf die Abholung des Empfängers. Ab diesem
   Zeitpunkt prüft die ETR-Anwendung auch den Status des Dokumentes (Bereitgestellt,
   Abgeholt,..) und kann den Status zum Portal/der Anwendung des Versenders zurückmelden (5,
   4, 3, 2).

   Der Empfänger der Nachricht wird über den Eingang informiert. Er nutzt den Posteingang von
   Mein ELSTER/Mein Unternehmenskonto (Mein UK) oder seine Drittherstelleranwendung (mit
   integriertem ERiC) um die Nachricht abzuholen (6). Hierbei fragt Mein ELSTER, Mein UK oder
   ERiC über die Eingangsserver (7) an der Bereitstellungsdatenbank (8) an, ob neue Dokumente
   bereitstehen. Ist dies der Fall, wird ihm die Nachricht zur Verfügung gestellt (9, 10).
```
