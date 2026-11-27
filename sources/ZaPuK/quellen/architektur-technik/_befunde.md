# Befunde: ZaPuK – Architektur und Technik

Stand der Auswertung: 2026-11-27 (Erstauswertung 2026-11-19). Dateien siehe `_index.md`. Hauptquelle ist die lokale Kopie des Tags `v1.1.1` in `../zapuk-v1.1.1/` („Fortschreibung nach der 51. Sitzung“, Architekturkonzept Stand 26.11.2026; Programm-README `../zapuk-v1.1.1/README.md` Z. 77–80). Fundstellen: "Z." = Originalzeile der genannten Datei in `../zapuk-v1.1.1/`, ohne Kopfversatz. Ausnahme: ZF11, TR, TR-A bis TR-C, TADR-E2 und ISS zählen weiter die Zeilen der Kopien in diesem Ordner (Originalzeile = Z. minus 5). Kennzeichnung: **[Folgerung]** eigene Ableitung, **[unverifiziert]** ohne ausreichenden Beleg, **[veraltet?]** Quelle älter oder durch neuere Fassung überholt, **[v1.1]** nur in den Dokumenten der Version 1.1 (Architekturkonzept v1.1.1, Zusammenfassung, Transitionen), nicht in einem ADR. Die Version 1.1 liegt dem IT-PLR-Beschluss B-2026/43-IT als Anlage bei; als „beschlossen“ bezeichnet das Architekturkonzept nur die Version 1.0 (AK11 Z. 7). Der frühere Marker **[Entwurf]** (nicht gemergter Branch `architecture-v1.1`) entfällt.

Status der ADRs nach dem Index in v1.1.1 (ADR-IDX Z. 17–66): "Operativ entschieden" = auf Arbeitsebene des FIT-AM gefasst, offene Punkte ändern die Entscheidung nicht; "Richtung entschieden" = Richtung steht, offene Punkte können Festlegungen noch ändern; "In Arbeit" = ADR wird ausgearbeitet, Optionen werden erörtert; "Unbearbeitet" = Bedarf erkannt, kein ADR (Legende Z. 81). ADR-0000 bis ADR-0016 sind zusätzlich Teil des IT-PLR-Beschlusses 2025/28 (Z. 17–33). Für spätere ADRs nennt der Index keinen IT-PLR-Beschluss, auch nicht B-2026/43-IT.

## Kürzel und Pfade

Alle Pfade relativ zu `sources/`. `V` = `ZaPuK/quellen/zapuk-v1.1.1/`.

- ADR-xxxx = `V/Zielarchitektur/Architekturentscheidungen/ADR-xxxx-….md` (alle ADRs, auch 1121 und 1122)
- ADR-IDX = `V/Zielarchitektur/Architekturentscheidungen/README.md`
- GLOS = `V/Glossar.md`; ABK = `V/Abkürzungen.md`
- AK11 = `V/Zielarchitektur/Architekturkonzept/README.md` (Architekturkonzept v1.1.1, Stand 26.11.2026) **[v1.1]**
- AK10 = `V/Zielarchitektur/Architekturkonzept/Vorversionen/v1.0/README.md` (Architekturkonzept v1.0, Sommer 2025; Zeilen gleich der früheren Fassung in `zapuk-main/`)
- ZF10 = `V/Zielarchitektur/Zusammenfassung/Vorversionen/v1.0/README.md` (Zeilen gleich der früheren Fassung in `zapuk-main/`)
- ANF = `V/Konsolidierte-Anforderungen/README.md` (v0.7.1, 24.02.2026, unverändert); RB = `V/Zielarchitektur/Rahmenbedingungen/README.md`
- ZF11 = `ZaPuK/quellen/architektur-technik/fitko_zapuk-zusammenfassung-v1.1_v1.1-entwurf_2026-11-13.md`; TR = `ZaPuK/quellen/architektur-technik/fitko_zapuk-transitionen-ueberblick_v1.1-entwurf_2026-11-13.md`; TR-A, TR-B, TR-C = Transitionspläne ZBP, MJP, MUK (`fitko_zapuk-transition-a-…`, `-b-…`, `-c-…`); TADR-E2 = `fitko_zapuk-tadr-mein-elster-002-…` (Entwurfsstand 13.11.2026; Abgleich mit v1.1.1 nicht Teil dieser Auswertung, Transitionspläne und TADRs fehlen in v1.1.1)
- ISS = `ZaPuK/quellen/architektur-technik/fitko_zapuk-opencode-issues-architektur_2026-11-19.md`

