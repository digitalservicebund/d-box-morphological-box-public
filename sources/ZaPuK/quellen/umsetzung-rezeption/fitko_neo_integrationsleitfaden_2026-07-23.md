# FITKO: Neo – Integrationsleitfaden für Fachbehörden

Quelle: https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Integrationsleitfaden.md
Abgerufen: 2026-11-19 (Commit „feat: add integration guide“ vom 2026-07-23)

Unverändert übernommener Markdown-Text. Bildverweise zeigen auf das Quell-Repository.

---

# Integrationsleitfaden

[[_TOC_]]

## 1 Einführung
Dieser Integrationsleitfaden dient der Anbindung von Fachverfahren an die im Neo-Vorhaben im Rahmen der [Zielarchitektur Postfach- und Kommunikationslösungen](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk) entwickelte Matrix-basierte Kommunikationsinfrastruktur.
Weitere Informationen zum Neo-Vorhaben finden Sie in der [README-Datei](./README.md) dieses openCode-Repositories.

Die im Neo-Vorhaben entwickelte Matrix-basierte Kommunikationsinfrastrktur ist darauf ausgelegt, den digitalen Austausch zwischen Behörden und Privatpersonen zu standardisieren und zu vereinfachen.
Um das Potenzial von Neo in der Praxis zu erproben und weiterzuentwickeln, wird die Lösung schrittweise verschiedene Fachverfahren integriert, die in der deutschen Verwaltungslandschaft bereits im Einsatz sind.

Viele Fachverfahren sind in eine heterogenen IT-Landschaft eingebettet, in der Systembrüche und mangelnde Interoperabilität weit verbreitet sind.
Infolgedessen ist die Kommunikation zwischen Behörden und Privatpersonen häufig auf einseitige Mitteilungen beschränkt.
Eine technisch bedingte Einschränkung verhindert bislang den wechselseitigen Austausch, auch wenn ein konkreter Bedarf vorliegt.
Hinzu kommt, dass IT-Sicherheitsanforderungen, insbesondere in Bezug auf Verschlüsselung, in der Praxis häufig nicht durchgängig erfüllt werden.
Genau hier setzt Neo an: Als einheitliche und sichere Kommunikationsschicht ermöglicht Neo bestehenden Fachverfahren die Kommunikation mit Privatpersonen über einen einfach zu nutenden Kommunikationskanal (web-basiert und als iOS-App).

Der in diesem Dokument beschriebene Funktionsumfang ermöglicht eine Ersteinschätzung, ob für Fachbehörden relevante Use-Cases bereits über Neo abgebildet werden können.

Dieses Dokument beschreibt nicht nur die durch Neo realisierten Funktionalitäten, sondern auch die notwendigen Schnittstellen sowie nicht-funktionalen Anforderungen, die Qualität, Sicherheit, Nutzendenerfahrung und langfristige Nachhaltigkeit der Integration sicherstellen.

Ziel des Dokuments ist es, bei der Identifikation realitätsnaher Erprobungsmöglichkeiten zu unterstützen und die nötigen Schritte aufzuzeigen, welche zur Durchführung einer solchen Erprobung nötig sind.

## 2 Ziele des Neo-Vorhabens
- Ziel dieses Vorhabens ist es, das Kommunikationsprotokoll Matrix als technische Basis für eine G2C-Kommunikationsinfrastruktur zu erproben. Hierzu wurde eine Lösungsarchitektur entwickelt, die durch den Einsatz bestehender Authentifizierungskomponenten (eID/BundID) im Verbund mit dem offenem Kommunikationsprotokoll Matrix zur Realisierung einer sog. Antragsfolgekommunikation genutzt werden kann.

- Das offene Kommunikationsprotokoll Matrix realisiert dabei die Kommunikation zwischen Fachverfahren/Behörden und Privatpersonen und stellt maschinenlesbare Schnittstellen für IT-Systeme der Verwaltung bereit. Insbesondere wurde dazu im Vorhaben ein gleichberechtigter Zugang zu den Kommunikationsinhalten über mehrere Kanäle (Webanwendung, Smartphone-App) verprobt.

- Das Vorhaben zielt darauf ab, fortgeschrittene Anwendungsfälle für die Kommunikation zwischen Behörden und Privatpersonen (G2C-Kommunikation) zu evaluieren und damit die Erfahrung für Privatpersonen und Behörden zu verbessern. Es wurde ein produktionsreifer Prototyp (sowohl Web- als auch Mobile-App) entwickelt, der durch Kombination von Security & Privacy by Design sowie Nutzendenfreundlichkeit und Barrierefreiheit eine zukunftssichere, modulare und erweiterbare Architektur schafft. Der Prototyp implementiert modernste End-to-End-Verschlüsselung, eine für Fachverfahren und mobile Endgeräte gleichermaßen nutzbare Schnittstelle und bidirektionale Kommunikation.

**Vorhabenstreiber des Neo-Vorhabens:**

![Vorhabenstreiber des Neo-Vorhabens](images/abbildung_1_projekt_ziele_1.png)

![Projektziele](images/abbildung_2_projekt_ziele_2.png)

## 3 Geschäftsprozess
Das auslösende Prozessereignis ist eine Antragstellung über einen Online-Dienst.
Die Privatperson authentifiziert sich zunächst mit der BundID, bevor sie den Antrag einreicht.
Nach erfolgreicher Authentifizierung kann sie den Antrag abschließen und an das Behörde-Backend übermitteln.
Falls FIT-Connect genutzt wird, fungiert FIT-Connect dabei als zentrale Schnittstelle, die den Nachrichtenaustausch zwischen dem Online-Dienst und dem Behörden-Backend mittels FIT-Connect SDK koordiniert.
In der Rolle des „Postboten" ermöglicht FIT-Connect die bidirektionale Kommunikation zwischen der Privatperson und den Sachbearbeitenden im Fachverfahren.

