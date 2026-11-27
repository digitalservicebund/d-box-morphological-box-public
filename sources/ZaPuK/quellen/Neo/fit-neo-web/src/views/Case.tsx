// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import {
  type CaseStore,
  ClientState,
  RoomListStore,
  useClientStore,
  usePollForStore,
} from '@matrix-g2x/neo-web-sdk/react';
import { useCallback } from 'react';
import { useTranslation } from 'react-i18next';
import { useParams } from 'react-router';
import type { LoadMediaContent } from '../hooks/useMediaObjectUrl';
import AppLayout from './AppLayout';
import AuthenticatedCase from './Case/AuthenticatedCase/AuthenticatedCase';

export default function Case() {
  const { t } = useTranslation();
  const clientStore = useClientStore();
  const loadMediaContent = useCallback<LoadMediaContent>(
    (uri, signal) => clientStore.getMediaContent(uri, signal),
    [clientStore],
  );
  const isAuthenticated = clientStore.clientState === ClientState.LoggedIn;
  const { roomId } = useParams<{ roomId: string }>();
  const decodedRoomId = roomId ? decodeURIComponent(roomId) : '';

  const {
    store: caseStore,
    loading: loadingCase,
    timeoutReached: timeoutCase,
  } = usePollForStore<CaseStore>(
    () => clientStore.getCaseStore(decodedRoomId),
    5000,
  );
  const {
    store: roomListStore,
    loading: loadingRoomList,
    timeoutReached: timeoutRoomList,
  } = usePollForStore<RoomListStore>(() => {
    const store = clientStore.getRoomListStore();
    if (!store) return undefined;
    // Also wait for the target room's display name to be loaded so the page
    // never renders with a raw room ID or causes a two-phase flash.
    const rooms = store.getSnapshot().rooms;
    const room = rooms.find((r) => r.roomId === decodedRoomId);
    if (!room || !room.name) return undefined;
    return store;
  }, 5000);

  if (decodedRoomId && clientStore.hasJoinFailed(decodedRoomId)) {
    return (
      <AppLayout>
        <p>
          {t(
            'case.joinFailed',
            'Dem Antrag konnte nicht beigetreten werden. Bitte versuchen Sie es später erneut.',
          )}
        </p>
      </AppLayout>
    );
  }

  if (!isAuthenticated || loadingCase || loadingRoomList) {
    return (
      <AppLayout>
        <p>{t('case.loading', 'Vorgang wird geladen...')}</p>
      </AppLayout>
    );
  }

  if (
    timeoutRoomList ||
    timeoutCase ||
    !caseStore ||
    !roomListStore ||
    !roomId
  ) {
    return (
      <AppLayout>
        <p>{t('case.notFound', 'Antrag wurde nicht gefunden.')}</p>
      </AppLayout>
    );
  }

  return (
    <AuthenticatedCase
      roomId={decodedRoomId}
      roomListStore={roomListStore}
      caseStore={caseStore}
      loadMediaContent={loadMediaContent}
      onUploadMedia={(data, mimeType) =>
        clientStore.uploadMedia(data, mimeType)
      }
    />
  );
}
