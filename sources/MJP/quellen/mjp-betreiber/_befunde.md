# Befunde: Mein Justizpostfach (MJP), Betreiber- und Nutzersicht

Stand der Recherche: 2026-11-18. Schwerpunkt Betreiber und Nutzende. Technik (EGVP, OSCI, SAFE, XJustiz) und Recht (ERVV, ZPO, Zustellung) nur, soweit es für die Nutzersicht nötig war.

Kürzel der Fundstellen (alle Dateien in diesem Ordner, Z. = Zeile, S. = PDF-Seite):

- TB = mjp_webclient-ui-textbundle-de.md
- CFG = mjp_webclient-config.md
- JS = governikus_mjp_webclient-appjs-auszuege.md
- DSE = blk_mjp_datenschutzerklaerung.md
- IMP = blk_mjp_impressum.md
- FAQ = mjpserviceportal_faq.md
- SP = mjpserviceportal_startseite.md
- LÖ = blk-mjp-support_mjp_information-loeschen-von-nachrichten_2025-05.pdf
- ERN = blk-mjp-support_mjp_anleitung-schluesseldatei-erneuern_2026-10.pdf
- KW = blk-mjp-support_mjp_kennwort-pruefen_2025-02.pdf
- ENT = blk-mjp-support_mjp_nachrichten-entschluesseln_2025-05.pdf
- SPE = blk-mjp-support_mjp_nachrichten-speichern_2025-05.pdf
- AKT = blk-mjp-support_mjp_zugang-akteneinsichtsportal_2025-06.pdf
- EEB = blk-mjp-support_mjp_eeb-erstellen-versenden_2026-01.pdf
- V11 = blk-ag-it-standards_mjp-information-v1.1_2023-10-25.pdf
- V12 = blk-ag-it-standards_mjp-information-v1.2_2023-11-22.pdf
- ERL = jm-nrw_erlass-start-mjp-und-ebo-client_2023-11-03.pdf
- TP = bmi-dv-i-5_mjp-two-pager-behoerden-laender_2023-08.pdf
- PM = bmj_pm-62-2023_startschuss-mjp_2023-10-13.md
- SJ = servicejustiz-bmjv_mjp-einrichten.md
- JP = justizportal_erv-buerger.md
- IFS = ifs_erv-aktueller-stand_2025-11.pdf (Sekundärquelle)
- NP23 = netzpolitik_mjp-datenleck_2023-11-14.md (Sekundärquelle)
- NP25 = netzpolitik_mjp-privatsphaere_2025-06-13.md (Sekundärquelle)
- FDS-DP = fragdenstaat_mjp-datenpanne_bmi_2023-11.md
- FDS-DA = fragdenstaat_mjp-deutschland-app_blk-jumbw_2026-09.md

## Überblick

- MJP ist ein kostenloses Browser-Postfach für natürliche Personen zur Kommunikation mit Gerichten, Staatsanwaltschaften, Gerichtsvollziehern, Behörden (beBPo), Anwälten, Notaren und Steuerberatern (FAQ Z.43, Z.51). Es ist ein EGVP-Postfach und läuft über die EGVP-Infrastruktur (DSE Z.8). Rechtlich ist es der „Postfach- und Versanddienst des Nutzerkontos Bund gemäß § 13 ERVV“ (V11 S.1, JP Z.63).
- Start im Pilotbetrieb am 12.10.2023 (V11 S.1, ERL S.2). Laut BMJ startete es „seit dem 13. Oktober 2023“ (PM Z.15, Pilotbetrieb Z.33). Ursprüngliche Adresse: mein-justizpostfach.bund.de (V11 S.1). Heute mjp.justiz.de, API api.mjp.justiz.de (CFG). Client-Version 2.7.5. Update auf 2.7.2 am 05.10.2026 (SP Z.34).
- Nutzungszahlen: keine gefunden (siehe Lücken).

## Befunde je Zeile

### B1 Anbindung von Fachverfahren

