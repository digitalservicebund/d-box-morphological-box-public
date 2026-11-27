# Befunde ZaPuK – Recht, Governance, Finanzierung (Stand 2026-11-19, Nachtrag 2026-11-27)

Fundstellen: Datei (Kurzname) + Z. (Zeile) / S. (PDF-Seite) / Ziff. Kennzeichnung: **[Folgerung]** = eigene Ableitung, **[unverifiziert]** = nicht durch Primärquelle belegt, **[veraltet?]** = Quelle vermutlich überholt, **[Entwurf]** = nur im früheren Branch `architecture-v1.1`, nicht in v1.1.1 enthalten. **v1.1** = dem IT-Planungsrat zur 51. Sitzung vorgelegt, Anlage zu B-2026/43-IT, inhaltlich nicht beschlossen (siehe Abschnitt „Beschluss B-2026/43-IT“).

Seit 2026-11-27 beziehen sich RG, ÜH, TS, Z-README und Z-ZUS10 auf die lokale Kopie von ZaPuK v1.1.1 (`../zapuk-v1.1.1/`); alle Zeilenangaben unten sind umgerechnet. Alte Zeilen der Entwurfskopien: Abschnitt „Konkordanz“.

Kurzformen:

- RG = `../zapuk-v1.1.1/Zielarchitektur/Rechtsgutachten/README.md` (v1.1, Stand 19.10.2026, mit Abschnitt E); früher `FITKO_ZaPuK-Rechtsgutachten-v1-1_2026-10-06.md`
- ÜH = `../zapuk-v1.1.1/Zielarchitektur/Zusammenfassung/README.md` (Überblick und Handlungsempfehlungen v1.1, Stand 12.10.2026); früher `FITKO_ZaPuK-Ueberblick-Handlungsempfehlungen-v1-1_2026-10-06.md`
- TS = `../zapuk-v1.1.1/Zielarchitektur/Transitionen/README.md` (Übergreifende Transitionsstrategie v1.1)
- B43 = `IT-PLR_B-2026-43-IT_ZaPuK-Abschluss-Phase-II.md`; EIN = `IT-PLR_Beschluss_2026_43_ZaPuK_Einordnung_Rechtsgutachten.pdf` (FITKO-Kommentierung, S. = PDF-Seite); PR43 = `IT-PLR_Beschluss_2026_43_ZaPuK_Praesentation.pdf` (Foliensatz zur 51. Sitzung)
- AK = `FITKO_ZaPuK-Architekturkonzept-v1-1_2026-09-25.md` [Entwurf] (Zeilen dieser Kopie; Fortschreibung in v1.1.1 wertet `../architektur-technik/` aus)
- FITAB = `FITKO_FIT-AB-Beschluesse_2026-11-19.md`
- Z-README = `../zapuk-v1.1.1/README.md`; Z-ZUS10 = `../zapuk-v1.1.1/Zielarchitektur/Zusammenfassung/Vorversionen/v1.0/README.md` (v1.0, Juni 2025, zeilengleich mit der früheren Kopie in `../zapuk-v1.0.0-121-g7419d8b/`); ADR-xxxx = `../zapuk-v1.1.1/Zielarchitektur/Architekturentscheidungen/ADR-xxxx-*.md` (die hier zitierten Zeilen sind gegenüber `../zapuk-v1.0.0-121-g7419d8b/` unverändert, außer ADR-1145 Z. 163 → Z. 166); ANF = `../zapuk-v1.1.1/Konsolidierte-Anforderungen/README.md`
- B18 = `../../../BundID-ZBP/quellen/recht-governance/IT-PLR_B-2026-18-IT_D-Stack_Finanzierungsmodell.md`; B03 = `…/IT-PLR_B-2026-03-IT_Deutschland-Stack.md`; B2528 = `…/IT-PLR_Beschluss_2025-28_ZaPuK.md`; FöMA = `…/Foederale_Modernisierungsagenda_MPK_2025-12-04.pdf`; D9020 = `…/BT-Drs_21-9020_Antwort_KA_Deutschland-Stack.pdf`
- PM50 = `IT-PLR_Pressemitteilung_50-Sitzung_2026-06-17.md`; D7319 = `BT-Drs_21-7319_Antwort_KA_EUDI-Wallet.pdf`; DA-Zus = `IT-PLR_Beschluss_2026_04_Deutschland-Architektur_Zusammenfassung.pdf`

## Kern: Status und Rechtsgrundlage

- **Beschlusslage:** B-2024/28-IT (Auftrag Zielarchitektur), B-2025/28-IT (v1.0 beschlossen, Phase II an FITKO; Ziff. 6: FITKO soll bis zur 50. Sitzung Rechtsgrundlage und Finanzierung des gemeinschaftlichen Betriebs klären, B2528 Z. 38ff.), B-2026/03-IT (Plattformkern mit Basisdienst „Postfach“; Bund, Länder, Kommunen „streben … an“, B03 Z. 21–27), B-2026/04-IT (Funktionsbaustein „Postfach und Interaktion (G2C)“, DA-Zus S. 19), B-2026/18-IT (fachliche Verantwortung des Bundes für „Postfach/ Benachrichtigung (ZaPuk)“, B18 Z. 23; Betriebs- und Finanzierungsmodell bis zur 52. Sitzung, Z. 29–31), B-2026/43-IT (Abschluss Phase II, Bestätigung der Programmverantwortung des Bundes, B43 Z. 10–12).
- **FIT-AB 2026/12 (15.09.2026, einstimmig):** empfiehlt Aufbau auf Basis ZaPuK, Phase III „im Rahmen des D-Stacks unter Koordination des Bundes“, zentrales Postfach-Frontend in Bundesportal/Deutschland-App, zentrale Backends für Privatpersonen und Organisationen „in voneinander unabhängigen Betriebsorganisationen“, FIT-Connect-SSP als Rollout-Infrastruktur, Anpassung von BundID/ZBP, MJP, MUK und ELSTER-Fachportal (FITAB Z. 15–26).
- **Beschluss B-2026/43-IT (51. Sitzung, 25.11.2026):** siehe eigenen Abschnitt unten. Kurz: Der IT-PLR „sieht“ Phase II „als abgeschlossen an“ (B43 Z. 10), der Bund „bestätigt die Übernahme der Programmverantwortung gemäß Beschluss B-2026/18-IT“ (B43 Z. 12). Von den vier in ÜH Z. 48 erbetenen Entscheidungen (Bestätigung v1.1, „verbindliche Festlegung, dass die bestehenden Postfachlösungen hin zur ZaPuK konsolidiert werden“, Phase III unter Federführung des BMDS, Beauftragung von Rechtsetzungs- und Governance-Schritten) enthält der Beschlusstext keine ausdrücklich; nur die Programmverantwortung des Bundes ist bestätigt.
- **Rechtsgutachten (Phase II, AP 2.7):** Es ist öffentlich und Anlage zu B-2026/43-IT, Textfassung in v1.1.1 (RG Z. 1–46). Neu gegenüber dem Entwurf: Abschnitt E „Möglichkeiten und Grenzen des IT-StV (Ergänzende Stellungnahme vom 19.10.2026)“ (RG Z. 3485–3573, siehe unten). Die FITKO hat es mit einer eigenen Kommentierung eingeordnet (EIN, siehe unten). Es wurde im Auftrag der FITKO von Eichler Kern Klein und PLANIT // LEGAL erstellt und bewertet v1.1 (Stand 08.09.2026). Ergebnis: ZaPuK ist „grundsätzlich realisierbar“ (ÜH Z. 205ff.). Eine einzelne Verfassungsnorm reicht nicht, möglich ist eine **Kombination** (RG Z. 1170–1177):
  - Art. 86 GG: zentrale Softwareentwicklung beim Bund.
  - Art. 91c Abs. 5 GG: Bundesgesetz zur Postfachstruktur „einschließlich … Anschlussverpflichtung von Ländern und Gemeinden“ (Z. 1175).
  - Art. 87 Abs. 3 GG: Der Bund kann den Betrieb des Privatpersonen-Backends an sich ziehen und auf eine nachgeordnete Behörde im Geschäftsbereich des BMDS übertragen; daraus folgt seine Finanzierungspflicht (Z. 1066–1075).
  - Art. 91c Abs. 1/2 GG: Staatsvertrag (eigener ZaPuK-StV, D-Stack-StV in Anlehnung an den NOOTS-StV oder Ergänzung des IT-StV). Damit kann der Betrieb der Organisations-Backends an die FITKO delegiert werden (Z. 1176–1177).
  - Für die Justiz ist Art. 74 Abs. 1 Nr. 1 GG nötig (Z. 1066ff.).
  - Eine Grundgesetzänderung ist nicht zwingend, wäre aber „wünschenswert“ (Art. 91c GG neu, FöMA Maßnahme 201; RG Z. 1179–1183).
