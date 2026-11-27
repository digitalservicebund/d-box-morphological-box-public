// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.tasks

import org.gradle.api.DefaultTask
import org.gradle.api.file.DirectoryProperty
import org.gradle.api.file.FileTree
import org.gradle.api.file.RegularFileProperty
import org.gradle.api.tasks.InputFile
import org.gradle.api.tasks.InputFiles
import org.gradle.api.tasks.Internal
import org.gradle.api.tasks.OutputDirectory
import org.gradle.api.tasks.PathSensitive
import org.gradle.api.tasks.PathSensitivity
import org.gradle.api.tasks.TaskAction
import org.gradle.process.ExecOperations
import javax.inject.Inject

internal abstract class UniffiBindgenTask @Inject constructor(
    private val execOperations: ExecOperations
) : DefaultTask() {
    @get:Internal
    abstract val workspace: DirectoryProperty

    @get:InputFiles
    @get:PathSensitive(PathSensitivity.RELATIVE)
    val inputs: FileTree
        get() = workspace.asFileTree.matching {
            include("**/*.rs")
            include("**/Cargo.toml")
            include("**/Cargo.lock")
            exclude("target/**")
            exclude("build/**")
        }

    @get:InputFile
    @get:PathSensitive(PathSensitivity.NAME_ONLY)
    abstract val nativeLibrary: RegularFileProperty

    @get:OutputDirectory
    abstract val bindingDir: DirectoryProperty

    @TaskAction
    fun generate() {
        val bindingDir = bindingDir.get().asFile

        bindingDir.mkdirs()
        execOperations.exec {
            workingDir(workspace)
            executable("cargo")
            args("run")
            args("--package", "uniffi-bindgen")
            args("--")
            args("generate", nativeLibrary.get().asFile.absolutePath)
            args("--language", "kotlin")
            args("--out-dir", bindingDir.absolutePath)
            args("--no-format")
        }
    }
}