- Gerichte, Staatsanwaltschaften und Behörden mit beBPo erreichen MJP über ihre EGVP-Postfächer. Sie finden MJP-Nutzende im SAFE-Verzeichnis (V11 S.1). MJP-Einträge tragen dort automatisch den Rollenwert `ozg_postfach` (V11 S.1, Fn. 2).
- Seit 2023 kamen beA, beN und beSt als Absender hinzu: 2023 noch „in einer weiteren Ausbaustufe“ (V11 S.1), beA erst 2024 (ERL S.3) **[veraltet?]**. Heute zählt die DSE Anwälte, Notare und Steuerberater zu denen, die den Eintrag einsehen können (DSE Z.96, FAQ Z.298).

### B3 Adressierung der Person / Organisation

- Die Adressierung läuft über den SAFE-Eintrag mit Vorname, Nachname, Anschrift und Länderkennung aus der BundID sowie eine eindeutige Nutzer-ID bzw. SAFE-ID (DSE Z.78–96). Die Anschrift ist die aus dem Ausweis. Eine abweichende Korrespondenzadresse ist nicht möglich (FAQ Z.150, IFS S.2).
- Opt-out aus der Suche: Bei Auskunftssperre lässt sich die Auffindbarkeit über die SAFE-Suche ausschließen. Dann ist man nur für Absender erreichbar, die die SAFE-ID kennen (TB Z.215–216, FAQ Z.298). 2025 war das noch angekündigt (NP25 Z.63) **[veraltet?]**.
- Der VHN jeder ausgehenden Nachricht enthält die Adressangaben (FAQ Z.301).

### B4 Inhalt an die Person / Organisation

- Eingehende Nachrichten enthalten Dokumente, die technische Datei `xjustiz_nachricht.xml` und einen Prüfvermerk, der im Client erzeugt wird (SPE S.1). Ein Gericht kann ein eEB anfordern, die Anforderung steckt in der `xjustiz_nachricht.xml` (EEB S.1).

### P1 Benachrichtigung

- Bei einer neuen Nachricht benachrichtigt MJP das BundID-Postfach. Die BundID schickt dann eine E-Mail an die persönliche Adresse (FAQ Z.323). Die E-Mail verweist auf das BundID-Postfach, nicht auf das MJP (FAQ Z.204). Technisch hinterlegt MJP bei der Anmeldung ein „ZBP-Mailbox-Handle“ (TB Z.571 `SET_ZBP_MAILBOX_HANDLE_FAILED`). Nach einer Kontolöschung können noch Benachrichtigungen im BundID-Postfach ankommen (TB Z.760).
- Die E-Mail-Benachrichtigung gibt es seit Ende 2024 (IFS S.1) **[unverifiziert]**.
- Vor der automatischen Löschung kommen Hinweise über die BundID, 30 und 10 Tage vorher (LÖ S.1, FAQ Z.236).
- **[Folgerung]** Die Einstufung „E-Mail-Hinweis ohne Inhalt“ in der morph_box stimmt. Der Hinweis läuft aber indirekt über ZBP und BundID-E-Mail, nicht über MJP selbst.

### P2 Authentifizierung

- Anmeldung nur über BundID mit Vertrauensniveau „hoch“. Bei „substanziell“ oder „niedrig“ wird die Anmeldung abgelehnt (TB Z.575–577, FAQ Z.59, DSE Z.78). Vor- und Nachname sind Pflicht (TB Z.579). Je Konto ist nur eine Sitzung erlaubt (TB Z.563, Z.565).
- Als Online-Ausweis gelten Personalausweis, Unionsbürgerkarte und eAT (SJ Z.54). Ohne Online-Ausweis gibt es kein MJP (SJ Z.66). Ist der Ausweis abgelaufen, ist kein Zugriff möglich, bis ein neuer Ausweis in der BundID hinterlegt ist (FAQ Z.142–152).
- EU-eID: Das Serviceformular hat die Kategorie „Freischaltung Mein Justizpostfach für Nutzer mit EU Identität“ (SP Z.125). Der Client kennt den Zustand „angelegt, nicht freigeschaltet“: Der Support verlangt dann einen Nachweis der Anschrift (TB Z.789). **[Folgerung]** Wer sich mit einer EU-eID über die BundID anmeldet, wird offenbar erst nach manueller Adressprüfung freigeschaltet.
- Zum Lesen braucht man zusätzlich die Schlüsseldatei (.p12) und ihr Passwort (FAQ Z.339). **[Folgerung]** Das ist faktisch ein zweiter Faktor, aber nur für die Entschlüsselung, nicht für die Anmeldung.
- Für Bayern gibt es einen gesonderten Weg zu den bayerischen Gerichten über die BayernID (JP Z.71). **[Folgerung]** Das ist ein eigenes Angebot, keine Anmeldeoption des MJP.

