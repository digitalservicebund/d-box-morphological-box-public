// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.exceptions;

public class NeoConnectionException extends NeoException {
    public NeoConnectionException(String message) {
        super(message);
    }

    public NeoConnectionException(String message, Throwable cause) {
        super(message, cause);
    }
}
