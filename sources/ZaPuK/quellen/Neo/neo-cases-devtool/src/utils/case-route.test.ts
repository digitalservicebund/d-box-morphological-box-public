// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it } from 'vite-plus/test';
import { caseHash, caseIdFromHash } from './case-route';

it('round-trips Matrix room IDs through hash routes', () => {
  for (const id of ['!room:example.org', '!opaque-v12-room', '!special/#?%:example.org']) {
    expect(caseIdFromHash(caseHash(id))).toBe(id);
  }
  expect(caseHash(null)).toBe('');
});

it('ignores invalid, unrelated, and malformed hashes', () => {
  for (const hash of ['', '#other', '#/cases/', '#/cases/not-a-room', '#/cases/!bad%20room', '#/cases/%zz']) {
    expect(caseIdFromHash(hash)).toBeNull();
  }
});
