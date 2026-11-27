// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

export const errorMessage = (cause: unknown): string => (cause instanceof Error ? cause.message : String(cause));
