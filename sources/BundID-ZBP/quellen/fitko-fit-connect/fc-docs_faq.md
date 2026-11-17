# FAQ (FIT-Connect)

- Quelle: <https://docs.fitko.de/fit-connect/docs/faq>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
FAQ

1 Über FIT-Connect

1.1 Was ist FIT-Connect?

   FIT-Connect ist eine IT-Infrastruktur zur elektronischen Übermittlung von Anträgen (Einreichungen) von einem Sender (Onlinedienst) zu einem
   adressierten Empfänger (Verwaltungssystem/Subscriber). Zudem ermöglicht FIT-Connect die weitere Kommunikation zwischen Sender und Empfänger, etwa die
   Zustellung des Bescheids. Eine ausführliche Beschreibung von FIT-Connect finden Sie ab der Seite Was ist FIT-Connect?.

1.2 Was sind die größten Vorteile von FIT-Connect?

   FIT-Connect erleichtert die maschinenlesbare und verschlüsselte Kommunikation zwischen Antragsteller:innen und den zuständigen Fachbehörden
   (Verwaltungssystemen). FIT-Connect erlaubt die standardisierte und sichere elektronische Zustellung von Einreichungen (Anträgen) an adressierbare
   Empfänger (Verwaltungssysteme). FIT-Connect ermöglicht zudem die Entkopplung der Entwicklung von Onlinediensten und Verwaltungssystemen. Dies
   vereinfacht die langfristige Pflege der Sender und Empfänger. Mehr dazu erfahren Sie auf der Seite Vorteile von FIT-Connect.

1.3 Wer ist Zielgruppe von FIT-Connect? Muss ich mich als Vertreter:in einer Kommune mit FIT-Connect vertraut machen?

   Informationen für die einzelnen Zielgruppen finden Sie auf der Seite Wer kann FIT-Connect nutzen?. Vertreter:innen von Kommunen sollten sich
   grundsätzlich mit FIT-Connect vertraut machen. Unsere Einstiegsdokumentation hilft Ihnen dabei. Darüber hinaus sollten Sie auch mit Ihren
   Verwaltungssystemherstellern in Kontakt treten, denn immer mehr Hersteller entwickeln bereits Schnittstellen für den Empfang von Anträgen über
   FIT-Connect. Die FITKO bietet ausgewähltes Informationsmaterial und auch auf Kommunen maßgeschneiderte Informationsveranstaltungen an.

1.4 Welche Vorteile bringt FIT-Connect für ein Umsetzungsprojekt, in dem sowohl ein Onlinedienst als auch ein Verwaltungssystem angebunden werden?

   FIT-Connect ermöglicht die für eine schnelle OZG-Umsetzung erforderliche Entkopplung der Entwicklung von EfA-Anträgen (Onlinediensten) einerseits und
   der Anbindung von Verwaltungssystemen andererseits. Die Teams zur Entwicklung von Onlinediensten und Verwaltungssystemen können autark voneinander
   arbeiten. FIT-Connect ist hierbei die definierte Bezugsgröße. Zudem vereinfacht die Entkopplung auch die langfristige Pflege der entwickelten Lösungen.

1.5 Was kostet es, ein Verwaltungssystem bei FIT-Connect zu registrieren?

   FIT-Connect wird durch den Bund und die Länder finanziert. Die Registrierung von Verwaltungssystemen und Onlinediensten sowie die Nutzung von
   FIT-Connect als technische Infrastrukturkomponente sind daher kostenlos. Allerdings obliegt die Entscheidung, ob es zu einer Kostenumlage auf die
   Kommunen des Landes kommt, den jeweiligen Landesregierungen. Dies kann insbesondere dann relevant sein, wenn ein EfA-Dienst eines anderen Bundeslandes
   im Rahmen der so genannten Nachnutzung zur Verfügung gestellt wird. Bitte wenden Sie sich hierzu an die zuständigen Stellen in den Landesministerien.

   Auf der Seite Aufwandsschätzung erhalten Sie einen Überblick über die zu erwartenden Aufwände, über deren Treiber und über Optionen zur Verringerung
   von Aufwänden. Darüber hinaus finden Sie in der Schritt-für-Schritt-Anleitung der Anbindungsdokumentation eine auf Erfahrungswerten basierende
   Aufwandsprognose für jeden Anbindungsschritt.

1.6 Wer trägt die Kosten für FIT-Connect?

   Die Kosten für den Betrieb der Infrastruktur von FIT-Connect werden vom Bund und den Ländern getragen, nicht von den Kommunen. Es kann jedoch nicht
   ausgeschlossen werden, dass das jeweilige Bundesland eine Umlage für die Nachnutzung von FIT-Connect an die Kommunen weiterreicht.

