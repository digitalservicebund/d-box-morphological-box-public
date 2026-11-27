// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { CaseRoom } from '../../matrix/types';

export const caseSortLabels = {
  agency: 'Agency',
  title: 'Title',
  updated: 'Last update',
  created: 'Creation',
} as const;
export type CaseSortKey = keyof typeof caseSortLabels;
export interface CaseListOptions {
  query: string;
  sortBy: CaseSortKey;
  direction: 'asc' | 'desc';
}
const collator = new Intl.Collator(undefined, { numeric: true, sensitivity: 'base' });

const sortValue = (room: CaseRoom, key: CaseSortKey): string | number | null => {
  switch (key) {
    case 'agency':
      return room.topic?.trim() || null;
    case 'title':
      return room.name;
    case 'updated':
      return room.updatedAt;
    case 'created':
      return room.createdAt;
  }
};

const compare = (left: CaseRoom, right: CaseRoom, options: CaseListOptions): number => {
  const a = sortValue(left, options.sortBy);
  const b = sortValue(right, options.sortBy);
  // Missing server timestamps/agency metadata stay last in either direction.
  if (a === null && b !== null) return 1;
  if (b === null && a !== null) return -1;
  const primary =
    typeof a === 'number' && typeof b === 'number' ? a - b : collator.compare(String(a ?? ''), String(b ?? ''));
  return (
    primary * (options.direction === 'asc' ? 1 : -1) ||
    collator.compare(left.name, right.name) ||
    left.id.localeCompare(right.id)
  );
};

/** Filters case titles, agency names and room IDs, then applies stable, non-mutating sorting. */
export const filterCases = (rooms: readonly CaseRoom[], options: CaseListOptions): CaseRoom[] => {
  const query = options.query.trim().toLocaleLowerCase();
  return rooms
    .filter((room) => [room.name, room.topic ?? '', room.id].some((value) => value.toLocaleLowerCase().includes(query)))
    .sort((a, b) => compare(a, b, options));
};
