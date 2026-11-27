// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { RoomListItem } from '@matrix-g2x/neo-web-sdk/react';
import { useWindowVirtualizer } from '@tanstack/react-virtual';
import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import Card from '../../components/Card/Card';
import Clock from '../../components/Icons/Badge/Clock';

function getColumns() {
  return window.innerWidth < 768 ? 1 : 2;
}

export default function Overview({
  rooms,
  numRooms,
  handleRoomClick,
  loadMore,
}: {
  rooms: RoomListItem[];
  numRooms: number;
  handleRoomClick: (roomId: string) => void;
  loadMore: () => void;
}) {
  const [columns, setColumns] = useState(getColumns);

  useEffect(() => {
    const onResize = () => setColumns(getColumns());
    window.addEventListener('resize', onResize);
    return () => window.removeEventListener('resize', onResize);
  }, []);

  const sortedRooms = rooms.toSorted((a, b) => {
    const aT = a.lastActivity ?? BigInt(0);
    const bT = b.lastActivity ?? BigInt(0);
    if (bT !== aT) return Number(bT - aT);
    const aName = a.name ?? '';
    const bName = b.name ?? '';
    return aName.localeCompare(bName, undefined, { sensitivity: 'base' });
  });

  const rows: RoomListItem[][] = [];
  for (let i = 0; i < sortedRooms.length; i += columns) {
    rows.push(sortedRooms.slice(i, i + columns));
  }

  const listRef = useRef<HTMLDivElement>(null);
  const [scrollMargin, setScrollMargin] = useState(0);

  useLayoutEffect(() => {
    setScrollMargin(listRef.current?.offsetTop ?? 0);
  }, []);

  // oxlint-disable-next-line react/incompatible-library
  const virtualizer = useWindowVirtualizer({
    count: rows.length,
    estimateSize: () => 200,
    overscan: 3,
    scrollMargin,
  });

  const virtualItems = virtualizer.getVirtualItems();

  useEffect(() => {
    const lastItem = virtualItems[virtualItems.length - 1];
    if (!lastItem) return;
    if (lastItem.index >= rows.length - 1 && rooms.length < numRooms) {
      loadMore();
    }
  }, [virtualItems, rows.length, rooms.length, numRooms, loadMore]);

  return (
    <div ref={listRef} className="requests-container">
      <div style={{ height: virtualizer.getTotalSize(), position: 'relative' }}>
        {virtualItems.map((virtualRow) => (
          <div
            key={virtualRow.key}
            ref={virtualizer.measureElement}
            data-index={virtualRow.index}
            className="requests-grid"
            style={{
              position: 'absolute',
              top: 0,
              left: 0,
              width: '100%',
              transform: `translateY(${virtualRow.start - virtualizer.options.scrollMargin}px)`,
            }}
          >
            {rows[virtualRow.index].map((room) => (
              <Card
                key={room.roomId}
                title={room.name ?? 'Unknown'}
                text={room.topic ?? ''}
                badge={{
                  title: 'Vorgang in Prüfung',
                  type: 'neutral',
                  icon: <Clock />,
                }}
                onOpen={() => handleRoomClick(room.roomId)}
              />
            ))}
          </div>
        ))}
      </div>
    </div>
  );
}
