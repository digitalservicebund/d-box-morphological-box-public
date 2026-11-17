# ZBP (Zentrales Bürgerpostfach der BundID): Zusammenfassung für die d-box

Wie das Zentrale Bürgerpostfach (ZBP, auch „BundID-Postfach“) aufgesetzt ist: Betrieb, Anbindung, Recht. Eingeordnet nach den Zeilen von `morph_box.csv`. Die Zusammenfassung verweist auf die Quellen, statt sie zu wiederholen. Für Details den Pointer öffnen.

Stand: 2026-11-17. Grundlage ist eine Webrecherche. Alle Primärquellen liegen in `quellen/`, jeder Unterordner hat eine `_index.md` mit URL, Herausgeber, Datum und Inhaltsnotiz.

## Kennzeichnung

- Ohne Kennzeichnung: in einer Primärquelle belegt.
- **[Folgerung]**: aus mehreren Quellen geschlossen, nicht ausdrücklich belegt.
- **[unverifiziert]**: nur Sekundärquelle oder Annahme.
- **[veraltet?]**: Quelle ist älter, aktueller Stand nicht belegt.

## Kürzel für Quellen

Pfade relativ zu `quellen/`.

| Kürzel | Datei | Inhalt |
|---|---|---|
| DSE | `bundid-betreiber/bundid_datenschutzerklaerung.md` | Datenschutzerklärung id.bund.de: Verantwortliche, Datenflüsse, Postkorb-Handle, Löschfristen |
| FAQ | `bundid-betreiber/bundid_faq_hilfreiche-informationen.md` | FAQ id.bund.de: Postfach, Benachrichtigung, Antworten, Löschung |
| UI | `bundid-betreiber/bundid_ui-texte_postfach.md` | Postfach-Texte der Web-App (Status, Fehlermeldungen) |
| IMP | `bundid-betreiber/bundid_impressum.md` | Impressum: BMDS, AKDB, ITZBund |
| PAP | `bundid-betreiber/personalausweisportal_anbindung-an-die-bundid.md` | Anbindung an die BundID (SSP, Postfach-Zertifikate) |
| SID | `bundid-betreiber/SID_Kundenforum-2025_BundID-Bafoeg21_Bescheiduebermittlung-ZBP_2025-09-16.pdf` | Praxisbericht Bescheidübermittlung ins ZBP (Folien 23–40) |
| MV | `bundid-betreiber/MV_Digitale-Bescheidzustellung-nach-9-OZG_FITKO-stellt-vor_2025-10-10.pdf` | Bescheidzustellung nach § 9 OZG in M-V |
| FC-E | `fitko-fit-connect/fc-docs_zbp-entscheider.md` | FIT-Connect: Entscheidungshilfe ZBP (direkt vs. ZBP-Adapter) |
| FC-T | `fitko-fit-connect/fc-docs_zbp-technische-anbindung.md` | FIT-Connect: technische Anbindung ZBP, Limits, Reject-Gründe |
| FC-R | `fitko-fit-connect/fc-docs_sdk-rezept-zbp-anbinden.md` | SDK-Rezept: Nachricht bauen, Payload, Stolperfallen |
| SDK | `fitko-fit-connect/java-sdk_zbp-client/` | Java-SDK-Code: Endpunkte, Token, Umschlag |
| SCH | `fitko-fit-connect/schema_id.bund.de_message_v6_1.0.0_zbp-message.schema.json` | JSON-Schema der ZBP-Nachricht über den Adapter |
| PLR24 | `fitko-fit-connect/itplr_beschluss-2024-28_postfach-kommunikationsloesungen_architekturueberblick.pdf` | IT-PLR 2024/28: Bebauungsplan Postfachlösungen (S. 8–9) |
| WP22 | `recht-governance/IT-PLR_Beschluss2022-04_Zentrales_Buergerpostfach_Whitepaper.pdf` | Whitepaper ZBP 2022 (SOAP, Anhänge, Lesebestätigung) |
| SB23 | `recht-governance/IT-PLR_Beschluss2023-29_ZBP_Statusbericht.pdf` | ZBP-Statusbericht 2023 |
| OZG | `recht-governance/OZG_gesetze-im-internet_2026-11-17.md` | OZG, aktuelle Fassung |
| D8093 | `recht-governance/BT-Drs_20-8093_Gesetzentwurf_OZG-Aenderungsgesetz.pdf` | Gesetzentwurf OZG-Änderungsgesetz mit Begründung |
| D5367 | `recht-governance/BT-Drs_21-5367_Antwort_KA_BundID-ZBP.pdf` | Antwort der Bundesregierung zu BundID/ZBP (2026) |
| DIdG | `recht-governance/BR-Drs_322-26_Gesetzentwurf_DIdG.pdf`, `BR-Drs_322-26B_…`, `BT-Drs_21-8617_…`, `BR-Drs_618-26_…` | Digitale-Identitäten-Gesetz: Entwurf, Stellungnahme Bundesrat, Beschlussempfehlung, Gesetzesbeschluss |

