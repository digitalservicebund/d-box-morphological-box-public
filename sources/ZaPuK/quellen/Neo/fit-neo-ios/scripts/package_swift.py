#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
#
# SPDX-License-Identifier: EUPL-1.2

import sys
import json

print("// swift-tools-version:6.1")
print("// The swift-tools-version declares the minimum version of Swift required to build this package.")
print("import PackageDescription\n")
print("let package = Package(\n\tname: \"DepsOnly\",\n\tdependencies: [")

names = []
packages = []

for package in json.load(sys.stdin)['pins']:
    names.append(package["identity"])
    current = ""
    current += "\t\t.package(url: \"" + package["location"] + "\",\n"
    state = package["state"]
    if state["version"] != None:
        current += "\t\t\texact: \"" + state["version"] + "\")"
    elif state["branch"] != None:
        current += "\t\t\tbranch: \"" + state["branch"] + "\")"
    elif state["revision"] != None:
        current += "\t\t\trevision: \"" + state["revision"] + "\")"
    packages.append(current)
print(",\n".join([package for package in packages]))
print("\t],\n\ttargets: [\n\t\t.target(name: \"DepsOnly\", dependencies: [")
print(",\n".join(["\t\t\t\"" + name + "\"" for name in names]))
print("\t\t])\n\t]\n)")
