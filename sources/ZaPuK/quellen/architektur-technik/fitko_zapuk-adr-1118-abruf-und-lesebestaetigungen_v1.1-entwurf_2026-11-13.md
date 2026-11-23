<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/ef99118c0607dd7fb11a005593569a36a0f84343/Zielarchitektur/Architekturentscheidungen/ADR-1118-Abruf-und-Lesebest%C3%A4tigungen.md -->
<!-- Branch: architecture-v1.1, Commit ef99118c06 (2026-11-13); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-1118-Abruf-und-Lesebestätigungen.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 2cf091cab3); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# ADR-1118: Abruf- und Lesebestätigungen

> **Status:** ✔️ Operativ entschieden | **Datum:** 13. November 2026

## Kontext und Problembeschreibung

Die automatisierte, serverseitig signierte [Bereitstellungsbestätigung](../../Glossar.md#bereitstellungsbestätigung) beweist, dass eine [Nachricht](../../Glossar.md#nachricht) zu einem bestimmten Zeitpunkt in den [Machtbereich](../../Glossar.md#machtbereich) der empfangenden Person gelangt ist. An die Bereitstellung knüpft die [Bekanntgabefiktion](../../Glossar.md#bekanntgabefiktion) an, für ein vom [Nutzerkonto](../../Glossar.md#nutzerkonto) entkoppeltes [Postfach](../../Glossar.md#postfach) erst nach einer Anpassung von § 9 OZG (vgl. [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). Die Bereitstellungsbestätigung beantwortet nur die Frage, ob eine Nachricht bereitgestellt wurde.

Im Folgenden geht es um die Frage, ob eine Nachricht zur Kenntnis genommen wurde. Dafür stehen zwei Konzepte zur Verfügung.

- **Abrufbestätigung:** Eine automatisiert erstellte Quittung über den Abruf einer Nachricht oder eines [Anhangs](../../Glossar.md#anhang) aus dem Postfach.
- **Lesebestätigung (Empfangsbekenntnis):** Eine durch eine bewusste Handlung ausgelöste Bestätigung über die Kenntnisnahme einer Nachricht.

### Abgrenzung

- **[ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md):** Bereitstellungsbestätigung als automatisierter Nachweis des Eingangs im Machtbereich (Transportebene).

- **[ADR-0017](./ADR-0017-Authentizität-und-Integrität-von-Inhalten.md):** [Authentizität](../../Glossar.md#authentizität) und [Integrität](../../Glossar.md#integrität) des Nachrichteninhalts.

- **Absendergesteuerte Übermittlungsoptionen:** Steuerung pro Nachricht, ob eine Abruf- oder [Lesebestätigung](../../Glossar.md#lesebestätigung) angefordert wird, sowie Berechtigungsprüfung des [Absenders](../../Glossar.md#absenderin) werden in einer Folge-Entscheidung geregelt.

## Entscheidungstreiber

- **Abrufbestätigung:** Das System übermittelt automatisch eine [Abrufbestätigung](../../Glossar.md#abrufbestätigung) im Falle eines erfolgreichen Abrufs einer Nachricht aus dem Postfach (ANF_FUN_ZUS_046)

- **Aktive Kenntnisnahme:** [Empfänger](../../Glossar.md#empfängerin) sollen die Kenntnisnahme einer Nachricht aktiv bestätigen können.

- **Empfangsbekenntnis:** Unterstützung der förmlichen Zustellung mit fristauslösender Wirkung und aktiv erzeugter Empfangsquittung (vgl. § 5 VwZG, § 173 Abs. 3 ZPO)

- **E2EE-Konformität:** Innerhalb der Infrastruktur können Bestätigungen nur ohne Zugriff auf den Nachrichtenklartext erzeugt werden (SIC_VER_ÜBE_032)

- **Bestandslösungen:** Die Zielarchitektur muss die in den Bestandslösungen etablierten Bestätigungsmuster abbilden oder verbessern.

## Betrachtete Optionen

### Option 1: Matrix Read Receipts (`m.read`)

Nutzung des nativen Matrix-Verfahrens für Abrufbestätigungen. Read Receipts (`m.read`) werden als unverschlüsselte ephemere [Events](../../Glossar.md#event) übertragen[^1].

- Gut, denn das Verfahren ist im Matrix-Protokoll nativ vorhanden und erfordert keine Eigenentwicklung.

- Schlecht, denn Read Receipts sind unverschlüsselt. Der Homeserver-Betreiber kann nachvollziehen, wann welche Nachricht abgerufen wurde.

- Schlecht, denn Lesebestätigungen für einzelne Nachrichten mit [QES](../../Glossar.md#qualifizierte-elektronische-signatur) sind mit ephemeren Events nicht nativ umsetzbar.

### Option 2: Verschlüsselte anwendungsspezifische Events

Sowohl Abrufbestätigung als auch Lesebestätigung werden als verschlüsselte Custom Events (ggf. To-device Events) im Raum gesendet. Die Bestätigungsart wird im Event-Content unterschieden.

- Gut, denn Abrufbestätigung und Lesebestätigung sind getrennt und semantisch eindeutig.

- Gut, denn die Events sind verschlüsselt. Es entstehen keine weiteren für den Homeserver-Betreiber sichtbaren Metadaten.

- Gut, denn die Umsetzung kann erweitert werden. Wenn [Nutzende](../../Glossar.md#nutzerinnen) das Lesen der Nachrichten aktiv und zurechenbar bestätigen, kann das Event als [elektronisches Empfangsbekenntnis](../../Glossar.md#elektronisches-empfangsbekenntnis) gelten.

- Gut, denn die Bestätigung kann unabhängig von der Reihenfolge in der Timeline für beliebige einzelne Events erzeugt werden

- Schlecht, denn konforme Clients müssen mehrere Event-Typen implementieren.

- Schlecht, denn die Abrufbestätigung ist durch einen modifizierten Client wie bei Option 1 (m.read) unterdrückbar, jedoch ohne den Vorteil der nativen Protokollunterstützung.

### Option 3: MSC4089 (Delivery Receipts)

MSC4089[^2] definiert Delivery Receipts auf Protokollebene als Erweiterung der Matrix-Spezifikation. Der [MSC](../../Glossar.md#msc) befindet sich im Entwurfsstadium.

- Gut, denn die Bestätigung wäre nativ im Protokoll verankert.

- Schlecht, denn der MSC adressiert weder Persistenz noch Authentizität der Quittung.

- Schlecht, denn eine Unterscheidung zwischen Abrufbestätigung, Lesebestätigung und Anhang-Bestätigung ist nicht vorgesehen.

- Schlecht, denn der MSC ist nicht finalisiert und zeitlich nicht planbar.

## Architekturentscheidung

Gewählte Option 2: **Verschlüsselte anwendungsspezifische Events**, weil diese Option die automatische Abrufbestätigung (ANF_FUN_ZUS_046), die aktive Lesebestätigung und die Erweiterbarkeit zum elektronischen Empfangsbekenntnis (§ 173 Abs. 3 ZPO) unter Wahrung der E2EE-Konformität (SIC_VER_ÜBE_032) abdeckt.

### Details der gewählten Option

Lese- und Abrufbestätigungen werden als verschlüsselte Room-Events gesendet. Eine serverseitige Protokollierung durch den [Quittungsdienst](../../Glossar.md#quittungsdienst) oder andere Infrastrukturkomponenten findet nicht statt.

Der Client des Empfängers übermittelt Abrufbestätigungen nur auf Anforderung durch den Absender (vgl. Folge-Entscheidung). Dabei wird pro Anforderung pro Nachricht eine Bestätigung gesendet. Mit ihr kann der Zeitpunkt des Abrufs einer Nachricht und verknüpfter Anhänge belegt werden.

#### Abrufbestätigung (Nachricht)

Der Client sendet ohne aktive Nutzerhandlung automatisch ein verschlüsseltes Event, sobald die Nachricht entschlüsselt und angezeigt wurde. Ein Client kann dies unterlassen, und der [Verbund](../../Glossar.md#verbund) kann es nicht erzwingen. Die Architektur verlässt sich deshalb nicht auf die Abrufbestätigung. Als belastbarer Nachweis dient die serverseitige Bereitstellungsbestätigung ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). Wo keine Bekanntgabe- oder [Zustellfiktion](../../Glossar.md#zustellfiktion) greift, etwa bei Dokumenten, die keine [Verwaltungsakte](../../Glossar.md#verwaltungsakt) sind, bleibt die Abrufbestätigung der einzige Nachweis des tatsächlichen Abrufs.

#### Abrufbestätigung (Anhang)

Für Anhänge wird eine separate Abrufbestätigung gesendet, sobald der Anhang heruntergeladen und entschlüsselt wurde. Das Event enthält eine Referenz auf die Originalnachricht sowie den Hash des entschlüsselten Anhangs. Damit ist nachweisbar, dass der Anhang erfolgreich abgerufen und entschlüsselt werden konnte. Die beweissichere Übermittlung von Anhängen wird in einer Folge-Entscheidung betrachtet.

#### Lesebestätigung

Für die aktive Lesebestätigung wird ein eigener Event-Typ definiert. Der Empfänger bestätigt im Client aktiv, dass die Nachricht zur Kenntnis genommen wurde. Das Event enthält eine Referenz auf die Originalnachricht[^3], wobei diese Daten verschlüsselt werden sollen, sowie einen Zeitstempel der Bestätigung.

Der Absender kann in den [vertraulichen Metadaten](../../Glossar.md#vertrauliche-metadaten) einer Nachricht eine Lesebestätigung anfordern. Der Client des Empfängers zeigt in diesem Fall einen entsprechenden Hinweis an, der die rechtliche Grundlage und Auswirkung der Bestätigung erläutert. Der Empfänger kann die Bestätigung senden, ist dazu aber nicht verpflichtet.

Als elektronisches Empfangsbekenntnis nach § 173 Abs. 3 ZPO ist die entworfene Lesebestätigung geeignet, wenn die beteiligte Person sie aktiv und zurechenbar abgibt. Das Empfangsdatum kann frei angegeben werden und wird nicht aus dem Zeitpunkt der Bestätigung abgeleitet.

#### Verhältnis zu ADR-0018

Die Bereitstellungsbestätigung ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)) bleibt der rechtlich notwendige Nachweis des Zugangs. Die hier definierten Bestätigungen ersetzen diese nicht.

| Bestätigungsart                              | Auslösung                 | Umsetzung                                    | Rechtliche Wirkung                                   |
|----------------------------------------------|---------------------------|----------------------------------------------|------------------------------------------------------|
| Bereitstellungsbestätigung ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md))        | Automatisch, serverseitig | Siegel des Betreibers (Backend Empfänger)    | Grundlage der Bekanntgabe- und Zustellfiktion        |
| Abrufbestätigung (Nachricht)                 | Automatisch, clientseitig | Verschlüsseltes Custom Event                 | Keine Fiktion, frei zu würdigendes Beweismittel      |
| Abrufbestätigung (Anhang)                    | Automatisch, clientseitig | Verschlüsseltes Custom Event mit Anhang-Hash | Keine Fiktion, frei zu würdigendes Beweismittel      |
| Lesebestätigung (Empfangsbekenntnis)         | Aktive Nutzerhandlung     | Verschlüsseltes Custom Event                 | Keine Fiktion, frei zu würdigendes Beweismittel      |
| Lesebestätigung als Empfangsbekenntnis (ZPO) | Aktive Nutzerhandlung     | Verschlüsseltes Custom Event, optional QES   | Elektronisches Empfangsbekenntnis (§ 173 Abs. 3 ZPO) |

![Ablauf der Bestätigungstypen](images/ADR-1118-flow.svg)

## Auswirkungen

- Clients müssen drei Event-Typen implementieren, nämlich das automatische Senden der Abrufbestätigung für Nachrichten, das automatische Senden der Abrufbestätigung für Anhänge sowie die Funktionalität für die aktive Lesebestätigung.

- Abruf- und Lesebestätigungen sind eine freiwillige Funktion des Clients und werden nicht über [Anschlussbedingungen](../../Glossar.md#anschlussbedingungen) erzwungen. Der zentrale [Postfach-Zugang](../../Glossar.md#postfach-zugang) bzw. das SDK implementieren die Abrufbestätigung.

## Risiken

- Unterdrückbarkeit der Abrufbestätigung: Ein modifizierter oder nicht-konformer Client kann die Abrufbestätigung unterdrücken. Eine technische Erzwingung ist nicht möglich, und der Verbund schreibt sie auch nicht vor.

- Sollte MSC4089 oder ein vergleichbarer MSC angenommen werden und verschlüsselte Bestätigungen nativ im Matrix-Protokoll einführen, ist zu prüfen, ob die hier definierten Custom Events durch das native Verfahren ersetzt werden können.

- Die Anhang-Abrufbestätigung setzt vollständigen Download und Entschlüsselung am Endgerät voraus. Sehr große Anhänge können auf ressourcenbeschränkten Geräten praktisch nicht vollständig abgerufen werden. In diesem Fall entsteht keine Bestätigung. Verfügbarkeits- und Bereitstellungsnachweise für Anhänge regelt eine Folge-Entscheidung.

## Folge-Entscheidungen

- **Absendergesteuerte Übermittlungsoptionen:** Steuerung pro Nachricht, ob eine Abruf- oder Lesebestätigung angefordert wird, sowie Berechtigungsprüfung des Absenders, siehe [ADR-1119](./ADR-1119-Absendergesteuerte-Übermittlungsoptionen-mit-Berechtigungsprüfung.md).

- **Beweissichere Übermittlung von Anhängen:** Verhältnis der Anhang-Abrufbestätigung zum Nachweis des erfolgreichen Empfänger-Downloads, siehe [ADR-1120](./ADR-1120-Beweissichere-Übermittlung-von-Anhängen.md).

## Offene Punkte

- Nachweis der Benachrichtigung über die Bereitstellung. An ihre Absendung knüpft § 37 Abs. 2a SGB X die Bekanntgabe.

## Referenzen

[^1]: [Matrix Spec, Receipts](https://spec.matrix.org/latest/client-server-api/#receipts), abgerufen am 13. Mai 2026.

[^2]: [Matrix Spec Proposal MSC4089: Delivery Receipts](https://github.com/matrix-org/matrix-spec-proposals/pull/4089), abgerufen am 13. Mai 2026.

[^3]: [Matrix Spec, Forming relationships between events](https://spec.matrix.org/latest/client-server-api/#forming-relationships-between-events), abgerufen am 13. Mai 2026.