- **Kein Postfach-Gesetz und keine D-Stack-Rechtsgrundlage gefunden.** Das DIdG (BR-Drs. 618/26, Bundesrat 20.11.2026) regelt Bürgerkonto und EUDI-Wallet, ein ZaPuK-Postfach nicht. RG empfiehlt seine Neufassung von § 3 OZG als Anknüpfung für die Identifizierungsmittel des ZaPuK-Postfachs (RG Tab. 2, Z. 645ff.; ÜH Z. 213). **[Folgerung]**

## Beschluss B-2026/43-IT (51. Sitzung, 25.11.2026)

- **Wortlaut** (B43 Z. 10–12, vollständig):
  1. „Der IT-Planungsrat sieht die in Beschluss B-2025/28-IT beauftragte Transitionsplanung und Validierung der Zielarchitektur Postfach- und Kommunikationslösungen (ZaPuK) als Phase II eines Umsetzungsprogramms zur Etablierung einer einheitlichen föderalen Postfach- und Kommunikationsinfrastruktur als abgeschlossen an.“
  2. „Der Bund bestätigt die Übernahme der Programmverantwortung gemäß Beschluss B-2026/18-IT. Die bisherigen Programmbeteiligten sichern den Übergang.“
- **Anlagen** (B43 Z. 14–21): Präsentation (PR43), Überblick (= ÜH), Zielarchitektur (= Architekturkonzept v1.1), Transitionsstrategie (= TS), Rechtsgutachten (= RG), Einordnung des Rechtsgutachtens durch die FITKO (EIN).
- **Was beschlossen ist:** nur (a) die Feststellung, dass Phase II abgeschlossen ist, und (b) die Bestätigung der Programmverantwortung des Bundes nach B-2026/18-IT, mit Übergangszusage der bisherigen Beteiligten (FITKO, Hamburg, Sachsen-Anhalt).
- **Was nicht im Beschlusstext steht** (Abgleich mit den vier erbetenen Punkten in ÜH Z. 48):
  1. keine Bestätigung oder Verabschiedung der Zielarchitektur v1.1; die Dokumente sind nur Anlagen. Das ZaPuK-Repo bezeichnet v1.0 als „beschlossen“, v1.1 nur als „Abgabefassung für die 51. Sitzung des IT-Planungsrats, Anlagen zu Beschluss 2026/43-IT“ (Z-README Z. 76–77); früher hieß es, in der 51. Sitzung solle v1.1 „verabschiedet“ werden (`../zapuk-v1.0.0-121-g7419d8b/README.md` Z. 49), jetzt nur „dem IT-Planungsrat zur 51. Sitzung vorgelegt“ (Z-README Z. 49);
  2. keine verbindliche Festlegung zur Konsolidierung der Bestandslösungen, keine Teilnahme- oder Anschlusspflicht;
  3. keine ausdrückliche Initiierung von Phase III und keine Nennung des BMDS; Phase III unter BMDS-Federführung steht nur im Überblick (ÜH Z. 41–42) und im Foliensatz (PR43 S. 20); der Beschluss spricht von „Programmverantwortung“ des Bundes;
  4. kein Auftrag für Rechtsetzung, Staatsvertrag oder Governance;
  5. keine Finanzierungsentscheidung; das Finanzierungskonzept kommt erst zur 52. Sitzung (PR43 S. 14; ÜH Z. 194; B18 Z. 29–31).
- **Pressemitteilung zur 51. Sitzung** (25.11.2026): erwähnt ZaPuK nicht (siehe `_index.md`).
- **[Folgerung]** Statusformel für die CSV-Dateien: „v1.1 (Anlage zu B-2026/43-IT, nicht beschlossen)“ statt „Entwurf v1.1“; „Phase II abgeschlossen, Programmverantwortung beim Bund (B-2026/43-IT)“ statt „Beschluss am 25.11.2026 vorgesehen“.

## FITKO-Einordnung des Rechtsgutachtens (EIN, Stand 06.10.2026)

- Gegenstand: drei „hervorzuhebende Gesichtspunkte“ zum Gutachten und zum Überblick, „insbesondere die verfassungsrechtliche Einordnung der Betriebs-, Finanzierungs- und Nutzungsmodelle“ (EIN S. 3, Ziff. 1).
- **Ziff. 2.1 (S. 3):** Betriebs-, Finanzierungs- und Nutzungsmodell trennen. Aus der fehlenden Tragfähigkeit einer Rechtsgrundlage für die Verbindlichkeit der Nutzung folge nicht ihre fehlende Tragfähigkeit für Entwicklung, Betrieb oder Bereitstellung. Möglich: Entwicklung, Betrieb und Bereitstellung über Art. 91c Abs. 1 GG mit dem IT-StV, Verbindlichkeit der Nutzung durch „flankierende gesetzliche Vorgaben“. Ob die Nutzung überhaupt verbindlich werden soll, „obliegt dabei der politischen Verständigung von Bund und Ländern“.
- **Ziff. 2.2 (S. 4):** Der IT-PLR kann nach § 1 Abs. 1 S. 1 Nr. 3, 4 IT-StV Lösungen betreiben lassen und Produkte steuern; ZaPuK könne „ganz oder teilweise“ im bestehenden Produktmanagement-Rahmen etabliert werden. Welchen Zusatzbedarf ein neuer oder geänderter Staatsvertrag decken soll, sei „klärungsbedürftig“. Finanzierung: § 9 Abs. 4 S. 1 Hs. 2 IT-StV eröffnet Spielraum für vom Königsteiner Schlüssel abweichende Regeln.
- **Ziff. 2.3 (S. 5):** Ob neben Art. 91c Abs. 5 GG zusätzlich Art. 86 bzw. Art. 87 Abs. 3 GG nötig ist, sei „klärungsbedürftig“; Vorbild § 3 Abs. 1 OZG (Bürgerkonto mit Postfach beim Bund, gestützt auf Art. 91c Abs. 5 GG) und DIdG-E (BT-Drs. 21/7408).
- **[Folgerung]** Die FITKO relativiert die Gutachten-Empfehlungen zum Staatsvertrag und zu Art. 87 Abs. 3 GG, nicht deren Befund, dass eine Anschlusspflicht ein Gesetz (Art. 91c Abs. 5 GG) oder einen Staatsvertrag braucht.

## Rechtsgutachten, Abschnitt E (Ergänzende Stellungnahme vom 19.10.2026, RG Z. 3485–3573)

- Neu gegenüber dem Entwurf vom 06.10.2026, reagiert erkennbar auf die FITKO-Einordnung.
- IT-StV ermöglicht grundsätzlich, ZaPuK ganz oder teilweise (Backends der Organisationen, Adressregister, SSP) durch die FITKO bereitzustellen und im Produktmanagement betreiben zu lassen (RG Z. 3501–3503).
- Grenzen: keine verbindliche Anschluss- und Nutzungspflicht allein über den IT-StV (Z. 3511); Justiz nicht im Zuständigkeitsbereich des IT-PLR (Z. 3515–3525); Finanzierung über den Wirtschaftsplan unsicher, völlig einseitige Bundeslast verfassungsrechtlich nicht getragen (Z. 3527–3545); Governance des Verbunds ungeregelt (Z. 3547–3551); OZG- und Verfahrensrechtsänderungen bleiben nötig, verbindliche Anbindung von Ländern und Kommunen über das OZG (Art. 91c Abs. 5 GG) oder Staatsvertrag (Z. 3553–3559).
- Ergebnis: zusätzliche staatsvertragliche Regelungen bleiben „vorzugswürdig“ (Governance, D-Stack-weit, konsensuale Verbindlichkeit, Finanzierung) (Z. 3561–3571).

