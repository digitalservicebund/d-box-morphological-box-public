// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { z } from 'zod';
import { relateBlock, serializeBlock, validateEnvelope } from '../domain/neo/serialize-envelope';
import type { BlockDraft, EventPayload } from '../domain/neo/types';
import type { MatrixService } from './matrix-service';
import { readSession, readStored, writeStored } from './storage';
import { sanitizeHtml } from '../utils/rich-text';
import { legacyPayloads } from '../domain/neo/legacy-compatibility';

const schema = z.compile(
  z.object({
    roomId: z.string(),
    legacyCompatibility: z.boolean().optional(),
    account: z.string(),
    sessionDatabase: z.string(),
    transactions: z.array(z.string()).min(1),
    ids: z.array(z.string()),
    blocks: z.array(z.object({ type: z.string(), content: z.record(z.string(), z.unknown()) })).min(1),
  }),
);
export type SendJournal = z.infer<typeof schema>;
const storageKey = (account: string, roomId: string): string => `send:${account}:${roomId}`;

export const restoreSend = async (account: string, roomId: string): Promise<SendJournal | undefined> => {
  const value = await readStored(storageKey(account, roomId));
  if (value === undefined) return;
  const journal = schema.parse(value);
  const session = await readSession();
  if (journal.sessionDatabase !== session?.database) {
    throw new Error('This pending send belongs to an earlier device session and cannot safely be resumed.');
  }
  if (
    journal.account !== account ||
    journal.roomId !== roomId ||
    journal.ids.length > journal.blocks.length ||
    journal.transactions.length !== journal.blocks.length
  )
    throw new Error('Invalid pending send state.');
  return journal;
};

/** Uploads media before freezing and durably recording a send operation. */
export const prepareEnvelope = async (
  service: MatrixService,
  account: string,
  roomId: string,
  draft: readonly BlockDraft[],
  legacyCompatibility = false,
): Promise<SendJournal> => {
  validateEnvelope(draft);
  const blocks: EventPayload[] = [];
  for (const block of draft) {
    const media = block.file
      ? await service.upload(
          roomId,
          new Uint8Array(await block.file.arrayBuffer()),
          block.file.type || 'application/octet-stream',
        )
      : undefined;
    blocks.push(serializeBlock({ ...block, html: block.html ? sanitizeHtml(block.html) : '' }, media));
  }
  return preparePayloadEnvelope(
    account,
    roomId,
    legacyCompatibility ? legacyPayloads(blocks) : blocks,
    legacyCompatibility,
  );
};

/** Records already validated event payloads before any part of their envelope is published. */
export const preparePayloadEnvelope = async (
  account: string,
  roomId: string,
  blocks: EventPayload[],
  legacyCompatibility = false,
): Promise<SendJournal> => {
  if (blocks.some((block) => new TextEncoder().encode(JSON.stringify(block.content)).length > 40_000))
    throw new Error('Block content is too large for a Matrix event.');
  const session = await readSession();
  if (!session || session.profile.mxid !== account) throw new Error('The agency session has ended.');
  const journal: SendJournal = {
    account,
    legacyCompatibility,
    sessionDatabase: session.database,
    roomId,
    blocks,
    ids: [],
    transactions: blocks.map(() => crypto.randomUUID()),
  };
  await writeStored(storageKey(account, roomId), journal);
  return journal;
};

export const discardPendingSend = (account: string, roomId: string): Promise<void> =>
  writeStored(storageKey(account, roomId), undefined);

/**
 * Sends a frozen envelope sequentially and resumes it with stable Matrix transaction IDs.
 * @param send Matrix event transport.
 * @param journal Durable operation snapshot, updated only after server acknowledgements.
 * @param checkpoint Persists progress before the next event is sent.
 * @returns The dialog's root event ID.
 */
export const publishEnvelope = async (
  send: MatrixService['send'],
  journal: SendJournal,
  checkpoint: (journal: SendJournal) => Promise<void>,
): Promise<string> => {
  for (let index = journal.ids.length; index < journal.blocks.length; index += 1) {
    const block = journal.blocks[index];
    const transaction = journal.transactions[index];
    if (!block || !transaction) throw new Error('Incomplete send snapshot.');
    const payload = relateBlock(block, index, journal.blocks.length, journal.ids);
    const id = await send(journal.roomId, payload.type, payload.content, transaction);
    journal.ids.push(id);
    await checkpoint(journal);
  }
  return journal.ids[0]!;
};

export const sendEnvelope = async (
  service: Pick<MatrixService, 'send'>,
  journal: SendJournal,
  onProgress: (sent: number) => void,
): Promise<string> => {
  const key = storageKey(journal.account, journal.roomId);
  const id = await publishEnvelope(service.send, journal, async (progress) => {
    await writeStored(key, progress);
    onProgress(progress.ids.length);
  });
  await writeStored(key, undefined);
  return id;
};
