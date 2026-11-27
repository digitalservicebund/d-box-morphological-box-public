// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { CaseTemplate } from '../domain/neo/case-template';
import type { MatrixService } from './matrix-service';
import { copyTemplateMedia } from './import-media';
import { preparePayloadEnvelope, sendEnvelope, type SendJournal } from './send-envelope';

export interface ImportProgress {
  completed: number;
  pending?: SendJournal;
}

/** Continues a bulk import without replaying dialogs that were already acknowledged. */
export const importCaseDialogs = async (
  service: Pick<MatrixService, 'send' | 'download' | 'upload'>,
  account: string,
  roomId: string,
  template: CaseTemplate,
  progress: ImportProgress,
  onProgress: (completed: number) => void,
): Promise<void> => {
  while (progress.completed < template.dialogs.length) {
    const dialog = template.dialogs[progress.completed];
    if (!dialog) throw new Error('Missing imported dialog.');
    if (!progress.pending) {
      const blocks = [];
      for (const block of dialog.blocks) blocks.push(await copyTemplateMedia(service, roomId, block));
      progress.pending = await preparePayloadEnvelope(account, roomId, blocks);
    }
    await sendEnvelope(service, progress.pending, () => undefined);
    progress.pending = undefined;
    progress.completed += 1;
    onProgress(progress.completed);
  }
};
