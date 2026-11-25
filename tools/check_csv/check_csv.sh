#!/usr/bin/env bash
# Prüft morph_box.csv, existing_solutions.csv, abkuerzungen.csv,
# zentrale_dienste.csv, vertraulichkeit_stufen.csv,
# vertraulichkeit_kurzfassung.csv und vertraulichkeit_querschnittsthemen.csv
# (bzw. übergebene CSV-Dateien).
#
# Geprüft wird:
#   - Kodierung: gültiges UTF-8, kein BOM
#   - Zeilenenden: einheitlich LF oder CRLF, keine Leerzeilen
#   - CSV-Syntax: Komma als Trennzeichen, korrekt balancierte Anführungszeichen,
#     Zeilenumbrüche nur in quotierten Zellen
#   - Header: je nach Datei exakte Spaltennamen oder eindeutige Lösungsnamen
#   - abkuerzungen.csv: Header "Abkürzung","Bedeutung", Abkürzungen eindeutig
#   - Spaltenanzahl: jede Zeile hat genau so viele Spalten wie der Header
#   - zentrale_dienste.csv: exakter Header, "Dienst" nicht leer und eindeutig,
#     "Funktionen" nicht leer
#   - vertraulichkeit_stufen.csv: exakter Header, "Stufe" nicht leer,
#     "Verschlüsselung" nicht leer und eindeutig
#   - vertraulichkeit_kurzfassung.csv: transponiert, eine Spalte je Stufe;
#     Header "Stufe" plus nicht leere, eindeutige Stufennamen, erste Spalte
#     mit exakten Zeilenbezeichnungen, Zeile "Verschlüsselung" nicht leer
#     und eindeutig
#   - vertraulichkeit_querschnittsthemen.csv: exakter Header, "Thema" nicht
#     leer und eindeutig
#   - Spalte "Dimension": nicht leer
#   - morph_box.csv: gültige "Auswahl" und keine Lücken in Ausprägungen
#   - Zellinhalte: kein Markdown (**fett**, *kursiv*, __, `code`, [Link](url),
#     Aufzählungen, Überschriften)
#
# Aufruf:  tools/check_csv/check_csv.sh [datei.csv ...]
# Ohne Argument wird morph_box.csv im Repository-Root geprüft.
# Exit-Code 0 = alles in Ordnung, 1 = Formatfehler gefunden.

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ $# -eq 0 ]]; then
  set -- "$script_dir/../../morph_box.csv"
fi

expected_header='Cluster|Dimension|Auswahl|Wer entscheidet|Was wird hier entschieden|Ausprägung 1|Ausprägung 2|Ausprägung 3|Ausprägung 4|Ausprägung 5|Ausprägung 6|Ausprägung 7|Ausprägung 8'
glossary_header='Abkürzung|Bedeutung'
services_header='Dienst|Funktionen|Kasten-Dimension|Story und Schritt|Betreiber|Einschränkungen und offene Fragen|Quelle'
levels_header='Stufe|Verschlüsselung|Endgerät Absender:in / Empfänger:in|Organisation (Server, Fachverfahren)|Betreiber des Postfach-Backends|Dritte (Netz, Hoster, externe Dienstleister)|Metadaten beim Betreiber|ZaPuK-Status|Beispiele'
crosscut_header='Thema|Zugriff auf Inhalt: Endgerät Absender:in / Empfänger:in|Zugriff auf Inhalt: Organisation (Server, Fachverfahren)|Zugriff auf Inhalt: Betreiber des Postfach-Backends|Zugriff auf Inhalt: Dritte (Netz, Hoster, externe Dienstleister)|Daten, auf die die Organisation Zugriff hat|Daten, auf die der Betreiber Zugriff hat|ZaPuK-Status|Beispiele'
levels_short_header='Stufe|Verschlüsselung|Zugriff auf Inhalt: Endgerät Absender:in / Empfänger:in|Zugriff auf Inhalt: Organisation (Server, Fachverfahren)|Zugriff auf Inhalt: Betreiber des Postfach-Backends|Zugriff auf Inhalt: Dritte (Netz, Hoster, externe Dienstleister)|Zugriff auf Inhalt: Details|Daten, auf die die Organisation Zugriff hat|Daten, auf die der Betreiber Zugriff hat|ZaPuK-Status|Beispiele'

status=0

