// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { z } from 'zod';

const homeserver = z.url().refine((value) => ['https:', 'http:'].includes(new URL(value).protocol));
const schema = z.compile(z.object({ homeservers: z.tuple([homeserver], homeserver) }));

/** Loads and validates the public runtime settings before the application starts. */
export const loadRuntimeConfig = async (url: string): Promise<z.infer<typeof schema>> => {
  const response = await fetch(url, { cache: 'no-store' });
  if (!response.ok) throw new Error(`Could not load config.json (HTTP ${response.status}).`);
  return schema.parse(await response.json());
};
