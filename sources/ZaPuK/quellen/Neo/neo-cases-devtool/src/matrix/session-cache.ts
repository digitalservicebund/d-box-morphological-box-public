// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { matrixSessionSchema, type MatrixSessionData } from './types';

/** Holds the SDK's live session, including rotated tokens, outside React state. */
export class SessionCache {
  private current?: MatrixSessionData;

  update(value: unknown): void {
    this.current = matrixSessionSchema.parse(value);
  }

  snapshot(): MatrixSessionData {
    if (!this.current) throw new Error('No active Matrix session.');
    return { ...this.current };
  }

  accessToken(): string {
    return this.snapshot().access_token;
  }

  clear(): void {
    this.current = undefined;
  }
}
