# ADR-1145: Anschlussbedingungen und Föderationskontrolle

> **Status:** ✔️ Operativ entschieden | **Datum:** 14. Oktober 2026

## Kontext und Problembeschreibung

In der Zielarchitektur ist festgelegt, dass [Postfach-Backends](../../Glossar.md#postfach-backend) durch Dritte betrieben werden können. Für die Föderation mit den anderen Backends ist eine Registrierung notwendig, wobei [Anschlussbedingungen](../../Glossar.md#anschlussbedingungen) zugesichert werden ([ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md), [ADR-0008](./ADR-0008-Einheitliche-Anbindung-ueber-ein-zentrales-Self-Service-Portal.md)). Aus den gefällten Architekturentscheidungen (ADR) leiten sich auch verbindliche Mindeststandards für an der Kommunikation teilnehmende [Organisationen](../../Glossar.md#organisation) ab. Im Folgenden werden die Anschlussbedingungen nicht abschließend festgelegt. Es werden die aus den ADR abgeleiteten Bedingungen konsolidiert und festgelegt, wie deren Einhaltung gewährleistet werden kann.

Die [Föderationsliste](../../Glossar.md#föderationsliste) ist das verbundweite Verzeichnis der aufgenommenen Postfach-Backends mit ihren Domains, deren Inhabern und ihrem Betreiber. Sie wird wie die [Trust-Liste](../../Glossar.md#trust-liste) durch einen [Vertrauensanker](../../Glossar.md#vertrauensanker) signiert ([ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md)). Nur ein dort eingetragenes Backend nimmt am [Verbund](../../Glossar.md#verbund) teil (vgl. Folge-Entscheidung). Der TI-Messenger der gematik steuert seine Föderation ähnlich mittels Registrierungsdienst und zentral bereitgestellter Föderationsliste.[^1]

Es soll hier ein Rahmen vorgegeben werden, der den Prüfbedarf einzelner Bedingungen festlegt, wobei damit verbundene Aufwände möglichst gering zu halten sind. Betreiber von Postfach-Backends sind bekannte und haftbare Organisationen bzw. [öffentliche Stellen](../../Glossar.md#öffentliche-stelle). [Privatpersonen](../../Glossar.md#privatperson) betreiben keine eigenen Backends, sie nutzen das zentrale Postfach-Backend ([ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md), [ADR-1114](./ADR-1114-Getrennter-Betrieb-der-zentralen-Postfach-Backends-nach-Teilnehmendengruppen.md)). Die Umsetzung der [Bereitstellungsbestätigungen](../../Glossar.md#bereitstellungsbestätigung) in den Postfach-Backends setzt einen haftbaren Betreiber voraus ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). Jedes anzuschließende Backend wird über den Onboardingprozess im [Self-Service-Portal](../../Glossar.md#self-service-portal) registriert. Dabei wird eine Adress-Domain reserviert (vgl. [ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md)) und einer Organisation über deren öffentliche Schlüssel zugeordnet.

Die technische Basis ermöglicht eine weitere Eingrenzung der zu regelnden Anschlussbedingungen. Matrix ist ein offen spezifizierter Standard. Das Protokoll ist durch die Öffentlichkeit prüfbar und ermöglicht somit unabhängige Sicherheitsanalysen. Probleme auf Protokollebene müssen nicht für jedes System einzeln geprüft werden. Mit Matrix setzt die Zielarchitektur ein Ende-zu-Ende-verschlüsseltes System um, das die Sicherheit und im Besonderen die Vertraulichkeit der Kommunikation gewährleistet. Die Anschlussbedingungen betreffen deshalb vor allem das, was die Zielarchitektur dem Standard Matrix hinzufügt bzw. im Standard anpasst. Durch die [Identitätsnachweise](../../Glossar.md#fachlicher-identitätsnachweis) der Teilnehmenden ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)) und die von den Backends signierten Quittungen ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)) ist die Kommunikation nachweisbar. Ob ein Backend regelkonform arbeitet, wird im Betrieb auch durch andere Backends festgestellt, die z. B. Signaturen prüfen.

### Abgrenzung

- **Postfach-Zugänge:** Ein konformes Verhalten der Matrix-Clients lässt sich technisch nicht erzwingen. Die Implementierung der Bestätigungen im Client ist freiwillig ([ADR-1118](./ADR-1118-Abruf-und-Lesebestätigungen.md)).
- **Onboarding-Ablauf:** Der konkrete Prozess im SSP wird in einer Folge-Entscheidung geregelt.
- **Anschlussvertrag:** Vertragstext, vollständig definierte Kriterien für Betreiber und festgelegte Service-Level werden durch die Governance-Strukturen festgelegt und aus den Anschlussbedingungen abgeleitet.

## Entscheidungstreiber

- **Prüfaufwand:** Zertifizierung und Tests für die Zulassung anzuschließender Komponenten verursachen Aufwände, die den Anschluss verzögern und die Akzeptanz potenzieller Betreiber verringern können ([ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md)).

- **Teilnehmerkreis:** Die im Verbund angeschlossenen Organisationen und Betreiber sind eindeutig identifiziert ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)).

- **Matrix:** Das Protokoll ist offen spezifiziert und standardisiert, was Prüfaufwände verringert. Mit der Zielarchitektur notwendige Abweichungen gegenüber Standard Matrix müssen geprüft und sollten standardisiert werden.

- **Risikobewertung:** Jede Anschlussbedingung wird danach eingestuft, welcher Schaden bei einer Nichteinhaltung entsteht und ob ein Verstoß technisch feststellbar ist.

- **Schutzziele:** [Ende-zu-Ende-Verschlüsselung](../../Glossar.md#ende-zu-ende-verschlüsselung) (SIC_VER_ÜBE_026, SIC_VER_ÜBE_032), Zero-Trust-Ansatz (SIC_ÜBE_ÜBE_018), Datenminimierung (SIC_VER_ÜBE_051, DAT_ÜBE_ÜBE_021).

- **Beweissicherheit:** Quittungen müssen korrekt ausgestellt werden, damit der Zugang belegbar ist (ANF_FUN_ZUS_024, SIC_NAC_NIC_004).

- **Trust-Liste:** [Nachweisstellen](../../Glossar.md#nachweisstelle) des Verbunds werden mit Gültigkeit und Schlüssel in der Trust-Liste geführt ([ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)).

## Betrachtete Optionen

### Option A: Vollständige Zertifizierung und Auditierung

Vor der Aufnahme eines Postfach-Backends in den Verbund wird die Implementierung der Software und die umgesetzte Integration genau geprüft und zertifiziert. Der Betrieb des Backends wird fortwährend auditiert.

- Gut, denn die Sicherheit im Verbund wird auf einem einheitlichen hohen Niveau gewährleistet.

- Gut, denn die Nichteinhaltung von Anschlussbedingungen fällt bereits vor dem Anschluss auf.

- Schlecht, denn der Aufwand für die Governance des Verbundes durch die Zertifizierung ist hoch. Die notwendigen Prozesse skalieren schlecht mit der Zahl der Teilnehmenden.

- Schlecht, denn der Aufwand für die Teilnehmenden bzw. Betreiber ist sehr hoch.

- Schlecht, denn das Onboarding ist ein langwieriger Prozess und der Anschluss kann unattraktiv werden.

### Option B: Reine vertragliche Absicherung

Die Betreiber werden lediglich vertraglich zur Einhaltung der Anschlussbedingungen verpflichtet.

- Gut, denn der Aufwand für die Governance des Verbundes wird minimal gehalten.

- Gut, denn der Aufwand für die Teilnehmenden bzw. Betreiber wird minimal gehalten.

- Gut, denn der Onboardingprozess ist unkompliziert.

- Schlecht, denn nicht eingehaltene Anschlussbedingungen bleiben bis zum Auftreten von Problemen unentdeckt.

- Schlecht, denn die Sicherheit im Verbund ist weitgehend nur vom Engagement der Teilnehmenden bzw. Betreiber abhängig.

### Option C: Risikobasierte Durchsetzung und Absicherung

Die vertragliche Selbstverpflichtung der angeschlossenen Organisationen und Betreiber sichert den Verbund grundlegend ab. Zu dem Vertrag werden technische Maßnahmen implementiert bzw. umgesetzt, die Verstöße automatisch verhindern oder nachträglich erkennbar machen. Für kritische Anschlussbedingungen mit besonders hohem Risiko für die Teilnehmenden des Verbunds werden anlassbezogene oder stichprobenartige Prüfungen durchgeführt. Die Aufnahme und der Ausschluss werden über die Föderationsliste gesteuert.

- Gut, denn nicht eingehaltene Anschlussbedingungen werden weitgehend technisch verhindert und ggf. nachträglich erkannt.

- Gut, denn der Aufwand für die Governance des Verbundes wird dem spezifischen Risiko angepasst und ist insgesamt gering.

- Gut, denn der Aufwand für die Teilnehmenden bzw. Betreiber ist gering, da Prüfungen nur im Rahmen von Stichproben oder anlassbezogen stattfinden.

- Gut, denn der Onboardingprozess stellt nur eine geringe Hürde dar.

- Schlecht, denn bei technisch nicht prüfbaren Bedingungen ist die Sicherheit weiterhin vom Teilnehmenden bzw. Betreiber abhängig, womit ein Restrisiko bestehen bleibt.

## Architekturentscheidung

Gewählt wird **Option C (Risikobasierte Durchsetzung und Absicherung)**. Backends werden nicht einzeln zertifiziert oder auditiert. Bekannte haftbare Betreiber, technisch durchgesetzte Regeln und die Föderationsliste sind die Grundlage zur Durchsetzung der Anschlussbedingungen und Gewährleistung der Sicherheit im Verbund. Verbindlich werden die Anschlussbedingungen durch den [Anschlussvertrag](../../Glossar.md#anschlussvertrag) im SSP. Die rechtliche Ausgestaltung ist zu klären.

Aufnahme und Ausschluss der Backends werden über die Föderationsliste gesteuert. Ein Backend nimmt erst nach Abschluss des Onboardings teil und kann bei Verstoß ausgeschlossen werden. Bei einem Sicherheitsvorfall oder begründetem Verdacht auf einen Verstoß kann ein Backend vorübergehend suspendiert werden. Die anderen Backends föderieren dann nicht mehr mit ihm, bis über die Wiederaufnahme oder den Ausschluss entschieden ist. Bei einem Verstoß haftet der Betreiber für eventuell aufgetretene Schäden, was im Anschlussvertrag geregelt ist. Jedes Backend föderiert ausschließlich mit in der Föderationsliste gelisteten Backends.

### Arten der Absicherung

Es gibt unterschiedliche Arten, wie die Einhaltung der Anschlussbedingungen abgesichert wird:

- **strukturell:** der mögliche Schaden ist durch Teilnehmerkreis, Onboarding oder Architektur begrenzt, eine gesonderte Prüfung findet nicht statt. Dazu zählen Bedingungen, deren Verletzung überwiegend die eigene Organisation schädigt.

- **technisch geprüft:** automatisch durchgesetzt oder geprüft durch Kommunikationspartner, SSP oder Kryptografie, nicht durch den Betreiber selbst. Dazu gehört z. B. die Identitätsprüfung beim Onboarding, bei welcher das [Org-Konto](../../Glossar.md#org-konto) den Betreiber ausweist ([ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md), [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md)).

- **erkennbar:** ein Verstoß wird nicht verhindert, aber im Nachhinein nachweisbar, etwa über Signaturen, Quittungen oder Attestierungen.

- **vertraglich:** durch Selbstverpflichtung und Haftung im Anschlussvertrag, den die teilnehmende Organisation im SSP-Onboarding annimmt. Mit ihm übernimmt sie die Haftung für die in ihrem [Machtbereich](../../Glossar.md#machtbereich) betriebenen Komponenten.

- **Konformitätsprüfung im Einzelfall:** anlassbezogene oder stichprobenartige Prüfung als Ausnahme für wenige, von außen nicht beobachtbare Bedingungen, deren Nichteinhaltung ein hohes Risiko birgt.

## Umsetzung der gewählten Option

Die folgenden Bedingungen wurden aus den Architekturentscheidungen abgeleitet. Ein verbindlicher Katalog der Bedingungen wird über die Governance festgelegt (vgl. Abgrenzung).

Die Anschlussbedingungen gelten für zwei Rollen. Die Bedingungen an den Backend-Betrieb muss der **Betreiber des Postfach-Backends** erfüllen. Sie gelten einheitlich für die zentralen Backends ([ADR-1114](./ADR-1114-Getrennter-Betrieb-der-zentralen-Postfach-Backends-nach-Teilnehmendengruppen.md)) und für dezentrale Backends Dritter ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)). Die Bedingungen an [Fachverfahren](../../Glossar.md#fachverfahren), [Berechtigungsverzeichnis](../../Glossar.md#berechtigungsverzeichnis), [RMS](../../Glossar.md#raum-management-service), [Org-IdP-Adapter](../../Glossar.md#org-idp-adapter) und [E2EE-Endpunkte](../../Glossar.md#e2ee-endpunkt) muss die **angeschlossene Organisation** erfüllen, da diese Komponenten in ihrem Machtbereich liegen. Ihre technischen Systeme, zum Beispiel RMS und angebundene Fachverfahren, registriert sie mit den zugehörigen Schlüsseln im SSP (vgl. Folge-Entscheidung). Fachverfahren mit integrierten [Postfach-Zugängen](../../Glossar.md#postfach-zugang) werden von der betreibenden Organisation verantwortet, die einen Anschlussvertrag angenommen hat. Bei den zentral bereitgestellten Komponenten RMS, Berechtigungsverzeichnis und zentraler Nachweisstelle muss deren Betreiber die Bedingungen erfüllen, wobei die nutzende Organisation für die Pflege der Berechtigungen verantwortlich bleibt. Beim souveränen Betrieb ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md), Modell C) fallen beide Rollen zusammen. Bei der Minimalanbindung und beim Teilbetrieb (Modell A und B) betreibt die Organisation kein eigenes Backend, für ihre eigenen Komponenten bleibt sie aber verantwortlich. Betreiber und Organisationen nehmen den Anschlussvertrag jeweils im SSP-Onboarding an. Technisch prüfbare Kriterien, wie die Beachtung der Föderationsliste und der [Quittungsdienst](../../Glossar.md#quittungsdienst), werden im laufenden Betrieb durch die anderen Backends des Verbunds geprüft. Fehler bzw. Probleme fallen so frühzeitig auf, was eine zeitnahe Mitigation ermöglicht.

### Bedingungen an den Backend-Betrieb

Mit der Registrierung im SSP schließen die Betreiber das Backend mit seinen [Postfächern](../../Glossar.md#postfach) an den Verbund an. Verfügbarkeit und Reaktionszeiten werden im Anschlussvertrag als Service-Level geregelt (vgl. [ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md)). Werden [Nachrichten](../../Glossar.md#nachricht) nicht angenommen oder wird eine Bereitstellungsbestätigung nicht übermittelt, kann dies zum Ausschluss aus dem Verbund führen.

| Anschlussbedingung | Pflicht des Backend-Betreibers | Bedrohung bei Nichteinhaltung | Absicherung |
| --- | --- | --- | --- |
| Betreiber ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)) | Backends dürfen nur von Organisationen betrieben werden (haftbar, Kriterien im Anschlussvertrag) | Aufwände bei Onboarding und Betrieb steigen | strukturell, durch Teilnehmerkreis und Onboarding |
| [Identifizierung](../../Glossar.md#identifizierung) des Betreibers ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)) | Nachweis der [Organisationsidentität](../../Glossar.md#organisationsidentität) beim Onboarding | unbekannter, nicht haftbarer Betreiber | technisch geprüft beim Onboarding |
| Schlüsselregistrierung und PKI ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md), [ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md)) | registriert seine Schlüssel im SSP, verwendet nur Zertifikate zugelassener Vertrauensanker | gefälschte Quittungen, gebrochene Transportverschlüsselung | technisch geprüft, kryptografische Bindung |
| Beachtung der Föderationsliste | föderiert nur mit gelisteten Backends, setzt Suspendierungen und Ausschlüsse zeitnah um | eigene Postfächer kommunizieren mit ungeprüften Teilnehmenden, keine gültigen Nachweise | vertraglich, strukturell begrenzt durch gegenseitige Durchsetzung |
| Quittungsdienst ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md), [ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md)) | erzeugt mit Schlüssel des Betreibers signierte Bereitstellungsbestätigungen | gefälschter oder zurückgehaltener Nachweis der Bereitstellung, keine [Bekanntgabefiktion](../../Glossar.md#bekanntgabefiktion) | erkennbar über Siegel, Übermittlungsversuche werden beim Absender-Backend protokolliert |
| Proaktiver Pull ([ADR-1119](./ADR-1119-Absendergesteuerte-Übermittlungsoptionen-mit-Berechtigungsprüfung.md), [ADR-1120](./ADR-1120-Beweissichere-Übermittlung-von-Anhängen.md)) | lädt [Anhänge](../../Glossar.md#anhang) auf Anforderung des [Absenders](../../Glossar.md#absenderin) proaktiv in das eigene Medien-Repository | Anhänge sind beim Abruf nicht verfügbar, die Bereitstellung ist nicht beweissicher | erkennbar über die Abrufe beim Absender-Backend |
| Registereinträge im Profil ([ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)) | registriert Organisationsadressen und signiert die Einträge | Rückantwortadressen sind nicht prüfbar | erkennbar, da die Abfrage fehlschlägt |
| Localparts ([ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md)) | verwendet für natürliche Personen Localparts ohne direkten oder indirekten Personenbezug | Identifizierung natürlicher Personen über die Adresse | erkennbar |
| Trennung der zentralen Nachweisstelle ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md), [ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)) | betreibt die zentrale Nachweisstelle für Privatpersonen nicht selbst oder strikt getrennt vom Backend (Mandanten, Schlüssel, Betriebsumgebung) | Verkettung der [Bereichs-MXIDs](../../Glossar.md#bereichs-mxid) mit [Identitäten](../../Glossar.md#identität) und Kommunikationsbeziehungen | vertraglich, Konformitätsprüfung im Einzelfall |
| Zweckbindung von Metadaten und Verbindungsdaten ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md), [ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md)) | wertet Metadaten und Verbindungsdaten der Netzwerkebene nur zweckgebunden aus, speichert Metadaten nur wenn notwendig | Überwachung von Teilnehmern, Verbindungsdaten gelangen in falsche Hände | vertraglich, Konformitätsprüfung im Einzelfall |
| Sicherer Betrieb ([ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md)) | betreibt das Backend sicher nach Stand der Technik | ein unsicheres Backend gefährdet die eigenen Postfächer, die Metadaten der Kommunikationspartner und den eigenen Signaturschlüssel | vertraglich, strukturell durch E2EE begrenzt |
| Meldepflicht | meldet Sicherheitsvorfälle unverzüglich | kompromittierte Schlüssel, gefälschte Quittungen und Nachweise | vertraglich, Konformitätsprüfung im Einzelfall |

### Bedingungen an die anzuschließende Organisation bzw. Fachverfahren

| Anschlussbedingung | Pflicht der Organisation | Bedrohung bei Nichteinhaltung | Absicherung |
| --- | --- | --- | --- |
| Adressregister-Prüfung vor Übermittlung ([ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)) | sendende Fachverfahren prüfen die einer Adresse zugehörige Organisationsidentität vor der ersten Übermittlung durch eine [Adressregister-Abfrage](../../Glossar.md#adressregister-abfrage) | Versand an die falsche Organisation | vertraglich, im Nachhinein erkennbar |
| Berechtigungsverzeichnis und RMS ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md), [ADR-1112](./ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md)) | setzt Postfachberechtigungen durch, protokolliert Änderungen revisionssicher | unbefugte Postfach-Zugriffe innerhalb der Organisation | strukturell, da der Schaden organisationsintern bleibt, ergänzend vertraglich |
| Org-IdP-Adapter als Nachweisstelle ([ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md), [ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md)) | interne Identitäten werden nur gefiltert und pseudonym nach außen gegeben, stellt Identitätsnachweise nur nach Berechtigungsprüfung aus, hält Schlüssel- und Statushistorien langfristig verfügbar | nicht berechtigte Mitarbeiter schreiben Nachrichten im Namen der Organisation, Offenlegung interner Daten der Organisation und Mitarbeiter, nicht prüfbare Nachweise | vertraglich, Konformitätsprüfung im Einzelfall |
| Prüfung der Identitätsnachweise ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md), [ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md), [ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md)) | Fachverfahren prüfen Identitätsnachweise gegen die Trust-Liste der zugelassenen Nachweisstellen | Akzeptanz gefälschter oder gesperrter Identitäten | vertraglich |
| Malware-Prüfung nach dem Empfang ([ADR-1130](./ADR-1130-Malware-Prüfung.md)) | Prüfung an internem E2EE-Endpunkt der Kommunikation mit organisationsinternem Scandienst oder Endgeräteschutz | Verbreitung von Malware innerhalb der Organisation | strukturell, da der Schaden organisationsintern bleibt, ergänzend vertraglich |
| Malware-Prüfung vor dem Versand ([ADR-1130](./ADR-1130-Malware-Prüfung.md)) | prüft Anhänge vor dem Versand an Privatpersonen auf Schadsoftware | Verbreitung von Malware an Privatpersonen, deren Endgeräte oft schwächer geschützt sind | vertraglich |
| Archivierung ([ADR-1135](./ADR-1135-Veraktung-und-Aufbewahrung.md)) | nutzt die Schnittstellen für regelmäßige Exporte | Verlust relevanter Kommunikation | vertraglich, strukturell begrenzt, da eigene Aktenführung geschädigt |
| E2EE-Endpunkttreue ([ADR-1100](./ADR-1100-Bilaterale-Kommunikation-Festlegung-E2EE-Endpunkte.md), [ADR-1101](./ADR-1101-Mehrbenutzer-Kommunikation-Festlegung-E2EE-Endpunkte.md)) | terminiert E2EE ausschließlich im eigenen Machtbereich, sichert serverseitige E2EE-Teilnehmer ab | Datenabfluss aus der Organisation | vertraglich, Konformitätsprüfung im Einzelfall, insbesondere bei hohem Schutzbedarf |

