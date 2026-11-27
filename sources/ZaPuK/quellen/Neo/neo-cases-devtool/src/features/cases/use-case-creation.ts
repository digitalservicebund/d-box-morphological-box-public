// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useRef, useState } from 'react';
import type { CaseTemplate } from '../../domain/neo/case-template';
import type { CaseInput } from '../../matrix/types';
import { importCaseDialogs, type ImportProgress } from '../../matrix/import-case';
import { useAuth } from '../auth/auth-context';

export const useCaseCreation = () => {
  const [busy, setBusy] = useState(false);
  const [createdId, setCreatedId] = useState<string>();
  const [completed, setCompleted] = useState(0);
  const room = useRef<string | undefined>(undefined);
  const running = useRef(false);
  const progress = useRef<ImportProgress>({ completed: 0 });
  const { service, profile, refreshRooms } = useAuth();
  const create = async (input: CaseInput, template?: CaseTemplate): Promise<string | undefined> => {
    if (running.current || !service || !profile) return;
    running.current = true;
    setBusy(true);
    try {
      if (!room.current) {
        room.current = await service.create(input);
        setCreatedId(room.current);
      }
      if (template)
        await importCaseDialogs(service, profile.mxid, room.current, template, progress.current, setCompleted);
      await refreshRooms();
      return room.current;
    } finally {
      running.current = false;
      setBusy(false);
    }
  };
  return { busy, createdId, completed, create };
};
