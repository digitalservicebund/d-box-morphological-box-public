// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

/** Starts a local JSON download and releases the temporary URL after the browser accepts it. */
export const downloadJson = (value: unknown, name: string): void => {
  const blob = new Blob([JSON.stringify(value, null, 2)], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  const filename = name.replace(/[^\p{L}\p{N}._-]+/gu, '-').slice(0, 100) || 'case';
  link.href = url;
  link.download = `${filename}.json`;
  link.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
};
