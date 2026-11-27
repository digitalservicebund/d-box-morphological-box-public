# Neo: Umsetzung der ZaPuK-ADRs im Zeitverlauf

Stand: 2026-11-27. Grundlage ist die vollständige Git-Historie (alle Branches) der 16 Repositories der Gruppe <https://gitlab.opencode.de/fitko/matrix-g2x>, abgerufen am 2026-11-27. Exporte der Standard-Branches liegen unter `quellen/Neo/` (siehe `quellen/Neo/_index.md`), die Historie selbst nur upstream. ADR-Daten und -Status nach `quellen/zapuk-v1.1.1/Zielarchitektur/Architekturentscheidungen/`.

## Lesehinweise

- Kein Code-Commit nennt eine ZaPuK-ADR-Nummer. Die Zuordnung ist überwiegend inhaltlich. Explizite Verweise auf ZaPuK-ADRs gibt es nur in `solution-architecture`, ab 2026-02-13.
- Neo hat eigene ADRs 0000–0004 (`solution-architecture/architecture_decisions/`). Sie sind nicht mit den ZaPuK-ADRs gleicher Nummer identisch.
- Datum = Autorendatum. Arbeits-Branch ist meist `develop`; „gemergt“ heißt: auf `develop` bzw. dem Standard-Branch.
- Die Zuordnung stammt aus einer Git-Auswertung mit Stichproben. Megolm, bPK2 als Localpart und `postkorbHandle` sind direkt geprüft.

## Zeitleiste (gemergt)

| Datum | ZaPuK-ADR (Datum der ADR) | Repo · Commit | Umsetzung in Neo | Art |
|---|---|---|---|---|
| 2023-06-05 | 0011 Kommunikationsschicht (2025-04-15) | gitlab-profile `668ec2c` | FITKO-Steckbrief (26.05.2023) zur FIT-AB-Sitzung vom 05.06.2023: Matrix als technische Basis einer G2C/G2B-Infrastruktur erproben; README: Pilotierung „angestrebt“. Den Beschluss vom 5.6.2023 nennt erst das Logbuch (`f1d1956`, 2025-02-07). Liegt vor der ADR; ADR-0011 verweist nicht auf den Pilot. | Vorhabenssteckbrief |
| 2025-06-20 | 0011, 0012, 0014, 0009, 0010 | solution-architecture `8dbc894`, `a6d60c2` | Matrix als Vorgabe, E2EE nach OZG, BundID/eID-Login (AusweisApp SDK), Erweiterung des FIT-Connect-SDK für Fachverfahren, Matrix neben dem ZBP | Architekturdoku |
| 2025-09-04 (Commit 2026-02-20) | 0007, 0002/1114, 1145, 1104 | infrastructure `f4cfbbb` | Zwei ESS-Homeserver `ess1`/`ess2` (Synapse, MAS, Element). Geschlossene Föderation per `federation_domain_whitelist` mit nur diesen beiden Servern. `serverName` `*.dev.neo.fitko.dev` | Infrastruktur |
| 2025-09-16 | 0011 | fit-neo-ios `ca99b4b` | Erster Matrix-Client-Code (matrix-rust-components-swift) | Code |
| 2025-09-17 bis 2026-02-16 | 0014 (2025-04-28) | infrastructure `7e02d76`, `74debe4`, `8143a53` | Keycloak mit BundID-SAML-Plugin, Login-Kette MAS → Keycloak → BundID | Infrastruktur |
| 2025-10-28 | 1114 (2026-10-09) | solution-architecture `4abd11a` | Getrennte Synapse/MAS/PostgreSQL für Bürger und Behörden. **Abweichung:** „Create Matrix account with bPK2 as localpart“ (personenbezogener Localpart, widerspricht 1104/1105) | Architekturdoku |
| 2025-11-03 | 0011 | fit-neo-web `ab05962` | Web-Client auf matrix-rust-sdk (WASM) | Code |
| 2025-11-05 | 0012 / 1100 | fit-neo-web `eb129c9` | E2EE mit dem Standard-Kryptostack von matrix-rust-sdk (Olm/Megolm). **Abweichung: kein MLS** | Code |
| 2025-12-04 | 1147 (2026-11-26) | fit-neo-web `e68e5f4` | Serverseitiges Schlüssel-Backup und Cross-Signing (Option A), keine Oberfläche für Wiederherstellungsschlüssel | Code |
| 2026-01-09 | 0014 | fit-neo-ios `5b70a32` | „Mit BundID einloggen“ (OIDC über MAS) | Code |
| 2026-02-13 | 0002 / 0004 / 1114 | solution-architecture `16e7787`, `e9377e8`, `d9e630e` | Neo-ADR 0001–0003: getrennte Homeserver für Verwaltung und Privatpersonen, getrennte Betreiber, vorerst keine eigenen Homeserver Dritter (teilweise Abweichung von 0004). **Erste explizite ZaPuK-Verweise** | Neo-ADR |
| 2026-03-13 | 0012 / 1100 / 1147 | fit-neo-ios `7325af5` | E2EE und Schlüssel-Backup in iOS | Code |
| 2026-03-17 bis 2026-10-05 | 0009 (2025-04-14) | neo-matrix-java-sdk `6aced00`, neo-rust-sdk `86875fa`, neo-web-sdk `d3e05ac`, neo-rust-sdk `536e886` | SDKs: Java (Fachverfahren), Rust, Web/WASM, Kotlin (Android) | Code |
| 2026-04-12 | 0014 | fit-neo-web `56ed48d` | BundID-Login im Web | Code |
| 2026-04-27 | 1102 (Option C) / 1100 | neo-matrix-java-sdk `4b5da53` | Ein verschlüsselter Raum je Vorgang; E2EE bilateral zwischen Fachverfahren und Bürger, kein FP-Bot | Code |
| 2026-04-28 | 1120 (teilweise) | solution-architecture `97b4b27` | Anhänge als `mxc`-Medien; E2EE der Anhänge verpflichtend ab `f7c0efa` (2026-11-17) | Architekturdoku |
| 2026-05-26 | 0012 | solution-architecture `b11553d` | Megolm in der Spezifikation festgeschrieben; in neo-cases-devtool `5a05bb9` (2026-10-26) hart codiert (`m.megolm.v1.aes-sha2`) | Spezifikation |
| 2026-06-02 / 2026-06-12 | 0014 | bundid-eid-proxy `c63ee36`, infrastructure `25f8d19` | PoC für nativen eID-Login, auf Dev ausgerollt | PoC |
| 2026-06-05 | 1118 (schwach) | fit-neo-web `1792a0e` | Gelesen/ungelesen aus Read Receipts; keine formale Abruf- oder Lesebestätigung | Code |
| 2026-06-09 | 1102 / 1109 | solution-architecture `b604f7e` | „Each case is represented by a Matrix room“ | Architekturdoku |
| 2026-07-23 | 1114, 1105, 0009, 1122 | gitlab-profile `82ff239` | Integrationsleitfaden: ess1 = Privatpersonen, ess2 = Fachbehörden; MXID aus dem `postkorbHandle` gebildet („pseudonymisiert“); Anbindung über FIT-Connect-SDK; E-Mail-/Push-Benachrichtigung „in Arbeit“ | Doku |
| 2026-09-10 bis 2026-11-20 | 0010 (Protocol Converter) | osci-transport-rs `f0df7d4` ff. | OSCI-Transport-Bibliothek „to support a Matrix-OSCI converter“; der Konverter selbst fehlt noch | Code |
| 2026-10-02 | 1102 | neo-rust-sdk `019fdee` | Vorgangs-ID und Vorgangszustand; Details in einem verschlüsselten Event (Metadatenminimierung) | Code |
| 2026-10-29 | 0018, 1118, 1119, 1120 | solution-architecture `e858ca1` | Konzept 303 zu Bereitstellungs-, Abruf- und Lesebestätigung. **Explizite ADR-Verweise** | Konzept |

