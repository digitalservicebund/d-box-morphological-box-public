# ELSTER: Zusammenfassung für die d-box

Wie ELSTER mit Personen, Organisationen und Behörden kommuniziert, eingeordnet nach den Zeilen von `morph_box.csv`. Die Zusammenfassung verweist auf die Quellen, statt sie zu wiederholen. Für Details den Pointer öffnen.

Stand: 2026-11-17. Grundlage ist die ERiC-Dokumentation 44.1.6.0 (Mai 2026) in `Dev Dokumentation/`, ergänzt um Webquellen und die ZaPuK- und Neo-Quellen.

## Kennzeichnung

- Ohne Kennzeichnung: in der ERiC-Dokumentation belegt.
- **[Web]**: nur aus einer Webquelle (Abschnitt Quellen unten), nicht in der ERiC-Doku.
- **[unverifiziert]**: Annahme oder Vorwissen, noch nicht belegt.

## Kürzel für Quellen

Pfade relativ zu `sources/ELSTER/Dev Dokumentation/`.

| Kürzel | Datei | Inhalt |
|---|---|---|
| EHB | `ERiC-Entwicklerhandbuch.pdf` | Entwicklerhandbuch, 514 Seiten. Angabe als Kapitel (Seite). |
| SSb-B | `Datenarten/ElsterBereitstellung/Dokumentation/ElsterBereitstellung-SSB-V5.0.8.pdf` | Senderseite: Einstellen ins Postfach |
| SSb-D | `Datenarten/ElsterDatenabholung/Dokumentation/ElsterDatenabholung-v32.pdf` | Empfängerseite: Abholen, Statusabfrage |
| EDS | `ElsterBasisSchema/Dokumentation/Einheitliche_Datenschnittstelle_XML_11.pdf` | ElsterXML-Umschlag, Verschlüsselung, Transport |
| LIZ | `lizenz.pdf` | ERiC-Lizenzvertrag |
| TUT | `Tutorial/ERiC-Tutorial.pdf`, `Tutorial/Beispiele/PostfachAnfrage.xml`, `PostfachBestaetigung.xml` | Durchgerechnete Beispiele |

ZaPuK-ADRs liegen unter `../zapuk-main/Zielarchitektur/Architekturentscheidungen/`.

## 1. Systembild

```
Mein ELSTER (Web) / ElsterSecure (Login-App) / Drittsoftware mit ERiC + otto
   │ HTTPS; ElsterXML: TransferHeader im Klartext,
   │ DatenTeil gzip + CMS (RSA-OAEP-3072 + AES-256-CBC), bei send-Auth signiert
   ▼
Clearingstelle / Annahmeserver ──► Verteilung an die Fachverfahren der Länder
   │
   ├─ Postman ─► BEM (Bereitstellung Event Manager) ─► BEST-DB (verschlüsselt)
   │               ▲ ElsterBereitstellung                │ WIESEL: Konten, E-Mail-Adressen,
   │               │ (Finanzämter, GeCo der Landes-      │ Poolschlüssel, Depseudonymisierung
   │               │  kopfstellen, OZG-Plattform,        │
   │               │  berechtigte andere Behörden)       ▼
   └─ ElsterDatenabholung: PostfachAnfrage ─► OTTER (Object Store) ─otto─► Client
                            PostfachBestaetigung (Pflicht, innerhalb von 24 h)
```

- **ELSTER-Datenschnittstelle (EDS 1.1, 1.2, 2.6, 2.7.1):**
  - Der TransferHeader bleibt weitgehend unverschlüsselt, weil die Clearingstelle damit die Daten an die Länder verteilt.
  - NutzdatenHeader und Nutzdaten sind komprimiert, mit CMS EnvelopedData verschlüsselt und Base64-kodiert.
  - Die Antwort ist mit dem Session Key verschlüsselt oder mit einem mitgegebenen `TransportSchluessel` (RSA 2048).
  - Seit Dezember 2025 AES statt 3DES (EDS, Änderungsnachweis 4.3.0).
  - Die Schnittstelle darf laut EDS auch ohne ERiC selbst implementiert werden.
- **TLS:** nur ECDHE mit AES-GCM sowie die Suites von TLS 1.3 (EHB 5.3.13, S. 134).
- **Abholen (EHB 9.2.4, 9.3, S. 283–299; SSb-D 6):**
  1. `PostfachAnfrage` an die Annahmeserver liefert Referenz-IDs.
  2. Mit diesen IDs lädt `otto` die Daten blockweise direkt von **OTTER**, mehrere GB sind vorgesehen.
  3. `PostfachBestaetigung` bestätigt den Abruf. Sie muss innerhalb von 24 Stunden kommen, sonst droht dem Hersteller eine Sperre.
  4. Ausnahme: VaSt-Belege liegen nicht auf OTTER und brauchen keine Bestätigung.
- **Große Anhänge** ab 20 MiB können per `otto` vorab zu OTTER hochgeladen und im XML über ihre ID referenziert werden (EHB 9.3.2). Nicht referenzierte Objekte werden nach 7 Tagen gelöscht.
- **Clients:** Mein ELSTER ist nur ein Client. Jede Software mit ERiC kann das Postfach vollständig abfragen, abholen und beantworten.

## 2. Betrieb, Governance, Teilnahme

