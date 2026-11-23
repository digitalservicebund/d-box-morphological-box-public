# Befunde: ZaPuK – Architektur und Technik

Stand der Auswertung: 2026-11-19. Dateien siehe `_index.md`. Upstream-Stand: `main` Commit `cc150e2977` (2026-11-12), Entwurfsbranch `architecture-v1.1` Commit `ef99118c06` (2026-11-13). Fundstellen: "Z." = Zeile der genannten Datei. Bei Kopien in diesem Ordner zählen die Zeilen der Kopie (Originalzeile = Z. minus 5). Kennzeichnung: **[Folgerung]** eigene Ableitung, **[unverifiziert]** ohne ausreichenden Beleg, **[veraltet?]** Quelle älter oder durch neuere Fassung überholt, **[Entwurf]** nur im nicht gemergten Branch `architecture-v1.1`.

Status der ADRs nach dem Index auf `main` (ADR-IDX Z. 22–70): "Operativ entschieden" = auf Arbeitsebene des FIT-AM gefasst, offene Punkte ändern die Entscheidung nicht; "Richtung entschieden" = Richtung steht, offene Punkte können Festlegungen noch ändern; "Unbearbeitet" = Bedarf erkannt, kein ADR (Legende Z. 85). ADR-0000 bis ADR-0016 sind zusätzlich Teil des IT-PLR-Beschlusses 2025/28 (Z. 22–38).

## Kürzel und Pfade

Alle Pfade relativ zu `sources/`.

- ADR-xxxx (main) = `ZaPuK/quellen/architektur-technik/fitko_zapuk-adr-xxxx-…_main_2026-11-12.md` (Text auf `main` geändert): 0000–0016, 0018, 1102, 1103, 1105, 1106, 1107, 1108, 1109, 1111, 1119, 1120, 1135, 1144, 1147, 1148
- ADR-xxxx (lokal) = `zapuk-main/Zielarchitektur/Architekturentscheidungen/ADR-xxxx-….md` (Text auf `main` unverändert): 0017, 1100, 1101, 1104, 1110, 1112, 1113, 1114, 1118, 1130, 1145, 1146
- ADR-xxxx (v1.1) = `ZaPuK/quellen/architektur-technik/fitko_zapuk-adr-xxxx-…_v1.1-entwurf_2026-11-13.md` **[Entwurf]**
- ADR-IDX = `ZaPuK/quellen/architektur-technik/fitko_zapuk-adr-index_main_2026-11-12.md`
- GLOS = `ZaPuK/quellen/architektur-technik/fitko_zapuk-glossar_main_2026-11-12.md`
- AK10 = `zapuk-main/Zielarchitektur/Architekturkonzept/README.md` (Architekturkonzept v1.0, Sommer 2025)
- AK11 = `ZaPuK/quellen/architektur-technik/fitko_zapuk-architekturkonzept-v1.1_v1.1-entwurf_2026-11-13.md` **[Entwurf]**
- ZF10 = `zapuk-main/Zielarchitektur/Zusammenfassung/README.md`; ZF11 = `ZaPuK/quellen/architektur-technik/fitko_zapuk-zusammenfassung-v1.1_v1.1-entwurf_2026-11-13.md` **[Entwurf]**
- ANF = `zapuk-main/Konsolidierte-Anforderungen/README.md` (v0.7.1, 24.02.2026)
- TR = `ZaPuK/quellen/architektur-technik/fitko_zapuk-transitionen-ueberblick_v1.1-entwurf_2026-11-13.md`; TR-A, TR-B, TR-C = Transitionspläne ZBP, MJP, MUK (`fitko_zapuk-transition-a-…`, `-b-…`, `-c-…`); TADR-E2 = `fitko_zapuk-tadr-mein-elster-002-…` **[Entwurf]**
- ISS = `ZaPuK/quellen/architektur-technik/fitko_zapuk-opencode-issues-architektur_2026-11-19.md`

## Kurzbild der Architektur

1. **Topologie:** Hybride, föderierte Client-Server-Architektur (ADR-0002 (main) Z. 99). Vier zentrale Postfach-Backends je Teilnehmendengruppe (Privatpersonen, private Organisationen, öffentliche Stellen, Organisationen der Justiz). Das Backend für Privatpersonen hat einen anderen Betreiber als die Backends für Organisationen (ADR-1114 (lokal) Z. 64–72, operativ entschieden 09.10.2026). Organisationen dürfen eigene Backends betreiben (ADR-0004 (main) Z. 74), Privatpersonen nicht (ADR-1114 (lokal) Z. 72; AK11 Z. 741, Z. 1123).
2. **Protokoll:** Matrix als Kommunikationsschicht (ADR-0011 (main) Z. 444), MLS als Ziel-E2EE, übergangsweise Olm/Megolm (ADR-0012 (main) Z. 207–214). E2EE endet im Endgerät (ADR-1100 (lokal) Z. 57–65). Bei Funktionspostfächern ist zusätzlich ein technischer Nutzer mit Schlüsseln im Machtbereich der Organisation zulässig (ADR-1101 (lokal) Z. 114–134).
3. **Zugänge:** Ein zentraler Postfach-Zugang für alle Gruppen (ADR-0005 (main) Z. 93). Laut Entwurf v1.1 im Bundesportal und in der Deutschland-App, als Web-App und mobile App (AK11 Z. 221, Z. 411). Daneben beliebige Drittzugänge ohne Zulassung (ADR-0003 (main) Z. 76–83): Fachverfahren, Unternehmensanwendungen, Fachportale wie ELSTER (AK11 Z. 507–532).
4. **Anbindung:** Zentrales SSP (ADR-0008 (main) Z. 66; ADR-1148 (main) Z. 72) und SDK (ADR-0009 (main) Z. 64), das SDK ist optional, die Matrix-API reicht (AK11 Z. 553). Laut Entwurf v1.1 entwickelt FIT-Connect die Backends, betreibt die Backends für Organisationen und bringt SSP und SDK ein (AK11 Z. 205–217, Z. 294–305, Z. 342).
5. **Identität und Adressen:** Postfach getrennt vom Nutzerkonto (ADR-1110 (lokal) Z. 74–80). Privatpersonen: bereichsspezifische pseudonyme MXID, ein Zuordnungsdienst verknüpft BundID, eID und EUDI-Wallet (ADR-1105 (main) Z. 209–231, Richtung entschieden). Organisationen: föderiertes Adressregister (ADR-1111 (main) Z. 78). Fachliche Identität über einen Identitätsnachweis je Nachricht im E2EE-Kanal (ADR-1106 (main) Z. 88–94; ADR-1107 (main) Z. 85).
6. **Nachweise:** Bereitstellungsbestätigung durch das Empfänger-Backend (ADR-0018 (main) Z. 135–151). Abruf- und Lesebestätigungen clientseitig, optional (ADR-1118 (lokal) Z. 74–110). Vom Absender gesteuerte Übermittlungsoptionen (ADR-1119 (main) Z. 50–61).
7. **Bestandssysteme:** v1.0 verbietet Messaging-Bridges und erlaubt für Frontends Channel Adapter (AK10 Z. 401–403). Der Entwurf v1.1 sieht in der Transition Bridges vor: ZBP-Matrix-Bridge, Zustell-Bridge in der ELSTER-Domäne, Postfach-2.0-Matrix-Bridge (AK11 Z. 636; TR Z. 63, Z. 85–92, Z. 113, Z. 117). EGVP bleibt bestehen und wird über einen OSCI-Matrix-Protokollkonverter angebunden (TR Z. 96–101).

## Befunde je Dimension

