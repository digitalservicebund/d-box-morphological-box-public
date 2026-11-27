// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.config;

public record NeoConfig(
        AuthConfig auth,
        EncryptionConfig encryption,
        NeoEnvironment environment,
        String proxyUrl,
        boolean disableSslVerification
) {
    public static Builder builder() {
        return new Builder();
    }

    public static class Builder {
        private AuthConfig auth;
        private EncryptionConfig encryption;
        private NeoEnvironment environment = NeoEnvironment.DEV;
        private String proxyUrl;
        private boolean disableSslVerification;

        public Builder withAuth(AuthConfig auth) {
            this.auth = auth;
            return this;
        }

        public Builder withEncryption(EncryptionConfig encryption) {
            this.encryption = encryption;
            return this;
        }

        public Builder withEnvironment(NeoEnvironment environment) {
            this.environment = environment;
            return this;
        }

        public Builder withProxyUrl(String proxyUrl) {
            this.proxyUrl = proxyUrl;
            return this;
        }

        public Builder withDisableSslVerification(boolean disableSslVerification) {
            this.disableSslVerification = disableSslVerification;
            return this;
        }

        public NeoConfig build() {
            return new NeoConfig(auth, encryption, environment, proxyUrl, disableSslVerification);
        }
    }
}