- **Betreiber:** Der Freistaat Bayern, vertreten durch das BayLfSt, „als bundesweiter Koordinator des Verfahrens ELSTER der Finanzverwaltungen von Bund und Ländern“ (LIZ § 1). Laut Web betreibt das BayLfSt ELSTER im Auftrag von Bund und Ländern auf Grundlage von § 20 Abs. 3 FVG **[Web]**.
- **Steuerung:** über KONSENS nach dem KONSENS-Gesetz (KONSENS-G vom 14.08.2017, anzuwenden ab 01.01.2019; löst das Verwaltungsabkommen KONSENS von 2007 im Bereich der Auftragsverwaltung ab, § 29) **[Web]**:
  - Auftraggeber-Gremium aus Bund und allen Ländern, Vorsitz Bund; Vorhabensplan zur Genehmigung durch die Finanzminister von Bund und Ländern (§ 8).
  - Steuerungsgruppe Informationstechnik aus Bund und BW, BY, HE, NI, NW: „Beschlüsse der Steuerungsgruppe Informationstechnik binden alle Länder und verpflichten diese zur Umsetzung.“ (§ 9 Abs. 4). Sie bestimmt das Auftrag nehmende Land (§ 11).
  - Geschäftsstelle IT beim BMF (§ 10), operative Gesamtleitung (§ 13); Kosten: Länder nach Königsteiner Schlüssel, Bund 13 % (§ 24).
  - ELSTER „wird in Bayern im Auftrag des Bundes und der Länder im Vorhaben „KONSENS“ entwickelt“ (PM Bayerisches Finanzministerium Nr. 161/24 vom 13.06.2024). KONSENS-Broschüre NRW 2015: Auftrag nehmende Länder BY und NW, Verfahrensmanagement in BY, ZPS ELSTER-Betrieb und -Hotline in BY, ELSTER-Support in NI (Stand 2015). Das BayLfSt nimmt die stellvertretende Gesamtleitung von KONSENS wahr.
  - KONSENS-G, FVG und AO nennen ELSTER nicht ausdrücklich; die Zuordnung stammt aus Verwaltungsquellen.
- **Postfach als Basisdienst:** Für andere Behörden ist das Postfach eine „IT-Komponente im Sinne des § 2 Abs. 7 OZG“. Für die Inhalte haftet die Finanzverwaltung nicht (LIZ § 18).
- **Software-Hersteller:**
  - Registrierung im Entwicklerbereich, das BayLfSt prüft die Absicht **[Web]**.
  - Kostenloser Lizenzvertrag; Nutzung nur für Zwecke nach §§ 72a, 87b–87d AO (LIZ §§ 2, 3).
  - Hersteller-ID **je Produkt**, Pflicht in jedem Datensatz (EHB 6.5.3, S. 170).
  - Bei Verstoß Sperre der Hersteller-ID (LIZ, Abschnitt Haftung und Sperre).
  - Testfinanzämter, Teststeuernummern und Testmerker: EHB 6.5 (S. 168).
- **Andere Behörden als Sender:**
  - Nur „speziell berechtigte Verwaltungen“ mit Organisationszertifikat (EHB 9.15.4).
  - Der Absender wird über `AuthAccountID` im TransferHeader erkannt (SSb-B 8.1, 9.1).
  - Den Registrierungsprozess als „zentrale Stelle“ beschreibt eine Handreichung **[Web]**.
  - Bayerische „Verwaltungsvorschrift über die Betriebsbedingungen des einheitlichen Organisationskontos“ vom 30.03.2022 (BayMBl. 2022 Nr. 237), Nr. 4.1.2: „Bei dem Postfach 2.0 handelt es sich um ein Postfach nach § 2 Abs. 7 OZG.“ Übermittlung nach § 9 OZG; „Voraussetzung hierfür ist die Anbindung der nutzungsberechtigten Stelle an die NEZO-Schnittstelle.“ Die VV ist ein Angebot ohne Anbindungspflicht oder Stichtag; Dienstleister sind erlaubt (Nr. 3.3) **[Web]**.
  - Bremer Senatsvorlage vom 02.12.2022 zur „Verwaltungsvereinbarung über die Anbindung des Organisationskontos bei Bund und Ländern“ (Bund und 16 Länder, ab 01.01.2023 nach IT-PLR-Beschluss 2022/37): DIHK, BDI, BDA und ZDH „fordern eine bundesweite Anbindungsverpflichtung“. Das ist eine Forderung der Verbände, keine Pflicht; die Vereinbarung selbst verpflichtet einzelne Behörden nicht **[Web]**.
  - Anreize für Behörden sind in keinem der Dokumente genannt.
- **Personen und Organisationen:** Registrierung mit IdNr. oder Organisations-Steuernummer, über Mein ELSTER oder den Registrierungsassistenten ohne Portal-Login (EHB 5.2, S. 113).

## 3. Identität und Zertifikate

Quelle: EHB 5.2 (S. 112–125).

| Mittel | Eigenschaften |
|---|---|
| **Portalzertifikat (POZ)** | Personengebunden, befristet. Zwei Schlüsselpaare: eines zum Signieren, eines zum Verschlüsseln. Formen: **Zertifikatsdatei** (eine PIN, unbegrenzt viele Fehlversuche), **Sicherheitsstick** (Sperre nach 3 Fehlversuchen), **Signaturkarte** (je nach Hersteller). Beantragt bei der Registrierung. |
| **Clientseitig erzeugtes Zertifikat (CEZ)** | Von ERiC erzeugt, unbefristet, ein Schlüsselpaar. **Nur zum Abholen und Entschlüsseln**, nicht zum Signieren. Darf ausdrücklich „gemeinsames Geheimnis mehrerer Personen“ sein. |
| **Ad-hoc-Zertifikat (AHZ)** | Für nPA und eAT über den ELSTER-eID-Server (TR-03124). Nur im Speicher, 24 Stunden gültig, ohne PIN. |
| **ElsterSecure** | App mit Gerätebiometrie als zweitem Faktor, ohne Zertifikat **[Web]** |