1.7 Können Kosten umgelegt werden?

   Ja. Den jeweiligen Landesregierungen obliegt die Entscheidung, ob es zu einer Kostenumlage auf die Kommunen des Landes kommt, wenn ein EfA-Dienst eines
   anderen Bundeslandes im Rahmen der Nachnutzung zur Verfügung gestellt wird. Hierzu befragen Sie bitte die zuständigen Stellen in den Landesministerien.

1.8 Wer trägt die Kosten für die Anbindung eines Fachverfahrens?

   Der Hersteller eines Fachverfahrens kann die Kosten für die Anbindung des Verfahrens an FIT-Connect der jeweiligen Kommune in Rechnung stellen.

1.9 Wer trägt die Kosten für die Nachnutzung der Anbindung an FIT-Connect?

   Wenn eine Kommune die Anbindung eines Verfahrens an FIT-Connect nicht selbst beauftragt, sondern eine bereits entwickelte Anbindung übernimmt, können
   auf die Kommune Kosten für diese Nachnutzung zukommen.

1.10 Fallen Kosten für ein V-PKI-Zertifikat an?

   Ja. Für die Verschlüsselung der Daten ist ein V-PKI-Zertifikat erforderlich. Für das Zertifikat fallen Kosten bei den Kommunen an.

1.11 Seit wann steht FIT-Connect für den produktiven Einsatz für Portale und Verwaltungssysteme zur Verfügung?

   FIT-Connect ist seit dem Frühjahr 2022 produktiv und wird kontinuierlich von immer mehr Onlinediensten und Verwaltungssystemen genutzt. Der erste
   Onlinedienst, der über FIT-Connect produktiv ging, war im März 2022 das Verfahren zur Beantragung des Schwerbehindertenausweises. Viele Onlinedienste
   und Verwaltungssysteme, die seitdem an FIT-Connect angeschlossen wurden, sind in unserem Anbindungskatalog verzeichnet.

1.12 Gibt es Erfahrungen, wie aufwendig die technische Anbindung von Onlinediensten oder Verwaltungssystemen an das Testsystem von FIT-Connect ist?

   Durch die schlanke REST API, die auch über SDKs von den anbindenden Systemen implementiert werden kann, ist es prinzipiell möglich, ein
   Verwaltungssystem zeitnah in die Testphase zu bringen. Auch die Anbindung eines empfangenden Verwaltungssystems und ein erster Empfangstest können
   innerhalb kurzer Zeit eingerichtet werden. Vergleichbare Größenordnungen sind auch für den Anbindungsaufwand von Onlinediensten erwartbar. Der Aufwand
   für eine tiefgehende Integration in Verwaltungssysteme geht jedoch darüber hinaus.

1.13 Finden regelmäßig Informationsveranstaltungen zu FIT-Connect statt? Gibt es spezielle Formate für Kommunen?

   Ja, es finden regelmäßig Online-Veranstaltungen zu FIT-Connect sowie zur Anbindung von Onlinediensten und Verwaltungssystemen statt.

2 FIT-Connect - fachlich

2.1 Kann FIT-Connect für EfA-Anträge genutzt werden?

   Ja, FIT-Connect wurde vom IT-Planungsrat gezielt für zentral bereitgestellte Onlinedienste (EfA-Anträge) entwickelt.

2.2 Wird FIT-Connect zwingende Voraussetzung für alle EfA-Leistungen werden?

   Selbstverständlich erfolgt die Nutzung von FIT-Connect auf freiwilliger Basis. FIT-Connect verfolgt jedoch den Anspruch, die einfachste, komfortabelste
   und flexibelste Lösung zu sein, um EfA-Onlinedienste mit den verarbeitenden Behörden zu verbinden. FIT-Connect wurde exakt für diesen Zweck entwickelt
   und gewährleistet die Vermittlung zwischen EfA-Anträgen und den bearbeitenden Stellen auf optimale Weise.

2.3 FIT-Connect unterstützt interoperable Postfächer und die Anbindung verschiedener Rückkanalkomponenten (FIT-Connect Booklet). Was bedeutet das?

   Über das Metadatenschema werden Informationen über den Rückkanal (Postfach ID) transportiert. Der Zustelldienst implementiert aber selbst nicht den
   Rückkanal von FINK (Föderiertes Identitätsmanagement interoperabler Nutzerkonten).

2.4 Wie grenzt sich FIT-Connect von anderen Projekten wie dem DVDV und OSCI-Intermediär ab?

   Für FIT-Connect ist kein DVDV-Eintragungskonzept erforderlich. Das DVDV ist über einen eigenen Microservice mit separatem Datenbestand an FIT-Connect
   angebunden. Die Eintragung von Zustellpunkten im Self-Service-Portal von FIT-Connect erfolgt dabei vollständig automatisch über diesen Microservice.
   Die im DVDV hinterlegten Parameter können über den Routingdienst von FIT-Connect abgerufen werden. Darüber hinaus ist für FIT-Connect grundsätzlich
   kein OSCI-Intermediär notwendig.