### B1 Anbindung von Fachverfahren

Festlegung:
- Fachverfahren sind Postfach-Zugänge (Matrix-Clients) über die offene API oder das SDK. Eine Zulassung gibt es nicht (ADR-0003 (main) Z. 76–83, Option 4 "Offenheit für Drittanwendungen", operativ entschieden 15.04.2025; AK11 Z. 528, Z. 753).
- SDK zentral bereitgestellt (ADR-0009 (main) Z. 64, operativ entschieden 14.04.2025). Es ist nicht verpflichtend, die API ist ohne SDK vollständig nutzbar (AK11 Z. 553). Automatisierte Einlieferung (M2M) mit OAuth-2.0-Client-Credentials über die föderale API-Autorisierungsinfrastruktur (FöPD), Systemidentität über X.509 (V-PKI, ELSTER-Organisationszertifikat) (AK11 Z. 555, Z. 1034–1035) **[Entwurf]**. ADR-1149 "API-Autorisierung der Server-zu-Server-Zugriffe" ist unbearbeitet (ADR-IDX Z. 66).
- Registrierung der Organisation im zentralen SSP, automatisiert. Nur Backends werden manuell freigegeben (ADR-0008 (main) Z. 66; ADR-1148 (main) Z. 72, operativ entschieden 12.10.2026).
- Anbindung immer nur an ein Backend des Verbunds (ADR-1114 (lokal) Z. 72; AK11 Z. 292).
- FIT-Connect: v1.0 empfiehlt FIT-Connect als zentralen Anbindungsweg, Voraussetzung Matrix und MLS (ZF10 Z. 232). Der Entwurf v1.1 konkretisiert das: FIT-Connect entwickelt die Backends, betreibt die Backends für Organisationen, das SDK erhält ein ZaPuK-Modul. Antragstellung und Folgekommunikation laufen über dieselbe Anbindung (AK11 Z. 138, Z. 205–217, Z. 352–353, Z. 359; ZF11 Z. 99, Z. 253–257; TR Z. 40; TR-A Z. 127) **[Entwurf]**.
- Als begründete Ausnahme dürfen Fachverfahren ohne Matrix-Integration die E2EE an der Organisationsgrenze enden lassen (ADR-1101 (lokal) Z. 130, Option C).

Erwogen und verworfen:
- FIT-Connect als Kommunikationsschicht (ADR-0011 (main) Z. 326ff., Option 6: "nationale Eigenentwicklung", hohe Aufwände für Messaging- und Postfachfunktionen), ebenso OSCI/XTA (Option 5, Z. 271ff.), AS4, XRoad.
- Zulassung oder Zertifizierung von Postfach-Zugängen (ADR-0003 (main) Optionen 1–3, Z. 33–63).
- Mehrere fachlich zugeschnittene SSPs (ADR-0008 (main) Option 2, Z. 54).

Bestandssysteme:
- v1.0: Bestandslösungen müssen auf Matrix und MLS migrieren. Messaging-Bridges sind verboten, Channel Adapter nur für die Integration von Frontends, außerhalb der E2EE (AK10 Z. 393–409). **[veraltet?]**: Der Entwurf v1.1 enthält das Kapitel 3 nicht mehr. Er unterscheidet Protokollkonverter (E2EE bleibt) von Bridges, die in der Transition übergangsweise vorgesehen sind und Nachrichten annehmen, die in der Bestandslösung ohnehin nicht E2EE-verschlüsselt vorliegen (AK11 Z. 636; TR Z. 63, Z. 71).
- ZBP-API bleibt übergangsweise als "Legacy-Fassade" über eine ZBP-Matrix-Bridge, ohne Neu-Registrierungen (AK11 Z. 329; TR Z. 85–92; TR-A Z. 97, Z. 115, Z. 137) **[Entwurf]**.
- Mein ELSTER: Zustell-Bridge in der ELSTER-Domäne, später Anbindung über das SDK, dauerhafter Parallelbetrieb (TADR-E2 Z. 44–48, Z. 70, operativ entschieden 25.09.2026; TR Z. 117) **[Entwurf]**.
- EGVP bleibt und wird über einen OSCI-Matrix-Protokollkonverter am Justiz-Backend angebunden (AK11 Z. 343, Z. 632–634; TR Z. 96–101).

Offen: Sprachen und Plattformen der SDKs (ADR-0009 (main) Z. 68); ADR-1149; FIT-Connect-Produktschnitt "vorbehaltlich" (AK11 Z. 185, Z. 305).

### B2 Zugang ohne Fachverfahren

- Der zentrale Postfach-Zugang (Web und App) steht allen Teilnehmendengruppen zur Verfügung, auch öffentlichen Stellen (ADR-0005 (main) Z. 37–41, Z. 93; AK11 Z. 751). Funktionspostfächer einer Behörde ohne eigene IT: Betriebsmodell A mit zentralem RMS (ADR-1103 (main) Z. 55–67).
- Ein Versanddienst oder Upload für Massenversand und ein Papier-Rückfall sind in ZaPuK nicht vorgesehen **[unverifiziert]** (in ADRs, AK10, AK11 und ANF nicht gefunden).
- Drittclients als Zugang sind ohne Zulassung möglich (ADR-0003 (main) Z. 76–83).

### B3 Adressierung der Person / Organisation

Festlegung Privatpersonen (ADR-1105 (main), Richtung entschieden seit 12.11.2026, zuvor operativ entschieden 06.11.2026):
- Teil 1: bereichsspezifische MXID (Option 1B) (Z. 209). Bereich = eine oder mehrere Behörden mit derselben Bereichs-MXID (GLOS Z. 184–194).
- Teil 2: Ein Zuordnungsdienst je Bereich (2E) oder zentral (2E’) ordnet die Kennungen von BundID, eID und EUDI-Wallet derselben zufälligen Bereichs-MXID zu (Z. 215–221). Zielbild 2C: Pseudonym aus einem personengebundenen Schlüssel, ggf. aus dem Melderegister, wie das österreichische bPK aus der Stammzahl (Z. 122, Z. 223).
- Die Behörde erhält die Bereichs-MXID mit dem Antrag. Ein Erstkontakt ohne vorheriges Verfahren ist nicht möglich, ein Adressregister für Privatpersonen gibt es nicht (Z. 225, Z. 250; AK11 Z. 396, Z. 1018, Z. 1028). Die Aufnahme einer Postfachadresse ins Melderegister wird geprüft (AK11 Z. 400, Z. 1030).
- Verworfen: globale MXID 1A (Z. 211), vorgangsspezifische MXID 1C "zunächst nicht" (Z. 213), Pseudonym nur über BundID 2A (Z. 227), BundID-Pseudonyme in der EUDI-Wallet 2B (Z. 229), verschiedene Postfächer je Authentisierungssystem 2D, ausgeschlossen (Z. 231).
- Offen: Zuschnitt der Bereiche und damit Wahl 2E oder 2E’, Betreiber der Zuordnungsdienste, Wiederherstellung nach Ausweiswechsel, Betreibertrennung von Zuordnungsdienst und Nachweisstelle (Z. 294–306; ADR-IDX Z. 46).

