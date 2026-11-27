// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it, vi } from 'vite-plus/test';
import { exposeRelations, installRelationTransport } from './relation-transport';

it('exposes scalable relations without altering SDK ciphertext or the legacy relation', () => {
  const body = { algorithm: 'm.megolm.v1.aes-sha2', ciphertext: 'opaque', 'm.relates_to': { event_id: '$root' } };
  const relations = [{ event_id: '$root', rel_type: 'm.thread' }];
  expect(exposeRelations(body, relations)).toEqual({ ...body, 'im.nheko.relations.v1.relations': relations });
  expect(body).not.toHaveProperty('im.nheko.relations.v1.relations');
});

it('only amends explicitly registered encrypted send requests, including URL-escaped room IDs', async () => {
  const original = globalThis.fetch;
  const fetch = vi.fn().mockResolvedValue(new Response('{}'));
  globalThis.fetch = fetch;
  try {
    const pending = installRelationTransport();
    pending.set('https://example.org/_matrix/client/v3/rooms/!room:example.org/send/m.room.encrypted/txn', [
      { event_id: '$root' },
    ]);
    await globalThis.fetch(
      'https://example.org/_matrix/client/v3/rooms/%21room%3Aexample.org/send/m.room.encrypted/txn',
      {
        method: 'PUT',
        body: JSON.stringify({ algorithm: 'm.megolm.v1.aes-sha2', ciphertext: 'opaque' }),
      },
    );
    const sent = fetch.mock.calls[0]?.[0] as Request;
    expect(await sent.json()).toHaveProperty('im.nheko.relations.v1.relations');
    await globalThis.fetch(
      'https://other.example/_matrix/client/v3/rooms/%21room%3Aexample.org/send/m.room.encrypted/txn',
      {
        method: 'PUT',
        body: JSON.stringify({ algorithm: 'm.megolm.v1.aes-sha2', ciphertext: 'opaque' }),
      },
    );
    const unrelated = fetch.mock.calls[1]?.[0] as Request;
    expect(await unrelated.json()).not.toHaveProperty('im.nheko.relations.v1.relations');
  } finally {
    globalThis.fetch = original;
  }
});
