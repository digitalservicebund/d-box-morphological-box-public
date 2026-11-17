# Metadatenschema

- Quelle: <https://docs.fitko.de/fit-connect/docs/development/abk_metadatenschema>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Metadatenschema

Abkündigungen

     * Ab 05.08.2026 nicht mehr gültige Versionen: 1.3.0, 1.4.0, 1.5.0., 1.6.0. Nur noch 2.x gültig.
     * Ab 01.04.2026 nicht mehr gültige Version: 1.0.0, 1.1.0, 1.1.1, 1.2.0
     * Ab 01.04.2026 gültige Versionen: ab 1.3.0

Übergangsfrist

   Der Umstieg auf das Metadatenschema 2.x ist bis 27. Oktober 2026 erforderlich. Nach Ablauf dieser Frist werden ältere Versionen (1.x) nicht länger
   unterstützt.
   Hinweis

   Weitere Informationen zum Thema finden Sie hier: Metadatenschema

Änderungen durch Metadatenschemawechsel

    Von → Nach                             Grund/Highlight
   1.0.0 → 1.1.0
     * Refactoring zu $defs, Signatur/Hash via $ref

   1.1.0 → 1.1.1
     * RegEx-Validierung für elster.accountId entfernt

   1.1.1 → 1.2.0
     * Neues (optionales) Feld: fitConnect

   1.2.0 → 1.3.0
     * Neues (optionales) Feld: attachments[].fragments

   1.3.0 → 1.4.0
     * Neue Felder: idBundDeMailbox, idBundDeApplicationId

   1.4.0 → 1.5.0
     * Erweiterung von purpose.enum

   1.5.0 → 2.0.0
     * Modularisierung durch Einführung von dataSets, wodurch authenticationInformation abgelöst wird
     * contentStructure.data nun verpflichtend
     * $schema nun verpflichtend
     * publicServiceType wurde entfernt
     * replyChannel.fink.finkPostfachRef erfordert nun mindestens ein Zeichen
     * Neues (optionales) Feld: additionalReferenceInfo.zustimmungZumDigitalenBescheid
     * Neues (optionales) Feld: author

   Detaillierte Informationen zu neuen Funktionen, Anpassungen und technischen Hinweisen finden Sie unter FIT-Connect 2.0.
```
