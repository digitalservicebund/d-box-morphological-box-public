# Verwendung von Callbacks

- Quelle: <https://docs.fitko.de/fit-connect/docs/details/callbacks>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Verwendung von Callbacks

   Der Zustelldienst von FIT-Connect informiert Onlinedienste und Verwaltungssysteme (API Clients) aktiv über neue Einreichungen oder Statusupdates.
   Hierzu werden HTTP-Callbacks genutzt, die auch als Webhooks bezeichnet werden. Webhooks ermöglichen es, API Clients aktiv über diese Ereignisse zu
   informieren, ohne dass eine regelmäßige Abfrage (Polling) nötig wäre. Technisch werden Webhooks als HTTP-POST-Request realisiert.

   Im Folgenden verwenden wir den Begriff "Callback" synonym zu "Webhook".

Callback-Übersicht

   Callbacks werden sowohl an einreichende als auch an empfangende Systeme versendet. Hier gilt grundsätzlich, dass ein System über alle Veränderungen an
   einer Einreichung oder Rückantwort informiert wird, die es nicht selbst vorgenommen hat. Sobald eine Einreichung an den Zustelldienst übergeben wurde,
   wird das einreichende System über Zustandsänderungen informiert. Das empfangende System erhält hingegen nur einen Callback über die Einreichung, sofern
   dieser konfiguriert wurde.

   Der Zustelldienst führt automatisiert Veränderungen an Einreichungen und Rückantworten durch, sobald Limits zur automatischen Löschung oder Ablehnung
   überschritten wurden. In diesem Fall müssen sowohl die einreichende Seite als auch die empfangende Seite informiert werden.
              Ereignis                 Auslöser      Callback an Onlinedienst Callback an Verwaltungssystem  Callback-Typ
   create-submission               Onlinedienst      nein                     nein                          -
   submit-submission               Onlinedienst      nein                     ja                            NEW_SUBMISSIONS
   notify-submission               Zustelldienst     ja                       nein                          NEW_EVENTS
   forward-submission (deprecated) Verwaltungssystem ja                       nein                          NEW_EVENTS
   reject-submission               Verwaltungssystem ja                       nein                          NEW_EVENTS
   reject-submission               Zustelldienst     ja                       ja                            NEW_EVENTS
   accept-submission               Verwaltungssystem ja                       nein                          NEW_EVENTS
   delete-submission               Zustelldienst     ja                       ja                            NEW_EVENTS
   create-reply                    Verwaltungssystem nein                     nein                          -
   submit-reply                    Verwaltungssystem nein                     ja                            NEW_REPLIES
   notify-reply                    Zustelldienst     ja                       nein                          NEW_EVENTS
   reject-reply                    Onlinedienst      ja                       nein                          NEW_EVENTS
   reject-reply                    Zustelldienst     ja                       ja                            NEW_EVENTS
   accept-reply                    Onlinedienst      ja                       nein                          NEW_EVENTS
   delete-reply                    Zustelldienst     ja                       ja                            NEW_EVENTS

Callback-URL

   API Clients stellen zum Empfang von Callbacks einen HTTP-Endpunkt bereit, an den der Zustelldienst einen HTTP-POST-Request übermitteln kann. Die URL
   dieses Callback-Endpunkts bezeichnen wir als Callback-URL (callbackUrl). Sie wird von dem an FIT-Connect angebundenen System festgelegt.

   Der Callback-Endpunkt muss von außen über das Internet erreichbar sein, damit der Zustelldienst Callbacks an diesen Endpunkt senden kann. Der
   Callback-Endpunkt muss zwingend über TLS (HTTPS) abgesichert sein und darf nicht ohne Verschlüsselung via HTTP erreichbar sein. Der Zustelldienst wird
   Callbacks nur über eine via HTTPS verschlüsselte Verbindung auslösen.

   Wird eine Callback-URL an einem Zustellpunkt gesetzt, überprüft FIT-Connect die Erreichbarkeit dieser URL mit einer HTTP HEAD-Anfrage. Auf diese
   Anfrage muss ihr System in einer von Ihnen gewünschten Weise mit HTTP antworten (es gilt also jeder Status Code als "erreichbar"). Schlägt diese
   Überprüfung auf Erreichbarkeit fehl, wird das Speichern oder Editieren des Zustellpunkts mit einer entsprechenden Fehlermeldung zurückgewiesen. Für die
   Callback-URLs, die Onlinedienste dynamisch an Submissions oder Cases setzen können, wird eine solche Überprüfung nicht durchgeführt.

   API Clients müssen auf eingehende Callbacks mit einer HTTP-Response mit einem Status Code 2xx Success antworten (z. B. 200 OK oder 202 Accepted).

   Eine solche Callback-URL kann z. B. wie folgt aussehen:

   https://fachverfahren.beispielstadt.example.org/callbacks/fit-connect

   Warnung

   Da der Callback-Endpunkt öffentlich über das Internet erreichbar ist, müssen angebundene Systeme prüfen, ob eingehende Callbacks von einem
   vertrauenswürdigen Zustelldienst stammen. Zu diesem Zweck müssen die im Abschnitt Prüfung von Callbacks beschriebenen Prüfungen durchgeführt werden.

