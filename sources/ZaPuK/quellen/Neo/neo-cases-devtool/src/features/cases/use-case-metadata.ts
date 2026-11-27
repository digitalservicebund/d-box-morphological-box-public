// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useRef, useState, type FormEvent } from 'react';
import { useDialogClose } from '../../components/ui/use-dialog-close';
import { caseMetadataSchema } from '../../domain/neo/validation';
import type { CaseRoom } from '../../matrix/types';
import { errorMessage } from '../../utils/errors';
import { fieldErrors } from '../../utils/field-errors';
import { useAuth } from '../auth/auth-context';
import { useNotify } from '../notifications/notification-context';

/** Saves editable case metadata while retaining entered values for a failed-operation retry. */
export const useCaseMetadata = (room: CaseRoom, onClose: () => void) => {
  const [input, setInput] = useState({ name: room.name, topic: room.topic ?? '' });
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [failure, setFailure] = useState('');
  const [busy, setBusy] = useState(false);
  const saving = useRef(false);
  const ref = useDialogClose(onClose, busy);
  const { service, connected, refreshRooms } = useAuth();
  const notify = useNotify();
  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (!service || !connected || saving.current) return;
    setFailure('');
    const result = caseMetadataSchema.safeParse(input);
    if (!result.success) {
      setErrors(fieldErrors(result.error));
      return;
    }
    setErrors({});
    setInput(result.data);
    saving.current = true;
    setBusy(true);
    try {
      await service.updateCaseMetadata(room.id, result.data.name, result.data.topic);
      await refreshRooms();
      notify('Case metadata updated.', 'success');
      onClose();
    } catch (cause) {
      setFailure(
        `Could not finish updating case metadata: ${errorMessage(cause)} Some changes may already have been saved. You can retry.`,
      );
      await refreshRooms().catch(() => undefined);
    } finally {
      saving.current = false;
      setBusy(false);
    }
  };
  return { ref, input, setInput, errors, failure, busy, connected, submit };
};
