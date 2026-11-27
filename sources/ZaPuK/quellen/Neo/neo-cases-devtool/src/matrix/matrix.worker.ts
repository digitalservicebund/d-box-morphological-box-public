// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import init, { MatrixClient } from './generated/matrix';
import { installRelationTransport } from './relation-transport';
import type { WorkerRequest, WorkerResponse } from './types';

const pendingRelations = installRelationTransport();
let client: MatrixClient | undefined;
let homeserver = '';
const ready = init();

const string = (value: unknown): string => {
  if (typeof value !== 'string') throw new Error('Invalid SDK argument');
  return value;
};
const optional = (value: unknown): string | undefined => (value == null ? undefined : string(value));
const bytes = (value: unknown): Uint8Array => {
  if (!(value instanceof Uint8Array)) throw new Error('Invalid media bytes');
  return value;
};
const json = async (value: Promise<string>): Promise<unknown> => JSON.parse(await value) as unknown;

const releaseRelations = (urls: string[], relations: unknown): void => {
  for (const url of urls) if (pendingRelations.get(url) === relations) pendingRelations.delete(url);
};

const send = async (sdk: MatrixClient, args: unknown[]): Promise<string> => {
  const [roomId, type, content, transaction] = [string(args[0]), string(args[1]), string(args[2]), string(args[3])];
  const parsed = JSON.parse(content) as Record<string, unknown>;
  const path = `/rooms/${encodeURIComponent(roomId)}/send/m.room.encrypted/${encodeURIComponent(transaction)}`;
  const urls = ['v3', 'r0'].map((version) => {
    const url = new URL(`${homeserver}/_matrix/client/${version}${path}`);
    return url.origin + decodeURIComponent(url.pathname);
  });
  for (const url of urls) pendingRelations.set(url, parsed['im.nheko.relations.v1.relations']);
  try {
    return await sdk.send(roomId, type, content, transaction);
  } finally {
    releaseRelations(urls, parsed['im.nheko.relations.v1.relations']);
  }
};

const handlers: Record<string, (sdk: MatrixClient, args: unknown[]) => unknown> = {
  login: async (sdk, args) => {
    const session = await json(sdk.login(string(args[0]), string(args[1])));
    homeserver = sdk.homeserver().replace(/\/$/, '');
    return session;
  },
  homeserver: (sdk) => sdk.homeserver(),
  profile: (sdk) => json(sdk.profile()),
  senderProfile: (sdk, args) => json(sdk.sender_profile(string(args[0]))),
  setDisplayName: (sdk, args) => sdk.set_display_name(string(args[0])),
  setAvatar: (sdk, args) => sdk.set_avatar(bytes(args[0]), string(args[1])),
  session: (sdk) => JSON.parse(sdk.session()) as unknown,
  logout: (sdk) => sdk.logout(),
  rooms: (sdk) => json(sdk.rooms()),
  join: (sdk, args) => sdk.join(string(args[0])),
  invite: (sdk, args) => sdk.invite(string(args[0]), string(args[1])),
  updateCaseMetadata: (sdk, args) => sdk.update_case_metadata(string(args[0]), string(args[1]), string(args[2])),
  remove: (sdk, args) => json(sdk.remove_case(string(args[0]))),
  participants: (sdk, args) => json(sdk.participants(string(args[0]))),
  sync: (sdk, args) => json(sdk.sync(optional(args[0]))),
  history: (sdk, args) => json(sdk.history(string(args[0]), optional(args[1]))),
  event: (sdk, args) => json(sdk.event(string(args[0]), string(args[1]))),
  create: (sdk, args) => sdk.create_case(string(args[0]), string(args[1]), string(args[2]), args[3] === true),
  upload: (sdk, args) => json(sdk.upload(string(args[0]), bytes(args[1]), string(args[2]))),
  download: (sdk, args) => sdk.download(JSON.stringify(args[0])),
  send,
};

const dispatch = async ({ method, args }: WorkerRequest): Promise<unknown> => {
  await ready;
  if (method === 'open') {
    homeserver = string(args[0]).replace(/\/$/, '');
    client = await MatrixClient.open(homeserver, string(args[1]), optional(args[2]));
    homeserver = client.homeserver().replace(/\/$/, '');
    return;
  }
  if (!client) throw new Error('Matrix client is not initialized');
  const handler = Object.hasOwn(handlers, method) ? handlers[method] : undefined;
  if (!handler) throw new Error(`Unknown Matrix operation: ${method}`);
  return handler(client, args);
};

globalThis.addEventListener('message', (event: MessageEvent<WorkerRequest>) => {
  void dispatch(event.data).then(
    (value) => globalThis.postMessage({ id: event.data.id, value } satisfies WorkerResponse),
    (cause: unknown) =>
      globalThis.postMessage({
        id: event.data.id,
        error: cause instanceof Error ? cause.message : String(cause),
      } satisfies WorkerResponse),
  );
});
