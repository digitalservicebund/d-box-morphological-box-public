# ZaPuK: Transitionsbetrachtung C – Mein Unternehmenskonto (Entwurf v1.1)

Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/ef99118c0607dd7fb11a005593569a36a0f84343/Zielarchitektur/Transitionen/C--Mein-Unternehmenskonto/README.md
Branch: architecture-v1.1 (Entwurf v1.1, noch nicht in main), Commit ef99118c06 (2026-11-13)
Abgerufen: 2026-11-19 über die GitLab-API

Primärquelle (FITKO / FIT-AB, Entwurfsstand). Inhalt unverändert ab der Linie unten; Bildverweise zeigen auf das Repository. Zeilenangaben in _befunde.md beziehen sich auf diese Datei.

---

# Föderale Zielarchitektur für Postfach- und Kommunikationslösungen

***Bauplan einer modernen Kommunikationsinfrastruktur für Privatpersonen, private Organisationen und öffentliche Stellen***

**Version 1.1** &ndash; **Stand: 06.10.2026** &ndash; **Anhang C: Transitionsplan Mein Unternehmenskonto**

::TODO
> Dieses Dokument ist die maschinenlesbare Textfassung des vom IT-Planungsrat im PDF-Format verabschiedeten Dokuments ["Zielarchitektur" in Version 1.0](./2025-06-26_Beschluss-2025-28_ZaPuK_Architekturkonzept.pdf). Bei der Überführung wurden redaktionelle Änderungen für eine bessere Lesbarkeit und Anpassung an die openCode-Struktur vorgenommen.

------

## Nutzungsbedingungen

