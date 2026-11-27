// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, type FC } from 'react';
import { MdAdd, MdRefresh } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../auth/auth-context';
import { useWorkspace } from '../cases/workspace-context';
import { useNotify } from '../notifications/notification-context';
import { useDialogs } from './use-dialogs';
import { CaseHeader } from './CaseHeader';
import { TimelineCards } from './TimelineCards';
import { DialogComposer } from './DialogComposer';
import { ExportCaseButton } from './ExportCaseButton';
import { CaseActions } from '../cases/CaseActions';
import { CaseRouteNotice } from '../cases/CaseRouteNotice';
import { usePendingSend } from './use-pending-send';
import styles from './DialogTimeline.module.css';

export interface DialogTimelineProps {
  roomId: string;
}

export const DialogTimeline: FC<DialogTimelineProps> = ({ roomId }) => {
  const { pending, setPending, checking } = usePendingSend(roomId);
  const { rooms, connected } = useAuth();
  const { draftOpen, openDraft } = useWorkspace();
  const notify = useNotify();
  const room = rooms.find((item) => item.id === roomId);
  const { dialogs, loading, error, encryptedCount, reload } = useDialogs(roomId, Boolean(room && !room.invited));
  useEffect(() => {
    if (error) notify(`Could not load case: ${error}`, 'error');
  }, [error, notify]);
  if (!room || room.invited) return <CaseRouteNotice room={room} />;
  return (
    <div className={styles.timeline}>
      <CaseHeader
        room={room}
        count={dialogs.length}
        inactive={draftOpen}
        canRefresh={connected && !loading}
        refresh={reload}
      />
      <div className={styles.toolbar} inert={draftOpen}>
        <Button icon={pending ? MdRefresh : MdAdd} onClick={openDraft} disabled={!connected || checking}>
          {pending ? 'Resume pending dialog' : 'Add a new dialog'}
        </Button>
        <ExportCaseButton room={room} dialogs={dialogs} disabled={loading || Boolean(error) || encryptedCount > 0} />
        <CaseActions room={room} disabled={!connected} />
        <span>Newest dialogs first</span>
      </div>
      {draftOpen && (
        <DialogComposer
          roomId={roomId}
          pending={pending}
          onPending={setPending}
          onSaved={() => {
            setPending(undefined);
            reload();
          }}
        />
      )}
      <TimelineCards
        dialogs={dialogs}
        loading={loading}
        inactive={draftOpen}
        error={error}
        encryptedCount={encryptedCount}
        reload={reload}
      />
    </div>
  );
};
