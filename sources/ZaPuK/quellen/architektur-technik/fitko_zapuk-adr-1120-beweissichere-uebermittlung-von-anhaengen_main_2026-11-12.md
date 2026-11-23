<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/cc150e2977575d0a3e0c065521cdb1656eaae074/Zielarchitektur/Architekturentscheidungen/ADR-1120-Beweissichere-%C3%9Cbermittlung-von-Anh%C3%A4ngen.md -->
<!-- Branch: main, Commit cc150e2977 (2026-11-12); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-1120-Beweissichere-Übermittlung-von-Anhängen.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob cf8b99cb15); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# ADR-1120: Beweissichere Übermittlung von Anhängen

> **Status:** ✔️ Operativ entschieden | **Datum:** 12. November 2026

## Kontext und Problembeschreibung

Zur beweissicheren Übermittlung wird der Eingang einer [Nachricht](../../Glossar.md#nachricht) im [Machtbereich](../../Glossar.md#machtbereich) der [Empfänger](../../Glossar.md#empfängerin) mit einer [Bereitstellungsbestätigung](../../Glossar.md#bereitstellungsbestätigung) quittiert ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). [Anhänge](../../Glossar.md#anhang) werden im Matrix-Protokoll allerdings nicht direkt mit der Nachricht übertragen. Den Empfänger erreicht zunächst nur ein Link (URI) und eine Prüfsumme (Hash) der Datei. Die Datei selbst bleibt physisch im Media-Repository des Absender-Servers und wird vom Empfänger aktiv heruntergeladen (Pull-Prinzip).

![Problem: Pull-Prinzip Matrix](images/ADR-1120-pull-prinzip.svg)

Die Bereitstellungsbestätigung beweist den Eingang der Nachricht mit Link und Hash des Anhangs. Dass die Datei zum Quittierungszeitpunkt tatsächlich abrufbar und unverändert war, ist damit nicht bewiesen. Gegenstand dieser ADR ist die Frage, wie Verfügbarkeit und [Integrität](../../Glossar.md#integrität) von Anhängen beweissicher abgedeckt werden können.

Die mxc-URI und der Hash des Anhangs sind Bestandteil des verschlüsselten Nachrichteninhalts. Ein Dienst des Empfänger-Backends kann sie daher nicht ohne Weiteres extrahieren und die Datei proaktiv laden.

### Abgrenzung

- **[ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md):** Nachweis des Nachrichteneingangs im Machtbereich der Empfänger
- **[ADR-0017](./ADR-0017-Authentizität-und-Integrität-von-Inhalten.md):** Nachweis der [Authentizität](../../Glossar.md#authentizität) des Inhalts von Nachrichten

## Entscheidungstreiber

- **Bekanntgabefiktion:** Die [Bekanntgabefiktion](../../Glossar.md#bekanntgabefiktion) bei elektronischer Übermittlung von [Verwaltungsakten](../../Glossar.md#verwaltungsakt) muss umgesetzt werden (ANF_FUN_ZUS_024).

- **Verschlüsselung:** Die Verschlüsselung darf weder von Betreibern noch von Dritten gebrochen werden (SIC_VER_ÜBE_032).

- **Zero Trust:** Der Zero-Trust-Ansatz ist umzusetzen (SIC_ÜBE_ÜBE_018).

## Betrachtete Optionen

### Option 1: Upload in das Empfänger-Backend (Push)

[Absender](../../Glossar.md#absenderin) übermitteln Anhänge direkt in das Medien-Repository der Empfänger. Vor dem Upload der Datei muss zunächst geprüft werden, ob das Empfänger-Backend diese [Übermittlungsoption](../../Glossar.md#übermittlungsoption) anbietet. Nach einer Prüfung der Absenderidentität autorisiert das Backend den Absender, die Datei in sein Repository zu übermitteln ([ADR-1119](./ADR-1119-Absendergesteuerte-Übermittlungsoptionen-mit-Berechtigungsprüfung.md)). Dadurch kann der Anhang direkt in den Machtbereich des Empfängers gelangen und unabhängig von der Infrastruktur des Absenders heruntergeladen werden.

- Gut, denn durch den Upload gelangen Anhänge bereits beim Versand in den Machtbereich des Empfängers. Sie sind nun unabhängig vom Backend des Absenders durch den Empfänger herunterladbar.

- Gut, denn der Anhang ist nun durch die Bereitstellungsbestätigung des Empfänger-Backends abgedeckt und kann durch den Absender nicht mehr manipuliert werden ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

- Gut, denn die [Ende-zu-Ende-Verschlüsselung](../../Glossar.md#ende-zu-ende-verschlüsselung) bleibt vollständig erhalten.

- Schlecht, denn das Empfänger-Backend wird bereits mit Daten bespielt, selbst wenn diese nicht heruntergeladen werden. Mit Limitierungen (Quotas) müssen Empfänger-Backends gegen das Fluten mit Daten geschützt werden.

- Schlecht, denn ist das Backend des Empfängers nicht verfügbar, ist auch so keine Übermittlung des Anhangs möglich.

- Schlecht, denn das Empfänger-Backend muss einen Endpunkt für den Upload anbieten, die [Autorisierung](../../Glossar.md#autorisierung) umsetzen und entsprechende Kapazitäten zur Verfügung stellen.

- Schlecht, denn diese Lösung per Push ist für ein dezentrales Netzwerk mit vielen Teilnehmern ungeeignet. In Räumen mit [Postfächern](../../Glossar.md#postfach) unterschiedlicher Backends entsteht direkt hoher Netzwerkverkehr. Aus diesem Grund ist in Matrix das Pull-Prinzip umgesetzt.

### Option 2: Unabhängiger Prüfdienst

Auf Nachfrage prüft ein unabhängiger Dienst (Medien-Prüfdienst) die Verfügbarkeit und Integrität des Anhangs in den Repositorys der kommunizierenden Parteien und erzeugt einen signierten Nachweis mit dem Ergebnis. Der Nachweis enthält Statusinformationen und einen Hashwert der verschlüsselten Datei für das jeweilige Repository. Wenn die Hashwerte der geprüften Repositorys gleich sind, gilt der Zugang als bewiesen. Der Absender kann direkt eine Prüfung veranlassen. Dann löst der Prüfdienst einen Pull der Datei durch das Empfänger-Backend aus. Damit wäre der Zugang im Empfänger-Backend direkt beweisbar.

- Gut, denn die Ende-zu-Ende-Verschlüsselung bleibt vollständig erhalten.

- Gut, denn der Prüfdienst erzeugt einen signierten Nachweis, der einen Zugang beweisen kann.

- Schlecht, denn die Netzwerklast wird erhöht, da der Prüfdienst die verschlüsselten Anhänge aus den Repositorys herunterladen muss.

- Schlecht, denn der Prüfdienst kann nur den korrekten Zugang bestätigen. Bei einer Manipulation lässt sich so die verursachende Seite nicht feststellen.

- Schlecht, denn der Nachweis des Prüfdienstes steht erst zur Verfügung, wenn die Datei in allen Repositorys überprüft wurde, was den Übermittlungsprozess verzögert.

- Schlecht, denn ist das Backend des Empfängers nicht verfügbar, ist auch so keine Übermittlung des Anhangs möglich.

- Schlecht, denn der Prüfdienst muss hochverfügbar betrieben werden und hohe Lasten bewältigen. Im System ist er eine kritische Infrastrukturkomponente.

- Schlecht, denn die Backends müssen einen Endpunkt für den Prüfdienst anbieten und eine Autorisierung.

### Option 3: Proaktiver Pull des Empfänger-Backends

Das Empfänger-Backend ruft die Datei beim Absender-Backend direkt nach dem Erhalt der Nachricht ab, die den Link auf den Anhang enthält. Dadurch liegt die Datei bei Quittierung nachweislich im Machtbereich des Empfängers.

Voraussetzung ist, dass das Empfänger-Backend die mxc-URI der Datei kennt. Die mxc-URI ist aber Teil des verschlüsselten Payloads der Nachricht und vom Backend nicht einsehbar. Lösbar ist das nur, indem die mxc-URI im Klartextteil des [Events](../../Glossar.md#event) transportiert wird. Die vorgeschlagene Spezifikationsänderung des Matrixprotokolls MSC4396 (Inline linked media) entwirft eine solche Lösung[^1].

- Gut, denn durch den proaktiven Abruf gelangen Anhänge bereits beim Versand in den Machtbereich des Empfängers. Sie sind nun unabhängig vom Backend des Absenders durch den Empfänger herunterladbar.

- Gut, denn der Anhang ist nun durch die Bereitstellungsbestätigung des Empfänger-Backends abgedeckt und kann durch den Absender nicht mehr manipuliert werden ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

- Gut, denn die Ende-zu-Ende-Verschlüsselung bleibt vollständig erhalten.

- Gut, denn Anhänge können eindeutig einem Raum- und Event-Kontext zugeordnet werden, was z.B. eine spätere Löschung von Räumen vereinfacht und bessere Optionen für passgenaues Rate-Limiting ermöglicht.

- Gut, denn es gibt mit [MSC](../../Glossar.md#msc) 4396 bereits eine Erweiterung der Matrix-Spezifikation zu diesem Zweck.

- Schlecht, denn das Empfänger-Backend wird bereits mit Daten bespielt, selbst wenn diese nicht heruntergeladen werden. Mit Limitierungen (Quotas) müssen Empfänger-Backends gegen das Fluten mit Daten geschützt werden.

- Schlecht, denn die mxc-URIs liegen im Event-Klartext und geben dem Server-Betreiber weitere Metadaten preis.

- Schlecht, denn ist das Backend des Empfängers nicht verfügbar, ist auch so keine Übermittlung des Anhangs möglich.

- Schlecht, denn der proaktive Abruf ist für ein dezentrales Netzwerk mit vielen Teilnehmern ungeeignet. In Räumen mit Postfächern unterschiedlicher Backends entsteht direkt hoher Netzwerkverkehr. Aus diesem Grund ist in Matrix das Pull-Prinzip umgesetzt.

### Option 4: Zentrales Media-Repository

Alle Anhänge werden in einem zentral betriebenen Media-Repository gespeichert, das von allen Backends im [Verbund](../../Glossar.md#verbund) gemeinsam genutzt wird. Die Datei ist unabhängig von der Erreichbarkeit einzelner [Postfach-Backends](../../Glossar.md#postfach-backend) abrufbar.

- Gut, denn das Pull-Problem entfällt. Die Datei ist auch dann erreichbar, wenn der Absender-Server offline ist.

- Gut, denn die Ende-zu-Ende-Verschlüsselung bleibt vollständig erhalten.

- Schlecht, denn die Datei liegt nicht im Machtbereich des Empfängers, sondern im Machtbereich eines zentralen Dritten.

- Schlecht, denn dies widerspricht der hybriden Föderationstopologie aus [ADR-0002](./ADR-0002-Infrastrukturtopologie.md).

- Schlecht, denn es entsteht ein Single Point of Failure und hochattraktives Angriffsziel.

- Schlecht, denn die Konzentration aller Anhänge ermöglicht Metadaten-Korrelation über sämtliche Kommunikationsbeziehungen im Verbund.

## Architekturentscheidung

Gewählt wird:

- **Option 3** (Proaktiver Pull des Empfänger-Backends), weil die Datei direkt in den Machtbereich des Empfängers übermittelt wird und eine Bereitstellungsbestätigung dies unmittelbar quittieren kann. Die Option wird aufgrund der Bindung an den jeweiligen Matrix-Kontext (Raum, Event) gewählt. Die dafür nötige Übertragung der mxc-URI im Klartextteil des Events wird über eine Anpassung des Matrix-Standards umgesetzt, etwa nach MSC4396.[^1] Zusätzliche Dienste sind nicht notwendig und die Ende-zu-Ende-Verschlüsselung bleibt erhalten.

Option 3 wird ggü. **Option 1** bevorzugt, da Option 3 bessere Möglichkeiten zur Einschränkung eines unberechtigten Uploads (z.B. raumspezifisches Rate-Limiting) bietet. **Option 2** wird aufgrund der hohen Komplexität und erhöhter Netzwerklast ausgeschlossen. **Option 4** wird ausgeschlossen, da diese Lösung Metadaten-Korrelation über sämtliche Kommunikationsbeziehungen im Verbund ermöglichen würde.

## Auswirkungen

- Ein proaktiver Pull wird über eine Übermittlungsoption veranlasst ([ADR-1119](./ADR-1119-Absendergesteuerte-Übermittlungsoptionen-mit-Berechtigungsprüfung.md)).

## Risiken

**Option 3 (Pull)**

- Es entstehen weitere Meta-Daten.

- Verzögert sich die Anpassung des Matrix-Standards, verzögert sich die Umsetzung.

- Eine korrekte Übermittlung der verschlüsselten Datei sagt nichts darüber aus, ob die beim Empfänger dann vorliegende Datei korrekt entschlüsselt werden konnte. Der Beweis ist vergleichsweise aufwändig durch Zugriff auf die Schlüssel der Teilnehmer zu führen.

## Folge-Entscheidungen

- [Anschlussbedingungen](../../Glossar.md#anschlussbedingungen): Unterstützung des proaktiven Pulls durch alle Postfach-Backends, siehe [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md).

## Offene Punkte

- Es ist zu entscheiden, ob der proaktive Pull als Übermittlungsoption optional oder mandatorisch umgesetzt wird ([ADR-1119](./ADR-1119-Absendergesteuerte-Übermittlungsoptionen-mit-Berechtigungsprüfung.md)). Gegen mandatorische Umsetzung sprechen Metadaten und Netzwerklast.

## Referenzen

[^1]: [Matrix Spec Proposal MSC4396: Inline linked media](https://github.com/matrix-org/matrix-spec-proposals/pull/4396), abgerufen am 13. Mai 2026.
