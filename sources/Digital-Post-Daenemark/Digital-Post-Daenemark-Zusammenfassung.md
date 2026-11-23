# Digital Post (Dänemark): Zusammenfassung für die d-box

Wie Digital Post, die staatliche Postlösung Dänemarks nach dem Gesetz über Digital Post von öffentlichen Absendern, aufgesetzt ist: Betrieb, Anbindung, Recht. Eingeordnet nach den Zeilen von `morph_box.csv`. Die Zusammenfassung verweist auf die Quellen, statt sie zu wiederholen. Für Details den Pointer öffnen.

Stand: 2026-11-23. Grundlage ist `../Referenzen-Digital-Post-Daenemark.md`, ergänzt um Bekendtgørelser, die MeMo-Spezifikation, Leitfäden der Digitaliseringsstyrelsen und die Anlagen der Tilslutningsaftale. Alle Quellen liegen in `quellen/`, jeder Unterordner hat eine `_index.md` mit URL, Herausgeber, Datum und Inhaltsnotiz. Die Quellen sind dänisch, TI und MeMo englisch.

## Kennzeichnung

Wie in `../Referenzen-Digital-Post-Daenemark.md`:

- Ohne Kennzeichnung: Stufe 3, in einer Primärquelle im Volltext belegt.
- **[Indiz]**: Stufe 2, Sekundärquelle, Rückschluss aus einer Primärquelle oder Fehlanzeige nach gezielter Suche.
- **[Annahme]**: Stufe 1, kein Beleg.

## Kürzel für Quellen

Pfade relativ zu `quellen/`. Z. = Zeile der Markdown-Kopie, S. = PDF-Seite. Kürzel wie in `../Referenzen-Digital-Post-Daenemark.md`, neue Kürzel sind mit (neu) markiert.

| Kürzel | Datei | Inhalt |
|---|---|---|
| LBK | `recht-governance/LBK_2021-686_lov-om-digital-post.pdf` | Gesetz über Digital Post, konsolidiert 2021 |
| BEK17 (neu) | `recht-governance/BEK_2021-2017_forvaltning-digital-post.pdf` | Verwaltung der Digital Post, Befreiung von Personen, Papier durch den Absender (§ 13) |
| BEK985 (neu) | `recht-governance/BEK_2013-985_fritagelse-juridiske-enheder.pdf` | Befreiung von Unternehmen, Fjernprint für Weiterleitung (§ 8) |
| BEK19 (neu) | `recht-governance/BEK_2021-2019_dataansvar-offentlige-afsendere.pdf` | Digitaliseringsstyrelsen als Datenverarbeiter, Sicherheitsmaßnahmen |
| BEK331 (neu) | `recht-governance/BEK_2022-331_godkendelse-visningsklienter.pdf` | Zulassung privater Clients |
| BD | `betreiber/borger-dk_digital-post_hjaelp.md` | Hilfe für Personen |
| DP / OM / PF / LEV | `betreiber/digst_digital-post_startseite.md`, `…_om-loesningen.md`, `…_platforme.md`, `…_leverandoerer.md` | Betreiberseiten |
| TAL (neu) | `betreiber/digst_digital-post_tal-og-statistik.md` | Nutzungszahlen |
| VIRK | `betreiber/virk_hjaelp_digital-post.md` | Hilfe für Unternehmen |
| FP | `betreiber/dansk-blindesamfund_fuldmagt-fritagelse-digital-post_2021-11.pdf` | Vollmachtsvordruck, Sekundärquelle |
| TI | `anbindung-technik/digst_digital-post_technical-integration_v1.51_2026-05.pdf` | Kernspezifikation für Absender-, Empfangssysteme und Clients |
| MEMO (neu) | `anbindung-technik/digst_memo-modelbeskrivelse_v1.2_2025-10.pdf` | MeMo-Datenmodell |
| AS / KS / RET | `anbindung-technik/digst_digital-post_afsender-og-modtagersystemer.md`, `…_kontaktstruktur.md`, `…_rettighedsstyring.md` | Behördenanbindung |
| ADM / RETV (neu) | `anbindung-technik/digst_vejledning-administrativ-adgang_2026-09.pdf`, `…_vejledning-rettighedsportalen_v2.0_2026-08.pdf` | Leitfäden Administrativ Adgang und Rettighedsportal |
| HU | `anbindung-technik/digst_digital-post_hjaelpeunivers-send-bedre-digital-post.md` | MeMo-Auszeichnung |
| VK / TA | `anbindung-technik/digst_digital-post_bliv-visningsklient.md`, `…_tilslutningsaftale-visningsklienter.md` und `.pdf` | Zulassung privater Clients |
| UB04A (neu) | `anbindung-technik/digst_tilslutningsaftale-underbilag-04a-teknik-sikkerhed_2025-03.pdf` | Sicherheitsanforderungen an Clients |

