// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { MatrixService } from './matrix-service';
import type { SyncUpdate } from './types';

export interface SyncCallbacks {
  active: () => boolean;
  update: (value: SyncUpdate) => Promise<void>;
  connection: (connected: boolean, cause?: unknown) => void;
}

/** Runs one cancellable SDK sync loop; the client worker owns network cancellation. */
export const synchronize = async (client: MatrixService, callbacks: SyncCallbacks): Promise<void> => {
  let since: string | undefined;
  let offline = false;
  while (callbacks.active()) {
    try {
      const update = await client.sync(since);
      if (!callbacks.active()) return;
      since = update.nextBatch;
      callbacks.connection(true, offline ? 'restored' : undefined);
      offline = false;
      await callbacks.update(update);
    } catch (cause) {
      if (!callbacks.active()) return;
      if (!offline) callbacks.connection(false, cause);
      offline = true;
      await new Promise((resolve) => setTimeout(resolve, 5000));
    }
  }
};
