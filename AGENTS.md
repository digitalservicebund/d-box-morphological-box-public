# AGENTS.md

This repository stores a single CSV file, `morph_box.csv`, that represents a morphological box used for decision-making and stakeholder discussion.

This file is meant to be shared across a small engineering team and used as a source for import/export into Miro and other lightweight tooling. AI agents must treat it as a structured data artifact, not as free-form prose.

`sources.md` contains the main pointers to call in context on demand.

## Format check

The technical CSV format is enforced by `tools/check_csv/check_csv.sh` (run automatically as a lefthook pre-commit hook). It checks: UTF-8 without BOM, consistent line endings, comma delimiter, balanced quoting, exact header and column count, `Auswahl` values, no gaps between `Ausprägung` cells, and no Markdown in cell values.

Run it after every edit:

```sh
tools/check_csv/check_csv.sh
```

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
"Zeile","Auswahl","Was wird hier entschieden","Ausprägung 1","Ausprägung 2","Ausprägung 3","Ausprägung 4","Ausprägung 5","Ausprägung 6","Ausprägung 7","Ausprägung 8"
```

Meaning of columns:

- `Zeile`: row label / decision dimension, e.g. `1 Anbindung der Behörde`
- `Auswahl`: decision type. `exklusiv` means only one option can be chosen for this row. `Bündel` means multiple options can be combined.
- `Was wird hier entschieden`: explanatory text describing the decision dimension
- `Ausprägung 1` ... `Ausprägung 8`: possible values / variants for that dimension

Empty cells are valid and should remain empty.

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
