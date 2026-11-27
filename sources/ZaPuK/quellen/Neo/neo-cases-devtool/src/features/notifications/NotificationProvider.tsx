// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useRef, useState, type FC, type PropsWithChildren } from 'react';
import { NotificationContext, type Notice, type Notify } from './notification-context';
import { Notification } from './Notification';

export const NotificationProvider: FC<PropsWithChildren> = ({ children }) => {
  const [queue, setQueue] = useState<Notice[]>([]);
  const id = useRef(0);
  const notify = useCallback<Notify>((message, kind = 'info') => {
    setQueue((current) =>
      current.some((notice) => notice.message === message)
        ? current
        : [...current.slice(-4), { id: ++id.current, message, kind }],
    );
  }, []);
  const dismiss = useCallback(() => setQueue((current) => current.slice(1)), []);
  return (
    <NotificationContext.Provider value={notify}>
      {children}
      {queue[0] && <Notification key={queue[0].id} notice={queue[0]} onDismiss={dismiss} />}
    </NotificationContext.Provider>
  );
};