- **Vorgangsarten (EHB 5.2.1):**
  - `send-Auth`: signiert mit POZ oder AHZ.
  - `send-Auth-Part`: wie `send-Auth`, aber bei Sammeldaten bricht ein fehlerhafter Block nur sich selbst ab.
  - `send-NoSig`: unsigniert, bei Erklärungen mit Papier-Nachreichung.
- **Unterstützte Sticks und Karten:** `Unterstuetzte_Signaturkarten_und_Sticks/`.
- **ELSTER als IdP:** ELSTER-Zertifikate dienen auch für BundID und Mein Unternehmenskonto (ZaPuK ADR-0014, ADR-1105, ADR-1111).

## 4. Postfach und Konto

- **Konto:** Das Postfach gehört zum Mein-ELSTER-Benutzerkonto, adressiert über die **accountId**. Mein ELSTER ist kein Nutzerkonto im Sinne des OZG (ZaPuK ADR-1110).
- **Ablauf beim Einstellen (SSb-B 4.1):**
  1. Der Datenlieferant schickt eine ElsterXML `ElsterBereitstellung`.
  2. BEM holt sie von Postman ab.
  3. BEM speichert sie verschlüsselt in der BEST-DB.
  4. Optional folgen E-Mail und Protokolle.
- **Status (SSb-D 6.6):** 05 eingestellt → 07 in Bekanntgabe (E-Mail versendet) → 10 bereitgestellt → 20 abgeholt → 30 bestätigt → 90 gelöscht. 09 heißt fehlerhaft, 99 unbekannt.
- **Löschfristen (EHB 9.2.5):** 180 Tage. Für `EPBescheid`, Gewerbesteuerbescheide und DIVA-Datenarten **4 Jahre**.
- **Arten von Inhalten (EHB 9.2.6, Tabellen 9-7 bis 9-9, S. 286–295):**
  - DIVA-Bescheide der Steuerverwaltung (`DivaBescheidESt`, `DivaBescheidUSt` …).
  - Postfach 2.0 (`EP*`).
  - OZG-Datenarten (`OZGBescheid`, `OZGMitteilung`, `OZGPPNachricht`).
  - Bescheiddaten (`ESB`, `USB`, `GSB`, nicht rechtsgültig).
  - Rückmeldungen und Protokolle.
- **Größenlimits:**
  - ElsterXML höchstens 7,159 GB, Element `Daten` höchstens 1000 MB (SSb-B 4.2, 5.4).
  - `EPAntwort`: höchstens 20 Anhänge, insgesamt 36,9 MiB, einzeln 10 MiB (EHB 9.15.5).
  - `OZGPPNachricht`: höchstens 100 Anhänge, insgesamt 5 GiB, einzeln 2 GiB (EHB 9.15.6).
  - Die Angabe „5 GiB je Nachricht“ aus Webquellen zum Postfach 2.0 betrifft also OZG-PLUS.

## 5. Einordnung nach `morph_box.csv`

Die Ziffern verweisen auf die Ausprägungen der jeweiligen Zeile in `morph_box.csv`.

### Behörde

- **B1 Anbindung von Fachverfahren: 1 + 2 + 6.**
  - Zentrale Schnittstelle `ElsterBereitstellung` (Datenarten `EPBescheid`, `EPMitteilung`, `EPKurzmitteilung`).
  - ERiC als SDK.
  - Lieferung auch über GeCo der Landeskopfstellen und die OZG-Plattform (SSb-B 4.1; EHB 9.15, S. 417).
  - Gewerbesteuerbescheide der Kommunen kommen nicht über ERiC herein, lassen sich aber abholen (EHB 9.15.1).
- **B2 Zugang ohne Fachverfahren:** In der Entwickler-Doku nichts dokumentiert. Laut Web:
  - ELSTER-Transfer (ETR) gibt es als Mein ELSTER und als Java-Anwendung „mit Web-Interface und Command-line-Interface“. Die Anwendung läuft „üblicherweise ständig auf einem Server oder Rechner“, mit Ein- und Ausgangsordnern, JSON-Adressdateien und REST-API (ETR-Hilfe) **[Web]**.
  - Für Bescheide in das Postfach „ist die ELSTER-Transfer-Anwendung zwingend erforderlich“; webbasiertes ETR in Mein ELSTER ist „nicht für den Rückkanal geeignet“ (Leitfaden Gewerbesteuerbescheid, 24.08.2023) **[Web]**.
  - Abschnitt 4.3.2 „Upload über ELSTER-Transfer (direkt)“ besteht nur aus Screenshots (S. 10–11): In der lokalen Web-Oberfläche „Bereitstellung von Dokumenten“ → „Neues Dokument bereitstellen“; Datenart wählen, Empfänger als Benutzerkonto-ID, Datei(en) hochladen (im Beispiel PDF), Datum des Bescheids, optional Lieferticket und Geschäftszeichen **[Web]**. Postfach 2.0 Featureübersicht, Abschnitt 2.1, beschreibt denselben Ablauf für `EPBescheid` mit Empfängerreferenz und Anschreiben **[Web]**.
  - FAQ Digitaler Gewerbesteuerbescheid (26.07.2024), Frage 9: „Ferner erlaubt die ELSTER-Transfer-Anwendung auch eine manuelle Bedienung des Rückkanals.“ Frage 15: maschinelle Bekanntgabe über die REST-API „wird … empfohlen“, die manuelle Bekanntgabe ist „-aktuell noch mögliche-“ **[Web]**.
  - Ohne Fachverfahren heißt also nicht ohne ELSTER-Transfer: Die Anwendung mit Organisationszertifikat und Berechtigung muss installiert sein. Ob beim manuellen Gewerbesteuerbescheid PDF/A-3 mit eingebettetem XML Pflicht ist (Abschnitt 4.2 verlangt es für das HKR-System), ist offen.
  - Ein zentraler Versanddienst oder Druckdienst ist nicht beschrieben.
