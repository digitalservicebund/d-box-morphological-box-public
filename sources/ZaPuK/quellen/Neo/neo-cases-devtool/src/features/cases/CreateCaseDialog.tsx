// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdAdd, MdClose, MdRefresh } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import { TextField } from '../../components/ui/TextField';
import { Checkbox } from '../../components/ui/Checkbox';
import { CaseImportField } from './CaseImportField';
import { useCaseSetup } from './use-case-setup';
import styles from './CreateCaseDialog.module.css';

export interface CreateCaseDialogProps {
  onClose: () => void;
}

export const CreateCaseDialog: FC<CreateCaseDialogProps> = ({ onClose }) => {
  const { ref, input, setInput, errors, failure, creation, file, close, submit, disabled, connected, recipientDomain } =
    useCaseSetup(onClose);
  return (
    <m3-dialog ref={ref} open headline="Create a case">
      <form id="create-case-form" className={styles.form} onSubmit={(event) => void submit(event)}>
        <CaseImportField
          name={file.name}
          error={file.error}
          loading={file.loading}
          count={file.template?.dialogs.length}
          disabled={disabled}
          onFile={file.load}
        />
        <TextField
          label="Citizen MXID"
          name="recipient"
          value={input.recipient}
          onChange={(recipient) => setInput({ ...input, recipient })}
          placeholder="@citizen or @citizen:example.org"
          error={errors.recipient}
          required
          disabled={disabled}
        />
        {recipientDomain && <p className={styles.hint}>Without a domain, the recipient uses {recipientDomain}.</p>}
        <TextField
          label="Case title"
          name="case-title"
          value={input.name}
          onChange={(name) => setInput({ ...input, name })}
          error={errors.name}
          required
          disabled={disabled}
        />
        <TextField
          label="Responsible agency / topic"
          name="agency"
          value={input.topic}
          onChange={(topic) => setInput({ ...input, topic })}
          error={errors.topic}
          required
          disabled={disabled}
        />
        <Checkbox
          label="Enable end-to-end encryption"
          checked={input.encrypted}
          onChange={(encrypted) => setInput({ ...input, encrypted })}
          disabled={disabled}
        />
        <p className={styles.hint}>
          {input.encrypted
            ? 'Messages and attachments will be encrypted.'
            : 'Messages and attachments will be unencrypted.'}
        </p>
        {creation.createdId && (
          <p role="status">
            Case created. {creation.completed} of {file.template?.dialogs.length ?? 0} dialogs imported. Retry continues
            in this same case.
          </p>
        )}
        {failure && (
          <p className={styles.error} role="alert">
            {failure}
          </p>
        )}
      </form>
      <Button icon={MdClose} variant="text" slot="actions" onClick={close} disabled={creation.busy}>
        {creation.createdId ? 'Close' : 'Cancel'}
      </Button>
      <Button
        icon={creation.createdId ? MdRefresh : MdAdd}
        slot="actions"
        type="submit"
        form="create-case-form"
        loading={creation.busy}
        disabled={!connected || creation.busy || file.loading || Boolean(file.error)}
      >
        {creation.createdId ? 'Retry import' : 'Create case'}
      </Button>
    </m3-dialog>
  );
};
