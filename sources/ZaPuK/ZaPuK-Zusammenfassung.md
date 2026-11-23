# ZaPuK (Zielarchitektur für Postfach- und Kommunikationslösungen): Zusammenfassung für die d-box

Wie die ZaPuK, die föderale Zielarchitektur für Postfach- und Kommunikationslösungen (FIT-AB, FITKO, IT-Planungsrat), die künftige Postfachinfrastruktur aufsetzt: Architektur, Betrieb, Recht, Transition der Bestandslösungen. Eingeordnet nach den Zeilen von `morph_box.csv`. Die Zusammenfassung verweist auf die Quellen, statt sie zu wiederholen. Für Details den Pointer öffnen.

Anders als die anderen Spalten in `existing_solutions.csv` beschreibt ZaPuK **keine laufende Lösung**, sondern ein Zielbild. Laufende Software gibt es nur aus dem Pilotvorhaben **Neo** (Matrix-basierter Kommunikationskanal für die BundID, abgeschlossen am 23.06.2026, ohne Wirkbetrieb). Die Zusammenfassung trennt deshalb, was entschieden, was Entwurf und was im Neo-Pilot gezeigt ist.

Stand: 2026-11-19. Grundlage:

- die lokale Kopie `../zapuk-main/` des openCode-Repos <https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk>,
- der Upstream-Stand `main` (Commit `cc150e2977`, 12.11.2026) und der nicht gemergte Entwurfsbranch `architecture-v1.1` (Commit `ef99118c06`, 13.11.2026),
- Beschlüsse des IT-Planungsrats und des FIT-AB, das Rechtsgutachten zur ZaPuK, Neo-Unterlagen und Presse.

Alle neuen Quellen liegen in `quellen/` in drei Unterordnern. Jeder Unterordner hat eine `_index.md` mit URL, Herausgeber, Datum und Inhaltsnotiz und eine `_befunde.md` mit den Einzelbefunden je Zeile.

## Kennzeichnung

- Ohne Kennzeichnung: in einer Primärquelle belegt (ADR auf `main`, Beschluss, Gesetz).
- **[Entwurf]**: nur im Branch `architecture-v1.1` (Architekturkonzept v1.1, Überblick v1.1, Transitionspläne, Rechtsgutachten). Die Version 1.1 soll der IT-Planungsrat in seiner 51. Sitzung am 25.11.2026 beschließen.
- **[Folgerung]**: aus mehreren Quellen geschlossen, nicht ausdrücklich belegt.
- **[unverifiziert]**: nur Sekundärquelle oder Annahme.
- **[veraltet?]**: Quelle ist älter, durch neuere Fassung überholt oder aktueller Stand nicht belegt.

Status der ADRs nach dem Index auf `main`: „Operativ entschieden“ (auf Arbeitsebene gefasst; ADR-0000 bis ADR-0016 zusätzlich Teil von B-2025/28-IT), „Richtung entschieden“ (seit 12.11.2026 für ADR-1105, ADR-1106, ADR-1109, ADR-1147), „Unbearbeitet“ (Bedarf erkannt, kein ADR).

## Kürzel für Quellen

Pfade relativ zu `quellen/`: `AT/` = `architektur-technik/`, `RG/` = `recht-governance/`, `UR/` = `umsetzung-rezeption/`; `ZK/` = `../zapuk-main/`. Z. = Zeile der Datei, S. = PDF-Seite. In `AT/` beginnen Kopien mit fünf Kopfzeilen, Originalzeile = Z. minus 5.