- **B3 Adressierung: 1.** Ordnungsbegriffe (SSb-B 5.2, 7.5–7.7, 8.5, 8.16):
  - `accountId`: 10 Ziffern, oder pseudonym als **`ek-` + 40 Hex-Zeichen**, aufgelöst über WIESEL.
  - `vnid`: Bevollmächtigte.
  - `ags` mit `bundesland` oder `registrierungsbundesland`: Konto einer Gemeinde.
  - `fingerprint`: CEZ.
  - `funktionspostfachid`: OZG-PLUS.
  - `empfaengerreferenz`: Unteradressierung.
  - Ist das Konto inaktiv oder unbekannt, wird die Nachricht abgelehnt (Status 09).
  - Für DIVA sind zusätzlich `steuernummer` oder `geschaeftszeichen` nötig, optional `idnr` und `widnr`.
- **B4 Inhalt an Person / Organisation: 1 + 2 + 3.**
  - Bescheid als PDF.
  - Bescheiddaten strukturiert, aber nicht rechtsgültig (EHB 7.4, S. 184; Format EHB 15.4).
  - `EPKurzmitteilung` als reiner Text.
  - HTML-Formatierung und `Anschreiben` im Postfach 2.0 (SSb-B 5.3.4, 11; SSb-D 9.1.1).
  - Anhänge mit `DokumentHierarchieTyp` (SSb-B 6.2.1).

### Person / Organisation

- **P1 Benachrichtigung: 2.**
  - E-Mail ohne Inhalt. Für DIVA, `EPBescheid` und `EPAntwort` Pflicht, bei DIVA an **genau eine** Adresse (SSb-B 5.5, 7.24, 7.25, 8.11).
  - Die Adresse ermittelt WIESEL automatisch aus der `accountId` (`EmpfaengerErmitteln=AUTOMATISCH`). Vertreter:innen werden mitbenachrichtigt.
  - Teilweise legt ELSTER den Inhalt zentral fest; die Absenderin hat darauf keinen Einfluss (SSb-B 5.5.1).
  - Bei Löschung eines Anhangs gibt es eine optionale E-Mail (EHB 5.4.11.2).
- **P2 Authentifizierung: 8 + 1, ähnlich 4** (ElsterSecure). Details siehe Abschnitt 3.
- **P3 Organisationszugang: 2, technisch auch 1.**
  - Organisationszertifikat. Ein CEZ kann geteilt werden (EHB 5.2.2.1).
  - Bescheiddaten für mehrere Bearbeiter laufen über einen CEZ-Fingerprint (EHB 7.4.1).
  - Eine Vertretungsregelung für Abwesenheiten in Unternehmen ist **nur im Testbetrieb** (SSb-D 6.1).
- **P4 Vertretung: 2 + 5.**
  - Empfangs- und Bekanntgabevollmacht. Bevollmächtigte haben eine eigene `vnid` und willigen selbst in die elektronische Bekanntgabe ein (EHB 7.5 Stufe 2, 9.16.1.3).
  - Kammer-Vollmachtsdatenbank: betrieben von der **Bundessteuerberaterkammer mit DATEV** (EHB 9.17.3, S. 443).
  - ELSTER-eigene Vollmachten ohne Vollmachtsvermutung und Abrufvollmacht, die keine Empfangsvollmacht ist (EHB 9.17).
  - Vollmachten für Lohnsteuerhilfevereine (EHB 9.14; `Datenarten/ElsterBRM/Vollmachten/`).
  - `EPAntwortKopie` informiert die vertretene Person, wenn jemand in ihrer Vertretung antwortet (EHB Tabelle 9-9).
- **P5 Clients: 1 + 5.** Web-Portal Mein ELSTER sowie Drittsoftware über ERiC mit vollem Lese- und Schreibzugriff.
- **P6 Anbindung eigener Systeme: 2.** ERiC für Steuer-, Kanzlei-, Lohn- und ERP-Software; frei implementierbare XML-Schnittstelle (EDS).
- **P7 Antwort und Erstkontakt: 4 + 3.**
  - Finanzamt: Verfahren `ElsterNachricht`, adressiert über die Finanzamtsnummer (`Empfaenger id="F"`). Liste der Finanzämter in `Finanzamtsdaten.ods` (EHB 9.11, S. 347).
  - Postfach 2.0: Antworten nur, wenn die Absenderin `MetaInfoMap typ="ANTWORT"` setzt, mit Zuordnungskriterium (UUID), vorgegebenen Antwortbetreffs und Antwortfrist. Ohne Frist gelten bei Mitteilungen 60 Tage, bei Bescheiden die DIVA-Löschfrist (SSb-B 5.3.5; EHB 9.15.2, S. 419).
  - Auf `OZGPPNachricht` kann man nicht antworten.
