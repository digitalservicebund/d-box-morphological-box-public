# netzpolitik.org: Verwaltungsdigitalisierung: Arbeitsgruppe empfiehlt Matrix-Protokoll für Behördenkommunikation

Quelle: https://netzpolitik.org/2025/verwaltungsdigitalisierung-arbeitsgruppe-empfiehlt-matrix-protokoll-fuer-behoerdenkommunikation/
Datum Dokument: 2025-07-01
Abgerufen: 2026-11-19

Sekundärquelle (Presse). Nur Artikeltext, ohne Navigation, Werbung, Spendenaufrufe und Kommentare; automatisch aus HTML extrahiert.

---

# Verwaltungsdigitalisierung : Arbeitsgruppe empfiehlt Matrix-Protokoll für Behördenkommunikation

Online-Kommunikation mit Behörden ist für Bürger*innen und Unternehmen kompliziert, für Verwaltungen oft teuer und aufwendig. Das soll sich ändern: Eine Arbeitsgruppe von IT-Architekten aus Bund und Ländern schlägt vor, wie alle von einer gemeinsamen Infrastruktur profitieren, und empfehlen das offene Kommunikationsprotokoll Matrix.

Zentrales Bürgerpostfach , BundID , OZG-PLUS-Postfach , MeinJustizpostfach und Elster – um mit Behörden zu kommunizieren, richten sich Bürger*innen, Unternehmen und Organisationen verschiedene Postfächer ein. Denn verschiedene Verwaltungen unterhalten jeweils eigene Kommunikationslösungen.

Die stehen derzeit für sich und sind kaum interoperabel. Denn ob Gerichte, Finanzämter, Gesundheitsämter oder Ausländerbehörden: Alle haben eine eigene IT und IT-Architektur, historisch gewachsen und mit unterschiedlichen technischen und rechtlichen Voraussetzungen.

Das soll sich ändern. Bund und Länder wollen eine „moderne und vertrauenswürdige Kommunikationsinfrastruktur im öffentlichen Sektor, die die bislang isolierten Lösungen schrittweise bündelt und die Interoperabilität stärkt“, so André Göbel , Präsident der Föderalen IT-Kooperation (FITKO).

Die Basiskomponente für die Kommunikation ist Teil der geplanten Deutschland-Architektur (PDF) , mit der das Bund-Länder-Gremium IT-Planungsrat neuen Schwung in die Digitalisierung der öffentlichen Verwaltung bringen will. Neben der Basiskomponente Postfächer stehen unter anderem auch Komponenten für Bezahlvorgänge und Authentifizierung an.

## Vorteile einer gemeinsamen Infrastruktur

Der Flickenteppich aus verschiedenen Postfach-Lösungen sei sowohl für Bürger*innen und Unternehmen als auch für die Verwaltung selbst unübersichtlich, sagt Dominik Braun gegenüber netzpolitik.org. Er ist Referent für IT-Architekturmanagement bei der FITKO und arbeitet mit Vertreter*innen der Länder Hamburg und Sachsen-Anhalt an der Zielarchitektur für Postfach- und Kommunikationslösungen, kurz ZaPuK.

Das Ziel sei, dass sich die einen besser orientieren können, wo sie welche Post vom Staat bekommen; und die anderen müssten ihre Postfächer nicht getrennt voneinander anbinden und unterschiedliche Schnittstellen, Protokolle und Verschlüsselungsverfahren nutzen.

Das könne finanzielle und personelle Ressourcen einsparen und langfristig auch Vorteile im Bereich IT-Sicherheit bieten. Das sagt Andreas Altmann, Projektmanager beim Ministerium für Infrastruktur und Digitales des Landes Sachsen-Anhalt, und Teil von ZaPuK. Mit einem neuen Beschluss des IT-Planungsrats hat das Team nun das Go für die nächste Phase.

## Matrix-Protokoll könnte sich durchsetzen

