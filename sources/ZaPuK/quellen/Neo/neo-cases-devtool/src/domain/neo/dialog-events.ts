// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Dialog, MatrixEvent } from './types';

/** Collects the displayed agency and citizen events in envelope order, once per Matrix event ID. */
export const dialogEvents = (dialog: Dialog): MatrixEvent[] => {
  const events = [dialog.agency, ...dialog.responses].flatMap((envelope) => envelope.blocks);
  return [...new Map(events.map((event) => [event.event_id, event])).values()];
};
