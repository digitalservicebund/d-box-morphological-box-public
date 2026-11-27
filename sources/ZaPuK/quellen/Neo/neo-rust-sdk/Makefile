# SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
#
# SPDX-License-Identifier: CC0-1.0

# Check for macOS and use GNU tools instead
REALPATH := $(if $(filter Darwin,$(shell uname -s)),grealpath,realpath)
PATCH    := $(if $(filter Darwin,$(shell uname -s)),gpatch,patch)

# Only the targets the iOS app actually needs: physical iOS devices and
# the Apple Silicon simulator. Override on the command line if
# more slices are needed, e.g.:
#   make build-framework BUILD_TARGETS="aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios"
BUILD_TARGETS ?= aarch64-apple-ios aarch64-apple-ios-sim
BUILD_PROFILE ?= release # reldev?
TARGET_DIR := matrix-rust-sdk/target

.PHONY: dependencies
dependencies: \
	clean \
	matrix-rust-sdk/Cargo.toml \
	matrix-rust-sdk/bindings/matrix-sdk-ffi/Cargo.toml \
	matrix-rust-sdk/bindings/matrix-sdk-ffi/src/lib.rs \
	matrix-rust-sdk/crates/matrix-sdk-events \
	matrix-rust-sdk/bindings/matrix-sdk-ffi/src/neo

matrix-rust-sdk/crates/%: crates/%
	ln -s $(shell $(REALPATH) -s --relative-to="$(@D)" $<) $@

.PHONY: clean
clean:
	git --git-dir=matrix-rust-sdk/.git reset --hard
	git --git-dir=matrix-rust-sdk/.git clean -fdx

.PHONY: matrix-rust-sdk/Cargo.toml
matrix-rust-sdk/Cargo.toml: crates/dependencies.patch
	$(PATCH) $@ $<

.PHONY: matrix-rust-sdk/bindings/matrix-sdk-ffi/Cargo.toml
matrix-rust-sdk/bindings/matrix-sdk-ffi/Cargo.toml: crates/ffi-dependencies.patch
	$(PATCH) $@ $<

.PHONY: matrix-rust-sdk/bindings/matrix-sdk-ffi/src/lib.rs
matrix-rust-sdk/bindings/matrix-sdk-ffi/src/lib.rs: crates/ffi-modules.patch
	$(PATCH) $@ $<

.PHONY: matrix-rust-sdk/bindings/matrix-sdk-ffi/src/neo
matrix-rust-sdk/bindings/matrix-sdk-ffi/src/neo: crates/matrix-sdk-ffi/
	ln -s $(shell $(REALPATH) -s --relative-to="$(@D)" $<) $@

.PHONY: build-framework
build-framework: dependencies
	cd matrix-rust-sdk && CARGO_INCREMENTAL=0 cargo xtask swift build-framework \
		--profile $(BUILD_PROFILE) \
		--sequentially \
		$(foreach target,$(BUILD_TARGETS),--target $(target))
	$(MAKE) clean-build-intermediates

.PHONY: clean-build-intermediates
clean-build-intermediates: $(addprefix clean-target-,$(BUILD_TARGETS))
	rm -rf $(TARGET_DIR)/$(BUILD_PROFILE)
	rm -rf matrix-rust-sdk/bindings/apple/generated/lipo

clean-target-%:
	rm -rf $(TARGET_DIR)/$*

test:
	cd matrix-rust-sdk && cargo test -p matrix-sdk-events

rust-check:
	cd matrix-rust-sdk && cargo fmt --check -p matrix-sdk-events
