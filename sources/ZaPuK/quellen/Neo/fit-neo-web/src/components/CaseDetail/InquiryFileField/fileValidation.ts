// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

export const MAX_FILE_BYTES = 10 * 1024 * 1024;
export const ACCEPTED_MIME_TYPES = [
  'image/png',
  'image/jpeg',
  'application/pdf',
];

export function validateSelectedFile(
  file: Pick<File, 'type' | 'size'>,
): 'type' | 'size' | null {
  if (!ACCEPTED_MIME_TYPES.includes(file.type)) return 'type';
  if (file.size > MAX_FILE_BYTES) return 'size';
  return null;
}
