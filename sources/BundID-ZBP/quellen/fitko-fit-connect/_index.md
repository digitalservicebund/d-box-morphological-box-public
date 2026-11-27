# Quellen: FITKO / FIT-Connect / IT-Planungsrat – Anbindung des Zentralen Bürgerpostfachs (ZBP)

Abgerufen am 2026-11-17. HTML-Seiten von docs.fitko.de wurden mit `lynx -dump` in Text umgewandelt (Navigation entfernt). Am Dateianfang stehen jeweils Quelle und Abrufdatum. PDFs, JSON-Schemas, OpenAPI-Dateien und Quellcode wurden unverändert gespeichert. Die FIT-Connect-Doku steht unter CC BY 4.0, die APIs unter CC BY-SA 4.0 und der Java-SDK-Code unter EUPL-1.2.

Verwandte Quellen in anderen Ordnern (nicht erneut heruntergeladen): `../recht-governance/` (IT-PLR-Beschlüsse 2022-04 und 2023-29 zum ZBP, BT-Drs. 21/5367), `../../../zapuk-v1.0.0-121-g7419d8b/`, `../../../neo-solution-architecture-develop/`.

## FIT-Connect-Dokumentation (docs.fitko.de)

| Datei | URL | Titel / Herausgeber | Datum | Inhalt |
|---|---|---|---|---|
| fc-docs_zbp-entscheider.md | https://docs.fitko.de/fit-connect/docs/organisation-tasks/buergerpostfach-entscheider | Zentrales Bürgerpostfach – FITKO | o. D. (Stand 2026-11) | Seite für Entscheider: ZBP-REST-API nur aus NdB/Verbindungsnetz erreichbar; außerhalb davon ZBP-Adapter als Brückenkopf; Client-ID im BundID-SSP angeben |
| fc-docs_zbp-technische-anbindung.md | https://docs.fitko.de/fit-connect/docs/zbp/ | FIT-Connect und das Zentrale Bürgerpostfach (ZBP) – FITKO | o. D. | Technik: Destination-ID des Adapters, Leistungsschlüssel message_v6/status_v6, Envelope-Felder, Grenzen für Anlagen, Fehlercodes im Reject-Event, Gültigkeit des Tokens (30 Min.), Postkorb-Handle |
| fc-docs_sdk-rezept-zbp-anbinden.md | https://docs.fitko.de/fit-connect/docs/sdks/rezepte/zbp | ZBP anbinden (SDK-Rezept) – FITKO | SDK 4.0.0 | Direkter Weg (mTLS) und Weg über den Adapter, ZBP-Umgebungen/URLs, Felder von CreateMessage/CreateState, Statusverlauf, Senden im Auftrag eines Autors |
| fc-docs_anbindung-einstieg.md | https://docs.fitko.de/fit-connect/docs/Anbindung/Einstieg | Einstieg in die Anbindung – FITKO | o. D. | ZBP als vierter Anbindungsfall neben Senden und Empfangen; Fragen für die Projektleitung |
| fc-docs_anbindung-anleitung.md | https://docs.fitko.de/fit-connect/docs/Anbindung/Anleitung | Schritt-für-Schritt-Anleitung – FITKO | o. D. | Rückkanal für Bescheide wählen (ZBP, FIT-Connect, ELSTER, FINK, E-Mail, Brief); Aufwandsschätzungen |
| fc-docs_anbindungsszenarien.md | https://docs.fitko.de/fit-connect/docs/Marketing/Szenarien | Anbindungsszenarien – FITKO | o. D. | Szenario 3: Anbindung über den ZBP-Adapter |
| fc-docs_metadaten-replyChannel.md | https://docs.fitko.de/fit-connect/docs/metadata/replyChannel | Rückkanal (replyChannel) – FITKO | o. D. | Rückkanäle E-Mail, De-Mail, ELSTER, FIT-Connect, BundID; ohne replyChannel wird per Post geantwortet |
| fc-docs_metadaten-additionalReferenceInfo.md | https://docs.fitko.de/fit-connect/docs/metadata/additionalReferenceInfo | Weitere Angaben – FITKO | o. D. | idBundDeApplicationId (Antrags-ID für den Statusmonitor), zustimmungZumDigitalenBescheid |
| fc-docs_rueckkanaele-overview.md | https://docs.fitko.de/fit-connect/docs/reply-channels/overview | Rückkanäle, Überblick – FITKO | o. D. | Bidirektionale Kommunikation über FIT-Connect (Case, Replies); externe Kanäle werden in FIT-Connect nicht abgebildet |
| fc-docs_bidiko-entscheider.md | https://docs.fitko.de/fit-connect/docs/organisation-tasks/bidiko-entscheider | Dialog während der Antragsbearbeitung – FITKO | o. D. | Vorhandene Antwortwege: Bürgerpostfächer (FINK), ELSTER, De-Mail, E-Mail |
| fc-docs_verschluesselung.md | https://docs.fitko.de/fit-connect/docs/getting-started/encryption | Verschlüsselte Übertragung von Antragsdaten – FITKO | o. D. | Hybride Verschlüsselung mit JWE; nur das Zielsystem kann entschlüsseln |
| fc-docs_authentifizierung.md | https://docs.fitko.de/fit-connect/docs/getting-started/authentication | Authentifizierung – FITKO | o. D. | OAuth Client Credentials (client_id/secret aus dem SSP), Scopes |
| fc-docs_accept-reject.md | https://docs.fitko.de/fit-connect/docs/sending/accept-reject | Empfangsbestätigung oder Zurückweisung – FITKO | o. D. | Statusmodell; accept-/reject-submission als signiertes SET und Übermittlungsnachweis |
| fc-docs_event-log-events.md | https://docs.fitko.de/fit-connect/docs/getting-started/event-log/events | Ereignisse – FITKO | o. D. | Event-Typen, auch bei Replies |
| fc-docs_callbacks.md | https://docs.fitko.de/fit-connect/docs/details/callbacks | Verwendung von Callbacks – FITKO | o. D. | Callback-Mechanismus und Signaturprüfung |
| fc-docs_limits.md | https://docs.fitko.de/fit-connect/docs/getting-started/limits | Limits bei Einreichungen – FITKO | o. D. | Allgemeine FIT-Connect-Grenzen für Größen, Fristen und Rate-Limits |
| fc-docs_loeschfristen.md | https://docs.fitko.de/fit-connect/docs/getting-started/notifications-and-deletion-deadlines | Benachrichtigungen und Löschfristen – FITKO | o. D. | Reject nach 14 Tagen, Löschung nach 21 Tagen, SETs werden nach 730 Tagen gelöscht |
| fc-docs_roadmap.md | https://docs.fitko.de/fit-connect/docs/roadmap | Roadmap – FITKO | 2026 | „Pilotierung ZBP-Anbindung: Fertig“; „ZBP-Adapter: Bidirektionale Kommunikation: Noch offen“ |
| fc-docs_abkuendigungen.md | https://docs.fitko.de/fit-connect/docs/development/discontinuation | Abkündigungen – FITKO | Stand 2026-11 | API v1 und Metadaten 1.x laufen am 27.10.2026 in PROD aus, API v2 am 27.10.2027 |
| fc-docs_abk-metadatenschema.md | https://docs.fitko.de/fit-connect/docs/development/abk_metadatenschema | Abkündigung Metadatenschema – FITKO | o. D. | idBundDeMailbox und idBundDeApplicationId kamen mit Metadaten 1.4.0 hinzu |
| fc-docs_migration-guide-3-0.md | https://docs.fitko.de/fit-connect/docs/major-release/fit-connect_3_0/migration_guide_3_0 | Migration-Guide v3 – FITKO | o. D. | In Metadaten 3.0.0 wird replyChannel zum Dataset reply-channel 1.0.0 (mehrere Kanäle möglich) |
| fc-docs_changelog-java-sdk-legacy.md | https://docs.fitko.de/fit-connect/docs/sdks/java-sdk/changelog | Changelog Java-SDK (Legacy) – FITKO | 2024–2025 | 2.2.0 (2024-09-16) BundID-Rückkanal; 2.3.0 (2024-09-26) ZBP-Client; 2.4.0 (2024-11-29) Zertifikat als Cookie statt mTLS |
| fc-docs_changelog-net-sdk.md | https://docs.fitko.de/fit-connect/docs/sdks/net-sdk/changelog | Changelog .NET-SDK – FITKO | 2023–2026 | 2.3.0 (2024-11-18) BundID-Rückkanal; 2.3.9/2.4.0 (2025-08-21) ZBP-Unterstützung |
| fc-docs_changelog-sdk-4.md | https://docs.fitko.de/fit-connect/docs/sdks/referenz/changelog | Changelog & Migration SDK 4 – FITKO | 2026 | SDK 4.0: Modul zbp-client bzw. Fitko.FitConnect.Zbp, ZBPEnvelopeBuilder öffentlich, OfIdBundDeMailbox |
| fc-docs_sdk-exceptions.md | https://docs.fitko.de/fit-connect/docs/sdks/referenz/exceptions | Exceptions – FITKO | SDK 4.0 | ZBPException / FitConnectZbpException |
| fc-docs_faq.md | https://docs.fitko.de/fit-connect/docs/faq | FAQ – FITKO | o. D. | 2.3: Der Zustelldienst implementiert den FINK-Rückkanal nicht selbst, sondern transportiert nur die Postfach-ID |
| fc-docs_mediathek-dokumente.md | https://docs.fitko.de/fit-connect/docs/mediathek/documents | Dokumente – FITKO | o. D. | Liste der Checklisten und Prozessgrafiken, auch zum ZBP-Adapter (ein Link fehlerhaft, siehe unten) |

