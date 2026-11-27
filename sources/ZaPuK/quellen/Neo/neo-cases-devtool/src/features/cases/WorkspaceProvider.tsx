// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useEffect, useMemo, useState, type FC, type PropsWithChildren } from 'react';
import { WorkspaceContext } from './workspace-context';
import { caseHash, caseIdFromHash, replaceCaseHash } from '../../utils/case-route';
import { useNotify } from '../notifications/notification-context';

export const WorkspaceProvider: FC<PropsWithChildren> = ({ children }) => {
  const [selectedId, setSelectedId] = useState<string | null>(() => caseIdFromHash(window.location.hash));
  const [draftOpen, setDraftOpen] = useState(false);
  const notify = useNotify();
  useEffect(() => {
    const navigate = () => {
      const id = caseIdFromHash(window.location.hash);
      if (draftOpen && id !== selectedId) {
        replaceCaseHash(selectedId);
        notify('Close or cancel the current dialog before changing cases.');
        return;
      }
      setSelectedId(id);
    };
    window.addEventListener('hashchange', navigate);
    return () => window.removeEventListener('hashchange', navigate);
  }, [draftOpen, selectedId, notify]);
  const select = useCallback(
    (id: string | null) => {
      if (draftOpen) return;
      setSelectedId(id);
      if (id === null) replaceCaseHash(null);
      else if (window.location.hash !== caseHash(id)) window.location.hash = caseHash(id);
    },
    [draftOpen],
  );
  const openDraft = useCallback(() => setDraftOpen(true), []);
  const closeDraft = useCallback(() => setDraftOpen(false), []);
  const value = useMemo(
    () => ({ selectedId, draftOpen, select, openDraft, closeDraft }),
    [selectedId, draftOpen, select, openDraft, closeDraft],
  );
  return <WorkspaceContext.Provider value={value}>{children}</WorkspaceContext.Provider>;
};
