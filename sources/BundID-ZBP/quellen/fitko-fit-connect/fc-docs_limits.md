# Limits bei Einreichungen

- Quelle: <https://docs.fitko.de/fit-connect/docs/getting-started/limits>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Limits

Wie groß dürfen Einreichungen sein?

   Diese Seite zeigt, welche Limits (maximalen Größen) Sie bei FIT-Connect einhalten müssen, um Einreichungen sicher zu übertragen.

   Die folgenden Werte stellen Richtwerte dar, die eine sichere Übertragung von Einreichungen über FIT-Connect ermöglichen.

   Die Werte gelten für Datensätze und Anlagen vor der Verschlüsselung.

   Manche Grenzwerte können individuell pro Onlinedienst oder Zustellpunkt angepasst werden. In der Spalte "Größe" ist der Standardwert angegeben; die
   Spalte "anpassbar" enthält die einstellbaren Grenzen.
       Limit      Größe  anpassbar                Hinweise
   Metadatensatz (tbd)             Geplant ist eine Begrenzung auf 350 kB
   Fachdatensatz 14,9 MB
   Anlage, eine  500 MB  ≤ 500 MB
   Anlagen, alle 2 GB    ≤ 5 GB

Anzahl

       Limit         pro      Wert              Hinweise
   Anlagen       Übermittlung 100  Anzahl ist individuell einstellbar
   Zustellpunkte Client       5000

Zeit

             Limit               von      Dauer  Hinweise
   Absenden                  Einreichung 1 Tag
   Abholen                   Einreichung 14 Tage
   Verbleib nach Abholung    Einreichung 7 Tage
   Verbleib nach Rückweisung Einreichung 7 Tage
   Absenden                  Antwort     1 Tag
   Abholung                  Antwort     14 Tage
   Verbleib nach Abholung    Antwort     7 Tage
   Verbleib nach Rückweisung Antwort     7 Tage

Durchsatz

RateLimit-Remaining

   Dieser Header gibt die Anzahl der aktuell verfügbaren Anfragen im aktuellen Zeitfenster und der aktuell wirkenden Beschränkung zurück.

RateLimit-Reset

   Dieser Header gibt die Zeit in Sekunden an, wann das aktuelle Zeitfenster abläuft. Danach beginnt ein neues Zeitfenster für die aktuell wirkende
   Beschränkung.

Überschreiten des Limits

   Wird eine Anfrage-Begrenzung überschritten, wird diese mit dem HTTP-Status-Code 429 beantwortet und kein Inhalt vom Dienst zurückgegeben. Die Anfrage
   sollte dann, unter Berücksichtigung des Headers RateLimit-Reset wiederholt werden.

Grenzwerte

    API/Anwendung                        Bereich                       Anzahl Zeiteinheit       Hinweise
   Destination API Kein Rate Limit                                     -      -
   Routing API     Alle Endpunkte                                      5      1 sec
   Submission API  */attachments/* Requests pro Zeitfenster            48/64  6 sec       48 average, 64 burst
   Submission API  Parallele attachment Requests pro fachlichem Client 3      -
   Submission API  Parallele attachment Requests GLOBAL                60     -
```
