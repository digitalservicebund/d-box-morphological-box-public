// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { afterEach, beforeEach, expect, it, vi } from 'vite-plus/test';
import { readServerPreference, writeServerPreference } from './server-preference';

const homeservers = ['https://first.example.org', 'https://second.example.org'] as const;
let values: Map<string, string>;

beforeEach(() => {
  values = new Map();
  vi.stubGlobal('localStorage', {
    getItem: (key: string) => values.get(key) ?? null,
    setItem: (key: string, value: string) => values.set(key, value),
  });
});
afterEach(() => vi.unstubAllGlobals());

it('defaults to the first server without a stored selection', () => {
  expect(readServerPreference(homeservers)).toBe(homeservers[0]);
});

it('remembers the last selection independently of login success', () => {
  writeServerPreference(homeservers[1]);
  expect(readServerPreference(homeservers)).toBe(homeservers[1]);
  writeServerPreference(homeservers[0]);
  expect(readServerPreference(homeservers)).toBe(homeservers[0]);
  expect(values.size).toBe(1);
});

it('uses the saved URL even after the configured servers are reordered', () => {
  writeServerPreference(homeservers[0]);
  expect(readServerPreference([homeservers[1], homeservers[0]])).toBe(homeservers[0]);
});

it('falls back when the saved server is removed or invalid', () => {
  for (const value of ['https://removed.example.org', 'invalid', '']) {
    writeServerPreference(value);
    expect(readServerPreference(homeservers)).toBe(homeservers[0]);
  }
});

it('supports configurations containing only one server', () => {
  expect(readServerPreference([homeservers[1]])).toBe(homeservers[1]);
});

it('does not block login when storage access is denied', () => {
  vi.stubGlobal('localStorage', {
    getItem: () => {
      throw new Error('Storage blocked');
    },
    setItem: () => {
      throw new Error('Storage blocked');
    },
  });
  expect(readServerPreference(homeservers)).toBe(homeservers[0]);
  expect(() => writeServerPreference(homeservers[1])).not.toThrow();
});
