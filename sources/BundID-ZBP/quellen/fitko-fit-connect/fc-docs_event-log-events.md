# Ereignisse

- Quelle: <https://docs.fitko.de/fit-connect/docs/getting-started/event-log/events>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Ereignisse

   Das folgende Statusdiagramm zeigt die Status und Ereignisse einer Einreichung. Die Status sind als orange Ovale dargestellt. Rechtecke stehen für
   Ereignisse. Blau dargestellte Ereignisse werden vom Zustelldienst, grüne vom empfangenden System erstellt und signiert.

Statusdiagramm

Einreichung

   Statusdiagramm Einreichung

Antwort

   Statusdiagramm Antwort

Legende

   Statusdiagramm Legende

   Das Akzeptieren oder Zurückweisen von Einreichungen findet auf einer rein technischen Ebene statt und trifft keine Aussage über die fachliche
   Korrektheit der Einreichungen. Gründe für technische Rückweisungen wären beispielsweise Probleme bei der Entschlüsselung oder Validierungsfehler der
   Datenstrukturen.

Übersicht über zulässige Ereignisse (events)

   In der folgenden Tabelle sind die möglichen Ereignisse, ihre Beschreibungen und die Autoren aufgeführt und beschrieben.
   Event Autor Bedeutung
   https://schema.fitko.de/fit-connect/events/create-submission ↓ Zustelldienst Die Einreichung wurde durch den Onlinedienst angelegt.
   https://schema.fitko.de/fit-connect/events/submit-submission ↓ Zustelldienst Die Einreichung wurde durch den Onlinedienst abgesendet.
   https://schema.fitko.de/fit-connect/events/notify-submission ↓ Zustelldienst Das Verwaltungssystem hat Kenntnis von der Einreichung erlangt (das
   Verwaltungssystem wurde per Callback informiert oder hat die Liste der verfügbaren Submissions abgerufen).
   https://schema.fitko.de/fit-connect/events/forward-submission ↓ Empfangendes System Deprecated: Dieses Event kann mit der Submission-API v2 nicht mehr
   ausgelöst werden. Onlinedienste müssen derzeit aber dennoch dieses Event verstehen, da die Gegenseite weiterhin die v1-APIs nutzen könnte. Bedeutung:
   Ein nachgelagertes System hat die Einreichung zur Weiterleitung übernommen.
   https://schema.fitko.de/fit-connect/events/reject-submission ↓ Empfangendes System/Zustelldienst Die Einreichung wurde durch den Empfänger
   zurückgewiesen oder die Einreichung war mehr als 14 Tage im Status submitted und wurde deshalb vom Zustelldienst zurückgewiesen.
   https://schema.fitko.de/fit-connect/events/accept-submission ↓ Empfangendes System Die Einreichung wurde durch den Empfänger akzeptiert.
   https://schema.fitko.de/fit-connect/events/delete-submission ↓ Zustelldienst Die Einreichung wurde durch den Zustelldienst gelöscht.
   https://schema.fitko.de/fit-connect/events/create-reply ↓ Zustelldienst Die Antwort wurde durch das Verwaltungssystem angelegt.
   https://schema.fitko.de/fit-connect/events/submit-reply ↓ Zustelldienst Die Antwort wurde durch das Verwaltungssystem abgesendet.
   https://schema.fitko.de/fit-connect/events/notify-reply ↓ Zustelldienst Der Empfänger (Onlinedienst) wurde per Webhook über die Antwort informiert.
   https://schema.fitko.de/fit-connect/events/reject-reply ↓ Zustelldienst Die Antwort wurde durch den Empfänger zurückgewiesen oder die Antwort war mehr
   als 14 Tage im Status submitted und wurde deshalb vom Zustelldienst zurückgewiesen.
   https://schema.fitko.de/fit-connect/events/accept-reply ↓ Zustelldienst Die Antwort wurde durch den Empfänger akzeptiert.
   https://schema.fitko.de/fit-connect/events/delete-reply ↓ Zustelldienst Die Antwort wurde durch den Zustelldienst gelöscht.

Submission Events

