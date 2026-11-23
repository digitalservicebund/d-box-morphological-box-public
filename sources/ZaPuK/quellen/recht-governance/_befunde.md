# Befunde ZaPuK – Recht, Governance, Finanzierung (Stand 2026-11-19)

Fundstellen: Datei (Kurzname) + Z. (Zeile) / S. (PDF-Seite) / Ziff. Kennzeichnung: **[Folgerung]** = eigene Ableitung, **[unverifiziert]** = nicht durch Primärquelle belegt, **[veraltet?]** = Quelle vermutlich überholt, **[Entwurf]** = ZaPuK-Dokument v1.1, vom IT-Planungsrat noch nicht beschlossen (Vorlage 51. Sitzung am 25.11.2026).

Kurzformen:

- RG = `FITKO_ZaPuK-Rechtsgutachten-v1-1_2026-10-06.md` [Entwurf]
- ÜH = `FITKO_ZaPuK-Ueberblick-Handlungsempfehlungen-v1-1_2026-10-06.md` [Entwurf]
- AK = `FITKO_ZaPuK-Architekturkonzept-v1-1_2026-09-25.md` [Entwurf]
- FITAB = `FITKO_FIT-AB-Beschluesse_2026-11-19.md`
- Z-README = `../../../zapuk-main/README.md`; Z-ZUS10 = `../../../zapuk-main/Zielarchitektur/Zusammenfassung/README.md` (v1.0, Juni 2025); ADR-xxxx = `../../../zapuk-main/Zielarchitektur/Architekturentscheidungen/ADR-xxxx-*.md`; ANF = `../../../zapuk-main/Konsolidierte-Anforderungen/README.md`
- B18 = `../../../BundID-ZBP/quellen/recht-governance/IT-PLR_B-2026-18-IT_D-Stack_Finanzierungsmodell.md`; B03 = `…/IT-PLR_B-2026-03-IT_Deutschland-Stack.md`; B2528 = `…/IT-PLR_Beschluss_2025-28_ZaPuK.md`; FöMA = `…/Foederale_Modernisierungsagenda_MPK_2025-12-04.pdf`; D9020 = `…/BT-Drs_21-9020_Antwort_KA_Deutschland-Stack.pdf`
- PM50 = `IT-PLR_Pressemitteilung_50-Sitzung_2026-06-17.md`; D7319 = `BT-Drs_21-7319_Antwort_KA_EUDI-Wallet.pdf`; DA-Zus = `IT-PLR_Beschluss_2026_04_Deutschland-Architektur_Zusammenfassung.pdf`

## Kern: Status und Rechtsgrundlage