### P3 Organisationszugang

- Kein Organisationszugang. MJP ist nur für natürliche Personen mit personengebundener BundID (FAQ Z.59, Z.85). Für Organisationen ist das eBO mit kostenpflichtigen Drittprodukten vorgesehen (JP, Abschnitt eBO; V12 S.1). MJP richtet sich nicht an professionelle Verfahrensbeteiligte (FAQ Z.85, V12 S.1).

### P4 Vertretung

- Es gibt keine Vertretung und keine Übertragung. Der Zugriff ist nur mit dem eigenen Ausweis samt PIN und der eigenen Schlüsseldatei samt Passwort möglich. Inhalte lassen sich nicht auf andere Personen übertragen (FAQ Z.162). Empfehlung: eine Sicherungskopie der Schlüsseldatei „für Fälle, in denen Ihnen ein Zugriff nicht mehr möglich ist“ (FAQ Z.164). Mehrere Konten lassen sich nicht zusammenführen (FAQ Z.77).

### P5 Clients

- Es gibt nur einen Web-Client im Browser (FAQ Z.67, Z.91–95). Keine App, kein Smartphone: Die Schlüsseldatei lässt sich dort nicht nutzen. Man braucht einen PC oder Laptop mit Dateisystemzugriff (FAQ Z.401). Keine Einbindung in Outlook oder andere E-Mail-Programme (FAQ Z.91–95).
- Die BLK hat „keine konkreten Planungen“ für eine mobile Nutzung (FDS-DA Z.107). Das FAQ kündigt sie für „zukünftige Ausbaustufen“ an (FAQ Z.401).
- Der Client stammt aus der Governikus-Codebasis, Copyright Governikus 2022 (JS Z.11). Er nutzt dieselben Bausteine wie das OZG-PLUS-Postfach, etwa Feldnamen `ozgppCreationTime` und Komponenten `ozg-*` (JS, Abschnitt Abruf).

### P6 Anbindung eigener Systeme

- Keine Schnittstelle für eigene Systeme dokumentiert. Nachrichten aus früher genutzter ERV-Software lassen sich nicht übernehmen (FAQ Z.77). Man kann eine `xjustiz_nachricht.xml` „aus einer Fachanwendung“ hochladen (TB Z.233), etwa ein extern erzeugtes eEB (FAQ Z.309–311, EEB S.1).
- Im MJP legt man Zugangsdaten für das Akteneinsichtsportal fest. Die Anmeldung dort läuft über den SAFE-Verzeichnisdienst (TB Z.791, AKT S.1–3).

### P7 Antwort und Erstkontakt

- Anlasslose Nachrichten sind möglich. Den Empfänger sucht man im SAFE-Verzeichnis: Gerichte, Staatsanwaltschaften, Behörden, Anwälte, Notare, Steuerberater (TB Z.316, FAQ Z.347). Andere MJP- oder eBO-Postfächer sind nicht adressierbar (FAQ Z.51, ERL S.2).
- **[Folgerung]** Eine eigene Antwortfunktion fehlt. Weder im Textbundle noch in app.js gibt es „reply“ oder „Antwort“. Eine Antwort ist also eine neue Nachricht mit erneuter Empfängersuche und Aktenzeichen. Die Felder „Aktenzeichen des Empfängers“ (mit „neu“ und „unbekannt“) und „Aktenzeichen des Absenders“ gibt es (TB Z.300–324).

### P8 Inhalt an die Behörde

