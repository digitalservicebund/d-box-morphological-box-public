# Authentifizierung

- Quelle: <https://docs.fitko.de/fit-connect/docs/getting-started/authentication>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Authentifizierung

   Nachfolgend wird der Abruf von Access Tokens beim Autorisierungsdienst sowie der Zugriff auf die FIT-Connect Submission API mittels Access Tokens
   beschrieben.
   Hinweis

   Als Voraussetzungen hierfür ist es notwendig, Accounts für API Clients im Self-Service-Portal zu registrieren.

   Access Tokens enthalten die Berechtigungen (OAuth-Scopes), die definieren, auf welche Ressourcen oder Funktionen ein Client zugreifen darf. Jedem
   Client sind bei der Registrierung eine oder mehrere Scopes zugeordnet. Ruft ein Client ein Access Token ohne explizite Angabe von Scopes ab, erhält er
   automatisch ein Token mit allen ihm zugewiesenen Scopes. Alternativ kann ein Client beim Abruf gezielt nur bestimmte Scopes anfordern – erhält dann
   aber auch nur Zugriff auf die damit verbundenen Berechtigungen.

   Ein Verwaltungssystem-Client besitzt typischerweise zwei Scopes:
     * 'manage-destinations': für das Verwalten der Zustellpunkte
     * 'subscribe-destinations': für das Abholen von Übertragungen

   Theoretisch könnte ein Client aus Sicherheitsgründen nur den Scope 'subscribe-destinations' anfordern, wenn er ausschließlich Übertragungen abholt.
   Praktisch wird davon dringend abgeraten, da sich Scope-Namen ändern können. Wenn ein Client explizit einen bestimmten Scope anfordert und dieser
   umbenannt oder angepasst wird, kann er kein gültiges Access Token mehr erhalten. Deshalb empfehlen wir, beim Abruf auf die explizite Angabe von Scopes
   zu verzichten und immer alle zugewiesenen Scopes automatisch mitzunehmen.

   Weitere Details und konkrete Beispiele zur sicheren Implementierung findest du auf dieser Seite.

Abruf von Access Tokens beim Autorisierungsdienst

   Hinweis

   Die URL der Submission API und die OAuth-Token-URL finden sich im Artikel Betriebsumgebungen.

   Fast alle Anfragen an die FIT-Connect Submission API müssen authentifiziert werden. Hierfür ist ein Access Token notwendig, das beim
   Autorisierungsdienst über die hierfür vorgesehene OAuth-Token-URL abgerufen werden kann. Für den Abruf von Access Tokens ist die Konfiguration eines
   API Client im Self-Service-Portal nötig. Bei der Konfiguration eines API Client werden client_id (Zugangs-Kennung) und client_secret
   (Zugangs-Geheimnis) erzeugt.

   Da ein Token max. 30 Minuten gültig ist, muss dieses regelmäßig erneuert werden.

Abfrage von Access Tokens mit allen zugewiesenen Scopes

   Achtung

   Beim Abruf von Access-Tokens ist es ist technisch möglich, die gewünschten OAuth-Scopes explizit anzugeben. Jedoch können Namen und Anzahl von
   OAuth-Scopes im Rahmen von API Updates durch FIT-Connect geändert werden. Und dies auch ohne Veröffentlichung einer Major-Version und der Beachtung der
   zugehörigen Migrationsfrist. Die explizite Angabe von OAuth-Scopes kann beim Abholen eines Access-Tokens unter diesen Umständen zu einem Bruch der
   Verbindung führen. Daher raten wir ausdrücklich von der expliziten Angabe von OAuth-Scopes ab!
     * .NET (SDK)
     * curl
     * Java (SDK)
     * C#
     * JavaScript

   Den Abruf von Access Token kapselt die FIT-Connect Java-SDK vollständig. Hierzu müssen lediglich client_id (Zugangs-Kennung) und client_secret
   (Zugangs-Geheimnis) von Sender und/oder Empfänger sowie die verwendete FIT-Connect Umgebung in einer Konfigurationsdatei angeben und daraus eine
   ApplicationConfig erzeugt werden. Danach wird automatisch ein Access Token geholt, sofern dies für einen Aufruf notwendig ist:

   var configPath = Path.of("...");

   var config = ApplicationConfigLoader.loadConfigFromPath(configPath.resolve("config.yml"));
   var senderClient = ClientFactory.createSenderClient(config);
   senderClient.listCases();

   Diese Funktionalität wird durch das .NET-SDK bereits intern umgesetzt und ist durch einen Aufruf der SDK-Methode ClientFactory.GetSenderClient(...)
   automatisch mit abgedeckt:

   var sender = ClientFactory

   .GetSenderClient(FitConnectEnvironment.Testing, clientId, clientSecret, logger)

   Der Quellcode oben ist ein Auszug aus dem Projekt ConsoleAppExample, das im Repository Codebeispiele - examples der FITKO hinterlegt ist. Eine
   Beschreibung des .NET-SDKs finden Sie im Hauptmenü unter "SDKs > .NET-SDK".
     __________________________________________________________________________________________________________________________________________________

   // TBD

   const CLIENT_ID = 'af30fa56-4493-4e38-8298-7cc532d52469';

   const CLIENT_SECRET = 'eTVnxAEvCRz0OXexqi62PV_72gOMeiXd8zOVh9ruWhk';
   fetch('https://auth-testing.fit-connect.fitko.dev/token', {
   'method': 'POST',
   'body': 'grant_type=client_credentials&client_id=' + CLIENT_ID + '&client_secret=' + CLIENT_SECRET,
   'headers': {
   'Content-Type': 'application/x-www-form-urlencoded',
   },
   'redirect': 'follow'
   })
   .then((response) => response.text())
   .then((body) => {
   console.log(body);
   });

   $ export OAUTH_URL=https://auth-testing.fit-connect.fitko.dev/token

   $ export CLIENT_ID=<client_id>
   $ export CLIENT_SECRET=<client_secret>
   $ curl \
   -H "Content-Type: application/x-www-form-urlencoded" \
   --data "grant_type=client_credentials&client_id=$CLIENT_ID&client_secret=$CLIENT_SECRET" \
   -X POST "$OAUTH_URL"
   {
   "access_token": "eyJraWQi...",
   "expires_in": 1800,
   "scope": "send:region:DE01010",
   "token_type": "Bearer"
   }
     __________________________________________________________________________________________________________________________________________________

   Wenn keine spezifischer Scope angefragt wird (siehe nächster Abschnitt), enthält der Access Token alle Scopes, die zu diesem Zeitpunkt in den
   Zugangsdaten im Self-Service-Portal hinterlegt wurden.

Zugriff auf die API mittels Access Token

   Access Tokens werden vom Autorisierungsdienst erzeugt und DÜRFEN NIEMALS an dritte Systeme (mit Ausnahme des Zustelldienstes zum Zwecke der
   Authentifizierung) weitergegeben werden.

   Beim Zugriff auf die Submission API muss ein Access Token im Authorization-Header mit Bearer-Authentifizierungsschema gemäß RFC 6750 an den
   Zustelldienst übermittelt werden:

   Beispiel

   POST /submission-api/v2/submissions HTTP/1.1

   Host: test.fit-connect.fitko.dev
   Authorization: Bearer ey...
   Hinweis

   Die URL der Submission API findet sich im Artikel Betriebsumgebungen.
```
