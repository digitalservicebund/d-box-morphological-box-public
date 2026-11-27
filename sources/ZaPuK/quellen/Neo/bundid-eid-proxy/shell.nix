# SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
#
# SPDX-License-Identifier: CC0-1.0

{
  pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    copier
    reuse

    nodejs

    (ausweisapp.overrideAttrs (old: {
      cmakeFlags = (old.cmakeFlags or []) ++ [
        (pkgs.lib.cmakeBool "CONTAINER_SDK" true)
      ];

      # NOTE: Fix building without resources/translations.
      preBuild = "";
    }))
  ];
}
