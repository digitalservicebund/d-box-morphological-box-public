// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC, PropsWithChildren } from 'react';
import { AuthContext } from './auth-context';
import { useAuthState } from './use-auth-state';

export const AuthProvider: FC<PropsWithChildren> = ({ children }) => {
  const state = useAuthState();
  return <AuthContext.Provider value={state}>{children}</AuthContext.Provider>;
};
