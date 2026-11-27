// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import {
  ClientState,
  type RoomListStore,
  useClientStore,
  usePollForStore,
} from '@matrix-g2x/neo-web-sdk/react';
import { useTranslation } from 'react-i18next';
import { LoadingSpinner } from '../components/LoadingSpinner/LoadingSpinner';
import AppLayout from './AppLayout';
import './MainView.css';
import AuthenticatedApp from './MainView/AuthenticatedApp';

function App() {
  const { t } = useTranslation();
  const clientStore = useClientStore();
  const isAuthenticated = clientStore.clientState === ClientState.LoggedIn;

  const {
    store: roomListStore,
    loading: loadingRoomList,
    timeoutReached: timeoutRoomList,
  } = usePollForStore<RoomListStore>(
    () => clientStore.getRoomListStore(),
    5000,
  );

  if (!isAuthenticated) {
    return (
      <AppLayout>
        <p>{t('loadingRooms', 'Loading replies...')}</p>
      </AppLayout>
    );
  }

  if (loadingRoomList) {
    return (
      <AppLayout>
        <LoadingSpinner text={t('loadingRoom', 'Loading room list ...')} />
      </AppLayout>
    );
  }
  if (timeoutRoomList || !roomListStore) {
    return (
      <AppLayout>
        {t(
          'failedToLoadRooms',
          'Error while loading rooms. Please reload the page.',
        )}
      </AppLayout>
    );
  }

  return <AuthenticatedApp roomListStore={roomListStore} />;
}

export default App;