Festlegung Organisationen:
- MXID ohne Personenbezug: `@[orgpräfix]_[funktion]:post.gov.de` zentral, `@[funktion]:[org-domain]` souverän, Privatpersonen `@[bereichspseudonym]:privat.post.gov.de` (ADR-1104 (lokal) Z. 52–54, Z. 62–79, operativ entschieden 11.11.2026). Eigene Domain per `.well-known` auch ohne eigenen Homeserver (Z. 65).
- Föderiertes Adressregister: Jedes Backend führt die Einträge seiner Domain, signiert im Matrix-Profil. Abfrage nach Organisationsmerkmalen und Prüfung einer bekannten Adresse (ADR-1111 (main) Z. 78, Option D, operativ entschieden 12.11.2026). Verworfen: Adresse als Identitätsmerkmal (A), separater Adressnachweis (B), Verwendung ohne Prüfung (C) (Z. 28–63). Offen: Weiterleitung oder Replikation bei der Suche über Domains, MSC4258 (AK11 Z. 862, Z. 994).
- Justiz: SAFE wird als Adressregister nachgenutzt (AK11 Z. 302, Z. 395). Adressen im Justiz-Backend nach dem Muster `@[SAFE-ID]:domain` (TR-B Z. 37) **[Entwurf]**.
- Berufsträger dürfen mit ihrer MXID in Kammerregistern geführt werden (AK11 Z. 1002) **[Entwurf]**.

### B4 Inhalt an die Person / Organisation

- Nachrichtentext mit Formatierung, Tabellen, Links, Medien (ANF Z. 90–94); Anhänge beliebigen Formats, per Allowlist einschränkbar (ANF Z. 87–88). Anhänge liegen verschlüsselt im Content Repository und werden auf Anforderung proaktiv ins Empfänger-Backend gezogen (ADR-1120 (main) Z. 115–119; AK11 Z. 570, Z. 1045).
- Strukturierte Daten und fachliche Metadaten (Aktenzeichen, Antrags-ID) sind Anforderungen (ANF Z. 85, Z. 95–99). ADR-0001 erlaubt wenige querschnittliche Erweiterungen, etwa Terminabfragen und Abstimmungen, aber keine Fachlogik (ADR-0001 (main) Z. 62). Vordefinierte Antwortoptionen (ANF Z. 101, ANF_FUN_NAC_125). Statusinformationen zum Verfahren (ANF Z. 100). Die Statusmonitor-Daten des ZBP werden ins Matrix-Backend migriert (TR-A Z. 124) **[Entwurf]**.
- Gesiegelte oder signierte Dokumente: Die Behörde siegelt vor dem Versand, z. B. über den Siegeldienst des Bundes; das System transportiert nur und prüft keine Siegel (ADR-0017 (lokal) Z. 57–65, Z. 67–75, operativ entschieden 24.09.2026).
- Nachweise für die EUDI-Wallet kommen nicht als Anhang, sondern als Abholschein (OID4VCI Credential Offer) zusammen mit dem Bescheid (ADR-1146 (lokal) Z. 66–76, operativ entschieden 14.10.2026; AK11 Z. 899). Verworfen: Anhang-Lösung, indirekte Ausstellung über BundID/ZBP (Z. 74).

### P1 Benachrichtigung

- Anforderung: Benachrichtigung per E-Mail oder App-Push (ANF Z. 153, ANF_FUN_ZUS_026).
- Entwurf v1.1: Der Benachrichtigungsdienst ist Teil jedes Postfach-Backends (SMTP, APNs/FCM) und relevant für Bekanntgabefristen (AK11 Z. 571). Im Anwendungsfall bekommt die Person einen Push auf das Mobilgerät (AK11 Z. 477) **[Entwurf]**.
- Offen: ADR "Benachrichtigungsdienst für Bereitstellungen im Postfach" ist unbearbeitet (ADR-IDX Z. 67). Nachweis der Benachrichtigung, an die § 37 Abs. 2a SGB X die Bekanntgabe knüpft (ADR-1118 (v1.1) Offene Punkte, Z. 138–141) **[Entwurf]**. SMS, Erinnerung bei Nichtabruf und Brief als Rückfall sind nicht behandelt **[unverifiziert]**.

### P2 Authentifizierung

- Privatpersonen: eID, BundID und perspektivisch EUDI-Wallet (ADR-0014 (main) Z. 97–102, operativ entschieden 28.04.2025). Die BundID bringt eigene Mittel mit (Passwort, ELSTER-Zertifikat, eID, eIDAS) auf den Niveaus normal bis hoch (Z. 72).
- eID direkt bei der Behörde, ohne BundID und Nachweisstelle (Kanalbindung, Option A); BundID und Org-Konto übergangsweise über einen kurzlebigen Identitätsnachweis der Nachweisstelle (Option C). Verworfen: Nachweis exklusiv je Nachricht (Option B) (ADR-1106 (main) Z. 88–94, Richtung entschieden seit 12.11.2026; offen: gebundener Schlüssel, Gültigkeit der eID-Bindung, ADR-IDX Z. 47).
- Öffentliche Stellen: X.509 aus der V-PKI und Organisationskonto; verworfen: öffentliche PKI; EUDI-Wallet perspektivisch (ADR-0015 (main) Z. 107–112). Private Organisationen: X.509 aus einer PKI (ELSTER-PKI ertüchtigen) und Organisationskonto (ADR-0016 (main) Z. 90–93; AK11 Z. 799, Z. 831).
- Eigenes IdM einer Organisation über einen Org-IdP-Adapter als Nachweisstelle (ADR-1108 (main) Z. 63, operativ entschieden 12.11.2026). Damit überholt: "Eigene Identity Provider … werden nicht unterstützt" (AK10 Z. 345) **[veraltet?]**.
- Vertrauensniveau im Nachweis als Claim. Der Absender kann ein Mindestniveau für das Lesen verlangen (AK11 Z. 827; ADR-1119 (main) Z. 61).

### P3 Organisationszugang

- Funktionspostfach als fachliches Objekt mit eigener MXID. Jeder Vorgang bekommt einen eigenen Raum, ein technischer Nutzer (RMS) lädt Berechtigte ein (ADR-1102 (main) Z. 79–91, Option C, operativ entschieden 12.11.2026).
- Berechtigungsverzeichnis zentral im Backend oder organisationsintern mit eigenem IdM (ADR-1112 (lokal) Z. 56–58, Option C, operativ entschieden 09.10.2026). Verworfen: Pflege im Org-Konto (A, Z. 26) und im SSP (B, Z. 36).
- Verworfen: geteilte Zugangsdaten und Schlüssel (ADR-1102 (main) Option A, Z. 43–53, "widerspricht Zero Trust"), statische Gruppenräume (Option B, Z. 55–63).
- Mitarbeitende treten nach außen nicht persönlich auf, intern bleibt über das Berechtigungsverzeichnis nachvollziehbar, wer gehandelt hat (AK11 Z. 856).

### P4 Vertretung

- Die handelnde Person nutzt ihre eigene Identität und ihr eigenes Postfach. Vollmacht oder gesetzliche Vertretung wird als Ergänzung des Identitätsnachweises oder als signiertes Dokument übermittelt, die Prüfung liegt beim Empfänger (ADR-1113 (lokal) Z. 53–59, Option A, operativ entschieden 24.09.2026).
- Verworfen: Zugriff auf das Postfach der vertretenen Person (Option B, Z. 39–51: keine strikte E2EE, Datenschutzprobleme).
- Vollmachts- und Vormundschaftsregister entwirft ZaPuK nicht (Z. 57). "Ein zentrales Vollmachtsregister ist nicht Bestandteil der Infrastruktur" (ZF11 Z. 115) **[Entwurf]**.
- Die Bearbeitungsvertretung innerhalb einer Organisation läuft über RMS und Berechtigungsverzeichnis (ADR-1102 (main) Z. 85–99).

### P5 Clients