| Kürzel | Datei | Inhalt |
|---|---|---|
| ADR-xxxx | `AT/fitko_zapuk-adr-xxxx-…_main_2026-11-12.md` (auf `main` geändert) oder `ZK/Zielarchitektur/Architekturentscheidungen/ADR-xxxx-….md` (unverändert) | Architekturentscheidungen; Zuordnung je ADR in `AT/_befunde.md` Z. 11–13 |
| ADR-IDX | `AT/fitko_zapuk-adr-index_main_2026-11-12.md` | ADR-Index mit Status und offenen Punkten |
| AK10 | `ZK/Zielarchitektur/Architekturkonzept/README.md` | Architekturkonzept v1.0 (Sommer 2025), Kap. 3 Brownfield, Bridge-Verbot **[veraltet?]** |
| AK11 | `AT/fitko_zapuk-architekturkonzept-v1.1_v1.1-entwurf_2026-11-13.md` (Kopie auch `RG/FITKO_ZaPuK-Architekturkonzept-v1-1_2026-09-25.md`) | Architekturkonzept v1.1 **[Entwurf]** |
| ZF10 / ÜH | `ZK/Zielarchitektur/Zusammenfassung/README.md` / `RG/FITKO_ZaPuK-Ueberblick-Handlungsempfehlungen-v1-1_2026-10-06.md` (Kopie auch `AT/fitko_zapuk-zusammenfassung-v1.1_…`) | Überblick v1.0 mit Kosten (3.3) / Überblick und Handlungsempfehlungen v1.1 **[Entwurf]** |
| RGA | `RG/FITKO_ZaPuK-Rechtsgutachten-v1-1_2026-10-06.md` (Kopie auch `AT/fitko_zapuk-rechtsgutachten_…`) | Rechtsgutachten, Eichler Kern Klein und PLANIT // LEGAL i. A. der FITKO, Stand 06.10.2026 **[Entwurf]** |
| ANF | `ZK/Konsolidierte-Anforderungen/README.md` | Anforderungen v0.7.1 (24.02.2026) |
| GLOS | `AT/fitko_zapuk-glossar_main_2026-11-12.md` | Glossar mit Bereich, Betriebsmodellen |
| TS / TA / TB / TC / TE1 / TE2 | `UR/fitko_zapuk-transitionsstrategie_…`, `UR/fitko_zapuk-transition-a-zbp_…`, `-b-mjp_…`, `-c-muk_…`, `UR/fitko_zapuk-tadr-mein-elster-001_…`, `-002_…` (Kopien auch in `AT/`) | Transitionsstrategie und Transitionspläne ZBP, MJP, MUK, Mein ELSTER **[Entwurf]** |
| ISS / KON | `AT/fitko_zapuk-opencode-issues-architektur_2026-11-19.md`, `UR/fitko_zapuk_konsultation-phase2-stellungnahmen_2026-02-bis-04.md` | Konsultationsbeiträge (Dataport, RLP, SET u. a.) |
| FITAB | `RG/FITKO_FIT-AB-Beschluesse_2026-11-19.md`, `UR/fit-ab_beschluesse-2026-11-und-2026-12_2026-09-15.md` | FIT-AB 2026/11 (Abschluss Neo), 2026/12 (Empfehlung Phase III) |
| B18 / B03 / B2528 | `../BundID-ZBP/quellen/recht-governance/IT-PLR_B-2026-18-IT_…`, `…_B-2026-03-IT_…`, `…_Beschluss_2025-28_ZaPuK.md` | IT-PLR-Beschlüsse: Basiskomponente Postfach/Benachrichtigung, D-Stack, ZaPuK v1.0 |
| PM50 / D7319 | `RG/IT-PLR_Pressemitteilung_50-Sitzung_2026-06-17.md`, `RG/BT-Drs_21-7319_Antwort_KA_EUDI-Wallet.pdf` | Pressemitteilung 50. Sitzung; Antwort der Bundesregierung (FITKO federführend) |
| VA / IL / VS | `UR/fitko_neo_fit-ab-vorhabensabschluss_2026-06-23.pdf`, `UR/fitko_neo_integrationsleitfaden_2026-07-23.md`, `UR/fitko_neo_vorhabensseite_2026-07.md` | Neo: Abschluss, Integrationsleitfaden, Vorhabensseite |
| PI / OC | `UR/fitko_neo_planning-issues-auszug_2026-11-19.md`, `UR/fitko_neo_opencode-projektuebersicht_2026-11-19.md` | Neo-Arbeitsstände (Issues), Repositories |
| NEO | `../neo-solution-architecture-develop/` | Neo arc42-Snapshot |
| Presse | `UR/heise_*`, `UR/heise-ct_*`, `UR/netzpolitik_*`, `UR/d64_*`, `UR/element_*`, `UR/set_*` | Sekundärquellen; Element und SET mit Eigeninteresse |

