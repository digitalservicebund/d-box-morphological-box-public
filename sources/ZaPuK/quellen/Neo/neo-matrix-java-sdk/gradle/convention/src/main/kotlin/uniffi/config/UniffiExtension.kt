// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.config

import org.gradle.api.NamedDomainObjectContainer
import org.gradle.api.provider.ListProperty
import org.gradle.api.provider.Property

interface UniffiExtension {
    val workspace: Property<String>

    val packageId: Property<String>
    val release: Property<Boolean>
    val features: ListProperty<String>
    val targets: ListProperty<String>

    val crates: NamedDomainObjectContainer<UniffiCrateConfig>
}
