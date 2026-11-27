// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdClose, MdSave } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import { TextField } from '../../components/ui/TextField';
import type { CaseRoom } from '../../matrix/types';
import { useCaseMetadata } from './use-case-metadata';
import styles from './CreateCaseDialog.module.css';

export interface CaseMetadataDialogProps {
  room: CaseRoom;
  onClose: () => void;
}

export const CaseMetadataDialog: FC<CaseMetadataDialogProps> = ({ room, onClose }) => {
  const { ref, input, setInput, errors, failure, busy, connected, submit } = useCaseMetadata(room, onClose);
  return (
    <m3-dialog ref={ref} open headline="Change case metadata">
      <form id="case-metadata-form" className={styles.form} onSubmit={(event) => void submit(event)}>
        <TextField
          label="Case title"
          name="case-title"
          value={input.name}
          onChange={(name) => setInput({ ...input, name })}
          error={errors.name}
          required
          disabled={busy}
        />
        <TextField
          label="Agency name"
          name="agency"
          value={input.topic}
          onChange={(topic) => setInput({ ...input, topic })}
          error={errors.topic}
          required
          disabled={busy}
        />
        {failure && (
          <p className={styles.error} role="alert">
            {failure}
          </p>
        )}
      </form>
      <Button slot="actions" icon={MdClose} variant="text" onClick={onClose} disabled={busy}>
        Cancel
      </Button>
      <Button
        slot="actions"
        icon={MdSave}
        type="submit"
        form="case-metadata-form"
        loading={busy}
        disabled={!connected || busy}
      >
        Save changes
      </Button>
    </m3-dialog>
  );
};
