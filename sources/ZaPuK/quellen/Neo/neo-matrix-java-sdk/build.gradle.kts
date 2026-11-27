// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

plugins {
    alias(libs.plugins.kotlin.jvm) apply false
}

allprojects {
    group = "dev.fitko.neo"
    version = "0.2.0".let { version ->
        System.getenv("SNAPSHOT_SUFFIX")?.let { suffix -> "$version-$suffix" }
            ?: version
    }
}
