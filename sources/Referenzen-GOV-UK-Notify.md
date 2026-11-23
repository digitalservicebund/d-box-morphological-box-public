# GOV.UK Notify (UK): Quellen

Nachweise für die Spalte `UK (kein zentrales Postfach)` in `existing_solutions.csv` und die GOV.UK-Notify-Einträge in `morph_box.csv`. Statt `[unverifiziert]` stehen hier und in den GOV.UK-Notify-Einträgen der CSV-Dateien `[Indiz]` (Stufe 2) und `[Annahme]` (Stufe 1); unmarkierte Aussagen sind Stufe 3. `Fehlanzeige` heißt: in den Quellen gesucht, nicht gefunden.

Evidenzstufen (Spalte `Stufe`):

- **3 (Belegt / Primärquelle):** Gesetzestext, Betreiberdokument oder Quellcode des Betreibers, im Volltext geprüft.
- **2 (Sekundär / Indiz):** Herstelleraussage, Pressemitteilung, veraltete Angabe oder Rückschluss aus einer Primärquelle; `Fehlanzeige` bei gezielter Suche zählt als Indiz für eine negative Aussage.
- **1 (Annahme / Hypothese):** kein Beleg, nicht einmal ein Indiz; offen und zu prüfen.

Bei gemischten Zeilen steht zuerst die Stufe des belegten Teils, danach die Stufe des Teils, der in der Aussage mit `[Indiz]` oder `[Annahme]` markiert ist.

Stand: 2026-11-23. Lokale Kopien aller Hauptquellen und ergänzender Gesetzestexte liegen in `GOV-UK-Notify/quellen/` (Unterordner `betreiber/`, `anbindung-technik/`, `recht-governance/`, je mit `_index.md`); Zusammenfassung: `GOV-UK-Notify/GOV-UK-Notify-Zusammenfassung.md`. Alle Hauptquellen wurden an diesem Tag im Volltext geprüft (Betreiberseiten des GDS, README des Quellcodes auf GitHub, GOV.UK, legislation.gov.uk).

## Hauptquellen

| Kürzel | Quelle | Stufe |
|---|---|---|
| FEAT | GOV.UK Notify, Features: <https://www.notifications.service.gov.uk/features> | 3 |
| WHO | GOV.UK Notify, Who can use Notify: <https://www.notifications.service.gov.uk/features/who-can-use-notify> | 3 |
| SEC | GOV.UK Notify, Security features: <https://www.notifications.service.gov.uk/features/security> | 3 |
| PERF | GOV.UK Notify, Performance: <https://www.notifications.service.gov.uk/features/performance> | 3 |
| PRICE | GOV.UK Notify, Pricing: <https://www.notifications.service.gov.uk/pricing> | 3 |
| PRICE-SMS | GOV.UK Notify, Text message pricing: <https://www.notifications.service.gov.uk/pricing/text-messages> | 3 |
| PRICE-L | GOV.UK Notify, Letter pricing: <https://www.notifications.service.gov.uk/pricing/letters> | 3 |
| API | GOV.UK Notify, API documentation (Client-Bibliotheken): <https://www.notifications.service.gov.uk/using-notify/api-documentation> | 3 |
| REST | GOV.UK Notify, REST API documentation: <https://docs.notifications.service.gov.uk/rest-api.html> | 3 |
| TPL | GOV.UK Notify, Templates: <https://www.notifications.service.gov.uk/using-notify/templates> | 3 |
| PERS | GOV.UK Notify, Personalisation: <https://www.notifications.service.gov.uk/using-notify/personalisation> | 3 |
| BULK | GOV.UK Notify, Bulk sending: <https://www.notifications.service.gov.uk/using-notify/bulk-sending> | 3 |
| FILE | GOV.UK Notify, Send files by email: <https://www.notifications.service.gov.uk/using-notify/send-files-by-email> | 3 |
| LETTER | GOV.UK Notify, Upload a letter: <https://www.notifications.service.gov.uk/using-notify/upload-a-letter> | 3 |
| RX-SMS | GOV.UK Notify, Receive text messages: <https://www.notifications.service.gov.uk/using-notify/receive-text-messages> | 3 |
| REPLY | GOV.UK Notify, Reply-to email address: <https://www.notifications.service.gov.uk/using-notify/reply-to-email-address> | 3 |
| SENDER | GOV.UK Notify, Text message sender ID: <https://www.notifications.service.gov.uk/using-notify/text-message-sender> | 3 |
| DEL | GOV.UK Notify, Delivery times: <https://www.notifications.service.gov.uk/using-notify/delivery-times> | 3 |
| STATUS | GOV.UK Notify, Message status: <https://www.notifications.service.gov.uk/using-notify/message-status/email> | 3 |
| RET | GOV.UK Notify, Data retention period: <https://www.notifications.service.gov.uk/using-notify/data-retention-period> | 3 |
| TEAM | GOV.UK Notify, Team members and permissions: <https://www.notifications.service.gov.uk/using-notify/team-members-and-permissions> | 3 |
| UNSUB | GOV.UK Notify, Unsubscribe links: <https://www.notifications.service.gov.uk/using-notify/unsubscribe-links> | 3 |
| TOU | GOV.UK Notify, Terms of use: <https://www.notifications.service.gov.uk/terms-of-use> | 3 |
| GH | Quellcode GOV.UK Notify (GDS): <https://github.com/alphagov/notifications-api> | 3 |
| HMRC | GOV.UK, Personal tax account: <https://www.gov.uk/personal-tax-account> | 3 |
| ECA8 | Electronic Communications Act 2000, s. 8 (Verordnungsermächtigung für elektronische Kommunikation statt Schriftform): <https://www.legislation.gov.uk/ukpga/2000/7/section/8> | 3 |
| IA7 | Interpretation Act 1978, s. 7 (Zustellung per Post): <https://www.legislation.gov.uk/ukpga/1978/30/section/7> | 3 |

