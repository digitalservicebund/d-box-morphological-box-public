# Mein Postkorb (Österreich): Zusammenfassung für die d-box

Wie die österreichische elektronische Zustellung (eZustellung) mit dem Anzeigemodul „Mein Postkorb“ aufgesetzt ist: Betrieb, Anbindung, Recht. Eingeordnet nach den Zeilen von `morph_box.csv`. Die Zusammenfassung verweist auf die Quellen, statt sie zu wiederholen. Für Details den Pointer öffnen.

Stand: 2026-11-23. Grundlage sind die Hauptquellen aus `../Referenzen-Mein-Postkorb-Oesterreich.md`, am 2026-11-23 heruntergeladen. Alle Quellen liegen in `quellen/`, jeder Unterordner hat eine `_index.md` mit URL, Herausgeber, Datum und Inhaltsnotiz.

## Kennzeichnung

Wie in `../Referenzen-Mein-Postkorb-Oesterreich.md`:

- Ohne Kennzeichnung: Stufe 3, in einer Primärquelle im Volltext belegt.
- **[Indiz]**: Stufe 2, Sekundärquelle, Rückschluss aus einer Primärquelle oder Fehlanzeige bei gezielter Suche.
- **[Annahme]**: Stufe 1, kein Beleg.

## Kürzel für Quellen

Pfade relativ zu `quellen/`. Z. = Zeile der Markdown-Kopie, S. = gedruckte Seite, PDF-S. = Seite der PDF-Datei. Kürzel wie in `../Referenzen-Mein-Postkorb-Oesterreich.md`.

| Kürzel | Datei | Inhalt |
|---|---|---|
| ZustG | `recht-governance/ris_zustg_2026-11-23.md` | Zustellgesetz, 3. Abschnitt ab Z. 1029 (§ 28); § 35 Z. 1667, § 36 Z. 1809, § 37b Z. 1999, § 40 Z. 2116 |
| E-GovG | `recht-governance/ris_e-govg_2026-11-23.md` | § 1a Z. 295, § 1b Z. 339, § 19 Z. 2298, § 20 Z. 2335 |
| OE-7 | `recht-governance/oe_recht-auf-elektronischen-verkehr.md` | Recht auf elektronischen Verkehr, Teilnahmepflicht Unternehmen |
| WP | `anbindung-technik/bka_whitepaper_e-zustellung-behoerden.pdf` | White Paper für Behörden (06/2025): Kernquelle für die Behördenseite |
| ZUSE | `anbindung-technik/bka_zuse_{zuseorg,zusemsg,zuseamod,zusetnvz}_2.4.0.pdf` | Spezifikation E-Zustellung v2.4.0 (01/2025) |
| ZUSE-BD | `anbindung-technik/bka_zuse_begleitdokument-rollout_2025-q1.pdf` | Rollout neuer Funktionen Q1 2025 |
| BKA-B / BKA-T | `anbindung-technik/bka_e-zustellung-behoerden.md`, `…-technische-informationen.md` | Behördenseite, Liste der Zustelldienste, Portale, MOA-ZS |
| BKA-E / BKA-P / BKA-U | `betreiber/bka_e-zustellung-uebersicht.md`, `bka_faq-e-zustellung-privatpersonen.md`, `…-unternehmen.md` | Übersicht und FAQ |
| OE-1 … OE-6 | `betreiber/oe_*.md` | Hilfe- und Themenseiten oesterreich.gv.at |
| USP-1 … USP-5 | `betreiber/usp_*.md` | Unternehmensservice Portal: eZustellung, Mein Postkorb, Postbevollmächtigte, Vertretung |

## 1. Systembild

- Die eZustellung ist ein **Verbund**: Behörden liefern über ein Zustellsystem ein, das ist entweder ein zugelassener Zustelldienst (Hpc DUAL, Österreichische Post, Bundesrechenzentrum, VENDO; BKA-T Z. 28–45) oder ein eigenes Kommunikationssystem der Behörde (ZustG § 28 Abs. 3, Z. 1029 ff.; WP 3.1).
- Zentral vom Bund betrieben werden das **Teilnehmerverzeichnis** (TNVZ, ZustG § 28a, Z. 1096) und das **Anzeigemodul** „Mein Postkorb“ (ZustG § 37b, Z. 1999). Das Zustellsystem fragt das TNVZ ab, hält das Dokument bereit und meldet Metadaten an das Anzeigemodul (ZUSE zuseamod PDF-S. 103).
- „Mein Postkorb“ ist kein eigenes Nutzerkonto, sondern in mehrere Portale eingebunden: oesterreich.gv.at, App ID Austria, USP, JustizOnline, eAMA (BKA-T Z. 17–20; OE-1).
- Für Fachverfahren gibt es die Open-Source-Middleware MOA-ZS mit einheitlicher Schnittstelle (BKA-T Z. 59–71).
- Wer nicht registriert ist, bekommt Papier: duale Zustellung aus demselben Versand (WP 4.2.1).

