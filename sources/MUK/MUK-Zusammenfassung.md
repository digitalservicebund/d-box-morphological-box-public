# MUK (Mein Unternehmenskonto): Zusammenfassung für die d-box

Wie Mein Unternehmenskonto (MUK), das einheitliche Organisationskonto nach § 2 Abs. 5 und § 3 Abs. 2 OZG, mit seinen beiden Postfächern aufgesetzt ist: Betrieb, Anbindung, Recht. Eingeordnet nach den Zeilen von `morph_box.csv`. Die Zusammenfassung verweist auf die Quellen, statt sie zu wiederholen. Für Details den Pointer öffnen.

Stand: 2026-11-18. Grundlage ist eine Webrecherche, ausgehend von <https://info.mein-unternehmenskonto.de/>. Alle Primärquellen liegen in `quellen/`, jeder Unterordner hat eine `_index.md` mit URL, Herausgeber, Datum und Inhaltsnotiz. Quellen, die schon in `../BundID-ZBP/quellen/` oder `../ELSTER/` liegen, sind dort nicht doppelt abgelegt.

## Kennzeichnung

- Ohne Kennzeichnung: in einer Primärquelle belegt.
- **[Folgerung]**: aus mehreren Quellen geschlossen, nicht ausdrücklich belegt.
- **[unverifiziert]**: nur Sekundärquelle oder Annahme.
- **[veraltet?]**: Quelle ist älter, aktueller Stand nicht belegt.

## Kürzel für Quellen

Pfade relativ zu `quellen/`. Z. = Zeile der Markdown-Kopie, S. = gedruckte Seite, PDF-S. = Seite der PDF-Datei. Vollständige Liste mit URLs: `../Referenzen-MUK.md`.

| Kürzel | Datei | Inhalt |
|---|---|---|
| FAQ | `muk-betreiber/stmd_muk-info_hilfe-faq.md` | FAQ der Infoseite (StMD): Konto, Zertifikate, Postfach 2.0, OZG-PLUS-Postfach, Vertretung, Bekanntgabe |
| RECHT | `muk-betreiber/stmd_muk-info_rechtliche-hintergrundinformationen.md` | Vertrauensniveau „substanziell“, keine eIDAS-Notifizierung geplant |
| PF / FUNK | `muk-betreiber/stmd_muk-info_postfachfunktionen.md`, `…_funktionalitaeten-mein-uk.md` | Kontomodell: Steuernummer, Benutzerkonten, je Konto ein Postfach 2.0 |
| HL | `muk-betreiber/stmd_muk_postfach20-funktionale-highlights-v4_2026-04.pdf` | Postfach 2.0, Funktionen 2026: Limits, Status, Aufbewahrung, Virenprüfung |
| PF23 | `muk-betreiber/stmd_muk_postfach20-bedienung-featureuebersicht-v1_2023-11.pdf` | Postfach 2.0, Bedienung 2023 **[veraltet?]** |
| BSQS | `muk-betreiber/dataport_muk_quickstart-berechtigungssteuerung-organisationen-v1.4_2025-05.pdf` | Berechtigungssteuerung (Admins, Gruppen, Rechte) |
| ZLF | `muk-betreiber/baylfst-stmd_muk_zertifikatsleitfaden-v3.0_2026-11.pdf` | Zertifikatsleitfaden 11/2026: Weitergabe, Erneuerung, Innenvollmacht |
| DSE / IMP | `muk-betreiber/baylfst_muk-portal_datenschutz.md`, `…_impressum.md` | Verantwortlicher BayLfSt, Server in Deutschland |
| OZGPP-UI / -CFG / -DS | `muk-betreiber/governikus_ozgpp_*.md` | OZG-PLUS-Postfach: UI-Texte, Web-Client-Konfiguration, Datenschutz |
| AL25 | `muk-betreiber/itplr-al-runde_unternehmenskonto-bericht-top06_2025-04-29.pdf` | Bericht an die IT-PLR-AL-Runde 04/2025 mit Nutzungszahlen (auch `recht-governance/IT-PLR_Information2025-06-AL_Unternehmenskonto_Bericht.pdf`) |
| IL | `anbindung-technik/muk_postfach-2.0_integrationsleitfaden_v9_2026-04.pdf` | Integrationsleitfaden Postfach 2.0, v9 04/2026: Kernquelle für die Behördenseite |
| BPZ | `anbindung-technik/muk_beipackzettel_integration-ozgpp-und-postfach-2.0_v1_2026-03.pdf` | Gemeinsame Einlieferung in Postfach 2.0 und OZG-PLUS-Postfach (seit 03/2026) |
| NEZO | `anbindung-technik/muk_nezo-integrationsleitfaden_v366_2026-09.pdf` | Login-Anbindung (SAML), Pseudonym `ek-…` als Postfachhandle, Funktionspostfächer-Attribut |
| SSP | `anbindung-technik/muk_ssp-quick-start-guide_v1.3_2024-12.pdf` | Self-Service-Portal: Vorhaben, Nutzungsarten, Postfachrechte |
| ETR-V | `anbindung-technik/elster_verwaltung_elster-transfer.md` | ELSTER-Transfer für Behörden |
| OZGPP-IL / -API / -LU | `anbindung-technik/governikus_ozgpp_integrationsleitfaden_v1.11_2026-04.pdf`, `openapi_ozgpp-*.yaml` | OZG-PLUS-Postfach: Integration, REST-API, Lookup der Funktionspostfächer |
| GOV-FPF | `anbindung-technik/governikus_mein-unternehmenskonto_funktionspostfaecher.md` | Governikus-FAQ zu Funktionspostfächern, E2EE, Virenprüfung |
| VV | `recht-governance/IT-PLR_Beschluss2022-37_Verwaltungsvereinbarung_Organisationskonto.pdf` | Verwaltungsvereinbarung Organisationskonto (ab 01.01.2023) |
| ORGV | `recht-governance/OZG3Abs2V_Organisationskonto-Verordnung_gesetze-im-internet_2026-11-18.md` | Verordnung nach § 3 Abs. 2 OZG: Aufgabe an Bayern und Bremen |
| PLR25 | `recht-governance/IT-PLR_Beschluss_2025-29_Unternehmenskonto_Projektsteuerung.md` | Projektsteuerung, einheitliches Produkt bis Ende Q1 2027 |
| OZG | `../BundID-ZBP/quellen/recht-governance/OZG_gesetze-im-internet_2026-11-17.md` | OZG, aktuelle Fassung |
| D8093 | `../BundID-ZBP/quellen/recht-governance/BT-Drs_20-8093_Gesetzentwurf_OZG-Aenderungsgesetz.pdf` | Begründung OZGÄndG: Verbindlichkeit des Organisationskontos |