Die klare Empfehlung der Arbeitsgruppe ist das Matrix-Protokoll . Matrix ist ein Protokoll für die Echtzeitkommunikation, dahinter ein offener Standard für dezentralisierte und interoperable Kommunikation. Damit können Nutzer*innen chatten oder telefonieren, ohne von einem spezifischen Diensteanbieter abhängig zu sein.

Matrix soll den Standard Online Services Computer Interface , kurz OSCI, ablösen, so Altmann gegenüber netzpolitik.org. OSCI gibt es seit dem Jahr 2000. Die „ Protokollstandards für den sicheren elektronischen Nachrichtenaustausch über das Internet und andere Netze“ wurden speziell für die öffentliche Verwaltung in Deutschland entwickelt.

Kommunen können entsprechende Standards über das ITZBund beziehen, beispielsweise XMeld für das Ummelden der Wohnungsadresse oder XAusländer für Ausländerbehörden. Laut Bundesmeldedatendigitalisierungs- oder Bundesmeldedatenabruf-Verordnung und Gesetz über das Ausländerzentralregister sind sie sogar dazu verpflichtet, OSCI zu nutzen.

## IT-Sicherheit im Vordergrund

Gegenüber Alternativen wie dem international genutzten Matrix-Protokoll wirkt OSCI etwas verstaubt. Derzeit setzt beispielsweise der elektronische Rechtsverkehr für den Transport und die Zwischenspeicherung von Nachrichten auf OSCI auf. Wesentliche Nachteile des Standards: Er sei nicht auf Echtzeitkommunikation ausgelegt und OSCI basiere auf Technologien, die auf dem Markt und in der öffentlichen Verwaltung zunehmend weniger verbreitet sind, so die IT-Architekten von ZaPuK auf OpenCode . Vor allem aber unterstütze die eingesetzte XML-Encryption „keine modernen kryptografischen Anforderungen“.

Zur Ende-zu-Ende-Verschlüsselung empfiehlt die Gruppe die Sicherheitsschicht Messaging Layer Security , kurz MLS. Das sei besonders wichtig, so Dominik Braun gegenüber netzpolitik.org. „Als Bürger will ich nicht, dass jede Behörde nachvollziehen kann, was ich mit einer anderen Behörde kommuniziere.“ Dabei spielten verfassungsrechtliche und Privatsphäreaspekte eine Rolle. „Einzelne Behörden sollen über mich nicht Informationen bei sich bündeln können.“

In Sachen IT-Sicherheit sei laut Braun zudem eine Zero-Trust-Betriebsumgebung geplant. Das entspricht auch der Föderalen Digitalstrategie des IT-Planungsrats (PDF) . Danach soll jede technische Instanz „nur die minimalen, absolut notwendigen Rechte bekommen“, um bei einem Angriff die Risiken eines Datenabflusses zu reduzieren.

## Eine Lösung für alles wäre schön

Ideal wäre: Bürger*innen, Unternehmen, Organisationen und Behörden nutzen eine Postfach- und Kommunikations-Lösung für alles, so Braun. Daher ist die Empfehlung „die Frontends der bestehenden Lösungen wie MeinJustizpostfach , BundID und OZG-PLUS-Konto in einen Client zusammenzuziehen, der als ein Produkt weiterentwickelt wird“.

Ob es dazu kommt, hänge aber von weiteren Entscheidungen ab, zum Beispiel davon, ob es ein zentrales Verwaltungsportal geben soll. Technisch seien verschiedene Anwendungen denkbar.

Ein paar Designziele stehen jedoch fest: Ende-zu-Ende-Verschlüsselung, Echtzeitverhalten, Mobilfähigkeit und Standards aus offenen und wachsenden Ökosystemen wie Matrix. Außerdem ein Anspruch und rechtliche Vorgabe: Die Kommunikation muss bidirektional verlaufen können – „alles von der Zustellung von Bescheiden bis zu Rückfragen an Behörden über Chat“.

Wichtig sei, dass es im Backend ein koordiniertes einheitliches Produktmanagement gibt, so Braun.

## Eine lange Liste von Anforderungen