- Ein zentraler Postfach-Zugang für alle Gruppen, konfigurierbar (ADR-0005 (main) Z. 93). Laut Entwurf: Web-App und mobile Apps (iOS, Android, /e/OS, postmarketOS), modular einbettbar, im Bundesportal und in der Deutschland-App. Er ersetzt die Postfach-Frontends von BundID, MJP und MUK (AK11 Z. 221, Z. 231, Z. 411, Z. 423; ZF11 Z. 133). Grundlage sind Neo Web-App und iOS-App (AK11 Z. 239) **[Entwurf]**.
- Fachportale (z. B. ELSTER, Justizportale) als dezentrale Zugänge, die nur die Bereichs-MXID ihres Bereichs sehen (AK11 Z. 524). Drittclients über die offene API, ohne Zertifizierung (ADR-0003 (main) Z. 76–83; AK11 Z. 532, Z. 753).
- Der zentrale Zugang führt die bereichsspezifischen Postfächer einer Person in einer Ansicht zusammen (AK11 Z. 520; ADR-1105 (main) Z. 58–70).
- v1.0 empfiehlt, die Frontends von ZBP, MJP, OZG-PLUS-Postfach und Postfach 2.0 zusammenzuführen (ZF10 Z. 222).

### P6 Anbindung eigener Systeme

- Unternehmensanwendungen sind Postfach-Zugänge über die API oder das SDK, interaktiv und automatisiert (AK11 Z. 513, Z. 528; ADR-0003 (main) Z. 76–83).
- Automatisiert über Client Credentials und FöPD (AK11 Z. 1034) **[Entwurf]**. Über FIT-Connect sollen Unternehmen künftig Anträge und maschinenlesbare Folgekommunikation über eine Anbindung abwickeln (AK11 Z. 359) **[Entwurf]**.
- Eigenes Backend im Verbund: Betriebsmodell C, vergleichbar mit dem TI-Messenger (ADR-1103 (main) Z. 83–99).
- Kritik in der Konsultation: M2M und Maschinenidentitäten unzureichend, API-first statt SDK-first gefordert (ISS Z. 30–126, Dataport). Die Antworten waren nicht abrufbar.

### P7 Antwort und Erstkontakt

- Bidirektional; die öffentliche Stelle kann Antworten zulassen oder untersagen (ANF Z. 86, Z. 130). Ein "Neue Nachricht"-Button ist gefordert (ANF Z. 127).
- Erstkontakt einer Person mit einer Behörde: Der Zugang ermittelt über die Adressregister-Abfrage die Adresse und den Bereich der Behörde und nutzt dafür die passende Bereichs-MXID (AK11 Z. 594, Z. 864; ADR-1111 (main) Z. 78). Bei einem Funktionspostfach legt der FP-Bot den Vorgangsraum an (AK11 Z. 1053).
- Erstkontakt einer Behörde mit einer Person: nur mit bekannter Bereichs-MXID (siehe B3). Organisationen sind nach Registrierung ohne Einwilligung erreichbar, Privatpersonen nur nach Einwilligung (ANF Z. 129, Z. 131).
- Offen: Zuständigkeitsfinder (ANF Z. 125; AK10 Z. 361), Suche über Domaingrenzen (AK11 Z. 994).

### P8 Inhalt an die Behörde

- Wie B4, ohne Unterschied nach Richtung: Text, Anhänge, strukturierte Daten, vordefinierte Antwortoptionen (ANF Z. 85–101). Privatpersonen siegeln oder signieren perspektivisch mit QES über die EUDI-Wallet (ADR-0017 (lokal) Z. 53, Risiken Z. 77–80).
- Ob Formulardaten ohne vorherige Anfrage vorgesehen sind, legt ZaPuK nicht fest (ADR-0001 (main) Z. 62: keine Fachlogik in der Infrastruktur) **[Folgerung]**.

### P9 Postfach und Nutzerkonto

- Das Postfach ist vom Nutzerkonto getrennt, hat einen eigenen Lebenszyklus und eine eigene Adresse und ist mit jedem zugelassenen Identifizierungsmittel nutzbar (ADR-1110 (lokal) Z. 74–80, Option B, operativ entschieden 13.10.2026; AK11 Z. 136, Z. 749). Verworfen: Postfach bleibt Teil des Nutzerkontos (Option A, Z. 36).
- Entwurf: Damit die Bekanntgabefiktion an die Bereitstellung anknüpft, ist § 9 OZG anzupassen. Bis zur Änderung der Governance laufen Nutzerkonto-Postfächer parallel weiter (ADR-1110 (v1.1) Z. 92–99) **[Entwurf]**.

### Ü1 Modus

- Matrix-Räume: jeder Vorgang in einem kryptografisch isolierten Vorgangsraum, asynchron und in Echtzeit nutzbar (ADR-0011 (main) Z. 444; ADR-1102 (main) Z. 79; AK11 Z. 46). Außerdem Audio und Video über MatrixRTC (AK11 Z. 148, Z. 160) **[Entwurf]**.
- Verworfen als Kommunikationsschicht: E-Mail, XMPP, MIMI, OSCI/XTA, FIT-Connect, AS4, XRoad, IRIS Connect, Eigenentwicklung (ADR-0011 (main) Z. 105–442).

### Ü2 Vertraulichkeit

- E2EE bis zum Endgerät, privates Schlüsselmaterial verlässt das Gerät nicht (ADR-1100 (lokal) Z. 57–65, Option A, operativ entschieden 24.09.2026). Ausgeschlossen: E2EE endet auf einem Server außerhalb des Machtbereichs (Option B, Z. 47, Z. 65).
- Funktionspostfächer (ADR-1101 (lokal) Z. 114–134, Option B): technischer Nutzer wahlweise mit Schlüsseln auf dem Org-Server (dann ist die E2EE nicht strikt umgesetzt, ADR-1101 (v1.1) Z. 101) oder ohne Schlüssel. Option A ohne Automatisierung für höchsten Schutzbedarf, Option C bis zur Org-Grenze als begründete Ausnahme, Option D (Entschlüsselung außerhalb der Organisation) ausgeschlossen.
- Schlüsselwiederherstellung: Key Backup auf dem Server, verschlüsselt mit einem Wiederherstellungsschlüssel, den nur die Person hat; je Bereichs-MXID ein eigenes Backup. Nachrangig Re-Authentisierung und erneute Schlüsselweitergabe. Key Escrow ausgeschlossen (ADR-1147 (main) Z. 80–88, Richtung entschieden seit 12.11.2026; offen: Wiederherstellung ohne Wiederherstellungsschlüssel, ADR-IDX Z. 64).
- Metadaten bleiben beim Betreiber sichtbar (unverschlüsselte State Events, Bereitstellungsbestätigung). Gegenmaßnahmen: Betreibertrennung, Pseudonyme, Metadatenminimierung (AK11 Z. 269–277, Z. 940–944). ADR "Metadatenminimierung im Betrieb" ist unbearbeitet (ADR-IDX Z. 69).
- Konsultationsfrage: Der Betreiber eines Browser-Clients kann potenziell auf Klartext und Zugänge zugreifen (ISS Z. 132–149). Eine Antwort war nicht abrufbar; in ADRs nicht behandelt **[unverifiziert]**.

### Ü3 Echtheit

