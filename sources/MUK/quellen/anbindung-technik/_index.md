# Quellen: Mein Unternehmenskonto (MUK) – Anbindung von Behörden, Onlinediensten und Fachverfahren, Technik

Abgerufen am 2026-11-18. HTML-Seiten wurden mit `lynx -dump -nolist` in Text umgewandelt (Navigation, Cookie-Banner und Fußzeile entfernt). Am Dateianfang stehen jeweils Titel, Quelle und Abrufdatum. PDFs und OpenAPI-Dateien wurden unverändert gespeichert. Die Downloads unter `service.mein-unternehmenskonto.de/api/downloads/<name>` liefern immer die aktuelle Fassung; Stand und Version stehen im Dateinamen.

Themen der parallelen Ordner (hier nicht erneut gesichert): `../muk-betreiber/` (Produkt- und Nutzersicht, FAQ, Datenschutz, Impressum, Registrierungsleitfaden, Quickstart-Guide für nutzende Organisationen, Infosheets für Unternehmen) und `../recht-governance/` (OZG, IT-PLR-Beschlüsse, Drucksachen, BayVV).

## Bereits vorhanden in anderen Ordnern (nicht dupliziert)

| Datei | Inhalt (Bezug zum MUK) |
|---|---|
| ../../../BundID-ZBP/quellen/fitko-fit-connect/fc-docs_metadaten-replyChannel.md | FIT-Connect-Rückkanäle: E-Mail, De-Mail, ELSTER, FIT-Connect, BundID |
| ../../../BundID-ZBP/quellen/fitko-fit-connect/schema_fit-connect_dataset_reply-channel-1.0.0.schema.json | `replyChannels.elster` mit `accountId` (Pflicht), `lieferTicket`, `geschaeftszeichen` (max. 60); Verweis auf die ETR-Schnittstellenhilfe |
| ../../../BundID-ZBP/quellen/fitko-fit-connect/schema_fit-connect_metadata_2.1.0.schema.json | `replyChannel.elster` (Metadaten 2.x) |
| ../../../BundID-ZBP/quellen/fitko-fit-connect/fc-docs_bidiko-entscheider.md | ELSTER als vorhandener Antwortweg: „Das Verwaltungssystem sendet Übermittlungen zum Postfach des ELSTER-Benutzerkontos“ |
| ../../../BundID-ZBP/quellen/fitko-fit-connect/fc-docs_anbindung-anleitung.md | Rückkanalwahl ZBP, FIT-Connect, ELSTER, FINK, E-Mail, Brief |
| ../../../BundID-ZBP/quellen/fitko-fit-connect/itplr_beschluss-2024-28_postfach-kommunikationsloesungen_architekturueberblick.pdf | Bebauungsplan inkl. Postfach 2.0 und OZG-PLUS-Postfach |
| ../../../BundID-ZBP/quellen/fitko-fit-connect/itplr_beschluss-2025-28_zapuk_ueberblick.pdf | ZaPuK: Frontends von ZBP, MJP, OZGPP und Postfach 2.0 zusammenführen |
| ../../../ELSTER/Dev Dokumentation/Datenarten/ElsterBereitstellung/Dokumentation/ElsterBereitstellung-SSB-V5.0.8.pdf | Schnittstelle ElsterBereitstellung (Senderseite), Datenarten `EP*`, `OZGPPNachricht`, Adressierung, Verschlüsselung in der BEST-DB |
| ../../../ELSTER/Dev Dokumentation/ERiC-Entwicklerhandbuch.pdf | Kap. 9.15 Postfach 2.0 (EPAntwort, OZGPPNachricht, Limits) |
| ../../../ELSTER/ELSTER-Zusammenfassung.md, ../../../Referenzen-ELSTER.md | Bisherige Auswertung (P20, ZERT-LF, ETR, LF-GEWST, FAQ-GEWST) |

## Mein Unternehmenskonto: Webseiten (info.mein-unternehmenskonto.de)