2.5 Senden und Empfangen von Anträgen

2.5.1 Wer klärt Probleme beim Transport, wenn beispielsweise Übermittlungen nicht im Verwaltungssystem ankommen?

   Bei Problemen im Bereich Transport ist grundsätzlich der Support von FIT-Connect zuständig. Unseren Support erreichen Sie über die Seite Technischer
   Support.

2.5.2 Wann sehe ich welche Ansicht in FIT-Connect – und mit welchem Account?

   In FIT-Connect hängen die verfügbaren Ansichten und Funktionen von der Rolle und der Zugriffsberechtigung des Accounts ab:
     * Zugriff über das Zertifikat Mein Unternehmenskonto (MUK): Beim Login via "Mein Unternehmenskonto" wird ein technischer Zugang mit einem konkreten
       Mandanten (Organisation) und der Owner-Rolle verknüpft.
     * Verwaltende Rolle: Sie ermöglicht Zugriff auf administrative Funktionen, z. B. auf das Erstellen und Bearbeiten von Zustellpunkten. Diese Rolle ist
       per Login mit "Mein Unternehmenskonto (User)" möglich. Hierbei ist kein MUK-Zertifikat notwendig, sondern der Benutzername und das selbst gewählte
       Passwort.
     * Lesende Rolle: Diese Ansicht zeigt Ihnen Statusinformationen, Logs und grundlegende Metadaten, ohne Eingriffsmöglichkeiten an. Diese Rolle ist per
       Login mit "Mein Unternehmenskonto (User)" möglich. Hierbei ist kein MUK-Zertifikat notwendig, sondern der Benutzername und das selbst gewählte
       Passwort. Die Teamfunktionalität erlaubt es, mehreren Personen Zugriff auf ein Projekt zu geben – mit jeweils individuellen Rechten für die Ansicht
       von Zustellpunkten und Clients.

   Wichtig: Eine Person sieht nur die Umgebung, die vom jeweiligen Owner freigegeben wurde. Nutzer:innen können – jeweils für unterschiedliche Owner –
   mehrere Zugänge besitzen, sehen dabei aber nur die für sie freigegebenen Informationen.

   Die Ansicht, die angezeigt wird, ergibt sich aus ...
    1. dem verwendeten MUK-Zertifikat,
    2. der zugewiesenen Rolle im FIT-Connect-Verwaltungsbereich des Owners und
    3. der jeweiligen Umgebung (Test-, Stage- oder Produmgebung).

   Bei eventuellen Problemen sollten zuerst die Login-Methode und die eigene Rolle geprüft werden. Falls dies nicht zum Erfolg führt, kann gerne das
   FIT-Connect Anbindungsmanagement kontaktiert werden.

2.5.3 Um Anträge über FIT-Connect zu empfangen, muss mindestens ein Zustellpunkt im Self-Service-Portal von FIT-Connect angelegt werden. Kann man einen
zentralen Zustellpunkt für eine ganze Behörde oder auch für eine Menge von Verwaltungssystemen anlegen, über den dann alle angebotenen Leistungen zentral
empfangen werden können?

   Ja, das ist möglich. Ein Zustellpunkt kann mehrere Leistungen umfassen. Konfiguriert werden kann dies über das Self-Service-Portal von FIT-Connect
   sowie über die Destination API. Einige Konzeptvorschläge finden Sie unter Konzeption von Zustellpunkten.

2.5.4 Manche Bundesländer transportieren im kommunalen Bereich über 25 Mio. Übermittlungen pro Jahr. Gibt es bei FIT-Connect Größenbeschränkungen z. B. in
Bezug auf die Anzahl von Übermittlungen pro Zeiteinheit oder auch des Umfangs einer Übermittlung?

   Nein. Die Konfiguration ist anpassbar. Aktuell liegt das Limit bei 2 GB pro Antrag.

2.5.5 In welchen Datenformaten werden Daten über FIT-Connect übertragen?

   Was genau übertragen wird, entscheidet alleine der Onlinedienst. FIT-Connect hat aufgrund der Ende-zu-Ende-Verschlüsselung keinen Einfluss/Zugriff auf
   die übermittelten Daten. Bei FIT-Connect ist per Nutzungsbedingungen vorgegeben, dass Daten maschinenlesbar übertragen werden müssen (XÖV oder
   FIM-Stammdaten), um eine automatisierte Verarbeitung der Daten zu erlauben.