## PDFs (FITKO, IT-Planungsrat, Dritte)

| Datei | URL | Titel / Herausgeber | Datum | Inhalt |
|---|---|---|---|---|
| fitko_zbp-adapter_checkliste_beauftragende-behoerde.pdf | https://docs.fitko.de/fit-connect/assets/files/FITConnect_Checkliste_beauftragende_Behoerde-063bb16a275b6f5d9a605d02cd0cb8e6.pdf | Checkliste für beauftragende Behörden (ZBP-Adapter) – FITKO | o. D. | Die beauftragende Behörde unterzeichnet nur Nutzungsbedingungen und AVV; Anbindungsmanagement fit-connect@spotgroup.de |
| fitko_zbp-adapter_prozessgrafik_beauftragende-behoerde.pdf | https://docs.fitko.de/fit-connect/assets/files/Prozessgrafik_Beauftragende-Behoerden-5a16d1df24898140d01206491713e6b7.pdf | Prozessablauf beauftragende Behörden (ZBP-Adapter) – FITKO | o. D. | Grafik ohne Textebene |
| fitko_zbp-adapter_prozessgrafik_anwendungsbetreiber.pdf | https://docs.fitko.de/fit-connect/assets/files/Prozessgrafik_Anwendungsbetreiber_FITKO-4734199b432a03213d1abe12aec3d2eb.pdf | Prozessablauf Betreiber der angebundenen Anwendung (ZBP-Adapter) – FITKO | o. D. | Grafik: SSP-Test → AVV/NB → ZBP: Nutzungsbedingungen und Beauftragung des Postfach-Zertifikats → Zugangsdaten STAGE/PROD → Adapter wird konfiguriert |
| fitko_fit-connect_handout_it-security.pdf | https://docs.fitko.de/fit-connect/assets/files/FIT-Connect_Hand_Out_IT-Security-06ae55908f3ad944343b621d53db2bcf.pdf | IT-Security Handout – FITKO | o. D. | E2E-Verschlüsselung, V-PKI-Zertifikate, OAuth 2.0, Absenderprüfung |
| fitko_stellt-vor_fit-connect_2025-11.pdf | https://www.fitko.de/fileadmin/fitko/veranstaltungen/die_fitko_stellt_vor/2025_FITKOstelltvor_FIT-Connect..pdf | Die FITKO stellt vor: FIT-Connect – FITKO | PDF 2025-11-18 | Folien 8–9: ZBP über SDK oder Adapter („Brückenkopf“); „derzeit auf Test verfügbar, Stage/Prod in Pilotierung“ |
| fitko_stellt-vor_fit-connect_2024-11-12.pdf | https://fitko.de/fileadmin/fitko/veranstaltungen/die_fitko_stellt_vor/20241112_FSV_FIT_Connect_Praesentation.pdf | Die FITKO stellt vor: FIT-Connect – FITKO | 2024-11-12 | Über 420 Hochschulen können FIT-Connect zur Übermittlung an das ZBP nutzen; Empfehlung der KMK |
| fitko_stellt-vor_neo_2026-01-23_aktualisiert.pdf | https://www.fitko.de/fileadmin/fitko/veranstaltungen/die_fitko_stellt_vor/2026-01-23-Die-FITKO-stellt-vor-Neo_aktualisiert.pdf | Die FITKO stellt vor: Neo – FITKO / Föderales IT-Architekturboard | 2026-01-23 | „BundID-Postfach = ZBP“; Neo mit FIT-Connect-Kompatibilität und Erweiterung des FIT-Connect-SDK |
| itplr_beschluss-2024-28_postfach-kommunikationsloesungen_architekturueberblick.pdf | https://it-planungsrat.de/fileadmin/beschluesse/2024/Beschluss2024-28_Postfach-und_Kommunikationsl%C3%B6sungen_Architektur%C3%BCberblick.pdf | Architekturüberblick Postfach- und Kommunikationslösungen – IT-PLR / FITKO | 44. Sitzung, 2024-06-19 | Bebauungsplan: ZBP mit Bidirektionalität „in Umsetzung“ und ohne E2E (Folien 8–9) |
| itplr_beschluss-2025-28_zapuk_ueberblick.pdf | https://www.it-planungsrat.de/fileadmin/beschluesse/2025/Beschluss_2025_28_ZaPuK_%C3%9Cberblick.pdf | Zielarchitektur Postfach- und Kommunikationslösungen (ZaPuK), Überblick – IT-PLR | 47. Sitzung, PDF 2025-05-13 | Frontends von ZBP, MJP, OZGPP und Postfach 2.0 sollen zusammengeführt werden; FIT-Connect als zentraler Anbindungsweg (Abschnitt 4.1.2) |
| itplr_beschluss-2020-42_fink_technischer-abschlussbericht.pdf | https://media.frag-den-staat.de/files/foi/614150/Beschluss2020-42_FINK_Interoperable_Servicekonten_Technischer_Abschlussbericht.pdf | Technischer Abschlussbericht FINK – IT-PLR (über FragDenStaat) | PDF 2020-09-09 | FINK-Metadatenserver, Spezifikation „Postfächer Interoperabler Servicekonten“ v0.5 |
| sid-sachsen_kundenforum-2025_bundid-bafoeg21.pdf | https://www.sid.sachsen.de/download/SID_Kundenforum_2025_ORANGE_BundID_Bafog21.pdf | SID Kundenforum 2025: BundID / BAföG21 – Staatsbetrieb SID Sachsen | 2025-09-16 | Varianten der Anbindung an das ZBP (PostfachHandle aus der Authentifizierung, ZBP-Modul, FIT-Connect-Client + ZBP-Adapter); Anbindung nur an das ZBP ist über das BundID-SSP möglich |

