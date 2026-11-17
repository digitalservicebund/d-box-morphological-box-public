# Rückkanal

- Quelle: <https://docs.fitko.de/fit-connect/docs/metadata/replyChannel>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Rückkanal

replyChannel

   Diese Seite beschreibt den Bereich replyChannel der Metadaten.

   Dieser Bereich dient dazu, dem Empfänger eines Antrags (dem Verwaltungssystem) mitzuteilen, auf welchem Weg die antragstellende Person die Antwort zum
   Antrag erhalten will.

   Der gewählte Rückkanal muss mit den angebotenen Rückkanälen kompatibel sein, d. h. der gewählte Rückkanal muss vom Zustellpunkt des zuständigen
   Verwaltungssystems auch angeboten werden. Deshalb ist es wichtig, sich vor der Einreichung eines Antrags über die Rückkanäle zu informieren, die der
   zuständige Zustellpunkt anbietet.
   Die Seite Zustellpunkt ermitteln beschreibt im Absatz Zustellpunkt-Informationen über die Submission API ermitteln, wie Sie die Submission API von
   FIT-Connect verwenden, um zu erfahren, welche Rückkanäle ein Zustellpunkt eines Verwaltungssystems anbietet.

Welche Rückkanäle gibt es?

   Die folgenden Rückkanäle (Wege zu antworten) sind möglich: E-Mail, De-Mail, ELSTER, FIT-Connect, BundID Auf der Seite Überblick finden Sie dazu weitere
   Details im Absatz Schema für Metadaten.

Angaben zum Rückkanal können entfallen

   Der Bereich replyChannel kann fehlen, d.h. er ist optional (nicht erforderlich). Wenn der Bereich fehlt, dann bedeutet das, dass das Verwaltungssystem
   auf dem Postweg antworten wird.

FIT-Connect als Rückkanal

   Im folgenden Beispiel kann das Verwaltungssystem über FIT-Connect antworten, weil unter replyChannel der Wert fitConnect eingetragen ist. Das sendende
   System (der Onlinedienst) hat im Attribut processStandards den Prozessstandard urn:xoev-de:bmk:standard:xbau_2.4 eingetragen. Das sendende System kann
   auch mehrere Prozessstandards zur Auswahl anbieten, sofern sie vom empfangenden System auch angeboten werden.
   Die Seite Zustellpunkt ermitteln beschreibt im Absatz Zustellpunkt-Infromationen über die Submission API ermitteln wie Sie die Submission API von
   FIT-Connect verwenden, um zu erfahren, welche Prozessstandards der Zustellpunkt des zuständigen Verwaltungssystem anbietet.

   {

   "$schema": "https://schema.fitko.de/fit-connect/metadata/1.2.0/metadata.schema.json",
   // ...
   "replyChannel": {
   "fitConnect": {
   "processStandards": [
   "urn:xoev-de:bmk:standard:xbau_2.4",
   ],
   "encryptionPublicKey": {
   // ...
   }
   }
   }
   }

   Im Beispiel oben unterstützt das sendende System FIT-Connect als Rückkanal. Das Verwaltungssystem kann diese Möglichkeit nutzen und über FIT-Connect
   antworten. Der Sender (Onlinedienst) unterstützt den Prozessstandard XBau in der Version 2.4. Das Verwaltungssystem soll die Antwort (Reply) zum Antrag
   verschlüsseln, mit dem Schlüssel, der im Feld: encryptionPublicKey eingetragen ist (im Beispiel oben nicht ausgefüllt).
   Wichtig!

   Der im Feld encryptionPublicKey eingetragene Schlüssel (Public Key) dient der Verschlüsselung von Antworten (Replies) des Verwaltungssystems an den
   Onlinedienst.
   Um die Kommunikationskanäle zwischen einem Verwaltungssystem und mehreren Antragstellenden kryptographisch zu trennen, MUSS im Onlinedienst für jede
   antragstellende Person ein neues Schlüsselpaar generiert werden. Idealerweise SOLLTE für jeden einzelnen Vorgang (Case) ein neues Schlüsselpaar
   generiert werden. Hersteller und Betreiber von Onlinediensten MÜSSEN geeignete Maßnahmen ergreifen, um kryptografische Schlüssel derart zu speichern
   bzw. aufzubewahren, sodass Unbefugte nicht darauf zugreifen können. Betreiber MÜSSEN in diesem Zusammenhang die Einhaltung der Vorgaben aus
   Grundschutz-Baustein CON.1 sicherstellen.

E-Mail als Rückkanal

Verschlüsselung mit PGP

   Wird der Rückkanal eMail gewählt, dann kann im Feld pgpPublicKey ein öffentlicher Schlüssel zur Verschlüsselung der Antwort angegeben werden. Dieser
   ist wie folgt zu befüllen:

   Der Inhalt des Feldes ist ein durch gpg --export --armor "$KEY_ID" erzeugter ASCII-String in dem Zeilenumbrüche durch die Escape-Sequenz (\n) ersetzt
   wurden.

   Die Kodierung erfolgt wie folgt:

   $ KEY_ID='EEADEDE070FA9658F0BE084AC20D38DB3EA14B59'

   $ gpg --export --armor "$KEY_ID" | python3 -c 'import sys; print(sys.stdin.read().replace("\n", "\\n"))'
   -----BEGIN PGP PUBLIC KEY BLOCK-----\n\nmQINBEBEADSlS...9jlG\n-----END PGP PUBLIC KEY BLOCK-----\n

   Die Dekodierung ist obiger Schritt in Gegenrichtung:

   $ KEY_ASCII='-----BEGIN PGP PUBLIC KEY BLOCK-----\n\nmQINBEBEADSlS...9jlG\n-----END PGP PUBLIC KEY BLOCK-----\n'

   $ echo "$KEY_ASCII" | python3 -c 'import sys; print(sys.stdin.read().replace("\\n", "\n"))'
   -----BEGIN PGP PUBLIC KEY BLOCK-----
   mQINBEBEADSlS...9jlG
   -----END PGP PUBLIC KEY BLOCK-----

   Der gewählte Rückkanal kann nun folgendermaßen angegeben werden:

   {

   "$schema": "https://schema.fitko.de/fit-connect/metadata/1.2.0/metadata.schema.json",
   // ...
   "replyChannel": {
   "eMail": {
   "address": "onlineservice@example.org",
   "pgpPublicKey": "-----BEGIN PGP PUBLIC KEY BLOCK-----\n\n ..."
   }
   }
```
