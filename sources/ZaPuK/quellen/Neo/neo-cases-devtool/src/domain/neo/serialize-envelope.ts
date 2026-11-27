// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { blockLabels, isInquiry, keys, types } from './identifiers';
import type { BlockDraft, EventPayload } from './types';
import { validateDraft } from './validation';
import { serializeContent } from './serialize-content';

/** Serializes one validated draft block, with all required text fallbacks. */
export const serializeBlock = (block: BlockDraft, media?: Record<string, unknown>): EventPayload => {
  const errors = validateDraft(block);
  if (errors.length) throw new Error(`${blockLabels[block.kind]}: ${errors.join(' ')}`);
  const content = serializeContent(block, media);
  if (new TextEncoder().encode(JSON.stringify(content)).length > 40_000)
    throw new Error('Block content is too large for a Matrix event.');
  return { type: types[block.kind], content };
};

/** Adds immutable chain and thread relations after preceding sends have returned event IDs. */
export const relateBlock = (
  payload: EventPayload,
  index: number,
  count: number,
  ids: readonly string[],
): EventPayload => {
  if (!index)
    return { ...payload, content: { ...payload.content, ...(count > 1 ? { [keys.length]: count - 1 } : {}) } };
  const root = ids[0];
  const previous = ids[index - 1];
  if (!root || !previous) throw new Error('The preceding building block has not been sent.');
  return {
    ...payload,
    content: {
      ...payload.content,
      'm.relates_to': { rel_type: 'm.thread', event_id: root, 'm.in_reply_to': { event_id: previous } },
      [keys.relations]: [
        { rel_type: keys.chain, event_id: previous },
        { rel_type: 'm.thread', event_id: root },
      ],
    },
  };
};

export const validateEnvelope = (blocks: readonly BlockDraft[]): void => {
  if (!blocks.length) throw new Error('Add at least one building block.');
  if (blocks.filter((block) => isInquiry(types[block.kind])).length > 1)
    throw new Error('A dialog may contain only one inquiry.');
  const errors = blocks.flatMap((block, index) =>
    validateDraft(block).map((message) => `Block ${index + 1}: ${message}`),
  );
  if (errors.length) throw new Error(errors.join('\n'));
};
