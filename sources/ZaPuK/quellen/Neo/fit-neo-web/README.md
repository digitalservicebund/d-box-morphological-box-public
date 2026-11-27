<!--
SPDX-FileCopyrightText: 2025-2026 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Neo Web App

## About

This is the web application repository for the
[Matrix-based G2X communication pilot](https://gitlab.opencode.de/fitko/matrix-g2x),
code name "Neo".

### Environment Access

To use and test the Neo Web app, you need an account on one of the available
environments:

- **DEV Environment**: <https://ess1.dev.neo.fitko.dev>
- **DEMO Environment**: <https://ess1.demo.neo.fitko.dev>

Please contact the Neo team to request access credentials for either
environment. Without valid credentials for one of these environments, you will
not be able to log in and use the app.

## Development Guidelines

For information on how to use the conventions defined in this project, check
out the [expected development workflow](docs/workflow.md).

Please also see [CONTRIBUTING.md](./CONTRIBUTING.md) for details on how
you can contribute to this project.

## Development Setup

This next section goes into what you have to install to build this project
on your development machine.

### Prerequisites

This is a ReactJS based web application that uses WASM to interface with
the Rust components. You need to have the following installed:

- Node.js
- yarn
- wasm-bindgen-cli (see below)

### Install dependencies

To install the necessary dependencies, run the following command.
This assumes you have yarn and Node.js installed.

```bash
yarn install
```

### Changing the homeserver

The homeserver is configured at runtime via `public/config.json`,
which is fetched by the app on startup. Edit this file to change the homeserver:

```json
{
  "matrixHomeserver": "https://your.homeserver.url/"
}
```

### Run development server

To start the development server, run the following command:

```bash
yarn dev
```

### Build for production

If building for production manually (when not using Docker),
run the following command:

```bash
yarn build
```

## Development

As part of the development process, you can use the following commands.

Additionally we expect you to run the linting command before committing
your code to ensure code quality.
Tests are run automatically in the CI/CD pipeline.

### Linting

To lint the code, run the following command:

```bash
yarn lint
```

### Formatting

Format the code with [Oxfmt](https://oxc.rs/docs/guide/usage/formatter.html):

```bash
yarn format:write
```

Check formatting without changing files:

```bash
yarn format:check
```

### Testing

To run the tests, use the following commands:

```bash
yarn test
yarn e2e
```

### Storybook

Storybook is a tool we use for documenting the UI components.
To start Storybook, run the following command:

```bash
yarn storybook
```

### The Matrix SDK dependency

The Rust backend is code generated and published as the npm package
[`@matrix-g2x/neo-web-sdk`](https://gitlab.opencode.de/fitko/matrix-g2x/neo-web-sdk),
built from its own repository. This project depends on it like any other
yarn dependency; the `@matrix-g2x` scope is mapped to that project's GitLab
npm registry in [.yarnrc.yml](./.yarnrc.yml). Since `neo-web-sdk` is a public
project, no authentication token is required to install it.

#### Depending on an unstable SDK prerelease

While `neo-web-sdk`'s Rust layer is still moving and hasn't cut a stable
release upstream, this project can depend on a prerelease instead of a
`main`-published version. `neo-web-sdk`'s CI publishes every push to its
`develop` branch as a prerelease (versioned `0.0.0-next-<git short sha>`,
e.g. `0.0.0-next-a1b2c3d`) under the npm dist-tag `next`, kept separate from
`latest` so it's never picked up by accident.

Pin the exact prerelease version in `package.json` (a caret range doesn't
work reliably across prerelease bumps):

```json
"@matrix-g2x/neo-web-sdk": "0.0.0-next-a1b2c3d"
```

Renovate will keep proposing newer `next`-tagged prereleases once pinned
this way. Switch back to a caret range on a real stable version once
`neo-web-sdk` has cut one from `main`.

#### Local development against an unreleased SDK change

If you need to test a change to `neo-web-sdk` before it's published, use
[`yarn link`](https://yarnpkg.com/cli/link) to point this project at your
local checkout instead of the registry-resolved version:

```bash
# In your neo-web-sdk checkout, build the package first:
make build

# Then, in this project:
yarn link /path/to/neo-web-sdk
```

This rewrites the `@matrix-g2x/neo-web-sdk` resolution to a `portal:`
reference to your local checkout until you undo it. To go back to the
registry-resolved version, run:

```bash
yarn up @matrix-g2x/neo-web-sdk
```

### Translations

Translations are managed using `i18next` and `i18next-parser`.
To extract translation keys from the source code, run:

```bash
yarn extract-translations
```

This will scan the source files and
update the translation JSON files located in the `public/locales` directory.
You should review the changes and provide translations for any new keys.
Also make sure to run this command before committing changes
that include new translation keys.

Our default language is English (`en`). If you add new languages,
make sure to update the `i18next-parser.config.js` and
the `src/i18n.ts` files accordingly.

## Contributing

To contribute take a look at the [contributing guidelines](CONTRIBUTING.md).

## Licensing

This project is licensed under the [European Union Public License 1.2](https://spdx.org/licenses/EUPL-1.2.html).

For information on how to manage license and copyright annotations for your
contributions, check our [development workflow](docs/workflow.md) documentation.
