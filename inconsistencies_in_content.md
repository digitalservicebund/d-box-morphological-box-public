# Inhaltliche Auffälligkeiten

Gesammelt bei der Einordnung der Evidenzstufen (Stand 2026-11-23). Nichts davon ist in `morph_box.csv` oder `existing_solutions.csv` korrigiert. Details und Fundstellen stehen in der jeweiligen `sources/Referenzen-<Name>.md`.

## Übergreifend

- **Uneinheitliche Prüftiefe bei Aussagen ohne Marker:** Bei BundID, ELSTER, MJP, ZaPuK, Dänemark und Norwegen sind auch Aussagen markiert, die bisher keinen Marker hatten, aber nur auf Fehlanzeige oder Rückschluss beruhen. Bei MUK, Österreich und UK ist das noch nicht systematisch geschehen.
- **Webquellen als Stufe 3:** Für Dänemark, Norwegen, Österreich und UK gibt es keine lokalen Kopien. Die Agents haben die Online-Quellen abgerufen, teils nur über Zusammenfassungen eines Abruf-Tools (Dänemark, Norwegen). Bei Dänemark stehen ältere Primärquellen auf Stufe 3, die der Agent nicht selbst nachgeprüft hat (nur OM und BD).
- **Ü3 Ausprägung 5 in `morph_box.csv` („Vertrauensniveau hoch“):** Unternehmenskonto und BundID stehen dort. Laut § 9a Abs. 6 und § 3 Abs. 4 Nr. 2 OZG genügt aber ELSTER substanziell. § 9a Abs. 5 OZG gilt nur für Online-Formulare eines Verwaltungsportals und knüpft an § 3 Abs. 4 OZG an, nicht ausdrücklich an das Vertrauensniveau hoch. Die Zuordnung „hoch / § 3a VwVfG“ ist fraglich.

## BundID / ZBP

- **B2:** Die Folie MV F4 zeigt eine Webanwendung, die alle Behörden in M-V nutzen können (Testumgebung, Versand ins MUK). „Keine Web-Oberfläche für Behörden“ gilt nur für das ZBP selbst. Der Text in der Referenzen-Datei („kein Zugang für Behörden ohne Fachverfahren“) weicht im Wortlaut von der CSV-Zelle ab.
- **P2:** Die Angabe „ab Januar 2027“, die aus dem Personalausweisportal stammen soll, steht nicht in der lokalen Kopie dieser Seite. Den Start von d-you nennt nur BT-Drs. 21/9020 S. 2.
- **P4:** Das Whitepaper 2022 (WP22 S. 26) nennt eine „vorkonzeptionierte“ Vertreterregelung mit Zugriff auf fremde Postfächer. Heute ist keine dokumentiert.
- **Ü2:** WP22 nennt „verschlüsselt gespeichert“, die aktuelle Datenschutzerklärung sagt dazu nichts.
- **R4 Ausprägung 1 („ohne Anreize“):** BT-Drs. D9824 S. 6: „Die Kosten der BundID trägt derzeit der Bund“ (2023). Das kann als Anreiz gelten.

## Mein Unternehmenskonto (MUK)

- **P4:** „externe Vertretung, z. B. durch Steuerberatung“ lässt offen, ob es sie gibt oder nicht.

## ELSTER

- **P9:** Dass Mein ELSTER „kein Nutzerkonto im Sinne des OZG“ ist, steht nur in ZaPuK ADR-1110; dessen Fußnote verweist auf § 122a AO, nicht auf das OZG. BAYVV Nr. 4.1.2 nennt das Postfach 2.0 ein Postfach nach § 2 Abs. 7 OZG. Es geht um verschiedene Portale, sollte aber geklärt werden.
- **P4:** Belegt sind nur die eigene Kennung `vnid` (SSb-B 7.7) und die eigene Einwilligung. Das „eigene Postfach“ der Bevollmächtigten ist ein Rückschluss; betrifft auch `morph_box.csv`.
- **P6:** ERP-Software kommt weder in der ERiC-Doku noch im Lizenzvertrag vor. Die MUK-FAQ nennt nur eine M2M-Abholung in Unternehmensinfrastrukturen.
- **I1:** Bayern als „Auftrag nehmendes Land für ELSTER“ beruht nur auf einer Pressemitteilung (PM-161) und einer Broschüre (NRW-2015), beide nicht geöffnet. KONSENS-G nennt ELSTER nicht.
- **Ü3 Ausprägung 1 in `morph_box.csv`:** ELSTER steht unter „keine“ Echtheitssicherung, obwohl das Fehlen eines Siegels nur eine Fehlanzeige ist. Nicht markiert, weil die Stelle mit dem Unternehmenskonto geteilt ist.
- Weiter offen aus der Referenzen-Datei: Zertifikatsleitfaden „je Unternehmen“ gegenüber „je Steuernummer“ in der CSV; `Virengeprueft` bei `OZGPPNachricht`.