2.5.6 Werden die übertragenen Daten auf unsichere Inhalte geprüft?

   Bei FIT-Connect sind die übertragenen Daten Ende-zu-Ende verschlüsselt. FIT-Connect kann deshalb keine Prüfung auf unsichere Inhalte durchführen.

2.5.7 Angenommen ein Antrag wurde an einen Zustellpunkt zugestellt, aber er wird nicht abgeholt. Wie verhält sich FIT-Connect? Wie lange werden die
Übermittlungen von FIT-Connect gespeichert? Nach welcher Zeit werden sie gelöscht? Gibt es ein Löschkonzept?

   Ja, es gibt ein Löschkonzept. Wenn eine Abholung eines Antrags bestätigt wird, löscht FIT-Connect den Antrag unmittelbar. Wird ein Antrag 21 Tage lang
   nicht abgeholt, wird er ebenfalls gelöscht.

2.6 Beauftragung der Anbindung

2.6.1 Gibt es eine Testumgebung für FIT-Connect und wie kann man auf dieser Umgebung testen?

   Das Self-Service-Portal der Testumgebung steht allen Interessierten zur Verfügung, um API Clients für die Testinstanz des FIT-Connect-Zustelldienstes
   zu registrieren. Sie können dafür einen eigenen Zugang erstellen. Weitere Informationen zu den verfügbaren Optionen finden Sie unter
   Accountregistrierung und Zugangsdaten-Verwaltung.

2.6.2 Muss die Konfiguration von Onlinediensten bzw. von Verwaltungssystemen einzeln und pro Behörde vorgenommen werden oder kann hier ein Dienstleister eine
Konfiguration für alle seine Kunden anlegen?

   Ja, die Konfiguration von Onlinediensten und Verwaltungssystemen kann von einem IT-Dienstleister zentral für alle seine Kunden vorgenommen werden,
   statt dass jede Behörde dies einzeln durchführen muss. Besonders kleinere Kommunen nutzen häufig IT-Dienstleister, um ihre Verwaltungssysteme zu
   konfigurieren und an FIT-Connect anzubinden. Verwaltungssystemhersteller und andere Dienstleister profitieren von neuen Werkzeugen, die die Pflege der
   FIT-Connect-Anbindungen erleichtern. Wenn es erforderlich ist, Daten in die Redaktionssysteme der Länder zu übertragen, bieten IT-Dienstleister auch
   hierbei Unterstützung. Dadurch wird sichergestellt, dass sowohl große als auch kleine Kommunen problemlos an FIT-Connect angebunden werden können.

2.6.3 Wann müssen der AVV und die Nutzungsbedingungen unterschrieben werden?

   Der AVV und die Nutzungsbedingungen müssen unterschrieben werden, wenn die Staging- und Produktivumgebung von FIT-Connect genutzt werden soll. Für die
   Testumgebung sind keine administrativen Schritte notwendig.

2.6.4 Wer muss den AVV und die Nutzungsbedingungen unterschreiben?

   Empfangende Systeme (meist Verwaltungssysteme und Behörden) müssen die Nutzungsbedingungen unterschreiben. Sendende Systeme (meist Onlinedienste)
   müssen den AVV und die Nutzungsbedingungen unterschreiben. Die Vertragsdokumente werden bei Unternehmen von der prokura innehabenden Person
   unterschrieben. Bei Behörden ist hierzu in der Regel die Amtsleitung, Abteilungsleitung oder sonstige Amtsträger im Rahmen ihrer dienstlichen
   Funktionen oder Befugnisse zuständig. Sie erhalten die Dokumente nach Einwahl in die Stage- bzw. Produktivumgebung vom Self-Service-Portal (SSP).

2.6.5 Müssen Empfänger von FIT-Connect, die die bidirektionale Kommunikation nutzen, auch den AVV unterschreiben?

   Ja. Nutzer, die die bidirektionale Kommunikation nutzen, werden damit auch zu sendenden Systemen und müssen den AVV abschließen.

2.6.6 Gibt es bei FIT-Connect Größenbeschränkungen z. B. in Bezug auf die Anzahl der Übermittlungen pro Zeiteinheit oder auch der Größe der Übermittlungen?

   Für die Anzahl der Übermittlungen pro Zeiteinheit gibt es keine Beschränkungen. Es gibt jedoch eine Maximalgröße für Anhänge, die bei 2 GB pro Antrag
   liegt. Diese kann bei Bedarf angepasst werden.

2.6.7 Können sich auch Dokumenten-Management-Systeme als Empfänger an FIT-Connect anschließen?

   Ja, prinzipiell kann jede Software, die eine entsprechende Schnittstelle bereitstellt, Anträge über FIT-Connect empfangen.