## 2. Betrieb, Governance, Teilnahme

- Zustelldienste werden per Bescheid zugelassen, mit Konformitätsbewertung, Gutachten alle zwei Jahre, Aufsicht und Widerruf (ZustG §§ 30, 31, Z. 1448, 1541). Kommunikationssysteme der Behörden werden nur registriert (BKA-T).
- Teilnahmepflicht für Unternehmen seit 2020 (E-GovG § 1b, Z. 339; OE-7 Z. 28–32); Personen freiwillig. Recht auf elektronischen Verkehr in Bundessachen (E-GovG § 1a, Z. 295).
- Kosten: Behörde zahlt das Zustellsystem, keine gesetzliche Preisbindung mehr (BKA-B Z. 40–44); Anzeigemodul und TNVZ bis 25 Mio. Einlieferungen pro Jahr nicht zu verrechnen (ZustG Z. 2284).
- Nur Verantwortliche des öffentlichen Bereichs dürfen über das System versenden (ZUSE zuseorg; BKA-P Z. 93; BKA-U Z. 157).

## 3. Identität und Adressierung

- Adressierung über das TNVZ: bPK Zustellung, Stammzahl (Firmenbuch-, ZVR-, ERsB-Nummer), ERV-Code, GLN; Abfrage auch über Name und Anschrift (ZustG §§ 28a, 28b, 34; ZUSE zusetnvz).
- Login: nachweisliche Zustellungen nur mit ID Austria oder EU-Login, sonst auch FinanzOnline- oder USP-Kennung (OE-4 Z. 23; USP-3 Z. 89; WP Tabelle 2).

## 4. Postfach und Organisationen

- Unternehmen: USP-Administrator:in vergibt die Rolle Postbevollmächtigte:r, Einzelvertretungsbefugte automatisch (USP-2 Z. 59; USP-4 Z. 20). Alle Postbevollmächtigten sehen alle Nachrichten.
- Vertretung: Vollmachtenservice der Stammzahlenregisterbehörde; kein gemeinsames Postfach für Familien (BKA-P Z. 436–448). Vollmachten zwischen Unternehmen über das USP-Vertretungsmanagement (USP-5).
- Automatische Abholung per Webservice mit Zertifikat, Weiterleitung in den ERV der Justiz (USP-1 Z. 31; USP-2; WP 3.4.1, 6.1).

## 5. Einordnung nach `morph_box.csv`

### Behörde

| Zeile | Österreich |
|---|---|
| B1 Anbindung von Fachverfahren | Behörde wählt das Zustellsystem (ZustG § 28 Abs. 3; WP 3.1); einheitliche ZUSE-Schnittstelle (ZUSE zusemsg, zuseorg); MOA-ZS (BKA-T Z. 59–71). |
| B2 Zugang ohne Fachverfahren | duale Zustellung über Dienstleister (WP 4.2.1). |
| B3 Adressierung | TNVZ mit bPK, Stammzahl, ERV-Code, GLN, Name und Anschrift (ZustG §§ 28a, 28b, 34; ZUSE zusetnvz). |
| B4 Inhalt an die Person / Organisation | übliche Formate, insbesondere PDF (WP S. 14); Amtssignatur (E-GovG § 19, Z. 2298). |

### Person / Organisation

| Zeile | Österreich |
|---|---|
| P1 Benachrichtigung | E-Mail, zweite Verständigung nach 48 Stunden (ZustG § 35, Z. 1728; OE-2 Z. 69); Push optional (OE-3); Papier als Rückfall (OE-1). Nachrichten „nur zur Information“ (z. B. auch über FinanzOnline bereitgestellt) ohne Verständigung (OE-1). |
| P2 Authentifizierung | ID Austria oder EU-Login für nachweisliche Zustellungen (OE-4 Z. 23; USP-3 Z. 89). |
| P3 Organisationszugang | Postbevollmächtigte im USP (USP-2 Z. 59; USP-4 Z. 20; WP 5). |
| P4 Vertretung | Vollmachtenservice; kein gemeinsames Postfach (BKA-P Z. 436–448); USP-5. |
| P5 Clients | Anzeigemodul in mehreren Portalen und der App (ZustG § 37b; BKA-T Z. 17–20; OE-1). |
| P6 Anbindung eigener Systeme | Webservice, ERV (USP-1 Z. 31; USP-2; WP 3.4.1, 6.1). |
| P7 Antwort und Erstkontakt | Antwortformular (BKA-B Z. 46–48); kein anlassloser Erstkontakt (BKA-P Z. 93; BKA-U Z. 157). |
| P8 Inhalt an die Behörde | Inhalt des Antwortformulars **[Indiz]**: ZUSE kennt nur URL-Action-Links aus der Nachricht in die Versender-Applikation, Domains vorab freigeschaltet (zuseamod 3.2.1.2.1.6, PDF-S. 60). |
| P9 Postfach und Nutzerkonto | Registrierung im TNVZ getrennt vom Portal (ZustG §§ 28b, 37b; WP 3.4). |