create-submission

   Mit dem Event https://schema.fitko.de/fit-connect/events/create-submission dokumentiert der Zustelldienst, dass eine Einreichung angelegt wurde.
   Beispiel: create-submission payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "ada1b5b4-1bd2-4fab-b236-c30ef88e8c72",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "submission:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/create-submission": {
   }
   }
   }

submit-submission

   Mit dem Event https://schema.fitko.de/fit-connect/events/submit-submission dokumentiert der Zustelldienst, dass die Einreichung abgesendet wurde.

   Das Event enthält das Objekt authenticationTags, das die Authentication Tags der verschlüsselten Inhalte enthält.
     * metadata: Authentication Tag des Metadatensatzes
     * data: Authentication Tag des Fachdatensatzes
     * attachments: Objekt mit den Authentication Tags der Anlagen mit ihren attachmentIds als Schlüssel

   Beispiel: submit-submission payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "25d2eb77-458d-4c9d-991c-6428c4651646",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "submission:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/submit-submission": {
   "authenticationTags": {
   "metadata": "XFBoMYUZodetZdvTiFvSkQ",
   "data": "UCGiqJxhBI3IFVdPalHHvA",
   "attachments": {
   "0b799252-deb9-42b0-98d3-c50d24bbafe0": "rT99rwrBTbTI7IJM8fU3El",
   "25abf553-0e53-43b9-a14a-1581b32a9ee5": "i7226HEB7IchCxNuh7lCiu",
   "046a9fa5-bed6-494b-aab6-d41056c6db79": "d48LxeolRdtFF4nzQibeYO"
   }
   }
   }
   }
   }

notify-submission

   Mit dem Event https://schema.fitko.de/fit-connect/events/notify-submission dokumentiert der Zustelldienst, dass das empfangende System Kenntnis von der
   Einreichung erlangt hat. Wie dies erfolgt ist, wird mit dem Eintrag notifyType dokumentiert.
    1. "notifyType": "callback" - Der Subscriber wurde per Callback informiert.

   Beispiel: notify-submission (callback) payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "116c3c03-5f31-4d4c-9e65-d36e1e3895f7",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "submission:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/notify-submission": {
   "notifyType": "callback"
   }
   }
   }
    2. "notifyType": "polling" - Der Subscriber hat die Liste der Submission über GET /v2/submissions abgerufen.

   Beispiel: notify-submission (polling) payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "6127ff87-2786-4d0f-90fe-c531672f5de1",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "submission:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/notify-submission": {
   "notifyType": "polling"
   }
   }
   }

forward-submission

   Hinweis

   Dieses Event kann mit der Submission-API v2 nicht mehr ausgelöst werden. Onlinedienste müssen derzeit aber dennoch dieses Event verstehen, da die
   Gegenseite weiterhin die v1-APIs nutzen könnte. Da die v1-APIs bereits abgekündigt sind, sollten Sie dieses Event auf keinen Fall verwenden. Die
   Beschreibung hier dient lediglich historischen Zwecken.

   Mit dem Event https://schema.fitko.de/fit-connect/events/forward-submission dokumentiert ein nachgelagertes System, dass es die Einreichung zur
   Weiterleitung übernommen hat.
   Beispiel: forward-submission payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "c0cda2aa-bf79-4427-86f3-9f973bad2ecd",
   "iss": "40847c29-06aa-40e2-bf28-c29884c694c4",
   "iat": 1622796532,
   "sub": "submission:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/forward-submission": {
   }
   }
   }