## Befunde je morph_box-Zeile

### I1 Betrieb und Governance

- **Empfohlene Verantwortlichkeiten (v1.1, nicht beschlossen)** (ÜH Z. 128–134, Abb. 2; PR43 S. 8; AK Z. 307–366):
  - **BMDS:** Zentraler Postfach-Zugang und Backend für Privatpersonen, Betrieb und Entwicklung des Zugangs, „gegebenenfalls delegiert an eine nachgeordnete Behörde oder einen Dienstleister“. Begründung u. a.: „Der Bund kommt seiner Bereitstellungs-Verantwortung für das Postfach des Bürgerkontos gemäß § 3 Abs. 1 OZG nach“ (AK Z. 321).
  - **FITKO-Produktmanagement (FIT-Connect):** Entwicklung aller Backends; Betrieb der drei Organisations-Backends (private Organisationen, öffentliche Stellen, Justiz, gebündelt bei einer Stelle); SSP, SDK und Adressregister.
  - **BLK Justiz:** SAFE-Register.
  - **Steuerung:** „Governance des D-Stacks (in Ausgestaltung)“. Neue Organisationen sind nicht vorgesehen (ÜH Z. 128–134).
- **ITZBund wird in v1.1 nicht genannt.** **[Folgerung]** Nahe liegt eine Delegation an eine BMDS-nachgeordnete Stelle; das ITZBund betreibt heute die BundID. Beleg dafür fehlt **[unverifiziert]**.
- ADR-1114 legt nur die Trennung fest, keine konkreten Betreiber (ADR-1114 Z. 17). v1.1 und RG Z. 588 ordnen die Betreiber zu: Organisations-Backends „durch FIT Connect“, Privatpersonen-Backend „durch den Bund“.
- **Rolle der Länder:**
  - Phase I/II: Federführung gemeinsam bei FITKO, Hamburg und Sachsen-Anhalt (Z-README Z. 57).
  - Phase III: Steuerungskreis aus BMDS, FITKO und Ländervertreter:innen, dazu ein Sounding Board. IT-PLR bzw. AL-Runde sind übergeordnet; bei föderaler Bebauung wird das FIT-AB eingebunden (ÜH Z. 300; PR43 S. 20 „Mit Phase 3 übernimmt das BMDS die Federführung und Programmleitung“). Der Beschluss selbst nennt das BMDS nicht, nur „der Bund“ (B43 Z. 12).
  - Dauerhaft: Produktgovernance nach FITKO-Produktmanagement-Modell. Empfohlen wird eine gemeinsame Governance der Bausteine „Datenaustausch“ und „Postfach und Interaktion“ (ÜH Z. 308–310).
  - Das „Digital Center Deutschland Stack“ (DCDS) soll „Umsetzungsmuskel“ für D-Stack-Produkte des Bundes werden (ÜH Z. 318).
  - Betriebsplattform ist die Deutsche Verwaltungscloud (ÜH Z. 310).
- **Dritte:** Organisationen und private Dritte dürfen eigene Backends betreiben (ADR-0004), Privatpersonen nicht (ADR-1114 Z. 72, 78).
- **Rechtliche Voraussetzung der Betriebsübernahme:**
  - Die Übernahme der MUK-Postfächer durch FIT-Connect „erfordert die Anpassung der Verordnung nach § 3 Abs. 2 OZG“ (AK Z. 365).
  - Datenschutzrechtliche Verantwortung der Backend-Betreiber: Zuweisung per Rechtsvorschrift nach dem Vorbild von § 8a Abs. 4 OZG empfohlen (RG Z. 1855ff.).
  - Gemeinsame Vollzugsverantwortung mit der FITKO braucht eine Legitimation über Art. 91c Abs. 1 GG (Verbot der Mischverwaltung, RG Z. 1066ff.).
- **Widerspruch zur Zuständigkeit:** BT-Drs. 21/7319 (Juli 2026) nennt die FITKO als „federführend zuständige Stelle“ für ZaPuK (D7319 S. 5). Der BRH (31.07.2026, nicht abgelegt, S. 25) schreibt „Zuständig ist die FITKO“. Seit B-2026/18-IT trägt aber der Bund die fachliche Verantwortung; B-2026/43-IT bestätigt die Programmverantwortung des Bundes (B43 Z. 12). **[Folgerung]** Die Angaben beschreiben Phase II und sind für Phase III **[veraltet?]**.

### I5 Aufnahme in den Verbund

- ADR-1145 (Option C, risikobasiert): Anschlussvertrag im SSP, Föderationsliste, Suspendierung und Ausschluss. „Die rechtliche Ausgestaltung ist zu klären“ (ADR-1145 Z. 83). Offen ist, wer über Suspendierung und Ausschluss entscheidet (Z. 166).
- AK v1.1: Backends werden „durch die Verbund-Governance in einem manuellen Freigabeschritt“ in die Föderationsliste aufgenommen (AK Z. 582).
- **RG bestätigt den Ansatz als „rechtlich tragfähig“ und „einer Zertifizierung nach dem Vorbild des § 325 SGB V oder der De-Mail-Akkreditierung vorzuziehen“.** Es empfiehlt einen **dreistufigen Rahmen** (RG Z. 3468ff.; Tab. 2 Z. 645ff.):
  1. Rechtsverordnung nach §§ 5, 6 OZG mit Wirkung gegenüber allen Stellen.
  2. IT-PLR-Beschluss nach § 2 IT-StV.
  3. Anschlussvertrag im SSP für Betreiberpflichten, Haftung und Föderationskontrolle. Das Regelwerk soll der IT-PLR beschließen, nicht der Betreiber.
- **Vorbild Verbindungsnetz (IT-NetzG):** Auskunftspflicht mit Zwei-Wochen-Frist; gestuftes Ausschlussverfahren (Stellungnahme, Abmahnung, Kündigung im Einvernehmen mit einem IT-PLR-Gremium); Haftungs-Durchleitung (RG Z. 3420–3435, 3468ff.).
- **Grenzen von Verträgen:** keine Bindung von Ländern und Kommunen, die nicht beitreten; keine Drittwirkung; kein Ersatz für die Anerkennung als sicherer Übermittlungsweg oder qualifizierter Zustelldienst (RG Z. 3366ff., 3448ff.). Die ITSiV-PV gilt schon heute: § 2 Abs. 6 S. 2 nennt das Postfach ausdrücklich (Pentest und Webcheck vor Anbindung).
- **Formfrage:** Bei öffentlich-rechtlicher Ausgestaltung gilt § 57 VwVfG (Schriftform). Ein reiner Klick im SSP genügt nicht; sonst droht Nichtigkeit (RG Z. 3468ff.).
- **ERV:** Die ERV-Fähigkeit soll als eigene Anschlussstufe mit Regel-Konformitätsprüfung geführt werden. Empfohlen wird ein Zulassungsverfahren per Rechtsverordnung nach § 130a Abs. 4 S. 1 Nr. 5 ZPO (RG Z. 3255ff.; in RG Tab. 2 als „§ 30a“ verschrieben).

### R1 Schriftformersatz

- **ZaPuK selbst ersetzt die Schriftform nicht.**
  - Behördenseitig gilt der Ersatz derzeit nur durch QES oder qualifiziertes Siegel (§ 3a Abs. 2/3 Nr. 3 VwVfG, § 9a Abs. 6 OZG, § 87a Abs. 4 AO, § 36a SGB I). ZaPuK übermittelt solche Dokumente unverändert (ADR-0017: Siegelung durch den Absender vor dem Versand; RG Z. 1559ff., 1609ff.).
  - „Für den Schriftformersatz ist eine Beweiskraft gegenüber Dritten erforderlich“ (ADR-0017 Z. 11).
