<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Zielarchitektur/Architekturentscheidungen/ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob cae137b8cc); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# ADR-1105: Pseudonyme Adressen von Privatpersonen

> **Status:** 🧭 Richtung entschieden | **Datum:** 12. November 2026

## Kontext und Problembeschreibung

[Privatpersonen](../../Glossar.md#privatperson) sollen in der Zielarchitektur über ihre Adressen nicht eindeutig identifiziert werden können. Stattdessen sollen [Pseudonyme](../../Glossar.md#pseudonym) verwendet werden, die für die Kommunikation innerhalb eines [Bereichs](../../Glossar.md#bereich) stabil bleiben. So soll eine bereichsübergreifende Verkettung der Kommunikation von Privatpersonen über eindeutige Identifier verhindert werden. Was ein Bereich konkret umfasst, etwa eine [Behörde](../../Glossar.md#behörde) oder ein Verwaltungsbereich, ist noch zu prüfen und festzulegen (vgl. Offene Punkte).

Für die Adressierung im [Verbund](../../Glossar.md#verbund) werden die [MXIDs](../../Glossar.md#mxid) verwendet. Sie enthalten neben der Domain des Backends einen Localpart ([ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md)). Die Zielarchitektur sieht pseudonyme Localparts vor. Die Domain ergibt sich über das zentrale [Postfach-Backend](../../Glossar.md#postfach-backend) für Privatpersonen. In Matrix sind die Adressen Bestandteile der unverschlüsselten [Adressierungsmetadaten](../../Glossar.md#adressierungsmetadaten), was hinsichtlich der Privatsphäre und des Datenschutzes Herausforderungen mit sich bringt, da die Postfach-Betreiber diese Daten einsehen können. Unverschlüsselt sind neben den MXIDs aller Mitglieder eines Raums auch die Geräte-ID und der Geräteschlüssel des [Absenders](../../Glossar.md#absenderin). Für eine Privatperson darf daher in verschiedenen Bereichen keine gemeinsame Kennung entstehen, weder in der MXID noch bei Geräte-ID und Schlüsseln. Dies gilt ebenso für die Kennungen der [Authentisierungssysteme](../../Glossar.md#authentisierungssystem).

Diese Architekturentscheidung betrachtet Möglichkeiten, wie diese pseudonymen Adressen gebildet werden und dabei der [Identität](../../Glossar.md#identität) der Privatperson eindeutig zugeordnet sind, wobei eine Stabilität der Adresse bei der Kommunikation mit denselben Stellen erreicht werden muss, da das [Postfach](../../Glossar.md#postfach) und die Mitgliedschaft in den [Vorgangsräumen](../../Glossar.md#vorgangsraum) an die MXID gebunden sind. Die Zielarchitektur unterstützt für Privatpersonen die Authentisierungssysteme [BundID](../../Glossar.md#bundid), [EUDI-Wallet](../../Glossar.md#europäische-brieftasche-für-die-digitale-identität) und [eID](../../Glossar.md#eid-funktion), was zu Herausforderungen bzgl. der Bildung einer einheitlichen bereichsspezifischen Adresse führt. Damit alle Authentisierungssysteme zum selben Postfach führen können, kann das Postfach kein Bestandteil eines einzelnen [Nutzerkontos](../../Glossar.md#nutzerkonto) sein (vgl. Folge-Entscheidung).

Auf jedem dieser Authentisierungssysteme werden die Pseudonyme auf unterschiedliche Art und Weise erzeugt.

| Authentisierungssystem | Pseudonym                  | Erzeuger    |
|-----------------------|----------------------------|-------------|
| eID-Server (direkt)   | Restricted ID je Sektor    | Ausweis     |
| BundID                | bPK2 je [Onlinedienst](../../Glossar.md#onlinedienst)       | BundID      |
| EUDI-Wallet           | Pseudonym je Relying Party | EUDI-Wallet |

Keine dieser Kennungen bleibt über alle Authentisierungssysteme oder einen Ausweiswechsel hinweg stabil. Die Restricted ID (sektorspezifischer Identifier) der eID ist abhängig vom Berechtigungszertifikat des eID-Servers der Gegenstelle und ändert sich mit einem neuen Ausweis. Das Pseudonym der BundID (bPK2) ist abhängig vom Onlinedienst. Das Pseudonym der EUDI-Wallet ist für jede Relying Party verschieden.[^1]

Eine Kennung, die eine Person je Bereich über alle Authentisierungssysteme und einen Ausweiswechsel hinweg eindeutig bezeichnet, gibt es derzeit nicht. Diese Architekturentscheidung begründet zunächst, warum bereichsspezifische Adressen umzusetzen sind (Teil 1), und beantwortet dann die Frage, wo eine Zuordnung dieser Adressen zu den Authentisierungssystemen erfolgt (Teil 2).

## Entscheidungstreiber

- **Nichtverkettbarkeit:** Global eindeutige Identifikatoren dürfen nicht verwendet werden (ANF_FUN_ADR_024). Kein Betreiber allein soll die Adressen einer Person in verschiedenen Bereichen zusammenführen und mit ihren Kommunikationsbeziehungen verknüpfen können.
- **Pseudonyme:** Privatpersonen ist es möglich, unter einem Pseudonym zu kommunizieren (ANF_FUN_ADR_085).
- **Datenschutz:** Die Zielarchitektur setzt das Prinzip Privacy by Design sowie technische und organisatorische Schutzmaßnahmen um (DAT_ÜBE_ÜBE_021, DAT_ÜBE_ÜBE_022).
- **Authentisierungssysteme:** Privatpersonen können sich mit BundID, eID und EUDI-Wallet authentifizieren (ANF_FUN_IDE_023, [ADR-0014](./ADR-0014-Authentizität-Identität-von-Privatpersonen.md)).
- **Zentraler Postfach-Zugang:** Privatpersonen ist es möglich, über einen einzigen [Postfach-Zugang](../../Glossar.md#postfach-zugang) mit allen Bereichen der [öffentlichen Stellen](../../Glossar.md#öffentliche-stelle) zu kommunizieren (RAH_ÜBE_144).
- **Adressstruktur:** Die Localparts der Adressen natürlicher Personen sind Pseudonyme ([ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md)).
- **Postfachzuordnung:** Unabhängig vom Authentisierungssystem und auch nach dem Wechsel des Ausweises müssen die Postfachadressen im jeweiligen Bereich stabil bleiben, da sonst mehrere Postfächer entstehen und [Vorgänge](../../Glossar.md#vorgang) jeweils nur über das initiierende Authentisierungssystem erreichbar wären.

## Betrachtete Optionen

Die Optionen sind in Teil 1 und Teil 2 gegliedert. In Teil 1 wird der Geltungsbereich der MXID bewertet, also global, je Bereich oder je Vorgang. In Teil 2 werden Optionen betrachtet, wie unterschiedliche Authentisierungssysteme zum selben Postfach führen.

### Teil 1: Geltungsbereich der MXID

#### Option 1A: Globale MXID

Die Privatperson erhält nur eine Adresse in der Zielarchitektur. Jede Behörde nutzt diese eine Adresse, womit eine Verkettung der Kommunikation durch Abgleich der Behördendaten möglich wäre.

- Gut, denn für eine Privatperson existiert nur eine gültige Adresse, was Aufwände spart und eine gute Benutzbarkeit einfach umsetzbar macht.

- Schlecht, denn die Daten einer Privatperson in den Behörden wären über diesen globalen Identifier verkettbar, was gravierende Probleme hinsichtlich der Privatsphäre und des Datenschutzes mit sich brächte.

- Schlecht, denn die MXID liegt bei jeder [Nachricht](../../Glossar.md#nachricht) und in der Mitgliederliste jedes Raums unverschlüsselt im Postfach-Backend vor. Eine Profilbildung der Privatperson durch Betreiber ließe sich weder durch Protokollierungsverbote noch separate Postfach-Backends für öffentliche Stellen verhindern.

- Schlecht, denn Fachportale unterschiedlicher Bereiche können in integrierten Postfach-Zugängen nur mittels Client-seitiger Filterung erreichen, dass nicht auch Nachrichten anderer Bereiche angezeigt werden.

#### Option 1B: Bereichsspezifische MXIDs

Die Privatperson erhält für jeden Bereich eine eigene pseudonyme Adresse ([Bereichs-MXID](../../Glossar.md#bereichs-mxid)) in der Zielarchitektur. Die Stellen in separaten Bereichen nutzen somit unterschiedliche Adressen derselben Privatperson, womit eine Verkettung der Kommunikation durch Abgleich der Behördendaten nicht möglich wäre.

- Gut, denn die Verkettung der Kommunikation von Privatpersonen über Bereiche hinweg ist durch einen Datenabgleich zwischen Stellen der unterschiedlichen Bereiche nicht möglich.

- Gut, denn wenn öffentliche Stellen ihre Postfächer in eigenen Backends mit eigener Domain betreiben, kann der Betreiber des Privatpersonen-Backends die Kommunikation nicht über die Adressen zu einem Profil verknüpfen.

- Gut, denn Fachportale unterschiedlicher Bereiche können in integrierten Postfach-Zugängen nur die relevanten dem Bereich zugehörigen Nachrichten anzeigen, ohne dass eine Filterung erforderlich ist.

- Schlecht, denn im zentralen Postfach-Zugang müssen die Postfächer aller Bereiche mit allen Bereichs-MXIDs der Person zusammen dargestellt werden. Das erhöht den Entwicklungsaufwand für den Client und kann die Benutzbarkeit verschlechtern.

- Schlecht, denn die Authentisierungssysteme liefern unterschiedliche Kennungen. Je Bereich ist deshalb eine Zuordnung zur Bereichs-MXID notwendig (vgl. Teil 2).

#### Option 1C: Vorgangsspezifische MXIDs

Für jeden Vorgang werden auf Behördenseite und auf der Seite der Privatperson eigene pseudonyme MXIDs erstellt. Diese werden nicht in anderen Vorgängen verwendet. Jede Seite meldet sich an einem Hauptkonto an und wird gegen dieses verifiziert. Die vorgangsspezifischen MXIDs gehören zu eigenen Konten (Puppet-Accounts), die der Client des Hauptkontos stellvertretend nutzt. Ihre Zugänge sind in den verschlüsselten Kontodaten des Hauptkontos gespeichert. Über eine Server-Erweiterung erlaubt der [Homeserver](../../Glossar.md#homeserver) einem Hauptkonto, Puppet-Accounts anzulegen und hinter einer oder mehreren eindeutigen Kennungen zu hinterlegen. Die Kennung muss der Behörde bekannt sein und wurde z. B. im [Antrag](../../Glossar.md#antrag) übermittelt. Um einen Vorgang zu starten, fragt ein Homeserver beim anderen einen neuen Puppet-Account für diese Kennung an und verwendet ihn als Endpunkt. Der Homeserver gibt jeden Puppet-Account genau einmal heraus und löscht danach die Zuordnung zur Kennung.

- Gut, denn jeder Vorgang erhält eine eigene Adresse, sodass Vorgänge einer Person nicht über ihre Adressen verknüpft werden können.

- Schlecht, denn die Puppet-Accounts kann nur der Client im Voraus anlegen. Ist dieser Vorrat erschöpft, kann bis zur nächsten Anmeldung der Person kein Vorgang mit ihr eröffnet werden.

- Schlecht, denn der Client muss für jeden Vorgang ein eigenes Konto synchronisieren. Bei vielen Vorgängen erhöht das die Last auf Gerät und Server.

- Schlecht, denn die Zuordnung liegt nur clientseitig verschlüsselt vor und kann nach einem Schlüsselverlust nicht wiederhergestellt werden.

- Schlecht, denn die Lösung erfordert eine Erweiterung der Homeserver, die die Matrix-Spezifikation nicht vorsieht.

- Schlecht, denn der Betreiber des Homeservers kennt die Zuordnung der hinterlegten Puppet-Accounts zum Hauptkonto, bis die MXID eines Puppet-Accounts für einen neuen Vorgang verwendet wird. Die anschließende Löschung der Zuordnung kann nur organisatorisch gesichert werden.

- Schlecht, denn die Kennung ist im Grunde eine Adresse. Um eine Verkettung über verschiedene Bereiche hinweg zu verhindern, wären bereichsspezifische Kennungen notwendig.

### Teil 2: Zuordnung der Authentisierungssysteme zum Postfach

Die Optionen in Teil 2 beschreiben Lösungen, um bei Nutzung aller Authentisierungssysteme dieselben Postfächer bzw. Bereichs-MXIDs einer Person zuzuordnen. Es wird betrachtet, wie sich ein Wechsel des Ausweises auswirkt und wie die Postfächer der Bereiche im zentralen Postfach-Zugang gemeinsam dargestellt werden können.

#### Option 2A: Pseudonym-Erzeugung ausschließlich über BundID

Eine [Authentifizierung](../../Glossar.md#authentifizierung) wird nur über die BundID zugelassen. Dies ist möglich, da die BundID alle notwendigen [Authentisierungsmittel](../../Glossar.md#authentisierungsmittel) unterstützt, also eID und ELSTER-Zertifikat und künftig auch die EUDI-Wallet.[^2] Eine Anmeldung mit eID oder EUDI-Wallet direkt bei der Gegenstelle im Bereich ist so nicht möglich.

- Gut, denn die Pseudonymerzeugung erfolgt an genau einer Stelle.

- Schlecht, denn eID und EUDI-Wallet können nicht direkt für die [Authentisierung](../../Glossar.md#authentisierung) gegenüber der Gegenstelle genutzt werden. Die Anmeldung hängt damit vollständig von der Verfügbarkeit der BundID ab.

- Schlecht, denn die Postfachadresse hängt am Pseudonym der BundID. Eine Änderung des Verfahrens zur Bildung der Pseudonyme würde alle bestehenden Adressen verändern.

- Schlecht, denn BundID kann alle Bereichspseudonyme einer Person zuordnen.

#### Option 2B: BundID stellt bereichsspezifische Pseudonyme als Verifiable Credentials in die EUDI-Wallet aus

BundID erzeugt einmalig je Bereich ein Pseudonym, signiert es als Verifiable Credential und legt es in die EUDI-Wallet ab. Die Wallet nutzt dann dieses Pseudonym bei der Anmeldung, wo daraus dann die Postfachadresse gebildet wird.

- Gut, denn die Anmeldung über EUDI-Wallet und BundID liefert für denselben Bereich dasselbe Pseudonym.

- Schlecht, denn bei der direkten Anmeldung mit der eID wird die Restricted ID des nPA verwendet und nicht das Pseudonym der BundID, weshalb eine Zuordnung notwendig wäre.

- Schlecht, denn die Anmeldung über die EUDI-Wallet benötigt je Bereich ein Pseudonym der BundID. Dieses muss im Voraus für alle Bereiche bzw. vor der ersten Nutzung eines Bereichs ausgestellt werden.

- Schlecht, denn die Postfachadresse hängt am Pseudonym der BundID. Eine Änderung des Verfahrens zur Bildung der Pseudonyme würde alle bestehenden Adressen verändern.

- Schlecht, denn BundID kann alle Bereichspseudonyme einer Person zuordnen.

#### Option 2C: Gemeinsame Pseudonyme aus einem zentral vergebenen, personengebundenen Schlüssel

Eine zentrale Stelle vergibt bei der Registrierung der Person einen personengebundenen Schlüssel, der nicht am Ausweis hängt. Das Pseudonym eines Bereichs wird als Hashwert aus diesem Schlüssel und der Bereichskennung gebildet. Somit liefern alle Authentisierungssysteme für einen Bereich dasselbe Pseudonym. In Österreich wird das bereichsspezifische Personenkennzeichen so aus der Stammzahl gebildet, die sich aus dem Zentralen Melderegister ableitet.[^3]

- Gut, denn dieselbe Person erhält über jedes Authentisierungssystem dasselbe bereichsspezifische Pseudonym, auch nach einem Ausweiswechsel.

- Gut, denn mit bPk#[^4] liegt ein dezentrales Verfahren vor, mit dem sich die Option umsetzen ließe. Pseudonyme werden dort lokal auf dem Endgerät der [Nutzenden](../../Glossar.md#nutzerinnen) berechnet. Eine zentrale Stelle ist daran nicht beteiligt und erfährt somit nicht, welche Bereiche von wem genutzt werden.

- Schlecht, denn die zentrale Stelle gibt die Schlüssel der Personen heraus. Sie kann deshalb die Pseudonyme einer Person über alle Bereiche verknüpfen und der Person zuordnen.

- Schlecht, denn Betreiber bzw. Entwickler von BundID, eID und EUDI-Wallet müssten sich abstimmen und dieses Verfahren einheitlich umsetzen.

#### Option 2D: Verschiedene Postfächer je Authentisierungssystem akzeptieren

Pro Person, Bereich und Authentisierungssystem entsteht ein eigenes Postfach mit eigenem Pseudonym. Privatpersonen müssen sich für ihre Postfächer über die unterschiedlichen Authentisierungssysteme separat anmelden.

- Gut, denn es wäre keine Änderung an bestehenden Identitätssystemen erforderlich.

- Schlecht, denn die Benutzbarkeit wäre massiv gestört. Nutzende müssten sich im gleichen Bereich über unterschiedliche Authentisierungssysteme anmelden, um alle ihre Nachrichten einzusehen.

- Schlecht, denn bei einem Ausweiswechsel entstünde ein neues Postfach. Das bisherige wäre über die neue eID nicht mehr erreichbar.

#### Option 2E: Zuordnungsdienst je Bereich

Authentisiert sich eine Person in einem Bereich das erste Mal, wird eine zufällige Bereichs-MXID erzeugt. Für den jeweiligen Bereich verknüpft ein [Zuordnungsdienst](../../Glossar.md#zuordnungsdienst) die Bereichs-MXID mit der Kennung des gewählten Authentisierungssystems, also mit dem Pseudonym der BundID oder der EUDI-Wallet oder mit der Restricted ID der eID. Dafür ist der Zuordnungsdienst für den Bereich bei den Authentisierungssystemen registriert. Er verfügt also über ein Berechtigungszertifikat mit eigenem Sektor für die eID und ist als Dienst bei der BundID sowie als Relying Party der EUDI-Wallet registriert. Dadurch erhält er Kennungen, die nur in diesem Bereich gelten. Bei der EUDI-Wallet setzt das voraus, dass er keine Attestierung anfordert, die für die Wallet eindeutig ist.

Bei einer Anmeldung über andere Authentisierungssysteme werden auch diese mit der bestehenden Bereichs-MXID verknüpft, womit für einen Bereich keine zweite Bereichs-MXID der Person entsteht. Die Verknüpfung ist in einem angemeldeten Postfach-Zugang möglich, indem sich die Person über das neue Authentisierungssystem authentisiert. Ist die Person noch nicht angemeldet, muss sie sich für die Verknüpfung nacheinander über das bestehende und das neue Authentisierungssystem authentisieren. Vor dem Anlegen einer neuen Bereichs-MXID wird der Person die Verknüpfung mit einer bestehenden Bereichs-MXID angeboten.

Der Zuordnungsdienst vermittelt wie ein ID-Broker zwischen den Authentisierungssystemen und dem Postfach-Backend. Gegenüber Postfach-Zugang und Postfach-Backend weist er nur die Bereichs-MXID und keine Identitätsmerkmale der Person aus.

Im zentralen Postfach-Zugang muss sich die Person für jeden Bereich auf jedem Gerät einmal gesondert anmelden. Besteht bereits eine Sitzung in der BundID, kann die erneute Anmeldung dort entfallen. Bei Verwendung der eID ist je Bereich die PIN einzugeben. Die Sitzung am Postfach-Backend bleibt danach auf dem Gerät erhalten, bis sie abläuft.

Der Zuordnungsdienst kann von einer Stelle des Bereichs selbst betrieben werden. Zudem ist ein gemeinsamer Betrieb der Zuordnungsdienste mehrerer Bereiche möglich, etwa als Bestandteil des zentralen Postfach-Backends oder auch als separater Dienst. Da jeder Bereich dabei seine eigenen Registrierungen bei den Authentisierungssystemen behält, kann auch der gemeinsame Betreiber die Bereiche einer Person nicht über die Kennungen verknüpfen.

![Option 2E: Zuordnungsdienst je Bereich](images/ADR-1105-option-2e.svg)

- Gut, denn alle unterstützten Authentisierungssysteme lassen sich mit demselben Postfach verknüpfen, sodass je Bereich nur ein Postfach entsteht.

- Gut, denn die Bereichs-MXID ist nicht vom Authentisierungssystem abhängig. Ein neuer Ausweis ändert nur die Zuordnung und nicht die Adresse. Auch ein Pseudonym nach Option 2C kann später als weitere Kennung zugeordnet werden.

- Gut, denn keine Stelle kennt alle Bereichs-MXIDs einer Person. Deshalb kann auch keine Stelle sie über die Kennungen der Authentisierungssysteme zusammenführen.

- Gut, denn werden das Postfach-Backend und der Zuordnungsdienst von unterschiedlichen Stellen betrieben, kennt der Betreiber des Postfach-Backends nur die Bereichs-MXIDs. Er erhält keine Kennungen der Authentisierungssysteme, über die er die Person identifizieren könnte.

- Schlecht, denn der Zuordnungsdienst verursacht Mehraufwand in Betrieb und Entwicklung.

- Schlecht, denn zur Verknüpfung eines weiteren Authentisierungssystems ist einmalig eine zusätzliche Anmeldung über ein bereits verknüpftes Authentisierungssystem notwendig, sofern die Person nicht schon in einem Postfach-Zugang angemeldet ist.

- Schlecht, denn der Aufwand für die Registrierungen des Zuordnungsdienstes bei der eID, der BundID und der EUDI-Wallet steigt mit der Zahl der Bereiche.

- Schlecht, denn die Liste der Bereiche, in denen die Person ein Postfach hat, wird nicht zentral bereitgestellt und muss vom Client selbst vorgehalten werden.

#### Option 2E’: Zentraler Zuordnungsdienst

Im Gegensatz zu Option 2E wird nur ein Zuordnungsdienst umgesetzt, der die Bereichs-MXIDs aller Bereiche verwaltet. Eine Person erhält nach einer Anmeldung automatisch Zugriff auf die Postfächer aller ihrer Bereiche. Auch die Anmeldung in Fachportalen läuft über diesen zentralen Dienst.

Umgesetzt werden kann der Zuordnungsdienst in einer zentralen Authentifizierungskomponente des D-Stacks, etwa in der BundID. Einen von der Behörde geforderten [Identitätsnachweis](../../Glossar.md#fachlicher-identitätsnachweis) erbringt die Person unabhängig davon direkt mit der eID oder der EUDI-Wallet ([ADR-0014](./ADR-0014-Authentizität-Identität-von-Privatpersonen.md), vgl. Folge-Entscheidung).

Der zentrale Zuordnungsdienst kennt alle Bereichs-MXIDs einer Person. Er darf deshalb nicht vom Betreiber des Postfach-Backends betrieben werden, da dieser sonst die Kommunikationsbeziehungen der Person über alle Bereiche verknüpfen könnte.

![Option 2E’: zentraler Zuordnungsdienst](images/ADR-1105-option-2e-zentral.svg)

- Gut, denn wie bei Option 2E führen alle Authentisierungssysteme je Bereich zum selben Postfach. Die Bereichs-MXID hängt an keinem Authentisierungssystem, und der Betreiber des Postfach-Backends erhält keine Kennungen der Authentisierungssysteme.

- Gut, denn nach einer Anmeldung erhält der zentrale Postfach-Zugang alle Bereichs-MXIDs der Person, auch aus Bereichen, auf die bisher nur über ein Fachportal zugegriffen wurde.

- Gut, denn der Zuordnungsdienst muss bei eID, BundID und EUDI-Wallet nur einmal registriert werden.

- Schlecht, denn der zentrale Zuordnungsdienst kennt alle Bereichs-MXIDs einer Person und erfährt bei jeder Anmeldung den genutzten Bereich.

- Schlecht, denn übernimmt die BundID die Aufgabe des zentralen Zuordnungsdienstes, ist eine Verkettung der Identität einer Person mit allen ihren Bereichs-MXIDs ohne Weiteres möglich.

- Schlecht, denn wie bei Option 2E erfordert die Verknüpfung eines weiteren Authentisierungssystems einmalig eine zusätzliche Anmeldung über ein bereits verknüpftes Authentisierungssystem, sofern die Person nicht schon in einem Postfach-Zugang angemeldet ist.

### Optionsvergleich Teil 2

Die Spalten zu eID-Server, BundID und EUDI-Wallet geben an, ob das jeweilige Authentisierungssystem für die Anmeldung am Postfach genutzt werden kann. Die letzte Spalte nennt die Stelle, die die Pseudonyme einer Person über alle Bereiche verknüpfen kann.

| Option | eID-Server | BundID | EUDI-Wallet | Zentrale Verknüpfung                    |
|--------|------------|--------|-------------|-----------------------------------------|
| 2A     | nein       | ja     | nein        | BundID                                  |
| 2B     | nein       | ja     | ja          | BundID                                  |
| 2C     | ja         | ja     | ja          | Stelle mit personengebundenem Schlüssel |
| 2D     | ja         | ja     | ja          | keine, aber kein gemeinsames Postfach   |
| 2E     | ja         | ja     | ja          | keine                                   |
| 2E’    | ja         | ja     | ja          | zentraler Zuordnungsdienst              |

## Architekturentscheidung

**Teil 1: gewählt wird Option 1B (bereichsspezifische MXIDs)**, weil Behörden unterschiedlicher Bereiche die Kommunikation einer Person nicht über ihre Adresse zusammenführen können und die Separierung der Backends für öffentliche Stellen und für Privatpersonen auch die Verkettung über die Adressen in den Backends erschwert (vgl. Folge-Entscheidung). Eine Verkettung über Metadaten der unteren Netzwerkebenen (z. B. IP-Adressen) wird damit nicht verhindert. Hierfür ist eine zusätzliche Minimierung der Metadaten erforderlich (vgl. Folge-Entscheidung).

**Option 1A (globale MXID) wird nicht gewählt**, da eine globale MXID die Kommunikation einer Person über alle Bereiche verkettbar macht.

**Option 1C (vorgangsspezifische MXIDs) wird zunächst nicht gewählt**, weil für die Kontenzuordnung nach einem Schlüsselverlust noch keine Lösung existiert (vgl. Offene Punkte) und die Löschung der Zuordnung beim Homeserver nur organisatorisch gesichert werden kann. Darüber hinaus müssten die Kennungen ebenso wie die Adressen nach Option 1B bereichsspezifisch sein, sodass Option 1C die Bereichstrennung nicht ersetzt.

**Teil 2: gewählt werden Option 2E (Zuordnungsdienst je Bereich) und Option 2E’ (zentraler Zuordnungsdienst) als Alternativen**, weil alle Authentisierungssysteme je Bereich zum selben Postfach führen und kein Betreiber allein die Bereichs-MXIDs einer Person mit ihren Kommunikationsbeziehungen verknüpfen kann. Die Bereichs-MXID ist nicht vom genutzten Authentisierungssystem abhängig. Ein neuer Ausweis ändert die Adresse nicht. Postfach-Zugang und Postfach-Backend erhalten vom Zuordnungsdienst nur die Bereichs-MXID und keine Identitätsmerkmale der Person. Der Zuordnungsdienst kann nicht als [Adressregister](../../Glossar.md#adressregister) zur Suche von Postfachadressen genutzt werden.

![Anmeldung und Verknüpfung am Beispiel von Option 2E](images/ADR-1105-anmeldung-verknuepfung.svg)

Entschieden ist damit, dass ein Zuordnungsdienst die Kennungen der Authentisierungssysteme der Bereichs-MXID zuordnet. Offen bleibt, ob dieser Dienst je Bereich oder zentral umgesetzt wird. Wird der Bereich als Verwaltungsbereich definiert, wie ihn RAH_ÜBE_144 beschreibt, ist eine Umsetzung von Option 2E vertretbar, bei der für jeden Bereich eine Registrierung des Zuordnungsdienstes notwendig ist. Gibt es eine große Anzahl von Bereichen, etwa wenn jede Behörde ein eigener Bereich ist, ist eine Umsetzung von Option 2E’ realistischer, da der Zuordnungsdienst dort nur einmal je Authentisierungssystem registriert werden muss. Die Festlegung des Umfangs der Bereiche ist noch zu treffen (vgl. Offene Punkte). Grundsätzlich ist auch eine Kombination der Optionen möglich, bei der die Anmeldung für einzelne Behörden über den zentralen Zuordnungsdienst erfolgt. Das Postfach-Backend nimmt in jedem Fall nur Bereichs-MXIDs eines zugelassenen Zuordnungsdienstes und nur für dessen Bereiche entgegen.

Da der zentrale Zuordnungsdienst (Option 2E’) alle Bereichs-MXIDs einer Person kennt, darf er nicht vom Betreiber des Postfach-Backends betrieben werden. Werden die Zuordnungsdienste bei Option 2E als Bestandteil des zentralen Postfach-Backends betrieben, darf dessen Betreiber nicht zugleich die BundID betreiben, da er sonst Identität, Bereichs-MXIDs und Kommunikationsbeziehungen einer Person verknüpfen könnte.

**Option 2C (gemeinsame Pseudonyme aus einem zentral vergebenen, personengebundenen Schlüssel) wird als Zielbild weiterverfolgt**, weil das Pseudonym nach Option 2C über jedes Authentisierungssystem und auch nach einem Ausweiswechsel gleich bleibt. Mit Option 2E oder 2E’ wird es im Zuordnungsdienst als weitere Kennung der bestehenden Bereichs-MXID zugeordnet. Die MXIDs ändern sich dadurch nicht. Die Verknüpfung weiterer Authentisierungssysteme entfällt dann, ebenso die erneute Zuordnung nach einem Ausweiswechsel, die ohne angemeldetes Gerät und ohne weiteres verknüpftes Authentisierungssystem nur über eine Behörde möglich ist. Für diese Option wäre ein personengebundener Schlüssel etwa auf Grundlage des Melderegisters notwendig, wie ihn Österreich mit der Stammzahl kennt.[^3] Die Stelle, die diese Schlüssel herausgibt, kann allerdings die Pseudonyme einer Person in allen Bereichen verknüpfen. Den Bereichs-MXIDs und Kommunikationsbeziehungen kann sie diese aber nur zuordnen, wenn sie zugleich einen Zuordnungsdienst oder das Postfach-Backend betreibt. Eine Umsetzung benötigt eine Abstimmung zwischen Entwicklern der BundID, der eID und der EUDI-Wallet zum Ableitungsverfahren der Pseudonyme.

Der Erstkontakt einer Behörde zu einer Privatperson ohne vorangegangenes Verfahren wird durch keine der gewählten Optionen ermöglicht, da der Zuordnungsdienst nicht als Adressregister fungieren kann.

**Option 2A (Pseudonym-Erzeugung ausschließlich über BundID) wird nicht gewählt**, da eine direkte Anmeldung mit eID oder EUDI-Wallet nicht möglich ist, die Anmeldung vollständig von der Verfügbarkeit der BundID abhängt und die Postfachadresse am Pseudonym der BundID hängt.

**Option 2B (bereichsspezifische Pseudonyme der BundID in der EUDI-Wallet) wird nicht gewählt**, da dies keine Lösung für die direkte Anmeldung über den eID-Server der Gegenstelle wäre. Zudem würde eine spätere Umstellung auf Option 2C alle MXIDs ändern.

**Option 2D (verschiedene Postfächer je Authentisierungssystem) wird ausgeschlossen**, da für eine Person in einem Bereich mehrere Adressen und Postfächer entstünden. Vorgänge wären nur über das Authentisierungssystem erreichbar, über das sie begonnen wurden.

### Abgrenzung und ergänzende Maßnahmen

Die bereichsspezifischen pseudonymen Adressen allein stellen die [Nichtverkettbarkeit](../../Glossar.md#nichtverkettbarkeit) nicht vollständig her. Dazu sind weitere Maßnahmen notwendig.

| Bedrohung                                                                                                                | Schutz bei 2E | Schutz bei 2E’ | Ergänzende Maßnahme                                                                          |
|--------------------------------------------------------------------------------------------------------------------------|---------------|----------------|----------------------------------------------------------------------------------------------|
| Behörden verschiedener Bereiche verketten die Kommunikation einer Person über die Adressen                               | ja            | ja             | nicht erforderlich                                                                           |
| Behörde gibt MXID und Identität an anderen Bereich weiter                                                                | nein          | nein           | Datenschutzrecht                                                                             |
| Betreiber des Postfach-Backends führt Bereichs-MXIDs einer Person über die Kennungen der Authentisierungssysteme zusammen | ja            | ja             | Betreibertrennung von Postfach-Backend und BundID                                            |
| Betreiber führen Bereichs-MXIDs einer Person über IP-Adressen und Zeitpunkte zusammen                                    | nein          | nein           | Metadatenminimierung (vgl. Folge-Entscheidung)                                               |
| Betreiber des Zuordnungsdienstes führt Bereichs-MXIDs einer Person zusammen                                              | ja            | nein           | Betreibertrennung von Zuordnungsdienst und Postfach-Backend für Privatpersonen               |
| Bei Nutzung der BundID können der Betreiber der [Nachweisstelle](../../Glossar.md#nachweisstelle) und der Betreiber des Postfach-Backends gemeinsam Bereichs-MXIDs der Identität zuordnen | nein | nein | Nachweis ohne MXID (vgl. Folge-Entscheidung), Betreibertrennung von Nachweisstelle und Postfach-Backend |
| Betreiber von Nachweisstelle und zentralem Zuordnungsdienst ordnet Identität, Bereichs-MXIDs und Bereiche einer Person einander zu | ja | nein | Betreibertrennung von Zuordnungsdienst und Nachweisstelle (vgl. Offene Punkte) |
| Betreiber von Postfach-Backend für öffentliche Stellen und zentralem Zuordnungsdienst verkettet die Kommunikation einer Person über die Bereiche der öffentlichen Stellen dieses Backends | ja | nein | Betreibertrennung von Zuordnungsdienst und Postfach-Backends für [Organisationen](../../Glossar.md#organisation) (vgl. Offene Punkte) |
| Betreiber eines Postfach-Backends und der Nachweisstelle verkettet Identitäten und Kommunikationsbeziehungen | nein | nein | Trennung von Nachweisstelle und Postfach-Backends (Anschlussbedingung, vgl. Folge-Entscheidung) |
| BundID als Authentisierungssystem erfährt Bereich der Anmeldung                                                           | nein          | ja             | bei 2E vermeidbar, wenn die Person eID oder EUDI-Wallet nutzt, sonst akzeptiertes Restrisiko |

Nur Adressen von Organisationen werden im Adressregister geführt (vgl. Folge-Entscheidung). Behörden nutzen zum Kontakt mit Privatpersonen die im Antrag oder auf anderem Wege ihnen zuvor mitgeteilten pseudonymen Adressen.

Hier wird die Verkettung über Angaben nicht betrachtet, die im Verfahren fachlich gemacht werden. Zum Beispiel ist ein Zusammenführen von Daten behördenübergreifend mit Identitätsnachweisen denkbar.

## Auswirkungen

- Schreibt eine Privatperson eine Behörde an, ermittelt ihr Client zunächst den Bereich über eine [Adressregister-Abfrage](../../Glossar.md#adressregister-abfrage) (vgl. Folge-Entscheidung). Bei der ersten Kommunikation mit diesem Bereich meldet sich die Person beim Zuordnungsdienst des Bereichs an. Dieser erzeugt die Bereichs-MXID und ordnet sie der Kennung des genutzten Authentisierungssystems zu.

- Der zentrale Postfach-Zugang stellt die Postfächer der Bereiche in einer gemeinsamen Ansicht dar. Bei Option 2E wird die Liste der Bereiche verschlüsselt vorgehalten.

- Der Zuordnungsdienst wird als eigene Komponente oder bei Option 2E auch als Funktion des Postfach-Backends umgesetzt.

- Jede Bereichs-MXID erhält eine eigene kryptografische Identität mit eigenen Schlüsseln und eigener Geräte-ID. Einen bereichsübergreifenden Master Signing Key gibt es nicht. Ein neues Gerät muss deshalb für jede Bereichs-MXID gesondert verifiziert werden, was im Client in einem Schritt erfolgen kann.

## Risiken

- Betreibt dieselbe Stelle BundID und Postfach-Backend, scheidet die BundID als zentraler Zuordnungsdienst aus, da sonst eine Verkettung möglich wäre.

- Wer Anmeldungen für mehrere Bereiche verarbeitet, kann sie über Zeitpunkte und IP-Adressen verknüpfen. Das betrifft einen Betreiber mehrerer Zuordnungsdienste und eine Stelle, die BundID und Postfach-Backend gemeinsam betreibt (vgl. Folge-Entscheidung).

- Steht nach einem Ausweiswechsel oder einem Wechsel der Wallet weder ein angemeldetes Gerät noch ein weiteres verknüpftes Authentisierungssystem zur Verfügung, ist eine Zuordnung nur über eine Behörde möglich, der die Person bekannt ist. Dieser Weg muss vor Missbrauch geschützt werden (vgl. Offene Punkte).

- Meldet sich eine Person über ein neues Authentisierungssystem an, ohne es zu verknüpfen, entsteht im Bereich eine zweite Bereichs-MXID. Matrix sieht keine Zusammenführung von Konten vor.

- Ändert sich die Zuordnung einer öffentlichen Stelle zu einem Bereich, müssen bestehende Vorgänge auf die Bereichs-MXID des neuen Bereichs umgestellt werden. Andernfalls muss die öffentliche Stelle Adressen einer Person aus beiden Bereichen nutzen.

- Übernimmt eine Bestandslösung übergangsweise eine bestehende Kennung wie das Postfachhandle der BundID als Localpart, entspricht das Option 1A. Ein späterer Wechsel auf Bereichs-MXIDs würde alle Adressen der Privatpersonen ändern.

- Der Client hält je Bereich eine eigene, lange gültige Sitzung. Das erhöht den Schaden bei Verlust eines Geräts. Gerätesperre und der Widerruf einzelner Sitzungen begrenzen das Risiko.

## Folge-Entscheidungen

- Fachlicher Identitätsnachweis im Matrix-Nachrichtenkontext: Identitätsnachweis mit eID oder EUDI-Wallet gegenüber der Behörde und Nachweis ohne Bereichs-MXID bei Nutzung der BundID, siehe [ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md).

- Entkopplung von Nutzerkonto und Postfach: Postfach als eigene Komponente neben dem Nutzerkonto, siehe [ADR-1110](./ADR-1110-Entkopplung-von-Nutzerkonto-und-Postfach.md).

- Auflösung und Lebenszyklus organisationsbezogener Postfachadressen: Adressregister nur für Organisationen, mit Angabe des Bereichs öffentlicher Stellen, siehe [ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md).

- Getrennter Betrieb der zentralen Postfach-Backends nach [Teilnehmendengruppen](../../Glossar.md#teilnehmendengruppe): getrennte Backends für Privatpersonen und Organisationen, siehe [ADR-1114](./ADR-1114-Getrennter-Betrieb-der-zentralen-Postfach-Backends-nach-Teilnehmendengruppen.md).

- [Anschlussbedingungen](../../Glossar.md#anschlussbedingungen) und [Föderationskontrolle](../../Glossar.md#föderationskontrolle): Trennung der zentralen Nachweisstelle für Privatpersonen vom Betrieb der Postfach-Backends als Anschlussbedingung, siehe [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md).

- Metadatenminimierung im Betrieb: Regeln für Verbindungs- und Gerätedaten der Betreiber wie IP-Adressen und Zeitpunkte von Anmeldungen.

## Offene Punkte

- Für die Wahl zwischen Option 2E und Option 2E’ ist festzulegen, was ein Bereich umfasst, also eine einzelne Behörde oder ein Verwaltungsbereich mit mehreren Behörden. RAH_ÜBE_144 nennt Verwaltungsbereiche wie die Allgemeine Innere Verwaltung, das Schulwesen oder die Gesundheit.

- Festlegung der Betreiber der Zuordnungsdienste entsprechend den Einschränkungen der entschiedenen Optionen.

- Wiederherstellung der Adress-Zuordnung nach einem Ausweiswechsel oder einem Wechsel der Wallet, wenn weder ein angemeldetes Gerät noch ein weiteres verknüpftes Authentisierungssystem existiert, ggf. über eine Behörde, sowie Schutz dieses Wegs vor Missbrauch.

- Prüfung, ob ein personengebundener Schlüssel auf Grundlage des Melderegisters eingeführt werden kann (Option 2C), und Abstimmung mit den Entwicklern von BundID, eID und EUDI-Wallet zu einer einheitlichen Ableitung der Pseudonyme.

- Prüfung, ob Zuordnungsdienst und Nachweisstelle für Privatpersonen von verschiedenen Stellen betrieben werden müssen. In der Nachweisstelle sind Identität und gebundener Schlüssel bekannt. Im Zuordnungsdienst ist der Bereich jeder Bereichs-MXID bekannt. Bei gemeinsamem Betrieb scheidet Option 2E’ aus, da sich sonst ableiten ließe, mit welchen Bereichen eine Person kommuniziert.

- Prüfung, ob der zentrale Zuordnungsdienst (Option 2E’) und die Postfach-Backends für Organisationen von verschiedenen Stellen betrieben werden müssen. Ein Backend für öffentliche Stellen kennt die Bereichs-MXIDs der Privatpersonen, die mit seinen öffentlichen Stellen kommunizieren.

## Referenzen

[^1]: [EU-Kommission, Architecture and Reference Framework (ARF), Diskussionsthema E zu Pseudonymen](https://eudi.dev/latest/discussion-topics/e-pseudonyms-including-user-authentication-mechanism/), abgerufen am 14. Oktober 2026.

[^2]: [IT-Planungsrat, Beschluss B-2026/19-IT, Verwaltungsanbindung EUDI-Wallet, Abschnitt 3.1](https://www.it-planungsrat.de/beschluss/b-2026-19-it), abgerufen am 14. Oktober 2026.

[^3]: [Bundeskanzleramt Österreich, Bereichsspezifische Personenkennzeichen](https://www.bundeskanzleramt.gv.at/agenda/digitalisierung/stammzahlenregisterbehoerde/bereichsspezifische-personenkennzeichen/beschreibung.html), abgerufen am 12. Oktober 2026.

[^4]: [Krenn, Lesaignoux, Ramacher, bPk# Delegatable Pseudonyms and Their Applications to National eID Systems, ASIA CCS 2026](https://doi.org/10.1145/3779208.3785289), abgerufen am 2. Juli 2026.
