// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

rootProject.name = "matrix-sdk-jvm"

pluginManagement {
    includeBuild("gradle/convention")
}

dependencyResolutionManagement {
    repositories {
        mavenCentral()
    }
}

include(":bindings")
include(":sdk")
include(":sample")