- **Hinkanal heute:** QES; Online-Formular mit eID (§ 3a Abs. 3 Nr. 1 VwVfG, § 9a Abs. 5 OZG); EGVP-Postfächer nach § 3a Abs. 3 Nr. 2 VwVfG. Ein ZaPuK-Postfach ist nicht genannt (RG Z. 1559ff.).
- **Ab 1.7.2027** (Gesetz für den Bürokratierückbau im Bereich des BMI vom 3.7.2026, RG Z. 1585ff.):
  - § 3a Abs. 2 VwVfG n.F.: Die Schriftform wird schon durch ein „elektronisches Dokument“ (Textform) gewahrt, soweit das Fachrecht nichts anderes bestimmt.
  - Wo das Fachrecht die elektronische Form verlangt (§ 3b n.F.), gibt es drei Wege: Formular nach Identifizierung, QES oder Siegel, sicherer Übermittlungsweg nach § 3c.
  - Die Anerkennung von ZaPuK als sicherer Übermittlungsweg setzt eine Rechtsverordnung des BMI im Einvernehmen mit dem BMDS nach § 3c Abs. 2 VwVfG n.F. voraus. Die materiellen Kriterien erfülle ZaPuK bereits (RG Z. 1609ff., 3468ff.).
  - **§ 3b S. 1 Nr. 3 n.F. erfasst nur das „Einreichen“, also den Hinkanal.** RG empfiehlt, ihn vor Inkrafttreten auf behördenseitige Übermittlung zu erstrecken und Parallelregeln in § 87a AO und § 36a SGB I zu schaffen (RG Z. 1605).
- **Gegenüber Gerichten (ERV):** ZaPuK ist schon sicherer Übermittlungsweg nach § 130a Abs. 4 S. 1 Nr. 4 ZPO (Nutzerkonto, wie MJP; RG Z. 3062, Z. 3078, Z. 3152); eine Verordnung nach Nr. 5 ist der rechtssicherste Weg (Z. 3146, Z. 3162). Mit der Entkopplung entfällt Nr. 4, wenn die Verweisung nicht mitgeführt wird (Z. 3163); dafür sind Anpassungen in ERVV § 11, § 13 nötig, und Matrix muss anstelle von OSCI anerkannt werden. Die einfache Signatur bleibt zwingend (RG Tab. 2; Z. 3140ff.).
- **[Folgerung]** Das morph_box-Zielbild „ersetzt die Schriftform in beide Richtungen“ ist mit ZaPuK nach geltendem Recht nur über QES/Siegel erreichbar. Ab 7/2027 reicht im Hinkanal überwiegend Textform. Für den Rückkanal ist eine Gesetzesänderung nötig (Erstreckung § 3b Nr. 3, AO, SGB I).

### R2 Teilnahmepflicht

- **Freiwilligkeitsgrundsatz:** Elektronische Übermittlung setzt einen vom Empfänger eröffneten Zugang voraus (§ 3a Abs. 1 VwVfG, § 36a SGB I, § 87a Abs. 1 AO, § 5 Abs. 5 VwZG). Eine Pflicht für Bürger:innen wurde 2002 und 2013 bewusst nicht geschaffen; die Nutzung des Bürgerkontos ist freiwillig (§ 3 Abs. 1 S. 2 OZG) (RG Z. 1206–1226).
- **§ 3a Abs. 1 VwVfG n.F. (ab 1.7.2027):** Kommunikation „soll“ elektronisch erfolgen, gilt aber nur bei bereits eröffnetem Zugang. Die Zugangseröffnung durch die Privatperson bleibt laut Gesetzesbegründung erforderlich; Papier bleibt wählbar (RG Z. 1238ff.).
- **ZaPuK-Umsetzung:**
  - **Privatpersonen:** konkludente Zugangseröffnung durch Antragstellung; das bereichsspezifische Pseudonym dient als Rückantwortadresse. Proaktive Kontaktaufnahme ist nur mit einer Adresse aus einem früheren Vorgang möglich, sonst erst nach Authentisierung. Eine „kalte“ Erstansprache ist rechtlich unzulässig und technisch nicht möglich, weil es kein Adressregister für Privatpersonen gibt.
  - **Organisationen:** Registrierung im SSP mit Eintrag im Adressregister gilt als konkludente Zugangseröffnung (RG Z. 1250ff.; ÜH Z. 209; ADR-1148: Registrierung = Opt-in für Adressierbarkeit).
- **Empfohlene Rechtsänderung für proaktive Zustellung** (RG Z. 1246–1248):
  - spezialgesetzliche Fiktion nach dem Vorbild von § 9 OZG und § 122a AO, begrenzt auf Verfahren und Nachrichtentypen;
  - **Opt-in je Bereich**, einmalig und widerruflich, mit Benachrichtigung über einen zweiten Kanal;
  - funktionierende Schlüsselwiederherstellung;
  - Rückfallebene Papier.
- **[Folgerung]** ZaPuK entspricht heute „freiwillig, Zugang je Antrag“. Die Empfehlung zielt auf „Opt-in je Bereich“, also nicht auf eine Pflicht. Der Ausschluss eines Bürger-Adressregisters schließt „Pflicht für alle“ (Dänemark-Modell) faktisch aus.
- Unternehmen: Keine ZaPuK-spezifische Pflicht gefunden. Die Digital-only-Regel des OZGÄndG für Unternehmensleistungen erwähnt nur die Rahmenbedingungen (`../zapuk-v1.1.1/Zielarchitektur/Rahmenbedingungen/README.md` Z. 54, Zeile in v1.1.1 geprüft).

### R3 Bekanntgabe und förmliche Zustellung

- **Bekanntgabe** (RG Z. 1313ff., 1450ff., 1506ff.):
  - Bei dezentralen Organisations-Backends: Übermittlungsvariante nach § 41 Abs. 2 VwVfG, § 37 Abs. 2 SGB X, § 122 Abs. 2a AO. Der Zugang wird durch die serverseitige Bereitstellungsbestätigung (ADR-0018) belegt.
  - Bei zentralen Backends: Abrufmodelle (§ 41 Abs. 2a VwVfG mit Abrufnachweis, § 9 OZG, § 122a AO, § 37 Abs. 2a/2b SGB X).
  - ZaPuK muss Bereitstellung und Zeitpunkt, Abruf und möglichst den Zugang der Benachrichtigung nachweisen. Das leisten die Bereitstellungsbestätigung (ADR-0018), die Abrufbestätigung (ADR-1118) und die absendergesteuerten Optionen (ADR-1119).
  - Der Anhang muss im Empfänger-Backend liegen, ein Link auf das Absender-Repository genügt nicht (ADR-1120).
  - Verschlüsselungsfehler sollen verhindert oder revisionssicher dokumentiert werden.
- **§ 9 OZG gilt nur für das Postfach eines Nutzerkontos.** Für das entkoppelte ZaPuK-Postfach greift sonst § 41 Abs. 2a VwVfG mit tatsächlichem Abruf. Daher müssen § 9 OZG und § 122a Abs. 5 AO angepasst werden (ADR-0018 Z. 234; ADR-1110 Z. 84; RG Tab. 2).
- **Harmonisierung (FöMA Maßnahme 207, Frist laut FöMA 30.09.2026):** RG empfiehlt ein einheitliches Bekanntgabeereignis (z. B. Bereitstellung) mit Fiktion, zentral im VwVfG, mit Verweisen im Fachrecht (RG Z. 1517ff.). ÜH nennt Anpassungsbedarf in OZG, AO, VwVfG des Bundes und der Länder, VwZG des Bundes und der Länder sowie SGB X (ÜH Z. 37). **[Lücke]** Ein Gesetzentwurf zu Maßnahme 207 wurde nicht gefunden.
- **Förmliche Zustellung (VwZG)** (RG Z. 1528ff., 1554ff.):
  - **§ 5 Abs. 5 VwZG ist schon heute die Rechtsgrundlage:** elektronische Zustellung gegen Empfangsbekenntnis bei eröffnetem Zugang, mit QES am Dokument.
  - Das elektronische Empfangsbekenntnis braucht eine QES des Empfängers; eine automatische Bestätigung genügt nicht.
  - Eine Fiktion am 4. Tag gibt es nur, wenn das Verfahren auf Verlangen des Empfängers elektronisch geführt wird (§ 5 Abs. 7 S. 2).
  - § 5 Abs. 4 VwZG erleichtert die Zustellung an Behörden, Anwälte usw.
  - § 5a VwZG (De-Mail) ist gestrichen; das De-Mail-G tritt mit Ablauf des 31.12.2026 außer Kraft.
  - ZaPuK bietet eine aktive Lesebestätigung, erweiterbar zum Empfangsbekenntnis; das Zustelldatum soll frei eintragbar sein (ADR-1118 Z. 96–110).
