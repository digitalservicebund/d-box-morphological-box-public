/*
 * SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
 *
 * SPDX-License-Identifier: EUPL-1.2
 */

import { useTranslation } from 'react-i18next';
import { useLanguage } from '../../hooks/useLanguage.tsx';
import styles from './language-menu-panel.module.css';

interface LanguageMenuPanelProps {
  onClose: () => void;
}

export default function LanguageMenuPanel({ onClose }: LanguageMenuPanelProps) {
  const { t } = useTranslation();
  const { currentLanguage, supportedLanguages, changeLanguage, languageNames } =
    useLanguage();

  return (
    <div
      id="bund-id-header-language-menu"
      className={styles.panel}
      role="listbox"
      aria-label={t('bundIdHeader.language', 'Sprache')}
    >
      {supportedLanguages.map((language) => (
        <button
          key={language}
          className={styles.option}
          onClick={() => {
            changeLanguage(language);
            onClose();
          }}
          role="option"
          aria-selected={currentLanguage === language}
        >
          {languageNames.of(language)}
        </button>
      ))}
    </div>
  );
}
