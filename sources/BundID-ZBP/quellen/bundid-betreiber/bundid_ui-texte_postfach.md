# BundID – UI-Texte zu Postfach, Statusmonitor und Antwort-Upload (Nutzerkonto)

Quelle: https://id.bund.de/ (Textinhalte aus dem CMS-Sprachbundle https://id.bund.de/cms/assets/de.u87bk8e7.js, Abschnitt B5.postkorb; die Website ist eine JavaScript-Anwendung, Text daher aus dem Bundle extrahiert)
Abgerufen: 2026-11-17

- `.tabName`: Postfach

### Postfach

Hier können Sie Ihre Nachrichten lesen und verwalten. Nachrichten, die Sie mit Ihrem aktuellen Zugang nicht öffnen können, sind in der Suche, Filterung und Sortierung nicht enthalten.

- `.timestamp`: {date}, {time}
{numberOfMessages} Nachricht | {numberOfMessages} Nachrichten

- `.messageCounterNew`: {numberOfMessages} neu (ungelesen)
- `.ribbonUnreadMessage`: NEU
Sie haben nach §28 Verwaltungsverfahrensgesetz (VwVfG) das Recht, sich zu Ihrem Anliegen zu äußern. Nutzen Sie dafür gerne die angebotenen Kommunikationswege des Senders. Mehr Informationen finden Sie unter Hilfe .

- `.emptyTable.disclaimer`: KEINE NACHRICHTEN IM POSTFACH GEFUNDEN.
- `.emptyTable.text`: Sobald Sie neue Nachrichten erhalten, erscheinen diese hier.
Ihr Postfach ist aktuell aufgrund von Wartungsarbeiten nicht erreichbar.

- `.maintenance.disclaimerError`: Ihr Postfach ist aktuell nicht erreichbar.
- `.maintenance.text`: Bitte versuchen Sie es später noch einmal.
Diese Nachrichten enthalten sensible Daten. Um sie zu öffnen, müssen Sie sich mit einem anderen Zugang (z. B. Ihrem Online-Ausweis) anmelden.

- `.confidentialMessage.button`: MIT ANDEREM ZUGANG ANMELDEN
- `.searchSection.filterInput`: IN NACHRICHTEN SUCHEN
0 AUSGEWÄHLTE NACHRICHTEN LÖSCHEN | 1 AUSGEWÄHLTE NACHRICHT LÖSCHEN | {n} AUSGEWÄHLTE NACHRICHTEN LÖSCHEN


### SORTIEREN

- `.sort.options.dateDesc`: Änderungsdatum absteigend
- `.sort.options.dateAsc`: Änderungsdatum aufsteigend

### Antragsbezeichnung


### Nach Absender

- `.filter.options.read`: Gelesen
- `.filter.options.unread`: Neu (Ungelesen)
- `.filter.buttons.open`: Filter
- `.filter.buttons.delete`: FILTER LÖSCHEN
- `.filter.buttons.apply`: ANWENDEN
- `.list.statusAnswerRequiered`: Antwort erforderlich
Bis einschließlich { acceptedUntil } können Sie antworten. Danach ist keine Antwort mehr möglich.

Seitens der Verwaltung wurde eine Anfrage an Sie gestellt, die eine Antwort erfordert.

- `.list.statusAnswered`: Geantwortet
- `.list.statusAnsweredInfo`: Sie haben auf die Anfrage der Verwaltung geantwortet.
- `.list.replyTimeExpired`: Antwort verpasst
Seitens der Verwaltung wurde eine Anfrage mit Antwortfrist an Sie gestellt, ohne dass Sie innerhalb der vorgegebenen Frist geantwortet haben.

Sie haben die Antwortfrist mit Stichtag { acceptedUntil } verpasst und können deshalb keine Antwort mehr abgeben.

- `.list.closedByAuthority`: Anfrage geschlossen
Die Antwortmöglichkeit wurde seitens der Verwaltung geschlossen. Die Gründe erfragen Sie bitte bei der Verwaltung.

