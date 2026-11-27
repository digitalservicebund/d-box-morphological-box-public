// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { ReactNode } from 'react';
import React, { createContext } from 'react';
import type ClientStore from './ClientStore';

interface ClientContextType {
  clientStore: ClientStore;
}

/**
 * React context for managing client-related state and functionality.
 *
 * Provides access to client data, operations, and state management throughout
 * the component tree. The context value can be undefined when used outside
 * of a ClientProvider.
 *
 * @example
 * ```tsx
 * const clientContext = useContext(ClientContext);
 * if (!clientContext) {
 *   throw new Error('ClientContext must be used within a ClientProvider');
 * }
 * ```
 */
export const ClientContext = createContext<ClientContextType | undefined>(
  undefined,
);
export type { ClientContextType };

/**
 * Props for the ClientProvider component.
 *
 * @interface ClientProviderProps
 * @property {ClientStore} clientStore - The client store instance to be provided to child components
 * @property {ReactNode} children - The child components that will have access to the client context
 */
interface ClientProviderProps {
  clientStore: ClientStore;
  children: ReactNode;
}

/**
 * Provider component that makes the client store available to child components through React Context.
 *
 * @param props - The provider props
 * @param props.clientStore - The client store instance to be provided to child components
 * @param props.children - React child elements that will have access to the client context
 * @returns A React Context Provider that wraps children with client store access
 */
export const ClientProvider: React.FC<ClientProviderProps> = ({
  clientStore,
  children,
}) => {
  return (
    <ClientContext.Provider value={{ clientStore }}>
      {children}
    </ClientContext.Provider>
  );
};
