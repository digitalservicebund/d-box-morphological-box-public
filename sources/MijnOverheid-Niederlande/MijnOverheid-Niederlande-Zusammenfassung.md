# MijnOverheid Berichtenbox (Niederlande): Zusammenfassung für die d-box

Wie die Niederlande ihr Behördenpostfach aufgesetzt haben und wohin sie es umbauen: heute die zentrale MijnOverheid Berichtenbox, künftig das Federatief Berichtenstelsel (FBS). Eingeordnet nach den Zeilen von `morph_box.csv`. Die Zusammenfassung verweist auf die Quellen, statt sie zu wiederholen.

Stand: 2026-11-26. Grundlage sind die Hauptquellen aus `../Referenzen-MijnOverheid-Niederlande.md`, am 2026-11-26 heruntergeladen. Alle Quellen liegen in `quellen/`, jeder Unterordner hat eine `_index.md` mit URL, Herausgeber, Datum und Inhaltsnotiz.

## Kennzeichnung

Wie in `../Referenzen-MijnOverheid-Niederlande.md`:

- Ohne Kennzeichnung: Stufe 3, in einer Primärquelle im Volltext belegt.
- **[Indiz]**: Stufe 2, Sekundärquelle, Proof of Concept, Rückschluss aus einer Primärquelle oder Fehlanzeige bei gezielter Suche.
- **[Annahme]**: Stufe 1, kein Beleg.

Aussagen zum FBS beschreiben ein Zielbild oder einen Pilot, nicht den Betrieb.

## Kürzel für Quellen

Kürzel wie in `../Referenzen-MijnOverheid-Niederlande.md`. Pfade relativ zu `quellen/`; Z. = Zeile der Textkopie, S. = Seite.

| Kürzel | Ordner | Inhalt |
|---|---|---|
| AWB, WMEBV, STB-*, WDO, BDO, RVWDO, AWR, RBB, DW | `recht-governance/` | Gesetze und Verordnungen: Awb Afdeling 2.3, Wmebv, Wet digitale overheid, Dienstenwet, Steuerrecht |
| KST-MVT, KST-816, KST-928, WMEBV-HR | `recht-governance/` | Gesetzesbegründung, Kamerbriefe zum FBS, Handreiking Wmebv |
| LOG-BB-*, DK-1 | `anbindung-technik/` | Anschluss an die MijnOverheid Berichtenbox: Aansluithandleiding 1.6.4, Testcheckliste, XSD, Digikoppeling |
| LOG-FBS-*, MOZA-1, MOZA-2, MOZA-POC-* | `anbindung-technik/` | FBS: Zielbild, Planung der Migration, Proof of Concept |
| DO-BBVB-*, RVO-1 | `anbindung-technik/` | Berichtenbox voor bedrijven, Behördenseite |
| MO-*, LOG-MO-*, DIGID-*, NLWW-*, TK-* | `betreiber/` | Nutzersicht MijnOverheid, DigiD, DigiD Machtigen |
| OP-*, EH-* | `betreiber/` | Berichtenbox voor bedrijven, Unternehmensseite, eHerkenning |

## 1. Systembild

- **Heute, Personen:** Die MijnOverheid Berichtenbox ist Teil des MijnOverheid-Kontos, das je BSN ab 14 Jahren besteht (MO-1 Z. 60–66; RVWDO Z. 140 ff.). Logius betreibt sie im Auftrag des Innenministeriums BZK (LOG-FBS-1 Z. 9; WDO Z. 159). Alle Nachrichten liegen zentral im Speicher GLOBE (LOG-FBS-3 Z. 10–11). Personen empfangen nur, sie antworten nicht über die Box (MO-1 Z. 110–116).
- **Heute, Unternehmen:** Die Berichtenbox voor bedrijven ist ein getrennter Dienst von RVO mit DICTU, mit eHerkenning und in beide Richtungen (DO-BBVB-1; OP-2). Daneben hat die Belastingdienst eigene Post in Mijn Belastingdienst Zakelijk (MOZA-1 Z. 40–50).
- **Anbindung:** Behörden liefern über Digikoppeling (ebMS2 im Batch, WUS für die Opt-in-Prüfung) mit PKIoverheid-Zertifikat ein, selbst oder über einen Intermediär (LOG-BB-2 Z. 18–38).
- **Zielbild FBS:** Die Nachricht bleibt im Nachrichtenmagazin der absendenden Behörde; die Behörde meldet sie im Verbund an, Portal und App lesen sie dort (LOG-FBS-2 Z. 24, 31–33; MOZA-2 Z. 40–49). Zentral laufen nur Nachrichtenliste, Benachrichtigung, Notificatieprofiel und digitale Erreichbarkeit (LOG-FBS-3 Z. 31–52). Behörden ohne eigenes Magazin nutzen das gemeinsame Magazin BBO (LOG-FBS-1 Z. 9–16).

