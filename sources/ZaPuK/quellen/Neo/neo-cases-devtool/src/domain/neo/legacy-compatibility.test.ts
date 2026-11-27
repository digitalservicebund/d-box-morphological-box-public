// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it } from 'vite-plus/test';
import { createBlock } from '../../features/dialogs/draft-reducer';
import { exportCaseTemplate, parseCaseTemplate } from './case-template';
import { isInquiry, keys, legacyTypes, types } from './identifiers';
import { canonicalPayload, legacyPayloads } from './legacy-compatibility';
import { parseDialogs } from './parse-dialogs';
import { relateBlock, serializeBlock } from './serialize-envelope';
import type { MatrixEvent } from './types';
import { validContent } from './validation';

it.each(['textInquiry', 'fileInquiry', 'pollInquiry'] as const)(
  'adapts %s without mutating canonical content',
  (kind) => {
    const original = serializeBlock({
      ...createBlock(kind),
      text: 'Please respond',
      title: 'Response title',
      label: 'Response label',
      answers: [{ id: 'yes', text: 'Yes' }],
    });
    const before = structuredClone(original);
    const adapted = legacyPayloads([original]);
    const inquiry = adapted.at(-1)!;
    expect(inquiry.type).toBe(legacyTypes[kind]);
    expect(isInquiry(inquiry.type)).toBe(true);
    expect(validContent(inquiry)).toBe(true);
    expect(inquiry.content[types.textInquiry]).toBeUndefined();
    expect(inquiry.content[legacyTypes.textInquiry]).toMatchObject({
      [keys.label]: { 'm.text': [{ body: 'Response label' }] },
    });
    if (kind !== 'pollInquiry') {
      expect(adapted).toHaveLength(2);
      expect(adapted[0]).toEqual({ type: types.text, content: { 'm.text': original.content['m.text'] } });
      expect(inquiry.content[legacyTypes.textInquiry]).toMatchObject({
        [keys.title]: { 'm.text': [{ body: 'Response title' }] },
      });
    } else expect(adapted).toHaveLength(1);
    expect(canonicalPayload(inquiry).type).toBe(types[kind]);
    expect(original).toEqual(before);
  },
);

it('provides required empty SDK infobox fields and preserves supplied fields', () => {
  const plain = serializeBlock({ ...createBlock('infobox'), text: 'Notice' });
  const adapted = legacyPayloads([plain])[0]!;
  expect(plain.content[keys.title]).toBeUndefined();
  expect(plain.content[keys.hyperlink]).toBeUndefined();
  expect(adapted.content[keys.hyperlink]).toEqual({
    [keys.label]: { 'm.text': [{ body: '', mimetype: 'text/plain' }] },
    url: '',
  });
  expect(validContent(adapted)).toBe(true);
  expect(validContent({ ...adapted, content: { ...adapted.content, [keys.hyperlink]: { url: '' } } })).toBe(false);
  const complete = serializeBlock({
    ...createBlock('infobox'),
    text: 'Notice',
    title: 'Title',
    label: 'Read more',
    url: 'https://example.org',
  });
  expect(legacyPayloads([complete])).toEqual([complete]);
});

it.each(['textInquiry', 'fileInquiry', 'pollInquiry'] as const)(
  'matches responses to legacy %s while retaining wire events and templates',
  (kind) => {
    const payloads = legacyPayloads([
      serializeBlock({ ...createBlock(kind), text: 'Question', answers: [{ id: 'yes', text: 'Yes' }] }),
    ]);
    const ids = payloads.map((_, index) => `$block${index}`);
    const events: MatrixEvent[] = payloads.map((payload, index) => ({
      ...relateBlock(payload, index, payloads.length, ids),
      event_id: ids[index]!,
      sender: '@agency:test',
      origin_server_ts: 1,
    }));
    const inquiry = events.at(-1)!;
    const response =
      kind === 'fileInquiry'
        ? {
            type: types.file,
            content: { 'm.text': [{ body: 'Attachment' }], [types.file]: { uri: 'mxc://test/file', name: 'file.pdf' } },
          }
        : kind === 'pollInquiry'
          ? { type: types.pollResponse, content: { [keys.selections]: ['yes'] } }
          : { type: types.text, content: { 'm.text': [{ body: 'Answer' }] } };
    events.push({
      ...response,
      event_id: '$reply',
      sender: '@citizen:test',
      origin_server_ts: 2,
      content: {
        ...response.content,
        [keys.relations]: [
          { rel_type: 'm.thread', event_id: ids[0] },
          { rel_type: keys.chain, event_id: inquiry.event_id },
          { rel_type: keys.pollRelation, event_id: inquiry.event_id },
        ],
      },
    });
    const dialogs = parseDialogs(events, '@agency:test', '@citizen:test');
    expect(dialogs[0]?.agency.blocks.at(-1)).toBe(inquiry);
    expect(dialogs[0]?.responses[0]?.blocks[0]?.event_id).toBe('$reply');
    const template = exportCaseTemplate({ name: 'Case', topic: '', encrypted: false }, dialogs);
    expect(parseCaseTemplate(template).dialogs[0]?.blocks).toEqual(payloads);
  },
);

it('validates legacy poll content instead of accepting an arbitrary text fallback', () => {
  expect(validContent({ type: legacyTypes.pollInquiry, content: { 'm.text': [{ body: 'Invalid poll' }] } })).toBe(
    false,
  );
});
