// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import i18n from 'i18next';
import LanguageDetector from 'i18next-browser-languagedetector';
import Backend from 'i18next-http-backend';
import { initReactI18next } from 'react-i18next';

void i18n
  .use(Backend)
  .use(LanguageDetector)
  .use(initReactI18next) // passes i18n down to react-i18next
  .init({
    fallbackLng: 'en', // default language to use
    supportedLngs: ['de', 'en'],

    interpolation: {
      escapeValue: false, // react already safes from xss
    },

    backend: {
      loadPath: '/locales/{{lng}}/{{ns}}.json',
    },
  });

// Update document title when language changes or translations are loaded
const updateDocumentTitle = () => {
  document.title = i18n.t('appTitle', 'My Digital Government');
};

// Listen for language change events
i18n.on('languageChanged', updateDocumentTitle);

// Listen for when translations are loaded
i18n.on('loaded', updateDocumentTitle);

// Set initial title after i18n is initialized
i18n.on('initialized', updateDocumentTitle);

export default i18n;
