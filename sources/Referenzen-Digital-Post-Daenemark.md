# Digital Post (Dänemark): Quellen

Nachweise für die Spalte `Dänemark` in `existing_solutions.csv` und die Dänemark-Einträge in `morph_box.csv`. Einträge mit `[unverifiziert]` sind in keiner der Quellen ausdrücklich belegt. Abgerufen am 2026-11-16 und 2026-11-17, nur öffentlich zugängliche Seiten ohne Anmeldung.

## Hauptquellen

| Kürzel | Quelle |
|---|---|
| LBK | Bekendtgørelse af lov om Digital Post fra offentlige afsendere, LBK nr. 686 af 15. april 2021 (§§ 2–5, 7, 8, 10, 10 a, 10 b, 10 c), Retsinformation: <https://www.retsinformation.dk/eli/lta/2021/686> |
| BD | borger.dk, Hilfe zu Digital Post (Verschlüsselung, Benachrichtigung, Ausland, Konkurs, Todesfall, forkyndelse): <https://www.borger.dk/hjaelp-og-vejledning/hvad-har-du-brug-for-hjaelp-til/digital-post/Post> |
| DP | Digitaliseringsstyrelsen, Digital Post (Plattformen): <https://digst.dk/it-loesninger/digital-post/> |
| OM | Digitaliseringsstyrelsen, Om løsningen: <https://digst.dk/it-loesninger/digital-post/om-loesningen/> |
| PF | Digitaliseringsstyrelsen, Platforme til Digital Post: <https://digst.dk/it-loesninger/digital-post/om-loesningen/platforme/> |
| LEV | Digitaliseringsstyrelsen, Leverandører til Digital Post-løsningen: <https://digst.dk/it-loesninger/digital-post/om-loesningen/leverandoerer/> |
| AS | Digitaliseringsstyrelsen, Afsender- og modtagersystemer: <https://digst.dk/it-loesninger/digital-post/anvendelse-for-offentlige-myndigheder/afsender-og-modtagersystemer/> |
| KS | Digitaliseringsstyrelsen, Kontaktstruktur: <https://digst.dk/it-loesninger/digital-post/anvendelse-for-offentlige-myndigheder/kontaktstruktur/> |
| RET | Digitaliseringsstyrelsen, Rettighedsstyring: <https://digst.dk/it-loesninger/digital-post/anvendelse-for-offentlige-myndigheder/rettighedsstyring/> |
| VK | Digitaliseringsstyrelsen, Bliv visningsklient: <https://digst.dk/it-loesninger/digital-post/bliv-visningsklient/> |
| TA | Digitaliseringsstyrelsen, Tilslutningsaftale for kommercielle visningsklienter: <https://digst.dk/it-loesninger/digital-post/bliv-visningsklient/tilslutningsaftale-for-kommercielle-visningsklienter/> |
| HU | Digitaliseringsstyrelsen, Hjælpeunivers „Send bedre Digital Post“ (MeMo): <https://digst.dk/it-loesninger/digital-post/hjaelpeunivers-send-bedre-digital-post/> |
| VT | frühere Version von `existing_solutions.csv` (Teamrecherche, keine Primärquelle) |

## Nachweise je Dimension

