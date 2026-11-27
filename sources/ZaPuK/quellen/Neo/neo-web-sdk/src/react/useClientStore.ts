// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useContext, useSyncExternalStore } from 'react';
import { ClientContext } from './ClientContext';
import type ClientStore from './ClientStore';

/**
 * Hook to access the client store from the ClientContext.
 *
 * @returns The client store instance
 * @throws {Error} When used outside of a ClientProvider
 *
 * @example
 * ```typescript
 * const MyComponent = () => {
 *   const clientStore = useClientStore();
 *   // Use the client store...
 * };
 * ```
 */
export const useClientStore = (): ClientStore => {
	const context = useContext(ClientContext);
	if (context === undefined) {
		throw new Error('useClientStore must be used within a ClientProvider');
	}
	const { clientStore } = context;
	// Subscribe to clientStore state changes so components re-render when
	// clientState changes (e.g. on logout after an expired refresh token).
	useSyncExternalStore(
		clientStore.subscribe.bind(clientStore),
		clientStore.getSnapshot.bind(clientStore),
	);
	return clientStore;
};
