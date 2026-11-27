// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useState } from 'react';
import { restoreSend, type SendJournal } from '../../matrix/send-envelope';
import { errorMessage } from '../../utils/errors';
import { useAuth } from '../auth/auth-context';
import { useNotify } from '../notifications/notification-context';
import { useWorkspace } from '../cases/workspace-context';

export const usePendingSend = (roomId: string) => {
  const [pending, setPending] = useState<SendJournal>();
  const [checking, setChecking] = useState(true);
  const { profile } = useAuth();
  const { openDraft } = useWorkspace();
  const notify = useNotify();
  useEffect(() => {
    let active = true;
    if (!profile) return;
    void restoreSend(profile.mxid, roomId)
      .then((journal) => {
        if (!active) return;
        setPending(journal);
        if (journal) openDraft();
      })
      .catch((cause: unknown) => notify(`Could not restore pending dialog: ${errorMessage(cause)}`, 'error'))
      .finally(() => {
        if (active) setChecking(false);
      });
    return () => {
      active = false;
    };
  }, [profile, roomId, openDraft, notify]);
  return { pending, setPending, checking };
};
