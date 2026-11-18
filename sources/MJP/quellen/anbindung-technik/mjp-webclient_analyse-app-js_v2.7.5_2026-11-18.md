# Mein Justizpostfach – Webclient: Auszüge aus dem ausgelieferten JavaScript (Version 2.7.5)

Quelle: https://mjp.justiz.de/app.js?v=2.7.5 (eingebunden von https://mjp.justiz.de/, `<meta name="description" content="Webclient für Mein Justizpostfach">`)
Abgerufen: 2026-11-18
SHA-256 `app.js`: ac69b1bb036afe481493a8103fe10bb8f0239c22ca8805b3360ac39c691ccf75 (927 429 Byte, minifiziert)

Hinweis: Die Datei wurde nur statisch gelesen, nicht ausgeführt. Sie ist nicht abgelegt (proprietärer Code, Copyright-Vermerke der Governikus GmbH & Co. KG). Unten stehen wörtliche Auszüge mit Byte-Offset `[n]` in der genannten Fassung; Variablennamen sind minifiziert. Ergänzend abgelegt: `mjp-webclient_config_v2.7.5_2026-11-18.json`, `mjp-webclient_productinfo_v2.7.5_2026-11-18.json`, `mjp-webclient_i18n-messages-de_v2.7.5_2026-11-18.json` (Texte der Oberfläche, für Zitate formatiert), `mjp-webclient_xoev-egvp-roles_2026-11-18.json`.

## 1 Hersteller, Bibliotheken

- `Productinfo.json`: `"herstellerDesProdukts": "Governikus GmbH & Co. KG"`, `"version": "2.7.5"`.
- Mehrere Module tragen `Copyright (C) 2022 Governikus GmbH & Co KG` [616136], [682632], [691720], [694321]: XML-Encryption-Parser mit OSCI-Namensraum (`osci128:IvLength`, `xenc:EncryptedKey`, `ds:KeyInfo`) und XJustiz-Erzeugung (`tns:nachricht.gds.basisnachricht.0005006`, `ruecksendungEEB.erforderlich`).
- Kryptobibliotheken: Web Crypto API (`crypto.subtle`) und node-forge (`pkcs12.toPkcs12Asn1`, `pki.createCertificate`).

## 2 Schlüsselerzeugung im Browser

- Algorithmen [48900]: `asymmetricEncryption:{"rsa-oaep":{name:"RSA-OAEP",modulusLength:3072,…,hash:"SHA-256"}}`; symmetrisch AES-GCM/AES-CBC 128–256.
- Schlüsselpaar und Zertifikat [443927]: `async function Rp(e){const t=await crypto.subtle.generateKey(Rt.asymmetricEncryption["rsa-oaep"],!0,["encrypt","decrypt"]) … s.validity.notAfter=n` (Gültigkeit 3 Jahre: `n.setUTCFullYear(i.getUTCFullYear()+3)`) `… s.setSubject(a),s.setIssuer(a)` → selbstsigniertes X.509-Zertifikat, im Browser erzeugt.
- PKCS#12 [481555]: `generateUserPKCS12KeyStore(e,t){…const{keyPair:r,forgePrivateKey:i,x509Cert:n}=await Rp(e) … a=$o.pkcs12.toPkcs12Asn1(i,n,t,s); … Tp.pkcs12KeyStore=…, Tp.privateKeyPkcs8=await window.crypto.subtle.exportKey("pkcs8",r.privateKey)` – der Keystore wird mit dem Nutzerpasswort `t` geschützt.
- Export als Datei [539550], [541578]: Dateiname `mjp_<initial>_<nachname>_<JJJJ-MM-TT>_<ms>.p12`; `exportKeystore()` speichert den Keystore lokal über den Dateidialog.
- Übertragung nur des Zertifikats: API `POST /safe/account/modify/certificate` (`modifyReaderCertificate`) [502300]; Oberflächentext „Wichtig: Die persönliche Schlüsseldatei wird nicht an den Server übermittelt.“ (i18n Z. 372) und „Nach einem Klick auf diese Schaltfläche wird Ihr öffentliches Zertifikat an den Postfachserver übermittelt.“ (i18n Z. 639).
- Zertifikatserneuerung: neuer Schlüssel, alter Schlüssel bleibt im neuen Keystore als „Vorheriger persönlicher Schlüssel“ (i18n `renewal.step3.certificate.key.friendly.name.previous`); Klasse mit `_oldPrivateKeys`, `_oldCerts` [46632].
- Zwischenspeicher im Browser [47940]: `persist(){const e=localStorage; … e.setItem("pkcs12KeyStore",…), … e.setItem("privateKeyPkcs8",…)}` – Methode zum Ablegen des privaten Schlüssels im `localStorage` ist vorhanden; ein Aufruf von `persist()`/`restore()` findet sich in der Datei nicht (Zeichenfolge `persist()` nur in der Definition), wirkt also ungenutzt [Folgerung].

