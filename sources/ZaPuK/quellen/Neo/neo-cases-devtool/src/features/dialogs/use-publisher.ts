// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useRef, useState } from 'react';
import type { BlockDraft } from '../../domain/neo/types';
import { prepareEnvelope, sendEnvelope, type SendJournal } from '../../matrix/send-envelope';
import { errorMessage } from '../../utils/errors';
import { useAuth } from '../auth/auth-context';
import { useWorkspace } from '../cases/workspace-context';
import { useNotify } from '../notifications/notification-context';

export interface PublisherOptions {
  roomId: string;
  blocks: readonly BlockDraft[];
  legacyCompatibility: boolean;
  pending?: SendJournal;
  onPending: (journal: SendJournal) => void;
  onSaved: () => void;
}

export const usePublisher = ({
  roomId,
  blocks,
  legacyCompatibility,
  pending,
  onPending,
  onSaved,
}: PublisherOptions) => {
  const [busy, setBusy] = useState(false);
  const [journal, setJournal] = useState(pending);
  const [progress, setProgress] = useState(pending?.ids.length ?? 0);
  const [error, setError] = useState('');
  const saving = useRef(false);
  const { service, profile, connected } = useAuth();
  const { closeDraft } = useWorkspace();
  const notify = useNotify();
  const save = async () => {
    if (!service || !profile || !connected || saving.current) return;
    saving.current = true;
    setBusy(true);
    setError('');
    try {
      const snapshot = journal ?? (await prepareEnvelope(service, profile.mxid, roomId, blocks, legacyCompatibility));
      setJournal(snapshot);
      onPending(snapshot);
      await sendEnvelope(service, snapshot, setProgress);
      notify('Dialog sent.', 'success');
      onSaved();
      closeDraft();
    } catch (cause) {
      const message = errorMessage(cause);
      setError(message);
      notify(`Could not send dialog: ${message}`, 'error');
    } finally {
      setBusy(false);
      saving.current = false;
    }
  };
  const status = journal
    ? `Sending block ${Math.min(progress + 1, journal.blocks.length)} of ${journal.blocks.length}…`
    : 'Preparing attachments…';
  return { busy, journal, progress, error, save, status, connected, closeDraft };
};
