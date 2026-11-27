// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { canonicalType, keys, legacyTypes, types } from './identifiers';
import { object } from './relations';
import type { EventPayload } from './types';

const emptyTitle = () => ({ 'm.text': [{ body: '', mimetype: 'text/plain' }] });
const aliases = new Map<string, string>(
  Object.entries(legacyTypes).map(([kind, type]) => [types[kind as keyof typeof legacyTypes], type]),
);

const legacyBlock = ({ type, content: original }: EventPayload): EventPayload => {
  const content = { ...original };
  if (type === types.infobox) {
    content[keys.title] ??= emptyTitle();
    content[keys.hyperlink] ??= { [keys.label]: emptyTitle(), url: '' };
  }
  if (type === types.textInquiry || type === types.fileInquiry) {
    content[legacyTypes.textInquiry] = {
      ...object(content[types.textInquiry]),
      ...(type === types.fileInquiry ? object(content[types.fileInquiry]) : {}),
    };
    delete content[types.textInquiry];
    delete content[types.fileInquiry];
  } else if (type === types.pollInquiry && content[types.textInquiry] !== undefined) {
    content[legacyTypes.textInquiry] = content[types.textInquiry];
    delete content[types.textInquiry];
  }
  return { type: aliases.get(type) ?? type, content };
};

/** Adapts canonical drafts before chain lengths and durable transaction IDs are assigned. */
export const legacyPayloads = (blocks: readonly EventPayload[]): EventPayload[] =>
  blocks.flatMap((block) => {
    const adapted = legacyBlock(block);
    return block.type === types.textInquiry || block.type === types.fileInquiry
      ? [{ type: types.text, content: { 'm.text': block.content['m.text'] } }, adapted]
      : [adapted];
  });

/** Creates a canonical view for validation/rendering without changing the original wire event. */
export const canonicalPayload = (payload: EventPayload): EventPayload => {
  const type = canonicalType(payload.type);
  const content = { ...payload.content };
  if (payload.type !== type && content[legacyTypes.textInquiry] !== undefined) {
    content[types.textInquiry] = content[legacyTypes.textInquiry];
    if (type === types.fileInquiry) content[types.fileInquiry] = content[legacyTypes.textInquiry];
    delete content[legacyTypes.textInquiry];
  }
  return { type, content };
};
