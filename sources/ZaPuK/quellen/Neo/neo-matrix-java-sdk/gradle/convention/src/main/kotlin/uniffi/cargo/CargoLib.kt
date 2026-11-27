// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.cargo

import java.io.Serializable

internal data class CargoLib(
    val name: String? = null,
    val crateTypes: List<String> = emptyList()
) : Serializable
