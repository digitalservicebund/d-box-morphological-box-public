<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Zielarchitektur/Architekturentscheidungen/ADR-1148-Onboarding-im-Self-Service-Portal.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-1148-Onboarding-im-Self-Service-Portal.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 826caee40e); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# ADR-1148: Onboarding im Self-Service-Portal

> **Status:** ✔️ Operativ entschieden | **Datum:** 12. November 2026

## Kontext und Problembeschreibung

Die Voraussetzung zur Teilnahme am [Verbund](../../Glossar.md#verbund) der Zielarchitektur ist ein Onboarding im [Self-Service-Portal](../../Glossar.md#self-service-portal) ([ADR-0008](./ADR-0008-Einheitliche-Anbindung-ueber-ein-zentrales-Self-Service-Portal.md)) unter Annahme der [Anschlussbedingungen](../../Glossar.md#anschlussbedingungen) ([ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md)). Nach der [Identifizierung](../../Glossar.md#identifizierung) und der Angabe der notwendigen Daten wird der [Anschlussvertrag](../../Glossar.md#anschlussvertrag) geschlossen. [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md) konsolidiert die Anschlussbedingungen und legt fest, wie ihre Einhaltung gewährleistet wird. In diesem ADR wird der Ablauf des Onboardings entworfen.

Die Registrierung erfolgt durch [Organisationen](../../Glossar.md#organisation), die [Postfächer](../../Glossar.md#postfach) im Verbund konfigurieren bzw. nutzen wollen. Eine registrierte Organisation kann in den zentralen [Postfach-Backends](../../Glossar.md#postfach-backend) Postfächer konfigurieren. Darüber hinaus ist auch die Registrierung mehrerer organisationseigener Postfach-Backends möglich ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)). Auch die zentralen Postfach-Backends für [Privatpersonen](../../Glossar.md#privatperson), [private Organisationen](../../Glossar.md#private-organisation), [öffentliche Stellen](../../Glossar.md#öffentliche-stelle) und [Organisationen der Justiz](../../Glossar.md#organisation-der-justiz) ([ADR-1114](./ADR-1114-Getrennter-Betrieb-der-zentralen-Postfach-Backends-nach-Teilnehmendengruppen.md)) werden vom jeweiligen Betreiber über den Onboarding-Prozess registriert. Die Anschlussbedingungen gelten damit einheitlich für alle Teilnehmer.

Während die Auswirkungen einer Fehlkonfiguration von Postfächern begrenzt sind, kann die Aufnahme eines defekten oder kompromittierten Postfach-Backends in die [Föderationsliste](../../Glossar.md#föderationsliste) größeren Schaden verursachen. Andere Postfach-Backends vertrauen der Föderationsliste. Bis das Backend suspendiert wird, können zum Beispiel fehlerhafte [Bereitstellungsbestätigungen](../../Glossar.md#bereitstellungsbestätigung) ausgestellt worden sein. Daraus folgt, dass für die Aufnahme von Organisationen und die Konfiguration ihrer Postfächer eine automatisierte Umsetzung vertretbar ist. Dagegen ist für die Aufnahme der Postfach-Backends in die Föderationsliste zu entscheiden, ob zusätzliche Absicherungen bzw. Prozessschritte notwendig sind.

### Abgrenzung

- In diesem ADR wird der Onboarding-Prozess betrachtet, Details zu den Anschlussbedingungen finden sich in [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md).
- Die Festlegung der [Identifizierungsmittel](../../Glossar.md#authentisierungsmittel) für Organisationen ([Org-Konto](../../Glossar.md#org-konto), Verwaltungs-PKI) findet sich in [ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md) und [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md).
- Anforderungen bzgl. der [Nachweisstellen](../../Glossar.md#nachweisstelle) und [Trust-Liste](../../Glossar.md#trust-liste) finden sich in [ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md) und [ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md). Hier wird festgelegt, in welchem Schritt ihre Registrierung erfolgt.
- Für Privatpersonen gibt es keinen Zugang zum SSP, da sie keine eigenen Backends betreiben und das zentrale Postfach-Backend nutzen ([ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md), [ADR-1114](./ADR-1114-Getrennter-Betrieb-der-zentralen-Postfach-Backends-nach-Teilnehmendengruppen.md)).
- Die Verwaltung von Mitarbeitenden und [Bearbeitungsvertretungen](../../Glossar.md#bearbeitungsvertretung) wird über das [Berechtigungsverzeichnis](../../Glossar.md#berechtigungsverzeichnis) umgesetzt ([ADR-1112](./ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md)). Zur [externen Vertretung](../../Glossar.md#externe-vertretung) siehe [ADR-1113](./ADR-1113-Externe-Bevollmaechtigung-und-gesetzliche-Vertretung.md).

## Entscheidungstreiber

- **Zulässiger Erstkontakt:** Nur registrierte Organisationen können kontaktiert werden (ANF_FUN_ADR_061).

- **Backend-Onboarding:** Ein Postfach-Backend föderiert erst nach einem erfolgreichen Onboarding mit anderen Backends des Verbunds ([ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md)).

- **Org-Konto als IdP:** Organisationen nutzen das Org-Konto zur [Authentifizierung](../../Glossar.md#authentifizierung) (ANF_FUN_IDE_024, ANF_FUN_IDE_050).

- **Self-Service:** Das Onboarding wird als Self-Service umgesetzt und verursacht geringe Aufwände für Teilnehmer ([ADR-0008](./ADR-0008-Einheitliche-Anbindung-ueber-ein-zentrales-Self-Service-Portal.md)).

- **Betriebsmodelle:** Das Onboarding muss alle [Betriebsmodelle](../../Glossar.md#betriebsmodell) abdecken. Ein späterer Wechsel des Betriebsmodells soll ohne erneutes Onboarding möglich sein ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md), [ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md)).

- **Einfacher Einstieg:** Kleine Organisationen, die lediglich eine Adresse benötigen, sollen das Onboarding einfach und schnell durchführen können.

- **Integrierbarkeit:** Die Onboarding-Prozesse sollen über eine API in andere Systeme integriert werden können.

## Betrachtete Optionen

### Option A: Vollautomatische Aufnahme

Alle Schritte des Onboardings werden automatisch geprüft und direkt durchgeführt. Dies gilt auch für die Aufnahme der Postfach-Backends in die Föderationsliste.

- Gut, denn es entstehen keine manuellen Aufwände beim zentralen Support (Governance des Verbunds) für die Aufnahme neuer Backends.

- Schlecht, denn die Aufnahme falscher bzw. fehlerhafter Backends kann erst nachträglich durch eine Sperrung korrigiert werden, wobei der Schaden bereits entstanden sein kann.

- Schlecht, denn nicht alle Bedingungen (z. B. Anforderungen an Betreiber) sind automatisch prüfbar ([ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md)).

### Option B: Manuelle Freigabe für die Aufnahme von Postfach-Backends

Die registrierten Organisationen und konfigurierten Postfächer werden nach einer automatischen Prüfung direkt aktiv. Eine Aufnahme eines Postfach-Backends in die Föderationsliste muss dagegen durch eine zuständige Stelle im Support (Governance des Verbunds) manuell freigegeben werden.

- Gut, denn für die Aufnahme der Organisationen und konfigurierten Postfächer ist keine manuelle Prüfung vorgesehen, was Aufwände und Zeit spart.

- Gut, denn die Aufnahme von Postfach-Backends in die Föderation ist im Rahmen eines nachvollziehbaren Entscheidungsprozesses steuerbar. Auch die Suspendierung und der Ausschluss der Backends werden somit sorgfältig geprüft ([ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md)).

- Schlecht, denn die Aufnahme von Postfach-Backends verursacht manuelle Aufwände und benötigt mehr Zeit im Vergleich zu einer automatischen Freigabe.

### Option C: Umfangreiches Zulassungsverfahren je Postfach-Backend

Die am Verbund aufzunehmenden Postfach-Backends müssen vor einer Teilnahme intensiv geprüft werden. Sowohl die Implementierung als auch Konzepte und Konfigurationen für den Betrieb werden entsprechend untersucht und zugelassen.

Option C wird ausgeschlossen. Entsprechend [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md) werden die Anschlussbedingungen risikobasiert durchgesetzt. Eine Zertifizierung bzw. Auditierung der Postfach-Backends im Rahmen eines umfangreichen Zulassungsverfahrens ist nicht geplant.

## Architekturentscheidung

Gewählte Option: **Option B (Manuelle Freigabe für die Aufnahme von Postfach-Backends)**, weil Aufwände so minimiert werden und nur da entstehen, wo Anschlussbedingungen nicht automatisch prüfbar sind. Dazu gehört die Zulässigkeit von Betreibern. Die Aufnahme der Organisationen selbst und die Freigabe der einzelnen Postfachadressen werden weiterhin automatisiert durchgeführt.

## Umsetzung der gewählten Option

Der Prozess des Onboardings beginnt mit der [Authentisierung](../../Glossar.md#authentisierung) der vertretungsberechtigten Person einer Organisation im SSP über das Org-Konto ([ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md), [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md)). Bei der Registrierung sind ggf. weitere Stammdaten der Organisation zu hinterlegen, die sich nicht automatisch aus der Identifizierung ergeben. Anschließend nimmt die vertretungsberechtigte Person die Anschlussbedingungen inkl. Selbstauskunft an ([ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md)). Die Annahme erfolgt nach der Authentisierung über das Org-Konto. Weitere Prüfungen erfolgen automatisch.

Mit der Registrierung wird somit zeitgleich das Opt-in für die Adressierbarkeit gegeben. Nach Einrichtung und Aktivierung der [Funktionspostfächer](../../Glossar.md#funktionspostfach) wird die Organisation im [Adressregister](../../Glossar.md#adressregister) gelistet und kann ohne weitere Einwilligung kontaktiert werden ([ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)).

Die Registrierungsprozesse sind über eine API in andere Prozesse (z. B. Unternehmensgründung, Kontoeröffnung bei den [Authentisierungssystemen](../../Glossar.md#authentisierungssystem)) einfach integrierbar.

![Rollen, Domains und Föderationsliste](images/ADR-1148-rollen-domains-foederationsliste.svg)

Registrierte Organisationen konfigurieren ihre Funktionspostfächer und pflegen die Adresseinträge im Adressregister in der Administrationsoberfläche des zentralen Postfach-Backends ([ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)), wobei die Eindeutigkeit der Organisationspräfixe automatisch sichergestellt wird ([ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md)). Zudem registrieren sie ihre technischen Systeme, zum Beispiel [RMS](../../Glossar.md#raum-management-service) und angebundene [Fachverfahren](../../Glossar.md#fachverfahren), mit den zugehörigen Schlüsseln ([ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md)). Verfügt eine Organisation über ein internes Identitätsmanagement, registriert sie ihren [Org-IdP-Adapter](../../Glossar.md#org-idp-adapter), der als Nachweisstelle in die Trust-Liste übernommen wird ([ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)). Auch das erfolgt automatisch, da diese Nachweisstelle über ihren Eintrag in der Trust-Liste nur Personen der eigenen Organisation ausweisen darf, wodurch der mögliche Schaden begrenzt ist. Der Adapter darf dabei höchstens das Vertrauensniveau des Onboardings bescheinigen. Ein höheres Vertrauensniveau wäre ggf. durch eine manuelle Freigabe der zuständigen Stelle möglich.

Betreiber registrieren je Postfach-Backend eine Adress-Domain sowie die öffentlichen Schlüssel für die jeweiligen Zwecke, wie das an den Betreiber gebundene Siegelzertifikat für die Bereitstellungsbestätigungen ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md), [ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md)) und aktivieren optionale Funktionen, zum Beispiel zusätzliche [Übermittlungsoptionen](../../Glossar.md#übermittlungsoption) ([ADR-1119](./ADR-1119-Absendergesteuerte-Übermittlungsoptionen-mit-Berechtigungsprüfung.md), [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md)). Jede Adress-Domain ist einem Inhaber zugeordnet. Der Bezug der Adressen zur Organisation wird im Adressregister geführt. Im souveränen Betrieb des Backends wird der Bezug über die Domain hergestellt. Beim Betrieb im zentralen Backend werden die Organisationen mit den Adressen bzw. dem Präfix verknüpft ([ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md), [ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)). Bei der Registrierung werden die Kontrolle über die Domain und der Besitz der privaten Schlüssel automatisch geprüft. Wird das Backend im Auftrag für die Organisation durch einen Dienstleister betrieben, muss die Organisation im SSP zustimmen und die Kontrolle der Domain somit an den Betreiber übergeben. Durch die abschließende manuelle Freigabe der zuständigen Stelle wird der Eintrag in der Föderationsliste aktiv. Bei zentral betriebenen Backends wird dabei auch die Instanz der zentralen Nachweisstelle in die Trust-Liste eingetragen ([ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)). Sie stellt Nachweise für alle Teilnehmer des Backends aus, ihre Aufnahme ist deshalb an die manuelle Freigabe gekoppelt. Im Prozess sind Anschlussverträge für den Betrieb der Postfach-Backends mit dem Betreiber zu schließen.

![Ablauf des Onboardings im Self-Service-Portal](images/ADR-1148-onboarding-ablauf.svg)

Der Umfang der im SSP zu registrierenden Systeme und vorzunehmenden Konfigurationen hängt vom Betriebsmodell ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)) und von der Rolle als Betreiber ab.

| Betriebsmodell          | SSP Registrierung                                                                                        |
|-------------------------|----------------------------------------------------------------------------------------------------------|
| A, Minimalanbindung     | [Organisationsidentität](../../Glossar.md#organisationsidentität), Vertragsannahme, Organisationspräfix                                             |
| B, Teilbetrieb          | zusätzlich eigene Systeme wie Berechtigungsverzeichnis, RMS und ggf. Org-IdP-Adapter                     |
| C, souveräner Betrieb   | zusätzlich Adress-Domain, Schlüssel des Backends und optionale Funktionen                                |
| Fremdbetrieb für Dritte | Organisationsidentität und Vertragsannahme, dazu je betriebenem Backend Domain, Schlüssel und Funktionen |

Bei einem Wechsel des Betriebsmodells in den souveränen Betrieb mit eigener Domain muss die Migration der Adressen eingeplant werden ([ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md), [ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)).

Wechselt der Betreiber, bleiben Domain bzw. Adresspräfix der Organisation zugeordnet. Eine Migration der Adressen ist nur bei einem Wechsel der Domain notwendig ([ADR-1104](./ADR-1104-MXID-Struktur-Domain-Governance.md), [ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)).

Modular: Auch andere Basisdienste wie FIT-Connect oder die FöPD nutzen Self-Service-Portale für die Registrierung der Organisationen. Ziel ist, dass sich Organisationen einmal registrieren und den Anschluss an mehrere Basisdienste in einem übergreifenden SSP verwalten können. Das SSP wird so entworfen, dass es als Modul in ein zentrales Portal aufgenommen werden könnte, aber auch allein lauffähig ist. Dafür werden die SSP-Funktionen über APIs angeboten. Für einen Betrieb in einem zentralen SSP wären zunächst Vertrags- und Governance-Fragen wie z. B. bzgl. des Supports übergreifend abzustimmen.

Zentrale Backends: Auch die zentralen Postfach-Backends werden durch ihre Betreiber im SSP registriert. Voraussetzung ist ein Betreibervertrag bzw. eine Verwaltungsvereinbarung, die anstelle des Anschlussvertrags bzw. ergänzend zu ihm geschlossen wird.

Der TI-Messenger der gematik hat mit dem Registrierungs-Dienst und der signierten Föderationsliste ein Konzept, das für den Entwurf hier als Vorlage dienen kann.[^1]

## Auswirkungen

- Einträge in der Föderationsliste werden ausschließlich über den Freigabeschritt vorgenommen.

- Die zentralen und dezentralen Backends werden einheitlich über die Föderationsliste gesteuert ([ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md)).

- Organisationen ohne eigenes Backend nehmen ohne manuelle Freigabe am Verbund teil.

- Die Onboarding-Prozesse sind per API in andere Prozesse (z. B. Unternehmensgründung) einfach integrierbar.

## Risiken

- Ein Ausfall des Org-Kontos blockiert die Registrierung neuer Organisationen.

- Eine manuelle Freigabe kann den Anschluss neuer Backends verzögern.

## Offene Punkte

- Benennung der zuständigen Stelle für die Freigabe (vgl. [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md)).

- Betreiber-Kriterien (vgl. [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md)).

- Identifizierungsmittel für öffentliche Stellen, insbesondere die Nutzung der Verwaltungs-PKI ([ADR-0015](./ADR-0015-Authentizität-Identität-öffentlicher-Stellen.md), [ADR-0016](./ADR-0016-Authentizität-Identität-privater-Organisationen.md)).

- Weiterer Funktionsumfang des SSP (vgl. [ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md), [ADR-1112](./ADR-1112-Berechtigungsverzeichnis-Funktionspostfächer.md)).

- Klärung zu einem übergreifenden Self-Service-Portal, ggf. im Rahmen des D-Stack (FIT-Connect, FöPD und [BundID](../../Glossar.md#bundid)).

- Verträge für Betreiber und Organisationen.

- Prozess zum Nachweis der Inhaberschaft bzw. Kontrolle von Domains ([ADR-1144](./ADR-1144-PKI-Anforderungen-und-Vertrauensinfrastruktur.md)).

## Referenzen

[^1]: [gematik gemSpec TI-M Basis V1.2.0, §3.2.1 (Registrierungs-Dienst mit I_Registration, Administrations-Account und Föderationsliste)](https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.2.0/), abgerufen am 24. August 2026.