- **Qualifizierter Zustelldienst (eIDAS Art. 43/44):** ADR-0018 orientiert sich an ERDS. Die Vermutungswirkung nach Art. 43 Abs. 2 setzt den QTSP-Status voraus. Empfohlen wird, ihn bei den zentralen Betreibern bzw. einem Quittungs- und Zeitstempeldienst zu bündeln (RG Z. 3468ff.; ADR-0018 Z. 11; ANF_FUN_USE_077).
- **[Folgerung]** Die morph_box-Formulierung „Rechtsgrundlage … ist zu prüfen“ greift zu kurz. Laut RG besteht sie (§ 5 Abs. 5/7 VwZG); praktisches Hindernis ist die QES-Pflicht für das Empfangsbekenntnis von Bürger:innen.

### R4 Teilnahme der Behörden

- **Heute keine Teilnahmepflicht für ZaPuK.**
  - B-2026/03-IT: Bund, Länder und Kommunen „streben die Nutzung … an“ (B03 Z. 24–27).
  - B-2026/18-IT Ziff. 5: Die Pflicht der Länder zu Anbindung, Flächendeckung und Nutzung gilt nur für die Basiskomponenten der **Ziff. 3** (eID/EUDI-Wallet, FIT-Connect, NOOTS), **nicht für Postfach/Benachrichtigung** (B18 Z. 33).
  - Meilensteinplanung des Bundes und der Länder bis zur 52. Sitzung (B18 Z. 37–40).
- **Widerspruch:**
  - PM50 Z. 21 („Die Länder haben eine verbindliche Anbindung und Flächendeckung der Komponenten zugesagt“) ist pauschal formuliert.
  - Das Rechtsgutachten (RG Z. 574) liest Ziff. 5 so, dass sich die Länder für ZaPuK „verbindlich“ zur Anbindung verpflichten. **[Folgerung]** Das deckt der Beschlusswortlaut nicht.
- **Vorgeschlagen, nicht beschlossen:** „Verbindliche Festlegung, dass die bestehenden Postfachlösungen hin zur ZaPuK konsolidiert werden“ (ÜH Z. 48). B-2026/43-IT enthält diese Festlegung nicht (B43 Z. 10–12). Die Transition setzt „verbindliche Grundsatzentscheidungen der zuständigen Stellen“ voraus (ÜH Z. 144, 190).
- **Rechtliche Wege (RG):**
  - Art. 91c Abs. 5 GG trägt eine bundesgesetzliche Anschlussverpflichtung von Ländern und Kommunen. Für Kommunen gilt das nur ohne erhebliche, einer Aufgabenübertragung vergleichbare Lasten und mit Gestaltungsspielraum (RG Z. 1075, 1175). Abschnitt E bekräftigt: verbindliche Anbindung der Länder einschließlich der Kommunen über das OZG auf Basis von Art. 91c Abs. 5 GG oder per Staatsvertrag (RG Z. 3559).
  - Ein Staatsvertrag ermöglicht „verbindliche Vorgaben über die Ablösung bestehender Postfach- und Kommunikationslösungen“ (Z. 1176).
  - **Nicht** über den IT-Staatsvertrag allein: „Allein über den IT-Staatsvertrag kann eine verbindliche Anschluss- und Nutzungsverpflichtung hinsichtlich des ZaPuK-Kommunikationsverbunds gegenüber den Ländern (einschließlich der Kommunen) bzw. dem Bund nicht hergestellt werden.“ (RG Z. 3511, Abschnitt E).
  - FITKO-Einordnung: Ob und in welchem Maß die Umsetzung mit einer rechtlich verbindlichen Nutzung verbunden sein soll, „obliegt dabei der politischen Verständigung von Bund und Ländern“ (EIN S. 3, Ziff. 2.1).
  - IT-PLR-Standards nach § 2 IT-StV und Verordnungen nach §§ 5, 6 OZG binden alle Stellen (Z. 3448ff.).
  - § 2 Abs. 1 EGovG verpflichtet Bundesbehörden bereits, einen elektronischen Zugang zu eröffnen, legt aber kein Verfahren fest (RG Z. 1224).
- **Justiz:** Eigene IT-Governance über den E-Justice-Rat (RG Z. 2916ff.; Abschnitt E III.2, Z. 3515–3525). ZaPuK will „nicht in die Entscheidungsprozesse der Justizverwaltung eingreifen“ (AK Z. 354). FIT-AB empfiehlt eine Zusammenarbeit IT-PLR/E-Justice-Rat (FITAB Z. 15–26).
- **Anreize und Finanzierung für Behörden:** offen (Finanzierungskonzept zur 52. Sitzung, ÜH Z. 194; PR43 S. 14). Kostenfreiheit gilt nur für Privatpersonen (ANF RAH_ÜBE_148/149).
- **[Folgerung]** ZaPuK-Ziel ist die Option „verbindliche Teilnahmepflicht“, rechtlich noch nicht hergestellt. Stichtage gibt es nicht; ein einheitlicher Stichtagswechsel ist ausdrücklich nicht vorgesehen (ÜH Z. 144). Auch nach der 51. Sitzung ist keine Pflicht beschlossen (B43).

### Weitere Zeilen

- **P9 Postfach und Nutzerkonto:** Die Entkopplung (ADR-1110) erfordert Änderungen an § 2 Abs. 7, § 9 OZG und § 122a Abs. 5 AO sowie an § 130a Abs. 4 Nr. 4 ZPO und den Parallelnormen (RG Tab. 2; ADR-1110 Z. 70, 84).
- **P7 Antwort und Erstkontakt:**
  - Behördlicher Erstkontakt mit Privatpersonen ist rechtlich nur nach vorheriger Mitwirkung möglich (s. R2).
  - Für Kontaktaufnahme durch Personen zu Behörden über das Adressregister: kein rechtlicher Befund.
  - BT-Drs. 21/7319: keine Postfachfunktion in der EUDI-Wallet (D7319 S. 5).
- **P1 Benachrichtigung:**
  - D-Architektur: „Verwaltungskunden können selbst festlegen über welchen Kanal sie Benachrichtigungen … erhalten wollen“ (DA-Zus S. 19).
  - RG: Für § 37 Abs. 2a SGB X wäre der Zugang der Benachrichtigung nachzuweisen, ersatzweise der Abruf (RG Z. 1506ff.).
  - B-2026/18-IT nennt die Komponente „Postfach/ Benachrichtigung“.
- **Ü2 Vertraulichkeit:** E2EE ist rechtlich nicht vorgeschrieben, aber „praktisch geboten“. Empfohlen wird eine Verankerung per Verordnung oder Gesetz (RG Z. 2215ff., 2243ff.). ZaPuK gilt als nummernunabhängiger interpersoneller TK-Dienst mit Fernmeldegeheimnis (ÜH Z. 205ff.).
- **I2 / Aufbewahrung:** Es gibt keine gesetzliche Dauervorhaltepflicht. Empfohlen wird eine Regelung der Vorhalte-, Export- und Löschfristen (RG Tab. 2; ADR-1135).

## Governance, Kosten, Finanzierung, Zeitplan

