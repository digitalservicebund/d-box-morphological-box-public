<!--
SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Neo Web SDK

## About

This is the TypeScript/WebAssembly SDK repository for the
[Matrix-based G2X communication pilot](https://gitlab.opencode.de/fitko/matrix-g2x),
code name "Neo". It builds and publishes the generated Matrix Rust SDK
bindings consumed by the Neo web app, as the npm package
[`@matrix-g2x/neo-web-sdk`](https://gitlab.opencode.de/fitko/matrix-g2x/neo-web-sdk/-/packages).

## Development Guidelines

For information on how to use the conventions defined in this project, check
out the [expected development workflow](docs/workflow.md).

Please also see [CONTRIBUTING.md](./CONTRIBUTING.md) for details on how
you can contribute to this project.

## Development Setup

This next section goes into what you have to install to build this project
on your development machine.

### Prerequisites

This project generates its bindings from a Rust codebase compiled to
WebAssembly. You need to have the following installed:

- Node.js
- yarn
- Rust, with the `wasm32-unknown-unknown` target (channel pinned in
  [rust-toolchain.toml](./rust-toolchain.toml))
- [`wasm-bindgen-cli`](https://github.com/rustwasm/wasm-bindgen)

Alternatively, run `nix-shell` to get a shell with all of the above already
set up.

### Install dependencies

To install the necessary dependencies, run the following command.
This assumes you have yarn and Node.js installed.

```bash
yarn install
```

### Generating the bindings

The Rust backend is code generated. You will need to generate them.
To do so you first must install the yarn dependencies:

```bash
yarn install
```

After that we provide a Makefile target to generate the bindings.
To generate the bindings, run:

```bash
make bindings
```

## Consuming this package

`@matrix-g2x/neo-web-sdk` is published to this project's npm package
registry on GitLab. To install it in another project, add the following to
that project's `.yarnrc.yml`:

```yaml
npmScopes:
  matrix-g2x:
    npmRegistryServer: 'https://gitlab.opencode.de/api/v4/projects/11767/packages/npm/'
```

Since this project is public, no authentication token is required to install
the package. Then add it as a normal dependency:

```bash
yarn add @matrix-g2x/neo-web-sdk
```

### Prereleases

Every push to `develop` publishes a prerelease (versioned `0.0.0-next-<git
short sha>`, e.g. `0.0.0-next-a1b2c3d`) under the npm dist-tag `next`, in
addition to the stable releases `main` publishes under `latest`. This lets a
consuming project pick up in-progress SDK work before a stable release
exists, without a plain `yarn add @matrix-g2x/neo-web-sdk` ever resolving to
a prerelease by accident.

Unlike `main`'s releases, `develop` prereleases are published directly from
CI without semantic-release, git tags, or a GitLab Release - each is just a
one-off version tied to a commit, and the `next` dist-tag is simply
changed to point at whichever one was published most recently.

To consume a prerelease, either install the `next` tag directly:

```bash
yarn add @matrix-g2x/neo-web-sdk@next
```

or pin the exact prerelease version in `package.json` (recommended for
reproducible builds, since `next` always points at whatever was published
most recently):

```json
"@matrix-g2x/neo-web-sdk": "0.0.0-next-a1b2c3d"
```

Renovate will keep proposing newer `next`-tagged prereleases once a project
is pinned to one, the same way it does for stable versions.

### Testing a local change against a consuming project

If you're changing something here and want to test it in a consuming
project (e.g. the Neo web app) before publishing, use
[`yarn link`](https://yarnpkg.com/cli/link) from that project instead of
installing the registry-resolved version:

```bash
# In this repository, build the package first:
make build

# Then, in the consuming project:
yarn link /path/to/neo-web-sdk
```

This rewrites the consuming project's `@matrix-g2x/neo-web-sdk` resolution to
a `portal:` reference to your local checkout until you undo it. To go back
to the registry-resolved version in the consuming project, run:

```bash
yarn up @matrix-g2x/neo-web-sdk
```

## Contributing

To contribute take a look at the [contributing guidelines](CONTRIBUTING.md).

## Licensing

This project is licensed under the [European Union Public License 1.2](https://spdx.org/licenses/EUPL-1.2.html).

For information on how to manage license and copyright annotations for your
contributions, check our [development workflow](docs/workflow.md) documentation.
