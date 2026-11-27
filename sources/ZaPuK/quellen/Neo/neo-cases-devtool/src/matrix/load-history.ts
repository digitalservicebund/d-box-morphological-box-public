// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { MatrixService } from './matrix-service';

/** Loads every accessible page, detecting a server that repeats a pagination token. */
export const loadHistory = async (
  service: MatrixService,
  roomId: string,
  merge: (events: unknown[]) => void,
  cancelled: () => boolean,
): Promise<void> => {
  let cursor: string | undefined;
  const cursors = new Set<string>();
  do {
    const page = await service.history(roomId, cursor);
    if (cancelled()) return;
    merge(page.events);
    cursor = page.next ?? undefined;
    if (cursor && cursors.has(cursor)) throw new Error('The server repeated a history page. Retry loading this case.');
    if (cursor) cursors.add(cursor);
  } while (cursor && !cancelled());
};