Das ZaPuK-Team nahm im September seine Arbeit auf. Seither „haben wir alle Postfächer beleuchtet: elektronisches Gerichts- und Verwaltungspostfach, Telematik-Infrastruktur aus dem öffentlichen Gesundheitswesen, zentrales Bürgerkonto, OZG-Plus, De-Mail. Dabei haben wir rund 900 Anforderungen ermittelt und auf 160 konsolidiert“, so Andreas Altmann gegenüber netzpolitik.org. Eine entsprechende Liste ist auf OpenCode öffentlich einsehbar wie auch eine weitere Dokumentation des Vorhabens.

Dort können Interessierte technische Überlegungen des Teams nachvollziehen, bewerten, kommentieren und diskutieren. „Wir sind da für jede Meinung, für jeden konstruktiven Beitrag echt dankbar“, so Altmann.

Teil der Auswertung waren zudem Best-Practice-Beispiele, Dokumente und Interviews mit den Betreibern der bestehenden Postfach-Lösungen. Was sind Gemeinsamkeiten, wo unterscheiden sich die Ansprüche? Organisationskonten müssen andere Anforderungen erfüllen als Privatkonten. „Zeitstempel sind zum Beispiel für den elektronischen Rechtsverkehr wichtig, aber nicht per se in anderen Verwaltungskontexten“, erklärt Braun.

Auch gut 60 Gesetze und Verordnungen bezog das Team für die Zielarchitektur ein, so Altmann, vor allem die Datenschutzgrundverordnung, die Single-Digital-Gateway-Verordnung, eIDAS unter anderem. Die Idee sei, dass die Anforderungen langfristig als MUSS zu lesen sind, erklärt Braun. Für die nächste Phase sei ein juristisches Gutachten zur Frage angedacht, was möglich ist und was verändert werden müsste.

## Viele Akteure, viele Gespräche

Dafür hat das Team Zeit bis zur 50. Sitzung des IT-Planungsrats im Sommer 2026. Und auch weitere Fragen soll es bis dahin klären: Wie wird das Vorhaben finanziert? Welche Standardisierungsbedarfe gibt es? Wie wird der Weg für bestehende Lösungen in die gemeinsame Infrastruktur geebnet?

Es gebe riesige IT-Infrastrukturen, so Braun. Da könne man nicht einfach unbedarft an den einzelnen Schrauben drehen. „Unser Ansatz ist maximal konstruktiv. In vielen Domänen funktioniert schon vieles sehr gut.“

Jetzt gelte es, mit den Verantwortlichen der jeweiligen Lösungen zu klären, auszuhandeln, teilweise neu zu erarbeiten, was realistisch möglich sei. Neben vielen anderen sind Gesprächspartner die Betreiber von KONSENS-Verbund (koordinierte neue Softwareentwicklung der Steuerverwaltung), Unternehmenskonto, BundID-Konto, Telematik sowie elektronischem Gerichts- und Verwaltungspostfach .

Braun ist zuversichtlich: „Wir haben mitgedacht, dass die verschiedenen Verwaltungsbereiche ihre eigene Backend-Infrastruktur betreiben können, wenn sie das wollen. Sofern Verwaltungen die Anschlussbedingungen an diese Kommunikations-Infrastruktur erfüllen, kann Behörde X im Land Y sich theoretisch dazu entscheiden, die selbst zu implementieren und sich damit an die Architektur anzuschließen.“

Die Verantwortung für den Betrieb hätten Verwaltungen demnach im eigenen Haus. Für diese dezentral organisierte Infrastruktur seien Matrix, aber auch Mastodon Vorbilder gewesen. Die Hypothese sei: „Wenn jeder für das eigene Backend verantwortlich ist, ist es auch einfacher, diversen rechtlichen Auflagen gerecht zu werden.“

Bis Jahresende brauchen wir noch 444.170 Euro. Damit wir weiterhin genau hinschauen können, wenn Freiheit unter Druck gerät.

Wir machen die Arbeit, die sonst keiner macht. Wir lesen Gesetzentwürfe im Original, befreien Dokumente und warnen als Sachverständige vor neuer Überwachung.
