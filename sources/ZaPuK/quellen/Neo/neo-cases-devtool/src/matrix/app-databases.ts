// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

/** Limits resets to this application's stores, including orphaned SDK sessions. */
export const isAppDatabase = (name: string): boolean =>
  name === 'neo-cases-devtool' || name.startsWith('neo-cases-devtool-');

const deleteDatabase = (name: string): Promise<void> =>
  new Promise((resolve, reject) => {
    const request = indexedDB.deleteDatabase(name);
    const timeout = setTimeout(
      () => reject(new Error('Close other tabs of this app and retry clearing local data.')),
      10_000,
    );
    request.onsuccess = () => {
      clearTimeout(timeout);
      resolve();
    };
    request.onerror = () => {
      clearTimeout(timeout);
      reject(request.error ?? new Error('Could not clear local data.'));
    };
    // A reset broadcast closes cooperating tabs' SDK workers; wait for their connections to close.
    request.onblocked = () => undefined;
  });

/** Deletes app sessions, pending sends, media and SDK databases without touching unrelated origin data. */
export const clearAppDatabases = async (): Promise<void> => {
  const databases = await indexedDB.databases();
  await deleteDatabase('neo-cases-devtool');
  const names = databases.flatMap(({ name }) =>
    name && name !== 'neo-cases-devtool' && isAppDatabase(name) ? [name] : [],
  );
  await Promise.all(names.map(deleteDatabase));
};