## 1. Systembild

- **Ziel:** ZaPuK ordnet die Kommunikation zwischen öffentlichen Stellen, privaten Organisationen und Bürger:innen neu, von der Zustellung von Bescheiden bis zu Rückfragen in Echtzeit. Bestehende Postfächer (ZBP der BundID, Mein Justizpostfach, OZG-PLUS-Postfach und Postfach 2.0 im Unternehmenskonto, Mein ELSTER) sollen konsolidiert werden (`ZK/README.md` Z. 39, Z. 53; FITAB). Nach B-2026/18-IT wird die Basiskomponente „Postfach/Benachrichtigung“ des Deutschland-Stacks nach den Vorgaben der ZaPuK aufgebaut (B18 Z. 23).
- **Topologie:** hybride, föderierte Client-Server-Architektur (ADR-0002). Vier zentrale Postfach-Backends je Teilnehmendengruppe: Privatpersonen, private Organisationen, öffentliche Stellen, Justiz. Das Backend der Privatpersonen hat einen anderen Betreiber als die Backends der Organisationen (ADR-1114). Organisationen und Dritte dürfen eigene Backends betreiben (ADR-0004), Privatpersonen nicht.
- **Protokoll:** Matrix als Kommunikationsschicht (ADR-0011), MLS als Ziel der Ende-zu-Ende-Verschlüsselung, übergangsweise Olm/Megolm (ADR-0012). Die Verschlüsselung endet im Endgerät (ADR-1100).
- **Zugänge:** ein zentraler Postfach-Zugang für alle Gruppen (ADR-0005), laut Entwurf als Web-App und mobile Apps im Bundesportal und in der Deutschland-App (AK11 Z. 221, Z. 411) **[Entwurf]**. Daneben Drittzugänge ohne Zulassung: Fachverfahren, Unternehmensanwendungen, Fachportale wie ELSTER (ADR-0003).
- **Anbindung:** zentrales Self-Service-Portal (SSP) und optionales SDK (ADR-0008, ADR-0009, ADR-1148). Laut Entwurf v1.1 entwickelt FIT-Connect die Backends, betreibt die Backends der Organisationen und bringt SSP und SDK ein (AK11 Z. 205–217) **[Entwurf]**.
- **Postfach und Identität:** Postfach getrennt vom Nutzerkonto (ADR-1110). Privatpersonen haben pseudonyme Adressen je Bereich (ADR-1105), Organisationen ein föderiertes Adressregister (ADR-1111). Die fachliche Identität wird je Nachricht im verschlüsselten Kanal nachgewiesen (ADR-1106 bis ADR-1108).
- **Historie:**
  - Phase I (09/2024–06/2025) durch das FIT-AB nach B-2024/28-IT; v1.0 beschlossen mit B-2025/28-IT.
  - Phase II „Transitionsplanung und Validierung“ unter Federführung von FITKO, Hamburg und Sachsen-Anhalt; Pilot Neo (Zuschlag an adesso, Element und Nordeck am 03.06.2025, Abschluss FIT-AB 2026/11 am 23.06.2026).
  - Phase III (Umsetzung) unter Federführung des BMDS, Beschluss in der 51. Sitzung am 25.11.2026 vorgesehen, Start voraussichtlich Anfang 2027 (`ZK/README.md` Z. 49, Z. 304; ÜH Z. 53).

## 2. Betrieb, Governance, Finanzierung