- **Beschlusslage:** B-2024/28-IT (Auftrag Zielarchitektur), B-2025/28-IT (v1.0 beschlossen, Phase II an FITKO; Ziff. 6: FITKO soll bis zur 50. Sitzung Rechtsgrundlage und Finanzierung des gemeinschaftlichen Betriebs klären, B2528 Z. 38ff.), B-2026/03-IT (Plattformkern mit Basisdienst „Postfach“; Bund, Länder, Kommunen „streben … an“, B03 Z. 21–27), B-2026/04-IT (Funktionsbaustein „Postfach und Interaktion (G2C)“, DA-Zus S. 19), B-2026/18-IT (fachliche Verantwortung des Bundes für „Postfach/ Benachrichtigung (ZaPuk)“, B18 Z. 23; Betriebs- und Finanzierungsmodell bis zur 52. Sitzung, Z. 29–31).
- **FIT-AB 2026/12 (15.09.2026, einstimmig):** empfiehlt Aufbau auf Basis ZaPuK, Phase III „im Rahmen des D-Stacks unter Koordination des Bundes“, zentrales Postfach-Frontend in Bundesportal/Deutschland-App, zentrale Backends für Privatpersonen und Organisationen „in voneinander unabhängigen Betriebsorganisationen“, FIT-Connect-SSP als Rollout-Infrastruktur, Anpassung von BundID/ZBP, MJP, MUK und ELSTER-Fachportal (FITAB Z. 15–26).
- **Version 1.1 und Phase III sind noch nicht beschlossen.** Die 51. Sitzung findet am 25.11.2026 statt (`IT-PLR_Sitzungstermine_2026-11-19.md`). Der IT-PLR soll über vier Punkte entscheiden: Bestätigung v1.1, „verbindliche Festlegung, dass die bestehenden Postfachlösungen hin zur ZaPuK konsolidiert werden“, Phase III unter Federführung des BMDS, Beauftragung von Rechtsetzungs- und Governance-Schritten (ÜH Z. 53).
- **Rechtsgutachten (Phase II, AP 2.7):** Es ist öffentlich, aber bisher nur im Branch `architecture-v1.1` (RG Z. 1–40). Es wurde im Auftrag der FITKO von Eichler Kern Klein und PLANIT // LEGAL erstellt und bewertet v1.1 (Stand 08.09.2026). Ergebnis: ZaPuK ist „grundsätzlich realisierbar“ (ÜH Z. 210ff.). Eine einzelne Verfassungsnorm reicht nicht, möglich ist eine **Kombination** (RG Z. 1155–1162):
  - Art. 86 GG: zentrale Softwareentwicklung beim Bund.
  - Art. 91c Abs. 5 GG: Bundesgesetz zur Postfachstruktur „einschließlich … Anschlussverpflichtung von Ländern und Gemeinden“ (Z. 1160).
  - Art. 87 Abs. 3 GG: Der Bund kann den Betrieb des Privatpersonen-Backends an sich ziehen und auf eine nachgeordnete Behörde im Geschäftsbereich des BMDS übertragen; daraus folgt seine Finanzierungspflicht (Z. 1051–1060).
  - Art. 91c Abs. 1/2 GG: Staatsvertrag (eigener ZaPuK-StV, D-Stack-StV in Anlehnung an den NOOTS-StV oder Ergänzung des IT-StV). Damit kann der Betrieb der Organisations-Backends an die FITKO delegiert werden (Z. 1161–1162).
  - Für die Justiz ist Art. 74 Abs. 1 Nr. 1 GG nötig (Z. 1051ff.).
  - Eine Grundgesetzänderung ist nicht zwingend, wäre aber „wünschenswert“ (Art. 91c GG neu, FöMA Maßnahme 201; RG Z. 1164–1168).
- **Kein Postfach-Gesetz und keine D-Stack-Rechtsgrundlage gefunden.** Das DIdG (BR-Drs. 618/26, Bundesrat 20.11.2026) regelt Bürgerkonto und EUDI-Wallet, ein ZaPuK-Postfach nicht. RG empfiehlt seine Neufassung von § 3 OZG als Anknüpfung für die Identifizierungsmittel des ZaPuK-Postfachs (RG Tab. 2, Z. 630ff.; ÜH Z. 218). **[Folgerung]**

## Befunde je morph_box-Zeile

### I1 Betrieb und Governance

- **Empfohlene Verantwortlichkeiten (v1.1)** [Entwurf] (ÜH Z. 133–139, Abb. 2; AK Z. 307–366):
  - **BMDS:** Zentraler Postfach-Zugang und Backend für Privatpersonen, Betrieb und Entwicklung des Zugangs, „gegebenenfalls delegiert an eine nachgeordnete Behörde oder einen Dienstleister“. Begründung u. a.: „Der Bund kommt seiner Bereitstellungs-Verantwortung für das Postfach des Bürgerkontos gemäß § 3 Abs. 1 OZG nach“ (AK Z. 321).
  - **FITKO-Produktmanagement (FIT-Connect):** Entwicklung aller Backends; Betrieb der drei Organisations-Backends (private Organisationen, öffentliche Stellen, Justiz, gebündelt bei einer Stelle); SSP, SDK und Adressregister.
  - **BLK Justiz:** SAFE-Register.
  - **Steuerung:** „Governance des D-Stacks (in Ausgestaltung)“. Neue Organisationen sind nicht vorgesehen (ÜH Z. 133–139).
