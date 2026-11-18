# Mein Justizpostfach – Auszüge aus dem Webclient-Code (app.js)

Quelle: https://mjp.justiz.de/app.js?v=2.7.5 (minifiziert, ca. 930 KB; nicht ausgeführt, nur per Textsuche ausgewertet)
Abgerufen: 2026-11-18

Wörtliche Auszüge (je Fundstelle ±100–250 Zeichen), keine vollständige Kopie.

## Urheberhinweis

```text
 * Copyright (C) 2022 Governikus GmbH & Co KG. All rights reserved.
```

## Krypto-Parameter (Konstante Rt / Lt)

```text
const Rt={symmetricEncryption:{"aes128-gcm":{name:"AES-GCM",length:128},"aes192-gcm":{name:"AES-GCM",length:192},"aes256-gcm":{name:"AES-GCM",length:256},"aes128-cbc":{name:"AES-CBC",length:128},"aes192-cbc":{name:"AES-CBC",length:192},"aes256-cbc":{name:"AES-CBC",length:256}},asymmetricEncryption:{"rsa-oaep":{name:"RSA-OAEP",modulusLength:3072,publicExponent:new Uint8Array([1,0,1]),hash:"SHA-256"}},hashing:{"sha-256":"SHA-256","sha-384":"SHA-384","sha-512":"SHA-512"},signatures:{"rsassa-pkcs1-v1_5":{name:"RSASSA-PKCS1-v1_5",modulusLength:3072,publicExponent:new Uint8Array([1,0,1]),hash:"SHA-256"},"rsa-pss":{name:"RSA-PSS",modulusLength:3072,publicExponent:new Uint8Array([1,0,1]),hash:"SHA-256",saltLength:32}}},Dt=Rt.sy
```

## Schlüsselspeicher im Browser (Klasse mit persist/restore)

```text
set pkcs12KeyStore(e){this._pkcs12KeyStore=e}get pkcs12KeyStore(){return this._pkcs12KeyStore}set privateKeyPkcs8(e){this._privateKeyPkcs8=e,this.restartTimer()}get privateKeyPkcs8(){return this._privateKeyPkcs8}persist(){const e=localStorage;this._x509v3CertPem&&e.setItem("x509v3CertPem",this._x509v3CertPem),this._pkcs12KeyStore&&e.setItem("pkcs12KeyStore",h(this._pkcs12KeyStore)),this._privateKeyPkcs8&&e.setItem("privateKeyPkcs8",h(this._privateKeyPkcs8))}restore(){const e=localStorage;e.getItem("x509v3CertPem")&&(this._x509v3CertPem=e.getItem("x509v3CertPem")),e.getItem("pkcs12KeyStore")&&(this._pkcs12KeyStore=d(e.getItem("pkcs12KeyStore"))),e.getItem("privateKeyPkcs8")&&(this._privateKeyPkcs8=d(e.getItem(
```

Hinweis: Eine Textsuche nach Aufrufen `.persist()` / `.restore()` ergab keine Treffer außerhalb der Definition.

## API-Pfade (Basis https://api.mjp.justiz.de/api/v1/public)

```text
"/mailbox-management/mailbox"
"/messages/ebo/incoming"
"/messages/ebo/outgoing"
"/messages/ebo/outgoing/attachment"
"/messages/ebo/outgoing/message"
"/safe/account"
"/safe/account/hidden"
"/safe/account/modify/certificate"
"/safe/accounts"
"/safe/account/setAuthentication"
"/safe/account/unsetAuthentication"
"/safe/mycertificate"
"/user/baseInfo"
"/user/extendedInfo"
"/user/readerCertificateExpiryWarningShown"
"/user/safeAccount"
```

## SAFE-Authentifizierung (Akteneinsichtsportal-Zugangsdaten)

```text
static setSafeAuthentication(e){return Kg({method:"POST",path:"/safe/account/setAuthentication",body:e,mediaType:"application/json",errors:{400:"Password or username is invalid.",500:"An unexpected error occurred."}})}static unsetSafeAuthentication(){return Kg({method:"POST",path:"/safe/account/unsetAuthentication",errors:{500:"Authentication could not be randomized."}})}static toggleSafeAccountHidden(e){return Kg({method:"POST",path:"/safe/acc
```

## Abruf eingehender Nachrichten, Sortierfeld

```text
static listIncomingMessagesEbo(e,t,r=!1,i="ozgppCreationTime"){return xg({method:"GET",path:"/messages/ebo/incoming",query:{page:e,itemsPerPage:t,ascending:r,sortBy:i},errors:{400:"Request is invalid.",401:"User is not authenticat
```

## Registrierung: Schlüsselerzeugung im Browser und lokaler Export der .p12-Datei

```text
async validateAndSubmitForm(e){this.preventDefaultBehaviour(e),this.resetErrorMessage(),this.setBusy();try{await this.loadExtendedUserInfo();try{await this.generateKeyStore(),await this.exportKeystore();const e=new Event("submit");this.dispatchEvent(e)}catch(e){this.onKeystoreError(e)}}catch(e){this.onLoadExtendedUserInfoError(e)}finally{this.unsetBusy()}}async generateKeyStore(){if(null==this.pkcs12KeyStore){const e=ly.getUser().getX509Attributes(),t=this.registrationEl.getPasswordForm();await cy.generateUserPKCS12KeyStore(e,t.getValues().password),this.pkcs12KeyStore=cy.getUserPKCS12KeyStore()}return Promise.resolve()}async exportKeystore(){const e=ly.getUser(),t=oy(e.firstName,e.lastName),r={suggestedName:t,types:[{description:he.gettext(uy),accept:{}}]};var i,n,s;r.types[0].accept[Mm.pkcs12]=[".p12"],await dy.export((i=this.pkcs12KeyStore,n=Mm.pkcs12,s=t,new
```
