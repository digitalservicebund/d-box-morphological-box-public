// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it } from 'vite-plus/test';
import { isAppDatabase } from './app-databases';

it('includes app metadata and all SDK stores, including orphaned sessions', () => {
  expect(isAppDatabase('neo-cases-devtool')).toBe(true);
  expect(isAppDatabase('neo-cases-devtool-old-session::crypto')).toBe(true);
  expect(isAppDatabase('neo-cases-devtool-new-session::state')).toBe(true);
});

it('does not clear unrelated databases on the same origin', () => {
  for (const name of ['other-app', 'matrix-store', 'neo-cases-devtoollication', 'prefix-neo-cases-devtool-test']) {
    expect(isAppDatabase(name)).toBe(false);
  }
});
