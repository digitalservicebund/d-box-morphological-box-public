// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { describe, expect, it } from 'vitest';
import { MAX_FILE_BYTES, validateSelectedFile } from './fileValidation';

describe('validateSelectedFile', () => {
  it.each(['image/png', 'image/jpeg', 'application/pdf'])(
    'accepts %s at the size limit',
    (type) => {
      expect(validateSelectedFile({ type, size: MAX_FILE_BYTES })).toBeNull();
    },
  );

  it('rejects unsupported types before checking size', () => {
    expect(
      validateSelectedFile({ type: 'text/plain', size: MAX_FILE_BYTES + 1 }),
    ).toBe('type');
  });

  it('rejects files larger than the size limit', () => {
    expect(
      validateSelectedFile({ type: 'image/png', size: MAX_FILE_BYTES + 1 }),
    ).toBe('size');
  });
});
