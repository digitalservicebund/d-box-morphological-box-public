// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { z } from 'zod';
import { keys, types, type BlockKind } from './identifiers';
import type { BlockDraft, MatrixEvent } from './types';
import { canonicalPayload } from './legacy-compatibility';

const nonempty = z.string().trim().min(1, 'This field is required.');
const https = z
  .url()
  .refine((value) => URL.canParse(value) && new URL(value).protocol === 'https:', 'Use an HTTPS URL.');
const representation = z.object({ body: z.string(), mimetype: z.string().optional(), lang: z.string().optional() });
const text = z.array(representation).min(0);
const plainText = text.refine((items) => items.some((item) => !item.mimetype || item.mimetype === 'text/plain'));
const title = z.object({ 'm.text': plainText });
const link = z.object({ [keys.label]: title, url: https });
const emptyLink = z.object({
  [keys.label]: z.object({ 'm.text': z.tuple([z.object({ body: z.literal(''), mimetype: z.literal('text/plain') })]) }),
  url: z.literal(''),
});
const file = z.object({
  uri: z.string().regex(/^mxc:\/\/[^/]+\/.+/),
  name: nonempty,
  mimetype: z.string().optional(),
  size: z.number().int().nonnegative().optional(),
  [keys.encryptedFile]: z
    .object({
      key: z.object({
        kty: z.literal('oct'),
        k: nonempty,
        alg: z.literal('A256CTR'),
        key_ops: z.array(z.enum(['encrypt', 'decrypt'])),
        ext: z.boolean(),
      }),
      iv: nonempty,
      hashes: z.object({ sha256: nonempty }),
      v: z.literal('v2'),
    })
    .optional(),
});
const inquiryText = z.object({ [keys.title]: title.optional(), [keys.label]: title.optional() });
const poll = z
  .object({
    question: z.object({ 'm.text': text }),
    answers: z.array(z.object({ 'm.id': nonempty, 'm.text': plainText })).min(1),
    max_selections: z.number().int().positive().optional(),
    kind: z.enum(['m.disclosed', 'm.undisclosed']).optional(),
  })
  .refine((value) => {
    const ids = value.answers.map((answer) => answer['m.id']);
    return (
      ids.includes(keys.customAnswer) &&
      new Set(ids).size === ids.length &&
      (value.max_selections === undefined || value.max_selections === 1 || value.max_selections === ids.length)
    );
  });

const base = z.looseObject({ 'm.text': text });
const schemaEntries: [string, z.ZodType][] = [
  [types.text, base],
  [types.button, base.extend({ [keys.hyperlink]: link })],
  [
    types.infobox,
    base.extend({
      [keys.infoboxText]: z.object({ 'm.text': text }),
      [keys.title]: title.optional(),
      [keys.hyperlink]: z.union([link, emptyLink]).optional(),
    }),
  ],
  ...[types.file, types.image, types.video, types.audio].map((type): [string, z.ZodType] => [
    type,
    base.extend({ [types.file]: file, [keys.title]: title.optional() }),
  ]),
  [types.textInquiry, base.extend({ [types.textInquiry]: inquiryText.optional() })],
  [
    types.fileInquiry,
    base.extend({
      [types.fileInquiry]: z.object({ [keys.title]: title.optional() }).optional(),
      [types.textInquiry]: inquiryText.optional(),
    }),
  ],
  [types.pollInquiry, base.extend({ [keys.poll]: poll, [types.textInquiry]: inquiryText.optional() })],
  [types.pollResponse, z.looseObject({ [keys.selections]: z.array(z.string()) })],
];
const schemas = new Map(schemaEntries.map(([type, schema]) => [type, z.compile(schema)]));
const textFallback = z.compile(base);

const eventSchema = z.compile(
  z.looseObject({
    event_id: z.string().min(1),
    sender: z.string().min(1),
    origin_server_ts: z.number().nonnegative(),
    type: z.string(),
    content: z.record(z.string(), z.unknown()),
    state_key: z.string().optional(),
    redacts: z.string().optional(),
    unsigned: z.record(z.string(), z.unknown()).optional(),
  }),
);

/** Parses an untrusted SDK event; content validation follows after redaction handling. */
export const parseEvent = (value: unknown): MatrixEvent | undefined => {
  const result = eventSchema.safeParse(value);
  return result.success ? result.data : undefined;
};

export const validContent = (event: Pick<MatrixEvent, 'type' | 'content'>): boolean => {
  const payload = canonicalPayload(event);
  return (schemas.get(payload.type) ?? textFallback).safeParse(payload.content).success;
};

export const recipientSchema = z
  .string()
  .trim()
  .regex(/^@[^:\s]+:[^\s]+$/, 'Enter a complete Matrix ID, e.g. @citizen:example.org.');

export const caseMetadataSchema = z.compile(
  z.object({
    name: nonempty.max(200),
    topic: nonempty.max(500),
  }),
);

export const caseInputSchema = z.compile(
  caseMetadataSchema.extend({
    recipient: recipientSchema,
    encrypted: z.boolean(),
  }),
);

const fields = z.object({
  text: z.string(),
  html: z.string(),
  title: z.string(),
  label: z.string(),
  url: z.string(),
  answers: z.array(z.object({ id: nonempty, text: z.string() })),
  multiple: z.boolean(),
});
const attachment = z.instanceof(File, { error: 'Choose a file.' });
const prompt = fields.extend({ text: nonempty });
const draftSchemas: Record<BlockKind, z.ZodType> = {
  text: prompt,
  textInquiry: prompt,
  fileInquiry: prompt,
  button: fields.extend({ label: nonempty, url: https }),
  infobox: prompt.refine(
    (block) => (!block.url && !block.label) || (https.safeParse(block.url).success && Boolean(block.label.trim())),
    'Enter a link label and a valid HTTPS URL, or leave both empty.',
  ),
  file: fields.extend({ file: attachment }),
  image: fields.extend({
    file: attachment.refine((value) => value.type.startsWith('image/'), 'Choose an image file.'),
  }),
  video: fields.extend({ file: attachment.refine((value) => value.type.startsWith('video/'), 'Choose a video file.') }),
  audio: fields.extend({
    file: attachment.refine((value) => value.type.startsWith('audio/'), 'Choose an audio file.'),
  }),
  pollInquiry: prompt.extend({
    answers: z.array(z.object({ id: nonempty, text: nonempty })).min(1, 'Add at least one option.'),
  }),
};
const compiledDraftSchemas = new Map(Object.entries(draftSchemas).map(([kind, schema]) => [kind, z.compile(schema)]));

/** Validates a composer block and returns actionable errors before network operations. */
export const validateDraft = (block: BlockDraft): string[] => {
  const result = compiledDraftSchemas.get(block.kind)?.safeParse(block);
  if (!result) return ['Unknown block type.'];
  return result.success ? [] : result.error.issues.map((issue) => `${issue.path.join('.')}: ${issue.message}`);
};