- **ITZBund wird in v1.1 nicht genannt.** **[Folgerung]** Nahe liegt eine Delegation an eine BMDS-nachgeordnete Stelle; das ITZBund betreibt heute die BundID. Beleg dafür fehlt **[unverifiziert]**.
- ADR-1114 legt nur die Trennung fest, keine konkreten Betreiber (ADR-1114 Z. 17). v1.1 und RG Z. 573 ordnen die Betreiber zu: Organisations-Backends „durch FIT Connect“, Privatpersonen-Backend „durch den Bund“.
- **Rolle der Länder:**
  - Phase I/II: Federführung gemeinsam bei FITKO, Hamburg und Sachsen-Anhalt (Z-README Z. 57).
  - Phase III: Steuerungskreis aus BMDS, FITKO und Ländervertreter:innen, dazu ein Sounding Board. IT-PLR bzw. AL-Runde sind übergeordnet; bei föderaler Bebauung wird das FIT-AB eingebunden (ÜH Z. 305).
  - Dauerhaft: Produktgovernance nach FITKO-Produktmanagement-Modell. Empfohlen wird eine gemeinsame Governance der Bausteine „Datenaustausch“ und „Postfach und Interaktion“ (ÜH Z. 313–315).
  - Das „Digital Center Deutschland Stack“ (DCDS) soll „Umsetzungsmuskel“ für D-Stack-Produkte des Bundes werden (ÜH Z. 323).
  - Betriebsplattform ist die Deutsche Verwaltungscloud (ÜH Z. 315).
- **Dritte:** Organisationen und private Dritte dürfen eigene Backends betreiben (ADR-0004), Privatpersonen nicht (ADR-1114 Z. 72, 78).
- **Rechtliche Voraussetzung der Betriebsübernahme:**
  - Die Übernahme der MUK-Postfächer durch FIT-Connect „erfordert die Anpassung der Verordnung nach § 3 Abs. 2 OZG“ (AK Z. 365).
  - Datenschutzrechtliche Verantwortung der Backend-Betreiber: Zuweisung per Rechtsvorschrift nach dem Vorbild von § 8a Abs. 4 OZG empfohlen (RG Z. 1840ff.).
  - Gemeinsame Vollzugsverantwortung mit der FITKO braucht eine Legitimation über Art. 91c Abs. 1 GG (Verbot der Mischverwaltung, RG Z. 1051ff.).
- **Widerspruch zur Zuständigkeit:** BT-Drs. 21/7319 (Juli 2026) nennt die FITKO als „federführend zuständige Stelle“ für ZaPuK (D7319 S. 5). Der BRH (31.07.2026, nicht abgelegt, S. 25) schreibt „Zuständig ist die FITKO“. Seit B-2026/18-IT trägt aber der Bund die fachliche Verantwortung. **[Folgerung]** Die Angaben beschreiben Phase II und sind für Phase III **[veraltet?]**.

### I5 Aufnahme in den Verbund

- ADR-1145 (Option C, risikobasiert): Anschlussvertrag im SSP, Föderationsliste, Suspendierung und Ausschluss. „Die rechtliche Ausgestaltung ist zu klären“ (ADR-1145 Z. 83). Offen ist, wer über Suspendierung und Ausschluss entscheidet (Z. 163).
- AK v1.1: Backends werden „durch die Verbund-Governance in einem manuellen Freigabeschritt“ in die Föderationsliste aufgenommen (AK Z. 582).
- **RG bestätigt den Ansatz als „rechtlich tragfähig“ und „einer Zertifizierung nach dem Vorbild des § 325 SGB V oder der De-Mail-Akkreditierung vorzuziehen“.** Es empfiehlt einen **dreistufigen Rahmen** (RG Z. 3453ff.; Tab. 2 Z. 630ff.):
  1. Rechtsverordnung nach §§ 5, 6 OZG mit Wirkung gegenüber allen Stellen.
  2. IT-PLR-Beschluss nach § 2 IT-StV.
  3. Anschlussvertrag im SSP für Betreiberpflichten, Haftung und Föderationskontrolle. Das Regelwerk soll der IT-PLR beschließen, nicht der Betreiber.
- **Vorbild Verbindungsnetz (IT-NetzG):** Auskunftspflicht mit Zwei-Wochen-Frist; gestuftes Ausschlussverfahren (Stellungnahme, Abmahnung, Kündigung im Einvernehmen mit einem IT-PLR-Gremium); Haftungs-Durchleitung (RG Z. 3405–3420, 3453ff.).
- **Grenzen von Verträgen:** keine Bindung von Ländern und Kommunen, die nicht beitreten; keine Drittwirkung; kein Ersatz für die Anerkennung als sicherer Übermittlungsweg oder qualifizierter Zustelldienst (RG Z. 3351ff., 3433ff.). Die ITSiV-PV gilt schon heute: § 2 Abs. 6 S. 2 nennt das Postfach ausdrücklich (Pentest und Webcheck vor Anbindung).
- **Formfrage:** Bei öffentlich-rechtlicher Ausgestaltung gilt § 57 VwVfG (Schriftform). Ein reiner Klick im SSP genügt nicht; sonst droht Nichtigkeit (RG Z. 3453ff.).
- **ERV:** Die ERV-Fähigkeit soll als eigene Anschlussstufe mit Regel-Konformitätsprüfung geführt werden. Empfohlen wird ein Zulassungsverfahren per Rechtsverordnung nach § 130a Abs. 4 S. 1 Nr. 5 ZPO (RG Z. 3240ff.; in RG Tab. 2 als „§ 30a“ verschrieben).

