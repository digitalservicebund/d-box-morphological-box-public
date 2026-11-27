// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi

import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.api.publish.PublishingExtension
import org.gradle.api.publish.maven.MavenPublication
import org.gradle.api.tasks.Copy
import org.gradle.api.tasks.bundling.Jar
import org.gradle.kotlin.dsl.configure
import org.gradle.kotlin.dsl.named
import org.gradle.kotlin.dsl.register
import org.gradle.kotlin.dsl.withType
import org.gradle.language.jvm.tasks.ProcessResources
import org.jetbrains.kotlin.gradle.dsl.KotlinJvmProjectExtension
import uniffi.config.UniffiCrateData
import uniffi.config.UniffiExtension
import uniffi.tasks.CargoBuildTask
import uniffi.tasks.UniffiBindgenTask
import uniffi.tasks.UniffiConfigTask
import uniffi.util.getJnaPlatform

class UniffiPlugin : Plugin<Project> {
    override fun apply(project: Project) {
        val extension = project.extensions.create("uniffi", UniffiExtension::class.java)

        extension.workspace.convention("src/main/rust")
        extension.release.convention(true)

        with(project) {
            val workspace = layout.projectDirectory.dir(extension.workspace)
            val generatedSourceDir = layout.buildDirectory.dir("generated/source/uniffi/main/kotlin")
            val generatedResourceDir = layout.buildDirectory.dir("generated/resources/uniffi/main")

            val crates = provider {
                extension.crates.map {
                    UniffiCrateData(
                        name = it.name,
                        path = it.path.get(),
                        packageName = it.packageName.get(),
                        cdylibName = it.cdylibName.orNull,
                        android = it.android.getOrElse(false),
                        androidCleaner = it.androidCleaner.getOrElse(false),
                    )
                }
            }

            val packageCrate = provider {
                val packageId = extension.packageId.get()
                val crates = crates.get()

                crates.find { it.name == packageId }
            }

            val bindingsTask = tasks.register<UniffiBindgenTask>("generateBindings") {
                this.workspace.set(workspace)
                this.bindingDir.set(generatedSourceDir)
            }

            afterEvaluate {
                for (target in extension.targets.get()) {
                    val jnaPlatform = getJnaPlatform(target)

                    val buildTask = tasks.register<CargoBuildTask>("buildRustLibrary-$target") {
                        this.workspace.set(workspace)
                        this.crate.set(packageCrate)
                        this.features.set(extension.features)
                        this.release.set(extension.release)
                        this.target.set(target)
                    }

                    val configTask = tasks.register<UniffiConfigTask>("configureUniffi-$target") {
                        this.workspace.set(workspace)
                        this.crates.set(crates)
                    }

                    bindingsTask.configure {
                        if (!nativeLibrary.isPresent) {
                            dependsOn(configTask)
                            this.nativeLibrary.set(buildTask.flatMap { it.artifact })
                        }
                    }

                    val packageTask = tasks.register<Copy>("packageNativeLibs-$target") {
                        from(buildTask.flatMap { it.artifact })
                        into(generatedResourceDir.map { generatedResourceDir ->
                            generatedResourceDir.dir(jnaPlatform)
                        })
                    }

                    val jarTask = tasks.register<Jar>("jar-$target") {
                        archiveClassifier.set(jnaPlatform)
                        from(packageTask) {
                            into(jnaPlatform)
                        }
                    }

                    artifacts.add("archives", jarTask)

                    pluginManager.withPlugin("maven-publish") {
                        extensions.configure<PublishingExtension> {
                            publications.withType<MavenPublication> {
                                artifact(jarTask)
                            }
                        }
                    }

                    pluginManager.withPlugin("java") {
                        tasks.named<ProcessResources>("processResources") {
                            dependsOn(packageTask)
                        }
                        tasks.named<Jar>("jar") {
                            exclude("$jnaPlatform/**")
                        }
                    }
                }

                pluginManager.withPlugin("org.jetbrains.kotlin.jvm") {
                    extensions.configure<KotlinJvmProjectExtension> {
                        sourceSets.getByName("main") {
                            kotlin.srcDir(bindingsTask.flatMap { it.bindingDir })
                            resources.srcDir(generatedResourceDir)
                        }
                    }
                }
            }
        }
    }
}
