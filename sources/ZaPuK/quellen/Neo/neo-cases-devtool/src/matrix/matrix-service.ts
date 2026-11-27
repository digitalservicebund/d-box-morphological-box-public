// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type {
  CaseInput,
  CaseRoom,
  HistoryPage,
  Profile,
  SyncUpdate,
  WorkerResponse,
  RemoveCaseResult,
  MatrixSessionData,
  SessionUpdate,
} from './types';
import { profileSchema, roomSchema, senderProfileSchema, type SenderProfile } from './types';
import { SessionCache } from './session-cache';

export class MatrixService {
  private readonly worker = new Worker(new URL('./matrix.worker.ts', import.meta.url), { type: 'module' });
  private sequence = 0;
  private disposed = false;
  private readonly sessionCache = new SessionCache();
  private readonly senderProfiles = new Map<string, { fetched: number; value: Promise<SenderProfile> }>();
  private sessionListener?: (session: MatrixSessionData) => void;
  private readonly pending = new Map<
    number,
    { resolve: (value: unknown) => void; reject: (cause: Error) => void; timer: ReturnType<typeof setTimeout> }
  >();

  constructor() {
    this.worker.onmessage = ({ data }: MessageEvent<WorkerResponse | SessionUpdate>) => {
      if (this.disposed) return;
      if ('kind' in data) {
        this.sessionCache.update(data.session);
        this.sessionListener?.(this.sessionCache.snapshot());
        return;
      }
      const job = this.pending.get(data.id);
      if (job) clearTimeout(job.timer);
      this.pending.delete(data.id);
      if (data.error) job?.reject(new Error(data.error));
      else job?.resolve(data.value);
    };
    this.worker.onerror = (event) => this.dispose(new Error(event.message || 'Matrix WASM worker failed to load'));
  }

  private call<T>(method: string, ...args: unknown[]): Promise<T> {
    if (this.disposed) return Promise.reject(new Error('Matrix session ended'));
    return new Promise((resolve, reject) => {
      const id = ++this.sequence;
      const timer = setTimeout(
        () => {
          this.pending.delete(id);
          reject(new Error(`Matrix ${method} timed out. Check the connection and retry.`));
        },
        ['upload', 'download', 'setAvatar'].includes(method) ? 600_000 : 180_000,
      );
      this.pending.set(id, { resolve: (value) => resolve(value as T), reject, timer });
      this.worker.postMessage({ id, method, args });
    });
  }

  open = async (homeserver: string, database: string, session?: unknown): Promise<void> => {
    if (session) this.sessionCache.update(session);
    await this.call('open', homeserver, database, session ? JSON.stringify(session) : undefined);
  };
  login = async (username: string, password: string): Promise<MatrixSessionData> => {
    this.sessionCache.update(await this.call('login', username, password));
    return this.sessionCache.snapshot();
  };
  homeserver = (): Promise<string> => this.call('homeserver');
  currentSession = (): MatrixSessionData => this.sessionCache.snapshot();
  accessToken = (): string => this.sessionCache.accessToken();
  watchSession = (listener: (session: MatrixSessionData) => void): void => {
    this.sessionListener = listener;
  };
  profile = async (): Promise<Profile> => profileSchema.parse(await this.call('profile'));
  senderProfile = (sender: string): Promise<SenderProfile> => {
    const cached = this.senderProfiles.get(sender);
    if (cached && Date.now() - cached.fetched < 60_000) return cached.value;
    const value = this.call('senderProfile', sender)
      .then((data) => senderProfileSchema.parse(data))
      .catch((cause: unknown) => {
        if (this.senderProfiles.get(sender)?.value === value) this.senderProfiles.delete(sender);
        throw cause;
      });
    this.senderProfiles.set(sender, { fetched: Date.now(), value });
    return value;
  };
  setDisplayName = (name: string): Promise<void> => this.call('setDisplayName', name);
  setAvatar = (bytes: Uint8Array, mimetype: string): Promise<void> => this.call('setAvatar', bytes, mimetype);
  logout = (): Promise<void> => this.call('logout');
  sync = (since?: string): Promise<SyncUpdate> => this.call('sync', since);
  rooms = async (): Promise<CaseRoom[]> => (await this.call<unknown[]>('rooms')).map((room) => roomSchema.parse(room));
  join = (id: string): Promise<void> => this.call('join', id);
  invite = (id: string, recipient: string): Promise<void> => this.call('invite', id, recipient);
  updateCaseMetadata = (id: string, name: string, topic: string): Promise<void> =>
    this.call('updateCaseMetadata', id, name, topic);
  removeCase = (id: string): Promise<RemoveCaseResult> => this.call('remove', id);
  participants = (id: string): Promise<string[]> => this.call('participants', id);
  history = (id: string, from?: string): Promise<HistoryPage> => this.call('history', id, from);
  event = (id: string, eventId: string): Promise<unknown> => this.call('event', id, eventId);
  create = (input: CaseInput): Promise<string> =>
    this.call('create', input.recipient, input.name, input.topic, input.encrypted);
  send = (id: string, type: string, content: Record<string, unknown>, transaction: string): Promise<string> =>
    this.call('send', id, type, JSON.stringify(content), transaction);
  upload = (id: string, bytes: Uint8Array, mimetype: string): Promise<Record<string, unknown>> =>
    this.call('upload', id, bytes, mimetype);
  download = (descriptor: Record<string, unknown>): Promise<Uint8Array> => this.call('download', descriptor);

  /** Releases the SDK and aborts outstanding network operations by terminating its worker. */
  dispose(cause = new Error('Matrix session ended')): void {
    this.disposed = true;
    this.sessionListener = undefined;
    this.sessionCache.clear();
    this.senderProfiles.clear();
    this.worker.terminate();
    for (const job of this.pending.values()) {
      clearTimeout(job.timer);
      job.reject(cause);
    }
    this.pending.clear();
  }
}
