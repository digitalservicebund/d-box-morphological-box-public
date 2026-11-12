#!/usr/bin/env sh

# SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
#
# SPDX-License-Identifier: EUPL-1.2

set -euo pipefail

# Remove the output directory if it exists
rm -rf output/
uid="$(id -u):$(id -g)"

echo
echo "Generate Mermaid.js diagrams"
docker run -it --rm -u "$uid" -v "$PWD":/usr/local/structurizr:z structurizr/cli export \
  -workspace /usr/local/structurizr/bundid-pw-auth.dsl \
  -format mermaid \
  -output /usr/local/structurizr/output/mermaidjs

echo
echo "Generate Static HTML diagrams"
docker run -it --rm -u "$uid" -v "$PWD":/usr/local/structurizr:z structurizr/cli export \
  -workspace /usr/local/structurizr/bundid-pw-auth.dsl \
  -format static \
  -output /usr/local/structurizr/output/static


echo
echo "Generate D2 files"
docker run -it --rm -u "$uid" -v "$PWD":/usr/local/structurizr:z structurizr/cli export \
  -workspace /usr/local/structurizr/bundid-pw-auth.dsl \
  -format d2 \
  -output /usr/local/structurizr/output/d2


echo
echo "Generate PNG images from D2 files using the locally installed d2 CLI"
mkdir -p output/images
find output/d2/ -name '*.d2' -print0 | xargs -0 -P 0 -r -I {} sh -c 'd2 --theme 8 --layout elk "$1" "output/images/$(basename "${1%.d2}.png")"' -- {}