## 3 Versand (Bürger:in → Justiz/Behörde)

- [784195 ff.] `prepareMessage()`: `getAccountInfoAndPublicKey(senderSafeId)` und `getAccountInfoAndPublicKey(recipient.safeId)` lesen über `POST /safe/account` das `readerCertificate` aus dem SAFE-Verzeichnis [772708].
- Anhänge werden vor dem Upload im Browser mit einem zufälligen symmetrischen Schlüssel verschlüsselt [774037]: `encryptAttachment(e,t){const r=await oC.aes_encrypt(e,t);…}` → `uploadAttachment({attachmentUuid, fileName: e.file.name, hashAlgorithm, hashValue, encryptedFileData})` (Dateiname und SHA-256 des Klartexts gehen unverschlüsselt an den Server).
- Der symmetrische Schlüssel wird je einmal für das Leserzertifikat der Empfänger:in (`readerEncryption`) und für das eigene Zertifikat (`senderEncryption`) als XML-Encryption verpackt (`createEncryptedDataXml`); die Empfängerdaten für die eigene Gesendet-Ansicht werden als JWE mit dem eigenen öffentlichen Schlüssel verschlüsselt (`createEncryptedMetadata(s.safeAccount,n)`).
- Die XJustiz-Nachricht (`xjustiz_nachricht.xml`, `nachricht.gds.uebermittlungSchriftgutobjekte` bzw. Basisnachricht) wird im Browser erzeugt und ebenfalls verschlüsselt hochgeladen (`createXJustizNachricht`) [771200 ff.]; Felder `eilig` → `sendungsprioritaet`, Empfängertyp `gericht`/`rvTraeger`/`polizeibehoerde`/`finanzbehoerde`/`sonstige`.
- API (Basis `https://api.mjp.justiz.de/api/v1/public`, config Z. 4–5): `POST /messages/ebo/outgoing/attachment` (multipart, Fehler `413: Maximum upload size is exceeded`), `POST /messages/ebo/outgoing/message`; Status `SENDING`/`SENT`/`FAILED` [493397].
- Prüfungen im Client [698289 ff.]: Gesamtgröße ≤ `maxUplSize` (config Z. 35–38: 209 715 200 Byte, „200 MB“), Anzahl ≤ 1000 (config Z. 42), Dateinamenregeln, keine Dateien `vhn.xml`, `vhn.xml.p7s`, `project_coco`, `vhn_coco` [699538 ff.]; `validateAtLeastOnePdf`.

## 4 Empfang (Justiz/Behörde → Bürger:in)

