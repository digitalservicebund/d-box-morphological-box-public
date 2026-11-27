# ADR-1122: Benachrichtigungsdienst für Bereitstellungen im Postfach

> **Status:** 🛠️ In Arbeit | **Datum:** 23. November 2026

## Kontext und Problembeschreibung

Bei der Bereitstellung von [Nachrichten](../../Glossar.md#nachricht) im [Postfach](../../Glossar.md#postfach) soll die [Empfänger:in](../../Glossar.md#empfängerin) mit einer [Eingangsbenachrichtigung](../../Glossar.md#eingangsbenachrichtigung) auf einem anderen Kanal, z. B. per E-Mail (SMTP) oder Push (APNs/FCM), informiert werden. Je nach Gesetz löst die Benachrichtigung eine Frist aus oder weist nur auf die Bereitstellung hin (vgl. [ADR-1121](./ADR-1121-Ausgestaltung-der-Abrufbestätigung.md)).

- **Benachrichtigung:** Nach § 37 Abs. 2a Satz 4 SGB X[^1] gilt ein zum Abruf bereitgestellter [Verwaltungsakt](../../Glossar.md#verwaltungsakt) am vierten Tag nach Absendung der elektronischen Benachrichtigung über die Bereitstellung des Verwaltungsaktes an die abrufberechtigte Person als bekannt gegeben. Im Zweifel hat die [Behörde](../../Glossar.md#behörde) den [Zugang](../../Glossar.md#zugang) der Benachrichtigung nachzuweisen. Kann die Behörde den von der abrufberechtigten Person bestrittenen Zugang der Benachrichtigung nicht nachweisen, gilt der Verwaltungsakt an dem Tag als bekannt gegeben, an dem die abrufberechtigte Person den Verwaltungsakt abgerufen hat.

- **Bereitstellung:** Nach § 9 Abs. 1 Satz 4 OZG[^2] und § 122a Abs. 4 Satz 1 AO[^3] gilt der Verwaltungsakt am vierten Tag nach der Bereitstellung zum Abruf als bekannt gegeben. Nachweis ist die [Bereitstellungsbestätigung](../../Glossar.md#bereitstellungsbestätigung) ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). Nach § 9 Abs. 1 Satz 6 OZG wird spätestens am Tag der Bereitstellung zum Abruf über die zu diesem Zweck angegebene Adresse über die Möglichkeit des Abrufs benachrichtigt. Nach § 122a Abs. 1 Satz 3 AO ist die abrufberechtigte Person am Tag der Bereitstellung elektronisch über die Abrufmöglichkeit und ihre Rechtswirkungen zu benachrichtigen. Die Benachrichtigung ist danach vorgeschrieben, löst aber keine Frist aus. § 9 Abs. 1 OZG gilt nur für das Postfach eines OZG-Nutzerkontos und wäre anzupassen, damit die [Bekanntgabefiktion](../../Glossar.md#bekanntgabefiktion) auch für vom [Nutzerkonto](../../Glossar.md#nutzerkonto) entkoppelte Postfächer an die Bereitstellung anknüpft ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

Der Zugang einer Benachrichtigung per E-Mail oder Push lässt sich allerdings nicht verlässlich nachweisen.

In dieser Architekturentscheidung wird festgelegt, welche Komponente die Benachrichtigung umsetzt, wo die Kontaktdaten liegen und welche Nachweise entstehen.

## Entscheidungstreiber

- **Bekanntgabe:** Das System erfüllt die gesetzlich definierten Anforderungen zur rechtssicheren Realisierung einer Bekanntgabefiktion bei der elektronischen Übermittlung von Verwaltungsakten (ANF_FUN_ZUS_024).
- **Optionale Nachweise:** Das System unterstützt auch Kommunikation ohne Anforderungen hinsichtlich der Beweisbarkeit der erfolgten Übermittlung (ANF_FUN_ZUS_044).
- **Erreichbarkeit:** Die Empfänger:in kann sich über dritte Wege wie E-Mail oder App-basierte Push-Notification informieren lassen, wenn eine neue Nachricht in ihrem Postfach eingegangen ist (ANF_FUN_ZUS_026).
- **Metadatenschutz:** Sensible Informationen wie Namen und Inhalte einer Nachricht werden nicht in für Dritte zugänglicher Form über Push-Server übertragen (SIC_ÜBE_ÜBE_017). Metadaten, die nicht zwingend für die Übermittlung erforderlich sind, werden vermieden, verschlüsselt oder verschleiert (SIC_VER_ÜBE_051).
- **Nichtverkettbarkeit:** Kein Betreiber allein soll die Adressen einer [Privatperson](../../Glossar.md#privatperson) in verschiedenen [Bereichen](../../Glossar.md#bereich) zusammenführen und mit ihren Kommunikationsbeziehungen verknüpfen können ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md)). Das gilt auch für Kontaktdaten.

## Betrachtete Optionen

### Option A: Benachrichtigung durch das Empfänger-Backend

Der [Benachrichtigungsdienst](../../Glossar.md#benachrichtigungsdienst) im [Postfach-Backend](../../Glossar.md#postfach-backend) der Empfänger:in (Empfänger-Backend) sendet die Benachrichtigungen per Push oder E-Mail entsprechend ihren Einstellungen. Die Kontaktdaten führt es je [Bereichs-MXID](../../Glossar.md#bereichs-mxid), da eine Privatperson je Bereich eine eigene Adresse (MXID) hat ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md)). Auf Anforderung der absendenden Behörde bestätigt der [Quittungsdienst](../../Glossar.md#quittungsdienst) das Versenden mit einer [Benachrichtigungsbestätigung](../../Glossar.md#benachrichtigungsbestätigung). Diese Bestätigung ist analog zur Bereitstellungsbestätigung umgesetzt (vgl. [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)).

- Gut, denn eine Umsetzung von Benachrichtigung und Nachweis ist zentral und nicht in jedem [Fachverfahren](../../Glossar.md#fachverfahren) einzeln notwendig.
- Gut, denn die Benachrichtigung ist durch die Empfänger:in je Kanal und [Vorgang](../../Glossar.md#vorgang) einstellbar.
- Gut, denn Push, E-Mail und diese Einstellungen sind im Matrix-Protokoll spezifiziert.[^4]
- Gut, denn die Benachrichtigung wird vom Postfach-Backend versendet, sodass die Behörde für den E-Mail-Anbieter nicht erkennbar ist.
- Schlecht, denn dieselben Kontaktdaten (E-Mail-Adresse, Push-Registrierung) bei mehreren Bereichs-MXIDs erlauben dem Betreiber des Postfach-Backends, die MXIDs der Person zusammenzuführen ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md)).

### Option B: Benachrichtigung durch das Fachverfahren

Das Fachverfahren benachrichtigt eigenständig, z. B. über die im [Antrag](../../Glossar.md#antrag) angegebene E-Mail-Adresse.

- Gut, denn im Postfach-Backend liegen keine Kontaktdaten vor.
- Schlecht, denn eine Umsetzung von Benachrichtigung und Nachweis ist in jedem Fachverfahren einzeln notwendig.
- Schlecht, denn die Benachrichtigung ist durch die Empfänger:in nicht im Postfach einstellbar.
- Schlecht, denn die absendende Behörde ist für den E-Mail-Anbieter erkennbar.

### Option C: Benachrichtigung durch das Nutzerkonto

Das Nutzerkonto benachrichtigt über die dort hinterlegte E-Mail-Adresse.

- Gut, denn die E-Mail-Adresse ist nur einmal hinterlegt und im Postfach-Backend liegen keine Kontaktdaten vor.
- Schlecht, denn der Betreiber des Nutzerkontos erfährt von jeder Nachricht aus jedem Bereich. Dies widerspricht [ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md) und der Entkopplung von Nutzerkonto und Postfach ([ADR-1110](./ADR-1110-Entkopplung-von-Nutzerkonto-und-Postfach.md)).
- Schlecht, denn ohne Nutzerkonto, z. B. bei direkter Anmeldung mit eID oder EUDI-Wallet, ist keine Benachrichtigung möglich.

## Architekturentscheidung

Gewählte Option: **Option A (Benachrichtigung durch das Empfänger-Backend)**, weil Benachrichtigung und Nachweis zentral und nicht in jedem Fachverfahren einzeln umgesetzt werden, die Empfänger:in die Benachrichtigung selbst einstellen kann und die Behörde für den E-Mail-Anbieter nicht erkennbar ist. Eine Behörde kann zusätzlich selbst benachrichtigen (Option B). **Option C wird ausgeschlossen**, da sie der Entkopplung von Nutzerkonto und Postfach widerspricht ([ADR-1110](./ADR-1110-Entkopplung-von-Nutzerkonto-und-Postfach.md)).

- Die Empfänger:in legt die Benachrichtigungskanäle fest und kann die Benachrichtigung je Vorgang abschalten. Benachrichtigt wird nur über Kanäle, die sie aktiviert hat. Der Text ist für alle Nachrichten gleich. Text und Push enthalten keine Angaben zu Bereich, Behörde oder Vorgang.

- Die absendende Behörde fordert die Benachrichtigungsbestätigung pro Nachricht an. Der Quittungsdienst bestätigt signiert und unverschlüsselt im Raum, dass und wann die Benachrichtigung abgesendet wurde ([ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md)). Meldet er ebenso und ohne Angabe des Grundes, dass nicht gesendet wurde, gibt die Behörde den Verwaltungsakt auf andere Weise bekannt.

- Die Benachrichtigungsbestätigung belegt die Absendung und nicht den Zugang der Benachrichtigung. Wird der Zugang bestritten, gilt nach § 37 Abs. 2a SGB X der Tag des Abrufs. Für diese Verfahren fordert die Behörde deshalb zusätzlich die [Abrufbestätigung](../../Glossar.md#abrufbestätigung) an ([ADR-1118](./ADR-1118-Abruf-und-Lesebestätigungen.md), [ADR-1121](./ADR-1121-Ausgestaltung-der-Abrufbestätigung.md)).

## Auswirkungen

- Die [Übermittlungsoptionen](../../Glossar.md#übermittlungsoption) nach [ADR-1119](./ADR-1119-Absendergesteuerte-Übermittlungsoptionen-mit-Berechtigungsprüfung.md) werden um „Benachrichtigungsbestätigung anfordern“ ergänzt.
- Der Quittungsdienst nach [ADR-0018](./ADR-0018-Nichtabstreitbarkeit-Übermittlung.md) erzeugt zusätzlich die Benachrichtigungsbestätigung.
- [Postfach-Zugänge](../../Glossar.md#postfach-zugang) bieten die Einstellungen zur Benachrichtigung an und weisen auf die Folgen des Abschaltens hin.

## Risiken

- Enthält die E-Mail-Adresse den Namen, ist die Person für den Betreiber des Postfach-Backends ggf. identifizierbar.
- Apple und Google sehen, dass und wann ein Gerät einen Push erhält, und können daraus auf die Nutzung des Postfachs schließen.

## Folge-Entscheidungen

- **[Anschlussbedingungen](../../Glossar.md#anschlussbedingungen):** Postfach-Backends für Privatpersonen müssen Benachrichtigung und Benachrichtigungsbestätigung anbieten, siehe [ADR-1145](./ADR-1145-Anschlussbedingungen-und-Foederationskontrolle.md).

## Offene Punkte

- **Rechtliche Einordnung:** Genügt ein Push als elektronische Benachrichtigung? Deckt ein für alle Nachrichten gleicher Text die Hinweise nach § 122a Abs. 1 Satz 3 AO und § 37 Abs. 2a Satz 4 SGB X ab?
- **Nachweis des Zugangs:** Apple und Google melden beim Versand eines Push nur die Annahme und nicht die Ankunft auf dem Gerät. Zu prüfen ist, ob sich der Zugang der Benachrichtigung auf anderem Weg nachweisen lässt.
- **Verkettung:** Die Verkettung der Bereichs-MXIDs über dieselbe E-Mail-Adresse oder dasselbe Gerät muss verhindert werden. Ggf. können die Kontaktdaten bei einer vom Postfach-Backend getrennten Stelle geführt werden.

## Referenzen

[^1]: [§ 37 SGB X Bekanntgabe des Verwaltungsaktes](https://www.gesetze-im-internet.de/sgb_10/__37.html), abgerufen am 23. November 2026.

[^2]: [§ 9 OZG Bekanntgabe des Verwaltungsaktes](https://www.gesetze-im-internet.de/ozg/__9.html), abgerufen am 23. November 2026.

[^3]: [§ 122a AO Bekanntgabe von Verwaltungsakten durch Bereitstellung zum Datenabruf](https://www.gesetze-im-internet.de/ao_1977/__122a.html), abgerufen am 23. November 2026.

[^4]: [Matrix Spec, Push Notifications](https://spec.matrix.org/latest/client-server-api/#push-notifications), abgerufen am 23. November 2026.
