// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useState } from 'react';
import type { SenderProfile } from '../../matrix/types';
import { useAuth } from '../auth/auth-context';

/** Resolves the actual sender; the local account profile stays live after edits in this app. */
export const useSenderProfile = (sender: string): SenderProfile => {
  const { service, profile } = useAuth();
  const [resolved, setResolved] = useState<SenderProfile>();
  const own = profile?.mxid === sender;
  useEffect(() => {
    let active = true;
    setResolved(undefined);
    if (service && !own)
      void service
        .senderProfile(sender)
        .then((value) => {
          if (active) setResolved(value);
        })
        .catch(() => undefined);
    return () => {
      active = false;
    };
  }, [service, sender, own]);
  if (own && profile) return profile;
  return resolved?.mxid === sender ? resolved : { mxid: sender, displayName: null, avatarUrl: null };
};
