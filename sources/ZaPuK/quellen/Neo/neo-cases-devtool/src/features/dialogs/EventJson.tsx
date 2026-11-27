// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useMemo, type FC } from 'react';
import type { MatrixEvent } from '../../domain/neo/types';
import { Textarea } from '../../components/ui/Textarea';

export interface EventJsonProps {
  event: MatrixEvent;
  hideLabel?: boolean;
}

export const EventJson: FC<EventJsonProps> = ({ event, hideLabel }) => {
  const json = useMemo(() => JSON.stringify(event, null, 2), [event]);
  return (
    <Textarea
      label={event.event_id}
      value={json}
      readOnly
      monospace
      hideLabel={hideLabel}
      rows={Math.min(22, Math.max(8, json.split('\n').length))}
    />
  );
};