2.7 IT-Sicherheit und Datenschutz

2.7.1 Muss für den Empfang von Daten (Pull aus dem Verwaltungssystem zu FIT-Connect) die Firewall geöffnet werden?

   Da prinzipiell neben der Konfiguration eines Callbacks auch die Möglichkeit zum Polling durch den Empfänger gegeben ist, lautet die Antwort nein. Wenn
   Sie hingegen mit Callbacks arbeiten wollen, muss der Callback die Firewall passieren dürfen.

2.7.2 Muss der Onlinedienst das Vertrauensniveau des Verwaltungssystems prüfen?

   Ja. Grundsätzlich muss der Onlinedienst vor dem Versenden sicherstellen, dass schutzwürdige Daten nicht in ungesicherte Systeme übertragen werden.

2.7.3 Wenn ich meinen Account lösche: Wie lange werden die Accountdaten aufbewahrt, bevor sie endgültig gelöscht werden?

   Die Accountdaten werden nach Löschung des Accounts innerhalb von 90 Tagen (3 Monate) gelöscht.

2.7.4 Werden Anhänge auf Viren überprüft?

   Da übertragene Daten im Zustelldienst verschlüsselt vorliegen, kann FIT-Connect für diese keine Virenprüfung durchführen.

2.7.5 Von wem muss ein Virenscan durchgeführt werden?

   Laut den Nutzungsbedingungen muss das Verwaltungssystem oder der Onlinedienst die Virenscans durchführen. Wir empfehlen eine Orientierung an der
   BSI-Richtlinie „TR-03172 Portalverbund“. Demnach verpflichten sich alle beteiligten Stakeholder zur Gewährleistung der IT-Sicherheit durch Maßnahmen,
   die dem aktuellen Stand der Technik entsprechen.

2.7.6 Muss für jeden Zustellpunkt ein eigenes Zertifikat verwendet werden?

   Nein. Ein einziges Zertifikat der Verwaltungs-PKI reicht für die Anbindung von bis zu 30 Verfahren Ihrer Behörde an FIT-Connect. Immer vorausgesetzt,
   dass die BSI-Sicherheitsanforderungen eingehalten werden.

2.7.7 Wenn eine Behörde mehrere Leistungen über FIT-Connect anbietet, erfordert dies dann die Konfiguration mehrerer Zustellpunkte?

   Sie können entweder für Ihre Behörde einen Zustellpunkt konfigurieren, der mehrere Leistungen umfasst oder für jede Leistung einen eigenen Zustellpunkt
   anlegen. Wichtig ist, dass Sie pro Verwaltungssystem einen Zustellpunkt anlegen. Werden in diesem Verwaltungssystem mehrere Leistungen bearbeitet? Wenn
   ja, genügt ein Zustellpunkt. Nutzen Sie für jede Leistung ein anderes Verwaltungssystem, brauchen Sie auch jeweils einen weiteren Zustellpunkt. Die
   Seite Zustellpunkte stellt hierzu verschiedene Konzepte vor.

2.7.8 Kann man einen zentralen Zustellpunkt pro Behörde anlegen, über den alle angebotenen Leistungen zentral eingehen?

   Es muss pro Verwaltungssystem (z. B. Fachverfahren) ein Zustellpunkt angelegt werden. Wenn mehrere Behörden bzw. Abteilungen dasselbe Verwaltungssystem
   nutzen, genügt ein Zustellpunkt. Nutzt jede Behörde bzw. Abteilung ein eigenes Verwaltungssystem, auch wenn es immer dieselbe Leistung ist, erfordert
   dies jeweils einen eigenen Zustellpunkt.

2.7.9 Kann man über einen Zustellpunkt mehrere Behörden anschließen?

   Ja. Wenn alle diese Behörden dasselbe Verwaltungssystem nutzen, kann ein Zustellpunkt mehrere Regionen (= mehrere Kommunen) oder Behörden und auch
   mehrere Leistungen umfassen. Nutzt jede Behörde bzw. Kommune ein eigenes Verwaltungssystem, erfordert dies jeweils einen eigenen Zustellpunkt.

2.7.10 Dürfen nur Behörden Clients und Zustellpunkte im Self-Service-Portal der FITKO anlegen?

   Nein. Jede Person, die von einer Behörde autorisiert wurde oder die die Nutzungsbedingungen selbst unterschrieben hat, darf das Self-Service-Portal
   nutzen. Darüber hinaus kann die Verwaltung auch von Dienstleistern übernommen werden.

