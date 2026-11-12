# Änderungen: morph_box.csv → morph_box_clustered.csv

## Leitlinien

- **Eine Entscheidung pro Zeile.** 
- **Nur das Nötigste:** Neue Zeile nur, wenn klar unabhängig von bestehenden Dimensionen. Sonst die vorhandenen Ausprägungen erweitern.
- **Generische, atomare Ausprägungen:** Jede Dimension soll eine bestehende Lösung (Land, BundID, ELSTER, Neo, ZaPuK …) eindeutig verorten können.
- **Produktentscheidungen vor Vorgaben:** Zeilen, die das Ministerium entscheidet oder die bereits gesetzt sind, stehen am Ende.

## Anpassung der Struktur


- **Fünf Cluster**:
  - **B Behörde** und **P Person / Organisation:** die beiden Seiten des Austauschs.
  - **Ü Übermittlung:** Eigenschaften der Nachricht im Transit.
  - **I Infrastruktur und Betrieb:** wer die d-box betreibt und wo die Daten liegen.
  - **R Recht:** rechtliche Vorgaben und Entscheidungen des BMDS.

- Spalte `Wer entscheidet` um klar zu machen, was vom BMDS gesetzt ist (+ `Auswahl`-Wert `gesetzt`). Die übrigen Zeilen sind noch offen, sollten aber vermutlich vom DCDS in der Zielarchitektur entschieden werden.


## Zuordnung der Zeilen

| Alt | Neu | Änderung |
|---|---|---|
| 1a Anbindung von Fachverfahren | B1 Anbindung von Fachverfahren | unverändert |
| 1b Allgemeines Behörden-Postfach | B2 Zugang ohne Fachverfahren | umbenannt |
| 2 Adressierung von Personen oder Organisationen | B3 Adressierung der Person / Organisation | umbenannt |
| 11 Nachrichtenmodus und Format | B4 Inhalt an die Person / Organisation + Ü1 Modus | geteilt |
| 3 Benachrichtigung | P1 Benachrichtigung | Formulierung |
| 14 Authentifizierung | P2 Authentifizierung | verschoben |
| 9 Vertretung und Organisationszugang | P3 Organisationszugang + P4 Vertretung | geteilt |
| 4 Ort des Lesens | P5 Clients | umbenannt, Frage erweitert |
| — | P6 Anbindung eigener Systeme | neu |
| 5 Rückkanal | P7 Antwort und Erstkontakt | umbenannt, Ausprägungen überarbeitet |
| — | P8 Inhalt an die Behörde | neu |
| 12 Schutz: Vertraulichkeit | Ü2 Vertraulichkeit | umbenannt |
| 10 Echtheit und Schriftform | Ü3 Echtheit + R1 Schriftformersatz | geteilt |
| 6 Nachweise an die Behörde | Ü4 Ereignisse und Bestätigungen + R3 Zustellung und Beweiskraft | geteilt |
| 7a Einbettung + 7b Betreibermodell | I1 Betrieb und Governance | zusammengeführt |
| 13 Datenhaltung | I2 Datenhaltung und Versand | überarbeitet |
| 8 Teilnahme | R2 Teilnahmepflicht | umbenannt |

## Wichtige Änderungen

### Behörde

- **B4 Inhalt an die Person / Organisation** (neu, `Bündel`): Die alte Zeile 11 hat den Übermittlungsmodus mit dem Format des Inhalts vermischt. B4 enthält jetzt die Formatseite. „Link statt Inhalt“ aus Datenhaltung hierher verschoben, weil es beschreibt, was beim Empfänger ankommt. Über den Begriff „Anhänge“ werden Dokumente ausdrücklich erfasst. Aufbewahrung und Veraktung von Dokumenten liegen bewusst außerhalb des Produkts.

### Person / Organisation