## 2. Betrieb, Governance, Teilnahme

- **Stand der Umstellung:** Seit 2.11.2026 nimmt Logius keine neuen Anschlüsse an GLOBE an (LOG-FBS-6 Z. 9–11). 2027 sollen alle Absender auf BBO umziehen, mit einer kurzen Pause im digitalen Versand Ende April und neuen Anschlüssen ab Mitte Juni (LOG-FBS-5). Phase 2 mit eigenen Magazinen läuft als Proof of Concept und Pilot in der OBIS-Arbeitsgruppe Berichten (MOZA-2 Z. 9, 127–147).
- **Governance:** Sponsorgroep, Tactisch Overleg und Expertgroep unter der GDI-Governance (LOG-FBS-2 Z. 38–45; KST-816). Ein Afsprakenstelsel mit Stelselafspraken nach BOMOS ist angekündigt, aber nicht veröffentlicht [Indiz] (KST-816 PDF-S. 4; ACICT-1 PDF-S. 5).
- **Kosten:** Heute rechnet Logius Anschluss und Nutzung ab [Indiz]; für das FBS ist ein „toekomstbestendig kostenmodel“ geplant (LOG-FBS-1 Z. 15–16). BBO wurde für 2026 bis 2030 über 21 Mio. € ausgeschrieben; 2024 gab es über 103 Mio. Nachrichten von mehr als 400 Organisationen (TED-1).
- **Teilnahme der Behörden:** freiwillig; art. 5 Abs. 5 Wdo erlaubt eine Anschlusspflicht per Verordnung, die nicht genutzt wird [Indiz] (WDO Z. 209). Seit 1.1.2026 müssen alle Behörden elektronische Eingänge über benannte Kanäle annehmen (art. 2:13 Awb), die Berichtenbox ist dafür aber kein Kanal (WMEBV-HR Z. 877).
- **Teilnahme der Personen:** Opt-in je Behörde: Die Person wählt in MijnOverheid, von welchen Behörden sie digital Post erhält (art. 2:8 Awb; RVWDO art. 5). Sonderfall Belastingdienst: ganz elektronisch oder ganz Papier, für einzelne Nachrichten Pflicht per Regeling (AWR art. 3a, 3b).

## 3. Identität und Adressierung

- Adresse ist die BSN; vor dem Versand prüft die Behörde, ob die Person sie abonniert hat (LOG-BB-2 Z. 210–235).
- Anmeldung mit DigiD oder einem eIDAS-Mittel; Nachrichten mit Bezorgbevestiging nur mit DigiD-App, SMS oder Berichtenbox-App (MO-3 Z. 69).
- Unternehmen: Box-Name im KVK-Handelsregister als digitale Postadresse, Anmeldung mit eHerkenning ab EH2+ über private zugelassene Anbieter (OP-3; EH-2).
- FBS soll BSN, RSIN, KVK-Nummer und andere Kennungen verarbeiten (LOG-FBS-2 Z. 34–37).

## 4. Postfach, Vertretung und Organisationen

- Das Konto ist nicht kündbar; Nachrichten bleiben, bis die Person sie löscht (MO-1 Z. 95–98, 121–123). Nach einem Todesfall nehmen Behörden keine neuen Nachrichten an (MO-1 Z. 149–156).
- Vertretung über DigiD Machtigen je Organisation und Dienst, mit Enddatum und Freischaltcode per Brief an die Meldeadresse (DIGID-5). Bevollmächtigte lesen im eigenen Konto, nicht in der App und nicht bei Bezorgbevestiging (MO-1; MO-3 Z. 81). Gesetzliche Vertretung ist nicht abgebildet [Indiz] (TK-1).
- Unternehmen: ein Postfach je Unternehmen, mehrere Bevollmächtigte, Rechte im Machtigingenregister des eHerkenning-Anbieters (OP-2; EH-3).

## 5. Einordnung nach `morph_box.csv`

### Behörde

- **B1:** Ausprägung 1 (zentrale Schnittstelle, Digikoppeling, Intermediäre). Das FBS-Magazin passt in keine Ausprägung von B1; es ist unter I1, I2 und I3 eingetragen.
- **B2:** Ausprägung 1 nur für die Berichtenbox voor bedrijven; MijnOverheid hat keinen Web-Zugang zum Versenden [Indiz] und keinen hybriden Versand.
- **B3:** Ausprägung 2 (zentrales Verzeichnis mit Kennung), mit Opt-in je Behörde.
- **B4:** Ausprägungen 1, 2 und 5 mit engen Grenzen (4000 Zeichen, 500 kB).
- **B5:** Ausprägung 2 (PDF/A Pflicht für alle Anhänge).

