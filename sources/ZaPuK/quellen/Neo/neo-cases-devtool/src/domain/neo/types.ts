// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { BlockKind } from './identifiers';

export interface MatrixEvent {
  [key: string]: unknown;
  event_id: string;
  sender: string;
  origin_server_ts: number;
  type: string;
  content: Record<string, unknown>;
  state_key?: string;
  redacts?: string;
  unsigned?: Record<string, unknown>;
}

export interface Envelope {
  id: string;
  sender: string;
  timestamp: number;
  blocks: MatrixEvent[];
}
export interface Dialog {
  id: string;
  timestamp: number;
  agency: Envelope;
  responses: Envelope[];
}
export interface EventPayload {
  type: string;
  content: Record<string, unknown>;
}
export interface BlockDraft {
  id: string;
  kind: BlockKind;
  text: string;
  html: string;
  title: string;
  label: string;
  url: string;
  answers: { id: string; text: string }[];
  multiple: boolean;
  file?: File;
}