- **Kosten (nur v1.0, Juni 2025)** [veraltet?]:
  - Bestandslösungen je Lösung 0,5–2,5 Mio. EUR/a Betrieb und 1–2 Mio. EUR/a Weiterentwicklung (Z-ZUS10 Z. 178–180).
  - Phase 2: ca. 1,7 Mio. EUR plus 3 VZÄ (Z. 190).
  - Zentrale Bausteine langfristig 1,8–2,3 Mio. EUR/a Betrieb, 1,5–2 Mio. EUR/a Weiterentwicklung, Folgeprojekte 1–1,3 Mio. EUR (Z. 198).
  - Phase 3 nicht schätzbar (Z. 194).
- **v1.1 nennt keine Zahlen.** Das Finanzierungskonzept kommt „erst zur 52. Sitzung“ (17.03.2027) (ÜH Z. 192–194; PR43 S. 14; B18 Z. 29–31; Z-README FAQ Z. 314–318). B-2026/43-IT trifft keine Finanzierungsentscheidung.
- **Finanzierungslogik laut RG:**
  - Die Finanzierung folgt der Vollzugszuständigkeit (Konnexität).
  - Der Bund finanziert, was er nach Art. 86/87 Abs. 3 GG selbst vollzieht, z. B. das Privatpersonen-Backend.
  - Für an die FITKO delegierte Teile kommt „keine alleinige Finanzierung durch den Bund in Betracht“; naheliegend ist der modifizierte Königsteiner Schlüssel (RG Z. 1127–1152, 1176).
  - Abschnitt E (19.10.2026): Bei ZaPuK als Produkt des IT-PLR ist unklar, ob § 9 Abs. 4 S. 1 Hs. 2 IT-StV Abweichungen vom Königsteiner Schlüssel auch für Entwicklung und Bereitstellung erlaubt; verfassungsrechtlich trüge Art. 91c GG „jedenfalls eine völlig einseitige Verortung von Verantwortlichkeiten und Lasten auf Seiten des Bundes nicht“ (RG Z. 3527–3545).
  - FITKO-Einordnung: § 9 Abs. 4 S. 1 Hs. 2 IT-StV eröffnet „Spielraum“ für abweichende Finanzierung; Grenzen aus Art. 91c GG und Finanzverfassung bleiben maßgeblich (EIN S. 4).
- **[Folgerung]** Der Bund finanziert FIT-Connect nach B-2026/18-IT Ziff. 3 bzw. B-2026/29-IT. Ob davon auch die bei FIT-Connect verorteten Organisations-Backends erfasst sind, ist offen.
- **Zeitplan:**
  - v1.0 sah die Migration der Bestandslösungen bis Ende 2027 und den Betrieb ab 2028 vor (Z-ZUS10 Z. 266) [veraltet?].
  - v1.1:
    - Quality Gate 2 am 25.11.2026 (51. Sitzung); Phase II laut B-2026/43-IT abgeschlossen (B43 Z. 10).
    - Phase III mit 3A „Zentrale Komponenten“ und 3B „Transition“ (BundID/ZBP, MJP, ELSTER, MUK), ohne Jahreszahlen (Abb. 4).
    - Projekt läuft bis Start Phase III „vsl. Anfang 2027“ (Z-README Z. 331).
    - Transitionsphasen: A 3–6 Monate, B 6–8 Monate, C 1–2 Monate, Abschaltung „nach 24 Monaten“ (TS Z. 65–71, Abb. 1).
    - Reihenfolge: Verbindlichkeit, zentrale Komponenten, Pilot Privatpersonen, Migration Privatpersonen, Organisationen, ELSTER, Abschaltung (ÜH Z. 190).
  - Rechtliche Termine: De-Mail-G außer Kraft 31.12.2026; § 3a–3c VwVfG n.F. ab 1.7.2027; FöMA Maßnahme 207 bis 30.09.2026 (FöMA S. 35); Bundesrat zum DIdG am 20.11.2026.
- **Bundestag:**
  - BT-Drs. 21/9020: „Postfach (in Ausgestaltung)“; Betriebs- und Finanzierungsmodell bis zur 52. Sitzung (D9020 S. 2).
  - BT-Drs. 21/7319: FITKO federführend, Anknüpfung an die EUDI-Wallet bei Identifizierung (S. 4–5).
  - Kein Haushaltstitel „Postfach“ im Bundeshaushalt 2027 gefunden **[Lücke]**.

## Abgleich morph_box.csv

- **I1, Option 4 „Verbund staatlicher Betreiber über Standard, getrennte Betreiber für Privatpersonen und Organisationen, Privatpersonen nur im zentralen Backend (ZaPuK ADR-1114)“:** Im Kern korrekt. Zwei Präzisierungen:
  - „staatlicher“ ist zu eng: Private Organisationen dürfen eigene Backends betreiben (ADR-0004, ADR-1145).
  - Die konkrete Zuordnung fehlt (v1.1 [Entwurf]): BMDS für Zugang und Privatpersonen-Backend, FITKO/FIT-Connect für die Organisations-Backends.
  - Vorschlag für den Klammerzusatz: „(ZaPuK ADR-1114; v1.1: BMDS für Privatpersonen, FIT-Connect/FITKO für Organisationen, eigene Backends Dritter zulässig)“.
- **I5, Option 4:** korrekt. Ergänzbar: RG empfiehlt einen dreistufigen Rahmen (Verordnung nach §§ 5, 6 OZG, IT-PLR-Beschluss, Anschlussvertrag) und für den ERV eine Zulassung per Verordnung. **Fehlende Option [Folgerung]:** „Zulassung per Rechtsverordnung (Anforderungen im Recht, z. B. beBPo § 6 ERVV, § 3c VwVfG n.F.)“. Sie ist im Zielbild nur für die ERV-Stufe empfohlen; der Verweis auf ADR-1145 Option B in Option 3 bleibt korrekt.
- **R1 (gesetzt):** Keine ZaPuK-Angabe. **[Folgerung]** Das Zielbild „beide Richtungen“ ist nach RG nicht durch ZaPuK allein erreichbar:
  - Hinkanal: ab 7/2027 Textform; elektronische Form über einen sicheren Übermittlungsweg erst nach Verordnung nach § 3c Abs. 2.
  - Rückkanal: nur QES/Siegel, solange § 3b Nr. 3 n.F. nicht erweitert ist.
  - Für existing_solutions: „ZaPuK selbst kein Schriftformersatz, transportiert QES-/Siegel-Dokumente (ADR-0017); als sicherer Übermittlungsweg nach § 3c VwVfG n.F. erst nach Rechtsverordnung“.
- **R2:** Keine ZaPuK-Angabe. Passend ist Option 1 („freiwillig“) mit der Besonderheit „Zugangseröffnung je Antrag / SSP-Registrierung“. Die RG-Empfehlung (Opt-in je Bereich) liegt nahe an Option 2 „Opt-in je Behörde“. **Mögliche neue Option:** „freiwillig, Zugangseröffnung konkludent je Vorgang, kein Adressregister für Personen, Erstkontakt nur nach Mitwirkung (ZaPuK)“.
- **R3, Option 4 „Behörde wählt je Nachricht … (ZaPuK absendergesteuerte Übermittlungsoptionen, ADR-1119)“:** korrekt. Der ZaPuK-Bezug fehlt aber auch in:
  - Option 1: Bekanntgabe mit Bereitstellungsbestätigung, ADR-0018;
  - Option 3: förmliche Zustellung nach § 5 Abs. 5 VwZG mit QES und aktiver Lesebestätigung als Empfangsbekenntnis, ADR-1118.
  - Im „Was wird entschieden“-Text sollte „Rechtsgrundlage … zu prüfen“ auf § 5 Abs. 5/7 VwZG verweisen (QES-Empfangsbekenntnis als Hürde).