- **P8 Inhalt an die Behörde: 1 + 2 + 4.**
  - Strukturierte Datenarten: `Einspruch`, `SonstigeNachrichten`, `Belegnachreichung`, `Fristverlaengerung`, `AenderungAdresse` … (EHB 9.11).
  - `SonstigeNachrichten` nur mit `send-Auth` und einem Organisationszertifikat oder einem mit IdNr. registrierten Zertifikat (EHB 9.11.6, S. 361).
  - `EPAntwort` als PDF-Anhang (EHB 9.15.5).
  - Zeichen- und Anhangslimits je Datenart: `Datenarten/ElsterNachricht/<Datenart>/Jahresdokumentation/`. Belegnachreichung: höchstens 20 Anhänge, nur `application/pdf`, PDF-Scan mit höchstens 100 Seiten, je höchstens 10.485.760 Bytes (`Datenarten/ElsterNachricht/Belegnachreichung/Schema/24/Belegnachreichung-24-Nutzdaten.xsd`). Forenbeiträge nennen in Mein ELSTER zusätzlich ein Gesamtlimit von etwa 14 MB **[Web, Forum]**.
- **P9 Postfach und Nutzerkonto: 1.** Siehe Abschnitt 4.

### Übermittlung

- **Ü1 Modus: 1.** Asynchron, Pull mit Bestätigung.
- **Ü2 Vertraulichkeit: 1, mit Einschränkung.**
  - Transport: TLS plus CMS-Verschlüsselung an die Clearingstelle.
  - In der BEST-DB: verschlüsselt mit dem **WIESEL-Poolschlüssel** (der Betreiber kann entschlüsseln; nötig für `send-Auth`-Abholung) oder mit dem X.509-Zertifikat der Empfänger:in (`MetaAttribut Verschluesselung`, daraus der Fingerprint) (SSb-B 5.3.2).
  - Keine Ende-zu-Ende-Verschlüsselung, weil ELSTER selbst verschlüsselt.
  - Anhänge im ElsterXML: Base64 und als Anhang selbst unverschlüsselt, geschützt durch die Verschlüsselung des DatenTeils (EHB 5.4.11).
- **Ü3 Echtheit: 5 im Rückkanal, 1 von der Behörde.**
  - Rückkanal: Signatur mit POZ oder AHZ (`SigUser` im TransferHeader; EDS 2.4.1).
  - Behördenrichtung: Die Absenderin ist über ihr Organisationszertifikat authentifiziert. Ein Siegel auf Bescheiden kommt in der Doku nicht vor.
- **Ü4 Ereignisse und Bestätigungen: 1 + 2 + 5.**
  - `Statusabfrage` mit Rolle `sender` oder `empfaenger` liefert Status, Zeitpunkt des E-Mail-Versands, **Abrufzeitpunkt** und Löschzeitpunkt (SSb-D 6.6).
  - Bei der Abgabe gibt es eine sofortige Rückantwort mit Transferticket und PDF-Protokoll (EHB 6.2.5).
  - Die Absenderin einer `EPAntwort` bekommt eine Info-E-Mail (SSb-B 9.7).
  - Eine Lesebestätigung gibt es nicht.
- **Ü5 Interoperabilität: 1.** Sonderweg: `OZGPPNachricht` liefert über ELSTER an das OZG-PLUS-Postfach (EHB 9.15.6).
- **Ü6 Malware-Prüfung: 1 + 6 + 7.**
  - Eingang: Scan auf dem Server, aktive Inhalte werden entfernt, schadhafte Anhänge gelöscht, Rückmeldung über `AnhangRueckmeldung` (EHB 5.4.11, S. 148; Hinweistext Pflicht in der Software).
  - Ausgang: Bei `EPBescheid` und `EPMitteilung` muss die Absenderin `Virengeprueft=true` setzen (EHB Tabelle 5-12).
  - Bei DIVA sind Scan und Bereinigung **verboten** (SSb-B 7.26).
  - Formate: Anhänge an das Finanzamt nur als PDF (siehe P8). Für Anhänge der Behörden (`OZGPPNachricht`: bis 100 Anhänge, je 2 GiB, insgesamt 5 GiB, EHB 9.15.6) ist keine Formatliste dokumentiert. Eine PDF/A-Pflicht ist nicht gefunden.

### Infrastruktur und Betrieb