- **P3 Organisationszugang** und **P4 Vertretung:** zwei unabhängige Entscheidungen, die davor in einer einzelnen `exklusiv`-Zeile vermengt waren. Funktionspostfächer sind etwas anderes als Vollmachten. (ZaPuK trennt das ebenfalls: ADR-1102 und 1112 behandeln Funktionspostfächer, ADR-1113 die externe Bevollmächtigung.)
- **P5 Clients:** „Ort des Lesens“ war zu eng, Antworten haben gefehlt. Die Ausprägung „mehrere Clients“ aus der alten Zeile 7b gehört hierher, unter „Drittclients“.
- **P6 Anbindung eigener Systeme** (neu): Unternehmen binden ERP-, Steuer- oder Kanzleisoftware an ihr Postfach an. Das spiegelt B1 und B2: P5 deckt Oberflächen für Menschen ab, P6 die maschinelle Verarbeitung.
- **P7 Antwort und Erstkontakt:**
  - Die Frage enthält jetzt auch, wie die Person die Behörde findet (Adressierung ohne eigene Zeile).
  - „Freie Nachricht ohne Anlass“ ist in zwei Ausprägungen geteilt: Auswahl der Behörde über ein Verzeichnis (ZaPuK-Adressregister, ADR-1111) oder Einstieg über die Website der Behörde.
  - Die Formular-Ausprägung ist nach P8 gewandert.
  - Die Polen-Ausprägung ist nach R3 gewandert, weil sie den Nachweis betrifft und nicht, wann eine Person schreibt.
- **P8 Inhalt an die Behörde** (neu, `Bündel`): Gegenstück zu B4. Die beiden Richtungen werden getrennt entschieden, wie Dänemark und ELSTER zeigen, deshalb hat eine Zeile nicht gereicht.

### Übermittlung

- **Ü3 Echtheit:**
  - Umfasst jetzt „Nachricht und Anhänge“.
  - `exklusiv` -> `Bündel`, weil Siegel der Behörde und Signatur der Person kombinierbar sind.
  - Die Schriftform-Frage ist nach R1 gewandert als `gesetzt`.
- **Ü4 Ereignisse und Bestätigungen:** verallgemeinert von „welchen Nachweis bekommt die Behörde“ zu „welche Ereignisse werden erfasst, und wer sieht sie“. Das deckt beide Richtungen und den Betreiber ab und biete Platz für das bisher fehlende „Lesebestätigung an die Person“. Der rechtliche Teil der alten Zeile 6 ist nach R3 gewandert. *Hier ist eine weitere Trennung in die beiden Parteien denkbar.*
### Infrastruktur und Betrieb

Governance (I1) und Technik (I2) sind getrennte Zeilen, da sie divergieren können: Neo hat ein staatliches System, aber getrennte Betreiber je Nutzergruppe, und Schweden hat eine zentrale Infrastruktur mit privaten Operatoren.

- **I1 Betrieb und Governance:** führt 7a und 7b zusammen.
  - Die drei D-Stack-Ausprägungen (autark, DCDS-Plattform, D-App) bleiben zur Transparenz erhalten, als Varianten von „ein staatlicher Betreiber“.
  - Der Client-Aspekt von 7b ist nach P5 gewandert.
- **I2 Datenhaltung und Versand:** das technische Gegenstück zu I1.
  - Fragt, wo Backends und Daten liegen, und damit auch, von wo Nachrichten versendet werden: zentral, getrennt nach Nutzergruppe (Neo), föderiert (ZaPuK), je Behörde (UK) oder bei den privaten Anbietern.
  - „zweckgebundene Verarbeitung und Protokollierung“ ist entfallen, weil es eine Rechtsvorgabe ist und kein Ort.

### Recht

- **R1 Schriftformersatz** (`gesetzt`): Den Schriftformersatz in beide Richtungen gibt das Projekt vor, darum keine Wahl. Die Ausprägungen in Ü3 sind die Wege, ihn zu erfüllen.

- **R3 Zustellung und Beweiskraft:** aus der alten Zeile 6 herausgelöst. Die Polen-Ausprägung lautet jetzt „in beide Richtungen“.

### Übergreifend

- „Person / Organisation“ wird überall gleich geschrieben, mit Leerzeichen um den Schrägstrich. „Bürger:in“ wurde ersetzt.
- Die Sätze „Hier geht es um …, nicht um … (Zeile X)“ wurden aus den Fragen entfernt, weil die geschärften Namen die Abgrenzung klar machen.
- Beispiele, die die Quellen nicht bestätigen, sind mit `[unverifiziert]` markiert.