Konfiguration von Callbacks

   Eine Konfiguration von Callbacks ist über das Self-Service-Portal und über die API-Endpunkte PUT /v2/destinations/{destinationId} bzw. PATCH
   /v2/destinations/{destinationId} möglich. Bei der Konfiguration werden die Callback-URL und ein Callback-Secret vom API Client festgelegt. Das
   Callback-Secret dient der Überprüfung der Echtheit (Authentizität) von eingehenden Callbacks (siehe nächster Abschnitt). Das angegebene Callback-Secret
   kann über die API nur geschrieben und aktualisiert, aber nicht gelesen werden und darf nicht an Dritte weitergegeben werden.

   Ein sicheres Callback-Secret kann über die folgenden Aufrufe erzeugt werden:
     * Python: python -c 'import secrets; print(secrets.token_urlsafe(32))'
     * Ruby: ruby -rsecurerandom -e 'puts SecureRandom.hex(32)'
     * pwgen: pwgen --secure 64 1

   Die Einrichtung von Callbacks im Self-Service-Portal wird auf der Seite Zustellpunkt anlegen näher beschrieben.

Konfiguration von Callbacks für Zustellpunkte

   Über die API können Callbacks für Zustellpunkte wie folgt konfiguriert werden:
     * curl
     * Java
     * JavaScript

   $ DESTINATION_API=https://test.fit-connect.fitko.dev/destination-api

   $ JWT_TOKEN=...
   $ DESTINATION_ID=...
   $ CALLBACK_URL=https://fachverfahren.beispielstadt.example.org/callbacks/fit-connect
   $ CALLBACK_SECRET=insecure_unsafe_qHScgrg_kP-R31jHUwp3GkVkGJolvBchz65b74Lzue0
   $ curl -X PATCH \
   -H "Content-Type: application/json" \
   -H "Authorization: Bearer $JWT_TOKEN" \
   --data "{ \"callback\": { \"url\": \"$CALLBACK_URL\", \"secret\": \"$CALLBACK_SECRET\" }}" \
   "$DESTINATION_API/v2/destinations/$DESTINATION_ID"
   Hinweis

   Hier wird zu einem späteren Zeitpunkt ein Codebeispiel eingefügt. Danke für Ihr Verständnis.
   Hinweis

   Hier wird zu einem späteren Zeitpunkt ein Codebeispiel eingefügt. Danke für Ihr Verständnis.

Konfiguration von Callbacks für Onlinedienste

   Bei der Erstellung von Einreichungen über die Submission-API (POST /v2/submissions) kann ebenfalls ein Callback konfiguriert werden. Hierbei muss
   ebenfalls ein Callback-Objekt mit url und secret übergeben werden, damit der Zustelldienst über neue Ereignisse informieren kann.

   Die Callback-Adresse muss für jede Adresse neu gesetzt werden und stellt einen optionalen Parameter dar.

   Alternativ können die Events über die Submission-API unter Angabe der Case-ID unter GET /v2/cases/{caseId}/events oder mit der Submission-ID unter GET
   /v2/submissions/{submissionId}/events abgefragt werden.