## Nur auf nicht gemergten Branches

| Datum | ZaPuK-ADR | Repo · Commit (Branch) | Inhalt |
|---|---|---|---|
| 2025-08-07 | 0013 / 1106 | solution-architecture `bc0485a` (`brose/feat-adr0002-device-verification`) | Geräteverifikation mit eID-Methode |
| 2026-01-09 / 2026-01-13 | 0014 / 0015 / 1144 | solution-architecture `9eddc01` (`braid/mutual-identification`), `bf34595` (`braid/x.509-verification`) | Gegenseitige Identifikation: Behörde per X.509, Bürger per eID/BundID/EUDI-Wallet |
| 2026-07-31 | 0014 | solution-architecture `b1d305f` (`braid/mutual-identification`) | „aligned with … ZaPuK ADR-0014“ |
| 2026-10-12 | 0018 | solution-architecture `9fdc51d` (`ben/server-delivery-receipts`) | Serversignierte Bereitstellungsbestätigung (`m.server_receipt`) |
| 2026-10-20 | 1106 / 1105 | solution-architecture `5436d7e` (`ben/auth-adr`) | Vorab-Identifikation per BundID-Token, eID im E2EE-Kanal, bereichsspezifisches Kennzeichen |
| 2026-10-30 | 1147 | solution-architecture `72cb9ca` (`ben/592-keyrecovery`) | „In accordance with ZaPuk ADR #1147“: Konzept Schlüsselwiederherstellung |
| 2026-11-04 | 1144 (OSCI-seitig) | osci-transport-rs `8fc2cc2` (`marcel-nordeck/feat/cvs-ocsp-xkms`) | Zertifikatskettenprüfung, OCSP |

## Befunde

1. **Neo geht ZaPuK oft voraus.** Viele ZaPuK-ADRs sind auf November 2026 datiert (u. a. 0018, 1114, 1118, 1147). Neo hatte diese Punkte Monate vorher gebaut oder entworfen. Hier hat der Pilot die Entscheidung eher gespeist als nachträglich umgesetzt.
2. **Abweichungen:**
   - E2EE mit Megolm statt MLS (ADR-0012).
   - Localpart der MXID: `solution-architecture` nennt bPK2 (widerspricht 1104/1105), der Integrationsleitfaden das `postkorbHandle`. Die Neo-Dokumente widersprechen sich hier.
   - Vorerst keine Homeserver Dritter (Neo-ADR 0003 gegenüber ZaPuK-ADR 0004).
3. **Ohne Spur in Neo:** 0008/1148 (Self-Service-Portal, Onboarding), 1130 (Malware-Prüfung), 1135 (Veraktung), 1146 (EUDI-Wallet), 1101/1112/1113 (Funktionspostfächer, Berechtigungen, Vertretung), 1107/1108 (Nachweisformat, Nachweisstellen), 1103, 1110, 1111, 1121, 1149.