## 1. Systembild

- Das ZBP ist das Postfach im zentralen Bürgerkonto des Bundes (BundID). Es ging am 25./26.03.2024 live; dabei wurden 3,8 Mio. BundID-Postfächer migriert (`bundid-betreiber/digitale-verwaltung_2024-03-28_…md`). Basis war das NK-Bund-Postfach (WP22).
- Nur Empfang: Behörden schreiben, Bürgerinnen und Bürger lesen. Antworten sind nur auf eine Anfrage der Behörde möglich, wenn die Behörde sie freischaltet; Erstkontakt durch die Person gibt es nicht (FAQ, UI).
- Zwei Wege für Absender, eine Payload (`CreateMessage` / `CreateState`) (FC-R „Zwei Wege, eine Payload“):
  1. **Direkt**: proprietäre REST-API (`PUT /v6/mailbox/messages`, `POST /v6/mailbox/applications/states`), erreichbar „ausschließlich innerhalb der Netze des Bundes (NdB) oder innerhalb des Verbindungsnetzes“ (FC-E; SDK). Synchron.
  2. **ZBP-Adapter** von FIT-Connect (FITKO) für Absender außerhalb NdB/VN: normale FIT-Connect-Einreichung an eine Adapter-Destination, Leistungsschlüssel `urn:schema-fitko-de:fit-connect:id.bund.de:message_v6` bzw. `…:status_v6`. Asynchron (FC-T).
- Neben Nachrichten gibt es den **Statusmonitor**: Antragsstatus `INITIATED → SUBMITTED → RECEIVED → PROCESSING → ACTION_REQUIRED → COMPLETED`, Schlüssel `applicationId` (FC-R). Pilot seit 08/2024, voll seit 10/2025 (D5367).
- Historie: 2022 SAML (Login) plus SOAP (Postfach) (WP22; `bundid-betreiber/BMI_BundID-im-vierten-Jahr_…pdf` S. 9–11). Seit 2024 REST; SOAP wurde bis 30.06.2025 auf REST umgeleitet (`bundid-betreiber/digitale-verwaltung_2024-03-28_…md`).

## 2. Betrieb, Governance, Teilnahme

- Verantwortlicher: BMDS, Referat DS II 7 (Identitäts- und Nachweisdienste, Wallet). Realisierung: AKDB. Betrieb: ITZBund als Auftragsverarbeiter nach Art. 28 DSGVO, Rechenzentren nur in Deutschland (IMP; DSE § 3).
- AKDB beschreibt ein gemeinsames Produkt „Nutzerkonto inkl. zentralem Bürger-Postfach (Bayern-ID, Bund-ID, zBP)“ (`bundid-betreiber/AKDB_DSGVO-Informationsblatt_…pdf`).
- Zuständigkeit ungeklärt: Der Organisationserlass vom 6.5.2025 belässt „Identitätsmanagement“ beim BMI (`recht-governance/Organisationserlass_Bundeskanzler_2025-05-06.md`, XII Nr. 2). § 3 Abs. 1 OZG nennt das BMI. Betrieb und Beantwortung parlamentarischer Anfragen liegen aber beim BMDS (D5367).
- FITKO betreibt FIT-Connect und den ZBP-Adapter im Auftrag des IT-Planungsrats (FC-E).
- Kosten: Mehraufwand des Bundes für BundID laut OZG-Reform 25 Mio. €/Jahr (D8093 S. 30). „Die Kosten der BundID trägt derzeit der Bund“ (`recht-governance/BT-Drs_20-9824_…pdf` S. 6). Kostenfreiheit der Anbindung für Behörden nur sekundär belegt **[unverifiziert]**.
- Nutzung (Jan. 2026): ca. 5,7 Mio. Konten, ca. 2 Mio. Logins/Monat, ca. 1.890 angebundene Dienste (`recht-governance/FITKO_Die-FITKO-stellt-vor_Neo_2026-01-23.pdf` S. 6). Zahlen zu ZBP-Nachrichten oder Bekanntgaben gibt es nicht; die Bundesregierung erhebt sie nicht (D5367 S. 3).