| Datei | URL | Titel / Herausgeber | Datum Dokument | Inhalt (kurz) |
|---|---|---|---|---|
| muk-info_behoerden-als-dienstanbieter.md | https://info.mein-unternehmenskonto.de/behoerden-als-dienstanbieter/ | Behörden als Dienstanbieter – MUK (BayLfSt/Bremen) | o. D. (Stand 2026-11) | Wer anbinden kann; Weg über SSP: NEZO & Postfach 2.0, Berechtigungssteuerung (Anfrage Dataport), OZG-PLUS-Postfach (Anfrage Governikus); „Kostenfreie Anbindung“ |
| muk-info_behoerden-als-nutzer.md | https://info.mein-unternehmenskonto.de/behoerden-als-nutzer/ | Behörden als Nutzer – MUK | o. D. | Behörden mit Steuernummer können MUK als Antragsteller nutzen |
| muk-info_technische-hintergrundinformationen.md | https://info.mein-unternehmenskonto.de/technische-hintergrundinformationen/ | Technische Hintergrundinformationen – MUK | o. D. | NEZO und SSP in Kürze |
| muk-info_verwaltungsportale.md | https://info.mein-unternehmenskonto.de/verwaltungsportale/ | Verwaltungsportale – MUK | Stand Mai 2025 | 924 Anbindungspartner; Portale (Bundesportal, SV-Meldeportal, HB, MV, NRW, SH) |
| muk-info_downloads.md | https://info.mein-unternehmenskonto.de/downloads/ | Downloads – MUK | o. D. | Liste der Dokumente für Unternehmen, Behörden und Technik |
| muk-info_news_bidirektionale-kommunikation_2023-07-28.md | https://info.mein-unternehmenskonto.de/bidirektionalekommunikation/ | Bidirektionale Kommunikation über MUK – MUK | 2023-07-28 | Seit Juli 2023 Antworten aus dem Postfach 2.0, „sofern die Behörde … dies zulässt“ |

## Mein Unternehmenskonto: Dokumente für Behörden und Technik (service.mein-unternehmenskonto.de)

