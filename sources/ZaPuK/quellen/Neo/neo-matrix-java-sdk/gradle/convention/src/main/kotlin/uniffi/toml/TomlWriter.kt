// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.toml

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.properties.Properties
import kotlinx.serialization.properties.encodeToMap
import java.io.File

internal object TomlWriter {
    @OptIn(ExperimentalSerializationApi::class)
    internal inline fun <reified T> encodeToFile(file: File, value: T) =
        encodeToFile(file, Properties.encodeToMap(value))

    @OptIn(ExperimentalSerializationApi::class)
    private fun encodeToFile(file: File, value: Map<String, Any?>) {
        file.bufferedWriter(Charsets.UTF_8).use { writer ->
            value.toSortedMap().forEach { (key, value) ->
                when (value) {
                    null -> Unit
                    is String -> writer.write("$key = \"$value\"\n")
                    else -> writer.write("$key = $value\n")
                }
            }
        }
    }
}