- Nur Dokumente. Mindestens ein PDF ist Pflicht (TB Z.85, FAQ Z.347). Schriftsätze müssen als PDF kommen, zusätzlich ist TIFF zulässig. Für Beweismittel gibt es keine Formatbeschränkung (FAQ Z.216). Grenzen: höchstens 1000 Dateien und 200 MB insgesamt (CFG Z.43–50, FAQ Z.331). Dateinamen sind auf 90 Zeichen und einen festen Zeichensatz begrenzt (FAQ Z.257).
- Strukturierte Metadaten: Empfänger, Behörde oder Gericht, Sachgebiet, Aktenzeichen, „Eilig“, „Vertraulich“ mit Begründung (TB Z.294–324). Außerdem kann man eine eigene `xjustiz_nachricht.xml` hochladen (TB Z.233).
- **[Folgerung]** Es gibt kein Freitextfeld: Weder app.js noch das Textbundle enthalten eine `textarea` oder ein Nachrichtenfeld. Die Einordnung „Freitext (… Mein Justizpostfach …)“ in morph_box P8 ist damit nicht belegt und vermutlich falsch. Passend dazu warnt das Justizportal beim eBO, Eingaben im Nachrichtenfeld könnten unwirksam sein (JP Z.116).

### P9 Postfach und Nutzerkonto

- MJP ist eine eigenständige Webanwendung. Die BundID dient als Identitätsdienst (IdP) und als Kanal für Benachrichtigungen. Das MJP-System speichert dauerhaft die SAFE-Nutzer-ID und eine BundID-Kennung (DSE Z.110–111).
- Geplant war die Integration in die BundID: „langfristig“ (V11 S.1), laut Zeitstrahl 2024 (TP S.1). Hamburg schreibt noch „perspektivisch“ (siehe _index.md). **[Folgerung]** Bis 2026-11 ist das nicht umgesetzt. Das JuM BW prüft stattdessen die Integration in die „föderale Zielarchitektur für Postfach- und Kommunikationslösungen der FITKO“ (FDS-DA Z.161).
- Lebenszyklus:
  - Nutzende können das Konto jederzeit löschen. Voraussetzung: alle empfangenen Nachrichten gelesen und keine ausgehende im Status „wird gesendet“ (TB Z.754–759, FAQ Z.429). Gelöscht werden alle Nachrichten, das Konto und der SAFE-Eintrag (TB Z.754).
  - Wer die Einwilligung widerruft, löscht damit das Postfach (TB Z.212, DSE Z.142).
  - Die Schlüsseldatei läuft nach 3 Jahren ab. Danach kann man weder senden noch empfangen, bis sie erneuert ist. Die Erneuerung ist seit Version 2.7.2 möglich und übernimmt die alten Schlüssel (ERN S.1, FAQ Z.182, TB Z.675, SP Z.34).
  - Bei verlorener Schlüsseldatei oder vergessenem Passwort gibt es keine Wiederherstellung, das Postfach muss gelöscht werden (FAQ Z.289, KW S.1).
- Aufbewahrung:
  - Nachrichten werden seit dem 31.08.2025 nach 90 Tagen automatisch gelöscht (LÖ S.1, FAQ Z.236). Widerspruch zu ungelesenen Nachrichten siehe unten.
  - Gelesene Nachrichten lassen sich manuell löschen, ungelesene nicht (FAQ Z.174, TB Z.392).
  - Logs und Backups werden 90 Tage verschlüsselt aufbewahrt, Webserver-Logs ebenfalls 90 Tage (DSE Z.53, Z.144). Sitzungs-Cookies gelten höchstens 120 Minuten (DSE Z.64).

### Ü2 Vertraulichkeit

Prüfung der morph_box-Aussage „Ende-zu-Ende mit Schlüsselverwahrung beim Betreiber (Justizpostfach-Webclient)“: **nicht belegt, nach Primärquellen widerlegt.**

- Das Schlüsselpaar entsteht bei der Registrierung im Browser: `generateUserPKCS12KeyStore`, danach Export als lokale .p12-Datei (JS, Abschnitt Registrierung; TB Z.176–180).
  - „Wichtig: Die persönliche Schlüsseldatei wird nicht an den Server übermittelt.“ (TB Z.132)
  - „Die persönliche Schlüsseldatei wird bei der Prüfung NICHT an den Server übermittelt.“ (TB Z.194)
  - Zum Server geht nur das öffentliche Zertifikat (TB Z.221, DSE Z.88). Die SAFE-Einträge erhalten das Zertifikat (FAQ Z.383).
