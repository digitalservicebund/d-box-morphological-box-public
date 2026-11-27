// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

export interface TimelineReply {
  date: string;
  sender: string;
  title: string;
  body?: string;
}

export interface TimelineEntry {
  id: string;
  date: string;
  sender?: string;
  title: string;
  body?: string;
  attachmentName?: string;
  extraCount?: number;
  deadline?: string;
  reply?: TimelineReply;
  status?: 'default' | 'warning' | 'success';
  isNew?: boolean;
}

export interface TimelineSection {
  id: string;
  label: string;
  entries: TimelineEntry[];
}

export interface TimelineProps {
  title?: string;
  sections: TimelineSection[];
  selectedId?: string;
  onSelect?: (id: string) => void;
  onStartReached?: () => void;
  isLoadingMore?: boolean;
}
