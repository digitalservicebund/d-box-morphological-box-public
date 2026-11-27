// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

/* oxlint-disable react/only-export-components */

import { uniffiInitAsync } from '@matrix-g2x/neo-web-sdk';
import { ClientProvider, ClientStore } from '@matrix-g2x/neo-web-sdk/react';
import { lazy, StrictMode, Suspense } from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter, Route, Routes } from 'react-router';
import PrivateRoute from './components/PrivateRoute/index.tsx';
import { getConfig, loadConfig } from './config.ts';
import './i18n';

// Shared with the IndexedDB-clear workaround below — must match the dbName
// passed into `new ClientStore()` so the two can never drift apart.
const MATRIX_DB_PREFIX = 'fit-neo-';
export const MATRIX_DB_NAME = `${MATRIX_DB_PREFIX}matrix-store`;

// Lazy-loaded views for route-based code splitting
const Login = lazy(() => import('./views/Login.tsx'));
const Logout = lazy(() => import('./views/Logout.tsx'));
const MainView = lazy(() => import('./views/MainView.tsx'));
const OidcCallback = lazy(() => import('./views/OidcCallback.tsx'));
const Case = lazy(() => import('./views/Case.tsx'));

import { LazyFallback } from './components/LazyFallback.tsx';
import './css/fonts.css';
import './css/reset.css';
// Keep this side-effect import separate so the import organizer preserves order.
import './css/index.css';
import './css/theme.css';
import NotFound from './views/NotFound.tsx';

// Load runtime config before anything else. Throws if config.json is missing
// or returns a non-OK response, rendering an error instead of the app.
try {
  await loadConfig();
} catch (e) {
  document.getElementById('root')!.textContent =
    `Configuration error: ${e instanceof Error ? e.message : String(e)}`;
  throw e;
}

// If a previous login attempt detected a stale device ID in the crypto store,
// Clear all Matrix IndexedDB stores now — before any WASM objects are
// created so no connections are open and deleteDatabase succeeds immediately.
// We use indexedDB.databases() to discover all stores dynamically so that
// newly added stores (e.g. event cache) are also wiped without needing to
// maintain a hardcoded list.
if (sessionStorage.getItem('fit-neo-needs-db-clear')) {
  sessionStorage.removeItem('fit-neo-needs-db-clear');
  const deleteDb = (name: string) =>
    new Promise<void>((resolve) => {
      const req = indexedDB.deleteDatabase(name);
      req.onsuccess = () => resolve();
      req.onerror = () => resolve();
      req.onblocked = () => resolve(); // no connections exist at this point
    });
  const allDbs = await indexedDB.databases();
  await Promise.all(
    allDbs
      .map((db) => db.name)
      .filter((name): name is string => !!name?.startsWith(MATRIX_DB_PREFIX))
      .map(deleteDb),
  );
}

// Initialize the client store and attempt to load any existing session
await uniffiInitAsync();
const { oidc, matrixHomeserver } = getConfig();
const clientStore = new ClientStore({
  dbName: MATRIX_DB_NAME,
  matrixHomeserver,
  oidc: {
    clientName: oidc?.clientName,
    clientUri: oidc?.clientUri,
  },
});
await clientStore.tryLoadSession();

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ClientProvider clientStore={clientStore}>
      <BrowserRouter>
        <Suspense fallback={<LazyFallback />}>
          <Routes>
            <Route path="/" element={<PrivateRoute />}>
              <Route index element={<MainView />} />
              <Route path="documents" element={<MainView />} />
              <Route path="profile" element={<MainView />} />
              <Route path="settings" element={<MainView />} />
              {/* Antrag url with urlencoded roomId param */}
              <Route path="case/:roomId" element={<Case />} />
            </Route>
            <Route path="/login" element={<Login />} />
            <Route path="/login/callback" element={<OidcCallback />} />
            <Route path="/logout" element={<Logout />} />
            <Route path="*" element={<NotFound />} />
          </Routes>
        </Suspense>
      </BrowserRouter>
    </ClientProvider>
  </StrictMode>,
);
