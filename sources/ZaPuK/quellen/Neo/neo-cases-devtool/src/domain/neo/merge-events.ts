// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { MatrixEvent } from './types';
import { parseEvent } from './validation';

/** Merges live and paginated events without resurrecting redactions or undoing decryption. */
export const mergeEvents = (target: Map<string, MatrixEvent>, values: unknown[]): void => {
  for (const value of values) {
    const event = parseEvent(value);
    if (!event) continue;
    const previous = target.get(event.event_id);
    if (previous?.unsigned?.redacted_because) continue;
    const staleCiphertext = previous && previous.type !== 'm.room.encrypted' && event.type === 'm.room.encrypted';
    if (staleCiphertext && !event.unsigned?.redacted_because) continue;
    target.set(event.event_id, event);
  }
};
