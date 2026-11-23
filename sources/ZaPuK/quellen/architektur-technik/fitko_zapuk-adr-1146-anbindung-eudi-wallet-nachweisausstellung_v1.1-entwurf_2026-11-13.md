<!-- Kopie aus dem openCode-Repository ZaPuK (FITKO / FIT-AB, IT-Planungsrat) -->
<!-- Quelle: https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/ef99118c0607dd7fb11a005593569a36a0f84343/Zielarchitektur/Architekturentscheidungen/ADR-1146-Anbindung-EUDI-Wallet-Nachweisausstellung.md -->
<!-- Branch: architecture-v1.1, Commit ef99118c06 (2026-11-13); Pfad im Repo: Zielarchitektur/Architekturentscheidungen/ADR-1146-Anbindung-EUDI-Wallet-Nachweisausstellung.md -->
<!-- Abgerufen: 2026-11-19 ueber die GitLab-API (Blob 8c6968ad3a); Inhalt unveraendert ab Zeile 6 (Originalzeile = Zeile hier minus 5) -->

# ADR-1146: Anbindung der EUDI-Wallet, Nachweisausstellung in die Wallet

> **Status:** ✔️ Operativ entschieden | **Datum:** 13. November 2026

## Kontext und Problembeschreibung

In dieser Architekturentscheidung wird die Ausstellung von Nachweisen innerhalb der Zielarchitektur entworfen, wobei eine Lösung dem Prinzip der [Ende-zu-Ende-Verschlüsselung](../../Glossar.md#ende-zu-ende-verschlüsselung) (E2EE) folgen und keine Aussteller-Komponente in der Zielarchitektur einführen soll.

Das mit dem Beschluss 2026/19 des IT-Planungsrates vorgelegte Konzept zur Verwaltungsanbindung der [EUDI-Wallet](../../Glossar.md#europäische-brieftasche-für-die-digitale-identität) beschreibt Optionen, wie die Ausstellung von Nachweisen in die EUDI-Wallet implementiert werden kann.[^1] Zudem enthält das Konzept einen Prüfauftrag an [ZaPuK](../../Glossar.md#zapuk) und den Bund, ob eine elektronische Übermittlung von [Bescheiden](../../Glossar.md#bescheid) auch ohne vorherige Antragstellung mit BundID-Authentifizierung ermöglicht werden kann. Die Anbindung der Nachweisausstellung muss bei der Umsetzung des zentralen [Postfach-Zugangs](../../Glossar.md#postfach-zugang) berücksichtigt werden.

### Abgrenzung

- Eine elektronische Attributsbescheinigung (EAA) belegt bestimmte Eigenschaften einer [Identität](../../Glossar.md#identität). Die [Bekanntgabe](../../Glossar.md#bekanntgabe) bzw. Bereitstellung von Bescheiden erfolgt über die [Postfächer](../../Glossar.md#postfach) der Zielarchitektur.[^2]
- Die Wallet-Funktionalitäten [Identifizierung](../../Glossar.md#identifizierung) und Entgegennahme von Nachweisen werden hier nicht betrachtet.
- Die European Business Wallet wird hier nicht betrachtet.
- Der [Identitätsnachweis](../../Glossar.md#fachlicher-identitätsnachweis), den eine [Nachweisstelle](../../Glossar.md#nachweisstelle) je [Nachricht](../../Glossar.md#nachricht) ausstellt, wird hier nicht betrachtet ([ADR-1106](./ADR-1106-Fachlicher-Identitaetsnachweis-Matrix-Nachrichtenkontext.md) bis [ADR-1108](./ADR-1108-Nachweisstellen-Org-IdP-Adapter.md)).

## Entscheidungstreiber

- **Medienbruchfreie Übermittlung:** Von [öffentlichen Stellen](../../Glossar.md#öffentliche-stelle) ausgestellte Nachweise (QEAA) werden medienbruchfrei in eine EUDI-Wallet übermittelt (ANF_FUN_NAC_205).
- **Ende-zu-Ende-Verschlüsselung:** Die Verschlüsselung darf weder von Betreibern noch von Dritten gebrochen werden (SIC_VER_ÜBE_032).
- **Prüfauftrag:** Zu prüfen ist, ob Bescheide auch ohne vorherige Antragstellung mit BundID-Authentifizierung elektronisch übermittelt werden können (BMDS-Konzept, S. 21).[^2]

## Betrachtete Optionen

### Option A: Indirekte Anbindung über BundID/ZBP (später ZID)

Bei dieser Option übernimmt eine zentrale Stelle ([ZBP](../../Glossar.md#zentrales-bürgerpostfach), ZID) die Umwandlung der Formate und Siegelung der Nachweise mittels QTSP. Die [Fachverfahren](../../Glossar.md#fachverfahren) übermitteln die Daten schemakonform über eine spezielle Anbindung an den zentralen Dienst (ZBP bzw. ZID). Der zentrale Dienst erhält somit Zugriff auf den Klartext der Nachweise. Eine Ende-zu-Ende-Verschlüsselung der Nachweisdaten ist so nicht gegeben.

- Gut, denn die indirekte Anbindung der EUDI-Wallet für die Ausstellung von Nachweisen soll bis Januar 2027 produktiv sein.
- Gut, denn die Wallet-Protokolle und die Siegelung werden zentral entwickelt. Die Fachverfahren müssen keinen öffentlichen Aussteller-Endpunkt betreiben.
- Schlecht, denn die zentrale Stelle hat Zugriff auf den Klartext der Nachweisdaten.
- Schlecht, denn Bescheid und Nachweis werden getrennt übermittelt. Der Bescheid wird in das Postfach gesendet, der Nachweis dagegen über den ZID herausgegeben.

### Option B: Abholschein (direkte Anbindung, OID4VCI Pre-Authorized Code)

Die im Fachverfahren vorliegenden Nachweisdaten werden zur Abholung bereitgestellt. Der Aussteller erzeugt einen Abholschein in Form eines Links oder QR-Codes, der technisch als ein Credential Offer (OID4VCI Pre-Authorized Code) umgesetzt ist. Dieser Abholschein wird nun über das Postfach Ende-zu-Ende-verschlüsselt übermittelt. Im Postfach-Zugang wird er verwendet, um mittels der Wallet-App den Nachweis beim Aussteller abzuholen. Bei der Abholung über das Protokoll OID4VCI wird der Nachweis an die Wallet gebunden ([Holder-Binding](../../Glossar.md#holder-binding)). Dazu ist eine Re-Identifizierung des [Nutzenden](../../Glossar.md#nutzerinnen) durch den Aussteller notwendig (vgl. BMDS-Konzept, S. 25 f.). Dafür gleicht die den Wallet-Adapter betreibende [Behörde](../../Glossar.md#behörde) die PID der Wallet mit den Angaben zur Person ab. Die Behörde siegelt die Nachweise über einen QTSP, wobei nur ein Hashwert des Nachweises und nicht der Klartext an den QTSP übermittelt wird.

![Option B: Abholschein mit Ausstellung durch die Behörde](images/ADR-1146-option-b.svg)

- Gut, denn die Nachweisdaten bleiben entsprechend den E2EE-Anforderungen vertraulich.
- Gut, denn die Umsetzung entspricht dem Standardprotokoll (OID4VCI, ARF-konform) und der Anforderung an eine Re-Identifizierung beim Abholen des Nachweises.
- Gut, denn ein Bescheid und der Abholschein sind im selben Postfach-Kontext umsetzbar, was die User-Experience verbessert.
- Schlecht, denn Behörden müssen einen öffentlichen Issuer-Endpunkt für den Wallet-Adapter betreiben.

### Option B’: Abholschein mit Ausstellung über den ZID (indirekte Anbindung)

Der hier skizzierte Prozess entspricht grundsätzlich Option B, wobei Aussteller der Nachweise der zentrale Issuer-Dienst (ZID) und nicht die Behörde ist. Die Daten der Nachweise soll der ZID nicht als Klartext erhalten. Deshalb werden die im Fachverfahren vorliegenden Nachweisdaten durch die Behörde für die selektive Offenlegung (z. B. SD-JWT VC) aufbereitet. Dabei wird für jede Angabe zusammen mit einem Salt ein Hashwert gebildet. Die Behörde siegelt die Hashwerte und übergibt sie dem ZID. Die Angaben selbst werden mit ihren Salts und dem Abholschein durch die Behörde in das Postfach der Nutzenden Ende-zu-Ende-verschlüsselt versendet. Mit dem Abholschein können die Nutzenden den Nachweis dann über die Wallet-App beim ZID abholen (OID4VCI). Für die im BMDS-Konzept geforderte Re-Identifizierung bei der Ausstellung durch den ZID übergibt die Wallet dem ZID die PID (vgl. BMDS-Konzept, S. 25 f.). Der ZID gleicht sie mit Angaben zur Person ab, etwa Name und Geburtsdatum, die die Behörde mit den Hashwerten hinterlegt hat. Der ZID siegelt im Auftrag der Behörde mittels QTSP einen Nachweis, der nur die Hashwerte der Angaben enthält, und bindet ihn an die Wallet. Die Wallet kann den Nachweis dann um die Angaben im Klartext ergänzen und diese nachweisbar machen, da sich mit den Salts deren Hashwerte bilden lassen.

- Gut, denn E2EE für die Nachweisdaten bleibt prinzipiell erhalten, da der ZID außer den Angaben zur Person für die Re-Identifizierung nur gesiegelte Hashwerte erhält.
- Gut, denn Behörden bzw. Fachverfahren müssen keinen öffentlichen Endpunkt für einen EUDI-Wallet-Adapter anbieten.
- Gut, denn ein Bescheid und der Abholschein sind im selben Postfach-Kontext umsetzbar, was die User-Experience verbessert.
- Schlecht, denn OID4VCI sieht diesen Prozess so nicht vor. Im Gegensatz zur Option B werden die Angaben nicht vom Aussteller, sondern über das Postfach an die Wallet gegeben. Eine Erweiterung der EUDI-Wallet und ggf. der genutzten Protokolle wäre notwendig.
- Schlecht, denn durch die Re-Identifizierung bei der Ausstellung erfährt der ZID als zentrale Stelle, welche Person welchen Nachweis erhält.

### Option C: Anhang-Lösung (EAA als gesiegelter Anhang)

Die durch die Behörde gesiegelten Nachweise (EAA) werden direkt in das Postfach der Nutzenden versendet. Ein Abholschein wird nicht umgesetzt. Der Postfach-Zugang erlaubt das Speichern der Nachweise direkt in der Wallet. Soll der Nachweis für das Holder-Binding an einen Schlüssel der Wallet gebunden werden, müsste der dafür notwendige Schlüssel mit dem [Antrag](../../Glossar.md#antrag) übermittelt werden.

- Gut, denn die Vertraulichkeit der Nachweisdaten bleibt vollständig erhalten. Dem E2EE-Prinzip der Zielarchitektur wird entsprochen.
- Gut, denn ein öffentlicher Aussteller-Endpunkt wäre so nicht notwendig. Die Nachweise und Bescheide lägen zusammen im Postfach der Nutzenden.
- Schlecht, denn die Umsetzung von Holder-Binding würde einen Antrag mit der Wallet voraussetzen.
- Schlecht, denn der dateibasierte Import in die Wallet ist im ARF-Trust-Modell für die Ausstellung nicht vorgesehen. Der Aussteller muss vor jeder Ausstellung die Wallet prüfen.[^3]
- Schlecht, denn die Re-Identifizierung durch den Aussteller ist so nicht umsetzbar, wird aber im aktuellen BMDS-Konzept gefordert.

## Architekturentscheidung

Gewählte Option: **B, Abholschein (direkte Anbindung, OID4VCI Pre-Authorized Code)**, weil durch diese Lösung die Vertraulichkeit der Nachweisdaten sichergestellt ist und dem E2EE-Paradigma der Zielarchitektur entsprochen wird (SIC_VER_ÜBE_032), diese Lösung mit den Anforderungen bzgl. der Re-Identifizierung beim Abruf vereinbar ist und sie eine gute Benutzbarkeit durch die gemeinsame Auslieferung von Abholschein und Bescheid ermöglicht. Der Abholschein ist technisch gesehen ein Verweis auf die öffentlichen Endpunkte der Aussteller. Im Zielzustand wären dies die Wallet-Adapter der Fachverfahren. Benachrichtigungen und Abholscheine werden über die Postfächer der Zielarchitektur an die Nutzenden übermittelt.

![Ablauf der Nachweisausstellung bei Option B](images/ADR-1146-ablauf-option-b.svg)

**Option B’ (Abholschein mit Ausstellung über den ZID) wird als Variante von Option B weiter untersucht**, weil Fachverfahren so keinen Aussteller-Endpunkt betreiben müssen und der ZID die Nachweisdaten nicht im Klartext erhalten muss. Diese Lösung wäre vor allem für eine Transition wichtig, wenn Fachverfahren noch keine Wallet-Adapter betreiben. Die Prüfung hinsichtlich einer Konformität zu QEAA und der Kompatibilität zu OID4VCI ist vor einer Umsetzung notwendig.

**Option A (indirekte Anbindung über BundID/ZBP) wird nicht gewählt**, da der zentrale Dienst den Klartext der Nachweisdaten erhält, was dem E2EE-Prinzip widerspricht. Die indirekte Ausstellung über das ZBP wird im Transitionsplan berücksichtigt. **Option C (Anhang-Lösung) wird nicht gewählt**, da der direkte Versand der Nachweise über die Postfächer derzeit nicht standardkonform ist. Zwar bietet sie den Vorteil, dass die Fachverfahren keine öffentlichen Endpunkte für die Wallet bereitstellen müssen. Doch der dateibasierte Import ist im ARF-Trust-Modell für die Ausstellung nicht vorgesehen und die im BMDS-Konzept geforderte Re-Identifizierung durch den Aussteller würde entfallen.

Zur Ausstellung von Nachweisen aus Registern (NOOTS) wird hier keine Festlegung getroffen. Hierfür ist bereits ein eigener Issuer-Dienst für NOOTS durch den Bund geplant (vgl. BMDS-Konzept, S. 30 f.).

## Auswirkungen

- Postfach-Zugänge: Darstellung des Abholscheins, Unterstützung von haip-vci://-Deep-Links.
- Prüfauftrag (BMDS-Konzept S. 21): Die Postfachadresse steht im Key-Binding-JWT zum Identitätsnachweis ([ADR-1107](./ADR-1107-Nachweisformat-Identitaetsnachweis-Matrix-Nachrichten.md)). Je nach Umsetzung der Zuordnung erreicht die Person ihr Postfach auch mit [eID](../../Glossar.md#eid-funktion) oder EUDI-Wallet ohne Anmeldung über die [BundID](../../Glossar.md#bundid) ([ADR-1105](./ADR-1105-Pseudonym-Adressierung-von-Privatpersonen.md)).
- Transition ZBP: Der Parallelbetrieb von Legacy-APIs und Wallet-Anbindung sowie die indirekte Ausstellung über das ZBP werden in den Transitionsplan aufgenommen.
- Umsetzung initial nur mit Same-Device und [fortgeschrittenem elektronischem Siegel](../../Glossar.md#fortgeschrittenes-elektronisches-siegel), später mit Cross-Device (QR-Code) und [qualifiziertem elektronischem Siegel](../../Glossar.md#qualifiziertes-elektronisches-siegel) bzw. QEAA.

## Risiken

- EUDI-Wallet in laufender Entwicklung, Zeitplan Wallet-Adapter und ZID (Bund, in Konzeption). Option B hängt nicht vom ZID ab.
- Der Standard zur Wallet ist noch in Arbeit. Vor der Implementierung muss der Entwurf mit dem aktuellen Stand abgeglichen werden.

## Offene Punkte

- Option B’ muss weiter geprüft werden. Im Besonderen sind die Konformität zu QEAA und die Kompatibilität zu OID4VCI zu prüfen.

## Referenzen

[^1]: [IT-Planungsrat, Beschluss B-2026/19-IT, Verwaltungsanbindung EUDI-Wallet](https://www.it-planungsrat.de/beschluss/b-2026-19-it), abgerufen am 11. August 2026.

[^2]: [Verwaltungsanbindung EUDI-Wallet, Konzept zum Zielbild (BMDS), Stand 17.06.2026](https://www.it-planungsrat.de/fileadmin/beschluesse/2026/Beschluss_2026_19_Verwaltungsanbindung_EUDI-Wallet_Konzept_zum_Zielbild.pdf), abgerufen am 11. August 2026.

[^3]: [EU-Kommission, Architecture and Reference Framework (ARF), v3.0.0, §6.6.2 Trust-Modell Ausstellung](https://eudi.dev/latest/main/06-trust-model/), abgerufen am 14. Oktober 2026.