- `.list.closedByAuthorityExplanation`: Die Anfrage wurde durch die Verwaltung geschlossen.
- `.list.statusInformation`: Der Status der Nachricht hat folgende Bedeutung:
- `.antwortCount`: Letzte Antwort

### Nachricht wirklich löschen?


### Sind Sie sicher, dass Sie diese Nachricht löschen möchten?

Bitte beachten Sie, dass nach dem Löschen kein Zugriff mehr auf den Inhalt der Nachricht besteht.

- `.modalDeleteOne.disclaimerText1`: HINWEIS:
Denken Sie daran, die Nachricht über die Funktion „Drucken als PDF“ zu speichern, um einen Nachweis für Ihre Unterlagen zu behalten.

- `.modalDeleteOne.disclaimerText2`: WICHTIG:
Durch das Löschen der Nachricht kann auch die Möglichkeit zur Antwort oder weiteren Kommunikation mit dem Empfänger entfallen.

- `.modalDeleteOne.buttonCancel`: ABBRECHEN
- `.modalDeleteOne.buttonConfirm`: NACHRICHT ENDGÜLTIG LÖSCHEN

### Möchten Sie diese Nachricht wirklich endgültig löschen? | Möchten Sie diese Nachrichten wirklich endgültig löschen?

Die Nachricht wird nicht archiviert und kann nicht wiederhergestellt werden. | Die Nachrichten werden nicht archiviert und können nicht wiederhergestellt werden.

Die Nachricht {betreffliste} kann aktuell nicht gelöscht werden. | Die Nachrichten {betreffliste} können aktuell nicht gelöscht werden.

Die von Ihnen gesendete Antwort befindet sich derzeit noch in Übermittlung. Bitte versuchen Sie zu einem späteren Zeitpunkt erneut, die Nachricht zu löschen. | Die von Ihnen gesendete Antwort befindet sich derzeit noch in Übermittlung. Bitte versuchen Sie zu einem späteren Zeitpunkt erneut, die Nachricht zu löschen. | Die von Ihnen gesendeten Antworten befinden sich derzeit noch in Übermittlung. Bitte versuchen Sie zu einem späteren Zeitpunkt erneut, die Nachricht zu löschen.

- `.modalDeletionNotPossible.buttonCancel`: ABBRECHEN
- `.modalDeletionNotPossible.buttonDeleteOthers`: Zulässige Nachricht löschen | Zulässige Nachrichten löschen

### Ihre Nachricht

- `.details.noSubject`: Kein Betreff
- `.details.downloadButton`: HERUNTERLADEN (PDF)
- `.details.printButton`: DRUCKEN
- `.details.deleteButton`: NACHRICHT LÖSCHEN
- `.details.backtoallButton`: ZURÜCK ZU ALLEN NACHRICHTEN
- `.details.prevmessageButton`: Vorherige Nachricht
- `.details.nextMessageButton`: Nächste Nachricht
- `.details.anfrage.replyButton`: JETZT BEANTWORTEN
- `.details.anfrage.deleteButton`: NACHRICHT LÖSCHEN

### Gesendete Antwort | Gesendete Antworten


### { number } Antwort | { number } Antworten


### Ihre Antwort

- `.details.anfrage.reply.text1`: Mit * markierte Felder sind Pflichtfelder
- `.details.anfrage.reply.textAreaLabel`: Ihre Antwort*
- `.details.anfrage.reply.textAreaPlaceholder`: Verfassen Sie hier Ihre Nachricht.
- `.details.anfrage.reply.textAreaCounter`: { numberMin } Zeichen / max. { numberMax }

### DOKUMENTEN-UPLOAD

- `.details.anfrage.reply.upload.text1`: Formulare, Dokumente und Bilddateien auf dieses Feld ziehen
- `.details.anfrage.reply.upload.text2`: oder
- `.details.anfrage.reply.upload.uploadButton`: DATEI AUSWÄHLEN
- `.details.anfrage.reply.upload.text3`: Folgende Dateiformate können hochgeladen werden:
- `.details.anfrage.reply.upload.text4`: Maximale Dateigröße einzelner Dokumente:
- `.details.anfrage.reply.upload.text5`: Maximale Dateigröße in Summe:
- `.details.anfrage.reply.upload.uploadInProgressText`: Wartet auf Dokument

