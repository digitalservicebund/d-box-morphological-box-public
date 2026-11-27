// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it, vi } from 'vite-plus/test';
import { keys, types } from '../domain/neo/identifiers';
import { publishEnvelope, type SendJournal } from './send-envelope';

it('resumes a partial envelope with the same transaction IDs and correct previous/root relations', async () => {
  const journal: SendJournal = {
    account: '@agency:test',
    sessionDatabase: 'same-device',
    roomId: '!case:test',
    ids: [],
    transactions: ['txn-head', 'txn-tail'],
    blocks: [
      { type: types.text, content: { 'm.text': [{ body: 'A' }] } },
      { type: types.text, content: { 'm.text': [{ body: 'B' }] } },
    ],
  };
  const send = vi
    .fn()
    .mockResolvedValueOnce('$head')
    .mockRejectedValueOnce(new Error('Connection lost'))
    .mockResolvedValueOnce('$tail');
  const checkpoint = vi.fn().mockResolvedValue(undefined);
  await expect(publishEnvelope(send, journal, checkpoint)).rejects.toThrow('Connection lost');
  expect(journal.ids).toEqual(['$head']);
  await expect(publishEnvelope(send, journal, checkpoint)).resolves.toBe('$head');
  expect(send.mock.calls.map((call) => call[3])).toEqual(['txn-head', 'txn-tail', 'txn-tail']);
  expect(send.mock.calls[0]?.[2]).toMatchObject({ [keys.length]: 1 });
  expect(send.mock.calls[2]?.[2]).toMatchObject({
    'm.relates_to': { rel_type: 'm.thread', event_id: '$head', 'm.in_reply_to': { event_id: '$head' } },
    [keys.relations]: [
      { rel_type: keys.chain, event_id: '$head' },
      { rel_type: 'm.thread', event_id: '$head' },
    ],
  });
  expect(checkpoint).toHaveBeenCalledTimes(2);
});

it('can finish an acknowledged envelope without sending any event twice', async () => {
  const journal: SendJournal = {
    account: '@a:test',
    sessionDatabase: 'same-device',
    roomId: '!r:test',
    ids: ['$sent'],
    transactions: ['txn'],
    blocks: [{ type: types.text, content: {} }],
  };
  const send = vi.fn();
  expect(await publishEnvelope(send, journal, vi.fn())).toBe('$sent');
  expect(send).not.toHaveBeenCalled();
});
