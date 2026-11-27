// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { ClientState, useClientStore } from '@matrix-g2x/neo-web-sdk/react';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router';
import BundIdHeader from '../components/BundIdHeader/BundIdHeader';
import Link from '../components/Link/Link';
import './AppLayout.css';

export default function AppLayout({ children }: { children: React.ReactNode }) {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const clientStore = useClientStore();
  const isLoggedIn = clientStore.clientState === ClientState.LoggedIn;

  return (
    <div className="app-layout">
      <a href="#main-content" className="visually-hidden skip-link">
        {t('skipToContent', 'Zum Inhalt springen')}
      </a>
      <BundIdHeader
        onLogout={isLoggedIn ? () => navigate('/logout') : undefined}
      />
      <main id="main-content">{children}</main>
      <footer>
        <Link href="/impress">{t('imprint', 'Impressum')}</Link>
        <Link href="/datenschutz">{t('dataProtection', 'Datenschutz')}</Link>
        <Link href="/accessibility">
          {t('accessibility', 'Erklärung zur Barrierefreiheit')}
        </Link>
        {/* TODO add search icon here */}
      </footer>
    </div>
  );
}
