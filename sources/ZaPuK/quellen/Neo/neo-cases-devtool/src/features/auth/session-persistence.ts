// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { MatrixService } from '../../matrix/matrix-service';
import { writeSession, type StoredSession } from '../../matrix/storage';
import type { useClientLifecycle } from './use-client-lifecycle';
import type { Notify } from '../notifications/notification-context';

/** Persists SDK token rotations before subsequent session/profile writes can overtake them. */
export const persistActiveSession = async (
  client: MatrixService,
  data: StoredSession,
  lifecycle: ReturnType<typeof useClientLifecycle>,
  epoch: number,
  notify: Notify,
): Promise<boolean> => {
  const { active, stored, generation } = lifecycle;
  const isCurrent = () => active.current === client && generation.current === epoch;
  const homeserver = await client.homeserver();
  if (!isCurrent()) return false;
  const current = { ...data, homeserver, session: client.currentSession() };
  stored.current = current;
  client.watchSession((session) => {
    if (!isCurrent() || !stored.current) return;
    const updated = { ...stored.current, session };
    stored.current = updated;
    void writeSession(updated, isCurrent).catch(() =>
      notify('Could not persist the refreshed Matrix session.', 'error'),
    );
  });
  await writeSession(current, isCurrent);
  return isCurrent();
};