### Person / Organisation

- **P1:** Ausprägungen 2, 3, 5; Hinweis gesetzlich Pflicht (art. 2:10 Awb).
- **P2:** Ausprägungen 1, 2, 4; eHerkenning als Ausprägung 6 für Unternehmen.
- **P3:** Ausprägung 2 für die Berichtenbox voor bedrijven.
- **P4:** Ausprägung 4 (DigiD Machtigen als zentrales Register).
- **P5:** Ausprägungen 1 und 2; für Unternehmen Ausprägung 6.
- **P6:** Ausprägung 1 [Indiz].
- **P7, P8:** Personen Ausprägung 1 (nur Empfang); Berichtenbox voor bedrijven P7 Ausprägung 4, P8 Ausprägungen 1 und 2.
- **P9:** Ausprägung 1 (Teil des MijnOverheid-Kontos).
- **P10:** Ausprägungen 3 und 6; neue Ausprägung 7 (Freischaltcode an die Meldeadresse).

### Übermittlung

- **Ü1:** Ausprägung 1. **Ü2:** Ausprägung 1. **Ü3:** Ausprägung 1. **Ü5:** Ausprägung 1 [Indiz]. **Ü6:** Ausprägung 7.
- **Ü4:** Ausprägungen 1, 2 (Bezorgbevestiging), 5 und 7; kein Lesestatus, aber Angabe zum Öffnen auf Anfrage (siehe 6.).
- **Ü7:** heute Ausprägung 3 über Digikoppeling; FBS nähert sich Ausprägung 4 [Indiz].

### Infrastruktur und Betrieb

- **I1:** heute Ausprägung 1; FBS Ausprägung 8 (Betrieb je Behörde mit gemeinsamen zentralen Bausteinen).
- **I2:** heute Ausprägung 1; FBS neue Ausprägung 8 (Speicherung beim Absender).
- **I3:** heute und FBS Phase 1 Ausprägung 1; FBS Phase 2 neue Ausprägung 4 (Absender-Magazin).
- **I4:** entfällt für MijnOverheid; Berichtenbox voor bedrijven Ausprägung 5.
- **I5:** Ausprägung 3 für Absender; FBS-Proof-of-Concept nähert sich Ausprägung 4 [Indiz].

### Recht

- **R1:** Ausprägung 1 nur in Richtung Behörde an Person über die Berichtenbox.
- **R2:** Ausprägung 2 (Opt-in je Behörde); Belastingdienst neue Ausprägung 6.
- **R3:** Ausprägung 1; keine förmliche Zustellung (Ausprägung 6) [Indiz].
- **R4:** Ausprägung 1 für MijnOverheid; Ausprägung 3 für die Berichtenbox voor bedrijven und für die Annahme elektronischer Eingänge.

## 6. Widersprüche und offene Punkte

- **Lesestatus:** FAQ und Aansluithandleiding sagen „kein Lesestatus“, Privacyverklaring und art. 8 Besluit digitale overheid erlauben auf Anfrage eine Angabe zum Öffnen für eine Erinnerung (MO-1 Z. 238–241; LOG-BB-2 Z. 91–93; MO-6 Z. 222; BDO Z. 517).
- **Pflicht für Unternehmen:** Die Handreiking nennt die Berichtenbox voor bedrijven verpflichtend, art. 14 Dienstenwet verlangt eine Erklärung des Unternehmens (WMEBV-HR Z. 491; DW Z. 253).
- **Hinweispflicht:** seit 1.1.2026 (art. 2:10 Abs. 1 Awb), nicht erst 2027, wie die Presse meldet; ab 2027 gilt nur Abs. 2 zum Inhalt (STB-2024-321).
- **Schnittstelle für Absender nach dem Umzug auf BBO:** Ob die ebMS/WUS-Anbindung bleibt, ist nicht veröffentlicht.

## 7. Lücken

- Stelselafspraken, Aansluitvoorwaarden und Tarife des FBS nicht veröffentlicht.
- Malware-Prüfung, Verschlüsselung der Ablage und Signatur auf Dokumenten nicht dokumentiert.
- machtigen.digid.nl und NORA-PDFs wegen Bot-Schutz nicht abrufbar; Logius-Voorwaarden HTTP 404.
- Intervall der Erinnerung bei Nichtabruf nur aus einem Forum von 2018.

## 8. Verwandte lokale Quellen

- `../Referenzen-MijnOverheid-Niederlande.md`: Nachweise je Dimension und Evidenzstufen.
- `../Mein-Postkorb-Oesterreich/`: zum Vergleich ein Verbund mit zugelassenen Zustelldiensten und zentralem Anzeigemodul.
- `../Digital-Postkasse-Norwegen/`: zum Vergleich ein Verbund mit privaten Postfachanbietern.
