// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { createContext, useContext } from 'react';

export interface WorkspaceState {
  selectedId: string | null;
  draftOpen: boolean;
  select: (id: string | null) => void;
  openDraft: () => void;
  closeDraft: () => void;
}
export const WorkspaceContext = createContext<WorkspaceState | undefined>(undefined);
export const useWorkspace = (): WorkspaceState => {
  const value = useContext(WorkspaceContext);
  if (!value) throw new Error('WorkspaceProvider is missing.');
  return value;
};
