<!--
SPDX-FileCopyrightText: 2025, 2026 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Neo Matrix SDK (JVM bindings)

## Docs

- [**Usage Documentation**](USAGE.md)
- [**Sample Code**](sample/src/main/java/com/example/neo/Main.java)

## Usage

```toml
[versions]
matrix-sdk = "0.2.0-17687e74"

[libraries]
matrix-sdk = { module = "dev.fitko.neo:matrix-sdk", version.ref = "matrix-sdk" }
neo-sdk = { module = "dev.fitko.neo:neo-sdk", version.ref = "matrix-sdk" }
```

```kotlin
dependencies {
    implementation(libs.matrix.sdk)
    implementation(libs.neo.sdk)

    // Import the native libraries corresponding to your target platform
    runtimeOnly(variantOf(libs.matrix.sdk) { classifier("linux-x86-64") })
    runtimeOnly(variantOf(libs.matrix.sdk) { classifier("linux-aarch64") })
    runtimeOnly(variantOf(libs.matrix.sdk) { classifier("win32-x86-64") })
    runtimeOnly(variantOf(libs.matrix.sdk) { classifier("darwin-aarch64") })
}
```

## Dev Setup

### 1. Clone the project and submodules

```bash
git clone https://gitlab.opencode.de/fitko/matrix-g2x/neo-matrix-java-sdk.git

cd neo-matrix-java-sdk
git submodule init
git submodule update

cd bindings/src/main/rust
git submodule init
git submodule update
make
```

### 2. Create .env file for SDK Sample (Optional)

```ini
MATRIX_HOST=ess1.dev.neo.fitko.dev
MATRIX_USER=...
MATRIX_PASS=...
MATRIX_KEY=...

UNIFFI_TARGETS=x86_64-unknown-linux-gnu
```

| Env Variable | Description |
| ------------ | ----------- |
| `MATRIX_HOST` | Matrix Servername, server part of a @user:server mxid |
| `MATRIX_USER` | Matrix Username, user part of a @user:server mxid |
| `MATRIX_PASS` | Matrix password for the given user |
| `MATRIX_KEY` | Matrix recovery key, created during initial crypto setup |
| `UNIFFI_TARGETS` | Targets triples to build the rust SDK for |

Valid targets:

- `x86_64-pc-windows-msvc`
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `aarch64-apple-darwin`

### 3. Execute Sample (Optional)

```bash
dotenv ./gradlew :sample:run
```

## Dependencies

- JDK 17
- GNU Make
- rust & cargo
  - `rustup target add x86_64-unknown-linux-gnu`
- clang & llvm
- mold

Linux aarch64 builds:

- `rustup target add aarch64-unknown-linux-gnu`
- gcc-aarch64-linux-gnu
- g++-aarch64-linux-gnu

Win32 builds:

- `rustup target add x86_64-pc-windows-msvc`
- `cargo install --locked cargo-xwin`
- `cargo xwin cache xwin`

## Development Guidelines

For information on how to use the conventions defined in this project, check
out the [expected development workflow](docs/workflow.md).

Please also see [CONTRIBUTING.md](./CONTRIBUTING.md) for details on how
you can contribute to this project template.

## Licensing

This project is licensed under the [European Union Public License 1.2](https://spdx.org/licenses/EUPL-1.2.html).

For information on how to manage license and copyright annotations for your
contributions, check our [development workflow](docs/workflow.md) documentation.
