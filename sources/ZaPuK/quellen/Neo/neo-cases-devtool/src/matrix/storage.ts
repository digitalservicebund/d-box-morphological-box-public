// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { z } from 'zod';
import { matrixSessionSchema, profileSchema } from './types';

const database = 'neo-cases-devtool';
const sessionSchema = z.compile(
  z.object({
    homeserver: z.url(),
    database: z.string().startsWith('neo-cases-devtool-'),
    session: matrixSessionSchema,
    profile: profileSchema,
  }),
);
export type StoredSession = z.infer<typeof sessionSchema>;

const open = (): Promise<IDBDatabase> =>
  new Promise((resolve, reject) => {
    const request = indexedDB.open(database, 1);
    request.onupgradeneeded = () => request.result.createObjectStore('settings');
    request.onsuccess = () => {
      request.result.onversionchange = () => request.result.close();
      resolve(request.result);
    };
    request.onerror = () => reject(request.error ?? new Error('IndexedDB could not be opened.'));
  });

/** Reads application-owned state; the Rust SDK owns its separate Matrix stores. */
export const readStored = async (key: string): Promise<unknown> => {
  const db = await open();
  try {
    return await new Promise((resolve, reject) => {
      const request = db.transaction('settings').objectStore('settings').get(key);
      request.onsuccess = () => resolve(request.result as unknown);
      request.onerror = () => reject(request.error ?? new Error('Could not read browser storage.'));
    });
  } finally {
    db.close();
  }
};

/** Commits a storage change before acknowledging it to callers. Undefined deletes the key. */
export const writeStored = async (key: string, value: unknown): Promise<void> => {
  const db = await open();
  try {
    await new Promise<void>((resolve, reject) => {
      const transaction = db.transaction('settings', 'readwrite');
      const store = transaction.objectStore('settings');
      if (value === undefined) store.delete(key);
      else store.put(value, key);
      transaction.oncomplete = () => resolve();
      transaction.onabort = transaction.onerror = () =>
        reject(transaction.error ?? new Error('Could not save browser state.'));
    });
  } finally {
    db.close();
  }
};

let sessionWrites: Promise<void> = Promise.resolve();

/** Serializes token/profile updates so a slow write cannot restore stale refresh tokens. */
export const writeSession = (value: StoredSession | undefined, active = () => true): Promise<void> => {
  const next = sessionWrites
    .catch(() => undefined)
    .then(async () => {
      if (active()) await writeStored('session', value);
    });
  sessionWrites = next;
  return next;
};

export const readSession = async (): Promise<StoredSession | undefined> => {
  const value = await readStored('session');
  if (value === undefined) return;
  return sessionSchema.parse(value);
};

/** Clears application data belonging to a logged-out account, without touching other accounts. */
export const clearPendingSends = async (account: string): Promise<void> => {
  const db = await open();
  try {
    await new Promise<void>((resolve, reject) => {
      const transaction = db.transaction('settings', 'readwrite');
      const request = transaction.objectStore('settings').openCursor();
      request.onsuccess = () => {
        const cursor = request.result;
        if (!cursor) return;
        if (typeof cursor.key === 'string' && cursor.key.startsWith(`send:${account}:`)) cursor.delete();
        cursor.continue();
      };
      transaction.oncomplete = () => resolve();
      transaction.onabort = transaction.onerror = () =>
        reject(transaction.error ?? new Error('Could not clear pending sends.'));
    });
  } finally {
    db.close();
  }
};

export const clearSdkStorage = async (prefix: string): Promise<void> => {
  const databases = await indexedDB.databases();
  await Promise.all(
    databases
      .filter((entry) => entry.name?.startsWith(prefix))
      .map(
        (entry) =>
          new Promise<void>((resolve, reject) => {
            const request = indexedDB.deleteDatabase(entry.name!);
            request.onsuccess = () => resolve();
            request.onerror = () => reject(request.error ?? new Error('Could not clear Matrix storage.'));
            request.onblocked = () => reject(new Error('Close other tabs using this session to clear its storage.'));
          }),
      ),
  );
};
