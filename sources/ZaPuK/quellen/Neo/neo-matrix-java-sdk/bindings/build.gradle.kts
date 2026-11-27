// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

plugins {
    alias(libs.plugins.kotlin.jvm)
    id("maven-publish")
    id("uniffi")
}

kotlin {
    jvmToolchain(17)
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            artifactId = "matrix-sdk"
            from(components["java"])
        }
    }

    repositories {
        val ciJobToken = System.getenv("CI_JOB_TOKEN")
        val ciProjectId = System.getenv("CI_PROJECT_ID")
        if (!ciJobToken.isNullOrEmpty() && !ciProjectId.isNullOrEmpty()) {
            maven {
                url = uri("https://gitlab.opencode.de/api/v4/projects/${ciProjectId}/packages/maven")
                name = "GitLab"
                credentials(HttpHeaderCredentials::class) {
                    name = "Job-Token"
                    value = System.getenv("CI_JOB_TOKEN")
                }
                authentication {
                    create("header", HttpHeaderAuthentication::class)
                }
            }
        }
    }
}

repositories {
    mavenCentral()
}

dependencies {
    api(libs.jna)
    implementation(libs.kotlinx.coroutines.core)
}


uniffi {
    workspace.set("src/main/rust/matrix-rust-sdk")

    packageId.set("matrix-sdk-ffi")
    release.set(false)
    val uniffiTargets = System.getenv("UNIFFI_TARGETS")?.split(",") ?: listOf(
        "x86_64-pc-windows-msvc",
        "x86_64-unknown-linux-gnu",
        "aarch64-unknown-linux-gnu",
    )
    targets.set(uniffiTargets)

    features.set(listOf(
        "bundled-sqlite"
    ))

    crates {
        create("matrix-sdk-base") {
            path.set("crates")
            packageName.set("dev.fitko.neo.matrix.sdk.internal.base")
            android.set(false)
            androidCleaner.set(false)
        }
        create("matrix-sdk-common") {
            path.set("crates")
            packageName.set("dev.fitko.neo.matrix.sdk.internal.common")
            android.set(false)
            androidCleaner.set(false)
        }
        create("matrix-sdk-crypto-ffi") {
            path.set("bindings")
            packageName.set("dev.fitko.neo.matrix.sdk.crypto")
            cdylibName.set("matrix_sdk_crypto_ffi")
            android.set(false)
            androidCleaner.set(false)
        }
        create("matrix-sdk-crypto") {
            path.set("crates")
            packageName.set("dev.fitko.neo.matrix.sdk.internal.crypto")
            android.set(false)
            androidCleaner.set(false)
        }
        create("matrix-sdk-ffi") {
            path.set("bindings")
            packageName.set("dev.fitko.neo.matrix.sdk")
            cdylibName.set("matrix_sdk_ffi")
            android.set(false)
            androidCleaner.set(false)
        }
        create("matrix-sdk-events") {
            path.set("crates")
            packageName.set("dev.fitko.neo.matrix.sdk.events")
            android.set(false)
            androidCleaner.set(false)
        }
        create("matrix-sdk") {
            path.set("crates")
            packageName.set("dev.fitko.neo.matrix.sdk.internal")
            android.set(false)
            androidCleaner.set(false)
        }
    }
}