## Nachweise je Dimension

| Dimension | Aussage | Quellen | Stufe |
|---|---|---|---|
| B1 | REST-API mit API-Schlüssel als JSON Web Token; Client-Bibliotheken für Java, .NET, Node.js, PHP, Python, Ruby; Callbacks für Zustellstatus und eingehende SMS | API; REST; SEC | 3 |
| B2 | Web-Oberfläche mit Vorlagen, Einzel- und Massenversand per Tabellen-Upload; E-Mail, SMS und Brief aus einer Plattform; Notify druckt und versendet Briefe | FEAT; TPL; BULK | 3 |
| B3 | Behörde adressiert mit eigenen Kontaktdaten (E-Mail-Adresse, Mobilnummer, Postanschrift); kein zentrales Verzeichnis | REST; BULK | 3 |
| B4 | Vorlagen mit Platzhaltern; Dateien als verschlüsselter Download-Link statt Anhang, Formate, bis 2 MB, 1 bis 78 Wochen abrufbar; Briefe aus Vorlage oder PDF | PERS; FILE; REST; LETTER | 3 |
| P1 | Nachricht selbst per E-Mail oder SMS mit Inhalt, auch sensible Platzhalter wie Sicherheitscodes; Brief per Royal Mail; Kanalwahl durch die Behörde | FEAT; PERS; DEL | 3 |
| P2 | kein Postfach; optional Bestätigung der E-Mail-Adresse vor Datei-Download (`confirm_email_before_download`) | FILE; REST | 3 |
| P3, P4, P9 | kein Postfach und kein Nutzerkonto für Empfangende; keine Vertretungsfunktion | FEAT; WHO („not for personal use“) | 3 |
| P5 | Empfang in eigenem E-Mail-Programm, SMS-App und Papierpost; Konten einzelner Verwaltungszweige außerhalb von Notify, z. B. HMRC personal tax account | FEAT; HMRC | 3 |
| P6 | keine Schnittstelle für Empfangende [Indiz]; Rückschluss aus dem Fehlen eines Postfachs, API nur für Dienste öffentlicher Stellen | FEAT; API; REST; Schnittstelle für Empfangende: Fehlanzeige | 2 |
| P7, P8 | Antwort und anlasslose SMS an eindeutige Rufnummer, im Dashboard oder per API; E-Mail-Antworten an Reply-to-Adresse außerhalb von Notify | RX-SMS; REST; REPLY | 3 |
| Ü1 | Versand über E-Mail, SMS und Brief; 95 % der E-Mails und SMS in 10 Sekunden; Briefe am nächsten Werktag | FEAT; DEL; PERF | 3 |
| Ü2 | TLS 1.2; E-Mail nur opportunistisch per TLS; keine Ende-zu-Ende-Verschlüsselung für E-Mail und SMS; AES-256 im Speicher; Dateien mit AWS SSE-C | SEC | 3 |
| Ü3 | DKIM, SPF, DMARC; E-Mail-Branding und SMS-Absenderkennung; kein Siegel, keine Signatur | SEC; FEAT; SENDER | 3 |
| Ü4 | Zustellstatus per Dashboard, API und Callback; keine Öffnungs- oder Klickverfolgung | DEL; STATUS; REST | 3 |
| Ü5 | keine Interoperabilität mit anderen Postfachsystemen; Versand auch an internationale Nummern und Anschriften | FEAT; PRICE-SMS | 3 |
| Ü6 | Virenprüfung für Dateien per E-Mail und PDF-Briefe; zulässige Dateiformate eingeschränkt | REST; FILE | 3 |
| I1 | zentraler Betrieb durch den Government Digital Service (GDS); Zustellung über Unterauftragnehmer für E-Mail, SMS und Druck | FEAT; SEC; GH | 3 |
| I2 | Inhalte und Empfängerdaten nur befristet, Standard 7 Tage, wählbar 3 bis 90 Tage; AWS in UK und Irland | SEC; RET; DEL | 3 |
| I3 | Behörden nutzen nur den zentralen Dienst über Web-Oberfläche oder API | FEAT; API | 3 |
| I4 | Team-Mitglieder je Dienst mit Berechtigungen; eingehende SMS im Dashboard oder per API; keine Ende-zu-Ende-Verschlüsselung | TEAM; RX-SMS; SEC | 3 |
| I5 | ein Betreiber; Nutzung nur für öffentliche Stellen nach Annahme des Data Processing and Financial Agreement | WHO; TOU | 3 |
| R1 | maßgeblich ist das Recht des jeweiligen Verfahrens: elektronische Kommunikation statt Schriftform oder Post nur per Verordnung je Gesetz, Zustellung per Post nach IA7; keine eigene Regelung zu Schriftform oder Rechtswirkung in Notify [Indiz] | ECA8; IA7; TOU; FEAT; Regelung zu Rechtswirkung in Notify: Fehlanzeige | 3, 2 |
| R2 | keine Registrierung der Empfangenden; Abmeldung bei Abo-E-Mails verpflichtend | UNSUB; WHO | 3 |
| R3 | keine förmliche Zustellung; Briefe per Royal Mail ohne Zustellnachweis | DEL; PRICE-L | 3 |
| R4 | freiwillige Nutzung; E-Mails kostenlos, Freikontingent für SMS, keine Einrichtungs- oder Monatskosten, Briefe kostenpflichtig | PRICE; PRICE-SMS; PRICE-L; WHO | 3 |

## Evidenzstufen der markierten Stellen

Einordnung der früheren `[unverifiziert]`-Stellen, Stand 2026-11-23. Stufe 1 und 2 werden über den genannten Prüfweg geprüft. In `morph_box.csv` gibt es keine Markierung, die sich auf GOV.UK Notify oder das UK bezieht.

| Datei, Dimension | Aussage | Stufe | Begründung | Prüfweg |
|---|---|---|---|---|
| `existing_solutions.csv`, P6 | keine Schnittstelle für Empfangende | 2 | Rückschluss aus dem Fehlen eines Postfachs (FEAT, WHO); API und REST-Dokumentation beschreiben nur Schnittstellen für sendende Dienste; Fehlanzeige | Anfrage GDS / Notify-Team |
| `existing_solutions.csv`, R1 | Notify regelt keine Schriftform oder Rechtswirkung | 2 | Fehlanzeige in Nutzungsbedingungen (TOU) und Betreiberseiten; der Teil „Recht des jeweiligen Verfahrens maßgeblich“ ist mit ECA8 und IA7 belegt (Stufe 3) und nicht mehr markiert | Anfrage GDS |
