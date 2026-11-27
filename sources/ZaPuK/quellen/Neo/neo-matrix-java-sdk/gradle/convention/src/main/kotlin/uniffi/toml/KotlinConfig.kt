// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.toml

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
internal data class KotlinConfig(
    @SerialName("package_name") val packageName: String,
    @SerialName("cdylib_name") val cdylibName: String? = null,
    @SerialName("android") val android: Boolean,
    @SerialName("android_cleaner") val androidCleaner: Boolean,
)
