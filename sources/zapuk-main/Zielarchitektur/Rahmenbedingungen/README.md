# Föderale Zielarchitektur für Postfach- und Kommunikationslösungen

***Zukunftsweisende föderale Infrastruktur für die Kommunikation zwischen Privatpersonen, privaten Organisationen und öffentlichen Stellen***

**Version 1.0** &ndash; **Stand: 13.05.2025** &ndash; **03 Glossar und Rahmenbedingungen**

> Dieses Dokument ist die maschinenlesbare Textfassung des vom IT-Planungsrat im PDF-Format verabschiedeten Dokuments ["03 Glossar und Rahmenbedingungen" in Version 1.0](./2025-06-26_Beschluss-2025-28_ZaPuK_Glossar-und-Rahmenbedingungen.pdf). Bei der Überführung wurden redaktionelle Änderungen für eine bessere Lesbarkeit und Anpassung an die openCode-Struktur vorgenommen.

-----

| Titel | Dokumentart | Inhalt |
| ----- | ----------- | ------ |
| [Überblick und Handlungsempfehlung](../Zusammenfassung/) | Klammerdokument | Übergreifende Zusammenfassung des Vorhabens und Darstellung strategischer Implikationen |
| [01 Anforderungserhebung und Bestandsanalyse](../../Konsolidierte-Anforderungen/Lesefassung/) | Begleitdokument | Beschreibung des Vorgehens und der Erkenntnisse aus der Anforderungs- und Bestandsanalyse mit Auflistung der 151 konsolidierten Anforderungen |
| [02 Konzept Zielarchitektur](../Architekturkonzept/) | Begleitdokument | Darstellung der auf die Ziele des Vorhabens ausgerichtete Zielarchitektur zur Umsetzung der identifizierten Anforderungen |
| **03 Glossar und Rahmenbedingungen** | **Begleitdokument** | **Übersicht über die zentralen Begriffe und Rahmenbedingungen** |

## Nutzungsbedingungen

