// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.exceptions;

public class NeoLifecycleException extends NeoException {
    public NeoLifecycleException(String message) {
        super(message);
    }

    public NeoLifecycleException(String message, Throwable cause) {
        super(message, cause);
    }
}
