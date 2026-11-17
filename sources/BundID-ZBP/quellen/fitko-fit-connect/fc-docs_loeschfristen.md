# Benachrichtigungen und Löschfristen

- Quelle: <https://docs.fitko.de/fit-connect/docs/getting-started/notifications-and-deletion-deadlines>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Benachrichtigungen und Löschfristen

   Diese Seite dokumentiert die Zeitpunkte, an denen Löschaktionen in verschiedenen Szenarien automatisiert durchgeführt werden. Darüber hinaus erhalten
   Sie Auskunft zu den Benachrichtungsmethoden, die bei diesen Szenarien genutzt werden.
   Hinweis

   Grundsätzlich gilt: Die für Einreichungen geltenden Fristen werden analog auf Antworten (Replies) angewendet. Die Fristen für eine Zurückweisung
   (Reject durch ZSD) und Löschung (Delete) von Antworten sind somit identisch mit den Löschfristen von Einreichungen.

Löschung nach erfolgreicher Abholung/Antwort

   Nach der erfolgreichen Abholung einer Einreichung (Submission) durch ein Verwaltungssystem bzw. einer erfolgreichen Antwort (Reply) durch einen
   Onlinedienst wird die Einreichung bzw. Antwort im Zustelldienst nach 7 Tagen gelöscht.

Löschung bei nicht erfolgter Abholung/Antwort

   Bei nicht erfolgter Abholung einer Einreichung durch das Verwaltungssystem bzw. einer fehlenden Antwort durch einen Onlinedienst erfolgt 3 Tage sowie 9
   Tage nach Eingang eine Benachrichtigung an die am Zustellpunkt hinterlegte E-Mail-Adresse.
   Hinweis

   Beachten Sie, dass bei der Löschung von Antworten keine automatisierte Benachrichtigung eines technischen Ansprechpartners möglich ist, da dieser
   Kontakt im Zustellpunkt hinterlegt ist und ein Onlinedienst (Sender-Client) keinen Zustellpunkt enthält.

   14 Tage nach Eingang einer neuen Einreichung wird eine Einreichung als nicht zustellbar markiert. Dies geschieht durch ein Reject Event. Ab diesem
   Zeitpunkt kann diese Einreichung nicht länger abgerufen werden.

   Sie werden über diese Ereignisse im Event Log informiert. Wie Sie das Event Log abrufen können, wird auf der Seite Event Log abrufen näher beschrieben.

   Die endgültige Löschung der Einreichung erfolgt nach weiteren 7 Tagen.
   Tage nach Eingang einer ...        ... Einreichung (Empfänger)         ... Antwort (Sender)
   3                           E-Mail an Betreiber des Verwaltungssystems -/- (siehe Hinweis)
   9                           E-Mail an Betreiber des Verwaltungssystems -/- (siehe Hinweis)
   14                          Reject Event                               Reject Event
   21                          Löschung der Einreichung                   Löschung der Antwort
   Wichtig

   Bitte deaktivieren Sie Ihren Zustellpunkt, wenn Probleme auftreten, die länger als 14 Tage andauern, damit der Onlinedienst informiert wird.

Löschung unvollständiger Einreichungen

   Unvollständige Einreichungen werden nach einem (1) Tag gelöscht.

Löschung von Security Event Tokens (SETs) aus dem Event Log

   730 Tage nach dem letzten Event zu einem bestimmten Case werden alle zum Case gehörenden Security Event Tokens gelöscht. Dies betrifft auch alle
   weiteren Metadaten zum Case. Ein Nachweis über Versand und Empfang ist nach Verstreichen dieser Frist also nicht mehr möglich.
```
