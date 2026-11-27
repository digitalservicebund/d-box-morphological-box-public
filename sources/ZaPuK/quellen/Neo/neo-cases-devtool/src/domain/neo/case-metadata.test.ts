// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it } from 'vite-plus/test';
import { caseInputSchema, caseMetadataSchema } from './validation';

it('trims case metadata and applies the same rules to edits and case creation', () => {
  expect(caseMetadataSchema.parse({ name: ' New title ', topic: ' Agency ' })).toEqual({
    name: 'New title',
    topic: 'Agency',
  });
  const examples = [
    { name: '', topic: 'Agency' },
    { name: 'Title', topic: '  ' },
    { name: 'a'.repeat(201), topic: 'Agency' },
    { name: 'Title', topic: 'a'.repeat(501) },
    { name: 'a'.repeat(200), topic: 'a'.repeat(500) },
  ];
  for (const [index, input] of examples.entries()) {
    const expected = index === examples.length - 1;
    expect(caseMetadataSchema.safeParse(input).success).toBe(expected);
    expect(caseInputSchema.safeParse({ ...input, recipient: '@citizen:example.org', encrypted: true }).success).toBe(
      expected,
    );
  }
});
