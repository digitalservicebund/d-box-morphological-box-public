// SPDX-FileCopyrightText: 2025 Element
// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import {
	EventOrTransactionId,
	MessageType,
	TimelineDiff,
	TimelineDiff_Tags,
	TimelineItemContent,
	VirtualTimelineItem,
	type EventTimelineItem,
	type PaginationStatus,
	type RoomInterface,
	type TaskHandleInterface,
	type TimelineInterface,
	type TimelineItemInterface,
} from '../index.web';
import { StateStore } from './StateStore';
import { makeBrandedType, printRustError } from './utils';

interface TimelineViewState {
	items: TimelineItem<TimelineItemKind>[];
	showTopSpinner: boolean;
	firstItemIndex: number;
}

export const TimelineItemKind = makeBrandedType(
	{
		Event: 'Event',
		Virtual: 'Virtual',
		Spinner: 'Spinner',
	},
	'TimelineItemKind',
);

export type TimelineItemKind =
	(typeof TimelineItemKind)[keyof typeof TimelineItemKind];

export function isVirtualEvent(
	item: TimelineItem<TimelineItemKind> | undefined,
): item is TimelineItem<typeof TimelineItemKind.Virtual> {
	return item?.kind === TimelineItemKind.Virtual;
}

export function isRealEvent(
	item: TimelineItem<TimelineItemKind> | undefined,
): item is TimelineItem<typeof TimelineItemKind.Event> {
	return item?.kind === TimelineItemKind.Event;
}

export class TimelineItem<
	K extends
		| typeof TimelineItemKind.Event
		| typeof TimelineItemKind.Virtual
		| typeof TimelineItemKind.Spinner,
> {
	item: K extends typeof TimelineItemKind.Event
		? EventTimelineItem
		: K extends typeof TimelineItemKind.Spinner
			? 'spinner'
			: VirtualTimelineItem;
	kind: K;
	continuation = false;

	constructor(
		kind: K,
		item: K extends typeof TimelineItemKind.Event
			? EventTimelineItem
			: K extends typeof TimelineItemKind.Spinner
				? 'spinner'
				: VirtualTimelineItem,
	) {
		this.kind = kind;
		this.item = item;
	}

	getInternalId = (): string => {
		if (this.kind === TimelineItemKind.Spinner) {
			return 'spinner';
		}
		if (isVirtualEvent(this)) {
			if (VirtualTimelineItem.TimelineStart.instanceOf(this.item)) {
				return 'start';
			}
			if (VirtualTimelineItem.DateDivider.instanceOf(this.item)) {
				return `divider-${this.item.inner.ts}`;
			}
			if (VirtualTimelineItem.ReadMarker.instanceOf(this.item)) {
				return 'readmarker';
			}
			return '0';
		}
		const event = this.item as EventTimelineItem;
		if (EventOrTransactionId.EventId.instanceOf(event.eventOrTransactionId)) {
			return event.eventOrTransactionId.inner.eventId;
		}
		if (
			EventOrTransactionId.TransactionId.instanceOf(event.eventOrTransactionId)
		) {
			return event.eventOrTransactionId.inner.transactionId;
		}
		return '1';
	};

	updateContinuation(prevItem: TimelineItem<TimelineItemKind>) {
		this.continuation =
			prevItem &&
			isRealEvent(this) &&
			isRealEvent(prevItem) &&
			TimelineItemContent.MsgLike.instanceOf(this.item.content) &&
			TimelineItemContent.MsgLike.instanceOf(prevItem.item.content) &&
			this.item.sender === prevItem.item.sender;
	}
}

export class WrapperVirtualTimelineItem extends TimelineItem<
	typeof TimelineItemKind.Virtual
> {
	constructor(item: VirtualTimelineItem) {
		super(TimelineItemKind.Virtual, item);
	}
}

export class RealEventTimelineItem extends TimelineItem<
	typeof TimelineItemKind.Event
> {
	constructor(item: EventTimelineItem) {
		super(TimelineItemKind.Event, item);
	}
}

const INITIAL_FIRST_TIME_INDEX = 10000;
export default class TimelineStore extends StateStore<TimelineViewState> {
	running = false;
	paginationStatus?: PaginationStatus;
	firstItemId?: string;
	hasMoreItems = true;
	private timelinePromise: Promise<TimelineInterface>;

	constructor(public readonly room: RoomInterface) {
		super({
			items: [],
			showTopSpinner: true,
			firstItemIndex: INITIAL_FIRST_TIME_INDEX,
		});
		this.timelinePromise = this.room.timeline();
	}

	private parseItem(
		item?: TimelineItemInterface,
	): TimelineItem<TimelineItemKind> {
		if (item?.asEvent()) {
			return new RealEventTimelineItem(item.asEvent()!);
		}
		if (item?.asVirtual()) {
			return new WrapperVirtualTimelineItem(item.asVirtual()!);
		}
		throw new Error('Something unknown');
	}

	sendMessage = async (msg: string) => {
		try {
			const timeline = await this.timelinePromise;
			const event = timeline.createMessageContent(
				MessageType.Text.new({
					content: {
						body: msg,
						formatted: undefined,
					},
				}),
			)!;
			console.log('sending', msg);
			await timeline.send(event);
			console.log('sent', msg);
		} catch (e) {
			printRustError('Failed to send message', e);
		}
	};