- **Verantwortlichkeiten nach Entwurf v1.1** (ÜH Z. 133–139; AK11 Z. 294–305) **[Entwurf]**:
  - BMDS: zentraler Zugang und Backend der Privatpersonen, ggf. delegiert an eine nachgeordnete Behörde oder einen Dienstleister. Betriebsumgebung offen: BundID-Umgebung oder ITZBund (TA Z. 36, Z. 152).
  - FITKO-Produktmanagement (FIT-Connect): Entwicklung aller Backends, Betrieb der drei Backends der Organisationen, SSP, SDK, Adressregister, Protokollkonverter.
  - BLK: SAFE-Register.
  - Steuerung über die „Governance des D-Stacks (in Ausgestaltung)“, Phase III mit Steuerungskreis aus BMDS, FITKO und Ländern (ÜH Z. 305).
- **FIT-AB 2026/12** (15.09.2026) empfiehlt Phase III im D-Stack unter Koordination des Bundes, Frontend in Bundesportal und D-App, Backends „in voneinander unabhängigen Betriebsorganisationen“, FIT-Connect-SSP als Rollout-Infrastruktur (FITAB).
- **Verfassungsrecht laut Rechtsgutachten:** Keine Einzelnorm reicht. Tragfähig ist eine Kombination aus Art. 86 GG (Entwicklung beim Bund), Art. 87 Abs. 3 und Art. 91c Abs. 5 GG (Betrieb des Backends der Privatpersonen durch eine Bundesbehörde, Anschlussverpflichtung per Bundesgesetz) und einem Staatsvertrag nach Art. 91c Abs. 1/2 GG für die Delegation an die FITKO (RGA Z. 1155–1162) **[Entwurf]**.
- **Finanzierung:**
  - Kostenschätzung nur aus v1.0: Betrieb zentraler Bausteine 1,8–2,3 Mio. EUR/a, Weiterentwicklung 1,5–2 Mio. EUR/a (ZF10 Z. 198) **[veraltet?]**. v1.1 nennt keine Zahlen.
  - Das Finanzierungskonzept kommt zur 52. Sitzung am 17.03.2027 (B18 Z. 29–31; ÜH Z. 197–199).
  - Laut Gutachten folgt die Finanzierung der Vollzugszuständigkeit: Was der Bund selbst vollzieht, zahlt er; für an die FITKO delegierte Teile kommt keine alleinige Bundesfinanzierung in Betracht (RGA Z. 1112–1137).
- **Zeitplan:** keine festen Termine. Transitionsphasen A 3–6, B 6–8, C 1–2 Monate, Abschaltung der Altsysteme nach 24 Monaten (TS, ABB1) **[Entwurf]**. v1.0 sah Betrieb ab 2028 vor **[veraltet?]**.

## 3. Identität, Adressen, Schlüssel

- **Privatpersonen:** Anmeldung mit eID direkt, über die BundID mit ihren Mitteln oder perspektivisch mit der EUDI-Wallet (ADR-0014). Eine Nachweisstelle stellt einen kurzlebigen Identitätsnachweis aus, der je Nachricht mitgesendet wird (ADR-1106, Richtung entschieden; ADR-1107: SD-JWT VC).
- **Adressen von Privatpersonen:** bereichsspezifische MXID; ein Zuordnungsdienst verknüpft die Kennungen von BundID, eID und EUDI-Wallet. Die Behörde erhält die Adresse mit dem Antrag. Kein Adressregister für Privatpersonen, kein Erstkontakt ohne vorheriges Verfahren (ADR-1105, Richtung entschieden). In der Transition gilt zunächst das BundID-Postfachhandle, Pseudonyme folgen „mittelfristig“ (TA Z. 48) **[Entwurf]**.
- **Organisationen:** Organisationskonto und X.509 aus V-PKI oder ELSTER-PKI (ADR-0015, ADR-0016), eigenes Identitätsmanagement über Org-IdP-Adapter (ADR-1108). Adressen ohne Personenbezug (ADR-1104), föderiertes Adressregister mit signierten Einträgen (ADR-1111). Die Justiz nutzt SAFE weiter (AK11 Z. 302).
- **Schlüssel:** Key Backup auf dem Server, verschlüsselt mit einem Wiederherstellungsschlüssel, den nur die Person hat; kein Key Escrow (ADR-1147, Richtung entschieden). Bei Funktionspostfächern darf ein technischer Nutzer Schlüssel im Machtbereich der Organisation halten, die Verschlüsselung ist dann nicht strikt (ADR-1101 Option B).