reject-submission

   Mit dem Event https://schema.fitko.de/fit-connect/events/reject-submission dokumentiert das empfangende System, dass die Einreichung zurückgewiesen
   wird.

   Alternativ kann auch der Zustelldienst Einreichungen als rejected markieren, wenn diese mehr als 14 Tage im Status submitted verbleiben. Unter
   Benachrichtigungen und Löschfristen finden sich hierzu genaue Angaben.

   Das Event enthält ein Array problems, dass die Fehler der Einreichung dokumentiert. Der Aufbau ist an RFC 7807 angelehnt, lässt jedoch den Status aus,
   weil hier kein passender HTTP Status Code angegeben werden kann.
     * type: Fehlercode in Form einer URI.
     * title: Für Menschen verständliche Fehlermeldung
     * detail: Details zum Fehler, z.B. eine technische Fehlermeldung (optional)
     * instance: Betroffener Teil der Übertragung. Mögliche Werte: submission, metadata, data, attachment: + UUID des Attachments oder other

   Beispiel: reject-submission payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "4ac47caa-bce1-435a-b04f-3322b224b32e",
   "iss": "40847c29-06aa-40e2-bf28-c29884c694c4",
   "iat": 1622796532,
   "sub": "submission:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/reject-submission": {
   "problems": [
   {
   "type": "https://schema.fitko.de/fit-connect/events/problems/authentication-tag-incorrect",
   "title": "Das Authentication Tag des Metadatensatzes ist ungültig",
   "detail": "Das Authentication Tag des Metadatensatzes stimmt nicht mit dem im Submit-Submission-Event angegebenen Wert überein.",
   "instance": "metadata"
   }
   ]
   }
   }
   }

accept-submission

   Mit dem Event https://schema.fitko.de/fit-connect/events/accept-submission dokumentiert das empfangende System, dass die Einreichung akzeptiert wurde.

   Das Event muss eine mit dem submit-submission Event übereinstimmende Liste von Authentication Tags enthalten. Das empfangende System dokumentiert
   damit, dass es die Authentication Tags überprüft hat.
   Beispiel: accept-submission payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "8538165b-9ce3-4097-871d-5b9581a3b4d9",
   "iss": "40847c29-06aa-40e2-bf28-c29884c694c4",
   "iat": 1622796532,
   "sub": "submission:f65feab2-4883-4dff-85fb-169448545d9f",
   "txn": "case:f73d30c6-8894-4444-8687-00ae756fea90",
   "events": {
   "https://schema.fitko.de/fit-connect/events/accept-submission": {
   "authenticationTags": {
   "metadata": "XFBoMYUZodetZdvTiFvSkQ",
   "data": "UCGiqJxhBI3IFVdPalHHvA",
   "attachments": {
   "0b799252-deb9-42b0-98d3-c50d24bbafe0": "rT99rwrBTbTI7IJM8fU3El",
   "25abf553-0e53-43b9-a14a-1581b32a9ee5": "i7226HEB7IchCxNuh7lCiu",
   "046a9fa5-bed6-494b-aab6-d41056c6db79": "d48LxeolRdtFF4nzQibeYO"
   }
   }
   }
   }
   }

   Sofern Probleme in der Einreichung gefunden wurden, die jedoch nicht zu einer Zurückweisung geführt haben, werden diese analog zu der problems Liste in
   reject-submission dokumentiert.
   Beispiel: accept-submission payload im Falle eines Fehlers bei der Einreichung

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "6872b19f-5ee2-47d7-a0e9-ebfe87ab2563",
   "iss": "40847c29-06aa-40e2-bf28-c29884c694c4",
   "iat": 1622796532,
   "sub": "submission:F65FEAB2-4883-4DFF-85FB-169448545D9F",
   "txn": "case:F73D30C6-8894-4444-8687-00AE756FEA90",
   "events": {
   "https://schema.fitko.de/fit-connect/events/accept-submission": {
   "problems": [
   {
   "type": "https://schema.fitko.de/fit-connect/events/problems/schema-missing",
   "title": "Schemareferenz fehlt im Metadatensatz",
   "detail": "Die Referenz auf das Metadatenschema ('$schema') fehlt im Metadatensatz.",
   "instance": "metadata"
   }
   ],
   "authenticationTags": {
   "metadata": "XFBoMYUZodetZdvTiFvSkQ",
   "data": "UCGiqJxhBI3IFVdPalHHvA",
   "attachments": {
   "0b799252-deb9-42b0-98d3-c50d24bbafe0": "rT99rwrBTbTI7IJM8fU3El",
   "25abf553-0e53-43b9-a14a-1581b32a9ee5": "i7226HEB7IchCxNuh7lCiu",
   "046a9fa5-bed6-494b-aab6-d41056c6db79": "d48LxeolRdtFF4nzQibeYO"
   }
   }
   }
   }
   }

