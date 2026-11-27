// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdForum, MdRefresh } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import type { Dialog } from '../../domain/neo/types';
import { DialogCard } from './DialogCard';
import styles from './DialogTimeline.module.css';

export interface TimelineCardsProps {
  dialogs: Dialog[];
  loading: boolean;
  inactive: boolean;
  error: string;
  encryptedCount: number;
  reload: () => void;
}

export const TimelineCards: FC<TimelineCardsProps> = ({
  dialogs,
  loading,
  inactive,
  error,
  encryptedCount,
  reload,
}) => (
  <div className={styles.cards} inert={inactive}>
    {loading && (
      <p className={styles.hint} role="status">
        Loading case history…
      </p>
    )}
    {error && (
      <div className={styles.problem}>
        <p>{error}</p>
        <Button icon={MdRefresh} variant="text" onClick={reload}>
          Retry
        </Button>
      </div>
    )}
    {encryptedCount > 0 && (
      <p className={styles.hint}>
        {encryptedCount} encrypted events are awaiting decryption keys. Refresh after keys become available.
      </p>
    )}
    {!loading && !dialogs.length && !inactive && (
      <div className={styles.empty}>
        <MdForum size={32} aria-hidden="true" />
        <h2>No dialogs yet</h2>
        <p>Create a dialog to start the conversation.</p>
      </div>
    )}
    {dialogs.map((dialog) => (
      <DialogCard key={dialog.id} dialog={dialog} />
    ))}
  </div>
);
