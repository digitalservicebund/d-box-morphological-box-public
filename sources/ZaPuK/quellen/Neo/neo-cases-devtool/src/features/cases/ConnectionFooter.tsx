// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { useAuth } from '../auth/auth-context';
import styles from './ConnectionFooter.module.css';
import { homeserverDomain } from '../../utils/matrix-address';

export const ConnectionFooter: FC = () => {
  const { connected, profile, phase, homeserver } = useAuth();
  const status = connected
    ? 'Connected'
    : profile
      ? 'Connecting…'
      : phase === 'loggingIn'
        ? 'Signing in…'
        : 'Not connected';
  return (
    <footer className={styles.footer} role="status" aria-live="polite">
      <span className={styles.status}>
        <span className={styles.dot} data-connected={connected} data-active={Boolean(profile)} aria-hidden="true" />
        {status}
      </span>
      <span className={styles.server} title={homeserver}>
        {homeserverDomain(homeserver)}
      </span>
    </footer>
  );
};
