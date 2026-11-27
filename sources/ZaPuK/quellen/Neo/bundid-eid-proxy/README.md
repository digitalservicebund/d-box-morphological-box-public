<!--
SPDX-FileCopyrightText: 2025-2026 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# BundID eID Proxy

> [!warning] Proof of Concept
> This code is for demo purposes only! For production proxy
> service should not be used, especially due to the security
> implications of proxying a SAML login through third-party
> infrastructure.

## About

This repository contains a minimal proof of concept to hide the
BundID UI for native eID flows.

The server component implements BundID's SAML POST endpoint for
starting a downstream authentication flow
\[Authentifizierung als Fachverfahren\] and emulates a BundID
frontend session.

Once the server obtains an eID tcTokenURL, it redirects to a
native eID client. After completing the eID flow, the eID client
needs to navigate to refresh url, with the base swapped for the
server's origin, completing the SAML flow.

The loading spinner UI has been copied from the
[Neo Web Client](https://gitlab.opencode.de/fitko/matrix-g2x/fit-neo-web).

## How to use it

Install the required dependencies either using the nix shell:

- To use your host's `nixpkgs` checkout, use `nix-shell`
- To use the pinned `nixpkgs` checkout, use `nix develop`

or using your system's package manager for `nodejs` and an OCI runtime
(podman, docker) for headless AusweisApp.

Install node dependencies using `npm install`.

Using Firefox, install the
[Request Control](https://addons.mozilla.org/en-US/firefox/addon/requestcontrol/)
addon and configure the following rule
`Redirect: int.id.bund.de → http://localhost:1234{pathname}{search}` by loading
[request-control-rules.json](./request-control-rules.json).

Start the following processes in parallel:

```shell
src/server.mts https://int.id.bund.de http://localhost:1235`
src/client.mts firefox

# When using nix
AusweisApp

# When using OCI
podman run --name ausweisapp --replace --rm -p 127.0.0.1:24727:24727 governikus/ausweisapp2
```

Navigate to <https://keycloak.dev.neo.fitko.dev> in Firefox and use the BundID upstream.

### Development Guidelines

For information on how to use the conventions defined in this project, check
out the [expected development workflow](docs/workflow.md).

Please also see [CONTRIBUTING.md](./CONTRIBUTING.md) for details on how
you can contribute to this project template.

## Licensing

This project is licensed under the [European Union Public License 1.2](https://spdx.org/licenses/EUPL-1.2.html).

For information on how to manage license and copyright annotations for your
contributions, check our [development workflow](docs/workflow.md) documentation.
