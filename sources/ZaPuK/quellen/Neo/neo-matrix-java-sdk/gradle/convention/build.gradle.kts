// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

plugins {
    `kotlin-dsl`
    alias(libs.plugins.kotlin.serialization)
}

repositories {
    mavenCentral()
}

dependencies {
    compileOnly(libs.kotlin.gradlePlugin)
    compileOnly(libs.checkerframework.annotations)
    implementation(libs.tomlj)
    implementation(libs.kotlinx.serialization.properties)
}

gradlePlugin {
    plugins {
        create("uniffi") {
            id = "uniffi"
            implementationClass = "uniffi.UniffiPlugin"
        }
    }
}