### R1 Schriftformersatz

- **ZaPuK selbst ersetzt die Schriftform nicht.**
  - Behördenseitig gilt der Ersatz derzeit nur durch QES oder qualifiziertes Siegel (§ 3a Abs. 2/3 Nr. 3 VwVfG, § 9a Abs. 6 OZG, § 87a Abs. 4 AO, § 36a SGB I). ZaPuK übermittelt solche Dokumente unverändert (ADR-0017: Siegelung durch den Absender vor dem Versand; RG Z. 1544ff., 1594ff.).
  - „Für den Schriftformersatz ist eine Beweiskraft gegenüber Dritten erforderlich“ (ADR-0017 Z. 11).
- **Hinkanal heute:** QES; Online-Formular mit eID (§ 3a Abs. 3 Nr. 1 VwVfG, § 9a Abs. 5 OZG); EGVP-Postfächer nach § 3a Abs. 3 Nr. 2 VwVfG. Ein ZaPuK-Postfach ist nicht genannt (RG Z. 1544ff.).
- **Ab 1.7.2027** (Gesetz für den Bürokratierückbau im Bereich des BMI vom 3.7.2026, RG Z. 1570ff.):
  - § 3a Abs. 2 VwVfG n.F.: Die Schriftform wird schon durch ein „elektronisches Dokument“ (Textform) gewahrt, soweit das Fachrecht nichts anderes bestimmt.
  - Wo das Fachrecht die elektronische Form verlangt (§ 3b n.F.), gibt es drei Wege: Formular nach Identifizierung, QES oder Siegel, sicherer Übermittlungsweg nach § 3c.
  - Die Anerkennung von ZaPuK als sicherer Übermittlungsweg setzt eine Rechtsverordnung des BMI im Einvernehmen mit dem BMDS nach § 3c Abs. 2 VwVfG n.F. voraus. Die materiellen Kriterien erfülle ZaPuK bereits (RG Z. 1594ff., 3453ff.).
  - **§ 3b S. 1 Nr. 3 n.F. erfasst nur das „Einreichen“, also den Hinkanal.** RG empfiehlt, ihn vor Inkrafttreten auf behördenseitige Übermittlung zu erstrecken und Parallelregeln in § 87a AO und § 36a SGB I zu schaffen (RG Z. 1590).
- **Gegenüber Gerichten (ERV):** ZaPuK wäre sicherer Übermittlungsweg erst nach einer Verordnung nach § 130a Abs. 4 S. 1 Nr. 5 ZPO. Nr. 4 (Nutzerkonto) entfällt mit der Entkopplung; dafür sind Anpassungen in ERVV § 11, § 13 nötig, und Matrix muss anstelle von OSCI anerkannt werden. Die einfache Signatur bleibt zwingend (RG Tab. 2; Z. 3125ff.).
- **[Folgerung]** Das morph_box-Zielbild „ersetzt die Schriftform in beide Richtungen“ ist mit ZaPuK nach geltendem Recht nur über QES/Siegel erreichbar. Ab 7/2027 reicht im Hinkanal überwiegend Textform. Für den Rückkanal ist eine Gesetzesänderung nötig (Erstreckung § 3b Nr. 3, AO, SGB I).

### R2 Teilnahmepflicht