Der überwiegende Teil der Bedingungen erfordert keine externe Prüfung. Sie sind strukturell begrenzt, technisch geprüft oder im Nachhinein erkennbar. Von außen schwer prüfbar und damit auf eine juristische Durchsetzung, z. B. über Haftungsregeln, angewiesen sind vor allem die Umsetzung von E2EE, der Schutz der Metadaten, die Meldepflicht bei Sicherheitsvorfällen, die Malware-Prüfung vor dem Versand und das Verhalten der Nachweisstellen. Diese Bedingungen betreffen Funktionen im Machtbereich der teilnehmenden Stelle. Da ihre Nichteinhaltung ein hohes Risiko birgt, kann für sie zusätzlich eine anlassbezogene oder stichprobenartige Konformitätsprüfung oder ein Audit durchgeführt werden. Verarbeitet eine Organisation Inhalte mit hohem Schutzbedarf, kann sie darüber hinaus für die Komponenten in ihrem Machtbereich und bei ihren Dienstleistern eigene Prüfungen veranlassen.

## Auswirkungen

- Der Anschlussvertrag und die Selbstauskunft zu den Anschlussbedingungen werden Teil des SSP-Onboardings für Organisationen und Backends.

- Prüfungen werden nur anlassbezogen oder stichprobenartig für kritische Punkte durchgeführt.