| Datei | URL | Titel / Herausgeber | Datum Dokument | Inhalt (kurz) |
|---|---|---|---|---|
| muk_postfach-2.0_integrationsleitfaden_v9_2026-04.pdf | https://service.mein-unternehmenskonto.de/api/downloads/integrationsleitfaden_postfach | Postfach 2.0 Baustein 4 – Integrationsleitfaden – BayLfSt | Stand 04/2026, Version 9.0 | Kernquelle: Voraussetzungen (NEZO, Org.-Zertifikat, ETR), Sonderfälle (ohne NEZO, ERiC statt ETR), REST-Felder je Datenart, Limits, Antwort, Status, Netze/IP, E4K, E-Mail-Vorlagen, Glossar |
| muk_beipackzettel_integration-ozgpp-und-postfach-2.0_v1_2026-03.pdf | https://service.mein-unternehmenskonto.de/api/downloads/beipackzettel_postfaecher | Beipackzettel – Integration von OZG-PLUS-Postfach und Postfach 2.0 für die Verwaltungsseite – BayLfSt | Stand 03/2026, Version 1 | Vereinheitlichte Verwaltungsschnittstelle: ein ETR-Weg für persönliches Postfach und Funktionspostfächer; Postfach-Handle bzw. FPF-ID + öffentlicher Schlüssel aus NEZO; `OZGPPNachricht` |
| muk_postfach-2.0_funktionale-highlights_v4_2026-04.pdf | https://service.mein-unternehmenskonto.de/api/downloads/bedienungsanleitung_postfach | Postfach 2.0 – Funktionale Highlights – BayLfSt | Stand 04/2026, Version 4 | Versand über ETR oder ERiC; 100 Anhänge, 2 GiB/Anhang, 5 GiB/Nachricht; Virenprüfung; Antwort als PDF; Bekanntgabe- und Lesestatus; Duplikatskontrolle; Vorhaltefristen |
| muk_postfach-2.0_infosheet_2026-04.pdf | https://service.mein-unternehmenskonto.de/api/downloads/infosheet_postfach | Info Sheet Postfach 2.0 – BayLfSt | PDF 2026-04-22 | Funktionsliste, Roadmap der Anbindung in 6 Schritten, Links |
| muk_postfach-2.0_bedienung-featureuebersicht_v1_2023-11.pdf | https://info.mein-unternehmenskonto.de/wp-content/uploads/2023/11/Postfach-2.0-Bedienung-und-Featureuebersicht-Postfach-2.0-1.0_komprimiert.pdf | Postfach 2.0 – Bedienung und Featureübersicht – BayLfSt | Stand 11/2023, Version 01 | [veraltet] Alte Limits (50 MiB/Anhang, 250 MiB/Nachricht, 50 PDFs, 10 Antwortlinks); Duplikat-ID aus PDF-Hash; Löschregeln; in `Referenzen-ELSTER.md` als P20 zitiert |
| muk_nezo-integrationsleitfaden_v366_2026-09.pdf | https://service.mein-unternehmenskonto.de/api/downloads/nezo_integrationsleitfaden | NEZO Integrationsleitfaden – BayLfSt | Stand 09/2026, Version 366 | SAML-IdP; NameID `ek-…` = pseudonymisierte Konto-ID je SP = Postfachhandle; DatenuebermittlerPseudonymId `du-…`; Attribut Funktionspostfächer; Bausteinpseudonyme (JWT, signiert und verschlüsselt) |
| muk_ssp-quick-start-guide_v1.3_2024-12.pdf | https://service.mein-unternehmenskonto.de/api/downloads/ssp_quick_start_guide | Self Service Portal – Quick Start Guide – BayLfSt | Stand 12/2024, Version 1.3 | Vorhaben, Vollmacht für Dienstleister, Postfachrechte, Service Provider, Ticket-Kategorien |
| muk_infosheet_erste-schritte-anbindung_2023-01.pdf | https://service.mein-unternehmenskonto.de/api/downloads/infosheetanbindungsschritte | Erste Schritte zur Anbindung der Schnittstellen von Mein UK – MUK | PDF 2023-01-31 | [veraltet?] Ablauf SSP → E4K → Postfachrechte + ETR → Go-Live; OZGPP-Pilot mit Java-SDK |
| muk_infosheet_einbindung-durch-behoerden_2026-09.pdf | https://service.mein-unternehmenskonto.de/api/downloads/muk_infosheet | Mein Unternehmenskonto – Einbindung durch Behörden – MUK | PDF 2026-09-03 | Schnittstellen NEZO, Postfachschnittstelle (Postfach 2.0 und Funktionspostfächer), Berechtigungssteuerung; IT-Dienstleister mit einfacher Vollmacht |
| muk_ssp_wechsel-vorhabensverantwortlicher_2024-02.pdf | https://service.mein-unternehmenskonto.de/api/downloads/vv_wechsel | Wechsel des Vorhabensverantwortlichen – MUK | PDF 2024-02-21 | Ablauf beim Wechsel des VV im SSP |
| muk_zertifikatsleitfaden_2026-11.pdf | https://service.mein-unternehmenskonto.de/api/downloads/zertifikatsleitfaden | Leitfaden zur Verwendung & Verwaltung von ELSTER-Organisationszertifikaten im Rahmen von MUK – BayLfSt | Stand 11/2026, Version 3.0 | 1:1 Zertifikat ↔ Mitarbeiterpostfach; Verlust des Postfachs bei Neuregistrierung |
| muk_zertifikatsleitfaden_2024-03.pdf | https://info.mein-unternehmenskonto.de/wp-content/uploads/2024/03/20240313_Zertifikatsleitfaden.pdf | Zertifikatsleitfaden – BayLfSt | 2024-03-13 | [veraltet] Vorgängerfassung, in `Referenzen-ELSTER.md` als ZERT-LF zitiert |
| muk_berechtigungssteuerung_quickstart-anbieter-vwl_v1.0_2025-04.pdf | https://service.mein-unternehmenskonto.de/api/downloads/quick_guide_anbieter_vwl | Quickstart Guide für Anbieter von Verwaltungsleistungen (Behörden) – Dataport i. A. Bremen | V1.0, PDF 2025-04-16 | Registrierung von Online-Diensten in der Berechtigungssteuerung |
| muk_berechtigungssteuerung_quickstart-betreiber-onlinedienste_v1.1_2025-04.pdf | https://service.mein-unternehmenskonto.de/api/downloads/quickstart_guide_betreiber_onlinedienste | Quickstart Guide der Berechtigungssteuerung für technische Betreiber von Online-Diensten – Dataport i. A. Bremen | Stand 02/2025, V1.1 | OAuth-Clients, Integrations- und Produktionsumgebung |