## Schemas und API-Spezifikationen (schema.fitko.de)

| Datei | URL | Titel / Herausgeber | Datum | Inhalt |
|---|---|---|---|---|
| schema_id.bund.de_message_v6_1.0.0_zbp-message.schema.json | https://schema.fitko.de/fit-connect/id.bund.de/message_v6/1.0.0/zbp-message.schema.json | Nachricht Zentrales Bürgerpostfach BundID – FITKO | v1.0.0 | Envelope mit den Pflichtfeldern authorCertificate, authorToken, content und sha512sum (684 Zeichen) |
| schema_fit-connect_metadata_2.1.0.schema.json | https://schema.fitko.de/fit-connect/metadata/2.1.0/metadata.schema.json | Metadatenschema 2.1.0 – FITKO | 2.1.0 | replyChannel.idBundDeMailbox.mailboxUuid („legacyPostkorbHandle“ aus dem SAML-Token); fink als deprecated markiert |
| schema_fit-connect_metadata_3.0.0.schema.json | https://schema.fitko.de/fit-connect/metadata/3.0.0/metadata.schema.json | Metadatenschema 3.0.0 – FITKO | 3.0.0 | Kein replyChannel mehr auf Wurzelebene; idBundDeApplicationId („DeutschlandID Statusmonitor“) |
| schema_fit-connect_dataset_reply-channel-1.0.0.schema.json | https://schema.fitko.de/fit-connect/metadata/dataset/reply-channel-1.0.0.schema.json | Dataset reply-channel 1.0.0 – FITKO | 1.0.0 | idBundDeMailbox.mailboxUuid; fink wurde entfernt |
| openapi_submission-api_3.1.1.yaml | https://schema.fitko.de/fit-connect/submission-api/3.1.1/submission-api.yaml | Submission API V3 – FITKO | 3.1.1 | Destination-replyChannels.idBundDeMailbox.useStatusMonitor; Attachment-Limits |
| openapi_destination-api_3.1.0.yaml | https://schema.fitko.de/fit-connect/destination-api/3.1.0/destination-api.yaml | Destination API – FITKO | 3.1.0 | Wie oben: idBundDeMailbox/useStatusMonitor am Zustellpunkt |

