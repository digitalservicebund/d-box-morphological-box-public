# Zentrales Bürgerpostfach (ZBP)

- Quelle: <https://docs.fitko.de/fit-connect/docs/organisation-tasks/buergerpostfach-entscheider>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Zentrales Bürgerpostfach

   Das Zentrale Bürgerpostfach (ZBP) als Postfach der BundID ermöglicht es Bund, Ländern und Kommunen sowie den mittelbaren Verwaltungen (z. B.
   Hochschulen und Kammern), Bescheide oder Informationen an Bürger:innen elektronisch zuzustellen. Zudem können Bürger:innen mithilfe von Statusmeldungen
   über den Bearbeitungsstand ihres Antrags informiert werden.

   Den Verwaltungssystemen oder Onlinediensten der Verwaltung bietet das ZBP dafür eine REST-Schnittstelle an, die allerdings ausschließlich innerhalb der
   Netze des Bundes (NdB) oder innerhalb des Verbindungsnetzes angesprochen werden kann.

Anbindung innerhalb der Netze des Bundes

   ZBP-Adapter

   Wird die Anwendung, die an das ZBP angebunden werden soll, innerhalb der Netze des Bundes oder im Verbindungsnetz betrieben oder hat sie eine
   Verbindung dorthin, kann die Anbindung durch die Implementierung der Schnittstelle des ZBP erfolgen. Dabei kann man auf eine eigene Implementierung
   zurückgreifen.

   Einfacher ist es jedoch, die dafür bereitgestellte Funktion der FIT-Connect-SDKs zu verwenden.

Anbindung außerhalb der Netze des Bundes

   Ohne Verbindung zu den Netzen des Bundes ist der direkte Zugriff auf die Schnittstelle des ZBP verwehrt.

   Problematik ohne ZBP-Adapter

   Mithilfe der Brückenkopf-Anwendung ZBP-Adapter gibt es jedoch eine Möglichkeit, Nachrichten und Statusmeldungen über FIT-Connect an das ZBP zu senden:

   Lösung mit ZBP-Adapter

   Normalerweise nutzen Sender wie Onlinedienste und Antragsplattformen der Verwaltung bzw. Softwareanwendungen in Unternehmen FIT-Connect, um Anträge,
   Berichte oder ähnliches an einen Empfänger - in der Regel ist dies das System einer zuständigen Fachbehörde (Verwaltungssystem) - zu übermitteln.

   Für die Anbindung des ZBP steht der Brückenkopf als Empfänger bereit. Er nimmt über FIT-Connect die für das ZBP bestimmten Nachrichten und
   Statusmeldungen entgegen und leitet sie an das ZBP weiter.

ZBP-Adapter im Detail

   Der ZBP‑Adapter bietet eine alternative Möglichkeit, ein Fachverfahren an das Zentrale Bürgerpostfach (ZBP) der BundID anzubinden. Die technische
   Umsetzung des Adapters erfolgt über die Komponente FIT‑Connect. FIT‑Connect ist eine von der Föderalen IT‑Kooperation (FITKO) bereitgestellte
   IT‑Komponente. Sie ermöglicht eine standardisierte, maschinenlesbare und verschlüsselte Kommunikation zwischen Onlinediensten und Fachverfahren, indem
   sie Online‑Verwaltungsdienste mit den Systemen zur Antragsbearbeitung in Bund, Ländern und Kommunen verbindet.

   Im Kontext der BundID fungiert FIT‑Connect als Vermittlungskomponente zwischen Fachverfahren und dem Zentralen Bürgerpostfach und übernimmt damit die
   Funktion eines ZBP‑Adapters. Voraussetzung für die Nutzung des ZBP‑Adapters ist also eine bestehende FIT‑Connect‑Anbindung.

   Bei der Beantragung der Komponente "Postfach" im Self Service Portal der BundID begründen Sie, warum kein Anschluss an die Netze des Bundes vorliegt
   und auch nicht hergestellt werden kann. In diesem Fall müssen Sie die von FIT‑Connect vergebene Client‑ID hinterlegen. Diese wird im Anschluss
   automatisch geprüft.

   Die Client‑ID ist die eindeutige technische Kennung einer Anwendung innerhalb von FIT‑Connect und dient der Identifikation des sendenden Systems. Die
   Client‑ID kann entweder direkt im Rahmen der Antragstellung im Self Service Portal der BundID angegeben oder nachträglich über den entsprechenden
   Menüpunkt gepflegt werden.
   Quelle: https://ssp.id.bund.de/ip/de/zbp-adapter?id=kb_article_view&sysparm_article=KB0010306 (nicht öffentlich, nur mit BundID-SSP-Login einsehbar)
```
