# Digital Post (Dänemark): Quellen

Nachweise für die Spalte `Dänemark` in `existing_solutions.csv` und die Dänemark-Einträge in `morph_box.csv`. Evidenzstufen: Statt `[unverifiziert]` stehen hier und in den Dänemark-Einträgen der CSV-Dateien `[Indiz]` (Stufe 2) und `[Annahme]` (Stufe 1); unmarkierte Aussagen sind Stufe 3. `Fehlanzeige` heißt: in den Quellen gesucht, nicht gefunden.

Evidenzstufen (Spalte `Stufe`):

- **3 (Belegt / Primärquelle):** Gesetzestext oder Betreiberdokument, online im Volltext geprüft.
- **2 (Sekundär / Indiz):** Herstelleraussage, Pressemitteilung, Sekundärquelle, veraltete Angabe oder Rückschluss aus einer Primärquelle; `Fehlanzeige` bei gezielter Suche zählt als Indiz für eine negative Aussage.
- **1 (Annahme / Hypothese):** kein Beleg, nicht einmal ein Indiz; offen und zu prüfen.

Bei gemischten Zeilen steht zuerst die Stufe des belegten Teils, danach die Stufe des Teils, der in der Aussage mit `[Indiz]` oder `[Annahme]` markiert ist.

Abgerufen am 2026-11-16 und 2026-11-17, ergänzt am 2026-11-23 (TI, VIRK, FP), nur öffentlich zugängliche Seiten und Dokumente ohne Anmeldung. Es gibt keine lokalen Kopien; TI wurde als PDF heruntergeladen und im Volltext durchsucht, die einschlägigen Abschnitte sind gelesen.

## Hauptquellen

| Kürzel | Quelle | Stufe |
|---|---|---|
| LBK | Bekendtgørelse af lov om Digital Post fra offentlige afsendere, LBK nr. 686 af 15. april 2021 (§§ 2–5, 7, 8, 10, 10 a, 10 b, 10 c), Retsinformation: <https://www.retsinformation.dk/eli/lta/2021/686> | 3 |
| BD | borger.dk, Hilfe zu Digital Post (Verschlüsselung, Benachrichtigung, Ausland, Konkurs, Todesfall, forkyndelse): <https://www.borger.dk/hjaelp-og-vejledning/hvad-har-du-brug-for-hjaelp-til/digital-post/Post> | 3 |
| DP | Digitaliseringsstyrelsen, Digital Post (Plattformen): <https://digst.dk/it-loesninger/digital-post/> | 3 |
| OM | Digitaliseringsstyrelsen, Om løsningen: <https://digst.dk/it-loesninger/digital-post/om-loesningen/> | 3 |
| PF | Digitaliseringsstyrelsen, Platforme til Digital Post: <https://digst.dk/it-loesninger/digital-post/om-loesningen/platforme/> | 3 |
| LEV | Digitaliseringsstyrelsen, Leverandører til Digital Post-løsningen: <https://digst.dk/it-loesninger/digital-post/om-loesningen/leverandoerer/> | 3 |
| AS | Digitaliseringsstyrelsen, Afsender- og modtagersystemer: <https://digst.dk/it-loesninger/digital-post/anvendelse-for-offentlige-myndigheder/afsender-og-modtagersystemer/> | 3 |
| KS | Digitaliseringsstyrelsen, Kontaktstruktur: <https://digst.dk/it-loesninger/digital-post/anvendelse-for-offentlige-myndigheder/kontaktstruktur/> | 3 |
| RET | Digitaliseringsstyrelsen, Rettighedsstyring: <https://digst.dk/it-loesninger/digital-post/anvendelse-for-offentlige-myndigheder/rettighedsstyring/> | 3 |
| VK | Digitaliseringsstyrelsen, Bliv visningsklient: <https://digst.dk/it-loesninger/digital-post/bliv-visningsklient/> | 3 |
| TA | Digitaliseringsstyrelsen, Tilslutningsaftale for kommercielle visningsklienter: <https://digst.dk/it-loesninger/digital-post/bliv-visningsklient/tilslutningsaftale-for-kommercielle-visningsklienter/> | 3 |
| HU | Digitaliseringsstyrelsen, Hjælpeunivers „Send bedre Digital Post“ (MeMo): <https://digst.dk/it-loesninger/digital-post/hjaelpeunivers-send-bedre-digital-post/> | 2 |
| VT | frühere Version von `existing_solutions.csv` (Teamrecherche, keine Primärquelle) | 2 |
| TI | Digitaliseringsstyrelsen, Digital Post – Technical Integration v1.51, exportiert am 04.05.2026 (Kap. 6.9.3, 10.2.3, 10.8, 12.4, 13.1, Fehlercodes): <https://www.digitaliser.dk/Media/639071721293694222/Digital%20Post%20-%20Technical%20Integration%20v1.51.pdf> | 3 |
| VIRK | Virk, Hjælp zu Digital Post für Unternehmen (MitID Erhverv, Rettighedsportal, Modtagersystem): <https://virk.dk/vejledning/virk-hjaelp/digital-post> | 3 |
| FP | Dansk Blindesamfund, Vordruck Vollmacht zur Befreiung von Digital Post, erwähnt Fjernprint-Ordnungen (Sekundärquelle): <https://blind.dk/wp-content/uploads/2023/10/Fuldmagt-til-anmodning-om-fritagelse-fra-digital-post.pdf> | 2 |

