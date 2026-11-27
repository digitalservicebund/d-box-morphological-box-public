// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { createContext, useContext } from 'react';
import type { MatrixService } from '../../matrix/matrix-service';
import type { CaseRoom, Profile, SyncUpdate } from '../../matrix/types';

export interface AuthState {
  phase: 'restoring' | 'loggedOut' | 'loggingIn' | 'loggedIn' | 'loggingOut';
  profile: Profile | null;
  service: MatrixService | null;
  rooms: CaseRoom[];
  connected: boolean;
  homeserver: string;
  login: (username: string, password: string, homeserver: string) => Promise<void>;
  logout: () => Promise<void>;
  clearLocalData: () => Promise<void>;
  refreshRooms: () => Promise<void>;
  refreshProfile: () => Promise<void>;
  subscribe: (listener: (update: SyncUpdate) => void) => () => void;
}
export const AuthContext = createContext<AuthState | undefined>(undefined);
export const useAuth = (): AuthState => {
  const value = useContext(AuthContext);
  if (!value) throw new Error('AuthProvider is missing.');
  return value;
};
