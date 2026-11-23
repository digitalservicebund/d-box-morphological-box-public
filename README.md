# d-box Morphological Box

The main element of this repository is a morphological box for structured decision-making about the d-box, a planned central, bidirectional mailbox (Verwaltungspostfach) for the German public administration and part of the Deutschland-Stack.

## Main artifact

[morph_box.csv](morph_box.csv) is the morphological box. Each row is a decision dimension (`Dimension`) with its options (`Ausprägung`). The file can be imported into and exported from Miro.

## Related artifacts

These files build on or support the morphological box:

- [existing_solutions.csv](existing_solutions.csv): compares existing German and international solutions against the dimensions of the morphological box, one column per solution.
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
