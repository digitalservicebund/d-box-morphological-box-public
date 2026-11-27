// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { z } from 'zod';

export const roomSchema = z.compile(
  z.object({
    id: z.string(),
    name: z.string(),
    topic: z.string().nullable(),
    encrypted: z.boolean(),
    invited: z.boolean(),
    citizen: z.string().nullable().optional(),
    createdAt: z.number().nonnegative().nullable().default(null),
    updatedAt: z.number().nonnegative().nullable().default(null),
  }),
);
export type CaseRoom = z.infer<typeof roomSchema>;
export const profileSchema = z.compile(
  z.object({
    mxid: z.string(),
    displayName: z.string().nullable(),
    avatarUrl: z.string().nullable().default(null),
    canSetDisplayName: z.boolean().default(true),
    canSetAvatar: z.boolean().default(true),
  }),
);
export type Profile = z.infer<typeof profileSchema>;
export const senderProfileSchema = z.compile(profileSchema.pick({ mxid: true, displayName: true, avatarUrl: true }));
export type SenderProfile = z.infer<typeof senderProfileSchema>;
export const matrixSessionSchema = z.compile(
  z.looseObject({
    user_id: z.string().min(1),
    device_id: z.string().min(1),
    access_token: z.string().min(1),
    refresh_token: z.string().nullable().optional(),
  }),
);
export type MatrixSessionData = z.infer<typeof matrixSessionSchema>;
export interface HistoryPage {
  events: unknown[];
  next: string | null;
}
export interface SyncUpdate {
  nextBatch: string;
  rooms: { roomId: string; events: unknown[]; limited: boolean }[];
  keysChanged: boolean;
}
export interface CaseInput {
  recipient: string;
  name: string;
  topic: string;
  encrypted: boolean;
}
export interface RemoveCaseResult {
  forgotten: boolean;
  warning: string | null;
}
export interface WorkerRequest {
  id: number;
  method: string;
  args: unknown[];
}
export interface WorkerResponse {
  id: number;
  value?: unknown;
  error?: string;
}
export interface SessionUpdate {
  kind: 'session';
  session: unknown;
}
