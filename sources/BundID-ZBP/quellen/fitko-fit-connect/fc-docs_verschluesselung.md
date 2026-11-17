# Verschlüsselte Übertragung von Antragsdaten

- Quelle: <https://docs.fitko.de/fit-connect/docs/getting-started/encryption>
- Abgerufen: 2026-11-17
- Herausgeber: FITKO (Föderale IT-Kooperation), FIT-Connect-Dokumentation (docs.fitko.de), Lizenz CC BY 4.0
- Konvertierung: lynx -dump (Text, Navigation entfernt)

```text
Verschlüsselte Übertragung von Antragsdaten

   Ziel von FIT-Connect ist es, ein möglichst einfaches, sicheres und klar definiertes Vorgehen zur Einreichung von Antragsdaten zu etablieren. Deshalb
   erfolgt eine Übertragung von Antragsdaten mit FIT-Connect ausschließlich verschlüsselt.

Warum ist Ende-zu-Ende-Verschlüsselung wichtig?

   Im Kontext von Anträgen an Behörden werden häufig personenbezogene Daten übermittelt. FIT-Connect bietet zur Absicherung der Übertragung von
   Antragsdaten und Antragsmetadaten eine auf asymmetrischer Kryptographie basierende Inhaltsdatenverschlüsselung. Abgesehen von denen für die
   Übermittlung zwingend notwendigen technischen Daten können so nur die Zielbehörde bzw. das Ziel-Verwaltungssystem die Antragsdaten und Antragsmetadaten
   entschlüsseln und lesen.

   Die FIT-Connect Architektur unterstützt grundsätzlich zwei Umsetzungsvarianten für die verschlüsselte Übermittlung von Antragsdaten. Abhängig vom
   Schutzbedarf der zu übermittelnden Daten sollte entschieden werden, welche Umsetzungsvariante einzusetzen ist.

   Besonders schutzbedürftige personenbezogene Daten dürfen nach den Vorgaben des BSI nur Ende-zu-Ende-verschlüsselt übertragen werden. Für diese Daten
   ist es notwendig, dass die Antragsdaten bereits auf dem Endgerät der Anwender:innen verschlüsselt werden (Ende-zu-Ende-Verschlüsselung).

   Für weniger kritische Daten kann alternativ auch eine Verschlüsselung der Daten im Backend des Onlinedienstes erfolgen. Diese Umsetzungsvariante bietet
   eine geringere Sicherheit als die Umsetzung der Ende-zu-Ende-Verschlüsselung ab dem Endgerät der Antragsteller:in. Zudem erhöhen sich in dieser
   Umsetzungsvariante die IT-Sicherheits- und Datenschutzanforderungen an den Betrieb des Onlinedienstes, da Antragsdaten in dieser Umsetzungsvariante im
   Klartext durch das Backend des Onlinedienstes verarbeitet werden.
   Grafik zur Veranschaulichung einer Ende-zu-Ende-Verschlüsselung

   Zur Verschlüsselung von Antragsdaten im Endgerät der Antragsteller:in kann das FIT-Connect-JavaScript-SDK genutzt werden. Das SDK garantiert dabei die
   korrekte Implementierung der erforderlichen kryptographischen Operationen im Browser der Antragsteller:in. So verlassen sämtliche Inhaltsdaten das
   Endgerät der Nutzer:in immer verschlüsselt. Beispiele, wie Anträge im Rahmen von FIT-Connect verschlüsselt werden können, finden sich unter
   Verschlüsselung.

Exkurs: Grundlagen zur eingesetzten Verschlüsselung

   Die Inhaltsdatenverschlüsselung von FIT-Connect wird mithilfe von hybrider Verschlüsselung auf Basis von JSON Web Encryption (JWE) umgesetzt. Dieser
   Ansatz kombiniert die Vorteile von symmetrischer und asymmetrischer Verschlüsselung. Im Folgenden sollen die Grundlagen kurz erläutert werden.

Symmetrische Verschlüsselung

   Eine Verschlüsselung wird als symmetrisch bezeichnet, wenn für die Ver- und Entschlüsselung derselbe Schlüssel verwendet wird. Das ist gleichzeitig
   auch der größte Nachteil der symmetrischen Verschlüsselung. Für eine sichere Kommunikation muss immer zuerst allen beteiligten Parteien der geheime
   Verschlüsselungsschlüssel übermittelt werden. Man spricht hier auch von einem gemeinsamen Geheimnis (shared secret). Kommt dieses abhanden oder wird
   beim Austausch abgefangen, ist die Sicherheit der Kommunikation nicht mehr gewährleistet. Jeder, der in Besitz dieses Schlüssels gelangt, ist im
   Stande, alle mit ihm verschlüsselten Übermittlung zu entschlüsseln und damit lesen zu können. Dem gegenüber steht der große Vorteil, dass es mit
   symmetrischer Verschlüsselung schnell und effizient möglich ist, auch große Datenmengen zu verschlüsseln.

Asymmetrische Verschlüsselung

   Eine Verschlüsselung wird als asymmetrisch bezeichnet, wenn es für Ver- und Entschlüsselung im Gegensatz zur symmetrischer Verschlüsselung jeweils
   einen eigenen Schlüssel gibt. Die beiden Schlüssel bilden zusammen ein Schlüsselpaar, bestehend aus einem öffentlichen Schlüssel (public key) und einem
   geheimzuhaltenden privaten Schlüssel (private key, auch secret key). Der große Vorteil dieses Verfahrens besteht darin, dass für eine sichere
   Kommunikation den sendenden Parteien nur der öffentliche Schlüssel ausgehändigt werden muss. Lediglich die zum öffentlichen Schlüssel passenden
   privaten Schlüssel, mit denen empfangende Übermittlung wieder entschlüsselt werden können, müssen die Übermittlung-Empfänger geheim halten. Der
   öffentliche Schlüssel baut bildlich gesprochen einen Schutzcontainer um die Übermittlung auf, die nur der zugehörige Empfänger öffnen kann, wie z. B.
   bei einem Briefkasten, der auch nur vom Besitzer des passenden Briefkastenschlüssels geöffnet werden kann.
   Beziehung öffentlicher und privater Schlüssel

   Im Vergleich zur symmetrischen Verschlüsselung sind die Schlüssel hier jedoch um ein vielfaches länger und die Ver- und Entschlüsselung von
   Übermittlungen erfordert mehr Rechenleistung. Aus diesem Grund werden die beiden Verfahren bei der Ende-zu-Ende-Verschlüsselung von FIT-Connect
   kombiniert (sog. hybride Verschlüsselung).

Hybride Verschlüsselung

   Kombiniert man die Vorteile von symmetrischer und asymmetrischer Verschlüsselung in einem gemeinsamen Verfahren, spricht man von einem hybriden
   Verschlüsselungsverfahren. Mithilfe von asymmetrischer Verschlüsselung, d. h. dem öffentlichen Schlüssel des Empfängers, verschlüsselt man dabei den
   symmetrischen Verschlüsselungsschlüssel und kann diesen verschlüsselten Schlüssel dann auch auf einem ungesicherten Kanal übertragen. Der symmetrische
   Verschlüsselungsschlüssel kann jetzt in Folge für die eigentliche Inhaltsverschlüsselung genutzt werden, welche dank symmetrischer Verfahren schnell
   und effizient verschlüsseln kann.

Sichere Implementierung der Ende-zu-Ende-Verschlüsselung mit FIT-Connect

   Die folgenden Abschnitte geben wichtige grundlegende Hinweise zur sicheren Implementierung der Ende-zu-Ende-Verschlüsselung mit FIT-Connect. Konkrete
   Implementierungsbeispiele finden sich darüber hinaus in den Artikeln zur Verschlüsselung und Entschlüsselung von Antragsdaten. Vorgaben zu zulässigen
   kryptographischen Algorithmen und Schlüssellängen finden sich im Artikel Vorgaben für kryptographische Verfahren.

Herkunft und Zugehörigkeit von Schlüsseln prüfen

   Wird einem Sender der öffentliche Schlüssel vom designierten Empfänger der Übermittlung im Vorlauf nicht sicher und idealerweise sogar persönlich
   übermittelt, muss auf anderem Weg der oder die Eigentümerin eines Schlüssels ermittelt werden. Nur so kann man sicher gehen, auch den richtigen
   Schlüssel zu verwenden.

   Für genau diesen Fall wurden digitale Zertifikate entwickelt. Digitale Zertifikate sind nichts weiter als öffentliche Schlüssel, angereichert mit
   beglaubigten Informationen zu seiner Herkunft und dem Eigentümer. Diese Beglaubigung von Schlüssel und Metainformationen erfolgt durch eine
   vertrauenswürdige dritte Person oder Institution - üblicherweise einer so genannten Zertifizierungstelle (CA - Certificate Authority) - mithilfe seiner
   digitalen, für Dritte prüfbaren Unterschrift.

   Ein so ausgestelltes Zertifikat kann dann vom Empfänger wie eine Visitenkarte vor Beginn der verschlüsselten Kommunikation verteilt werden, da es nur
   öffentliche und auch prüfbare Informationen enthält.

   Die für die Verschlüsselung verwendeten öffentlichen Schlüssel lassen sich so vor Verwendung anhand des zugehörigen Zertifikats aus der Verwaltungs-PKI
   auf Gültigkeit überprüfen. Im Artikel Überprüfen öffentlicher Schlüssel werden die zur Prüfung notwendigen Schritte näher erläutert. Behörden, die zum
   Empfang von Anträgen berechtigt sind, können sich, wie im Artikel Zertifikatsbeantragung beschrieben, Zertifikate aus der Verwaltungs-PKI ausstellen
   lassen. Für sendende Systeme ist derzeit keine Zertifikatsbeantragung notwendig.

   Nicht vertrauenswürdige Schlüssel werden abgelehnt und dürfen nicht für die Verschlüsselung von Anträgen verwendet werden. Die Prüfung von Zertifikaten
   ist zwingend erforderlich, da sonst Angriffe wie Man-in-the-middle-Attacken möglich sind.
   Grafik zur Veranschaulichung der Einbindung einer PKI zur Verhinderung von Man-in-the-Middle-Angriffen

JSON Web Encryption und JWE Compact Serialization

   Die Grundlagen zu Verschlüsselung, hybriden Verfahren und Zertifikaten finden in FIT-Connect in Form von JSON Web Encryption (JWE) bei der
   Verschlüsselung von Antragsdaten (Submissions) Anwendung. JWE standardisiert die einheitliche Anwendung von kryptographischen Algorithmen und die
   Repräsentation der dabei genutzten Datenstrukturen. Dazu nutzt FIT-Connect die JWE Compact Serialization. Die Daten innerhalb eines JWE werden
   möglichst sparsam kodiert und als eine kompakte, URL-sichere Zeichenkette (String) dargestellt. Der String besteht dabei aus fünf konkatenierten, durch
   Punkte (.) voneinander getrennten, BASE64URL kodierten Elementen:
     * JOSE Header - beschreibt mit welchen asymmetrischen Verfahren der symmetrische Schlüssel zur Inhaltsverschlüsselung verschlüsselt wurde und mit
       welchem symmetrischen Verfahren die Inhaltsdaten (Ciphertext) verschlüsselt wurden
     * JWE Encrypted Key - asymmetrisch verschlüsselter, symmetrischer Schlüssel zur Inhaltsverschlüsselung
     * JWE initialization vector - zufällig ausgewürfelter Initialisierungsvektor, der als Input für die symmetrische Verschlüsselung benötigt wird
     * JWE Ciphertext - mit dem symmetrischen Schlüssel zur Inhaltsverschlüsselung verschlüsselte Inhaltsdaten
     * JWE Authentication Tag- bei der Verschlüsselung der Inhaltsdaten zusätzlich ausgegebene Kontrolldaten zur Integritätssicherung

   In unserem Anwendungsfall werden die benötigten öffentlichen Schlüssel der Empfänger mit ihren herkunftsbestätigenden Zertifikaten im System als JSON
   Web Key (JWK) hinterlegt und können auf Anfrage über die API von sendenden Clients abgerufen werden. Wie dieser JWK-Abruf im Detail erfolgt, wie Sie
   die im JWK enthaltenen Zertifikate prüfen können und wie dann mit Hilfe von JWE Antragsdaten verschlüsselt werden, wird im Artikel Verschlüsseln
   beschrieben. Die Entschlüsselung von Antragsdaten wird im Artikel Entschlüsseln beschrieben.
```
