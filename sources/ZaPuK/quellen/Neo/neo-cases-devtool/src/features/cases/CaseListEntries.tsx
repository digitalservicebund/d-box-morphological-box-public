// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useRef, useState, type FC } from 'react';
import { MdLockOutline, MdLockOpen, MdFolderOpen } from 'react-icons/md';
import type { CaseRoom } from '../../matrix/types';
import { errorMessage } from '../../utils/errors';
import { useAuth } from '../auth/auth-context';
import { useNotify } from '../notifications/notification-context';
import { useWorkspace } from './workspace-context';
import styles from './CaseList.module.css';

export interface CaseListEntriesProps {
  rooms: CaseRoom[];
  resetKey: string;
  filtered: boolean;
}

export const CaseListEntries: FC<CaseListEntriesProps> = ({ rooms, resetKey, filtered }) => {
  const [joining, setJoining] = useState<string>();
  const ref = useRef<HTMLDivElement>(null);
  const { service, refreshRooms, connected, phase } = useAuth();
  const { selectedId, select, draftOpen } = useWorkspace();
  const notify = useNotify();
  useEffect(() => {
    if (ref.current) ref.current.scrollTop = 0;
  }, [resetKey]);
  const choose = async (room: CaseRoom) => {
    if (draftOpen || joining) return;
    if (room.invited && service) {
      setJoining(room.id);
      try {
        await service.join(room.id);
        await refreshRooms();
      } catch (cause) {
        notify(`Could not join case: ${errorMessage(cause)}`, 'error');
        return;
      } finally {
        setJoining(undefined);
      }
    }
    select(room.id);
  };
  return (
    <div ref={ref} className={styles.scrollArea}>
      {phase === 'loggedIn' && (
        <>
          <m3-list className={styles.list}>
            {rooms.map((room) => {
              const separator = room.citizen?.indexOf(':') ?? -1;
              const localEnd = separator < 0 ? room.citizen?.length : separator;
              return (
                <m3-list-item
                  key={room.id}
                  className={room.encrypted ? styles.encrypted : styles.unencrypted}
                  selected={selectedId === room.id}
                  lines={room.citizen ? '3' : '2'}
                  shape="rounded"
                  onClick={() => void choose(room)}
                  aria-current={selectedId === room.id ? 'true' : undefined}
                >
                  <span slot="leading">
                    {room.encrypted ? (
                      <MdLockOutline aria-label="Encrypted" />
                    ) : (
                      <MdLockOpen aria-label="Unencrypted" />
                    )}
                  </span>
                  {room.name}
                  <span slot="supporting-text">
                    {joining === room.id ? 'Joining…' : room.invited ? 'Invitation' : room.topic || room.id}
                  </span>
                  {room.citizen && (
                    <span slot="tertiary-text" className={styles.citizen} title={room.citizen}>
                      <strong>{room.citizen.slice(0, localEnd)}</strong>
                      {room.citizen.slice(localEnd)}
                    </span>
                  )}
                </m3-list-item>
              );
            })}
          </m3-list>
          {!rooms.length && (
            <div className={styles.empty}>
              <MdFolderOpen size={28} aria-hidden="true" />
              <p>
                {filtered
                  ? 'No cases match your search.'
                  : connected
                    ? 'Your cases will appear here.'
                    : 'Synchronizing cases…'}
              </p>
            </div>
          )}
        </>
      )}
    </div>
  );
};
