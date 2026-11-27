// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.tasks

import org.gradle.api.DefaultTask
import org.gradle.api.file.DirectoryProperty
import org.gradle.api.file.FileTree
import org.gradle.api.file.RegularFileProperty
import org.gradle.api.provider.ListProperty
import org.gradle.api.provider.Property
import org.gradle.api.provider.Provider
import org.gradle.api.tasks.Input
import org.gradle.api.tasks.InputFiles
import org.gradle.api.tasks.Internal
import org.gradle.api.tasks.Optional
import org.gradle.api.tasks.OutputFile
import org.gradle.api.tasks.PathSensitive
import org.gradle.api.tasks.PathSensitivity
import org.gradle.api.tasks.TaskAction
import org.gradle.process.ExecOperations
import uniffi.cargo.CargoParser
import uniffi.config.UniffiCrateData
import javax.inject.Inject

internal abstract class CargoBuildTask @Inject constructor(
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

    @get:Input
    abstract val crate: Property<UniffiCrateData>

    @get:Input
    abstract val features: ListProperty<String>

    @get:Input
    abstract val release: Property<Boolean>

    @get:Input
    abstract val target: Property<String>

    @get:Input
    @get:Optional
    abstract val buildTool: Property<String>

    @get:OutputFile
    abstract val artifact: RegularFileProperty

    init {
        val profile = release.map { release ->
            if (release) "release" else "debug"
        }

        val cargoPackage = workspace.zip(crate) { workspace, crate ->
            CargoParser.parseManifest(workspace.dir(crate.path).dir(crate.name))
        }

        val libName: Provider<String> = target.zip(cargoPackage) { target, cargoPackage ->
            val library = cargoPackage.libName
            val splitTarget = target.split('-')
            when {
                splitTarget.contains("windows") -> "$library.dll"
                splitTarget.contains("linux") -> "lib$library.so"
                splitTarget.contains("apple") -> "lib$library.dylib"
                else -> error("Unsupported target $target")
            }
        }

        artifact.convention(project.provider {
            val workspace = workspace.get()
            val target = target.get()
            val profile = profile.get()
            val libName = libName.get()

            workspace.dir("target").dir(target).dir(profile).file(libName)
        })

        buildTool.convention(target.map { target ->
            if (target.endsWith("windows-msvc")) {
                "xwin"
            } else {
                null
            }
        })
    }

    @TaskAction
    fun build() {
        execOperations.exec {
            workingDir(workspace)
            environment(
                "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER",
                "aarch64-linux-gnu-gcc"
            )
            environment(
                "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER",
                "clang"
            )
            environment(
                "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS",
                "-C link-arg=-fuse-ld=mold"
            )

            executable("cargo")

            if (buildTool.isPresent) {
                args(buildTool.get())
            }

            args("build")

            if (release.get()) {
                args("--release")
            }
            if (target.isPresent) {
                args("--target", target.get())
            }
            if (crate.isPresent) {
                args("--package", crate.get().name)
            }
            if (features.isPresent) {
                args("--no-default-features")
                args("--features", features.get().joinToString(","))
            }
        }
    }
}
