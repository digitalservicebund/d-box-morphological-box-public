# Quellen: Standardisierung von Proxy Re-Encryption

Abgerufen am 2026-11-18. Hintergrund zur Aussage „kein etablierter Standard“ für Proxy Re-Encryption im OZG-PLUS-Postfach (`morph_box.csv` I4 Ausprägung 6 und Ü2 Ausprägung 3). HTML-Seiten mit `lynx -dump -nolist` in Text umgewandelt, PDFs unverändert gespeichert.

| Datei | URL | Titel / Herausgeber | Datum | Inhalt |
|---|---|---|---|---|
| ieee_1363.3-2013_status.md | https://standards.ieee.org/ieee/1363.3/3822/ | IEEE SA, IEEE 1363.3-2013 | 2013, inaktiv seit 2024-03-21 | Identitätsbasierte Verfahren mit Pairings; Status „Inactive-Reserved“; Kurzbeschreibung nennt Proxy Re-Encryption nicht |
| wikipedia_ieee-p1363.md | https://en.wikipedia.org/wiki/IEEE_P1363 | Wikipedia, IEEE P1363 | Abruf 2026-11-18 | 1363.3-2013 enthält laut Artikel u. a. Proxy Re-Encryption auf Basis bilinearer Pairings |
| etsi_tr-103-719_v1.1.1_2022-03.pdf | https://www.etsi.org/deliver/etsi_TR/103700_103799/103719/01.01.01_60/tr_103719v010101p.pdf | ETSI, TR 103 719 V1.1.1, Guide to Identity-Based Cryptography | 2022-03 | Technical Report, kein Standard; Abschnitt 5.2.7 beschreibt Proxy Re-Encryption als Eigenschaft einiger IBE-Verfahren |
| ietf-cfrg_proxy-re-encryption-proposal_2016-08.md | https://mailarchive.ietf.org/arch/msg/cfrg/xSuAKoMXygK9JYbQnHZ0HtYSe-k/ | IETF/IRTF CFRG, Mailingliste | 22.08.2016 | Vorschlag mit Individual Draft `draft-hallambaker-mesh-recrypt-00`, nicht von einer Arbeitsgruppe übernommen |
| bsi_tr-02102-1_2026-01.pdf | https://www.bsi.bund.de/SharedDocs/Downloads/DE/BSI/Publikationen/TechnischeRichtlinien/TR02102/BSI-TR-02102.pdf | BSI, TR-02102-1 Kryptographische Verfahren: Empfehlungen und Schlüssellängen | 2026-01 | empfiehlt u. a. ECIES (Abschnitt 2.3.4); Proxy Re-Encryption nicht erwähnt |

## Gesucht, nicht gefunden

- ISO/IEC: kein Standard und kein Projekt zu Proxy Re-Encryption gefunden; ISO/IEC 18033-6 und 18033-8 betreffen homomorphe Verschlüsselung.
- NIST: keine Veröffentlichung zu Proxy Re-Encryption gefunden.
- Governikus: Name und Spezifikation des in `proxycrypt` umgesetzten Verfahrens nicht öffentlich dokumentiert (vgl. `../anbindung-technik/governikus_ozgpp_integrationsleitfaden_v1.11_2026-04.pdf`, PDF-S. 20).