## 1. Systembild

- MUK ist das bundesweit einheitliche Organisationskonto für Unternehmen, Vereine, Behörden und sonstige Organisationen (ORGV § 1; § 2 Abs. 5 OZG). Es besteht aus sechs Bausteinen (VV S. 6):
  1. MUK-Frontend (Portal `mein-unternehmenskonto.de`),
  2. NEZO: Nutzerkonto-Login über ELSTER-Organisationszertifikate,
  3. NEZOP: Datenübermittlung an Onlinedienste,
  4. **Postfach 2.0** (ELSTER-Infrastruktur, BayLfSt),
  5. **OZG-PLUS-Postfach** (Governikus, Bremen),
  6. Autorisierungsmodul bzw. Berechtigungssteuerung (Dataport, Bremen).
- **Zwei Postfächer mit unterschiedlicher Architektur:**
  - *Postfach 2.0*: persönliches Postfach je Benutzerkonto (= je Organisationszertifikat), zentral auf ELSTER-Servern, Virenprüfung zentral, keine Ende-zu-Ende-Verschlüsselung **[Folgerung]**. Auch in Mein ELSTER sichtbar (FAQ Z. 317).
  - *OZG-PLUS-Postfach*: Funktionspostfächer (bis 500 je Unternehmen, OZGPP-CFG Z. 52) mit Ende-zu-Ende-Verschlüsselung per Proxy Re-Encryption, Schlüssel im Browser (GOV-FPF; OZGPP-IL PDF-S. 15, 39, 45). Kommunikation von beiden Seiten möglich (FAQ Z. 358 ff.).
