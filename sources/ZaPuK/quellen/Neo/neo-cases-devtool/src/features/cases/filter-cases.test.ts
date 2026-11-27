// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it } from 'vite-plus/test';
import type { CaseRoom } from '../../matrix/types';
import { filterCases, type CaseListOptions } from './filter-cases';

const rooms: CaseRoom[] = [
  {
    id: '!z:test',
    name: 'Case 10',
    topic: 'Beta agency',
    encrypted: false,
    invited: false,
    createdAt: 0,
    updatedAt: 300,
  },
  {
    id: '!a:test',
    name: 'case 2',
    topic: 'Gamma agency',
    encrypted: true,
    invited: false,
    createdAt: 100,
    updatedAt: 100,
  },
  {
    id: '!m:test',
    name: 'Case 3',
    topic: 'Alpha agency',
    encrypted: true,
    invited: false,
    createdAt: 200,
    updatedAt: 200,
  },
  {
    id: '!unknown:test',
    name: 'Invitation',
    topic: null,
    encrypted: false,
    invited: true,
    createdAt: null,
    updatedAt: null,
  },
];
const options: CaseListOptions = { query: '', sortBy: 'title', direction: 'asc' };
const ids = (patch: Partial<CaseListOptions>) => filterCases(rooms, { ...options, ...patch }).map((room) => room.id);

it('searches title, agency, and room ID without case sensitivity or surrounding spaces', () => {
  expect(ids({ query: '  CASE 2  ' })).toEqual(['!a:test']);
  expect(ids({ query: 'alpha agency' })).toEqual(['!m:test']);
  expect(ids({ query: '!unknown:TEST' })).toEqual(['!unknown:test']);
  expect(ids({ query: 'missing' })).toEqual([]);
});

it('sorts naturally by title and agency while leaving missing agencies last in either direction', () => {
  expect(ids({})).toEqual(['!a:test', '!m:test', '!z:test', '!unknown:test']);
  expect(ids({ sortBy: 'agency' })).toEqual(['!m:test', '!z:test', '!a:test', '!unknown:test']);
  expect(ids({ sortBy: 'agency', direction: 'desc' })).toEqual(['!a:test', '!z:test', '!m:test', '!unknown:test']);
});

it('distinguishes creation order from last update and keeps unknown dates last', () => {
  expect(ids({ sortBy: 'created' })).toEqual(['!z:test', '!a:test', '!m:test', '!unknown:test']);
  expect(ids({ sortBy: 'created', direction: 'desc' })).toEqual(['!m:test', '!a:test', '!z:test', '!unknown:test']);
  expect(ids({ sortBy: 'updated' })).toEqual(['!a:test', '!m:test', '!z:test', '!unknown:test']);
  expect(ids({ sortBy: 'updated', direction: 'desc' })).toEqual(['!z:test', '!m:test', '!a:test', '!unknown:test']);
});

it('does not mutate SDK room order and uses stable tie-breakers', () => {
  const original = [...rooms];
  ids({ sortBy: 'created' });
  expect(rooms).toEqual(original);
  const ties = [
    { ...rooms[0]!, id: '!b:test' },
    { ...rooms[0]!, id: '!a:test' },
  ];
  expect(filterCases(ties, options).map((room) => room.id)).toEqual(['!a:test', '!b:test']);
});
