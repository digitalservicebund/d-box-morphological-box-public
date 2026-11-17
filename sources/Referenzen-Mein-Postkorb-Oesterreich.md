# Mein Postkorb (Österreich): Quellen

Nachweise für die Spalte `Österreich` in `existing_solutions.csv` und die Österreich-Einträge in `morph_box.csv`. Einträge mit `[unverifiziert]` sind in keiner der Quellen ausdrücklich belegt.

## Hauptquellen

| Kürzel | Quelle |
|---|---|
| ZustG | Zustellgesetz, 3. Abschnitt (§§ 28–37b), RIS: <https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=Bundesnormen&Gesetzesnummer=10005522> |
| E-GovG | E-Government-Gesetz (§§ 1a, 1b, 19, 20), RIS: <https://www.ris.bka.gv.at/GeltendeFassung.wxe?Abfrage=Bundesnormen&Gesetzesnummer=20003230> |
| WP | BKA, White Paper „Elektronische Zustellung für Behörden“: <https://www.bundeskanzleramt.gv.at/dam/jcr:55d6c9d8-8b6d-4a7b-adec-f9416b8f1561/whitepaper_e-zustellung_behoerden.pdf> |
| ZUSE | BKA, Spezifikation E-Zustellung v2.4.0 (ZIP mit zuseorg, zusemsg, zuseamod, zusetnvz): <https://www.bundeskanzleramt.gv.at/dam/jcr:1b184f8a-eb43-447f-afee-2c389d31bd0b/eZustellung_ZUSE_Q1_2025_nbf.zip> |
| BKA-B | BKA, E-Zustellung für Behörden: <https://www.bundeskanzleramt.gv.at/agenda/digitalisierung/elektronische-zustellung/e-zustellung-behoerden.html> |
| BKA-T | BKA, Technische Informationen (Portale, zugelassene Zustelldienste, Kommunikationssysteme, MOA-ZS): <https://www.bundeskanzleramt.gv.at/agenda/digitalisierung/elektronische-zustellung/e-zustellung-technische-informationen.html> |
| BKA-P | BKA, FAQ Privatpersonen: <https://www.bundeskanzleramt.gv.at/agenda/digitalisierung/elektronische-zustellung/faq-e-zustellung-privatpersonen.html> |
| BKA-U | BKA, FAQ Unternehmen: <https://www.bundeskanzleramt.gv.at/agenda/digitalisierung/elektronische-zustellung/faq-e-zustellung-unternehmen.html> |
| OE-1 | oesterreich.gv.at, Die elektronische Zustellung und „Mein Postkorb“: <https://www.oesterreich.gv.at/de/hilfe/Die-elektronische-Zustellung-und-Mein-Postkorb> |
| OE-2 | oesterreich.gv.at, Abholen eines behördlichen Schriftstücks: <https://www.oesterreich.gv.at/de/hilfe/Abholen-eines-behördlichen-Schriftstücks-in-„Mein-Postkorb“-> |
| OE-3 | oesterreich.gv.at, Push-Benachrichtigungen: <https://www.oesterreich.gv.at/de/hilfe/Push-Benachrichtigungen-aktivieren-für-Mein-Postkorb> |
| OE-4 | oesterreich.gv.at, Ablauf der Zustellung: <https://www.oesterreich.gv.at/de/themen/egovernment_moderne_verwaltung/Elektronische-Zustellung/ablauf-der-zustellung-über-meinpostkorb> |
| OE-5 | oesterreich.gv.at, E-Mail-Benachrichtigung: <https://www.oesterreich.gv.at/de/themen/egovernment_moderne_verwaltung/Elektronische-Zustellung/e-mail-benachrichtigung> |
| OE-6 | oesterreich.gv.at, Teilnehmerverzeichnis: <https://www.oesterreich.gv.at/de/themen/egovernment_moderne_verwaltung/Elektronische-Zustellung/teilnehmerverzeichnis> |
| OE-7 | oesterreich.gv.at, Recht auf elektronischen Verkehr: <https://www.oesterreich.gv.at/de/themen/egovernment_moderne_verwaltung/Elektronische-Zustellung/recht-auf-elektronischen-verkehr> |
| USP-1 | USP, Elektronische Zustellung: <https://www.usp.gv.at/laufender-betrieb/elektronische-zustellung.html> |
| USP-2 | USP, Mein Postkorb: <https://www.usp.gv.at/laufender-betrieb/elektronische-zustellung/meinpostkorb.html> |
| USP-3 | USP-Hilfe, Mein Postkorb: <https://www.usp.gv.at/hilfe-und-support/mein-postkorb.html> |
| USP-4 | USP-Hilfe, Postbevollmächtigte hinzufügen: <https://www.usp.gv.at/hilfe-und-support/mein-postkorb/postbevollmaechtigten-hinzuf-gen.html> |
| USP-5 | USP-Hilfe, Vertretungsmanagement: <https://www.usp.gv.at/hilfe-und-support/vollmachten/vertretungsmanagement.html> |

