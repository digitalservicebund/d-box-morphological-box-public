// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

const key = 'neo-cases-devtool:homeserver';

/** Restores an allowed server preference, falling back to the first configured URL. */
export const readServerPreference = (homeservers: readonly [string, ...string[]]): string => {
  try {
    const saved = localStorage.getItem(key);
    if (saved && homeservers.includes(saved)) return saved;
  } catch {
    // Storage can be blocked by browser privacy settings.
  }
  return homeservers[0];
};

/** Remembers the selection without requiring browser storage to be available. */
export const writeServerPreference = (homeserver: string): void => {
  try {
    localStorage.setItem(key, homeserver);
  } catch {
    // A preference must not prevent login when browser storage is unavailable.
  }
};
