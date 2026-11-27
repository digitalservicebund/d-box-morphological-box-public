// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { useAuth } from '../features/auth/auth-context';
import { WorkspaceProvider } from '../features/cases/WorkspaceProvider';
import { AppLayout } from '../layout/AppLayout';

export const Workspace: FC = () => {
  const { profile } = useAuth();
  return (
    <WorkspaceProvider key={profile?.mxid ?? 'logged-out'}>
      <AppLayout />
    </WorkspaceProvider>
  );
};
