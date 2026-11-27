// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.exceptions;

public class NeoAuthenticationException extends NeoException {
    public NeoAuthenticationException(String message) {
        super(message);
    }

    public NeoAuthenticationException(String message, Throwable cause) {
        super(message, cause);
    }
}
