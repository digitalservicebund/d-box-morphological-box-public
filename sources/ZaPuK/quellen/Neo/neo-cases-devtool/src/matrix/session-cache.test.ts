// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it } from 'vite-plus/test';
import { SessionCache } from './session-cache';

const original = {
  user_id: '@agency:test',
  device_id: 'device',
  access_token: 'initial-token',
  refresh_token: 'initial-refresh',
};

it('copies the live refreshed token instead of a stale login token', () => {
  const cache = new SessionCache();
  cache.update(original);
  expect(cache.accessToken()).toBe('initial-token');
  cache.update({ ...original, access_token: 'rotated-token', refresh_token: 'rotated-refresh' });
  expect(cache.accessToken()).toBe('rotated-token');
  expect(cache.snapshot().refresh_token).toBe('rotated-refresh');
});

it('does not expose mutable session state and clears access on logout', () => {
  const cache = new SessionCache();
  cache.update(original);
  const snapshot = cache.snapshot();
  snapshot.access_token = 'changed-by-caller';
  expect(cache.accessToken()).toBe('initial-token');
  cache.clear();
  expect(() => cache.accessToken()).toThrow('No active Matrix session');
});