- **Freiwilligkeitsgrundsatz:** Elektronische Übermittlung setzt einen vom Empfänger eröffneten Zugang voraus (§ 3a Abs. 1 VwVfG, § 36a SGB I, § 87a Abs. 1 AO, § 5 Abs. 5 VwZG). Eine Pflicht für Bürger:innen wurde 2002 und 2013 bewusst nicht geschaffen; die Nutzung des Bürgerkontos ist freiwillig (§ 3 Abs. 1 S. 2 OZG) (RG Z. 1191–1211).
- **§ 3a Abs. 1 VwVfG n.F. (ab 1.7.2027):** Kommunikation „soll“ elektronisch erfolgen, gilt aber nur bei bereits eröffnetem Zugang. Die Zugangseröffnung durch die Privatperson bleibt laut Gesetzesbegründung erforderlich; Papier bleibt wählbar (RG Z. 1223ff.).
- **ZaPuK-Umsetzung:**
  - **Privatpersonen:** konkludente Zugangseröffnung durch Antragstellung; das bereichsspezifische Pseudonym dient als Rückantwortadresse. Proaktive Kontaktaufnahme ist nur mit einer Adresse aus einem früheren Vorgang möglich, sonst erst nach Authentisierung. Eine „kalte“ Erstansprache ist rechtlich unzulässig und technisch nicht möglich, weil es kein Adressregister für Privatpersonen gibt.
  - **Organisationen:** Registrierung im SSP mit Eintrag im Adressregister gilt als konkludente Zugangseröffnung (RG Z. 1235ff.; ÜH Z. 214; ADR-1148: Registrierung = Opt-in für Adressierbarkeit).
- **Empfohlene Rechtsänderung für proaktive Zustellung** (RG Z. 1231–1233):
  - spezialgesetzliche Fiktion nach dem Vorbild von § 9 OZG und § 122a AO, begrenzt auf Verfahren und Nachrichtentypen;
  - **Opt-in je Bereich**, einmalig und widerruflich, mit Benachrichtigung über einen zweiten Kanal;
  - funktionierende Schlüsselwiederherstellung;
  - Rückfallebene Papier.
- **[Folgerung]** ZaPuK entspricht heute „freiwillig, Zugang je Antrag“. Die Empfehlung zielt auf „Opt-in je Bereich“, also nicht auf eine Pflicht. Der Ausschluss eines Bürger-Adressregisters schließt „Pflicht für alle“ (Dänemark-Modell) faktisch aus.
- Unternehmen: Keine ZaPuK-spezifische Pflicht gefunden. Die Digital-only-Regel des OZGÄndG für Unternehmensleistungen erwähnt nur die Rahmenbedingungen (`../../../zapuk-main/Zielarchitektur/Rahmenbedingungen/README.md` Z. 54).

### R3 Bekanntgabe und förmliche Zustellung

- **Bekanntgabe** (RG Z. 1298ff., 1435ff., 1491ff.):
  - Bei dezentralen Organisations-Backends: Übermittlungsvariante nach § 41 Abs. 2 VwVfG, § 37 Abs. 2 SGB X, § 122 Abs. 2a AO. Der Zugang wird durch die serverseitige Bereitstellungsbestätigung (ADR-0018) belegt.
  - Bei zentralen Backends: Abrufmodelle (§ 41 Abs. 2a VwVfG mit Abrufnachweis, § 9 OZG, § 122a AO, § 37 Abs. 2a/2b SGB X).
  - ZaPuK muss Bereitstellung und Zeitpunkt, Abruf und möglichst den Zugang der Benachrichtigung nachweisen. Das leisten die Bereitstellungsbestätigung (ADR-0018), die Abrufbestätigung (ADR-1118) und die absendergesteuerten Optionen (ADR-1119).
  - Der Anhang muss im Empfänger-Backend liegen, ein Link auf das Absender-Repository genügt nicht (ADR-1120).
  - Verschlüsselungsfehler sollen verhindert oder revisionssicher dokumentiert werden.
