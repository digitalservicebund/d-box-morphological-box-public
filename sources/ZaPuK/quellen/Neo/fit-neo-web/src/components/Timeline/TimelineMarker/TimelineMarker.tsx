// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Warning from '../../Icons/Badge/Warning';
import Done from '../../Icons/Done';
import './TimelineMarker.css';

interface TimelineMarkerProps {
  selected: boolean;
  status?: 'default' | 'warning' | 'success';
  isNew?: boolean;
}

export default function TimelineMarker({
  selected,
  status,
  isNew,
}: TimelineMarkerProps) {
  const classes = [
    'timeline-marker',
    selected ? 'timeline-marker-selected' : null,
    status === 'warning' ? 'timeline-marker-warning' : null,
    status === 'success' ? 'timeline-marker-success' : null,
    isNew ? 'timeline-marker-new' : null,
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <div className={classes} aria-hidden="true">
      <div className="timeline-marker-icon">
        {status === 'warning' ? <Warning /> : <Done />}
      </div>
    </div>
  );
}
