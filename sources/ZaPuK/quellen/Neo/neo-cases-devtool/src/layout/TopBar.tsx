// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useState, type FC } from 'react';
import { MdLogout } from 'react-icons/md';
import { Button } from '../components/ui/Button';
import { useAuth } from '../features/auth/auth-context';
import { useWorkspace } from '../features/cases/workspace-context';
import styles from './TopBar.module.css';
import { ProfileDialog } from '../features/profile/ProfileDialog';
import { UserAvatar } from '../features/profile/UserAvatar';

export const TopBar: FC = () => {
  const { profile, logout, phase } = useAuth();
  const { draftOpen } = useWorkspace();
  const [editing, setEditing] = useState(false);
  const close = useCallback(() => setEditing(false), []);
  return (
    <>
      <div inert={draftOpen} className={styles.bar}>
        <m3-top-app-bar>
          <span slot="navigation-icon" className={styles.logo}>
            <img src={`${import.meta.env.BASE_URL}neo.svg`} alt="Neo" width="96" height="32" />
          </span>
          <div slot="actions" className={styles.actions}>
            {profile ? (
              <>
                <Button
                  variant="text"
                  label="Edit profile"
                  onClick={() => setEditing(true)}
                  disabled={phase === 'loggingOut'}
                >
                  <span className={styles.user}>
                    <UserAvatar avatarUrl={profile.avatarUrl} />
                    <span className={styles.identity}>
                      <strong>{profile.displayName || profile.mxid}</strong>
                      <span>{profile.mxid}</span>
                    </span>
                  </span>
                </Button>
                <Button icon={MdLogout} variant="text" onClick={() => void logout()} loading={phase === 'loggingOut'}>
                  Log out
                </Button>
              </>
            ) : (
              <span className={styles.loggedOut}>Not logged-in</span>
            )}
          </div>
        </m3-top-app-bar>
      </div>
      {editing && profile && <ProfileDialog profile={profile} onClose={close} />}
    </>
  );
};
