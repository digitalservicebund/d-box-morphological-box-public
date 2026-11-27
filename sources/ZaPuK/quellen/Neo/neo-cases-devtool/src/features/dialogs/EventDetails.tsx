// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useState, type FC } from 'react';
import type { MatrixEvent } from '../../domain/neo/types';
import { EventJson } from './EventJson';
import styles from './DialogEventsDialog.module.css';

export interface EventDetailsProps {
  event: MatrixEvent;
}

export const EventDetails: FC<EventDetailsProps> = ({ event }) => {
  const [expanded, setExpanded] = useState(false);
  return (
    <details className={styles.event} onToggle={(change) => setExpanded(change.currentTarget.open)}>
      <summary>{event.event_id}</summary>
      {expanded && (
        <div className={styles.json}>
          <EventJson event={event} hideLabel />
        </div>
      )}
    </details>
  );
};
