// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it, vi } from 'vite-plus/test';
import { importCaseDialogs, type ImportProgress } from './import-case';
import { copyTemplateMedia } from './import-media';
import type { MatrixService } from './matrix-service';
import type { CaseTemplate } from '../domain/neo/case-template';
import { keys, types } from '../domain/neo/identifiers';

vi.mock('./storage', () => ({
  readSession: vi.fn(async () => ({ database: 'same-session', profile: { mxid: '@agency:test' } })),
  writeStored: vi.fn(async () => undefined),
  readStored: vi.fn(async () => undefined),
}));

it('retries only the unfinished portion of a multi-dialog case import', async () => {
  const message = { type: types.text, content: { 'm.text': [{ body: 'Test' }] } };
  const template: CaseTemplate = {
    format: 'neo-agency-case',
    version: 1,
    case: { name: 'Import', topic: '', encrypted: false },
    dialogs: [{ blocks: [message] }, { blocks: [message, message] }],
  };
  const send = vi
    .fn<MatrixService['send']>()
    .mockResolvedValueOnce('$first')
    .mockResolvedValueOnce('$second')
    .mockRejectedValueOnce(new Error('Interrupted'))
    .mockResolvedValueOnce('$tail');
  const service = { send, upload: vi.fn<MatrixService['upload']>(), download: vi.fn<MatrixService['download']>() };
  const progress: ImportProgress = { completed: 0 };
  await expect(importCaseDialogs(service, '@agency:test', '!new', template, progress, vi.fn())).rejects.toThrow(
    'Interrupted',
  );
  expect(progress.completed).toBe(1);
  expect(progress.pending?.ids).toEqual(['$second']);
  await importCaseDialogs(service, '@agency:test', '!new', template, progress, vi.fn());
  expect(progress.completed).toBe(2);
  expect(progress.pending).toBeUndefined();
  expect(send).toHaveBeenCalledTimes(4);
  expect(send.mock.calls[2]?.[3]).toBe(send.mock.calls[3]?.[3]);
  expect(send.mock.calls[3]?.[2]['m.relates_to']).toMatchObject({ event_id: '$second' });
});

it('copies attachments and replaces their encryption metadata for the destination room', async () => {
  const previous = { key: { k: 'old-key' }, iv: 'old-iv', v: 'v2' };
  const payload = {
    type: types.file,
    content: {
      'm.text': [{ body: 'File' }],
      [types.file]: {
        uri: 'mxc://old/file',
        name: 'attachment.txt',
        mimetype: 'text/plain',
        [keys.encryptedFile]: previous,
      },
    },
  };
  const bytes = new Uint8Array([1, 2, 3]);
  const service = {
    download: vi.fn<MatrixService['download']>().mockResolvedValue(bytes),
    upload: vi
      .fn<MatrixService['upload']>()
      .mockResolvedValueOnce({ url: 'mxc://new/plain' })
      .mockResolvedValueOnce({ url: 'mxc://new/encrypted', key: { k: 'fresh-key' }, iv: 'fresh-iv', v: 'v2' }),
  };
  const plain = await copyTemplateMedia(service, '!plain', payload);
  expect(plain.content[types.file]).not.toHaveProperty(keys.encryptedFile);
  expect(plain.content[types.file]).toMatchObject({ uri: 'mxc://new/plain', size: 3 });
  const encrypted = await copyTemplateMedia(service, '!encrypted', payload);
  expect(encrypted.content[types.file]).toHaveProperty(keys.encryptedFile, {
    key: { k: 'fresh-key' },
    iv: 'fresh-iv',
    v: 'v2',
  });
  expect(service.download).toHaveBeenCalledWith({ url: 'mxc://old/file', ...previous });
  expect(service.upload).toHaveBeenCalledWith('!encrypted', bytes, 'text/plain');
  expect(payload.content[types.file].uri).toBe('mxc://old/file');
});
