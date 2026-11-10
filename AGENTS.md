# AGENTS.md

This repository stores a single CSV file, `morph_box.csv`, that represents a morphological box used for decision-making and stakeholder discussion.

This file is meant to be shared across a small engineering team and used as a source for import/export into Miro and other lightweight tooling. AI agents must treat it as a structured data artifact, not as free-form prose.

## Core rules

- Work primarily in `morph_box.csv`.
- Preserve the CSV as valid UTF-8 encoded text.
- Preserve the exact column order and header names.
- Do not reorder rows unless explicitly requested.
- Keep the original row numbering and labels unless there is a clear, explicit task to change them.
- Preserve the German language and domain terminology already used in the table.
- Keep the table machine-readable: no markdown tables, no prose inserted into the CSV, no extra columns.
- Output strictly plain text inside the CSV cells. Do not use Markdown formatting (for example, `**bold**`, `*italics*`, or bullet lists) inside the cell values.

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

Important:

- Empty cells are valid and should remain empty.
- Trailing empty values may be left blank.
- Do not insert new columns.
- Do not rename columns.
- Strictly use a comma (`,`) as the column delimiter, never a semicolon (`;`).
- Do not convert the CSV into Excel-specific formatting or other locale-specific output.

## Editing policy

When changing values:

- Prefer minimal, targeted edits.
- Keep the original semantics unless the task explicitly asks for a change.
- If a row's meaning is corrected or clarified, do so in the same style as the existing document.
- Preserve punctuation, capitalization, and wording wherever possible.
- If adding a new option, place it in the next available `Ausprägung N` slot and leave unused trailing cells empty.
- If removing an option, do not leave dangling commas or malformed quoting; retain valid CSV syntax.

## Miro import/export compatibility

Because this repository is used to move content between the CSV and Miro, the CSV must remain import-friendly:

- Use plain CSV quoting rules with double quotes around values that contain commas.
- Preserve embedded commas, quotes, and apostrophes correctly.
- If a cell value requires line breaks, the entire cell value MUST be wrapped in double quotes.
- Do not wrap the entire file in markdown fences or add comments inside the CSV.
- Keep newline handling stable; standard LF or CRLF is acceptable as long as the file remains valid CSV.
- Strictly use a comma (`,`) as the column delimiter, never a semicolon (`;`).
- Avoid locale-specific formatting such as decimal separators, date formatting, or semicolon-based CSV.

## Safe AI workflow

Before editing:

1. Read the relevant row(s) in `morph_box.csv`.
2. Check the surrounding rows to preserve context and terminology.
3. Confirm the intended change is consistent with the row's decision category.
4. Edit only the necessary cells.

After editing:

1. Verify the CSV still parses as valid CSV.
2. Verify headers and column count remain unchanged.
3. Ensure quotes are balanced and no rows are malformed.
4. Keep the file easy to import into Miro without manual cleanup.

## What not to do

- Do not rewrite the whole file for style alone.
- Do not reorder columns or rows casually.
- Do not add YAML/JSON/TOML metadata inside the CSV.
- Do not convert text to English unless explicitly requested.
- Do not create new files unless clearly needed for a workflow helper.
- Do not add secrets, credentials, or environment-specific config to the repository.
- Do not use Markdown formatting inside cell values.

## When in doubt

Prefer stability over elegance. This file is a collaborative decision artifact; preserving structure and meaning is more important than cosmetic cleanup.

The goal is to keep `morph_box.csv` readable by both humans and AI systems while remaining easy to import/export into Miro and similar visual collaboration tools.
