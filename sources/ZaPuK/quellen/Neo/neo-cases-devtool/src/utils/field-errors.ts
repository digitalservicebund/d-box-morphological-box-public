// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { ZodError } from 'zod';

/** Maps schema validation issues to the form fields that display them. */
export const fieldErrors = (error: ZodError): Record<string, string> =>
  Object.fromEntries(error.issues.map((issue) => [String(issue.path[0]), issue.message]));
