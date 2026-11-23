<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Zielarchitektur/Architekturentscheidungen/ADR-1135-Veraktung-und-Aufbewahrung.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-1135-Veraktung-und-Aufbewahrung.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob bac081233c); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# ADR-1135: Veraktung und Aufbewahrung Ende-zu-Ende-verschlüsselter Nachrichten

> **Status:** ✔️ Operativ entschieden | **Datum:** 11. November 2026

## Kontext und Problembeschreibung

Die Kommunikation der Zielarchitektur muss von Teilnehmern in externen Systemen abgelegt werden können. [Behörden](../../Glossar.md#behörde) nutzen zur dauerhaften Vorhaltung rechtlich relevanter Dokumente Facharchive oder E-Akte-Systeme. Auch private Unternehmen unterliegen Aufbewahrungspflichten, etwa aus dem Handels- und Steuerrecht. Dieser Export der Daten ist nur über die [E2EE-Endpunkte](../../Glossar.md#e2ee-endpunkt) möglich. Diese Endpunkte liegen je nach gewähltem Modell im Endgerät oder auf einem Server der [Organisation](../../Glossar.md#organisation) ([ADR-1100](./ADR-1100-Bilaterale-Kommunikation-Festlegung-E2EE-Endpunkte.md), [ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md), [ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)).

In dieser Architekturentscheidung werden mögliche Optionen zur Umsetzung betrachtet. Darüber hinaus wird festgelegt, welche Datenpunkte exportierte [Nachrichten](../../Glossar.md#nachricht) enthalten müssen, um den unterschiedlichen Anforderungen gerecht zu werden.

### Abgrenzung

- **Löschfristen:** Inhaltsdaten, Kommunikationsmetadaten und Verkehrsdaten erhalten jeweils eigene Fristen. Die Löschung ist nachzuweisen, die [Löschquittung](../../Glossar.md#löschquittung) ist hierfür geeignet ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md), vgl. Folge-Entscheidung).
- **Langfristige Archivierung:** Umsetzung einer langfristigen Vorhaltung hinsichtlich der Beweisbarkeit.

## Entscheidungstreiber

- **Aufbewahrungspflichten:** Rechtlich relevante Daten müssen in ein System zur Archivierung bzw. Veraktung überführt werden können.

- **E2EE-Konformität:** Die [Ende-zu-Ende-Verschlüsselung](../../Glossar.md#ende-zu-ende-verschlüsselung) der Kommunikation soll nicht gebrochen werden (SIC_VER_ÜBE_032).

- **Nachweisübernahme:** Quittungen ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md), [ADR-1118](./ADR-1118-Abruf-und-Lesebestätigungen.md)) und andere Nachweise (z. B. [ADR-1120](./ADR-1120-Beweissichere-Übermittlung-von-Anhängen.md)) müssen mit den zugehörigen Daten archiviert werden (ANF_FUN_ZUS_044). Der [Identitätsnachweis](../../Glossar.md#fachlicher-identitätsnachweis) ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)) wird übernommen, wenn ein Nachweis der [Authentisierung](../../Glossar.md#authentisierung) für den [Vorgang](../../Glossar.md#vorgang) erforderlich ist.

- **Datenminimierung:** Personenbezogene Daten sind nur bei Notwendigkeit ins Archiv zu übernehmen (DAT_ÜBE_ÜBE_021).

## Betrachtete Optionen

### Option A: Serverseitiger Klartext-Export

Der [Homeserver](../../Glossar.md#homeserver) exportiert Daten im Klartext in die Archivsysteme.

- Gut, denn dies lässt sich auch im zentralen Betrieb vergleichsweise einfach umsetzen.

- Schlecht, denn der Server bräuchte Zugriff auf den Klartext (SIC_VER_ÜBE_032).

- Schlecht, denn dies widerspricht dem Zero-Trust-Ansatz.

- Schlecht, denn bei einem zentralen Betrieb widerspräche dies [ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md) und [ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md).

### Option B: Zentraler Aufbewahrungsdienst

Der Homeserver übergibt die Ende-zu-Ende-verschlüsselten Nachrichten und Dateien direkt an einen zentralen Aufbewahrungsdienst.

- Gut, denn dies lässt sich auch im zentralen Betrieb vergleichsweise einfach umsetzen.

- Gut, denn der Server bräuchte keinen Zugriff auf den Klartext (SIC_VER_ÜBE_032).

- Schlecht, denn es entstehen Metadaten an zentraler Stelle, was ein Datenschutzproblem mit sich brächte.

- Schlecht, denn ein zentraler Dienst bringt Probleme bzgl. der Ausfallsicherheit und Latenz mit sich.

- Schlecht, denn dies widerspricht dem Zero-Trust-Ansatz.

- Schlecht, denn in der Zielarchitektur muss ein geeignetes Schlüsselmanagement entworfen werden, das für alle Teilnehmer geeignet ist, um langfristig die Verfügbarkeit der Daten zu sichern.

### Option C: Archivierung im Machtbereich des Besitzers

Der Export der Archivdaten findet im [Machtbereich](../../Glossar.md#machtbereich) der jeweiligen Datenbesitzer in den nach [ADR-1100](./ADR-1100-Bilaterale-Kommunikation-Festlegung-E2EE-Endpunkte.md) bis [ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md) zulässigen E2EE-Endpunkt statt.

- Gut, denn der Server bräuchte keinen Zugriff auf den Klartext (SIC_VER_ÜBE_032).

- Gut, denn es entstünden keine Metadaten außerhalb des Machtbereichs der Datenbesitzer.

- Gut, denn der Prozess ist durch die Datenbesitzer auf ihre Bedürfnisse anpassbar.

- Gut, denn eine gezielte clientseitige gestützte Archivierung bleibt möglich und kann manuell erfolgen.

- Schlecht, denn die Kommunikationsteilnehmer müssen selbst ggf. komplexe Prozesse umsetzen.

- Schlecht, denn alle Teilnehmer, die eine Archivierung benötigen, haben Mehraufwände im Betrieb.

### Option D: Postfach-Backend als Archivsystem

In Matrix speichern die Homeserver die Daten der Kommunikation in einer eigenen Datenbank. Sie sind verschlüsselt abgelegt. Bei der geeigneten Konfiguration von Löschfristen über Matrix Retention Policies wäre es denkbar, die [Postfach-Backends](../../Glossar.md#postfach-backend) mit ihren Homeservern direkt als Archivsystem zu nutzen. Eine genauere Analyse bleibt dabei notwendig.

- Gut, denn der Server bräuchte keinen Zugriff auf den Klartext (SIC_VER_ÜBE_032).

- Gut, denn bei souveränem Betrieb entstünden keine Metadaten außerhalb des Machtbereichs der Datenbesitzer.

- Gut, denn der Prozess ist durch die Datenbesitzer auf ihre Bedürfnisse anpassbar.

- Gut, denn eine gezielte clientseitige gestützte Archivierung bleibt möglich und kann manuell erfolgen.

- Schlecht, denn die Datenmengen entstünden immer in den Postfach-Backends.

- Schlecht, denn in der Zielarchitektur muss ein geeignetes Schlüsselmanagement entworfen werden, das für alle Teilnehmer geeignet ist, um langfristig die Verfügbarkeit der Daten zu sichern.

- Schlecht, denn ein Entwurf muss die unterschiedlichen Anforderungen aller Teilnehmer abdecken und die Löschfristen müssten detailliert konfigurierbar sein (ggf. im Client machbar).

## Architekturentscheidung

Gewählte Option: **Option C, Übernahme im Machtbereich der Datenbesitzer**. Die Zielarchitektur sieht keinen zentralen Aufbewahrungsdienst vor. Empfangende Organisationen übernehmen Nachrichten über ihre Endgeräte oder über einen [technischen Nutzer](../../Glossar.md#technischer-nutzer) auf ihren Servern, sofern dieser nach [ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md) und [ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md) als E2EE-Teilnehmer konfiguriert ist. Mit dem technischen Nutzer greift ein [Fachverfahren](../../Glossar.md#fachverfahren) oder eine Übernahmekomponente auf die Nachrichten in den [Vorgangsräumen](../../Glossar.md#vorgangsraum) zu. Der TI-Messenger der gematik sieht die Übernahme ebenfalls in die Archivsysteme der jeweiligen Einrichtung vor.[^1]

Für ausgehende Nachrichten gilt dasselbe Modell. Die sendende Stelle übernimmt aus Endgerät oder Fachverfahren heraus, dort liegt der Klartext bereits vor. Die nach dem Versand eingehenden Quittungen werden mit Bezug zum Vorgang ebenfalls übernommen.

Die übernehmende Stelle übergibt der E-Akte nicht die Nachricht allein, sondern ein Mindest-Exportpaket. Es enthält die Nachrichten und [Anhänge](../../Glossar.md#anhang) mit den zum Versandzeitpunkt geprüften Nachweisartefakten.

**Option D wird ausgeschlossen**, weil Daten dauerhaft in den Postfach-Backends anfielen und dort eine Schlüsselverwaltung für die langfristige Verfügbarkeit umgesetzt werden müsste.

### Exportpaket

Im Folgenden werden für die Archivierung bzw. Veraktung notwendige Datenpunkte gelistet.

| Bestandteil                     | Quelle                                                                                                                                                             |
|---------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Vorgangsbezug                   | Vorgangsraum-ID (nach einem Raum-Upgrade auch die IDs der Vorgängerräume), [Funktionspostfach](../../Glossar.md#funktionspostfach), Versandzeitpunkt, technische Sender-MXID                                                                                       |
| Fachliche Referenzen            | Aktenzeichen, Antrags-ID                                                                                                                                           |
| Nachricht und Anhänge           | entschlüsselte Inhalte, wenn notwendig                                                                                                                             |
| Identitätsnachweis              | signierter Identitätsnachweis, sofern fachlich erforderlich                                                                                                        |
| [Bereitstellungsbestätigung](../../Glossar.md#bereitstellungsbestätigung)      | siehe [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md), in maschinell prüfbarer Originalform                                                                                                               |
| Abruf- oder [Lesebestätigung](../../Glossar.md#lesebestätigung)     | siehe [ADR-1118](./ADR-1118-Abruf-und-Lesebestätigungen.md), sofern fachlich relevant                                                                                                                           |
| Anhangsnachweis                 | Bereitstellungsbestätigung des Anhangs nach [ADR-1120](./ADR-1120-Beweissichere-Übermittlung-von-Anhängen.md), sofern genutzt                                                                                               |
| Signatur oder Siegel auf Anhang | signiertes Original nach [ADR-0017](./ADR-0017-Authentizität-und-Integrität-von-Inhalten.md), sofern vorhanden                                                                                                                |
| Chiffrat der Nachricht          | übermitteltes Chiffrat oder die zu seiner Reproduktion nötigen Parameter, sonst ist der Ciphertext-Hash der Bereitstellungsbestätigung nicht verwertbar ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)) |
| Berechtigungsprotokoll          | auswertbares Protokoll der Postfachmitgliedschaften und Berechtigungsänderungen ([ADR-1112](./ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md)), sofern Funktionspostfach                                               |

### Lösungen für einen Export

Je nach E2EE-Endpunkt können unterschiedliche Lösungen für einen Export ermöglicht werden.

| E2EE-Endpunkt                                                        | Umsetzung                                                                      |
|----------------------------------------------------------------------|--------------------------------------------------------------------------------|
| Endgerät                                                             | manuell je Vorgang oder regelbasiert per SDK, einziger Weg im [Betriebsmodell](../../Glossar.md#betriebsmodell) A |
| technischer Nutzer als E2EE-Teilnehmer ([ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md) Option B, [ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)) | regelbasiert für alle Vorgänge des Funktionspostfachs                          |

## Auswirkungen

- Das Exportpaket ist standardisierbar.

- Die Nachweise sind Bestandteile des Exportpakets.

- Das SDK kann den Export unterstützen und senkt den Implementierungsaufwand für Clients und Fachverfahren.

- Die Trust-Historien müssen für die Dauer der Aufbewahrungsfristen verfügbar sein.

## Folge-Entscheidungen

- Löschung und Aufbewahrung im [Postfach](../../Glossar.md#postfach): Löschfristen und Nachweis der Löschung, siehe ADR-XXXX.

## Referenzen

[^1]: [gematik gemSpec TI-M Basis V1.2.0, A_25424 (Archivierung in Archivsysteme)](https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.2.0/), abgerufen am 12. Mai 2026.
