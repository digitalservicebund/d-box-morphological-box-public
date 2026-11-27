// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useRef, useState, type FC } from 'react';
import { useDialogClose } from '../../components/ui/use-dialog-close';
import { MdDeleteOutline, MdClose } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import type { CaseRoom } from '../../matrix/types';
import { discardPendingSend } from '../../matrix/send-envelope';
import { caseIdFromHash } from '../../utils/case-route';
import { errorMessage } from '../../utils/errors';
import { useAuth } from '../auth/auth-context';
import { useNotify } from '../notifications/notification-context';
import { useWorkspace } from './workspace-context';

export interface RemoveCaseDialogProps {
  room: CaseRoom;
  onClose: () => void;
}

export const RemoveCaseDialog: FC<RemoveCaseDialogProps> = ({ room, onClose }) => {
  const [busy, setBusy] = useState(false);
  const ref = useDialogClose(onClose, busy);
  const running = useRef(false);
  const { service, profile, connected, refreshRooms } = useAuth();
  const { select } = useWorkspace();
  const notify = useNotify();
  const remove = async () => {
    if (!service || !profile || running.current) return;
    running.current = true;
    setBusy(true);
    try {
      const result = await service.removeCase(room.id);
      await discardPendingSend(profile.mxid, room.id).catch((cause: unknown) =>
        notify(`Pending-send cleanup failed: ${errorMessage(cause)}`, 'error'),
      );
      if (caseIdFromHash(window.location.hash) === room.id) select(null);
      await refreshRooms().catch((cause: unknown) =>
        notify(`Case list refresh failed: ${errorMessage(cause)}`, 'error'),
      );
      notify(
        result.forgotten ? 'Case removed.' : `Left the case, but could not finish forgetting it: ${result.warning}`,
        result.forgotten ? 'success' : 'error',
      );
      onClose();
    } catch (cause) {
      notify(`Could not remove case: ${errorMessage(cause)}`, 'error');
    } finally {
      running.current = false;
      setBusy(false);
    }
  };
  return (
    <m3-dialog ref={ref} open headline="Remove case?">
      <p>
        Remove <strong>{room.name}</strong> from this account?
      </p>
      <p>
        This leaves the Matrix room and forgets it for the agency account. It does not delete the room for the citizen
        or other members.
      </p>
      <p>Any pending send for this case will be cleared.</p>
      <Button slot="actions" icon={MdClose} variant="text" onClick={onClose} disabled={busy}>
        Cancel
      </Button>
      <Button
        slot="actions"
        icon={MdDeleteOutline}
        danger
        onClick={() => void remove()}
        loading={busy}
        disabled={!connected || busy}
      >
        Remove case
      </Button>
    </m3-dialog>
  );
};
