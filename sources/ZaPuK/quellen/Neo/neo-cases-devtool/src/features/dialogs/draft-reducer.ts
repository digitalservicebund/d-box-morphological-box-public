// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { isInquiry, types, type BlockKind } from '../../domain/neo/identifiers';
import type { BlockDraft } from '../../domain/neo/types';

export const createBlock = (kind: BlockKind): BlockDraft => ({
  id: crypto.randomUUID(),
  kind,
  text: '',
  html: '',
  title: '',
  label: '',
  url: '',
  multiple: false,
  answers: kind === 'pollInquiry' ? [{ id: crypto.randomUUID(), text: '' }] : [],
});

export type DraftAction =
  | { type: 'add'; kind: BlockKind }
  | { type: 'remove'; id: string }
  | { type: 'update'; id: string; patch: Partial<Omit<BlockDraft, 'id' | 'kind'>> };

export const draftReducer = (state: BlockDraft[], action: DraftAction): BlockDraft[] => {
  switch (action.type) {
    case 'add':
      if (isInquiry(types[action.kind]) && state.some((block) => isInquiry(types[block.kind]))) return state;
      return [...state, createBlock(action.kind)];
    case 'remove':
      return state.filter((block) => block.id !== action.id);
    case 'update':
      return state.map((block) => (block.id === action.id ? { ...block, ...action.patch } : block));
  }
};
