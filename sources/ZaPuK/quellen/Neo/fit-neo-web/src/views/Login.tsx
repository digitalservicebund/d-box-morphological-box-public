// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { ClientState, useClientStore } from '@matrix-g2x/neo-web-sdk/react';
import { useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Navigate, useNavigate } from 'react-router';
import Button from '../components/Button/Button';
import AlertDiamond from '../components/Icons/AlertDiamond';
import ArrowRight from '../components/Icons/ArrowRight';
import AppLayout from './AppLayout';
import './Login.css';
import {
  OIDC_BROADCAST_CHANNEL,
  type OidcCallbackMessage,
} from './OidcCallback';

function Login() {
  const { t } = useTranslation();
  const clientStore = useClientStore();
  const navigate = useNavigate();

  const isAuthenticated = clientStore.clientState === ClientState.LoggedIn;

  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | undefined>();
  const [retryHint] = useState<boolean>(() => {
    const flag = sessionStorage.getItem('fit-neo-login-retry');
    if (flag) sessionStorage.removeItem('fit-neo-login-retry');
    return flag !== null;
  });
  const popupRef = useRef<Window | null>(null);

  // Listen for the callback URL broadcast by the OidcCallback popup.
  // BroadcastChannel is used instead of window.opener.postMessage because
  // MAS sets Cross-Origin-Opener-Policy which nulls window.opener.
  useEffect(() => {
    const channel = new BroadcastChannel(OIDC_BROADCAST_CHANNEL);

    channel.onmessage = async (event: MessageEvent<OidcCallbackMessage>) => {
      if (event.data?.type !== 'oidc-callback') return;

      popupRef.current?.close();
      popupRef.current = null;

      try {
        await clientStore.completeOidcLogin(event.data.callbackUrl);
        void navigate('/', { replace: true });
      } catch (err) {
        console.error('OIDC login complete failed:', err);
        setError(
          t(
            'login.errors.oidcFailed',
            'Anmeldung fehlgeschlagen. Bitte versuchen Sie es erneut.',
          ),
        );
        setIsLoading(false);
      }
    };

    return () => channel.close();
  }, [clientStore, navigate, t]);

  // If already authenticated, redirect to home
  if (isAuthenticated) {
    return <Navigate to="/" replace />;
  }

  const handleOidcLogin = async () => {
    setIsLoading(true);
    setError(undefined);

    const popup = window.open(
      'about:blank',
      `oidc-login-${Date.now()}`,
      'popup=yes,width=520,height=700,left=400,top=100',
    );

    try {
      const url = await clientStore.startOidcLogin();

      if (!popup || popup.closed) {
        // Popup was blocked — fall back to a full-page redirect.
        window.location.href = url;
        return;
      }
      popup.location.href = url;
      popupRef.current = popup;
    } catch (err) {
      popup?.close();
      console.error('startOidcLogin failed:', err);
      setError(
        t(
          'login.errors.oidcFailed',
          'Anmeldung fehlgeschlagen. Bitte versuchen Sie es erneut.',
        ),
      );
      setIsLoading(false);
    }
  };

  return (
    <>
      <AppLayout>
        <div
          style={{
            minHeight: 'calc(100vh - 200px)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            padding: '2rem 1rem',
          }}
        >
          <div style={{ width: '100%', maxWidth: '600px' }}>
            <div
              style={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                gap: '2rem',
              }}
            >
              {/* Adler.svg logo */}
              <img src="/adler.svg" alt="Logo" style={{ width: '120px' }} />

              <p className="kern-body">
                {t(
                  'login.subtitle',
                  'Einfach und sicher mit der Verwaltung kommunizieren',
                )}
              </p>

              <p
                className="kern-body"
                style={{
                  textAlign: 'center',
                  color: 'var(--layout-text-muted)',
                }}
              >
                {t(
                  'login.description',
                  'Klicken Sie auf „Mit BundID anmelden", um sich mit Ihrem BundID-Konto anzumelden. Sie werden in einem Popup-Fenster zur sicheren BundID-Anmeldeseite weitergeleitet und kehren danach automatisch hierher zurück.',
                )}
              </p>

              {retryHint && !error && (
                <p className="login-hint" role="status">
                  {t(
                    'login.retryHint',
                    'Die Anmeldung konnte nicht abgeschlossen werden. Bitte versuchen Sie es erneut.',
                  )}
                </p>
              )}

              {error && (
                <p className="login-error" role="alert">
                  <AlertDiamond />
                  {error}
                </p>
              )}

              <Button
                type="primary"
                disabled={isLoading}
                onClick={handleOidcLogin}
                icon={<ArrowRight />}
              >
                {isLoading
                  ? t('login.signingIn', 'Anmeldung läuft ...')
                  : t('login.signInButton', 'Mit BundID anmelden')}
              </Button>

              <p
                className="kern-body"
                style={{
                  textAlign: 'center',
                  fontSize: 'var(--typography-font-size-static-small)',
                  color: 'var(--layout-text-muted)',
                }}
              >
                {t(
                  'login.popupNote',
                  'Sollte sich kein Popup öffnen, prüfen Sie bitte Ihren Popup-Blocker.',
                )}
              </p>
            </div>
          </div>
        </div>
      </AppLayout>
    </>
  );
}

export default Login;