check_file() {
  local file="$1" schema=morph
  local errors=0

  local header="$expected_header"

  if [[ "$(basename "$file")" == "existing_solutions.csv" ]]; then
    schema=solutions
  elif [[ "$(basename "$file")" == "abkuerzungen.csv" ]]; then
    schema=glossary
    header="$glossary_header"
  elif [[ "$(basename "$file")" == "zentrale_dienste.csv" ]]; then
    schema=services
    header="$services_header"
  elif [[ "$(basename "$file")" == "vertraulichkeit_stufen.csv" ]]; then
    schema=levels
    header="$levels_header"
  elif [[ "$(basename "$file")" == "vertraulichkeit_kurzfassung.csv" ]]; then
    schema=levels_t
    header="$levels_short_header"
  elif [[ "$(basename "$file")" == "vertraulichkeit_querschnittsthemen.csv" ]]; then
    schema=crosscut
    header="$crosscut_header"
  fi

  if [[ ! -f "$file" ]]; then
    echo "$file: Datei nicht gefunden" >&2
    return 1
  fi

  # Kodierung
  if ! iconv -f UTF-8 -t UTF-8 "$file" 2>/dev/null | cat >/dev/null; then
    echo "$file: kein gültiges UTF-8" >&2
    errors=$((errors + 1))
  fi
  if [[ "$(head -c 3 "$file" | od -An -tx1 | tr -d ' \n')" == "efbbbf" ]]; then
    echo "$file: enthält ein UTF-8-BOM (bitte ohne BOM speichern)" >&2
    errors=$((errors + 1))
  fi

  # Zeilenenden: entweder alle CRLF oder alle LF
  local total crlf
  total=$(grep -c '' "$file" || true)
  crlf=$(grep -c $'\r$' "$file" || true)
  if [[ "$crlf" -ne 0 && "$crlf" -ne "$total" ]]; then
    echo "$file: gemischte Zeilenenden (LF und CRLF)" >&2
    errors=$((errors + 1))
  fi

  # CSV-Struktur und Inhalte
  if ! awk -v file="$file" -v expected="$header" -v schema="$schema" '
    BEGIN {
      ncols = split(expected, hdr, "|")
      if (schema == "levels_t") nlab = split(expected, lab, "|")
      zeile = 2; auswahl = 3; first_option = 6
      if (schema == "solutions" || schema == "glossary" || schema == "services" || schema == "levels" || schema == "levels_t" || schema == "crosscut") zeile = 1
      errs = 0; rec = 0; nf = 0; buf = ""
      in_q = 0; was_q = 0; after_q = 0; bad = 0
    }
    function err(line, msg) {
      printf "%s:%d: %s\n", file, line, msg > "/dev/stderr"
      errs++
    }
    function flush_field() {
      nf++; f[nf] = buf
      buf = ""; was_q = 0; after_q = 0
    }
    function is_markdown(s) {
      if (s ~ /\*\*/ || s ~ /__/ || s ~ /`/) return 1
      if (s ~ /\[[^]]*\]\([^)]*\)/) return 1
      if (s ~ /(^|[ \n(])\*[^ *\n][^*\n]*\*/) return 1
      if (s ~ /(^|\n)[ \t]*[-*+][ \t]/) return 1
      if (s ~ /(^|\n)#+[ \t]/) return 1
      return 0
    }
    function check_record(   i, gap) {
      rec++
      if (bad) return
      if (schema == "solutions" && rec == 1) {
        ncols = nf
        if (f[1] != "Dimension") err(start, "Header Spalte 1: erwartet \"Dimension\", gefunden \"" f[1] "\"")
        if (ncols < 2) err(start, "Header benötigt mindestens eine Lösungsspalte")
        for (i = 1; i <= ncols; i++) {
          hdr[i] = f[i]
          if (i > 1 && f[i] == "") err(start, "Header Spalte " i " ist leer")
          if (i > 1 && f[i] != "" && seen[f[i]]++) err(start, "doppelter Header: \"" f[i] "\"")
          if (is_markdown(f[i])) err(start, "Markdown-Formatierung in Header Spalte " i)
        }
        return
      }
      if (schema == "levels_t" && rec == 1) {
        ncols = nf
        if (f[1] != lab[1]) err(start, "Header Spalte 1: erwartet \"" lab[1] "\", gefunden \"" f[1] "\"")
        if (ncols < 2) err(start, "Header benötigt mindestens eine Stufe")
        for (i = 1; i <= ncols; i++) {
          hdr[i] = f[i]
          if (i > 1 && f[i] == "") err(start, "Header Spalte " i " (Stufe) ist leer")
          if (i > 1 && f[i] != "" && seen[f[i]]++) err(start, "doppelte Stufe: \"" f[i] "\"")
          if (is_markdown(f[i])) err(start, "Markdown-Formatierung in Header Spalte " i)
        }
        return
      }
      if (nf != ncols) {
        err(start, "erwartet " ncols " Spalten, gefunden " nf " (Trennzeichen muss \",\" sein)")
        return
      }
      if (rec == 1) {
        for (i = 1; i <= ncols; i++)
          if (f[i] != hdr[i])
            err(start, "Header Spalte " i ": erwartet \"" hdr[i] "\", gefunden \"" f[i] "\"")
        return
      }
      if (f[zeile] == "") err(start, "Spalte \"" hdr[zeile] "\" ist leer")
      if (schema == "glossary") {
        if (f[2] == "") err(start, "Spalte \"Bedeutung\" ist leer (unbekannt: \"?\")")
        if (f[1] != "" && known[f[1]]++) err(start, "doppelte Abkürzung: \"" f[1] "\"")
      }
      if (schema == "services") {
        if (f[2] == "") err(start, "Spalte \"Funktionen\" ist leer")
        if (f[1] != "" && known[f[1]]++) err(start, "doppelter Dienst: \"" f[1] "\"")
      }
      if (schema == "levels") {
        if (f[2] == "") err(start, "Spalte \"Verschlüsselung\" ist leer")
        if (f[2] != "" && known[f[2]]++) err(start, "doppelte Verschlüsselung: \"" f[2] "\"")
      }
      if (schema == "levels_t") {
        if (rec <= nlab && f[1] != lab[rec]) err(start, "Zeile " rec ": erwartet \"" lab[rec] "\", gefunden \"" f[1] "\"")
        if (f[1] == "Verschlüsselung")
          for (i = 2; i <= ncols; i++) {
            if (f[i] == "") err(start, "Verschlüsselung für \"" hdr[i] "\" ist leer")
            else if (known[f[i]]++) err(start, "doppelte Verschlüsselung: \"" f[i] "\"")
          }
      }
      if (schema == "crosscut") {
        if (f[1] != "" && known[f[1]]++) err(start, "doppeltes Thema: \"" f[1] "\"")
      }
      if (schema == "morph") {
        if (f[auswahl] != "exklusiv" && f[auswahl] != "Bündel" && f[auswahl] != "gesetzt")
          err(start, "Spalte \"Auswahl\" muss \"exklusiv\", \"Bündel\" oder \"gesetzt\" sein, gefunden \"" f[auswahl] "\"")
        gap = 0
        for (i = first_option; i <= ncols; i++) {
          if (f[i] == "") gap = i
          else if (gap) { err(start, "Lücke: \"" hdr[gap] "\" ist leer, aber \"" hdr[i] "\" ist belegt"); break }
        }
      }
      for (i = 1; i <= ncols; i++)
        if (is_markdown(f[i]))
          err(start, "Markdown-Formatierung in Spalte \"" hdr[i] "\"")
    }
    {
      sub(/\r$/, "")
      if (!in_q) {
        start = NR; nf = 0; bad = 0
        if ($0 == "") { err(NR, "Leerzeile"); next }
      }
      n = length($0)
      for (i = 1; i <= n; i++) {
        c = substr($0, i, 1)
        if (in_q) {
          if (c == "\"") {
            if (substr($0, i + 1, 1) == "\"") { buf = buf "\""; i++ }
            else { in_q = 0; after_q = 1 }
          } else buf = buf c
        } else if (c == ",") {
          flush_field()
        } else if (after_q) {
          if (!bad) {
            if (c == ";") err(NR, "Semikolon als Trennzeichen gefunden, erlaubt ist nur \",\"")
            else err(NR, "Zeichen nach schließendem Anführungszeichen (Quote nicht verdoppelt?)")
          }
          bad = 1; buf = buf c
        } else if (c == "\"") {
          if (buf == "" && !was_q) { in_q = 1; was_q = 1 }
          else {
            if (!bad) err(NR, "Anführungszeichen in nicht quotierter Zelle")
            bad = 1; buf = buf c
          }
        } else buf = buf c
      }
      if (in_q) { buf = buf "\n"; next }
      flush_field()
      check_record()
    }
    END {
      if (in_q) err(start, "Anführungszeichen nicht geschlossen (Datensatz beginnt hier)")
      if (NR == 0) err(1, "Datei ist leer")
      if (schema == "levels_t" && NR > 0 && rec != nlab) err(NR, "erwartet " nlab " Zeilen, gefunden " rec)
      exit errs > 0
    }
  ' "$file"; then
    errors=$((errors + 1))
  fi

  if [[ $errors -eq 0 ]]; then
    echo "$file: OK"
    return 0
  fi
  return 1
}

for file in "$@"; do
  check_file "$file" || status=1
done

exit $status