- **I1 Betrieb und Governance: 1.** Siehe Abschnitt 2.
- **I2 Datenhaltung und Versand: 1.** Zentral in BEST-DB und OTTER. Wo die Rechenzentren genau stehen, ist offen. Das BayLfSt betreibt eigene Rechenzentren **[Web]**.
- **I3 Betriebsmodell für Organisationen: 1.**
- **I4 Funktionspostfächer und interne Bearbeitung: 5.**
  - Adressierung: „Ihr zuständiges Finanzamt und die zuständigen Bearbeiter können Sie über die Eingabe eines der folgenden Felder adressieren: Steuernummer …“ (Technisches Finanzamt Cottbus, Anleitung ELSTER-Kontaktformular, 2026) **[Web]**.
  - ELSTER liefert strukturierte Daten für eine „vorgangsgesteuerte, teilweise sogar bereits halbautomatische Weiterverarbeitung“ in den Fachverfahren (Nds. Landtag Drs. 18/909, 07.05.2018) **[Web]**. Eine E-Akte der Steuerverwaltung ist im Aufbau (Rechnungshof RLP 2019).
  - Wie Nachrichten im Finanzamt an Bearbeiter verteilt werden (Posteingangskorb, Verteilregeln, Bearbeitungsdauer), ist öffentlich nicht dokumentiert; vermutlich in den KONSENS-Fachverfahren **[unverifiziert]**.
  - Rückkanal je Land unterschiedlich: Niedersachsen: „Eine Beantwortung über ELSTER ist dem Finanzamt technisch nicht möglich.“ Baden-Württemberg: bei elektronischer Bekanntgabe Antwort „in Ihr persönliches Postfach in Mein ELSTER“ **[Web]**.
  - Organisationen teilen sich ein Organisationszertifikat oder CEZ (siehe P3). Leitfaden Organisationszertifikate (03/2024): mehrere Zertifikate je Unternehmen möglich, je Zertifikat ein Benutzerkonto und Mitarbeiterpostfach; „Zusätzlich können Funktionspostfächer angelegt werden, auf die mehrere Mitarbeitende Zugriff haben.“ Die Berechtigungssteuerung ist noch nicht vollständig ausgerollt **[Web]**.
- **I5 Aufnahme in den Verbund: 1.** Für Drittsoftware faktisch **3**: Lizenzvertrag und Hersteller-ID, Sanktion ist die Sperre.

### Recht

- **R1 Schriftformersatz:**
  - Behörde an Person: Ein elektronischer Verwaltungsakt erfüllt die Schriftform, wenn er in einem sicheren Verfahren nach § 87a Abs. 7 oder 8 AO übermittelt oder zum Abruf bereitgestellt wird (§ 87a Abs. 4 AO); § 122a Abs. 1 AO verweist auf § 87a Abs. 8 AO **[Web]**.
  - Person an Finanzamt: § 87a Abs. 3 AO lässt die unmittelbare Abgabe in einem elektronischen Formular der Behörde zu, § 87a Abs. 6 AO verlangt für Datensätze eine authentifizierte Übermittlung. Satz 5 verlangt bei öffentlich zugänglichen Netzen einen elektronischen Identitätsnachweis oder ein sicheres Verfahren der Finanzverwaltung. AEAO zu § 87a Nr. 3.1: Die Schriftform kann durch Übermittlung in einem Verfahren nach § 87a Abs. 3 Sätze 4 bis 8 AO ersetzt werden; Nr. 3.2.1: Bei amtlichem Datensatz wird das Unterschrifterfordernis durch die Authentifizierung verdrängt. ELSTER wird dort nicht namentlich genannt; die Einordnung von Mein ELSTER als solches Formular ist abgeleitet **[Web]**. Der Einspruch ist nach § 357 Abs. 1 Satz 1 AO „schriftlich oder elektronisch einzureichen“ **[Web]**.
- **R2 Teilnahmepflicht: 1 → 3.**
  - Elektronische Bekanntgabe nur mit Einwilligung **je Steuernummer**. Für Ehepaare und Organisationen zusätzlich ein Einwilligungscode per Brief (EWC); nach fünf Fehleingaben wird er gesperrt; neuere Einwilligungen überschreiben ältere (EHB 9.16.2, 9.16.3, S. 430–437).
  - Ab 2027 ist die elektronische Bekanntgabe Regelfall; Papier gibt es auf Antrag **[Web]**. Technisch dafür: `DauerPostBeStAntrag` und `-Widerruf` (EHB 9.10.1, S. 341).
- **R3 Bekanntgabe und förmliche Zustellung: 1.**
  - DIVA nach § 122a AO (EHB 7.5, S. 187; 9.16.1.1).
  - Das `bescheiddatum` hält den Datensatz in Status 05; an diesem Tag folgen E-Mail und Bereitstellung (SSb-B 4.1).
  - Der Bescheid gilt am **vierten Tag nach Bereitstellung** als bekannt gegeben (§ 122a Abs. 4 Satz 1 AO). „Dritter“ wurde durch Art. 20 des Postrechtsmodernisierungsgesetzes vom 15.07.2024 durch „vierter“ ersetzt, in Kraft seit 01.01.2025 **[Web]**. Übergangsregel Art. 97 § 1 Abs. 15 EGAO: gilt für alle Verwaltungsakte, die „nach dem 31. Dezember 2024 … elektronisch zum Abruf bereit gestellt werden“. Maßgeblich ist die Bereitstellung, nicht der Erlass.
  - Neufassung von § 122a AO durch das Vierte Bürokratieentlastungsgesetz vom 23.10.2024, ab 01.01.2026: Bekanntgabe „soll“ durch Bereitstellung erfolgen, Papier nur auf Antrag (§ 122a Abs. 2 AO). Nach Art. 97 § 28 Abs. 2 EGAO gilt der Regelfall (Abs. 1 Satz 2) erst für Verwaltungsakte, die nach dem 31.12.2026 erlassen werden; die Verschiebung kam mit dem Gesetz zur Anpassung des Mindeststeuergesetzes vom 22.12.2025 **[Web]**.
  - Eine förmliche Zustellung über ELSTER (§ 122 Abs. 5 AO mit VwZG) ist nicht gefunden.