## Nachweise je Dimension

| Zeile | Aussage | Quellen | Stufe |
|---|---|---|---|
| B1 | Behörden brauchen ein Absender- und ein Empfangssystem; Versand und Empfang aus Fachsystemen wie Outlook und E-Akte (ESDH); Nachrichtenformat MeMo | AS; OM; HU | 3 |
| B2 | Versand über Distributionsdienstleister, Papier für befreite Empfänger aus demselben Versand (Fjernprint) [Indiz]; Digital Post weist nicht obligatorische Post an befreite Empfänger ab (`recipient.is.exempt`), Absendersystem kann den Befreiungsstatus vorher abfragen | TI Kap. 12.4.2, Fehlercodes; FP | 3, Fjernprint: 2 |
| B3 | Adressierung an CPR- oder CVR-Nummer, ohne dass die Behörde ein eigenes Empfängerverzeichnis pflegt | OM | 3 |
| B4 | MeMo mit Auszeichnung von Antwortmöglichkeiten, Fristen, Terminen und Links zu Selbstbedienungslösungen | OM; HU | 3 |
| B4 | Hauptdokument mit bis zu 10 weiteren Dokumenten (AdditionalDocuments / TechnicalDocuments); Formate nach Whitelist, z. B. PDF und HTML für das Hauptdokument | TI Kap. 10.2.3, 13.1 | 3 |
| P1 | E-Mail und SMS ohne klickbaren Link, Push in der Digital Post-App, Zeitfenster wählbar; Papier nur nach Befreiung | BD | 3 |
| P2 | MitID für Personen | BD | 3 |
| P2, P3 | MitID Erhverv für Unternehmen; Zugang zur Digital Post des Unternehmens zusätzlich über Rechteadministrator:in im Rettighedsportal | VIRK | 3 |
| P3 | Behörden verwalten Rechte von Mitarbeitenden und Partnern im Rettighedsportal, mindestens eine Rechteadministratorin bzw. ein Rechteadministrator | RET | 3 |
| P4 | Lese- und Schreibzugang für andere Personen; Insolvenzverwalter:in auf Anordnung des Gerichts für höchstens 90 Tage; Nachlassverwalter:in über das Rettighedsportal für Digital Post | BD | 3 |
| P5 | staatliche Clients borger.dk, Virk und Digital Post-App; private Clients e-Boks und mit.dk, von Digitaliseringsstyrelsen zugelassen; alle zeigen dasselbe Postfach | DP; OM; PF; LBK § 10 b | 3 |
| P6 | Unternehmen können Post über ein Modtagersystem in eigenen Systemen empfangen; diese Post ist dann nicht mehr in den Clients lesbar | VIRK; TI Kap. 12.4 | 3 |
| P7 | Behörde zeichnet Antwortmöglichkeiten aus; anlasslose Nachricht an sichtbare Kontaktpunkte der Kontaktstruktur; Behörden müssen Digital Post empfangen und beantworten können | OM; KS | 3 |
| P8 | Personen und Unternehmen können über Digital Post mit Behörden kommunizieren | LBK § 8; BD | 3 |
| P8 | Freitext (HTML-Hauptdokument eines Entwurfs) und Dateianhänge über die Clients; je Anhang höchstens 10 MB, je Nachricht höchstens 70 MB | TI Kap. 6.9.3, 10.2.3 | 3 |
| P9 | ein Postfach je CPR- oder CVR-Nummer, Zugang mit MitID über mehrere Clients; bei Tod gesperrt, Aufbewahrung 5 Jahre | OM; BD | 3 |
| Ü1 | asynchroner digitaler Brief | OM; LBK § 7 | 3 |
| Ü2 | verschlüsselte Übermittlung; Inhaltsdaten beim Lieferanten gespeichert und an private Clients vermittelt, daher keine Ende-zu-Ende-Verschlüsselung [Indiz] | BD; LBK §§ 10 b, 10 c | 3, keine E2EE: 2 |
| Ü3 | Nachricht gilt gesetzlich als vom angegebenen Absender stammend; Zugang mit MitID | LBK § 10 Nr. 2; BD | 3 |
| Ü4 | Bereitstellung an Behörde [Indiz]; Digital Post sendet je MeMo eine Geschäftsquittung (Business Receipt) an das Absendersystem, `COMPLETED` heißt validiert; Ereignisse zur Nachricht im Ereignisprotokoll abrufbar | TI Kap. 10.8, 12.4.2; VT | 3, Bereitstellung: 2 |
| Ü4 | forenklet digital forkyndelse: Gericht wird beim Öffnen automatisch benachrichtigt; Aktivitätsprotokoll für Postfachinhaber:in | BD | 3 |
| Ü5 | keine Interoperabilität nach außen [Indiz]; Fehlanzeige in TI zu eDelivery, Peppol oder grenzüberschreitender Zustellung | TI | 2 |
| Ü6 | Virenprüfung von Anhängen beim Versand und im Client (`memo.infected`, `message.file.content.infected`, `scanState`); zulässige Dateiformate je Dokumenttyp per Whitelist; HTML-Dokumente gegen Whitelist validiert | TI Kap. 10.8.5, 13.1, Fehlercodes | 3 |
| I1 | Digitaliseringsstyrelsen als Betreiber; Netcompany, Fellowmind und Statens IT als Lieferanten für Entwicklung und Betrieb | LBK § 2; LEV | 3 |
| I2 | eine Postlösung, ein Postfach, alle Clients greifen auf dieselben Daten zu | LBK § 2; OM; PF | 3 |
| I3 | Behörden binden eigene Absender- und Empfangssysteme an, kein eigenes Postfach-Backend | AS | 3 |
| I4 | Eingang über Kontaktpunkte, Weiterleitung über Empfangssystem in Fachsysteme, Rechte im Rettighedsportal | KS; RET | 3 |
| I5 | private Clients nur mit Zulassung durch Digitaliseringsstyrelsen, Nachweis sicherheitstechnischer und technischer Anforderungen, Tilslutningsaftale | LBK § 10 b; VK; TA; OM | 3 |
| R1 | Kommunikation in beide Richtungen mit Rechtswirkung, auch für Entscheidungen | LBK §§ 7, 8, 10; OM | 3 |
| R2 | Pflicht für Personen ab 15 Jahren mit Wohnsitz in Dänemark und für juristische Einheiten mit CVR-Nummer; Befreiung durch die Kommune; freiwillige Teilnahme ohne Wohnsitz in Dänemark | LBK §§ 3–5; BD | 3 |
| R3 | Zugang, sobald die Nachricht für die Person abrufbar ist, auch in einem Client; forenklet digital forkyndelse der Gerichte, wirksam beim Öffnen oder Bearbeiten | LBK § 10 Nr. 1; BD | 3 |
| R4 | Behörden müssen Digital Post empfangen und beantworten können; Versand über Digital Post ist eine Kann-Bestimmung | OM; LBK § 7 | 3 |