## Mein Justizpostfach (MJP)

- **R1:** § 3a Abs. 3 Nr. 2 Buchst. c VwVfG erfasst Postfächer natürlicher Personen, die nach einer Identifizierung gemäß ERVV eingerichtet wurden, und könnte MJP gegenüber Verwaltungsbehörden doch abdecken. MJP gilt seit 10/2026 auch gegenüber Gerichtsvollzieher:innen; „nur gegenüber Gerichten“ in `morph_box.csv` R1 passt nicht ganz.
- **R3:** Ob MJP ein Postfach nach § 2 Abs. 7 OZG ist, bleibt offen.
- **P1:** § 9 OZG kennt eine Benachrichtigungspflicht; ob sie für MJP gilt, ist nicht geprüft.
- **I2:** Für 2023 sind BMI/ITZBund genannt (Handreichung). Nach dem Betreiberwechsel 07/2026 sind Rechenzentrum und Auftraggeber offen.
- Weiter offen aus der Referenzen-Datei (Abschnitt „Abweichungen“): Startdatum 12. oder 13.10.2023; Verantwortung BMI, BMJ oder BLK; ELSTER laut ERVV gegenüber Vertrauensniveau hoch im Client; Löschung laut LÖ gegenüber FAQ; mobile Nutzung laut FAQ gegenüber FDS-DA.

## ZaPuK

- **Papier-Rückfall:** Laut `existing_solutions.csv` ist kein Papier-Rückfall vorgesehen. Das Rechtsgutachten (RGA Z. 1233) empfiehlt eine Rückfallebene für nicht digital erreichbare Personen; R2 in der Referenzen-Datei nennt den Papier-Rückfall als Empfehlung.
- **Massenversand:** `morph_box.csv` B2 Ausprägung 1 führt ZaPuK unter „Web-Oberfläche … auch für Massenversand per Tabellen-Upload“. Laut `existing_solutions.csv` ist Massenversand bei ZaPuK nicht vorgesehen.
- **Push in Neo:** Der Integrationsleitfaden beschreibt Push in der App als Funktion, laut Issue-Tracker ist Push nicht umgesetzt. Auch „E-Mail im Web“ ist laut IL Z. 290 nur „UI in Arbeit“; die CSV behauptet dafür allerdings nur die Konzeption.
- **P8 Ausprägung 3 in `morph_box.csv`:** Neo steht bei „strukturierte Antwort über ein bereitgestelltes Formular“. Neo hat nur Einfach- und Mehrfachauswahl, Formulare sind nicht umgesetzt (siehe B4).

## Dänemark

- **B2 / Fjernprint:** Laut Technical Integration (TI) lehnt Digital Post nicht obligatorische Post an befreite Empfänger ab (`recipient.is.exempt` = NOT_ALLOWED). Den Papierversand organisiert die absendende Behörde selbst. „Papier aus demselben Versand“ stimmt für Digital Post wohl nicht; Fjernprint nennt nur eine Sekundärquelle.
- **Ü6 in `existing_solutions.csv`:** Die Zelle steht auf „?“, obwohl TI eine Virenprüfung der Anhänge (`memo.infected`, `scanState`) und Positivlisten für Dateiformate belegt. Vorschlag: „Virenprüfung der Anhänge; Dateiformate nach Positivliste je Dokumenttyp“.
- **P5 / I2 („alle Clients zeigen dasselbe Postfach“):** Laut virk.dk ist Post, die ein Unternehmen über ein Modtagersystem abholt, danach nicht mehr in den Clients lesbar.
- **Ü4:** Die Geschäftsquittung mit Status `COMPLETED` bestätigt nur die Validierung der Nachricht, nicht ausdrücklich die Bereitstellung im Postfach.
- **Ü2 Ausprägung 1 in `morph_box.csv`** („alle Vergleichsländer, u. a. Dänemark“) beruht auf derselben Folgerung zur fehlenden Ende-zu-Ende-Verschlüsselung, ist aber unmarkiert.

