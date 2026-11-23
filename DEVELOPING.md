# Developing

The CSV format is checked automatically before each commit via [lefthook](https://github.com/evilmartians/lefthook). After cloning, install lefthook and activate the hooks once:

```sh
lefthook install
```

To run the check manually:

```sh
tools/check_csv/check_csv.sh
```

The default checks `morph_box.csv`; pass `existing_solutions.csv` or `abkuerzungen.csv` to check those files, or pass both file names to check them together. The morphological box uses the clustered header and permits `exklusiv`, `Bündel`, and `gesetzt` in `Auswahl`.

The checker has its own tests, which run automatically before each push. When changing the checker, add a good and a bad test case for each new or changed check in `tools/check_csv/tests/fixtures/` (`good/<name>.csv`, or `bad/<name>.csv` plus `bad/<name>.expected` containing the expected error message). For the solution schema, use `solutions/good/<name>/existing_solutions.csv` or `solutions/bad/<name>/existing_solutions.csv` with an `expected` file in the same directory. For the glossary, use `glossary/good/<name>/abkuerzungen.csv` or `glossary/bad/<name>/abkuerzungen.csv` with an `expected` file in the same directory. For named morphological-box fixtures, use the same structure under `clustered/` with the filename `morph_box.csv`:

```sh
tools/check_csv/tests/run_tests.sh
```
