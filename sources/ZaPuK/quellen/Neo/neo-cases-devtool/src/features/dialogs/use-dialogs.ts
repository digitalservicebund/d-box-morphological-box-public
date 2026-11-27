// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { parseDialogs } from '../../domain/neo/parse-dialogs';
import { referencedIds } from '../../domain/neo/relations';
import type { MatrixEvent } from '../../domain/neo/types';
import { mergeEvents } from '../../domain/neo/merge-events';
import { useAuth } from '../auth/auth-context';
import { errorMessage } from '../../utils/errors';
import { loadHistory } from '../../matrix/load-history';

export const useDialogs = (roomId: string, enabled = true) => {
  const { service, profile, subscribe } = useAuth();
  const [events, setEvents] = useState(new Map<string, MatrixEvent>());
  const [citizen, setCitizen] = useState<string>();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [revision, setRevision] = useState(0);
  const current = useRef(new Map<string, MatrixEvent>());
  const reload = useCallback(() => setRevision((value) => value + 1), []);

  useEffect(() => {
    if (!service || !enabled) return;
    let cancelled = false;
    let retrying = false;
    current.current = new Map();
    setEvents(new Map());
    setCitizen(undefined);
    setLoading(true);
    setError('');
    const merge = (values: unknown[]) => {
      if (cancelled) return;
      mergeEvents(current.current, values);
      setEvents(new Map(current.current));
    };
    const resolveIds = async (ids: string[]) => {
      for (let index = 0; index < ids.length && !cancelled; index += 4) {
        const results = await Promise.allSettled(ids.slice(index, index + 4).map((id) => service.event(roomId, id)));
        merge(results.flatMap((result) => (result.status === 'fulfilled' ? [result.value] : [])));
      }
    };
    const retryDecryption = async () => {
      if (retrying) return;
      retrying = true;
      try {
        await resolveIds(
          [...current.current.values()]
            .filter((event) => event.type === 'm.room.encrypted')
            .map((event) => event.event_id),
        );
      } finally {
        retrying = false;
      }
    };
    const unsubscribe = subscribe((update) => {
      const room = update.rooms.find((item) => item.roomId === roomId);
      if (room) merge(room.events);
      if (update.keysChanged) void retryDecryption();
      if (room?.limited) reload();
    });
    const load = async () => {
      try {
        const participants = await service.participants(roomId);
        if (cancelled) return;
        if (participants.length === 1) setCitizen(participants[0]);
        await loadHistory(service, roomId, merge, () => cancelled);
        const attempted = new Set<string>();
        let missing: string[];
        do {
          missing = [...new Set([...current.current.values()].flatMap(referencedIds))].filter(
            (id) => !current.current.has(id) && !attempted.has(id),
          );
          missing.forEach((id) => attempted.add(id));
          await resolveIds(missing);
        } while (missing.length && !cancelled);
        await retryDecryption();
      } catch (cause) {
        if (!cancelled) setError(errorMessage(cause));
      } finally {
        if (!cancelled) setLoading(false);
      }
    };
    void load();
    return () => {
      cancelled = true;
      unsubscribe();
    };
  }, [service, roomId, enabled, subscribe, revision, reload]);

  const dialogs = useMemo(
    () => parseDialogs(events.values(), profile?.mxid ?? '', citizen),
    [events, profile, citizen],
  );
  const encryptedCount = [...events.values()].filter(
    (event) => event.type === 'm.room.encrypted' && !event.unsigned?.redacted_because,
  ).length;
  return { dialogs, loading, error, encryptedCount, reload };
};