![Kommunikation zw. Privatperson und Fachbehörde](images/abbildung_3_highlevel_bi_direk_kom.png)

Eine detaillierte Prozessdarstellung findet sich in Anhang A1 und A2 (Anwendungsfall iOS), sowie in Anhang B1 und B2 (Anwendungsfall Web).

## 4 Funktionsumfang der Lösung
### 4.1 Smartphine-App
Kernbestandteil des Neo-Vorhabens ist eine Smartphone-App für einen mobilen Zugang zur Matrix-basierten Kommunikationsinfrastruktur für Privatpersonen.
Die App wird zu einem späteren Zeitpunkt zu Demonstrationszwecken gegenüber Stakeholdern im Apple App Store bereitgestellt. Derzeit ist die App über Apple Testflight-Programm verfügbar.[^1]

#### 4.1.1 E-Mail und Push-Benachrichtigung
***Anwendungsfall:*** Im Rahmen der Bearbeitung erforderlicher Fachverfahrens-Prozessschritte informieren Behörden die Privatpersonen in Echtzeit über eine mobile Anwendung über notwendige Aktionen oder die Zustellung neuer Dokumente.

- *User Story Privatperson*: Ich möchte in Echtzeit über erforderliche Aktionen oder der Zustellung neuer Bescheide benachrichtigt werden, um zeitnah reagieren zu können, auch wenn meine Neo-Smartphone-Anwendung zu diesem Zeitpunkt nicht geöffnet ist.
- *User Story Fachbehörde*: Ich möchte, dass die Privatperson in Echtzeit über erforderliche Aktionen oder die Zustellung neuer Bescheide benachrichtigt werden, damit sie zeitnah reagieren können, auch wenn ihre Neo-Smartphone-Anwendung zu diesem Zeitpunkt nicht geöffnet ist.
<table>
  <tr>
    <th width="30">#</th>
    <th width="320">E-Mail- und Push-Benachrichtigungen</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>1</b></td>
    <td width="320"><b>UI in Arbeit</b></td>
    <td width="280">
      <b>Übersicht:</b> Privatpersonen werden über neu eingehende Nachrichten von Behörden per Push-Benachrichtigung informiert.<br><br>
      <b>Mehrwert:</b> Privatpersonen erlangen Kenntnis von erhaltenen Nachrichten und Aktualisierungen des Status eines Verwaltungsvorgangs und können umgehend handeln.<br><br>
      <b>Funktionsweise:</b> Dazu gehören: Antworten der Fachbehörde, Änderungen des Nachrichtenstatus, Zustellung von Bescheiden und Gutachten.
    </td>
  </tr>
</table>

#### 4.1.2 Kommunikationsfunktionen
***Anwendungsfall:*** Bidirektionale Echtzeit-Kommunikation zwischen Privatpersonen und Behörden über eine mobile Anwendung. Nach Antragstellung durch eine Privatperson in einem Onlinedienst übermittelt die Behörde Informationen an die Privatperson oder fordert über strukturierte, maschinenlesbare Nachrichten weitere Informationen oder Dokumente an, welche zur weiteren Antragsbearbeitung erforderlich sind.

- *User Story Privatperson:* Ich möchte über eine Smartphone-Anwendung Nachrichten von Behörden bezüglich der von mir initiierten Verwaltungsvorgänge schreiben, beantworten und Unterlagen versenden bzw. empfangen.

- *User Story Fachbehörde:* Ich möchte Freitext-Rückfragen oder strukturierte und (teil-)strukturierte Rückfragen an antragstellende Personen stellen sowie Dokumente und Bescheide versenden, eine Terminauswahl anbieten, frei definierbare Multiple-Choice-Abfragen oder Dialogsysteme selbst festlegen und den Privatpersonen über eine Smartphone-Anwendung zur Verfügung stellen.

