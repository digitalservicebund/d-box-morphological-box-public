// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useRef } from 'react';
import type { M3Dialog } from '@banegasn/m3-dialog';

/** Connects native dialog dismissal while preventing closure during a pending operation. */
export const useDialogClose = (onClose: () => void, busy: boolean) => {
  const ref = useRef<M3Dialog>(null);
  useEffect(() => {
    const dialog = ref.current;
    const preventClose = (event: Event) => {
      if (busy) event.preventDefault();
    };
    dialog?.addEventListener('dialog-request-close', preventClose);
    dialog?.addEventListener('dialog-close', onClose);
    return () => {
      dialog?.removeEventListener('dialog-request-close', preventClose);
      dialog?.removeEventListener('dialog-close', onClose);
    };
  }, [busy, onClose]);
  return ref;
};
