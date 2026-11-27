// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.tasks

import org.gradle.api.DefaultTask
import org.gradle.api.file.DirectoryProperty
import org.gradle.api.file.FileCollection
import org.gradle.api.file.FileTree
import org.gradle.api.model.ObjectFactory
import org.gradle.api.provider.ListProperty
import org.gradle.api.tasks.InputFiles
import org.gradle.api.tasks.Internal
import org.gradle.api.tasks.Nested
import org.gradle.api.tasks.OutputFiles
import org.gradle.api.tasks.PathSensitive
import org.gradle.api.tasks.PathSensitivity
import org.gradle.api.tasks.TaskAction
import uniffi.config.UniffiCrateData
import uniffi.toml.BindingsConfig
import uniffi.toml.KotlinConfig
import uniffi.toml.TomlWriter
import uniffi.toml.UniffiConfig
import javax.inject.Inject

internal abstract class UniffiConfigTask @Inject constructor(
    objects: ObjectFactory
) : DefaultTask() {
    @get:Internal
    abstract val workspace: DirectoryProperty

    @get:InputFiles
    @get:PathSensitive(PathSensitivity.RELATIVE)
    val workspaceInputs: FileTree
        get() = workspace.asFileTree.matching {
            include("**/Cargo.toml")
            include("**/Cargo.lock")
            exclude("target/**")
            exclude("build/**")
        }

    @get:Nested
    abstract val crates: ListProperty<UniffiCrateData>

    @get:OutputFiles
    val outputFiles: FileCollection = objects.fileCollection().from(
        workspace.zip(crates) { workspace, crates ->
            crates.mapNotNull { crate ->
                workspace.dir(crate.path).dir(crate.name).file("uniffi.toml")
            }
        }
    )

    @TaskAction
    fun generate() {
        val workspace = workspace.get()
        val crates = crates.get()
        for (crate in crates) {
            val configFile = workspace.dir(crate.path).dir(crate.name).file("uniffi.toml")
            TomlWriter.encodeToFile(configFile.asFile, UniffiConfig(
                bindings = BindingsConfig(
                    kotlin = KotlinConfig(
                        packageName = crate.packageName,
                        cdylibName = crate.cdylibName,
                        android = crate.android,
                        androidCleaner = crate.androidCleaner,
                    )
                )
            ))
        }
    }
}