- **R4:** Keine ZaPuK-Angabe. Heute ist das Option 1 („freiwillig“), Ziel Option 3. Hinweis: B-2026/18-IT Ziff. 5 verpflichtet die Länder nicht für das Postfach. Das RG (Z. 574) und PM50 sind hier ungenau.
- **„Wer entscheidet“ = BMDS (R1–R4):** Plausibel, weil die Federführung für Phase III beim BMDS liegen soll (ÜH Z. 41–42; PR43 S. 20; Beschluss nur „der Bund“, B43 Z. 12). Rechtlich entscheiden der Gesetzgeber (OZG, VwVfG, VwZG, AO, SGB X, ZPO), die Verordnungsgeber (BMI im Einvernehmen mit BMDS für § 3c VwVfG; BMJV für ZPO und ERVV), der IT-PLR (Standards, Anschlussregelwerk) und Bund/Länder per Staatsvertrag. **[Folgerung]**

## Widersprüche

- Länderpflicht: B18 Z. 33 (nur Ziff. 3) gegenüber PM50 Z. 21 und RG Z. 574 (auch ZaPuK).
- Federführung: B-2026/18-IT (Bund) gegenüber D7319 S. 5 und BRH S. 25 (FITKO) – Phasen-Unterschied **[Folgerung]**.
- Betreiber der Organisations-Backends: FIT-AB 2026/12 „voneinander unabhängige Betriebsorganisationen“ für Privatpersonen und Organisationen (FITAB Z. 15–26) passt zu v1.1. ADR-1114 legt sich bewusst auf keine Stelle fest.
- Stand des Architekturkonzepts: Das RG bewertet „Stand 8. September 2026“ (RG Z. 582), die Datei trägt „Stand 25.09.2026“ (AK Z. 1–8).
- Verfassungsrecht: RG hält für Bereitstellung und Finanzierung durch den Bund zusätzlich Art. 86 bzw. Art. 87 Abs. 3 GG für nötig (RG Z. 1170–1177); die FITKO hält für „klärungsbedürftig“, ob Art. 91c Abs. 5 GG allein eine zentrale Bereitstellung durch den Bund trägt (Vorbild § 3 Abs. 1 OZG, DIdG-E) (EIN S. 5). RG empfiehlt einen (zusätzlichen) Staatsvertrag, die FITKO fragt nach dessen Mehrwert gegenüber IT-StV und Produktmanagement-Modell (EIN S. 4); Abschnitt E hält zusätzliche staatsvertragliche Regelungen weiter für „vorzugswürdig“ (RG Z. 3565).
- Errata des RG (Z. 36–44) nennen nur Korrekturen vom 13.11.2026 und „Der Text des Gutachtens ist unverändert“, obwohl Stand (Z. 5: 19.10.2026) und Abschnitt E neu sind.

## Lücken

- B-2026/43-IT bestätigt v1.1 nicht ausdrücklich, legt keine verbindliche Konsolidierung fest und erteilt keinen Rechtsetzungs- oder Governance-Auftrag. Ob und wann das nachgeholt wird (etwa mit dem Finanzierungskonzept zur 52. Sitzung), ist offen. Ein Protokoll der 51. Sitzung lag am 2026-11-27 nicht vor.
- ZaPuK-Finanzierungskonzept und Betriebsmodell (52. Sitzung, 17.03.2027), Meilensteinplanung von Bund und Ländern (B18 Ziff. 6), D-Stack-Governance (B-2026/17-IT, 52. Sitzung): nicht öffentlich.
- Keine aktuellen Kostenzahlen nach v1.0; kein Haushaltstitel gefunden.
- Konkrete Betriebsstelle beim Bund (nachgeordnete Behörde? ITZBund? DCDS?) nicht benannt.
- Kein Gesetzentwurf zu FöMA Maßnahme 207 (Harmonisierung Bekanntgabe/Zustellung, Frist 30.09.2026) und kein Entwurf einer Verordnung nach § 3c Abs. 2 VwVfG n.F. gefunden.
- Kein Staatsvertragsentwurf (ZaPuK-StV oder D-Stack-StV) gefunden.
- Das RG behandelt Landesverfahrensrecht ausdrücklich nicht (Simultangesetzgebung erwartet, RG Z. 1204).
- Die DIP-Volltextsuche war ohne API-Schlüssel nicht möglich. Weitere Drucksachen mit „Postfach“ und D-Stack sind daher nicht ausgeschlossen.
- Anschlussvertrag und Nutzungsbedingungen (Fortschreibung der FIT-Connect-Nutzungsbedingungen, RG Z. 3382) existieren noch nicht.

## Änderungen v1.1.1 gegenüber den Entwurfskopien (Abgleich 2026-11-27)

- **ÜH** (`FITKO_ZaPuK-Ueberblick-Handlungsempfehlungen-v1-1_2026-10-06.md` → `../zapuk-v1.1.1/Zielarchitektur/Zusammenfassung/README.md`): gleiche Zeilenzahl, neue Z. = alte Z. − 5 (Kopf).
  - inhaltlich: Stand 06.10. → 12.10.2026 (Z. 5); Statushinweis jetzt „dem IT-Planungsrat zur 51. Sitzung vorgelegt“, Anlage zu B-2026/43-IT (Z. 7); Empfehlung Staatsvertrag ohne den Zusatz „ggf. übergreifend für den D-Stack“ (Z. 37, alt Z. 42).
  - redaktionell: Z. 19, 42, 120, 144 (Link), 174, 178, 180, 184, 198, 268, 274 (Link), 296 (Tippfehler, Grammatik).
- **RG** (`FITKO_ZaPuK-Rechtsgutachten-v1-1_2026-10-06.md` → `../zapuk-v1.1.1/Zielarchitektur/Rechtsgutachten/README.md`):
  - inhaltlich: Stand 06.10. → 19.10.2026 (Z. 5), Statushinweis mit B-2026/43-IT und Verweis auf die FITKO-Einordnung (Z. 7); neuer Abschnitt E (Z. 3485–3573) mit Inhaltsverzeichnis-Einträgen (Z. 550–569) und Fußnoten 410–427 (Z. 4468–4504).
  - redaktionell: Z. 19 (Link-Klammer).
  - Umrechnung: alte Z. 6–554 → neu Z. − 5; alte Z. 555–3469 → neu Z. + 15; alte Z. ab 3470 → neu Z. + 104. Der Text bis Abschnitt D ist unverändert.
- **Z-README** (`../zapuk-v1.0.0-121-g7419d8b/README.md` → `../zapuk-v1.1.1/README.md`):
  - inhaltlich: Z. 49 Status Phase II („sieht der IT-Planungsrat mit Beschluss 2026/43-IT als abgeschlossen an“, v1.1 „vorgelegt“ statt „soll … verabschiedet“ werden); Z. 68 neuer Beschluss-Eintrag; Z. 70–80 neuer Abschnitt „Versionen“; Z. 90–103 neuer Logbuch-Eintrag B-2026/43-IT mit Anlagen; Z. 331 FAQ Projektende („wurden … eingereicht“, Phase II abgeschlossen); Z. 243 Anforderungsliste „aktuell“ v0.7.1 statt v0.5.1; Anlagen zu B-2025/28 jetzt als Links auf it-planungsrat.de statt lokaler PDFs (Z. 169–174); Z. 39, 57 „Basisdienst ‚Postfach und Interaktion‘“ statt „Basiskomponente ‚Postfach/Benachrichtigung‘“.
  - redaktionell: Z. 144, 207, 250, 258, 285, 342, 348.
  - Umrechnung: alte Z. 1–67 gleich; alte Z. 68–76 → + 13; alte Z. ab 77 → + 27.
- **Z-ZUS10** (`../zapuk-v1.0.0-121-g7419d8b/Zielarchitektur/Zusammenfassung/README.md` → `../zapuk-v1.1.1/Zielarchitektur/Zusammenfassung/Vorversionen/v1.0/README.md`): zeilengleich, nur Links geändert (Z. 7, 18–20, 166, 324–325, 333). Die PDF-Fassung liegt nicht mehr im Repo, sondern unter <https://www.it-planungsrat.de/fileadmin/beschluesse/2025/Beschluss_2025_28_ZaPuK_%C3%9Cberblick.pdf>.
- **Abbildungen:** `FITKO_ZaPuK-v1-1_Abb2-…png` und `…_Abb4-…png` sind bytegleich mit `../zapuk-v1.1.1/Zielarchitektur/Zusammenfassung/images/image2.png` und `image4.png`.

