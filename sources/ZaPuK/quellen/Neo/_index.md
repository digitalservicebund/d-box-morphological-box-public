# Neo: Repositories der GitLab-Gruppe `fitko/matrix-g2x`

Abgerufen: 2026-11-27. Herausgeber: FITKO (Neo-Pilot, Kind-Projekt von ZaPuK). Gruppe: <https://gitlab.opencode.de/fitko/matrix-g2x>.

Jeder Unterordner ist ein Export (`git archive`) des Standard-Branches ohne Git-Historie und ohne Submodule (`neo-rust-sdk`, `neo-matrix-java-sdk` binden matrix-rust-sdk als Submodul ein, der Inhalt fehlt). Commits auf anderen Branches und die Historie, auf die sich `../../Neo-ADR-Umsetzung.md` stützt, sind nur upstream nachvollziehbar. Bei den meisten Repos ist `develop` der Arbeits-Branch, `main` ist dort veraltet.

Das ZaPuK-Repo der Gruppe (`it-planungsrat/fit-ab/zapuk`) liegt separat unter `../zapuk-v1.1.1/`. Der ältere Export `../../../neo-solution-architecture-develop/` entspricht einem früheren Stand von `solution-architecture/`.

| Ordner | URL | Branch | Commit | Datum | Commits | Inhalt |
|---|---|---|---|---|---|---|
| `bundid-eid-proxy/` | <https://gitlab.opencode.de/fitko/matrix-g2x/bundid-eid-proxy> | `develop` | `aa74fc5` | 2026-06-15 | 19 | PoC-Proxy für nativen eID-Login über BundID |
| `drachen-einfach/` | <https://gitlab.opencode.de/fitko/matrix-g2x/drachen-einfach> | `main` | `4eef13d` | 2026-06-17 | 43 | Demo-Onlinedienst (FIT-Connect-Formular, simulierter BundID-Login) |
| `fit-neo-ios/` | <https://gitlab.opencode.de/fitko/matrix-g2x/fit-neo-ios> | `develop` | `0181378` | 2026-11-11 | 60 | iOS-Client (matrix-rust-sdk, AusweisApp SDK) |
| `fit-neo-web/` | <https://gitlab.opencode.de/fitko/matrix-g2x/fit-neo-web> | `develop` | `d9d2cec` | 2026-11-17 | 350 | Web-Client, seit 2026-07-08 auf `neo-web-sdk` |
| `gitlab-profile/` | <https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile> | `main` | `82ff239` | 2026-07-23 | 63 | Gruppenprofil, Integrationsleitfaden, Leistungsbeschreibung, Folien, Videos |
| `git-project-template/` | <https://gitlab.opencode.de/fitko/matrix-g2x/git-project-template> | `develop` | `eb05e1e` | 2026-11-09 | 78 | Repo-Vorlage (keine Architekturinhalte) |
| `infrastructure/` | <https://gitlab.opencode.de/fitko/matrix-g2x/infrastructure> | `develop` | `5feafc0` | 2026-11-17 | 105 | Helm/ArgoCD: zwei ESS-Homeserver, Keycloak mit BundID, geschlossene Föderation |
| `neo-cases-devtool/` | <https://gitlab.opencode.de/fitko/matrix-g2x/neo-cases-devtool> | `develop` | `2b2d1f4` | 2026-11-25 | 17 | Entwicklerwerkzeug zum Anlegen von Vorgängen |
| `neo-matrix-java-sdk/` | <https://gitlab.opencode.de/fitko/matrix-g2x/neo-matrix-java-sdk> | `develop` | `0bb24bd` | 2026-05-21 | 39 | Java-SDK für Fachverfahren (UniFFI über matrix-rust-sdk) |
| `neo-rust-sdk/` | <https://gitlab.opencode.de/fitko/matrix-g2x/neo-rust-sdk> | `develop` | `22d91d9` | 2026-11-17 | 37 | Rust-SDK (Patches auf matrix-rust-sdk), Neo-Nachrichtentypen, Kotlin-Bindings |
| `neo-web-sdk/` | <https://gitlab.opencode.de/fitko/matrix-g2x/neo-web-sdk> | `develop` | `5d26420` | 2026-10-28 | 13 | Web/WASM-SDK |
| `osci-transport-rs/` | <https://gitlab.opencode.de/fitko/matrix-g2x/osci-transport-rs> | `develop` | `ad1f4a3` | 2026-11-21 | 123 | OSCI-Transport 1.2 in Rust, Grundlage eines Matrix-OSCI-Konverters |
| `planning/` | <https://gitlab.opencode.de/fitko/matrix-g2x/planning> | `main` | `3378e00` | 2026-07-15 | 29 | Nur Issue-Vorlagen |
| `planning-test-do-not-use/` | <https://gitlab.opencode.de/fitko/matrix-g2x/planning-test-do-not-use> | `main` | `863120b` | 2026-03-20 | 25 | Testkopie von `planning/` |
| `solution-architecture/` | <https://gitlab.opencode.de/fitko/matrix-g2x/solution-architecture> | `develop` | `8d6efb0` | 2026-11-26 | 50 | arc42-Lösungsarchitektur, Neo-ADRs 0000–0004, Konzepte, Structurizr |
| `ui-styleguide/` | <https://gitlab.opencode.de/fitko/matrix-g2x/ui-styleguide> | `main` | `8a6d699` | 2026-06-22 | 3 | UI-Styleguide (Bilder, Doku) |
