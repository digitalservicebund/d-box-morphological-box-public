// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { ClientState, useClientStore } from '@matrix-g2x/neo-web-sdk/react';
import { useEffect, useState } from 'react';

export function useUsername(): string | undefined {
  const clientStore = useClientStore();
  const isLoggedIn = clientStore.clientState === ClientState.LoggedIn;
  const client = clientStore.client;
  const userId = isLoggedIn ? client?.userId() : undefined;

  const [userProfile, setUserProfile] = useState<{
    userId: string;
    displayName: string;
  }>();

  useEffect(() => {
    if (!isLoggedIn || !client || !userId) {
      return;
    }

    const controller = new AbortController();
    client
      .displayName()
      .then((displayName) => {
        if (!controller.signal.aborted) {
          setUserProfile({ userId, displayName: displayName || userId });
        }
      })
      .catch(() => {
        if (!controller.signal.aborted) {
          setUserProfile({ userId, displayName: userId });
        }
      });

    return () => {
      controller.abort();
    };
  }, [client, isLoggedIn, userId]);

  if (!userId) return undefined;
  return userProfile?.userId === userId ? userProfile.displayName : userId;
}
