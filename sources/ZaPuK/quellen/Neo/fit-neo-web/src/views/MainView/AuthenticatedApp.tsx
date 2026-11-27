// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { RoomListStore } from '@matrix-g2x/neo-web-sdk/react';
import { useSyncExternalStore } from 'react';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router';
import Badge from '../../components/Badge/Badge';
import Bell from '../../components/Icons/Bell';
import ChevronDown from '../../components/Icons/ChevronDown';
import OneOneFiveFooter from '../../components/OneOneFiveFooter/OneOneFiveFooter';
import TabBar from '../../components/TabBar/TabBar';
import AppLayout from '../AppLayout';
import { MAIN_TABS } from '../constants';
import NoRequests from './NoRequests';
import Overview from './Overview';

type AuthenticatedAppProps = {
  roomListStore: RoomListStore;
};

export default function AuthenticatedApp({
  roomListStore,
}: AuthenticatedAppProps) {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const handleRoomClick = (roomId: string) => {
    roomListStore?.setActiveRoom(roomId);
    void navigate(`/case/${encodeURIComponent(roomId)}`);
  };

  const { rooms, numRooms } = useSyncExternalStore(
    roomListStore.subscribe,
    roomListStore.getSnapshot,
  );

  return (
    <AppLayout>
      <TabBar tabs={MAIN_TABS(t)} />
      {/* page title */}
      <div className="page-title">
        <div className="horizontal-end-aligned">
          <div className="buttons">
            <Badge
              type="neutral"
              icon={<Bell />}
              title={t('notifications', 'Meldungen')}
            />
            <ChevronDown />
          </div>
        </div>
        <h1>{t('vorgaenge.title', 'Vorgänge')}</h1>
      </div>
      {/* main content */}
      <div className="main-content">
        {numRooms > 0 ? (
          <Overview
            rooms={rooms}
            numRooms={numRooms}
            handleRoomClick={handleRoomClick}
            loadMore={roomListStore.loadMore}
          />
        ) : (
          <NoRequests />
        )}
      </div>
      <OneOneFiveFooter />
    </AppLayout>
  );
}
