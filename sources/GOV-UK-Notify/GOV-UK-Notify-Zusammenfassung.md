# GOV.UK Notify (UK, kein zentrales Postfach): Zusammenfassung für die d-box

Wie das UK ohne zentrales Postfach Nachrichten der Verwaltung zustellt: GOV.UK Notify als zentraler Versanddienst für E-Mail, SMS und Brief. Betrieb, Anbindung, Recht. Eingeordnet nach den Zeilen von `morph_box.csv`. Die Zusammenfassung verweist auf die Quellen, statt sie zu wiederholen. Für Details den Pointer öffnen.

Stand: 2026-11-23. Grundlage sind die Hauptquellen aus `../Referenzen-GOV-UK-Notify.md` und ergänzende Gesetzestexte von legislation.gov.uk. Alle Primärquellen liegen in `quellen/`, jeder Unterordner hat eine `_index.md` mit URL, Herausgeber, Datum und Inhaltsnotiz.

## Kennzeichnung

Wie in `../Referenzen-GOV-UK-Notify.md`:

- Ohne Kennzeichnung: Stufe 3, in einer Primärquelle im Volltext belegt.
- **[Indiz]**: Stufe 2, Sekundärquelle, Rückschluss aus einer Primärquelle oder Fehlanzeige nach gezielter Suche.
- **[Annahme]**: Stufe 1, kein Beleg, zu prüfen.

## Kürzel für Quellen

Pfade relativ zu `quellen/`. Z. = Zeile der Markdown-Kopie. Kürzel wie in `../Referenzen-GOV-UK-Notify.md`, neu sind ECA7, ECA8, ECA15, IA7, IA-S1 und SM.

| Kürzel | Datei | Inhalt |
|---|---|---|
| FEAT | `betreiber/notify_features.md` | Produktüberblick |
| WHO | `betreiber/notify_who-can-use-notify.md` | Nutzungsberechtigte, „not for personal use“ |
| SEC | `betreiber/notify_security.md` | Verschlüsselung, Datenstandorte, Unterauftragsverarbeiter, DKIM/SPF/DMARC |
| PERF | `betreiber/notify_performance.md` | Leistungs- und Nutzungsdaten |
| PRICE / PRICE-SMS / PRICE-L | `betreiber/notify_pricing*.md` | Preise E-Mail, SMS, Brief |
| HMRC | `betreiber/govuk_personal-tax-account.md` | Eigenes Konto der HMRC |
| API | `anbindung-technik/notify_api-documentation.md` | Client-Bibliotheken |
| REST | `anbindung-technik/notify_rest-api.md` | REST-API: Kernquelle für die Behördenseite |
| TPL / PERS / BULK | `anbindung-technik/notify_templates.md`, `…_personalisation.md`, `…_bulk-sending.md` | Vorlagen, Platzhalter, Massenversand |
| FILE / LETTER | `anbindung-technik/notify_send-files-by-email.md`, `…_upload-a-letter.md` | Dateien per Link, PDF-Briefe |
| RX-SMS / REPLY / SENDER | `anbindung-technik/notify_receive-text-messages.md`, `…_reply-to-email-address.md`, `…_text-message-sender.md` | Rückkanal, Absenderkennung |
| DEL / STATUS / RET | `anbindung-technik/notify_delivery-times.md`, `…_message-status_email.md`, `…_data-retention-period.md` | Laufzeiten, Status, Aufbewahrung |
| TEAM / UNSUB | `anbindung-technik/notify_team-members-and-permissions.md`, `…_unsubscribe-links.md` | Berechtigungen, Abmeldung |
| GH | `anbindung-technik/github_notifications-api_README.md` | README des Quellcodes (alphagov) |
| TOU | `recht-governance/notify_terms-of-use.md` | Nutzungsbedingungen |
| ECA7 / ECA8 / ECA15 | `recht-governance/legislation_eca-2000_s7.md`, `…_s8.md`, `…_s15.md` | Electronic Communications Act 2000: Signatur als Beweismittel, Verordnungsermächtigung, Begriffe |
| IA7 / IA-S1 | `recht-governance/legislation_interpretation-act-1978_s7.md`, `…_sch1.md` | Zustellung per Post, Definition „Writing“ |
| SM | `recht-governance/govuk_service-manual_sending-emails-and-text-messages.md` | GDS Service Manual: E-Mail und SMS planen |

