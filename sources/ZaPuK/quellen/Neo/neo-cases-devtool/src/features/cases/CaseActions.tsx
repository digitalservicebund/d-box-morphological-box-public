// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useState, type FC } from 'react';
import { MdDeleteOutline, MdEdit, MdMoreVert, MdPersonAdd } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import { MenuButton } from '../../components/ui/MenuButton';
import type { CaseRoom } from '../../matrix/types';
import { InviteCitizenDialog } from './InviteCitizenDialog';
import { RemoveCaseDialog } from './RemoveCaseDialog';
import { CaseMetadataDialog } from './CaseMetadataDialog';

const options = [
  { value: 'invite', label: 'Invite citizen', icon: MdPersonAdd },
  { value: 'remove', label: 'Remove case', icon: MdDeleteOutline },
];

export interface CaseActionsProps {
  room: CaseRoom;
  disabled: boolean;
}

export const CaseActions: FC<CaseActionsProps> = ({ room, disabled }) => {
  const [action, setAction] = useState('');
  const close = useCallback(() => setAction(''), []);
  return (
    <>
      <Button icon={MdEdit} variant="text" onClick={() => setAction('metadata')} disabled={disabled}>
        Change case metadata
      </Button>
      <MenuButton icon={MdMoreVert} label="Case actions" options={options} onSelect={setAction} disabled={disabled}>
        Case actions
      </MenuButton>
      {action === 'invite' && <InviteCitizenDialog room={room} onClose={close} />}
      {action === 'remove' && <RemoveCaseDialog room={room} onClose={close} />}
      {action === 'metadata' && <CaseMetadataDialog room={room} onClose={close} />}
    </>
  );
};
