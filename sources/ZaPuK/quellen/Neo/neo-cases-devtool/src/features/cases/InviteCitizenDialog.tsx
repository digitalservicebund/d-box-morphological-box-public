// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdClose, MdPersonAdd } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import { TextField } from '../../components/ui/TextField';
import type { CaseRoom } from '../../matrix/types';
import { useCitizenInvitation } from './use-citizen-invitation';
import styles from './CreateCaseDialog.module.css';

export interface InviteCitizenDialogProps {
  room: CaseRoom;
  onClose: () => void;
}

export const InviteCitizenDialog: FC<InviteCitizenDialogProps> = ({ room, onClose }) => {
  const { ref, recipient, setRecipient, error, failure, busy, connected, submit, recipientDomain } =
    useCitizenInvitation(room.id, onClose);
  return (
    <m3-dialog ref={ref} open headline="Invite citizen">
      <form id="invite-citizen-form" className={styles.form} onSubmit={(event) => void submit(event)}>
        <p>
          Invite a citizen to <strong>{room.name}</strong>.
        </p>
        <TextField
          label="Citizen MXID"
          name="recipient"
          value={recipient}
          onChange={setRecipient}
          placeholder="@citizen or @citizen:example.org"
          error={error}
          required
          disabled={busy}
        />
        {recipientDomain && <p className={styles.hint}>Without a domain, the recipient uses {recipientDomain}.</p>}
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
        icon={MdPersonAdd}
        type="submit"
        form="invite-citizen-form"
        loading={busy}
        disabled={!connected || busy}
      >
        Invite citizen
      </Button>
    </m3-dialog>
  );
};
