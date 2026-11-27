// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import {
  TimelineDiff_Tags,
  VirtualTimelineItem,
  type MessageServiceLike,
  type NeoCase,
  type NeoContent,
  type RoomInfo,
  type RoomLike,
  type TaskHandleLike,
  type TimelineDiff,
  type TimelineItemLike,
  type TimelineLike,
} from '../index.web';
import { StateStore } from './StateStore';
import { printRustError } from './utils';

// Lightweight projection of a timeline item — only what we need for read state.
type ReadItem =
  | { kind: 'event'; timestamp: bigint; hasOwnReceipt: boolean }
  | { kind: 'readMarker' };

interface CaseViewState {
  neoCase: NeoCase | null;
  loading: boolean;
  /**
   * Timestamp (ms) of the last event the current user has read.
   * null = could not be determined (treat as all read).
   */
  lastReadTimestamp: bigint | null;
  /** Maps Matrix user IDs to their resolved display names. */
  displayNames: Record<string, string>;
}

export class CaseStore extends StateStore<CaseViewState> {
  private readonly ownUserId: string;
  private timeline?: TimelineLike;
  private timelineListener?: TaskHandleLike;
  private roomInfoListener?: TaskHandleLike;
  private readItems: ReadItem[] = [];
  private reloadTimer?: ReturnType<typeof setTimeout>;
  private displayNamesGeneration = 0;

  constructor(
    private readonly messageService: MessageServiceLike,
    private readonly room: RoomLike,
    public readonly caseId: string,
  ) {
    super({
      neoCase: null,
      loading: true,
      lastReadTimestamp: null,
      displayNames: {},
    });
    this.ownUserId = room.ownUserId();
  }

  run = (): void => {
    void this.load();
    this.roomInfoListener = this.room.subscribeToRoomInfoUpdates({
      call: (_info: RoomInfo) => this.scheduleCaseReload(),
    });
    void this.subscribeToTimeline();
  };

  stop = (): void => {
    this.roomInfoListener?.cancel();
    this.timelineListener?.cancel();
    this.timeline = undefined;
    clearTimeout(this.reloadTimer);
  };

  private load = async (): Promise<void> => {
    try {
      const neoCase = await this.messageService.loadCase(this.caseId);
      this.setState({ ...this.viewState, neoCase, loading: false });
      void this.resolveDisplayNames(neoCase);
    } catch (e) {
      printRustError('Failed to load case', e);
      this.setState({ ...this.viewState, neoCase: null, loading: false });
    }
  };

  private resolveDisplayNames = async (neoCase: NeoCase): Promise<void> => {
    const generation = ++this.displayNamesGeneration;

    const senderIds = new Set<string>();
    for (const dialog of neoCase.dialogs) {
      for (const block of dialog.message.blocks) senderIds.add(block.sender);
      for (const envelope of dialog.answers) {
        for (const block of envelope.blocks) senderIds.add(block.sender);
      }
    }

    const resolved: Record<string, string> = {};
    await Promise.all(
      [...senderIds].map(async (userId) => {
        try {
          const name = await this.room.memberDisplayName(userId);
          if (name) resolved[userId] = name;
        } catch {
          // display name unavailable — fall back to user ID rendering
        }
      }),
    );

    if (
      generation === this.displayNamesGeneration &&
      Object.keys(resolved).length > 0
    ) {
      this.setState({ ...this.viewState, displayNames: resolved });
    }
  };

  private subscribeToTimeline = async (): Promise<void> => {
    this.timeline = await this.room.timeline();
    this.timelineListener = await this.timeline.addListener({
      onUpdate: (diffs: TimelineDiff[]) => this.onTimelineUpdate(diffs),
    });
  };

