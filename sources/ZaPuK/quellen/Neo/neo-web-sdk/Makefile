# SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
#
# SPDX-License-Identifier: EUPL-1.2

.DEFAULT_GOAL := build

.PHONY: build bindings bindings-ci check_bindings_unix clean package

CHECK_BINDINGS := check_bindings_unix
BINDINGS_TARGET := bindings

# neo-rust-sdk wrapper repository (FITKO fork of matrix-rust-sdk)
NEO_SDK_REPO   := https://gitlab.opencode.de/fitko/matrix-g2x/neo-rust-sdk.git
NEO_SDK_BRANCH := develop
# Wrapper clone directory
NEO_SDK_DIR    := rust_modules/neo-rust-sdk
# The matrix-rust-sdk submodule lives one level deeper inside the wrapper.
# This is the directory that owns the workspace Cargo.toml used for the build.
MATRIX_SDK_DIR := rust_modules/neo-rust-sdk/matrix-rust-sdk

# macOS ships BSD patch; prefer GNU patch (gpatch) when available.
PATCH := $(if $(filter Darwin,$(shell uname -s)),gpatch,patch)

# Clean up generated files
clean:
	@echo "Cleaning up generated files..."
	@rm -rf src/generated
	@rm -rf src/index.web.ts
	@rm -rf rust_modules
	@rm -rf dist
	@echo "Clean complete."

# Default build: ensure generated bindings exist, then assemble the
# publishable package layout.
build: $(CHECK_BINDINGS) package

# Unix-like check: run bindings target if src/generated is missing
check_bindings_unix:
	@[ -d src/generated ] || $(MAKE) $(BINDINGS_TARGET)

# CI target: skip rebuild if WASM exists and inputs haven't changed.
# Hash covers the ubrn config, toolchain, Makefile, patches, and package.json
# (so a uniffi-bindgen-react-native version bump triggers a full regeneration).
bindings-ci:
	@current_hash=$$(cat ubrn.config.yaml rust-toolchain.toml Makefile patches/*.patch package.json 2>/dev/null | sha256sum | cut -d' ' -f1); \
	cached_hash=$$(cat src/generated/.build-hash 2>/dev/null || echo ""); \
	if [ "$$current_hash" = "$$cached_hash" ] && [ -f src/generated/wasm-bindgen/index_bg.wasm ]; then \
		echo "Bindings up to date (hash: $$current_hash), skipping rebuild."; \
	else \
		$(MAKE) clean && $(MAKE) bindings && printf '%s' "$$current_hash" > src/generated/.build-hash; \
	fi

# Linux / macOS bindings generation
bindings:
	@echo "Generating bindings (unix)..."
	# 1. Clone the neo-rust-sdk wrapper with all submodules (idempotent)
	@[ -d $(NEO_SDK_DIR)/.git ] || \
		git clone --recurse-submodules --branch $(NEO_SDK_BRANCH) $(NEO_SDK_REPO) $(NEO_SDK_DIR)
	# 2. Apply FITKO's own patches to the submodule via the wrapper's Makefile.
	#    The sentinel lives in the wrapper dir so it survives the submodule reset.
	@if [ ! -f $(NEO_SDK_DIR)/.deps-applied ]; then \
		$(MAKE) -C $(NEO_SDK_DIR) && \
		touch $(NEO_SDK_DIR)/.deps-applied; \
	fi
	# 3. Apply our downstream patches on top (idempotent via sentinel)
	@if [ -n "$$(ls patches/*.patch 2>/dev/null)" ] && \
	   [ ! -f $(NEO_SDK_DIR)/.patches-applied ]; then \
		for p in patches/*.patch; do \
			$(PATCH) -p1 -d $(MATRIX_SDK_DIR) < "$$p"; \
		done && \
		touch $(NEO_SDK_DIR)/.patches-applied; \
	fi
	# 4. Pin the Rust toolchain inside the submodule if it does not already have one
	@[ -f $(MATRIX_SDK_DIR)/rust-toolchain.toml ] || \
		cp ./rust-toolchain.toml $(MATRIX_SDK_DIR)/
	# 5. Two-step ubrn build: comment out bindings/wasm first so the workspace
	#    compiles on the first pass (the crate doesn't exist yet), then restore it.
	@echo "Temporarily commenting out bindings/wasm in Cargo.toml"
	@sed -i.bak -E "s/^([[:space:]]*)(\"?bindings\/wasm\"?)/\1# \2/" \
		$(MATRIX_SDK_DIR)/Cargo.toml || true
	@echo "Running first web build (expected to fail — generates bindings/wasm crate)..."
	-@yarn ubrn:web:build:release
	@echo "Restoring bindings/wasm in Cargo.toml"
	@sed -i.bak 's/^[[:space:]]*#[[:space:]]*"\(bindings\/wasm\)",/    "\1",/' \
		$(MATRIX_SDK_DIR)/Cargo.toml || true
	@echo "Running web build again to compile the generated crate..."
	@yarn ubrn:web:build:release
	@echo "Removing explicit WASM init call from src/index.web.ts (bundler target: WASM loads via static import)"
	@sed -i.bak '/import initAsync/d' src/index.web.ts || true
	@sed -i.bak '/import wasmPath/d' src/index.web.ts || true
	@sed -i.bak '/await initAsync/d' src/index.web.ts || true
	@echo "Ensuring TypeScript/linters disable header is present before first export"
	@grep -q '/\* tslint:disable \*/' src/index.web.ts || \
		(cp src/index.web.ts src/index.web.ts.bak2 || true; \
		 awk 'BEGIN{inserted=0} /^export/ && !inserted \
		      {print "/* tslint:disable */"; print "/* eslint-disable */"; \
		       print "// @ts-nocheck"; inserted=1} {print}' \
		      src/index.web.ts > src/index.web.ts.tmp && \
		 mv src/index.web.ts.tmp src/index.web.ts) || true
	@echo "Cleaning up temporary backup files"
	@rm -f src/index.web.ts.bak src/index.web.ts.bak2 src/index.web.ts.tmp || true
	@echo "Optimizing WASM binary with wasm-opt..."
	@find src/generated -name "*.wasm" | while IFS= read -r wasm_file; do \
		echo "  Optimizing $$wasm_file..."; \
		yarn wasm-opt -Oz --strip-debug "$$wasm_file" -o "$$wasm_file"; \
	done
	@echo "Bindings generation complete."

# Assemble the publishable npm package layout from the generated bindings.
# The top-level *.ts files (barrel + ubrn-generated glue) are compiled to
# plain JS/d.ts via tsc; the wasm-bindgen subdirectory is already plain
# JS/d.ts/wasm output from wasm-bindgen itself and is copied verbatim so the
# relative `./wasm-bindgen/index.js` / `./wasm-bindgen/index_bg.wasm` imports
# inside the compiled files keep resolving correctly.
package:
	@echo "Assembling publishable package in dist/..."
	@rm -rf dist
	@yarn build:ts
	@mkdir -p dist/generated
	@cp -R src/generated/wasm-bindgen dist/generated/wasm-bindgen
	@echo "Package assembly complete."