2.7.11 Leiten Zustellpunkte automatisch Anträge an die Verwaltungssysteme weiter?

   Nein. Zustellpunkte stellen Anträge zur Abholung bereit. Verwaltungssysteme müssen selbstständig und regelmäßig prüfen, ob für sie Anträge
   (Einreichungen) vorliegen, um diese dann abzurufen (pull). Zusätzlich lassen sich die Verwaltungssysteme benachrichtigen, sobald ein Antrag für sie im
   Zustellpunkt eingegangen ist (callback).

2.7.12 Ist für FIT-Connect ein XTA-Server notwendig?

   Nein, für FIT-Connect ist grundsätzlich kein XTA-Server erforderlich.

2.7.13 Kann FIT-Connect auch in Szenarien ohne Verwaltungssystem genutzt werden (PDF-Übertragung)? Gibt es einen Minimal-Client wie z. B. COM Vibilia für
OSCI?

   Ein generischer Empfangsclient existiert nicht. Wir werden im Projekt jedoch entsprechende Tools zu Test-/Entwicklungszwecken bereitstellen. Darüber
   hinaus wäre eine leichtgewichtige Integration in bestehende Systeme (Ticket-Systeme, Mailsysteme) denkbar. Zudem können Systeme wie z. B. Governikus
   MultiMessenger, MODUL-F oder auch die OZG Cloud zum Empfang von Übermittlungen über FIT-Connect genutzt werden.

2.8 Konfiguration Self-Service-Portal

2.8.1 Wie kann ich auf das Self-Service-Portal zugreifen?

   In der Testumgebung ist die Anmeldung beim Self-Service-Portal per Username und Passwort über Ihren Account bei GitHub, openCode, Mein
   Unternehmenskonto oder GitLab möglich.

2.8.2 Beim Abruf des Tokens bekomme ich die Fehlermeldung "Invalid, unknown or malformed scope".

   Bei der Einrichtung eines empfangenden Systems (Subscriber) im Self-Service-Portal ist es notwendig, den zuvor erstellten Zustellpunkt auch dem API
   Client zuzuordnen.

2.8.3 Kann man wie bei der publicKey_encryption auch eine neue publicKey_signature_verification über die Schnittstelle hochladen?

   Ja, diese können genauso wie der Public Key für die Verschlüsselung hochgeladen werden. Im Gegensatz zu publicKey_encryption muss kein Schlüssel
   explizit aktiviert werden, da für die Signaturprüfung die neuen und alten Schlüssel gleichermaßen herangezogen werden können/müssen.

2.8.4 Besteht die Möglichkeit den Public Key über das Self-Service-Portal anzupassen, ohne zuvor die Destination zu löschen?

   Nein, dies ist aktuell nur über die API möglich. Im SSP muss die Destination neu angelegt werden.

2.9 Versand

2.9.1 Kann man nach dem erfolgreichen Versand einer Einreichung weitere Daten nachsenden, solange die Daten nicht abgeholt wurden?

   Im Nachgang können nur Anhänge überschrieben werden. Man kann also nichts zusätzlich einreichen, wenn dies nicht angekündigt wurde. Das Nachreichen
   einer Einreichung erzeugt eine neue Einreichung.

2.9.2 Was mache ich als Sender, wenn ich keine von der Destination angebotenen "Submission Schemas" unterstütze?

   Die Destination gibt die zu nutzenden Schemas vor. Falls eine Konvertierung in das gewünschte Format nicht automatisiert möglich ist, sollte dies dem
   Support des Senders auffallen. In diesem Fall sollte die organisatorische Abstimmung mit der für die Destination verantwortlichen Stelle gesucht
   werden.

2.9.3 Woher weiß der Empfänger, welcher Public Key zur Verschlüsselung verwendet werden soll? Zumal mehrere Schlüssel in einer Destination hinterlegt werden
können.

   Der für die Verschlüsselung aktive/zu nutzende Schlüssel wird im Feld encryptionKid angegeben. Anhand der ID können Sie aus dem Array den relevanten
   Schlüssel herausfinden.

2.9.4 Beim Empfänger ist "encryptedData" in dem Antrag immer null, obwohl es mitgesendet wird. Woran liegt das?

   Beachten Sie, dass Sie das Feld "data" bei der Antragsanlage immer auf "true" setzen müssen, damit auch diese Daten übermittelt werden.

2.9.5 Beim Versand erhalte ich die folgende Fehlermeldung: 422 Unprocessable Entity: Error during SET validation

   {

   "type": " <Link to={'https://schema.fitko.de/fit-connect/problems/security-event-token-validation",
   "title": "Error</Link> during SET validation",
   "status": 422,
   "detail": "There was a problem during the processing of the provided SET",
   "issue": "The specified public key in the header cannot be found."
   }

   Bei der Destination in der Testumgebung ist kein Schlüssel hinterlegt, der mit dem bereitgestellten Tool generiert werden sollte. Ersetzen Sie den
   Schlüssel, wie in der JWK-Dokumentation beschrieben.

