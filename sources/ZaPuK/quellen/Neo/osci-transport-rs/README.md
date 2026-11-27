<!--
SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# osci-transport

## About

A Rust implementation of [OSCI-Transport 1.2](https://www.xoev.de/sixcms/media.php/13/OSCI-1.2_mit_Korrigenda.pdf),
the German public-sector message transport protocol built on SOAP with its
own XML-DSig/XML-Enc-based signing and encryption. Built for the Matrix G2C
communication pilot (Neo) to support a Matrix-OSCI converter and iOS/wasm
client components.

This project is under active development and not yet production-ready.

It is made available publicly as open source software in the hope that it may
also be useful for reuse outside of this project.

## What's included

The workspace is split into one crate per responsibility:

- [osci-transport](./crates/osci-transport): message model, XML codec, dialog
  state machine, and the handler/router framework requests dispatch through.
- [osci-transport-crypto](./crates/osci-transport-crypto): the default
  RustCrypto-backed signing, signature verification, transport and content
  decryption/encryption used by the client and server crates.
- [osci-transport-server](./crates/osci-transport-server): the receive
  pipeline (decrypt, verify, parse, dispatch) for an OSCI recipient.
- [osci-transport-client](./crates/osci-transport-client): builds, signs,
  encrypts, sends and tracks requests to an Intermediär over HTTP.
- [osci-transport-ffi](./crates/osci-transport-ffi): UniFFI bindings exposing
  message classification and decryption to iOS, Android and, via
  [uniffi-bindgen-react-native](https://github.com/jhugman/uniffi-bindgen-react-native),
  React Native/wasm/Node.js.

Wire compatibility is checked two ways: a vendored corpus of real
Java-produced `.osci` samples (`tests/interop/corpus/`), and a manual
end-to-end test against Governikus's public test Intermediär
(`crates/osci-transport-client/tests/against_test_intermediary.rs`,
`#[ignore]`d - see that file for what round-trips today and what doesn't yet).

Also included, from the project template this repository was created from:

- A [LICENSE](./LICENSE) file and [LICENSES](./LICENSES/) folder which includes
  the software license's full-text under which this project is published
- A [.gitlab-ci.yaml](./.gitlab-ci.yml) file with quality gates and
  automation for software releases.
- A [CHANGELOG.md](./CHANGELOG.md) where the release history is kept. The
  contents are [automatically updated](./docs/workflow.md#releasing)
  by the `semantic-release` process.
- A [CONTRIBUTING.md](./CONTRIBUTING.md) file that details how you can
  contribute to this project.
- A [REUSE.toml](./REUSE.toml) file to annotate licenses and copyright
  information for assets that can't be inline header annotated.
- A [SECURITY.md](./SECURITY.md) file with project-wide security disclosure
  information.

and assorted configuration files for the included tools:

- **Automated Releases**: [semantic-release](https://semantic-release.gitbook.io/semantic-release/)
  for automated versioning and changelog generation.
- **Conventional Commits**: enforcing a consistent [commit message format](https://www.conventionalcommits.org/)
  for automated release notes.
- **License Compliance**: [REUSE](https://reuse.software)-compliant licensing setup.
- **Documentation linting and spell-checking**: pre-commit hooks and CI steps
  for linting and spell-checking markdown files.

## Building and testing

```sh
cargo build --workspace
cargo test --workspace
```

The live-Intermediär test needs network access and is excluded by default:

```sh
cargo test -p osci-transport-client --test against_test_intermediary -- --ignored --nocapture
```

## Development Guidelines

For information on how to use the conventions defined in this project, check
out the [expected development workflow](docs/workflow.md).

Please also see [CONTRIBUTING.md](./CONTRIBUTING.md) for details on how
you can contribute to this project.

## Licensing

This project is licensed under the [European Union Public License 1.2](https://spdx.org/licenses/EUPL-1.2.html).

For information on how to manage license and copyright annotations for your
contributions, check our [development workflow](docs/workflow.md) documentation.
