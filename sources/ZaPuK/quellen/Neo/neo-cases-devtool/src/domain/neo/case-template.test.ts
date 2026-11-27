// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it } from 'vite-plus/test';
import { exportCaseTemplate, parseCaseTemplate } from './case-template';
import { keys, types } from './identifiers';
import type { Dialog, MatrixEvent } from './types';
import { relateBlock } from './serialize-envelope';

const block = (id: string, body: string): MatrixEvent => ({
  event_id: id,
  type: types.text,
  sender: '@agency:test',
  origin_server_ts: 1,
  content: {
    'm.text': [{ body }],
    [keys.length]: 1,
    'm.relates_to': { event_id: '$old-root', rel_type: 'm.thread' },
    [keys.relations]: [{ event_id: '$old-previous', rel_type: keys.chain }],
  },
});
const dialog = (id: string, timestamp: number): Dialog => ({
  id,
  timestamp,
  agency: { id, timestamp, sender: '@agency:test', blocks: [block(id, id === '$old' ? 'Earlier' : 'Later')] },
  responses: [
    { id: '$citizen', timestamp: 999, sender: '@citizen:test', blocks: [block('$citizen', 'Citizen-only content')] },
  ],
});
const metadata = { name: 'Test case', topic: 'Test agency', encrypted: true };

it('exports only agency contents and metadata in replay order, without old event IDs or relationships', () => {
  const exported = exportCaseTemplate(metadata, [dialog('$new', 20), dialog('$old', 10)]);
  expect(exported.case).toEqual(metadata);
  expect(exported.dialogs.map((entry) => entry.blocks[0]?.content['m.text'])).toEqual([
    [{ body: 'Earlier' }],
    [{ body: 'Later' }],
  ]);
  const json = JSON.stringify(exported);
  expect(json).not.toContain('$old');
  expect(json).not.toContain('$new');
  expect(json).not.toContain('@agency');
  expect(json).not.toContain('Citizen-only');
  expect(parseCaseTemplate(JSON.parse(json))).toEqual(exported);
});

it('removes stale relationships from modified imports so new Matrix IDs can be assigned', () => {
  const template = parseCaseTemplate({
    format: 'neo-agency-case',
    version: 1,
    case: metadata,
    dialogs: [{ blocks: [block('$one', 'First'), block('$two', 'Second')] }],
  });
  const first = template.dialogs[0]!.blocks[0]!;
  const second = template.dialogs[0]!.blocks[1]!;
  expect(first.content).not.toHaveProperty(keys.length);
  expect(relateBlock(first, 0, 2, []).content[keys.length]).toBe(1);
  expect(relateBlock(second, 1, 2, ['$new-root']).content['m.relates_to']).toEqual({
    event_id: '$new-root',
    rel_type: 'm.thread',
    'm.in_reply_to': { event_id: '$new-root' },
  });
});

it('rejects unsupported files, state events, invalid blocks, and multiple inquiries', () => {
  const template = { format: 'neo-agency-case', version: 1, case: metadata, dialogs: [] };
  expect(() => parseCaseTemplate({ ...template, version: 2 })).toThrow();
  expect(() => parseCaseTemplate({ ...template, dialogs: [{ blocks: [] }] })).toThrow();
  expect(() =>
    parseCaseTemplate({ ...template, dialogs: [{ blocks: [{ type: 'm.room.encryption', content: {} }] }] }),
  ).toThrow();
  const inquiry = { type: types.textInquiry, content: { 'm.text': [{ body: 'Question' }] } };
  expect(() => parseCaseTemplate({ ...template, dialogs: [{ blocks: [inquiry, inquiry] }] })).toThrow('one inquiry');
  expect(() =>
    parseCaseTemplate({ ...template, dialogs: [{ blocks: [{ type: types.button, content: inquiry.content }] }] }),
  ).toThrow();
});