delete-submission

   Mit dem Event https://schema.fitko.de/fit-connect/events/delete-submission dokumentiert der Zustelldienst, dass die Einreichung gelöscht wurde. Die
   Fristen, wann Einreichungen mit welchem Status gelöscht werden, finden sich unter Benachrichtigungen und Löschfristen.
   Beispiel: delete-submission payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "29d091aa-86ab-4806-beb5-fb462d18e2a1",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "submission:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/delete-submission": {
   }
   }
   }

Reply Events

  create-reply

   Mit dem Event https://schema.fitko.de/fit-connect/events/create-reply dokumentiert der Zustelldienst, dass eine Antwort angelegt wurde.

   Beispiel: create-reply payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "ada1b5b4-1bd2-4fab-b236-c30ef88e8c72",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "reply:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/create-reply": {
   }
   }
   }

  submit-reply

   Mit dem Event https://schema.fitko.de/fit-connect/events/submit-reply dokumentiert der Zustelldienst, dass die Antwort abgesendet wurde.

   Das Event enthält das Objekt authenticationTags, das die Authentication Tags der verschlüsselten Inhalte enthält.
     * metadata: Authentication Tag des Metadatensatzes
     * data: Authentication Tag des Fachdatensatzes
     * attachments: Objekt mit den Authentication Tags der Anlagen mit ihren attachmentIds als Schlüssel

   Beispiel: submit-reply payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "25d2eb77-458d-4c9d-991c-6428c4651646",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "reply:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/submit-reply": {
   "authenticationTags": {
   "metadata": "XFBoMYUZodetZdvTiFvSkQ",
   "data": "UCGiqJxhBI3IFVdPalHHvA",
   "attachments": {
   "0b799252-deb9-42b0-98d3-c50d24bbafe0": "rT99rwrBTbTI7IJM8fU3El",
   "25abf553-0e53-43b9-a14a-1581b32a9ee5": "i7226HEB7IchCxNuh7lCiu",
   "046a9fa5-bed6-494b-aab6-d41056c6db79": "d48LxeolRdtFF4nzQibeYO"
   }
   }
   }
   }
   }

  notify-reply

   Mit dem Event https://schema.fitko.de/fit-connect/events/notify-reply dokumentiert der Zustelldienst, dass das sendende System Kenntnis von der Antwort
   erlangt hat. Wie dies erfolgt ist, wird mit dem Eintrag notifyType dokumentiert.
    1. "notifyType": "callback" - Der Sender wurde per Callback informiert.

   Beispiel: notify-reply (callback) payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "116c3c03-5f31-4d4c-9e65-d36e1e3895f7",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "reply:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/notify-reply": {
   "notifyType": "callback"
   }
   }
   }

    2. "notifyType": "polling" - Der Sender hat die Liste der Antworten über GET /v2/replys abgerufen.

   Beispiel: notify-reply (polling) payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "6127ff87-2786-4d0f-90fe-c531672f5de1",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "reply:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/notify-reply": {
   "notifyType": "polling"
   }
   }
   }

  reject-reply

   Mit dem Event https://schema.fitko.de/fit-connect/events/reject-reply dokumentiert der Zustelldienst, dass die Antwort zurückgewiesen wird.

   Alternativ kann auch der Zustelldienst Antworten als rejected markieren, wenn diese mehr als 14 Tage im Status submitted verbleiben. Unter
   Benachrichtigungen und Löschfristen finden sich hierzu genaue Angaben.

   Das Event enthält ein Array problems, dass die Fehler der Antwort dokumentiert. Der Aufbau ist an RFC 7807 angelehnt, lässt jedoch den Status aus, weil
   hier kein passender HTTP Status Code angegeben werden kann.
     * type: Fehlercode in Form einer URI.
     * title: Für Menschen verständliche Fehlermeldung
     * detail: Details zum Fehler, z.B. eine technische Fehlermeldung (optional)
     * instance: Betroffener Teil der Übertragung. Mögliche Werte: reply, metadata, data, attachment: + UUID des Attachments oder other

   Beispiel: reject-reply payload im Falle eines Fehlers bei der Einreichung

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "4ac47caa-bce1-435a-b04f-3322b224b32e",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "reply:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/reject-reply": {
   "problems": [
   {
   "type": "https://schema.fitko.de/fit-connect/events/problems/authentication-tag-incorrect",
   "title": "Das Authentication Tag des Metadatensatzes ist ungültig",
   "detail": "Das Authentication Tag des Metadatensatzes stimmt nicht mit dem im Submit-Submission-Event angegebenen Wert überein.",
   "instance": "metadata"
   }
   ]
   }
   }
   }

  accept-reply

   Mit dem Event https://schema.fitko.de/fit-connect/events/accept-reply dokumentiert der Zustelldienst, dass die Antwort akzeptiert wurde.

   Das Event muss eine mit dem submit-reply Event übereinstimmende Liste von Authentication Tags enthalten. Das sendende System dokumentiert damit, dass
   es die Authentication Tags überprüft hat.

   Beispiel: accept-reply payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "8538165b-9ce3-4097-871d-5b9581a3b4d9",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "reply:F65FEAB2-4883-4DFF-85FB-169448545D9F",
   "txn": "case:F73D30C6-8894-4444-8687-00AE756FEA90",
   "events": {
   "https://schema.fitko.de/fit-connect/events/accept-reply": {
   "authenticationTags": {
   "metadata": "XFBoMYUZodetZdvTiFvSkQ",
   "data": "UCGiqJxhBI3IFVdPalHHvA",
   "attachments": {
   "0b799252-deb9-42b0-98d3-c50d24bbafe0": "rT99rwrBTbTI7IJM8fU3El",
   "25abf553-0e53-43b9-a14a-1581b32a9ee5": "i7226HEB7IchCxNuh7lCiu",
   "046a9fa5-bed6-494b-aab6-d41056c6db79": "d48LxeolRdtFF4nzQibeYO"
   }
   }
   }
   }
   }

   Sofern Probleme in der Antwort gefunden wurden, die jedoch nicht zu einer Zurückweisung geführt haben, werden diese analog zu der problems Liste in
   reject-reply dokumentiert.

   Beispiel: accept-reply payload im Falle eines unkritischen Fehlers bei der Einreichung

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "6872b19f-5ee2-47d7-a0e9-ebfe87ab2563",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "reply:F65FEAB2-4883-4DFF-85FB-169448545D9F",
   "txn": "case:F73D30C6-8894-4444-8687-00AE756FEA90",
   "events": {
   "https://schema.fitko.de/fit-connect/events/accept-reply": {
   "problems": [
   {
   "type": "https://schema.fitko.de/fit-connect/events/problems/schema-missing",
   "title": "Schemareferenz fehlt im Metadatensatz",
   "detail": "Die Referenz auf das Metadatenschema ('$schema') fehlt im Metadatensatz.",
   "instance": "metadata"
   }
   ],
   "authenticationTags": {
   "metadata": "XFBoMYUZodetZdvTiFvSkQ",
   "data": "UCGiqJxhBI3IFVdPalHHvA",
   "attachments": {
   "0b799252-deb9-42b0-98d3-c50d24bbafe0": "rT99rwrBTbTI7IJM8fU3El",
   "25abf553-0e53-43b9-a14a-1581b32a9ee5": "i7226HEB7IchCxNuh7lCiu",
   "046a9fa5-bed6-494b-aab6-d41056c6db79": "d48LxeolRdtFF4nzQibeYO"
   }
   }
   }
   }
   }

  delete-reply

   Mit dem Event https://schema.fitko.de/fit-connect/events/delete-reply dokumentiert der Zustelldienst, dass die Antwort gelöscht wurde. Die Fristen,
   wann Antworten mit welchem Status gelöscht werden, finden sich unter Benachrichtigungen und Löschfristen.

   Beispiel: delete-reply payload

   {

   "$schema": "https://schema.fitko.de/fit-connect/set-payload/1.2.2/set-payload.schema.json",
   "jti": "29d091aa-86ab-4806-beb5-fb462d18e2a1",
   "iss": "https://test.fit-connect.fitko.dev/submission-api",
   "iat": 1622796532,
   "sub": "reply:02bf1d9f-282d-4abf-810a-c4104baf0afe",
   "txn": "case:452b5ee6-35df-441a-bd39-6141723cf914",
   "events": {
   "https://schema.fitko.de/fit-connect/events/delete-reply": {
   }
   }
   }
```
