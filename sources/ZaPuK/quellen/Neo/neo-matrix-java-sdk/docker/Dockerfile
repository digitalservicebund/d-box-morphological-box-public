# SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
#
# SPDX-License-Identifier: CC0-1.0

FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    git curl ca-certificates \
    build-essential \
    llvm clang \
    gcc-aarch64-linux-gnu g++-aarch64-linux-gnu \
    mold \
    && rm -rf /var/lib/apt/lists/*

ENV JAVA_HOME=/opt/java/openjdk
ENV PATH=$JAVA_HOME/bin:$PATH
COPY --from=eclipse-temurin:17-jdk-noble /opt/java /opt/java/

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
COPY --from=rust:bookworm /usr/local/rustup /usr/local/rustup/
COPY --from=rust:bookworm /usr/local/cargo /usr/local/cargo/

RUN rustup target add \
    x86_64-unknown-linux-gnu \
    aarch64-unknown-linux-gnu \
    x86_64-pc-windows-msvc

RUN cargo install --locked cargo-xwin
RUN cargo xwin cache xwin