- Änderungen an der Föderationsliste müssen von den Backends in kurzen Zeitabständen aktualisiert werden.

- Die Entscheidung des [ADR-0007](./ADR-0007-Implementierung-von-Postfach-Backends.md) zur Zulassung von Drittimplementierungen wird hier eingeschränkt. Die Teilnahme von Backends am Verbund setzt die Annahme eines Anschlussvertrages und die Aufnahme in die Föderationsliste voraus.

## Risiken

- Durch eine Haftungsübernahme der Betreiber und technische Absicherungen wird das Risiko minimiert, dass fehlerhafte Backends im Verbund Schäden anrichten.

- Ohne klar geregelte Zuständigkeiten und Sanktionen sind die Anschlussbedingungen nicht durchsetzbar.

## Folge-Entscheidungen

- SSP-Onboarding für Organisationen und Backends, siehe [ADR-1148](./ADR-1148-Onboarding-im-Self-Service-Portal.md).

## Offene Punkte

- Governance: Es ist zu klären, wer zum Beispiel über die Suspendierung und den Ausschluss von Backends entscheidet.

- Konkrete Ausgestaltung des Anschlussvertrags.

- Umfang und Auslösung der anlassbezogenen Prüfungen.

- Aufbewahrungsfristen und Löschprozesse.

- Verfügbarkeit und Reaktionszeiten als Service-Level im Anschlussvertrag (vgl. [ADR-0004](./ADR-0004-Bereitstellung-von-Postfach-Backends.md)).

## Referenzen

[^1]: [gematik gemSpec TI-M Basis V1.2.0, §2.1 TI-Messenger Föderation (Registrierungs-Dienst, Föderationsliste, Zulassung)](https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.2.0/), abgerufen am 10. Juni 2026.