Die Inhalte dieses Dokumentes unterliegen der [Creative Commons Namensnennung 4.0 International Public License (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).

------

## 1. Zweck dieses Dokuments

Dieses Dokument enthält den Plan für die Transition der beiden Postfächer des Mein Unternehmenskontos (MUK), das OZG-PLUS-Postfach (OZG-PP) und das Postfach 2.0, auf die Zielarchitektur Postfach- und Kommunikationslösungen (ZaPuK). Es beinhaltet die ZaPuK-seitige Empfehlung für das MUK-spezifische Zielbild nach Abschluss aller Transitionsarbeiten, den Ablauf der einzelnen Transitionsphasen inklusive konkreter Maßnahmen sowie die Verantwortlichkeiten für deren Umsetzung. Der Transitionsplan benennt die ungefähre Dauer der einzelnen Phasen. Der Inhalt des Transitionsplans, die konkrete Terminierung und verbindliche Festlegungen befinden sich aktuell (Stand: Oktober 2026) noch in Abstimmung zwischen dem Produktmanagement der FITKO, dem Bayrischen Staatsministerium für Digitales (StMD Bayern), dem Bayrischen Landesamt für Steuern (BayLfSt) und der Freien Hansestadt Bremen (FHB) als Betreiber:innen des MUK bzw. der MUK-Postfächer und dem Programm ZaPuK.

Es handelt sich bei der vorliegenden Fassung um ein lebendiges Dokument, welches im weiteren Verlauf der Transitionsgespräche bei Bedarf fortgeschrieben wird. Nicht Bestandteil ist hier die Transition des im Portal Mein ELSTER integrierten ELSTER-Postfachs. In diesem Dokument werden allein die Postfächer des MUK behandelt.

## 2. Transitions-Zielbild

Das zentrale Postfach-Frontend für Privatpersonen und Organisationen ersetzt gemäß ZaPuK-Architekturkonzept die bisherigen Postfach-Frontends in BundID, MJP und MUK. Damit ist keine parallele Weiterentwicklung dieser Frontends mehr erforderlich. BundID, MJP und MUK sollten nach Abschaltung der jeweiligen Postfach-UIs für einen Übergangszeitraum auf das zentrale Postfach-Frontend im Bundesportal verlinken.

Im Zielzustand bleibt das MUK der Identitätsanbieter für private Organisationen; die beiden Postfach-Backends und das Postfach-Frontend im MUK entfallen. Damit ist zukünftig keine Weiterentwicklung und insb. keine funktionale Erweiterung der beiden MUK-Postfächer mehr erforderlich.

Die Postfachfunktion wird über das zentrale Backend für private Organisationen und das zentrale Postfach-Frontend für Privatpersonen und Organisationen umgesetzt. Das Backend für private Organisationen soll gemäß ZaPuK-Architekturkonzept durch FIT-Connect betrieben werden. Die Umsetzung erlaubt eine Ende-zu-Ende-verschlüsselte Kommunikation mit Vertretungsmodell und Funktionspostfächern ([ADR-1102](/Zielarchitektur/Architekturentscheidungen/ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md), [ADR-1112](/Zielarchitektur/Architekturentscheidungen/ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md)). Das Adressregister wird föderiert in den Postfach-Backends geführt ([ADR-1104](/Zielarchitektur/Architekturentscheidungen/ADR-1104-MXID-Struktur-Domain-Governance.md), [ADR-1111](/Zielarchitektur/Architekturentscheidungen/ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)). Die Prüfung auf Viren und andere Malware wird an den E2EE-Endpunkten im Machtbereich der empfangenden Organisation umgesetzt ([ADR-1130](/Zielarchitektur/Architekturentscheidungen/ADR-1130-Malware-Prüfung.md)). Die Anbindung der Fachverfahren wird über die Matrix-basierten Schnittstellen bzw. mittels des FIT-Connect SDK mit ZaPuK-Modul umgesetzt.

Für eine Übergangszeit ist für die Transition des Postfachs 2.0 lediglich die Anzeige von Verweisen im zentralen Postfach-Frontend von ZaPuK möglich. Durch die Verweise können Nutzende in das bisherige Postfach-Frontend im MUK abspringen. Als weiterer Transitionsschritt kann das Postfach 2.0 vorübergehend über eine Postfach-2.0-Matrix-Bridge angebunden werden. Hinweis: Das im MUK verwendete Postfach 2.0 basiert technisch zwar auf dem im Portal Mein ELSTER integrierten ELSTER-Postfach, umfasst aber allein die Nachrichten für Anwendungsfälle des MUK.

Im OZG-Plus-Postfach wird auf den Zwischenschritt der Nachrichten mit Verweisen verzichtet, da eine direkte Ablösung schnell umsetzbar erscheint. Eine Bridge ist wegen der Ende-zu-Ende-Verschlüsselung auch nicht umsetzbar und ein Protokollkonverter würde unverhältnismäßig hohe Aufwände verursachen.

Die nachfolgende Abbildung visualisiert das in der Ziel-Bebauung angestrebte Zusammenspiel der Komponenten der Zielarchitektur.

![Übersicht über das Zusammenspiel der Postfach-Backends mit dem Frontend, Authentifierzungslösungen und weiteren Komponenten und Umsystemen.](images/image1.png)

> _Abbildung 1: Zusammenspiel der Komponenten der Zielarchitektur_

## 3. Transitionsphasen

Die Durchführung der Transition und Migration in der gleichnamigen Programmphase III des ZaPuK-Programms erfolgt in vier Transitionsphasen, die aufeinander aufbauen, sich jedoch zeitlich überlappen können. Für beide im MUK enthaltenen Postfachlösungen besteht ein gemeinsamer Phasenrahmen mit jeweils postfachspezifischen Teilen.

| Phase                                              | Inhalt                                                                                                                                                                                                                                                                                                                    | Vorbedingung                                                                                                                                                                                                     | Zeitlicher Rahmen                     |
| -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------- |
| **A**<br>Aufsetzen des Transitions-projektes       | Aufsetzen des Transitionsprojektes inkl. Meilensteinplanung, Allokation nötiger Finanzmittel, Durchführung ggf. erforderlicher Beauftragungen<br><br>Ziel: Transitionsprojekt ist aufgesetzt und handlungsfähig                                                                                                           | Abschluss ZaPuK-Phase II                                                                                                                                                                                         | 3-6 Monate                            |
| **B** Transitions-vorbereitung                     | Vorbereitung der Betriebsumgebung, Abschluss von Entwicklungsarbeiten, Produktivsetzung erforderlicher Backend-Komponenten & zentrales Postfach-Frontend, Qualitätssicherung, Einrichtung der Support-Prozesse, Migrationsvorbereitung<br><br>Ziel: Komponenten sind produktiv einsetzbar (Migration noch nicht erfolgt). | Erforderliche Beauftragungen abgeschlossen                                                                                                                                                                       | 6-8 Monate                            |
| **C** Migration                                    | Zustellung neuer Nachrichten über ZaPuK je Lösung, Umstellung Fachverfahren angestoßen                                                                                                                                                                                                                                    | Qualitätssicherungs-tests waren erfolgreich, zentrales Postfach-Frontend in Betrieb, Postfach-Backend für private Organisationen & Postfach-Backend für öffentliche Stellen in Betrieb, Adressregister produktiv | 1-2 Monate                            |
| **D** Abschaltung Altsysteme (Dekommis-sionierung) | Read-only-Betrieb bis Fristablauf, dann Abschaltung je Backend                                                                                                                                                                                                                                                            | Entscheidung Migration Nachrichtenbestand, Fachverfahren sind (nahezu) vollständig auf die Matrix-basierten Schnittstellen migriert                                                                              | nach Ablauf der Aufbewah-rungsfristen |

> _Tabelle 1: Übersicht der Transitionsphasen A bis D_

## 4. Detaillierte Phasenbeschreibung und Maßnahmenkatalog

Nachfolgend werden die Phasen A-D inklusive Maßnahmen, Verantwortlichkeiten, Voraussetzungen und zu erwartender Ergebnisse beschrieben. Dabei ist zu beachten, dass insbesondere die Auflistung der Maßnahmen noch nicht final ausgestaltet ist und sich durch die Transitionsgespräche Änderungen ergeben können.

### 4.1 Übergeordnete Maßnahmen (Mein Unternehmenskonto gesamt)

| Nr  | Maßnahme                                                                                                                                                                                                     | Verantwortlich                          | Voraussetzung                                                                             |                                                                                                    | Ergebnis |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------- | ----------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | -------- |
| A1  | Finanzierung und bedarfsgerechte Beauftragungen (Inhouse, Rahmenvertrag oder Vergabe) für Migrationstätigkeiten und ggf. notwendige Anpassung bestehender Support-, Datenschutz- und IT-Sicherheitskonzepte | FITKO-PM, StMD Bayern, BayLfSt, FHB     | Projektsteckbrief liegt vor                                                              | erfolgte Beauftragungen                                                                            |          |
| A2  | Mengengerüste ermitteln: Postfächer, Funktionspostfächer, Nachrichten, Anhänge, Lastprofile, Wachstum                                                                                                        | FITKO-PM, StMD Bayern, BayLfSt, FHB     | \-                                                                                        | Mengengerüste                                                                                      |          |
| A3  | Adressregister, Onboarding Organisationen, Abbildung der Bestandsadressen                                                                                                                                    | FIT-Connect, StMD Bayern, BayLfSt, FHB  | Konzept Adressüberführung (Vorbed.)                                                      | Organisationen adressierbar                                                                        |          |
| A4  | Org-IDP-Adapter ([ADR-1108](/Zielarchitektur/Architekturentscheidungen/ADR-1108-Nachweisstellen-Org-IdP-Adapter.md), [ADR-1112](/Zielarchitektur/Architekturentscheidungen/ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md))                                                                                                                                                                         | StMD Bayern, BayLfSt, ZaPuK             | Entwurf Berechtigungs-steuerung                                                           | Organisationen können eigene IDPs integrieren                                                      |          |
| A5  | Umstellung der Postfachfunktion der Fachverfahren von ELSTER-Transfer/ERiC und OZG-PP-REST-API auf Matrix-basierte Schnittstellen                                                                            | Fachbehörden/ Fachverfahrenshersteller | FIT-Connect SDK mit ZaPuK-Modul sowie entsprechende Anbindungsdokumentation stehen bereit | Fachverfahren nutzen Matrix-basierte Schnittstelle statt ELSTER-Transfer/ERiC bzw. OZG-PP-REST-API |          |
| A6  | Kommunikationsmaßnahmen verwaltungsseitig (insb. Kommunikation Roadmap an FV-Hersteller und Fachbehörden)                                                                                                    | FITKO-PM, StMD Bayern, BayLfSt, ZaPuK   |                                                                                           | Fachbehörden und IT-Dienstleister sind über Zeitpläne informiert                                   |          |
| A7  | Etablierung eines produktübergreifenden Wissensmanagements in den Support-Teams, ggf. Anpassung von Supportprozessen                                                                                         | StMD Bayern, BayLfSt, FHB, ZaPuK        |                                                                                           | Supportstrukturen sind gleichermaßen auskunftsfähig bei Fragen zur Postfach-Funktionalität        |          |
| A8  | Ertüchtigung Org-Identität, konforme Nachweisstelle (Org-Identität, Vertrauensniveau etc.), entsprechend ADR, PKI-Anbindung ([ADR-1106](/Zielarchitektur/Architekturentscheidungen/ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md), [ADR-1107](/Zielarchitektur/Architekturentscheidungen/ADR-1107-Nachweisformat-Identitaetsnachweis-Matrix-Nachrichten.md), [ADR-1108](/Zielarchitektur/Architekturentscheidungen/ADR-1108-Nachweisstellen-Org-IdP-Adapter.md))                                                   | StMD Bayern, BayLfSt, ZaPuK             |                                                                                           | ZaPuK-konforme Identitätsnachweise für Organisationen                                             |          |

> _Tabelle 2: Übergeordnete Maßnahmen für das MUK_

### 4.2 Teil 1: OZG-PLUS-Postfach

Es ist keine serverseitige Migration der Nachrichten möglich, da diese clientseitig verschlüsselt vorliegen (E2EE). Die Postfächer müssen neu angelegt und die zugehörigen Schlüssel erstellt und verteilt werden. Die nutzenden Organisationen müssen mitwirken, um die Rechte bzw. Gruppen der Postfächer entsprechend im neuen Postfach anzulegen. Die PRE-Schlüsselbeziehungen werden nicht migriert. Der Nachrichtenbestand wird nicht migriert.

#### 4.2.1 Phase B

| Nr   | Maßnahme                                                                                                            | Verantwortlich   | Voraussetzung  |                                                | Ergebnis |
| ---- | ------------------------------------------------------------------------------------------------------------------- | ---------------- | -------------- | ---------------------------------------------- | -------- |
| O-B1 | Abbildung Funktionspostfächer, Mitgliedschaften, Vertretungen ([ADR-1102](/Zielarchitektur/Architekturentscheidungen/ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md), [ADR-1112](/Zielarchitektur/Architekturentscheidungen/ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md))                                  | ZaPuK, FHB       |                | abgestimmtes Konzept Abbildung OZG-PP zu ZaPuK |          |
| O-B2 | Aufbau Postfach-Backend für private Organisationen und Postfach-Backend für öffentliche Stellen (identisch zu P-B3) | FIT-Connect      |                | Betriebs-bereites Zielsystem                   |          |
| O-B3 | Pilot mit ausgewählten Organisationen und Onlinediensten                                                            | FIT-Connect, FHB | A8, O-B1, O-B2 | Pilotergebnisse, Referenz für weitere Systeme |          |

> _Tabelle 3: Maßnahmen in Phase B für das OZG-PLUS-Postfach_

#### 4.2.2 Phase C

| Nr   | Maßnahme                                                                                                               | Verantwortlich                                 | Voraussetzung          | Ergebnis                   |
| ---- | ---------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------- | ---------------------- | -------------------------- |
| O-C1 | Umschaltung Zustellung auf ZaPuK, OZG-PP erhält keine neuen Nachrichten mehr                                           | Fachverfahren                                  | O-B2, O-C2, A3, A5, A8 | produktive Zustellung      |
| O-C2 | Konfiguration Funktionspostfächer und Berechtigungen durch die Organisationen                                          | Organisationen, Org-Admins                     | O-B1, A4               | Funktionspostfächer aktiv |
| O-C3 | Abkündigung OZG-PP-Webanwendung in Vorbereitung auf Dekommissionierung, Weiterleitung auf zentrales Postfach-Frontend | FITKO-PM, FHB in Abstimmung mit ZaPuK-Programm | O-C1                   | Zugang nur noch über ZaPuK |

> _Tabelle 4: Maßnahmen in Phase C für das OZG-PLUS-Postfach_

#### 4.2.3 Phase D

| Nr   | Maßnahme                                                                                                                      | Verantwortlich | Voraussetzung                                 | Ergebnis                  |
| ---- | ----------------------------------------------------------------------------------------------------------------------------- | -------------- | --------------------------------------------- | ------------------------- |
| O-D1 | Temporärer Read-only-Betrieb des OZG-PP-Backends bis Ende der Aufbewahrungsfristen, clientseitiger Export für Organisationen | FHB            | Entscheidung zu Nachrichtenbestand (Vorbed.) | befristeter Archivbetrieb |
| O-D2 | Abschaltung OZG-PP-Backend und REST-API                                                                                       | FITKO-PM, FHB  | O-D1, A5                                      | Stilllegung               |

> _Tabelle 5: Maßnahmen in Phase D für das OZG-PLUS-Postfach_

### 4.3 Teil 2: Postfach 2.0

Der Zugriff auf entschlüsselte Nachrichten auf dem Server entfällt hier im Zielbild (Auswirkung auf Malware-Prüfung). Für die Migration sind Übergangsschritte möglich. Zunächst wird eine Verlinkung im Frontend dargestellt und es werden Metadaten zu den im Postfach 2.0 eingehenden Nachrichten an die ZaPuK-Postfach-Backends übermittelt. Auch die Anbindung einer Bridge innerhalb der ELSTER-Domäne ist möglich, da die Nachrichten beim Eingang im Postfach 2.0 temporär unverschlüsselt im Server vorliegen.

#### 4.3.1 Phase B

| Nr   | Maßnahme                                                                                                            | Verantwortlich                    |                                | Voraussetzung | Ergebnis                                                                                                                                                |
| ---- | ------------------------------------------------------------------------------------------------------------------- | --------------------------------- | ------------------------------ | ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P-B1 | Transitionsschritt 1: Anzeige Nachrichteneingang mit Verlinkung                                                     | StMD Bayern, BayLfSt              |                                |               | Nachrichten im zentralen Postfach-Frontend sichtbar, Nachricht im Bestandssystem verlinkt                                                               |
| P-B2 | Transitionsschritt 2: Entscheidung und ggf. Implementierung Postfach-2.0-Matrix-Bridge                              | StMD Bayern, BayLfSt, FIT-Connect | P-B1, Bewertung Aufwand/Nutzen |               | Nachrichten werden vor Verschlüsselung im Postfach-2.0-Backend an ZaPuK-Postfach-Backend für öffentliche Stellen oder private Organisationen umgeleitet |
| P-B3 | Aufbau Postfach-Backend für private Organisationen und Postfach-Backend für öffentliche Stellen (identisch zu O-B2) | FIT-Connect                       |                                |               | betriebsbereites Zielsystem                                                                                                                             |

> _Tabelle 6: Maßnahmen in Phase B für das Postfach 2.0_

#### 4.3.2 Phase C

| Nr   | Maßnahme                                           | Verantwortlich | Voraussetzung    | Ergebnis              |
| ---- | -------------------------------------------------- | -------------- | ---------------- | --------------------- |
| P-C1 | Umschaltung Zustellung neuer Nachrichten auf ZaPuK | Fachverfahren  | P-B3, A3, A5, A8 | produktive Zustellung |

> _Tabelle 7: Maßnahmen in Phase C für das Postfach 2.0_

#### 4.3.3 Phase D

| Nr   | Maßnahme                                                                                                                                                           | Verantwortlich                 | Voraussetzung                               | Ergebnis                  |
| ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------ | ------------------------------------------- | ------------------------- |
| P-D1 | Temporärer Read-only-Betrieb des Postfach-2.0-Backends für Aufbewahrungs-dauer (Bescheid 4 Jahre, Mitteilung 180 Tage)                                             | StMD Bayern, BayLfSt           | Entscheidung Bestandsnachrichten (Vorbed.) | befristeter Archivbetrieb |
| P-D2 | Abschaltung Postfach-2.0-Backend, Rückbau Übergangsschritte                                                                                                        | FITKO-PM, StMD Bayern, BayLfSt | P-D1, A5                                    | Stilllegung               |
| P-D3 | Gemeinsamer Abschluss: Postfach-Frontend im MUK und Postfach-2.0-Backend entfallen, MUK bleibt weiterhin Org-Konto (Authentifizierung und Berechtigungssteuerung) | FITKO-PM, StMD Bayern, BayLfSt | O-D2, P-D2                                  | Verbund im Zielzustand    |

> _Tabelle 8: Maßnahmen in Phase D für das Postfach 2.0_

### 4.4 Offene Punkte

1. Mengengerüst des MUK: Anzahl Postfächer und Funktionspostfächer, Anzahl angebundener Systeme, Anzahl der täglich/monatlich versendeten Nachrichten, durchschnittliche und maximale Größe übermittelter Nachrichten, Gesamtzahl und Gesamtgröße der derzeit vorgehaltenen Nachrichten und Anhänge, Gesamtgröße Datenbank Produktivsystem.
2. Klärung widersprüchliche Angaben zu Anhangsgrößen bei OZG-PP (50 MB vs. 600 MB) auflösen
3. Betreiber der Nachweisstelle festlegen ([ADR-0016](/Zielarchitektur/Architekturentscheidungen/ADR-0016-Authentizität-Identität-privater-Organisationen.md))