### IHRE UPLOADS

- `.details.anfrage.reply.upload.deleteAttachmentButton`: ENTFERNEN
- `.details.anfrage.reply.upload.attachmentCounter`: { min } von max. { max } Dateien
- `.details.anfrage.reply.sendButton`: ANTWORT SENDEN

### Antwort wirklich versenden?

Bitte stellen Sie sicher, dass Ihre Antwort vollständig ist. Haben Sie den Nachrichtentext wie gewünscht verfasst und – falls notwendig – alle relevanten Dateien angehängt?

Sie können Ihre Eingabe jetzt noch einmal überprüfen oder direkt absenden.

- `.details.anfrage.reply.confirmationDialog.cancelButton`: ABBRECHEN
- `.details.anfrage.reply.confirmationDialog.sendButton`: ANTWORT SENDEN
- `.details.anfrage.reply.loadingText`: Ihre Antwort wird gesendet
- `.details.anfrage.reply.successField.top`: NACHRICHT ÜBERMITTELT

### Antwort versendet

- `.details.anfrage.reply.successField.text`: Ihre Nachricht wurde an die Behörde übermittelt

### Es ist ein Fehler aufgetreten

Ihre Antwort konnte nicht gesendet werden. Bitte verfassen Sie die Nachricht erneut.

- `.details.anfrage.reply.errorField.button`: NEUE ANTWORT ERSTELLEN

### ANHÄNGE

- `.details.attachments.downloadAllButton`: ALLE ANHÄNGE HERUNTERLADEN (ZIP)
- `.details.attachments.description`: { contenttype } {'|'} { dateigroesse }
- `.details.attachments.ariaLabelDownloadAttachment`: Anhang {filename} herunterladen

## Abschnitt B5.statusMonitor

- `.tabName`: Anträge

### Anträge

Hier können Sie sich über Ihre erstellten Anträge informieren. Anträge, die Sie mit Ihrem aktuellen Zugang nicht öffnen können, sind in der Suche, Filterung und Sortierung nicht enthalten.

- `.messageCounter`: {numberOfMessages} Anträge
- `.activityStatus`: {numberOfActivities} Aktivität erforderlich
- `.maintenance.disclaimerError`: Ihre Antragsübersicht ist aktuell nicht erreichbar.
- `.maintenance.text`: Bitte versuchen Sie es später erneut.

### Antragsdetails


### Gesamter Verlauf


### Verlauf

- `.antragDetails.accordionTitleCurrentStatus`: Aktuell:
- `.antragDetails.info`: Info
- `.antragDetails.backToAllButton`: ZURÜCK ZU ALLEN ANTRÄGEN
- `.antragDetails.prevAntragButton`: Vorheriger Antrag
- `.antragDetails.nextAntragButton`: Nächster Antrag
- `.antragDetails.moreAntraege`: Weitere Anträge
- `.antragDetails.dateTime`: Datum / Uhrzeit
- `.antragDetails.attributes.createDate`: Erstellungsdatum
- `.antragDetails.attributes.updatedDate`: Letzte Änderung
- `.antragDetails.attributes.sender`: Sender
- `.antragDetails.attributes.reference`: Referenz
- `.antragDetails.attributes.status`: Status
- `.antragDetails.attributes.statusDetails`: Details zum Status
- `.antragDetails.attributes.additionalInformation`: Weitere Informationen
- `.antragDetails.buttonDownloadOverview`: ÜBERSICHT HERUNTERLADEN
- `.antragDetails.buttonDelete`: ANTRAG ENTFERNEN
Diese Anträge enthalten sensible Daten. Um sie zu öffnen, müssen Sie sich mit einem anderen Zugang (z. B. mit Ihrem Online-Ausweis) anmelden.

