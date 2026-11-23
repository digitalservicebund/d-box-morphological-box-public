# Quellen: GOV.UK Notify – Anbindung von Behörden, Funktionen, Technik

Abgerufen am 2026-11-23. HTML-Seiten wurden mit `lynx -dump -nolist` in Text umgewandelt (Navigation und Fußzeile nicht entfernt). Am Dateianfang stehen jeweils Titel, Quelle und Abrufdatum. Vom Quellcode (GH) ist nur das README gesichert, nicht das Repository. Alle Seiten waren erreichbar.

Themen der parallelen Ordner: `../betreiber/` (Produkt, Sicherheit, Preise, Nutzungsberechtigte) und `../recht-governance/` (Nutzungsbedingungen, UK-Recht, GDS Service Manual).

| Datei | URL | Titel / Herausgeber | Datum Dokument | Inhalt (kurz) |
|---|---|---|---|---|
| notify_api-documentation.md | https://www.notifications.service.gov.uk/using-notify/api-documentation | Documentation / GDS | ohne Datum | Client-Bibliotheken Java, .NET, Node.js, PHP, Python, Ruby; REST |
| notify_rest-api.md | https://docs.notifications.service.gov.uk/rest-api.html | REST API documentation / GDS | ohne Datum | JWT-Authentifizierung, Senden, Status, Callbacks, Dateien (`confirm_email_before_download`), eingehende SMS, Fehlercodes |
| notify_templates.md | https://www.notifications.service.gov.uk/using-notify/templates | Templates / GDS | ohne Datum | Vorlagen für E-Mail, SMS, Brief |
| notify_personalisation.md | https://www.notifications.service.gov.uk/using-notify/personalisation | Personalisation / GDS | ohne Datum | Platzhalter, auch sensible Inhalte |
| notify_bulk-sending.md | https://www.notifications.service.gov.uk/using-notify/bulk-sending | Bulk sending / GDS | ohne Datum | Massenversand per Tabellen-Upload |
| notify_send-files-by-email.md | https://www.notifications.service.gov.uk/using-notify/send-files-by-email | Send files by email / GDS | ohne Datum | Download-Link statt Anhang, bis 2 MB, Abrufdauer, E-Mail-Bestätigung |
| notify_upload-a-letter.md | https://www.notifications.service.gov.uk/using-notify/upload-a-letter | Upload a letter / GDS | ohne Datum | PDF-Brief hochladen, Druck und Versand durch Notify |
| notify_receive-text-messages.md | https://www.notifications.service.gov.uk/using-notify/receive-text-messages | Receive text messages / GDS | ohne Datum | Eindeutige 11-stellige Rufnummer, Antworten im Dashboard oder per API |
| notify_reply-to-email-address.md | https://www.notifications.service.gov.uk/using-notify/reply-to-email-address | Reply-to email address / GDS | ohne Datum | Antworten gehen an eine Adresse der Behörde |
| notify_text-message-sender.md | https://www.notifications.service.gov.uk/using-notify/text-message-sender | Text message sender ID / GDS | ohne Datum | SMS-Absenderkennung |
| notify_delivery-times.md | https://www.notifications.service.gov.uk/using-notify/delivery-times | Delivery times / GDS | ohne Datum | 95 % in 10 Sekunden; Briefe am nächsten Werktag |
| notify_message-status_email.md | https://www.notifications.service.gov.uk/using-notify/message-status/email | Message status / GDS | ohne Datum | Statuswerte; keine Öffnungs- oder Lesebestätigung |
| notify_data-retention-period.md | https://www.notifications.service.gov.uk/using-notify/data-retention-period | Data retention period / GDS | ohne Datum | Löschung nach 7 Tagen, wählbar 3 bis 90 Tage |
| notify_team-members-and-permissions.md | https://www.notifications.service.gov.uk/using-notify/team-members-and-permissions | Team members and permissions / GDS | ohne Datum | Rollen und Berechtigungen im Dienst |
| notify_unsubscribe-links.md | https://www.notifications.service.gov.uk/using-notify/unsubscribe-links | Unsubscribe links / GDS | ohne Datum | Abmeldung bei Abo-E-Mails verpflichtend |
| github_notifications-api_README.md | https://github.com/alphagov/notifications-api | alphagov/notifications-api, README (Branch `main`) / GDS | laufend | Aufbau der API-Komponente, Abhängigkeiten (AWS, Celery), Entwicklung |