- Authentizität auf der E2EE-Schicht, gegenseitige Authentifizierung im Kanal (ADR-0013 (main) Z. 113, Option 3). Verworfen: keine Authentizität (Option 1, Z. 55) und Authentizität auf Transportebene zwischen Backends (Option 2, Z. 66).
- Identitätsnachweis je Nachricht als SD-JWT VC mit Holder-Binding und Key-Binding-JWT (Empfänger, Vorgang, MXID, Zeitpunkt), prüfbar gegen die Trust-Liste der Nachweisstellen (ADR-1106 (main) Z. 88–94; ADR-1107 (main) Z. 85–89; ADR-1108 (main) Z. 63; AK11 Z. 807–817). Die Organisationszugehörigkeit einer Absenderadresse wird über den signierten Registereintrag geprüft (ADR-1111 (main) Z. 78; AK11 Z. 996).
- Dokumentechtheit gegenüber Dritten über ein qualifiziertes Siegel des Absenders vor dem Versand (ADR-0017 (lokal) Z. 59–65, Option B). Verworfen: "System als Signaturdienst" (Option A, Z. 33–41), weil das System den Klartext bräuchte und Vertrauensdiensteanbieter sein müsste.
- Plausible Deniability im Kanal, Nichtabstreitbarkeit nur über Bestätigungen und Siegel außerhalb des Kanals (AK11 Z. 948–960).

### Ü4 Ereignisse und Bestätigungen

- Bereitstellungsbestätigung: Der Quittungsdienst des Empfänger-Backends erzeugt sie ohne Klartext (Hash über das Chiffrat, qualifizierter Zeitstempel, fortgeschrittenes Siegel des Betreibers) und stellt sie allen Raumteilnehmenden zu, auch einer Person, die an eine Behörde schreibt (ADR-0018 (main) Z. 135–151, Z. 161–184, operativ entschieden 12.11.2026; ANF Z. 156).
- Löschquittung bei Löschung einer quittierten Nachricht (ADR-0018 (main) Z. 185–193).
- Verworfen oder nachrangig: Standardisierung im Matrix-Protokoll (Option 2, wird parallel verfolgt), zentraler Bestätigungsdienst als Trusted Third Party (Option 3, Z. 93–103), Abrufbestätigung als Ersatz (Option 4), server-internes Transaktionsprotokoll (Option 5, Z. 119–131, nur als Indiz zulässig, Z. 151).
- Abruf- und Lesebestätigung als verschlüsselte Custom Events nur auf Anforderung des Absenders. Sie sind nicht erzwingbar und ersetzen die Bereitstellungsbestätigung nicht. Die Lesebestätigung kann als elektronisches Empfangsbekenntnis nach § 173 Abs. 3 ZPO dienen (ADR-1118 (lokal) Z. 74–110, Option 2, operativ entschieden 13.10.2026). Verworfen: Matrix Read Receipts `m.read` (Option 1, Z. 36), MSC4089 (Option 3, Z. 62).
- Übermittlungsoptionen sind absendergesteuert. Der Empfänger-Client setzt sie nur um, wenn der Absender per Identitätsnachweis berechtigt ist (ADR-1119 (main) Z. 50–61, Z. 73). Ob eine Privatperson von einer Behörde eine Lesebestätigung verlangen kann, ist nicht ausdrücklich geregelt **[unverifiziert]**.

### Ü5 Interoperabilität mit Systemen außerhalb

- Protokollkonverter auf der E2EE-Schicht (ADR-0010 (main) Z. 114, Option 2, operativ entschieden 15.04.2025). Verworfen: Bridge ohne E2EE (Option 1, Z. 38), vollständige Standardisierung (Option 3, Z. 81), keine Interoperabilität (Option 4, Z. 101).
- Konkret: ein OSCI-Matrix-Protokollkonverter für EGVP, bei dem die Postfach-Zugänge zusätzlich OSCI-Kryptografie und das EGVP-Inhaltsformat beherrschen müssen (AK11 Z. 632–634; TR Z. 96). Ein AS4-Gateway für eDelivery ist zulässig, wenn es nur verschlüsselte Daten transportiert (AK11 Z. 874) **[Entwurf]**.
- TI-Messenger und BundesMessenger: v1.0 lässt offen, ob sie reine Messenger bleiben oder konsolidiert werden (AK10 Z. 425–430). Der Entwurf v1.1 sieht den TI-Messenger als vergleichbares Föderationssystem mit Smartcard-PKI und Zertifizierung (AK11 Z. 880–894) **[Entwurf]**.

### Ü6 Malware-Prüfung

- Kombination aus Prüfung am Endgerät (B), im Server der Organisation durch einen technischen Nutzer mit E2EE-Teilnahme (C) und Delegation vom Endgerät an den Scandienst der Organisation (E). Vor dem Versand an Privatpersonen prüft die Organisation immer. Formate lassen sich im Zugang einschränken (ADR-1130 (lokal) Z. 122–143, operativ entschieden 08.10.2026). Die Prüfpflicht steht in den Anschlussbedingungen (AK11 Z. 936).
- Ausgeschlossen: zentraler Scanner (D, Z. 124). Nicht gewählt: Prüfung im Content Repository (A, Z. 32), weil dort nur Chiffrat liegt (AK11 Z. 932).
- Entwurf: Bei Option E weist der Zugang darauf hin, dass der Klartext das Endgerät verlässt (ADR-1130 (v1.1) Z. 163) **[Entwurf]**.

### I1 Betrieb und Governance (nur technischer Zuschnitt; Governance siehe `recht-governance/`)

- Getrennte Betreiber für das Privatpersonen-Backend und die gebündelten Organisations-Backends (ADR-1114 (lokal) Z. 64–72). Verworfen: ein Betreiber bzw. ein Backend für alle (Option A, Z. 30), was ADR-0006 Option 3 (ein zentrales Backend, ADR-0006 (main) Z. 71) anpasst (ADR-IDX Z. 28).
- Entwurf: Das BMDS betreibt das Backend für Privatpersonen und den zentralen Zugang, FITKO/FIT-Connect die Backends für Organisationen, SSP und Adressregister. SAFE bleibt bei der BLK (AK11 Z. 294–305; ZF11 Z. 133–137) **[Entwurf]**.
- Zusätzlich getrennt betrieben werden sollen: die zentrale Nachweisstelle für Privatpersonen (weder Betreiber des Backends noch der BundID) und der zentrale Zuordnungsdienst (nicht der Backend-Betreiber) (AK11 Z. 602, Z. 613–615; ADR-1105 (main) Z. 221).

### I2 Datenhaltung und Versand

- Föderiert, Nachrichten nur verschlüsselt im jeweiligen Backend (ADR-0002 (main) Z. 99; AK11 Z. 465). Vier zentrale Backends je Teilnehmendengruppe plus dezentrale Backends von Organisationen (ADR-1114 (lokal) Z. 70; ADR-1103 (main) Z. 147).
- v1.0 sah ein einziges zentrales Backend für alle vor (ADR-0006 (main) Z. 71; AK10 Z. 321, Z. 413) **[veraltet?]**.
- Keine zentrale Aufbewahrung. Die Übernahme in die E-Akte geschieht im Machtbereich der Datenbesitzer mit einem Exportpaket inklusive Nachweisen (ADR-1135 (main) Z. 97–103, Option C). Verworfen: Klartext-Export am Server (A), zentraler Aufbewahrungsdienst (B), Backend als Archiv (D, Z. 103).
- Das Postfach hält Nachrichten, bis Nutzende sie löschen oder eine vom Betreiber konfigurierte Löschfrist greift (AK11 Z. 974). ADR "Löschung und Aufbewahrung im Postfach" ist unbearbeitet (ADR-IDX Z. 68).
- Anhänge werden vom Empfänger-Backend proaktiv aus dem Absender-Backend gezogen; ein zentrales Media-Repository ist verworfen (ADR-1120 (main) Z. 115–119).

### I3 Betriebsmodell für Organisationen

