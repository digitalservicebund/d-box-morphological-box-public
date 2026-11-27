// SPDX-FileCopyrightText: 2025 Element
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * Abstract base class for implementing a state management store with subscription capabilities.
 *
 * This class provides a foundation for creating reactive state stores that can notify
 * subscribers when the state changes. It follows a publish-subscribe pattern where
 * components can subscribe to state changes and automatically react to updates.
 *
 * @template S - The type of the state object managed by this store
 *
 * @example
 * ```typescript
 * interface MyState {
 *   count: number;
 *   name: string;
 * }
 *
 * class MyStore extends StateStore<MyState> {
 *   increment() {
 *     this.setState({ ...this.getSnapshot(), count: this.getSnapshot().count + 1 });
 *   }
 * }
 *
 * const store = new MyStore({ count: 0, name: 'example' });
 * const unsubscribe = store.subscribe(() => console.log('State changed'));
 * ```
 */
export class StateStore<S> {
	/**
	 * Array of callback functions that are notified when the state changes.
	 * These listeners are typically invoked whenever the store's state is updated
	 * to allow components or other parts of the application to react to state changes.
	 *
	 * @private
	 */
	private listeners: CallableFunction[] = [];

	/**
	 * The current state of the component.
	 * @private
	 */
	private state: S;

	/**
	 * Creates a new StateStore instance with the provided initial state.
	 *
	 * @param initialState - The initial state value to set for this store
	 */
	constructor(initialState: S) {
		this.state = initialState;
	}

	/**
	 * Returns the current state snapshot.
	 *
	 * @returns The current state of type S
	 */
	getSnapshot = (): S => {
		return this.state;
	};

	/**
	 * Subscribes a listener function to state changes.
	 *
	 * @param listener - The callback function to be invoked when state changes occur
	 * @returns A function that can be called to unsubscribe the listener
	 *
	 * @example
	 * ```typescript
	 * const unsubscribe = stateStore.subscribe((newState) => {
	 *   console.log('State changed:', newState);
	 * });
	 *
	 * // Later, to unsubscribe:
	 * unsubscribe();
	 * ```
	 */
	subscribe = (listener: CallableFunction): (() => void) => {
		this.listeners = [...this.listeners, listener];
		return () => {
			this.listeners = this.listeners.filter((l) => l !== listener);
		};
	};

	/**
	 * Updates the current state and notifies all registered listeners.
	 *
	 * @param newState - The new state object to set
	 * @returns void
	 *
	 * @remarks
	 * This method performs the following operations:
	 * 1. Replaces the current state with the provided new state
	 * 2. Iterates through all registered listeners and invokes each one
	 *
	 * All listeners are called synchronously in the order they were registered.
	 */
	setState = (newState: S): void => {
		this.state = newState;
		for (const listener of this.listeners) {
			listener();
		}
	};

	/**
	 * Gets the current view state of the component.
	 *
	 * @returns The current state object of type S
	 */
	get viewState(): S {
		return this.state;
	}
}
