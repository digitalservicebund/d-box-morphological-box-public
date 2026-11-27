// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client;

import com.google.inject.Guice;
import com.google.inject.Injector;
import com.google.inject.Module;
import de.fitko.fitconnect.matrix.client.clients.NeoSenderClient;
import de.fitko.fitconnect.matrix.client.config.AuthConfig;
import de.fitko.fitconnect.matrix.client.config.EncryptionConfig;
import de.fitko.fitconnect.matrix.client.config.NeoConfig;
import de.fitko.fitconnect.matrix.client.config.NeoEnvironment;
import de.fitko.fitconnect.matrix.client.di.ClientModule;
import de.fitko.fitconnect.matrix.client.exceptions.NeoConfigurationException;
import de.fitko.fitconnect.matrix.client.services.NeoClientService;
import de.fitko.fitconnect.matrix.client.services.NeoRoomListService;
import de.fitko.fitconnect.matrix.client.services.NeoSyncService;

import java.util.ArrayList;
import java.util.List;

import static com.google.common.base.Strings.isNullOrEmpty;

public class NeoSdk {
    private final Injector injector;
    private final NeoConfig neoConfig;

    private NeoSdk(NeoConfig neoConfig) {
        this.neoConfig = neoConfig;
        List<com.google.inject.Module> modules = assembleModules(neoConfig);
        this.injector = Guice.createInjector(modules);
    }

    private NeoSdk(List<com.google.inject.Module> modules, NeoConfig neoConfig) {
        this.neoConfig = neoConfig;
        this.injector = Guice.createInjector(modules);
    }

    public static NeoSdk fromConfig(NeoConfig neoConfig) {
        return new NeoSdk(neoConfig);
    }

    public static NeoSdk.Builder fromConfigBuilder() {
        return new Builder();
    }

    public NeoClientService getLifecycleHandler() {
        if (neoConfig.auth() == null) {
            throw new NeoConfigurationException("Credentials not configured. Use Builder.sender() to set credentials.");
        }
        return injector.getInstance(NeoClientService.class);
    }

    public NeoSyncService getSyncService() {
        if (neoConfig.auth() == null) {
            throw new NeoConfigurationException("Credentials not configured. Use Builder.sender() to set credentials.");
        }
        return injector.getInstance(NeoSyncService.class);
    }

    public NeoRoomListService getRoomListService() {
        if (neoConfig.auth() == null) {
            throw new NeoConfigurationException("Credentials not configured. Use Builder.sender() to set credentials.");
        }
        return injector.getInstance(NeoRoomListService.class);
    }

    public NeoSenderClient createSenderClient() {
        if (neoConfig.auth() == null) {
            throw new NeoConfigurationException("Credentials not configured. Use Builder.sender() to set credentials.");
        }
        return injector.getInstance(NeoSenderClient.class);
    }

    private static List<com.google.inject.Module> assembleModules(NeoConfig config) {
        List<com.google.inject.Module> modules = new ArrayList<>();

        if (config.auth() != null) {
            modules.add(new ClientModule(config));
        }

        return modules;
    }

    public static class Builder {
        private AuthConfig auth;
        private EncryptionConfig encryption;
        private NeoEnvironment environment;
        private String proxyUrl;
        private boolean disableSslVerification;

        private Builder() {
        }

        public Builder auth(String clientId, String clientSecret) {
            if (isNullOrEmpty(clientId) || isNullOrEmpty(clientSecret)) {
                throw new NeoConfigurationException("Sender credentials must not be null or empty.");
            }
            this.auth = new AuthConfig(clientId, clientSecret);
            return this;
        }

        public Builder encryption(String recoveryKey) {
            if (isNullOrEmpty(recoveryKey)) {
                throw new NeoConfigurationException("Encryption key must not be null or empty.");
            }
            this.encryption = new EncryptionConfig(recoveryKey);
            return this;
        }

        public Builder environment(NeoEnvironment env) {
            if (env == null) {
                throw new NeoConfigurationException("Environment must not be null.");
            }
            this.environment = env;
            return this;
        }

        public Builder proxy(String proxyUrl) {
            this.proxyUrl = proxyUrl;
            return this;
        }

        public Builder disableSslVerification(boolean disableSslVerification) {
            this.disableSslVerification = disableSslVerification;
            return this;
        }

        public NeoSdk build() {
            return buildSdk();
        }

        private NeoSdk buildSdk() {
            if (environment == null) {
                throw new NeoConfigurationException("Missing environment configuration.");
            }

            NeoConfig neoConfig = NeoConfig.builder()
                    .withAuth(auth)
                    .withEncryption(encryption)
                    .withEnvironment(environment)
                    .withProxyUrl(proxyUrl)
                    .withDisableSslVerification(disableSslVerification)
                    .build();

            List<Module> modules = assembleModules(neoConfig);
            return new NeoSdk(modules, neoConfig);
        }
    }
}