## Evidenzstufen der markierten Stellen

Einordnung der früheren `[unverifiziert]`-Stellen, Stand 2026-11-23. Stufe 1 und 2 werden über den genannten Prüfweg geprüft.

| Datei, Zeile | Aussage | Stufe | Begründung | Prüfweg |
|---|---|---|---|---|
| `existing_solutions.csv`, B2 | Papier für befreite Empfänger aus demselben Versand (Fjernprint) | 2 | Fjernprint nur in Sekundärquelle (FP) erwähnt; laut TI weist Digital Post nicht obligatorische Post an Befreite ab, der Papierversand liegt damit beim Absender | Anfrage Digitaliseringsstyrelsen; Sagsbehandlingsvejledning prüfen |
| `existing_solutions.csv`, B4 | Hauptdokument und Anhänge, z. B. PDF | 3 | MainDocument mit bis zu 10 AdditionalDocuments, PDF in der Format-Whitelist (TI) | – |
| `existing_solutions.csv`, P2 | MitID Erhverv für Unternehmen | 3 | VIRK | – |
| `existing_solutions.csv`, P3 | Unternehmen über MitID Erhverv | 3 | VIRK, mit Rechteadministrator:in im Rettighedsportal | – |
| `existing_solutions.csv`, P6 | Unternehmen empfangen Post in eigenen Systemen | 3 | Modtagersystem (VIRK; TI) | – |
| `existing_solutions.csv`, P8 | Freitext und Dateianhänge | 3 | Entwurf mit HTML-Hauptdokument und Anhängen über die Clients (TI) | – |
| `existing_solutions.csv`, Ü2 | keine Ende-zu-Ende-Verschlüsselung | 2 | früher als „abgeleitet“ vermerkt (Folgerung aus Speicherung beim Lieferanten und Vermittlung an private Clients) | Anfrage Digitaliseringsstyrelsen; Sicherheitsdokumentation prüfen |
| `existing_solutions.csv`, I4 | keine Ende-zu-Ende-Verschlüsselung | 2 | wie Ü2 | wie Ü2 |
| `existing_solutions.csv`, Ü4 | Bereitstellung an Behörde | 2 | Geschäftsquittung bestätigt nur die Validierung (`COMPLETED`), Bereitstellung im Postfach daraus gefolgert; Angabe stammt aus VT | TI Kap. 10.8 und Ereignisprotokoll vollständig prüfen |
| `existing_solutions.csv`, Ü5 | keine Interoperabilität | 2 | Fehlanzeige in TI | Anfrage Digitaliseringsstyrelsen |
| `morph_box.csv`, B2 Ausprägung 2 | Distributionsdienstleister, digital und Papier aus einem Versand (Dänemark) | 2 | wie B2 oben | wie B2 oben |
| `morph_box.csv`, B4 Ausprägung 2 | Dokumente als Anhang, z. B. PDF (Dänemark) | 3 | wie B4 oben | – |
| `morph_box.csv`, P6 Ausprägung 3 | offene API (Dänemark für Unternehmen) | 3 | Modtagersystem für Unternehmen (VIRK; TI); Anschluss setzt Registrierung voraus | – |
| `morph_box.csv`, P8 Ausprägung 1 | Freitext (Dänemark) | 3 | wie P8 oben | – |
| `morph_box.csv`, P8 Ausprägung 2 | Dateien als Anhang (Dänemark) | 3 | wie P8 oben | – |
| `morph_box.csv`, Ü2 Ausprägung 1 | keine Ende-zu-Ende-Verschlüsselung (Dänemark) | 2 | wie Ü2 oben | wie Ü2 oben |
| `morph_box.csv`, Ü4 Ausprägung 1 | Bereitstellung oder Zustellstatus an Behörde (Dänemark) | 2 | wie Ü4 oben | wie Ü4 oben |
| `morph_box.csv`, Ü5 Ausprägung 1 | keine Interoperabilität (Dänemark) | 2 | wie Ü5 oben | wie Ü5 oben |
| `morph_box.csv`, I4 Ausprägung 5 | keine Ende-zu-Ende-Verschlüsselung (Dänemark) | 2 | wie Ü2 oben | wie Ü2 oben |
