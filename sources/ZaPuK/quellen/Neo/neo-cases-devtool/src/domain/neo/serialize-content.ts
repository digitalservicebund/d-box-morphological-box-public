// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { blockLabels, keys, types, type BlockKind } from './identifiers';
import type { BlockDraft } from './types';

export type Content = Record<string, unknown>;
const text = (body: string, html?: string): Content[] => [
  ...(html ? [{ body: html, mimetype: 'text/html' }] : []),
  { body, mimetype: 'text/plain' },
];
const title = (body: string): Content => ({ 'm.text': text(body) });
const optionalTitle = (value: string): Content => (value ? { [keys.title]: title(value) } : {});
const hyperlink = (block: BlockDraft): Content => ({ [keys.label]: title(block.label), url: block.url });
const fallback = (block: BlockDraft): Content => ({ 'm.text': text(block.text, block.html) });
const responseSettings = (block: BlockDraft): Content => ({
  ...optionalTitle(block.title),
  ...(block.label ? { [keys.label]: title(block.label) } : {}),
});
const optionalTextResponse = (block: BlockDraft): Content =>
  block.label ? { [types.textInquiry]: { [keys.label]: title(block.label) } } : {};

const pollContent = (block: BlockDraft): Content => {
  const answers = [
    ...block.answers.map((answer) => ({ 'm.id': answer.id, 'm.text': text(answer.text) })),
    { 'm.id': keys.customAnswer, 'm.text': text('Other answer') },
  ];
  return {
    [keys.poll]: {
      question: { 'm.text': text(block.text) },
      answers,
      max_selections: block.multiple ? answers.length : 1,
    },
    'm.text': text(`${block.text}\n${[...block.answers.map((answer) => answer.text), 'Other answer'].join('\n')}`),
    ...optionalTextResponse(block),
  };
};

const mediaContent = (block: BlockDraft, media?: Content): Content => {
  if (!media) throw new Error('Upload the attachment before sending.');
  const { url, ...encryption } = media;
  const mimetype = block.file?.type || 'application/octet-stream';
  const extensions: Record<string, string> = {
    'application/pdf': 'pdf',
    'image/png': 'png',
    'image/jpeg': 'jpg',
    'image/webp': 'webp',
    'image/gif': 'gif',
    'video/mp4': 'mp4',
    'audio/mpeg': 'mp3',
    'audio/ogg': 'ogg',
    'text/plain': 'txt',
  };
  const name = `attachment.${extensions[mimetype] ?? 'bin'}`;
  return {
    ...optionalTitle(block.title),
    [types.file]: {
      uri: url,
      name,
      mimetype,
      size: block.file?.size,
      ...(media.key ? { [keys.encryptedFile]: encryption } : {}),
    },
    'm.text': text(`${block.title || blockLabels[block.kind]}: ${name}`),
    ...(block.kind === 'image' && block.text ? { 'm.alt_text': { 'm.text': text(block.text) } } : {}),
  };
};

const serializers: Record<BlockKind, (block: BlockDraft, media?: Content) => Content> = {
  text: fallback,
  button: (block) => ({ [keys.hyperlink]: hyperlink(block), 'm.text': text(`${block.label}: ${block.url}`) }),
  infobox: (block) => ({
    ...optionalTitle(block.title),
    [keys.infoboxText]: fallback(block),
    ...(block.url ? { [keys.hyperlink]: hyperlink(block) } : {}),
    'm.text': text(
      [block.title, block.text, block.url ? `${block.label}: ${block.url}` : ''].filter(Boolean).join('\n\n'),
    ),
  }),
  textInquiry: (block) => ({ ...fallback(block), [types.textInquiry]: responseSettings(block) }),
  fileInquiry: (block) => ({
    ...fallback(block),
    [types.fileInquiry]: optionalTitle(block.title),
    ...optionalTextResponse(block),
  }),
  pollInquiry: pollContent,
  file: mediaContent,
  image: mediaContent,
  video: mediaContent,
  audio: mediaContent,
};

export const serializeContent = (block: BlockDraft, media?: Content): Content => serializers[block.kind](block, media);