Alte Kürzel und Zeilenangaben (Kopien `*_main_2026-11-12.md`, `*_v1.1-entwurf_2026-11-13.md`, `zapuk-main/`) rechnet der Abschnitt [Konkordanz der Fundstellen](#konkordanz-der-fundstellen-alt--v111) um.

## Kurzbild der Architektur

1. **Topologie:** Hybride, föderierte Client-Server-Architektur (ADR-0002 Z. 94). Vier zentrale Postfach-Backends je Teilnehmendengruppe (Privatpersonen, private Organisationen, öffentliche Stellen, Organisationen der Justiz). Das Backend für Privatpersonen hat einen anderen Betreiber als die Backends für Organisationen (ADR-1114 Z. 64–72, operativ entschieden 09.10.2026). Organisationen dürfen eigene Backends betreiben (ADR-0004 Z. 69), Privatpersonen nicht (ADR-1114 Z. 72; AK11 Z. 825, Z. 1207).
2. **Protokoll:** Matrix als Kommunikationsschicht (ADR-0011 Z. 439), MLS als Ziel-E2EE, übergangsweise Olm/Megolm (ADR-0012 Z. 202–209). E2EE endet im Endgerät (ADR-1100 Z. 57–65). Bei Funktionspostfächern ist zusätzlich ein technischer Nutzer mit Schlüsseln im Machtbereich der Organisation zulässig (ADR-1101 Z. 114–134).
3. **Zugänge:** Ein zentraler Postfach-Zugang für alle Gruppen (ADR-0005 Z. 88). Laut v1.1 im Bundesportal und in der Deutschland-App, als Web-App und mobile App (AK11 Z. 303, Z. 493). Daneben beliebige Drittzugänge ohne Zulassung (ADR-0003 Z. 71–78): Fachverfahren, Unternehmensanwendungen, Fachportale wie ELSTER (AK11 Z. 589–614).
4. **Anbindung:** Zentrales SSP (ADR-0008 Z. 61; ADR-1148 Z. 67) und SDK (ADR-0009 Z. 59), das SDK ist optional, die Matrix-API reicht (AK11 Z. 635). Laut v1.1 entwickelt FIT-Connect die Backends, betreibt die Backends für Organisationen und bringt SSP und SDK ein (AK11 Z. 287–299, Z. 376–387, Z. 424).
5. **Identität und Adressen:** Postfach getrennt vom Nutzerkonto (ADR-1110 Z. 74–80). Privatpersonen: bereichsspezifische pseudonyme MXID, ein Zuordnungsdienst verknüpft BundID, eID und EUDI-Wallet (ADR-1105 Z. 204–226, Richtung entschieden). Organisationen: föderiertes Adressregister (ADR-1111 Z. 73). Fachliche Identität über einen Identitätsnachweis je Nachricht im E2EE-Kanal (ADR-1106 Z. 83–89; ADR-1107 Z. 80).
6. **Nachweise:** Bereitstellungsbestätigung durch das Empfänger-Backend (ADR-0018 Z. 130–146). Abruf- und Lesebestätigungen clientseitig, optional (ADR-1118 Z. 74–110); ob zusätzlich das Backend den Abruf bestätigt, ist offen (ADR-1121, In Arbeit). Benachrichtigung durch das Empfänger-Backend mit Benachrichtigungsbestätigung auf Anforderung (ADR-1122, In Arbeit). Vom Absender gesteuerte Übermittlungsoptionen (ADR-1119 Z. 45–57).
7. **Bestandssysteme:** v1.0 verbietet Messaging-Bridges und erlaubt für Frontends Channel Adapter (AK10 Z. 401–403). v1.1 sieht in der Transition Bridges vor: ZBP-Matrix-Bridge, Zustell-Bridge in der ELSTER-Domäne, Postfach-2.0-Matrix-Bridge (AK11 Z. 718; TR Z. 63, Z. 85–92, Z. 113, Z. 117). EGVP bleibt bestehen und wird über einen OSCI-Matrix-Protokollkonverter angebunden (TR Z. 96–101).

## Änderungen v1.1.1 gegenüber `main` 2026-11-12 und Entwurf 2026-11-13

Verglichen wurden alle Dateien dieses Ordners (ohne die fünf Kopfzeilen) und `zapuk-main/` mit `../zapuk-v1.1.1/`, Umfang: ADR-Index, alle ADRs, Architekturkonzept v1.1 und v1.0, Glossar, Abkürzungen, Konsolidierte Anforderungen (README, Lesefassung), Rahmenbedingungen, `Zielarchitektur/README.md`, Konsultationen. Zusammenfassung, Transitionen, Rechtsgutachten und Programm-README sind hier nicht ausgewertet.

### Inhaltlich

1. **Status v1.1:** Das Architekturkonzept ist kein Entwurf mehr. Es „schreibt die Fassung fort, die dem IT-Planungsrat zur 51. Sitzung vorgelegt wurde (Version 1.1). Sie liegt dem Beschluss B-2026/43-IT als PDF bei“; „beschlossen“ heißt nur Version 1.0 (AK11 Z. 5, Z. 7). `Zielarchitektur/README.md` Z. 13 nennt das Architekturkonzept jetzt „Fortschreibung“ von v1.0, der Hinweis „Stand Sommer 2026“ entfällt; Transitionsstrategie und Rechtsgutachten sind verlinkt (Z. 26–27).
2. **ADR-1122 Benachrichtigungsdienst für Bereitstellungen (neu, In Arbeit, 23.11.2026; ADR-IDX Z. 55, vorher „Unbearbeitet“):** Gewählt Option A, Benachrichtigung durch den Benachrichtigungsdienst im Empfänger-Backend per E-Mail oder Push, Kontaktdaten je Bereichs-MXID (Z. 29, Z. 56). Die Behörde darf zusätzlich selbst benachrichtigen (Option B). Option C, Benachrichtigung durch das Nutzerkonto, ist ausgeschlossen (Z. 56). Die Empfänger:in wählt die Kanäle und kann je Vorgang abschalten; der Text ist für alle Nachrichten gleich und nennt weder Bereich noch Behörde noch Vorgang (Z. 58). Auf Anforderung je Nachricht bestätigt der Quittungsdienst signiert und unverschlüsselt im Raum, dass und wann benachrichtigt wurde, oder meldet, dass nicht benachrichtigt wurde; dann gibt die Behörde auf andere Weise bekannt (Z. 60). Die Bestätigung belegt nur die Absendung, nicht den Zugang; für § 37 Abs. 2a SGB X fordert die Behörde zusätzlich die Abrufbestätigung an (Z. 62). Folgen: neue Übermittlungsoption, Quittungsdienst erzeugt die Benachrichtigungsbestätigung, Benachrichtigung als Anschlussbedingung für Backends der Privatpersonen (Z. 66–68, Z. 77). Offen: ob Push als elektronische Benachrichtigung genügt, Nachweis des Zugangs, Verkettung der Bereichs-MXIDs über dieselbe E-Mail-Adresse oder dasselbe Gerät (Z. 81–83). SMS, Erinnerung und Brief kommen nicht vor.
3. **ADR-1121 Ausgestaltung der Abrufbestätigung (neu, In Arbeit, 23.11.2026; ADR-IDX Z. 54):** Optionen A (nur Client, wie ADR-1118), B (nur Quittungsdienst des Backends bestätigt das Herunterladen eines verschlüsselten Anhangs) und C (beides) (Z. 32–58). Entscheidung offen wegen rechtlicher Fragen, u. a. Anpassung von § 9 OZG; bis dahin gilt Option A (Z. 62). Bei B oder C entstünde ein für die Betreiber sichtbarer Abrufzeitpunkt, und Zugänge dürften Anhänge erst auf Anforderung laden (Z. 49, Z. 58, Z. 68–70). Offen: Rechtsgrundlage für entkoppelte Postfächer, ob ein Download als Abruf gilt, Ende der Bereitstellung nach zehn Tagen (§ 41 Abs. 2a Satz 4 VwVfG) (Z. 78–80).
4. **ADR-1118:** Offener Punkt „Nachweis der Benachrichtigung“ aus dem Entwurf entfällt, Folge-Entscheidung ADR-1121 (Z. 135). Neues Risiko: Ohne Anpassung von § 9 OZG ist der tatsächliche Abruf maßgeblich, den nur die Abrufbestätigung belegt; ob sie freiwillig bleiben kann, ist zu prüfen (Z. 127). Die Bereitstellungsbestätigung ist „Nachweis der Bereitstellung“ statt „des Zugangs“ (Z. 102, Z. 106). Datum 26.11.2026.
5. **ADR-0018 / Glossar:** Die Bereitstellungsbestätigung „dokumentiert diesen Eingang“ statt „diesen Zugang“ (ADR-0018 Z. 9). Neu im Glossar: Der Zugang setzt neben dem Eingang voraus, dass die Empfänger:in entschlüsseln kann, „das belegt die Bestätigung nicht“ (GLOS Z. 231). Rechtsgrundlage um § 37 Abs. 2a SGB X und § 122a AO ergänzt (ADR-0018 Z. 234, wie Entwurf).
6. **ADR-1119:** Neue Übermittlungsoption „Benachrichtigungsbestätigung anfordern“ (Backend, für § 37 Abs. 2a SGB X, Folge-Entscheidung) (Z. 53, Z. 91).
7. **ADR-1110:** Die Risiken „Bestandslösungen“ (Nutzerkonto-Postfächer laufen bei verzögerter Governance-Änderung parallel weiter) und „Zweite Adresse“ aus dem Entwurf sind entfallen; es bleibt nur „§ 9 OZG anzupassen“ (Z. 90). Datum 26.11.2026.
8. **ADR-1105:** Weitere Authentisierungssysteme werden nur verknüpft, „sofern die Person die Verknüpfung vornimmt“ (Z. 141); Zuordnungsdienst ohne Identitätsdaten (Z. 139, wie Entwurf). Status weiter „Richtung entschieden“.
9. **ADR-1147:** Neuer Treiber Schlüsselverwaltung (Z. 23), neue Konsequenzen: kein Schlüsselzugriff für Betreiber und Behörden, archivierte Nachweise bleiben nach Schlüsselverlust prüfbar (Z. 97, Z. 99). Status weiter „Richtung entschieden“, Datum 26.11.2026.
10. **ADR-1145:** Neue Anschlussbedingung „Verwahrung der Wiederherstellungsschlüssel“ (Z. 137, wie Entwurf); neue Konsequenz: ADR-0004 wird eingeschränkt, Anschluss nur mit Anschlussvertrag (Z. 150). Datum 26.11.2026.
11. **ADR-1146:** Prüfauftrag präzisiert: Ohne vorherige Kommunikation kann eine Behörde keine Bescheide übermitteln, da der Zuordnungsdienst kein Adressregister ist (Z. 81). Datum 26.11.2026.
12. **ADR-1135:** Neues Risiko: Übernimmt eine Organisation Nachweise und Chiffrat nicht vollständig, ist die Übermittlung später nicht mehr beweissicher (Z. 138). Datum 26.11.2026.
13. **ADR-1106, ADR-1101, ADR-0004, ADR-1130:** Stand des Entwurfs übernommen (Holder-Binding als offener Punkt, ADR-1106 Z. 31; „E2EE nicht strikt umgesetzt“, ADR-1101 Z. 96; Hinweis Phase 2, ADR-0004 Z. 3; Hinweis bei Option E, ADR-1130 Z. 159).
14. **ADR-Index:** ADR-1121 und ADR-1122 „In Arbeit“ (Z. 54–55); unbearbeitet bleiben ADR-1149 API-Autorisierung, Löschung und Aufbewahrung, Metadatenminimierung, nicht entschlüsselbare Nachrichten (Z. 63–66). Tippfehler „12.12.2026“ korrigiert (Z. 6).
15. **Architekturkonzept v1.1.1 gegenüber Entwurf:** Kap. 3 Geschäftsarchitektur ist jetzt ausgeführt (Prozesssicht, Rollen, Aktivitäten, Informationssicht, Z. 156–249; vorher nur Verweis auf v1.0); dadurch verschieben sich die Zeilen ab Kap. 4 um 87. Benachrichtigungsdienst mit Beleg ADR-1122 (Z. 653), Abrufbestätigung mit ADR-1121 (Z. 755), beide in der ADR-Liste (Z. 803–804). Nachweisstelle dem Backend „zugeordnet“ statt „Bestandteil“ (Z. 695). Berechtigungsverzeichnis im Betriebsmodell B auch zentral (Z. 703). Justiz-Backend ersetzt das MJP-Backend zusammen mit dem Backend für Privatpersonen und dem Protokollkonverter (Z. 445). „Verwaltungsportal Bund“ nicht mehr als Fachportal genannt (Z. 606). Neue Fußnote 8 zur Föderalen Referenzarchitektur (Z. 1217).
16. **Glossar:** Neue Einträge Anonymität (Z. 63), Benachrichtigungsbestätigung (Z. 179), Benachrichtigungsdienst (Z. 187), bereichsspezifisches Personenkennzeichen (bPK, Österreich; Z. 213), Bereitstellung (Z. 219), Unbeobachtbarkeit (Z. 817), Zweckbindung (Z. 943, Auswertung von Metadaten nur zweckgebunden laut Anschlussbedingungen). Bekanntgabe EN jetzt *disclosure*; Nachricht = ein oder mehrere Events (Z. 545). Abkürzungen: bPK, E-GovG, SDM ergänzt.

### Nur redaktionell oder Linkpfade

- ADR-0000 bis ADR-0016, ADR-0017, ADR-1100, ADR-1102 bis ADR-1104, ADR-1107 bis ADR-1109, ADR-1111 bis ADR-1114, ADR-1120, ADR-1144, ADR-1148: Text gleich `main` 2026-11-12 (gegenüber `zapuk-main/` nur Status „Operativ entschieden“ statt „Beschlossen“ und die Änderungen vom 12.11.2026, siehe `_index.md`).
- ADR-0018, ADR-1101, ADR-1130 gegenüber Entwurf: Zeichensetzung, Formatierung, Anforderungs-ID (ADR-1101 Z. 106 SIC_VER_ÜBE_026).
- Architekturkonzept v1.0 (`Vorversionen/v1.0/README.md`): nur PDF-Link und absolute Linkpfade, Zeilen 1:1 wie `zapuk-main/`. Ebenso Zusammenfassung v1.0 (`Zusammenfassung/Vorversionen/v1.0/README.md`).
- Konsolidierte Anforderungen README und Vorversionen: bytegleich. Lesefassung, Rahmenbedingungen, Konsultationsseiten Phase 1 und 2: nur Linkziele (PDF-Links auf it-planungsrat.de, Architekturkonzept auf `Vorversionen/v1.0/`, Leitfragen auf Issue-Vorlage).
- Glossar und Abkürzungen haben in v1.1.1 CRLF-Zeilenenden; die Kopien in diesem Ordner LF.

## Befunde je Dimension

### B1 Anbindung von Fachverfahren

Festlegung:
- Fachverfahren sind Postfach-Zugänge (Matrix-Clients) über die offene API oder das SDK. Eine Zulassung gibt es nicht (ADR-0003 Z. 71–78, Option 4 "Offenheit für Drittanwendungen", operativ entschieden 15.04.2025; AK11 Z. 610, Z. 837).
- SDK zentral bereitgestellt (ADR-0009 Z. 59, operativ entschieden 14.04.2025). Es ist nicht verpflichtend, die API ist ohne SDK vollständig nutzbar (AK11 Z. 635). Automatisierte Einlieferung (M2M) mit OAuth-2.0-Client-Credentials über die föderale API-Autorisierungsinfrastruktur (FöPD), Systemidentität über X.509 (V-PKI, ELSTER-Organisationszertifikat) (AK11 Z. 637, Z. 1118–1119) **[v1.1]**. ADR-1149 "API-Autorisierung der Server-zu-Server-Zugriffe" ist unbearbeitet (ADR-IDX Z. 63).
- Registrierung der Organisation im zentralen SSP, automatisiert. Nur Backends werden manuell freigegeben (ADR-0008 Z. 61; ADR-1148 Z. 67, operativ entschieden 12.10.2026).
- Anbindung immer nur an ein Backend des Verbunds (ADR-1114 Z. 72; AK11 Z. 374).
- FIT-Connect: v1.0 empfiehlt FIT-Connect als zentralen Anbindungsweg, Voraussetzung Matrix und MLS (ZF10 Z. 232). v1.1 konkretisiert das: FIT-Connect entwickelt die Backends, betreibt die Backends für Organisationen, das SDK erhält ein ZaPuK-Modul. Antragstellung und Folgekommunikation laufen über dieselbe Anbindung (AK11 Z. 132, Z. 287–299, Z. 434–435, Z. 441; ZF11 Z. 99, Z. 253–257; TR Z. 40; TR-A Z. 127) **[v1.1]**.
- Als begründete Ausnahme dürfen Fachverfahren ohne Matrix-Integration die E2EE an der Organisationsgrenze enden lassen (ADR-1101 Z. 130, Option C).

Erwogen und verworfen:
- FIT-Connect als Kommunikationsschicht (ADR-0011 Z. 321ff., Option 6: "nationale Eigenentwicklung", hohe Aufwände für Messaging- und Postfachfunktionen), ebenso OSCI/XTA (Option 5, Z. 266ff.), AS4, XRoad.
- Zulassung oder Zertifizierung von Postfach-Zugängen (ADR-0003 Optionen 1–3, Z. 28–58).
- Mehrere fachlich zugeschnittene SSPs (ADR-0008 Option 2, Z. 49).

Bestandssysteme:
- v1.0: Bestandslösungen müssen auf Matrix und MLS migrieren. Messaging-Bridges sind verboten, Channel Adapter nur für die Integration von Frontends, außerhalb der E2EE (AK10 Z. 393–409, Kap. 3.2). **[veraltet?]**: v1.1 hat kein Brownfield-Kapitel mehr (Kap. 3 ist jetzt die Geschäftsarchitektur, AK11 Z. 156–249), der Begriff Channel Adapter fehlt. v1.1 unterscheidet Protokollkonverter (E2EE bleibt) von Bridges, die in der Transition übergangsweise vorgesehen sind und Nachrichten annehmen, die in der Bestandslösung ohnehin nicht E2EE-verschlüsselt vorliegen (AK11 Z. 718; TR Z. 63, Z. 71).
- ZBP-API bleibt übergangsweise als "Legacy-Fassade" über eine ZBP-Matrix-Bridge, ohne Neu-Registrierungen (AK11 Z. 411; TR Z. 85–92; TR-A Z. 97, Z. 115, Z. 137) **[v1.1]**.
- Mein ELSTER: Zustell-Bridge in der ELSTER-Domäne, später Anbindung über das SDK, dauerhafter Parallelbetrieb (TADR-E2 Z. 44–48, Z. 70, operativ entschieden 25.09.2026; TR Z. 117) **[v1.1]**.
- EGVP bleibt und wird über einen OSCI-Matrix-Protokollkonverter am Justiz-Backend angebunden (AK11 Z. 425, Z. 714–716; TR Z. 96–101).

Offen: Sprachen und Plattformen der SDKs (ADR-0009 Z. 63); ADR-1149; FIT-Connect-Produktschnitt "vorbehaltlich" (AK11 Z. 267, Z. 387).

### B2 Zugang ohne Fachverfahren

- Der zentrale Postfach-Zugang (Web und App) steht allen Teilnehmendengruppen zur Verfügung, auch öffentlichen Stellen (ADR-0005 Z. 32–36, Z. 88; AK11 Z. 835). Funktionspostfächer einer Behörde ohne eigene IT: Betriebsmodell A mit zentralem RMS (ADR-1103 Z. 50–62).
- Ein Versanddienst oder Upload für Massenversand und ein Papier-Rückfall sind in ZaPuK nicht vorgesehen **[unverifiziert]** (in ADRs, AK10, AK11 und ANF nicht gefunden, auch nicht in v1.1.1).
- Drittclients als Zugang sind ohne Zulassung möglich (ADR-0003 Z. 71–78).

### B3 Adressierung der Person / Organisation

Festlegung Privatpersonen (ADR-1105, Richtung entschieden seit 12.11.2026, Datum der Fassung 13.11.2026, zuvor operativ entschieden 06.11.2026):
- Teil 1: bereichsspezifische MXID (Option 1B) (Z. 204). Bereich = eine oder mehrere Behörden mit derselben Bereichs-MXID (GLOS Z. 201–211).
- Teil 2: Ein Zuordnungsdienst je Bereich (2E) oder zentral (2E’) ordnet die Kennungen von BundID, eID und EUDI-Wallet derselben zufälligen Bereichs-MXID zu (Z. 210–216). Weitere Authentisierungssysteme werden nur verknüpft, wenn die Person es tut (Z. 141). Zielbild 2C: Pseudonym aus einem personengebundenen Schlüssel, ggf. aus dem Melderegister, wie das österreichische bPK aus der Stammzahl (Z. 117, Z. 218; GLOS Z. 213).
- Die Behörde erhält die Bereichs-MXID mit dem Antrag. Ein Erstkontakt ohne vorheriges Verfahren ist nicht möglich, ein Adressregister für Privatpersonen gibt es nicht (Z. 220, Z. 245; ADR-1146 Z. 81; AK11 Z. 478, Z. 1102, Z. 1112). Die Aufnahme einer Postfachadresse ins Melderegister wird geprüft (AK11 Z. 482, Z. 1114).
- Verworfen: globale MXID 1A (Z. 206), vorgangsspezifische MXID 1C "zunächst nicht" (Z. 208), Pseudonym nur über BundID 2A (Z. 222), BundID-Pseudonyme in der EUDI-Wallet 2B (Z. 224), verschiedene Postfächer je Authentisierungssystem 2D, ausgeschlossen (Z. 226).
- Offen: Zuschnitt der Bereiche und damit Wahl 2E oder 2E’, Betreiber der Zuordnungsdienste, Wiederherstellung nach Ausweiswechsel, Betreibertrennung von Zuordnungsdienst und Nachweisstelle (Z. 289–301; ADR-IDX Z. 41).

Festlegung Organisationen:
- MXID ohne Personenbezug: `@[orgpräfix]_[funktion]:post.gov.de` zentral, `@[funktion]:[org-domain]` souverän, Privatpersonen `@[bereichspseudonym]:privat.post.gov.de` (ADR-1104 Z. 52–54, Z. 62–79, operativ entschieden 11.11.2026). Eigene Domain per `.well-known` auch ohne eigenen Homeserver (Z. 65).
- Föderiertes Adressregister: Jedes Backend führt die Einträge seiner Domain, signiert im Matrix-Profil. Abfrage nach Organisationsmerkmalen und Prüfung einer bekannten Adresse (ADR-1111 Z. 73, Option D, operativ entschieden 12.11.2026). Verworfen: Adresse als Identitätsmerkmal (A), separater Adressnachweis (B), Verwendung ohne Prüfung (C) (Z. 23–58). Offen: Weiterleitung oder Replikation bei der Suche über Domains, MSC4258 (AK11 Z. 946, Z. 1078).
- Justiz: SAFE wird als Adressregister nachgenutzt (AK11 Z. 384, Z. 477). Adressen im Justiz-Backend nach dem Muster `@[SAFE-ID]:domain` (TR-B Z. 37) **[v1.1]**.
- Berufsträger dürfen mit ihrer MXID in Kammerregistern geführt werden (AK11 Z. 1086) **[v1.1]**.

### B4 Inhalt an die Person / Organisation

- Nachrichtentext mit Formatierung, Tabellen, Links, Medien (ANF Z. 90–94); Anhänge beliebigen Formats, per Allowlist einschränkbar (ANF Z. 87–88). Anhänge liegen verschlüsselt im Content Repository und werden auf Anforderung proaktiv ins Empfänger-Backend gezogen (ADR-1120 Z. 110–114; AK11 Z. 652, Z. 1129).
- Strukturierte Daten und fachliche Metadaten (Aktenzeichen, Antrags-ID) sind Anforderungen (ANF Z. 85, Z. 95–99). ADR-0001 erlaubt wenige querschnittliche Erweiterungen, etwa Terminabfragen und Abstimmungen, aber keine Fachlogik (ADR-0001 Z. 57). Vordefinierte Antwortoptionen (ANF Z. 101, ANF_FUN_NAC_125). Statusinformationen zum Verfahren (ANF Z. 100). Die Statusmonitor-Daten des ZBP werden ins Matrix-Backend migriert (TR-A Z. 124) **[v1.1]**.
- Gesiegelte oder signierte Dokumente: Die Behörde siegelt vor dem Versand, z. B. über den Siegeldienst des Bundes; das System transportiert nur und prüft keine Siegel (ADR-0017 Z. 57–65, Z. 67–75, operativ entschieden 24.09.2026).
- Nachweise für die EUDI-Wallet kommen nicht als Anhang, sondern als Abholschein (OID4VCI Credential Offer) zusammen mit dem Bescheid (ADR-1146 Z. 66–76, operativ entschieden, Datum der Fassung 26.11.2026; AK11 Z. 983). Verworfen: Anhang-Lösung, indirekte Ausstellung über BundID/ZBP (Z. 74).

### P1 Benachrichtigung

- Anforderung: Benachrichtigung per E-Mail oder App-Push (ANF Z. 153, ANF_FUN_ZUS_026).
- Der Benachrichtigungsdienst ist Teil jedes Postfach-Backends (SMTP, APNs/FCM) und relevant für Bekanntgabefristen (AK11 Z. 653) **[v1.1]**. Im Anwendungsfall bekommt die Person einen Push auf das Mobilgerät (AK11 Z. 559) **[v1.1]**.
- ADR-1122 (In Arbeit, ADR-IDX Z. 55): Das Empfänger-Backend benachrichtigt per E-Mail oder Push nach den Einstellungen der Empfänger:in, je Kanal und Vorgang abschaltbar; der Text ist einheitlich, ohne Bereich, Behörde oder Vorgang; Kontaktdaten je Bereichs-MXID (Z. 29, Z. 56–58). Die Behörde kann zusätzlich selbst benachrichtigen (Option B), Benachrichtigung über das Nutzerkonto ist ausgeschlossen (Option C, Z. 56). Benachrichtigungsbestätigung auf Anforderung; meldet der Quittungsdienst, dass nicht benachrichtigt wurde, gibt die Behörde auf andere Weise bekannt (Z. 60).
- Offen: ob ein Push als elektronische Benachrichtigung genügt, Nachweis des Zugangs der Benachrichtigung (§ 37 Abs. 2a SGB X), Verkettung der Bereichs-MXIDs über dieselben Kontaktdaten (ADR-1122 Z. 81–83). SMS, Erinnerung bei Nichtabruf und Brief als Rückfall sind nicht behandelt **[unverifiziert]** (auch in ADR-1121, ADR-1122 und AK11 v1.1.1 nicht; ADR-1121 Z. 80 nennt nur das Ende der Bereitstellung nach zehn Tagen als offen).

### P2 Authentifizierung

- Privatpersonen: eID, BundID und perspektivisch EUDI-Wallet (ADR-0014 Z. 92–97, operativ entschieden 28.04.2025). Die BundID bringt eigene Mittel mit (Passwort, ELSTER-Zertifikat, eID, eIDAS) auf den Niveaus normal bis hoch (Z. 67).
- eID direkt bei der Behörde, ohne BundID und Nachweisstelle (Kanalbindung, Option A); BundID und Org-Konto übergangsweise über einen kurzlebigen Identitätsnachweis der Nachweisstelle (Option C). Verworfen: Nachweis exklusiv je Nachricht (Option B) (ADR-1106 Z. 83–89, Richtung entschieden seit 12.11.2026; offen: gebundener Schlüssel, Gültigkeit der eID-Bindung, ADR-IDX Z. 42; Holder-Binding als offener Punkt, ADR-1106 Z. 31).
- Öffentliche Stellen: X.509 aus der V-PKI und Organisationskonto; verworfen: öffentliche PKI; EUDI-Wallet perspektivisch (ADR-0015 Z. 102–107). Private Organisationen: X.509 aus einer PKI (ELSTER-PKI ertüchtigen) und Organisationskonto (ADR-0016 Z. 85–88; AK11 Z. 883, Z. 915).
- Eigenes IdM einer Organisation über einen Org-IdP-Adapter als Nachweisstelle (ADR-1108 Z. 58, operativ entschieden 12.11.2026). Damit überholt: "Eigene Identity Provider … werden nicht unterstützt" (AK10 Z. 345) **[veraltet?]**.
- Vertrauensniveau im Nachweis als Claim. Der Absender kann ein Mindestniveau für das Lesen verlangen (AK11 Z. 911; ADR-1119 Z. 57).

### P3 Organisationszugang

- Funktionspostfach als fachliches Objekt mit eigener MXID. Jeder Vorgang bekommt einen eigenen Raum, ein technischer Nutzer (RMS) lädt Berechtigte ein (ADR-1102 Z. 74–86, Option C, operativ entschieden 12.11.2026).
- Berechtigungsverzeichnis zentral im Backend oder organisationsintern mit eigenem IdM (ADR-1112 Z. 56–58, Option C, operativ entschieden 09.10.2026); laut v1.1 im Betriebsmodell B auch zentral (AK11 Z. 703). Verworfen: Pflege im Org-Konto (A, Z. 26) und im SSP (B, Z. 36).
- Verworfen: geteilte Zugangsdaten und Schlüssel (ADR-1102 Option A, Z. 38–48, "widerspricht Zero Trust"), statische Gruppenräume (Option B, Z. 50–58).
- Mitarbeitende treten nach außen nicht persönlich auf, intern bleibt über das Berechtigungsverzeichnis nachvollziehbar, wer gehandelt hat (AK11 Z. 940).

### P4 Vertretung

- Die handelnde Person nutzt ihre eigene Identität und ihr eigenes Postfach. Vollmacht oder gesetzliche Vertretung wird als Ergänzung des Identitätsnachweises oder als signiertes Dokument übermittelt, die Prüfung liegt beim Empfänger (ADR-1113 Z. 53–59, Option A, operativ entschieden 24.09.2026).
- Verworfen: Zugriff auf das Postfach der vertretenen Person (Option B, Z. 39–51: keine strikte E2EE, Datenschutzprobleme).
- Vollmachts- und Vormundschaftsregister entwirft ZaPuK nicht (Z. 57). "Ein zentrales Vollmachtsregister ist nicht Bestandteil der Infrastruktur" (ZF11 Z. 115) **[v1.1]**.
- Die Bearbeitungsvertretung innerhalb einer Organisation läuft über RMS und Berechtigungsverzeichnis (ADR-1102 Z. 80–94).

### P5 Clients

- Ein zentraler Postfach-Zugang für alle Gruppen, konfigurierbar (ADR-0005 Z. 88). Laut v1.1: Web-App und mobile Apps (iOS, Android, /e/OS, postmarketOS), modular einbettbar, im Bundesportal und in der Deutschland-App. Er ersetzt die Postfach-Frontends von BundID, MJP und MUK (AK11 Z. 303, Z. 313, Z. 493, Z. 505; ZF11 Z. 133). Grundlage sind Neo Web-App und iOS-App (AK11 Z. 321) **[v1.1]**.
- Fachportale (z. B. ELSTER, Justizportale) als dezentrale Zugänge, die nur die Bereichs-MXID ihres Bereichs sehen (AK11 Z. 606). Drittclients über die offene API, ohne Zertifizierung (ADR-0003 Z. 71–78; AK11 Z. 614, Z. 837).
- Der zentrale Zugang führt die bereichsspezifischen Postfächer einer Person in einer Ansicht zusammen (AK11 Z. 602; ADR-1105 Z. 53–65).
- v1.0 empfiehlt, die Frontends von ZBP, MJP, OZG-PLUS-Postfach und Postfach 2.0 zusammenzuführen (ZF10 Z. 222).

### P6 Anbindung eigener Systeme

- Unternehmensanwendungen sind Postfach-Zugänge über die API oder das SDK, interaktiv und automatisiert (AK11 Z. 595, Z. 610; ADR-0003 Z. 71–78).
- Automatisiert über Client Credentials und FöPD (AK11 Z. 1118) **[v1.1]**. Über FIT-Connect sollen Unternehmen künftig Anträge und maschinenlesbare Folgekommunikation über eine Anbindung abwickeln (AK11 Z. 441) **[v1.1]**.
- Eigenes Backend im Verbund: Betriebsmodell C, vergleichbar mit dem TI-Messenger (ADR-1103 Z. 78–94).
- Kritik in der Konsultation: M2M und Maschinenidentitäten unzureichend, API-first statt SDK-first gefordert (ISS Z. 30–126, Dataport). Die Antworten waren nicht abrufbar.

### P7 Antwort und Erstkontakt

- Bidirektional; die öffentliche Stelle kann Antworten zulassen oder untersagen (ANF Z. 86, Z. 130). Ein "Neue Nachricht"-Button ist gefordert (ANF Z. 127).
- Erstkontakt einer Person mit einer Behörde: Der Zugang ermittelt über die Adressregister-Abfrage die Adresse und den Bereich der Behörde und nutzt dafür die passende Bereichs-MXID (AK11 Z. 676, Z. 948; ADR-1111 Z. 73). Bei einem Funktionspostfach legt der FP-Bot den Vorgangsraum an (AK11 Z. 1137).
- Erstkontakt einer Behörde mit einer Person: nur mit bekannter Bereichs-MXID (siehe B3; ADR-1146 Z. 81). Organisationen sind nach Registrierung ohne Einwilligung erreichbar, Privatpersonen nur nach Einwilligung (ANF Z. 129, Z. 131).
- Offen: Zuständigkeitsfinder (ANF Z. 125; AK10 Z. 361), Suche über Domaingrenzen (AK11 Z. 1078).

### P8 Inhalt an die Behörde

- Wie B4, ohne Unterschied nach Richtung: Text, Anhänge, strukturierte Daten, vordefinierte Antwortoptionen (ANF Z. 85–101). Privatpersonen siegeln oder signieren perspektivisch mit QES über die EUDI-Wallet (ADR-0017 Z. 53, Risiken Z. 77–80).
- Ob Formulardaten ohne vorherige Anfrage vorgesehen sind, legt ZaPuK nicht fest (ADR-0001 Z. 57: keine Fachlogik in der Infrastruktur) **[Folgerung]**.

### P9 Postfach und Nutzerkonto

- Das Postfach ist vom Nutzerkonto getrennt, hat einen eigenen Lebenszyklus und eine eigene Adresse und ist mit jedem zugelassenen Identifizierungsmittel nutzbar (ADR-1110 Z. 74–80, Option B, operativ entschieden 13.10.2026, Datum der Fassung 26.11.2026; AK11 Z. 130, Z. 833). Verworfen: Postfach bleibt Teil des Nutzerkontos (Option A, Z. 36).
- Damit die Bekanntgabefiktion an die Bereitstellung anknüpft, ist § 9 OZG anzupassen (ADR-1110 Z. 90). Die im Entwurf vom 13.11.2026 genannten Risiken „Parallelbetrieb der Nutzerkonto-Postfächer bei verzögerter Governance-Änderung“ und „zweite Adresse ohne Verknüpfung“ stehen in v1.1.1 nicht mehr im ADR.

### Ü1 Modus

- Matrix-Räume: jeder Vorgang in einem kryptografisch isolierten Vorgangsraum, asynchron und in Echtzeit nutzbar (ADR-0011 Z. 439; ADR-1102 Z. 74; AK11 Z. 40). Außerdem Audio und Video über MatrixRTC (AK11 Z. 142, Z. 154) **[v1.1]**.
- Verworfen als Kommunikationsschicht: E-Mail, XMPP, MIMI, OSCI/XTA, FIT-Connect, AS4, XRoad, IRIS Connect, Eigenentwicklung (ADR-0011 Z. 100–437).

### Ü2 Vertraulichkeit

- E2EE bis zum Endgerät, privates Schlüsselmaterial verlässt das Gerät nicht (ADR-1100 Z. 57–65, Option A, operativ entschieden 24.09.2026). Ausgeschlossen: E2EE endet auf einem Server außerhalb des Machtbereichs (Option B, Z. 47, Z. 65).
- Funktionspostfächer (ADR-1101 Z. 114–134, Option B): technischer Nutzer wahlweise mit Schlüsseln auf dem Org-Server (dann ist die E2EE nicht strikt umgesetzt, ADR-1101 Z. 96) oder ohne Schlüssel. Option A ohne Automatisierung für höchsten Schutzbedarf, Option C bis zur Org-Grenze als begründete Ausnahme, Option D (Entschlüsselung außerhalb der Organisation) ausgeschlossen.
- Schlüsselwiederherstellung: Key Backup auf dem Server, verschlüsselt mit einem Wiederherstellungsschlüssel, den nur die Person hat; je Bereichs-MXID ein eigenes Backup. Nachrangig Re-Authentisierung und erneute Schlüsselweitergabe. Key Escrow ausgeschlossen, Betreiber und Behörden haben keinen Zugriff auf die Schlüssel der Nutzenden (ADR-1147 Z. 77–85, Z. 97, Richtung entschieden seit 12.11.2026, Datum der Fassung 26.11.2026; offen: Wiederherstellung ohne Wiederherstellungsschlüssel, ADR-IDX Z. 61). Organisationen müssen ihre Wiederherstellungsschlüssel sicher verwahren (ADR-1145 Z. 137).
- Metadaten bleiben beim Betreiber sichtbar (unverschlüsselte State Events, Bereitstellungsbestätigung, künftig ggf. Benachrichtigungsbestätigung und Abrufzeitpunkt, ADR-1122 Z. 60; ADR-1121 Z. 49). Gegenmaßnahmen: Betreibertrennung, Pseudonyme, Metadatenminimierung (AK11 Z. 351–359, Z. 1024–1028). ADR "Metadatenminimierung im Betrieb" ist unbearbeitet (ADR-IDX Z. 65).
- Konsultationsfrage: Der Betreiber eines Browser-Clients kann potenziell auf Klartext und Zugänge zugreifen (ISS Z. 132–149). Eine Antwort war nicht abrufbar; in ADRs nicht behandelt **[unverifiziert]**.

### Ü3 Echtheit

- Authentizität auf der E2EE-Schicht, gegenseitige Authentifizierung im Kanal (ADR-0013 Z. 108, Option 3). Verworfen: keine Authentizität (Option 1, Z. 50) und Authentizität auf Transportebene zwischen Backends (Option 2, Z. 61).
- Identitätsnachweis je Nachricht als SD-JWT VC mit Holder-Binding und Key-Binding-JWT (Empfänger, Vorgang, MXID, Zeitpunkt), prüfbar gegen die Trust-Liste der Nachweisstellen (ADR-1106 Z. 83–89; ADR-1107 Z. 80–84; ADR-1108 Z. 58; AK11 Z. 891–901). Die Organisationszugehörigkeit einer Absenderadresse wird über den signierten Registereintrag geprüft (ADR-1111 Z. 73; AK11 Z. 1080).
- Dokumentechtheit gegenüber Dritten über ein qualifiziertes Siegel des Absenders vor dem Versand (ADR-0017 Z. 59–65, Option B). Verworfen: "System als Signaturdienst" (Option A, Z. 33–41), weil das System den Klartext bräuchte und Vertrauensdiensteanbieter sein müsste.
- Plausible Deniability im Kanal, Nichtabstreitbarkeit nur über Bestätigungen und Siegel außerhalb des Kanals (AK11 Z. 1032–1044).

### Ü4 Ereignisse und Bestätigungen

- Bereitstellungsbestätigung: Der Quittungsdienst des Empfänger-Backends erzeugt sie ohne Klartext (Hash über das Chiffrat, qualifizierter Zeitstempel, fortgeschrittenes Siegel des Betreibers) und stellt sie allen Raumteilnehmenden zu, auch einer Person, die an eine Behörde schreibt (ADR-0018 Z. 130–146, Z. 156–179, operativ entschieden, Datum der Fassung 13.11.2026; ANF Z. 156). Sie dokumentiert den Eingang (ADR-0018 Z. 9); dass die Empfänger:in entschlüsseln kann und die Nachricht damit zugegangen ist, belegt sie nicht (GLOS Z. 231).
- Löschquittung bei Löschung einer quittierten Nachricht (ADR-0018 Z. 180–188).
- Verworfen oder nachrangig: Standardisierung im Matrix-Protokoll (Option 2, wird parallel verfolgt), zentraler Bestätigungsdienst als Trusted Third Party (Option 3, Z. 88–98), Abrufbestätigung als Ersatz (Option 4), server-internes Transaktionsprotokoll (Option 5, Z. 114–126, nur als Indiz zulässig, Z. 146).
- Abruf- und Lesebestätigung als verschlüsselte Custom Events nur auf Anforderung des Absenders. Sie sind nicht erzwingbar und ersetzen die Bereitstellungsbestätigung nicht. Die Lesebestätigung kann als elektronisches Empfangsbekenntnis nach § 173 Abs. 3 ZPO dienen (ADR-1118 Z. 74–110, Option 2, operativ entschieden 13.10.2026, Datum der Fassung 26.11.2026). Verworfen: Matrix Read Receipts `m.read` (Option 1, Z. 36), MSC4089 (Option 3, Z. 62). Ohne Anpassung von § 9 OZG belegt nur die Abrufbestätigung den maßgeblichen Abruf; ob sie freiwillig bleiben kann, ist zu prüfen (Z. 127).
- Ausgestaltung der Abrufbestätigung offen (ADR-1121, In Arbeit): nur Client (A, gilt vorerst), nur Backend bestätigt den Download eines Anhangs (B) oder beides (C); B und C wären nicht unterdrückbar, machen den Abrufzeitpunkt aber für Betreiber sichtbar (Z. 32–62).
- Benachrichtigungsbestätigung des Quittungsdienstes auf Anforderung der Behörde: dass und wann benachrichtigt wurde, oder dass nicht benachrichtigt wurde; belegt nicht den Zugang der Benachrichtigung (ADR-1122 Z. 60–62, In Arbeit; GLOS Z. 179).
- Übermittlungsoptionen sind absendergesteuert. Der Empfänger-Client setzt sie nur um, wenn der Absender per Identitätsnachweis berechtigt ist (ADR-1119 Z. 45–57, Z. 69). Neu ist die Backend-Option „Benachrichtigungsbestätigung anfordern“ (Z. 53). Ob eine Privatperson von einer Behörde eine Lesebestätigung verlangen kann, ist nicht ausdrücklich geregelt **[unverifiziert]**.

### Ü5 Interoperabilität mit Systemen außerhalb

- Protokollkonverter auf der E2EE-Schicht (ADR-0010 Z. 109, Option 2, operativ entschieden 15.04.2025). Verworfen: Bridge ohne E2EE (Option 1, Z. 33), vollständige Standardisierung (Option 3, Z. 76), keine Interoperabilität (Option 4, Z. 96).
- Konkret: ein OSCI-Matrix-Protokollkonverter für EGVP, bei dem die Postfach-Zugänge zusätzlich OSCI-Kryptografie und das EGVP-Inhaltsformat beherrschen müssen (AK11 Z. 714–716; TR Z. 96). Ein AS4-Gateway für eDelivery ist zulässig, wenn es nur verschlüsselte Daten transportiert (AK11 Z. 958) **[v1.1]**.
- TI-Messenger und BundesMessenger: v1.0 lässt offen, ob sie reine Messenger bleiben oder konsolidiert werden (AK10 Z. 425–430). v1.1 sieht den TI-Messenger als vergleichbares Föderationssystem mit Smartcard-PKI und Zertifizierung (AK11 Z. 964–978) **[v1.1]**.

### Ü6 Malware-Prüfung

- Kombination aus Prüfung am Endgerät (B), im Server der Organisation durch einen technischen Nutzer mit E2EE-Teilnahme (C) und Delegation vom Endgerät an den Scandienst der Organisation (E). Vor dem Versand an Privatpersonen prüft die Organisation immer. Formate lassen sich im Zugang einschränken (ADR-1130 Z. 122–143, operativ entschieden 08.10.2026). Die Prüfpflicht steht in den Anschlussbedingungen (AK11 Z. 1020).
- Ausgeschlossen: zentraler Scanner (D, Z. 124). Nicht gewählt: Prüfung im Content Repository (A, Z. 32), weil dort nur Chiffrat liegt (AK11 Z. 1016).
- Bei Option E weist der Zugang darauf hin, dass der Klartext das Endgerät verlässt (ADR-1130 Z. 159).

### I1 Betrieb und Governance (nur technischer Zuschnitt; Governance siehe `recht-governance/`)

- Getrennte Betreiber für das Privatpersonen-Backend und die gebündelten Organisations-Backends (ADR-1114 Z. 64–72). Verworfen: ein Betreiber bzw. ein Backend für alle (Option A, Z. 30), was ADR-0006 Option 3 (ein zentrales Backend, ADR-0006 Z. 66) anpasst (ADR-IDX Z. 23).
- Das BMDS betreibt das Backend für Privatpersonen und den zentralen Zugang, FITKO/FIT-Connect die Backends für Organisationen, SSP und Adressregister. SAFE bleibt bei der BLK (AK11 Z. 376–387; ZF11 Z. 133–137) **[v1.1]**.
- Zusätzlich getrennt betrieben werden sollen: die zentrale Nachweisstelle für Privatpersonen (weder Betreiber des Backends noch der BundID; jedem zentralen Backend ist eine Instanz „zugeordnet“, nicht mehr „Bestandteil“) und der zentrale Zuordnungsdienst (nicht der Backend-Betreiber) (AK11 Z. 684, Z. 695–697; ADR-1105 Z. 216).

### I2 Datenhaltung und Versand

- Föderiert, Nachrichten nur verschlüsselt im jeweiligen Backend (ADR-0002 Z. 94; AK11 Z. 547). Vier zentrale Backends je Teilnehmendengruppe plus dezentrale Backends von Organisationen (ADR-1114 Z. 70; ADR-1103 Z. 142).
- v1.0 sah ein einziges zentrales Backend für alle vor (ADR-0006 Z. 66; AK10 Z. 321, Z. 413) **[veraltet?]**.
- Keine zentrale Aufbewahrung. Die Übernahme in die E-Akte geschieht im Machtbereich der Datenbesitzer mit einem Exportpaket inklusive Nachweisen (ADR-1135 Z. 92–98, Option C, Datum der Fassung 26.11.2026). Verworfen: Klartext-Export am Server (A), zentraler Aufbewahrungsdienst (B), Backend als Archiv (D, Z. 98). Neues Risiko: Ohne vollständige Übernahme von Nachweisen und Chiffrat ist die Übermittlung später nicht beweissicher (Z. 138).
- Das Postfach hält Nachrichten, bis Nutzende sie löschen oder eine vom Betreiber konfigurierte Löschfrist greift (AK11 Z. 1058). ADR "Löschung und Aufbewahrung im Postfach" ist unbearbeitet (ADR-IDX Z. 64).
- Anhänge werden vom Empfänger-Backend proaktiv aus dem Absender-Backend gezogen; ein zentrales Media-Repository ist verworfen (ADR-1120 Z. 110–114).

### I3 Betriebsmodell für Organisationen

- Die Betriebsmodelle A (Minimalanbindung), B (Teilbetrieb mit eigenem oder beauftragtem RMS) und C (souverän, eigenes Backend und eigene Domain) werden alle unterstützt. Die Organisation wählt selbst und kann wechseln (ADR-1103 Z. 140–154, operativ entschieden 11.11.2026; GLOS Z. 241–245).
- Bei A gibt es keinen serverseitigen Historienzugriff für neue Mitglieder ohne Schlüsselweitergabe, keine serverseitige Archivierung und keine Malware-Prüfung (Z. 162ff.). Auch bei C ist eine Registrierung im SSP Pflicht (Z. 154).

### I4 Funktionspostfächer und interne Bearbeitung

- Vorgangsräume je Fall (ADR-1102 Z. 74). Der RMS läuft organisationseigen, wahlweise mit E2EE-Teilnahme, oder zentral ohne Schlüssel (Z. 82–86).
- Raumtopologie: vermittelnder FP-Bot mit Schlüsseln und internem Vorgangsraum (Option B, nur Betriebsmodell B/C) oder gemeinsamer Raum mit schlüssellosem FP-Bot (Option C, einzige Option im Betriebsmodell A). Verworfen: gemeinsamer Raum, vom Absender erzeugt (Option A) (ADR-1109 Z. 28–38, Z. 76–90, Richtung entschieden seit 12.11.2026; offen: Weitergabe historischer Schlüssel an neue Mitglieder, MSC4268, Z. 112–116).
- Manuelle Raumverwaltung durch Moderatoren für hohe Vertraulichkeit (AK11 Z. 861).
- Unter MLS bleibt die automatisierte Mitgliedschaftsverwaltung ohne Schlüssel womöglich nicht möglich (AK11 Z. 863; ADR-1103 Risiko Option A).

### I5 Aufnahme in den Verbund

- Risikobasiert: bekannte haftbare Betreiber, technisch durchgesetzte Regeln, Föderationsliste mit Aufnahme, Suspendierung und Ausschluss, Anschlussvertrag im SSP (ADR-1145 Z. 81–99, Option C, operativ entschieden 14.10.2026, Datum der Fassung 26.11.2026). Verworfen: vollständige Zertifizierung und Auditierung (Option A, Z. 39), rein vertragliche Absicherung (Option B, Z. 53). Neu: Verwahrung der Wiederherstellungsschlüssel als Bedingung (Z. 137); ADR-0004 wird eingeschränkt, Anschluss nur mit Anschlussvertrag (Z. 150).
- Onboarding: Backends werden manuell freigegeben, Organisationen und Adressen automatisiert (ADR-1148 Z. 67, Option B). Verworfen: vollautomatische Aufnahme (Option A, Z. 39) und umfangreiches Zulassungsverfahren je Backend (Option C, Z. 59).
- PKI: Zertifizierungsstelle des Verbunds für die Server-Server-API. Föderationsliste und Trust-Liste haben jeweils eigene Signaturschlüssel. Für die Client-Server-API gilt die öffentliche PKI (ADR-1144 Z. 115–121). Verworfen: beliebige CAs (1A), V-PKI (1B), Signatur der Listen durch einen Vertrauensdiensteanbieter (2C).
- ADR-1122 kündigt als Folge-Entscheidung an, dass Backends für Privatpersonen Benachrichtigung und Benachrichtigungsbestätigung anbieten müssen (ADR-1122 Z. 77); ADR-1145 enthält das in v1.1.1 noch nicht.

### R1–R4 (nur technische Anknüpfungspunkte; Auswertung in `recht-governance/`)

- R3: Die Bereitstellungsbestätigung ist Grundlage für die Bekanntgabefiktion; für das entkoppelte Postfach ist § 9 OZG anzupassen, Sozial- und Steuerverfahren haben mit § 37 Abs. 2a SGB X und § 122a AO eigene Regeln (ADR-0018 Z. 26, Z. 234; ADR-1118 Z. 104, Z. 127). Die Lesebestätigung kann als eEB nach § 173 Abs. 3 ZPO dienen (ADR-1118 Z. 98). ADR-1121 stellt die drei Bekanntgabe-Regime gegenüber: § 41 Abs. 2a VwVfG (Tag nach dem Abruf, Ende der Bereitstellung nach zehn Tagen), § 9 OZG und § 122a AO (vierter Tag nach Bereitstellung), § 37 Abs. 2a SGB X (vierter Tag nach Absendung der Benachrichtigung, sonst Tag des Abrufs) (ADR-1121 Z. 11–13; ADR-1122 Z. 9–11). Die Übermittlungsoptionen (ADR-1119) sind ein technischer Mechanismus, keine rechtliche Wahl zwischen einfacher und förmlicher Zustellung **[Folgerung]**.
- R1: Qualifiziertes Siegel des Absenders für den Schriftformersatz (ADR-0017 Z. 11).

## Abgleich morph_box.csv

### Abgleich mit v1.1.1 (2026-11-27)

Geprüft wurden alle ZaPuK- und Neo-Verweise in `morph_box.csv` und die Spalte `ZaPuK (Zielarchitektur, Deutschland)` in `existing_solutions.csv`, soweit sie Architektur, ADRs und Glossar betreffen. Vorschläge, keine Änderungen an den CSV-Dateien.

1. **Qualifier „Entwurf v1.1“:** Architekturaussagen aus AK11 stehen in v1.1.1 unverändert (Konkordanz unten), die Version 1.1 ist aber kein unveröffentlichter Entwurf mehr. Vorschlag: „Entwurf v1.1“ durch „v1.1“ ersetzen (B1 A3, A8; B3 A1; P1 A2, A3; P5 A3; P6 A3; Ü5 A2, A3; I1 A4; R4 A3 sowie die Zellen B1, B3, P1, P5, P6, P7, P9, Ü1, Ü5, Ü6, I1, R4 in `existing_solutions.csv`). Die Aussagen aus den Transitionsplänen (B1 A8, B3 A1) sind davon zu trennen, weil die Pläne in v1.1.1 fehlen.
2. **P1:** „eigenes ADR unbearbeitet“ ist überholt, ADR-1122 ist „In Arbeit“ und hat Option A gewählt. P1 A2 und A3 um ADR-1122 ergänzen; P1 A2 passt genau („einheitlicher Text ohne Bereich, Behörde oder Vorgang“).
3. **Ü4:** ADR-1121 (Abrufbestätigung durch Client und/oder Backend, In Arbeit) bei A2 und A4 ergänzen; Benachrichtigungsbestätigung (ADR-1122) passt zu A1. Alle acht Ausprägungen sind belegt, eine neue Option ginge nur über eine Erweiterung der Spalten.
4. **B1 A8:** „Architekturkonzept v1.0, 3.2“ bleibt richtig (jetzt `Vorversionen/v1.0`, Z. 393–409); in v1.1.1 gibt es kein Kapitel 3.2 zu Bestandslösungen mehr.
5. **P9:** Der in `existing_solutions.csv` nicht verwendete Parallelbetrieb der Nutzerkonto-Postfächer steht nicht mehr in ADR-1110; kein Änderungsbedarf an den CSV-Dateien, nur an Belegen, die ADR-1110 (v1.1) Z. 92–99 zitieren.
6. Statusangaben „Richtung entschieden“ bei B3 A3, P2 A1, I4 A3, Ü2 A5 stimmen weiter (ADR-IDX Z. 41, Z. 42, Z. 45, Z. 61).

### Abgleich vom 2026-11-19 (Korrekturen übernommen)

Geprüft wurden alle ZaPuK-Verweise in `morph_box.csv` (Stand 2026-11-19) gegen `main` und den Entwurf v1.1. Die Korrekturen sind laut `../../../Referenzen-ZaPuK.md` in `morph_box.csv` übernommen. Fundstellen auf v1.1.1 umgerechnet.

#### Korrekt

- B3 A3 (ADR-1105, ADR-1111, Glossar; Zuschnitt der Bereiche offen): stimmt.
- B1 A7 und I5 A4 (ADR-0008, ADR-1148, ADR-1144, ADR-1145) stimmen inhaltlich.
- Ü1 A3 (ADR-1102).
- Ü2 A2 (ADR-1100 Option A, ADR-0012), A4 (Key Escrow, mit ADR-1147 unvereinbar = Option C ausgeschlossen), A5 (ADR-1147), A6 (ADR-1101 Option B).
- Ü5 A1 bis A4 (ADR-0010 Optionen 4, 1, 2, 3).
- Ü6 A1 (Option A nicht gewählt), A2 (B), A3 (C), A4 (D ausgeschlossen), A5 (E), A6 und A7 (ADR-1130 Z. 128, Z. 143).
- P9 A2 (ADR-1110), I2 A3 (ADR-0002, ADR-0004), I2 A6 (ADR-1135), I3 A1 bis A3 (ADR-1103), I4 A1 und A2 (ADR-1102 Optionen A und B), I4 A4 (ADR-1101 Option C), I5 A2 (ADR-1145 Option A), I5 A3 (ADR-1145 Option B), I1 A4 (ADR-1114), P6 A5.

#### Abweichungen und Präzisierungen

1. **Status veraltet:** ADR-1105, ADR-1106, ADR-1109 und ADR-1147 sind seit 12.11.2026 nur noch "Richtung entschieden" (ADR-IDX Z. 41–61). Betroffen: B3 A3, P2 A1, I4 A3, Ü2 A5. Die "kritische Annahme" bei B3 A3 wird dadurch gestützt.
2. **B1 A3 (FIT-Connect):** Die Belege ADR-0009, ADR-0011, ADR-0012 tragen die Aussage nicht direkt. ADR-0011 hat FIT-Connect als Kommunikationsschicht verworfen (Option 6). Die Aussage "FIT-Connect als Anbindungsweg, Voraussetzung Matrix und MLS" steht in ZF10 Z. 232. In v1.1 ist FIT-Connect außerdem Entwickler der Backends, Betreiber der Organisations-Backends und Träger von SSP und SDK (AK11 Z. 287–299, Z. 424).
3. **B1 A8 (Channel Adapter, keine Messaging-Bridge):** entspricht v1.0 (AK10 Z. 401–403). v1.1 sieht in der Transition Bridges ausdrücklich vor (ZBP-Matrix-Bridge, ELSTER-Zustell-Bridge, Postfach-2.0-Bridge; AK11 Z. 718; TR Z. 63). Der Channel Adapter kommt in v1.1 nicht mehr vor.
4. **B1 A6 (SDK):** ZaPuK fehlte, obwohl ADR-0009 das SDK festlegt. B1 A7 nannte ADR-1101 als Beleg; passender sind ADR-0003 (Offenheit für Drittanwendungen) und ADR-0009. ADR-1101 betrifft nur E2EE-Endpunkte.
5. **B1 A4 (Landesportale, Servicekonten, kommunale Rechenzentren als Backend-Betreiber):** ZaPuK nennt "behördliche Backends für die eigene Behörde und ggf. Nachnutzende" (ADR-0004 Z. 81) und Betrieb durch IT-Dienstleister in Auftragsverarbeitung (ADR-1103 Z. 48). Landesportale und Servicekonten nennt ZaPuK nicht **[Folgerung]**.
6. **B2:** Eine ZaPuK-Option fehlte: zentraler Postfach-Zugang (Web und App) auch für Behörden, mit Funktionspostfächern im Betriebsmodell A (ADR-0005, ADR-1103).
7. **B4:** ZaPuK fehlte bei A2 (Anhänge), A3 (strukturierte Daten, ANF_FUN_NAC_183, ADR-0001) und A6 (vordefinierte Antwortoptionen, ANF_FUN_NAC_125). Neu wäre eine Option "Abholschein für einen Nachweis in die EUDI-Wallet zusammen mit dem Bescheid" (ADR-1146). A4 passt teilweise: gesiegeltes Dokument der Behörde nach ADR-0017.
8. **P1:** ZaPuK fehlte bei A2 (E-Mail-Hinweis) und A3 (Push), beides ANF_FUN_ZUS_026 und AK11 Z. 653. Das zugehörige ADR war unbearbeitet, seit v1.1.1 ADR-1122 „In Arbeit“.
9. **P2 A8 ("ZaPuK ADR-0014" beim Fachzertifikat):** ADR-0014 nennt ELSTER-Zertifikate nur als Anmeldemittel der BundID (Z. 67). Für Organisationen ist das ELSTER-Organisationszertifikat Identifizierungsmittel im Org-Konto (AK11 Z. 883), Systeme weisen sich per X.509 aus (ADR-0015, ADR-0016). ZaPuK fehlte bei A2 (BundID und Org-Konto mit Vertrauensniveaus, ADR-0014 Option 2, ADR-0015 und ADR-0016) und A3 (EUDI-Wallet, ADR-0014 Option 3, ADR-1146).
10. **P3 A1 ("von ZaPuK ausgeschlossen"):** ADR-1102 nennt Option A "nicht gewählt" (Z. 74), nicht "ausgeschlossen"; die Begründung lautet "widerspricht Zero Trust". Bei P3 A2 (Organisationskonto mit Rollen) fehlte "ZaPuK ADR-1112 Option A erwogen, nicht gewählt". P3 A4 sollte ADR-1108 (Org-IdP-Adapter) und ADR-1112 nennen.
11. **P4:** Bei A3 (Postvollmacht in der d-box) fehlte "ZaPuK ADR-1113 Option B erwogen, nicht gewählt". Bei A4 (zentrales Vollmachtsregister): ZaPuK sieht keins vor (ADR-1113 Z. 57; ZF11 Z. 115). A5 sollte ADR-1113 Option A nennen.
12. **P5:** ZaPuK fehlte bei A5 (Drittclients über offene Schnittstelle, ADR-0003) und bei A1 und A3 (laut v1.1 Bundesportal und Deutschland-App, AK11 Z. 493). Fachportale mit Bereichssicht (AK11 Z. 606) sind eine eigene Variante.
13. **P6 A4 ("technischer Nutzer, Anbindung über SDK (ZaPuK Teilbetrieb)"):** Eine Anbindung von Unternehmensanwendungen über API oder SDK ist in allen Betriebsmodellen möglich (AK11 Z. 610). An den Teilbetrieb gebunden ist nur der RMS mit E2EE-Teilnahme. Bei P6 A3 (offene API) fehlte ZaPuK (ADR-0003, Matrix-Client-Server-API, Client Credentials).
14. **P7:** Bei A3 (Antwort an die sendende Behörde) fehlte ZaPuK (bidirektional, Behörde kann Antworten untersagen; ANF_FUN_NAC_185, ANF_FUN_ADR_058).
15. **Ü2 A1:** Hier kann "ZaPuK ADR-1100 Option B ausgeschlossen" ergänzt werden. Bei Ü2 A6 ist "kritische Annahme" gedeckt: ADR-1101 Z. 96 nennt die E2EE in diesen Räumen "nicht strikt umgesetzt".
16. **Ü3:** ZaPuK fehlte ganz. Vorschläge: A1 "ZaPuK ADR-0013 Optionen 1 und 2 erwogen, nicht gewählt"; A2 "Siegel des Betreibers": nach ZaPuK nur auf der Bereitstellungsbestätigung (ADR-0018), ein Signaturdienst des Systems ist verworfen (ADR-0017 Option A); A3 "Siegel der Behörde vor dem Versand (ZaPuK ADR-0017 Option B)"; neue Option "Identitätsnachweis je Nachricht im E2EE-Kanal (SD-JWT VC), prüfbar über Trust-Liste (ZaPuK ADR-0013 Option 3, ADR-1106, ADR-1107, ADR-1108)".
17. **Ü4:** ZaPuK fehlte ganz. Vorschläge: A1 und A5 (ADR-0018, Bereitstellungsbestätigung an alle Teilnehmenden), A2 und A3 (ADR-1118 auf Anforderung), A4 (ADR-0018 Optionen 3 und 5 erwogen, nur als Indiz), neue Variante "Löschquittung" (ADR-0018 Z. 180).
18. **I2:** Bei A1 (ein Backend für alle) fehlte "ZaPuK ADR-0006, v1.0, durch ADR-1114 angepasst". A2 (getrennte Backends je Nutzergruppe) gilt auch für ZaPuK (ADR-1114: vier Backends). ZaPuK ist damit eine Kombination aus A2 und A3 **[Folgerung]**.
19. **I4 A3:** ADR-1109 (FP-Bot, Optionen B und C) fehlte in der Liste. Es fehlten ZaPuK-Varianten: "manuell verwaltete Gruppenräume ohne technischen Nutzer (ADR-1101 Option A, höchster Schutzbedarf)" und "Entschlüsselung außerhalb der Organisation (ADR-1101 Option D, ausgeschlossen)".
20. **I5 A4:** ADR-1148 (manuelle Freigabe der Backends) und ADR-1148 Optionen A und C fehlten.
21. **R3 A4 (ADR-1119 als "Behörde wählt je Nachricht einfach oder nachweislich"):** ADR-1119 steuert, welche Nachweise erzeugt werden (Bereitstellung, Abruf, Lesen, Empfangsbekenntnis, seit v1.1.1 auch Benachrichtigung). Über die rechtliche Zustellart entscheidet es nicht **[Folgerung]**; Prüfung im Ordner `recht-governance/`.
22. **I1 A4:** Die Formulierung "Verbund staatlicher Betreiber über Standard" war enger als ZaPuK. Auch nicht-staatliche Betreiber dürfen Backends für Organisationen betreiben (ADR-0004 Z. 77–81; ADR-1144 Z. 115).

#### Fehlende ZaPuK-Dimensionen (keine passende Zeile in morph_box.csv)

- Nachweis der fachlichen Identität je Nachricht und Trust-Liste (ADR-1106 bis 1108), nur teilweise in Ü3.
- Metadatenschutz und Betreibertrennung (ADR-1114, AK11 Z. 1024–1028), nur indirekt in I1 und I2.
- Schlüsselwiederherstellung (ADR-1147) steht nur als Option in Ü2.
- Beweissichere Anhänge, Pull-Prinzip (ADR-1120).

## Zelltexte für die Spalte "ZaPuK" in existing_solutions.csv

Die Spalte ist seit 2026-11-23 befüllt, Belege in `../../../Referenzen-ZaPuK.md`. Der frühere Vorschlag (Stand `main` 2026-11-12) entfällt; Änderungsvorschläge aus v1.1.1 siehe [Abgleich mit v1.1.1](#abgleich-mit-v111-2026-11-27).

## Widersprüche und Unstimmigkeiten in den Quellen

- **Bridge-Verbot vs. Transitions-Bridges:** AK10 Z. 401 verbietet Messaging-Bridges, v1.1 sieht sie übergangsweise vor (AK11 Z. 718; TR Z. 63). AK11 Z. 956 sagt weiterhin "Bridges sollen dagegen nicht verwendet werden" (bezogen auf ADR-0010 für Systeme außerhalb).
- **Eigene IdP:** AK10 Z. 345 "nicht unterstützt" vs. ADR-1108 Org-IdP-Adapter.
- **Ein vs. vier Backends:** ADR-0006, AK10 Z. 321 und Z. 413 vs. ADR-1114, ADR-1103 Z. 142.
- **Statusangaben:** `zapuk-main/` führte ADR-1105, ADR-1106, ADR-1109, ADR-1147 als "Operativ entschieden", `main` und v1.1.1 als "Richtung entschieden". `sources.md` Abschnitt 1 stützt sich auf den alten Stand (u. a. "No messaging bridges", Liste der offenen ADRs ohne "nicht entschlüsselbare Nachrichten").
- **Folge-Entscheidung ohne Gegenstück:** ADR-1122 Z. 77 verweist für die Pflicht zur Benachrichtigung auf die Anschlussbedingungen in ADR-1145; ADR-1145 enthält sie in v1.1.1 nicht.
- **„In Arbeit“ mit gewählter Option:** ADR-1122 nennt eine gewählte Option (Z. 56), ADR-1121 eine Übergangsregel (Z. 62), obwohl beide „In Arbeit“ sind; Status laut Legende: Optionen werden noch erörtert (ADR-IDX Z. 81).
- **Bereitstellungsbestätigung und Zugang:** ADR-0018 Z. 9 sagt weiterhin „Der Zugang … findet objektiv mit dem Eingang im Postfach-Backend … statt“, das Glossar sagt, der Zugang setze zusätzlich die Entschlüsselbarkeit voraus (GLOS Z. 231).
- Behoben in v1.1.1: Tippfehler "Seit 12.12.2026" im ADR-Index (jetzt Z. 6 „12.11.2026“); fehlendes Kapitel 3 im Architekturkonzept v1.1 (jetzt Z. 156–249).

## Lücken

- Ob der IT-Planungsrat die Version 1.1 inhaltlich beschlossen hat, geht aus den Architekturdokumenten nicht hervor: Sie liegt B-2026/43-IT als Anlage bei, das Logbuch nennt nur den Abschluss der Phase II (`../zapuk-v1.1.1/README.md` Z. 77, Z. 92) **[Folgerung]**; Prüfung im Ordner `recht-governance/`.
- Antworten des ZaPuK-Teams auf die Konsultations-Issues sind ohne openCode-Anmeldung nicht lesbar. Eine veröffentlichte Auswertung der Konsultation Phase 2 wurde nicht gefunden.
- Unbearbeitete ADRs: API-Autorisierung (1149), Löschung und Aufbewahrung, Metadatenminimierung, nicht entschlüsselbare Nachrichten (ADR-IDX Z. 63–66). In Arbeit: ADR-1121 Abrufbestätigung, ADR-1122 Benachrichtigungsdienst (Z. 54–55).
- Nicht gefunden: Massenversand, Druck- oder Papier-Rückfall, SMS, Erinnerung bei Nichtabruf (auch in v1.1.1 und ADR-1122 nicht), Mengengerüst (im Transitionsplan ZBP als offener Klärungspunkt, TR-A Z. 148), konkrete SDK-Sprachen, Größenlimits für Anhänge (Anforderung "beliebig groß", ANF Z. 84).
- Für BundesMessenger und TI-Messenger gibt es keine Transitionspläne; sie werden nur im Vergleich behandelt (AK11 Z. 964–978).
- Die Transitionspläne A bis C und die TADRs zu Mein ELSTER liegen in v1.1.1 nicht vor, nur `Zielarchitektur/Transitionen/README.md`; ihr Stand nach der 51. Sitzung ist offen.
- Die Abbildungen der Transitionspläne (Phasen und Zeitachsen) liegen nur als Bild vor und wurden nicht ausgewertet.

## Konkordanz der Fundstellen (alt → v1.1.1)

Umrechnung aller Fundstellen aus `../../../Referenzen-ZaPuK.md` (Ref) und `../../ZaPuK-Zusammenfassung.md` (Zus), die auf Dateien im Umfang dieses Ordners zeigen. „Z. alt“ wie dort zitiert: bei Kopien in diesem Ordner Zeile der Kopie (ADR-IDX, GLOS, AK11, ADR-xxxx auf `main` geändert, ADR-xxxx (v1.1-Entwurf)), sonst Zeile in `zapuk-main/` (AK10, ZF10, ANF, ADR-0017, 1100, 1101, 1104, 1110, 1112–1114, 1118, 1130, 1145, 1146). „Fundstelle v1.1.1“ mit den neuen Kürzeln oben (Originalzeilen in `../zapuk-v1.1.1/`). „gleich“ = Zeile inhaltlich unverändert.

| Kürzel alt | Z. alt | zitiert in | Fundstelle v1.1.1 | Inhalt |
|---|---|---|---|---|
| ADR-0001 | 62 | Ref B4; Ref P8 | ADR-0001 Z. 57 | gleich |
| ADR-0002 | 99 | Ref I2 | ADR-0002 Z. 94 | gleich |
| ADR-0003 | 76–83 | Ref B1; Ref B2; Ref P5 | ADR-0003 Z. 71–78 | gleich |
| ADR-0004 | 74–86 | Ref I1 | ADR-0004 Z. 69–81 | gleich |
| ADR-0005 | 37–41 | Ref B2 | ADR-0005 Z. 32–36 | gleich |
| ADR-0005 | 93 | Ref B2; Ref P5 | ADR-0005 Z. 88 | gleich |
| ADR-0006 | 71 | Ref I2 | ADR-0006 Z. 66 | gleich |
| ADR-0008 | 66 | Ref B1 | ADR-0008 Z. 61 | gleich |
| ADR-0009 | 64 | Ref B1 | ADR-0009 Z. 59 | gleich |
| ADR-0010 | 38–114 | Ref Ü5 | ADR-0010 Z. 33–109 | gleich |
| ADR-0011 | 105–444 | Ref Ü1 | ADR-0011 Z. 100–439 | gleich |
| ADR-0011 | 326ff. | Ref B1 | ADR-0011 Z. 321ff. | gleich |
| ADR-0012 | 207–214 | Ref Ü2 | ADR-0012 Z. 202–209 | gleich |
| ADR-0013 | 55–113 | Ref Ü3 | ADR-0013 Z. 50–108 | gleich |
| ADR-0014 | 72 | Ref P2 | ADR-0014 Z. 67 | gleich |
| ADR-0014 | 97–102 | Ref P2 | ADR-0014 Z. 92–97 | gleich |
| ADR-0015 | 107–112 | Ref P2 | ADR-0015 Z. 102–107 | gleich |
| ADR-0016 | 90–93 | Ref P2 | ADR-0016 Z. 85–88 | gleich |
| ADR-0017 | 11 | Ref R1 | ADR-0017 Z. 11 | gleich |
| ADR-0017 | 33–65 | Ref Ü3 | ADR-0017 Z. 33–65 | gleich |
| ADR-0017 | 53 | Ref P8 | ADR-0017 Z. 53 | gleich |
| ADR-0017 | 57–75 | Ref B4 | ADR-0017 Z. 57–75 | gleich |
| ADR-0017 | 57–65 | Ref R1 | ADR-0017 Z. 57–65 | gleich |
| ADR-0018 | 135–193 | Ref Ü4 | ADR-0018 Z. 130–188 | gleich (Z. 154 nur „aufrechterhalten“) |
| ADR-0018 | 234 | Ref R3 | ADR-0018 Z. 234 | **Zitierfehler:** Kopie-Z. 234 ist leer, gemeint ist Kopie-Z. 239 = Originalzeile 234. Neu ADR-0018 Z. 234: Aussage (§ 9 OZG anzupassen) gleich, ergänzt um § 37 Abs. 2a SGB X und § 122a AO als eigene Regelungen |
| ADR-1100 | 57–65 | Ref Ü2 | ADR-1100 Z. 57–65 | gleich |
| ADR-1101 (v1.1-Entwurf) | 101 | Ref Ü2 | ADR-1101 Z. 96 | gleich („E2EE … nicht strikt umgesetzt“), jetzt in der veröffentlichten Fassung |
| ADR-1101 | 114–134 | Ref Ü2; Ref I4 | ADR-1101 Z. 114–134 | gleich |
| ADR-1102 | 43–63 | Ref P3 | ADR-1102 Z. 38–58 | gleich |
| ADR-1102 | 43–91 | Ref I4 | ADR-1102 Z. 38–86 | gleich |
| ADR-1102 | 79–91 | Ref P3 | ADR-1102 Z. 74–86 | gleich |
| ADR-1102 | 79 | Ref Ü1 | ADR-1102 Z. 74 | gleich |
| ADR-1102 | 85–99 | Ref P4 | ADR-1102 Z. 80–94 | gleich |
| ADR-1103 | 55–67 | Ref B2 | ADR-1103 Z. 50–62 | gleich |
| ADR-1103 | 83–99 | Ref P6 | ADR-1103 Z. 78–94 | gleich |
| ADR-1103 | 145–159 | Ref I3 | ADR-1103 Z. 140–154 | gleich |
| ADR-1104 | 52–79 | Ref B3 | ADR-1104 Z. 52–79 | gleich |
| ADR-1105 | 209–231 | Ref B3 | ADR-1105 Z. 204–226 | gleich |
| ADR-1105 | 225 | Ref P7 | ADR-1105 Z. 220 | gleich |
| ADR-1105 | 250 | Ref B3; Ref R2 | ADR-1105 Z. 245 | gleich |
| ADR-1105 | 294–306 | Ref B3 | ADR-1105 Z. 289–301 | gleich |
| ADR-1106 | 88–94 | Ref P2; Ref Ü3 | ADR-1106 Z. 83–89 | gleich |
| ADR-1107 | 85–89 | Ref Ü3 | ADR-1107 Z. 80–84 | gleich |
| ADR-1108 | 63 | Ref P2; Ref Ü3 | ADR-1108 Z. 58 | gleich |
| ADR-1109 | 33–43 | Ref I4 | ADR-1109 Z. 28–38 | gleich |
| ADR-1109 | 81–95 | Ref I4 | ADR-1109 Z. 76–90 | gleich |
| ADR-1110 | 74–84 | Ref P9 | ADR-1110 Z. 74–84 | gleich |
| ADR-1110 | 84 | Ref R3 | ADR-1110 Z. 84 | gleich |
| ADR-1111 | 78 | Ref B3; Ref P7; Ref Ü3 | ADR-1111 Z. 73 | gleich |
| ADR-1112 | 26 | Ref P3 | ADR-1112 Z. 26 | gleich |
| ADR-1112 | 36 | Ref P3 | ADR-1112 Z. 36 | gleich |
| ADR-1112 | 56–58 | Ref P3 | ADR-1112 Z. 56–58 | gleich |
| ADR-1113 | 39–59 | Ref P4 | ADR-1113 Z. 39–59 | gleich |
| ADR-1114 | 64–72 | Ref I1 | ADR-1114 Z. 64–72 | gleich |
| ADR-1114 | 70 | Ref I2 | ADR-1114 Z. 70 | gleich |
| ADR-1118 | 74–110 | Ref Ü4 | ADR-1118 Z. 74–110 | Aussage gleich; Z. 102 „Nachweis der Bereitstellung“ statt „des Zugangs“, Z. 106 Bekanntgabefiktion „bisher nur im Postfach eines Nutzerkontos“; neu außerhalb: Risiko Z. 127 (ohne § 9-OZG-Anpassung zählt der tatsächliche Abruf), Folge ADR-1121 Z. 135 |
| ADR-1118 | 96–110 | Ref R3 | ADR-1118 Z. 96–110 | wie oben; eEB nach § 173 Abs. 3 ZPO (Z. 98) gleich |
| ADR-1119 | 50–61 | Ref Ü4; Ref Evid. Abweichungen R3 | ADR-1119 Z. 45–57 | ergänzt: neue Zeile Z. 53 „Benachrichtigungsbestätigung anfordern“ (Backend, Folge-Entscheidung ADR-1122), Folge Z. 91 |
| ADR-1119 | 61 | Ref P2 | ADR-1119 Z. 57 | gleich (Zeile um 1 verschoben) |
| ADR-1120 | 115–119 | Ref B4 | ADR-1120 Z. 110–114 | gleich |
| ADR-1130 | 32 | Ref Ü6 | ADR-1130 Z. 32 | gleich |
| ADR-1130 | 122–143 | Ref Ü6 | ADR-1130 Z. 122–143 | gleich |
| ADR-1135 | 97–103 | Ref I2 | ADR-1135 Z. 92–98 | gleich |
| ADR-1144 | 120–126 | Ref I5 | ADR-1144 Z. 115–121 | gleich |
| ADR-1145 | 39–99 | Ref I5 | ADR-1145 Z. 39–99 | gleich |
| ADR-1146 | 66–76 | Ref B4 | ADR-1146 Z. 66–76 | gleich |
| ADR-1147 | 80–88 | Ref Ü2 | ADR-1147 Z. 77–85 | gleich; Z. 79 umformuliert (Verwahrung des Wiederherstellungsschlüssels als Ausnahmefall manueller Interaktion); Status weiter „Richtung entschieden“, Datum 26.11.2026 |
| ADR-1148 | 44–72 | Ref I5 | ADR-1148 Z. 39–67 | gleich |
| ADR-1148 | 72 | Ref B1 | ADR-1148 Z. 67 | gleich |
| ADR-IDX | 46 | Ref B3 | ADR-IDX Z. 41 | gleich (ADR-1105 Richtung entschieden) |
| ADR-IDX | 46–64 | Ref Abw. Status | ADR-IDX Z. 41–61 | gleich (1105 Z. 41, 1106 Z. 42, 1109 Z. 45, 1147 Z. 61); dazwischen neu ADR-1121, ADR-1122 „In Arbeit“ (Z. 54–55) |
| ADR-IDX | 67 | Ref P1; Ref Evid. P1 | ADR-IDX (Z. 55 ADR-1122) | **entfallen:** Zeile „ADR-xxxx Benachrichtigungsdienst … Unbearbeitet“ ersetzt durch ADR-1122 „🛠️ In Arbeit“ (Z. 55) |
| ADR-IDX | 68 | Ref I2 | ADR-IDX Z. 64 | gleich (Löschung und Aufbewahrung weiter unbearbeitet) |
| AK10 | 321 | Zus Z. 152 | AK10 Z. 321 | gleich |
| AK10 | 345 | Ref Abw. Widersprüche in den ; Zus Z. 153 | AK10 Z. 345 | gleich („Eigene Identity Provider … nicht unterstützt“), nur Linkpfade |
| AK10 | 401–403 | Zus Z. 151 | AK10 Z. 401–403 | gleich (Bridge-Verbot, Channel Adapter), nur Linkpfade |
| AK10 | 413 | Zus Z. 152 | AK10 Z. 413 | gleich |
| AK11 | 148 | Ref Ü1 | AK11 Z. 142 | gleich (Satz umgestellt) |
| AK11 | 160 | Ref Ü1 | AK11 Z. 154 | gleich |
| AK11 | 205–217 | Ref B1; Zus Z. 55 | AK11 Z. 287–299 | gleich |
| AK11 | 221 | Ref P5; Zus Z. 54 | AK11 Z. 303 | gleich |
| AK11 | 269–277 | Ref Ü2 | AK11 Z. 351–359 | gleich |
| AK11 | 294–366 | Ref I1 | AK11 Z. 376–448 | gleich; Z. 386 Tabellennummer, Z. 412 „Postfach-Backend“ statt „Matrix-Backend“, Z. 445 Justiz-Backend ersetzt MJP-Backend „in Kombination mit dem Postfach-Backend für Privatpersonen und dem Protokollkonverter“ |
| AK11 | 294–305 | Zus Z. 64 | AK11 Z. 376–387 | gleich (Z. 386 nur Tabellennummer) |
| AK11 | 302 | Ref B3; Zus Z. 81 | AK11 Z. 384 | gleich |
| AK11 | 411 | Ref P5; Zus Z. 54 | AK11 Z. 493 | gleich |
| AK11 | 423 | Ref P5 | AK11 Z. 505 | gleich |
| AK11 | 513 | Ref P6 | AK11 Z. 595 | gleich |
| AK11 | 524 | Ref P5 | AK11 Z. 606 | gleich; Beispiel „Verwaltungsportal Bund“ gestrichen |
| AK11 | 528 | Ref P6; Ref Abw. P6 Ausprägung 4 | AK11 Z. 610 | gleich |
| AK11 | 553–555 | Ref B1 | AK11 Z. 635–637 | gleich |
| AK11 | 571 | Ref P1 | AK11 Z. 653 | gleich, Beleg ADR-1122 ergänzt |
| AK11 | 594 | Ref P7 | AK11 Z. 676 | gleich |
| AK11 | 632–634 | Ref Ü5 | AK11 Z. 714–716 | gleich (redaktionell) |
| AK11 | 777 | Ref I4 | AK11 Z. 861 | gleich |
| AK11 | 864 | Ref P7 | AK11 Z. 948 | gleich |
| AK11 | 874 | Ref Ü5 | AK11 Z. 958 | gleich |
| AK11 | 932–936 | Ref Ü6 | AK11 Z. 1016–1020 | gleich („Content Repository“ statt „Medien-Repository“) |
| AK11 | 1034 | Ref P6 | AK11 Z. 1118 | gleich |
| ANF | 84–101 | Ref B4; Zus Z. 86 | ANF Z. 84–101 | gleich |
| ANF | 85–101 | Ref P8 | ANF Z. 85–101 | gleich |
| ANF | 86 | Ref P7 | ANF Z. 86 | gleich |
| ANF | 125–131 | Ref P7 | ANF Z. 125–131 | gleich |
| ANF | 153 | Ref P1 | ANF Z. 153 | gleich |
| GLOS | 184–194 | Ref B3 | GLOS Z. 201–211 | gleich; Bereichs-MXID Z. 211 „verknüpften weiteren Authentisierungssystem“ |
| GLOS | 212–216 | Ref I3 | GLOS Z. 241–245 | gleich |
| ZF10 | 198 | Zus Z. 72 | ZF10 Z. 198 | gleich |
| ZF10 | 232 | Ref Abw. B1 Ausprägung 3 (FIT; Zus Z. 157 | ZF10 Z. 232 | gleich |

Weitere Bezüge:

| Bezug alt | zitiert in | Fundstelle v1.1.1 | Inhalt |
|---|---|---|---|
| `architektur-technik/_befunde.md` Z. 312 (Lücken, „Nicht gefunden …“) | Ref Evid. P1 | `_befunde.md` Abschnitt Lücken, Z. 336 | gleich, ergänzt „auch in v1.1.1 und ADR-1122 nicht“ |
| AK10 Kap. 3.2 (Channel Adapter, Bridge-Verbot) | `morph_box.csv` B1 A8; Zus Z. 151 | AK10 Z. 393–409 | gleich |
| AK10 Kap. 3.3 (ZBP-Bidirektionalität entbehrlich) | Zus Z. 158 | AK10 Z. 411–424 | gleich |
| ADR-IDX Z. 11 „Seit 12.12.2026“ (Tippfehler) | Zus Z. 154 | ADR-IDX Z. 6 | korrigiert auf „12.11.2026“ |
| ADR-Status „Richtung entschieden“ 1105, 1106, 1109, 1147 | Zus Z. 23; Ref Abw. Status | ADR-IDX Z. 41, Z. 42, Z. 45, Z. 61 | gleich |
| „Unbearbeitete ADRs: …, Benachrichtigungsdienst, …“ | Zus Z. 167 | ADR-IDX Z. 55, Z. 63–66 | **geändert:** Benachrichtigungsdienst ist ADR-1122 „In Arbeit“; neu ADR-1121 „In Arbeit“ |
| „ADR unbearbeitet“ (Benachrichtigung) | Ref P1; Zus Z. 109 | ADR-1122 (ADR-IDX Z. 55) | **geändert:** In Arbeit, Option A gewählt |
| ADR-1110 (v1.1) Z. 92–99 (Risiken § 9 OZG, Parallelbetrieb) | `_befunde.md` P9 (alt) | ADR-1110 Z. 88–90 | **teilweise entfallen:** nur § 9 OZG bleibt; Parallelbetrieb und zweite Adresse gestrichen |
| ADR-1118 (v1.1) Z. 138–141 (Offener Punkt Nachweis der Benachrichtigung) | `_befunde.md` P1 (alt) | ADR-1118 Z. 135 → ADR-1122 Z. 9, Z. 13, Z. 60–62, Z. 82 | **entfallen** in ADR-1118, verlagert nach ADR-1122 |
| AK11 Z. 164 (Kap. 3 verweist auf v1.0) | `_befunde.md` Widersprüche (alt) | AK11 Z. 156–249 | **entfallen:** Kap. 3 ausgeführt |

Umrechnungsregeln für andere Fundstellen der Vorfassung dieser Datei (Git-Historie) und der Kopien:

| Quelle alt | Regel Z. alt → Z. v1.1.1 |
|---|---|
| ADR-xxxx (main), Kopie `*_main_2026-11-12.md` | Z. minus 5; Ausnahmen: ADR-0018 Kopie-Z. 241–245 → Z. 236 (Offene Punkte jetzt Z. 258–260), Kopie-Z. 246–265 minus 9, ab Kopie-Z. 266 minus 5; ADR-1119 ab Kopie-Z. 58 minus 4, ab Kopie-Z. 94 minus 2; ADR-1135 ab Kopie-Z. 140 minus 1; ADR-1147 ab Kopie-Z. 28 minus 3, ab Kopie-Z. 99 plus 1 |
| ADR-xxxx (v1.1), Kopie `*_v1.1-entwurf_2026-11-13.md` | Z. minus 5; Ausnahmen: ADR-1110 Kopie-Z. 96–99 entfallen, ab Kopie-Z. 100 minus 9; ADR-1118 Kopie-Z. 132–138 minus 4, Kopie-Z. 139–141 → Z. 135, ab Kopie-Z. 142 minus 6; ADR-1130 Kopie-Z. 182 entfällt (Leerzeile), ab Kopie-Z. 183 minus 6; ADR-1145 ab Kopie-Z. 154 minus 3; ADR-1147 ab Kopie-Z. 101 minus 1 |
| ADR-xxxx (lokal), `zapuk-main/` | gleich; Ausnahmen: ADR-1110 ab Z. 87 plus 4; ADR-1118 Z. 127–132 plus 1, ab Z. 133 plus 3; ADR-1130 Z. 159–173 plus 2, Z. 174 entfällt (Leerzeile), ab Z. 175 plus 1; ADR-1145 Z. 137–147 plus 1, ab Z. 148 plus 3 |
| ADR-IDX, Kopie main | bis Kopie-Z. 58 minus 5; Kopie-Z. 59–66 minus 3; Kopie-Z. 67 entfällt (jetzt ADR-1122, Z. 55); ab Kopie-Z. 68 minus 4 |
| AK11, Kopie Entwurf | bis Kopie-Z. 12 minus 5, Kopie-Z. 13–164 minus 6 (Kopie-Z. 164 Hinweis ersetzt durch Z. 158–160); Kopie-Z. 165–167 minus 4; Kopie-Z. 168–720 plus 82; ab Kopie-Z. 721 plus 84 |
| AK10, ZF10, ANF, RB, Konsultationen, Lesefassung (`zapuk-main/`) | gleich |
| GLOS, Kopie main | Einzelumrechnung nötig (neue Einträge ab Originalzeile 63); Beispiele: Bereich Kopie-Z. 184 → Z. 201, Bereichs-MXID 190 → 207, Betriebsmodell 212 → 241, Funktionspostfach 394 → 423, Machtbereich 470 → 499, Protokollkonverter 658 → 689 |
