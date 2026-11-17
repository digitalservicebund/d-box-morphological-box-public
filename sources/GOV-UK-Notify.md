# GOV.UK Notify (UK): Quellen

Nachweise für die Spalte `UK (kein zentrales Postfach)` in `existing_solutions.csv` und die GOV.UK-Notify-Einträge in `morph_box.csv`. Einträge mit `[unverifiziert]` sind in keiner der Quellen ausdrücklich belegt.

## Hauptquellen

| Kürzel | Quelle |
|---|---|
| FEAT | GOV.UK Notify, Features: <https://www.notifications.service.gov.uk/features> |
| WHO | GOV.UK Notify, Who can use Notify: <https://www.notifications.service.gov.uk/features/who-can-use-notify> |
| SEC | GOV.UK Notify, Security features: <https://www.notifications.service.gov.uk/features/security> |
| PERF | GOV.UK Notify, Performance: <https://www.notifications.service.gov.uk/features/performance> |
| PRICE | GOV.UK Notify, Pricing: <https://www.notifications.service.gov.uk/pricing> |
| PRICE-SMS | GOV.UK Notify, Text message pricing: <https://www.notifications.service.gov.uk/pricing/text-messages> |
| PRICE-L | GOV.UK Notify, Letter pricing: <https://www.notifications.service.gov.uk/pricing/letters> |
| API | GOV.UK Notify, API documentation (Client-Bibliotheken): <https://www.notifications.service.gov.uk/using-notify/api-documentation> |
| REST | GOV.UK Notify, REST API documentation: <https://docs.notifications.service.gov.uk/rest-api.html> |
| TPL | GOV.UK Notify, Templates: <https://www.notifications.service.gov.uk/using-notify/templates> |
| PERS | GOV.UK Notify, Personalisation: <https://www.notifications.service.gov.uk/using-notify/personalisation> |
| BULK | GOV.UK Notify, Bulk sending: <https://www.notifications.service.gov.uk/using-notify/bulk-sending> |
| FILE | GOV.UK Notify, Send files by email: <https://www.notifications.service.gov.uk/using-notify/send-files-by-email> |
| LETTER | GOV.UK Notify, Upload a letter: <https://www.notifications.service.gov.uk/using-notify/upload-a-letter> |
| RX-SMS | GOV.UK Notify, Receive text messages: <https://www.notifications.service.gov.uk/using-notify/receive-text-messages> |
| REPLY | GOV.UK Notify, Reply-to email address: <https://www.notifications.service.gov.uk/using-notify/reply-to-email-address> |
| SENDER | GOV.UK Notify, Text message sender ID: <https://www.notifications.service.gov.uk/using-notify/text-message-sender> |
| DEL | GOV.UK Notify, Delivery times: <https://www.notifications.service.gov.uk/using-notify/delivery-times> |
| STATUS | GOV.UK Notify, Message status: <https://www.notifications.service.gov.uk/using-notify/message-status/email> |
| RET | GOV.UK Notify, Data retention period: <https://www.notifications.service.gov.uk/using-notify/data-retention-period> |
| TEAM | GOV.UK Notify, Team members and permissions: <https://www.notifications.service.gov.uk/using-notify/team-members-and-permissions> |
| UNSUB | GOV.UK Notify, Unsubscribe links: <https://www.notifications.service.gov.uk/using-notify/unsubscribe-links> |
| TOU | GOV.UK Notify, Terms of use: <https://www.notifications.service.gov.uk/terms-of-use> |
| GH | Quellcode GOV.UK Notify (GDS): <https://github.com/alphagov/notifications-api> |
| HMRC | GOV.UK, Personal tax account: <https://www.gov.uk/personal-tax-account> |

## Nachweise je Dimension

