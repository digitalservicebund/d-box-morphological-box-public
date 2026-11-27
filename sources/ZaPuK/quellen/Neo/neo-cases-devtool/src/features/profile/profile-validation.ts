// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { z } from 'zod';

export const avatarTypes = ['image/png', 'image/jpeg', 'image/webp', 'image/gif'];
export const profileInputSchema = z.compile(
  z.object({
    displayName: z.string().trim().max(200, 'Use a display name of at most 200 characters.'),
    avatar: z
      .instanceof(File)
      .refine((file) => avatarTypes.includes(file.type), 'Choose a PNG, JPEG, WebP, or GIF photo.')
      .refine((file) => file.size > 0 && file.size <= 10 * 1024 * 1024, 'The photo must be between 1 byte and 10 MB.')
      .optional(),
  }),
);
