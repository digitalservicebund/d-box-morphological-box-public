// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

package de.fitko.fitconnect.matrix.client.exceptions;

public class NeoException extends RuntimeException {
    public NeoException(String errorMessage) {
        super(errorMessage);
    }

    public NeoException(String errorMessage, Throwable throwable) {
        super(errorMessage, throwable);
    }
}