<table>
  <tr>
    <th width="30">#</th>
    <th width="320">Zustellung von Nachrichten und Bescheiden</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>1</b></td>
    <td width="320">
    <img src="images/ios_Freitext-und_Bearbeitungsfunktionen_1.png" alt="Freitext- und Bearbeitungsfunktionen iOS 1"><br>
    </td>
    <td width="280">
      <b>Übersicht:</b> Die Fachbehörde übermittelt Nachrichten, Dokumente oder Bescheide an die Privatperson.<br><br>
      <b>Mehrwert:</b> Es ermöglicht Behöëden und Sachbearbeiter:innen das Verfassen von Freitext-Nachrichten und rechtssichere übersenden von Dokumenten und Bescheiden.<br><br>
      <b>Funktionsweise:</b> Es sind die folgenden Formatierungsoptionen vorgesehen:<br>
      • Fettdruck, Kursivschrift oder unterstrichener Text;<br>
      • Überschriften und Listen;<br>
      • Einbettung von Links;<br>
      • Einbettung von klickbaren Buttons zur Navigation zu Online-Serices;<br>
      • Einbettung von Info-Boxen mit weiterführenden Informationen;<br>
      • Einbettung von QR-Codes und/oder Bildern/Logos<br><br>
      Darüber hinaus ist die Hinterlegung folgender Metadaten vorgesehen:<br>
      • Aktenzeichen;<br>
      • Informationen zur zuständige Sachearbeiter:in.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Freitext-Rückfragen</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>1</b></td>
    <td width="320">
    <img src="images/ios_Freitext-und_Bearbeitungsfunktionen_2.png" alt="Freitext- und Bearbeitungsfunktionen iOS 2">
    </td>
    <td width="280">
      <b>Übersicht:</b> Der Privatperson beantwortet eine Rückfragen der Fachbehörde in Form einer Freitext-Antwort.<br><br>
      <b>Mehrwert:</b> Es ermöglicht Fachbehörden das Stellen von Rückfragen mit Freitext-Antwortmöglichkeit.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Multiple-Choice-Abfragen</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>2</b></td>
    <td width="320">
    <img src="images/ios_Multiple-Choice-Abfragen.png" alt="ios Multiple-Choice Abfragen">
    </td>
    <td width="280">
      <b>Übersicht:</b> Behörden können strukturierte Nachrichten mit Mehrfachauswahl an Privatpersonen senden.<br><br>
      <b>Mehrwert:</b> Schnelle Bearbeitung von Rückfragen, Weiterverarbeitung der Antworten als maschinenlesbare Informationen.<br><br>
      <b>Funktionsweise:</b> Der:die Sachbearbeiter:in kann mit Hilfe einer strukturierten Nachrichten-Vorlage mehrere Antwortmöglichkeiten festlegen. Die Privatperson kann eine oder mehrere Antwortmöglichkeiten auswählen.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Single-Choice-Abfragen</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>3</b></td>
    <td width="320">
    <img src="images/ios_Single-Choice-Abfragen.png" alt="ios Single-Choice Abfragen">
    </td>
    <td width="280">
      <b>Übersicht:</b> Behörden können strukturierte Nachrichten mit Einfachauswahl an Privatpersonen senden.<br><br>
      <b>Mehrwert:</b> Schnelle Bearbeitung von Rückfragen, Weiterverarbeitung der Antworten als maschinenlesbare Informationen.<br><br>
      <b>Funktionsweise:</b> Der:die Sachbearbeiter:in kann mit Hilfe einer strukturierten Nachrichten-Vorlage mehrere Auswahlantworten festlegen. Die Privatperson kann <b>genau eine</b> Antwortmöglichkeit auswählen.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Terminauswahl</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>4</b></td>
    <td width="320"><b>UI in Arbeit</b></td>
    <td width="280">
      <b>Übersicht:</b> Behörden bieten den Privatpersonen eine Auswahl an Terminen an.<br><br>
      <b>Mehrwert:</b> Den Terminvereinbarungsprozess vereinfachen und verbessern.<br><br>
      <b>Funktionsweise:</b> Die Terminverwaltung umfasst:<br>
      • Privatpersonen vereinbaren einen Termin mit einer Behörde;<br>
      • Privatpersonen sagen einen Termin bei einer Behörde ab;<br>
      • Privatpersonen verschieben einen Termin bei einer Behörde;<br>
      • Privatpersonen geben einen Zeitrahmen an, in dem sie verfügbar sind;<br>
      • Privatpersonen erhalten einen Zeitrahmen, in dem die Behörde verfügbar ist.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Weiterleitungslink</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>5</b></td>
    <td width="320">
    <img src="images/ios_Weiterleitungslink.png" alt="ios Weiterleitungslink">
    </td>
    <td width="280">
      <b>Übersicht:</b> Behörden können in Nachrichten relevante Links einfügen. Links können als gut wahrnehmbarer Button dargestellt werden (siehe links: Button „Fortsetzung des Vorgangs").<br><br>
      <b>Mehrwert:</b> Für fachlich komplexe Anwendungsszenarien können Privatpersonen auf eine externe Webseite geleitet werden (z.B. den zur Antragstellung genutzten Onlinedienst).<br><br>
      <b>Funktionsweise:</b> Der Link führt zu einer Website, die im In-App-Browser oder in einem neuen externen Browser-Tab geöffnet wird.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Nachforderung von Dokumenten</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>6</b></td>
    <td width="320">
    <img src="images/ios_Nachricht_mit_Anhängen.png" alt="ios Nachricht mit Anhängen">
    </td>
    <td width="280">
      <b>Übersicht:</b> Hochladen von Anhängen in den von ZaPuK geforderten Formaten. Per Konfiguration können zudem Einschränkungen festgelegt werden (z. B. über einen MIME-Typ-Filter).<br><br>
      <b>Mehrwert:</b> Es ermöglicht Fachbehörden, für den Verwaltungsvorgang relevante Unterlagen anzufordern.<br><br>
      <b>Funktionsweise:</b> Die folgenden Datenformate werden unterstützt: PDF, CSV, XML, JSON, PNG, TXT, ICS, iCal, HEIC, WebP.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Bescheide oder Gutachten zustellen</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>8</b></td>
    <td width="320"><b>UI in Arbeit</b></td>
    <td width="280">
      <b>Übersicht:</b> Zustellung von Bescheiden und Gutachten in rechtsgültiger und nachweisbarer Form. Privatpersonen können alle formellen Entscheidungen, die von verschiedenen zuständigen Behörden versandt werden, an einem Ort in der App einsehen (Tab „Dokumente").<br><br>
      <b>Mehrwert:</b> Stellt sicher, dass Bescheide an Privatpersonen versandt werden, wodurch gewährleistet ist, dass der Empfänger den Bescheid entweder erhält oder als erhalten gilt. Eine Bescheidzustellung erfolgt auf Rechtsgrundlage des § 9 OZG.<br><br>
      <b>Funktionsweise:</b> Die Nachricht zur Zustellung von Bescheiden kann Folgendes enthalten:<br>
      • Datum und Uhrzeit der Zustellung;<br>
      • Name und Unterschrift des Verfassers sowie den amtlichen Stempel der Fachbehörde;<br>
      • Aktenzeichen der Fachbehörde;<br>
      • Zeitgestempelte Empfangs- oder Ablehnungsbestätigung;<br>
      • Weitere spezifische maschinenlesbare Metadaten/Attribute des Fachverfahrens (von der Fachbehörde festzulegen).
    </td>
  </tr>
</table>

#### 4.1.3 Status-Übersicht
***Anwendungsfall:*** Ermöglicht Privatpersonen, den Status von Vorgängen über ihre mobile Anwendung zu verfolgen.
Es gibt drei Akteure, deren Aktionen zu einer Statusänderung führen können: den Online-Dienst, der die Anfrage an die Fachbehörde sendet, die Fachbehörde, die den Nutzern Informationen zur Verfügung stellt, und den Nutzer, der der Fachbehörde Antworten und Fragen übermittelt.

- *User Story Privatperson:* Ich möchte kontinuierlich und in Echtzeit über den aktuellen Status meines Antrags informiert werden.
- *User Story Fachbehörde:* Ich möchte die antragstellende Privatperson in Echtzeit über erforderliche Aktivitäten informieren und über den Eingang von Bescheiden rechtzeitig informieren.

<table>
  <tr>
    <th width="30">#</th>
    <th width="320">Statuserklärung auf Zeitachse</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>1</b></td>
    <td width="320">
    <img src="images/ios_Statuserklärung_auf_Zeitachse.png" alt="ios Statuserklärung auf Zeitachse">
    </td>
    <td width="280">
      <b>Übersicht:</b> Ermöglicht Privatpersonen und Behörden, den Status von Vorgängen in Echtzeit zu erkennen.<br><br>
      <b>Mehrwert:</b> Privatpersonen und Sachbearbeiter:innen können den Bearbeitungsstand eines Vorgangs einfach und in maschinenlesbarer Form nachverfolgen. Die Nutzer:innen erhalten in Echtzeit aktuelle Informationen zum Status der einzelnen Verwaltungsvorgänge.<br><br>
      <b>Funktionsweise:</b> Folgende Status sind für den Workflow des Verfahrens bereits definiert:<br>
      • Vorgang gesendet<br>
      • Vorgang eingegangen<br>
      • Vorgang in Prüfung<br>
      • In Bearbeitung<br>
      • Aktivität erforderlich<br>
      • Vorgang abgeschlossen
    </td>
  </tr>
</table>


### 4.2 Webanwendung
Die Webanwendung unterstützt dieselben Kommunikations-Funktionalitäten wie die mobile App. Sie bietet einen webbasierten Zugang zur Kommunikationsinfrastruktur für Privatpersonen.

Über die Neo-Webanwendung erhalten Fachehörden die Möglichkeit, antragsbezogene Rückfragen, Terminbestätigungen und Bescheide direkt aus dem Fachverfahren heraus über einen standardisierten Kommunikationskanal an Privatpersonen zu versenden und zu empfangen.
Alle verwendeten und empfangenen Nachrichten sind jeweils sowohl in der Smartphone-Anwendung und gleichzeitig in der Webanwendung sichtbar.
Webanwendung und Smartphone-App stellen denselben Funktionsumfang bereit, sodass Privatpersonen frei zwischen den beiden Kanälen wählen könen.
Dies ermöglicht eine medienbruchfreie Kommmunikation mit Privatpersonen.

#### 4.2.1 E-Mail und Push-Benachrichtigung
***Anwendungsfall:*** Im Rahmen der Bearbeitung erforderlicher Fachverfahrens-Prozessschritte informieren Behörden die Privatpersonen in Echtzeit über eine Webanwendung über notwendige Aktionen oder die Zustellung neuer Dokumente.

- *User Story Privatperson*: Ich möchte in Echtzeit über erforderliche Aktionen oder der Zustellung neuer Bescheide benachrichtigt werden, um zeitnah reagieren zu können, auch wenn die Neo-Webanwendung zu diesem Zeitpunkt nicht geöffnet ist.
- *User Story Fachbehörde*: Ich möchte, dass die Privatperson in Echtzeit über erforderliche Aktionen oder die Zustellung neuer Bescheide benachrichtigt werden, damit sie zeitnah reagieren können, auch wenn ihre Neo-Webanwendung zu diesem Zeitpunkt nicht geöffnet ist.
<table>
  <tr>
    <th width="30">#</th>
    <th width="320">E-Mail- und Push-Benachrichtigungen</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>1</b></td>
    <td width="320"><b>UI in Arbeit</b></td>
    <td width="280">
      <b>Übersicht:</b> Privatpersonen werden über eingehende Meldungen von Behörden per E-Mail-Benachrichtigung informiert.<br><br>
      <b>Mehrwert:</b> Privatpersonen erlangen Kenntnis von erhaltenen Nachrichten und Aktualisierungen des Status eines Verwaltungsvorgangs und können umgehend handeln.<br><br>
      <b>Funktionsweise:</b> Dazu gehören: Antworten der Fachbehörde, Änderungen des Nachrichtenstatus, Bekanntgabe von Bescheiden und Gutachten.
    </td>
  </tr>
</table>

#### 4.2.2 Kommunikationsfunktionen
***Anwendungsfall:*** Bidirektionale Echtzeit-Kommunikation zwischen Privatpersonen und Behörden über eine Webanwendung.
Nach Antragstellung durch eine Privatperson in einem Onlinedienst übermittelt die Behörde Informationen an die Privatperson oder fordert über strukturierte, maschinenlesbare Nachrichten weitere Informationen oder Dokumente an, welche zur weiteren Antragsbearbeitung erforderlich sind.

- *User Story Privatperson:* Ich möchte über eine Webanwendung Nachrichten von Behörden bezüglich der von mir initiierten Verwaltungsvorgänge schreiben, beantworten und Unterlagen versenden bzw. empfangen.

- *User Story Fachbehörde:* Ich möchte Freitext-Rückfragen oder strukturierte und (teil-)strukturierte Rückfragen an antragstellende Personen stellen sowie Dokumente und Bescheide versenden, eine Terminauswahl anbieten, frei definierbare Multiple-Choice-Abfragen oder Dialogsysteme selbst festlegen und den Privatpersonen über eine Webanwendung zur Verfügung stellen.

<table>
  <tr>
    <th width="30">#</th>
    <th width="320">Zustellung von Nachrichten und Bescheiden</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>1</b></td>
    <td width="320">
    <img src="images/web_Freitext-und_Bearbeitungsfunktionen_1.png" alt="Freitext- und Bearbeitungsfunktionen Web 1"><br>
    </td>
    <td width="280">
      <b>Übersicht:</b> Die Fachbehörde übermittelt Nachrichten, Dokumente oder Bescheide an die Privatperson.<br><br>
      <b>Mehrwert:</b> Es ermöglicht Behöëden und Sachbearbeiter:innen das Verfassen von Freitext-Nachrichten und rechtssichere übersenden von Dokumenten und Bescheiden.<br><br>
      <b>Funktionsweise:</b> Es sind die folgenden Formatierungsoptionen vorgesehen:<br>
      • Fettdruck, Kursivschrift oder unterstrichener Text;<br>
      • Überschriften und Listen;<br>
      • Einbettung von Links;<br>
      • Einbettung von klickbaren Buttons zur Navigation zu Online-Serices;<br>
      • Einbettung von Info-Boxen mit weiterführenden Informationen;<br>
      • Einbettung von QR-Codes und/oder Bildern/Logos<br><br>
      Darüber hinaus ist die Hinterlegung folgender Metadaten vorgesehen:<br>
      • Aktenzeichen;<br>
      • Informationen zur zuständige Sachearbeiter:in.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Freitext-Rückfragen</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>1</b></td>
    <td width="320">
    <img src="images/web_Freitext-und_Bearbeitungsfunktionen_2.png" alt="Freitext- und Bearbeitungsfunktionen Web 2">
    </td>
    <td width="280">
      <b>Übersicht:</b> Der Privatperson beantwortet eine Rückfragen der Fachbehörde in Form einer Freitext-Antwort.<br><br>
      <b>Mehrwert:</b> Es ermöglicht Fachbehörden das Stellen von Rückfragen mit Freitext-Antwortmöglichkeit.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Multiple-Choice-Abfragen</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>2</b></td>
    <td width="320">
    <img src="images/web_Multiple-Choice-Abfragen.png" alt="Web Multiple-Choice Abfragen">
    </td>
    <td width="280">
      <b>Übersicht:</b> Behörden können strukturierte Nachrichten mit Mehrfachauswahl an Privatpersonen senden.<br><br>
      <b>Mehrwert:</b> Schnelle Bearbeitung von Rückfragen, Weiterverarbeitung der Antworten als maschinenlesbare Informationen.<br><br>
      <b>Funktionsweise:</b> Der:die Sachbearbeiter:in kann mit Hilfe einer strukturierten Nachrichten-Vorlage mehrere Antwortmöglichkeiten festlegen. Die Privatperson kann eine oder mehrere Antwortmöglichkeiten auswählen.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Single-Choice-Abfragen</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>3</b></td>
    <td width="320">
    <img src="images/web_Single-Choice-Abfragen.png" alt="Web Single-Choice Abfragen">
    </td>
    <td width="280">
      <b>Übersicht:</b> Behörden können strukturierte Nachrichten mit Einfachauswahl an Privatpersonen senden.<br><br>
      <b>Mehrwert:</b> Schnelle Bearbeitung von Rückfragen, Weiterverarbeitung der Antworten als maschinenlesbare Informationen.<br><br>
      <b>Funktionsweise:</b> Der:die Sachbearbeiter:in kann mit Hilfe einer strukturierten Nachrichten-Vorlage mehrere Auswahlantworten festlegen. Die Privatperson kann <b>genau eine</b> Antwortmöglichkeit auswählen.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Terminauswahl</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>4</b></td>
    <td width="320"><b>UI in Arbeit</b></td>
    <td width="280">
      <b>Übersicht:</b> Behörden bieten den Privatpersonen eine Auswahl an Terminen an.<br><br>
      <b>Mehrwert:</b> Den Terminvereinbarungsprozess vereinfachen und verbessern.<br><br>
      <b>Funktionsweise:</b> Die Terminverwaltung umfasst:<br>
      • Privatpersonen vereinbaren einen Termin mit einer Behörde;<br>
      • Privatpersonen sagen einen Termin bei einer Behörde ab;<br>
      • Privatpersonen verschieben einen Termin bei einer Behörde;<br>
      • Privatpersonen geben einen Zeitrahmen an, in dem sie verfügbar sind;<br>
      • Privatpersonen erhalten einen Zeitrahmen, in dem die Behörde verfügbar ist.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Weiterleitungslink</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>5</b></td>
    <td width="320">
    <img src="images/web_Weiterleitungslink.png" alt="Web Weiterleitungslink">
    </td>
    <td width="280">
      <b>Übersicht:</b> Behörden können in Nachrichten relevante Links einfügen. Links können als gut wahrnehmbarer Button dargestellt werden (siehe links: Button „Fortsetzung des Vorgangs").<br><br>
      <b>Mehrwert:</b> Für fachlich komplexe Anwendungsszenarien können Privatpersonen auf eine externe Webseite geleitet werden (z.B. den zur Antragstellung genutzten Onlinedienst).<br><br>
      <b>Funktionsweise:</b> Der Link führt zu einer Website, die in einem neuen Browser-Tab geöffnet wird.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Nachforderung von Dokumenten</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>6</b></td>
    <td width="320">
    <img src="images/web_Nachricht_mit_Anhängen.png" alt="Web Nachricht mit Anhängen">
    </td>
    <td width="280">
      <b>Übersicht:</b> Hochladen von Anhängen in den von ZaPuK geforderten Formaten. Per Konfiguration können zudem Einschränkungen festgelegt werden (z. B. über einen MIME-Typ-Filter).<br><br>
      <b>Mehrwert:</b> Es ermöglicht Fachbehörden, für den Verwaltungsvorgang relevante Unterlagen anzufordern.<br><br>
      <b>Funktionsweise:</b> Die folgenden Datenformate werden unterstützt: PDF, CSV, XML, JSON, PNG, TXT, ICS, iCal, HEIC, WebP.
    </td>
  </tr>
  <tr>
    <th width="30">#</th>
    <th width="320">Bescheide oder Gutachten zustellen</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>8</b></td>
    <td width="320"><b>UI in Arbeit</b></td>
    <td width="280">
      <b>Übersicht:</b> Zustellung von Bescheiden und Gutachten in rechtsgültiger und nachweisbarer Form. Privatpersonen können alle formellen Entscheidungen, die von verschiedenen zuständigen Behörden versandt werden, an einem Ort in der App einsehen (Tab „Dokumente").<br><br>
      <b>Mehrwert:</b> Stellt sicher, dass Bescheide an Privatpersonen versandt werden, wodurch gewährleistet ist, dass der Empfänger den Bescheid entweder erhält oder als erhalten gilt. Eine Bescheidzustellung erfolgt auf Rechtsgrundlage des § 9 OZG.<br><br>
      <b>Funktionsweise:</b> Die Nachricht zur Zustellung von Bescheiden kann Folgendes enthalten:<br>
      • Datum und Uhrzeit der Zustellung;<br>
      • Name und Unterschrift des Verfassers sowie den amtlichen Stempel der Fachbehörde;<br>
      • Aktenzeichen der Fachbehörde;<br>
      • Zeitgestempelte Empfangs- oder Ablehnungsbestätigung;<br>
      • Weitere spezifische maschinenlesbare Metadaten/Attribute des Fachverfahrens (von der Fachbehörde festzulegen).
    </td>
  </tr>
</table>

#### 4.2.3 Status-Übersicht

***Anwendungsfall:*** Ermöglicht Privatpersonen, den Status von Vorgängen über eine Webanwendung zu verfolgen.
Es gibt drei Akteure, deren Aktionen zu einer Statusänderung führen können: den Online-Dienst, der die Anfrage an die Fachbehörde sendet, die Fachbehörde, die den Nutzern Informationen zur Verfügung stellt, und den Nutzer, der der Fachbehörde Antworten und Fragen übermittelt.

- *User Story Privatperson:* Ich möchte kontinuierlich und in Echtzeit über den aktuellen Status meines Antrags informiert werden.
- *User Story Fachbehörde:* Ich möchte die antragstellende Privatperson in Echtzeit über erforderliche Aktivitäten informieren und über den Eingang von Bescheiden rechtzeitig informieren.

<table>
  <tr>
    <th width="30">#</th>
    <th width="320">Statuserklärung auf Zeitachse</th>
    <th width="280">Beschreibung</th>
  </tr>
  <tr>
    <td width="30"><b>1</b></td>
    <td width="320">
    <img src="images/web_Statuserklärung_auf_Zeitachse.png" alt="Web Statuserklärung auf Zeitachse">
    </td>
    <td width="280">
      <b>Übersicht:</b> Ermöglicht Privatpersonen und Behörden, den Status von Vorgängen in Echtzeit zu erkennen.<br><br>
      <b>Mehrwert:</b> Privatpersonen und Sachbearbeiter:innen können den Bearbeitungsstand eines Vorgangs einfach und in maschinenlesbarer Form nachverfolgen. Die Nutzer:innen erhalten in Echtzeit aktuelle Informationen zum Status der einzelnen Verwaltungsvorgänge.<br><br>
      <b>Funktionsweise:</b> Folgende Status sind für den Workflow des Verfahrens bereits definiert:<br>
      • Vorgang gesendet<br>
      • Vorgang eingegangen<br>
      • Vorgang in Prüfung<br>
      • In Bearbeitung<br>
      • Aktivität erforderlich<br>
      • Vorgang abgeschlossen
    </td>
  </tr>
</table>

### 4.3 Übergreifende Funktionalitäten
#### 4.3.1 Bedarfsgerechte Einschränkung von Kommunikation
***Anwendungsfall:*** Der:die Sachbearbeiter:in muss die bidirektionale Kommunikation über die mobile Anwendung bei Bedarf deaktivieren können.
Dies kann insbesondere nach Zustellung eines Bescheids erforderlich sein, um zu verhindern, dass Privatpersonen für neue Anträge veraltete Kommunikationskanäle nutzen.
Je nach Fachverfahren kann es zudem notwendig sein, die Kommunikation auf maschinenlesbare Nachrichten zu beschränken (die Beantwortung von Multiple-Choice-Rückfragen, der Upload von Dokumenten oder die Terminvereinbarung ist noch erlaubt, das Senden von Freitext-Nachrichten jedoch nicht).
Sachbearbeiter:innen sind berechtigt, die bidirektionale Kommunikation zu deaktivieren.

Privatpersonen können über die bereitgestellte Infrastruktur nicht untereinander kommunizieren.

- *User Story Privatperson:* Ich möchte sicherstellen, dass mir andere Privatpersonen keine privaten Nachrichten senden können.
- *User Story Fachbehörde:* Ich möchte bidirektionale Kommunikationskanäle nach Zustellung des Bescheids oder nach Abschluss des Verwaltungsverfahrens schließen oder, je nach Anforderung des Fachverfahrens, entsprechend konfigurieren oder beschränken können.

#### 4.3.2 Web-zu-Mobil-Cross-Signing

***Anwendungsfall:*** Privatpersonen melden sich einmalig in der Smartphone-App an und übertragen ihre Sitzung anschließend nahtlos ohne erneute Eingabe von Zugangsdaten an die Webanwendung.

- *User Story Privatperson:* Ich möchte mich einmalig mittels Personalausweis (eID-Karte) am Smartphone authentifizieren, um meine Identität für alle meine Sitzungen in den Neo-Web- und Smartphone-Anwendungen zu bestätigen.
- *User Story Fachbehörde:* -

## 5 Integrationsansatz
### 5.1 Integration der Antragsfolgekommunikation in den Antragsprozess

Die Antragsfolgenkommunikation wird weiterhin durch den etablierten Antragsprozess über Onlinedienste bzw. Verwaltungsportale initiiert.
Beim Login über die BundID erhält der Onlinedienst das *postkorbHandle* der Privatperson (auch als „legacyPostkorbHandle" bezeichnet).
Nach der Authentifizierung schließt die Privatperson das Anlegen des Antrags über einen Onlinedienst ab.
Dabei wird das *postkorbHandle* des Postfachs über FIT-Connect an die zuständige Fachbehörde übermittelt und zur Adressierung sowie Zustellung von Nachrichten verwendet.

<img src="images/abbildung_4_client-fit-connect-matrix-fachbehoerde.png" alt="Abbildung: FIT-Connect-Matrix-Fachbehörde">

Im FIT-Connect-SDK wird die Matrix-Adresse der Privatperson wie folgt aus dem *postkorbHandle* gebildet:

<div align="center">

`Matrix-ID: @<PostkorbHandle>:ess1.dev.neo.fitko.dev`

</div>

Nach der Annahme und Bearbeitung des Antrags durch eine Sachbearbeiter:in stellt die Neo-Infrastruktur, die im Wesentlichen aus zwei Matrix-Homeservern (ess1 und ess2) besteht, einen verschlüsselten und pseudonymisierten Rückkanal für die Antwort bereit.

### 5.3 Einfache Integration in Fachverfahren
Um die Kommunikation zwischen den für die Fachverfahren zuständigen Behörden und Privatpersonen zu ermöglichen, wurde das FIT-Connect SDK um ein Modul zur Einbindung der Matrix-Infrastruktur erweitert.
Dadurch können Hersteller von Fachverfahren Nachrichten nahtlos mit Privatpersonen austauschen.
Während FIT-Connect drei SDKs für Java, JavaScript und .NET bereitstellt, fokussiert sich das Neo-Vorhaben zum aktuellen Zeitpunkt zunächst ausschließlich auf die Breitstellung eines Moduls für das FIT-Connect-Java-SDK.
Die gesamte Dokumentation wird denselben Umfang und dieselbe Struktur wie die aktuelle FIT-Connect SDK-Dokumentation einhalten.

Auch wenn die Antragstellung nicht über FIT-Connect erfolgt, kann die bidirektionale Antragsfolgekommunikation in Fachverfahren über das entsprechende Modul des FIT-Connect-SDK realisiert werden.

Im aktuellen Neo-Projektstand ist es bereits möglich, ein Fachverfahren über die Entwicklungsumgebung an die bidirektionale Kommunikationslösung anzubinden.

Das **SDK** wird in FITKO-GitLab entwickelt: https://git.fitko.de/fit-connect/java-sdk/-/tree/neo/matrix-sdk-prototype/matrix

Das neue modulare FIT-Connect-SDK basiert auf dem bestehenden FIT-Connect SDK und enthält weitere zusätzliche Module (wie z.B. das Neo/Matrix-Modul).

### 5.4 Überblick Neo-DEV-Umgebung

Wir testen zunächst auf der DEV-Umgebung.

Die DEV-Umgebung besteht aus zwei Matrix-Servern: ess1 und ess2.

<img src="images/abbildung_6_Matrix-Servern-ess1_und_ess2.png" alt="Abbildung: Matrix-Servern ess1 undess2">

Beide Matrix-Server basieren auf der [Element Server Suite (ESS)](https://github.com/element-hq/ess-helm).

**ess1** ist der Matrix-Server für **Privatpersonen**.

- Admin-Oberfläche: https://admin-ess1.dev.neo.fitko.dev/
- Element Web Client (für Test- und Debug-Zwecke): https://element-ess1.dev.neo.fitko.dev/
- Neo Webanwendung: https://webapp.dev.neo.fitko.dev/
- iOS-App via Testflight: Bitte wendet euch an Milton im Matrix-Chat. Milton kann euch zu Testflight hinzufügen (Milton spricht nur Englisch).
- Matrix-Homeserver (synapse): `ess1.dev.neo.fitko.dev`
- **Authentifizierung**: ENTWEDER über manuell erstellte ess1-Accounts (via Janne) ODER über die Integrationsumgebung der BundID (int.id.bund.de).

**ess2** ist der Matrix-Server für **Fachbehörden**.

- An diesen Server bindet ihr eure Systeme / Fachverfahren an.
- Admin-Oberfläche: https://admin-ess2.dev.neo.fitko.dev/
- Element Web Client (für Test- und Debug-Zwecke): https://element-ess2.dev.neo.fitko.dev/
- Matrix-Homeserver (synapse): `ess2.dev.neo.fitko.dev`
- **Authentifizierung** der angebundenen Fachverfahren: über eigenständige ess2-Accounts (von Janne erstellt).

### 5.5 Login in Element Web auf ess1

Beim Login in Element Web auf ess1 (https://element-ess1.dev.neo.fitko.dev/) könnt ihr euch im Matrix Authentication Service (MAS) entweder direkt mit einem ess1-Account einloggen oder euch über einen BundID-Account (BundID-Integrationsumgebung, https://int.id.bund.de/) einloggen.
Aktuell seht ihr hierbei noch den zwischengeschalteten Keycloak.
MAS und Keycloak werden später automatisch auf die BundID weiterleiten, sodass ihr beide Login-Masken nicht mehr sehen werdet ([Issue #513](https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/issues/513)).

<img src="images/abbildung_7_login-maske.png" alt="Abbildung: Login Maske">

### 5.6 Login in Neo-Webclient auf ess1

Start: https://webapp.dev.neo.fitko.dev/

### 5.7 Login in die Smartphone-App auf ess1

Die über Testflight verfügbare iOS-App unterstützt momentan den Login per ess1-Account oder per BundID-Account auf der BundID-Integrationsumgebung.

## 6 Checkliste zur Anbindung an die Neo-Infrastruktur

Diese Checkliste umfasst 7 Schritte zur Neuanmeldung bei Neo:

### Schritt 1 – Fachliche Definition des Use-Cases

- [ ] Definieren Sie, über welches System Privatpersonen einen initialen Antrag stellen (Onlinedienst).
- [ ] Definieren Sie, welches IT-System zur Antragsbearbeitung genutzt wird (Fachverfahren).
- [ ] Stimmen Sie anhand des in Kapitel 4 beschriebenen Funktionsumfangs ab, welche Kommunikation zwischen den Sachbearbeiter:innen in Ihrer Fachbehörde und den Privatpersonen erfolgen soll.

### Schritt 2 – Voraussetzungen & Kontozugang

- [ ] Um mit unserem Projektteam in Kontakt zu treten, erstellen Sie einen Matrix-Account: https://matrix.org/try-matrix/
- [ ] Treten Sie unserem Matrix-Chat für die weitere Kommunikation bei: https://matrix.to/#/#neo-pilots:tchncs.de
- [ ] Ein ess1-Konto für Tests auf Privatpersonenseite einrichten (*über Janne Koschinski*). Dies ist erforderlich, um sich beim Neo-Webclient und der iOS-App anzumelden.
- [ ] Ein ess2-Konto für Ihr Fachverfahren einrichten (*über Janne Koschinski*). Dies ist erforderlich, um Ihr System mit dem Server für Fachbehörden zu verbinden.

### Schritt 3 – Einblick in die DEV-Umgebung

Die DEV-Umgebung besteht aus zwei Matrix-Servern, die beide auf der Element Server Suite (ESS) laufen:

- [ ] Überprüfen Sie, ob über die folgenden Links auf ess1 zugegriffen werden kann.

  **ess1** (für Bürger/Endnutzer):
  - Element-Webclient (Testen/Debugging): https://element-ess1.dev.neo.fitko.dev/
  - Neo-Webclient: https://webapp.dev.neo.fitko.dev/
  - Homeserver: `ess1.dev.neo.fitko.dev`

- [ ] Überprüfen Sie, ob über die folgenden Links auf ess2 zugegriffen werden kann.

  **ess2** (für Fachbehörden – hier das Fachverfahren verbinden):
  - Element-Webclient (Testen/Debugging): https://element-ess2.dev.neo.fitko.dev/
  - Homeserver: `ess2.dev.neo.fitko.dev`

### Schritt 4 – Anmeldung bei Element Web (ess1)

- [ ] Element Web öffnen (https://element-ess1.dev.neo.fitko.dev/) und sich mit einem manuellen ess1-Konto (*über Janne*) oder einem BundID-Konto über die Integrationsumgebung (https://int.id.bund.de/) anmelden.

  > **Hinweis:** Bei Verwendung der BundID wird Keycloak noch als Zwischenstufe angezeigt. Dies wird in [Issue #513](https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/work_items/513) entfernt.

### Schritt 5 – Anmeldung im Neo-Webclient (ess1)

- [ ] Neo-Webclient öffnen (https://webapp.dev.neo.fitko.dev/) und sich mit Ihrem ess1-Konto oder BundID-Konto (BundID-Integrationsumgebung) anmelden.

### Schritt 6 – Anmeldung in der iOS-App (optional)

- [ ] Matrix-Chat nutzen (https://matrix.to/#/#neo-pilots:tchncs.de, nur Englisch), um über Testflight zur iOS-Beta hinzugefügt zu werden (*über Milton*).
- [ ] iOS-App installieren und mit Ihrem ess1-Account oder BundID-Integration-Environment-Account anmelden.

### Schritt 7 – Anbindung des Fachverfahrens

- [ ] Das FIT-Connect Meta-SDK auf Ihrem Rechner einrichten. Das modulare FIT-Connect-SDK einschließlich des Neo/Matrix-Moduls aus dem GitLab-Repository klonen und konfigurieren: https://git.fitko.de/fit-connect/java-sdk/-/tree/neo/matrix-sdk-prototype/matrix
- [ ] Das Fachverfahren über Ihr ess2-Konto mit dem unter `ess2.dev.neo.fitko.dev` betriebenen Matrix-Homeserver mithilfe des SDK verbinden.

-----

## Anhang A1: iOS-Geschäftsprozess (mit FIT-Connect Zustelldienst)

![Anhang A1: iOS-Geschäftsprozess (mit FIT-Connect-Zustelldienstes)](images/Anhang_A1-iOS-Geschaeftsprozess_(mit_FIT-Connect_Zustelldienstes).svg)

## Anhang A2: iOS-Geschäftsprozess (ohne FIT-Connect Zustelldienst)

![Anhang A1: iOS-Geschäftsprozess (ohne FIT-Connect-Zustelldienstes)](images/Anhang_A2-iOS-Geschaeftsprozess_(ohne_FIT-Connect_Zustelldienstes).svg)

## Anhang B1: Web-Geschäftsprozess (mit FIT-Connect Zustelldienst)

![Anhang B1: Web-Geschäftsprozess (mit FIT-Connect-Zustelldienstes)](images/Anhang_B1-Web-Geschaeftsprozess_(mit_FIT-Connect_Zustelldienstes).svg)

## Anhang B2: Web-Geschäftsprozess (ohne FIT-Connect Zustelldienst)

![Anhang B2: Web-Geschäftsprozess (ohne FIT-Connect-Zustelldienstes)](images/Anhang_B2-Web-Geschaeftsprozess_(ohne_FIT-Connect_Zustelldienstes).svg)


[^1]: https://testflight.apple.com/