- Entschlüsselt wird im Browser, einmal je Sitzung mit Schlüsseldatei und Passwort (FAQ Z.339, ENT S.1, DSE Z.130). Versendet wird über „Verschlüsseln & senden“ (TB Z.228). Der Client erzeugt dafür einen symmetrischen Schlüssel (TB Z.237). Verfahren laut Code: RSA-OAEP 3072 / SHA-256 und AES-GCM, PBKDF2 mit 204800 Iterationen für den Keystore (JS Z.17).
- Es gibt keine Wiederherstellung durch den Betreiber (FAQ Z.289, KW S.1). Das spricht gegen eine Schlüsselverwahrung beim Betreiber.
- Amtliche Aussagen: Der Nutzer sichert die Schlüsseldatei „in seinem Machtbereich“ (V11 S.2). JuM BW: Ende-zu-Ende „bis in den Webclient des Bürgers“ (NP25 Z.71).
- Gegenposition: Markus Drenger sagt, die Nachrichten würden nur für den Transport verschlüsselt, der Betreiber könne lesen (NP23 Z.51–55, NP25 Z.73) **[unverifiziert]**. Der Client-Code und die Texte stützen das nicht. Drenger beschrieb 2023 eine geplante Anzeige im BundID-Webportal, die es so nicht gibt.
- Einschränkungen:
  - **[Folgerung]** Der Betreiber liefert den JavaScript-Code aus. Damit hängt die Vertraulichkeit vom Vertrauen in den ausgelieferten Code ab, ein typisches Webcrypto-Problem.
  - **[Folgerung]** app.js enthält eine Methode, die den privaten Schlüssel im `localStorage` des Browsers ablegen kann. Ein Aufruf wurde nicht gefunden (JS Z.23). Gespeichert würde ohnehin lokal, nicht beim Betreiber.
  - Der Betreiber verarbeitet Metadaten: Absender, Empfänger, Sachgebiet, Aktenzeichen (DSE, Abschnitt Postfachnutzung). Laut DSE Z.130 sind „alle Kommunikationsdaten verschlüsselt“. Ob die Metadaten beim Betreiber im Klartext liegen, ist offen.
- **[Folgerung]** Für MJP passt die Ausprägung „Ende-zu-Ende verschlüsselt (… EGVP/OSCI der Justiz …)“ besser, in der Variante Web-Client mit Schlüssel allein bei den Nutzenden. Die Schlüsselverwahrung liegt bei den Nutzenden: Datei plus Passwort, ohne Backup beim Betreiber.

### Ü3 Echtheit

- Der Prüfvermerk beim Empfänger weist MJP-Nachrichten als „Sicherer Übermittlungsweg aus dem Postfach- und Versanddienst eines OZG-Nutzerkontos“ aus (TB Z.471, V11 S.3). Er enthält den VHN mit Adressdaten (FAQ Z.301) und Angaben zu qualifizierten elektronischen Signaturen (TB Z.498). Details klärt der Technik-Agent.

### Ü4 Ereignisse und Bestätigungen

- Ausgehende Nachrichten haben einen Sendestatus: „wird gesendet“, „gesendet“ (grün, beim Empfänger eingegangen) oder „Versand fehlgeschlagen“ (TB Z.331–333, FAQ Z.276). Dazu gibt es eine automatische Eingangsbestätigung, die man ansehen und speichern kann (FAQ Z.276, TB Z.349, SPE S.1–2). Bei Fehlschlag muss man neu senden (FAQ Z.228).
- Für eingehende Nachrichten erstellt der Client einen Prüfvermerk (SPE S.1). Der Status gelesen/ungelesen ist nur für Nutzende sichtbar (TB Z.338).
- **[Folgerung]** Eine Lesebestätigung an den Absender wurde nicht gefunden. Ein eEB müssen Nutzende manuell erzeugen und senden (EEB S.1–4).

### I1 Betrieb und Governance

