#!/usr/bin/env bash
# Tests für check_csv.sh.
#
# Testdaten liegen in fixtures/:
#   good/<name>.csv       muss vom Checker als OK akzeptiert werden
#   bad/<name>.csv        muss vom Checker abgelehnt werden
#   bad/<name>.expected   Text, der in der Fehlermeldung vorkommen muss
#
# Neuer Testfall: Datei in good/ oder bad/ anlegen (bei bad/ mit .expected).
# Die Fixtures enthalten absichtlich kaputte Bytes (BOM, CRLF, ungültiges
# UTF-8); .gitattributes und .editorconfig in fixtures/ schützen sie davor,
# von Git oder Editoren verändert zu werden.
#
# Aufruf:  tools/check_csv/tests/run_tests.sh

set -uo pipefail

tests_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
checker="$tests_dir/../check_csv.sh"
fixtures="$tests_dir/fixtures"

pass=0
fail=0

ok()   { pass=$((pass + 1)); printf '  ok    %s\n' "$1"; }
nok()  { fail=$((fail + 1)); printf '  FAIL  %s: %s\n' "$1" "$2"; }

expect_ok() {
  local name="$1" file="$2" out
  if out=$("$checker" "$file" 2>&1); then ok "$name"
  else nok "$name" "erwartet OK, bekam:"$'\n'"$out"; fi
}

expect_error() {
  local name="$1" file="$2" msg="$3" out
  if out=$("$checker" "$file" 2>&1); then nok "$name" "erwartet Fehler \"$msg\", bekam OK"
  elif [[ "$out" != *"$msg"* ]]; then nok "$name" "erwartet Fehler \"$msg\", bekam:"$'\n'"$out"
  else ok "$name"; fi
}

printf 'Echte Datei\n'
expect_ok morph_box "$tests_dir/../../../morph_box.csv"
expect_ok existing_solutions "$tests_dir/../../../existing_solutions.csv"
expect_ok abkuerzungen "$tests_dir/../../../abkuerzungen.csv"
expect_ok vergleich_detailliert "$tests_dir/../../../vergleich_betrieb/detailliert.csv"
expect_ok vergleich_kompakt "$tests_dir/../../../vergleich_betrieb/kompakt.csv"
expect_error vergleich_spaltenzahl "$fixtures/comparison/bad/column_count/vergleich_betrieb/kompakt.csv" "erwartet 4 Spalten, gefunden 3"
expect_error vergleich_header "$fixtures/comparison/bad/header/vergleich_betrieb/detailliert.csv" "Header Spalte 1"
expect_error vergleich_markdown "$fixtures/comparison/bad/markdown/vergleich_betrieb/kompakt.csv" "Markdown-Formatierung"
expect_ok vertraulichkeit_stufen "$tests_dir/../../../vertraulichkeit_stufen.csv"
expect_ok vertraulichkeit_kurzfassung "$tests_dir/../../../vertraulichkeit_kurzfassung.csv"
expect_ok vertraulichkeit_querschnittsthemen "$tests_dir/../../../vertraulichkeit_querschnittsthemen.csv"
expect_error datei_fehlt "$fixtures/gibt_es_nicht.csv" "Datei nicht gefunden"
if out=$("$checker" 2>&1); then ok standard_morph_box
else nok standard_morph_box "erwartet OK, bekam:"$'\n'"$out"; fi

printf '\nMehrere Dateien in einem Aufruf\n'
# Ein Fehler in einer Datei muss den Gesamtaufruf scheitern lassen,
# egal ob die fehlerhafte Datei zuerst oder zuletzt geprüft wird.
expect_multi() {
  local name="$1" msg="$2" out
  shift 2
  if out=$("$checker" "$@" 2>&1); then nok "$name" "erwartet Fehler \"$msg\", bekam OK"
  elif [[ "$out" != *"$msg"* ]]; then nok "$name" "erwartet Fehler \"$msg\", bekam:"$'\n'"$out"
  else ok "$name"; fi
}
expect_multi mehrere_schlecht_zuerst "UTF-8-BOM" "$fixtures/bad/bom.csv" "$fixtures/good/nur_lf.csv"
expect_multi mehrere_schlecht_zuletzt "UTF-8-BOM" "$fixtures/good/nur_lf.csv" "$fixtures/bad/bom.csv"
if out=$("$checker" "$fixtures/good/nur_lf.csv" "$fixtures/good/nur_crlf.csv" 2>&1); then ok mehrere_gut
else nok mehrere_gut "erwartet OK, bekam:"$'\n'"$out"; fi
if out=$("$checker" "$tests_dir/../../../morph_box.csv" "$tests_dir/../../../existing_solutions.csv" 2>&1); then ok mehrere_schemas
else nok mehrere_schemas "erwartet OK, bekam:"$'\n'"$out"; fi

