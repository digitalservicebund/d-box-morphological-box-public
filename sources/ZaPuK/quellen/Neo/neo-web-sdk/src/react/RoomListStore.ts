// SPDX-FileCopyrightText: 2025 Element
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import {
	LatestEventValue,
	LatestEventValue_Tags,
	RoomListEntriesDynamicFilterKind,
	RoomListEntriesDynamicFilterKind_Tags,
	RoomListEntriesUpdate,
	RoomListLoadingState,
	type RoomInfo,
	type RoomInterface,
	type RoomListDynamicEntriesControllerInterface,
	type RoomListEntriesWithDynamicAdaptersResultInterface,
	type RoomListServiceInterface,
	type TaskHandleInterface,
} from '../index.web';
import { StateStore } from './StateStore';
import { applyDiff, debounce } from './utils';

/**
 * Represents a room list item that manages room information and events.
 *
 * This class wraps a room interface and provides functionality to load room data,
 * manage listeners for updates, and expose room properties like name and ID.
 */
export class RoomListItem {
	/** Optional room information containing metadata about the room */
	info?: RoomInfo;

	/** The most recent event in the room timeline */
	latestEvent?: LatestEventValue;

	/** Array of callback functions that are notified when room data changes */
	listeners: CallableFunction[] = [];

	/**
	 * Creates a new RoomListItem instance.
	 *
	 * @param room - The room interface that this item represents
	 */
	constructor(private readonly room: RoomInterface) {
		this.load();
	}

	/**
	 * Asynchronously loads room information and latest event data.
	 * After loading, emits notifications to all registered listeners.
	 */
	async load() {
		[this.info, this.latestEvent] = await Promise.all([
			this.room.roomInfo(),
			this.room.latestEvent(),
		]);
		this.emit();
	}

	/**
	 * Notifies all registered listeners by calling their callback functions.
	 */
	private emit() {
		for (const listener of this.listeners) {
			listener();
		}
	}

	/**
	 * Gets the unique identifier of the room.
	 *
	 * @returns The room ID
	 */
	get roomId() {
		return this.room.id();
	}

	/**
	 * Gets the display name of the room.
	 *
	 * @returns The room's display name, or undefined if room info is not loaded
	 */
	get name() {
		return this.info?.displayName;
	}

	/**
	 * Gets the topic of the room.
	 *
	 * @returns The room's topic, or undefined if room info is not loaded
	 */
	get topic() {
		return this.info?.topic;
	}

	/**
	 * Timestamp of the last activity in the room.
	 *
	 * @returns The timestamp of the last activity, or undefined if not available
	 */
	get lastActivity() {
		if (this.latestEvent?.tag === LatestEventValue_Tags.Remote) {
			return this.latestEvent.inner.timestamp;
		}
		return undefined;
	}

	/**
	 * Placeholder method for future implementation.
	 * Currently performs no operations.
	 */
	run() {}
}

type FiltersToBePicked = typeof RoomListEntriesDynamicFilterKind_Tags;
export type SupportedFilters = keyof Pick<
	FiltersToBePicked,
	'NonLeft' | 'Unread'
>;

/**
 * Configuration object containing predefined room list filters.
 *
 * Defines filter configurations for different room list views, each containing
 * a human-readable name and a corresponding filter method implementation.
 *
 * @remarks
 * Each filter combines multiple filter kinds using the `All` filter to ensure
 * all specified conditions are met. All filters include deduplication of room versions.
 *
 * @example
 * ```typescript
 * // Access the unread filter configuration
 * const unreadFilter = FILTERS[RoomListEntriesDynamicFilterKind_Tags.Unread];
 * console.log(unreadFilter.name); // "Unread"
 * ```
 */
export const FILTERS: Record<
	SupportedFilters,
	{ name: string; method: RoomListEntriesDynamicFilterKind }
> = {
	[RoomListEntriesDynamicFilterKind_Tags.NonLeft]: {
		name: 'All',
		method: new RoomListEntriesDynamicFilterKind.All({
			filters: [
				new RoomListEntriesDynamicFilterKind.NonLeft(),
				new RoomListEntriesDynamicFilterKind.DeduplicateVersions(),
			],
		}),
	},
	[RoomListEntriesDynamicFilterKind_Tags.Unread]: {
		name: 'Unread',
		method: new RoomListEntriesDynamicFilterKind.All({
			filters: [
				new RoomListEntriesDynamicFilterKind.Unread(),
				new RoomListEntriesDynamicFilterKind.DeduplicateVersions(),
			],
		}),
	},
};

/**
 * Represents the view state for a room list component.
 *
 * @interface RoomListViewState
 * @property {Array<RoomListItem>} rooms - Array of room list items to display
 * @property {number} numRooms - Total number of rooms available
 * @property {SupportedFilters} filter - Currently applied filter for the room list
 */
interface RoomListViewState {
	rooms: Array<RoomListItem>;
	numRooms: number;
	filter: SupportedFilters;
}