- **R4 Teilnahme der Behörden:** Finanzämter von Bund und Ländern nehmen gemeinsam teil. Eine Angebotspflicht in der AO gibt es nicht (§ 87a Abs. 1 AO: Zugangseröffnung). Die Teilnahme folgt aus § 12 KONSENS-G: „Die Länder sind verpflichtet, die durch die Auftrag nehmenden Länder entwickelten IT-Verfahren oder die entwickelte Software einheitlich … im eigenen Land einzusetzen“, und aus § 20 Abs. 3 FVG: technische Hilfstätigkeiten wie „die Entgegennahme elektronischer Steuererklärungen“ und „die elektronische Übermittlung von Steuerverwaltungsakten“ durch ein anderes Land sind „der sachlich und örtlich zuständigen Finanzbehörde zuzurechnen“ **[Web]**. Zusätzlich haben sich die Länder verständigt, statt De-Mail „grundsätzlich nur den Eingangskanal ELSTER“ vorzusehen (Nds. Drs. 18/909) **[Web]**. Für andere Behörden **1**: freiwillig, nur mit Berechtigung und Anbindung über Postfach 2.0; keine Pflicht und keine Anreize, Wirtschaftsverbände fordern eine Anbindungsverpflichtung (siehe Abschnitt 2).

## 6. Widersprüche und offene Punkte

- **Bekanntgabe-Tag:** EHB 7.5 (Fußnoten 47, 48) nennt den dritten Tag nach Absenden der Benachrichtigung. Das ist seit 01.01.2025 veraltet (Postrechtsmodernisierungsgesetz, aktueller § 122a Abs. 4 AO: vierter Tag nach Bereitstellung).
- **Rechtsgrundlage `EPBescheid`:** EHB 9.15.4 nennt § 9 OZG, EHB Tabelle 9-9 nennt § 122a AO.
- **Noch nicht belegt:**
  - Siegel oder Signatur auf Bescheiden (Ü3).
  - Standort der Rechenzentren.
  - Mein-ELSTER-Oberfläche und ElsterSecure im Detail: Die Doku richtet sich an Entwickler.
  - Interne Verteilung der Eingänge in den Finanzämtern (I4), nur Adressierung per Steuernummer belegt.
  - Wortlaut des Verwaltungsabkommens KONSENS von 2007.
  - PDF/A-3-Pflicht beim manuellen Upload von Gewerbesteuerbescheiden (B2).
- **Datenart `Vertretung` (SSb-D 6.1):** bisher nur im Testbetrieb.
- **Einwand der Steuerberaterkammer:** Laut `bewertungskriterien.md` bestreitet sie, dass das Nutzerkonto für Steuerbescheide ausreicht. Das ist noch rechtlich zu prüfen.

## 7. Webquellen

