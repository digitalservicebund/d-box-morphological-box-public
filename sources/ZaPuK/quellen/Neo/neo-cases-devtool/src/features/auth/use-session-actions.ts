// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useEffect, useRef } from 'react';
import { clearAppDatabases } from '../../matrix/app-databases';
import { clearPendingSends, clearSdkStorage, writeSession } from '../../matrix/storage';
import type { MatrixService } from '../../matrix/matrix-service';
import type { CaseRoom, Profile } from '../../matrix/types';
import { replaceCaseHash } from '../../utils/case-route';
import { errorMessage } from '../../utils/errors';
import { useNotify } from '../notifications/notification-context';
import type { useClientLifecycle } from './use-client-lifecycle';
import type { AuthState } from './auth-context';

export interface SessionActionOptions {
  lifecycle: ReturnType<typeof useClientLifecycle>;
  setPhase: (phase: AuthState['phase']) => void;
  setService: (client: MatrixService | null) => void;
  setProfile: (profile: Profile | null) => void;
  setRooms: (rooms: CaseRoom[]) => void;
  setConnected: (connected: boolean) => void;
}
const channelName = 'neo-agency-local-data';
const broadcast = (type: 'reset-start' | 'reset-finished') => {
  if (typeof BroadcastChannel === 'undefined') return;
  const channel = new BroadcastChannel(channelName);
  channel.postMessage({ type });
  channel.close();
};
const revokeSession = async (client: MatrixService | null): Promise<void> => {
  if (!client) return;
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    await Promise.race([
      client.logout(),
      new Promise<never>((_, reject) => {
        timer = setTimeout(() => reject(new Error('Server logout timed out.')), 10_000);
      }),
    ]);
  } finally {
    clearTimeout(timer);
  }
};

export const useSessionActions = ({
  lifecycle,
  setPhase,
  setService,
  setProfile,
  setRooms,
  setConnected,
}: SessionActionOptions) => {
  const busy = useRef(false);
  const notify = useNotify();
  const { active, stored, generation } = lifecycle;
  const reset = useCallback(
    (phase: AuthState['phase']) => {
      ++generation.current;
      active.current?.dispose();
      active.current = null;
      stored.current = undefined;
      setService(null);
      setProfile(null);
      setRooms([]);
      setConnected(false);
      setPhase(phase);
      replaceCaseHash(null);
    },
    [active, stored, generation, setService, setProfile, setRooms, setConnected, setPhase],
  );
  useEffect(() => {
    if (typeof BroadcastChannel === 'undefined') return;
    const channel = new BroadcastChannel(channelName);
    channel.onmessage = ({ data }: MessageEvent<{ type?: string }>) => {
      if (data.type === 'reset-start') {
        busy.current = true;
        reset('loggingOut');
      }
      if (data.type === 'reset-finished') {
        busy.current = false;
        setPhase('loggedOut');
      }
    };
    return () => channel.close();
  }, [reset, setPhase]);
  const endSession = useCallback(
    async (allData: boolean) => {
      if (busy.current) return;
      busy.current = true;
      ++generation.current;
      setPhase('loggingOut');
      const client = active.current;
      const session = stored.current;
      let revoked = true;
      try {
        await revokeSession(client);
      } catch {
        revoked = false;
      }
      if (allData) broadcast('reset-start');
      reset('loggingOut');
      try {
        await writeSession(undefined);
        if (allData) await clearAppDatabases();
        else if (session) {
          await clearPendingSends(session.profile.mxid);
          await clearSdkStorage(session.database);
        }
        if (!revoked)
          notify(
            `${allData ? 'Local data cleared' : 'Local logout completed'}, but server logout could not be confirmed.`,
            'error',
          );
        else if (allData) notify('Local user data cleared and logged out.', 'success');
      } catch (cause) {
        notify(`Logged out locally, but storage cleanup failed: ${errorMessage(cause)}`, 'error');
      } finally {
        if (allData) broadcast('reset-finished');
        busy.current = false;
        setPhase('loggedOut');
      }
    },
    [active, stored, generation, setPhase, reset, notify],
  );
  const logout = useCallback(() => endSession(false), [endSession]);
  const clearLocalData = useCallback(() => endSession(true), [endSession]);
  return { logout, clearLocalData };
};
