// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdFileDownload } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import { exportCaseTemplate } from '../../domain/neo/case-template';
import type { Dialog } from '../../domain/neo/types';
import type { CaseRoom } from '../../matrix/types';
import { downloadJson } from '../../utils/download-json';
import { errorMessage } from '../../utils/errors';
import { useNotify } from '../notifications/notification-context';

export interface ExportCaseButtonProps {
  room?: CaseRoom;
  dialogs: Dialog[];
  disabled: boolean;
}

export const ExportCaseButton: FC<ExportCaseButtonProps> = ({ room, dialogs, disabled }) => {
  const notify = useNotify();
  const download = () => {
    if (!room) return;
    try {
      const template = exportCaseTemplate(
        { name: room.name, topic: room.topic ?? '', encrypted: room.encrypted },
        dialogs,
      );
      downloadJson(template, room.name);
      notify('Agency dialogs exported as JSON.', 'success');
    } catch (cause) {
      notify(`Could not export case: ${errorMessage(cause)}`, 'error');
    }
  };
  return (
    <Button icon={MdFileDownload} variant="outlined" onClick={download} disabled={disabled || !room}>
      Export JSON
    </Button>
  );
};