- `.confidentialMessage.button`: MIT ANDEREM ZUGANG ANMELDEN
0 AUSGEWÄHLTE ANTRÄGE LÖSCHEN | 1 AUSGEWÄHLTEN ANTRAG LÖSCHEN | {n} AUSGEWÄHLTE ANTRÄGE LÖSCHEN

- `.searchSection.filterInput`: NACH BEZEICHNUNGEN EINES ANTRAGS SUCHEN
- `.filter.buttons.open`: Filter
- `.statusStates.INITIATED`: Antrag in Erstellung
- `.statusStates.SUBMITTED`: Antrag versendet
- `.statusStates.RECEIVED`: Antrag eingegangen
- `.statusStates.PROCESSING`: Antrag in Prüfung
- `.statusStates.ACTION_REQUIRED`: Aktivität erforderlich
- `.statusStates.COMPLETED`: Antrag abgeschlossen
- `.requestDeletion.alert`: ACHTUNG

### Möchten Sie Ihren abgeschlossenen Antrag wirklich aus der Übersicht entfernen?

Die Übersicht des Antrags und die zugehörigen Statusmeldungen können nicht wiederhergestellt werden.

HINWEIS: Der Antrag selbst ist davon nicht betroffen. Er besteht weiterhin und Sie entfernen nur die Daten und Statusmeldungen aus Ihrer Übersicht. Wie Sie eine zuständige Behörde kontaktieren können, bei der Sie einen Antrag gestellt haben, erfahren Sie in den FAQs.

- `.requestDeletion.buttonCancel`: ABBRECHEN
- `.requestDeletion.buttonDelete`: ANTRAG LÖSCHEN
Das Entfernen von Anträgen, die sich noch in der Bearbeitung befinden, ist leider nicht möglich.

- `.requestDeletionManyApplications.alert`: ACHTUNG

### Möchten Sie Ihre abgeschlossenen Anträge wirklich aus der Übersicht entfernen?

Die Übersichten der Anträge und die zugehörigen Statusmeldungen können nicht wiederhergestellt werden.

HINWEIS: Die Anträge selbst sind davon nicht betroffen. Sie bestehen weiterhin und Sie entfernen nur die Daten und Statusmeldungen aus Ihrer Übersicht. Wie Sie die zuständigen Behörden kontaktieren können, bei denen Sie Anträge gestellt haben, erfahren Sie in den FAQs.

- `.requestDeletionManyApplications.buttonCancel`: ABBRECHEN
- `.requestDeletionManyApplications.buttonDelete`: ANTRÄGE LÖSCHEN
Jeder Antrag hat eine individuelle Zeichenkombination, die Referenz genannt wird und die der Behörde bei der Zuordnung von Anträgen hilft. Sollen Sie mit der Behörde in Kontakt treten müssen, geben Sie immer die Referenz des Antrags an.

Jede Nachricht hat eine individuelle Zeichenkombination, die Referenz genannt wird und die der Behörde bei der Zuordnung von Anträgen hilft. Sollten Sie mit der Behörde in Kontakt treten, geben Sie immer die Referenz der Nachricht an.

- `.antragsstatusInfo`: Ihr Antragsstatus:

### Die Schritte Ihrer Antragsstellung haben folgende Bedeutung:

Antrag in Erstellung - Sie haben damit begonnen, einen Antrag auszufüllen. Dieser wurde jedoch noch nicht an die Behörde übermittelt und kann noch bearbeitet werden.

Antrag versendet - Sie haben den Antrag ausgefüllt. Der Antrag wird nun an die Behörde übermittelt.

Antrag eingegangen - Ihr Antrag ist bei der Behörde eingegangen.

Antrag in Prüfung - Ihr Antrag wird bei der Behörde geprüft und bearbeitet. Anschließend erhalten Sie eine Rückmeldung zu Ihrem Antrag.

Aktivität erforderlich - Wenn dieser Schritt angezeigt wird, wird Ihre Aktivität benötigt. Mehr Informationen finden Sie in Ihrem Postfach.

