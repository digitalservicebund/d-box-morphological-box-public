// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.api

data class UploadHandle(
    var url: String?,
    var current: Long,
    var total: Long,
)
