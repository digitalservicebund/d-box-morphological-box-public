// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useState, type FC } from 'react';
import { MdContentCopy, MdDeleteSweep } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import { errorMessage } from '../../utils/errors';
import { useAuth } from '../auth/auth-context';
import { useNotify } from '../notifications/notification-context';
import { ClearLocalDataDialog } from './ClearLocalDataDialog';
import styles from './ProfileDialog.module.css';

export interface ProfileSessionActionsProps {
  disabled: boolean;
}

export const ProfileSessionActions: FC<ProfileSessionActionsProps> = ({ disabled }) => {
  const [confirm, setConfirm] = useState(false);
  const { service } = useAuth();
  const notify = useNotify();
  const close = useCallback(() => setConfirm(false), []);
  const copy = async () => {
    if (!service) return;
    try {
      if (!navigator.clipboard) throw new Error('Clipboard access requires HTTPS or localhost.');
      // Read the live SDK session synchronously to preserve clipboard user activation.
      await navigator.clipboard.writeText(service.accessToken());
      notify('Access token copied.', 'success');
    } catch (cause) {
      notify(`Could not copy the access token: ${errorMessage(cause)}`, 'error');
    }
  };
  return (
    <section className={styles.sessionActions} aria-label="Session actions">
      <Button icon={MdContentCopy} variant="outlined" onClick={() => void copy()} disabled={disabled || !service}>
        Copy access token
      </Button>
      <Button icon={MdDeleteSweep} danger variant="text" onClick={() => setConfirm(true)} disabled={disabled}>
        Clear local data and log out
      </Button>
      {confirm && <ClearLocalDataDialog onClose={close} />}
    </section>
  );
};