- Anbieter heute: die Arbeitsgruppe IT-Standards in der Justiz der BLK, „im Auftrag der Länder und des Bundes“, vertreten durch ihren Vorsitz, das JuM Baden-Württemberg (IMP, DSE Z.8). Datenschutzrechtlich verantwortlich sind die Justizverwaltungen der Länder und das BMJV (DSE Z.14). Laut JuM BW selbst ist es „für den Betrieb des MJP und für die datenschutzrechtlichen Fragen zuständig“ (NP25 Z.29) **[unverifiziert]**. MJP wurde „im Auftrag der Länder und des Bundes entwickelt“ (FDS-DA Z.161).
- Support: „MJP-Support, ERV-Betriebsteam, Projektbüro der BLK-AG IT-Standards“, mjp-support@justiz.de, Serviceportal mjpserviceportal.de (LÖ S.1). Die Redaktionskonten des Serviceportals haben Adressen bei westernacher.com (FAQ, Kopf). **[Folgerung]** Westernacher ist vermutlich als Dienstleister für Support und Serviceportal tätig.
- Entwicklung: Governikus (JS Z.11). NP23 nennt Governikus als IT-Dienstleister (NP23 Z.47–49) **[unverifiziert]**.
- Herkunft 2023:
  - Initiative von BMJ und BMI. Das BMI führte MJP als BundID-Angebot (Two-Pager von Referat DV I 5, TP S.1; PM Z.25).
  - Die MJP-Server-Komponente lief bei „BMI/ITZBund“ (V11 S.2, Schaubild). Der Support lief über id.bund.de (V11 S.3) **[veraltet]**.
  - ebo.bund.de nannte laut Antragsteller das BMI als Herausgeberin (FDS-DP Z.126) **[unverifiziert]**. Das BMI verwies für die Datenpanne auf die BLK als verantwortliche Stelle (FDS-DP Z.100).
  - **[Folgerung]** Zwischen 2023 und heute ging die Trägerschaft sichtbar vom BMI-/BundID-Umfeld (bund.de-Domain, ITZBund) auf die Justiz (BLK, justiz.de-Domain) über. Wo MJP heute gehostet wird, ist offen.
- service.justiz.de, ein BMJV-Pilotprojekt, umgesetzt von der DigitalService GmbH, bietet nur eine Einrichtungsanleitung, nicht den MJP-Betrieb (SJ Z.190–192).
- Nutzung ist kostenlos (PM Z.15, FAQ Z.43). Zum Pilotstart hatte die BundID 3,4 Mio. Konten (PM Z.25). Das ist keine MJP-Zahl.
- Vorfall: Das SAFE-Verzeichnis mit MJP-Nutzerdaten war vom 13.10. bis 09.11.2023 ohne ausreichenden Schutz abrufbar (NP23 Z.43) **[unverifiziert]**. Am 13.11.2023 wurden die Nutzenden benachrichtigt (FDS-DP).

### I2 Datenhaltung und Versand

- Nachrichten liegen verschlüsselt beim Betreiber, bis sie gelöscht werden, spätestens nach 90 Tagen (DSE Z.130, LÖ S.1). Der SAFE-Verzeichnisdienst der Justiz speichert Name, Anschrift, Länderkennung und das Verschlüsselungszertifikat, solange das Postfach besteht (DSE Z.88–104). Rechtsgrundlage: Art. 6 Abs. 1 lit. c DSGVO in Verbindung mit § 13 Abs. 3 ERVV (DSE Z.98).
- Den SAFE-Eintrag sehen ERV-Teilnehmende: Justiz, Behörden, beA, beN, beSt (DSE Z.96). Nach einer Schätzung sind das rund 1 Mio. Zugriffsberechtigte (NP25 Z.33) **[unverifiziert]**.
- Der Speicherort, also Rechenzentrum oder Land, wird nicht genannt (Lücke).

### I3 Betriebsmodell für Organisationen

- Entfällt: MJP gibt es nur für natürliche Personen (siehe P3). Organisationen nutzen das eBO mit Software von Herstellern (JP, Abschnitt eBO).

### I5 Aufnahme in den Verbund

- Bei der Einrichtung wird automatisch ein SAFE-Eintrag mit dem Rollenwert `ozg_postfach` angelegt (V11 S.1, Fn. 2, DSE Z.96). Eine eigene Registrierung beim SAFE wie beim eBO entfällt (FAQ Z.383). Die Identität kommt aus der BundID.

### R1 Schriftformersatz

- MJP gilt als sicherer Übermittlungsweg nach § 130a Abs. 4 S. 1 Nr. 5 ZPO in Verbindung mit § 13 ERVV (ERL S.2, JP Z.63). Details klärt der Recht-Agent.

### R2 Teilnahmepflicht

