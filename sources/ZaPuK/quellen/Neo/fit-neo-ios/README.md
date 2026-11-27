<!--
SPDX-FileCopyrightText: 2025-2026 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Neo iOS App

## About

This is the iOS application repository for the
[Matrix-based G2X communication pilot](https://gitlab.opencode.de/fitko/matrix-g2x),
code name "Neo".

### Environment Access

To use and test the Neo iOS app, you need an account on one of the available
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

### Install Homebrew dependencies

We rely on [Homebrew](https://brew.sh/) to install tools that this project
requires, so run:

```sh
curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh
```

Then, run in this project's root folder:

```sh
brew bundle
```

### Generate the Xcode project

This project requires [Xcode 16.4](https://developer.apple.com/services-account/download?path=/Developer_Tools/Xcode_16.4/Xcode_16.4.xip)
or latest to be installed, as well as the corresponding Xcode Command Line
tools.

With both of these in place, you now can run:

```sh
xcodegen
```

which will create the Xcode project and you can open it with:

```sh
open FITNeo.xcodeproj
```

### Building the Neo Rust SDK

Because we currently don't have binary artifacts of our custom [neo-rust-sdk](https://gitlab.opencode.de/fitko/matrix-g2x/neo-rust-sdk),
the `xcodegen` action will try to find a local `.xcframework` and build it if its
not available.

For this to be successful, you must have the following installed:

- Rust and Cargo:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- GNU patch:

```sh
brew install gpatch
```

- GNU realpath:

```sh
brew install coreutils
```

- iOS targets:

```sh
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-darwin x86_64-apple-ios
```

Check the SDK build script at `scripts/preGenCommand.sh` if any steps are failing.

### Running the test targets

To run both the Unit Tests and UI Tests targets, use fastlane:

```sh
fastlane test
```

## Licensing

This project is licensed under the [European Union Public License 1.2](https://spdx.org/licenses/EUPL-1.2.html).

For information on how to manage license and copyright annotations for your
contributions, check our [development workflow](docs/workflow.md) documentation.
