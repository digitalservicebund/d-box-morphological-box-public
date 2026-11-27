// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { keys } from './identifiers';
import type { MatrixEvent } from './types';

export const object = (value: unknown): Record<string, unknown> =>
  value !== null && typeof value === 'object' && !Array.isArray(value) ? (value as Record<string, unknown>) : {};

export const relation = (event: MatrixEvent, type: string): string | undefined => {
  const relations = event.content[keys.relations];
  if (Array.isArray(relations)) {
    const match = relations.map(object).find((item) => item.rel_type === type && typeof item.event_id === 'string');
    if (match) return match.event_id as string;
  }
  const legacy = object(event.content['m.relates_to']);
  if (legacy.rel_type === type && typeof legacy.event_id === 'string') return legacy.event_id;
  if (type === keys.pollRelation && legacy.rel_type === 'm.reference' && typeof legacy.event_id === 'string') {
    return legacy.event_id;
  }
  return undefined;
};

export const replyTarget = (event: MatrixEvent): string | undefined => {
  const reply = object(object(event.content['m.relates_to'])['m.in_reply_to']);
  return typeof reply.event_id === 'string' ? reply.event_id : relation(event, 'm.in_reply_to');
};

export const chainTarget = (event: MatrixEvent): string | undefined =>
  relation(event, keys.chain) ?? replyTarget(event);

export const referencedIds = (event: MatrixEvent): string[] => {
  const scalable = event.content[keys.relations];
  const ids = Array.isArray(scalable) ? scalable.map((value) => object(value).event_id) : [];
  return [
    ...new Set(
      [...ids, relation(event, 'm.thread'), relation(event, keys.pollRelation), replyTarget(event)].filter(
        (value): value is string => typeof value === 'string',
      ),
    ),
  ];
};

export const threadRoot = (event: MatrixEvent, events: Map<string, MatrixEvent>): string | undefined => {
  const visited = new Set<string>();
  let current: MatrixEvent | undefined = event;
  while (current && !visited.has(current.event_id)) {
    visited.add(current.event_id);
    const thread = relation(current, 'm.thread');
    if (thread) return thread;
    const parent = relation(current, keys.pollRelation) ?? chainTarget(current);
    if (!parent) return current.event_id;
    current = events.get(parent);
  }
  return undefined;
};
