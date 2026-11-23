<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Zielarchitektur/Architekturentscheidungen/ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 20d3c3878a); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# ADR-1144: PKI-Anforderungen und Vertrauensinfrastruktur

> **Status:** ✔️ Operativ entschieden | **Datum:** 11. November 2026

## Kontext und Problembeschreibung

Diverse Komponenten der Zielarchitektur benötigen eine PKI für Schlüssel oder Zertifikate, die als [Vertrauensanker](../../Glossar.md#vertrauensanker), für die Transportverschlüsselung, die Signaturerzeugung und für Zeitstempel verwendet werden.

Die offene Architektur entsprechend dem Zero-Trust-Ansatz ermöglicht die Entwicklung und den Betrieb von [Postfach-Backends](../../Glossar.md#postfach-backend) durch Dritte ([ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md), [ADR-0007](./ADR-0007-Implementierung-von-Postfach-Backends.md)). Die Sicherheit des Gesamtsystems darf folglich nicht von der korrekten Funktionsweise einzelner Backends abhängen ([ADR-0013](./ADR-0013-Authentizität-in-der-Nachrichtenübermittlung.md)). Es dürfen somit nur Backends am [Verbund](../../Glossar.md#verbund) teilnehmen, die in der [Föderationsliste](../../Glossar.md#föderationsliste) geführt sind.

Neben den Quittungen wie [Bereitstellungsbestätigungen](../../Glossar.md#bereitstellungsbestätigung) ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)) sind vor allem die [Identitätsnachweise](../../Glossar.md#fachlicher-identitätsnachweis) der Kommunikationsteilnehmenden gegen die Vertrauensinfrastruktur prüfbar ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)). Für ihre Prüfung benötigt die Zielarchitektur eine [Trust-Liste](../../Glossar.md#trust-liste), die registrierte [Nachweisstellen](../../Glossar.md#nachweisstelle) mit Status führt ([ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)).

Hier wird entschieden, wie das Vertrauen im Verbund hergestellt wird. Insbesondere werden die Zertifikatsausstellung für die Server-Server-API sowie die Signierung und Pflege der Föderationsliste und der Trust-Liste betrachtet. Darüber hinaus werden die Anforderungen an die PKI-Systeme zusammengefasst, auf die andere Architekturentscheidungen verweisen.

Nicht betrachtet werden die Schlüsselinfrastruktur der Matrix-Ende-zu-Ende-Verschlüsselung, die Trust-Listen der [EUDI-Wallet](../../Glossar.md#europäische-brieftasche-für-die-digitale-identität) und die PKI der [eID](../../Glossar.md#eid-funktion).

## Entscheidungstreiber

- **Zero Trust:** Das System setzt den Zero-Trust-Ansatz um (SIC_ÜBE_ÜBE_018).

- **Föderationskontrolle:** Nur zugelassene Postfach-Backends und Nachweisstellen nehmen am Verbund teil. Sie können suspendiert oder ausgeschlossen werden ([ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md), [ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)).

- **Offener Verbund:** Postfach-Backends können auch von Dritten betrieben werden ([ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md)).

- **Organisationsidentität:** [Organisationen](../../Glossar.md#organisation) bzw. ihre Systeme können sich mittels Zertifikaten authentisieren ([ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md), [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md)).

- **Beweisbarkeit:** Die erfolgte Übermittlung und die [Authentizität](../../Glossar.md#authentizität) sind bei Bedarf auch gegenüber Dritten beweisbar (ANF_FUN_ZUS_044, SIC_AUT_ÜBE_013, [ADR-0017](./ADR-0017-Authentizität-und-Integrität-von-Inhalten.md), [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

- **Transportverschlüsselung:** Zusätzlich zur [Ende-zu-Ende-Verschlüsselung](../../Glossar.md#ende-zu-ende-verschlüsselung) wird flächendeckend Transportverschlüsselung eingesetzt (SIC_VER_ÜBE_055).

- **Crypto-Agilität:** Das System setzt Crypto-Agilität um. Die betrachteten kryptografischen Verfahren sind austauschbar (SIC_VER_ÜBE_031).

## Betrachtete Optionen

### Teil 1: Zertifikate der Server-Server-API

Im Folgenden wird entschieden, welche PKI die TLS-Zertifikate für die Kommunikation der Postfach-Backends untereinander (über die Server-Server-API) ausstellt.

#### Option 1A: Öffentliche PKI

Die TLS-Zertifikate werden wie in der öffentlichen Matrix-Föderation von öffentlichen Zertifizierungsstellen ausgestellt.

- Gut, denn es wird keine eigene Zertifizierungsstelle benötigt.

- Schlecht, denn jede öffentliche Zertifizierungsstelle kann Zertifikate für Domains der Postfach-Backends ausstellen.

- Schlecht, denn so bestimmen die Browser- und Betriebssystemhersteller die anerkannten Zertifizierungsstellen in der Föderation.

- Schlecht, denn die Zulassung der Backends durch die Verbund-Governance hätte keine Auswirkung auf die Transportebene.

#### Option 1B: V-PKI

Die TLS-Zertifikate werden von der Verwaltungs-PKI (V-PKI) ausgestellt.

- Gut, denn die V-PKI besteht bereits und ist auf die öffentliche Verwaltung beschränkt ([ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md)).

- Schlecht, denn Betreiber Dritter und [private Organisationen](../../Glossar.md#private-organisation) könnten diese PKI nicht nutzen.

- Schlecht, denn eine automatisierte Zertifikatserneuerung (ACME) wird derzeit nicht unterstützt ([ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md)).

#### Option 1C: Zertifizierungsstelle des Verbunds

Im Verbund wird eine eigene Zertifizierungsstelle etabliert, die von staatlichen Stellen betrieben wird und die TLS-Zertifikate ausstellt.

- Gut, denn der Verbund bestimmt selbst, welche Zertifizierungsstelle in der Föderation anerkannt wird.

- Gut, denn die Zulassung der Backends durch die Verbund-Governance hätte Auswirkung auf die Transportebene. Nur zugelassene Backends besäßen TLS-Zertifikate, die innerhalb des Verbunds akzeptiert würden.

- Gut, denn die PKI könnte modern umgesetzt werden und Zertifikate ausstellen, die sich automatisiert erneuern (ACME).

- Gut, denn die Zertifikate dieser PKI wären gleichermaßen für öffentliche und private Organisationen nutzbar.

- Schlecht, denn eine eigene Zertifizierungsstelle muss aufgebaut und betrieben werden.

### Teil 2: Signatur der Föderationsliste und der Trust-Liste

Im Folgenden wird betrachtet, welche PKI die Zertifikate herausgibt, mit denen die Listen der Verbund-Governance für zugelassene Nachweisstellen und die Backend-Föderation signiert werden.

#### Option 2A: Eigene Signaturschlüssel des Verbunds

Der Verbund gibt eigene Vertrauensanker heraus, wobei die Root-CAs offline bleiben. Die Listen werden mit von den Root-CAs abgeleiteten Zertifikaten signiert, so dass diese Zertifikate bei Kompromittierung gesperrt werden können.

- Gut, denn der Verbund verwaltet die Schlüssel selbst und ist von keinem Vertrauensdiensteanbieter abhängig.

- Schlecht, denn die Vertrauensanker müssen verbindlich veröffentlicht und an alle prüfenden Systeme verteilt werden.

- Schlecht, denn der Verbund muss die PKI selbst sicher betreiben.

#### Option 2B: V-PKI

Die Listen werden mit Zertifikaten der staatlich betriebenen V-PKI signiert.

- Gut, denn die V-PKI besteht bereits und es wird keine eigene PKI benötigt ([ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md)).

- Schlecht, denn das Root-Zertifikat der V-PKI muss an alle prüfenden Systeme verteilt werden.

- Schlecht, denn unter der Root-CA der V-PKI werden auch Zertifikate anderer Stellen ausgestellt, sodass im Verbund zusätzlich geprüft werden muss, ob das Zertifikat auf die zuständige Stelle ausgestellt ist.

#### Option 2C: Qualifizierte Siegel eines Vertrauensdiensteanbieters

Die Listen werden mit [qualifizierten elektronischen Siegeln](../../Glossar.md#qualifiziertes-elektronisches-siegel) versehen. Die Siegelzertifikate stellt ein qualifizierter Vertrauensdiensteanbieter auf die zuständige Stelle aus (eIDAS-VO).

- Gut, denn qualifizierte Siegel sind rechtlich anerkannt.

- Gut, denn Dritte wie Gerichte können die Prüfung über die Vertrauenslisten nach eIDAS-VO vornehmen, ohne einen Vertrauensanker des Verbunds zu kennen.

- Schlecht, denn das System wird von einem Vertrauensdiensteanbieter abhängig, der häufige Aktualisierungen unterstützen muss.

- Schlecht, denn prüfende Systeme müssen qualifizierte Siegel auswerten können.

- Schlecht, denn über die Vertrauenslisten nach eIDAS-VO werden auch Siegel anderer Stellen anerkannt, sodass im Verbund zusätzlich geprüft werden muss, ob das Siegel von der zuständigen Stelle stammt.

## Architekturentscheidung

**Teil 1: gewählte Option 1C Zertifizierungsstelle des Verbunds**, weil der Verbund selbst bestimmen kann, welche Zertifizierungsstelle in der Föderation anerkannt wird, und auch nicht-staatliche Betreiber mit gleicher PKI aufgenommen werden können. Andere Zertifizierungsstellen werden in der Föderation nicht akzeptiert.

**Teil 2: gewählte Option 2A Eigene Signaturschlüssel des Verbunds**, weil die Listen ohne Abhängigkeit von einem Vertrauensdiensteanbieter aktuell gehalten werden. Föderationsliste und Trust-Liste erhalten jeweils einen eigenen Vertrauensanker. **Option 2B kann perspektivisch gewählt werden**, wenn die V-PKI eine eigene Intermediate-CA für die Listen bereitstellt.

Unabhängig von den Entscheidungen in Teil 1 und 2 müssen die Zertifikate der Client-Server-API der Postfach-Backends von einer öffentlichen PKI ausgestellt werden, da Browser ihnen vertrauen müssen. [Qualifizierte elektronische Zeitstempel](../../Glossar.md#qualifizierter-elektronischer-zeitstempel) werden über die Vertrauenslisten nach eIDAS-VO geprüft ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

**Option 1A wird ausgeschlossen**, da nicht gesteuert werden kann, welche Zertifizierungsstellen Zertifikate für die Server-Server-API ausstellen. **Option 1B wird ausgeschlossen**, da die V-PKI nur für die öffentliche Verwaltung gedacht ist. **Option 2C wird ausgeschlossen**, da die Aktualisierung der Listen von einem Vertrauensdiensteanbieter abhängig würde.

## Umsetzung der gewählten Optionen

Im Folgenden werden die Komponenten der Vertrauensinfrastruktur der Zielarchitektur kurz gelistet.

| Vertrauensinfrastruktur | Details | Administration |
|---|---|---|
| Öffentliche PKI mit Browser-Vertrauen | TLS-Zertifikat der Domain der Client-Server-API des Postfach-Backends mit Root-Zertifikaten von Browser und OS als Vertrauensanker ([ADR-0011](./ADR-0011-Kommunikationsschicht.md)). | öffentliche Zertifizierungsstellen |
| PKI der Server-Server-API | TLS-Zertifikate der Postfach-Backends in der Föderation von einer Zertifizierungsstelle des Verbunds ([ADR-0011](./ADR-0011-Kommunikationsschicht.md), [ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)). Vertrauensanker ist deren Root-Zertifikat. | Zertifizierungsstelle des Verbunds, Zielbild D-Stack |
| Föderationsliste | Listet zugelassene Postfach-Backends mit Domain, Inhaber, Betreiber und Schlüsseln, etwa für Siegel der Bereitstellungsbestätigungen und Signatur der Adressregistereinträge ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md), [ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md), [ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)). Vertrauensanker ist das Root-Zertifikat der Liste. | zuständige Stelle der Verbund-Governance |
| Trust-Liste der Nachweisstellen | Listet zugelassene Nachweisstellen (zentrale Nachweisstelle, [Org-IdP-Adapter](../../Glossar.md#org-idp-adapter)) mit Schlüssel und Status ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md), [ADR-1107](./ADR-1107-Nachweisformat-Identitaetsnachweis-Matrix-Nachrichten.md), [ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)). Vertrauensanker ist das Root-Zertifikat der Liste. | zuständige Stelle der Verbund-Governance |
| PKI für Organisationen, V-PKI | Zertifikate der Systeme der Organisationen ([Fachverfahren](../../Glossar.md#fachverfahren), [RMS](../../Glossar.md#raum-management-service)) mit maschinenlesbaren Organisationsattributen ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)). Vertrauensanker ist das Root-Zertifikat einer ertüchtigten V-PKI für [öffentliche Stellen](../../Glossar.md#öffentliche-stelle) ([ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md)) bzw. einer PKI für private Organisationen, z. B. der ertüchtigten ELSTER-PKI ([ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md)). | Zertifizierungsstellen der V-PKI bzw. der genutzten PKI |
| Qualifizierte Vertrauensdienste | Qualifizierte Zeitstempel in Bereitstellungsbestätigungen ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)), ggf. qualifizierte Siegel und [QES](../../Glossar.md#qualifizierte-elektronische-signatur) ([ADR-0017](./ADR-0017-Authentizität-und-Integrität-von-Inhalten.md), [ADR-1118](./ADR-1118-Abruf-und-Lesebestätigungen.md)). Vertrauensanker sind Vertrauenslisten nach eIDAS-VO. | qualifizierte Vertrauensdiensteanbieter |

Die Zertifikate belegen die Zugehörigkeit der Schlüssel zu einer Domain oder Organisation. Ob Postfach-Backends und Nachweisstellen im Verbund zugelassen sind, ist aus der Föderationsliste bzw. der Trust-Liste abzuleiten. Die Gültigkeit einer Zulassung ergibt sich aus den dort gepflegten Zeiten für Aufnahme, Ausschluss und vorübergehende Suspendierung. Die Listen können von derselben Stelle administriert werden. Sie haben unterschiedliche Vertrauensanker, und ihre Einträge bleiben getrennt gültig und prüfbar. Mit der Aufnahme in die Föderationsliste kann ein Backend am Verbund teilnehmen. Identitätsnachweise darf ein System erst nach der Aufnahme in die Trust-Liste ausstellen ([ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)).

## Anforderungen an die PKI-Systeme

Die folgenden Festlegungen und Anforderungen gelten für alle PKI-Systeme der Zielarchitektur.

### Festlegungen

- Die Zertifikate und Schlüssel werden jeweils nur für bestimmte Zwecke verwendet. Zum Beispiel sollen die Postfach-Backends für die Signaturen der Bereitstellungsbestätigungen andere Schlüssel nutzen als für die Transportverschlüsselung.

- Das System setzt Crypto-Agilität um. Die betrachteten kryptografischen Verfahren sind austauschbar (SIC_VER_ÜBE_031). Es wird den Empfehlungen des BSI für kryptografische Verfahren gefolgt (BSI TR-02102).

- Private Signatur- und CA-Schlüssel werden in gesicherten Umgebungen wie Hardware-Sicherheitsmodulen vorgehalten.

- Empfänger-Clients, Fachverfahren und Postfach-Backends benötigen Zugriff auf die öffentlichen Schlüssel und den Status der jeweiligen PKIs.

- Kompromittierte Schlüssel und Zertifikate müssen gesperrt werden können.

- Zertifikate enthalten personenbezogene Daten nur, wenn es notwendig ist. Organisationsbezüge sind maschinenlesbar enthalten (DAT_ÜBE_ÜBE_021).

- Eine automatisierte Zertifikatserneuerung (ACME) soll verwendet werden, wenn Zertifikate regelmäßig erneuert werden müssen und die Bestimmungen der PKI dies erlauben.

- Öffentliche Schlüssel werden in einem JWKS-kompatiblen Format bereitgestellt, insbesondere in der Föderationsliste und der Trust-Liste. Wenn nötig, sind zugehörige X.509-Zertifikate enthalten. X.509-Zertifikate werden nicht für fachliche Identitätsnachweise natürlicher Personen verwendet ([ADR-1107](./ADR-1107-Nachweisformat-Identitaetsnachweis-Matrix-Nachrichten.md)).

### Föderationsliste und Trust-Liste

- **Vertrauensanker und Listen:** Für die Zielarchitektur sind die Föderationsliste der Postfach-Backends und die Trust-Liste der Nachweisstellen notwendig. Sie erlauben die Gültigkeitsprüfung von Zertifikaten und Schlüsseln. Die Listen werden jeweils unter einem eigenen Vertrauensanker signiert und von der zuständigen Stelle des Verbunds aktuell gehalten. Eine Orientierung für die Umsetzung bietet die Trust-service Status List (TSL) mit OCSP der gematik-Telematikinfrastruktur.[^1] Ein sogenannter Federation Master übernimmt dort die Verwaltung der Trust-Informationen.[^2]

- **Föderationsliste:** Die Föderationsliste führt je Postfach-Backend die Domain, den Inhaber, den Betreiber und die öffentlichen Schlüssel. Sie erlaubt die Abfrage nach dem Status des Eintrags, der einem Zeitraum zugeordnet ist.

- **Nachweisstellen:** Nachweisstellen signieren die fachlichen Identitätsnachweise und müssen zentral registriert werden ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md), [ADR-1107](./ADR-1107-Nachweisformat-Identitaetsnachweis-Matrix-Nachrichten.md), [ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)). Die Trust-Liste listet je Nachweisstelle den Schlüssel, die Gültigkeit und den Status. Bei einem Org-IdP-Adapter werden zusätzlich die Organisation gelistet, für deren Mitarbeitende Nachweise ausgestellt werden dürfen, und das höchste Vertrauensniveau, das der Adapter bescheinigen darf. Clients erkennen einen Nachweis nur bis zu diesem Niveau an. Die Nachweise des Org-IdP-Adapters sind verbundweit gültig.

- **Gültigkeit der Listen:** Die Systeme des Verbunds sollen die Föderationsliste und die Trust-Liste in kurzen Abständen aktualisieren. Sofern keine aktuelle Liste abrufbar ist, gilt die zuletzt abgerufene Liste bis zum Ende einer festzulegenden Gültigkeitsdauer. Hierdurch wird bestimmt, wie schnell eine Änderung in der Liste, etwa eine Suspendierung, im Verbund wirkt.

### Transport und technische Systeme

- **Transport-TLS für Postfach-Backends:** Für die Client-Server-API und die Server-Server-API der Postfach-Backends werden TLS-Zertifikate benötigt, die den Betreibern bzw. Domains eindeutig zugeordnet sind (SIC_VER_ÜBE_055). Die Transportverschlüsselung soll mindestens mit TLS 1.3 und post-quantum-sicherem Schlüsselaustausch umgesetzt sein.

- **Technische Nutzer:** Zertifikate der [technischen Nutzer](../../Glossar.md#technischer-nutzer) ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)) und der Fachverfahren müssen einer Organisation eindeutig zugeordnet sein und die [Organisationsidentität](../../Glossar.md#organisationsidentität) über maschinenlesbare Organisationsattribute nachweisen. Für öffentliche Stellen ist dafür die V-PKI ([ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md)) und für private Organisationen eine PKI mit gesicherten Organisationsattributen ([ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md)) vorgesehen.

### Beweisbarkeit

- **Siegel für Bereitstellungsbestätigungen:** Bereitstellungsbestätigungen und [Löschquittungen](../../Glossar.md#löschquittung) werden mit dem [fortgeschrittenen elektronischen Siegel](../../Glossar.md#fortgeschrittenes-elektronisches-siegel) des Betreibers des empfangenden Postfach-Backends signiert ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). Das Siegelzertifikat wird nach eIDAS-VO auf den Betreiber als juristische Person ausgestellt. Die Zuordnung des Zertifikats zur Domain des Backends wird bei der Registrierung im [SSP](../../Glossar.md#self-service-portal) festgelegt.

- **Zentraler Zeitstempeldienst:** Für die Zeitpunkte der Bereitstellung von [Nachrichten](../../Glossar.md#nachricht) sollen qualifizierte elektronische Zeitstempel in der Bereitstellungsbestätigung enthalten sein (ANF_FUN_ZUS_044, [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). Ein zentraler Zeitstempeldienst eines qualifizierten Vertrauensdiensteanbieters soll durch die Postfach-Backends angebunden werden, womit deren Betreiber keine Vertrauensdiensteanbieter sein müssten (vgl. [ADR-0017](./ADR-0017-Authentizität-und-Integrität-von-Inhalten.md)). Der Zeitstempeldienst erhält nur einen Hashwert. Nachrichtendaten werden nicht an ihn gegeben. Für qualifizierte Anwendungsfälle kommen qualifizierte elektronische Zeitstempel in Betracht (ANF_FUN_ZUS_044, [ADR-1120](./ADR-1120-Beweissichere-Übermittlung-von-Anhängen.md)).

- **Langzeitprüfbarkeit:** Nachweise müssen hinsichtlich ihrer Gültigkeit zum Signaturzeitpunkt prüfbar sein. Die Vertrauensinfrastruktur muss dafür historische Schlüssel und Statusverläufe langfristig vorhalten und für die Prüfung bereitstellen können. Statusänderungen werden nachvollziehbar protokolliert, z. B. in einem Transparency Log (vgl. [ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md), [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md)). Damit Dritte, z. B. Gerichte, die Nachweise auch ohne direkten Zugriff auf die Systeme der Zielarchitektur prüfen können, sollen die dafür nötigen Daten wie Zertifikate, Sperrstatus zum Signaturzeitpunkt und Zeitstempel mit dem Nachweis aufbewahrt werden, etwa bei der Veraktung ([ADR-1135](./ADR-1135-Veraktung-und-Aufbewahrung.md)). Da Zertifikate ablaufen und kryptografische Verfahren veralten können, ist während der Aufbewahrung der Erhalt des Beweiswerts zu gewährleisten, etwa durch das Erneuern von Zeitstempeln nach BSI TR-03125 (TR-ESOR).[^3]

## Auswirkungen

- Die Postfach-Backends sollen in der Föderation nur dem Root-Zertifikat der Zertifizierungsstelle des Verbunds vertrauen, was konform mit der Matrix-Spezifikation umsetzbar ist.

- Zur Ausstellung der Zertifikate für die Server-Server-API muss das Postfach-Backend im SSP registriert und dabei die Kontrolle über die zugehörige Domain nachgewiesen werden.

- Die Nutzung der V-PKI für öffentliche Stellen verlangt die bereits in [ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md) geforderte Ertüchtigung, insbesondere für maschinenlesbare Organisationsattribute und ACME.

- Die Vertrauensanker der Listen müssen verbindlich veröffentlicht und Clients, Fachverfahren und Postfach-Backends bereitgestellt werden.

## Risiken

- Eine Kompromittierung der Zertifizierungsstelle bzw. der Schlüssel des Verbunds hat Auswirkungen auf den gesamten Verbund. Private Signatur- und CA-Schlüssel müssen in gesicherten Umgebungen vorgehalten werden.

- Sind die Föderationsliste oder die Trust-Liste nicht erreichbar, gilt bis zum Ende ihrer Gültigkeitsdauer die zuletzt abgerufene Liste. Ist die Gültigkeitsdauer kurz, wirken Suspendierungen schnell. Ist sie lang, werden längere Ausfälle überbrückt.

- Ist die Zertifizierungsstelle des Verbunds nicht erreichbar, können Zertifikate weder erneuert noch gesperrt werden. Laufen Zertifikate in dieser Zeit ab, können die betroffenen Postfach-Backends nicht mehr an der Föderation teilnehmen.

## Offene Punkte

- **Zuständige Stelle:** Festlegung, wer die Trust-Liste und die Föderationsliste verwaltet und die Zertifizierungsstelle für die Server-Server-API betreibt. Zielbild ist eine Komponente im D-Stack. Übergangsweise kann ein System der Zielarchitektur diese Aufgaben übernehmen (vgl. [ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)).

- **Sperrwirkung:** Ob ein gesperrter Schlüssel ab dem Sperrzeitpunkt oder rückwirkend ungültig wird. Das betrifft die Einträge der Föderationsliste und der Trust-Liste sowie die Siegelzertifikate.

- **Zeitstempeldienst:** Anbieter des zentralen Zeitstempeldienstes, ggf. als gemeinsamer Dienst im D-Stack.

- **Aussteller der Siegelzertifikate:** Ob die Siegelzertifikate der Betreiber von einem qualifizierten Vertrauensdiensteanbieter stammen und damit über die Vertrauenslisten nach eIDAS-VO prüfbar sind.

## Referenzen

[^1]: [gematik gemSpec PKI (Spezifikation PKI), TUC_PKI-Use-Cases und Trust-service Status List](https://gemspec.gematik.de/docs/gemSpec/gemSpec_PKI/gemSpec_PKI_V2.26.0/), abgerufen am 12. Mai 2026.

[^2]: [gematik gemKPT Zero Trust V1.0.0, §3.2.1 (Federation Master als PKI-Verwalter des TI-Vertrauensraums)](https://gemspec.gematik.de/docs/gemKPT/gemKPT_Zero_Trust/gemKPT_Zero_Trust_V1.0.0/), abgerufen am 12. Mai 2026.

[^3]: [BSI TR-03125 Beweiswerterhaltung kryptographisch signierter Dokumente (TR-ESOR), Version 1.3](https://www.bsi.bund.de/DE/Themen/Unternehmen-und-Organisationen/Standards-und-Zertifizierung/Technische-Richtlinien/TR-nach-Thema-sortiert/tr03125/TR-03125_node.html), abgerufen am 12. Oktober 2026.
