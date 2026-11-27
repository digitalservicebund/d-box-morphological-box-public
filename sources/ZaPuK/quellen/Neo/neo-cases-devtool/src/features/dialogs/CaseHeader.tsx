// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdLockOutline, MdLockOpen, MdRefresh } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import type { CaseRoom } from '../../matrix/types';
import styles from './DialogTimeline.module.css';

export interface CaseHeaderProps {
  room?: CaseRoom;
  count: number;
  inactive: boolean;
  canRefresh: boolean;
  refresh: () => void;
}

export const CaseHeader: FC<CaseHeaderProps> = ({ room, count, inactive, canRefresh, refresh }) => (
  <header className={styles.header} inert={inactive}>
    <div>
      <span className={styles.eyebrow}>CASE WORKSPACE</span>
      <h1>{room?.name ?? 'Case'}</h1>
      <p>{room?.topic}</p>
      <small>
        <span className={room?.encrypted ? styles.encrypted : styles.unencrypted}>
          {room?.encrypted ? <MdLockOutline aria-hidden="true" /> : <MdLockOpen aria-hidden="true" />}
          {room?.encrypted ? 'End-to-end encrypted' : 'Unencrypted'}
        </span>
        · {count} {count === 1 ? 'dialog' : 'dialogs'}
      </small>
    </div>
    <Button icon={MdRefresh} label="Refresh case" variant="text" onClick={refresh} disabled={!canRefresh} />
  </header>
);
