// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useMemo, useRef } from 'react';
import type { MatrixService } from '../../matrix/matrix-service';
import type { StoredSession } from '../../matrix/storage';
import type { SyncUpdate } from '../../matrix/types';

/** Owns the mutable SDK lifecycle separately from React's rendered authentication state. */
export const useClientLifecycle = () => {
  const active = useRef<MatrixService | null>(null);
  const stored = useRef<StoredSession | undefined>(undefined);
  const generation = useRef(0);
  const listeners = useRef(new Set<(update: SyncUpdate) => void>());
  return useMemo(() => ({ active, stored, generation, listeners }), [active, stored, generation, listeners]);
};