## Quellcode (git.fitko.de, fit-connect/java-sdk, Branch main, Modul zbp-client)

| Datei | URL | Titel / Herausgeber | Datum | Inhalt |
|---|---|---|---|---|
| java-sdk_zbp-client/README.md | https://git.fitko.de/fit-connect/java-sdk/-/blob/main/zbp-client/README.md | FIT-Connect SDK – ZBP Client – FITKO | Stand 2026-11 | Nutzung direkt und über den Adapter, Tabelle der Umgebungen |
| java-sdk_zbp-client/ZBPApiService.java | https://git.fitko.de/fit-connect/java-sdk/-/blob/main/zbp-client/src/main/java/dev/fitko/fitconnect/zbp/internal/ZBPApiService.java | FITKO | Stand 2026-11 | ZBP-REST-Pfade: PUT /v6/mailbox/messages (multipart „json“ + „files“), POST /v6/mailbox/applications/states; Bearer-Token; optional Zertifikat als Cookie |
| java-sdk_zbp-client/TokenGenerator.java | https://git.fitko.de/fit-connect/java-sdk/-/blob/main/zbp-client/src/main/java/dev/fitko/fitconnect/zbp/internal/TokenGenerator.java | FITKO | Stand 2026-11 | Selbst signiertes JWT (RS512) mit den Claims signer und roles=[THIRD_PARTY], gültig 30 Minuten |
| java-sdk_zbp-client/ZBPEnvelopes.java | https://git.fitko.de/fit-connect/java-sdk/-/blob/main/zbp-client/src/main/java/dev/fitko/fitconnect/zbp/internal/ZBPEnvelopes.java | FITKO | Stand 2026-11 | sha512sum = SHA512withRSA-Signatur über den JSON-Payload (Base64); signer = CN des Zertifikats |
| java-sdk_zbp-client/ZBPEnvelopeBuilder.java | https://git.fitko.de/fit-connect/java-sdk/-/blob/main/zbp-client/src/main/java/dev/fitko/fitconnect/zbp/ZBPEnvelopeBuilder.java | FITKO | Stand 2026-11 | Öffentliche API zum Signieren des Envelopes für den Adapter |
| java-sdk_zbp-client/ZBPEnvironment.java | https://git.fitko.de/fit-connect/java-sdk/-/blob/main/zbp-client/src/main/java/dev/fitko/fitconnect/zbp/ZBPEnvironment.java | FITKO | Stand 2026-11 | ZBP-Basis-URLs (INT/PROD, Internet/NdB/NdB-VN) |

