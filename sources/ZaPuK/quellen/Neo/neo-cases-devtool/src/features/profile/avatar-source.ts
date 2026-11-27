// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { MatrixService } from '../../matrix/matrix-service';

interface AvatarEntry {
  users: number;
  source: Promise<string>;
  objectUrl?: string;
}
const clients = new WeakMap<Pick<MatrixService, 'download'>, Map<string, AvatarEntry>>();
export interface AvatarSource {
  source: Promise<string>;
  release: () => void;
}

/** Shares one authenticated download/object URL across all cards displaying the same avatar. */
export const acquireAvatar = (client: Pick<MatrixService, 'download'>, uri: string): AvatarSource => {
  let cache = clients.get(client);
  if (!cache) {
    cache = new Map();
    clients.set(client, cache);
  }
  let entry = cache.get(uri);
  if (!entry) {
    const created: AvatarEntry = { users: 0, source: Promise.resolve('') };
    const entries = cache;
    created.source = Promise.resolve()
      .then(() => client.download({ url: uri }))
      .then((data) => {
        const url = URL.createObjectURL(new Blob([new Uint8Array(data)]));
        created.objectUrl = url;
        if (!created.users) URL.revokeObjectURL(url);
        return url;
      })
      .catch((cause: unknown) => {
        if (entries.get(uri) === created) entries.delete(uri);
        throw cause;
      });
    cache.set(uri, created);
    entry = created;
  }
  entry.users += 1;
  const current = entry;
  let released = false;
  return {
    source: current.source,
    release: () => {
      if (released) return;
      released = true;
      current.users -= 1;
      if (current.users) return;
      if (cache.get(uri) === current) cache.delete(uri);
      if (current.objectUrl) URL.revokeObjectURL(current.objectUrl);
    },
  };
};
