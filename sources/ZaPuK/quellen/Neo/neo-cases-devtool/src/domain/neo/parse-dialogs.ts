// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { canonicalType, isInquiry, keys, types } from './identifiers';
import { chainTarget, object, relation, replyTarget, threadRoot } from './relations';
import type { Dialog, Envelope, MatrixEvent } from './types';
import { validContent } from './validation';

const assemble = (head: MatrixEvent, candidates: MatrixEvent[]): Envelope | undefined => {
  const declared = head.content[keys.length];
  const length = declared ?? 0;
  if (typeof length !== 'number' || !Number.isSafeInteger(length) || length < 0 || length > candidates.length) return;
  const blocks = [head];
  const seen = new Set([head.event_id]);
  let previous = head;
  for (let index = 0; index < length; index += 1) {
    const next = candidates.filter((event) => event.sender === head.sender && chainTarget(event) === previous.event_id);
    const child = next[0];
    if (next.length !== 1 || !child || seen.has(child.event_id) || child.content[keys.length] !== undefined) return;
    blocks.push(child);
    seen.add(child.event_id);
    previous = child;
  }
  if (blocks.some((block) => !validContent(block)) || blocks.filter((block) => isInquiry(block.type)).length > 1)
    return;
  return { id: head.event_id, sender: head.sender, timestamp: head.origin_server_ts, blocks };
};

const validResponse = (envelope: Envelope, inquiry: MatrixEvent): boolean => {
  const blocks = envelope.blocks;
  if (blocks.every((block) => block.type === types.text)) return true;
  if (canonicalType(inquiry.type) === types.textInquiry) return false;
  if (canonicalType(inquiry.type) === types.fileInquiry) {
    return (
      blocks.length === 1 &&
      [types.file, types.image, types.video, types.audio].some((type) => blocks[0]?.type === type)
    );
  }
  const response = blocks.find((block) => block.type === types.pollResponse);
  if (!response || relation(response, keys.pollRelation) !== inquiry.event_id) return false;
  const poll = object(inquiry.content[keys.poll]);
  const options = (poll.answers as unknown[]).map((answer) => object(answer)['m.id']);
  const selections = response.content[keys.selections] as string[];
  if (
    new Set(selections).size !== selections.length ||
    selections.length > Number(poll.max_selections ?? 1) ||
    selections.some((selection) => !options.includes(selection))
  )
    return false;
  if (selections.includes(keys.customAnswer)) {
    return (
      blocks.length === 2 &&
      blocks[0]?.type === types.text &&
      blocks[1] === response &&
      relation(response, keys.customRelation) === blocks[0].event_id
    );
  }
  return blocks.length === 1;
};

interface ResponseState {
  accepted: Set<string>;
  official: boolean;
  envelopes: Envelope[];
}

const appendResponse = (
  event: MatrixEvent,
  inquiry: MatrixEvent,
  candidates: MatrixEvent[],
  state: ResponseState,
): boolean => {
  if (state.accepted.has(event.event_id)) return false;
  const target = replyTarget(event) ?? chainTarget(event);
  const first = target === inquiry.event_id;
  if (!first && (!target || !state.accepted.has(target))) return false;
  const envelope = assemble(event, candidates);
  if (!envelope || envelope.blocks.some((block) => state.accepted.has(block.event_id))) return false;
  const textOnly = envelope.blocks.every((block) => block.type === types.text);
  if (!validResponse(envelope, inquiry) || (!textOnly && (!first || state.official))) return false;
  if (!textOnly) state.official = true;
  state.envelopes.push(envelope);
  envelope.blocks.forEach((block) => state.accepted.add(block.event_id));
  return true;
};

const parseResponses = (
  inquiry: MatrixEvent | undefined,
  citizen: string | undefined,
  candidates: MatrixEvent[],
): Envelope[] => {
  if (!inquiry || !citizen) return [];
  const state: ResponseState = { accepted: new Set(), official: false, envelopes: [] };
  const pending = candidates
    .filter((event) => event.sender === citizen)
    .sort((a, b) => a.origin_server_ts - b.origin_server_ts || a.event_id.localeCompare(b.event_id));
  // Resolve causal order even when server timestamps disagree with the reply chain.
  let changed = true;
  while (changed) {
    changed = false;
    for (const event of pending) if (appendResponse(event, inquiry, candidates, state)) changed = true;
  }
  return state.envelopes;
};

/**
 * Projects complete, non-redacted Neo dialogs in original-envelope timestamp order.
 * @param input Decrypted events, including redactions and previously loaded history.
 * @param agencyId Agency account whose cases are being simulated.
 * @param citizenId Intended citizen, resolved from room membership.
 * @returns Complete dialogs with valid citizen responses.
 */
export const parseDialogs = (input: Iterable<MatrixEvent>, agencyId: string, citizenId?: string): Dialog[] => {
  const all = [...input];
  const redacted = new Set(
    all
      .filter((event) => event.type === 'm.room.redaction')
      .map((event) => event.redacts ?? event.content.redacts)
      .filter((id): id is string => typeof id === 'string'),
  );
  const events = new Map(
    all
      .filter(
        (event) => event.state_key === undefined && !event.unsigned?.redacted_because && !redacted.has(event.event_id),
      )
      .map((event) => [event.event_id, event]),
  );
  const roots = [...events.values()].filter(
    (event) =>
      event.sender === agencyId &&
      !relation(event, 'm.thread') &&
      !chainTarget(event) &&
      validContent(event) &&
      event.type !== types.pollResponse,
  );
  const groups = new Map<string, MatrixEvent[]>();
  for (const event of events.values()) {
    const root = threadRoot(event, events);
    if (root) groups.set(root, [...(groups.get(root) ?? []), event]);
  }
  return roots
    .flatMap((root): Dialog[] => {
      const candidates = groups.get(root.event_id) ?? [];
      const agency = assemble(root, candidates);
      if (!agency) return [];
      const inquiry = agency.blocks.find((block) => isInquiry(block.type));
      const responses = parseResponses(inquiry, citizenId, candidates);
      return [{ id: root.event_id, timestamp: root.origin_server_ts, agency, responses }];
    })
    .sort((a, b) => b.timestamp - a.timestamp || a.id.localeCompare(b.id));
};
