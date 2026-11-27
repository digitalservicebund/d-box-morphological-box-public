// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { ClientStore } from '@matrix-g2x/neo-web-sdk/react';

export function createSessionClientBuilder() {
  const store = new ClientStore({
    dbName: 'session-binding-test',
    matrixHomeserver: 'https://example.invalid',
  });
  return Boolean(store.getClientBuilder());
}
