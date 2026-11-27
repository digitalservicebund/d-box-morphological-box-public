// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.di;

import com.google.inject.AbstractModule;
import com.google.inject.Provides;
import com.google.inject.Singleton;
import de.fitko.fitconnect.matrix.client.config.NeoConfig;
import de.fitko.fitconnect.matrix.client.services.NeoClientService;
import de.fitko.fitconnect.matrix.client.services.NeoRoomListService;
import de.fitko.fitconnect.matrix.client.services.NeoSyncService;

public class ClientModule extends AbstractModule {
    private final NeoConfig config;

    public ClientModule(NeoConfig config) {
        this.config = config;
    }

    @Provides
    @Singleton
    NeoClientService provideClientService() {
        return new NeoClientService(config);
    }

    @Provides
    @Singleton
    NeoSyncService provideSyncService(NeoClientService clientService) {
        return new NeoSyncService(clientService);
    }

    @Provides
    @Singleton
    NeoRoomListService provideRoomListService(NeoSyncService syncService) {
        return new NeoRoomListService(syncService);
    }
}