	backPaginate = async (): Promise<void> => {
		console.log('backPaginate');
		const timeline = await this.timelinePromise;
		const hasMore = !(await timeline.paginateBackwards(50));
		const shouldEmit = this.hasMoreItems !== hasMore;
		this.hasMoreItems = hasMore;
		if (shouldEmit) {
			this.setState({
				...this.viewState,
				showTopSpinner: hasMore,
			});
		}
	};

	onPaginationStatusUpdate = async (status: PaginationStatus) => {
		this.paginationStatus = status;
		console.log('onPaginationStatusUpdate', status);
	};

	onUpdate = (updates: TimelineDiff[]): void => {
		let newItems = [...this.viewState.items];

		for (const update of updates) {
			switch (update.tag) {
				case TimelineDiff_Tags.Set: {
					newItems[update.inner.index] = this.parseItem(update.inner.value);
					newItems = [...newItems];
					break;
				}
				case TimelineDiff_Tags.PushBack:
					newItems = [...newItems, this.parseItem(update.inner.value)];
					break;
				case TimelineDiff_Tags.PushFront:
					newItems = [this.parseItem(update.inner.value), ...newItems];
					break;
				case TimelineDiff_Tags.Clear:
					newItems = [];
					break;
				case TimelineDiff_Tags.PopFront:
					newItems.shift();
					newItems = [...newItems];
					break;
				case TimelineDiff_Tags.PopBack:
					newItems.pop();
					newItems = [...newItems];
					break;
				case TimelineDiff_Tags.Insert:
					newItems.splice(
						update.inner.index,
						0,
						this.parseItem(update.inner.value),
					);
					newItems = [...newItems];
					break;
				case TimelineDiff_Tags.Remove:
					newItems.splice(update.inner.index, 1);
					newItems = [...newItems];
					break;
				case TimelineDiff_Tags.Truncate:
					newItems = newItems.slice(0, update.inner.length);
					break;
				case TimelineDiff_Tags.Reset:
					newItems = update.inner.values.map(this.parseItem);
					break;
				case TimelineDiff_Tags.Append:
					newItems = [...newItems, ...update.inner.values.map(this.parseItem)];
					break;
			}
		}

		function findFirstEventItemId(
			items: TimelineItem<TimelineItemKind>[],
		): string | undefined {
			const eventItem = items.find(
				(item) => item?.kind === TimelineItemKind.Event,
			);
			return eventItem?.getInternalId();
		}

		newItems.map((curr, i, arr) => {
			if (i > 0) {
				curr.updateContinuation(arr[i - 1]);
			}
			return curr;
		});

		// virtuoso requires us to track the "firstItemIndex" so that it knows how to prepend items
		// to the list while maintaining the scroll position without jumps.
		// We use a large number, as per their docs it should never be negative.
		// https://virtuoso.dev/virtuoso-api/interfaces/VirtuosoProps/#firstitemindex
		let firstItemIndex: number = INITIAL_FIRST_TIME_INDEX;
		// We keep a reference to the first item by it's id.
		if (this.firstItemId) {
			// If we have a firstItemId, we need to find its index in the new items
			// and calculate the firstItemIndex based on that.
			const foundIndex = newItems.findIndex(
				(item) => item.getInternalId() === this.firstItemId,
			);

			if (foundIndex) {
				// If we found the item, we set the firstItemIndex to the difference
				// between the initial index and the found index.
				firstItemIndex = INITIAL_FIRST_TIME_INDEX - foundIndex;
			} else {
				// If we didn't find the item, we set the firstItemId to the first event
				this.firstItemId = findFirstEventItemId(newItems);
			}
		} else {
			// If we don't have a firstItemId, we find the first event item id
			this.firstItemId = findFirstEventItemId(newItems);
		}

		// Filter out items that EventTile would return null for
		const filteredItems = newItems.filter((item) => {
			// Only filter Event items with content types that EventTile would return null for
			if (item.kind === TimelineItemKind.Event) {
				const event = item as TimelineItem<typeof TimelineItemKind.Event>;
				// These content types are not shown in EventTile
				if (
					TimelineItemContent.CallInvite.instanceOf(event.item.content) ||
					TimelineItemContent.RtcNotification.instanceOf(event.item.content) ||
					TimelineItemContent.State.instanceOf(event.item.content)
				) {
					return false;
				}
				// Print warning for failed types
				if (
					TimelineItemContent.FailedToParseState.instanceOf(
						event.item.content,
					) ||
					TimelineItemContent.FailedToParseMessageLike.instanceOf(
						event.item.content,
					)
				) {
					console.warn('Filtered failed timeline item:', event.item.content);
					return false;
				}
			}
			return true;
		});

		this.setState({
			...this.viewState,
			items: filteredItems,
			firstItemIndex,
		});
	};

	timelineListener?: TaskHandleInterface;
	pagintationListener?: TaskHandleInterface;
	run = () => {
		if (!this.room) return;

		(async () => {
			console.log('subscribing to timeline', this.room.id());
			const timeline = await this.room.timeline();
			this.timelineListener = await timeline.addListener(this);
			this.pagintationListener = await timeline.subscribeToBackPaginationStatus(
				{
					onUpdate: this.onPaginationStatusUpdate,
				},
			);
			console.log('subscribed to timeline', this.room.id());
			this.running = true;
		})();
	};
	stop = () => {
		(async () => {
			console.log('unsubscribing to timeline', this.room.id());
			this.timelineListener?.cancel();
			this.running = false;
			console.log('unsubscribed to timeline', this.room.id());
		})();
	};
}