- Betrieb: [ELSTER, eine Erfolgsstory](https://www.elster.de/eportal/infoseite/elster_eine_erfolgsstory), [BayLfSt Rechenzentren 2026](https://www.lfst.bayern.de/fileadmin/RESSOURCEN/LfSt/Digital%40BayLfSt/Informationen_zu_den_Rechenzentren_03_2026.pdf)
- Entwickler-Registrierung: [ELSTER Entwickler](https://www.elster.de/eportal/infoseite/entwickler)
- Login: [ElsterSecure](https://www.elster.de/bportal/infoseite/elstersecure), [Registrierungsflyer](https://download.elster.de/download/dokumente/Elster_Flyer_Registrierung_neutral_BF_28-10-20.pdf)
- Postfach 2.0 und ELSTER-Transfer: [Was ist ELSTER-Transfer?](https://www.elster.de/elsterweb/infoseite/elstertransfer_hilfe_einfuehrung), [Leitfaden digitaler Gewerbesteuerbescheid über den ELSTER-Rückkanal](https://download.elster.de/download/elstertransfer/dokumente/20230824_Leitfaden_DigGewStBescheid_ELSTER-Rueckkanal.pdf), [Postfach 2.0 Bedienung und Featureübersicht](https://info.mein-unternehmenskonto.de/wp-content/uploads/2023/11/Postfach-2.0-Bedienung-und-Featureuebersicht-Postfach-2.0-1.0_komprimiert.pdf), [Integrationsleitfaden Postfach](https://service.mein-unternehmenskonto.de/api/downloads/integrationsleitfaden_postfach), [Handreichung Registrierung zentrale Stelle](https://itvsh.de/fileadmin/user_upload/Dokumente/DiPlan_SH/Handreichung_zum_Registrierungsprozess_von_ELSTER_zentrale_Stelle_1.2.pdf)
- Bekanntgabe ab 2027: [PwC](https://blogs.pwc.de/de/steuern-und-recht/article/257477/elektronische-bekanntgabe-von-steuerbescheiden-ab-2027/), [Ebner Stolz](https://www.ebnerstolz.de/de/unser-angebot/leistungen/steuerberatung/allgemeine-steuerberatung/elektronische-bekanntgabe-steuerbescheide-ab-01.01.2027-111408.html)
- Vollmachten: [ELSTER Vollmachtsdatenbank](https://www.elster.de/elsterweb/infoseite/vollmachtsdatenbank), [BStBK Vollmachtsdatenbank](https://www.bstbk.de/de/themen/vollmachtsdatenbank)
- Anhänge an Finanzämter: [Sachsen-Anhalt, digitale Belegübersendung](https://sachsen-anhalt.de/fileadmin/tsa_rssinclude/ministerium-der-finanzen_28_11_2020_pressemitteilung_jetzt-neu-digitale-beleguebersendung-an-finanzaemter.pdf)
- Organisationskonto und Teilnahme: [BayVV Betriebsbedingungen Organisationskonto vom 30.03.2022](https://www.gesetze-bayern.de/Content/Document/BayVV_206_D_12940/true), [Bremen, Senatsvorlage zur Verwaltungsvereinbarung Organisationskonto, 02.12.2022](https://www.rathaus.bremen.de/sixcms/media.php/13/top%202_20221220_Verwaltungsvereinbarung_Organisationskonto_Bremen.pdf), [Modulares Unternehmenskonto auf Basis von ELSTER](https://download.elster.de/download/dokumente/Modulares_Unternehmenskonto_auf_der_Basis_von_ELSTER.pdf)
- KONSENS und Verwaltung: [ELSTER für die Verwaltung](https://www.elster.de/elsterweb/infoseite/verwaltung?locale=de_DE), [Bremische Bürgerschaft Drs. 20/774](https://www.bremische-buergerschaft.de/drs_abo/2021-01-12_Drs-20-774_70e38.pdf), [KONSENS-G](https://www.gesetze-im-internet.de/konsens-g/), [§ 20 FVG](https://www.gesetze-im-internet.de/fvg_1971/__20.html), [BT-Drs. 18/12589 (Begründung KONSENS-G, S. 132–133)](https://dserver.bundestag.de/btd/18/125/1812589.pdf), [PM Bayerisches Finanzministerium Nr. 161/24 (Kopie schweinfurt.de)](https://schweinfurt.de/rathaus-politik/amtliche-bekanntmachungen/m_51974), [KONSENS-Broschüre NRW 2015](https://www.finanzverwaltung.nrw.de/sites/default/files/asset/document/konsens_broschuere_2015_web_reduziert_0.pdf), [BayLfSt Gebäudebroschüre 2024](https://www.lfst.bayern.de/fileadmin/RESSOURCEN/LfSt/UEber_uns/Gebaeudebroschuere_neu_20240430.pdf), [SH Landtag Umdruck 18/6914 (Änderung Verwaltungsabkommen KONSENS)](https://www.landtag.ltsh.de/infothek/wahl18/umdrucke/6900/umdruck-18-6914.pdf)
- Manueller Upload: [FAQ Digitaler Gewerbesteuerbescheid, 26.07.2024](https://download.elster.de/download/elstertransfer/dokumente/20240726_FAQ_Digitaler_Gewerbesteuerbescheid.pdf)
- Eingänge im Finanzamt: [FA Cottbus, Anleitung ELSTER-Kontaktformular](https://finanzamt.brandenburg.de/sixcms/media.php/9/Anleitung-zur-Nutzung-des-ELSTER-Kontaktformulars_07-2026.pdf), [LStN Niedersachsen, sonstige Nachricht](https://lstn.niedersachsen.de/download/165131), [Nds. Landtag Drs. 18/909](https://www.landtag-niedersachsen.de/Drucksachen/Drucksachen_18_02500/00501-01000/18-00909.pdf), [Finanzämter BW, Kontaktformular](https://finanzamt-bw.fv-bwl.de/,Lde/Startseite/Service/Kontaktformular), [Rechnungshof RLP 2019 Nr. 05](https://rechnungshof.rlp.de/fileadmin/rechnungshof/Jahresberichte/2019/Jahresbericht_2019_Nr._05_Struktur_der_Steuerverwaltung.pdf), [Leitfaden ELSTER-Organisationszertifikate 03/2024](https://info.mein-unternehmenskonto.de/wp-content/uploads/2024/03/20240313_Zertifikatsleitfaden.pdf)
- Anhangslimits in Mein ELSTER (Forum): [Maximale Dateigrößen bei Belegnachreichungen](https://forum.elster.de/anwenderforum/forum/elster-webanwendungen/mein-elster/412772-maximale-dateigr%C3%B6%C3%9Fen-bei-belegnachreichungen-viel-zu-klein)
- Gesetz: [§ 87a AO](https://www.gesetze-im-internet.de/ao_1977/__87a.html), [§ 122a AO](https://www.gesetze-im-internet.de/ao_1977/__122a.html), [§ 357 AO](https://www.gesetze-im-internet.de/ao_1977/__357.html), [Art. 97 § 1 EGAO](https://www.gesetze-im-internet.de/aoeg_1977/art_97__1.html), [Art. 97 § 28 EGAO](https://www.gesetze-im-internet.de/aoeg_1977/art_97__28.html), [AEAO zu § 87a (Abschrift steuerschroeder.de, AO-Handbuch 2026)](https://www.steuerschroeder.de/steuergesetze/aeao/87a), [Änderungshistorie § 122a AO (buzer.de)](https://www.buzer.de/122a_AO.htm)

## 8. Verwandte lokale Quellen

- ZaPuK ADR-0002 (ELSTER-Verfahren nach § 87a AO, Betriebsverantwortung), ADR-0014 und ADR-1105 (ELSTER-Zertifikat als Authentisierungsmittel der BundID), ADR-1110 (Mein ELSTER ist kein OZG-Nutzerkonto, § 122a AO), ADR-1111 (Postfachadresse im ELSTER-Organisationszertifikat).
- `../zapuk-main/Zielarchitektur/Architekturkonzept/README.md`: Bestandslösungen, darunter ELSTER Postfach 2.0.
- `../neo-solution-architecture-develop/concepts/303-delivery-retrieval-and-read-receipts.md`: § 122a Abs. 4 AO als Ausnahme von § 9 OZG.
