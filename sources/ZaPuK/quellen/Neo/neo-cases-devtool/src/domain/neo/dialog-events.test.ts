// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it } from 'vite-plus/test';
import { dialogEvents } from './dialog-events';
import { parseEvent } from './validation';
import type { Dialog, MatrixEvent } from './types';
import { types } from './identifiers';

const event = (id: string): MatrixEvent => ({
  event_id: id,
  sender: '@agency:test',
  origin_server_ts: 1,
  type: types.text,
  content: { 'm.text': [{ body: id }] },
});

it('retains the full SDK event JSON, including room IDs and extension metadata', () => {
  const raw = { ...event('$one'), room_id: '!room:test', 'example.metadata': { test: true }, unsigned: { age: 100 } };
  expect(parseEvent(raw)).toEqual(raw);
});

it('collects agency and citizen events once per event ID, in their displayed envelope order', () => {
  const first = event('$first');
  const second = event('$second');
  const answer = { ...event('$answer'), sender: '@citizen:test' };
  const dialog: Dialog = {
    id: first.event_id,
    timestamp: 1,
    agency: { id: first.event_id, sender: first.sender, timestamp: 1, blocks: [first, second] },
    responses: [{ id: answer.event_id, sender: answer.sender, timestamp: 2, blocks: [answer, answer] }],
  };
  expect(dialogEvents(dialog).map((entry) => entry.event_id)).toEqual(['$first', '$second', '$answer']);
  expect(dialogEvents(dialog)[2]?.content).toEqual(answer.content);
});
