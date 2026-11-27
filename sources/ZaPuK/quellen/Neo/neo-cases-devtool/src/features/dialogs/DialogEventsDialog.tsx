// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useMemo, useRef, type FC } from 'react';
import type { M3Dialog } from '@banegasn/m3-dialog';
import { MdClose } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import { dialogEvents } from '../../domain/neo/dialog-events';
import type { Dialog } from '../../domain/neo/types';
import { EventDetails } from './EventDetails';
import { EventJson } from './EventJson';
import styles from './DialogEventsDialog.module.css';

export interface DialogEventsDialogProps {
  dialog: Dialog;
  onClose: () => void;
}

export const DialogEventsDialog: FC<DialogEventsDialogProps> = ({ dialog, onClose }) => {
  const ref = useRef<M3Dialog>(null);
  const events = useMemo(() => dialogEvents(dialog), [dialog]);
  useEffect(() => {
    const element = ref.current;
    element?.addEventListener('dialog-close', onClose);
    return () => element?.removeEventListener('dialog-close', onClose);
  }, [onClose]);
  return (
    <m3-dialog ref={ref} open headline="Dialog events">
      <p className={styles.hint}>Agency and citizen events displayed in this dialog, decrypted when applicable.</p>
      <div className={styles.events}>
        {events.length === 1 && events[0] ? (
          <EventJson event={events[0]} />
        ) : (
          events.map((event) => <EventDetails key={event.event_id} event={event} />)
        )}
      </div>
      <Button slot="actions" icon={MdClose} variant="text" onClick={onClose}>
        Close
      </Button>
    </m3-dialog>
  );
};
