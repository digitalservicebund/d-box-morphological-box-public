// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { LoadingSpinner } from '../components/LoadingSpinner/LoadingSpinner';
import AppLayout from './AppLayout';

/** Channel name shared between Login.tsx and OidcCallback.tsx. */
export const OIDC_BROADCAST_CHANNEL = 'oidc-callback';

/** Message shape broadcast from OidcCallback to Login. */
export interface OidcCallbackMessage {
  type: 'oidc-callback';
  callbackUrl: string;
}

/**
 * Handles the OIDC redirect callback from the authorization server.
 *
 * Broadcasts the callback URL via BroadcastChannel to the Login page
 * (which still holds the client with the in-memory PKCE state), then
 * closes the popup.
 *
 * BroadcastChannel is used instead of window.opener.postMessage because
 * MAS (and most IdPs) set Cross-Origin-Opener-Policy: same-origin, which
 * nulls window.opener when the popup navigates to their domain.
 */
function OidcCallback() {
  const { t } = useTranslation();
  const calledRef = useRef(false);

  useEffect(() => {
    if (calledRef.current) return;
    calledRef.current = true;

    const channel = new BroadcastChannel(OIDC_BROADCAST_CHANNEL);
    const message: OidcCallbackMessage = {
      type: 'oidc-callback',
      callbackUrl: window.location.href,
    };
    channel.postMessage(message);
    channel.close();
    window.close();
  }, []);

  return (
    <AppLayout>
      <div
        style={{
          minHeight: 'calc(100vh - 200px)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        <LoadingSpinner
          text={t('login.completingSignIn', 'Anmeldung wird abgeschlossen ...')}
        />
      </div>
    </AppLayout>
  );
}

export default OidcCallback;