## 1. Systembild

- Im UK gibt es kein zentrales Postfach. GOV.UK Notify ist ein zentraler Versanddienst für öffentliche Stellen: E-Mail, SMS und Brief aus einer Plattform (FEAT Z. 28). Die Nachricht selbst geht in das E-Mail-Programm, die SMS-App oder den Briefkasten der Empfangenden.
- Behörden adressieren mit eigenen Kontaktdaten; es gibt kein zentrales Verzeichnis (REST, BULK).
- Nicht für Privatpersonen nutzbar (WHO Z. 42); Empfangende haben weder Konto noch Postfach.
- Fachliche Konten einzelner Verwaltungszweige existieren außerhalb von Notify, z. B. HMRC personal tax account (HMRC).
- Das GDS Service Manual empfiehlt, E-Mail oder SMS statt Brief zu erwägen und bei fehlender Reaktion per Brief nachzufassen (SM Z. 97).

## 2. Betrieb, Governance, Teilnahme

- Betreiber ist der Government Digital Service (GDS) (FEAT Z. 107; SEC Z. 260). Quellcode offen bei alphagov (GH).
- Daten in AWS-Rechenzentren in UK und Irland, Unterauftragsverarbeiter in UK und EWR (SEC Z. 63–65, 83).
- Nur öffentliche Stellen, nach Annahme des Data Processing and Financial Agreement (WHO Z. 28, 56).
- Kosten: E-Mails kostenlos, keine Monats-, Einrichtungs- oder Beschaffungskosten (PRICE Z. 29); SMS-Freikontingent (PRICE-SMS); Briefe kostenpflichtig, Royal Mail (PRICE-L Z. 32, 50).
- Nutzungsdaten je Organisation: PERF.

## 3. Anbindung

- REST-API mit API-Schlüssel als JSON Web Token (REST Z. 94–108); Client-Bibliotheken u. a. Java, Python (API Z. 56–60); Callbacks (API Z. 63–65).
- Web-Oberfläche mit Vorlagen und Massenversand (TPL; BULK).
- Dateien per Download-Link, bis 2 MB, optional E-Mail-Bestätigung (FILE Z. 59, 76); PDF-Briefe (LETTER Z. 51).
- Team-Mitglieder mit Berechtigungen (TEAM).

## 4. Einordnung nach `morph_box.csv`

### Behörde

- **B1:** REST-API mit JWT, Client-Bibliotheken, Callbacks (REST Z. 94; API Z. 56–65).
- **B2:** Web-Oberfläche, Vorlagen, Massenversand, Druck und Versand von Briefen (FEAT; TPL; BULK; LETTER Z. 51).
- **B3:** eigene Kontaktdaten der Behörde, kein Verzeichnis (REST; BULK).
- **B4:** Vorlagen mit Platzhaltern; Dateien als Link bis 2 MB; Briefe aus Vorlage oder PDF (PERS; FILE Z. 76; LETTER).

### Person / Organisation

- **P1:** Nachricht selbst per E-Mail oder SMS, Brief per Royal Mail (FEAT; PERS; DEL).
- **P2:** kein Postfach; optional `confirm_email_before_download` (FILE Z. 59; REST).
- **P3, P4, P9:** kein Postfach, kein Nutzerkonto, keine Vertretung (FEAT; WHO Z. 42).
- **P5:** eigene Clients; HMRC-Konto außerhalb (HMRC).
- **P6:** keine Schnittstelle für Empfangende **[Indiz]**: API und REST beschreiben nur Schnittstellen für sendende Dienste; Fehlanzeige auch in den lokalen Kopien.
- **P7, P8:** SMS-Antworten an eindeutige 11-stellige Rufnummer, im Dashboard oder per API (RX-SMS Z. 53–55); E-Mail-Antworten an Reply-to-Adresse außerhalb von Notify (REPLY Z. 51).

### Übermittlung

