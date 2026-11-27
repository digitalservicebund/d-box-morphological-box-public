/*
 * SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
 *
 * SPDX-License-Identifier: EUPL-1.2
 */

import { useTranslation } from 'react-i18next';
import TrustLevelIcon from '../Icons/TrustLevel.tsx';
import styles from './user-menu-panel.module.css';

interface UserMenuPanelProps {
  onClickAccount?: () => void;
  onClickLogout?: () => void;
  setUserMenuOpen: (open: boolean) => void;
  userName?: string;
}

/**
 * Backdrop + dropdown panel, rendered inside the user-area (position: relative)
 * so `right: 0` aligns with the person-icon button's right edge.
 */
const UserMenuPanel = ({
  onClickAccount,
  onClickLogout,
  setUserMenuOpen,
  userName,
}: UserMenuPanelProps) => {
  const { t } = useTranslation();

  return (
    <>
      {/* Transparent full-screen backdrop closes the menu on outside click */}
      <div
        className={styles.backdrop}
        onClick={() => setUserMenuOpen(false)}
        aria-hidden="true"
      />
      <div
        className={styles.panel}
        role="dialog"
        aria-label={t('bundIdHeader.userProfile', 'Nutzerprofil')}
      >
        <div className={styles.profile}>
          {userName && <p className={styles.name}>{userName}</p>}
          <div className={styles['trust-row']}>
            <div className={styles['trust-icon']}>
              <TrustLevelIcon />
            </div>
            <div className={styles.access}>
              {t(
                'bundIdHeader.userMenu.currentAccess',
                'AKTUELLER ZUGANG: Benutzername & Passwort',
              )}
            </div>
          </div>
          <div className={styles.level}>
            {t('bundIdHeader.userMenu.trustLevel', 'BASISREGISTRIERUNG')}
          </div>
        </div>
        <button
          className={styles.action}
          onClick={() => {
            setUserMenuOpen(false);
            onClickAccount?.();
          }}
        >
          {t('bundIdHeader.userMenu.myAccount', 'Mein Konto')}
        </button>
        <button
          className={styles.action}
          onClick={() => {
            setUserMenuOpen(false);
            onClickLogout?.();
          }}
        >
          {t('bundIdHeader.userMenu.logout', 'Abmelden')}
        </button>
      </div>
    </>
  );
};

export default UserMenuPanel;