- Seit März 2026 eine gemeinsame Einlieferung für beide Postfächer über ELSTER-Transfer (Datenart `OZGPPNachricht`); ein vereinheitlichtes Produkt ist bis Ende Q1 2027 geplant (BPZ S. 7–9; PLR25).
- Behörden senden über **ELSTER-Transfer** (lokale Anwendung mit Weboberfläche oder REST-API) oder **ERiC**; Unternehmen holen ab (Pull) (IL S. 9–17).
- Historie: IT-PLR-Gründungsbeschluss 2020/01 (14.02.2020), Rechtsgrundlage § 2 Abs. 5 OZG seit dem Gesetz zur Digitalisierung von Familienleistungen 2020, Verordnung 22.09.2021, Verwaltungsvereinbarung ab 01.01.2023, OZGÄndG 2024 mit Pflicht zur Verwendung für Behörden (`recht-governance/_index.md`; D8093 S. 2, 39). Bidirektionale Kommunikation (Antwortfunktion) seit Juli 2023 (`muk-betreiber/stmd_muk-info_2023-07-28_bidirektionale-kommunikation.md`).

## 2. Betrieb, Governance, Teilnahme

- Aufgabe gemeinsam an Bayern und Bremen übertragen (ORGV § 1). Bayern betreibt Bausteine 1–4, Bremen 5–6 (VV S. 6).
- Bayern: zuständig ist das StMD, Betrieb und datenschutzrechtliche Verantwortung beim BayLfSt (DSE Z. 54; `recht-governance/BayMBl_2022-237_…pdf` Nr. 1). Infoseite: StMD, Betrieb LDBV/IT-DLZ (`muk-betreiber/stmd_muk-info_impressum.md` Z. 24). Server in Deutschland (DSE Z. 63).
- Bremen: OZG-PLUS-Postfach vom Senator für Finanzen, betrieben von Governikus, „deutsches Rechenzentrum“ (`muk-betreiber/senfin-bremen_ozgpp_impressum.md`; OZGPP-DS Z. 81). Berechtigungssteuerung bei Dataport (BSQS).
- Datenschutz: Betreiberländer „gemeinsam verantwortlich“ (VV § 21), § 8 Abs. 10 OZG spricht von „ausschließlich verantwortlich“ der zuständigen Stelle (vereinbar über S. 2, formale Art.-26-Vereinbarung nicht gefunden).
- Steuerung: FITKO-Produktmanagement ab 2023/2024 (`recht-governance/IT-PLR_Beschluss_2022-37_…md`); 2025 ohne Produktboard, seitdem Projektsteuerung mit Lenkungskreis (BY, HB, Bund, HH, TH stimmberechtigt) (PLR25; `recht-governance/IT-PLR_Beschluss_2025-02-AL_…md`).
- Finanzierung: seit 2023 über IT-PLR-Mittel, für nutzende Behörden „unentgeltlich“ (VV S. 14, § 19). 2025–2027 aus dem Produktbudget (PLR25). Finanzaudit läuft (`recht-governance/IT-PLR_B-2026-08-IT_…md`). Kostenschätzungen 2020: Bausteine 1–4 Entwicklung 6,6 Mio. €, Pflege 3,0 Mio. € p. a., Betrieb 0,8 Mio. € p. a. **[veraltet?]** (`recht-governance/IT-PLR_Beschluss2020-01_Basiskonzept_…pdf` S. 15–16).
- Nutzung (Stand April 2025, AL25 S. 4, 6–7): ca. 685.668 Organisationen mit 2.669.664 Organisationszertifikaten, ca. 1 Mio. Logins/Monat, 899 NEZO-Anbindungspartner; Postfach: 356 Anbindungspartner, über 160.000 Konten mit Nachrichten, über 1,4 Mio. Nachrichten. Neuere Zahlen nicht gefunden.

## 3. Identität und Adressierung