## 1. Systembild

- Eine zentrale Postlösung der Digitaliseringsstyrelsen (LBK § 2, S. 1). Behörden senden über **Absendersysteme** und empfangen über **Empfangssysteme** (Modtagersystemer); ohne angeschlossenes Absendersystem kein Versand (AS Z. 10–18; ADM S. 13).
- Nachrichten im XML-Format **MeMo** mit Hauptdokument, bis zu 10 weiteren Dokumenten und Auszeichnungen wie `reply` und `replyByDateTime` (MEMO S. 9; TI Kap. 10.2.3, 13.1).
- Ein Postfach je CPR- oder CVR-Nummer; alle Clients greifen auf dieselben Daten im Hintergrundsystem zu (PF Z. 12): staatlich borger.dk, Virk, Digital Post-App, privat e-Boks und mit.dk (OM Z. 30–32).
- Unternehmen können statt der Clients ein eigenes Empfangssystem anschließen (VIRK Z. 390 ff.; TI Kap. 12.4).
- Schnittstellen: REST über mTLS mit OCES-Zertifikaten (TI S. 21); Zertifikate und TLS-Suites in TI-Anhang.
- Historie: Neue Lösung („NgDP“) seit 2022 nach Hjemtagelse von Datenverantwortung und Entscheidungskompetenz zur Digitaliseringsstyrelsen (LBK S. 1, Vorbemerkung zu Lov nr. 1941/2020; `betreiber/digst_digital-post_baggrund.md`). Umzug der Post beim Lieferantenwechsel nach BEK 2010/2020.

## 2. Betrieb, Governance, Teilnahme

- Betreiber Digitaliseringsstyrelsen; Netcompany (Entwicklung, Betrieb, Wartung), Fellowmind (Entwicklung), Statens IT (Teile des Betriebs) (LEV Z. 14–16).
- Datenschutz: Behörden sind für Inhalte verantwortlich, Digitaliseringsstyrelsen Datenverarbeiter (LBK § 2 a, S. 1; BEK19). Jährliche Aufsicht mit Revisionserklärungen (`recht-governance/digst_digital-post_tilsyn.md`).
- Private Clients nur nach Zulassung und Tilslutningsaftale (LBK § 10 b; BEK331; VK; TA). Clients dürfen keine Daten der Lösung dauerhaft speichern (UB04A K-10, S. 6).
- Nutzung: 253.774.703 Nachrichten 2025; 2. Halbjahr 2026 95,6 % der Personen angeschlossen, 4,4 % befreit (TAL Z. 28, 33).

## 3. Identität und Adressierung

- Adressierung an CPR- bzw. CVR-Nummer, die Behörde pflegt kein Empfängerverzeichnis (OM Z. 21).
- Absendersystem kann vorab abfragen, ob Empfänger befreit sind oder NemSMS nutzen (TI Kap. 12.4.2, S. 241).
- Personen: MitID (BD). Unternehmen: MitID Erhverv, Zugang über Rechteadministrator:in im Rettighedsportal (VIRK Z. 40–48).
- Behörden: Kontaktstruktur als Adressbuch mit Kontaktpunkten, jeder Kontaktpunkt hängt an einem Empfangssystem (KS Z. 12–16).

## 4. Postfach und Konto

