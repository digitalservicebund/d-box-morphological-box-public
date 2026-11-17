# Weitere Angaben

- Quelle: <https://docs.fitko.de/fit-connect/docs/metadata/additionalReferenceInfo>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Weitere Angaben

additionalReferenceInfo

   Diese Seite beschreibt den Bereich additionalReferenceInfo der Metadaten.

   Dieser Bereich dient dazu, dem Empfänger eines Antrags (dem Verwaltungssystem) weitere Informationen zu einem Antrag mitzuteilen.

   In diesem Bereich können zusätzliche Informationen zu einer Einreichung hinterlegt werden, wie zum Beispiel eine Referenz zu dieser Einreichung, die
   das sendende System (der Onlinedienst) erzeugt, sodass im Falle eines Problems diese Einreichung leichter im empfangenden System (im Verwaltungssystem)
   zu identifizieren ist. Dadurch sind Rückfragen außerhalb von FIT-Connect leichter.

   Der Bereich additionalReferenceInfo kann fehlen, d.h. er ist optional (nicht erforderlich).

   Ab Metadaten-Version 2.0.0 kann der Bereich additionalReferenceInfo nur noch die vier unten genannten Felder enthalten, wobei jedes Feld optional ist.
   In früheren Versionen konnte der Bereich um beliebige eigene Felder ergänzt werden.

   Angaben zum Zeitpunkt der Antragstellung sind im Feld applicationDate unterzubringen. Bei senderReference kann eine Referenz eingetragen werden, die
   das sendende System erzeugt hat. Bei idBundDeApplicationId kann als Wert die AntragsID aus dem BundID-Login eingetragen werden, falls als Rückkanal das
   Postfach bei BundID verwendet wird. Die Seite Rückkanäle beschreibt die möglichen Rückkanäle. zustimmungZumDigitalenBescheid kann die Zustimmung des
   Antragsstellers zur elektronischen Bescheidzustellung beinhalten. Wenn die Zustimmung des Antragstellers zur elektronischen Zustellung aktiv eingeholt
   wurde, muss der entsprechende Wert hier hinterlegt werden. Eine ausdrückliche Ablehnung ist durch den Boolean-Wert false zu übermitteln. Das Feld darf
   nicht weggelassen werden, um eine Ablehnung auszudrücken – in diesem Fall ist false erforderlich.

   {

   "$schema": "https://schema.fitko.de/fit-connect/metadata/1.2.0/metadata.schema.json",
   // ...
   "additionalReferenceInfo": {
   "applicationDate":"2025-01-01",
   "senderReference": "ABC-12345",
   "idBundDeApplicationId": "1234...",
   "zustimmungZumDigitalenBescheid": true
   }
   }
```
