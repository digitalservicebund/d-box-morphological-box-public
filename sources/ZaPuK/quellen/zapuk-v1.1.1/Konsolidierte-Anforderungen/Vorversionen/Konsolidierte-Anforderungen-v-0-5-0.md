<!-->
> ⚠️ **Hinweis: Nicht beschlossene Entwurfsversion** ⚠️
>
> - Die Beratung des IT-Planungsrates zu Fragen der föderalen IT-Architektur obliegt nach [FIT-AM Rahmenkonzept (B-2024/26-IT)](https://www.it-planungsrat.de/fileadmin/beschluesse/2024/Beschluss2024-26_F%C3%B6derales_IT-Architekturmanagement_Rahmenkonzept_Version_2.0.pdf) dem Föderalen IT-Architekturboard (FIT-AB). Die hier veröffentlichte Version der konsolidierten Anforderungsliste in Version 0.5 wurde noch nicht durch das FIT-AB beschlossen und ist daher strikt als ein vorläufiges Ergebnis auf Arbeitsebene zu lesen. (Stand: 14.02.2025)
-->
# Konsolidierte Anforderungsliste

**Version 0.5.0** | Stand: 14.02.2025

> Dies ist eine versionierte und archivierte Fassung der konsolidierten Anforderungsliste der Zielarchitektur Postfach- und Kommunikationslösungen. 
> Vorausgehende Versionen können [hier](../Vorversionen/) eingesehen werden.
> Die letzte veröffentliche Verison kann [hier](../README.md) eingesehen werden. 

**Hauptthemen:**

- [Thema: Übergreifend](#thema-übergreifend)
- [Thema: Use Cases](#thema-use-cases)
- [Thema: Nachrichten](#thema-nachrichten)
- [Thema: Identifikation](#thema-identifikation)
- [Thema: Adressierung](#thema-adressierung)
- [Thema: Transport](#thema-transport)
- [Thema: Zustellung](#thema-zustellung)
- [Thema: Speicherung](#thema-speicherung)
- [Thema: Anbindung](#thema-anbindung)
- [Thema: Sicherheit](#thema-sicherheit)
- [Thema: Architektur](#thema-architektur)
- [Thema: Postfachzugang für Bürger:innen](#thema-postfachzugang-für-bürgerinnen)

## Thema: Übergreifend

| Subthema[^2] | Konsolidierte Anforderung | Anforderungs-ID[^1] | Feedback[^3] |
| --- | --- | --- | --- |
| _NA_ | \[Das System\] soll vollständig E2E digitalisierte Verwaltungsprozesse unterstützen. | ANF_FUN_ÜBE_002 |  _Link folgt_ |
| _NA_ | \[Das System\] erlaubt eine einfache und intuitive Bedienung, um auch unregelmäßigen und nicht IT-affinen \[Nutzer:innen\] eine Verwendung zu ermöglichen. | BED_BEN_ÜBE_008 |  _Link folgt_ |
| _NA_ | \[Das System\] ermöglicht eine medienbruchfreie Kommunikation. | BED_BEN_ÜBE_002 |  _Link folgt_ |
| _NA_ | \[Das System\] ist sehr einfach nutzbar, um eine hohe Akzeptanz zu erfahren. | BED_AKZ_ÜBE_001 |  _Link folgt_ |
| _NA_ | \[Das System\] ermöglicht eine barrierefreie Kommunikation. | BED_BAR_ÜBE_003 |  _Link folgt_ |
| _NA_ | \[Das System\] erfüllt die Anforderungen an die Barrierefreiheit gemäß der Barrierefreie-Informationstechnik-Verordnung BITV 2.0. | BED_BAR_ÜBE_007 |  _Link folgt_ |
| _NA_ | \[Das System\] ermöglicht \[Privatpersonen\] und \[privaten Organisationen\] über einen einzigen Zugang eine Kommunikation über alle Bereiche der \[öffentlichen Stellen\] hinweg (Allgemeine Innere Verwaltung; Schulwesen; Wissenschaft; Forschung und Kultur; Gesundheit; Sport und Erholung; Sozialwesen; Steuerverwaltung; Wirtschaftsförderung; Gesundheitswesen; Justizverwaltung; usw.). | RAH_ÜBE_144 |  _Link folgt_ |
| _NA_ | \[Das System\] ermöglicht den verschiedenen Staatsgewalten (Judikative, Exekutive, Legislative) die unabhängige Wahrnehmung ihrer verfassungsmäßigen Rolle. | RAH_ÜBE_143 |  _Link folgt_ |
| _NA_ | \[Das System\] erfüllt die Anforderungen des Servicestandards für die digitale Verwaltung (DIN SPEC 66336 - Qualitätsanforderungen für Onlineservices und -portale der öffentlichen Verwaltung). | RAH_VOR_006 |  _Link folgt_ |
| _NA_ | Die Bestandteile \[des Systems\] sind unter einer offenen Software-Lizenz lizenziert. | POR_AUS_HER_002 |  _Link folgt_ |

## Thema: Use Cases

| Subthema | Konsolidierte Anforderung | Anforderungs-ID | Feedback |
| --- | --- | --- | --- |
| Use Case | \[Öffentliche Stellen\] können \[elektronischen Schriftverkehr\] in Form von \[Nachrichten\] mit \[Nutzer:innen\] und untereinander austauschen. | ANF_FUN_USE_036 |  _Link folgt_ |
| Use Case | \[Nutzer:innen\] können mit am \[Portalverbund\] angeschlossenen \[öffentlichen Stellen\] vorgangsbezogen kommunizieren. (i.S.v. §2 Abs. 7 OZG) | ANF_FUN_USE_001 |  _Link folgt_ |
| Use Case | \[Das System\] ermöglicht es \[Online-Diensten\] und \[Fachverfahren\] \[Nachrichten\] in das \[Postfach\] einer \[Privatperson\] und/oder einer \[privaten Organisation\] zuzustellen. | ANF_FUN_USE_060 |  _Link folgt_ |
| Use Case | \[Nutzer:innen\] können \[Nachrichten\] und deren \[Anhänge\] empfangen und einsehen. | ANF_FUN_USE_037 |  _Link folgt_ |
| Use Case | Zur \[Identifikation\] von \[privaten Organisationen\] werden mindestens die in § 3 OZG genannten \[Identifizierungsmittel\] unterstützt. Die \[Identifikation\] muss mindestens über das einheitliche Organisationskonto gemäß § 3 OZG erfolgen können. | ANF_FUN_USE_066 |  _Link folgt_ |
| Use Case | Das System ermöglicht eine \[Echtzeitkommunikation\] zwischen zwei \[Nutzer:innen\]. | ANF_FUN_USE_055 |  _Link folgt_ |
| Use Case | \[Das System\] ermöglicht \[öffentlichen Stellen\] das Versenden von \[Bescheiden\] an \[Privatpersonen\] , \[private Organisationen\] oder \[öffentliche Stellen\]. | ANF_FUN_USE_063 |  _Link folgt_ |
| Use Case | \[Das System\] ermöglicht \[Nutzer:innen\] den Empfang von \[Bescheiden\]. | ANF_FUN_USE_062 |  _Link folgt_ |
| Use Case | \[Das System\] ermöglicht das Beantworten von \[Nachrichten\]. | ANF_FUN_USE_064 |  _Link folgt_ |
| Use Case | \[Das System\] unterstützt eine bidirektionale Kommunikation, ggf. eingeschränkt durch die \[Absender:in\] auf unidirektional. | ANF_FUN_USE_069 |  _Link folgt_ |
| Use Case | \[Das System\] ermöglicht der \[Nutzer:in\] das Anlegen, Bearbeiten und Löschen von \[Nachrichten\] und \[Anhängen\] im Posteingang. | ANF_FUN_USE_070 |  _Link folgt_ |
| Use Case | \[Das System\] ermöglicht der \[Nutzer:in\] das Anlegen, Löschen, Sperren und die Wiederherstellung des eigenen \[Postfachs\]. | ANF_FUN_USE_071 |  _Link folgt_ |
| Use Case | \[Das System\] ermöglicht die Nutzung von \[Funktionspostfächern\]. | ANF_FUN_USE_072 |  _Link folgt_ |
| Use Case | \[Das System\] kann in einer zukünftigen Ausbaustufe zu einem eIDAS-konformen Dienst für die \[Zustellung\] \[elektronischer Einschreiben\] weiterentwickelt werden. | ANF_FUN_USE_077 |  _Link folgt_ |
| Teilnehmer | \[Das System\] kann von \[Privatpersonen\], \[privaten Organisationen\] und \[öffentlichen Stellen\] genutzt werden \[= Nutzer:innen\]. | ANF_FUN_USE_026 |  _Link folgt_ |
| Teilnehmer | \[Nachrichten\] können auch mit Hilfe von \[Drittanwendung\] an \[Nutzer:innen\] übermittelt und von diesen empfangen werden. | ANF_FUN_USE_048 |  _Link folgt_ |
| Teilnehmer | \[Das System\] ermöglicht die Nutzung durch vertretungsberechtigte natürliche Personen. | ANF_FUN_USE_075 |  _Link folgt_ |
| Teilnehmer | \[Das System\] unterscheidet in den Rollen \[Postfachmitglieder\] und \[Org-Admins\] im Kontext \[Funktionspostfach\]. | ANF_FUN_USE_074 |  _Link folgt_ |
| Administration | \[Org-Admins\] können Systemmeldungen an \[Nutzer:innen\] senden. | ANF_FUN_USE_059 |  _Link folgt_ |
| Administration | \[Öffentliche Stellen\] und \[private Organisationen\] können über eine Self-Service-Funktionalität eine Übersicht ihrer \[Postfächer\] einsehen. | ANF_FUN_USE_080 |  _Link folgt_ |


## Thema: Nachrichten

| Subthema | Konsolidierte Anforderung | Anforderungs-ID | Feedback |
| --- | --- | --- | --- |
| Nachricht | \[Das System\] ist aus technischer Sicht in der Lage, beliebig große \[Nachrichten\] und \[Anhänge\] zu versenden. | LEI_KAP_ÜBE_014 |  _Link folgt_ |
| Nachricht | Die Nachricht soll \[Strukturierte Daten\] übertragen können, um eine Maschinenlesbarkeit zu ermöglichen. | ANF_FUN_NAC_183 |  _Link folgt_ |
| Nachricht | \[Das System\] ermöglicht eine bidirektionale Kommunikation, die die \[öffentliche Stelle\] durch Einstellung zulassen/untersagen kann. | ANF_FUN_NAC_185 |  _Link folgt_ |
| Anhänge | \[Nachrichten\] können elektronische Dokumente als \[Anhänge\] enthalten. | ANF_FUN_NAC_118 |  _Link folgt_ |
| Anhänge | \[Das System\] unterstützt beliebige Dateiformate für \[Anhänge\], per Konfiguration können jedoch Restriktionen vorgenommen werden (Allowlist, z.B. per Mime Type-Filter. Beispiele: PDF, CSV, XML, JSON, PNG, TXT, ICS, ICAL). | ANF_FUN_NAC_189 |  _Link folgt_ |
| Anhänge | Die Art einer übermittelten \[Nachricht\] (z.B. \[Bescheid\], formlose Rückfrage, etc.) kann in maschinenlesbarer Form annotiert werden, um eine optimierte Darstellung in Clients zu ermöglichen (z.B. Übersicht aller erhaltenen \[Bescheide\]). | ANF_FUN_NAC_195 |  _Link folgt_ |
| Textformat | \[Nachrichtentext\] kann fett, kursiv und unterstrichen, als Überschriften und Aufzählungen formatiert werden. | ANF_FUN_NAC_119 |  _Link folgt_ |
| Textformat | \[Nachrichtentext\] kann Tabellen enthalten. | ANF_FUN_NAC_120 |  _Link folgt_ |
| Textformat | \[Nachrichtentext\] kann Links enthalten, um z.B. auf \[Onlinedienste\] oder Widerspruchsdienste verweisen zu können. | ANF_FUN_NAC_191 |  _Link folgt_ |
| Textformat | \[Nachrichten\] können Medien enthalten (z.B. QR-Codes, Logos). | ANF_FUN_NAC_121 |  _Link folgt_ |
| Fachliche Metadaten | \[Nachrichten\] können mit beliebigen, elektronisch auswertbaren fachlichen Metadaten versehen werden. | ANF_FUN_NAC_124 |  _Link folgt_ |
| Fachliche Metadaten | \[Nachrichten\] können mit einem Verfahrensreferenz (Aktenzeichen) versehen werden. | ANF_FUN_NAC_122 |  _Link folgt_ |
| Fachliche Metadaten | \[Nachrichten\] können mit einer Antragsreferenz versehen werden (Antrags-ID), um antragsbezogene Kommunikation zu ermöglichen. | ANF_FUN_NAC_123 |  _Link folgt_ |
| Fachliche Metadaten | In \[Nachrichten\] hinterlegte \[Referenzen\] und Metadaten liegen in strukturierter Form vor und sind maschinell auslesbar. | ANF_FUN_NAC_193 |  _Link folgt_ |
| Fachliche Metadaten | Als \[Fachliche Metadaten\] der \[Nachricht\] kann der \[Status der Nachricht\] geführt werden, sichtbar oder abfragbar für die \[Empfänger:in\] und ggf. die \[Absender:in\]. | ANF_FUN_NAC_186 |  _Link folgt_ |
| Fachliche Metadaten | \[Das System\] ermöglicht die Übermittlung von (teil-)strukturierten Statusinformationen zu einem Verwaltungsverfahren (\[Status der Antragsbearbeitung\] sowie \[Status der Antragsbearbeitung Informationen\]), welche sichtbar oder abfragbar für die \[Empfänger:in\] und ggf. die \[Absender:in\] sind. | ANF_FUN_NAC_187 |  _Link folgt_ |
| Fachliche Metadaten | \[Nachrichten\] können von der \[Absender:in\] mit vordefinierten Antwortoptionen versehen werden, die der \[Empfänger:in\] beim Lesen der \[Nachricht\] angeboten werden. | ANF_FUN_NAC_125 |  _Link folgt_ |


## Thema: Identifikation

| Subthema | Konsolidierte Anforderung | Anforderungs-ID | Feedback |
| --- | --- | --- | --- |
| Identifikation | Die Rolle einer \[Absender:in\] (\[öffentliche Stelle\], \[private Organisation\], \[Privatperson\], etc.) muss an einer \[Nachricht\] erkennbar sein. | ANF_FUN_IDE_001 |  _Link folgt_ |
| Identifikation | \[Das System\] stellt der \[Empfänger:in\] einer \[Nachricht\] die Information bereit, ob diese von einer natürlichen Person (z.B. einer Sachbearbeitung) oder automatisiert (z.B. \[Fachverfahren\], KI Chatbot) verfasst wurde. | ANF_FUN_IDE_047 |  _Link folgt_ |
| Identity Provider | Zur \[Identifikation\] von \[Privatpersonen\] werden mindestens die in § 3 OZG genannten \[Identifizierungsmittel\] unterstützt. Die \[Identifikation\] muss mindestens über das Bürgerkonto gemäß § 3 OZG (\[Bund-ID\] / \[DeutschlandID\]) erfolgen können. | ANF_FUN_IDE_023 |  _Link folgt_ |
| Identity Provider | Zur \[Identifikation\] von \[privaten Organisationen\] werden mindestens die in § 3 OZG genannten \[Identifizierungsmittel\] unterstützt. Die \[Identifikation\] muss mindestens über das einheitliche Organisationskonto gemäß § 3 OZG erfolgen können. | ANF_FUN_IDE_024 |  _Link folgt_ |
| Identity Provider | Die \[Identifikation\] über \[EUDI-Wallet\] mit starker kryptografischer Bindung muss perspektivisch möglich sein, sofern die organisatorischen und technischen Voraussetzungen hierzu geschaffen sind (vgl. eIDAS-Credential mit Anonymous Credentials, Zero Knowledge Proofs, Deniability, selbstverwaltete kryptografische Identität). | ANF_FUN_IDE_025 |  _Link folgt_ |
| Identity Provider | Zur \[Identifikation\] von \[öffentlichen Stellen\] werden mindestens die in § 3 OZG genannten \[Identifizierungsmittel\] unterstützt. Die \[Identifikation\] muss mindestens über das einheitliche Organisationskonto gemäß § 3 OZG erfolgen können. | ANF_FUN_IDE_050 |  _Link folgt_ |
| Authentisierungsmittel | \[Das System\] sollte in der Lage sein, zukünftig ggf. auch weitere vom BSI festgelegte \[Authentisierungsmittel\] zu unterstützen, wenn der initiale \[Nachweis\] der \[Identität\] natürlicher Personen maximal auf Vertrauensniveau "substantiell" erbracht wurde. (vgl. § 12 Abs. 5 OZG) | ANF_FUN_IDE_034 |  _Link folgt_ |
| Authentisierungsmittel | \[Das System\] unterstützt die Integration bestehender europäischer Identitätslösungen (\[Identitäten\] nach eIDAS 1.0). | ANF_FUN_IDE_044 |  _Link folgt_ |
| Authentifizierungs- und Autorisierungsprotokolle | \[Das System\] verwendet offene Standardprotokolle zur \[Authentifizierung\] und \[Autorisierung\] der \[Nutzer:innen\]. | ANF_FUN_IDE_027 |  _Link folgt_ |

## Thema: Adressierung

| Subthema | Konsolidierte Anforderung | Anforderungs-ID | Feedback |
| --- | --- | --- | --- |
| Adressierungsansatz | \[Öffentliche Stellen\] können zur Adressierung \[private Organisationen\] (bzw. deren jeweilige Organisationseinheiten) aus einem \[Adressbuch\] auswählen. | ANF_FUN_ADR_065 |  _Link folgt_ |
| Adressierungsansatz | \[Öffentliche Stellen\] können zur Adressierung andere \[öffentliche Stellen\] (bzw. deren jeweilige Organisationseinheiten) aus einem \[Adressbuch\] auswählen. | ANF_FUN_ADR_059 |  _Link folgt_ |
| Adressierungsansatz | In einer zukünftigen Ausbaustufe können \[Privatpersonen\] Servicecenter (z.B. 115) aus einem Verzeichnis auswählen und leichtgewichtig per Chat über \[das System\] kontaktieren. | ANF_FUN_ADR_060 |  _Link folgt_ |
| Adressierungsansatz | \[Öffentliche Stellen\] können \[Nachrichten\] an eine \[Privatperson\] übermitteln, sofern diese zuvor durch Inanspruchnahme einer elektronischen Verwaltungsleistung (vgl. § 9 Abs. 1 S. 2 OZG) oder Einwilligung einen Identifier für ihr \[Postfach\] an die \[öffentliche Stelle\] übermittelt hat. | ANF_FUN_ADR_020 |  _Link folgt_ |
| Adressierungsansatz | \[Nutzer:innen\] können Nachrichten initiativ an \[Nutzer:innen\] übermitteln. (i.S.v. "Neue Nachricht"-Button) | ANF_FUN_ADR_021 |  _Link folgt_ |
| Adressierungsansatz | \[Das System\] nutzt keine global eindeutigen Identifikatoren, die sich direkt oder indirekt auf \[natürliche Personen\] beziehen. | ANF_FUN_ADR_024 |  _Link folgt_ |
| Adressierungsansatz | \[Privatpersonen\] können nur nach vorheriger Einwilligung über \[das System\] kontaktiert werden; ein Widerruf der Einwilligung ist jederzeit möglich. | ANF_FUN_ADR_039 |  _Link folgt_ |
| Adressierungsansatz | \[Öffentliche Stellen\] können festlegen, ob Antworten auf gesendete \[Nachrichten\] möglich sind oder nicht. | ANF_FUN_ADR_058 |  _Link folgt_ |
| Adressierungsansatz | \[Private Organisationen\] und \[öffentliche Stellen\] können nach vorheriger Registrierung ohne erforderliche Einwilligung über \[das System\] kontaktiert werden. | ANF_FUN_ADR_061 |  _Link folgt_ |
| Funktionspostfach | \[Nachrichten\], die an \[öffentliche Stellen\] und \[private Organisationen\] versendet wurden, können von ein oder mehreren natürlichen Personen in der Rolle \[Postfachmitglied\] anhand von vorher durch \[Org-Admins\] definierten Zugriffsberechtigungen auf das jeweilige \[Postfach\] eingesehen werden ("\[Funktionspostfach\]-Funktionalität"). | ANF_FUN_ADR_026 |  _Link folgt_ |
| Funktionspostfach | \[Org-Admins\]\[Öffentlicher Stellen\] und \[private Organisationen\] können einzelnen natürlichen Personen Zugriff auf \[Postfächer\] erteilen und auch wieder entziehen. | ANF_FUN_ADR_062 |  _Link folgt_ |
| Funktionspostfach | Bei Erteilung von Zugriffen auf \[Postfächer\] von \[öffentlichen Stellen\] und \[privaten Organisationen\] ist die gesamte Kommunikationshistorie für die neu berechtigten \[Nutzer:innen\] einsehbar. | ANF_FUN_ADR_063 |  _Link folgt_ |
| Funktionspostfach | \[Org-Admins\] können Zugriffe auf \[Postfächer\] bei Bedarf auf Basis einer gruppenbasierten Rechteverwaltung managen (z.B. Zuweisung einer Berechtigung an eine zuvor definierte Gruppe von Personen). | ANF_FUN_ADR_071 |  _Link folgt_ |
| Funktionspostfach | Von \[Org-Admins\] durchgeführte administrative Änderungen werden revisionssicher protokolliert. | ANF_FUN_ADR_070 |  _Link folgt_ |
| Stellvertretung | \[Das System\] muss Vertretungsregelungen und Vollmachten unterstützen. | ANF_FUN_ADR_025 |  _Link folgt_ |


## Thema: Transport

| Subthema | Konsolidierte Anforderung | Anforderungs-ID | Feedback |
| --- | --- | --- | --- |
| Transport | \[Das System\] muss von \[Nutzer:innen\] über das Internet und aus Behördennetzen (insb. NdB, NdB-VN) ohne unsachgemäße Schwächung von Sicherheitsmaßnahmen erreichbar sein. | ANF_FUN_TRA_006 |  _Link folgt_ |
| Routing | \[Das System\] transportiert \[Nachrichten\] über das Internet. | ANF_FUN_TRA_017 |  _Link folgt_ |
| Durchgesetzte Complianceregeln | \[Das System\] erlaubt/unterbindet den Versand von \[Nachrichten\] anhand von konfigurierbaren, rollenbasierten Rechten, z.B. Unterbindung der C2C-Kommunikation. | ANF_FUN_TRA_007 |  _Link folgt_ |


## Thema: Zustellung

| Subthema | Konsolidierte Anforderung | Anforderungs-ID | Feedback |
| --- | --- | --- | --- |
| Zustellung | \[Das System\] unterstützt sowohl die rechtssichere Zustellung/Bekanntgabe von \[Verwaltungsakten\] oder \[Entscheidungen(juristisch)\] (z.B. unter Verwendung von \[qualifizierten elektronischen Signaturen\]/\[qualifizierten elektronischen Siegeln\]/Zeitstempeln), als auch Kommunikation ohne solche Anforderungen hinsichtlich der Beweisbarkeit der erfolgten Übermittlung und \[Authentizität\] gegenüber Dritten. | ANF_FUN_ZUS_044 |  _Link folgt_ |
| Empfangsbenachrichtigung | Die \[Empfänger:in\] einer \[Nachricht\] kann sich über dritte Wege (\[Eingangsbenachrichtigung Empfänger:in\]) informieren lassen, wenn eine neue \[Nachricht\] in ihrem \[Postfach\] eingegangen ist (E-Mail oder mobiler Push-Notification (App-basiert)). | ANF_FUN_ZUS_026 |  _Link folgt_ |
| Nichtabstreitbarkeit | \[Das System\] protokolliert den (rechtlich relevanten) Zeitpunkt der Übermittlung einer \[Nachricht\] und macht diesen den \[Absender:innen\] und den \[Empfänger:innen\] bekannt. | ANF_FUN_ZUS_020 |  _Link folgt_ |
| Nichtabstreitbarkeit | \[Das System\] ermöglicht bei Bedarf eine rechtssichere und zuverlässige Zustellung von \[Nachrichten\] an \[öffentliche Stellen\], die von keinem Kommunikationspartner abgestritten werden kann. | SIC_NAC_NIC_004 |  _Link folgt_ |
| Lesebestätigung | \[Das System\] übermittelt \[Absender:in\] und \[Empfänger:in\] automatisch eine \[Bereitstellungsbestätigung\] im Falle eines erfolgreichen Versands einer \[Nachricht\] an eine \[Empfänger:in\]. | ANF_FUN_ZUS_022 |  _Link folgt_ |
| Lesebestätigung | \[Das System\] übermittelt \[Absender:in\] und \[Empfänger:in\] automatisch eine \[Bereitstellungsbestätigung\] im Falle eines erfolgreichen Versands einer \[Nachricht\] an eine \[Empfänger:in\]. | ANF_FUN_ZUS_046 |  _Link folgt_ |
| Zustellfiktion | \[Das System\] erfüllt die gesetzlich definierten Anforderungen zur rechtsicheren Realisierung einer \[Bekanntgabe-\] bzw. \[Zustellfiktion\] bei der elektronischen Übermittlung von \[Verwaltungsakten\]. (vgl. § 122 AO, § 5 VwZG, § 9 OZG, § 41 VwVerfG) | ANF_FUN_ZUS_024 |  _Link folgt_ |
| Zustellgarantie | \[Das System\] stellt sicher, dass \[Nachrichten\] auch bei gescheiterten Übermittlungsversuchen nicht verloren gehen. | ANF_FUN_ZUS_023 |  _Link folgt_ |

## Thema: Speicherung

| Subthema | Konsolidierte Anforderung | Anforderungs-ID | Feedback |
| --- | --- | --- | --- |
| Speicherung von Nachrichten | \[Das System\] ermöglicht das Speichern ein- und ausgehender \[Nachrichten\]. | ANF_FUN_SPE_038 |  _Link folgt_ |
| Speicherung von Nachrichten | \[Das System\] ermöglicht das Speichern eines Entwurfs einer \[Nachricht\]. | ANF_FUN_SPE_042 |  _Link folgt_ |
| Speicherdauer | \[Das System\] speichert \[Nachrichten\] zuverlässig und dauerhaft, sofern \[Nutzer:innen\] keine Löschung der Daten angestoßen haben oder Löscherfordernisse gemäß gesetzlicher Löschfristen bestehen. | ANF_FUN_SPE_031 |  _Link folgt_ |
| Speicherdauer | \[Das System\] unterstützt \[öffentliche Stellen\] bei der Umsetzung von gesetzlichen Löschfristen oder anderen gesetzlichen Löscherfordernissen (bspw. Zeugenschutzprogramme, Rechte gemäß DSGVO) durch eine Funktionalität zur automatischen und/oder manuellen Löschung. | ANF_FUN_SPE_012 |  _Link folgt_ |


## Thema: Anbindung

| Subthema | Konsolidierte Anforderung | Anforderungs-ID | Feedback |
| --- | --- | --- | --- |
| Anbindung | Für alle Bausteine der Gesamtinfrastruktur steht mindestens eine unter freier Softwarelizenz lizenzierte Referenzimplementierung bereit, die nach hohen Sicherheits-, Usability- und Barrierefreiheits-Standards entwickelt wurden. | ANF_ARC_ANB_019 |  _Link folgt_ |
| Schnittstellen | Die Funktionalitäten des Systems stehen den \[Nutzer:innen\] ohne Funktionseinschränkungen über Schnittstellen zur Verfügung (Umsetzung des API-First Ansatz). | ANF_ARC_ANB_006 |  _Link folgt_ |
| Schnittstellen | Die Anbindung an \[das System\] muss so einfach sein, dass eine große Zahl an \[Nutzer:innen\] aufwandsarm an \[das System\] angebunden werden kann. | ANF_ARC_ANB_008 |  _Link folgt_ |
| Schnittstellen | Alle Schnittstellen \[des Systems\] basieren auf offenen Standards. | ANF_ARC_ANB_007 |  _Link folgt_ |
| Schnittstellen | Alle Schnittstellen \[des Systems\] sind ohne die Notwendigkeit des Einsatzes von proprietären Komponenten ansprechbar. | ANF_ARC_ANB_021 |  _Link folgt_ |
| API-Dokumentation | Alle Schnittstellen \[des Systems\] sind strukturiert, vollständig und öffentlich einsehbar dokumentiert. | ANF_ARC_ANB_009 |  _Link folgt_ |
| Software Development Kit | \[Das System\] unterstützt die einfache Anbindung durch Bereitstellung von Software Development Kits (SDKs). | ANF_ARC_ANB_018 |  _Link folgt_ |


## Thema: Sicherheit

| Subthema | Konsolidierte Anforderung | Anforderungs-ID | Feedback |
| --- | --- | --- | --- |
| Sicherheit | \[Das System\] setzt den Security by Design Ansatz um. | SIC_ÜBE_ÜBE_015 |  _Link folgt_ |
| Sicherheit | \[Das System\] setzt den Zero Trust Ansatz um. | SIC_ÜBE_ÜBE_018 |  _Link folgt_ |
| Sicherheit | Die \[Integrität\] und Ende-zu-Ende-Authentizität der übermittelten \[Nachrichten\] wird auf kryptographischer Ebene unter Wahrung glaubwürdiger Abstreitbarkeit (plausible deniability) realisiert, um im Falle von Kompromittierungen keine Echtheit einzelner übermittelter Informationen belegen zu können und so Individuen vor Stigmatisierung und Diskriminierung zu schützen. | SIC_AUT_ÜBE_014 |  _Link folgt_ |
| Sicherheit | \[Das System\] unterstützt eine Wiederherstellung des Zugangs nach Geräteverlust oder Verlust von Authentisierungsfaktoren (Passwörtern). | SIC_VER_ÜBE_058 |  _Link folgt_ |
| Schutzbedarf | Die technische Realisierung des Systems berücksichtigt mindestens die Anforderungen des BSI-Grundschutz auf dem Schutzbedarf "hoch". | SIC_ÜBE_ÜBE_005 |  _Link folgt_ |
| Schutzbedarf | \[Das System\] implementiert angemessene technische und organisatorische Maßnahmen zum Schutz personenbezogener Daten. (Art. 31 Abs. 1 DSGVO) | DAT_ÜBE_ÜBE_022 |  _Link folgt_ |
| Vertrauensniveau | \[Nutzer:innen\] können bis zum Vertrauensniveau "hoch" (gemäß BSI TR-03107-1) authentifiziert werden. | SIC_ÜBE_ÜBE_006 |  _Link folgt_ |
| Vertrauensniveau | \[Das System\] erlaubt Zugriff auf \[Nachrichten\] durch die \[Empfänger:in\] nur, wenn die \[Empfänger:in\] mit dem von der \[Absender:in\] für diese \[Nachricht\] definierten Vertrauensniveau authentifiziert wurde. | SIC_ÜBE_ÜBE_007 |  _Link folgt_ |
| Vertrauensniveau | Die \[Empfänger:in\] einer \[Nachricht\] kann nachvollziehen, auf welchem Vertrauensniveau die \[Absender:in\] authentifiziert wurde. | SIC_ÜBE_ÜBE_008 |  _Link folgt_ |
| Vertrauensniveau | Nachrichtenkommunikation kann auf unterschiedlichen Vertrauensniveaus erfolgen wofür entsprechend zugelassene \[Authentisierungsmittel\] zu verwenden sind. | SIC_VER_ÜBE_048 |  _Link folgt_ |
| Authentizität | \[Das System\] stellt die Authentizität einer \[Nachricht\] auf dem Vertrauensniveau sicher, auf dem die \[Absender:in\] authentifiziert wurde. | SIC_AUT_ÜBE_007 |  _Link folgt_ |
| Authentizität | \[Nutzer:innen\] können beim Senden einer \[Nachricht\] die \[Empfänger:in\] und beim Empfangen einer \[Nachricht\] die \[Absender:in\] klar, menschenlesbar und eindeutig identifizieren. | SIC_AUT_ÜBE_008 |  _Link folgt_ |
| Authentizität | \[Das System\] ermöglicht die \[qualifizierte elektronische Signatur\] oder \[qualifiziertes elektronisches Siegeln\] von \[Nachrichten\]. | SIC_AUT_ÜBE_013 |  _Link folgt_ |
| Integrität | \[Das System\] stellt die \[Integrität\] der übermittelten \[Nachrichten\] sicher. | SIC_INT_ÜBE_003 |  _Link folgt_ |
| Verschlüsselung bei Übermittlung | \[Das System\] stellt eine Ende-zu-Ende-Verschlüsselung der übermittelten \[Nachrichten\] (inklusive \[Anhängen\] und \[Vertraulichen Metadaten\]) sicher. | SIC_VER_ÜBE_026 |  _Link folgt_ |
| Verschlüsselung bei Übermittlung | Das eingesetzte Verschlüsselungsverfahren unterstützt moderne kryptographische Eigenschaften wie Perfect Forward Secrecy (PFS), Post-Compromise Security (PCS, auch: Future Secrecy) und Plausible Deniability. | SIC_VER_ÜBE_029 |  _Link folgt_ |
| Verschlüsselung bei Übermittlung | Die Verschlüsselung übermittelter und gespeicherter \[Nachrichten\] kann weder von der Betreiberin des Systems noch von Dritten gebrochen, umgangen oder entschlüsselt werden. | SIC_VER_ÜBE_032 |  _Link folgt_ |
| Verschlüsselung bei Übermittlung | \[Das System\] verwendet - zusätzlich zur Inhaltsdatenverschlüsselung (E2EE) - flächendeckende Transportverschlüsselung. | SIC_VER_ÜBE_055 |  _Link folgt_ |
| Nachrichtenverschlüsselung | \[Nachrichten\] werden ausschließlich in verschlüsselter Form gespeichert. | SIC_VER_ÜBE_027 |  _Link folgt_ |
| Nachrichtenverschlüsselung | Das für die E2E-Verschlüsselung genutzte private Schlüsselmaterial darf nur der \[Absender:in\] bzw. der \[Empfänger:in\] zugänglich sein (kein Zugriff durch Dritte Parteien). | SIC_VER_ÜBE_028 |  _Link folgt_ |
| Nachrichtenverschlüsselung | \[Das System\] setzt Crypto-Agilität um, um beim Bekanntwerden von Schwächen in eingesetzten Verschlüsselungsverfahren auf neue \[Verfahren\] wechseln zu können und um in der Zukunft Post-Quantum-Kryptographie zu realisieren. | SIC_VER_ÜBE_031 |  _Link folgt_ |
| Nachrichtenverschlüsselung | Alle Metadaten, die nicht zwingend für die korrekte Zustellung einer \[Nachricht\] erforderlich sind, werden vermieden, als Teil der übermittelten \[Nachricht\] als \[vertrauliche Metadaten\] verschlüsselt oder durch Privatsphäre-schützende Technologien (Privacy-enhancing Technologies, PET) verschleiert. | SIC_VER_ÜBE_051 |  _Link folgt_ |
| Malware Prüfung | \[Das System\] unterstützt die \[Nutzer:innen\] bei der Realisierung einer Prüfung von \[Nachrichten\] auf Schadinhalte nach dem Empfang von \[Nachrichten\] (z.B. durch Unterstützung in SDKs). | SIC_ÜBE_MAL_005 |  _Link folgt_ |
| Vertraulichkeit | Um eine rechtswirksame Zustellung zu gewährleisten und die Vertraulichkeit bei der Zustellung zu wahren, dürfen \[Nachrichten\] nur die richtige \[Empfänger:in\] erreichen. | SIC_VER_ÜBE_056 |  _Link folgt_ |
| Nachweisbarkeit | Von \[Org-Admins\] durchgeführte Änderungen werden revisionssicher protokolliert. | SIC_NAC_NAC_005 |  _Link folgt_ |
| Datenschutz | \[Das System\] setzt den Privacy by Design Ansatz um. | DAT_ÜBE_ÜBE_021 |  _Link folgt_ |
| Datenschutz | Es wird gewährleistet, dass eingesetzte IT-Systeme im Störungsfall wiederhergestellt werden können (Wiederherstellbarkeit). (§64 Abs. 3 Nr. 9 BDSG) | ZUV_WIE_ÜBE_003 |  _Link folgt_ |
| Datenschutz | \[Das System\] stellt sicher, dass sensible Informationen, wie z.B. Namen und Inhalte einer \[Nachricht\] nicht in für Dritte zugänglicher Form (unverschlüsselt) über Push Server (von Google und Apple) übertragen werden. | SIC_ÜBE_ÜBE_017 |  _Link folgt_ |
| Datenschutz | \[Das System\] unterstützt \[Privatpersonen\] bei der aufwandsarmen und zeitnahen Realisierung von Auskunftsbegehren durch technische Maßnahmen. (Art. 15 DSGVO) | DAT_ÜBE_ÜBE_020 |  _Link folgt_ |

## Thema: Architektur

| Subthema | Konsolidierte Anforderung | Anforderungs-ID | Feedback |
| --- | --- | --- | --- |
| Architektur | Zur Evaluierung und Erfolgsmessung stellt \[das System\] anonymisierte Daten zu erfolgten Anbindungen, Anzahl der Nutzenden, Anzahl der übermittelten \[Nachrichten\], Anzahl und Größe von \[Anhängen\], etc. als Open Data über das Verwaltungsdatenportal GovData bereit. | ANF_ARC_ARC_048 |  _Link folgt_ |
| Architektur | \[Das System\] gibt im Fehlerfall zuverlässig verständliche und aussagekräftige Fehlermeldungen aus. | BED_ABS_ÜBE_003 |  _Link folgt_ |
| Architektur | \[Das System\] unterstützt den Zeichensatz gem. DIN 91379 vollständig. | KOM_INT_STA_003 |  _Link folgt_ |
| Architektur | Das Antwortverhalten des Systems ermöglicht eine Echtzeit-Kommunikation (vergleichbar mit den Reaktionszeiten privater oder nicht-kommerzieller Messenger-Dienste). | LEI_ZEI_ÜBE_002 |  _Link folgt_ |
| Architektur | \[Das System\] soll ohne Bindung an einen Mobilfunkvertrag nutzbar sein, d.h. eine Telefonnummer ist zur Registrierung und Nutzung \[des Systems\] nicht nötig. | RAH_ÜBE_007 |  _Link folgt_ |
| Architektur | \[Das System\] ist auf Hochverfügbarkeit (99,9XX %) ausgelegt. | ZUV_VER_ÜBE_002 |  _Link folgt_ |
| Architektur | \[Das System\] muss organisatorisch und technisch resilient ausgelegt sein, um auch in Krisenfällen einen Nachrichtenaustausch sicherstellen zu können. | ZUV_ÜBE_RES_002 |  _Link folgt_ |
| Architektur | \[Das System\] muss skalierbar sein und sich dynamisch an Lasten anpassen (Skalierbarkeit & Elastizität). | LEI_KAP_ÜBE_012 |  _Link folgt_ |
| Architektur | \[Das System\] verfügt über ein robustes Backend, das auch im Internet DDOS Angriffen standhält. | ZUV_VER_ÜBE_006 |  _Link folgt_ |
| Client-Bausteine | \[Privatpersonen\] können \[das System\] einfach und komfortabel per mobiler App oder über einen responsiven webbasierten Zugang nutzen. | ANF_ARC_ARC_016 |  _Link folgt_ |
| Betrieb | \[Das System\] ist geeignet, um in der Deutschen Verwaltungscloud (DVC) betrieben zu werden. | ANF_ARC_ARC_018 |  _Link folgt_ |
| Administration | \[Das System\] ist in der Lage, die maximale Größe von \[Nachrichten\] und \[Anhängen\] per Konfigurationsoption einzuschränken. | LEI_KAP_ÜBE_011 |  _Link folgt_ |

## Thema: Postfachzugang für Bürger:innen

| Subthema | Konsolidierte Anforderung | Anforderungs-ID | Feedback |
| --- | --- | --- | --- |
| _NA_ | \[Das System\] macht eingehende und ausgehende \[Nachrichten\] im \[Postfach\] kenntlich. | ANF_FUN_CLB_032 |  _Link folgt_ |
| _NA_ | \[Das System\] macht neu empfangene und nicht gelesene \[Nachrichten\] im \[Postfach\] kenntlich. | ANF_FUN_CLB_031 |  _Link folgt_ |
| _NA_ | \[Nutzer:innen\] müssen die Möglichkeit besitzen, \[Nachrichten\] \[im System\] zu strukturieren und zu ordnen. | ANF_FUN_CLB_013 |  _Link folgt_ |
| _NA_ | \[Das System\] ermöglicht die manuelle sowie automatisierte Sortierung und Filterung von \[Nachrichten\] in bestimmte Kategorien/Ordnern, welche individuell durch die \[Nutzer:in\] definiert werden können. | ANF_FUN_CLB_029 |  _Link folgt_ |
| _NA_ | \[Das System\] ermöglicht die Sortierung von \[Nachrichten\] nach verschiedenen Merkmalen (z. B. Betreff, Absendezeitpunkt, \[Absender:in\], \[Anhänge\], Vorgang). | ANF_FUN_CLB_033 |  _Link folgt_ |
| _NA_ | \[Das System\] ermöglicht das Durchsuchen von \[Nachrichten\] innerhalb des \[Postfachs\]. | ANF_FUN_CLB_030 |  _Link folgt_ |
| _NA_ | \[Das System\] muss in verschiedenen Sprachen, mindestens jedoch in Deutsch und Englisch, nutzbar sein. | ANF_FUN_CLB_012 |  _Link folgt_ |
| _NA_ | \[Das System\] unterstützt die lokale Speicherung, das Herunterladen und den Export von \[Nachrichten\] und \[Anhängen\] (vgl. Art. 15 DSGVO). | ANF_FUN_CLB_026 |  _Link folgt_ |
| _NA_ | \[Nutzer:innen\] können Login-Versuche auf \[das System\] überprüfen und sich aus anderen Sitzungen aus der Ferne abmelden. | ANF_FUN_CLB_034 |  _Link folgt_ |
| _NA_ | \[Nutzer:innen\] müssen sich aus \[dem System\] ausloggen können. | ANF_FUN_CLB_011 |  _Link folgt_ |

---

[^1]: Die IDs der Anforderungen dienen ausschließlich der eindeutigen Identifizierbarkeit jeder Anforderung im Rahmen der eingesetzten Anforderungsmanagement-Methodik. Sie erlauben insbesondere die Rückverfolgbarkeit einzelner Anforderungen im Anforderungsregister, welches das Projektteam auf Arbeitsebene führt. Die IDs sind fortlaufend im Arbeitsprozess entstanden und sollten dementsprechend nicht inhaltlich interpretiert werden. 

[^2]: Anforderungen sind in ein Kategorienschema aus Themen und diesen nachgelagerten Subthemen einsortiert. Ein Überblick und Erläuterungen zum Kategorienschema sind [hier](url) einsehbar. 

[^3]: In dem hier jeweils verlinkten Gitlab-Issue kann die gegebene Anforderung während der Konsultationsphase im März und April 2025 öffentlich kommentiert werden.