/**
 * A state store that manages the room list view state and handles room list operations.
 *
 * This store extends StateStore to manage room list data, including loading states,
 * filtering, pagination, and room subscriptions. It provides methods to run the room list,
 * handle updates, and manage active room selections.
 *
 * @extends StateStore<RoomListViewState>
 */
export default class RoomListStore extends StateStore<RoomListViewState> {
	running = false;
	controller?: RoomListDynamicEntriesControllerInterface;
	loadingState?: RoomListLoadingState;
	stateStream?: TaskHandleInterface;
	activeRoom?: string;
	visibleRooms: string[] = [];

	constructor(private readonly roomListService: RoomListServiceInterface) {
		super({
			rooms: [],
			numRooms: -1,
			filter: RoomListEntriesDynamicFilterKind_Tags.NonLeft,
		});
	}

	/**
	 * Parses a room interface object and converts it into a RoomListItem instance.
	 *
	 * @param room - The room interface object to be parsed
	 * @returns A new RoomListItem instance created from the provided room data
	 * @private
	 */
	private parseRoom = (room: RoomInterface): RoomListItem => {
		const rli = new RoomListItem(room);
		return rli;
	};

	/**
	 * Handles updates to the room list loading state.
	 *
	 * Updates the internal loading state and, when the state indicates that rooms are loaded
	 * and contains maximum room count information, updates the view state with the number of rooms.
	 *
	 * @param state - The new room list loading state
	 */
	onLoadingStateUpdate = (state: RoomListLoadingState) => {
		this.loadingState = state;
		if (
			RoomListLoadingState.Loaded.instanceOf(state) &&
			state.inner.maximumNumberOfRooms !== undefined
		) {
			this.setState({
				...this.viewState,
				numRooms: state.inner.maximumNumberOfRooms,
			});
		}
	};

	/**
	 * Handles updates to the room list by applying differential changes to the current state.
	 *
	 * @param updates - Array of room list entry updates to be applied to the current room collection
	 * @returns A promise that resolves when the state has been successfully updated
	 */
	onUpdate = async (updates: RoomListEntriesUpdate[]): Promise<void> => {
		this.setState({
			...this.viewState,
			rooms: applyDiff(this.viewState.rooms, updates, this.parseRoom),
		});
	};

	roomListEntriesWithDynamicAdapters?: RoomListEntriesWithDynamicAdaptersResultInterface;

	/**
	 * Starts the room list store and initializes the room subscription.
	 *
	 * This method sets up an asynchronous operation that:
	 * - Subscribes to the room list service
	 * - Creates an abort controller for request cancellation
	 * - Initializes loading state tracking with update callbacks
	 * - Sets up room list entries with dynamic adapters (if not already initialized)
	 * - Applies the current view filter to the room list controller
	 * - Loads the first page of room entries
	 *
	 * The method updates the running state and logs progress information to the console.
	 * All operations are performed asynchronously without blocking the caller.
	 */
	run() {
		console.log('Running roomlist store with state', this.running);

		(async () => {
			console.log('subscribing to roomlist');

			this.running = true;
			const abortController = new AbortController();
			const roomList = await this.roomListService.allRooms({
				signal: abortController.signal,
			});
			const { state, stateStream } = roomList.loadingState({
				onUpdate: this.onLoadingStateUpdate,
			});
			this.stateStream = stateStream;
			this.loadingState = state;
			this.roomListEntriesWithDynamicAdapters ||=
				roomList.entriesWithDynamicAdapters(50, this);
			this.controller = this.roomListEntriesWithDynamicAdapters.controller();
			console.log('Apply filter', this.viewState.filter);
			this.controller.setFilter(FILTERS[this.viewState.filter].method);
			this.controller.addOnePage();
		})();
	}

	/**
	 * Subscribes to room updates for all visible rooms and the active room.
	 *
	 * This method collects all visible rooms and adds the currently active room (if any)
	 * to create a comprehensive set of rooms to monitor. It then subscribes to updates
	 * for all these rooms through the room list service.
	 */
	subscribeToRooms = (): void => {
		const rooms = new Set(this.visibleRooms);
		if (this.activeRoom) rooms.add(this.activeRoom);
		this.roomListService.subscribeToRooms([...rooms]);
	};

	/**
	 * Debounced version of subscribeToRooms that delays execution by 500ms.
	 * Uses trailing edge triggering to ensure the function is called after the delay period
	 * when no new calls are made. This helps prevent excessive API calls when multiple
	 * rapid subscription requests occur.
	 */
	subscribeToRoomsDebounced = debounce(
		(): void => {
			this.subscribeToRooms();
		},
		500,
		{ trailing: true },
	);

	loadMore = () => {
		this.controller?.addOnePage();
	};

	/**
	 * Sets the active room and subscribes to room updates.
	 *
	 * @param roomId - The unique identifier of the room to set as active
	 */
	setActiveRoom = (roomId: string) => {
		this.activeRoom = roomId;
		this.subscribeToRooms();
	};
}