## 3. Identität und Adressierung

- Login: Benutzername/Passwort (Basisregistrierung, optional 2FA per E-Mail-Code), ELSTER-Zertifikat für Privatpersonen (substanziell; Unternehmens-Zertifikate abgelehnt), Online-Ausweis, eAT, EU-Karte, Smart-eID (hoch), eIDAS-Identitäten. EUDI-Wallet laut Personalausweisportal „ab Januar 2027“ (FAQ; `bundid-betreiber/personalausweisportal_die-bundid.md`).
- Jede Nachricht trägt ein Vertrauensniveau (`stork_qaa_level`) und lässt sich nur nach Login auf diesem Niveau öffnen (DSE § 6.4; FC-R).
- Adresse ist das **Postkorb-Handle** („eine Kennziffer, mit der Ihr Postkorb eindeutig identifizierbar und adressierbar ist“, DSE § 6.4). Im Schema: `mailboxUuid`, in FIT-Connect-Metadaten `replyChannel.idBundDeMailbox.mailboxUuid` „aus dem SAML-Token (legacyPostkorbHandle)“ (`fitko-fit-connect/schema_fit-connect_metadata_2.1.0…`).
- Das Handle kommt beim Login vom BundID-Login an den Onlinedienst und wird von dort an das Fachverfahren weitergereicht (SID Folien 24–25; Dataport: SAML-Attribut `InboxReference`). Eine Adressierung ohne vorherigen Login der Person über einen Onlinedienst ist in den Quellen nicht beschrieben **[Folgerung]**.
- bPK2 als Adressierungsmerkmal kommt in keiner Quelle zum ZBP vor.

## 4. Postfach und Konto

- Das Postfach ist laut Gesetz „Bestandteil eines Nutzerkontos“ (§ 2 Abs. 7 OZG) und „untrennbar mit der sicheren Identifizierung … verbunden“ (D8093 S. 39). Ein Gast-Login legt kein Postfach an (FAQ).
- Kein Archiv: Von der Person gelöschte Nachrichten sind weg; die Behörde muss erneut senden (FAQ). Konten werden nach 24 Monaten Inaktivität gelöscht, mit Warnung nach 6 und 2 Monaten; Logs und Backups 90 Tage (DSE § 10; § 8 Abs. 7 OZG). AKDB: Konto endet mit Ablauf des Ausweisdokuments (widerspricht DSE oder geht darüber hinaus).
- Nur Web-App. Keine eigene BundID-App; AusweisApp und Wallet nur zum Login. Mein Justizpostfach setzt ein BundID-Konto voraus (FAQ).
- Keine Nutzung für Organisationen; das Unternehmenskonto ist ein eigenes System.
- Keine Vertretung; 2022 nur „vorkonzeptioniert“ (WP22).

## 5. Einordnung nach `morph_box.csv`

### Behörde

