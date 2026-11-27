// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package dev.fitko.neo.matrix.sample

import dev.fitko.neo.matrix.sdk.Client
import dev.fitko.neo.matrix.sdk.ClientBuilder
import kotlinx.coroutines.test.runTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs

class MatrixTest {
    @Test
    fun `test native library loads`() = runTest {
        val client = ClientBuilder()
            .serverName("matrix.org")
            .build()

        assertIs<Client>(client)
        assertEquals(
            "https://matrix-client.matrix.org/",
            client.homeserverLoginDetails().url()
        )
    }
}
