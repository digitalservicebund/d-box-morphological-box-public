// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { ClientState, useClientStore } from '@matrix-g2x/neo-web-sdk/react';
import { Navigate, Outlet, useLocation } from 'react-router';

/**
 * A route component that protects private pages by checking user authentication status.
 *
 * @param props - The component props
 * @param props.children - The child components to render when user is authenticated
 * @returns The children components if authenticated, otherwise redirects to login page
 *
 * @remarks
 * This component checks if the user is logged in or currently loading a session.
 * If not authenticated, it redirects to the login page while preserving the
 * current location for potential redirect after successful authentication.
 */
const PrivateRoute = () => {
  const clientStore = useClientStore();
  const isAuthenticated =
    clientStore.clientState === ClientState.LoggedIn ||
    clientStore.clientState === ClientState.LoadingSession;
  const location = useLocation();

  if (isAuthenticated) {
    return <Outlet />;
  }

  return <Navigate to="/login" state={{ from: location }} replace />;
};

export default PrivateRoute;
