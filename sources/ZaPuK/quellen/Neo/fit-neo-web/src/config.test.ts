// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

describe('config', () => {
  beforeEach(() => {
    vi.resetModules();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('getConfig throws before loadConfig is called', async () => {
    const { getConfig } = await import('./config.ts');
    expect(() => getConfig()).toThrow('getConfig() called before loadConfig()');
  });

  it('loadConfig fetches /config.json and getConfig returns it', async () => {
    const mockConfig = { matrixHomeserver: 'https://matrix.example.com' };
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue({
        ok: true,
        json: () => Promise.resolve(mockConfig),
      }),
    );

    const { loadConfig, getConfig } = await import('./config.ts');
    await loadConfig();

    expect(getConfig()).toEqual(mockConfig);
    expect(fetch).toHaveBeenCalledWith('/config.json');
  });

  it('loadConfig throws when the server returns an error response', async () => {
    vi.stubGlobal(
      'fetch',
      vi
        .fn()
        .mockResolvedValue({ ok: false, status: 404, statusText: 'Not Found' }),
    );

    const { loadConfig } = await import('./config.ts');
    await expect(loadConfig()).rejects.toThrow(
      'Failed to load config.json: 404 Not Found',
    );
  });
});