## ELSTER (elster.de, download.elster.de)

| Datei | URL | Titel / Herausgeber | Datum Dokument | Inhalt (kurz) |
|---|---|---|---|---|
| elster_verwaltung_elster-transfer.md | https://www.elster.de/elsterweb/infoseite/verwaltung | ELSTER-Transfer: Datenaustausch und Bereitstellung von Dokumenten – BayLfSt | Stand 2026-11 (ETR 26.09 vom 24.09.2026) | Voraussetzungen; nur Zertifikatsdatei; Postfach-Versand nur mit der ETR-Anwendung, nicht über Mein ELSTER; REST-API, Statusrückmeldung, Abholen von Antworten |
| elster_elstertransfer-einfuehrung.md | https://www.elster.de/elsterweb/infoseite/elstertransfer_hilfe_einfuehrung | Was ist ELSTER-Transfer (ETR)? – BayLfSt | o. D. | Rückkanal-Schnittstelle seit April 2021 (ETR 2.0.0); Adressierung über AccountID bzw. pseudonymisierte AccountID; AGS „geplant“; Bekanntgabe nach § 122a AO / § 41 Abs. 2a VwVfG; Architektur |
| elster_elstertransfer-faq.md | https://www.elster.de/elsterweb/infoseite/elstertransfer_hilfe_faq | ELSTER-Transfer – Häufig gestellte Fragen – BayLfSt | o. D. | Konfigurationstest, NdB ohne Update-Server, Zertifikatsberechtigungen |
| elster_download-elster-transfer_lizenzvertrag.md | https://www.elster.de/elsterweb/infoseite/download_elster-transfer | Lizenzvertrag ELSTER-Transfer – BayLfSt | Stand 2026-11 | Abkündigungen (CLI und BDatei-Import ab 26.01); unentgeltliche Überlassung; Rechtsgrundlagen §§ 8, 9 OZG. OSS-Lizenztexte gekürzt |
| elster_digitaler-gewerbesteuerbescheid.md | https://www.elster.de/elsterweb/infoseite/digitaler_gewerbesteuerbescheid | Digitaler Gewerbesteuerbescheid – BayLfSt | o. D. | Zustellwunsch seit April 2023; Bekanntgabe im MUK-Postfach; Nachnutzung „gebührenfrei“ |
| elster_modulares-unternehmenskonto_2020.pdf | https://download.elster.de/download/dokumente/Modulares_Unternehmenskonto_auf_der_Basis_von_ELSTER.pdf | Modulares Unternehmenskonto auf der Basis von ELSTER – BayLfSt | PDF 2020-06-02 | [veraltet?] Bausteine 1–3; Postfach 2.0 inkl. M2M; „Anbindung der Kommunen und Behörden über ELSTER-Transfer“ |
| elster_information-kommunen_digitaler-gewst-bescheid_2023-03.pdf | https://download.elster.de/download/elstertransfer/dokumente/20230330_Information-Kommunen.pdf | Information zum Projektstand für Kommunen – Themenfeld Steuern & Zoll | 2023-03/04 | Digitaler GewSt-Bescheid als PDF/A-3 mit eingebettetem XML; 11.000 Kommunen an ETR anzubinden |