- Die Teilnahme ist freiwillig und beruht auf Einwilligung (DSE Z.142, TB Z.212). Eigene Nutzungsbedingungen gibt es nicht: Der Link zu den „terms of conditions“ führt auf die Datenschutzerklärung (TB Z.192).

### R3 Bekanntgabe und förmliche Zustellung

- Bei Zustellungen an MJP-Nutzende gelten die gesetzlichen Regeln (V11 S.1, Fn. 3). Ein eEB wird manuell über eine `xjustiz_nachricht.xml` erstellt (EEB S.1). Details klärt der Recht-Agent.

## Widersprüche

1. **Startdatum:** 12.10.2023 (V11 S.1, ERL S.2) gegenüber 13.10.2023 (PM Z.15).
2. **Authentifizierung:** ERL S.2 (2023) erlaubt die BundID „mit … Personalausweis oder einem Elster-Zertifikat“. JP Z.69 spricht von „Online-Ausweis bzw. sonstiger elektronischer Identifizierungsmittel“. Client und FAQ verlangen nur Vertrauensniveau hoch und lehnen substanziell ab, ELSTER ist substanziell (TB Z.575). **[veraltet?]**
3. **Automatische Löschung:** LÖ S.1 (05/2025) und IFS S.1 sagen „unabhängig vom Status (gelesen, ungelesen …)“. FAQ Z.249 (10/2026) sagt „Ungelesene empfangene Nachrichten werden nicht automatisiert gelöscht“, ähnlich FAQ Z.206 („gelesener und gesendeter Nachrichten“). Wahrscheinlich wurde die Regel geändert.
4. **Ende-zu-Ende:** BLK, JuM BW und Hamburg sagen E2E. Drenger (NP23, NP25) sagt, nicht E2E. Die Client-Belege stützen E2E mit Schlüssel bei den Nutzenden (siehe Ü2). Die morph_box-Formulierung „Schlüsselverwahrung beim Betreiber“ stimmt mit keiner Primärquelle überein.
5. **Sichtbarkeit des SAFE-Eintrags:**
   - Erstinformation 2023: nur Justiz und Behörden, nicht andere MJP- oder eBO-Inhaber (V11 S.1, ERL S.2).
   - DSE Z.96: einsehbar für „Teilnehmende am elektronischen Rechtsverkehr“, ausdrücklich auch „Bürgerinnen und Bürger“.
   - FAQ Z.51: MJP-Nutzende können keine anderen Bürger erreichen.
   - DSE und FAQ widersprechen sich also bei der Einsicht durch Bürger.
6. **Mobile Nutzung:** Das FAQ (Z.401) kündigt sie an, die BLK hat „keine konkreten Planungen“ (FDS-DA Z.107).
7. **BundID-Integration:** 2023 für 2024 geplant (TP S.1). Bis heute nicht umgesetzt, stattdessen wird die FITKO-Zielarchitektur geprüft (FDS-DA Z.161).
8. **morph_box P8 „Freitext“** wird nicht gestützt: Es gibt kein Textfeld, nur Dokumente.

## Lücken

- Keine Nutzungszahlen: weder Anzahl der Postfächer noch Nachrichtenvolumen. Weder BMJ- noch BLK-Quellen, Pressemitteilungen oder Parlamentsantworten wurden gefunden.
- Betrieb heute ungeklärt: Rechenzentrum, Hosting-Land, Vertragsverhältnisse (Governikus, Westernacher, ggf. IT.NRW oder ITZBund), Kosten und Finanzierung (z. B. Umlage nach Königsteiner Schlüssel).
- Status Pilotbetrieb: offen, ob und wann er formell endete.
- EU-eID: Ablauf und Rechtsgrundlage der manuellen „Freischaltung für Nutzer mit EU Identität“ nicht dokumentiert.
- Malware-Prüfung (Ü6): keine Angaben gefunden.
- Metadaten beim Betreiber: offen, ob sie im Klartext oder verschlüsselt liegen und ob eingehende OSCI-Nachrichten serverseitig zwischengespeichert werden.
- Keine eigenen Nutzungsbedingungen, nur Datenschutzerklärung und Einwilligung.
- Das Dokument „EGVP-Drittprodukt-ID Governikus MJP“ (egvp.justiz.de) war nicht abrufbar (HTTP 502).
