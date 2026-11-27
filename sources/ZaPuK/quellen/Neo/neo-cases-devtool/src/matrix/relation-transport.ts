// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

const relationsKey = 'im.nheko.relations.v1.relations';

/**
 * Copies Neo relationships to the encrypted wrapper without touching its ciphertext.
 * @param body SDK-produced encrypted event content.
 * @param relations Relationships from the original custom event.
 * @returns Updated encrypted event content.
 */
export const exposeRelations = (body: Record<string, unknown>, relations: unknown): Record<string, unknown> => ({
  ...body,
  [relationsKey]: relations,
});

/**
 * Installs a worker-local transport shim for SDK sends registered by transaction URL.
 * @returns A registry of only the currently pending custom-event sends.
 */
export const installRelationTransport = (): Map<string, unknown> => {
  const pending = new Map<string, unknown>();
  const original = globalThis.fetch.bind(globalThis);
  globalThis.fetch = async (input, init) => {
    const request = new Request(input, init);
    const url = new URL(request.url);
    const relations = pending.get(url.origin + decodeURIComponent(url.pathname));
    if (request.method === 'PUT' && relations) {
      const body = (await request.clone().json()) as Record<string, unknown>;
      if (body.algorithm === 'm.megolm.v1.aes-sha2') {
        const headers = new Headers(request.headers);
        headers.delete('content-length');
        return original(
          new Request(request, { method: 'PUT', headers, body: JSON.stringify(exposeRelations(body, relations)) }),
        );
      }
    }
    return original(request);
  };
  return pending;
};
