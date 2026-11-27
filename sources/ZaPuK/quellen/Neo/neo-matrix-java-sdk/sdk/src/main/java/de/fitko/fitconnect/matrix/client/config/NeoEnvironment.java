// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.config;

public record NeoEnvironment(
        String name,
        String apiUrl
) {
    public static final NeoEnvironment DEV = new NeoEnvironment(
            "DEV",
            "https://synapse-ess1.dev.neo.fitko.dev"
    );

    public static final NeoEnvironment DEMO = new NeoEnvironment(
            "DEMO",
            "https://synapse-ess1.demo.neo.fitko.dev"
    );

    public static NeoEnvironment custom(
            String name,
            String authBaseUrl
    ) {
        return new NeoEnvironment(name, authBaseUrl);
    }

    public boolean isDev() {
        return this == DEV;
    }

    public boolean isDemo() {
        return this == DEMO;
    }

    public boolean isCustom() {
        return !(isDev() || isDemo());
    }
}