| Zeile | ZBP |
|---|---|
| B1 Anbindung von Fachverfahren | Direkt per REST-API (nur NdB/VN, mTLS mit „BPKI“-Client-Zertifikat plus selbst signiertem JWT RS512, 30 min) oder über den FIT-Connect-ZBP-Adapter (OAuth Client Credentials, signierter Umschlag mit `authorCertificate`, `authorToken`, `sha512sum`). SDKs Java und .NET (FC-E, FC-T, FC-R, SDK). Onboarding über das BundID-Self-Service-Portal (SSP): Postfach-Zertifikat beantragen, Testphase, Techniker-Workshop (PAP). |
| B2 Zugang ohne Fachverfahren | Nicht vorgesehen. Kein Web-Client für Behörden in den Quellen. |
| B3 Adressierung der Person / Organisation | Postkorb-Handle (`mailboxUuid`) aus dem SAML-Login; Statusmonitor über `applicationId` (DSE § 6.4; FC-T). Nur natürliche Personen. |
| B4 Inhalt an die Person / Organisation | JSON: Titel, `content` (HTML aus Allowlist, max. 1 MB), Anhänge mit SHA-512. Adapter: max. 30 MB gesamt, 30 MB je Datei, max. 100 Anhänge; Dateitypen pdf, Bilder, txt, ics, rtf, csv (FC-T). Direktweg: „vergleichbare“ Limits, nur im SSP dokumentiert. 2022: 20 Anhänge à 50 MB **[veraltet?]** (WP22). |

### Person / Organisation

| Zeile | ZBP |
|---|---|
| P1 Benachrichtigung | E-Mail ohne Inhalt, Absender nur @itzbund.de oder @eid-support.de; der Link kann die Login-Optionen auf das nötige Vertrauensniveau beschränken (FAQ). |
| P2 Authentifizierung | BundID-Login: Passwort (niedrig), ELSTER-Zertifikat (substanziell), Online-Ausweis/eAT/EU-Karte/Smart-eID (hoch); EUDI-Wallet ab Jan. 2027. Vertrauensniveau pro Nachricht. |
| P3 Organisationszugang | Keiner. Organisationen nutzen das Unternehmenskonto. |
| P4 Vertretung | Keine. |
| P5 Clients | Nur Web-App von id.bund.de. |
| P6 Anbindung eigener Systeme | Keine API für Bürgerinnen und Bürger in den Quellen. |
| P7 Antwort und Erstkontakt | Antwort nur auf Anfrage der Behörde, wenn freigeschaltet; Status „Antwort erforderlich / Geantwortet / Antwort verpasst / Anfrage geschlossen“, Fristen möglich (UI, FAQ). Kein Erstkontakt. Bundesregierung: Bidirektionalität „ab Juli 2026“ (D5367). FIT-Connect-Roadmap: „ZBP-Adapter: Bidirektionale Kommunikation zum ZBP — Noch offen“ (`fitko-fit-connect/fc-docs_roadmap.md`). |
| P8 Inhalt an die Behörde | Text und Uploads; Limits je Nachricht aus `antwortInformation`, wohl von der Behörde gesetzt **[Folgerung aus Client-Code]**. |
| P9 Postfach und Nutzerkonto | Postfach fest im Konto (§ 2 Abs. 7 OZG). Löschung nach 24 Monaten Inaktivität. |

### Übermittlung