printf '\nGute Daten (fixtures/good)\n'
for file in "$fixtures"/good/*.csv; do
  expect_ok "$(basename "$file" .csv)" "$file"
done

printf '\nSchlechte Daten (fixtures/bad)\n'
for file in "$fixtures"/bad/*.csv; do
  name="$(basename "$file" .csv)"
  expected="${file%.csv}.expected"
  if [[ ! -f "$expected" ]]; then
    nok "$name" "fehlende Datei bad/$name.expected"
    continue
  fi
  expect_error "$name" "$file" "$(<"$expected")"
done

for expected in "$fixtures"/bad/*.expected; do
  [[ -f "${expected%.expected}.csv" ]] || nok "$(basename "$expected")" "zugehörige .csv fehlt"
done

printf '\nDaten für existing_solutions.csv\n'
for file in "$fixtures"/solutions/good/*/existing_solutions.csv; do
  expect_ok "solutions_$(basename "$(dirname "$file")")" "$file"
done
for file in "$fixtures"/solutions/bad/*/existing_solutions.csv; do
  name="$(basename "$(dirname "$file")")"
  expected="$(dirname "$file")/expected"
  if [[ ! -f "$expected" ]]; then
    nok "solutions_$name" "fehlende Datei $expected"
    continue
  fi
  expect_error "solutions_$name" "$file" "$(<"$expected")"
done

printf '\nDaten für morph_box.csv\n'
for file in "$fixtures"/clustered/good/*/morph_box.csv; do
  expect_ok "morph_$(basename "$(dirname "$file")")" "$file"
done
for file in "$fixtures"/clustered/bad/*/morph_box.csv; do
  name="$(basename "$(dirname "$file")")"
  expected="$(dirname "$file")/expected"
  if [[ ! -f "$expected" ]]; then
    nok "morph_$name" "fehlende Datei $expected"
    continue
  fi
  expect_error "morph_$name" "$file" "$(<"$expected")"
done

printf '\nDaten für abkuerzungen.csv\n'
for file in "$fixtures"/glossary/good/*/abkuerzungen.csv; do
  expect_ok "glossary_$(basename "$(dirname "$file")")" "$file"
done
for file in "$fixtures"/glossary/bad/*/abkuerzungen.csv; do
  name="$(basename "$(dirname "$file")")"
  expected="$(dirname "$file")/expected"
  if [[ ! -f "$expected" ]]; then
    nok "glossary_$name" "fehlende Datei $expected"
    continue
  fi
  expect_error "glossary_$name" "$file" "$(<"$expected")"
done

printf '\nDaten für zentrale_dienste.csv\n'
for file in "$fixtures"/services/good/*/zentrale_dienste.csv; do
  expect_ok "services_$(basename "$(dirname "$file")")" "$file"
done
for file in "$fixtures"/services/bad/*/zentrale_dienste.csv; do
  name="$(basename "$(dirname "$file")")"
  expected="$(dirname "$file")/expected"
  if [[ ! -f "$expected" ]]; then
    nok "services_$name" "fehlende Datei $expected"
    continue
  fi
  expect_error "services_$name" "$file" "$(<"$expected")"
done

printf '\nDaten für vertraulichkeit_stufen.csv\n'
for file in "$fixtures"/levels/good/*/vertraulichkeit_stufen.csv; do
  expect_ok "levels_$(basename "$(dirname "$file")")" "$file"
done
for file in "$fixtures"/levels/bad/*/vertraulichkeit_stufen.csv; do
  name="$(basename "$(dirname "$file")")"
  expected="$(dirname "$file")/expected"
  if [[ ! -f "$expected" ]]; then
    nok "levels_$name" "fehlende Datei $expected"
    continue
  fi
  expect_error "levels_$name" "$file" "$(<"$expected")"
done

printf '\nDaten für vertraulichkeit_kurzfassung.csv\n'
for file in "$fixtures"/levels_short/good/*/vertraulichkeit_kurzfassung.csv; do
  expect_ok "levels_short_$(basename "$(dirname "$file")")" "$file"
done
for file in "$fixtures"/levels_short/bad/*/vertraulichkeit_kurzfassung.csv; do
  name="$(basename "$(dirname "$file")")"
  expected="$(dirname "$file")/expected"
  if [[ ! -f "$expected" ]]; then
    nok "levels_short_$name" "fehlende Datei $expected"
    continue
  fi
  expect_error "levels_short_$name" "$file" "$(<"$expected")"
done

printf '\nDaten für vertraulichkeit_querschnittsthemen.csv\n'
for file in "$fixtures"/crosscut/good/*/vertraulichkeit_querschnittsthemen.csv; do
  expect_ok "crosscut_$(basename "$(dirname "$file")")" "$file"
done
for file in "$fixtures"/crosscut/bad/*/vertraulichkeit_querschnittsthemen.csv; do
  name="$(basename "$(dirname "$file")")"
  expected="$(dirname "$file")/expected"
  if [[ ! -f "$expected" ]]; then
    nok "crosscut_$name" "fehlende Datei $expected"
    continue
  fi
  expect_error "crosscut_$name" "$file" "$(<"$expected")"
done

printf '\n%d bestanden, %d fehlgeschlagen\n' "$pass" "$fail"
[[ $fail -eq 0 ]]