Antrag abgeschlossen - Der Antrag wurde erfolgreich abgeschlossen. Sie finden alle "Dokumente zu Ihrem Antrag" auf der gleichnamigen Rubrik weiter unten auf der Seite.


## Abschnitt B0.validation.fileUpload


### Upload fehlgeschlagen

Dieses Dateiformat wird nicht unterstützt. Bitte verwenden Sie ein zugelassenes Format ({attachmentTypes}).

- `.type.text2`: Bitte laden Sie eine Datei im unterstützten Format hoch.

### Upload fehlgeschlagen

Die hochgeladene Datei übersteigt die maximal zulässige Größe von {maxAttachmentSize}.

Bitte wählen Sie eine kleinere Datei aus oder reduzieren Sie die Dateigröße.


### Upload fehlgeschlagen

Der ausgewählte Dateiname ist zu lang. Maximale Länge beträgt {maxFileNameLength} Zeichen.

- `.nameLength.text2`: Bitte wählen Sie einen kürzeren Dateinamen.

### Upload fehlgeschlagen

- `.nameInvalidCharacters.text1`: Der ausgewählte Dateiname enthält ungültige Zeichen.
Bitte verwenden Sie nur Buchstaben, Zahlen, Bindestriche oder Unterstriche.


### Upload fehlgeschlagen

Die ausgewählten Dateien überschreiten die maximal zulässige Gesamtgröße von {maxTotalAttachmentSize}.

Bitte entfernen Sie einige Dateien oder reduzieren Sie deren Größe.


### Upload fehlgeschlagen

Die Anzahl der Anhänge überschreitet die zulässige Obergrenze von {maxAttachmentCount}.

- `.attachmentCount.text2`: Bitte reduzieren Sie die Anzahl der hochgeladenen Dateien.

### Upload fehlgeschlagen

Sie haben bereits eine Datei mit dem Namen {name} für Ihre Antwortnachricht hochgeladen.

Bitte benennen Sie die Datei um oder prüfen Sie, ob das Dokument bereits angehängt wurde.


### Upload fehlgeschlagen

Es wurde schadhafter Code in der von Ihnen hochgeladenen Datei entdeckt.

- `.malwareDetected.text2`: Die Datei konnte nicht erfolgreich hochgeladen werden.

## Abschnitt B0.validation.replyAnswer

- `.message.minLength`: Ihre Nachricht muss mindestens {min} Zeichen enthalten.
- `.message.maxLength`: Ihre Nachricht darf nicht mehr als {max} Zeichen enthalten.
- `.message.required`: Bitte geben Sie eine Nachricht an.
- `.message.pattern`: Ihre Nachricht enthält unzulässige Zeichen.

## Abschnitt B1.zbp


### Ihr Postfach ist aktuell nicht erreichbar.

- `NotReachableError.text`: Bitte versuchen Sie es später erneut.

## Abschnitt B1.VAL_400_07


### The mandantId should be between 1 and 255 characters


### The dienstId is a required field


### The dienstId should be between 1 and 255 characters


### The postkorbHandleId is a required field


### The postkorbHandleId should be between 1 and 60 characters


### The content is a required field


### The content should be at least 1 character


### The storkQaaLevel is a required field


### Failed to insert the attachment


### The kontoId is a required field


## Abschnitt B3.landingFachverfahren.guestLogin


### GEHT ES AUCH OHNE BUNDID-KONTO?

Wenn Sie kein Konto erstellen möchten, können Sie sich als Gast anmelden . In diesem Fall wird für Sie kein BundID-Konto erstellt und kein Postfach angelegt.


## Abschnitt B4.details.benutzername.warndDialog


### Hinweis

Wenn Sie sich mit Benutzername und Passwort anmelden, können Sie eventuell nicht alle Bescheide und Nachrichten in Ihrem Postfach lesen. Dies ist je nach Online-Antrag und Sensibilität der Daten verschieden.

Wir empfehlen die Anmeldung mit einem Online-Ausweis. Damit können Sie immer alle Nachrichten lesen.

- `.closeButton`: WEITER MIT BENUTZERNAME UND PASSWORT
