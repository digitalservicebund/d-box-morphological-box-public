// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useRef, type FC } from 'react';
import type { M3Snackbar } from '@banegasn/m3-snackbar';
import { MdClose } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import type { Notice } from './notification-context';
import styles from './Notification.module.css';

export interface NotificationProps {
  notice: Notice;
  onDismiss: () => void;
}

export const Notification: FC<NotificationProps> = ({ notice, onDismiss }) => {
  const ref = useRef<M3Snackbar>(null);
  useEffect(() => {
    const element = ref.current;
    element?.addEventListener('snackbar-dismiss', onDismiss);
    return () => element?.removeEventListener('snackbar-dismiss', onDismiss);
  }, [onDismiss]);
  return (
    <div className={styles.host}>
      <m3-snackbar
        ref={ref}
        open
        message={notice.message}
        duration={notice.kind === 'error' ? 0 : 4500}
        live={notice.kind === 'error' ? 'assertive' : 'polite'}
        lines="2"
      >
        <Button icon={MdClose} label="Dismiss notification" variant="text" slot="action" onClick={onDismiss} />
      </m3-snackbar>
    </div>
  );
};
