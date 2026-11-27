// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

/**
 * Prints a Rust error message to the console with enhanced debugging information.
 *
 * This function handles Rust errors that may contain nested error information.
 * When the error object has an "inner" property, it logs both the main error
 * and the inner error details for better debugging.
 *
 * @param msg - A descriptive message to prefix the error log
 * @param e - The error object or value to be logged, which may contain nested error information
 *
 * @example
 * ```typescript
 * try {
 *   // Some Rust operation
 * } catch (error) {
 *   printRustError("Failed to execute Rust function", error);
 * }
 * ```
 */
export function printRustError(msg: string, e: unknown) {
	// TODO: Check if this is allowed with BSI guidelines or if we must remove this in prod
	if (typeof e === 'object' && e !== null && 'inner' in e) {
		console.error(`[RUST] ${msg}`, e, e.inner);
	}
	console.error(`[RUST] ${msg}`, e);
}

/**
 * Creates a branded type that adds a compile-time brand to an existing type.
 *
 * @template T - The base type to be branded
 * @template Brand - A string literal type that serves as the brand identifier
 *
 * @example
 * ```typescript
 * type UserId = Branded<number, 'UserId'>;
 * type ProductId = Branded<number, 'ProductId'>;
 *
 * const userId: UserId = 123 as UserId;
 * const productId: ProductId = 456 as ProductId;
 *
 * // This would cause a TypeScript error:
 * // const invalidAssignment: UserId = productId;
 * ```
 */
type Branded<T, Brand extends string> = T & { __brand: Brand };

/**
 * Creates a branded version of an object where all string values are branded with a specific type.
 *
 * @template T - A record type with string values that will be branded
 * @param obj - The object containing string values to be branded
 * @param brand - The brand string used for type branding
 * @returns A new object with the same structure but all values branded with the specified brand type
 *
 * @example
 * ```typescript
 * const colors = { red: "#ff0000", blue: "#0000ff" };
 * const brandedColors = makeBrandedType(colors, "Color");
 * // brandedColors has type { red: Branded<"#ff0000", "Color">, blue: Branded<"#0000ff", "Color"> }
 * ```
 */
export const makeBrandedType = <const T extends Record<string, string>>(
	obj: T,
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	brand: string,
) => {
	return Object.fromEntries(
		Object.entries(obj).map(([k, v]) => [
			k,
			v as Branded<typeof v, typeof brand>,
		]),
	) as {
		[K in keyof T]: Branded<T[K], typeof brand>;
	};
};

/**
 * Represents an update operation that sets a value at a specific index.
 *
 * @template V - The type of the value being set
 */
type SetUpdate<V> = {
	tag: 'Set';
	inner: {
		index: number;
		value: V;
	};
};

/**
 * Represents an update operation that pushes a value back to a collection or state.
 *
 * @template V - The type of the value being pushed back
 */
type PushBackUpdate<V> = {
	tag: 'PushBack';
	inner: {
		value: V;
	};
};

/**
 * Represents an update operation that adds a new value to the front of a collection.
 *
 * @template V - The type of the value being pushed to the front
 */
type PushFrontUpdate<V> = {
	tag: 'PushFront';
	inner: {
		value: V;
	};
};

/**
 * Represents a clear update operation.
 */
type ClearUpdate = {
	tag: 'Clear';
};

/**
 * Represents an update operation that removes the first element from a collection.
 */
type PopFrontUpdate = {
	tag: 'PopFront';
};

/**
 * Represents an update operation that pops back to a previous state.
 */
type PopBackUpdate = {
	tag: 'PopBack';
};

/**
 * Represents an insert operation for updating a data structure.
 *
 * @template V - The type of value being inserted

 */
type InsertUpdate<V> = {
	tag: 'Insert';
	inner: {
		index: number;
		value: V;
	};
};

/**
 * Represents an update operation that removes an item at a specific index.
 */
type RemoveUpdate = {
	tag: 'Remove';
	inner: {
		index: number;
	};
};

/**
 * Represents an update operation that truncates content to a specified length.
 */
type TruncateUpdate = {
	tag: 'Truncate';
	inner: {
		length: number;
	};
};

/**
 * Represents a reset update operation that replaces the current state with new values.
 *
 * @template V - The type of values being reset
 */
type ResetUpdate<V> = {
	tag: 'Reset';
	inner: {
		values: V[];
	};
};

/**
 * Represents an update operation that appends values to a collection.
 *
 * @template V - The type of values being appended
 */
type AppendUpdate<V> = {
	tag: 'Append';
	inner: {
		values: V[];
	};
};