- **§ 9 OZG gilt nur für das Postfach eines Nutzerkontos.** Für das entkoppelte ZaPuK-Postfach greift sonst § 41 Abs. 2a VwVfG mit tatsächlichem Abruf. Daher müssen § 9 OZG und § 122a Abs. 5 AO angepasst werden (ADR-0018 Z. 234; ADR-1110 Z. 84; RG Tab. 2).
- **Harmonisierung (FöMA Maßnahme 207, Frist laut FöMA 30.09.2026):** RG empfiehlt ein einheitliches Bekanntgabeereignis (z. B. Bereitstellung) mit Fiktion, zentral im VwVfG, mit Verweisen im Fachrecht (RG Z. 1502ff.). ÜH nennt Anpassungsbedarf in OZG, AO, VwVfG des Bundes und der Länder, VwZG des Bundes und der Länder sowie SGB X (ÜH Z. 42). **[Lücke]** Ein Gesetzentwurf zu Maßnahme 207 wurde nicht gefunden.
- **Förmliche Zustellung (VwZG)** (RG Z. 1513ff., 1539ff.):
  - **§ 5 Abs. 5 VwZG ist schon heute die Rechtsgrundlage:** elektronische Zustellung gegen Empfangsbekenntnis bei eröffnetem Zugang, mit QES am Dokument.
  - Das elektronische Empfangsbekenntnis braucht eine QES des Empfängers; eine automatische Bestätigung genügt nicht.
  - Eine Fiktion am 4. Tag gibt es nur, wenn das Verfahren auf Verlangen des Empfängers elektronisch geführt wird (§ 5 Abs. 7 S. 2).
  - § 5 Abs. 4 VwZG erleichtert die Zustellung an Behörden, Anwälte usw.
  - § 5a VwZG (De-Mail) ist gestrichen; das De-Mail-G tritt mit Ablauf des 31.12.2026 außer Kraft.
  - ZaPuK bietet eine aktive Lesebestätigung, erweiterbar zum Empfangsbekenntnis; das Zustelldatum soll frei eintragbar sein (ADR-1118 Z. 96–110).
- **Qualifizierter Zustelldienst (eIDAS Art. 43/44):** ADR-0018 orientiert sich an ERDS. Die Vermutungswirkung nach Art. 43 Abs. 2 setzt den QTSP-Status voraus. Empfohlen wird, ihn bei den zentralen Betreibern bzw. einem Quittungs- und Zeitstempeldienst zu bündeln (RG Z. 3453ff.; ADR-0018 Z. 11; ANF_FUN_USE_077).
- **[Folgerung]** Die morph_box-Formulierung „Rechtsgrundlage … ist zu prüfen“ greift zu kurz. Laut RG besteht sie (§ 5 Abs. 5/7 VwZG); praktisches Hindernis ist die QES-Pflicht für das Empfangsbekenntnis von Bürger:innen.

### R4 Teilnahme der Behörden

- **Heute keine Teilnahmepflicht für ZaPuK.**
  - B-2026/03-IT: Bund, Länder und Kommunen „streben die Nutzung … an“ (B03 Z. 24–27).
  - B-2026/18-IT Ziff. 5: Die Pflicht der Länder zu Anbindung, Flächendeckung und Nutzung gilt nur für die Basiskomponenten der **Ziff. 3** (eID/EUDI-Wallet, FIT-Connect, NOOTS), **nicht für Postfach/Benachrichtigung** (B18 Z. 33).
  - Meilensteinplanung des Bundes und der Länder bis zur 52. Sitzung (B18 Z. 37–40).
- **Widerspruch:**
  - PM50 Z. 21 („Die Länder haben eine verbindliche Anbindung und Flächendeckung der Komponenten zugesagt“) ist pauschal formuliert.
  - Das Rechtsgutachten (RG Z. 559) liest Ziff. 5 so, dass sich die Länder für ZaPuK „verbindlich“ zur Anbindung verpflichten. **[Folgerung]** Das deckt der Beschlusswortlaut nicht.
- **Geplant [Entwurf]:** „Verbindliche Festlegung, dass die bestehenden Postfachlösungen hin zur ZaPuK konsolidiert werden“ (ÜH Z. 53). Die Transition setzt „verbindliche Grundsatzentscheidungen der zuständigen Stellen“ voraus (ÜH Z. 149, 195).
- **Rechtliche Wege (RG):**
  - Art. 91c Abs. 5 GG trägt eine bundesgesetzliche Anschlussverpflichtung von Ländern und Kommunen. Für Kommunen gilt das nur ohne erhebliche, einer Aufgabenübertragung vergleichbare Lasten und mit Gestaltungsspielraum (RG Z. 1060, 1160).
  - Ein Staatsvertrag ermöglicht „verbindliche Vorgaben über die Ablösung bestehender Postfach- und Kommunikationslösungen“ (Z. 1161).
  - IT-PLR-Standards nach § 2 IT-StV und Verordnungen nach §§ 5, 6 OZG binden alle Stellen (Z. 3433ff.).
  - § 2 Abs. 1 EGovG verpflichtet Bundesbehörden bereits, einen elektronischen Zugang zu eröffnen, legt aber kein Verfahren fest (RG Z. 1209).
