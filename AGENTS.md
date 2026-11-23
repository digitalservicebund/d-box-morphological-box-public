# AGENTS.md

This repository stores a morphological box in `morph_box.csv` and a comparison of existing solutions in `existing_solutions.csv`. They support decision-making and stakeholder discussion.

These files are meant to be shared across a small engineering team and used with Miro and other lightweight tooling. AI agents must treat them as structured data artifacts, not as free-form prose.

`sources.md` contains the main pointers to call in context on demand.

## Format check

The technical format of both CSV files is enforced by `tools/check_csv/check_csv.sh` (run automatically as a lefthook pre-commit hook). It checks UTF-8 without BOM, consistent line endings, CSV syntax, column count and Markdown. For `morph_box.csv`, it also checks the exact header, `Auswahl` values and gaps between `Ausprägung` cells. For `existing_solutions.csv`, it checks the `Dimension` header and nonempty, unique solution names.

Run it after every edit:

```sh
tools/check_csv/check_csv.sh
```

Pass `existing_solutions.csv` explicitly to check it; the default is `morph_box.csv`.

## Core rules

- Work primarily in `morph_box.csv`.
- Do not reorder rows unless explicitly requested.
- Keep the original row numbering and labels unless there is a clear, explicit task to change them.
- Preserve the German language and domain terminology already used in the table.
- Keep the table machine-readable: no prose inserted into the CSV outside of cells.

## CSV schema

The file uses a standard CSV layout with a header row and one data row per option cluster.

Header:

```csv
"Cluster","Dimension","Auswahl","Wer entscheidet","Was wird hier entschieden","Ausprägung 1","Ausprägung 2","Ausprägung 3","Ausprägung 4","Ausprägung 5","Ausprägung 6","Ausprägung 7","Ausprägung 8"
```

Meaning of columns:

- `Cluster`: group of related decision dimensions
- `Dimension`: row label / decision dimension, e.g. `B1 Anbindung von Fachverfahren`
- `Auswahl`: decision type. `exklusiv` means only one option can be chosen, `Bündel` means multiple options can be combined, and `gesetzt` indicates a predetermined value.
- `Wer entscheidet`: decision-maker, if specified
- `Was wird hier entschieden`: explanatory text describing the decision dimension
- `Ausprägung 1` ... `Ausprägung 8`: possible values / variants for that dimension

Empty cells are valid and should remain empty.

## Existing solutions

`existing_solutions.csv` compares existing German and international solutions against the dimensions in `morph_box.csv`. Its first column, `Dimension`, contains the same dimension labels in the same order as the morphological box; each subsequent column represents one solution or country. Each cell describes how that solution addresses the dimension, potentially including qualifications or several aspects. It is not required to reproduce an `Ausprägung` verbatim.

Keep all solution columns and existing information when aligning dimensions, unless explicitly asked otherwise. When dimensions change, move or combine information only where it fits the current dimension. Leave cells for new dimensions empty unless an existing value or the current morphological box makes the assignment unambiguous. An empty cell means no assignment is recorded, not necessarily that the solution lacks the feature; preserve explicit uncertainty such as `?`.

The file has its own CSV schema, not the `Auswahl` and `Ausprägung` columns of `morph_box.csv`. Maintain valid UTF-8 CSV with a consistent column count and verify its dimension labels and order against `morph_box.csv` after changes.

Document the evidence for each researched solution in `sources/Referenzen-<Name>.md`, modelled on `sources/Referenzen-Mein-Postkorb-Oesterreich.md`: a `Hauptquellen` table with a short code (`Kürzel`) and link per source, then a `Nachweise je Dimension` table mapping each `Dimension` to its statement and source codes. Mark the evidence level of each statement in both the source file and the CSV cell: unmarked statements are level 3 (primary source checked in full text), `[Indiz]` is level 2 (secondary source, vendor claim, press release, outdated information, inference from a primary source, or no finding after a targeted search), `[Annahme]` is level 1 (no evidence, to be checked). Do not use `[unverifiziert]` in the CSV cells any more. The source file records the level per source (`Hauptquellen`) and per statement (`Nachweise je Dimension`, column `Stufe`) and lists every marked CSV statement with its reason and how to check it in the section `Evidenzstufen der markierten Stellen`. Statements about a solution without a source file are `[Annahme]`. Register the file in `sources.md`, both in the overview table and in the topic table.

## Abbreviations

`abkuerzungen.csv` explains abbreviations used in both CSV files, with the header `"Abkürzung","Bedeutung"`, one row per abbreviation, sorted alphabetically regardless of case. When you introduce a new abbreviation in a CSV, add it there. Mark unconfirmed meanings with `[unverifiziert]` and unknown ones with `?`; do not leave `Bedeutung` empty. Product names (e.g. BundID, MitID, FIT-Connect) and ADR numbers do not need entries. The checker validates this file under its own schema.

## Editing policy

When changing values:

- Prefer minimal, targeted edits.
- Keep the original semantics unless the task explicitly asks for a change.
- If a row's meaning is corrected or clarified, do so in the same style as the existing document.
- Preserve punctuation, capitalization, and wording wherever possible.
- If adding a new option, place it in the next available `Ausprägung N` slot and leave unused trailing cells empty.
- If removing an option, shift the following options left so no gap remains.

## Miro import/export compatibility

Because this repository is used to move content between the CSV and Miro, the CSV must remain import-friendly:

- Do not add comments inside the CSV.
- Avoid locale-specific formatting such as decimal separators or date formatting.

## Safe AI workflow

Before editing:

1. Read the relevant row(s) in `morph_box.csv`.
2. Check the surrounding rows to preserve context and terminology.
3. Confirm the intended change is consistent with the row's decision category.
4. Edit only the necessary cells.

After editing:

1. Run `tools/check_csv/check_csv.sh` and fix all reported errors.

## What not to do

- Do not rewrite the whole file for style alone.
- Do not reorder columns or rows casually.
- Do not add YAML/JSON/TOML metadata inside the CSV.
- Do not convert text to English unless explicitly requested.
- Do not create new files unless clearly needed for a workflow helper.
- Do not add secrets, credentials, or environment-specific config to the repository.

## When in doubt

Prefer stability over elegance. This file is a collaborative decision artifact; preserving structure and meaning is more important than cosmetic cleanup.

The goal is to keep `morph_box.csv` readable by both humans and AI systems while remaining easy to import/export into Miro and similar visual collaboration tools.
