// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { AuthProvider } from '../features/auth/AuthProvider';
import { NotificationProvider } from '../features/notifications/NotificationProvider';
import { Workspace } from './Workspace';

export const App: FC = () => (
  <NotificationProvider>
    <AuthProvider>
      <Workspace />
    </AuthProvider>
  </NotificationProvider>
);