- **Justiz:** Eigene IT-Governance über den E-Justice-Rat (RG Z. 2901ff.). ZaPuK will „nicht in die Entscheidungsprozesse der Justizverwaltung eingreifen“ (AK Z. 354). FIT-AB empfiehlt eine Zusammenarbeit IT-PLR/E-Justice-Rat (FITAB Z. 15–26).
- **Anreize und Finanzierung für Behörden:** offen (Finanzierungskonzept zur 52. Sitzung). Kostenfreiheit gilt nur für Privatpersonen (ANF RAH_ÜBE_148/149).
- **[Folgerung]** ZaPuK-Ziel ist die Option „verbindliche Teilnahmepflicht“, rechtlich noch nicht hergestellt. Stichtage gibt es nicht; ein einheitlicher Stichtagswechsel ist ausdrücklich nicht vorgesehen (ÜH Z. 149).

### Weitere Zeilen

- **P9 Postfach und Nutzerkonto:** Die Entkopplung (ADR-1110) erfordert Änderungen an § 2 Abs. 7, § 9 OZG und § 122a Abs. 5 AO sowie an § 130a Abs. 4 Nr. 4 ZPO und den Parallelnormen (RG Tab. 2; ADR-1110 Z. 70, 84).
- **P7 Antwort und Erstkontakt:**
  - Behördlicher Erstkontakt mit Privatpersonen ist rechtlich nur nach vorheriger Mitwirkung möglich (s. R2).
  - Für Kontaktaufnahme durch Personen zu Behörden über das Adressregister: kein rechtlicher Befund.
  - BT-Drs. 21/7319: keine Postfachfunktion in der EUDI-Wallet (D7319 S. 5).
- **P1 Benachrichtigung:**
  - D-Architektur: „Verwaltungskunden können selbst festlegen über welchen Kanal sie Benachrichtigungen … erhalten wollen“ (DA-Zus S. 19).
  - RG: Für § 37 Abs. 2a SGB X wäre der Zugang der Benachrichtigung nachzuweisen, ersatzweise der Abruf (RG Z. 1491ff.).
  - B-2026/18-IT nennt die Komponente „Postfach/ Benachrichtigung“.
- **Ü2 Vertraulichkeit:** E2EE ist rechtlich nicht vorgeschrieben, aber „praktisch geboten“. Empfohlen wird eine Verankerung per Verordnung oder Gesetz (RG Z. 2200ff., 2228ff.). ZaPuK gilt als nummernunabhängiger interpersoneller TK-Dienst mit Fernmeldegeheimnis (ÜH Z. 210ff.).
- **I2 / Aufbewahrung:** Es gibt keine gesetzliche Dauervorhaltepflicht. Empfohlen wird eine Regelung der Vorhalte-, Export- und Löschfristen (RG Tab. 2; ADR-1135).

## Governance, Kosten, Finanzierung, Zeitplan

- **Kosten (nur v1.0, Juni 2025)** [veraltet?]:
  - Bestandslösungen je Lösung 0,5–2,5 Mio. EUR/a Betrieb und 1–2 Mio. EUR/a Weiterentwicklung (Z-ZUS10 Z. 178–180).
  - Phase 2: ca. 1,7 Mio. EUR plus 3 VZÄ (Z. 190).
  - Zentrale Bausteine langfristig 1,8–2,3 Mio. EUR/a Betrieb, 1,5–2 Mio. EUR/a Weiterentwicklung, Folgeprojekte 1–1,3 Mio. EUR (Z. 198).
  - Phase 3 nicht schätzbar (Z. 194).
- **v1.1 nennt keine Zahlen.** Das Finanzierungskonzept kommt „erst zur 52. Sitzung“ (17.03.2027) (ÜH Z. 197–199; B18 Z. 29–31; Z-README FAQ Z. 287–291).
- **Finanzierungslogik laut RG:**
  - Die Finanzierung folgt der Vollzugszuständigkeit (Konnexität).
  - Der Bund finanziert, was er nach Art. 86/87 Abs. 3 GG selbst vollzieht, z. B. das Privatpersonen-Backend.
  - Für an die FITKO delegierte Teile kommt „keine alleinige Finanzierung durch den Bund in Betracht“; naheliegend ist der modifizierte Königsteiner Schlüssel (RG Z. 1112–1137, 1161).