  private extractReadItem(
    itemLike: TimelineItemLike | undefined,
  ): ReadItem | null {
    if (!itemLike) return null;

    const event = itemLike.asEvent();
    if (event) {
      return {
        kind: 'event',
        timestamp: BigInt(event.timestamp),
        hasOwnReceipt: event.readReceipts.has(this.ownUserId),
      };
    }

    const virtual = itemLike.asVirtual();
    if (virtual && VirtualTimelineItem.ReadMarker.instanceOf(virtual)) {
      return { kind: 'readMarker' };
    }

    return null;
  }

  private onTimelineUpdate = (diffs: TimelineDiff[]): void => {
    let items = [...this.readItems];

    for (const diff of diffs) {
      switch (diff.tag) {
        case TimelineDiff_Tags.Reset: {
          items = diff.inner.values
            .map((v) => this.extractReadItem(v))
            .filter((v): v is ReadItem => v !== null);
          break;
        }
        case TimelineDiff_Tags.Append: {
          const appended = diff.inner.values
            .map((v) => this.extractReadItem(v))
            .filter((v): v is ReadItem => v !== null);
          items = [...items, ...appended];
          break;
        }
        case TimelineDiff_Tags.PushBack: {
          const item = this.extractReadItem(diff.inner.value);
          if (item) items = [...items, item];
          break;
        }
        case TimelineDiff_Tags.PushFront: {
          const item = this.extractReadItem(diff.inner.value);
          if (item) items = [item, ...items];
          break;
        }
        case TimelineDiff_Tags.Set: {
          const item = this.extractReadItem(diff.inner.value);
          if (item) {
            items = [...items];
            items[diff.inner.index] = item;
          }
          break;
        }
        case TimelineDiff_Tags.Insert: {
          const item = this.extractReadItem(diff.inner.value);
          if (item) {
            items = [...items];
            items.splice(diff.inner.index, 0, item);
          }
          break;
        }
        case TimelineDiff_Tags.Remove: {
          items = [...items];
          items.splice(diff.inner.index, 1);
          break;
        }
        case TimelineDiff_Tags.PopFront: {
          items = items.slice(1);
          break;
        }
        case TimelineDiff_Tags.PopBack: {
          items = items.slice(0, -1);
          break;
        }
        case TimelineDiff_Tags.Truncate: {
          items = items.slice(0, diff.inner.length);
          break;
        }
        case TimelineDiff_Tags.Clear: {
          items = [];
          break;
        }
      }
    }

    this.readItems = items;
    this.recomputeReadState();
    this.scheduleCaseReload();
  };

  private scheduleCaseReload = (): void => {
    clearTimeout(this.reloadTimer);
    this.reloadTimer = setTimeout(() => void this.load(), 300);
  };

  private recomputeReadState = (): void => {
    const readMarkerIndex = this.readItems.findIndex(
      (i) => i.kind === 'readMarker',
    );

    let lastReadTimestamp: bigint | null = null;

    if (readMarkerIndex > 0) {
      // Walk backwards from the read marker to find the last event before it.
      for (let i = readMarkerIndex - 1; i >= 0; i--) {
        const item = this.readItems[i];
        if (item.kind === 'event') {
          lastReadTimestamp = item.timestamp;
          break;
        }
      }
    } else {
      // No read marker — fall back to the latest event where our own read
      // receipt is present.
      for (const item of this.readItems) {
        if (
          item.kind === 'event' &&
          item.hasOwnReceipt &&
          (lastReadTimestamp === null || item.timestamp > lastReadTimestamp)
        ) {
          lastReadTimestamp = item.timestamp;
        }
      }
    }

    if (lastReadTimestamp !== this.viewState.lastReadTimestamp) {
      this.setState({ ...this.viewState, lastReadTimestamp });
    }
  };

  sendEnvelope = (blocks: NeoContent[]): Promise<string> =>
    this.messageService.sendEnvelope(this.caseId, blocks);

  reload = (): void => {
    void this.load();
  };

  sendAnswer = (dialogId: string, blocks: NeoContent[]): Promise<string> =>
    this.messageService.sendAnswer(this.caseId, dialogId, blocks);
}

export default CaseStore;
