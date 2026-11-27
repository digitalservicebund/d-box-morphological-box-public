# SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
#
# SPDX-License-Identifier: CC0-1.0

{
  pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball {
        url = "https://github.com/oxalica/rust-overlay/archive/6fe5039018d05cee5d01dda7df1c0846fb7943a4.tar.gz";
        sha256 = "sha256-vf3cZf8imUlPzFtICa1uyReDzoPV0XhHOIRM3tqI5VY=";
      }))
    ];
  }
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    openssl
    yarn-berry
    wasm-bindgen-cli_0_2_108

    (rust-bin.stable."1.93.0".default.override {
      targets = [ "wasm32-unknown-unknown" ];
    })
  ];
}