- **Ü1:** 95 % der E-Mails und SMS in 10 Sekunden; Briefe am nächsten Werktag (DEL Z. 59, 70, 78; PERF).
- **Ü2:** TLS 1.2; E-Mail ohne TLS, wenn der Empfangsserver es nicht unterstützt; keine E2EE; AES-256; SSE-C (SEC Z. 107–133).
- **Ü3:** DMARC, SPF, DKIM (SEC Z. 228–230); Absenderkennung (SENDER); kein Siegel, keine Signatur.
- **Ü4:** Status per Dashboard, API, Callback; keine Öffnungs- oder Lesebestätigung (STATUS Z. 64; REST Z. 999–1000).
- **Ü5:** keine Interoperabilität mit Postfachsystemen; internationale Nummern und Anschriften (FEAT; PRICE-SMS; PRICE-L Z. 50).
- **Ü6:** Virenprüfung, Fehlerstatus `virus-scan-failed` (REST Z. 595, 1000; FILE).

### Infrastruktur und Betrieb

- **I1:** zentral durch GDS, Unterauftragnehmer (SEC Z. 83; GH).
- **I2:** Löschung nach 7 Tagen, wählbar 3 bis 90 (RET Z. 55, 72); AWS UK und Irland (SEC Z. 64).
- **I3:** nur zentraler Dienst (FEAT; API).
- **I4:** Team-Mitglieder mit Berechtigungen, eingehende SMS im Dashboard oder per API (TEAM; RX-SMS Z. 55).
- **I5:** ein Betreiber; nur öffentliche Stellen (WHO Z. 28; TOU).

### Recht

- **R1:** Notify selbst regelt keine Schriftform oder Rechtswirkung **[Indiz]** (Fehlanzeige in TOU). Maßgeblich ist das Recht des jeweiligen Verfahrens: Wo ein Gesetz Zustellung per Post verlangt, gilt sie mit ordnungsgemäß adressiertem, frankiertem Brief als bewirkt (IA7 Z. 100–103). Elektronische Kommunikation statt Schriftform oder Post erfordert eine Verordnung für das jeweilige Gesetz (ECA8 Z. 99–118); elektronische Signaturen sind als Beweismittel zulässig (ECA7 Z. 99–109). „Writing“ umfasst jede sichtbare Wiedergabe von Wörtern (IA-S1 Z. 313).
- **R2:** keine Registrierung der Empfangenden; Abmeldelink bei Abo-E-Mails Pflicht (UNSUB Z. 51–59).
- **R3:** keine förmliche Zustellung; Briefe per Royal Mail ohne Zustellnachweis (DEL; PRICE-L).
- **R4:** freiwillig; E-Mails kostenlos (PRICE Z. 29; WHO).

## 5. Widersprüche und offene Punkte

- Die CSV nennt „Briefe am nächsten Werktag“; DEL Z. 78 sagt genauer: Druck um 17:30 Uhr, Versand (dispatch) am nächsten Werktag. Gemeint ist der Versand, nicht die Zustellung.
- PRICE-L Z. 32 kündigt Preiserhöhungen zum 2026-12-07 an; die Preise in den Kopien können danach veraltet sein.
- Ob ein Brief über Notify den Anforderungen von IA7 genügt („pre-paying and posting“), ist nicht geprüft **[Annahme]**; naheliegend, da Notify über Royal Mail versendet.

## 6. Lücken

- P6: keine Primärquelle, die eine Schnittstelle für Empfangende ausdrücklich ausschließt; Prüfweg Anfrage GDS / Notify-Team.
- R1: keine Aussage des GDS zur Rechtswirkung von Notify-Nachrichten; keine Verordnung nach ECA8 geprüft, die E-Mail-Zustellung für ein konkretes Verfahren erlaubt.
- Quellcode nur als README gesichert; Details zu Unterauftragsverarbeitern stehen nur in SEC.

## 7. Verwandte lokale Quellen

- `../Referenzen-GOV-UK-Notify.md`: Quellenliste mit Evidenzstufen.
- `../Referenzen-Digital-Post-Daenemark.md`, `../Referenzen-Digital-Postkasse-Norwegen.md`, `../Referenzen-Mein-Postkorb-Oesterreich.md`: Vergleichslösungen mit zentralem Postfach.
- `../MUK/MUK-Zusammenfassung.md`: Vorbild für Aufbau und Stil.