| Zeile | Aussage | Quellen |
|---|---|---|
| B1 | Behörden brauchen ein Absender- und ein Empfangssystem; Versand und Empfang aus Fachsystemen wie Outlook und E-Akte (ESDH); Nachrichtenformat MeMo | AS; OM; HU |
| B2 | Versand über Distributionsdienstleister, Papier für befreite Empfänger aus demselben Versand (Fjernprint) [unverifiziert] | – |
| B3 | Adressierung an CPR- oder CVR-Nummer, ohne dass die Behörde ein eigenes Empfängerverzeichnis pflegt | OM |
| B4 | MeMo mit Auszeichnung von Antwortmöglichkeiten, Fristen, Terminen und Links zu Selbstbedienungslösungen | OM; HU |
| B4 | Hauptdokument mit Anhängen, z. B. PDF [unverifiziert] | – |
| P1 | E-Mail und SMS ohne klickbaren Link, Push in der Digital Post-App, Zeitfenster wählbar; Papier nur nach Befreiung | BD |
| P2 | MitID für Personen | BD |
| P2, P3 | MitID Erhverv für Unternehmen [unverifiziert] | – |
| P3 | Behörden verwalten Rechte von Mitarbeitenden und Partnern im Rettighedsportal, mindestens eine Rechteadministratorin bzw. ein Rechteadministrator | RET |
| P4 | Lese- und Schreibzugang für andere Personen; Insolvenzverwalter:in auf Anordnung des Gerichts für höchstens 90 Tage; Nachlassverwalter:in über das Rettighedsportal für Digital Post | BD |
| P5 | staatliche Clients borger.dk, Virk und Digital Post-App; private Clients e-Boks und mit.dk, von Digitaliseringsstyrelsen zugelassen; alle zeigen dasselbe Postfach | DP; OM; PF; LBK § 10 b |
| P6 | Unternehmen können Post in eigenen Systemen empfangen [unverifiziert] | – |
| P7 | Behörde zeichnet Antwortmöglichkeiten aus; anlasslose Nachricht an sichtbare Kontaktpunkte der Kontaktstruktur; Behörden müssen Digital Post empfangen und beantworten können | OM; KS |
| P8 | Personen und Unternehmen können über Digital Post mit Behörden kommunizieren | LBK § 8; BD |
| P8 | Freitext und Dateianhänge [unverifiziert] | – |
| P9 | ein Postfach je CPR- oder CVR-Nummer, Zugang mit MitID über mehrere Clients; bei Tod gesperrt, Aufbewahrung 5 Jahre | OM; BD |
| Ü1 | asynchroner digitaler Brief | OM; LBK § 7 |
| Ü2 | verschlüsselte Übermittlung; Inhaltsdaten beim Lieferanten gespeichert und an private Clients vermittelt, daher keine Ende-zu-Ende-Verschlüsselung (abgeleitet) | BD; LBK §§ 10 b, 10 c |
| Ü3 | Nachricht gilt gesetzlich als vom angegebenen Absender stammend; Zugang mit MitID | LBK § 10 Nr. 2; BD |
| Ü4 | Bereitstellung an Behörde [unverifiziert] | VT |
| Ü4 | forenklet digital forkyndelse: Gericht wird beim Öffnen automatisch benachrichtigt; Aktivitätsprotokoll für Postfachinhaber:in | BD |
| Ü5 | keine Interoperabilität nach außen [unverifiziert] | – |
| Ü6 | keine Angaben zur Malware-Prüfung und zu zulässigen Dateiformaten gefunden [unverifiziert] | – |
| I1 | Digitaliseringsstyrelsen als Betreiber; Netcompany, Fellowmind und Statens IT als Lieferanten für Entwicklung und Betrieb | LBK § 2; LEV |
| I2 | eine Postlösung, ein Postfach, alle Clients greifen auf dieselben Daten zu | LBK § 2; OM; PF |
| I3 | Behörden binden eigene Absender- und Empfangssysteme an, kein eigenes Postfach-Backend | AS |
| I4 | Eingang über Kontaktpunkte, Weiterleitung über Empfangssystem in Fachsysteme, Rechte im Rettighedsportal | KS; RET |
| I5 | private Clients nur mit Zulassung durch Digitaliseringsstyrelsen, Nachweis sicherheitstechnischer und technischer Anforderungen, Tilslutningsaftale | LBK § 10 b; VK; TA; OM |
| R1 | Kommunikation in beide Richtungen mit Rechtswirkung, auch für Entscheidungen | LBK §§ 7, 8, 10; OM |
| R2 | Pflicht für Personen ab 15 Jahren mit Wohnsitz in Dänemark und für juristische Einheiten mit CVR-Nummer; Befreiung durch die Kommune; freiwillige Teilnahme ohne Wohnsitz in Dänemark | LBK §§ 3–5; BD |
| R3 | Zugang, sobald die Nachricht für die Person abrufbar ist, auch in einem Client; forenklet digital forkyndelse der Gerichte, wirksam beim Öffnen oder Bearbeiten | LBK § 10 Nr. 1; BD |
| R4 | Behörden müssen Digital Post empfangen und beantworten können; Versand über Digital Post ist eine Kann-Bestimmung | OM; LBK § 7 |