- Login nur mit ELSTER-Organisationszertifikat (Softwaredatei plus Passwort), ausgestellt auf eine Steuernummer; kein Personalausweis (FAQ Z. 201; `muk-betreiber/stmd_muk_leitfaden-registrierung-schritt-fuer-schritt_2025-06.pdf` S. 20–24). Vertrauensniveau „substanziell“ (§ 87a Abs. 6 AO, § 3 Abs. 4 Nr. 2 OZG); keine eIDAS-Notifizierung geplant (RECHT Z. 24). Persönliches ELSTER-Zertifikat nur, wenn der Onlinedienst es zulässt (FAQ Z. 135, 209).
- eID nur indirekt: Eine „Handelnde Person“ kann über ihr persönliches ELSTER-Konto (Online-Ausweis) mit Namen und Geburtsdatum verknüpft werden (FAQ ca. Z. 177–195).
- Je Steuernummer viele gleichrangige Benutzerkonten (aktuell bis 17.000 Zertifikate, FAQ Z. 78), je Benutzerkonto genau ein Postfach 2.0 (PF Z. 14).
- **Adresse im Postfach 2.0** ist die `accountId`: entweder das pseudonyme SAML-NameID `ek-` + 40 Hex-Zeichen aus dem NEZO-Login (eindeutig je Benutzer und Service Provider), das „gleichzeitig als Postfachhandle“ dient, oder die 10-stellige ELSTER-AccountID (IL S. 19; NEZO PDF-S. 23, 52). Die Kennung gilt je Zertifikat, nicht je Organisation (IL S. 19).
- Ohne vorherigen Login nur, wenn die Behörde den Handle anders kennt, etwa aus einem Grundlagenbescheid des Finanzamts (IL S. 9). Kein öffentliches Verzeichnis **[Folgerung]**. Weitere Merkmale wie der AGS sind „geplant“.
- **Adresse im OZG-PLUS-Postfach**: Funktionspostfach-ID (UUID) aus dem SAML-Attribut `Funktionspostfaecher` oder per Lookup-API über das Bausteinpseudonym (NEZO PDF-S. 31, 66; OZGPP-LU).
- FIT-Connect kennt im Rückkanal-Schema nur `elster.accountId/lieferTicket/geschaeftszeichen` (`../BundID-ZBP/quellen/fitko-fit-connect/schema_fit-connect_dataset_reply-channel-1.0.0.schema.json`); eine Zustellung über FIT-Connect ins MUK ist nicht belegt.

## 4. Postfach und Konto

- Postfach 2.0 ist an das Benutzerkonto, also an das Zertifikat gebunden. Wird das Zertifikat gelöscht, sind Benutzerkonto, Postfach 2.0 und OZG-PLUS-Postfach-Inhalte weg (FAQ Z. 237). Eine Zertifikatserneuerung behält das Postfach; das Konto kann einer anderen Person übergeben werden (ZLF S. 10).
- Inhalte eines Postfachs sind für andere Benutzerkonten derselben Organisation nicht sichtbar (FAQ Z. 292). Gemeinsamer Zugriff läuft praktisch über Zertifikatskopien (ZLF S. 7; `muk-betreiber/stmd_muk-info_zertifikat-interner-prozess.md`) **[Folgerung]**.
- Aufbewahrung Postfach 2.0 (2026): Bescheid 4 Jahre, Mitteilung und Kurzmitteilung 180 Tage, unabhängig vom Abruf (HL S. 10). Kein Archiv; Download möglich. Keine Inaktivitäts-Löschfrist für Konten gefunden.
- OZG-PLUS-Postfach: Löschen erst nach Lesen und 3 Tagen; ohne Schlüsseldatei und Wiederherstellungssatz ist der Zugriff verloren (OZGPP-UI Z. 412, 533).
- Nur Web: Portal ohne Installation (`muk-betreiber/baylfst_muk-portal_startseite.md` Z. 35), OZG-PLUS-Postfach als Webanwendung, Postfach 2.0 auch in Mein ELSTER. Keine App gefunden.

## 5. Einordnung nach `morph_box.csv`

### Behörde