## Dritte: Governikus (OZG-PLUS-Postfach), Länder

| Datei | URL | Titel / Herausgeber | Datum Dokument | Inhalt (kurz) |
|---|---|---|---|---|
| governikus_mein-unternehmenskonto_funktionspostfaecher.md | https://www.governikus.de/loesungen/kundenprojekte/mein-unternehmenskonto/ | Mein Unternehmenskonto – Funktionspostfächer (OZG-PLUS-Postfach) – Governikus | Stand 2026-11 | E2E-Verschlüsselung (Fachverfahren ↔ einzelne Person, privater Schlüssel nur im Browser); keine serverseitige Virenprüfung; Limits; Pflicht zur Anbindung beider Postfächer; Timeline; FAQ |
| governikus_ozgpp_integrationsleitfaden_v1.11_2026-04.pdf | https://www.governikus.de/wp-content/uploads/2026/04/Integrationsleitfaden_OZGPP_v1_11.pdf | Integrationsleitfaden zur Anbindung des OZG-PLUS-Postfachs an bestehende Fachverfahren – Governikus | v1.11, 2026-04-01 | Direkte Anbindung: Lookup per Bausteinpseudonym, PRE-Public-Key, Java-SDK, XHE-Payloads (XMLENC, AES-256-GCM, ECIES), API-Key, Status, Umgebungen |
| governikus_ozgpp_releasenotes_1.8.0_2026-03.pdf | https://www.governikus.de/wp-content/uploads/2026/03/OZG-PLUS-Postfach-ReleaseNotes_1_8_0.pdf | Release Notes OZG-PLUS-Postfach 1.8.0 – Governikus | 03/2026 | Vereinheitlichte Verwaltungsschnittstelle; empfangbare Anhänge bis 2 GB |
| governikus_ozgpp_sdk-releasenotes_3.0.0_2026-05.pdf | https://www.governikus.de/wp-content/uploads/2026/05/OZG-PLUS-Postfach-SDK-ReleaseNotes_3_0_0.pdf | Release Notes OZG-PLUS-Postfach SDK 3.0.0 – Governikus | 05/2026 | Umbenennung von Builder-Methoden; Fix der URL-Validierung für Antwortlinks |
| openapi_ozgpp-elster-transfer-3.3.0.yaml | https://www.governikus.de/wp-content/uploads/2024/08/ozgpp-elster-transfer-3.3.0-rest-api-docs.yaml | Transfer REST-Schnittstelle für OZGPP auf reduzierter Basis von ELSTER-Transfer – Governikus | 3.3.0 (Upload 2024-08) | `file-upload`, `bereitstellungsauftrag/v3`, Status; `X-API-KEY`; viele ETR-Felder „Wird nicht unterstützt“ |
| openapi_ozgpp-lookup-1.0.0.yaml | https://www.governikus.de/wp-content/uploads/2026/01/lookup-1.0.0-rest-api-docs.yaml | OZGPP Lookup REST-Schnittstelle – Governikus | 1.0.0 (Upload 2026-01) | `POST /rest/mailboxes` mit Bausteinpseudonym (JWT); `GET /rest/mailboxes/{id}/encryption-key` |
| openapi_ozgpp-fv-base-response.yaml | https://www.governikus.de/wp-content/uploads/2023/07/fv-base-response.yaml | OZGPP Fachverfahren Service API (ErrorResponse) – Governikus | 1.0.0 | Von beiden Specs referenziertes Fehlerschema |
| itvsh_handreichung-registrierung-zentrale-stelle_v1.2_2025-09.pdf | https://itvsh.de/fileadmin/user_upload/Dokumente/DiPlan_SH/Handreichung_zum_Registrierungsprozess_von_ELSTER_zentrale_Stelle_1.2.pdf | Handreichung zum Registrierungsprozess von ELSTER-Organisationszertifikaten im Rahmen von MUK – Modell „zentrale Stelle“ – ITVSH | v1.2, Stand 2025-09-23 | Behörden als MUK-Nutzer: Steuernummer, Ordnungskriterium des FA Neubrandenburg, zentrale Zertifikatsverwaltung |
| st_ozg-sprechstunde_infoveranstaltung-uk-behoerden_2022-09.pdf | https://ozg.sachsen-anhalt.de/fileadmin/Bibliothek/Politik_und_Verwaltung/MF/OZG/Bilder/Veranstaltungen/OZG-Sprechstunden/25._InfoveranstaltungUK_Behoerden_aktuell.pdf | Infoveranstaltung Unternehmenskonto für Behörden (OZG-Sprechstunde Sachsen-Anhalt, Folien BayLfSt/Bremen) | PDF 2022-09-22 | [veraltet] Damaliger Stand: 250 MB, keine Funktionspostfächer, kein Push an Behörden; Kosten der Anbindung trägt Behörde; kein ELSTER-Support für Fachverfahren |

