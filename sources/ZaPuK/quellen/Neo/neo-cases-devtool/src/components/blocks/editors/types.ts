// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { BlockDraft } from '../../../domain/neo/types';

export interface EditorProps {
  block: BlockDraft;
  update: (patch: Partial<Omit<BlockDraft, 'id' | 'kind'>>) => void;
  disabled: boolean;
}