| Zeile | MUK |
|---|---|
| B1 Anbindung von Fachverfahren | ELSTER-Transfer (ETR) als lokal betriebene Anwendung: REST-API (erst Anhänge hochladen, dann Bereitstellungsauftrag) oder Weboberfläche; alternativ ERiC. Voraussetzungen: SSP-Vorhaben, ELSTER-Organisationszertifikat mit Postfachrechten, ETR. Erreichbar über Internet oder NdB-VN (IL S. 9–17, 30). Ein Zertifikat eines IT-Dienstleisters reicht für mehrere Behörden (IL S. 10). Seit 03/2026 auch Datenart `OZGPPNachricht` über ETR für das OZG-PLUS-Postfach (BPZ S. 7–9); alternativ direkte REST-API von `ozgpp.de` (OZGPP-IL PDF-S. 16). |
| B2 Zugang ohne Fachverfahren | Weboberfläche der lokal installierten ETR-Anwendung für Behörden mit wenigen Bescheiden (IL S. 10–11). Mein ELSTER allein reicht nicht (ETR-V). |
| B3 Adressierung der Person / Organisation | Pseudonymer Postfachhandle `ek-…` aus dem NEZO-Login oder ELSTER-AccountID, je Benutzerkonto; Funktionspostfach-ID für das OZG-PLUS-Postfach; ohne Login nur mit anderweitig bekanntem Handle; kein Verzeichnis. Unteradressierung über `empfaengerreferenz`. |
| B4 Inhalt an die Person / Organisation | Datenarten `EPBescheid`, `EPMitteilung`, `EPKurzmitteilung` (nur HTML), `Gewerbesteuerbescheid` (PDF/A-3 mit XML). Betreff, `htmlAnschreiben` bis 5000 Zeichen mit wenigen erlaubten Tags, Antwortlinks. Anhänge PDF, JPG, PNG, CSV, XML; bis 100 je Nachricht, 2 GiB je Anhang, 5 GiB je Nachricht (IL S. 19–24). 2023: 50 MiB / 250 MiB **[veraltet]** (PF23 S. 3). OZG-PLUS: XHE-Payloads (OZGPP-IL PDF-S. 16, 23). |

### Person / Organisation

| Zeile | MUK |
|---|---|
| P1 Benachrichtigung | E-Mail an die bei ELSTER hinterlegte oder im Antrag angegebene Adresse, fester Text mit optionalem Zusatztext (FAQ Z. 298, 743; PF23 S. 12–13). Ob Inhalte in der E-Mail stehen: **[unverifiziert]**, wohl nicht. OZG-PLUS: E-Mail an Mitglieder eines Funktionspostfachs. |
| P2 Authentifizierung | ELSTER-Organisationszertifikat (Softwaredatei + Passwort), substanziell; persönliches ELSTER-Zertifikat nur, wenn der Dienst es zulässt; kein Online-Ausweis; ausländische eIDs in Arbeit (FAQ Z. 103, 135, 201; RECHT Z. 24). |
| P3 Organisationszugang | Viele gleichrangige Benutzerkonten je Steuernummer, je Konto ein Zertifikat und ein Postfach 2.0. Berechtigungssteuerung mit Admins, Gruppen, Funktionsrollen (BSQS S. 8–15). Gemeinsame Postfächer nur im OZG-PLUS-Postfach (Funktionspostfächer); im Postfach 2.0 über Zertifikatskopien **[Folgerung]**. |
| P4 Vertretung | Intern: Zertifikatsweitergabe als „konkludente Innenvollmacht“ (ZLF S. 6); Vertretung im Postfach 2.0 „ab Sommer 2026“ angekündigt, Umsetzung **[unverifiziert]** (FAQ Z. 145, 329); Vertreterregelungen im OZG-PLUS-Postfach. Gesetzliche Vertreter und Prokura „aktuell nicht abgebildet“ (FAQ Z. 623 ff.). Externe Vertretung (Steuerberatung, Dienstleister) nicht belegt. |
| P5 Clients | Web: Portal Mein Unternehmenskonto, Postfach 2.0 auch in Mein ELSTER, OZG-PLUS-Postfach als Web-App. Keine App **[unverifiziert]**. |
| P6 Anbindung eigener Systeme | M2M-Abholung „in die Infrastrukturen von Unternehmen“ bzw. ins ERP erwähnt (FAQ Z. 776; AL25 S. 4), Abholung per ERiC (IL S. 6, 9). Dokumentation für Unternehmen nicht gefunden. |
| P7 Antwort und Erstkontakt | Postfach 2.0: Antwort nur, wenn die Behörde sie freischaltet (`antwort`, optional `antwortFrist`, vorgegebene Betreffe), seit 07/2023 mit mehreren Anhängen; Antwort wird als PDF erzeugt (IL S. 23–24; HL S. 5). Kein Erstkontakt **[Folgerung]**. OZG-PLUS: Kommunikation von beiden Seiten laut FAQ, Rückantwort aber über vom Fachverfahren vorgegebenen Kanal (OZG-PLUS, E-Mail oder URL) (FAQ Z. 358 ff.; OZGPP-IL PDF-S. 44). |
| P8 Inhalt an die Behörde | Freitext plus Anhänge (PDF, CSV, TXT, PNG, JPG, XML; bis 100 Dateien, HL S. 7); Antwort-PDF. OZG-PLUS: Upload bis 200 MB (OZGPP-CFG); FAQ nennt 600 MB je Anhang. |
| P9 Postfach und Nutzerkonto | Postfach Bestandteil des Organisationskontos (§ 2 Abs. 7 OZG), Postfach 2.0 hängt am Benutzerkonto bzw. Zertifikat; Löschung des Zertifikats löscht das Postfach. OZG-PLUS-Postfach als separater Baustein mit eigener Schlüsselverwaltung. |