## Nicht abrufbar / nicht gespeichert

- ELSTER-Transfer-Handbücher (Windows/Linux/Docker, Fortgeschrittene Konfiguration), „Howto: ELSTER-Transfer REST-API“, „Schnittstellen von ELSTER-Transfer“ und die OpenAPI-Dateien `elster-transfer-x.x.x-rest-api-docs.json/.yaml`: nur nach Bestätigung des Endbenutzer-Lizenzvertrags (POST-Formular `/elsterweb/lizenzvertrag/lizenzvertrag_elster-transfer`) erreichbar. Der Lizenzvertrag wurde nicht im Namen der Nutzer akzeptiert; die Dateien fehlen daher. Die reduzierte OZGPP-Variante der ETR-API (3.3.0) liegt vor.
- `https://www.elster.de/elsterweb/infoseite/elstertransfer_hilfe_schnittstellen` (im FIT-Connect-Schema `reply-channel-1.0.0` verlinkt): „Seite nicht gefunden“. `…/elstertransfer_hilfe_rest-api` (in der Blaupause verlinkt): zeigt nur noch die Hinweisseite „Hilfe zu ELSTER-Transfer bis Version 3.1.0“; Inhalte wurden laut ELSTER in die PDF-Handbücher des Downloadbereichs überführt.
- Integrationsleitfäden, Beipackzettel für E4K-Zertifikatsdateien und Downloadbereich im eingeloggten MUK Self Service Portal: Login mit ELSTER-Organisationszertifikat nötig.
- `https://download.elster.de/download/elstertransfer/dokumente/20230908_Blaupause.pdf` (Blaupause digitaler Gewerbesteuerbescheid, 48 S.): abgerufen, aber reine Bild-PDF ohne Textebene; nicht gespeichert.
- `https://www.governikus.de/wp-content/uploads/2025/02/Integrationsleitfaden_OZGPP_v1.7.pdf` (Link in der Governikus-FAQ): 404; stattdessen v1.11 gespeichert.
- `https://www.governikus.de/mein-uk/` (Link im Infosheet 2023): 404, Inhalt jetzt unter der Kundenprojekte-Seite.
- `https://git.fitko.de/fit-connect/planning/-/issues/1326` (Issue „Pattern im accountID im replyChannel-Objekt rausnehmen (Bugfix entsprechend Elster)“, aus der Websuche): 404 ohne Login. Laut Suchtreffer-Snippet liefert ELSTER 43-stellige alphanumerische IDs statt der im Schema erwarteten 10 Ziffern [unverifiziert].
- `https://ozg.sachsen-anhalt.de/…/Leitfaden_Organisationskonto_V1.0_49.Sprechstunde.pdf`: TLS-Fehler beim Abruf.
- Länder-Postfachdienste (z. B. M-V, Sachsen, Dataport) als Vermittler zum MUK-Postfach: keine öffentliche Quelle gefunden.
- Seite `https://info.mein-unternehmenskonto.de/behoerden/`: nur Platzhaltertext („Lorem ipsum“), nicht gespeichert.
