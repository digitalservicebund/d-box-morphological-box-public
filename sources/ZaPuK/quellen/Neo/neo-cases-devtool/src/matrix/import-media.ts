// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { keys, types } from '../domain/neo/identifiers';
import { object } from '../domain/neo/relations';
import type { EventPayload } from '../domain/neo/types';
import { sanitizeHtml } from '../utils/rich-text';
import type { MatrixService } from './matrix-service';

export type MediaTransfer = Pick<MatrixService, 'download' | 'upload'>;

const sanitizeRepresentations = (value: unknown): unknown => {
  if (Array.isArray(value)) return value.map(sanitizeRepresentations);
  if (value === null || typeof value !== 'object') return value;
  const result = Object.fromEntries(Object.entries(value).map(([key, entry]) => [key, sanitizeRepresentations(entry)]));
  if (result.mimetype === 'text/html' && typeof result.body === 'string') result.body = sanitizeHtml(result.body);
  return result;
};

const copyFile = async (service: MediaTransfer, roomId: string, value: unknown): Promise<Record<string, unknown>> => {
  const file = object(value);
  if (typeof file.uri !== 'string') throw new Error('An imported attachment has no Matrix media URI.');
  const bytes = await service.download({ url: file.uri, ...object(file[keys.encryptedFile]) });
  const mime = typeof file.mimetype === 'string' ? file.mimetype : 'application/octet-stream';
  const uploaded = await service.upload(roomId, bytes, mime);
  const { url, ...encryption } = uploaded;
  const result: Record<string, unknown> = { ...file, uri: url, size: bytes.byteLength };
  delete result[keys.encryptedFile];
  if (uploaded.key) result[keys.encryptedFile] = encryption;
  return result;
};

/** Reuploads imported media using the destination room's encryption, including thumbnails. */
export const copyTemplateMedia = async (
  service: MediaTransfer,
  roomId: string,
  payload: EventPayload,
): Promise<EventPayload> => {
  const content = object(sanitizeRepresentations(payload.content));
  if (content[types.file]) content[types.file] = await copyFile(service, roomId, content[types.file]);
  const thumbnails = content['dev.fitko.neo.thumbnail.v1'];
  if (Array.isArray(thumbnails)) {
    content['dev.fitko.neo.thumbnail.v1'] = await Promise.all(
      thumbnails.map(async (thumbnail) => {
        const item = object(thumbnail);
        return { ...item, [types.file]: await copyFile(service, roomId, item[types.file]) };
      }),
    );
  }
  return { ...payload, content };
};