## Norwegen

- **Ü2:** Die KRR-Doku nennt das Zertifikat „brukerens krypteringssertifikat“. CSV und Referenzen-Datei sprechen vom Zertifikat des Postfachanbieters.
- **P1 / R3:** VAR erwähnt keine Erinnerung nach einer Woche. Sie stammt aus EFF § 8 und gilt nur für Einzelentscheidungen (enkeltvedtak), nicht beim Versand an eine vom Empfänger angegebene Adresse. EFF § 8 nennt E-Mail und SMS nicht ausdrücklich. Die CSV verallgemeinert.
- **P1 „Papier“:** EFF § 32 regelt nur veraltete Kontaktdaten (18 Monate), nicht den Papierversand, wird aber dafür zitiert.
- **Falsch zugeordnete Quellen:** AKT nennt weder Digdir als Sentralforvalter noch Langzeitspeicherung nach Wahl der Person noch Verträge zwischen Digdir und Postfachanbietern. DPK nennt kein Sicherheitsniveau je Nachricht (P2). R4 war mit EF belegt, das dazu nichts enthält (jetzt RS).
- **R4:** Laut Suchtreffern hat Digdir vorgeschlagen, die DPI-Pflicht aus dem Digitalisierungsrundschreiben zu streichen; aktueller Stand offen.
- **R1:** Das forvaltningsloven soll durch LOV-2025-06-20-81 aufgehoben werden, Inkrafttreten offen. R1 könnte dadurch veralten.
- **KV:** Die Seiten beschreiben noch den Meldingsformidler der alten Infrastruktur, die mit Peppol abgelöst wurde.
- **Anbietername:** DPK nennt „Posten og e-Boks“, die CSV „Digipost“ (Produkt der Posten); nur ein Namensunterschied.

## Österreich

- **R2:** USP-3 und BKA-U nennen Kleinstunternehmen ohne Pflicht zur Umsatzsteuervoranmeldung „ausgenommen“. § 1b Abs. 4 E-GovG und oesterreich.gv.at (OE-7) sprechen von einem Widerspruch. Die CSV folgt dem Gesetz.
- **P2:** OE-4 erwähnt die FinanzOnline- und USP-Kennung nicht; belegt ist sie nur über das White Paper (Tabelle 2).
- **B3:** § 28b ZustG nennt keinen „ERV-Code“, sondern „Anschriftcodes des Zustellsystems gemäß § 28 Abs. 3 Z 3“; inhaltlich dasselbe.
- **I5:** „Kommunikationssysteme der Behörden nur registriert“ ist nicht eigens geprüft. BKA-T verweist auf § 37 Abs. 2a ZustG, wonach diese Systeme im Internet bekanntzugeben sind.

## UK (GOV.UK Notify)

- **I2:** CSV und Referenzen-Datei nennen nur „AWS in UK und Irland“. Die Sicherheitsseite (SEC) nennt zusätzlich Unterauftragsverarbeiter im UK und im EWR; SMS werden auch im Land des Empfangsgeräts gespeichert.
- **P2 / B4:** Die Bestätigung der E-Mail-Adresse vor dem Datei-Download ist laut REST-Doku standardmäßig an (`confirm_email_before_download`). In der CSV klingt sie wie eine Option.
- **Ü1:** FEAT nennt Druck und Versand bis 15 Uhr am nächsten Werktag, DEL Druck um 17:30 Uhr und Versand am nächsten Werktag. Passt grob zu „Briefe am nächsten Werktag“.
- **R2:** Die Nutzungsbedingungen verlangen eine Einwilligung der Empfangenden und verbieten unaufgeforderte Nachrichten. Fehlt in R2, widerspricht aber nicht.
- **R1 in `morph_box.csv`:** „Regelfall mit Opt-out, Antrag auf Papier (… UK HMRC ab 2026)“ ist unmarkiert und durch keine Quelle belegt, auch nicht durch die HMRC-Seite.
- **Unmarkierte Rückschlüsse:** „kein zentrales Verzeichnis“ (B3), „keine Vertretungsfunktion“ (P4), „keine Interoperabilität“ (Ü5) sind eigentlich Rückschlüsse, stehen aber auf Stufe 3.

## Schweden

- Keine Referenzen-Datei. Die Angabe in `morph_box.csv` I2 („Norwegen, Schweden“) ist deshalb `[Annahme]`.
