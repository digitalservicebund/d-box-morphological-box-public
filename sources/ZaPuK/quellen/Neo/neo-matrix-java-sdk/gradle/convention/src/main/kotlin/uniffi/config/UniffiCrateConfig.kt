// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.config

import org.gradle.api.provider.MapProperty
import org.gradle.api.provider.Property
import javax.inject.Inject

abstract class UniffiCrateConfig @Inject constructor(val name: String) {
    abstract val path: Property<String>

    abstract val packageName: Property<String>
    abstract val cdylibName: Property<String>
    abstract val android: Property<Boolean>
    abstract val androidCleaner: Property<Boolean>
}