## 4. Nachrichten und Funktionen

- **Inhalt:** Text, Anhänge in beliebigem Format, strukturierte Daten, vordefinierte Antwortoptionen (ANF Z. 84–101). Keine Fachlogik in der Infrastruktur, nur wenige querschnittliche Erweiterungen (ADR-0001). Siegel bringt der Absender vor dem Versand an, das System prüft sie nicht (ADR-0017). EUDI-Nachweise als Abholschein (ADR-1146).
- **Nachweise:** Bereitstellungsbestätigung des Empfänger-Backends ohne Klartext, mit qualifiziertem Zeitstempel und Siegel des Betreibers, an alle Raumteilnehmenden; Löschquittung (ADR-0018). Abruf- und Lesebestätigung clientseitig auf Anforderung (ADR-1118). Absendergesteuerte Übermittlungsoptionen (ADR-1119).
- **Funktionspostfächer:** Vorgangsraum je Fall, Raum-Management-Service lädt Berechtigte ein (ADR-1102), Berechtigungsverzeichnis zentral oder aus eigenem Identitätsmanagement (ADR-1112), Raumtopologie mit schlüssellosem oder vermittelndem Bot (ADR-1109). Betriebsmodelle Minimalanbindung, Teilbetrieb, souveräner Betrieb (ADR-1103).
- **Sicherheit und Aufbewahrung:** Malware-Prüfung nur an Endpunkten, Pflichtprüfung vor Versand an Privatpersonen (ADR-1130). Keine zentrale Aufbewahrung, Export in die E-Akte durch den Datenbesitzer (ADR-1135).
- **Neo-Pilot (tatsächlich gebaut):** iOS-App (nur TestFlight), Web-Client und Matrix-Modul im FIT-Connect-Java-SDK, gemeinsame Rust-Codebasis; Megolm statt MLS; fünf Nachrichtentypen (Informativ, Freitext, Einfach- und Mehrfachauswahl, Datei-Upload); Demo-Anbindungen GA-Lotse und Empfangsclient Baden-Württemberg; Nutzertests mit 32 Personen. Nicht erreicht: EUDI-Wallet, Self-Service-Registrierung von Behörden, technische Einschränkung der Antwortfunktion, vollständiger Betreiberausschluss bei Metadaten (VA S. 52–53).

## 5. Einordnung nach `morph_box.csv`

Die Zelltexte der Spalte `ZaPuK (Zielarchitektur, Deutschland)` in `existing_solutions.csv` fassen die folgenden Befunde zusammen. Belege je Zeile stehen in `../Referenzen-ZaPuK.md`.

### Behörde

| Zeile | ZaPuK |
|---|---|
| B1 Anbindung von Fachverfahren | Fachverfahren als eigener Matrix-Client über die offene API oder das SDK, Registrierung im SSP, keine Zulassung (ADR-0003, ADR-0008, ADR-0009, ADR-1148). FIT-Connect als Anbindungs- und Betriebsplattform **[Entwurf]**, als Protokoll verworfen (ADR-0011). Übergang über Bridges zu ZBP, ELSTER und Postfach 2.0 **[Entwurf]**. Passt zu Ausprägung 3, 6, 7 und 8. |
| B2 Zugang ohne Fachverfahren | zentraler Postfach-Zugang auch für Behörden, Funktionspostfächer über zentralen Raum-Management-Service (ADR-0005, ADR-1103). Kein Massenversand oder Druck **[unverifiziert]**. Passt zu Ausprägung 1. |
| B3 Adressierung der Person / Organisation | pseudonyme Bereichs-MXID, Organisationen über föderiertes Adressregister (ADR-1105, ADR-1111). In der Transition Ausprägung 1 (Postfachreferenz aus dem Antrag), im Ziel Ausprägung 3. |
| B4 Inhalt an die Person / Organisation | Text, Anhänge, strukturierte Daten, Antwortoptionen, Siegel der Behörde, EUDI-Abholschein. Passt zu Ausprägung 1–4, 6 und der neuen Ausprägung 8. |