## Konkordanz: Fundstellen in `../../../Referenzen-ZaPuK.md` und `../../ZaPuK-Zusammenfassung.md` (Recht, Governance, Überblick)

Alte Zeilen beziehen sich auf die Entwurfskopien in diesem Ordner (ÜH, RGA) bzw. auf `../zapuk-v1.0.0-121-g7419d8b/` (ZF10, ZK-README). „gleich“ = Zeileninhalt unverändert (zeilenweise per `difflib` und Sichtprüfung). Pfade relativ zu `sources/ZaPuK/quellen/zapuk-v1.1.1/`. Transitionsstrategie und Transitionspläne (TS, TA, TB, TC, TE1, TE2): `../umsetzung-rezeption/_befunde.md`, Abschnitt „Konkordanz“.

| Fundstelle | Kürzel, alte Z. | Neuer Pfad | Neue Z. | Inhalt |
|---|---|---|---|---|
| Referenzen, P4 | `AT/fitko_zapuk-zusammenfassung-v1.1_…` Z. 115 | `Zielarchitektur/Zusammenfassung/README.md` | Z. 110 | gleich |
| Referenzen, P9 | RGA Tab. 2 (Z. 630ff.) | `Zielarchitektur/Rechtsgutachten/README.md` | Tab. 2 (Z. 645ff.) | gleich |
| Referenzen, Ü2 | RGA Z. 2200ff. | `Zielarchitektur/Rechtsgutachten/README.md` | Z. 2215ff. | gleich |
| Referenzen, I1 | ÜH Z. 53 | `Zielarchitektur/Zusammenfassung/README.md` | Z. 48 | gleich; Status überholt: die vier erbetenen Entscheidungen sind in B-2026/43-IT nicht getroffen (B43 Z. 10–12) |
| Referenzen, I1; Zusammenfassung Z. 64 | ÜH Z. 133–139 | `Zielarchitektur/Zusammenfassung/README.md` | Z. 128–134 | gleich (Empfehlung, nicht beschlossen) |
| Referenzen, I1; Zusammenfassung Z. 73 | ÜH Z. 197–199 | `Zielarchitektur/Zusammenfassung/README.md` | Z. 192–194 | gleich |
| Referenzen, I1; Zusammenfassung Z. 70 | RGA Z. 1155–1162 | `Zielarchitektur/Rechtsgutachten/README.md` | Z. 1170–1177 | gleich; FITKO-Einordnung (EIN S. 4–5) und Abschnitt E (Z. 3485–3573) ergänzen |
| Referenzen, I5 | RGA Z. 3240ff. | `Zielarchitektur/Rechtsgutachten/README.md` | Z. 3255ff. | gleich |
| Referenzen, I5 | RGA Z. 3351–3466 | `Zielarchitektur/Rechtsgutachten/README.md` | Z. 3366–3481 | gleich |
| Referenzen, R1; Zusammenfassung Z. 144 | RGA Z. 1544–1600 | `Zielarchitektur/Rechtsgutachten/README.md` | Z. 1559–1615 | gleich |
| Referenzen, R1 | RGA Tab. 2, Z. 3125ff. | `Zielarchitektur/Rechtsgutachten/README.md` | Tab. 2 (Z. 645ff.), Z. 3140ff. | gleich |
| Referenzen, R2; Zusammenfassung Z. 145 | RGA Z. 1191–1240 | `Zielarchitektur/Rechtsgutachten/README.md` | Z. 1206–1255 | gleich |
| Referenzen, R2 | ÜH Z. 214 | `Zielarchitektur/Zusammenfassung/README.md` | Z. 209 | gleich |
| Referenzen, R3 | RGA Z. 1298ff. | `Zielarchitektur/Rechtsgutachten/README.md` | Z. 1313ff. | gleich |
| Referenzen, R3; Zusammenfassung Z. 146 | RGA Z. 1435–1543 | `Zielarchitektur/Rechtsgutachten/README.md` | Z. 1450–1558 | gleich |
| Referenzen, R4 | ÜH Z. 53 | `Zielarchitektur/Zusammenfassung/README.md` | Z. 48 | gleich; Vorschlag nicht beschlossen |
| Referenzen, R4 | ÜH Z. 149 | `Zielarchitektur/Zusammenfassung/README.md` | Z. 144 | redaktionell geändert (Link auf die Transitionsstrategie), Aussage gleich |
| Referenzen, R4; Evidenzstufen R4 | ÜH Z. 199 | `Zielarchitektur/Zusammenfassung/README.md` | Z. 194 | gleich |
| Referenzen, R4; Zusammenfassung Z. 147 | RGA Z. 1060, Z. 1160–1161 | `Zielarchitektur/Rechtsgutachten/README.md` | Z. 1075, Z. 1175–1176 | gleich; ergänzend Z. 3511 (keine Pflicht allein über IT-StV), Z. 3559 |
| Referenzen, Abweichungen; Zusammenfassung Z. 155 | RGA Z. 559 | `Zielarchitektur/Rechtsgutachten/README.md` | Z. 574 | gleich |
| Referenzen, Evidenzstufen B2 | RGA Z. 1233 | `Zielarchitektur/Rechtsgutachten/README.md` | Z. 1248 | gleich |
| Referenzen, Abweichungen B1 A3; Zusammenfassung Z. 157 | ZF10 Z. 232 | `Zielarchitektur/Zusammenfassung/Vorversionen/v1.0/README.md` | Z. 232 | gleich (v1.0); v1.1-Beleg für FIT-Connect als Anbindungsweg: ÜH Z. 94, Z. 310; TS Z. 34; PR43 S. 18 |
| Zusammenfassung Z. 72 | ZF10 Z. 198 | `Zielarchitektur/Zusammenfassung/Vorversionen/v1.0/README.md` | Z. 198 | gleich (v1.0, weiterhin einzige Kostenzahl) |
| Zusammenfassung Z. 51 | `ZK/README.md` Z. 39 | `README.md` | Z. 39 | geändert: „Basisdienst ‚Postfach und Interaktion‘“ statt „Basiskomponente ‚Postfach/Benachrichtigung‘“, Aussage sonst gleich |
| Zusammenfassung Z. 51 | `ZK/README.md` Z. 53 | `README.md` | Z. 53 | gleich |
| Zusammenfassung Z. 60 | `ZK/README.md` Z. 49 | `README.md` | Z. 49 | **inhaltlich geändert:** Phase II abgeschlossen (B-2026/43-IT), v1.1 nur „vorgelegt“; die Aussage „soll Version 1.1 … verabschiedet und Phase III … beschlossen werden“ ist entfallen |
| Zusammenfassung Z. 60 | `ZK/README.md` Z. 304 | `README.md` | Z. 331 | **inhaltlich geändert:** „wurden … eingereicht“, Phase II abgeschlossen; Start Phase III „vsl. Anfang 2027“ gleich |
| Zusammenfassung Z. 60 | ÜH Z. 53 | `Zielarchitektur/Zusammenfassung/README.md` | Z. 48 | gleich; Status überholt |
| Zusammenfassung Z. 68 | ÜH Z. 305 | `Zielarchitektur/Zusammenfassung/README.md` | Z. 300 | gleich |
| Zusammenfassung Z. 74 | RGA Z. 1112–1137 | `Zielarchitektur/Rechtsgutachten/README.md` | Z. 1127–1152 | gleich; ergänzend Abschnitt E III.3 (Z. 3527–3545) |
| Zusammenfassung Z. 35 (Kürzeltabelle) | ZF10 / ÜH, RGA | `Zielarchitektur/Zusammenfassung/Vorversionen/v1.0/README.md` / `Zielarchitektur/Zusammenfassung/README.md`, `Zielarchitektur/Rechtsgutachten/README.md` | – | Pfade ersetzen; ÜH und RGA nicht mehr **[Entwurf]** |
