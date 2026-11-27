// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useClientStore } from '@matrix-g2x/neo-web-sdk/react';
import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import AppLayout from './AppLayout';

function Logout() {
  const { t } = useTranslation();
  const clientStore = useClientStore();

  // On mount, trigger logout. logout() handles the hard redirect to /login
  // itself (after stopping the sync service) — a full page reload is the only
  // reliable way to release WASM IndexedDB connections so the next login can
  // clear the stores without being blocked.
  useEffect(() => {
    clientStore.logout();
    // oxlint-disable-next-line react/exhaustive-deps -- run on mount only
  }, []);

  return (
    <AppLayout>
      <p>{t('loggingOut', 'Logging out...')}</p>
    </AppLayout>
  );
}

export default Logout;
