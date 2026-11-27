// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

const prefix = '#/cases/';

/** Encodes opaque Matrix room IDs without treating their characters as URL syntax. */
export const caseHash = (id: string | null): string => (id ? `${prefix}${encodeURIComponent(id)}` : '');

/** Reads a case route safely, including Matrix v12 IDs without a server-name suffix. */
export const caseIdFromHash = (hash: string): string | null => {
  if (!hash.startsWith(prefix)) return null;
  try {
    const id = decodeURIComponent(hash.slice(prefix.length));
    return /^![^\s]+$/.test(id) ? id : null;
  } catch {
    return null;
  }
};

export const replaceCaseHash = (id: string | null): void => {
  window.history.replaceState(null, '', `${window.location.pathname}${window.location.search}${caseHash(id)}`);
};
