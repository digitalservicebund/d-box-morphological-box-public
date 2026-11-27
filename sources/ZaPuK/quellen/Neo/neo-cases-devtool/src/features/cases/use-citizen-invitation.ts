// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useRef, useState, type FormEvent } from 'react';
import { useDialogClose } from '../../components/ui/use-dialog-close';
import { recipientSchema } from '../../domain/neo/validation';
import { completeMxid, mxidDomain } from '../../utils/matrix-address';
import { errorMessage } from '../../utils/errors';
import { useAuth } from '../auth/auth-context';
import { useNotify } from '../notifications/notification-context';

/** Manages validation and retryable invitations for an existing case. */
export const useCitizenInvitation = (roomId: string, onClose: () => void) => {
  const [recipient, setRecipient] = useState('');
  const [error, setError] = useState('');
  const [failure, setFailure] = useState('');
  const [busy, setBusy] = useState(false);
  const running = useRef(false);
  const ref = useDialogClose(onClose, busy);
  const { service, profile, connected } = useAuth();
  const notify = useNotify();
  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (!service || !profile || !connected || running.current) return;
    setFailure('');
    const result = recipientSchema.safeParse(completeMxid(recipient, profile.mxid));
    if (!result.success) {
      setError(result.error.issues[0]!.message);
      return;
    }
    if (result.data === profile.mxid) {
      setError('Choose a citizen other than the agency account.');
      return;
    }
    setError('');
    setRecipient(result.data);
    running.current = true;
    setBusy(true);
    try {
      await service.invite(roomId, result.data);
      notify(`Invitation sent to ${result.data}.`, 'success');
      onClose();
    } catch (cause) {
      setFailure(`Could not invite citizen: ${errorMessage(cause)}`);
    } finally {
      running.current = false;
      setBusy(false);
    }
  };
  return {
    ref,
    recipient,
    setRecipient,
    error,
    failure,
    busy,
    connected,
    submit,
    recipientDomain: mxidDomain(profile?.mxid ?? ''),
  };
};