### Übermittlung

| Zeile | Österreich |
|---|---|
| Ü1 Modus | nachweislich / nicht nachweislich (WP Tabelle 2; ZustG §§ 35, 36). |
| Ü2 Vertraulichkeit | Dokument unverschlüsselt an Zustellsystem, dort für das Anzeigemodul verschlüsselt abgelegt (zuseamod 5.2, PDF-S. 103). |
| Ü3 Echtheit | Amtssignatur (E-GovG §§ 19, 20); signierte Übergabebestätigung und Zustellnachweis (ZUSE zuseorg); OE-5. |
| Ü4 Ereignisse und Bestätigungen | Übergabebestätigung, Zustellnachweis, Meldung bei Nichtabholung (ZustG §§ 29, 35; ZUSE zuseorg; WP 3.6). |
| Ü5 Interoperabilität | keine nach außen **[Indiz]** (Fehlanzeige in ZustG, WP, ZUSE, BKA-T, auch im lokalen Volltext); ERV als Zustellsystem. |
| Ü6 Malware-Prüfung | Zustellsystem prüft Anhänge auf Schadsoftware (zuseamod PDF-S. 103); Formate im TNVZ (ZustG § 28b Abs. 2; OE-6). |

### Infrastruktur und Betrieb

| Zeile | Österreich |
|---|---|
| I1 Betrieb und Governance | zugelassene Zustelldienste, zentrales TNVZ und Anzeigemodul (ZustG §§ 28a, 30, 37b; BKA-T). |
| I2 Datenhaltung und Versand | Bereithaltung beim Zustellsystem, Fristen und Löschung, Metadaten im Anzeigemodul (ZustG §§ 35, 36; OE-2; WP 3.6). |
| I3 Betriebsmodell für Organisationen | Behörden als Empfängerinnen über USP oder Portalverbund (WP 5; BKA-B). |
| I4 Funktionspostfächer | keine E2EE; alle Postbevollmächtigten sehen alles (ZUSE zusemsg; USP-4). |
| I5 Aufnahme in den Verbund | Zulassung per Bescheid, Gutachten, Aufsicht, Widerruf (ZustG §§ 30, 31). |

### Recht

| Zeile | Österreich |
|---|---|
| R1 Schriftformersatz | Rechtswirkung der Zustellung (ZustG §§ 35, 36); Beweiskraft des Ausdrucks (E-GovG § 20, Z. 2335). |
| R2 Teilnahmepflicht | Unternehmen mit Ausnahmen (E-GovG § 1b; OE-7 Z. 28–32; BKA-U). |
| R3 Bekanntgabe und förmliche Zustellung | Zustellzeitpunkt mit und ohne Zustellnachweis (ZustG §§ 35, 36; OE-2). |
| R4 Teilnahme der Behörden | Recht auf elektronischen Verkehr (E-GovG § 1a); Kosten, Freigrenze 25 Mio. (ZustG § 40, Z. 2284; WP 3.4; BKA-B). |

## 6. Widersprüche und offene Punkte

- **Verständigung:** OE-1 nimmt Nachrichten „nur zur Information“ (z. B. aus FinanzOnline) von der Verständigung aus; ZustG § 35 und die übrigen Quellen nennen diese Ausnahme nicht.
- **Antwortformular:** BKA-B bewirbt es als Vorteil, die ZUSE beschreibt nur Links in die Anwendung der Behörde. Ob es ein zentrales Formular im Anzeigemodul gibt, bleibt offen **[Indiz]**.
- **Aktualität:** Die oesterreich.gv.at-Themenseiten sind von 2024 (OE-4 bis OE-7), White Paper 06/2025, ZUSE 01/2025.

## 7. Lücken

- Nicht gefunden: Beispiel oder Spezifikation des Antwortformulars (P8), Aussagen zu EU-Interoperabilität (Ü5), Nutzungszahlen, Verschlüsselung at rest beim Zustellsystem im Detail.
- Nicht abgerufen: MOA-ZS-Dokumentation auf joinup.ec.europa.eu; XML-Schemata liegen vor, sind aber nicht ausgewertet.

## 8. Verwandte lokale Quellen

- `../Referenzen-Mein-Postkorb-Oesterreich.md`: Nachweise je Dimension und Evidenzstufen.
- `../MUK/MUK-Zusammenfassung.md`: deutsches Gegenstück für Organisationen (Postfach 2.0, OZG-PLUS-Postfach).
- `../BundID-ZBP/`: deutsches Gegenstück für natürliche Personen.