/**
 * A union type representing all possible update operations that can be performed on a collection or data structure.
 *
 * This type encompasses various mutation operations including:
 * - Setting values
 * - Adding elements to the front or back
 * - Removing elements from the front or back
 * - Clearing all elements
 * - Inserting elements at specific positions
 * - Removing specific elements
 * - Truncating the collection
 * - Resetting the entire collection
 * - Appending multiple elements
 *
 * @template V - The type of values stored in the collection
 */
type Update<V> =
	| SetUpdate<V>
	| PushBackUpdate<V>
	| PushFrontUpdate<V>
	| ClearUpdate
	| PopFrontUpdate
	| PopBackUpdate
	| InsertUpdate<V>
	| RemoveUpdate
	| TruncateUpdate
	| ResetUpdate<V>
	| AppendUpdate<V>;

/**
 * Applies a series of updates to an array of items, transforming input values using a mapper function.
 *
 * This function processes various types of array operations including adding, removing, updating,
 * and reordering elements. Each update is applied sequentially to produce a new array.
 *
 * @template I - The type of input values in the updates
 * @template V - The type of values in the resulting array
 *
 * @param items - The initial array of items to apply updates to
 * @param updates - An array of update operations to apply sequentially
 * @param mapper - A function that transforms input values of type I to output values of type V
 *
 * @returns A new array with all updates applied
 *
 * @example
 * ```typescript
 * const items = [1, 2, 3];
 * const updates = [
 *   { tag: "PushBack", inner: { value: 4 } },
 *   { tag: "Set", inner: { index: 0, value: 10 } }
 * ];
 * const result = applyDiff(items, updates, (x: number) => x);
 * // Result: [10, 2, 3, 4]
 * ```
 */
export function applyDiff<I, V>(
	items: V[],
	updates: Update<I>[],
	mapper: (value: I) => V,
): Array<V> {
	let newItems = [...items];

	for (const update of updates) {
		switch (update.tag) {
			case 'Set':
				newItems[update.inner.index] = mapper(update.inner.value);
				newItems = [...newItems];
				break;
			case 'PushBack':
				newItems = [...newItems, mapper(update.inner.value)];
				break;
			case 'PushFront':
				newItems = [mapper(update.inner.value), ...newItems];
				break;
			case 'Clear':
				newItems = [];
				break;
			case 'PopFront':
				newItems.shift();
				newItems = [...newItems];
				break;
			case 'PopBack':
				newItems.pop();
				newItems = [...newItems];
				break;
			case 'Insert':
				newItems.splice(update.inner.index, 0, mapper(update.inner.value));
				newItems = [...newItems];
				break;
			case 'Remove':
				newItems.splice(update.inner.index, 1);
				newItems = [...newItems];
				break;
			case 'Truncate':
				newItems = newItems.slice(0, update.inner.length);
				break;
			case 'Reset':
				newItems = update.inner.values.map(mapper);
				break;
			case 'Append':
				newItems = [...newItems, ...update.inner.values.map(mapper)];
				break;
		}
	}

	return newItems;
}

/**
 * Options for configuring debounce behavior.
 */
interface DebounceOptions {
	/** If true, invoke the function on the leading edge of the timeout. Default: false */
	leading?: boolean;
	/** If true, invoke the function on the trailing edge of the timeout. Default: true */
	trailing?: boolean;
}

/**
 * Creates a debounced function that delays invoking `func` until after `wait` milliseconds
 * have elapsed since the last time the debounced function was invoked.
 *
 * @template T - The type of the function to debounce
 * @param func - The function to debounce
 * @param wait - The number of milliseconds to delay
 * @param options - Options object to configure leading/trailing edge invocation
 * @returns The debounced function
 *
 * @example
 * ```typescript
 * // Trailing edge (default) - invokes after delay when calls stop
 * const debouncedFn = debounce(() => console.log('called'), 500);
 *
 * // Leading edge - invokes immediately, then ignores calls during wait period
 * const debouncedFn = debounce(() => console.log('called'), 500, { leading: true, trailing: false });
 *
 * // Both edges - invokes immediately and after delay
 * const debouncedFn = debounce(() => console.log('called'), 500, { leading: true, trailing: true });
 * ```
 */
export function debounce<T extends (...args: unknown[]) => void>(
	func: T,
	wait: number,
	options: DebounceOptions = {},
): (...args: Parameters<T>) => void {
	const { leading = false, trailing = true } = options;
	let timeout: ReturnType<typeof setTimeout> | null = null;
	let lastArgs: Parameters<T> | null = null;

	return (...args: Parameters<T>): void => {
		const callNow = leading && !timeout;

		lastArgs = args;

		if (timeout) {
			clearTimeout(timeout);
		}

		timeout = setTimeout(() => {
			timeout = null;
			if (trailing && lastArgs) {
				func(...lastArgs);
				lastArgs = null;
			}
		}, wait);

		if (callNow) {
			func(...args);
		}
	};
}