### Person / Organisation

| Zeile | ZaPuK |
|---|---|
| P1 Benachrichtigung | E-Mail und Push (Anforderung, Entwurf v1.1), ADR unbearbeitet. Neo: Push in der App, E-Mail im Web, Push-Umsetzung **[unverifiziert]**. Ausprägung 2 und 3. |
| P2 Authentifizierung | eID, BundID, perspektivisch EUDI-Wallet; Organisationen über Organisationskonto, X.509, Org-IdP-Adapter (ADR-0014 bis ADR-0016, ADR-1108). Ausprägung 1–3 und 8. |
| P3 Organisationszugang | Funktionspostfächer mit Vorgangsräumen, Berechtigungen aus Verzeichnis oder eigenem IdM (ADR-1102, ADR-1112). Ausprägung 3 und 4. |
| P4 Vertretung | eigene Identität und eigenes Postfach, Vollmacht als Nachweis (ADR-1113 Option A). Ausprägung 5. |
| P5 Clients | zentraler Client im Bundesportal und in der D-App **[Entwurf]**, Fachportale und Drittclients (ADR-0003, ADR-0005). Ausprägung 3 und 5; Neo als Pilot zu Ausprägung 2. |
| P6 Anbindung eigener Systeme | API oder SDK, Client Credentials **[Entwurf]**, eigenes Backend (ADR-1103). Ausprägung 3, 4 und 5. |
| P7 Antwort und Erstkontakt | bidirektional, Behörde kann Antworten untersagen; anlasslose Nachricht über das Adressregister (ADR-1111). Ausprägung 3 und 4. |
| P8 Inhalt an die Behörde | wie B4. Ausprägung 1–3; Ausprägung 4 legt ZaPuK nicht fest **[Folgerung]**. |
| P9 Postfach und Nutzerkonto | getrennt (ADR-1110). Ausprägung 2. |

### Übermittlung

| Zeile | ZaPuK |
|---|---|
| Ü1 Modus | Matrix-Räume je Vorgang (ADR-0011, ADR-1102). Ausprägung 3. |
| Ü2 Vertraulichkeit | Ende-zu-Ende bis Endgerät (ADR-1100), Ausnahme technischer Nutzer (ADR-1101 Option B), Key Backup (ADR-1147). Ausprägung 2, 5 und 6. |
| Ü3 Echtheit | Identitätsnachweis je Nachricht im Kanal (ADR-0013, ADR-1106 bis ADR-1108), Siegel des Absenders (ADR-0017). Ausprägung 3 und neue Ausprägung 6. |
| Ü4 Ereignisse und Bestätigungen | Bereitstellungsbestätigung an alle (ADR-0018), Abruf und Lesen auf Anforderung (ADR-1118), Löschquittung. Ausprägung 1, 2, 3, 5 und neue Ausprägung 8. |
| Ü5 Interoperabilität mit Systemen außerhalb | Protokollkonverter (ADR-0010), konkret OSCI-Matrix für EGVP **[Entwurf]**. Ausprägung 3. |
| Ü6 Malware-Prüfung | Endgerät, Server der Organisation, Scandienst der Organisation, Pflichtprüfung vor Versand, Formatbeschränkung (ADR-1130). Ausprägung 2, 3, 5, 6 und 7. |

### Infrastruktur und Betrieb

