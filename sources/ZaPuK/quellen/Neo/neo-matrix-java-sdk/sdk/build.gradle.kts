// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

plugins {
    alias(libs.plugins.kotlin.jvm)
    id("maven-publish")
    java
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            artifactId = "neo-sdk"
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

kotlin {
    jvmToolchain(17)
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(project(":bindings"))
    runtimeOnly(project(":bindings", configuration = "archives"))

    implementation(libs.kotlinx.coroutines.core)

    testImplementation(libs.kotlin.test)
    testImplementation(libs.kotlinx.coroutines.test)

    implementation("com.fasterxml.jackson.core:jackson-databind:2.21.2")
    implementation("com.fasterxml.jackson.core:jackson-annotations:2.21")
    implementation("com.google.inject:guice:7.0.0")

    compileOnly("org.projectlombok:lombok:1.18.46")
    annotationProcessor("org.projectlombok:lombok:1.18.46")

    testCompileOnly("org.projectlombok:lombok:1.18.46")
    testAnnotationProcessor("org.projectlombok:lombok:1.18.46")
}
