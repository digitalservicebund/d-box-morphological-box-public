# d-box Morphological Box

A morphological box (Morphologischer Kasten) for structured decision-making, stored as `morph_box.csv`.

Each row is a decision dimension (`Zeile`) with its options (`Ausprägung`). The file can be imported into and exported from Miro.

## Setup

The CSV format is checked automatically before each commit via [lefthook](https://github.com/evilmartians/lefthook). After cloning, install lefthook and activate the hooks once:

```sh
lefthook install
```

To run the check manually:

```sh
tools/check_csv/check_csv.sh
```

The checker has its own tests, which run automatically before each push. When changing the checker, add a good and a bad test case for each new or changed check as files in `tools/check_csv/tests/fixtures/` (`good/<name>.csv`, or `bad/<name>.csv` plus `bad/<name>.expected` containing the expected error message):

```sh
tools/check_csv/tests/run_tests.sh
```