### Übermittlung

| Zeile | MUK |
|---|---|
| Ü1 Modus | Asynchron. Behörde liefert über ETR ein, Unternehmen holt ab (Web oder M2M). ETR fragt Status bis zu 24 h ab; Durchlauf meist 5–10 min (IL S. 25, 40). Kein Push an die Behörde **[Folgerung]**. |
| Ü2 Vertraulichkeit | Postfach 2.0: ETR verschlüsselt den Transport zur ELSTER-Annahme, Inhalte zentral beim Betreiber lesbar **[Folgerung]**; Aussage nur „umfassende Sicherheitsmechanismen“ (HL S. 2). OZG-PLUS: Ende-zu-Ende vom Fachverfahren bis zum Mitglied des Funktionspostfachs (Proxy Re-Encryption, AES-256-GCM, ECIES, secp521r1); Metadaten unverschlüsselt (OZGPP-IL PDF-S. 15, 39, 45; OZGPP-DS Z. 79). |
| Ü3 Echtheit | Absendende Behörde (bzw. Dienstleister) authentisiert sich mit ELSTER-Organisationszertifikat mit Postfachrechten; Absendername ist Freitext (`absender`), Bindung an die Zertifikatsidentität nicht belegt (IL S. 10, 12). Siegel oder Signatur nicht vorgesehen **[unverifiziert]**. OZG-PLUS: API-Key je Fachverfahren. |
| Ü4 Ereignisse und Bestätigungen | ETR-Status: EINGESTELLT, DATEN_FEHLERHAFT, BEREIT_ZUR_ABHOLUNG, VOM_EMPFAENGER_ABGEHOLT (mit Abrufzeitpunkt), EINGANG_BESTAETIGT, GELOESCHT (IL S. 26–28). Beworben als „Bekanntgabe- und Lesenachweis“ für die Behörde (IL S. 6; HL S. 9); ein eigener Lesestatus ist in der Schnittstelle nicht dokumentiert. OZG-PLUS: analoge Status mit `emailVersandZeitpunkt` und `abrufZeitpunkt` (OZGPP-IL PDF-S. 19). |
| Ü5 Interoperabilität mit Systemen außerhalb | Keine. Postfach 2.0 und OZG-PLUS-Postfach werden zusammengeführt (BPZ). Keine Verbindung zu BundID/ZBP oder EU-Systemen belegt; keine eIDAS-Notifizierung geplant (RECHT Z. 24). |
| Ü6 Malware-Prüfung | Postfach 2.0: zentral, „Integrierte Virenprüfung aller Anhänge“ bei Annahme (IL S. 9; HL S. 3). OZG-PLUS: keine zentrale Prüfung, Empfänger selbst verantwortlich (FAQ Z. 802; GOV-FPF). |

### Infrastruktur und Betrieb

| Zeile | MUK |
|---|---|
| I1 Betrieb und Governance | Länderlösung im Auftrag des IT-PLR: Bayern (StMD, BayLfSt) für Bausteine 1–4, Bremen (SF, Governikus, Dataport) für 5–6; Projektsteuerung mit Lenkungskreis, FITKO ohne Stimmrecht (ORGV § 1; VV S. 6; PLR25). |
| I2 Datenhaltung und Versand | Postfach 2.0 zentral auf ELSTER-Servern in Deutschland (BayLfSt) (DSE Z. 63); OZG-PLUS-Postfach im deutschen Rechenzentrum von Governikus (OZGPP-DS Z. 81). Versand über das lokal betriebene ETR der Behörde oder eines IT-Dienstleisters (IL S. 14). |
| I3 Betriebsmodell für Organisationen | Behörden als Absender ohne eigenes Postfach-Backend, über eigene oder beim IT-DLZ betriebene ETR-Instanz. Unternehmen nutzen zentrale Postfächer. Behörden können MUK auch als Nutzer verwenden (`anbindung-technik/muk-info_behoerden-als-nutzer.md`). |
| I4 Funktionspostfächer und interne Bearbeitung | Unternehmen: Funktionspostfächer nur im OZG-PLUS-Postfach, E2EE endet beim einzelnen Mitglied. Postfach 2.0 ohne Funktionspostfächer. Behörden: Antworten holt die versendende ETR-Instanz ab **[Folgerung]**; interne Verteilung nicht dokumentiert. |
| I5 Aufnahme in den Verbund | Nutzungsberechtigt: Behörden aller Ebenen und Gerichte (VV § 2). Antrag im SSP mit Organisationszertifikat; Nutzungsarten „Nur NEZO“, „Nur Postfach 2.0“, „NEZO & Postfach 2.0“, „… & OZG-PLUS-Postfach“; Prüfung durch das MUK-Team; Postfachrechte vergibt der Vorhabensverantwortliche (IL S. 12; SSP S. 6–7). Dienstleister mit Vollmacht. Ausschluss bei wesentlichen Verstößen nach Hinweis (VV § 24). Testumgebung E4K, ETR-Sandbox, OZGPP-Staging. |

