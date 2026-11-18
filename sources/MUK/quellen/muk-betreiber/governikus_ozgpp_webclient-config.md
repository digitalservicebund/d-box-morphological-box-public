# OZG-PLUS-Postfach – Konfiguration des Webclients (Governikus)

Quelle: https://ozgpp.de/config/app/config.json
Abgerufen: 2026-11-18

Unveränderte JSON-Konfiguration (u. a. maxNumberOfMailboxes, retentionPeriod, maxUplSize):

```json
{
  "version": "1.8.2",
  "environment": "Production",
  "services": {
    "api": {
      "url": "https://ozgpp.de"
    },
    "auth": {
      "loginUrl": "https://ozgpp.de/ozg/auth/login",
      "logoutUrl": "https://ozgpp.de/ozg/auth/logout"
    }
  },
  "session": {
    "timeoutWarning": 300,
    "pollingIntervalTime": 60
  },
  "links": {
    "bundId": "https://id.bund.de/",
    "contactPage": "https://id.bund.de/",
    "mukStart": "https://info.mein-unternehmenskonto.de/",
    "mukLogin": "https://mein-unternehmenskonto.de/mup/meinelster",
    "mukContact": "https://info.mein-unternehmenskonto.de/kontakt/",
    "mukFaq": "https://info.mein-unternehmenskonto.de/hilfe/"
  },
  "passwd-policy": {
    "matches": [
      "^.{8,64}$","[a-zäöüß]","[A-ZÄÖÜ]","[0-9]","^[^\\s]*$"
    ]
  },
  "crypto": {
    "pre": {
      "curve": "secp521r1"
    }
  },
  "mailboxConfig": {
    "mailboxName": {
      "match": "^[a-zA-ZäöüÄÖÜß\\d\\-_/][a-zA-ZäöüÄÖÜß\\d\\s\\-_/]{1,48}[a-zA-ZäöüÄÖÜß\\d\\-_/]$",
      "minlength": 3,
      "maxlength": 50,
      "helpText": {
        "de": "Der Name eines Funktionspostfachs darf max. 50 Zeichen beinhalten. Es sind Zahlen, Buchstaben, Leerzeichen und die Sonderzeichen - _ / (= Minus, Unterstrich, Schrägstrich) erlaubt."
      }
    },
    "maxNumberOfMailboxes": 500
  },
  "messageConfig": {
    "retentionPeriod": 3,
    "maxUplSize": {
      "byte": 2.097152e+08,
      "text": "200 MB"
    },
    "file": {
      "options": {
        "multiple": true,
        "max": 1000,
        "types": []
      }
    },
    "letterBody": {
      "permittedHtmlTags": ["br","p","ul","ol","li","strong","em"]
    }
  },
  "listSettings": {
    "pageSize": 10,
    "searchSize": 100
  }
}
```
