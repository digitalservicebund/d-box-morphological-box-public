# Befunde: ZaPuK – Umsetzung, Pilotierung (Neo) und Rezeption

Stand der Recherche: 2026-11-19, Nachtrag 2026-11-27 (ZaPuK v1.1.1, B-2026/43-IT). Schwerpunkt: Ergebnisse und Abschluss von Neo, öffentlich dokumentierte Transitionspläne der Bestandslösungen, Rezeption in Presse und Fachöffentlichkeit. Technische ADR-Details klärt `../architektur-technik/`, Beschlüsse, Recht und Finanzierung `../recht-governance/`.

Wichtiger Stand-Hinweis (aktualisiert 2026-11-27): Der IT-Planungsrat hat in der 51. Sitzung am 25.11.2026 mit B-2026/43-IT nur festgestellt, dass Phase II abgeschlossen ist, und der Bund hat die Programmverantwortung bestätigt (B43 Z.10–12). Die Übergreifende Transitionsstrategie (TS) ist Anlage zum Beschluss und liegt in v1.1.1 vor; beschlossen ist sie nicht, sie heißt hier **v1.1**. Die Transitionspläne TA, TB, TC und die TADRs TE1, TE2 sind nicht in v1.1.1 und nicht in den Beschluss-Anlagen enthalten; sie bleiben **[Entwurf]** (Branch `architecture-v1.1`, Commit vom 2026-11-13, Branch upstream gelöscht). AK11 (Entwurfskopie) wertet `../architektur-technik/` gegen v1.1.1 aus. Früher hieß es, v1.1 solle in der 51. Sitzung „verabschiedet“ werden (`../zapuk-v1.0.0-121-g7419d8b/README.md` Z.49); v1.1.1 sagt nur noch „vorgelegt“ (ZK-README Z.49).