Prüfung von Callbacks

   Da der Callback-Endpunkt öffentlich über das Internet erreichbar ist, müssen angebundene Systeme prüfen, ob eingehende Callbacks von einem
   vertrauenswürdigen Zustelldienst stammen.

   Hierzu enthalten Callbacks einen Message Authentication Code (HMAC) gemäß RFC 2104 auf Basis des angegebenen Callback-Secrets. Ein HMAC kann als
   „symmetrische Signatur“ verstanden werden und ermöglicht die Prüfung der Herkunft und Integrität eines eingehenden Callbacks.

   Der HMAC wird im HTTP-Header callback-authentication übertragen. Um Replay-Angriffe zu vermeiden, enthält er einen aktuellen Timestamp. Dieser
   Timestamp wird im HTTP-Header callback-timestamp übertragen. Bei der Prüfung der Echtheit des ausgelösten Callbacks müssen API Clients prüfen, dass der
   angegebene Timestamp nicht älter als 5 Minuten ist.

   Das folgende Beispiel zeigt die Verwendung der HTTP-Header callback-authentication und callback-timestamp. Es ist zu beachten, dass das Feld
   submissionIds als deprecated markiert wurde und damit optional ist (das Feld wird in v3 entfernt). Es wurde durch das Feld submissions ersetzt.

   POST /callbacks/fit-connect

   callback-authentication:
   2056b372b5bcec06d8f11ab79b84b42d6cbe1c8e1178cdfa36e4385dcf717758aaa7599f417d9ec3e079087884f4fd59680bf713621383e2d4414ef74fb10df3
   callback-timestamp: 1672527599
   {
   "type":"https://schema.fitko.de/fit-connect/submission-api/callbacks/new-submissions",
   "submissions":[
   {
   "destinationId":"d12caea8-f372-4eb1-b102-b0a228253a11",
   "submissionId":"f39ab143-d91a-474a-b69f-b00f1a1873c2",
   "caseId":"9eec7d3e-dc66-4f82-9f52-1520bf96a32e"
   }
   ]
   }

   Der HMAC wird gebildet aus dem im HTTP-Header callback-timestamp übertragenen Zeitstempel und dem im HTTP-Body übertragenen Payload, getrennt durch das
   Zeichen . (Punkt), jeweils UTF-8-kodiert. Der HMAC wird hexadezimal kodiert übertragen. Als Hash-Algorithmus wird SHA-512 verwendet.

   callback-authentication = HEX(HMAC(key={callback-secret}, message={timestamp}.{http-body}))

   Um den HMAC zu verifizieren, bildet der API Client mithilfe des Callback Secret den HMAC nach und vergleicht diesen mit dem im HTTP-Header
   callback-authentication übertragenen HMAC.

   Bei der Prüfung müssen die folgenden Implementierungshinweise zwingend beachtet werden:
     * Das Callback Secret muss in den API Clients konfigurierbar sein und darf nicht fest im Quellcode eines API Clients verankert sein.
          + Dies kann beispielsweise durch die Konfiguration des Callback Secret in einer Konfigurationsdatei oder über eine Umgebungsvariable ($ export
            CALLBACK_SECRET=your_secret) erreicht werden.
     * Bei der Erzeugung des HMAC muss der Hash-Algorithmus SHA-512 verwendet werden.
     * Es muss geprüft werden, dass der angegebene Zeitstempel nicht älter als 5 Minuten ist.
     * Beim Vergleich des übertragenen HMAC und des vom API Client gebildeten HMAC muss ein zeitlich konstanter Zeichenfolgenvergleich (constant time
       string comparison) verwendet werden.
          + In Python kann dies über die Verwendung der Methode hmac.compare_digest erreicht werden.
          + In Ruby kann dies über die Verwendung der Methode secure_compare erreicht werden.
     * Callbacks mit ungültigem HMAC müssen von API Clients ignoriert werden.

   Dabei ist zunächst
     * der Zeitstempel (callback-timestamp-Header) und anschließend
     * der HMAC (callback-authentication-Header) zu prüfen:

   # 1. Timestamp überprüfen

   current_time_epoch = int(time.time())
   seconds_five_minutes = 60 * 5
   if current_time_epoch - request['headers']['callback-timestamp'] > seconds_five_minutes:
   print('Error: timestamp too old')
   sys.exit(1)
   else:
   print('timestamp ok')
   # 2. HMAC berechnen
   payload = str(request['headers']['callback-timestamp']) + '.' + request['body']
   expected_hmac = hmac.digest(CALLBACK_SECRET.encode("utf-8"), payload.encode("utf-8"), digest=sha512)
   expected_hmac_hex = binascii.hexlify(expected_hmac).decode('utf-8')
   print('hmac', expected_hmac_hex)
   # 3. Berechneten HMAC mit HMAC aus HTTP-Header vergleichen
   if not hmac.compare_digest(request['headers']['callback-authentication'], expected_hmac_hex):
   print('Error: invalid hmac')
   sys.exit(2)
   else:
   print('hmac ok')

   Das vollständige Script findet sich auch zur freien Verwendung im FIT-Connect-Tools-Repository.

