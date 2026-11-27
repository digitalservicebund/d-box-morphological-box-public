// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { describe, expect, it } from 'vitest';
import { formatDateTime } from './formatDateTime';

describe('formatDateTime', () => {
  const timestamp = BigInt(new Date(2026, 8, 24, 12, 34).getTime());

  it('formats the date using the selected language', () => {
    expect(formatDateTime(timestamp, 'de')).toContain('24.09.2026');
    expect(formatDateTime(timestamp, 'en')).toContain('09/24/2026');
  });
});
