# Bewertungskriterien für Lösungswege

Der morphologische Kasten in [morph_box.csv](morph_box.csv) beschreibt mögliche Lösungswege. Die folgenden Fragen dienen dazu, einen gewählten Weg zu bewerten; sie sind keine zusätzlichen Ausprägungen. Ein Kriterium kann mehrere Dimensionen betreffen.

Für eine Bewertung den betrachteten Lösungsweg und Vorgang benennen, die Fragen anhand von Belegen beantworten und offene Punkte als „ungeklärt“ kennzeichnen. Es sind noch keine Gewichtungen, Schwellenwerte oder Bewertungen einzelner Lösungen festgelegt. Eine gesetzliche oder projektseitige Vorgabe ist gesondert auf Erfüllung zu prüfen, nicht durch eine gute Gesamtbewertung auszugleichen.

## Recht und Nachweise

| Kriterium | Bewertungsfrage | Bezug im Kasten |
|---|---|---|
| Unterstützte Übermittlungsarten | Unterstützt der Lösungsweg elektronische Bekanntgabe, förmliche Zustellung oder beides für die betrachteten Vorgänge und Empfängergruppen? Welche Vorgänge bleiben auf Papier oder benötigen andere rechtlich zulässige Wege? | R3 |
| Rechtsgrundlage und Wirksamkeit | Ist das gewählte Verfahren für den jeweiligen Vorgang rechtlich zulässig und wann wird die Übermittlung wirksam? Welche Rechtsgrundlage müsste für eine elektronische förmliche Zustellung an Bürger:innen geklärt oder geschaffen werden? | R3 |
| Beweiskraft gegenüber der Behörde | Sind das relevante Ereignis, sein Zeitpunkt und gegebenenfalls die Zustellung so nachweisbar, wie es der Vorgang verlangt? Insbesondere unterscheiden, ob Bereitstellung, Abruf oder eine förmliche Zustellung belegt wird. | Ü4, R3 |
| Eingangsbeleg für Personen / Organisationen | Erhalten Absender:innen für Antworten und eigene Eingaben einen Beleg über den Eingang bei der Behörde, der für den Nachweis der Fristwahrung geeignet ist? | P7, Ü4 |
| Schriftformersatz | Erfüllt der Lösungsweg die gesetzte Projektvorgabe des Schriftformersatzes in beide Richtungen für die betrachteten Vorgänge? | Ü3, R1 |
| Übertragbarkeit von Beispielen | Sind die Verfahren und Nachweise aus Vergleichslösungen für den betrachteten deutschen Vorgang rechtlich geeignet? Insbesondere den qualifizierten Nachweis nach eIDAS (Polen) und die Wahl zwischen einfacher und nachweislicher Übermittlung (Österreich) prüfen. | R3 |

Die rechtlichen Aussagen sind keine abschließende Rechtsprüfung. Die benötigte Art der Bekanntgabe oder Zustellung und die Anforderungen an den Nachweis hängen vom jeweils geltenden Recht ab.

## Zugang, Sicherheit und Nutzbarkeit

| Kriterium | Bewertungsfrage | Bezug im Kasten |
|---|---|---|
| Vertrauensniveau | Erreichen die gewählten Authentifizierungsverfahren das für den jeweiligen Vorgang erforderliche Vertrauensniveau? Die Eignung hängt vom Verfahren und den geltenden Vorgaben ab. | P2, R3 |
| Nutzbarkeit im Massenfall | Können die betroffenen Personen / Organisationen das erforderliche Authentifizierungsverfahren praktisch und in ausreichender Zahl nutzen, insbesondere bei häufigen Vorgängen? | P2 |
| Vertraulichkeit | Wer kann Inhalte und gegebenenfalls Metadaten lesen, und genügt der Schutz den Anforderungen des Vorgangs? Auswirkungen der Schlüsselverwahrung und die Umsetzbarkeit der Ende-zu-Ende-Verschlüsselung berücksichtigen. | Ü2, I2 |
| Datenschutz und Protokollierung | Sind Verarbeitung und Datenhaltung zweckgebunden und ist die erforderliche Protokollierung gewährleistet? | B3, Ü4, I2 |
| Erreichbarkeit und Rückfall | Wie werden Personen / Organisationen über Nachrichten erreicht, und welche Rückfallwege bestehen, wenn der digitale Zugang nicht genutzt wird oder eine Befreiung greift? | P1, P2, R2 |

## Einführung und Betrieb

| Kriterium | Bewertungsfrage | Bezug im Kasten |
|---|---|---|
| Anbindung der Behörden | Wie aufwendig und langwierig ist die Anbindung von Fachverfahren und Behörden ohne Fachverfahren? Welche Voraussetzungen bestehen für Bestandssysteme? | B1, B2 |
| Teilnahme und Akzeptanz | Welche Behörden nehmen unter dem gewählten Modell voraussichtlich teil? Ist die Annahme freiwilliger Teilnahme tragfähig, und welche Wirkung haben Pflicht, Anreize oder Stichtage? | R4 |
| Betrieb und Kosten | Welche Folgen haben Betreiber- und Datenhaltungsmodell für Betrieb und Kosten? Wie belastbar sind die verfügbaren Angaben? | I1, I2 |

## Offene Prüfungen und Annahmen

- **Authentifizierung:** Die Einordnung und Eignung des ELSTER-Zertifikats prüfen. Laut Rückmeldung vom 12.11.2026 bestreitet die Steuerberaterkammer, dass das Nutzerkonto für Steuerbescheide ausreicht; diesen Einwand rechtlich prüfen. Die Rückmeldung liegt nicht als Datei im Repository vor.
- **Förmliche Zustellung:** Rechtsgrundlage und Voraussetzungen einer elektronischen Lösung für Bürger:innen klären; nicht aus der technischen Verfügbarkeit eines Nachweises auf rechtliche Zulässigkeit schließen.
- **Pseudonyme Adressierung:** Bei B3 ist offen, was einen Bereich ausmacht; die Ausprägung ist als kritische Annahme ohne Vergleichsland gekennzeichnet.
- **Ende-zu-Ende-Verschlüsselung:** Die entsprechende Ausprägung in Ü2 ist für den betrachteten Anwendungsfall als kritische Annahme gekennzeichnet; die Vergleichbarkeit der genannten Justizlösung prüfen.
- **Vergleichslösungen:** Im Kasten als `[unverifiziert]` oder „zu verifizieren“ markierte Beispiele vor einer Bewertung überprüfen. Ein Vergleichspunkt allein belegt weder Eignung noch Übertragbarkeit.
- **Kosten:** Für Neo liegen laut [sources.md](sources.md) keine konkreten Hosting-, Größenordnungs-, Kosten- oder SLA-Angaben vor. Fehlende Angaben nicht als fehlende Kosten oder Eigenschaften werten.

Quelleneinstiege zu den einzelnen Themen stehen in [sources.md](sources.md), insbesondere zum rechtlichen Konzept für Zustellung und Nachweise sowie zu den ZaPuK-Anforderungen und Architekturentscheidungen. Die Strukturänderungen des Kastens sind in [morph_box_changes.md](morph_box_changes.md) beschrieben.