### Recht

| Zeile | MUK |
|---|---|
| R1 Schriftformersatz | Behörde an Organisation: Dokument im Postfach mit qualifiziertem elektronischem Siegel ersetzt die Schriftform (§ 9a Abs. 6 OZG); Siegel im Postfach 2.0 nicht dokumentiert. Organisation an Behörde: Online-Formular nach Identifizierung über das Organisationskonto (§ 9a Abs. 5 OZG), ELSTER „substantiell“ genügt (§ 3 Abs. 4 Nr. 2 OZG). |
| R2 Teilnahmepflicht | Für Unternehmen keine Kontopflicht; Fachrecht kann sie vorsehen (`recht-governance/BT-Drs_19-23774_…pdf` S. 17). Bundesleistungen für Unternehmen sollen bis 31.12.2029 ausschließlich elektronisch angeboten werden (§ 1a Abs. 1 OZG) **[Folgerung zum Datum]**. Einwilligung in die Bekanntgabe gilt als erteilt, Opt-out möglich (§ 9 Abs. 1 S. 2 OZG). |
| R3 Bekanntgabe und förmliche Zustellung | Bekanntgabe durch Bereitstellung zum Abruf, Fiktion am vierten Tag (§ 9 Abs. 1 OZG); Steuern: § 122a AO, ohne Einwilligung, ebenfalls vierter Tag (`recht-governance/AO_122a_…md`). IL S. 37 nennt noch den dritten Tag **[veraltet]**. Förmliche Zustellung bisher nicht; JStG 2026 (§ 309a AO-E) soll Zustellung von Pfändungen an Kreditinstitute über ein Organisationskonto-Postfach erlauben, Bundesrat offen (`recht-governance/BT-Drs_21-8283_…pdf` S. 31, 118). |
| R4 Teilnahme der Behörden | Pflicht: Für öffentliche Stellen ist die „Verwendung des Organisationskontos verpflichtend“ (§ 3 Abs. 3 OZG), seit 24.07.2024, umfasst laut Begründung alle Bausteine nach Verfügbarkeit (D8093 S. 39). Ausnahmen § 12 Abs. 3, 4 OZG. Laut Governikus müssen Behörden beide Postfächer anbinden (GOV-FPF) **[unverifiziert]**. Anbindung und Betrieb unentgeltlich (VV § 19). Länder: Ende-zu-Ende-Förderverfahren mit MUK-Anbindung bis 31.12.2027 (`../BundID-ZBP/quellen/recht-governance/Foederale_Modernisierungsagenda_MPK_2025-12-04.pdf` S. 31). |

## 6. Widersprüche und offene Punkte