- Ein Postfach je CPR- oder CVR-Nummer, Zugang über mehrere Clients (OM; PF). Bei Tod gesperrt, Aufbewahrung 5 Jahre (BD).
- Lese- bzw. Lese- und Schreibzugang für Dritte, ersetzt keine Vollmacht (BD Z. 133–142).
- Für Unternehmen und Behörden Rechte im Rettighedsportal, u. a. „Skrive, besvare og videresende post“ (RETV S. 8; RET Z. 14).

## 5. Einordnung nach `morph_box.csv`

### Behörde

| Zeile | Digital Post |
|---|---|
| B1 Anbindung von Fachverfahren | Absender- und Empfangssysteme an der zentralen REST-Schnittstelle, Format MeMo; Anschluss in Administrativ Adgang (AS; ADM S. 13; TI Kap. 12.4). |
| B2 Zugang ohne Fachverfahren | Versand setzt ein Absendersystem voraus (ADM S. 13). Papier für befreite Empfänger liegt beim Absender (BEK17 § 13; BEK985 § 9); Digital Post weist nicht obligatorische Post an Befreite ab (TI S. 256). Fjernprint ist primär nur für die Weiterleitung ungeöffneter Post belegt (BEK985 § 8, S. 2); „Papier aus demselben Versand über Distributionsdienstleister“ bleibt **[Indiz]** (FP). |
| B3 Adressierung der Person / Organisation | CPR- oder CVR-Nummer, kein eigenes Verzeichnis (OM Z. 21). |
| B4 Inhalt an die Person / Organisation | MeMo mit Hauptdokument und bis zu 10 weiteren Dokumenten, Format-Whitelist (TI S. 245); Antwortmöglichkeit, Frist, Links zu Selbstbedienung (MEMO S. 9; HU Z. 10; OM Z. 22). |

### Person / Organisation

| Zeile | Digital Post |
|---|---|
| P1 Benachrichtigung | E-Mail, SMS, Push; SMS zwischen 10 und 18 Uhr (BD Z. 229–252). |
| P2 Authentifizierung | MitID, MitID Erhverv (BD; VIRK Z. 40). |
| P3 Organisationszugang | Rettighedsportal mit Rechteadministrator:innen (RET Z. 14; VIRK Z. 46–48; RETV). |
| P4 Vertretung | Lese- und Schreibzugang; Insolvenz- und Nachlassverwaltung (BD Z. 133–142). |
| P5 Clients | borger.dk, Virk, Digital Post-App, e-Boks, mit.dk (OM Z. 30–32; PF Z. 16–18). |
| P6 Anbindung eigener Systeme | Modtagersystem für Unternehmen (VIRK Z. 390 ff.; TI Kap. 12.4.3). |
| P7 Antwort und Erstkontakt | `reply` / ReplyData (MEMO S. 9, 29); anlasslos an Kontaktpunkte (KS). |
| P8 Inhalt an die Behörde | Freitext und Anhänge, je Anhang 10 MB (TI S. 114, 127). |
| P9 Postfach und Nutzerkonto | ein Postfach je CPR/CVR (OM; BD). |

### Übermittlung

| Zeile | Digital Post |
|---|---|
| Ü1 Modus | asynchron (LBK § 7; TI). |
| Ü2 Vertraulichkeit | Transport über mTLS (TI S. 21); Verschlüsselung als Sicherheitsmaßnahme genannt (BEK19 § 6, S. 3); borger.dk: Post „kann nie von anderen gelesen werden als dir und der Behörde“ (BD Z. 219). Gleichzeitig validiert die Plattform HTML, prüft Anhänge auf Viren (TI S. 193, 201) und stellt Inhalte über eine API allen Clients bereit (PF Z. 12); also keine Ende-zu-Ende-Verschlüsselung **[Indiz]**. |
| Ü3 Echtheit | gesetzliche Absendervermutung (LBK § 10). |
| Ü4 Ereignisse und Bestätigungen | Business Receipt je MeMo, sobald validiert und bereit zur Zustellung in Postfach oder Empfangssystem (TI S. 189); Ereignisprotokoll, u. a. erstes Öffnen einer forkyndelse (TI S. 136). |
| Ü5 Interoperabilität mit Systemen außerhalb | keine **[Indiz]**: Fehlanzeige zu eDelivery, Peppol, SDG in allen lokalen Quellen. |
| Ü6 Malware-Prüfung | Virenprüfung (`virus.detected`, `memo.infected`) (TI S. 193, 201, 256); Format-Whitelist (TI S. 245). |

