// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.cargo

import org.gradle.api.file.Directory
import org.tomlj.Toml
import org.tomlj.TomlArray

internal object CargoParser {
    fun parseManifest(directory: Directory): CargoPackage {
        val rootManifest = Toml.parse(directory.file("Cargo.toml").asFile.toPath())
        val name = rootManifest.getString("name") ?: directory.asFile.name
        val lib = rootManifest.getTable("lib")?.let { lib ->
            CargoLib(
                name = lib.getString("name"),
                crateTypes = lib.getArray("crate-type")?.toStringList().orEmpty()
            )
        }
        return CargoPackage(name, directory, lib)
    }

    private fun TomlArray.toStringList(): List<String> =
        List(size()) { getString(it) }
}
