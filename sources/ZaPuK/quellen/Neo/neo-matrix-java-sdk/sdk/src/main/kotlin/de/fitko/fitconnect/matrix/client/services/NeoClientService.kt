// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.services

import com.google.inject.Inject
import de.fitko.fitconnect.matrix.client.config.NeoConfig
import de.fitko.fitconnect.matrix.client.exceptions.NeoAuthenticationException
import de.fitko.fitconnect.matrix.client.exceptions.NeoConnectionException
import de.fitko.fitconnect.matrix.client.util.NeoLifecycleHolder
import dev.fitko.neo.matrix.sdk.Client
import dev.fitko.neo.matrix.sdk.ClientBuilder
import dev.fitko.neo.matrix.sdk.SlidingSyncVersionBuilder
import dev.fitko.neo.matrix.sdk.internal.crypto.DecryptionSettings

class NeoClientService @Inject constructor(
    private val neoConfig: NeoConfig
) : NeoLifecycleHolder<Client>() {
    override suspend fun create(): Client {
        val client = try {
            ClientBuilder()
                .serverName(neoConfig.environment().apiUrl())
                .slidingSyncVersionBuilder(SlidingSyncVersionBuilder.DISCOVER_NATIVE)
                .apply {
                    neoConfig.proxyUrl?.let { proxyUrl ->
                        proxy(proxyUrl)
                    }
                }
                .apply {
                    if (neoConfig.disableSslVerification) {
                        disableSslVerification()
                    }
                }
                .build()
        } catch (e: Exception) {
            throw NeoConnectionException("Could not connect to Matrix homeserver: ${neoConfig.environment().apiUrl()}", e)
        }

        try {
            client.login(
                username = neoConfig.auth().clientId(),
                password = neoConfig.auth().clientSecret(),
                initialDeviceName = "Neo SDK",
                deviceId = "neo-sdk"
            )
        } catch (e: Exception) {
            throw NeoAuthenticationException("Failed to authenticate with Matrix homeserver", e)
        }

        return client
    }

    override fun destroy(value: Client) {
        value.destroy()
    }
}