2.9.6 Beim Versand erhalte ich die folgende Fehlermeldung: 409 Conflict: Invalid state transition

   Die Statuswechsel dürfen nur in der festgelegten Reihenfolge durchgeführt werden. Beachten Sie die definierte Statusfolge.

2.9.7 Gibt es Eingangsstempel für Anträge mit Fristen?

   Ja. Über das Eingangsprotokoll kann das Ereignis 'Submit Submission' eingesehen werden (die Einreichung wurde durch den Sender versendet).

2.9.8 Was Schritte sind erforderlich, um die Signatur der Events (z. B. nach Jahren) zu prüfen, wenn die öffentlichen Schlüssel bereits getauscht wurden?

   Bei Änderung der Signaturschlüssel seitens des Senders wird empfohlen, die alten öffentlichen Schlüssel nicht zu löschen, sondern lediglich die neuen
   Schlüssel hinzuzufügen, sodass eine Signaturprüfung (gegen alle vorliegenden Schlüssel) weiterhin möglich ist.

2.10 Empfang

2.10.1 Wie ist "Issuer" beim SET der Empfangsbestätigung auszufüllen?

   Hier ist der Ersteller des SET einzutragen (Empfänger). Vom gleichen Empfänger sollte es auch immer identisch befüllt werden (z. B. die Haupt-URL).

2.10.2 Woher kommt die CaseID für die Empfangsbestätigung?

   Die CaseID wird beim Abholen der Submission mitgeliefert. Diese sollte in diesem Feld auch angegeben werden.

2.10.3 Ich möchte einen neuen Verschlüsselungsschlüssel hinterlegen, was ist zu tun?

   Zuerst muss der neue Public Key der Destination zugeordnet werden. Anschließend kann der neue Schlüssel über den Endpunkt aktiviert werden
   (encryptionKid).

2.10.4 Ich kann keinen Antrag empfangen oder senden. Was ist zu tun?

   Überprüfen Sie immer, mit welchem Token Sie die Aufrufe starten. Mit dem Token eines Senders können Sie die für den Empfänger spezifischen Endpunkte
   nicht aufrufen. Umgekehrt können Sie mit dem Token eines Empfängers keine Submissions versenden.

2.10.5 Bei mir schlagen die Signatur oder Signaturvalidierung fehl. Was kann die Ursache sein?

   Denken Sie immer daran, dass der Empfänger die Signatur mit seinem privaten Schlüssel durchführt und dann der Sender mit dem öffentlichen Schlüssel die
   Signatur validiert.

2.10.6 Bei mir schlägt die Ver- oder Entschlüsselung fehl. Was kann die Ursache sein?

   Denken Sie immer daran, dass der Sender mit dem öffentlichen Schlüssel der empfangenden Stelle verschlüsseln muss, damit der Empfänger die Daten mit
   seinem privaten Schlüssel entschlüsseln kann.

2.11 Metadatensatz/Fachdatensatz

2.11.1 Was bedeuten "authenticationInformation" und "identificationReport"?

   Der Bereich "authenticationInformation" ist im Metadatenschema näher beschrieben. Hier können alle für den Antrag relevanten Identifikationsnachweise
   angegeben werden (mehrere, da es auch mehrere Nachweise geben kann, beispielsweise für einen Ehepartner oder ein Kind). Die Inhalte ergeben sich aus
   dem genutzten Dienst (zum Beispiel eID oder Mein Unternehmenskonto).

2.11.2 Um welche Datei handelt es sich bei "content" im Bereich "authenticationInformation"?

   Der Nachweis wird als Base64Url-kodierte Zeichenkette angegeben. Die interne Struktur hängt vom type ab.

2.11.3 Gibt es einen mimeType im Bereich "authenticationInformation"?

   Kurze Antwort: Nein.

2.11.4 Wo muss "schemaURI" angegeben werden?

   Die schemaURI muss nicht unter $schema angegeben werden, sondern unter contentStructure data submissionSchema.

2.11.5 Was bedeutet der Inhalt von $schema? Und bleibt er immer gleich bzw. wie kann man ihn ermitteln?

   Grob zusammengefasst beschreibt das Metadatenschema den Metadatensatz. Im Beispielmetadatensatz wird im Feld $schema auf das zugrunde liegende
   Metadatenschema verwiesen. Die aktuelle Version kann auf der Seite Metadatenschema über den dort ganz oben befindlichen Link heruntergeladen werden.

2.12 Betrieb

2.12.1 Gibt es Wartungsfenster?

   Im Rahmen der täglichen Wartungsroutine findet zwischen 03:00 und 04:00 Uhr eine Datensicherung statt. Währenddessen ist FIT-Connect vorübergehend
   nicht erreichbar.