- Die Betriebsmodelle A (Minimalanbindung), B (Teilbetrieb mit eigenem oder beauftragtem RMS) und C (souverän, eigenes Backend und eigene Domain) werden alle unterstützt. Die Organisation wählt selbst und kann wechseln (ADR-1103 (main) Z. 145–159, operativ entschieden 11.11.2026; GLOS Z. 212–216).
- Bei A gibt es keinen serverseitigen Historienzugriff für neue Mitglieder ohne Schlüsselweitergabe, keine serverseitige Archivierung und keine Malware-Prüfung (Z. 167ff.). Auch bei C ist eine Registrierung im SSP Pflicht (Z. 159).

### I4 Funktionspostfächer und interne Bearbeitung

- Vorgangsräume je Fall (ADR-1102 (main) Z. 79). Der RMS läuft organisationseigen, wahlweise mit E2EE-Teilnahme, oder zentral ohne Schlüssel (Z. 87–91).
- Raumtopologie: vermittelnder FP-Bot mit Schlüsseln und internem Vorgangsraum (Option B, nur Betriebsmodell B/C) oder gemeinsamer Raum mit schlüssellosem FP-Bot (Option C, einzige Option im Betriebsmodell A). Verworfen: gemeinsamer Raum, vom Absender erzeugt (Option A) (ADR-1109 (main) Z. 33–43, Z. 81–95, Richtung entschieden seit 12.11.2026; offen: Weitergabe historischer Schlüssel an neue Mitglieder, MSC4268, Z. 117–121).
- Manuelle Raumverwaltung durch Moderatoren für hohe Vertraulichkeit (AK11 Z. 777).
- Unter MLS bleibt die automatisierte Mitgliedschaftsverwaltung ohne Schlüssel womöglich nicht möglich (AK11 Z. 779; ADR-1103 (main) Risiko Option A).

### I5 Aufnahme in den Verbund

- Risikobasiert: bekannte haftbare Betreiber, technisch durchgesetzte Regeln, Föderationsliste mit Aufnahme, Suspendierung und Ausschluss, Anschlussvertrag im SSP (ADR-1145 (lokal) Z. 81–99, Option C, operativ entschieden 14.10.2026). Verworfen: vollständige Zertifizierung und Auditierung (Option A, Z. 39), rein vertragliche Absicherung (Option B, Z. 53).
- Onboarding: Backends werden manuell freigegeben, Organisationen und Adressen automatisiert (ADR-1148 (main) Z. 72, Option B). Verworfen: vollautomatische Aufnahme (Option A, Z. 44) und umfangreiches Zulassungsverfahren je Backend (Option C, Z. 64).
- PKI: Zertifizierungsstelle des Verbunds für die Server-Server-API. Föderationsliste und Trust-Liste haben jeweils eigene Signaturschlüssel. Für die Client-Server-API gilt die öffentliche PKI (ADR-1144 (main) Z. 120–126). Verworfen: beliebige CAs (1A), V-PKI (1B), Signatur der Listen durch einen Vertrauensdiensteanbieter (2C).

### R1–R4 (nur technische Anknüpfungspunkte; Auswertung in `recht-governance/`)

- R3: Die Bereitstellungsbestätigung ist Grundlage für die Bekanntgabefiktion; für das entkoppelte Postfach ist § 9 OZG anzupassen (ADR-0018 (main) Z. 31, Z. 239; ADR-1118 (lokal) Z. 104). Die Lesebestätigung kann als eEB nach § 173 Abs. 3 ZPO dienen (ADR-1118 (lokal) Z. 98). Die Übermittlungsoptionen (ADR-1119) sind ein technischer Mechanismus, keine rechtliche Wahl zwischen einfacher und förmlicher Zustellung **[Folgerung]**.
- R1: Qualifiziertes Siegel des Absenders für den Schriftformersatz (ADR-0017 (lokal) Z. 11).

## Abgleich morph_box.csv

Geprüft wurden alle ZaPuK-Verweise in `morph_box.csv` (Stand 2026-11-19) gegen `main` und den Entwurf v1.1.

### Korrekt

- B3 A3 (ADR-1105, ADR-1111, Glossar; Zuschnitt der Bereiche offen): stimmt.
- B1 A7 und I5 A4 (ADR-0008, ADR-1148, ADR-1144, ADR-1145) stimmen inhaltlich.
- Ü1 A3 (ADR-1102).
- Ü2 A2 (ADR-1100 Option A, ADR-0012), A4 (Key Escrow, mit ADR-1147 unvereinbar = Option C ausgeschlossen), A5 (ADR-1147), A6 (ADR-1101 Option B).
- Ü5 A1 bis A4 (ADR-0010 Optionen 4, 1, 2, 3).
- Ü6 A1 (Option A nicht gewählt), A2 (B), A3 (C), A4 (D ausgeschlossen), A5 (E), A6 und A7 (ADR-1130 Z. 128, Z. 143).
- P9 A2 (ADR-1110), I2 A3 (ADR-0002, ADR-0004), I2 A6 (ADR-1135), I3 A1 bis A3 (ADR-1103), I4 A1 und A2 (ADR-1102 Optionen A und B), I4 A4 (ADR-1101 Option C), I5 A2 (ADR-1145 Option A), I5 A3 (ADR-1145 Option B), I1 A4 (ADR-1114), P6 A5.

### Abweichungen und Präzisierungen

