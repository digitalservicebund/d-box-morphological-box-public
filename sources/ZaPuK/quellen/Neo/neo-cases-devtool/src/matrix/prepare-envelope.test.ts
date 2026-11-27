// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it, vi } from 'vite-plus/test';
import { createBlock } from '../features/dialogs/draft-reducer';
import { keys, legacyTypes, types } from '../domain/neo/identifiers';
import { prepareEnvelope, publishEnvelope, restoreSend } from './send-envelope';
import { readStored, writeStored } from './storage';
import type { MatrixService } from './matrix-service';

vi.mock('./storage', () => ({
  readSession: vi.fn(async () => ({ profile: { mxid: '@agency:test' }, database: 'device' })),
  readStored: vi.fn(),
  writeStored: vi.fn(),
}));

it('defaults to spec payloads and freezes expanded legacy payloads through reload and retry', async () => {
  const draft = [{ ...createBlock('fileInquiry'), text: 'Upload the document' }];
  const service = { upload: vi.fn() } as unknown as MatrixService;
  const canonical = await prepareEnvelope(service, '@agency:test', '!room:test', draft);
  expect(canonical.legacyCompatibility).toBe(false);
  expect(canonical.blocks.map((block) => block.type)).toEqual([types.fileInquiry]);
  const journal = await prepareEnvelope(service, '@agency:test', '!room:test', draft, true);
  expect(journal.legacyCompatibility).toBe(true);
  expect(journal.blocks.map((block) => block.type)).toEqual([types.text, legacyTypes.fileInquiry]);
  expect(journal.transactions).toHaveLength(2);
  expect(writeStored).toHaveBeenLastCalledWith('send:@agency:test:!room:test', journal);
  const send = vi
    .fn()
    .mockResolvedValueOnce('$prompt')
    .mockRejectedValueOnce(new Error('Offline'))
    .mockResolvedValueOnce('$inquiry');
  await expect(publishEnvelope(send, journal, vi.fn())).rejects.toThrow('Offline');
  vi.mocked(readStored).mockResolvedValue(structuredClone(journal));
  const restored = (await restoreSend('@agency:test', '!room:test'))!;
  expect(restored.legacyCompatibility).toBe(true);
  await publishEnvelope(send, restored, vi.fn());
  expect(send.mock.calls[0]?.[2]).toMatchObject({ [keys.length]: 1 });
  expect(send.mock.calls[1]).toEqual(send.mock.calls[2]);
  expect(service.upload).not.toHaveBeenCalled();
  vi.mocked(readStored).mockResolvedValue({ ...canonical, legacyCompatibility: undefined });
  expect((await restoreSend('@agency:test', '!room:test'))?.blocks).toEqual(canonical.blocks);
});