Die Inhalte dieses Dokumentes unterliegen der [Creative Commons Namensnennung 4.0 International Public License (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).

------

## 1. Einleitung

Dieses Dokument bietet eine kontextualisierende Übersicht über die zentralen Begriffe und Rahmenbedingungen, die für die föderale Zielarchitektur für Postfach- und Kommunikationslösungen relevant sind.

Das Glossar ([Kapitel 2](#2-glossar-und-abkürzungen)) dient als Nachschlagewerk für feststehende, explizit definierte Begriffe, die in den Dokumenten "[Überblick und Handlungsempfehlung](../Zusammenfassung/)", "[01 Anforderungserhebung und Bestandsanalyse](../../Konsolidierte-Anforderungen/Lesefassung/)", "[02 Konzept Zielarchitektur](../Architekturkonzept/)" sowie den [konsolidierten Anforderungen](../../Konsolidierte-Anforderungen/) verwendet werden. Das Glossar führt zudem die Definitionen und Alternativbezeichnungen der Begriffe, um eine einheitliche und klare Verwendung zu gewährleisten.

Darüber hinaus sind in [Kapitel 3](../Rahmenbedingungen/) relevante rechtliche Rahmenbedingungen aufgeführt (u.a. Beschlüsse des IT-Planungsrats, relevante Gesetze und Verordnungen), die für die föderale Zielarchitektur Postfach- und Kommunikationslösungen von Bedeutung sind.

## 2. Glossar und Abkürzungen

Die jeweils aktuellen Versionen befinden sich auf der Plattform openCode:

- [Glossar](../../Glossar.md)
- [Abkürzungsverzeichnis](../../Abkürzungen.md)

## 3. Rahmenbedingungen

Die erfolgreiche Umsetzung der Zielarchitektur und Vorgehensweise erfordert eine Berücksichtigung von geltenden Rahmenbedingungen. Im Folgenden werden die relevanten rechtlichen und organisatorischen Vorgaben dargestellt, die die Ausgestaltung und Umsetzung des Projekts maßgeblich beeinflussen. Diese umfassen die einschlägigen Beschlüsse und Entscheidungen des IT-Planungsrats sowie der Abteilungsleiterrunde, ebenso wie einschlägige Gesetze und Verordnungen. Die entsprechenden Darstellungen und Erläuterungen dienen der Kontextualisierung der Liefergegenstände und erheben keinen Anspruch auf inhaltliche Vollständigkeit oder juristische Korrektheit.

### 3.1 Gesetze und Verordnungen

Die folgenden Gesetze und Verordnungen sind aufgrund ihrer Schnittstellen von besonderer Relevanz für die Konzeption der Zielarchitektur:

Tabelle 1: Gesetze und Verordnungen

| Gesetz | Erläuterung |
| --- | --- |
| eIDAS-Verordnung über elektronische Identifizierung und Vertrauensdienste 2.0 ([eIDAS-VO 2.0](https://eur-lex.europa.eu/legal-content/DE/TXT/PDF/?uri=OJ:L_202401183)) | Ist eine überarbeitete Version der ursprünglichen EU-Verordnung von 2014, die elektronische Identifizierung und Vertrauensdienste regelt. Sie zielt darauf ab, die grenzüberschreitende Nutzung digitaler Identitäten und Vertrauensdienste zu fördern und deren Sicherheit sowie Interoperabilität zu verbessern.<br><br>Berücksichtigungsgrund: Ermöglicht sichere Identitätsnachweise und digitale Signaturen, die für die Authentifizierung und den Schutz von Nachrichten in elektronischen Postfächern essenziell sind. |
| E-Government Gesetz ([EGovG](http://www.bgbl.de/xaver/bgbl/start.xav?startbk=Bundesanzeiger_BGBl&jumpTo=bgbl113s2749.pdf)) | Wurde eingeführt, um die Digitalisierung der öffentlichen Verwaltung voranzutreiben. Es verpflichtet Bund, Länder und Kommunen, elektronische Zugänge zu Verwaltungsdiensten bereitzustellen, medienbruchfreie Prozesse zu ermöglichen und die Nutzung moderner Technologien wie elektronischer Aktenführung oder  <br>Open Data zu fördern.<br><br>Berücksichtigungsgrund: Unterstützt die Einführung sicherer Kommunikationslösungen und ermöglicht den Austausch von Dokumenten zwischen Bürgern und Behörden über digitale Kanäle. |
| [Onlinezugangsgesetz (OZG)](https://www.recht.bund.de/bgbl/1/2024/245/regelungstext.pdf?__blob=publicationFile&v=3) | Wurde verabschiedet, um die Digitalisierung der öffentlichen Verwaltung voranzutreiben. Es verpflichtet Bund, Länder und Kommunen, Verwaltungsleistungen über elektronische Portale medienbruchfrei anzubieten.<br><br>Berücksichtigungsgrund: Elektronische Postfächer können als Teil eines nutzerfreundlichen digitalen Zugangs zu Verwaltungsdiensten dienen und medienbruchfreie Kommunikation ermöglichen. |
| Gesetz zur Änderung des Onlinezugangsgesetzes sowie weiterer Vorschriften zur Digitalisierung der Verwaltung ([OZG-Änderungsgesetz - OZGÄndG](https://www.recht.bund.de/bgbl/1/2024/245/regelungstext.pdf?__blob=publicationFile&v=3)) | Fördert die Ende-zu-Ende-Digitalisierung von Verwaltungsprozessen, verankert das Once-Only-Prinzip und ermöglicht die Nutzung der BundID (zukünftig DeutschlandID) als zentrales Bürgerkonto. Unternehmen profitieren durch ein digitales Organisationskonto, während unternehmensbezogene Leistungen künftig ausschließlich digital angeboten werden sollen (Digital-Only-Regelung). Zudem werden Standards und Schnittstellen für eine einheitliche Digitalisierung geschaffen, um Nutzerfreundlichkeit und Barrierefreiheit zu verbessern.<br><br>Berücksichtigungsgrund: Stärkt die Notwendigkeit zentraler digitaler Kommunikationslösungen wie elektronischer Postfächer zur Abwicklung von Verwaltungsprozessen ohne Papierform. |
| Single Digital Gateway-Verordnung ([SDG-VO](https://eur-lex.europa.eu/legal-content/DE/TXT/PDF/?uri=CELEX:32018R1724)) | Zielt darauf ab, Bürger:innen und privaten Organisationen einen zentralen digitalen Zugang zu Verwaltungsinformationen und -diensten in der gesamten EU zu ermöglichen. Über ein Portal werden nationale und europäische Plattformen verknüpft, um grenzüberschreitende Verwaltungsverfahren zu erleichtern. Die Verordnung fördert die Digitalisierung von Prozessen, die Transparenz von Vorschriften und die Anwendung des Once-Only-Prinzips, wodurch Daten nur einmal bereitgestellt werden müssen.<br><br>Berücksichtigungsgrund: Elektronische Postfächer können als Schnittstelle für grenzüberschreitende Kommunikation dienen und den sicheren Austausch von Verwaltungsdokumenten ermöglichen. |
| Verwaltungsverfahrensgesetz ([VwVfG](http://www.bgbl.de/xaver/bgbl/start.xav?startbk=Bundesanzeiger_BGBl&jumpTo=bgbl176s1253.pdf)) | Ist die zentrale Verfahrensordnung für die öffentlich-rechtliche Verwaltungstätigkeit in Deutschland. Es regelt, wie Behörden Verwaltungsakte erlassen, öffentlich-rechtliche Verträge abschließen und Verfahren durchführen.<br><br>Berücksichtigungsgrund: Elektronische Postfächer können die Zustellung von Verwaltungsakten digital unterstützen. |
| Verwaltungszustellungsgesetz ([VwZG](http://www.bgbl.de/xaver/bgbl/start.xav?startbk=Bundesanzeiger_BGBl&jumpTo=bgbl105s2354.pdf)) | Regelt die Form und das Verfahren der förmlichen Zustellung von Dokumenten durch Bundesbehörden und Landesfinanzbehörden. Ziel ist es, Empfänger:innen zuverlässig Kenntnis vom Inhalt eines Dokuments zu verschaffen und der Behörde Nachweis über Zeit und Ort der Zustellung zu ermöglichen, was der Rechtssicherheit dient.<br><br>Berücksichtigungsgrund: Elektronische Postfächer sind direkt relevant für die digitale Zustellung von Dokumenten. |

Des Weiteren wurden folgenden Gesetze und Verordnungen ebenfalls als potenziell relevant für eine umfassende Betrachtung der geltenden rechtlichen Rahmenbedingungen identifiziert. Eine detaillierte Prüfung erfolgte bisher bewusst nicht, da ihre analytische Relevanz angesichts verfügbarer operativer Kapazitäten als weniger zentral bewertet wurde:

- Abgabenordnung (AO)
- Barrierefreiheitsstärkungsgesetz (BFSG)
- Behindertengleichstellungsgesetz (BGG)
- Barrierefreie-Informationstechnik-Verordnung BITV 2.0 (BITV 2.0)
- Bundesdatenschutzgesetz (BDSG)
- Gesetz über das Bundesamt für Sicherheit in der Informationstechnik (BSIG)
- Digitale-Dienste-Gesetz (DDG)
- Gesetz für sichere digitale Kommunikation und Anwendungen im Gesundheitswesen (E-Health-Gesetz)
- Gesetz zur Einführung einer Identifikationsnummer (IDNrG)
- Verordnung zur Gewährleistung der IT-Sicherheit der im Portalverbund und zur Anbindung an den Portalverbund genutzten IT-Komponenten (ITSiV-PV)

Die obenstehend benannten regulatorischen Quellen stellen Randbedingungen für die Ausgestaltung einer Zielarchitektur für Postfach- und Kommunikationslösungen dar. Auf dieser Basis erfolgt im nächsten Schritt die Anforderungs- und Bestandsanalyse, die detailliert die bestehenden Strukturen untersucht und die spezifischen Anforderungen an die Zielarchitektur definiert.

### 3.2 Beschlüsse des IT-Planungsrates und der Abteilungsleiterrunde

Die folgenden Beschlüsse des IT-Planungsrats (IT-PLR) und der Abteilungsleiterrunde (AL-Runde) sind von besonderer Relevanz für Konzeption der Zielarchitektur:

Tabelle 2: Beschlüsse des IT-PLR und der AL-Runde

| Entscheidung | Erläuterung |
| --- | --- |
| IT-PLR \| [Beschluss 2024/28](https://www.it-planungsrat.de/beschluss/beschluss-2024-28): Postfach- und Kommunikationslösungen | Im Beschluss wird das Föderale IT-Architekturboard durch den IT-PLR beauftragt eine Zielarchitektur zur Konsolidierung von Postfach- und assoziierten Kommunikationslösungen zu Bürgern, Unternehmen und Organisationen in der föderalen IT-Landschaft auszuarbeiten. |
| AL-Runde \| [Beschluss 2024/04](https://www.it-planungsrat.de/beschluss/beschluss-2024-04-al): Einheitliches Unternehmenskonto | Der Beschluss beinhaltet die Vereinheitlichung der bestehenden Postfachsysteme (Postfach 2.0 und OZG-PLUS-Postfach) mit den notwendigen Ausbaustufen gegenüber der Verwaltung und den Unternehmen. Die Länder Bayern und Bremen werden mit der Planung und Umsetzung der Vereinheitlichung durch die AL-Runde beauftragt. |
| IT-PLR \| [Beschluss 2023/48](https://www.it-planungsrat.de/beschluss/beschluss-2023-48): Unternehmenskonto | In dem Beschluss werden die für das Produkt "Mein Unternehmenskonto" zuständigen Länder Bayern und Bremen u. a beauftragt die Bereitstellung einer zentralen Lösung (ein Postfach) mit allen relevanten Funktionalitäten zu prüfen. |
| IT-PLR \| [Beschluss 2023/11](https://www.it-planungsrat.de/beschluss/beschluss-2023-11): Behördennummer 115 | Der Beschluss 2023/11 behandelt die Weiterentwicklung und Umsetzung der Behördennummer 115. Diese Nummer dient als zentrale Anlaufstelle für Bürger, um Informationen und Dienstleistungen der öffentlichen Verwaltung zu erhalten. |
| IT-PLR \| [Beschluss 2023/29](https://www.it-planungsrat.de/beschluss/beschluss-2023-29): Zentrales Bürgerpostfach | Der Beschluss beschreibt die Wichtigkeit eines digitalen Rückkanals für Bürgerinnen und Bürger (Bidirektionalität) und Integration des Statusmonitors. |
| IT-PLR \| [Beschluss 2022/04](https://www.it-planungsrat.de/beschluss/beschluss-2022-04): Zentrales Bürgerpostfach | In dem Beschluss spricht sich der IT-PLR perspektivisch für ein zentrales Postfach aus. Die Länder sollen anstelle ihrer Länder-Postfachlösungen das NK-Bund-Postfach als Postfach nutzen können. |