| Zeile | Aussage | Quellen |
|---|---|---|
| B1 | REST-API mit API-Schlüssel als JSON Web Token; Client-Bibliotheken für Java, .NET, Node.js, PHP, Python, Ruby; Callbacks für Zustellstatus und eingehende SMS | API; REST; SEC |
| B2 | Web-Oberfläche mit Vorlagen, Einzel- und Massenversand per Tabellen-Upload; E-Mail, SMS und Brief aus einer Plattform; Notify druckt und versendet Briefe | FEAT; TPL; BULK |
| B3 | Behörde adressiert mit eigenen Kontaktdaten (E-Mail-Adresse, Mobilnummer, Postanschrift); kein zentrales Verzeichnis | REST; BULK |
| B4 | Vorlagen mit Platzhaltern; Dateien als verschlüsselter Download-Link statt Anhang, Formate, bis 2 MB, 1 bis 78 Wochen abrufbar; Briefe aus Vorlage oder PDF | PERS; FILE; REST; LETTER |
| P1 | Nachricht selbst per E-Mail oder SMS mit Inhalt, auch sensible Platzhalter wie Sicherheitscodes; Brief per Royal Mail; Kanalwahl durch die Behörde | FEAT; PERS; DEL |
| P2 | kein Postfach; optional Bestätigung der E-Mail-Adresse vor Datei-Download (`confirm_email_before_download`) | FILE; REST |
| P3, P4, P9 | kein Postfach und kein Nutzerkonto für Empfangende; keine Vertretungsfunktion | FEAT; WHO („not for personal use“) |
| P5 | Empfang in eigenem E-Mail-Programm, SMS-App und Papierpost; Konten einzelner Verwaltungszweige außerhalb von Notify, z. B. HMRC personal tax account | FEAT; HMRC |
| P6 | keine Schnittstelle für Empfangende [unverifiziert, folgt aus dem Fehlen eines Postfachs] | FEAT |
| P7, P8 | Antwort und anlasslose SMS an eindeutige Rufnummer, im Dashboard oder per API; E-Mail-Antworten an Reply-to-Adresse außerhalb von Notify | RX-SMS; REST; REPLY |
| Ü1 | Versand über E-Mail, SMS und Brief; 95 % der E-Mails und SMS in 10 Sekunden; Briefe am nächsten Werktag | FEAT; DEL; PERF |
| Ü2 | TLS 1.2; E-Mail nur opportunistisch per TLS; keine Ende-zu-Ende-Verschlüsselung für E-Mail und SMS; AES-256 im Speicher; Dateien mit AWS SSE-C | SEC |
| Ü3 | DKIM, SPF, DMARC; E-Mail-Branding und SMS-Absenderkennung; kein Siegel, keine Signatur | SEC; FEAT; SENDER |
| Ü4 | Zustellstatus per Dashboard, API und Callback; keine Öffnungs- oder Klickverfolgung | DEL; STATUS; REST |
| Ü5 | keine Interoperabilität mit anderen Postfachsystemen; Versand auch an internationale Nummern und Anschriften | FEAT; PRICE-SMS |
| Ü6 | Virenprüfung für Dateien per E-Mail und PDF-Briefe; zulässige Dateiformate eingeschränkt | REST; FILE |
| I1 | zentraler Betrieb durch den Government Digital Service (GDS); Zustellung über Unterauftragnehmer für E-Mail, SMS und Druck | FEAT; SEC; GH |
| I2 | Inhalte und Empfängerdaten nur befristet, Standard 7 Tage, wählbar 3 bis 90 Tage; AWS in UK und Irland | SEC; RET; DEL |
| I3 | Behörden nutzen nur den zentralen Dienst über Web-Oberfläche oder API | FEAT; API |
| I4 | Team-Mitglieder je Dienst mit Berechtigungen; eingehende SMS im Dashboard oder per API; keine Ende-zu-Ende-Verschlüsselung | TEAM; RX-SMS; SEC |
| I5 | ein Betreiber; Nutzung nur für öffentliche Stellen nach Annahme des Data Processing and Financial Agreement | WHO; TOU |
| R1 | keine eigene Regelung zu Schriftform oder Rechtswirkung; maßgeblich ist das Recht des jeweiligen Verfahrens [unverifiziert] | – |
| R2 | keine Registrierung der Empfangenden; Abmeldung bei Abo-E-Mails verpflichtend | UNSUB; WHO |
| R3 | keine förmliche Zustellung; Briefe per Royal Mail ohne Zustellnachweis | DEL; PRICE-L |
| R4 | freiwillige Nutzung; E-Mails kostenlos, Freikontingent für SMS, keine Einrichtungs- oder Monatskosten, Briefe kostenpflichtig | PRICE; PRICE-SMS; PRICE-L; WHO |
