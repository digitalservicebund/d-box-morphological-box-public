#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
#
# SPDX-License-Identifier: EUPL-1.2

# Builds the osci-transport FFI to wasm, generates the TypeScript bindings and
# assembles the package in dist/. Needs the rustup target wasm32-unknown-unknown
# and a wasm-bindgen CLI that matches the wasm-bindgen version in ../Cargo.lock:
# set UBRN_WASM_BINDGEN to its path, or put it on PATH.
set -euo pipefail
cd "$(dirname "$0")"

rm -rf src/generated src/index.web.ts dist rust_modules
yarn ubrn build web --config ubrn.config.yaml --release

# Bundler target: the wasm module loads through a static import, so the
# explicit init that ubrn generates must go.
entry=src/index.web.ts
sed -i '/import initAsync/d;/import wasmPath/d;/await initAsync/d' "$entry"
sed -i '1i /* tslint:disable */\n/* eslint-disable */\n// @ts-nocheck' "$entry"

find src/generated -name '*.wasm' -exec yarn wasm-opt -Oz --strip-debug {} -o {} \;

# The top-level files compile to plain JS and d.ts. The wasm-bindgen output is
# plain JS already and is copied as it is, so the relative imports keep working.
yarn tsc -p tsconfig.json
mkdir -p dist/generated
cp -R src/generated/wasm-bindgen dist/generated/wasm-bindgen
