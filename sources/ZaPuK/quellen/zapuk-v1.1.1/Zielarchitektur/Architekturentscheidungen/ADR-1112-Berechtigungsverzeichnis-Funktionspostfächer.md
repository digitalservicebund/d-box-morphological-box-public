# ADR-1112: Berechtigungsverzeichnis für Funktionspostfächer

> **Status:** ✔️ Operativ entschieden | **Datum:** 9. Oktober 2026

## Kontext und Problembeschreibung

Die in den [Funktionspostfächern](../../Glossar.md#funktionspostfach) einer [Organisation](../../Glossar.md#organisation) eingehenden [Nachrichten](../../Glossar.md#nachricht) werden durch Gruppen von Mitarbeitenden und [technischen Nutzern](../../Glossar.md#technischer-nutzer) bearbeitet ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)). Wenn eine Nachricht ein Funktionspostfach erreicht, erzeugt der [Raum-Management-Service](../../Glossar.md#raum-management-service) (RMS) einen [Vorgangsraum](../../Glossar.md#vorgangsraum) und lädt berechtigte Mitarbeitende und ggf. technische Nutzer in diesen Raum ein ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)). Die Zuordnung von Gruppen zu Funktionspostfächern wird ebenso wie die Gruppenzugehörigkeit und die Rechte der [Nutzenden](../../Glossar.md#nutzerinnen) in einem [Berechtigungsverzeichnis](../../Glossar.md#berechtigungsverzeichnis) gepflegt. Der RMS benötigt Zugriff auf dieses Verzeichnis. Dadurch sind auch [Bearbeitungsvertretungen](../../Glossar.md#bearbeitungsvertretung) umsetzbar, bei denen Nutzende zu einem späteren Zeitpunkt und für begrenzte Zeit einer Gruppe automatisiert hinzugefügt werden. Der Betrieb des RMS erfolgt je nach [Betriebsmodell](../../Glossar.md#betriebsmodell) organisationseigen oder zentral durch den Betreiber des zentralen [Postfach-Backends](../../Glossar.md#postfach-backend) ([ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md), [ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)). Im zentralen Betrieb legen die Organisationen ihre Funktionspostfächer in der Administrationsoberfläche des zentralen Postfach-Backends an und verwalten sie dort. Im dezentralen Betrieb obliegt diese Aufgabe der für das Backend verantwortlichen Organisation ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)).

Diese Architekturentscheidung legt die Verortung und die Umsetzungsdetails des Berechtigungsverzeichnisses im zentralen und dezentralen Betriebsmodell fest.

### Abgrenzung

Externe Bevollmächtigungen und gesetzliche Vertretung werden hier nicht behandelt (siehe Folge-Entscheidung).

## Entscheidungstreiber

- **Verzeichnis für Mitgliedschaften:** Es braucht ein Verzeichnis für Postfachmitgliedschaften und ihre Rechte im [Postfach](../../Glossar.md#postfach) (ANF_FUN_ADR_026, ANF_FUN_USE_074, ANF_FUN_ADR_062, ANF_FUN_ADR_071).
- **Betriebsmodelle:** Zentrale und dezentrale Betriebsmodelle sollen unterstützt werden (vgl. [ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)).
- **Organisations-IAM:** Organisationen mit eigener Benutzer- und Rechteverwaltung müssen ihre bestehenden Systeme nutzen können.
- **Vertretungen:** Vertretungen innerhalb der Gruppen müssen abgebildet werden (ANF_FUN_ADR_025).
- **Aktualität:** Änderungen an Rechten und Gruppen müssen sich zeitnah auswirken.
- **Auditierbarkeit:** Alle Änderungen an Zugriffsrechten müssen revisionssicher protokolliert werden (ANF_FUN_ADR_070, SIC_NAC_NAC_005).

## Betrachtete Optionen

### Option A: zentrale Pflege im Org-Konto

Alle Gruppen und Rechte werden im [Org-Konto](../../Glossar.md#org-konto) an zentraler Stelle gepflegt.

- Gut, denn die Verwaltung von [Organisationsidentität](../../Glossar.md#organisationsidentität), Gruppen und Rechten ist zentral an einem Ort möglich.

- Schlecht, denn Organisationen müssten ihre internen Hierarchien und Nutzenden im zentral betriebenen Org-Konto ggf. redundant pflegen.

- Schlecht, denn eine interne Benutzerverwaltung bei einem dezentralen Betriebsmodell ist so nicht umsetzbar.

### Option B: zentrale Pflege im Self-Service-Portal

Alle Gruppen und Rechte werden im [Self-Service-Portal](../../Glossar.md#self-service-portal) an zentraler Stelle gepflegt, in dem auch die Organisationen registriert werden.

- Gut, denn die Verwaltung von Funktionspostfächern, Gruppen und Rechten ist zentral an einem Ort möglich.

- Schlecht, denn Organisationen müssten ihre internen Hierarchien und Nutzenden im zentral betriebenen Self-Service-Portal ggf. redundant pflegen.

- Schlecht, denn eine interne Benutzerverwaltung bei einem dezentralen Betriebsmodell ist so nicht umsetzbar.

### Option C: Berechtigungsverzeichnis als zentrale oder organisationsinterne Komponente

Postfachmitgliedschaften, Rollen und Bearbeitungsvertretungen werden in einem Berechtigungsverzeichnis geführt. Dieses kann je nach Betriebsmodell zentral oder organisationsintern betrieben werden. Organisationen mit eigenem IAM können ihr internes Berechtigungssystem anbinden. Erforderlich ist, dass RMS, zuständige Backends und gegebenenfalls [Org-IdP-Adapter](../../Glossar.md#org-idp-adapter) die für [ZaPuK](../../Glossar.md#zapuk) benötigten Berechtigungsinformationen auswerten können. Das interne IAM muss dafür nicht direkt extern erreichbar sein.

- Gut, denn eine interne Benutzerverwaltung bei einem dezentralen Betriebsmodell ist umsetzbar.

- Gut, denn zentrale und souveräne Betriebsmodelle werden unterstützt.

- Schlecht, denn die Integration eines dezentralen Berechtigungsverzeichnisses verursacht Aufwände in den Organisationen.

## Architekturentscheidung

Gewählte Option: **Option C, Berechtigungsverzeichnis als zentrale oder organisationsinterne Komponente**, weil Organisationen so interne Berechtigungsverzeichnisse mit eigenem Identitätsmanagement verwenden können und ihre Organisationsstrukturen nicht redundant pflegen müssen.

## Umsetzung der gewählten Option

Die Pflege von Berechtigungen ist für Funktionspostfächer möglich und wird vom RMS entsprechend umgesetzt. Der RMS setzt diese Berechtigung für alle Vorgangsräume eines Funktionspostfachs gleich um. Eine individuelle Rechte-Konfiguration für einzelne [Vorgänge](../../Glossar.md#vorgang) ist nicht vorgesehen. Punktuelle Einladungen einzelner Personen durch Raummitglieder bleiben weiterhin möglich, wobei die Implementierung des RMS Änderungen der Gruppe durch Moderatoren im Raum respektieren kann.

Das zentrale Berechtigungsverzeichnis ist Bestandteil des zentralen Postfach-Backends und wird über dessen Administrationsoberfläche gepflegt. Zu jedem zentralen Postfach-Backend für Organisationen wird ein Berechtigungsverzeichnis betrieben (vgl. Folge-Entscheidung). Organisationen im Betriebsmodell A pflegen dort ihre Postfachmitgliedschaften, Rollen und Bearbeitungsvertretungen. Der zentral bereitgestellte RMS pflegt die Mitgliedschaften der Räume ohne E2EE-Teilnehmer zu sein, wobei neuen Mitgliedern nur eingeschränkt Zugriff auf alte Nachrichten des Raums ermöglicht wird (siehe [ADR-1102](./ADR-1102-Umsetzung-von-Funktionspostfächern-und-Vorgangsräumen.md)). Im Betriebsmodell B kann das zentrale Verzeichnis genutzt oder ein organisationsinternes betrieben werden. Ein Funktionspostfach wird im [Adressregister](../../Glossar.md#adressregister) erst aktiviert, wenn mindestens ein Mitglied zugeordnet ist. Sobald das letzte Mitglied entfernt wird, wird der Eintrag deaktiviert ([ADR-1111](./ADR-1111-Auflösung-Organisationsbezogene-Postfachadressen.md)).

Für Organisationen, die eine interne Identitäts- und Zugriffsverwaltung (IAM) nutzen, ist der Betrieb des RMS ebenfalls notwendig ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)). Die im [Verbund](../../Glossar.md#verbund) gültigen [Identitätsnachweise](../../Glossar.md#fachlicher-identitätsnachweis) werden dabei durch den Org-IdP-Adapter ausgestellt, der das interne Berechtigungsverzeichnis ebenfalls nutzen kann ([ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)). Die Umsetzung der Rechte auf den Funktionspostfächern liegt so vollständig und souverän innerhalb der IT-Infrastruktur der Organisation.

Der TI-Messenger der gematik überlässt die interne Berechtigung von Funktionsaccounts dem organisationsinternen IAM und führt kein verbundweites Berechtigungsverzeichnis.[^1]

## Auswirkungen

- Änderungen der Gruppen bzw. Rechte lösen in zugehörigen Vorgangsräumen eine Schlüsselrotation aus. Neue Nachrichten sind somit nur noch durch aktuelle Mitglieder der Räume entschlüsselbar.

- Durch das Berechtigungsverzeichnis im zentralen Postfach-Backend können die Funktionspostfächer auch im Betriebsmodell A ([ADR-1103](./ADR-1103-Mehrbenutzer-Organisationen-Betriebsmodelle.md)) automatisiert verwaltet werden.

## Risiken

- Das Berechtigungsverzeichnis ist eine kritische Komponente und muss entsprechend gehärtet sein. Änderungen sind revisionssicher zu protokollieren. Die Verantwortung liegt beim jeweiligen Betreiber.

- Die Zugriffe auf die Funktionspostfächer werden nicht direkt durch das Berechtigungsverzeichnis, sondern durch den RMS durchgesetzt. Der RMS muss Änderungen an Gruppenzugehörigkeit und Rechten zeitnah umsetzen.

- Auch aus den Gruppen ausgeschiedene Mitglieder behalten weiterhin Zugriff auf Nachrichten vor ihrem Austritt, da sie über die Schlüssel der alten Nachrichten bereits verfügen. Organisatorisch muss dies durch z. B. Löschung auf den Endgeräten mitigiert werden.

- Das zentral bereitgestellte Berechtigungsverzeichnis enthält organisationsinterne Informationen zu Gruppen und Vertretungen. Entsprechend sind Maßnahmen zur Absicherung umzusetzen (z.B. Protokolle, Zugriffsregeln).

## Folge-Entscheidungen

- [Anschlussbedingungen](../../Glossar.md#anschlussbedingungen), siehe [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md).

- Externe Bevollmächtigung und gesetzliche Vertretung, siehe [ADR-1113](./ADR-1113-Externe-Bevollmaechtigung-und-gesetzliche-Vertretung.md).

- Getrennter Betrieb der zentralen Postfach-Backends nach [Teilnehmendengruppen](../../Glossar.md#teilnehmendengruppe), siehe [ADR-1114](./ADR-1114-Getrennter-Betrieb-der-zentralen-Postfach-Backends-nach-Teilnehmendengruppen.md).

## Referenzen

[^1]: [gematik gemSpec TI-M Basis V1.2.0, §2.3.2 (Rolle „Org-Admin”) und §2.5.1 (Authentifizierungs-Dienst für Akteure)](https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.2.0/), abgerufen am 12. Mai 2026.
