// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useLanguage } from '../../hooks/useLanguage.tsx';
import { useUsername } from '../../hooks/useUsername';
import ChevronDown from '../Icons/ChevronDown.tsx';
import ChevronRightAltIcon from '../Icons/ChevronRightAlt.tsx';
import CloseAltIcon from '../Icons/CloseAlt.tsx';
import GlobeIcon from '../Icons/Globe.tsx';
import HelpIcon from '../Icons/Help.tsx';
import MenuIcon from '../Icons/Menu.tsx';
import SearchIcon from '../Icons/Search.tsx';
import UserIcon from '../Icons/User.tsx';
import backgroundSvg from './assets/background.svg';
import logoSvg from './assets/logo.svg';
import LanguageMenuPanel from './LanguageMenuPanel.tsx';
import UserMenuPanel from './UserMenuPanel.tsx';
import styles from './bund-id-header.module.css';

interface BundIdHeaderProps {
  /** Optional display-name override, useful for isolated component previews. */
  userName?: string;
  onSearch?: () => void;
  /** Called when the user clicks "Mein Konto" in the profile dropdown. */
  onUser?: () => void;
  /**
   * When provided the user icon and profile dropdown are rendered.
   * When omitted (e.g. on the login page) the user area is hidden entirely.
   */
  onLogout?: () => void;
}

export default function BundIdHeader({
  userName,
  onSearch,
  onUser,
  onLogout,
}: BundIdHeaderProps) {
  const { t } = useTranslation();
  const loggedInUserName = useUsername();
  const currentUserName = userName ?? loggedInUserName;
  const { currentLanguage, languageNames } = useLanguage();
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const [userMenuOpen, setUserMenuOpen] = useState(false);
  const [languageMenuOpen, setLanguageMenuOpen] = useState(false);

  return (
    <header
      className={styles.header}
      style={
        {
          '--bund-id-header-bg-image': `url(${backgroundSvg})`,
        } as React.CSSProperties
      }
    >
      <div className={styles.container}>
        <div className={`${styles['mobile-left']} ${styles.mobile}`}>
          <button
            className={styles['mobile-button']}
            onClick={() => setMobileMenuOpen((v) => !v)}
            aria-label={t('bundIdHeader.menu', 'Menü')}
            aria-expanded={mobileMenuOpen}
          >
            <MenuIcon />
          </button>
          <button
            role="search"
            aria-label={t('bundIdHeader.search', 'Suche')}
            className={styles['mobile-button']}
            onClick={onSearch}
          >
            <SearchIcon />
          </button>
        </div>
        <a href={'/'} className={styles['logo-link']}>
          <img className={styles.logo} src={logoSvg} alt={'BundID logo'} />
        </a>
        <div className={styles.actions}>
          <button
            role="search"
            className={`${styles.button} ${styles.desktop}`}
            onClick={onSearch}
          >
            <SearchIcon />
            <span>{t('bundIdHeader.search', 'Suche')}</span>
          </button>
          <a href={'/faq'} className={`${styles.button} ${styles.desktop}`}>
            <HelpIcon />
            {t('bundIdHeader.help', 'Hilfe')}
          </a>
          <div className={styles['language-area']}>
            <button
              className={`${styles.button} ${styles.desktop}`}
              onClick={() => setLanguageMenuOpen((open) => !open)}
              aria-label={t('bundIdHeader.language', 'Sprache')}
              aria-expanded={languageMenuOpen}
              aria-haspopup="listbox"
              aria-controls="bund-id-header-language-menu"
            >
              <span className={styles['globe-icon']}>
                <GlobeIcon />
              </span>
              {languageNames.of(currentLanguage)}
              <ChevronDown />
            </button>
            {languageMenuOpen && (
              <LanguageMenuPanel
                onClose={() => {
                  setLanguageMenuOpen(false);
                }}
              />
            )}
          </div>
          {onLogout !== undefined && (
            <div className={styles['user-area']}>
              <button
                className={styles.button}
                onClick={() => setUserMenuOpen((v) => !v)}
                aria-label={
                  currentUserName ??
                  t('bundIdHeader.userProfile', 'Nutzerprofil')
                }
                aria-expanded={userMenuOpen}
              >
                <UserIcon />
                <span className={styles.desktop}>{currentUserName}</span>
                <ChevronDown />
              </button>
              {userMenuOpen && (
                <UserMenuPanel
                  onClickAccount={onUser}
                  onClickLogout={onLogout}
                  setUserMenuOpen={setUserMenuOpen}
                  userName={currentUserName}
                />
              )}
            </div>
          )}
        </div>
      </div>

      {mobileMenuOpen && (
        <div className={styles['nav-overlay']}>
          <div className={styles['nav-overlay-bar']}>
            <span className={styles['nav-overlay-spacer']} aria-hidden="true" />
            <button
              className={styles['nav-close-button']}
              onClick={() => setMobileMenuOpen(false)}
              aria-label={t('bundIdHeader.closeMenu', 'Menü schließen')}
            >
              <CloseAltIcon />
            </button>
          </div>
          <div>
            <button
              className={styles['nav-item']}
              onClick={() => {
                setMobileMenuOpen(false);
                setLanguageMenuOpen(true);
              }}
            >
              <GlobeIcon />
              <span>{languageNames.of(currentLanguage)}</span>
              <ChevronRightAltIcon />
            </button>
            <a
              href={'/faq'}
              className={styles['nav-item']}
              onClick={() => setMobileMenuOpen(false)}
            >
              {t('bundIdHeader.help', 'Hilfe')}
            </a>
          </div>
        </div>
      )}
    </header>
  );
}
