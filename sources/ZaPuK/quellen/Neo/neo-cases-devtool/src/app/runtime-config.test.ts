// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { afterEach, expect, it, vi } from 'vite-plus/test';
import { loadRuntimeConfig } from './runtime-config';

afterEach(() => vi.unstubAllGlobals());

const respond = (body: unknown) => {
  const request = vi.fn().mockResolvedValue(new Response(JSON.stringify(body)));
  vi.stubGlobal('fetch', request);
  return request;
};

it('fetches runtime JSON without caching and preserves server order', async () => {
  const homeservers = ['https://second.example.org', 'http://localhost:8008'];
  const request = respond({ homeservers });
  await expect(loadRuntimeConfig('/devtool/config.json')).resolves.toEqual({ homeservers });
  expect(request).toHaveBeenCalledWith('/devtool/config.json', { cache: 'no-store' });
});

it.each([
  {},
  { homeserver: 'https://old.example.org' },
  { homeservers: [] },
  { homeservers: 'https://example.org' },
  { homeservers: ['not a URL'] },
  { homeservers: ['ftp://example.org'] },
  { homeservers: ['https://example.org', 'invalid'] },
])('rejects invalid runtime configuration: %j', async (body) => {
  respond(body);
  await expect(loadRuntimeConfig('/config.json')).rejects.toThrow();
});

it('reports a missing configuration instead of choosing a fallback server', async () => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue(new Response(null, { status: 404 })));
  await expect(loadRuntimeConfig('/config.json')).rejects.toThrow('Could not load config.json (HTTP 404).');
});

it('rejects malformed JSON', async () => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue(new Response('<html>SPA fallback</html>')));
  await expect(loadRuntimeConfig('/config.json')).rejects.toThrow();
});

it('propagates network failures', async () => {
  vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new Error('Network unavailable')));
  await expect(loadRuntimeConfig('/config.json')).rejects.toThrow('Network unavailable');
});
