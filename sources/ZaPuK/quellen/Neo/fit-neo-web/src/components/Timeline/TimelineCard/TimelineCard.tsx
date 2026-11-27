// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import Attach from '../../Icons/Attach';
import Warning from '../../Icons/Badge/Warning';
import ChevronRight from '../../Icons/ChevronRight';
import type { TimelineEntry } from '../timelineTypes';
import './TimelineCard.css';

export default function TimelineCard({
  entry,
  selected,
  onSelect,
}: {
  entry: TimelineEntry;
  selected: boolean;
  onSelect?: (id: string) => void;
}) {
  const handleClick = onSelect ? () => onSelect(entry.id) : undefined;

  const cardClasses = [
    'timeline-card',
    selected ? 'timeline-card-selected' : null,
    entry.isNew ? 'timeline-card-new' : null,
  ]
    .filter(Boolean)
    .join(' ');

  const cardContent = (
    <div className="timeline-card-content">
      <div className="timeline-card-header">
        <div className="timeline-card-info">
          <div className="timeline-card-metadata">
            <span className="timeline-card-date">{entry.date}</span>
            {entry.sender && (
              <span className="timeline-card-sender">{entry.sender}</span>
            )}
          </div>
          <div className="timeline-card-details">
            <span className="timeline-card-title">{entry.title}</span>
            {entry.body && (
              <span className="timeline-card-body">{entry.body}</span>
            )}
          </div>
        </div>
        <div className="timeline-card-chevron" aria-hidden="true">
          <ChevronRight />
        </div>
      </div>

      {(entry.attachmentName !== undefined ||
        entry.extraCount !== undefined ||
        entry.deadline !== undefined) && (
        <div className="timeline-card-badges">
          {entry.deadline !== undefined && (
            <div className="timeline-badge timeline-badge-deadline">
              <span className="timeline-badge-icon">
                <Warning />
              </span>
              <span className="timeline-badge-label">{entry.deadline}</span>
            </div>
          )}
          {entry.attachmentName !== undefined && (
            <div className="timeline-badge">
              <span className="timeline-badge-icon">
                <Attach />
              </span>
              <span className="timeline-badge-label">
                {entry.attachmentName}
              </span>
            </div>
          )}
          {entry.extraCount !== undefined && entry.extraCount > 0 && (
            <div className="timeline-badge timeline-badge-count">
              <span className="timeline-badge-label">+{entry.extraCount}</span>
            </div>
          )}
        </div>
      )}

      {entry.reply && (
        <div className="timeline-reply">
          <div className="timeline-reply-content">
            <div className="timeline-reply-info">
              <span className="timeline-reply-date">{entry.reply.date}</span>
              <span className="timeline-reply-sender">
                {entry.reply.sender}
              </span>
            </div>
            <div className="timeline-reply-text">
              <span className="timeline-reply-title">{entry.reply.title}</span>
              {entry.reply.body && (
                <span className="timeline-reply-body">{entry.reply.body}</span>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );

  return (
    <button
      className={cardClasses}
      onClick={handleClick}
      type="button"
      aria-pressed={selected}
      aria-label={entry.title}
    >
      {cardContent}
    </button>
  );
}
