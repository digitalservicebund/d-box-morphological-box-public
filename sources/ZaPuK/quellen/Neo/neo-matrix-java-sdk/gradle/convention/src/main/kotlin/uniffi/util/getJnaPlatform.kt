// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package uniffi.util

fun getJnaPlatform(target: String): String {
    val splitTarget = target.split('-')
    val arch = when {
        splitTarget.contains("aarch64") -> "aarch64"
        splitTarget.contains("x86_64") -> "x86-64"
        else -> error("Unsupported target architecture $target")
    }
    val os = when {
        splitTarget.contains("apple") -> "darwin"
        splitTarget.contains("windows") -> "win32"
        splitTarget.contains("linux") -> "linux"
        else -> error("Unsupported target os $target")
    }
    return "$os-$arch"
}