| Zeile | ZBP |
|---|---|
| Ü1 Modus | Direkt: synchroner REST-Aufruf, Antwort `messageUuid`. Adapter: asynchron über FIT-Connect, Event-Log mit `accept-submission`/`reject-submission`, Polling oder Callback (FC-T). |
| Ü2 Vertraulichkeit | Keine Ende-zu-Ende-Verschlüsselung: IT-PLR 2024 kennzeichnet das ZBP als „Keine E2E“ (PLR24 S. 9). Transport TLS/mTLS; FIT-Connect verschlüsselt per JWE bis zum Adapter, der Adapter entschlüsselt und reicht signierten Inhalt weiter **[Folgerung]**. 2022: „Nachrichten im Postfach werden verschlüsselt gespeichert“ (WP22) **[veraltet?]**. E2EE ist gesetzgeberisch „nach Zurverfügungstellung entsprechender Haushaltsmittel“ beabsichtigt (`recht-governance/BT-Drs_20-10417_…pdf` S. 24). |
| Ü3 Echtheit | Absender authentifiziert per mTLS-Zertifikat bzw. Signatur (SHA512withRSA) über den Inhalt; im ZBP erscheint der ursprüngliche Autor, nicht der Adapter (FC-R „Im Auftrag eines Autors“). Schriftformersatz über qualifiziertes elektronisches Siegel (§ 9a Abs. 6 OZG). |
| Ü4 Ereignisse und Bestätigungen | Adapter: accept/reject mit Gründen (Signature invalid, Token expired, Message rejected …) (FC-T). Optionales Feld `retrievalConfirmationAddress` („Abrufbestätigung“), Semantik nicht dokumentiert. 2022: synchrone technische Quittung plus Lesebestätigung beim Öffnen (WP22) **[veraltet?]**. M-V wertet die Speicherbestätigung des ZBP als Nachweis der Bereitstellung (MV Folie 6). |
| Ü5 Interoperabilität mit Systemen außerhalb | FIT-Connect-Adapter als Brücke für Absender außerhalb NdB/VN. FINK (Interoperable Servicekonten) ist in FIT-Connect-Metadaten 3.0 entfernt (`fitko-fit-connect/schema_fit-connect_metadata_3.0.0…`). Länderkonten werden ins ZBP migriert statt verbunden. |
| Ü6 Malware-Prüfung | Uploads der Bürgerinnen und Bürger werden automatisch geprüft; Makros können zur Ablehnung führen (FAQ; UI-Fehler `malwareDetected`). Prüfung eingehender Behördennachrichten nicht dokumentiert. |

### Infrastruktur und Betrieb

| Zeile | ZBP |
|---|---|
| I1 Betrieb und Governance | Bund (BMDS verantwortlich, ITZBund Betrieb, AKDB Entwicklung); FITKO für den Adapter. Ministerielle Zuständigkeit BMI vs. BMDS formal unklar (siehe 2.). |
| I2 Datenhaltung und Versand | Zentral beim ITZBund in Deutschland (DSE § 3). Inhalte für den Betreiber lesbar **[Folgerung aus fehlender E2EE]**. |
| I3 Betriebsmodell für Organisationen | Nicht anwendbar (nur natürliche Personen). |
| I4 Funktionspostfächer und interne Bearbeitung | Nicht vorhanden. |
| I5 Aufnahme in den Verbund | Behörden über das BundID-SSP (Admin-Registrierung mit ELSTER-Organisationszertifikat, Postfach-Zertifikat, Nutzungsbedingungen); reine ZBP-Anbindung ohne Login möglich (SID Folien 23–40). Adapter zusätzlich: FIT-Connect-Client-ID im SSP eintragen mit Begründung, warum kein NdB-Anschluss besteht; Destination-IDs für Stage/Prod vom BMDS-Anbindungsmanagement (FC-E, FC-T). Rollen: `fitko-fit-connect/fitko_zbp-adapter_prozessgrafik_*.pdf`. |

### Recht

| Zeile | ZBP |
|---|---|
| R1 Schriftformersatz | Dokumente mit qualifiziertem elektronischem Siegel ersetzen die Schriftform (§ 9a Abs. 6 OZG). |
| R2 Teilnahmepflicht | Freiwillig für Bürgerinnen und Bürger (§ 3 Abs. 1 S. 2 OZG). Keine Pflicht für Behörden, ZBP oder Statusmonitor zu nutzen (D5367 S. 3). Länderkonten dürfen nach § 12 Abs. 1 OZG noch 3 Jahre laufen, ab einer Bekanntmachung, die bis 10.7.2026 nicht erfolgt war (`recht-governance/BR-Drs_322-26B_…pdf` S. 5). 10 Länder sind umgestellt (D5367 S. 5). |
| R3 Bekanntgabe und förmliche Zustellung | Bekanntgabe nach § 9 Abs. 1 OZG: Verwaltungsakt gilt am vierten Tag nach Bereitstellung zum Abruf als bekannt gegeben (bis 1.1.2025: dritter Tag), bei früherem Abruf ab Abruf. Einwilligung gilt als erteilt, Widerspruch möglich (Opt-out kam erst im Vermittlungsausschuss, `recht-governance/BT-Drs_20-11790_…pdf` S. 2). Behörde trägt im Zweifel die Beweislast für Bereitstellung. Alternative: § 41 Abs. 2a VwVfG. Förmliche Zustellung ist nicht geregelt; Harmonisierung sollte bis 30.9.2026 kommen, kein Nachweis gefunden (`recht-governance/Foederale_Modernisierungsagenda_MPK_2025-12-04.pdf` S. 35). |
| R4 Teilnahme der Behörden | Freiwillig, alle Ebenen. Anbindung über Kooperationsvereinbarungen bzw. SSP-Nutzungsbedingungen. |