1. **Status veraltet:** ADR-1105, ADR-1106, ADR-1109 und ADR-1147 sind seit 12.11.2026 nur noch "Richtung entschieden" (ADR-IDX Z. 46–64). Betroffen: B3 A3, P2 A1, I4 A3, Ü2 A5. Die "kritische Annahme" bei B3 A3 wird dadurch gestützt.
2. **B1 A3 (FIT-Connect):** Die Belege ADR-0009, ADR-0011, ADR-0012 tragen die Aussage nicht direkt. ADR-0011 hat FIT-Connect als Kommunikationsschicht verworfen (Option 6). Die Aussage "FIT-Connect als Anbindungsweg, Voraussetzung Matrix und MLS" steht in ZF10 Z. 232. Im Entwurf v1.1 ist FIT-Connect außerdem Entwickler der Backends, Betreiber der Organisations-Backends und Träger von SSP und SDK (AK11 Z. 205–217, Z. 342).
3. **B1 A8 (Channel Adapter, keine Messaging-Bridge):** entspricht v1.0 (AK10 Z. 401–403). Der Entwurf v1.1 sieht in der Transition Bridges ausdrücklich vor (ZBP-Matrix-Bridge, ELSTER-Zustell-Bridge, Postfach-2.0-Bridge; AK11 Z. 636; TR Z. 63). Der Channel Adapter kommt in v1.1 nicht mehr vor. Vorschlag: "übergangsweise Bridge oder Legacy-Fassade für nicht E2EE-verschlüsselte Bestandsnachrichten (ZaPuK v1.1 Entwurf, Transitionsstrategie)" ergänzen.
4. **B1 A6 (SDK):** ZaPuK fehlt, obwohl ADR-0009 das SDK festlegt. B1 A7 nennt ADR-1101 als Beleg; passender sind ADR-0003 (Offenheit für Drittanwendungen) und ADR-0009. ADR-1101 betrifft nur E2EE-Endpunkte.
5. **B1 A4 (Landesportale, Servicekonten, kommunale Rechenzentren als Backend-Betreiber):** ZaPuK nennt "behördliche Backends für die eigene Behörde und ggf. Nachnutzende" (ADR-0004 (main) Z. 86) und Betrieb durch IT-Dienstleister in Auftragsverarbeitung (ADR-1103, Z. 48 lokal). Landesportale und Servicekonten nennt ZaPuK nicht **[Folgerung]**.
6. **B2:** Eine ZaPuK-Option fehlt: zentraler Postfach-Zugang (Web und App) auch für Behörden, mit Funktionspostfächern im Betriebsmodell A (ADR-0005, ADR-1103).
7. **B4:** ZaPuK fehlt bei A2 (Anhänge), A3 (strukturierte Daten, ANF_FUN_NAC_183, ADR-0001) und A6 (vordefinierte Antwortoptionen, ANF_FUN_NAC_125). Neu wäre eine Option "Abholschein für einen Nachweis in die EUDI-Wallet zusammen mit dem Bescheid" (ADR-1146). A4 passt teilweise: gesiegeltes Dokument der Behörde nach ADR-0017.
8. **P1:** ZaPuK fehlt bei A2 (E-Mail-Hinweis) und A3 (Push), beides ANF_FUN_ZUS_026 und AK11 Z. 571. Das zugehörige ADR ist unbearbeitet.
9. **P2 A8 ("ZaPuK ADR-0014" beim Fachzertifikat):** ADR-0014 nennt ELSTER-Zertifikate nur als Anmeldemittel der BundID (Z. 72). Für Organisationen ist das ELSTER-Organisationszertifikat Identifizierungsmittel im Org-Konto (AK11 Z. 799), Systeme weisen sich per X.509 aus (ADR-0015, ADR-0016). ZaPuK fehlt bei A2 (BundID und Org-Konto mit Vertrauensniveaus, ADR-0014 Option 2, ADR-0015 und ADR-0016) und A3 (EUDI-Wallet, ADR-0014 Option 3, ADR-1146).
10. **P3 A1 ("von ZaPuK ausgeschlossen"):** ADR-1102 nennt Option A "nicht gewählt" (Z. 79), nicht "ausgeschlossen"; die Begründung lautet "widerspricht Zero Trust". Bei P3 A2 (Organisationskonto mit Rollen) fehlt "ZaPuK ADR-1112 Option A erwogen, nicht gewählt". P3 A4 sollte ADR-1108 (Org-IdP-Adapter) und ADR-1112 nennen.
11. **P4:** Bei A3 (Postvollmacht in der d-box) fehlt "ZaPuK ADR-1113 Option B erwogen, nicht gewählt". Bei A4 (zentrales Vollmachtsregister): ZaPuK sieht keins vor (ADR-1113 Z. 57; ZF11 Z. 115). A5 sollte ADR-1113 Option A nennen.
12. **P5:** ZaPuK fehlt bei A5 (Drittclients über offene Schnittstelle, ADR-0003) und bei A1 und A3 (laut Entwurf Bundesportal und Deutschland-App, AK11 Z. 411). Fachportale mit Bereichssicht (AK11 Z. 524) sind eine eigene Variante.
13. **P6 A4 ("technischer Nutzer, Anbindung über SDK (ZaPuK Teilbetrieb)"):** Eine Anbindung von Unternehmensanwendungen über API oder SDK ist in allen Betriebsmodellen möglich (AK11 Z. 528). An den Teilbetrieb gebunden ist nur der RMS mit E2EE-Teilnahme. Bei P6 A3 (offene API) fehlt ZaPuK (ADR-0003, Matrix-Client-Server-API, Client Credentials).
14. **P7:** Bei A3 (Antwort an die sendende Behörde) fehlt ZaPuK (bidirektional, Behörde kann Antworten untersagen; ANF_FUN_NAC_185, ANF_FUN_ADR_058).
15. **Ü2 A1:** Hier kann "ZaPuK ADR-1100 Option B ausgeschlossen" ergänzt werden. Bei Ü2 A6 ist "kritische Annahme" gedeckt: ADR-1101 (v1.1) Z. 101 nennt die E2EE in diesen Räumen "nicht strikt umgesetzt".
16. **Ü3:** ZaPuK fehlt ganz. Vorschläge: A1 "ZaPuK ADR-0013 Optionen 1 und 2 erwogen, nicht gewählt"; A2 "Siegel des Betreibers": nach ZaPuK nur auf der Bereitstellungsbestätigung (ADR-0018), ein Signaturdienst des Systems ist verworfen (ADR-0017 Option A); A3 "Siegel der Behörde vor dem Versand (ZaPuK ADR-0017 Option B)"; neue Option "Identitätsnachweis je Nachricht im E2EE-Kanal (SD-JWT VC), prüfbar über Trust-Liste (ZaPuK ADR-0013 Option 3, ADR-1106, ADR-1107, ADR-1108)".
17. **Ü4:** ZaPuK fehlt ganz. Vorschläge: A1 und A5 (ADR-0018, Bereitstellungsbestätigung an alle Teilnehmenden), A2 und A3 (ADR-1118 auf Anforderung), A4 (ADR-0018 Optionen 3 und 5 erwogen, nur als Indiz), neue Variante "Löschquittung" (ADR-0018 Z. 185).
18. **I2:** Bei A1 (ein Backend für alle) fehlt "ZaPuK ADR-0006, v1.0, durch ADR-1114 angepasst". A2 (getrennte Backends je Nutzergruppe) gilt auch für ZaPuK (ADR-1114: vier Backends). ZaPuK ist damit eine Kombination aus A2 und A3 **[Folgerung]**.
19. **I4 A3:** ADR-1109 (FP-Bot, Optionen B und C) fehlt in der Liste. Es fehlen ZaPuK-Varianten: "manuell verwaltete Gruppenräume ohne technischen Nutzer (ADR-1101 Option A, höchster Schutzbedarf)" und "Entschlüsselung außerhalb der Organisation (ADR-1101 Option D, ausgeschlossen)".
20. **I5 A4:** ADR-1148 (manuelle Freigabe der Backends) und ADR-1148 Optionen A und C fehlen.
21. **R3 A4 (ADR-1119 als "Behörde wählt je Nachricht einfach oder nachweislich"):** ADR-1119 steuert, welche Nachweise erzeugt werden (Bereitstellung, Abruf, Lesen, Empfangsbekenntnis). Über die rechtliche Zustellart entscheidet es nicht **[Folgerung]**; Prüfung im Ordner `recht-governance/`.
22. **I1 A4:** Die Formulierung "Verbund staatlicher Betreiber über Standard" ist enger als ZaPuK. Auch nicht-staatliche Betreiber dürfen Backends für Organisationen betreiben (ADR-0004 (main) Z. 82–86; ADR-1144 (main) Z. 120).

### Fehlende ZaPuK-Dimensionen (keine passende Zeile in morph_box.csv)

- Nachweis der fachlichen Identität je Nachricht und Trust-Liste (ADR-1106 bis 1108), nur teilweise in Ü3.
- Metadatenschutz und Betreibertrennung (ADR-1114, AK11 Z. 940–944), nur indirekt in I1 und I2.
- Schlüsselwiederherstellung (ADR-1147) steht nur als Option in Ü2.
- Beweissichere Anhänge, Pull-Prinzip (ADR-1120).

## Vorschlag Zelltexte für eine Spalte "ZaPuK" in existing_solutions.csv

Knapp, Stand `main` 2026-11-12, Entwurfsinhalte gekennzeichnet. Belege siehe oben.

