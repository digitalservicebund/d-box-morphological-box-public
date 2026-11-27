// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

export const MAIN_TABS = (t: (key: string, fallback: string) => string) => [
  { to: '/', label: t('tabs.cases', 'Vorgänge'), aliases: ['/case'] },
  { to: '/documents', label: t('tabs.documents', 'Dokumente') },
  { to: '/profile', label: t('tabs.profile', 'Zugänge und Daten') },
  { to: '/settings', label: t('tabs.settings', 'Kontoaktivitäten') },
];