Kürzel der Fundstellen (Dateien in diesem Ordner, Z. = Zeile der gespeicherten Datei, S. = PDF-Seite, # = Issue-Nummer):

Neo, Primärquellen
- VS = fitko_neo_vorhabensseite_2026-07.md
- IL = fitko_neo_integrationsleitfaden_2026-07-23.md
- VA = fitko_neo_fit-ab-vorhabensabschluss_2026-06-23.pdf
- SI = fitko_neo_stakeholder-informationstermin_2026-03-25.pdf
- SA30 = fit-ab_neo_sachstand-30-sitzung_2026-03-10.pdf
- KO = fitko_neo_stakeholder-kickoff_2025-08-26.pdf
- MC25 = fitko_neo_matrix-conference-slides_2025-10-16.pdf
- STB = fit-ab_neo_steckbrief_2023-06-05.pdf
- FAB = fit-ab_beschluesse-2026-11-und-2026-12_2026-09-15.md
- OC = fitko_neo_opencode-projektuebersicht_2026-11-19.md
- PI = fitko_neo_planning-issues-auszug_2026-11-19.md (Angabe als PI #Nummer; Arbeitsstände, keine Beschlüsse)

Transition, Primärquellen ZaPuK
- TS = ../zapuk-v1.1.1/Zielarchitektur/Transitionen/README.md (v1.1, Anlage zu B-2026/43-IT; Zeilen seit 2026-11-27 nach dieser Datei; früher fitko_zapuk-transitionsstrategie_v1.1-entwurf_2026-11-13.md, Konkordanz unten)
- ABB1, ABB2 = ../zapuk-v1.1.1/Zielarchitektur/Transitionen/images/image1.png, image2.png (bytegleich mit fitko_zapuk-transitionsstrategie-abb1-phasen…png, …-abb2-muk-pfade…png)
- TA = fitko_zapuk-transition-a-zbp_v1.1-entwurf_2026-11-13.md [Entwurf, nicht in v1.1.1]
- TB = fitko_zapuk-transition-b-mjp_v1.1-entwurf_2026-11-13.md
- TC = fitko_zapuk-transition-c-muk_v1.1-entwurf_2026-11-13.md
- TE1, TE2 = fitko_zapuk-tadr-mein-elster-001 / -002_v1.1-entwurf_2026-11-13.md [Entwurf, nicht in v1.1.1]
- (TB, TC ebenso [Entwurf, nicht in v1.1.1])
- ZMC25 = fitko_zapuk_matrix-conference-slides_2025-10-16.pdf
- OZG65 = sachsen-anhalt_zapuk-neo_ozg-sprechstunde-65_2025-10.pdf
- KON = fitko_zapuk_konsultation-phase2-stellungnahmen_2026-02-bis-04.md

Rezeption (alle Sekundärquellen außer BMDS und HIB)
- HE25 = heise_matrix-statt-mjp-zbp_2025-06-17.md
- CT25 = heise-ct_neuer-anlauf-nach-de-mail_2025-06-26.md
- NP25 = netzpolitik_ag-empfiehlt-matrix_2025-07-01.md
- HEMC = heise_matrix-conference-2025_2025-10-17.md
- EG26 = egovernment_noots-zapuk-krankenkassen_2026-02-24.md
- HEDA = heise_deutschland-app-sap-telekom-wire_2026-04-07.md
- HE26 = heise_dstack-verbindlichkeit-basiskomponenten_2026-06-18.md
- NP26 = netzpolitik_dstack-schnittstellen-chaos_2026-06-25.md
- EL26 = element_zapuk-deutschland-stack_2026-06-30.md (Eigeninteresse)
- EGD = egovernment_dstack-andocken-databund_2026-08-07.md
- TWIM = matrix-org_twim-neo-open-source-wettbewerb_2026-08-28.md
- SET = set_zapuk-konsultationsbeitrag_2026-09-03.md (Eigeninteresse)
- MCW = matrix-org_conference-2026-wrap-up_2026-10-30.md
- OM = onlinemarktplatz_deutschland-app-verzoegerung_2026-11-01.md
- D64 = d64_dstack-dossier_2026-11-04.md
- HIB = bundestag-hib_dstack-basiskomponenten_2026-11-11.md (Parlamentsnachricht)
- BMDS = bmds_bundid-themenseite_2026-11.md (Primärquelle)

Externe Verweise (nicht in diesem Ordner)
- ZK-README = ../zapuk-v1.1.1/README.md (Zeilen seit 2026-11-27 nach dieser Datei)
- AK10 = ../zapuk-v1.1.1/Zielarchitektur/Architekturkonzept/Vorversionen/v1.0/README.md (v1.0; hier nur nach Kapiteln zitiert)
- B43 = ../recht-governance/IT-PLR_B-2026-43-IT_ZaPuK-Abschluss-Phase-II.md; PR43 = ../recht-governance/IT-PLR_Beschluss_2026_43_ZaPuK_Praesentation.pdf (Foliensatz zur 51. Sitzung)
- AK11 = ../architektur-technik/fitko_zapuk-architekturkonzept-v1.1_v1.1-entwurf_2026-11-13.md (Zeilen dieser Datei)
- NEO = ../../../neo-solution-architecture-develop/ (arc42-Snapshot)
- FSN = ../../../BundID-ZBP/quellen/fitko-fit-connect/fitko_stellt-vor_neo_2026-01-23_aktualisiert.pdf

## Überblick: Neo als Pilot

- **Auftrag und Träger:** Vorhaben des FIT-AB, umgesetzt von FITKO und BMDS (VS Z.172). Beschluss des FIT-AB vom 05.06.2023 (VS Z.132, STB). Neo erweitert die BundID „auf Grundlage des bestehenden rechtlichen Rahmens (§ 9 OZG i.V.m. § 8 Abs. 5 OZG)“ (VS Z.22). Bestandteile: Matrix-fähige Erweiterung der BundID-Weboberfläche, Smartphone-App, Tooling für Fachbehörden (VS Z.23).
- **Vergabe und Laufzeit:** Zuschlag am 03.06.2025 an adesso, Element und Nordeck (VS Z.117). Umsetzungsstart Juli 2025, Prototyp im Juni 2026 fertig (VA S.7). Laut SI S.14 endete das Vergabeverfahren bereits im „April 2025“ (Widerspruch, siehe unten). Wartungsvertrag laut publiccode.yml bis 2027-06-03 (OC). Vertragsoption „zur Weiterentwicklung des MVP und Vorbereitung einer möglichen Produktivsetzung“ (VA S.55).
- **Ergebnisse (VA S.52):** drei Matrix-Clients mit E2EE (iOS-App, Web-Client, FIT-Connect-SDK), Entkopplung von Authentifizierung und Kommunikation, AusweisApp-SDK direkt in der iOS-App, Synchronisation zwischen Endgeräten, Widerspruchsfunktion (MVP), fünf generische Nachrichtentypen, Dokumentation für Fachbehörden. Nutzertests mit 32 Personen, 122 User Requirements (VA S.16), 654 Anforderungen insgesamt (VA S.25). Beiträge zur Matrix-Spezifikation (MSC 4057, 4474 neu) (VA S.24).
- **Nicht erreicht (VA S.53):** geräteübergreifender Login (Device Switch), Terminvereinbarung, 115, Payments, vollständiger Betreiberausschluss bei Metadaten, Self-Service-Registrierung von Behörden, EUDI-Wallet als Anmeldemittel, Ausstellung von Nachweisen in die EUDI-Wallet, technische Einschränkung der Antwortfunktion durch Fachbehörden.
- **Pilotierung mit Behörden:** Zwei Live-Demos in der 31. FIT-AB-Sitzung mit GA-Lotse (Gesundheitsamt Frankfurt, Software der hessischen Gesundheitsämter) und BW-Empfangsclient (Baden-Württemberg) (VS Z.45, VA S.18). GA-Lotse war nicht an FIT-Connect angebunden, BW-EC vollständig (SA30 S.28–29). **[Folgerung]** Es handelte sich um Demo- und Testanbindungen in der DEV-Umgebung (IL Z.548–557), nicht um einen Wirkbetrieb mit echten Bürger:innen. Ein Produktivpilot ist nicht dokumentiert.
- **Abschluss:** FIT-AB 2026/11 vom 23.06.2026: technische Realisierbarkeit „als erbracht“ angesehen, Vorhaben abgeschlossen, Team unterstützt ZaPuK in AP 2.1 und 2.5, Bund soll Nutzung im D-Stack prüfen (FAB Z.35–42; ZK-README Z.104–109). Die Vorhabensseite datiert die Befassung auf den „25. Juli 2026“ (VS Z.79), Folien und Beschluss auf den 23.06.2026 (Widerspruch, wahrscheinlich Tippfehler).
- **Weiterarbeit nach dem Abschluss:** Die Repositories werden weiter aktiv entwickelt (Web-App, iOS-App, SDKs, Infrastruktur, letzte Commits 11/2026) (OC). Neu seit 09/2026: `osci-transport-rs` für die MJP/EGVP-Anbindung (OC; PI #717). Issues tragen das Label „Out of scope Phase IV“ (PI #553, #554) **[unverifiziert: Bedeutung von „Phase IV“ nicht erläutert]**.
- **Folgebeschluss:** FIT-AB 2026/12 vom 15.09.2026 empfiehlt Phase III im D-Stack „auf Basis der Arbeitsergebnisse aus Phase I und II sowie dem Neo-Vorhaben“: Frontend im Bundesportal und in der D-App, getrennte Backends für Privatpersonen und Organisationen in unabhängigen Betriebsorganisationen, FIT-Connect-SSP als Rollout-Infrastruktur, Anpassung von BundID, MJP, MUK und ELSTER nach den Transitionsplänen (FAB Z.12–34).
- **Öffentlichkeitsarbeit:** FIT-AB 2023 (Ideenskizze), Matrix Conference 2024 und 2025, ISB-Jahrestagung 07.10.2025, SCCON 02.10.2025, „Die FITKO stellt vor“ 23.01.2026, FIT-AB 10.03.2026, Stakeholdertermin 25.03.2026 (VS Logbuch). Über 750 Teilnehmende im Format „Die FITKO stellt vor“ (VA S.27). Nominierung für den Open-Source-Wettbewerb 2026 (TWIM Z.13). Auf der Matrix Conference 2026 kein eigener Neo-Vortrag, FITKO nur im Public-Sector-Panel (MCW Kopf, Z.15). Ein Zukunftskongress-Beitrag zu Neo wurde nicht gefunden; zur ZaPuK gibt es Folien vom Zukunftskongress 2025 in `../zapuk-v1.1.1/Foliensätze/`.

## Befunde je Zeile

### B1 Anbindung von Fachverfahren

- **Neo:** Anbindung über ein Matrix-Modul im FIT-Connect-SDK, zunächst nur für das Java-SDK (IL Z.525–527). Das Modul funktioniert auch ohne Antragstellung über FIT-Connect (IL Z.530). Das SDK nutzt dieselbe Matrix-Rust-SDK-Codebasis wie App und Web-Client (VA S.30). Aufwand für Hersteller „sehr gering, insb. wenn FIT-Connect SDK bereits im Einsatz ist“ (VA S.17), Selbstaussage des Projekts.
- **ZaPuK Phase III:** Fachverfahren werden „sukzessive über FIT-Connect angebunden“ (TS Z.34). Für das ZBP bleibt die ZBP-API über eine ZBP-Matrix-Bridge als „Legacy-Fassade“ für eine Übergangszeit bestehen, Neuregistrierungen sind dann nicht mehr möglich (TA Z.120–122). Der FIT-Connect-ZBP-Adapter bleibt in Phase C nutzbar (TA Z.120). MUK: Umstellung von ELSTER-Transfer/ERiC und OZG-PP-REST-API auf Matrix-Schnittstellen (TC Z.77).
- **Justiz:** Gerichte bleiben ohne Anpassung bei EGVP. Der OSCI-Matrix-Protokollkonverter verbindet beide Welten (TB Z.40).
- **Kritik:** Dataport fordert API-first statt SDK-first und M2M-Identitäten für Unternehmen (KON #24, #25). Hersteller verlangen schon heute fünfstellige Lizenzbeträge je Instanz für FIT-Connect-Adapter (NP26 Z.39), **[Folgerung]** ein Kostenrisiko auch für die ZaPuK-Umstellung.

### B2 Zugang ohne Fachverfahren

- Neo pilotierte mit dem BW-Empfangsclient, der Kommunen ohne Fachanwendung den Empfang digitaler Anträge ermöglicht (SI S.33). **[Folgerung]** Damit ist die Zeile „Client-Anwendung für Behörden ohne Fachverfahren“ für ZaPuK belegt, aber nur als Demo.

### B3 Adressierung der Person / Organisation

- **Neo:** Der Onlinedienst erhält beim BundID-Login das postkorbHandle. Es wird über FIT-Connect an die Fachbehörde übermittelt. Die Matrix-ID lautet `@<PostkorbHandle>:ess1.dev.neo.fitko.dev` (IL Z.508–518). Neo spricht von einem „pseudonymisierten Rückkanal“ (IL Z.522).
- **ZaPuK-Transition ZBP:** Das BundID-Postfachhandle soll als localpart der MXID übernommen werden. „Mittelfristig“ folgen bereichsspezifische Pseudonyme nach ADR-1105 (TA Z.48). Bei Anmeldung mit EUDI-Wallet wird eine Postfachadresse im Identitätsnachweis mitgeliefert (TA Z.50).
- **MJP:** Adressen im Format `@[SAFE-ID]:matrix.example.com`, ohne zusätzliches Verzeichnis (TB Z.42). Im Neo-Epic heißt es `@<SAFE_ID>:matrix.justiz.de` (PI #715).
- **MUK:** Adressregister und Abbildung der Bestandsadressen sind eine eigene Maßnahme (TC Z.75).
- **[Folgerung]** In der Transition gilt zunächst „Postfachreferenz aus dem Antrag“ (morph_box B3 Ausprägung 1). Die pseudonyme Bereichsadresse (Ausprägung 3) ist ausdrücklich ein späterer Schritt.

### B4 Inhalt an die Person / Organisation

- Neo-Nachrichtentypen: Informativ, Freie Antwort, Einfachauswahl, Mehrfachauswahl, Datei-Upload. „Außerdem: Terminauswahl, einfache Formulare etc.“ (VA S.58, SI S.21). Status auf einer Zeitachse und Weiterleitungslink zum Onlinedienst (IL Funktionsübersicht; SI S.45). Die Formate basieren auf MSC1767 Extensible Events (VA S.22).
- Terminvereinbarung ist nicht umgesetzt (VA S.53). Das Epic ist offen (PI-Suche: #441).
- ZaPuK-Transition: Die Statusmonitor-Funktion des ZBP bleibt erhalten und wird migriert (TA Z.38, Z.129).
- Bei der Übernahme von ZBP-Nachrichten wird HTML auf die Matrix-Tagliste reduziert (PI #756, #766). **[Folgerung]** Die Darstellung von Bestandsnachrichten kann sich ändern.

### P1 Benachrichtigung

- **Neo:** Laut Integrationsleitfaden: App per Push (IL Z.77), Web per E-Mail, unter anderem bei Bekanntgabe von Bescheiden (IL Z.292–294). Ebenso SI S.45 und SA30 S.42.
- **Umsetzungsstand:** Push in der iOS-App ist ein offenes Issue (Prio „Low“, „Refined“) (PI #464). Das Benachrichtigungskonzept (Push und E-Mail) ist offen (PI #105, #229). Push fehlt in der Liste „Was haben wir erreicht?“ (VA S.52). **[unverifiziert]** Ob Push im Pilot tatsächlich funktionierte.
- Push-Server von Google und Apple dürfen keine Klartextinhalte sehen (Anforderung ZaPuK_128, PI #464, #105).
- **ZaPuK-Transition:** E-Mail-Links verweisen nach der Umstellung direkt auf das Bundesportal (TA Z.131). ELSTER: Hinweise in der jeweils anderen Lösung nur als „Benachrichtigung ohne Inhalt“ (TE1 Z.67). Mobile ZaPuK-Clients sollen Push erleichtern (TE2 Z.34).

### P2 Authentifizierung

- **Neo:** BundID-IdP über Keycloak und Matrix Authentication Service, für Nutzende unsichtbar (SA30 S.23). Der Login per BundID (Benutzername/Passwort, ELSTER, eID) ist Voraussetzung für alle Integrationsoptionen. Die EUDI-Wallet ist „perspektivisch“ (VA S.36). In der iOS-App ist das AusweisApp-SDK direkt integriert (VA S.52). Ein eID-Login in der App ist umgesetzt (PI #225, Akzeptanzkriterien abgehakt). Proof of Concept `bundid-eid-proxy` blendet die BundID-Oberfläche aus, „nicht für Produktion“ (OC).
- **Zertifikate:** Behörden werden automatisch per Zertifikat abgesichert (SI S.25). Für Behörden X.509, für Bürger:innen eID (MC25 S.23). Geräteverifikation per eID ist offen (PI #186).
- **EUDI-Wallet:** In Neo nicht erreicht (VA S.53). Epic „Out of scope Phase IV“ (PI #554). In der ZBP-Transition: Identifizierung über BundID-SAML (F1) und Ausstellung von Attestations (F2) als Maßnahme B11 (TA Z.108). Zentrale Anbindung über die BundID oder dezentral über den Wallet-Adapter (TA Z.44). Die BundID bietet d-you ab Januar 2027 als Anmeldeoption an (BMDS Z.23, Z.39).
- **MUK:** bleibt Identitätsanbieter für Organisationen (TC Z.38). Org-IdP-Adapter erlaubt eigene IdPs von Organisationen (TC Z.76).

### P3 Organisationszugang

- Neo betraf nur Privatpersonen (VA S.55). Organisationsanwendungsfälle wurden nach dem Abschluss als Validierungsauftrag weitergeführt (VA S.48, PI #691).
- MUK-Transition: Funktionspostfächer, Mitgliedschaften und Vertretungen werden in ZaPuK abgebildet. Die Organisationen müssen die Rechte selbst neu konfigurieren (TC Z.86, Z.92, Z.103).

### P4 Vertretung

- MUK-Transition: Ende-zu-Ende-verschlüsselte Kommunikation „mit Vertretungsmodell und Funktionspostfächern“ (TC Z.40). Für Privatpersonen ist keine Vertretung dokumentiert. Konsultation: Amtsvormünder fehlen als Nutzergruppe (KON #29).

### P5 Clients

- **Neo:** iOS-App und Web-Client mit gemeinsamer Rust-Codebasis. Der Web-Client ist modular und „prinzipiell in verschiedenste Webanwendungen integrierbar“ (VA S.15). Die iOS-App gab es nur über TestFlight. Eine Bereitstellung im App Store „zu Demonstrationszwecken“ war angekündigt (IL Z.60). Kein Android-Client, Android „perspektivisch“ (VA S.48).
- **Kein eigenes Produkt:** „Es ist kein neues Produkt geplant, sondern Verbesserungen bestehender Produkte“ (VA S.20). Die Artefakte „sind nicht als eigenständige Produkte zu verstehen“ (VS Z.29).
- **Integrationsoptionen:** BundID, eigenständige iOS-App, Bundesportal, Deutschland-App, Bayern-App, BA-App, „beliebig kombinierbar“ (VA S.35–36).
- **Neue Richtung:** Statt der BundID-Oberfläche wird das Web-Design auf die Bundesportal-CI umgestellt (PI #812, in Arbeit seit 10/2026).
- **ZaPuK Phase III:** Ein zentraler Postfach-Client im Bundesportal und „perspektivisch“ in der D-App (TS Z.32, Z.83). Er entsteht auf Basis der Neo-Ergebnisse für Web, iOS, Android und Linux Mobile, zum Beispiel Sailfish OS und Mobian (TA Z.98–101). Integration in Bundesportal (B9) und D-App (B10) (TA Z.106–107). Der Client ersetzt die Postfach-Frontends von BundID, MJP und MUK (TA Z.42, AK11 Z.423). Empfohlene Verortung im D-Stack-Baustein „Zentrales Nutzenden-Frontend“ mit DigitalService (AK11 Z.231–239). Die Verortung bestätigt FIT-AB 2026/12 Nr. 2a (FAB Z.18).
- **Deutschland-App:** Laut Handelsblatt (über heise) soll Schwarz Digits den Messenger **Wire** als sicheren Kanal zwischen Bürger:innen und Behörden in die D-App integrieren (HEDA Z.33). **[Folgerung]** Das wäre ein zweiter, nicht Matrix-basierter Kanal und widerspräche der ZaPuK-Linie. Eine Bestätigung durch das BMDS liegt nicht vor **[unverifiziert]**. Die D-App selbst ist verzögert: Der für Mitte September 2026 geplante Probebetrieb lief nicht an, ein Starttermin fehlt (OM Z.17, Z.31). **[Folgerung]** Die D-App-Integration ist abhängig von einem Produkt ohne Termin.
- **Kritik:** SET bemängelt, dass die Föderation im Frontend nur mit einem eigenen Client wirkt (SET Z.39). SET fragt, ob der Betreiber eines Browser-Frontends Klartext sehen kann und ob deshalb Justiz und Exekutive getrennte Frontends brauchen (KON #27).

### P6 Anbindung eigener Systeme

- Dataport vermisst M2M-Szenarien für Unternehmen (OAuth2 Client Credentials, mTLS, Service-Accounts, Funktionspostfächer ohne interaktive Anmeldung) (KON #24).

### P7 Antwort und Erstkontakt

- **Neo:** bidirektional im Vorgang. Die Behörde kann die Antwortfunktion abschalten oder auf maschinenlesbare Antworten beschränken, zum Beispiel nach dem Bescheid. Privatpersonen können sich nicht gegenseitig schreiben (IL Z.486–494). Die technische Einschränkung durch Fachbehörden ist laut Abschluss aber „noch nicht erreicht“ (VA S.53).
- **BundID heute:** Rückfragen und Nachreichungen über das ZBP „seit Juli 2026“ bei ausgewählten Leistungen (BMDS Z.31). ZaPuK v1.0 hatte diesen Schritt für entbehrlich erklärt (AK10 „Die ZBP-Verantwortlichen planen …“, Kap. 3.3). **[Folgerung]** Der Bund hat die Bidirektionalität im Bestand trotzdem umgesetzt. Sie muss in der Transition erhalten bleiben (TA Z.38).
- **MJP:** bidirektionale Kommunikation zwischen EGVP-Teilnehmenden und Privatpersonen über ZaPuK nach Phase C (TB Z.111).

### P8 Inhalt an die Behörde

- Neo: Freitext, Einfach- und Mehrfachauswahl, Datei-Upload (VA S.58; IL Funktionsübersicht). Für MJP-Nachrichten muss der Client XJustiz und Prüfvermerk berücksichtigen, ebenso einen ZIP-Download (PI #801).

### P9 Postfach und Nutzerkonto

- „Lose Kopplung von Postfach und Authentifizierung nach Deutschland-Architektur“ (VA S.34–36). In der Transition bleibt die BundID „als Authentifizierungskomponente“ bestehen, das Postfach wandert ins Bundesportal (TS Z.79; TA Z.44). MUK bleibt „Org-Konto (Authentifizierung und Berechtigungssteuerung)“ (TC Z.145). **[Folgerung]** Das stützt morph_box P9 Ausprägung 2 (ADR-1110).

### Ü1 Modus

- Neo: „Mail statt Chat“ als UX-Annahme, vorgangsbezogene Navigation (SI S.17). Technisch Matrix-Räume je Vorgang mit Synchronisation zwischen Geräten (VA S.52).

### Ü2 Vertraulichkeit

- **Neo:** E2EE bis zum Endgerät, Betreiberausschluss bei Inhalten. Bei Metadaten „Unobservability“ nach dem Vorbild von Signal Sealed Sender (VA S.23; SI S.26). Der vollständige Metadaten-Betreiberausschluss ist nicht erreicht (VA S.53). Neo verwendet Megolm (NEO 05_building_block_view.md Z.195), nicht MLS.
- **MLS:** Die MLS-Integration in Matrix ist noch nicht abgeschlossen, bis dahin dienen Olm/Megolm als Übergang (AK11 Z.779, Z.785; HE25 Z.41). Element: MLS setzt einen zentralen Delivery-Server voraus, die Dezentralisierung ist in Arbeit (EL26 Z.53).
- **MJP-Transition:** Der zentrale Client soll EGVP-Nachrichten mit den bestehenden .p12-Schlüsseln der Nutzenden ver- und entschlüsseln (TB Z.38). Offener Punkt: Wer stellt neue Zertifikate aus (TB Z.132)?
- **OZG-PLUS-Postfach:** keine serverseitige Migration (E2EE). Proxy-Re-Encryption-Beziehungen und Nachrichtenbestand werden nicht migriert (TC Z.86).
- **Postfach 2.0 und ELSTER:** Nachrichten liegen beim Eingang unverschlüsselt vor. Deshalb ist eine Bridge in der ELSTER-Domäne möglich, ohne dass ZaPuK-Server Klartext sehen (TC Z.119; TE2 Z.17, Z.55).

### Ü3 Echtheit

- Neo: Behörden-Absicherung per Zertifikat, „Ende-zu-Ende-Verschlüsselung erfordert Ende-zu-Ende-Authentisierung“ (SI S.24–25). Beitrag zur zertifikatsbasierten Authentisierung in der Matrix-Spezifikation ist noch offen (VA S.53).

### Ü4 Ereignisse und Bestätigungen

- Neo: Wie Zustellfiktion und Nachweise (ADR-0018, ADR-1118) mit Unverkettbarkeit und E2EE zusammengehen, ist ein offenes Issue mit Prio 0 (PI #303). **[Folgerung]** Zu Bereitstellungs-, Abruf- und Lesebestätigungen ist aus Neo kein umgesetzter Stand belegt.

### Ü5 Interoperabilität mit Systemen außerhalb

- **Protokollkonverter EGVP:** Für MJP wird ein OSCI-Matrix-Protokollkonverter als Teil des Justiz-Backends gebaut. Er wird von FIT-Connect betrieben und hält die E2EE aufrecht. Die verschlüsselten OSCI-Container werden in Matrix-Nachrichten verpackt (TB Z.40; TS Z.90; AK11 Z.632–634). Neo entwickelt dafür seit 09/2026 `osci-transport-rs` (OC). PoC: Eine OSCI-Nachricht wird transportentschlüsselt in einen Matrix-Raum gestellt, die innere Verschlüsselung bleibt intakt (PI #750, „In Progress“).
- **Begriffsunschärfe:** Das Issue heißt „MJP Protocol converter: Bridge Messages to Matrix Room“ (PI #750). Die Abgrenzung zur Bridge ist im Text aber sauber (PI #717).
- **Bridges für Bestandssysteme:** Leitprinzip 3: „Bridges kommen nur übergangsweise und an zulässigen Stellen zum Einsatz“ (TS Z.57). Konkret: ZBP-Matrix-Bridge (TA Z.102, Z.120, D1 Z.146), optionale Postfach-2.0-Matrix-Bridge (TC Z.42, Z.126) und ELSTER-Zustell-Bridge (TE2 Z.51, Z.75). Für ELSTER wurde der Protokollkonverter ausdrücklich verworfen (TE2 Z.45–47). **[Folgerung]** Gegenüber v1.0 („verbietet daher den Einsatz von Messaging Bridges“, AK10 Kap. 3.2) lockert der Entwurf v1.1 das Bridge-Verbot für die Übergangszeit an Stellen, an denen ohnehin Klartext beim Altsystem liegt.
- **Peppol:** Anfrage von Rheinland-Pfalz in der Konsultation (KON #23).

### Ü6 Malware-Prüfung

- Postfach 2.0: Im Zielbild entfällt der Klartextzugriff auf dem Server, mit „Auswirkung auf Malware-Prüfung“ (TC Z.119). Aus Neo gibt es keinen dokumentierten Umsetzungsstand.

### I1 Betrieb und Governance

- **Phase III:** Federführung des BMDS laut Überblick v1.1 und Foliensatz (PR43 S.20), im Beschluss nur „Programmverantwortung“ des Bundes (B43 Z.12); Phase II abgeschlossen. Projekt läuft bis zum Start von Phase III, voraussichtlich Anfang 2027 (ZK-README Z.49, Z.331; B43 Z.10–12).
- **Betreiber:** Backend für Privatpersonen beim Bund (BMDS), Betriebsumgebung offen: BundID-Umgebung oder ITZBund (TA Z.36, Z.104, Z.152). Backends für Organisationen und Justiz sowie Protokollkonverter „durch FIT-Connect betrieben“ (TA Z.52; TB Z.40; TC Z.40). „Voneinander unabhängige Betriebsorganisationen“ (FAB Z.20). Frontend im D-Stack beim Bund (AK11 Z.231).
- **Rollen in der Transition:** BundID-Team, Bundesportal-Team, D-App-Team (TA Z.85). MJP-Team mit Governikus und BLK/JuM BW (TB Z.69, Z.77). StMD Bayern, BayLfSt, FHB und FITKO-PM für MUK (TC Z.73).
- **Neo-Infrastruktur:** Element Server Suite (Synapse, MAS) und Keycloak (OC; IL Z.548–557). **[Folgerung]** Das ist ein Hinweis auf Element als technischen Partner, nicht auf den Betreiber in Phase III.
- **Vorbedingungen:** Bund soll Meilensteinplanung spätestens zur 52. IT-PLR-Sitzung vorlegen, Betrieb und Zuständigkeiten sind offen (HIB Z.17, Z.21). Finanzierungsmodell für Postfach bis März 2027 (D64 Z.71).

### I2 Datenhaltung und Versand

- Neo: zwei Homeserver, ess1 für Privatpersonen und ess2 für Fachbehörden (IL Z.548, Z.557). ZaPuK v1.1: eigene Backends für private Organisationen, öffentliche Stellen und Justiz, dazu das Backend für Privatpersonen (AK11 Z.336).
- Migration: ZBP-Bestandsdaten (Nachrichten, Statusmonitor, Metadaten) und MJP-Bestand gehen ins Backend für Privatpersonen (TA Z.129; TB Z.115). Kein Transfer bei OZG-PP, Read-only-Betrieb bis Fristende (TC Z.112). Postfach 2.0 Read-only (Bescheid 4 Jahre, Mitteilung 180 Tage) (TC Z.143). ELSTER-Bestand bleibt bei ELSTER (TE2 Z.54).
- Speicherdauer über die Bestandslösungen hinweg ist ungeklärt (PI #820).

### I3 Betriebsmodell für Organisationen

- MUK-Transition: Pilot mit ausgewählten Organisationen und Onlinediensten (TC Z.94). Org-IdP-Adapter (TC Z.76).

### I4 Funktionspostfächer und interne Bearbeitung

- OZG-PP-Funktionspostfächer werden nach ADR-1102 und ADR-1112 in ZaPuK neu angelegt (TC Z.92). Kein Neo-Umsetzungsstand.

### I5 Aufnahme in den Verbund

- Self-Service-Registrierung von Behörden ist in Neo nicht erreicht (VA S.53). In der Transition wird das BundID-SSP nur noch zur Authentifizierung genutzt (TA Z.52). Das FIT-Connect-SSP wird zur Rollout- und Anbindungsinfrastruktur (FAB Z.22).

### R1–R4 (nur Hinweise, Details bei `../recht-governance/`)

- R3: Neo nannte „Bescheidzustellung inkl. rechtssicherer Bekanntgabe“ als Funktion (SI S.45) und stützte sich auf § 9 i. V. m. § 8 Abs. 5 OZG (VS Z.22). Die Bekanntgabefiktion des ZBP soll in der Transition erhalten bleiben (TA Z.38). Rechtliche Nachweisfragen sind offen (PI #303).
- R2/R4: Eine Bereitstellung im ZaPuK-Postfach setzt ein aktives Postfach der empfangenden Person voraus, Fristen-Risiko bei aufgeteilten Postfächern (TE1 Z.68–69).

## Transition Bestandslösungen

Gemeinsamer Rahmen (TS v1.1, nicht beschlossen; Pläne TA–TE2 **[Entwurf]**; keine verbindlichen Termine, TS Z.49): Vier Phasen, A Projekt aufsetzen (3–6 Monate), B vorbereiten (6–8 Monate), C migrieren und umschalten (1–2 Monate), D Altsysteme abschalten („nach 24 Monaten“) (ABB1; TS Z.65). Keine Stichtagsmigration (TS Z.30). Die Mittel sollen schon in Phase II bereitgestellt werden (TA Z.81). Der Zeitplan der ZaPuK sah 2025 einen laufenden Betrieb „ab 2028“ vor (ZMC25 S.41; HE25 Z.43; EL26 Z.23). **[Folgerung]** Bei Phase III ab Anfang 2027 und A+B = 9–14 Monaten ist die erste Migration (ZBP) frühestens Ende 2027 oder 2028 zu erwarten.

| Bestandslösung | Geplanter Pfad | Technik der Übergangsphase | Bestandsdaten | Belege |
|---|---|---|---|---|
| BundID / ZBP | Ausgangspunkt der Transition. Backend wird durch das Matrix-Backend für Privatpersonen ersetzt, Frontend durch den Client im Bundesportal und später in der D-App. BundID bleibt Identitätsdienst. | ZBP-Matrix-Bridge hinter der BundID-WAF als „Legacy-Fassade“ für die ZBP-API. Abschaltung nach Umstellung der Fachverfahren (D1). FIT-Connect-ZBP-Adapter zunächst weiter nutzbar. | werden migriert (Nachrichten, Statusmonitor, Metadaten), danach temporärer Read-only-Betrieb | TS Z.34, Z.79–86; TA Z.38–52, Z.102–136, Z.146 |
| Mein Justizpostfach | Funktionale Ablösung durch das zentrale Frontend sowie die Backends für Justiz-Organisationen und Privatpersonen. Abschaltung von MJP-Frontend und -Backend nach stabilem Betrieb. | OSCI-Matrix-Protokollkonverter, E2EE mit den bestehenden .p12-Schlüsseln im neuen Client. EGVP bleibt unverändert. | sollen ins Backend für Privatpersonen migriert werden | TS Z.90–95; TB Z.36–42, Z.92–126; PI #715, #750, #798, #801 |
| MUK: OZG-PLUS-Postfach | Direkte Ablösung. Neuanlage von Postfächern, Schlüsseln, Funktionspostfächern und Rollen. | keine Bridge (E2EE), kein Protokollkonverter (Aufwand) | keine serverseitige Migration, Read-only und clientseitiger Export bis Fristende | TS Z.105; TC Z.44, Z.86–113; ABB2 |
| MUK: Postfach 2.0 | Stufen: (1) Anzeige mit Link, (2) optional Bridge, (3) Umschaltung neuer Nachrichten, (4) Rückbau. MUK bleibt Org-Konto. | optionale Postfach-2.0-Matrix-Bridge in der ELSTER-Domäne | Read-only (Bescheid 4 Jahre, Mitteilung 180 Tage) | TS Z.107; TC Z.42, Z.119–145; ABB2 |
| Mein ELSTER-Postfach | Dauerhafter Parallelbetrieb. Nur Bescheide und Dialog (z. B. EPBescheid, EPMitteilung, EPAntwort) gehen nach ZaPuK, Statusmeldungen und M2M bleiben bei ELSTER. | Stufe 1 Zustell-Bridge in der ELSTER-Domäne mit Routing je Datenart, Stufe 2 native Matrix-Zustellung über SDK. ELSTER bewertet Aufwand selbst. | bleiben bei ELSTER (nur mit ELSTER-Schlüsseln lesbar) | TS Z.111; TE1 Z.38–69; TE2 Z.49–81 |
| EGVP (Justiz) | bleibt in seinen Kernfunktionen bestehen. Weitere Verschmelzung wird dem E-Justice-Rat nicht vorweggenommen. | Protokollkonverter (siehe MJP) | – | TS Z.34; AK11 Z.343, Z.355; FAB Z.30 (Zusammenarbeit mit E-Justice-Rat) |
| BundesMessenger | kein Transitionsplan. v1.0: entweder als Instant Messenger erhalten und auf Protokollebene anschließen oder vollständig konsolidieren, „ist zu klären“. | – | – | AK10 Kap. 3.4; nicht im Geltungsbereich von TS Z.40–45 |
| TI-Messenger | kein Transitionsplan. v1.1 beschreibt nur Gemeinsamkeiten (Föderation, Funktionsaccounts, souveräner Betrieb) und Unterschiede (Identität, Nachweise, Pseudonyme, risikobasiertes Onboarding). | – | – | AK11 Z.880–893; FAB Z.28 (Zusammenarbeit mit dem Gesundheitswesen) |

Weitere Befunde zur Transition:

- **Neo als Vorarbeit:** Die meisten Software-Maßnahmen in Phase B haben die Voraussetzung „Nutzung Neo-Arbeitsergebnisse“ (TA Z.98–111; TB Z.92–100). Neo arbeitet bereits an ZBP-Migration und -Bridge (PI #17 mit Transitionsschritten 0–3, #120, #504). Ein einmaliger Import von ZBP-Nachrichten über die BundID-Frontend-API ist in Arbeit (PI #607).
- **Transitions-Steckbriefe** je BundID, MJP, MUK und ELSTER waren als Liefergegenstand zur 51. IT-PLR-Sitzung geplant (VA S.46). Vorgelegt wurden nur die Strategie mit Kurzfassungen je Lösung (TS Z.73–111) und eine Übersichtsfolie (PR43 S.13); die ausführlichen Pläne fehlen in den Anlagen und in v1.1.1. Ihr Abstimmungsstand mit den Betreibern ist nicht öffentlich. MUK: Der Plan ist ausdrücklich eine „ZaPuK-seitige Empfehlung“ (TC Z.30). ELSTER bewertet Aufwand und Ausgestaltung selbst (TE1 Z.62; TE2 Z.77).
- **Offene Punkte laut Plänen:** Betriebsumgebung und Mengengerüst der BundID (TA Z.152–153), Mengengerüst des MUK, widersprüchliche Anhangsgrößen beim OZG-PP (50 vs. 600 MB), Betreiber der Nachweisstelle (TC Z.151–153), Zertifikatsausstellung für die Justizkommunikation (TB Z.132).
- **Channel Adapter:** Der Begriff aus v1.0 (AK10 Kap. 3.2) kommt in TS, TA, TB, TC, TE1, TE2 und im v1.1-Konzept nicht mehr vor. **[Folgerung]** Er wurde faktisch durch „Bridge (übergangsweise)“, „Legacy-Fassade“ und „Verlinkung“ ersetzt.
- **Bisherige Planung der Bestandslösungen:** MJP: Die BLK prüft laut einer FragDenStaat-Antwort die Integration in die FITKO-Zielarchitektur, eine mobile Nutzung ist nicht geplant (siehe `../../../MJP/quellen/mjp-betreiber/_befunde.md`, Widerspruch 6 und 7). BundID: Bidirektionalität seit Juli 2026 (BMDS Z.31). Die ITSG (GKV) will ihre Arbeitgeber-Postfächer einbringen (EG26 Z.35).

## Rezeption und Kritik

- **Grundtenor der Fachpresse:** sachlich bis wohlwollend, mit Skepsis zur Umsetzbarkeit. Die c’t fragt „Durchbruch für die Digitalisierung oder ein neues De-Mail-Fiasko?“ (CT25 Z.13). Sie betont, dass Justiz und Finanzen politisch mitziehen müssen und Matrix noch kein MLS kann (HE25 Z.39–41). netzpolitik.org berichtet ausführlich und ohne Kritik, mit O-Tönen von FITKO und Sachsen-Anhalt (NP25). Behörden Spiegel und Tagesspiegel Background wurden nicht gefunden bzw. nicht zugänglich.
- **D-Stack-Kontext:** ZaPuK gehört zu den fünf Basiskomponenten, ist aber „noch in Entwicklung“, die Verbindlichkeit für die Länder ist unklar (HE26 Z.27; HIB Z.15–17). Agora Digitale Transformation fordert verbindliche Qualitätskriterien, „The BundID is a cautionary example“ (HE26 Z.35). D64: Finanzierungsmodell für Postfach fehlt, Lasten bei Kommunen (D64 Z.93).
- **Kosten:** Keine öffentliche Kostenschätzung, sie kommt laut FAQ erst zur 52. IT-PLR-Sitzung (ZK-README Z.317). Die Migrationskosten stehen in einem separaten Finanzierungskonzept (AK11 Z.1127, Fußnote 5). Hinweise aus dem Umfeld: fünfstellige Adapterkosten je Fachverfahrensinstanz bei FIT-Connect und Umstiegsdauer 3–5 Jahre (NP26 Z.39, Z.61).
- **Verbände, Wirtschaft, Länder:** ITSG/GKV wollen frühzeitig eingebunden werden (EG26 Z.13, Z.25–35). Dataport fordert M2M und API-first (KON #24, #25). RLP fragt nach Peppol (KON #23). Die BStBK hat Stellung genommen, Inhalt nicht abrufbar (KON #28). Bayern stimmte im Dezember 2025 als einziges Land gegen die Verbindlichkeit des D-Stacks (HE26 Z.31). Für ZaPuK speziell ist keine Länderposition bekannt. Positionen von Bitkom, BSI und BfDI speziell zu ZaPuK oder Neo wurden nicht gefunden.
- **Anbieter:** Element (Neo-Auftragnehmer) feiert ZaPuK als Bestätigung für Matrix (EL26). SET (Anbieter von Postfachsoftware) kritisiert die Frontend-Föderation und wirbt für eigene Produkte (SET Z.39–43). Die Matrix.org Foundation berichtet positiv, die Inhalte stammen vom Neo-Team (TWIM). Element ist zugleich Auftragnehmer und Fürsprecher. **[Folgerung]** Das ist ein möglicher Interessenkonflikt bei positiven Bewertungen der Matrix-Wahl.
- **Kritische Sachpunkte:**
  1. E2EE gegen Server-Funktionen: Malware-Prüfung und Archivierung (TC Z.119; v1.0 ADR-1130/1135, siehe `../architektur-technik/`). Zustellnachweise gegen Unverkettbarkeit (PI #303).
  2. MLS ist in Matrix nicht fertig (AK11 Z.779; HE25 Z.41; EL26 Z.53).
  3. Pseudonyme Adressen: Zunächst wird das BundID-Handle weitergenutzt, Pseudonyme folgen „mittelfristig“ (TA Z.48).
  4. Abhängigkeiten: D-App ohne Termin und mit möglichem Wire-Kanal (OM Z.17; HEDA Z.33), BundID-Betriebsumgebung offen (TA Z.152), FIT-Connect als Betreiber der Organisations-Backends (TA Z.52), Mitwirkung von BLK, Bayern/Bremen und KONSENS bei den Plänen (TC Z.30; TE2 Z.77).
  5. Konsultation: Gewaltenteilung bei Browser-Frontends (KON #27), fehlende Nutzergruppen (Amtsvormünder, KON #29).

## Abgleich morph_box.csv

| Zeile / Ausprägung | Angabe in morph_box.csv | Befund | Vorschlag |
|---|---|---|---|
| B1 A3 | „FIT-Connect als Anbindungsweg, Voraussetzung Matrix und MLS in SDK und Infrastruktur (Neo, Zielbild; …)“ | Neo nutzt Matrix im FIT-Connect-Java-SDK, aber Megolm statt MLS (NEO 05 Z.195; AK11 Z.785). FIT-AB 2026/12 und TS bestätigen FIT-Connect als Weg (FAB Z.22; TS Z.34). | „MLS (Zielbild; Neo mit Olm/Megolm)“ präzisieren |
| B1 A8 | Channel Adapter, „keine Messaging-Bridge (ZaPuK Architekturkonzept 3.2, ADR-0010)“ | Entwurf v1.1 sieht übergangsweise Bridges vor (ZBP-Matrix-Bridge, Postfach-2.0- und ELSTER-Bridge). Der Begriff Channel Adapter fehlt in v1.1 (TS Z.57; TA Z.120; TE2 Z.75). | um „Übergangs-Bridge an Stellen mit Klartext beim Altsystem (ZaPuK v1.1-Entwurf)“ ergänzen oder als eigene Ausprägung |
| B3 A3 | pseudonyme Adresse je Bereich (ADR-1105) | In der Transition zunächst das BundID-Postfachhandle als localpart, Pseudonyme erst mittelfristig. Neo nutzt `@<PostkorbHandle>:…`, MJP `@[SAFE-ID]:…` (TA Z.48; IL Z.518; TB Z.42). | Hinweis „Übergang: Postfachreferenz aus dem Antrag (A1)“ |
| B4 A6 | „Anfrage mit bereitgestelltem Formular … (Neo)“ | Neo: Einfach-/Mehrfachauswahl, Datei-Upload, Freitext. Terminauswahl und Formulare nur angekündigt (VA S.53, S.58). | „strukturierte Rückfragen (Auswahl, Datei-Upload)“, Formular [unverifiziert] |
| P1 A3 | Push (Neo, …) | Push im Integrationsleitfaden beschrieben, im iOS-Issue aber offen und in der Ergebnisliste nicht genannt (IL Z.77; PI #464; VA S.52) | „Neo [unverifiziert]“ |
| P1 A2 | E-Mail-Hinweis ohne Inhalt (BundID, …) | Neo-Web und ZaPuK-Transition nutzen E-Mail-Hinweise (IL Z.292; TA Z.131) | Neo/ZaPuK ergänzen |
| P2 A1–A3 | eID, BundID, EUDI (Zielbild) | Neo: BundID über Keycloak/MAS, eID per AusweisApp-SDK in der App. EUDI in Neo nicht erreicht, in der ZBP-Transition Maßnahme B11 (SA30 S.23; VA S.52–53; TA Z.108). | konsistent; Neo bei A1/A2 ergänzbar |
| P5 A2 | „eigene App und Web-Client der d-box (Neo; …)“ | Neo-Artefakte sind ausdrücklich keine eigenständigen Produkte. Zielbild ist ein zentraler Client in Bundesportal und D-App (VS Z.29; VA S.20; TS Z.32; FAB Z.18). Eigenständig war nur die TestFlight-iOS-App im Pilot. | Neo bei A2 auf „Pilot“ einschränken; ZaPuK v1.1 / FIT-AB 2026/12 bei A3 ergänzen (Bundesportal und D-App), Android und Linux Mobile geplant |
| P5 A3 | „integriert in die D-App … (Kick-start; …)“ | gestützt durch FAB Z.18 und TA Z.106–107. Risiko: D-App verzögert und möglicher Wire-Kanal (OM Z.17; HEDA Z.33). | ZaPuK ergänzen; Risiko vermerken |
| P7 A3 | Antwort an die sendende Behörde (Neo, …) | bestätigt. Behörde kann die Antwort konzeptionell einschränken, technisch noch nicht umgesetzt (IL Z.486; VA S.53). BundID-ZBP kann seit Juli 2026 Rückfragen (BMDS Z.31). | ggf. BundID-Angabe in A2 aktualisieren (Aufgabe BundID-Spalte) |
| Ü2 A2 | E2EE (Neo, ZaPuK ADR-1100 Option A, ADR-0012) | Neo E2EE mit Megolm; Metadaten-Betreiberausschluss nur teilweise (VA S.23, S.53) | „Neo (Megolm)“ |
| Ü5 A2 | Bridge „ZaPuK ADR-0010 erwogen, nicht gewählt“ | Für Fremdsysteme richtig. Für Bestandssysteme sieht der Entwurf v1.1 übergangsweise Bridges vor (TS Z.57). | Klammer „für Bestandssysteme übergangsweise vorgesehen (v1.1-Entwurf)“ |
| Ü5 A3 | Protocol Converter (ADR-0010) | jetzt konkret: OSCI-Matrix-Protokollkonverter für EGVP/MJP, Prototyp in `osci-transport-rs` (TB Z.40; OC; PI #750) | Beispiel ergänzen |
| I1 A3 | „Betrieb als Teil der D-App (Kick-start)“ | ZaPuK verortet nur das Frontend in der D-App, die Backends bei BMDS bzw. FIT-Connect (TA Z.36, Z.52; FAB Z.18–20) | keine ZaPuK-Stütze für A3; ZaPuK bleibt A4 |
| I1 A4 | Verbund, getrennte Betreiber (ADR-1114) | bestätigt durch FAB Z.20 und TA/TB/TC (BMDS für Privatpersonen, FIT-Connect für Organisationen und Justiz) | Betreiber nennen |
| I2 A2 | getrennte Backends je Nutzergruppe (Neo) | Neo: ess1 und ess2. ZaPuK v1.1: vier Backends (Privatpersonen, private Organisationen, öffentliche Stellen, Justiz) (IL Z.548–557; AK11 Z.336) | ZaPuK v1.1 ergänzen |
| P9 A2 | Postfach getrennt vom Nutzerkonto (ADR-1110) | bestätigt durch die Transition (BundID und MUK bleiben nur Auth) (TS Z.79; TC Z.145) | – |

Für die neue Spalte „ZaPuK“ in `existing_solutions.csv` gilt: Die meisten Werte sind Zielbild oder Entwurf. Wo der Neo-Pilot etwas tatsächlich gezeigt hat, sollte das getrennt benannt werden, zum Beispiel „Neo-Pilot: …; Zielbild v1.1: …“.

## Widersprüche

1. **Abschlussdatum Neo:** FIT-AB-Beschluss 23.06.2026 (FAB Z.35; VA) gegenüber „25. Juli 2026 | Befassung“ auf der Vorhabensseite (VS Z.79).
2. **Vergabe:** Zuschlag am 03.06.2025 (VS Z.117; EL26 Z.45) gegenüber „April 2025 Abschluss des Vergabeverfahrens“ (SI S.14).
3. **Bridge-Verbot:** v1.0 verbietet Messaging-Bridges (AK10 Kap. 3.2). Entwurf v1.1 lässt sie übergangsweise zu (TS Z.57; TA; TC; TE2).
4. **Frontend-Ort:** Neo zielte auf die BundID-Weboberfläche (VS Z.23, Z.187). Die Transition verlegt den Client ins Bundesportal und in die D-App (TS Z.32), Neo stellt das Design um (PI #812).
5. **ZBP-Bidirektionalität:** v1.0 hielt den Ausbau für entbehrlich (AK10 Kap. 3.3). Die BundID hat ihn im Juli 2026 dennoch eingeführt (BMDS Z.31).
6. **D-App-Kanal:** ZaPuK/FIT-AB setzen auf Matrix im D-App-Frontend (FAB Z.18). Der Handelsblatt-Bericht nennt Wire als Kanal Bürger–Behörde in der D-App (HEDA Z.33) **[unverifiziert]**.
7. **Betriebsstart:** „ab 2028“ (ZMC25 S.41; EL26 Z.23; HE25 Z.43) ist mit Phase III ab Anfang 2027 und den Phasendauern vereinbar, aber nur für das ZBP. Für MUK hängt Phase D an Aufbewahrungsfristen (TC Z.61), ELSTER bleibt dauerhaft parallel (TS Z.111).

## Lücken

- Kein öffentlicher Produktivpilot von Neo mit echten Bürger:innen. Keine Nutzungszahlen, keine Lasttests veröffentlicht (Lasttest-Umgebung nur als Stub, OC).
- iOS-App nicht im App Store gefunden (nur TestFlight laut IL Z.60). Kein Android-Client.
- Umsetzungsstand von Push und E-Mail-Benachrichtigung in Neo unklar (PI #105, #229, #464).
- Transitionspläne nur als Entwurf, nicht Teil von v1.1.1 oder der Beschluss-Anlagen; die Strategie ist Anlage zu B-2026/43-IT, aber nicht beschlossen. Kein öffentlicher Abstimmungsstand mit BMDS/BundID, BLK, Bayern/Bremen und KONSENS. Keine Termine und keine Kosten. Finanzierungskonzept erst zur 52. Sitzung (Frühjahr 2027).
- Für BundesMessenger und TI-Messenger keine Transitionspläne.
- 51. IT-PLR-Sitzung (25.11.2026): B-2026/43-IT stellt nur den Abschluss von Phase II fest; keine Festlegung zu Transition, Reihenfolge, Terminen oder Konsolidierungspflicht (B43 Z.10–12).
- Stellungnahmen von Bitkom, BSI, BfDI, Matrix.org Foundation (formell) und Ländern speziell zu ZaPuK/Neo nicht gefunden. BStBK-Stellungnahme (KON #28) nicht abrufbar. Antworten des ZaPuK-Teams auf Konsultationsbeiträge nur mit Anmeldung sichtbar.
- Behörden Spiegel und Tagesspiegel Background: keine frei zugänglichen Artikel gefunden. c’t-Artikel vom 26.06.2025 nur im Anfang (Paywall).
- Rolle der Wire-Integration in der D-App und ihr Verhältnis zu ZaPuK sind nicht offiziell geklärt.
- ISB-Folien vom 07.10.2025 sind ein Bild-PDF und wurden nicht ausgewertet.

## Änderungen v1.1.1 gegenüber den Entwurfskopien (Abgleich 2026-11-27)

- **TS** (`fitko_zapuk-transitionsstrategie_v1.1-entwurf_2026-11-13.md` → `../zapuk-v1.1.1/Zielarchitektur/Transitionen/README.md`): nur redaktionell. Platzhalter `::TODO` und falscher Hinweis auf „Version 1.0“ entfernt, neuer Hinweis auf B-2026/43-IT und die PDF-Anlage (neue Z. 7); Grammatik in Z. 19, 34 („Kernfunktion“), 47, 75. Umrechnung: alte Z. 11–16 → neu Z. − 10; alte Z. 17–18 → neu Z. 7; alte Z. ab 19 → neu Z. − 11. Die PDF-Anlage `Beschluss_2026_43_ZaPuK_Transitionsstrategie.pdf` (12 S., 06.10.2026) entspricht dem Text und enthält ebenfalls keine Anhänge.
- **TA, TB, TC, TE1, TE2:** kein Gegenstück in v1.1.1 (Tag v1.1.0 hat keinen Ordner `Transitionen`, v1.1.1 nur `Transitionen/README.md` und zwei Abbildungen; geprüft über die GitLab-API am 2026-11-27). Die Kurzfassungen in TS Z. 77–111 und PR43 S. 13 decken einen Teil der Aussagen ab (Tabelle unten).
- **ZK-README:** siehe `../recht-governance/_befunde.md`, Abschnitt „Änderungen v1.1.1“ (Umrechnung: alte Z. 1–67 gleich, 68–76 → + 13, ab 77 → + 27).

## Konkordanz: Fundstellen in `../../../Referenzen-ZaPuK.md` und `../../ZaPuK-Zusammenfassung.md` (Transition)

Alte Zeilen beziehen sich auf die Kopien in diesem Ordner (zehn Kopfzeilen, Originalzeile = Z. − 10). Neue Pfade relativ zu `sources/ZaPuK/quellen/zapuk-v1.1.1/`. Überblick, Rechtsgutachten, ZF10 und ZK-README: `../recht-governance/_befunde.md`, Abschnitt „Konkordanz“.

| Fundstelle | Kürzel, alte Z. | Neuer Pfad | Neue Z. | Inhalt |
|---|---|---|---|---|
| Referenzen, P5 | TS Z. 43 | `Zielarchitektur/Transitionen/README.md` | Z. 32 | gleich |
| Referenzen, P9 | TS Z. 90 | `Zielarchitektur/Transitionen/README.md` | Z. 79 | gleich |
| Referenzen, Ü5 und Abweichungen B1 A8; Zusammenfassung Z. 151 | TS Z. 68 | `Zielarchitektur/Transitionen/README.md` | Z. 57 | gleich |
| Referenzen, B1 | TA Z. 120–122 | – (nur Entwurf) | TS Z. 79, Z. 85–86 | teilweise: ZBP-Matrix-Bridge für die ZBP-API, keine Neu-Registrierung, schrittweise Umstellung; „Legacy-Fassade“ und FIT-Connect-ZBP-Adapter nur in TA |
| Referenzen, B1 | TE2 Z. 44–48 | – (nur Entwurf) | TS Z. 111 | teilweise: „Bereitstell-Bridge“ und SDK für ELSTER; die verworfene Protokollkonverter-Option nur in TE2 |
| Referenzen, B1 | TC Z. 42 | – (nur Entwurf) | TS Z. 107 | weitgehend: Verlinkung, Metadaten an ZaPuK, Bridge in der ELSTER-Domäne möglich |
| Referenzen, B3; Zusammenfassung Z. 80 | TA Z. 48 | – (nur Entwurf) | kein Gegenstück in TS/ÜH | BundID-Postfachhandle als Localpart nur in TA; in v1.1.1 nur allgemein ADR-1105 Z. 271 (Bestandskennung als Localpart = Option 1A, Prüfung bei `../architektur-technik/`) |
| Referenzen, B3 | TB Z. 42 | – (nur Entwurf) | kein Gegenstück in TS/ÜH | `@[SAFE-ID]:…`-Adressen nur in TB |
| Referenzen, P5 | TA Z. 98–107 | – (nur Entwurf) | TS Z. 32, Z. 83 | teilweise: Frontend im Bundesportal und in der D-App; Plattformen Web, iOS, Android, Linux Mobile nur in TA |
| Referenzen, P9 | TC Z. 145 | – (nur Entwurf) | TS Z. 99 | gleiche Aussage: MUK bleibt Authentifizierungskomponente |
| Referenzen, Ü5 | TB Z. 40 | – (nur Entwurf) | TS Z. 90 | weitgehend: OSCI-Matrix-Protokollkonverter als Teil des Justiz-Backends, E2EE bleibt; Betrieb „durch FIT-Connect“ in v1.1 über ÜH Z. 130 |
| Referenzen, I1; Zusammenfassung Z. 65 | TA Z. 36 | – (nur Entwurf) | ÜH Z. 128–130 | Betrieb des Backends für Privatpersonen durch den Bund (BMDS) auch in ÜH |
| Referenzen, I1; Zusammenfassung Z. 65 | TA Z. 152 | – (nur Entwurf) | kein Gegenstück | offene Betriebsumgebung (BundID-Umgebung oder ITZBund) nur in TA |
| Zusammenfassung Z. 37 (Kürzeltabelle) | TS / TA / TB / TC / TE1 / TE2 | `Zielarchitektur/Transitionen/README.md` (nur TS) | – | TS-Pfad ersetzen; TA–TE2 bleiben Entwurfskopien in `UR/` |
| Zusammenfassung Z. 75, Referenzen Hauptquellen TS | TS, ABB1, ABB2 | `Zielarchitektur/Transitionen/README.md`, `images/image1.png`, `image2.png` | – | Abbildungen bytegleich |
