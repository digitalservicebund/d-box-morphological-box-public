// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useEffect, useRef, useState, type FormEvent } from 'react';
import type { M3Dialog } from '@banegasn/m3-dialog';
import type { CaseTemplate } from '../../domain/neo/case-template';
import { caseInputSchema } from '../../domain/neo/validation';
import { errorMessage } from '../../utils/errors';
import { fieldErrors } from '../../utils/field-errors';
import { useAuth } from '../auth/auth-context';
import { useNotify } from '../notifications/notification-context';
import { useWorkspace } from './workspace-context';
import { useCaseTemplateFile } from './use-case-template-file';
import { useCaseCreation } from './use-case-creation';
import { completeMxid, mxidDomain } from '../../utils/matrix-address';

export const useCaseSetup = (onClose: () => void) => {
  const [input, setInput] = useState({ recipient: '', name: '', topic: '', encrypted: true });
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [failure, setFailure] = useState('');
  const ref = useRef<M3Dialog>(null);
  const { connected, profile, refreshRooms } = useAuth();
  const { select } = useWorkspace();
  const notify = useNotify();
  const creation = useCaseCreation();
  const metadata = useCallback((value: CaseTemplate['case']) => setInput((current) => ({ ...current, ...value })), []);
  const file = useCaseTemplateFile(metadata);
  const close = useCallback(() => {
    if (creation.createdId) {
      select(creation.createdId);
      void refreshRooms().catch((cause: unknown) => notify(errorMessage(cause), 'error'));
    }
    onClose();
  }, [creation.createdId, select, refreshRooms, notify, onClose]);
  useEffect(() => {
    const element = ref.current;
    const requestClose = (event: Event) => {
      if (creation.busy) event.preventDefault();
    };
    element?.addEventListener('dialog-request-close', requestClose);
    element?.addEventListener('dialog-close', close);
    return () => {
      element?.removeEventListener('dialog-request-close', requestClose);
      element?.removeEventListener('dialog-close', close);
    };
  }, [close, creation.busy]);
  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (creation.busy || !connected || file.loading || file.error) return;
    const result = caseInputSchema.safeParse({
      ...input,
      recipient: completeMxid(input.recipient, profile?.mxid ?? ''),
    });
    if (!result.success) {
      setErrors(fieldErrors(result.error));
      return;
    }
    if (result.data.recipient === profile?.mxid) {
      setErrors({ recipient: 'Choose a citizen other than the agency account.' });
      return;
    }
    setErrors({});
    setInput((current) => ({ ...current, recipient: result.data.recipient }));
    setFailure('');
    try {
      const id = await creation.create(result.data, file.template);
      if (!id) return;
      select(id);
      notify(
        file.template
          ? `Case created with ${file.template.dialogs.length} imported dialogs.`
          : 'Case created and citizen invited.',
        'success',
      );
      onClose();
    } catch (cause) {
      const message = errorMessage(cause);
      setFailure(message);
      notify(`Case creation/import failed: ${message}`, 'error');
    }
  };
  const disabled = creation.busy || Boolean(creation.createdId);
  return {
    ref,
    input,
    setInput,
    errors,
    failure,
    creation,
    file,
    close,
    submit,
    disabled,
    connected,
    recipientDomain: mxidDomain(profile?.mxid ?? ''),
  };
};
