/*
 * SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
 *
 * SPDX-License-Identifier: EUPL-1.2
 */

import { useTranslation } from 'react-i18next';

export const useLanguage = () => {
  const { i18n } = useTranslation();

  const currentLanguage = i18n.resolvedLanguage ?? i18n.language;
  const changeLanguage = (language: string) => {
    void i18n.changeLanguage(language);
  };
  const languageNames = new Intl.DisplayNames(i18n.languages, {
    type: 'language',
  });
  const allLanguages = i18n.options.supportedLngs || [];
  const supportedLanguages = allLanguages.filter((lang) => lang !== 'cimode');

  return {
    currentLanguage,
    changeLanguage,
    languageNames,
    supportedLanguages,
  };
};
