// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useState, type FC } from 'react';
import { MdPersonOutline } from 'react-icons/md';
import { isInquiry } from '../../domain/neo/identifiers';
import type { Dialog } from '../../domain/neo/types';
import { BlockRenderer } from '../../components/blocks/BlockRenderer';
import styles from './DialogCard.module.css';
import { AgencyHeader } from './AgencyHeader';
import { DialogEventsDialog } from './DialogEventsDialog';

export interface DialogCardProps {
  dialog: Dialog;
}

export const DialogCard: FC<DialogCardProps> = ({ dialog }) => {
  const inquiry = dialog.agency.blocks.find((block) => isInquiry(block.type));
  const [inspecting, setInspecting] = useState(false);
  const closeEvents = useCallback(() => setInspecting(false), []);
  return (
    <>
      <m3-card variant="outlined" width="full" className={styles.card}>
        <AgencyHeader sender={dialog.agency.sender} timestamp={dialog.timestamp} />
        <div className={styles.blocks}>
          {dialog.agency.blocks.map((event) => (
            <BlockRenderer key={event.event_id} event={event} />
          ))}
        </div>
        {dialog.responses.length > 0 && (
          <div className={styles.responses}>
            {dialog.responses.map((response) => (
              <section key={response.id} className={styles.response}>
                <div className={styles.header}>
                  <MdPersonOutline aria-hidden="true" />
                  <div>
                    <strong>Citizen</strong>
                    <span className={styles.sender}>{response.sender}</span>
                  </div>
                  <time dateTime={new Date(response.timestamp).toISOString()}>
                    {new Date(response.timestamp).toLocaleString()}
                  </time>
                </div>
                <div className={styles.blocks}>
                  {response.blocks.map((event) => (
                    <BlockRenderer key={event.event_id} event={event} inquiry={inquiry} />
                  ))}
                </div>
              </section>
            ))}
          </div>
        )}
        <div slot="actions" className={styles.status}>
          {inquiry
            ? dialog.responses.length
              ? 'Citizen responded'
              : 'Awaiting citizen response'
            : 'Informative dialog'}
          <span className={styles.cardActions}>
            <m3-button variant="text" className={styles.eventLink} onClick={() => setInspecting(true)}>
              View events
            </m3-button>
            <span>Read-only</span>
          </span>
        </div>
      </m3-card>
      {inspecting && <DialogEventsDialog dialog={dialog} onClose={closeEvents} />}
    </>
  );
};
