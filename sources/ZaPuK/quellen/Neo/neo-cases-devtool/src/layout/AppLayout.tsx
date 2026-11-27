// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdForum } from 'react-icons/md';
import { useAuth } from '../features/auth/auth-context';
import { LoginForm } from '../features/auth/LoginForm';
import { CaseList } from '../features/cases/CaseList';
import { useWorkspace } from '../features/cases/workspace-context';
import { DialogTimeline } from '../features/dialogs/DialogTimeline';
import { TopBar } from './TopBar';
import styles from './AppLayout.module.css';

export const AppLayout: FC = () => {
  const { phase } = useAuth();
  const { selectedId, draftOpen } = useWorkspace();
  return (
    <div className={styles.app}>
      <TopBar />
      <div className={styles.columns}>
        <CaseList />
        <main className={styles.main}>
          {phase !== 'loggedIn' && phase !== 'loggingOut' ? (
            <LoginForm />
          ) : selectedId ? (
            <DialogTimeline key={selectedId} roomId={selectedId} />
          ) : (
            <div className={styles.empty}>
              <MdForum size={36} aria-hidden="true" />
              <p>Select a case on the left first or create one</p>
            </div>
          )}
        </main>
      </div>
      {draftOpen && <div className={styles.scrim} aria-hidden="true" />}
    </div>
  );
};