### Infrastruktur und Betrieb

| Zeile | Digital Post |
|---|---|
| I1 Betrieb und Governance | Digitaliseringsstyrelsen mit Lieferanten (LBK § 2; LEV). |
| I2 Datenhaltung und Versand | zentral, zentrales Archiv (PF Z. 12; `anbindung-technik/digst_tilslutningsaftale-bilag-04-…pdf` S. 6). |
| I3 Betriebsmodell für Organisationen | eigene Absender- und Empfangssysteme, kein eigenes Postfach-Backend (AS). |
| I4 Funktionspostfächer und interne Bearbeitung | Kontaktpunkte, Empfangssystem, Rettighedsportal (KS; RET; RETV); keine E2EE **[Indiz]** wie Ü2. |
| I5 Aufnahme in den Verbund | private Clients nur nach Zulassung (LBK § 10 b; BEK331; TA). |

### Recht

| Zeile | Digital Post |
|---|---|
| R1 Schriftformersatz | Kommunikation mit Rechtswirkung in beide Richtungen (LBK §§ 7, 8, 10). |
| R2 Teilnahmepflicht | Pflicht ab 15 und für CVR-Einheiten, Befreiung über die Kommune (LBK §§ 3–5; BEK17; BEK985; BD Z. 22). |
| R3 Bekanntgabe und förmliche Zustellung | Zugang mit Abrufbarkeit (LBK § 10 Nr. 1); forenklet digital forkyndelse (BD; TI S. 136). |
| R4 Teilnahme der Behörden | Empfang und Antwort Pflicht, Versand Kann (OM Z. 16; LBK § 7). |

## 6. Widersprüche und offene Punkte

- **Verschlüsselung:** borger.dk verspricht, dass nur Person und Behörde die Post lesen können (BD Z. 219–223). Die TI beschreibt aber serverseitige Validierung und Virenprüfung der Inhalte und Bereitstellung über APIs an mehrere Clients. Das spricht gegen Ende-zu-Ende-Verschlüsselung; ein ausdrücklicher Beleg fehlt.
- **Fjernprint:** FP und die frühere Tabelle beschreiben Papier aus demselben Versand. Nach BEK17 § 13 und BEK985 § 9 muss der Absender selbst für Papier sorgen, und Digital Post weist nicht obligatorische Post an Befreite ab (TI S. 256). Fjernprint ist primär nur für die Weiterleitung ungeöffneter Post an Unternehmen belegt (BEK985 § 8).
- **Rechtsstand:** LBK 686/2021 ist laut retsinformation.dk gültig; die Vorbemerkung nennt Teile mit Inkrafttreten auf Anordnung des Finanzministers (LBK S. 1). Die Inkraftsetzung ist nicht separat nachgewiesen.

## 7. Lücken

- Nicht gefunden: öffentliche Sicherheitsdokumentation zur Speicherung der Inhalte (at rest), Aussage zur grenzüberschreitenden Zustellung, Sagsbehandlingsvejledning zu Fjernprint.
- Nicht abgerufen: weitere Anlagen der Tilslutningsaftale, MeMo-Nebendokumente, „Events In Event-log.xlsx“, Videos (media.videotool.dk gesperrt), Gesetzesmaterialien.
- Alle Hauptquellen der Referenzen-Datei waren abrufbar.

## 8. Verwandte lokale Quellen

- `../Referenzen-Digital-Post-Daenemark.md`: Nachweise je Dimension und Evidenzstufen.
- `../MUK/MUK-Zusammenfassung.md`: gleiche Struktur für das deutsche Organisationskonto.
