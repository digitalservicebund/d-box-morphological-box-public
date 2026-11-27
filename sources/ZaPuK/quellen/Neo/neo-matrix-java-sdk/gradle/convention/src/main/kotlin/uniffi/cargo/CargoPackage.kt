// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.cargo

import org.gradle.api.file.Directory
import java.io.Serializable

internal data class CargoPackage(
    val name: String,
    val directory: Directory,
    val lib: CargoLib? = null,
) : Serializable {
    val libName: String
        get() = lib?.name ?: name.replace("-", "_")
}