Testen eigener Callbacks

   Für einen einfachen Funktionstest kann Webhook.site als öffentlich erreichbarer Callback-Endpunkt verwendet werden. Eingehende HTTP-Anfragen werden
   dort direkt im Browser angezeigt.
   hinweis

   Webhook.site ist ein externer Dienst. Verwenden Sie diesen ausschließlich in einer Testumgebung und mit Testdaten.

Callback-Endpunkt erstellen

   Öffnen Sie Webhook.site. Beim Aufruf wird automatisch eine individuelle URL erzeugt, beispielsweise:

   https://webhook.site/12345678-abcd-1234-abcd-123456789012

   Diese URL wird als Callback-URL der Destination verwendet.

Callback-Secret erzeugen

   Das Callback-Secret wird selbst festgelegt und dient FIT-Connect zur Absicherung der Callback-Nachrichten mittels HMAC.

   Ein geeignetes Secret kann beispielsweise mit Python erzeugt werden:

   python -c 'import secrets; print(secrets.token_urlsafe(32))'

   Die Ausgabe sieht beispielsweise wie folgt aus:

   R31jHUwp3GkVkGJolvBchz65b74Lzue0exampleSecret

   Wichtig

   Das Callback-Secret ist vertraulich zu behandeln und darf nicht an Dritte weitergegeben werden.

Callback konfigurieren

   Konfigurieren Sie an der Destination die zuvor ermittelten Werte:

   Callback-URL:

   https://webhook.site/12345678-abcd-1234-abcd-123456789012
   Callback-Secret:
   R31jHUwp3GkVkGJolvBchz65b74Lzue0exampleSecret

   Beim Speichern der Callback-URL prüft FIT-Connect deren Erreichbarkeit mit einer HTTP-HEAD-Anfrage.

Callback auslösen

   Übermitteln Sie anschließend mit einem Onlinedienst eine Einreichung an die konfigurierte Destination.

   Der Zustelldienst informiert die empfangende Destination über die neue Einreichung mit einem HTTP-POST an die hinterlegte Callback-URL.

   Der eingehende Request kann anschließend direkt auf Webhook.site betrachtet werden. Dort sind unter anderem die HTTP-Header und der JSON-Body des
   Callbacks sichtbar.

   Ein Callback für eine neue Einreichung enthält beispielsweise:

   {

   "type": "https://schema.fitko.de/fit-connect/submission-api/callbacks/new-submissions",
   "submissions": [
   {
   "destinationId": "...",
   "submissionId": "...",
   "caseId": "..."
   }
   ]
   }

   Zusätzlich übermittelt FIT-Connect die Header callback-timestamp und callback-authentication. Mit diesen und dem zuvor festgelegten Callback-Secret
   kann ein Verwaltungssystem die Authentizität und Integrität des Callbacks prüfen.
   hinweis

   Webhook.site dient in diesem Beispiel lediglich dazu, den Callback sichtbar zu machen. Eine produktive Implementierung muss insbesondere die von
   FIT-Connect vorgegebenen Prüfungen des Zeitstempels und des HMAC durchführen.

Wiederholung von Callbacks

   Falls Callbacks nicht erfolgreich zugestellt werden können, gibt es im Zustelldienst Retry-Mechanismen, die weitere Zustellversuche durchführen.
   Hierbei werden über einen Backoff-Mechanismus die Callback-Nachrichten erneut an die hinterlegte Adresse gesendet. Es wird bis zu 14 Tage versucht den
   Callback beim zu informierenden System zuzustellen.

   Eine Ausnahme bildet der Callback für submit-submission Events. Hier wird im Fehlerfall die Zustellung des Callbacks so lange versucht, bis die
   Einreichung sich nicht mehr im Zustand SUBMITTED befindet. Das kann durch die Annahme oder Ablehnung durch das empfangende System oder durch die
   automatische Ablehnung durch den Zustelldienst erfolgen.

   Hier finden Sie weitere Informationen zu Benachrichtigungen und Löschfristen.
```