- Entschlüsselung im Browser: Parser für `xenc:EncryptedData` sucht den `xenc:EncryptedKey` zum eigenen Leserzertifikat (`ReaderCertificateId must be provided to find Encrypted Key.`) [616136 ff.]; RSA-OAEP-Entschlüsselung über `window.crypto.subtle.decrypt` [443927 davor].
- Oberfläche: „Um die Detailansicht von Nachrichten anzeigen zu können, müssen Sie zuerst Ihre persönliche Schlüsseldatei importieren.“ (i18n `message_detail.error.private_key_missing.text`).
- Prüfvermerk und Eingangsbestätigung werden im Client aus Serverdaten erzeugt [739962 ff.]: `vhnXml`, `vhnSignatureCheck` (`VALID`/`INVALID`/`UNKNOWN`, [493114]), `vhnSignatureCheckSubject`, `osciMessageId`, `osciCreationTime`; Zuordnung der VHN-Signatur-CA zu Übermittlungswegen `BEA`, `BEN`, `BEBPO`, `BEST`, `EBO`, `OZG` sowie `DE_MAIL_CONFIRMED`/`DE_MAIL_UNCONFIRMED` [742182 ff.], [743056].

## 5 Verzeichnis (SAFE)

- API `POST /safe/accounts` (Suche), `POST /safe/account` (Lookup per SAFE-ID), `POST /safe/mycertificate`, `POST /safe/account/hidden` (Auffindbarkeit ausschalten), `POST /safe/account/setAuthentication` / `unsetAuthentication` [502300], [504059 ff.].
- Rollenliste `xoev/egvp_roles.json`: `egvp_bebpo`, `egvp_ben`, `egvp_bea`, `egvp_best`, `egvp_justiz`, `buerger_rueck` (Label „Gerichtsvollzieher“).

## 6 Sonstiges

- Nachrichtenformular (i18n Z. 653–911): Empfängersuche im SAFE, Empfängertyp/Behörde, Sachgebiet, Aktenzeichen des Empfängers (Text, „neu“, „unbekannt“), Aktenzeichen des Absenders, „eilig“, „vertraulich“ mit Begründung, Anhänge; kein Feld für einen Nachrichtentext gefunden. Pflicht: „Es muss mindestens ein PDF-Dokument ausgewählt werden.“ (i18n Z. 230–231). Eine `xjustiz_nachricht.xml` aus einer Fachanwendung kann hochgeladen werden (i18n Z. 674–675).
- Benachrichtigung über das BundID-Postfach: Fehlertext `SET_ZBP_MAILBOX_HANDLE_FAILED` „Die Benachrichtigungsfunktion für Ihr Konto konnte nicht aktiviert werden.“ (i18n Z. 1686–1689); „Kurz nach dem Löschen können noch Benachrichtigungen an ihr BundID-Postfach erfolgen“ (i18n Z. 2255–2256).
- Namensgebung aus dem OZG-PLUS-Postfach: Standard-Sortierfeld der Nachrichtenlisten `ozgppCreationTime` [493397], CSS-Klassen `ozg-form`, `ozg-logo`, Elemente `ozg-filechooser`, `ozg-vertraulich`.
- Datenschutzhinweis im Client [557777], [559289]: „Alle Kommunikationsdaten sind verschlüsselt. Um verschlüsselte Daten anzeigen zu können, müssen Sie Ihre persönliche Schlüsseldatei sowie das dazugehörige Passwort verwenden. Erst dann können Nachrichteninformationen im Internetbrowser angezeigt werden.“ – „Zum Zweck der Wartung und Fehlerbehebung … werden für die Postfächer Backup- und technische Log-Dateien geführt, welche bis zu einem Zeitraum von 90 Tagen verschlüsselt aufbewahrt und dann automatisch gelöscht werden.“
- Löschen: „… alle Ihre Nachrichten, Ihr Konto und der Eintrag im SAFE-Verzeichnis gelöscht …“ (i18n Z. 2237–2238); vor dem Löschen müssen alle empfangenen Nachrichten gelesen sein (i18n Z. 2250).
- Zertifikatsablauf (i18n Z. 1994–1995): Nach Ablauf kein Nachrichtenaustausch mehr, Zugriff auf alte Nachrichten bleibt.