## Nachweise je Dimension

| Zeile | Aussage | Quellen |
|---|---|---|
| B1 | Behörde wählt Zustellsystem: zugelassener Zustelldienst, Kommunikationssystem der Behörde oder ERV | ZustG § 28 Abs. 3; WP 3.1; BKA-T |
| B1 | einheitliche Schnittstelle, Middleware MOA-ZS, Liste der Zustelldienste und Kommunikationssysteme | ZUSE (zusemsg, zuseorg); BKA-T |
| B2 | duale Zustellung: elektronisch oder Ausdruck und Postversand | WP 4.2.1 |
| B3 | Teilnehmerverzeichnis, bPK Zustellung, Stammzahl, ERV-Code, GLN, Abfrage über Name und Anschrift | ZustG §§ 28a, 28b, 34; WP 3.5; ZUSE (zusetnvz) |
| B4 | übliche Formate, insbesondere PDF; Amtssignatur | WP Fußnote 8; E-GovG § 19 |
| P1 | E-Mail-Verständigung, zweite nach 48 Stunden, Push optional, Papier als Rückfall | ZustG §§ 35, 36; OE-1; OE-2; OE-3; OE-5; OE-6 |
| P2 | ID Austria oder EU-Login für nachweisliche, FinanzOnline-/USP-Kennung für nicht nachweisliche Zustellungen | OE-4; USP-3; WP Tabelle 2 |
| P3 | Rolle Postbevollmächtigte:r im USP, alle sehen alle Nachrichten | USP-2; USP-4; WP 5 |
| P4 | Vollmachtenservice der Stammzahlenregisterbehörde; kein gemeinsames Postfach; USP-Vertretungsmanagement | BKA-P; ZustG § 35; USP-5 |
| P5 | Anzeigemodul in oesterreich.gv.at, USP, JustizOnline, eAMA, App ID Austria | ZustG § 37b; BKA-T; OE-1 |
| P6 | automatische Abholung per Webservice mit Zertifikat; Weiterleitung in den ERV | USP-1; USP-2; WP 3.4.1, 6.1 |
| P7, P8 | Antwortformular für Rückantwort; Inhalt und Umfang [unverifiziert] | BKA-B |
| P9 | Registrierung im Teilnehmerverzeichnis getrennt vom Portal, Zugang über mehrere Portale | ZustG §§ 28b, 37b; WP 3.4 |
| Ü1 | Sendungsqualitäten nachweislich und nicht nachweislich | WP Tabelle 2; ZustG §§ 35, 36 |
| Ü2 | Dokument unverschlüsselt an Zustellsystem, dort für das Anzeigemodul verschlüsselt abgelegt | ZUSE (zusemsg, zuseamod 5.2, zuseorg) |
| Ü3 | Amtssignatur; amtssignierte Verständigung; signierte Übergabebestätigung und Zustellnachweis | E-GovG §§ 19, 20; ZUSE (zuseorg, zusemsg); OE-5 |
| Ü4 | Übergabebestätigung, Zustellnachweis, Verständigung des Absenders bei Nichtabholung | ZustG §§ 29, 35; ZUSE (zuseorg); WP 3.6 |
| Ü5 | keine Interoperabilität nach außen [unverifiziert]; ERV als Zustellsystem | ZustG § 28 Abs. 3 |
| Ü6 | Zustellsystem prüft Anhänge auf Schadsoftware; Formate im Teilnehmerverzeichnis | ZUSE (zuseamod); ZustG § 28b Abs. 2; OE-6 |
| I1 | zugelassene Zustelldienste, zentrales Teilnehmerverzeichnis und Anzeigemodul | ZustG §§ 28a, 30, 37b; BKA-T |
| I2 | Bereithaltung beim Zustellsystem, Fristen und Löschung, Metadaten im Anzeigemodul | ZustG §§ 35, 36; OE-2; WP 3.6 |
| I3 | Behörden als Empfängerinnen über USP oder Portalverbund | WP 5; BKA-B |
| I4 | keine Ende-zu-Ende-Verschlüsselung; Postbevollmächtigte sehen alle Nachrichten | ZUSE (zusemsg); USP-4 |
| I5 | Zulassung per Bescheid, Konformitätsbewertung, Gutachten alle zwei Jahre, Aufsicht, Widerruf | ZustG §§ 30, 31 |
| R1 | Rechtswirkung der Zustellung; Beweiskraft des Ausdrucks | ZustG §§ 35, 36; E-GovG § 20 |
| R2 | Teilnahmepflicht für Unternehmen mit Ausnahmen | E-GovG § 1b; OE-7; BKA-U |
| R3 | Zustellzeitpunkt mit und ohne Zustellnachweis | ZustG §§ 35, 36; OE-2 |
| R4 | Recht auf elektronischen Verkehr in Bundessachen; Kosten und Freigrenze 25 Mio. Einlieferungen | E-GovG § 1a; OE-7; ZustG § 40 Abs. 11; WP 3.4; BKA-B |
