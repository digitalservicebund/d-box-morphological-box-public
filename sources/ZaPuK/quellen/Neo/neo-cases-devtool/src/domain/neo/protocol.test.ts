// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { describe, expect, it } from 'vite-plus/test';
import { createBlock } from '../../features/dialogs/draft-reducer';
import { keys, types } from './identifiers';
import { parseDialogs } from './parse-dialogs';
import { relateBlock, serializeBlock, validateEnvelope } from './serialize-envelope';
import type { EventPayload, MatrixEvent } from './types';
import { validContent } from './validation';
import { mergeEvents } from './merge-events';

const agency = '@agency:example.org';
const citizen = '@citizen:example.org';
const event = (id: string, payload: EventPayload, sender = agency, timestamp = 100): MatrixEvent => ({
  event_id: id,
  type: payload.type,
  content: payload.content,
  sender,
  origin_server_ts: timestamp,
});
const message = (body = 'Hello'): EventPayload => ({ type: types.text, content: { 'm.text': [{ body }] } });
const envelope = (payloads: EventPayload[], prefix = 'a'): MatrixEvent[] => {
  const ids = payloads.map((_, index) => `$${prefix}${index}`);
  return payloads.map((payload, index) => event(ids[index]!, relateBlock(payload, index, ids.length, ids)));
};

describe('Neo multi-event integrity', () => {
  it('lets encrypted redactions replace decrypted content while ignoring stale ciphertext and history', () => {
    const decrypted = event('$encrypted', message());
    const ciphertext = event('$encrypted', { type: 'm.room.encrypted', content: {} });
    const map = new Map<string, MatrixEvent>();
    mergeEvents(map, [ciphertext, decrypted, ciphertext]);
    expect(map.get('$encrypted')?.type).toBe(types.text);
    mergeEvents(map, [{ ...ciphertext, unsigned: { redacted_because: {} } }, decrypted]);
    expect(map.get('$encrypted')?.unsigned?.redacted_because).toEqual({});
    expect(parseDialogs(map.values(), agency)).toEqual([]);
  });

  it('round-trips one block and an out-of-order chain using the declared following count', () => {
    const single = envelope([message()]);
    expect(single[0]?.content[keys.length]).toBeUndefined();
    const chain = envelope([message('first'), message('second'), message('third')]);
    expect(chain[0]?.content[keys.length]).toBe(2);
    expect(parseDialogs(chain.toReversed(), agency)[0]?.agency.blocks.map((block) => block.event_id)).toEqual([
      '$a0',
      '$a1',
      '$a2',
    ]);
    expect(parseDialogs(single, agency)).toHaveLength(1);
  });

  it('hides incomplete chains and rejects forks, nested heads, and invalid lengths', () => {
    const chain = envelope([message(), message()]);
    expect(parseDialogs(chain.slice(0, 1), agency)).toEqual([]);
    expect(parseDialogs([...chain, { ...chain[1]!, event_id: '$fork' }], agency)).toEqual([]);
    for (const length of [-1, 1.5, '2']) {
      expect(
        parseDialogs([{ ...chain[0]!, content: { ...chain[0]!.content, [keys.length]: length } }, chain[1]!], agency),
      ).toEqual([]);
    }
    expect(
      parseDialogs([chain[0]!, { ...chain[1]!, content: { ...chain[1]!.content, [keys.length]: 0 } }], agency),
    ).toEqual([]);
  });

  it('does not hide incomplete envelopes by silently removing a redacted block', () => {
    const chain = envelope([message(), message()]);
    const redaction = event('$r', { type: 'm.room.redaction', content: { redacts: '$a1' } });
    expect(parseDialogs([...chain, redaction], agency)).toEqual([]);
    expect(parseDialogs([{ ...chain[0]!, unsigned: { redacted_because: {} } }, chain[1]!], agency)).toEqual([]);
  });

  it('keeps replies inside a card without changing its original sorting timestamp', () => {
    const root = event(
      '$inquiry',
      { type: types.textInquiry, content: { 'm.text': [{ body: 'Question' }] } },
      agency,
      100,
    );
    const reply = event(
      '$reply',
      {
        ...message('Answer'),
        content: {
          ...message('Answer').content,
          'm.relates_to': {
            rel_type: 'm.thread',
            event_id: root.event_id,
            'm.in_reply_to': { event_id: root.event_id },
          },
        },
      },
      citizen,
      300,
    );
    const other = event('$newer', message(), agency, 200);
    const dialogs = parseDialogs([root, reply, other], agency, citizen);
    expect(dialogs.map((dialog) => dialog.id)).toEqual(['$newer', '$inquiry']);
    expect(dialogs[1]?.responses[0]?.blocks[0]?.event_id).toBe('$reply');
    expect(parseDialogs([root, { ...reply, sender: '@outsider:example.org' }], agency, citizen)[0]?.responses).toEqual(
      [],
    );
  });
});

