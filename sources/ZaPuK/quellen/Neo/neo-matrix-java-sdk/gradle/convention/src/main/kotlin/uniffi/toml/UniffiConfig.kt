// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.toml

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
internal data class UniffiConfig(
    @SerialName("bindings")
    val bindings: BindingsConfig
)
