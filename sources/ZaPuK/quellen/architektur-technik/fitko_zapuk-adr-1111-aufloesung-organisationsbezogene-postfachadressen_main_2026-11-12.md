<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Zielarchitektur/Architekturentscheidungen/ADR-1111-Aufl%C3%B6sung-Organisationsbezogene-Postfachadressen.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 9e6d839adb); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# ADR-1111: Auflösung und Lebenszyklus organisationsbezogener Postfachadressen

> **Status:** ✔️ Operativ entschieden | **Datum:** 12. November 2026

## Kontext und Problembeschreibung

In dieser Architekturentscheidung werden die Postfachadressen von [öffentlichen Stellen](../../Glossar.md#öffentliche-stelle) und [privaten Organisationen](../../Glossar.md#private-organisation) betrachtet. Soll eine [Organisation](../../Glossar.md#organisation) adressiert werden, muss zunächst eine Postfachadresse ermittelt werden. Im Falle einer Antragstellung erhält die [Behörde](../../Glossar.md#behörde) neben der im [Antrag](../../Glossar.md#antrag) enthaltenen [Organisationsidentität](../../Glossar.md#organisationsidentität) gegebenenfalls auch eine manuell angegebene Rückantwortadresse, an die eine Behörde Rückfragen bzw. [Bescheide](../../Glossar.md#bescheid) übermittelt. Eine Rückantwortadresse ist eine Postfachadresse ([MXID](../../Glossar.md#mxid)) der antragstellenden Organisation. Um eine Fehlübermittlung bei Rückantworten durch die Behörde zu verhindern, wird geprüft, ob die Postfachadresse zur Organisationsidentität des Antrags gehört. Dieser ADR diskutiert Optionen, wie diese Prüfung ermöglicht werden kann.

Die Domain einer Postfachadresse ist durch das [Postfach-Backend](../../Glossar.md#postfach-backend) vorgegeben, in dem der Postfachinhaber mit seinem Konto registriert ist. Durch die zulässigen [Betriebsmodelle](../../Glossar.md#betriebsmodell) ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)) wird auch eine dezentrale Pflege der Adressen notwendig. Dabei muss für die Abfrage eine einheitliche Schnittstelle für alle Teilnehmer im [Verbund](../../Glossar.md#verbund) bereitgestellt werden.

Dieser ADR behandelt lediglich Postfachadressen von Organisationen. Zu den Adressen einer Organisation gehören vor allem die Funktionspostfachadressen, die der Übermittlung von [Nachrichten](../../Glossar.md#nachricht) an Gruppen von Beschäftigten dienen. Die Adressierung mittels [Pseudonymen](../../Glossar.md#pseudonym) von [Privatpersonen](../../Glossar.md#privatperson) wird hier nicht betrachtet (vgl. [ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md)).

## Entscheidungstreiber

- **Geprüfte Zuordnung vor Verwendung:** Eine Postfachadresse einer Organisation darf erst verwendet werden, nachdem ihre Zugehörigkeit zu einer Organisationsidentität sicher geprüft wurde (SIC_VER_ÜBE_056, SIC_AUT_ÜBE_008).
- **Auffinden von Organisationsadressen:** Die Postfachadressen von Organisationen müssen sich ermitteln lassen (ANF_FUN_ADR_059, ANF_FUN_ADR_060, ANF_FUN_ADR_065).
- **Dezentrale Pflege:** Bei dezentralem Betrieb pflegen die jeweiligen Betreiber ihre Adressen selbst (vgl. [ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)).
- **Einheitliche Verbundschnittstelle:** Prüfung und Ermittlung von Adressen erfolgen über eine einheitliche Schnittstelle im gesamten Verbund.
- **Zero Trust:** Die Zuordnung der Organisationsidentitäten zu den [Postfächern](../../Glossar.md#postfach) folgt den Zero-Trust-Prinzipien (SIC_ÜBE_ÜBE_018).

## Betrachtete Optionen

### Option A: Postfachadresse als Identitätsmerkmal

Die Postfachadresse wird als Attribut in die [Identität](../../Glossar.md#identität) aufgenommen, etwa in das ELSTER-Organisationszertifikat oder ein V-PKI-Zertifikat.

- Gut, denn die Zugehörigkeit der Postfachadresse zur Identität ist kryptographisch prüfbar.

- Gut, denn die Adressprüfung der Rückantwortadresse erfordert keinen Netzwerkaufruf.

- Schlecht, denn der Lebenszyklus von Adresse und Identität würde eng aneinander gekoppelt werden.

- Schlecht, denn Organisationen verfügen in der Regel über mehrere Adressen, sodass die Identität häufig geändert werden müsste.

- Schlecht, denn das steht der Pflege von Adressen in dezentralen Betriebsmodellen entgegen.

### Option B: separater signierter Adressnachweis

Es werden eigene kryptographisch abgesicherte Nachweise etabliert. Diese müssen einem Antrag zusätzlich zum Identitätsnachweis hinzugefügt werden, um eine Prüfung der Adresszuordnung zur Organisation durch die Bearbeiter zu erlauben.

- Gut, denn die Identitätszuordnung zur Postfachadresse ist kryptographisch prüfbar.

- Gut, denn eine Umsetzung erlaubt die Pflege der Adressen bei einem dezentralen Betrieb durch die jeweiligen Betreiber.

- Neutral, denn die Adressprüfung der Rückantwortadresse erfordert nur zyklisch einen Netzwerkaufruf (Nachweisstatus).

- Schlecht, denn separate Nachweise würden zusätzliche Aufwände für die entsprechenden Prozesse und in der technischen Umsetzung mit sich bringen.

### Option C: Verwendung der Adresse ohne Prüfung

Die einen Antrag bearbeitende Stelle nutzt die angegebene Rückantwortadresse, ohne diese auf die Zugehörigkeit zur antragstellenden Organisation zu prüfen.

- Gut, denn so entstünden keine weiteren Aufwände für die Umsetzung sowohl des Registers als auch der Clients.

- Gut, denn die Adressprüfung der Rückantwortadresse erfordert keinen Netzwerkaufruf.

- Schlecht, denn Fehlübermittlung durch Tippfehler oder absichtliche Angabe einer fremden Adresse ist nicht ausgeschlossen.

### Option D: Föderiertes Adressregister mit eigener Abfrageschnittstelle

Für ein im Verbund föderiertes [Adressregister](../../Glossar.md#adressregister) wird eine eigene Abfrageschnittstelle festgelegt. Angelehnt an die Identity Service API der Matrix-Spezifikation[^1] bildet sie Merkmale der Identität auf eine MXID ab. Die Identity Service API kennt personenbezogene Merkmale wie E-Mail-Adresse oder Telefonnummer. Bei Abfrage mittels Merkmal wird in einer unsignierten Antwort die MXID zurückgegeben. Aus Datenschutzgründen ist eine Abfrage in umgekehrter Richtung nicht vorgesehen. Im Gegensatz zu Adressen natürlicher Personen sind bestimmte Organisationsadressen für die Öffentlichkeit oder bestimmte Stellen relevant. Eine Abfrage soll eine Suche von Organisationsadressen zu einer Organisation ebenso ermöglichen wie eine Prüfung der Zugehörigkeit von Adressen zu einer Organisation mittels signierter Antworten. Die Hoheit über die Postfachadressen und deren Pflege liegt bei den jeweiligen Backend-Betreibern. Im Adressregister sind die Postfachadressen mit der jeweiligen Organisationsidentität kryptographisch verknüpft, was verbundweit prüfbar ist. Vor der Verwendung von Rückantwortadressen aus Anträgen kann somit die notwendige Prüfung erfolgen. Die bestehende Identity Service API reicht für die Anforderungen an das Adressregister nicht aus. Sie muss erweitert oder durch eine eigene Schnittstelle ersetzt werden.

- Gut, denn die Identitätszuordnung zur Postfachadresse ist kryptographisch prüfbar.

- Gut, denn der Lebenszyklus von Identität und Adresse ist getrennt. Die Pflege skaliert gut mit höherer Anzahl von Adressen.

- Gut, denn eine Umsetzung erlaubt die Pflege der Adressen bei einem dezentralen Betrieb durch die jeweiligen Betreiber.

- Schlecht, denn die Adressprüfung der Rückantwortadresse erfordert immer einen Netzwerkaufruf (Adressregister).

## Architekturentscheidung

Gewählte Option: **Option D, Föderiertes Adressregister mit eigener Abfrageschnittstelle**, weil die Lebenszyklen von Identitäten und Postfachadressen getrennt umgesetzt sind und die Rückantwortadressen der Antragsteller hinsichtlich Organisationszugehörigkeit prüfbar sind. Zudem ist das Adressregister föderiert umsetzbar. Bei dezentralem Betrieb des Postfach-Backends können die Adressen durch die jeweiligen Betreiber gepflegt werden.

## Umsetzung der gewählten Option

Das Adressregister wird im gesamten Verbund einheitlich geführt. Jedes Postfach-Backend führt die Einträge seiner Domain. Das Adressregister besteht aus diesen Einträgen, verhält sich für abfragende Stellen aber wie ein einziges Register. Die [Adressregister-Abfrage](../../Glossar.md#adressregister-abfrage) umfasst die Suche nach Adressen und die Prüfung einer bekannten Adresse.

Die Adresseinträge der zentral betriebenen Domains bzw. Backends (vgl. Folge-Entscheidung) werden über die Administrationsoberfläche des zentralen Postfach-Backends gepflegt und dort vorgehalten. Die Registrierung der Organisation sowie ihrer Domain bzw. ihres Organisations-Präfixes erfolgt im [Self-Service-Portal](../../Glossar.md#self-service-portal) (SSP), das dafür Informationen und Zertifikate des Postfach-Backends der jeweiligen Domain benötigt (vgl. Folge-Entscheidung). Der lokale Anteil der MXID kann durch die jeweilige Organisation selbst angegeben werden (vgl. [ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md) zur MXID-Struktur). Organisationen pflegen also auch im zentralen Betrieb ihre Adressen selbst.

Die Betreiber der dezentralen Postfach-Backends können die Einträge ihrer Domain selbst bestimmen. Das von ihnen betriebene Register kann mit eigenen Prozessen gepflegt werden. Einträge müssen mit dem im SSP bei der Registrierung des Backends angegebenen Schlüssel signiert werden, so dass eine kryptographische Prüfung verbundweit ermöglicht wird. Dabei wird geprüft, ob die Organisation zur Domain des signierenden Backends gehört. Die Zuordnung von Domain und Organisation wird bei der Registrierung im SSP festgelegt und mit der signierten [Föderationsliste](../../Glossar.md#föderationsliste) verteilt (vgl. Folge-Entscheidung). Auch wenn die Zuordnung der Organisation zu dezentral betriebenen Domains über den Domainnamen denkbar ist, bleibt der signierte Registereintrag die verbindliche Quelle für die Prüfung.

Die Adressregister-Abfrage erlaubt neben der Prüfung der Organisationszugehörigkeit und der Suche von Adressen auch die Feststellung des Status des Registereintrags. Adressen können deaktiviert werden. Die Adresse eines [Funktionspostfachs](../../Glossar.md#funktionspostfach) ist erst aktiv, wenn dem Funktionspostfach im [Berechtigungsverzeichnis](../../Glossar.md#berechtigungsverzeichnis) mindestens ein Mitglied zugeordnet ist (vgl. Folge-Entscheidung). Wird das letzte Mitglied entfernt, wird der Eintrag inaktiv. Im zentralen Betriebsmodell setzt das Postfach-Backend den Status anhand des zentralen Berechtigungsverzeichnisses. Bei organisationsinternen Berechtigungsverzeichnissen ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md), Betriebsmodell B und C) setzt der [Raum-Management-Service](../../Glossar.md#raum-management-service) der Organisation den Status über die Schnittstelle des zuständigen Postfach-Backends. Eine Löschung inaktiver Einträge ist nicht vorgesehen. Somit bleibt eine Prüfung der Zuordnung später noch möglich.

Eine Abfrage des Adressregisters kann in unterschiedlichen Richtungen erfolgen. Zum einen ist eine Suche nach Adressen mittels Merkmalen der Organisation über die Schnittstelle für die Suche möglich und zum anderen kann eine bekannte Adresse auf Organisationszugehörigkeit geprüft werden. Die Suche von Matrix findet nur Adressen, die dem eigenen [Homeserver](../../Glossar.md#homeserver) bereits bekannt sind. Für die Suche über Domaingrenzen ist deshalb eine Erweiterung zu implementieren. Für die Prüfung der Organisationszugehörigkeit wird ein signierter Registereintrag als Feld zur MXID abgelegt (MSC4133).[^4] Eine Prüfung findet hier über die Profilschnittstelle des Backends statt. Der signierte Eintrag enthält die Adresse, die Organisationsidentität, den Status und Registrierungszeitpunkt. Bei öffentlichen Stellen gibt er zudem den [Bereich](../../Glossar.md#bereich) an, dem die Organisation angehört. Daran erkennt der [Postfach-Zugang](../../Glossar.md#postfach-zugang) einer Privatperson bei einer Kontaktaufnahme, welche ihrer bereichsspezifischen Adressen er verwendet ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md)).

Der in dieser Architekturentscheidung betrachtete Anwendungsfall der Antragstellung mit angegebener Rückantwortadresse nutzt die Zuordnung im Adressregister, um eine fehlerhafte Übermittlung von Antworten zu verhindern. Die jeweilige Fachverfahrenssoftware muss diese Prüfung implementieren. Über das SDK oder direkt über die Profilschnittstelle ist das technisch mit wenig Aufwand verbunden.

![Prüfung einer Rückantwortadresse](images/ADR-1111-pruefung-rueckantwortadresse.svg)

Die angeführten Funktionspostfachadressen zur Übermittlung von Nachrichten an Organisationseinheiten, Abteilungen oder Gruppen lassen sich durch die angebundene Organisation im zentralen oder dezentralen Modell selbständig pflegen. Ein Wechsel einer Organisation von einem Postfach-Backend in ein anderes, zum Beispiel bei Einführung eines eigenen Postfach-Backends, wird über Matrix-eigene Verfahren (`.well-known`-Konfiguration)[^2] unterstützt. Bei einem Umzug werden bestehende Adresseinträge deaktiviert und nicht gelöscht. Somit bleibt die Zuordnungsprüfung auch nach dem Umzug für die inaktiven Adressen der alten Domain möglich.

Das Adressregister trifft lediglich Aussagen über die Zuordnung von Adressen zu Organisationen. Die Zuordnung der Organisationsidentität zu einer konkreten Nachricht ist über verknüpfte Nachweise umgesetzt (siehe [ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)).

Eine im Betrieb befindliche vergleichbare Lösung ist das zentrale VZD-FHIR-Directory des TI-Messengers der gematik.[^3] In der Zielarchitektur ist das Adressregister dagegen föderiert.

## Auswirkungen

- Postfachadressen von Organisationen sind entgegen Lösungen im Bestand kein Merkmal der Identität ([ADR-1110](./ADR-1110-Entkopplung-von-Nutzerkonto-und-Postfach.md)). Deshalb müssen in Anträgen angegebene Rückantwortadressen vor erster Nutzung auf Zugehörigkeit zur antragstellenden Organisation im Adressregister geprüft werden.

- Die Feststellung, ob eine Postfachadresse (MXID) einer Organisation angehört, ist über das Adressregister zu treffen. Der Domain-Teil einer Adresse erlaubt keine verbindliche Aussage.

- Postfach-Zugänge müssen Zugriff auf die Schnittstelle des Adressregisters erhalten, bevor sie die in Anträgen angegebenen Rückantwortadressen nutzen können.

- [Fachverfahren](../../Glossar.md#fachverfahren) und Postfach-Zugänge prüfen vor der Übermittlung neben der Organisationszugehörigkeit auch den Status des Eintrags. An inaktive Adressen, auch an Funktionspostfächer ohne zugeordnetes Mitglied, wird nicht übermittelt.

## Risiken

- Bestimmte Teile des Adressregisters (die Einträge einzelner Postfach-Backends) können zum Beispiel zur Wartung kurzfristig nicht erreichbar sein. Durch entsprechendes Caching kann dieses Risiko verringert werden.

- [Anschlussbedingungen](../../Glossar.md#anschlussbedingungen) sollen die Prüfung unbekannter Adressen von Organisationen über das Adressregister vor einer Übermittlung verbindlich machen.

## Folge-Entscheidungen

- Onboarding im Self-Service-Portal: Registrierung von Organisation, Domain und Präfix, siehe [ADR-1148](./ADR-1148-Onboarding-im-Self-Service-Portal.md).

- Anschlussbedingungen zur verbindlichen Adressregister-Abfrage vor Übermittlung an unbekannte Adressen, siehe [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md).

- Getrennter Betrieb der zentralen Postfach-Backends nach [Teilnehmendengruppen](../../Glossar.md#teilnehmendengruppe): Adresseinträge je zentralem Backend, siehe [ADR-1114](./ADR-1114-Getrennter-Betrieb-der-zentralen-Postfach-Backends-nach-Teilnehmendengruppen.md).

- Berechtigungsverzeichnis für Rechte in Funktionspostfächern, siehe [ADR-1112](./ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md).

- PKI-Anforderungen zur Signatur der Einträge im Adressregister, siehe [ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md).

## Offene Punkte

- Spezifikation des signierten Registereintrags (Format, Signatur, Caching) und der Erweiterung für die Suche. Für die Prüfung genügen Felder im Matrix-Profil. Für die Suche sind die Verteilung im Verbund, also Weiterleitung der Anfragen oder Replikation der Einträge, und die Schnittstelle in einem [MSC](../../Glossar.md#msc) festzulegen (MSC4258 Federated User Directory).[^5] Das VZD-FHIR-Directory kann als Vergleich genutzt werden.

- Verhältnis der eindeutigen digitalen Adresse und des europäischen Digitalverzeichnisses der EU-Business-Wallet (Vorschlag COM(2025) 838) zum Adressregister.

## Referenzen

[^1]: [Matrix Specification, Identity Service API](https://spec.matrix.org/latest/identity-service-api/), abgerufen am 18. Juni 2026.

[^2]: [Matrix Specification, Client-Server API, Server Discovery](https://spec.matrix.org/latest/client-server-api/#server-discovery), abgerufen am 18. Juni 2026.

[^3]: [gematik gemSpec TI-M Basis V1.2.0, §2.5.3 (VZD-FHIR-Directory)](https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.2.0/), abgerufen am 12. November 2026.

[^4]: [Matrix Specification, Client-Server API, Profiles (eigene Profilfelder nach MSC4133 seit Matrix 1.16)](https://spec.matrix.org/latest/client-server-api/#profiles), abgerufen am 8. Oktober 2026.

[^5]: [MSC4258: Federated User Directory](https://github.com/matrix-org/matrix-spec-proposals/pull/4258), abgerufen am 12. November 2026.
