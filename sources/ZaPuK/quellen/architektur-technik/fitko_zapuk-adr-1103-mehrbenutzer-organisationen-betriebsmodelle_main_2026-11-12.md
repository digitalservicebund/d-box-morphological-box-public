<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Zielarchitektur/Architekturentscheidungen/ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob d51a3d9ae4); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# ADR-1103: Betriebsmodelle für Organisationen

> **Status:** ✔️ Operativ entschieden | **Datum:** 11. November 2026

## Kontext und Problembeschreibung

[Organisationen](../../Glossar.md#organisation) unterscheiden sich erheblich in ihren IT-Kapazitäten und in ihren Anforderungen an Souveränität und Datenhaltung. Ein kleines Unternehmen hat andere Anforderungen als ein Konzern mit eigener IT-Abteilung. Wie eine Organisation infrastrukturell an [ZaPuK](../../Glossar.md#zapuk) angebunden wird und wer welche Komponenten betreibt, hängt zudem davon ab, wo die Verschlüsselung terminiert wird, also im Endgerät der Mitarbeitenden oder im administrativen [Machtbereich](../../Glossar.md#machtbereich) der Organisation (vgl. [ADR-1100](./ADR-1100-Bilaterale-Kommunikation-Festlegung-E2EE-Endpunkte.md), [ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md)). Festzulegen ist, welche [Betriebsmodelle](../../Glossar.md#betriebsmodell) unterstützt werden und welche Verantwortlichkeiten sich daraus ergeben.

Die Betriebsmodelle in dieser Architekturentscheidung betreffen Organisationen. [Privatpersonen](../../Glossar.md#privatperson) nutzen ein dediziertes zentrales [Postfach-Backend](../../Glossar.md#postfach-backend) (vgl. Folge-Entscheidung).

### Komponenten

Es existieren **je Teilnehmendengruppe ein zentrales Postfach-Backend (vgl. Folge-Entscheidung)** sowie **dezentrale Postfach-Backends**.

In dieser Entscheidung wird das Privatpersonen-Backend nicht weiter betrachtet, da es nach aktuellem Entwurf keine Mehrbenutzerfunktionalität bieten muss.

#### Postfach-Backend

Umfasst die für eine Domain zuständigen, gemeinsam betriebenen Server-Komponenten:

- **Matrix-Homeserver:** Kommunikation, Föderation und verschlüsselte Speicherung von Nachrichten-Events.
- **Adressregister-Abfrage:** Schnittstelle des föderierten [Adressregisters](../../Glossar.md#adressregister) für Organisationen. Die Suche ist an den Matrix Identity Service angelehnt[^1], [^2] und bildet Merkmale einer Organisation auf [MXIDs](../../Glossar.md#mxid) ab. Bekannte Adressen werden über signierte Registereinträge im Matrix-Profil geprüft (vgl. Folge-Entscheidung).
- **Content Repository:** Speichert binäre [Anhänge](../../Glossar.md#anhang) als verschlüsselte Blobs. [^3]
- **Quittungsdienst:** Erzeugt die mit dem Siegel des Betreibers versehene [Bereitstellungsbestätigung](../../Glossar.md#bereitstellungsbestätigung) zur Absicherung der [Bekanntgabefiktion](../../Glossar.md#bekanntgabefiktion) bzw. der [Zustellfiktion](../../Glossar.md#zustellfiktion) (vgl. [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). Für den in der Quittung enthaltenen Eingangszeitpunkt wird der zentrale Zeitstempeldienst genutzt (vgl. Folge-Entscheidung). Die Umsetzung als Application Service, als Modul des [Homeservers](../../Glossar.md#homeserver) oder als Bot im eigenen Machtbereich wird durch den Betreiber festgelegt (vgl. [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

#### Weitere Komponenten

- **Self-Service-Portal (SSP):** Das [SSP](../../Glossar.md#self-service-portal) dient der Registrierung der Organisationen und der Homeserver. Die [Postfächer](../../Glossar.md#postfach) und die zentralen Berechtigungsverzeichnisse werden über die Administrationsoberfläche des jeweiligen zentralen Postfach-Backends gepflegt (vgl. Folge-Entscheidung). Hier sind die öffentlichen Schlüssel der Matrix-Homeserver und die Schlüssel für die Signatur der Registereinträge hinterlegt.

- **Identity Provider (IdP):** Der IdP authentifiziert die Kommunikationsteilnehmer. Nutzerkonten und Wallets sind Identitätsanbieter ohne Postfachfunktion (vgl. Folge-Entscheidung). Privatpersonen nutzen [BundID](../../Glossar.md#bundid), eID-Server und perspektivisch die [EUDI-Wallet](../../Glossar.md#europäische-brieftasche-für-die-digitale-identität). Mitarbeitende [privater Organisationen](../../Glossar.md#private-organisation) und [öffentlicher Stellen](../../Glossar.md#öffentliche-stelle) nutzen das [Org-Konto](../../Glossar.md#org-konto) ([ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md), [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md)). Souveräne Organisationen können ein organisationsinternes Identitätsmanagement verwenden, das über einen [Org-IdP-Adapter](../../Glossar.md#org-idp-adapter) anzubinden ist (vgl. Folge-Entscheidung).

- **Berechtigungsverzeichnis:** Die Pflege von Postfachmitgliedschaften, Rollen und [Bearbeitungsvertretungen](../../Glossar.md#bearbeitungsvertretung) für [Funktionspostfächer](../../Glossar.md#funktionspostfach) wird über ein [Berechtigungsverzeichnis](../../Glossar.md#berechtigungsverzeichnis) abgebildet. Es konfiguriert die Berechtigungen und Zuordnungen für den [RMS](../../Glossar.md#raum-management-service). Für jedes zentrale Postfach-Backend für Organisationen wird ein Berechtigungsverzeichnis betrieben.

- **Raum-Management-Service (RMS):** Der RMS setzt die Berechtigungen des Berechtigungsverzeichnisses automatisch um, indem er die Mitgliedschaften in den Räumen verwaltet. Er erlaubt die Konfiguration [technischer Nutzer](../../Glossar.md#technischer-nutzer), die als Raumadministrator mit oder ohne Schlüsselzugriff fungieren ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)).

## Entscheidungstreiber

- **Souveränität:** Organisationen müssen selbst entscheiden können, ob sie die Postfach-Backends sowie Identitäts- und Rechtemanagement selbst betreiben oder zentral bereitgestellte Dienste nutzen (vgl. [ADR-0002](./ADR-0002-Infrastrukturtopologie.md)).

- **Einfache Anbindung:** Die Anbindung von Organisationen und ihrer Infrastruktur wird über das zentrale Self-Service-Portal ermöglicht (vgl. [ADR-0008](./ADR-0008-Einheitliche-Anbindung-ueber-ein-zentrales-Self-Service-Portal.md), ANF_ARC_ANB_008). Die technische Anbindung der Clients (RMS, [Postfach-Zugänge](../../Glossar.md#postfach-zugang), Server) wird durch SDKs unterstützt (vgl. [ADR-0009](./ADR-0009-SDKs-zur-vereinfachten-Anbindung-von-Postfach-Zugängen.md), ANF_ARC_ANB_018).

- **Skalierbarkeit:** Das System muss für kleine Organisationen (z. B. Vereine) mit wenigen Postfächern einfach nutzbar sein. Darüber hinaus muss es für große Organisationen (z. B. [Behörden](../../Glossar.md#behörde)) mit komplexen Prozessen und Strukturen geeignet sein.

- **Offene Infrastruktur:** Drittimplementierungen von Postfach-Backends sind zugelassen. Eine Referenzimplementierung wird zentral bereitgestellt (vgl. [ADR-0007](./ADR-0007-Implementierung-von-Postfach-Backends.md), ANF_ARC_ANB_019).

## Betrachtete Optionen

Der administrative Machtbereich der Organisation umfasst sowohl den organisationsinternen Eigenbetrieb (eigene IT-Abteilung) als auch den Betrieb durch beauftragte Dritte (z. B. kommunale Rechenzentren oder IT-Dienstleister). In beiden Fällen behält die Organisation die rechtliche und fachliche Kontrolle über die Datenverarbeitung und das Schlüsselmaterial. Ein Dienstleister, der ein Postfach-Backend für Dritte betreibt und dafür den [Anschlussvertrag](../../Glossar.md#anschlussvertrag) annimmt (vgl. Folge-Entscheidungen [Anschlussbedingungen](../../Glossar.md#anschlussbedingungen) und Onboarding), handelt im Machtbereich der Organisation.

### Option A: Minimalanbindung

Die Organisation nutzt ausschließlich die zentral bereitgestellten Dienste und betreibt keinen eigenen RMS. Die Kommunikation erfolgt in Räumen mit zwei Teilnehmern, in manuell verwalteten Gruppenräumen ([ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md) Option A) oder in über den zentral bereitgestellten RMS automatisiert verwalteten Räumen ([ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md) Option B). Funktionspostfächer sind über den zentral bereitgestellten RMS verfügbar ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md), Variante ohne E2EE-Teilnahme), wobei ein Zugriff auf alte [Nachrichten](../../Glossar.md#nachricht) für neue Teilnehmer nur durch Weitergabe der Schlüssel von aktiven Teilnehmern möglich wird (ANF_FUN_ADR_063 eingeschränkt). Die Berechtigungen werden in der Administrationsoberfläche des zentralen Postfach-Backends gepflegt (vgl. Folge-Entscheidung). Die [E2EE](../../Glossar.md#ende-zu-ende-verschlüsselung) beginnt und endet im Endgerät der jeweiligen Teilnehmer. Bei dieser Umsetzung bleibt der Aufwand für die IT minimal. Die MXIDs enthalten die globale Domain des zentral betriebenen Postfach-Backends.

| Komponente                   | Betrieb                                                                                                                           |
|------------------------------|-----------------------------------------------------------------------------------------------------------------------------------|
| **Postfach-Backend**         | Zentrale Verantwortung                                                                                                            |
| **SSP**                      | Zentrale Verantwortung                                                                                                            |
| **IdP**                      | Externer IdP, im Standardpfad Org-Konto beziehungsweise externes [Authentisierungssystem](../../Glossar.md#authentisierungssystem)                                                  |
| **Berechtigungsverzeichnis** | Zentral im Postfach-Backend, Pflege über dessen Administrationsoberfläche (vgl. Folge-Entscheidung)                               |
| **RMS**                      | Zentral bereitgestellt, ohne E2EE-Teilnahme ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)), E2EE endet im Endgerät ([ADR-1100](./ADR-1100-Bilaterale-Kommunikation-Festlegung-E2EE-Endpunkte.md), [ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md) Option B ohne Schlüsselbesitz) |

![Option A: Minimalanbindung](images/ADR-1103-option-a.svg)

### Option B: Teilbetrieb (mit Funktionspostfächern)

Organisationen, die Funktionspostfächer mit vollem Funktionsumfang benötigen (Historienzugriff über E2EE-Teilnahme des technischen Nutzers, serverseitige Archivierung oder Malware-Prüfung), müssen den Raum-Management-Service selbst betreiben bzw. betreiben lassen (Auftragsverarbeitung). Das Berechtigungsverzeichnis kann organisationseigen betrieben oder zentral genutzt werden. Diese Infrastruktur nutzt das zentrale Postfach-Backend zum Transport und zur Ablage der Nachrichten bzw. Dateien. Der Aufwand für die Bereitstellung ist deutlich größer gegenüber Option A. Die MXIDs enthalten die globale Domain des zentral betriebenen Postfach-Backends. Die Postfachzuordnung kann in der Administrationsoberfläche des zentralen Postfach-Backends verwaltet werden (geringster Aufwand) oder Org-intern über ein eigenes Berechtigungsverzeichnis erfolgen.

| Komponente                   | Betrieb                                                               |
|------------------------------|-----------------------------------------------------------------------|
| **Postfach-Backend**         | Zentrale Verantwortung                                                |
| **SSP**                      | Zentrale Verantwortung                                                |
| **IdP**                      | Externer IdP, im Standardpfad Org-Konto oder externes Authentisierungssystem |
| **Berechtigungsverzeichnis** | Zentral oder Verantwortung der Organisation                           |
| **RMS**                      | Verantwortung der Organisation                                        |

![Option B: Teilbetrieb](images/ADR-1103-option-b.svg)

### Option C: Souveräner Betrieb

Die Organisation betreibt die gesamte Infrastruktur eigenständig. Dies bietet maximale Autonomie bei höchstem IT-Aufwand. Die MXIDs der Organisation verfügen über eine eigene Domain.

Da die Organisation den Postfach-Backend-Server eigenständig betreibt, ist sie selbst für die Erzeugung und Signatur der Bereitstellungsbestätigungen verantwortlich (vgl. [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). Bei Minimalanbindung und Teilbetrieb übernimmt dies der Betreiber des zentralen Postfach-Backends. Auch bei souveränem Betrieb ist der Einsatz von Funktionspostfächern und RMS optional. Eine Organisation kann Option C nutzen und ausschließlich mit Einzelchats und manuellen Gruppenräumen ([ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md) Option A) arbeiten.

Der Betrieb der TI-Messenger der gematik ist mit diesem Betriebsmodell vergleichbar. Organisationen des Gesundheitswesens können eigene Homeserver betreiben und über einen zentralen Registrierungsdienst anmelden.[^4] In der Zielarchitektur übernimmt das Self-Service-Portal (SSP) diese Funktion (vgl. [ADR-0008](./ADR-0008-Einheitliche-Anbindung-ueber-ein-zentrales-Self-Service-Portal.md)).

| Komponente                   | Betrieb                                                            |
|------------------------------|--------------------------------------------------------------------|
| **Postfach-Backend**         | Verantwortung der Organisation                                     |
| **SSP**                      | Zentral (nur Registrierung im [Verbund](../../Glossar.md#verbund))                   |
| **IdP**                      | Org-Konto, externes Authentisierungssystem oder organisationsinterner IdP |
| **Berechtigungsverzeichnis** | Verantwortung der Organisation                                     |
| **RMS**                      | Verantwortung der Organisation                                     |

![Option C: Souveräner Betrieb](images/ADR-1103-option-c.svg)

### Gegenüberstellung der Optionen

#### Option A: Minimalanbindung

- Gut, denn der Aufwand für eine Anbindung ist minimal.

- Gut, denn die Ende-zu-Ende-Verschlüsselung endet ausschließlich in Endgeräten der Teilnehmer.

- Gut, denn Funktionspostfächer mit automatisierten Bearbeitungsvertretungen sind über den zentral bereitgestellten RMS prinzipiell verfügbar ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md))

- Schlecht, denn der Zugriff auf alte Nachrichten ist für neue Teilnehmer im Raum von der Schlüsselweitergabe aktiver Mitglieder abhängig (ANF_FUN_ADR_063 eingeschränkt)

- Schlecht, denn es gibt keine Möglichkeit, serverseitige Dienste für die Archivierung oder Malware-Prüfung anzubinden.

- Schlecht, denn die MXIDs haben keine eigene Domain.

#### Option B: Teilbetrieb

- Gut, denn der Aufwand für eine Anbindung ist moderat.

- Gut, denn die Ende-zu-Ende-Verschlüsselung endet für entsprechend konfigurierte Räume in Endgeräten der Teilnehmer. Die Einschränkung ist, dass in Gruppenräumen neue Teilnehmer alte Nachrichten nur sehen können, wenn ein Teilnehmer mit den entsprechenden Schlüsseln diese aktiv weitergibt.

- Gut, denn eine automatisierte Verwaltung von Funktionspostfächern ist mit vollem Zugriff auf alte Nachrichten (E2EE-Teilnahme des technischen Nutzers) möglich.

- Gut, denn die Organisation muss kein eigenes Postfach-Backend betreiben.

- Schlecht, denn die Organisation muss den RMS und ggf. ein Berechtigungsverzeichnis betreiben.

- Schlecht, denn die MXIDs haben keine eigene Domain.

#### Option C: Souveräner Betrieb

- Gut, denn die Organisation erhält weitgehende Souveränität über die Infrastruktur und ihre Daten.

- Gut, denn die Organisation erhält ihre eigene Adress-Domain.

- Gut, denn die Ende-zu-Ende-Verschlüsselung endet für entsprechend konfigurierte Räume in Endgeräten der Teilnehmer. Die Einschränkung ist, dass in Gruppenräumen neue Teilnehmer alte Nachrichten nur sehen können, wenn ein Teilnehmer mit den entsprechenden Schlüsseln diese aktiv weitergibt.

- Schlecht, denn für Funktionspostfächer muss die Organisation den RMS und ggf. ein Berechtigungsverzeichnis betreiben.

- Schlecht, denn der IT-Aufwand bzw. die Betriebsverantwortung ist hoch.

## Architekturentscheidung

Gewählte Option: **Die Betriebsmodelle A, B und C werden unterstützt**, weil sich Organisationen in ihren Anforderungen stark unterscheiden und unterschiedliche IT-Fähigkeiten mitbringen. Die Organisation entscheidet selbst, welches Modell sie umsetzt. Unabhängig vom gewählten Modell gelten folgende Vorgaben:

- Für jede [Teilnehmendengruppe](../../Glossar.md#teilnehmendengruppe) (Privatpersonen, private Organisationen, öffentliche Stellen, [Organisationen der Justiz](../../Glossar.md#organisation-der-justiz)) wird ein eigenes Postfach-Backend bereitgestellt (vgl. Folge-Entscheidung).

- Die Ende-zu-Ende-Verschlüsselung terminiert nach [ADR-1100](./ADR-1100-Bilaterale-Kommunikation-Festlegung-E2EE-Endpunkte.md) und [ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md) im Endgerät. Ein zusätzlicher serverseitiger [E2EE-Endpunkt](../../Glossar.md#e2ee-endpunkt) ist nur im Machtbereich der Organisation zulässig.

- Alle Schnittstellen basieren auf offenen Standards und sind ohne proprietäre Komponenten nutzbar (ANF_ARC_ANB_007, ANF_ARC_ANB_021).

- Die Anbindung der Matrix-Clients (z. B. für Funktionspostfächer) an den Verbund wird durch SDKs unterstützt (vgl. ANF_ARC_ANB_018).

- Eine schnelle Anbindung von Organisationen ist mit Option A möglich. Sobald automatisierte Bearbeitungsvertretungen mit zuverlässigem Zugriff auf alte Nachrichten, serverseitige Archivierung oder Malware-Prüfung benötigt werden, ist ein Wechsel zu Option B oder C notwendig.

- Drittimplementierungen von Postfach-Backends sind unter Einhaltung der Anschlussbedingungen zugelassen (vgl. [ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md)). Für diese wird eine Referenzimplementierung bereitgestellt (vgl. [ADR-0007](./ADR-0007-Implementierung-von-Postfach-Backends.md)).

- Auch bei souveränem Betrieb (Option C) muss das Postfach-Backend im SSP registriert werden, bevor es am Verbund teilnehmen kann.

## Auswirkungen

- Die hybride Infrastrukturtopologie nach [ADR-0002](./ADR-0002-Infrastrukturtopologie.md) wird durch die Betriebsmodelle A, B und C konkretisiert.

- Organisationen können mit Option A beginnen und später zu Option B oder C wechseln. Ohne `.well-known`-Delegation ändert der Wechsel zu Option C die Domain und damit alle Adressen der Organisation (vgl. Folge-Entscheidung).

## Risiken

- **Option A:** Funktionspostfächer sind mittels zentral bereitgestellten RMS ohne E2EE-Teilnahme umgesetzt. Ohne Schlüsselweitergabe durch aktive Teilnehmer ist kein Zugriff auf alte Nachrichten möglich. Es gibt keine serverseitige Archivierung oder Malware-Prüfung. Ob ein zentraler RMS mit MLS umsetzbar bleibt, ist derzeit nicht abschließend zu beurteilen ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)). Das zentrale Berechtigungsverzeichnis enthält Informationen zu Gruppen und andere Interna der Organisation. Das führt zu doppelter Pflege von internen Daten beim zentralen Betreiber.

- **Option B:** Die Organisation muss sicherstellen, dass der Betreiber des RMS die Anforderungen aus [ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md) und [ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md) einhält. Die vertragliche Absicherung (Auftragsverarbeitung) ist notwendig.

- **Option C:** Die Organisation trägt die volle Verantwortung für Betrieb, Wartung und Absicherung aller Komponenten. Die mit [fortgeschrittenen elektronischen Siegeln](../../Glossar.md#fortgeschrittenes-elektronisches-siegel) signierten Bereitstellungsbestätigungen der Backends staatlicher Organisationen können die Beweiskraft öffentlicher elektronischer Dokumente nach § 371a Abs. 3 ZPO erreichen, wenn ihre Form durch Rechtsvorschrift festgelegt wird ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). Dagegen erreichen die Quittungen privat betriebener Backends diese Beweiskraft nicht.

- **Option B und C:** Die Wahl der RMS-Konfiguration (E2EE-Teilnahme, Moderatoren etc. vgl. [ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)) richtet sich nach den Anforderungen der Organisation. Ist eine besonders hohe Vertraulichkeit der Kommunikation notwendig, ist eine manuelle Raumverwaltung durch Moderatoren möglich.

- **Alle Optionen:** Ein RMS ist als kritische Komponente entsprechend abzusichern, da eine Kompromittierung auf alle Räume wirkt, die durch ihn verwaltet werden. Die zentral bereitgestellten RMS verfügen über keine Schlüssel und haben somit keinen Klartextzugriff, womit der Schaden begrenzt bliebe. Allerdings können über den RMS über Einladungen Unbefugte Zugriff erlangen.

## Folge-Entscheidungen

- PKI-Vertrauensanker: Zertifikate für den Matrix-Verbund, siehe [ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md).

- MXID-Struktur und Domain-Governance: Adressformat, Migration von Backends, siehe [ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md).

- Postfachadressen von Organisationen: Adressregister für Organisationen und Adressregister-Abfrage, siehe [ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md).

- Berechtigungsverzeichnis: Konfiguration und Umsetzung der Berechtigungen je Funktionspostfach, siehe [ADR-1112](./ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md).

- Anschlussbedingungen: verbindliche Vorgaben für Postfach-Backends, siehe [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md).

- [Nachweisstellen](../../Glossar.md#nachweisstelle) und Org-IdP-Adapter: Anbindung eines organisationsinternen Identitätsmanagements, siehe [ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md).

- Entkopplung von [Nutzerkonto](../../Glossar.md#nutzerkonto) und Postfach, siehe [ADR-1110](./ADR-1110-Entkopplung-von-Nutzerkonto-und-Postfach.md).

- Betrieb der zentralen Postfach-Backends getrennt nach Teilnehmendengruppen, siehe [ADR-1114](./ADR-1114-Getrennter-Betrieb-der-zentralen-Postfach-Backends-nach-Teilnehmendengruppen.md).

- Onboarding im Self-Service-Portal: Registrierung von Organisationen und Backends, Anschlussvertrag, siehe [ADR-1148](./ADR-1148-Onboarding-im-Self-Service-Portal.md).

## Referenzen

[^1]: [Matrix Spec, Introduction to the Matrix APIs](https://spec.matrix.org/latest/#introduction-to-the-matrix-apis), abgerufen am 11. Mai 2026.

[^2]: [Matrix Spec, Architecture Identity](https://spec.matrix.org/latest/#identity), abgerufen am 11. Mai 2026.

[^3]: [Matrix Spec, Content Repository](https://spec.matrix.org/v1.17/client-server-api/#content-repository), abgerufen am 11. Mai 2026.

[^4]: [gematik gemSpec TI-M Basis V1.2.0, §2.1 TI-Messenger Föderation (eigene Messenger-Services, Registrierungs-Dienst, Föderationsliste)](https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.2.0/), abgerufen am 11. Mai 2026.
