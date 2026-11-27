// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.util

import de.fitko.fitconnect.matrix.client.exceptions.NeoLifecycleException
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.future.asCompletableFuture
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import java.io.Closeable
import java.util.concurrent.CompletableFuture

abstract class NeoLifecycleHolder<T> : Closeable {
    private val coroutineScope = CoroutineScope(Dispatchers.Default)

    private val mutex = Mutex()
    private var instance: T? = null

    protected abstract suspend fun create(): T
    protected abstract fun destroy(value: T)

    fun start(): CompletableFuture<Unit> = coroutineScope.async {
        if (instance == null) {
            mutex.withLock {
                if (instance == null) {
                    instance = create()
                }
            }
        }
    }.asCompletableFuture()

    fun getInstance(): T {
        return instance ?: throw NeoLifecycleException("Tried getting instance before it was started")
    }

    override fun close() {
        instance?.let {
            try {
                destroy(it)
            } catch (e: Exception) {
                throw NeoLifecycleException("Could not destroy instance", e)
            } finally {
                instance = null
            }
        }
    }
}
