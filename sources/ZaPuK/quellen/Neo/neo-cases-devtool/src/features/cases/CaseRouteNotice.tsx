// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useState, type FC } from 'react';
import { MdLogin, MdArrowBack } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import type { CaseRoom } from '../../matrix/types';
import { errorMessage } from '../../utils/errors';
import { useAuth } from '../auth/auth-context';
import { useNotify } from '../notifications/notification-context';
import { useWorkspace } from './workspace-context';
import styles from '../dialogs/DialogTimeline.module.css';

export interface CaseRouteNoticeProps {
  room?: CaseRoom;
}

export const CaseRouteNotice: FC<CaseRouteNoticeProps> = ({ room }) => {
  const [busy, setBusy] = useState(false);
  const { connected, service, refreshRooms } = useAuth();
  const { select } = useWorkspace();
  const notify = useNotify();
  const join = async () => {
    if (!service || !room || busy) return;
    setBusy(true);
    try {
      await service.join(room.id);
      await refreshRooms();
    } catch (cause) {
      notify(`Could not join case: ${errorMessage(cause)}`, 'error');
    } finally {
      setBusy(false);
    }
  };
  return (
    <div className={styles.empty}>
      {room?.invited ? (
        <>
          <h2>{room.name}</h2>
          <p>Join the case invitation to view its dialogs.</p>
          <Button icon={MdLogin} onClick={() => void join()} loading={busy} disabled={!connected || busy}>
            Join case
          </Button>
        </>
      ) : (
        <p role="status">{connected ? 'This case is not available to the logged-in account.' : 'Loading cases…'}</p>
      )}
      <p>
        <Button icon={MdArrowBack} variant="text" onClick={() => select(null)}>
          Clear selection
        </Button>
      </p>
    </div>
  );
};