describe('building-block contract', () => {
  it('renders unknown extensible event types using their text fallback, without treating them as inquiries', () => {
    const unknown = event('$future', { type: 'dev.fitko.neo.future.v2', content: message('Fallback').content });
    expect(parseDialogs([unknown], agency)[0]?.agency.blocks[0]?.event_id).toBe('$future');
  });

  it('uses spec inquiry identifiers, fallbacks, and exactly one inquiry', () => {
    const text = { ...createBlock('textInquiry'), text: 'Please explain', title: 'Explanation' };
    const file = { ...createBlock('fileInquiry'), text: 'Please upload', title: 'Evidence' };
    expect(serializeBlock(text).type).toBe('dev.fitko.neo.inquiry.message.v1');
    expect(serializeBlock(file).content[types.fileInquiry]).toBeDefined();
    expect(() => validateEnvelope([text, file])).toThrow('only one inquiry');
    expect(validContent(event('$file-inquiry', serializeBlock(file)))).toBe(true);
  });

  it('includes the mandatory custom poll answer in multi-select limits', () => {
    const block = {
      ...createBlock('pollInquiry'),
      text: 'Which?',
      multiple: true,
      answers: [
        { id: 'one', text: 'One' },
        { id: 'two', text: 'Two' },
      ],
    };
    const payload = serializeBlock(block);
    const poll = payload.content[keys.poll] as { answers: { 'm.id': string }[]; max_selections: number };
    expect(poll.max_selections).toBe(3);
    expect(poll.answers.at(-1)?.['m.id']).toBe(keys.customAnswer);
    expect(validContent(event('$poll', payload))).toBe(true);
    poll.answers.pop();
    expect(validContent(event('$invalid', payload))).toBe(false);
  });

  it('resolves legacy poll responses through m.reference into their parent dialog', () => {
    const poll = serializeBlock({
      ...createBlock('pollInquiry'),
      text: 'Which?',
      answers: [{ id: 'one', text: 'One' }],
    });
    const chain = envelope([message(), poll]);
    const response = event(
      '$response',
      {
        type: types.pollResponse,
        content: {
          [keys.selections]: ['one'],
          'm.relates_to': { rel_type: 'm.reference', event_id: '$a1', 'm.in_reply_to': { event_id: '$a1' } },
        },
      },
      citizen,
    );
    expect(parseDialogs([...chain, response], agency, citizen)[0]?.responses).toHaveLength(1);
    response.content[keys.selections] = [keys.customAnswer];
    expect(parseDialogs([...chain, response], agency, citizen)[0]?.responses).toEqual([]);
  });

  it('never transmits a local filename and preserves the encrypted media descriptor', () => {
    const block = {
      ...createBlock('file'),
      file: new File(['test'], 'private-citizen-name.pdf', { type: 'application/pdf' }),
    };
    const descriptor = {
      url: 'mxc://example.org/123',
      key: { k: 'test' },
      iv: 'iv',
      hashes: { sha256: 'hash' },
      v: 'v2',
    };
    const payload = serializeBlock(block, descriptor);
    const content = payload.content[types.file] as Record<string, unknown>;
    expect(content.name).toBe('attachment.pdf');
    expect(content[keys.encryptedFile]).toEqual({ key: descriptor.key, iv: 'iv', hashes: descriptor.hashes, v: 'v2' });
    expect(JSON.stringify(payload)).not.toContain('private-citizen-name');
  });
});
