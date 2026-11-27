// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { applyDiff, debounce } from './utils';

// Identity mapper — keeps raw values as-is for test clarity
const id = <T>(x: T) => x;

describe('applyDiff', () => {
	it('PushBack appends a value', () => {
		const result = applyDiff(
			[1, 2],
			[{ tag: 'PushBack', inner: { value: 3 } }],
			id,
		);
		expect(result).toEqual([1, 2, 3]);
	});

	it('PushFront prepends a value', () => {
		const result = applyDiff(
			[2, 3],
			[{ tag: 'PushFront', inner: { value: 1 } }],
			id,
		);
		expect(result).toEqual([1, 2, 3]);
	});

	it('Insert at a given index', () => {
		const result = applyDiff(
			['a', 'c'],
			[{ tag: 'Insert', inner: { index: 1, value: 'b' } }],
			id,
		);
		expect(result).toEqual(['a', 'b', 'c']);
	});

	it('Remove by index', () => {
		const result = applyDiff(
			['a', 'b', 'c'],
			[{ tag: 'Remove', inner: { index: 1 } }],
			id,
		);
		expect(result).toEqual(['a', 'c']);
	});

	it('Set overwrites at index', () => {
		const result = applyDiff(
			[1, 2, 3],
			[{ tag: 'Set', inner: { index: 1, value: 99 } }],
			id,
		);
		expect(result).toEqual([1, 99, 3]);
	});

	it('Clear empties the array', () => {
		const result = applyDiff([1, 2, 3], [{ tag: 'Clear' }], id);
		expect(result).toEqual([]);
	});

	it('Truncate shortens the array', () => {
		const result = applyDiff(
			[1, 2, 3, 4],
			[{ tag: 'Truncate', inner: { length: 2 } }],
			id,
		);
		expect(result).toEqual([1, 2]);
	});

	it('Reset replaces the entire array', () => {
		const result = applyDiff(
			[1, 2, 3],
			[{ tag: 'Reset', inner: { values: [10, 20] } }],
			id,
		);
		expect(result).toEqual([10, 20]);
	});

	it('Append adds multiple values', () => {
		const result = applyDiff(
			[1],
			[{ tag: 'Append', inner: { values: [2, 3] } }],
			id,
		);
		expect(result).toEqual([1, 2, 3]);
	});

	it('applies the mapper function to incoming values', () => {
		const result = applyDiff(
			[],
			[{ tag: 'PushBack', inner: { value: 'hello' } }],
			(s: string) => s.toUpperCase(),
		);
		expect(result).toEqual(['HELLO']);
	});

	it('applies multiple updates in sequence (realistic SDK update batch)', () => {
		const result = applyDiff(
			['a', 'b', 'c'],
			[
				{ tag: 'PushBack', inner: { value: 'd' } },
				{ tag: 'Remove', inner: { index: 0 } },
				{ tag: 'Set', inner: { index: 0, value: 'B' } },
			],
			id,
		);
		// After PushBack: ['a','b','c','d']
		// After Remove index 0: ['b','c','d']
		// After Set index 0 → 'B': ['B','c','d']
		expect(result).toEqual(['B', 'c', 'd']);
	});
});

describe('debounce', () => {
	beforeEach(() => {
		vi.useFakeTimers();
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it('trailing edge (default): only fires after the wait period', () => {
		const fn = vi.fn();
		const debounced = debounce(fn, 100);

		debounced();
		debounced();
		debounced();
		expect(fn).not.toHaveBeenCalled();

		vi.advanceTimersByTime(100);
		expect(fn).toHaveBeenCalledTimes(1);
	});

	it('trailing edge: only the last call args are used', () => {
		const fn = vi.fn();
		const debounced = debounce(fn, 100);

		debounced('first');
		debounced('second');
		debounced('third');

		vi.advanceTimersByTime(100);
		expect(fn).toHaveBeenCalledWith('third');
	});

	it('leading edge: fires immediately on the first call', () => {
		const fn = vi.fn();
		const debounced = debounce(fn, 100, { leading: true, trailing: false });

		debounced('a');
		expect(fn).toHaveBeenCalledTimes(1);
		expect(fn).toHaveBeenCalledWith('a');

		// Calls during the wait period are ignored
		debounced('b');
		debounced('c');
		vi.advanceTimersByTime(100);
		expect(fn).toHaveBeenCalledTimes(1);
	});

	it('leading + trailing: fires immediately and once more after the wait', () => {
		const fn = vi.fn();
		const debounced = debounce(fn, 100, { leading: true, trailing: true });

		debounced('first');
		expect(fn).toHaveBeenCalledTimes(1);

		debounced('second');
		debounced('third');
		vi.advanceTimersByTime(100);

		expect(fn).toHaveBeenCalledTimes(2);
		expect(fn).toHaveBeenLastCalledWith('third');
	});
});
