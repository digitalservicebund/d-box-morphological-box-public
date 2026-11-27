# d-box Morphologischer Kasten

[English version below](#english)

Kern dieses Repositorys ist ein morphologischer Kasten für strukturierte Entscheidungen zur d-box, einem geplanten zentralen, bidirektionalen Verwaltungspostfach für die deutsche öffentliche Verwaltung und Teil des Deutschland-Stacks.

## Hauptartefakt

[morph_box.csv](morph_box.csv) ist der morphologische Kasten. Jede Zeile ist eine Entscheidungsdimension (`Dimension`) mit ihren Ausprägungen (`Ausprägung`). Die Datei kann in Miro importiert und aus Miro exportiert werden.

## Weitere Artefakte

Diese Dateien bauen auf dem morphologischen Kasten auf oder ergänzen ihn:

- [existing_solutions.csv](existing_solutions.csv): vergleicht bestehende deutsche und internationale Lösungen anhand der Dimensionen des morphologischen Kastens, eine Spalte je Lösung.
- [vergleich_betrieb/](vergleich_betrieb): vergleicht drei Betriebsmodelle (ein staatlicher Betreiber, staatlich-föderierter Verbund, offener Betreiberverbund) in einer kompakten und einer detaillierten Tabelle, ergänzt um eine Einordnung zu Art. 91c GG.
- [vertraulichkeit/](vertraulichkeit): vergleicht Vertraulichkeitsstufen von Transportverschlüsselung bis Ende-zu-Ende-Verschlüsselung bis zum Endgerät: wer welche Inhalte und Metadaten lesen kann, Funktionsumfang, Aufwand und ZaPuK-Status. Dazu eine Kurzfassung und stufenübergreifende Querschnittsthemen wie Schlüssel-Backup.
- [zentrale_dienste.csv](zentrale_dienste.csv): listet zentrale Dienste (z. B. BundID) mit ihren Funktionen, der zugehörigen Dimension des Kastens, Betreiber, Einschränkungen und Quellen.
- [abkuerzungen.csv](abkuerzungen.csv): erklärt die in `morph_box.csv` und `existing_solutions.csv` verwendeten Abkürzungen (Spalten `Abkürzung`, `Bedeutung`).
- [bewertungskriterien.md](bewertungskriterien.md): enthält Fragen zur Bewertung eines gewählten Lösungswegs. Die Kriterien sind keine zusätzlichen Ausprägungen.

## Quellen

[sources/](sources) enthält die Belege für den morphologischen Kasten und den Vergleich bestehender Lösungen:

- Je untersuchter Lösung ein Ordner (z. B. `BundID-ZBP/`, `ELSTER/`, `Digital-Post-Daenemark/`) mit einer verdichteten Analyse und lokalen Kopien von Primärquellen wie Gesetzen, Spezifikationen und offizieller Dokumentation.
- Je Lösung eine Datei `Referenzen-<Name>.md`, die die Hauptquellen mit Kürzeln auflistet und jeder Dimension ihre Aussage, Quellen und Evidenzstufe zuordnet. Die Evidenzstufen sind auch in den CSV-Zellen markiert: Unmarkierte Aussagen wurden in einer Primärquelle geprüft, `[Indiz]` stützt sich auf schwächere Belege und `[Annahme]` ist noch zu prüfen.
- Spiegel der ZaPuK-Zielarchitektur (`zapuk-main/`) und von Neo, ihrer Matrix-basierten Referenzimplementierung (`neo-solution-architecture-develop/`).

[sources.md](sources.md) ist die Übersicht über diesen Ordner. Sie beschreibt jeden Ordner und jede Datei und wofür sie sich eignen.

## Einrichtung

Siehe [DEVELOPING.md](DEVELOPING.md) für die Formatprüfung, Git-Hooks und die Tests des Prüfskripts.

## Sicherheit

Sicherheitslücken meldest du bitte nicht über GitHub-Issues, sondern wie in [SECURITY.md](SECURITY.md) beschrieben.

---

<a id="english"></a>

# d-box Morphological Box

The main element of this repository is a morphological box for structured decision-making about the d-box, a planned central, bidirectional mailbox (Verwaltungspostfach) for the German public administration and part of the Deutschland-Stack.

## Main artifact

[morph_box.csv](morph_box.csv) is the morphological box. Each row is a decision dimension (`Dimension`) with its options (`Ausprägung`). The file can be imported into and exported from Miro.

## Related artifacts

These files build on or support the morphological box:

- [existing_solutions.csv](existing_solutions.csv): compares existing German and international solutions against the dimensions of the morphological box, one column per solution.
- [vergleich_betrieb/](vergleich_betrieb): compares three operating models (a single state operator, a state-run federated network, an open network of operators) in a compact and a detailed table, plus an assessment of Article 91c of the Basic Law (GG).
- [vertraulichkeit/](vertraulichkeit): compares confidentiality levels from transport encryption to end-to-end encryption to the device: who can read which content and metadata, functional scope, effort and ZaPuK status. Also includes a short version and cross-cutting topics such as key backup.
- [zentrale_dienste.csv](zentrale_dienste.csv): lists central services (e.g. BundID) with their functions, the related dimension of the box, operator, limitations and sources.
- [abkuerzungen.csv](abkuerzungen.csv): explains the abbreviations used in `morph_box.csv` and `existing_solutions.csv` (columns `Abkürzung`, `Bedeutung`).
- [bewertungskriterien.md](bewertungskriterien.md): lists questions for assessing a chosen solution path. The criteria are not additional options.

## Sources

[sources/](sources) holds the evidence behind the morphological box and the comparison of existing solutions:

- One folder per researched solution (e.g. `BundID-ZBP/`, `ELSTER/`, `Digital-Post-Daenemark/`) with a condensed analysis and local copies of primary sources such as laws, specifications and official documentation.
- One `Referenzen-<Name>.md` file per solution, listing the main sources with short codes and mapping each dimension to its statement, sources and evidence level. The evidence levels are also marked in the CSV cells: unmarked statements were checked in a primary source, `[Indiz]` rests on weaker evidence, and `[Annahme]` is still to be checked.
- Mirrors of the ZaPuK target architecture (`zapuk-main/`) and of Neo, its Matrix-based reference implementation (`neo-solution-architecture-develop/`).

[sources.md](sources.md) is the navigation map for this folder. It describes each folder and file and what to use it for.

## Setup

See [DEVELOPING.md](DEVELOPING.md) for the format check, Git hooks and checker tests.

## Security

Please do not report security vulnerabilities through GitHub issues; follow [SECURITY.md](SECURITY.md) instead.
