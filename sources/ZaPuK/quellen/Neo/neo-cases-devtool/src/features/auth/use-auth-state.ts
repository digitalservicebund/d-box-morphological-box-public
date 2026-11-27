// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useEffect, useMemo, useState } from 'react';
import { config } from '../../app/config';
import { MatrixService } from '../../matrix/matrix-service';
import { synchronize } from '../../matrix/synchronize';
import { clearSdkStorage, readSession, type StoredSession } from '../../matrix/storage';
import type { CaseRoom, Profile, SyncUpdate } from '../../matrix/types';
import { errorMessage } from '../../utils/errors';
import { useNotify } from '../notifications/notification-context';
import type { AuthState } from './auth-context';
import { useClientLifecycle } from './use-client-lifecycle';
import { useAccountProfile } from './use-account-profile';
import { persistActiveSession } from './session-persistence';
import { useSessionActions } from './use-session-actions';

export const useAuthState = (): AuthState => {
  const [phase, setPhase] = useState<AuthState['phase']>('restoring');
  const [service, setService] = useState<MatrixService | null>(null);
  const [rooms, setRooms] = useState<CaseRoom[]>([]);
  const [connected, setConnected] = useState(false);
  const lifecycle = useClientLifecycle();
  const { active, stored, generation, listeners } = lifecycle;
  const { profile, setProfile, refreshProfile } = useAccountProfile(lifecycle);
  const notify = useNotify();
  const { logout, clearLocalData } = useSessionActions({
    lifecycle,
    setPhase,
    setService,
    setProfile,
    setRooms,
    setConnected,
  });

  const refreshRooms = useCallback(async () => {
    const client = active.current;
    if (client) {
      const items = await client.rooms();
      if (active.current === client) setRooms(items.sort((a, b) => a.name.localeCompare(b.name)));
    }
  }, []);

  const attach = useCallback(
    async (client: MatrixService, session: StoredSession, epoch: number) => {
      if (!(await persistActiveSession(client, session, lifecycle, epoch, notify))) return;
      setService(client);
      setProfile(session.profile);
      setPhase('loggedIn');
      void synchronize(client, {
        active: () => active.current === client && generation.current === epoch,
        update: async (update) => {
          for (const listener of listeners.current) listener(update);
          await refreshRooms();
        },
        connection: (online, cause) => {
          setConnected(online);
          if (!online) notify(`Synchronization paused: ${errorMessage(cause)}`, 'error');
          if (cause === 'restored') notify('Connection restored.', 'success');
        },
      });
    },
    [refreshRooms, lifecycle, notify],
  );

  useEffect(() => {
    const epoch = ++generation.current;
    const restore = async () => {
      try {
        const session = await readSession();
        if (epoch !== generation.current) return;
        if (!session) {
          setPhase('loggedOut');
          return;
        }
        const client = new MatrixService();
        active.current = client;
        await client.open(session.homeserver, session.database, session.session);
        session.profile = await client.profile().catch(() => session.profile);
        if (epoch !== generation.current) {
          client.dispose();
          return;
        }
        await attach(client, session, epoch);
      } catch (cause) {
        if (epoch !== generation.current) return;
        active.current?.dispose();
        active.current = null;
        setPhase('loggedOut');
        notify(`Session could not be restored: ${errorMessage(cause)}`, 'error');
      }
    };
    void restore();
    return () => {
      ++generation.current;
      active.current?.dispose();
      active.current = null;
    };
  }, [attach, notify]);

  const login = useCallback(
    async (username: string, password: string, homeserver: string) => {
      if (!config.homeservers.includes(homeserver)) throw new Error('Select a configured server.');
      setPhase('loggingIn');
      const epoch = ++generation.current;
      const client = new MatrixService();
      active.current?.dispose();
      active.current = client;
      const database = `neo-cases-devtool-${crypto.randomUUID()}`;
      try {
        await client.open(homeserver, database);
        const session = await client.login(username.trim(), password);
        let nextProfile: Profile;
        try {
          nextProfile = await client.profile();
        } catch {
          if (typeof session.user_id !== 'string') throw new Error('The server did not return an account ID.');
          nextProfile = {
            mxid: session.user_id,
            displayName: null,
            avatarUrl: null,
            canSetDisplayName: true,
            canSetAvatar: true,
          };
        }
        const data: StoredSession = { database, homeserver, session, profile: nextProfile };
        await attach(client, data, epoch);
        if (generation.current === epoch) notify('Logged in.', 'success');
      } catch (cause) {
        client.dispose();
        if (active.current === client) active.current = null;
        if (generation.current === epoch) setPhase('loggedOut');
        void clearSdkStorage(database).catch(() => undefined);
        throw cause;
      }
    },
    [attach, notify],
  );

  const subscribe = useCallback((listener: (update: SyncUpdate) => void) => {
    listeners.current.add(listener);
    return () => {
      listeners.current.delete(listener);
    };
  }, []);
  const homeserver = stored.current?.homeserver ?? config.homeservers[0];
  return useMemo(
    () => ({
      phase,
      profile,
      service,
      rooms,
      connected,
      homeserver,
      login,
      logout,
      clearLocalData,
      refreshRooms,
      refreshProfile,
      subscribe,
    }),
    [
      phase,
      profile,
      service,
      rooms,
      connected,
      homeserver,
      login,
      logout,
      clearLocalData,
      refreshRooms,
      refreshProfile,
      subscribe,
    ],
  );
};
