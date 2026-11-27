// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useWindowVirtualizer } from '@tanstack/react-virtual';
import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { ErrorBoundary } from '../ErrorBoundary';
import { LoadingSpinner } from '../LoadingSpinner/LoadingSpinner';
import './Timeline.css';
import TimelineCard from './TimelineCard/TimelineCard';
import TimelineMarker from './TimelineMarker/TimelineMarker';
import type { TimelineEntry, TimelineProps } from './timelineTypes';
export type {
  TimelineEntry,
  TimelineProps,
  TimelineReply,
  TimelineSection,
} from './timelineTypes';

type FlatItem =
  | { type: 'spinner' }
  | { type: 'header'; label: string }
  | { type: 'entry'; entry: TimelineEntry };

const ESTIMATED_HEADER_HEIGHT = 56;
const ESTIMATED_SPINNER_HEIGHT = 60;
const ESTIMATED_ENTRY_HEIGHT = 150;
const VIRTUALIZER_OVERSCAN = 5;

export default function Timeline({
  title,
  sections,
  selectedId,
  onSelect,
  onStartReached,
  isLoadingMore,
}: TimelineProps) {
  const { t } = useTranslation();
  const heading = title ?? t('timeline.title', 'Nachrichtenverlauf');

  const flatItems: FlatItem[] = [];
  if (isLoadingMore) {
    flatItems.push({ type: 'spinner' });
  }
  for (const section of sections) {
    flatItems.push({ type: 'header', label: section.label });
    for (const entry of section.entries) {
      flatItems.push({ type: 'entry', entry });
    }
  }

  const listRef = useRef<HTMLDivElement>(null);
  const [scrollMargin, setScrollMargin] = useState(0);

  useLayoutEffect(() => {
    setScrollMargin(listRef.current?.offsetTop ?? 0);
  }, []);

  // oxlint-disable-next-line react/incompatible-library
  const virtualizer = useWindowVirtualizer({
    count: flatItems.length,
    estimateSize: (index) => {
      const item = flatItems[index];
      if (item?.type === 'header') return ESTIMATED_HEADER_HEIGHT;
      if (item?.type === 'spinner') return ESTIMATED_SPINNER_HEIGHT;
      return ESTIMATED_ENTRY_HEIGHT;
    },
    overscan: VIRTUALIZER_OVERSCAN,
    scrollMargin,
  });

  const virtualItems = virtualizer.getVirtualItems();

  const startReachedFired = useRef(false);
  useEffect(() => {
    if (!onStartReached || isLoadingMore) return;
    const first = virtualItems[0];
    if (first?.index === 0 && !startReachedFired.current) {
      startReachedFired.current = true;
      onStartReached();
    } else if (!first || first.index > 0) {
      startReachedFired.current = false;
    }
  }, [virtualItems, onStartReached, isLoadingMore]);

  return (
    <div className="timeline-nav">
      <h2 className="timeline-title">{heading}</h2>
      <div
        ref={listRef}
        className="timeline-content"
        style={{ height: virtualizer.getTotalSize(), position: 'relative' }}
      >
        {virtualItems.map((virtualItem) => {
          const item = flatItems[virtualItem.index];
          if (!item) return null;
          return (
            <div
              key={virtualItem.key}
              ref={virtualizer.measureElement}
              data-index={virtualItem.index}
              style={{
                position: 'absolute',
                top: 0,
                left: 0,
                width: '100%',
                transform: `translateY(${virtualItem.start - virtualizer.options.scrollMargin}px)`,
              }}
            >
              {item.type === 'spinner' && (
                <LoadingSpinner
                  inline
                  text={t('timeline.loadingMore', 'Weitere Nachrichten laden…')}
                />
              )}
              {item.type === 'header' && (
                <div className="timeline-section-wrapper">
                  <div className="timeline-section">
                    <span className="timeline-section-label">{item.label}</span>
                  </div>
                </div>
              )}
              {item.type === 'entry' && (
                <ErrorBoundary fallback={null}>
                  <div className="timeline-row">
                    <TimelineMarker
                      selected={selectedId === item.entry.id}
                      status={item.entry.status}
                      isNew={item.entry.isNew}
                    />
                    <TimelineCard
                      entry={item.entry}
                      selected={selectedId === item.entry.id}
                      onSelect={onSelect}
                    />
                  </div>
                </ErrorBoundary>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}
