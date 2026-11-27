#!/bin/bash

# SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
#
# SPDX-License-Identifier: EUPL-1.2

# Resolve the Neo Rust SDK location. On CI runners (GitLab CI sets CI=true)
# the SDK lives in a fixed, persistent checkout so the expensive Rust build
# is reused across pipelines; locally it is a sibling of this repository.
# Keep in sync with scripts/packages/matrix-{local,ci}.yml.
if [ "${CI:-}" = "true" ]; then
    NEO_RUST_SDK_PATH="/Users/admin/neo-rust-sdk"
else
    NEO_RUST_SDK_PATH="../neo-rust-sdk"
fi

# Check if there is already a build of the Swift bindings framework
if [ -d "$NEO_RUST_SDK_PATH/matrix-rust-sdk/bindings/apple/generated/MatrixSDKFFI.xcframework" ]; then
    echo "Swift bindings framework already built, skipping build step."
else
    echo "Swift bindings framework not found, building now..."

    # Get the Neo Rust SDK and submodules (if not already present)
    if [ ! -d "$NEO_RUST_SDK_PATH" ]; then
        GIT_CONFIG_NOSYSTEM=1 git clone --no-local --recurse-submodules https://gitlab.opencode.de/fitko/matrix-g2x/neo-rust-sdk.git "$NEO_RUST_SDK_PATH"
    fi

    cd "$NEO_RUST_SDK_PATH" && make build-framework && cd -
fi
