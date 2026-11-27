// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useTranslation } from 'react-i18next';
import { LoadingSpinner } from './LoadingSpinner/LoadingSpinner';

export function LazyFallback() {
  const { t } = useTranslation();
  return (
    <div
      style={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        height: '100vh',
      }}
    >
      <LoadingSpinner text={t('loading', 'Lädt...')} />
    </div>
  );
}
