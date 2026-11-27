// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { afterEach, expect, it, vi } from 'vite-plus/test';
import { MatrixService } from './matrix-service';

afterEach(() => vi.unstubAllGlobals());

it.each([
  {
    method: 'invite',
    args: ['!case:test', '@citizen:example.org'],
    run: (service: MatrixService) => service.invite('!case:test', '@citizen:example.org'),
  },
  {
    method: 'updateCaseMetadata',
    args: ['!case:test', 'New title', 'New agency'],
    run: (service: MatrixService) => service.updateCaseMetadata('!case:test', 'New title', 'New agency'),
  },
])('$method targets the selected room and allows retry after a server error', async ({ method, args, run }) => {
  class WorkerStub {
    onmessage?: (event: { data: { id: number; error?: string } }) => void;
    postMessage = vi.fn();
    terminate = vi.fn();
  }
  const worker = new WorkerStub();
  vi.stubGlobal(
    'Worker',
    class {
      constructor() {
        return worker;
      }
    },
  );
  const service = new MatrixService();
  try {
    const failed = run(service);
    const rejection = expect(failed).rejects.toThrow('Operation failed');
    expect(worker.postMessage).toHaveBeenLastCalledWith({
      id: 1,
      method,
      args,
    });
    worker.onmessage?.({ data: { id: 1, error: 'Operation failed' } });
    await rejection;
    const retry = run(service);
    expect(worker.postMessage).toHaveBeenLastCalledWith({
      id: 2,
      method,
      args,
    });
    worker.onmessage?.({ data: { id: 2 } });
    await expect(retry).resolves.toBeUndefined();
  } finally {
    service.dispose();
  }
});