2.12.2 Wer klärt Probleme beim Transport, wenn Übermittlungen nicht beim Empfänger angekommen sind?

   Grundsätzlich ist bei Problemen der Support von FIT-Connect zuständig. Eine wiederholte Nichtabholung durch das empfangende System stellt einen Verstoß
   gegen die Nutzungsbedingungen dar und kann Konsequenzen bis hin zum Ausschluss haben. Es ähnelt damit den Regeln in Fachstandards wie XMeld zum
   Zusammenspiel von Akteuren im Informationsverbund.

2.12.3 Wie ist die aktuelle Update-Frequenz von FIT-Connect und wie häufig ändern sich die APIs aufgrund von Breaking Changes?

   Tooling-Verbesserungen und UI-Updates finden in der Regel alle drei Wochen statt. Weitere Informationen erhalten Sie in unseren Abkündigungen, dem
   Changelog und unserer Deployment-Kommunikation.

2.12.4 Nach welchem Standard ist das Rechenzentrum, in dem die Daten gespeichert und verarbeitet werden, zertifiziert?

   Die Rechenzentren sind nach IT-Grundschutz des BSI (ISO 27001) zertifiziert.

2.12.5 Wer hostet FIT-Connect?

   Für die produktive FIT-Connect Umgebung werden zwei Server genutzt. Der Authentifizierungsserver mit dem Self-Service-Portal und dem OAuth-Server
   befindet sich im Rechenzentrum von Global Access. Der Zustelldienst wird vom Landesbetrieb IT.Niedersachsen im Rechenzentrum von Dataport betrieben.

2.12.6 Wie werden Proxy-Server in Fit-Connect verwendet?

   In FIT-Connect werden Proxy-Server an mehreren Stellen genutzt:
   Proxy-Server Übersicht

   In jedem Rechenzentrum gibt es zwei Arten von Proxy-Servern:
     * Der Proxy-Server vermittelt Verbindungen von den Servern in das Internet. Ursprünglich wurde der Proxy-Server zum Caching eingesetzt, mittlerweile
       wird er verstärkt zur Kontrolle und Protokollierung von Verbindungen genutzt.
     * Der Reverse Proxy-Server vermittelt Verbindungen aus dem Internet zu den Servern. Je nach Ausführung, wird er auch als "API-Gateway", "Application
       Level Gateway" (ALG) oder "Web Application Firewall" (WAF) bezeichnet.

   Bei einem Client (Onlinedienst oder Fachverfahren), können bis zu vier Proxy-Server beteiligt sein:
     * Der Reverse Proxy-Server von FIT-Connect
     * Der Proxy-Server von FIT-Connect
     * Der Reverse Proxy-Server des Clients
     * Der Proxy-Server des Clients

   Wie kommt eine Verbindung zustande?

   Eine ausgehende Verbindung kann mit oder ohne Proxy-Server aufgebaut werden:
   Proxy-Server Übersicht

   In der oben abgebildeten Grafik steht jeder Pfeil für eine Netzwerkverbindung. Bis auf eine Ausnahme handelt es sich um HTTP-Verbindungen, bei denen
   sich der Server mit einem Zertifikat ausweist.

   Ohne Proxy-Server kann der Client die Verbindung direkt zum Server aufbauen und der Server weist sich mit seinem Zertifikat aus.

   Im zweiten Fall ist dem Client bekannt, dass er einen HTTP-Proxy nutzen muss. Er wendet sich an den Proxy-Server und übergibt die Serveradresse, die er
   kontaktieren möchte. Der Proxy-Server öffnet daraufhin einen Port, über den der Client den Server erreichen kann. Die eigentliche HTTP-Verbindung wird
   also vom Client zum Server hergestellt. Dabei weist sich der Server gegenüber dem Client mit seinem Zertifikat aus.

   Wenn der Proxy-Server die Verbindung kontrolliert, unterbricht er die verschlüsselte Verbindung. Dadurch entstehen zwei Verbindungen: Vom Client zum
   Proxy-Server und vom Proxy-Server zum Server. Der Server kann sich mit seinem Zertifikat gegenüber dem Proxy-Server ausweisen; der Proxy-Server besitzt
   dieses Zertifikat jedoch nicht. Daher erzeugt er ein improvisiertes Zertifikat, das der Client im Normalfall nicht akzeptieren würde.

   Dies wird in der Regel gelöst, indem der Proxy-Server eine Zertifizierungsstelle enthält, die das adhoc erzeugte Zertifikat beglaubigt. Wird das
   Zertifikat der Zertifizierungstelle dem Client hinzugefügt, akzeptiert er das Verbindungszertifikat des Proxys-Servers.
```
