<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/ef99118c0607dd7fb11a005593569a36a0f84343/Zielarchitektur/Architekturentscheidungen/ADR-1147-Schluesselwiederherstellung.md -->
<!-- Branch: architecture-v1.1, Commit ef99118c06 (2026-11-13); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-1147-Schluesselwiederherstellung.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 5b77d8df3d); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# ADR-1147: Schlüsselwiederherstellung für E2EE-Postfächer

> **Status:** 🧭 Richtung entschieden | **Datum:** 13. November 2026

## Kontext und Problembeschreibung

In der Zielarchitektur liegen [Nachrichten](../../Glossar.md#nachricht) auf dem Server nur verschlüsselt vor ([ADR-0012](./ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md)). Geht das Endgerät und mit ihm die dort gespeicherten Schlüssel verloren, sind ältere Nachrichten ohne eine Wiederherstellung der Schlüssel nicht mehr lesbar. Dann lässt sich auch bei vorliegender [Bereitstellungsbestätigung](../../Glossar.md#bereitstellungsbestätigung) ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)) der Erhalt von Nachrichten abstreiten, was den Anforderungen an die [Bekanntgabe](../../Glossar.md#bekanntgabe) von [Verwaltungsakten](../../Glossar.md#verwaltungsakt) zuwiderläuft. Auch für die Akzeptanz des [Postfachs](../../Glossar.md#postfach) ist es notwendig, eine einfache und sichere Möglichkeit zu schaffen, Geräte zu wechseln und Nachrichten wiederherzustellen.

In dieser Architekturentscheidung werden Konzepte zur Schlüsselwiederherstellung im Matrix-basierten Kommunikationsnetzwerk der Zielarchitektur betrachtet und eine Auswahl getroffen.

Die unterschiedlichen Nutzergruppen stellen unterschiedliche Anforderungen an das System. Während [Privatpersonen](../../Glossar.md#privatperson) meist nur ein einziges Gerät nutzen, kaum Verständnis für die Technologie besitzen, lange Recovery-Keys im Allgemeinen ablehnen und das Verlustrisiko hier sehr hoch ist, finden Geräte- und Personalwechsel in Unternehmen häufig statt. Hier darf der Zugriff der [Organisation](../../Glossar.md#organisation) auf die Nachrichten nicht an einzelnen Personen oder Geräten hängen. Für größere Unternehmen und staatliche Organisationen wurde mit dem [Raum-Management-Service](../../Glossar.md#raum-management-service) bereits ein Konzept entworfen, das mit dem Ausfall einzelner Nutzender bzw. Geräte umgehen kann ([ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md), [ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)). [Technische Nutzer](../../Glossar.md#technischer-nutzer) halten Schlüssel gesichert vor, und die Berechtigungsverwaltung erlaubt die Konfiguration weiterer Zugänge.

Zu prüfen ist, ob es eine gemeinsame Lösung für alle [Teilnehmendengruppen](../../Glossar.md#teilnehmendengruppe) gibt, um Entwicklung und Support einfach zu halten. Diese muss in der breiten Masse durch wenig technikaffine [Nutzende](../../Glossar.md#nutzerinnen) einfach anwendbar sein, wobei zu beachten ist, dass Privatpersonen das Postfach meist nur selten nutzen. Ein Schlüsselverlust wird oft erst dann auffallen, wenn der Zugriff auf die Nachrichten dringend benötigt wird.

## Entscheidungstreiber

- **Zugang nach Geräteverlust:** Das System unterstützt eine Wiederherstellung des Zugangs nach Geräteverlust (SIC_VER_ÜBE_058).

- **E2EE-Konformität:** Betreiber der Infrastruktur und Dritte können den Klartext der Nachrichten nicht einsehen (SIC_VER_ÜBE_032).

- **Massentauglichkeit:** Die Wiederherstellung ist unabhängig von technischer Vorerfahrung und auch bei nur gelegentlicher Nutzung möglich (BED_BEN_ÜBE_008).

- **Schlüsselverwaltung:** Das System verwaltet kryptografische Schlüssel für Nutzende weitestgehend automatisch und transparent im Hintergrund. Eine manuelle Interaktion mit Schlüsseln ist nur in klar definierten Ausnahmefällen nötig (ANF_ARC_ARC_050).

- **Nichtverkettbarkeit:** Die [Bereichs-MXIDs](../../Glossar.md#bereichs-mxid) einer Person dürfen auch über die Wiederherstellungsschlüssel nicht zusammengeführt werden können. Globale Identifikatoren dürfen nicht verwendet werden (ANF_FUN_ADR_024).

- **Crypto-Agilität:** Das System setzt Crypto-Agilität um. Die betrachteten kryptografischen Verfahren sind austauschbar (SIC_VER_ÜBE_031).

- **Vertrauensniveau:** Die Re-Authentisierung erfolgt mindestens auf dem [Vertrauensniveau](../../Glossar.md#vertrauensniveau), mit dem der Zugang ursprünglich eingerichtet wurde.

## Betrachtete Optionen

### Option A: Serverseitiges Key Backup (Matrix Key Backup und SSSS)

Matrix bietet mit dem serverseitigen Key Backup[^2] die Möglichkeit, die Schlüssel der Nachrichten eines Matrix-Kontos gesammelt im [Postfach-Backend](../../Glossar.md#postfach-backend) aufzubewahren, damit beim Login eines neuen Gerätes vorherige Nachrichten unverzüglich entschlüsselt werden können. Der Client verschlüsselt das Backup, bevor er es ablegt. Den Schlüssel zum Entschlüsseln des Backups (Backup-Schlüssel) legt Secure Secret Storage and Sharing (SSSS)[^1] ebenfalls geschützt im Postfach-Backend ab. Geschützt ist er durch einen Wiederherstellungsschlüssel, den nur die Nutzenden kennen. Diesen geben sie auf einem neuen Gerät ein, womit der Client den Backup-Schlüssel erhält und das Backup öffnen kann. Dank der Verschlüsselung des Backups erhalten Betreiber der Infrastruktur und Dritte keinen Zugang zu den dort enthaltenen Schlüsseln der Kommunikation. Als Wiederherstellungsschlüssel dient ein zufällig erzeugter Schlüssel, den die Nutzenden im Passwort-Manager speichern oder ausdrucken können. Alternativ ist es möglich, ihn durch eine Password Based Key Derivation Function (PBKDF) aus einer selbst vergebenen Passphrase abzuleiten. Letzteres birgt die Gefahr, dass Nutzende die Passphrase zur Schlüsselwiederherstellung mit ihrem Account-Passwort verwechseln.

Neben der einfachen Nutzung eines zufälligen oder selbst gewählten Schlüssels ist es alternativ möglich, den Wiederherstellungsschlüssel aus einem Passkey (FIDO2) abzuleiten, um das Hindernis einer leicht zu vergessenden oder zu verlegenden Passphrase zu vermeiden (vgl. Offene Punkte).

Als mögliche Erweiterung könnte ein (t,n)-Schwellwertverfahren umgesetzt werden, bei dem der Schlüssel auf mehrere Personen verteilt wird und nur eine bestimmte Anzahl für die Wiederherstellung nötig ist (Secret Sharing). Gerade für Organisationen lässt sich hierdurch der Ausfall einzelner Träger der Wiederherstellungsschlüssel mitigieren.

- Gut, denn die Schlüssel werden im Client verschlüsselt, bevor sie im Postfach-Backend hinterlegt werden. Betreiber der Infrastruktur und Dritte können den Klartext der Nachrichten nicht einsehen (SIC_VER_ÜBE_032).

- Gut, denn mit dem Wiederherstellungsschlüssel wird auch die kryptografische Identität wiederhergestellt. Gesprächspartner müssen sie nicht erneut verifizieren.

- Gut, denn das Verfahren ermöglicht eine Wiederherstellung der Schlüssel durch Nutzende ohne Beteiligung Dritter.

- Gut, denn das Verfahren ist im Matrix-Standard spezifiziert und weit verbreitet, was die Implementierung vereinfacht.

- Schlecht, denn bei Verlust des Endgeräts sind ohne den Wiederherstellungsschlüssel bzw. die Passphrase auch die Nachrichten verloren. Der Wiederherstellungsschlüssel muss sorgfältig gesichert sein.

- Schlecht, denn der öffentliche Schlüssel des Backups liegt im Postfach-Backend. Um eine Verkettung von Accounts über [Bereiche](../../Glossar.md#bereich) hinweg zu verhindern, muss für jede Bereichs-MXID ein eigenes Backup mit eigenem Backup-Schlüssel verwendet werden.

### Option B: Re-Authentisierung und erneute Schlüsselweitergabe

Wenn die Schlüssel der kryptografischen [Identität](../../Glossar.md#identität) über andere Geräte oder das Backup nicht wiederherstellbar sind, kann für dieselbe [MXID](../../Glossar.md#mxid) eine neue kryptografische Identität erzeugt werden. Nach der erneuten Anmeldung am Postfach-Backend ([ADR-1110](./ADR-1110-Entkopplung-von-Nutzerkonto-und-Postfach.md)) muss die neue Identität in den Clients der Gesprächspartner verifiziert werden. Das geschieht wie bei den übermittelten Nachrichten über den [Identitätsnachweis](../../Glossar.md#fachlicher-identitätsnachweis), der an die neue Schlüsselidentität gebunden ist ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)). Anmeldung und Identitätsnachweis benötigen mindestens das Vertrauensniveau, mit dem der Zugang ursprünglich eingerichtet wurde. Die neue Identität erhält nun von den Gesprächspartnern wieder die Schlüssel neuer Nachrichten und, soweit noch vorhanden, auch die Schlüssel vorheriger Nachrichten. Matrix sieht die Weitergabe vorheriger Schlüssel derzeit nur beim Einladen in einen Raum vor (MSC4268)[^3].

- Gut, denn Nutzende müssen keinen Wiederherstellungsschlüssel vorhalten und sicher aufbewahren.

- Gut, denn eine Wiederherstellung ist allein über das [Nutzerkonto](../../Glossar.md#nutzerkonto) möglich ([ADR-1110](./ADR-1110-Entkopplung-von-Nutzerkonto-und-Postfach.md)).

- Schlecht, denn der Zugriff nach der Wiederherstellung ist nur auf jene Nachrichten möglich, für die die Gesprächspartner noch Schlüssel vorhalten. Die Wiederherstellung ist somit von den Gesprächspartnern abhängig.

- Schlecht, denn bei der [eID](../../Glossar.md#eid-funktion) ist für die Wiederherstellung die erneute [Authentifizierung](../../Glossar.md#authentifizierung) bei jedem Gesprächspartner bzw. Bereich notwendig.

- Schlecht, denn Matrix sieht die Weitergabe vorheriger Schlüssel an bisherige Mitglieder mit neuer kryptografischer Identität nicht direkt vor. Mit bestehender Implementierung wären ein Austritt und ein erneutes Einladen des betroffenen Mitglieds notwendig (vgl. MSC4268).

### Option C: Hinterlegung der Schlüssel bei zentraler Stelle (Key Escrow)

Eine zentrale Stelle kann die Wiederherstellungsschlüssel vorhalten, um sie nach einer Authentifizierung an die Nutzenden herauszugeben. Die Schlüssel könnten mittels (t,n)-Schwellwertverfahren auf mehrere Stellen aufgeteilt werden, so dass nicht eine Stelle allein die Wiederherstellung vornehmen kann.

- Gut, denn Nutzende müssen selbst keinen Wiederherstellungsschlüssel sicher aufbewahren.

- Schlecht, denn die den Schlüssel vorhaltenden Stellen können Zugriff auf den Klartext der Nachrichten erlangen. Auch bei einer Aufteilung des Schlüssels auf mehrere Stellen kann dies nicht ausgeschlossen werden, im Besonderen wenn für die Speicherung staatliche Infrastruktur verwendet wird.

## Architekturentscheidung

Gewählte Optionen: **Option A (Serverseitiges Key Backup)** als Grundlage und **Option B (Re-Authentisierung und erneute Schlüsselweitergabe)** als nachrangige Möglichkeit, weil das Key Backup eine Wiederherstellung der Schlüssel durch Nutzende ohne Beteiligung Dritter ermöglicht und dabei die kryptografische Identität erhalten bleibt. Das Key Backup wird durch Option B ergänzt, da der Wiederherstellungsschlüssel sorgfältig gesichert sein muss und dies nicht allen Nutzenden gelingen wird. Bei Option B müssen Nutzende keine Schlüssel vorhalten und sicher aufbewahren. Vorherige Nachrichten werden dann aber nur lesbar, soweit Gesprächspartner die Schlüssel noch besitzen. Bei der Kommunikation mit [öffentlichen Stellen](../../Glossar.md#öffentliche-stelle) kann von einer langen Vorhaltefrist der Schlüssel ausgegangen werden.

**Option C wird ausgeschlossen**, da sie dem Paradigma der [Ende-zu-Ende-Verschlüsselung](../../Glossar.md#ende-zu-ende-verschlüsselung) widerspricht (SIC_VER_ÜBE_032). Die Verwahrung des Wiederherstellungsschlüssels bleibt damit als manuelle Interaktion mit Schlüsseln nötig (ANF_ARC_ARC_050).

Nutzende mit mehreren Geräten können mittels Key Sharing zwischen per Cross-Signing verifizierten Geräten die Schlüssel zwischen den Geräten übertragen. Sofern nur ein Gerät verloren ist, bleiben sie im Besitz der Schlüssel.

Organisationen gleichen den Ausfall einzelner Geräte oder Mitarbeitender durch eigene Prozesse und Infrastruktur aus. Sie verwahren die Wiederherstellungsschlüssel sicher, etwa aufgeteilt auf mehrere Personen. Zudem gewährleisten die technischen Nutzer des Raum-Management-Service mit Schlüsselzugriff die Verfügbarkeit der Schlüssel in den Räumen ([ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md), [ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)). Verliert eine einzelne Person ihre Schlüssel, bleibt der Zugriff der Organisation über die anderen Mitglieder des Raums erhalten.

Privatpersonen richten das Key Backup verpflichtend bei der Ersteinrichtung des [Postfach-Zugangs](../../Glossar.md#postfach-zugang) ein. Für jede Bereichs-MXID wird ein eigenes Backup mit eigenem Backup-Schlüssel angelegt, damit die Accounts im Postfach-Backend nicht verkettet werden können ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md)). Die Backup-Schlüssel aller Accounts einer Person sichert ein gemeinsamer, zufällig erzeugter Wiederherstellungsschlüssel. Diesen kann die Person im Schlüsselbund des Betriebssystems, in einem Passwort-Manager oder als Papierausdruck verwahren.

![Schlüsselhierarchie des Key Backups einer Privatperson](images/ADR-1147-schluesselhierarchie.svg)

## Auswirkungen

- Privatpersonen müssen bei der Ersteinrichtung des Postfach-Zugangs das Key Backup verpflichtend einrichten und anleitende Unterstützung für die Verwahrung des Wiederherstellungsschlüssels erhalten.

- Im Client (Postfach-Zugang) muss ein vorhandener Wiederherstellungsschlüssel für weitere Bereichs-MXIDs verwendet werden. Die im Postfach-Backend für den Wiederherstellungsschlüssel vorgehaltenen Daten wie z. B. die Schlüsselkennung müssen für jeden Account zufällig erzeugt worden sein, um eine Verkettung zu verhindern.

- Die neue Schlüsselidentität (Option B) hat keine Auswirkung auf das [Holder-Binding](../../Glossar.md#holder-binding) der Identitätsnachweise zuvor versandter Nachrichten. Diese Identitätsnachweise bleiben für den Versandzeitpunkt gültig ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)).

## Risiken

- Bei Privatpersonen ist mit häufigem Verlust des Wiederherstellungsschlüssels zu rechnen, wobei dies oft erst dann auffallen wird, wenn der Zugriff auf die Nachrichten dringend benötigt wird.

- Nach einer Re-Authentisierung sind vorherige Nachrichten nur lesbar, soweit Gesprächspartner die Schlüssel erneut teilen bzw. teilen können.

- Wird der Wiederherstellungsschlüssel kompromittiert, sind die Backups aller Accounts der Person für jeden lesbar, der Zugriff auf die Backups hat.

- Bei Option B kann ein gemeinsamer Betreiber von Postfach-Backend und Nachweisstelle eine neue kryptografische Identität erzeugen und so vorherige Nachrichten lesen.

- Mit dem Wechsel auf MLS sind das Key Backup und die Weitergabe vorheriger Schlüssel neu zu bewerten ([ADR-0012](./ADR-0012-Ende-zu-Ende-Verschlüsselungsschicht.md)).

## Offene Punkte

- Bei Geräteverlust muss es möglich sein, den Zugang des verlorenen Geräts zu sperren.

- Detaillierte UX bzw. Beschreibung von User-Stories zu Key Backups und Wiederherstellungsschlüssel.

- Ableitung des Wiederherstellungsschlüssels aus einem Passkey (FIDO2).

- Prozess zum Identitätsnachweis mit dem vom [Absender](../../Glossar.md#absenderin) festgelegten Vertrauensniveau, bevor Gesprächspartner die Schlüssel neuer oder vorheriger Nachrichten mit der neuen Schlüsselidentität teilen (vgl. [ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)).

- Nach Austritt und erneuter Einladung kann das einladende Mitglied die Schlüssel an den Eingeladenen weitergeben, sofern der Raum die Historie freigibt (MSC4268).[^3] Lädt ein technischer Nutzer ohne Schlüssel ein, ist das noch offen ([ADR-1109](./ADR-1109-Raumtopologie-Funktionspostfaecher.md)).

- Umgang mit nicht entschlüsselbaren Nachrichten und Auswirkung auf die Bekanntgabe, etwa erneute Übermittlung bei ausbleibender [Abrufbestätigung](../../Glossar.md#abrufbestätigung) ([ADR-1118](./ADR-1118-Abruf-und-Lesebestätigungen.md), [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

## Referenzen

[^1]: [Matrix Spec, Secret storage](https://spec.matrix.org/latest/client-server-api/#secret-storage), abgerufen am 30. September 2026.

[^2]: [Matrix Spec, Server-side key backups](https://spec.matrix.org/latest/client-server-api/#server-side-key-backups), abgerufen am 30. September 2026.

[^3]: [Matrix Spec Proposal MSC4268, Sharing room keys for past messages (seit Matrix 1.19)](https://github.com/matrix-org/matrix-spec-proposals/pull/4268), abgerufen am 9. Oktober 2026.