## 6. Widersprüche und offene Punkte

- **Zuständigkeit BMI vs. BMDS**: Organisationserlass und § 3 OZG nennen das BMI, Impressum und Datenschutzerklärung das BMDS.
- **Netzzugang**: FC-E sagt „ausschließlich NdB/VN“, das Java-SDK kennt aber eine Umgebung `INT_INTERNET` (`https://int.zbp.bund.de`). Gilt wohl nur für Integration **[Folgerung]**.
- **Adapter-Authentifizierung**: Java-SDK 2.4.0 (2024) nennt das Postfach-Zertifikat „als Cookie im HTTP-Header“, SDK 4 dokumentiert mTLS (`fitko-fit-connect/fc-docs_changelog-java-sdk-legacy.md`).
- **Bidirektionalität**: Bundesregierung „ab Juli 2026“, FIT-Connect-Roadmap „noch offen“. Ob die Freischaltung erfolgt ist, ist nicht belegt.
- **Verschlüsselung at rest** und **Abruf-/Lesebestätigung**: nur Stand 2022 belegt.
- **Kontolaufzeit**: AKDB koppelt sie an das Ausweisdokument, BMDS nur an Inaktivität.
- **DIdG**: Identifizierung ohne BundID würde Nutzer ohne Postfach erzeugen. Der Bundestag hat daraufhin festgeschrieben, dass das zentrale Bürgerkonto „stets gewährleistet“ ist (`recht-governance/BT-Drs_21-8617_…pdf` S. 16). Gesetzesbeschluss vom 15.10.2026, zweiter Durchgang Bundesrat bis 20.11.2026, noch nicht in Kraft.
- **Zukunft**: ZaPuK sieht vor, das ZBP-Backend abzulösen und das Postfach aus der BundID herauszulösen (`../zapuk-main/Glossar.md`, Eintrag ZBP; `../zapuk-main/Zielarchitektur/Architekturentscheidungen/ADR-1110*.md`). Das kollidiert mit § 2 Abs. 7 OZG. Neo (Matrix) soll laut `../neo-solution-architecture-develop/` das ZBP ersetzen.

## 7. Lücken

- Die eigentliche ZBP-API-Doku (erlaubte HTML-Tags, Fehlercodes, aktuelle Limits, Release Notes) liegt im Hilfecenter des BundID-SSP (`https://ssp.id.bund.de/ip?id=kb_home`, Kategorie „BundID Anbindung: Zentrales Bürgerpostfach“). Die Seite braucht JavaScript bzw. Login und wurde nicht gesichert.
- Nicht gefunden: Nutzungsbedingungen für Behörden, öffentliche OpenAPI-Spezifikation des ZBP, Bericht nach § 9 Abs. 2 OZG (fällig 10.12.2025), Rechtsverordnung nach § 3 Abs. 1 S. 3 OZG, Bericht des Bundesrechnungshofs zur BundID.
- Die Checkliste „Betreiber … ZBP-Adapter“ in der FITKO-Mediathek verlinkt auf ein falsches Dokument.

## 8. Verwandte lokale Quellen

- `../zapuk-main/`: ZBP als Bestandssystem, Ablösung, ADR-0011 (ZBP-Anbindung über FIT-Connect), ADR-1110 (Entkopplung Nutzerkonto und Postfach).
- `../neo-solution-architecture-develop/`: Matrix-Nachfolger, der in die BundID integriert werden und das ZBP ersetzen soll.
- `../ELSTER-Zusammenfassung.md`: ELSTER-Postfach als Vergleich (ELSTER-Zertifikat ist auch BundID-Login-Mittel).
