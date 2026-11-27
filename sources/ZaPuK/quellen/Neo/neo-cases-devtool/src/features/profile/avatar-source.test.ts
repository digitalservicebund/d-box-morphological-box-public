// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it, vi } from 'vite-plus/test';
import { acquireAvatar } from './avatar-source';

it('shares avatar downloads and retains the URL until the last card releases it', async () => {
  const create = vi.spyOn(URL, 'createObjectURL').mockReturnValue('blob:shared');
  const revoke = vi.spyOn(URL, 'revokeObjectURL').mockImplementation(() => undefined);
  try {
    const client = { download: vi.fn(async () => new Uint8Array([1, 2, 3])) };
    const first = acquireAvatar(client, 'mxc://test/avatar');
    const second = acquireAvatar(client, 'mxc://test/avatar');
    expect(await first.source).toBe(await second.source);
    expect(client.download).toHaveBeenCalledTimes(1);
    expect(create).toHaveBeenCalledTimes(1);
    first.release();
    expect(revoke).not.toHaveBeenCalled();
    second.release();
    second.release();
    expect(revoke).toHaveBeenCalledExactlyOnceWith('blob:shared');
  } finally {
    vi.restoreAllMocks();
  }
});

it('releases an avatar even if its last card disappears before the download finishes', async () => {
  vi.spyOn(URL, 'createObjectURL').mockReturnValue('blob:late');
  const revoke = vi.spyOn(URL, 'revokeObjectURL').mockImplementation(() => undefined);
  try {
    let finish!: (data: Uint8Array) => void;
    const download = new Promise<Uint8Array>((resolve) => {
      finish = resolve;
    });
    const avatar = acquireAvatar({ download: () => download }, 'mxc://test/late');
    avatar.release();
    finish(new Uint8Array([1]));
    await avatar.source;
    expect(revoke).toHaveBeenCalledExactlyOnceWith('blob:late');
  } finally {
    vi.restoreAllMocks();
  }
});

it('allows retrying a failed avatar without an old subscriber invalidating the new cache entry', async () => {
  vi.spyOn(URL, 'createObjectURL').mockReturnValue('blob:retry');
  const revoke = vi.spyOn(URL, 'revokeObjectURL').mockImplementation(() => undefined);
  try {
    const client = {
      download: vi
        .fn<() => Promise<Uint8Array>>()
        .mockRejectedValueOnce(new Error('Unavailable'))
        .mockResolvedValue(new Uint8Array([1])),
    };
    const failed = acquireAvatar(client, 'mxc://test/retry');
    await expect(failed.source).rejects.toThrow('Unavailable');
    const retried = acquireAvatar(client, 'mxc://test/retry');
    expect(await retried.source).toBe('blob:retry');
    failed.release();
    const shared = acquireAvatar(client, 'mxc://test/retry');
    expect(client.download).toHaveBeenCalledTimes(2);
    retried.release();
    shared.release();
    expect(revoke).toHaveBeenCalledExactlyOnceWith('blob:retry');
  } finally {
    vi.restoreAllMocks();
  }
});
