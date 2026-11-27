// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.config;

public record EncryptionConfig(String recoveryKey) {
    public boolean isValid() {
        return recoveryKey != null && !recoveryKey.isEmpty();
    }
}