| Zeile | ZaPuK |
|---|---|
| I1 Betrieb und Governance | getrennte Betreiber (ADR-1114), BMDS und FITKO/FIT-Connect **[Entwurf]**. Ausprägung 4. |
| I2 Datenhaltung und Versand | vier zentrale Backends plus eigene Backends, keine zentrale Aufbewahrung (ADR-1114, ADR-1135). Kombination aus Ausprägung 2, 3 und 6 **[Folgerung]**. |
| I3 Betriebsmodell für Organisationen | alle drei Modelle wählbar (ADR-1103). Ausprägung 1–3. |
| I4 Funktionspostfächer und interne Bearbeitung | Vorgangsraum mit Bot (ADR-1102, ADR-1109), manuelle Räume für hohen Schutzbedarf, Ausnahme bis zur Organisationsgrenze (ADR-1101). Ausprägung 3, 4 und neue Ausprägung 7. |
| I5 Aufnahme in den Verbund | risikobasiert (ADR-1144, ADR-1145, ADR-1148). Ausprägung 4. |

### Recht

| Zeile | ZaPuK |
|---|---|
| R1 Schriftformersatz | ZaPuK allein ersetzt die Schriftform nicht. Ab 01.07.2027 genügt im Regelfall ein elektronisches Dokument (§ 3a Abs. 2 VwVfG n. F.); als sicherer Übermittlungsweg nach § 3c VwVfG n. F. erst nach Rechtsverordnung und nur für das Einreichen (RGA Z. 1544–1600) **[Entwurf]**. Das Zielbild „beide Richtungen“ braucht eine Gesetzesänderung **[Folgerung]**. |
| R2 Teilnahmepflicht | freiwillig, Zugangseröffnung je Antrag oder Registrierung im SSP; Gutachten empfiehlt Opt-in je Bereich mit gesetzlicher Fiktion (RGA Z. 1191–1240). Ausprägung 1, Empfehlung nahe Ausprägung 2. |
| R3 Bekanntgabe und förmliche Zustellung | Nachweise für Bekanntgabe vorhanden (ADR-0018, ADR-1118, ADR-1119); § 9 OZG und § 122a AO sind für das entkoppelte Postfach anzupassen; förmliche Zustellung nach § 5 Abs. 5 VwZG schon möglich, Hürde ist das Empfangsbekenntnis mit QES (RGA Z. 1435–1543). Ausprägung 1, 3 und 4. |
| R4 Teilnahme der Behörden | heute Ausprägung 1; Ziel Ausprägung 3 über verbindliche Festlegung in der 51. Sitzung **[Entwurf]**, Bundesgesetz nach Art. 91c Abs. 5 GG oder Staatsvertrag (RGA Z. 1060, 1160–1161). |

## 6. Widersprüche und offene Punkte

