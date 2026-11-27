// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useState } from 'react';
import type { Profile } from '../../matrix/types';
import { writeSession } from '../../matrix/storage';
import type { useClientLifecycle } from './use-client-lifecycle';

/** Keeps the rendered profile and persisted session metadata in sync after account edits. */
export const useAccountProfile = ({ active, stored }: ReturnType<typeof useClientLifecycle>) => {
  const [profile, setProfile] = useState<Profile | null>(null);
  const refreshProfile = useCallback(async () => {
    const client = active.current;
    if (!client || !stored.current) return;
    const nextProfile = await client.profile();
    if (active.current !== client || !stored.current) return;
    const updated = { ...stored.current, session: client.currentSession(), profile: nextProfile };
    stored.current = updated;
    setProfile(nextProfile);
    await writeSession(updated, () => active.current === client);
  }, [active, stored]);
  return { profile, setProfile, refreshProfile };
};
