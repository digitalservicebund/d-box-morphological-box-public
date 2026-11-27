// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.config

import org.gradle.api.tasks.Input
import org.gradle.api.tasks.Optional
import java.io.Serializable

internal data class UniffiCrateData(
    @Input val name: String,
    @Input val path: String,
    @Input val packageName: String,
    @Input @Optional val cdylibName: String?,
    @Input val android: Boolean,
    @Input val androidCleaner: Boolean,
) : Serializable
