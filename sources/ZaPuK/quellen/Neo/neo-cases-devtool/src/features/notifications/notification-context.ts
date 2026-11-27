// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { createContext, useContext } from 'react';

export interface Notice {
  id: number;
  message: string;
  kind: 'success' | 'error' | 'info';
}
export type Notify = (message: string, kind?: Notice['kind']) => void;
export const NotificationContext = createContext<Notify | undefined>(undefined);
export const useNotify = (): Notify => {
  const context = useContext(NotificationContext);
  if (!context) throw new Error('NotificationProvider is missing.');
  return context;
};