- **Zwei Postfächer, ein Konto:** Die VV verlangt, dass Nutzer das Zielpostfach wählen können (VV § 11 Abs. 3). Ob die Vereinheitlichung (geplant 02/2026, in HL 04/2026 als „NEU“ beworben) produktiv ist, ist nicht ausdrücklich bestätigt.
- **Bekanntgabe-Einwilligung:** VV (S. 7–8) und BayVV verlangen noch eine Einwilligung; § 9 Abs. 1 OZG fingiert sie seit 2024. IL S. 11 nennt beides. Die FAQ (Z. 497, Stand 2023) beschreibt die Opt-out-Regel als „nicht in Kraft“ **[veraltet]**.
- **Bekanntgabefiktion:** IL nennt den dritten Tag (S. 6, 37); § 9 Abs. 1 OZG und § 122a AO den vierten.
- **Limits:** Anhänge im Postfach 2.0 2023 50 MiB / 250 MiB (PF23, FAQ Z. 763), 2026 2 GiB / 5 GiB (IL, HL). `../Referenzen-ELSTER.md` (P20) zitiert noch die Fassung von 2023. OZG-PLUS: 200 MB (OZGPP-CFG) vs. 600 MB (FAQ).
- **Formate:** HL nennt TXT, IL nicht.
- **Zertifikate je Steuernummer:** 17.000 (FAQ Z. 78, ZLF), 5000 (FAQ Z. 402, 2023), 1500 (2024), 500 (Glossar), 200 (2020). Aktuell gilt wohl 17.000.
- **Aufbewahrung:** 2023 ab Abruf (FAQ Z. 310 f.), 2026 unabhängig vom Abruf (HL S. 10).
- **Lesenachweis:** beworben, aber nur „abgeholt“ und „Eingang bestätigt“ als Status dokumentiert.
- **Kosten:** VV und Infoseite „keine Kosten“; 2022 hieß es, „die Anbindung ist eigenständig zu bezahlen“ (`anbindung-technik/st_ozg-sprechstunde_…pdf` PDF-S. 6), gemeint sind wohl die eigenen Integrationskosten **[Folgerung]**.
- **IHK Magdeburg** (2026) nennt Zertifikate personengebunden und den ersten Aktivierer automatisch Admin; die offiziellen Quellen widersprechen (FAQ Z. 141; BSQS S. 8).
- **Zukunft:** ZaPuK ADR-1110 sieht vor, dass BundID und MUK „keine Postfachfunktionalität mehr“ umsetzen (`../ZaPuK/quellen/zapuk-v1.1.1/Zielarchitektur/Architekturentscheidungen/ADR-1110*.md` Z. 86). Das kollidiert mit § 2 Abs. 7 OZG und mit der Pflicht nach § 3 Abs. 3 OZG.
- **Vertrauensniveau:** Die Bundesregierung wollte langfristig „hoch“ (D8093 S. 123); geltendes Recht unbefristet „substantiell“. DIdG würde nach Vertrauensniveau differenzieren, der Bundesrat lehnt das ab (`../BundID-ZBP/quellen/recht-governance/BR-Drs_322-26B_…pdf` S. 6).

## 7. Lücken

- ETR-Handbücher, REST-Howto und die originale ETR-OpenAPI liegen hinter einem Lizenz-Klick auf elster.de und wurden nicht abgerufen. Teile stehen im lokalen ELSTER-Paket (`../ELSTER/`, ElsterBereitstellung-SSB).
- Nicht gefunden: Nutzungsbedingungen für Unternehmen, Inhalt der Benachrichtigungs-E-Mail, Verschlüsselung at rest im Postfach 2.0, Siegel oder Signatur der Behörde, M2M-Doku für Unternehmen, externe Vertretung, Löschfrist bei Inaktivität, Bericht nach § 9 Abs. 2 OZG, Ergebnis des Finanzaudits, Unterzeichnungsstand der VV, Nutzungszahlen 2026.
- Nicht abrufbar: hamburg.de (TLS-Fehler), governikus.de/mein-uk (404), ELSTER-Registrierungsseite (Warteraum), Portal-Hilfe (Login/JavaScript), Bayern-Pressemitteilung zum Start 2021.

## 8. Verwandte lokale Quellen

- `../ELSTER/ELSTER-Zusammenfassung.md` und `../Referenzen-ELSTER.md`: ELSTER-Postfach, ElsterBereitstellung (Postfach 2.0, EPBescheid, EPAntwort, OZGPPNachricht), ERiC.
- `../BundID-ZBP/ZBP-Zusammenfassung.md`: das Gegenstück für natürliche Personen; dort liegen auch OZG, OZGÄndG-, DIdG-Drucksachen und die IT-PLR-Beschlüsse zu ZaPuK.
- `../ZaPuK/quellen/zapuk-v1.1.1/`: MUK als Bestandssystem, ADR-0016 (Identität privater Organisationen), ADR-1110 (Entkopplung Nutzerkonto und Postfach), ADR-1111 (organisationsbezogene Adressen).