- B1: Fachverfahren als eigener Matrix-Client über offene API oder optionales SDK, Registrierung im zentralen SSP, M2M über Client Credentials [Entwurf v1.1]; FIT-Connect soll SDK, SSP und Backends für Organisationen stellen [Entwurf v1.1]; Bestandsschnittstellen übergangsweise über Bridges (ZBP-API, ELSTER) [Entwurf v1.1]; EGVP über OSCI-Matrix-Protokollkonverter
- B2: zentraler Postfach-Zugang (Web, App) für alle Gruppen auch ohne Fachverfahren; Funktionspostfächer über zentralen RMS (Betriebsmodell A); kein Massenversand- oder Druckdienst beschrieben
- B3: Privatpersonen: pseudonyme Bereichs-MXID je Bereich über Zuordnungsdienst, kommt mit dem Antrag, kein Adressregister, kein Erstkontakt ohne Verfahren (Richtung entschieden, Bereichszuschnitt offen); Organisationen: föderiertes Adressregister, MXID ohne Personenbezug; Justiz über SAFE
- B4: Text, Anhänge, strukturierte Daten und Antwortoptionen (Anforderungen); vom Absender gesiegelte Dokumente; Abholschein für EUDI-Wallet-Nachweise
- P1: Benachrichtigungsdienst im Backend per E-Mail und Push [Entwurf v1.1]; eigenes ADR unbearbeitet
- P2: eID, BundID, EUDI-Wallet (perspektivisch); Organisationen über Org-Konto (MUK) und X.509 (V-PKI, ELSTER-PKI), eigenes IdM über Org-IdP-Adapter; Vertrauensniveau im Identitätsnachweis
- P3: Funktionspostfächer mit Vorgangsräumen und Raum-Management-Service, Berechtigungsverzeichnis zentral oder aus eigenem IdM; geteilte Zugangsdaten verworfen
- P4: Vertretung mit eigener Identität und eigenem Postfach, Vollmacht als Nachweis oder signiertes Dokument; kein zentrales Vollmachtsregister
- P5: ein zentraler Zugang (Web, iOS, Android) im Bundesportal und in der Deutschland-App [Entwurf v1.1]; Fachportale und Drittclients über offene API ohne Zulassung
- P6: Unternehmensanwendungen über API oder SDK, automatisiert über Client Credentials [Entwurf v1.1]; eigenes Backend im souveränen Betrieb
- P7: bidirektional, Behörde kann Antworten untersagen; anlassloser Kontakt zu Behörden über Adressregister, Bereich wird automatisch gewählt
- P8: wie B4 (Text, Anhänge, strukturierte Daten); QES von Privatpersonen perspektivisch über EUDI-Wallet
- P9: Postfach vom Nutzerkonto getrennt, eigene Adresse, alle Identifizierungsmittel; § 9 OZG anzupassen [Entwurf v1.1]
- Ü1: Matrix-Räume je Vorgang, asynchron und Echtzeit
- Ü2: E2EE bis Endgerät (Matrix, Olm/Megolm, Ziel MLS); bei Funktionspostfächern optional technischer Nutzer mit Schlüsseln im Machtbereich der Organisation; Key Backup nur mit Wiederherstellungsschlüssel der Person, kein Key Escrow (Richtung entschieden)
- Ü3: Authentizität im E2EE-Kanal mit Identitätsnachweis je Nachricht (SD-JWT VC, Trust-Liste); Dokumentechtheit über Siegel des Absenders; Organisationsadresse über signierten Registereintrag prüfbar
- Ü4: Bereitstellungsbestätigung des Empfänger-Backends für alle Teilnehmenden (Siegel, qualifizierter Zeitstempel, ohne Klartext); Abruf- und Lesebestätigung clientseitig auf Anforderung; Löschquittung
- Ü5: Protokollkonverter auf E2EE-Ebene (OSCI für EGVP, ggf. AS4-Gateway); Bridges nur übergangsweise in der Transition [Entwurf v1.1]
- Ü6: an E2EE-Endpunkten: Endgerät, Server der Organisation, Scandienst der Organisation; Pflichtprüfung vor Versand an Privatpersonen; Formatbeschränkung im Zugang; kein zentraler Scanner
- I1: getrennte Betreiber für Privatpersonen-Backend und Organisations-Backends; BMDS bzw. FITKO/FIT-Connect [Entwurf v1.1]
- I2: föderiert, vier zentrale Backends je Teilnehmendengruppe plus eigene Backends von Organisationen; keine zentrale Aufbewahrung, Übernahme in die E-Akte durch den Datenbesitzer
- I3: Minimalanbindung, Teilbetrieb (eigener RMS) oder souveräner Betrieb (eigenes Backend, eigene Domain), frei wählbar
- I4: Vorgangsraum je Fall mit FP-Bot; schlüssellos (zentral, strikte E2EE) oder vermittelnd mit Schlüsseln auf Org-Server; Ausnahme E2EE bis Org-Grenze
- I5: risikobasiert: Anschlussvertrag im SSP, manuelle Freigabe der Backends, Föderationsliste mit Suspendierung und Ausschluss, Zertifizierungsstelle des Verbunds; keine Zertifizierung je Backend
- R1–R4: siehe `recht-governance/`

## Widersprüche und Unstimmigkeiten in den Quellen

- **Bridge-Verbot vs. Transitions-Bridges:** AK10 Z. 401 verbietet Messaging-Bridges, der Entwurf v1.1 sieht sie übergangsweise vor (AK11 Z. 636; TR Z. 63). AK11 Z. 872 sagt weiterhin "Bridges sollen dagegen nicht verwendet werden" (bezogen auf ADR-0010 für Systeme außerhalb).
- **Eigene IdP:** AK10 Z. 345 "nicht unterstützt" vs. ADR-1108 Org-IdP-Adapter.
- **Ein vs. vier Backends:** ADR-0006, AK10 Z. 321 und Z. 413 vs. ADR-1114, ADR-1103 Z. 147.
- **Statusangaben:** `zapuk-main/` führt ADR-1105, ADR-1106, ADR-1109, ADR-1147 als "Operativ entschieden", `main` als "Richtung entschieden". `sources.md` Abschnitt 1 stützt sich auf den alten Stand (u. a. "No messaging bridges", Liste der offenen ADRs ohne "nicht entschlüsselbare Nachrichten").
- **Tippfehler** "Seit 12.12.2026" im ADR-Index auf `main` (Z. 11).
- **AK11 Kap. 3** verweist noch auf v1.0, "Inhalte werden vor Einreichung zur 51. Sitzung … übertragen" (AK11 Z. 164): Der Entwurf ist unvollständig.

## Lücken

- Der Entwurfsbranch `architecture-v1.1` ist nicht gemergt; alle Aussagen daraus sind vorläufig. Ob der IT-Planungsrat v1.1 bestätigt hat, ist hier nicht geprüft (siehe `recht-governance/`).
- Antworten des ZaPuK-Teams auf die Konsultations-Issues sind ohne openCode-Anmeldung nicht lesbar. Eine veröffentlichte Auswertung der Konsultation Phase 2 wurde nicht gefunden.
- Unbearbeitete ADRs: API-Autorisierung (1149), Benachrichtigungsdienst, Löschung und Aufbewahrung, Metadatenminimierung, nicht entschlüsselbare Nachrichten (ADR-IDX Z. 66–70).
- Nicht gefunden: Massenversand, Druck- oder Papier-Rückfall, SMS, Erinnerung bei Nichtabruf, Mengengerüst (im Transitionsplan ZBP als offener Klärungspunkt, TR-A Z. 148), konkrete SDK-Sprachen, Größenlimits für Anhänge (Anforderung "beliebig groß", ANF Z. 84).
- Für BundesMessenger und TI-Messenger gibt es keine Transitionspläne; sie werden nur im Vergleich behandelt (AK11 Z. 880–894).
- Die Abbildungen der Transitionspläne (Phasen und Zeitachsen) liegen nur als Bild vor und wurden nicht ausgewertet.
