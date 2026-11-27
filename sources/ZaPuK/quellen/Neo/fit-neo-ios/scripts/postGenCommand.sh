#!/bin/bash

# SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
#
# SPDX-License-Identifier: EUPL-1.2

# Resolve package dependencies and generate Package.resolved
xcodebuild -resolvePackageDependencies

# Generate Package.swift for renovate to check for dependencies updates
cat FITNeo.xcodeproj/project.xcworkspace/xcshareddata/swiftpm/Package.resolved | python3 scripts/package_swift.py > Package.swift

# Move file header template in project shared data folder
cp scripts/templates/IDETemplateMacros.plist FITNeo.xcodeproj/xcshareddata/