- **Bridge-Verbot:** v1.0 verbietet Messaging-Bridges und erlaubt nur Channel Adapter für Frontends (AK10 Z. 401–403). Der Entwurf v1.1 sieht Bridges übergangsweise vor, wo Nachrichten im Bestand ohnehin unverschlüsselt liegen (TS Z. 68; TA; TC; TE2). Der Begriff Channel Adapter kommt in v1.1 nicht mehr vor.
- **Ein oder vier Backends:** ADR-0006 und AK10 Z. 321, Z. 413 sehen ein zentrales Backend vor, ADR-1114 und ADR-1103 vier **[veraltet?]**.
- **Eigene Identity Provider:** AK10 Z. 345 „nicht unterstützt“, ADR-1108 sieht den Org-IdP-Adapter vor.
- **Status der ADRs:** `../zapuk-main/` führt ADR-1105, 1106, 1109 und 1147 als „Operativ entschieden“, `main` seit 12.11.2026 als „Richtung entschieden“. Der Index auf `main` schreibt „Seit 12.12.2026“ (Tippfehler).
- **Länderpflicht:** B-2026/18-IT Ziff. 5 verpflichtet die Länder nur für die Komponenten der Ziff. 3 (B18 Z. 33). PM50 Z. 21 und RGA Z. 559 lesen das auch für das Postfach.
- **Federführung:** BT-Drs. 21/7319 (S. 5) und der Bundesrechnungshof (31.07.2026, nicht veröffentlicht, nicht abgelegt) nennen die FITKO, B-2026/18-IT den Bund. Vermutlich Phasenunterschied **[Folgerung]**.
- **FIT-Connect:** v1.0 empfiehlt FIT-Connect als zentralen Anbindungsweg (ZF10 Z. 232), ADR-0011 verwirft FIT-Connect als Kommunikationsschicht. v1.1 macht FIT-Connect zu Plattform und Betreiber, nicht zum Protokoll.
- **ZBP-Bidirektionalität:** v1.0 hielt den Ausbau für entbehrlich (AK10 Kap. 3.3), die BundID bietet Rückfragen seit Juli 2026 (`UR/bmds_bundid-themenseite_2026-11.md` Z. 31).
- **Deutschland-App:** FIT-AB 2026/12 verortet das Frontend dort; laut Handelsblatt (über heise) soll Wire als Kanal zwischen Bürger:innen und Behörden in die App kommen (`UR/heise_deutschland-app-sap-telekom-wire_2026-04-07.md` Z. 33) **[unverifiziert]**; die App ist ohne Starttermin verschoben.
- **Neo-Daten:** Abschluss am 23.06.2026 (Beschluss) gegenüber „25. Juli 2026“ (VS Z. 79); Zuschlag 03.06.2025 gegenüber „April 2025“.

## 7. Lücken

- Ergebnis der 51. Sitzung des IT-Planungsrats (25.11.2026): v1.1, verbindliche Konsolidierung und Phase III sind noch nicht beschlossen.
- Finanzierungskonzept, Betriebskonzept, konkrete Betriebsstelle beim Bund, Meilensteinplanung der Länder, Governance des D-Stacks.
- Gesetzentwurf zur Harmonisierung von Bekanntgabe und Zustellung (FöMA Maßnahme 207), Verordnung nach § 3c VwVfG n. F., Staatsvertragsentwurf, Anschlussvertrag.
- Unbearbeitete ADRs: API-Autorisierung (ADR-1149), Benachrichtigungsdienst, Löschung und Aufbewahrung, Metadatenminimierung, nicht entschlüsselbare Nachrichten.
- Massenversand, Druck und Papier-Rückfall, SMS, Erinnerung bei Nichtabruf, Mengengerüst, SDK-Sprachen.
- Kein Wirkbetrieb von Neo, keine Nutzungszahlen; Push-Umsetzung in der iOS-App unklar.
- Transitionspläne für BundesMessenger und TI-Messenger; Abstimmungsstand der Pläne mit BLK, Bayern und Bremen, KONSENS.
- Antworten des ZaPuK-Teams auf Konsultationsbeiträge (nur mit openCode-Anmeldung); Positionen von Bitkom, BSI, BfDI; Behörden Spiegel und Tagesspiegel Background.

## 8. Hinweise zur Ablage

- `../zapuk-main/` bleibt unverändert. `AT/` enthält nur Dateien, die upstream neu sind oder sich geändert haben, jeweils mit Kopf (Commit-URL, Branch, Abrufdatum).
- Einige Dokumente aus dem Branch `architecture-v1.1` liegen inhaltlich doppelt in zwei Unterordnern, mit unterschiedlichem Kopf: Architekturkonzept v1.1 und Überblick v1.1 (`AT/`, `RG/`), Rechtsgutachten (`AT/`, `RG/`), Transitionspläne und TADRs zu Mein ELSTER (`AT/`, `UR/`). Die Zeilenangaben in den `_befunde.md` beziehen sich jeweils auf die Kopie im eigenen Ordner.
- Der Bericht des Bundesrechnungshofs vom 31.07.2026 ist laut Deckblatt nicht zur Veröffentlichung bestimmt und nicht abgelegt.
- Belege je Zeile stehen in `../Referenzen-ZaPuK.md`; der Ordner ist in `../../sources.md` eingetragen.