- **[Folgerung]** Der Bund finanziert FIT-Connect nach B-2026/18-IT Ziff. 3 bzw. B-2026/29-IT. Ob davon auch die bei FIT-Connect verorteten Organisations-Backends erfasst sind, ist offen.
- **Zeitplan:**
  - v1.0 sah die Migration der Bestandslösungen bis Ende 2027 und den Betrieb ab 2028 vor (Z-ZUS10 Z. 266) [veraltet?].
  - v1.1:
    - Quality Gate 2 am 25.11.2026 (51. Sitzung).
    - Phase III mit 3A „Zentrale Komponenten“ und 3B „Transition“ (BundID/ZBP, MJP, ELSTER, MUK), ohne Jahreszahlen (Abb. 4).
    - Projekt läuft bis Start Phase III „vsl. Anfang 2027“ (Z-README Z. 304).
    - Transitionsphasen: A 3–6 Monate, B 6–8 Monate, C 1–2 Monate, Abschaltung „nach 24 Monaten“ (Transitionsstrategie, Abb. 1, nicht abgelegt).
    - Reihenfolge: Verbindlichkeit, zentrale Komponenten, Pilot Privatpersonen, Migration Privatpersonen, Organisationen, ELSTER, Abschaltung (ÜH Z. 195).
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
- **R4:** Keine ZaPuK-Angabe. Heute ist das Option 1 („freiwillig“), Ziel Option 3. Hinweis: B-2026/18-IT Ziff. 5 verpflichtet die Länder nicht für das Postfach. Das RG (Z. 559) und PM50 sind hier ungenau.
- **„Wer entscheidet“ = BMDS (R1–R4):** Plausibel, weil die Federführung für Phase III beim BMDS liegt. Rechtlich entscheiden der Gesetzgeber (OZG, VwVfG, VwZG, AO, SGB X, ZPO), die Verordnungsgeber (BMI im Einvernehmen mit BMDS für § 3c VwVfG; BMJV für ZPO und ERVV), der IT-PLR (Standards, Anschlussregelwerk) und Bund/Länder per Staatsvertrag. **[Folgerung]**

## Widersprüche

- Länderpflicht: B18 Z. 33 (nur Ziff. 3) gegenüber PM50 Z. 21 und RG Z. 559 (auch ZaPuK).
- Federführung: B-2026/18-IT (Bund) gegenüber D7319 S. 5 und BRH S. 25 (FITKO) – Phasen-Unterschied **[Folgerung]**.
- Betreiber der Organisations-Backends: FIT-AB 2026/12 „voneinander unabhängige Betriebsorganisationen“ für Privatpersonen und Organisationen (FITAB Z. 15–26) passt zu v1.1. ADR-1114 legt sich bewusst auf keine Stelle fest.
- Stand des Architekturkonzepts: Das RG bewertet „Stand 8. September 2026“ (RG Z. 1–40), die Datei trägt „Stand 25.09.2026“ (AK Z. 1–8).

## Lücken

- Beschlüsse der 51. Sitzung (25.11.2026) liegen noch nicht vor. Ob v1.1, die verbindliche Konsolidierung und Phase III beschlossen werden, ist offen.
- ZaPuK-Finanzierungskonzept und Betriebsmodell (52. Sitzung, 17.03.2027), Meilensteinplanung von Bund und Ländern (B18 Ziff. 6), D-Stack-Governance (B-2026/17-IT, 52. Sitzung): nicht öffentlich.
- Keine aktuellen Kostenzahlen nach v1.0; kein Haushaltstitel gefunden.
- Konkrete Betriebsstelle beim Bund (nachgeordnete Behörde? ITZBund? DCDS?) nicht benannt.
- Kein Gesetzentwurf zu FöMA Maßnahme 207 (Harmonisierung Bekanntgabe/Zustellung, Frist 30.09.2026) und kein Entwurf einer Verordnung nach § 3c Abs. 2 VwVfG n.F. gefunden.
- Kein Staatsvertragsentwurf (ZaPuK-StV oder D-Stack-StV) gefunden.
- Das RG behandelt Landesverfahrensrecht ausdrücklich nicht (Simultangesetzgebung erwartet, RG Z. 1189).
- Die DIP-Volltextsuche war ohne API-Schlüssel nicht möglich. Weitere Drucksachen mit „Postfach“ und D-Stack sind daher nicht ausgeschlossen.
- Anschlussvertrag und Nutzungsbedingungen (Fortschreibung der FIT-Connect-Nutzungsbedingungen, RG Z. 3367) existieren noch nicht.