## Nicht gespeichert / fehlgeschlagen

- `https://docs.fitko.de/fit-connect/assets/files/FIT-Connect-Checkliste-Campus-Management-Systeme-…pdf`: Die Mediathek verlinkt diese Datei als „Checkliste für Betreiber der angebundenen Anwendung für den ZBP-Adapter“, sie enthält aber die Checkliste für Campus-Management-Systeme. Die eigentliche Betreiber-Checkliste wurde nicht gefunden.
- `https://docs.fitko.de/fit-connect/assets/files/11-24_Roadmap-FIT-Connect_1-…pdf` (aus der Websuche): Es wird HTML statt PDF geliefert, die Datei ist offenbar nicht mehr vorhanden.
- `https://ssp.id.bund.de/ip/de/zbp-adapter?id=kb_article_view&sysparm_article=KB0010306` (ZBP-Adapter-Artikel im BundID-SSP) und die „ZBP-Dokumentation“ bzw. „technische Informationen im Downloadbereich des SSP der BundID“: nur mit Login zugänglich und nicht abgerufen.
- `https://gitlab.opencode.de/sh/zit/ozg-cloud/app/postfach/fitconnect-zbp` (OZG-Cloud Schleswig-Holstein): Das Repository existiert, die README ist aber leer.
- Volltextsuche in den Repos git.fitko.de/fit-connect/docs und /konzepte: 401 (die Blob-Suche braucht einen Login).
