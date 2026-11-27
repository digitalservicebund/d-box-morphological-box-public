// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useRef, useState, type FC } from 'react';
import type { M3Dialog } from '@banegasn/m3-dialog';
import { MdDeleteSweep, MdClose } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../auth/auth-context';

export interface ClearLocalDataDialogProps {
  onClose: () => void;
}

export const ClearLocalDataDialog: FC<ClearLocalDataDialogProps> = ({ onClose }) => {
  const [busy, setBusy] = useState(false);
  const ref = useRef<M3Dialog>(null);
  const { clearLocalData } = useAuth();
  useEffect(() => {
    const dialog = ref.current;
    const prevent = (event: Event) => {
      if (busy) event.preventDefault();
    };
    dialog?.addEventListener('dialog-request-close', prevent);
    dialog?.addEventListener('dialog-close', onClose);
    return () => {
      dialog?.removeEventListener('dialog-request-close', prevent);
      dialog?.removeEventListener('dialog-close', onClose);
    };
  }, [busy, onClose]);
  return (
    <m3-dialog ref={ref} open headline="Clear local user data?">
      <p>
        This logs you out and deletes this app’s IndexedDB data in this browser: saved sessions, cached cases and media,
        encryption keys, and pending sends.
      </p>
      <p>
        Other open tabs of this app will also be logged out locally. Your Matrix account and messages on the server are
        not deleted.
      </p>
      <Button slot="actions" icon={MdClose} variant="text" onClick={onClose} disabled={busy}>
        Cancel
      </Button>
      <Button
        slot="actions"
        icon={MdDeleteSweep}
        danger
        loading={busy}
        disabled={busy}
        onClick={() => {
          setBusy(true);
          void clearLocalData().finally(() => setBusy(false));
        }}
      >
        Clear data and log out
      </Button>
    </m3-dialog>
  );
};
