// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it } from 'vite-plus/test';
import { profileSchema } from './types';

it('restores sessions saved before avatar metadata was added', () => {
  expect(profileSchema.parse({ mxid: '@agency:test', displayName: 'Agency' })).toEqual({
    mxid: '@agency:test',
    displayName: 'Agency',
    avatarUrl: null,
    canSetDisplayName: true,
    canSetAvatar: true,
  });
});
