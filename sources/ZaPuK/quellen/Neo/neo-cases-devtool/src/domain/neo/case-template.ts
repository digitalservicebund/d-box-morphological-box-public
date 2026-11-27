// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { z } from 'zod';
import { isInquiry, keys, types, blockKinds, legacyTypes } from './identifiers';
import type { Dialog, EventPayload } from './types';
import { validContent } from './validation';

const relationKeys = new Set<string>(['m.relates_to', keys.relations, keys.length]);
const agencyTypes = new Set<string>([...blockKinds.map((kind) => types[kind]), ...Object.values(legacyTypes)]);

/** Removes event-ID relationships; the importer rebuilds them from block order. */
const templatePayload = (payload: EventPayload): EventPayload => ({
  type: payload.type,
  content: Object.fromEntries(Object.entries(payload.content).filter(([key]) => !relationKeys.has(key))),
});

const blockSchema = z
  .object({ type: z.string(), content: z.record(z.string(), z.unknown()) })
  .superRefine((block, ctx) => {
    if (!agencyTypes.has(block.type))
      ctx.addIssue({ code: 'custom', message: `Unsupported agency block type: ${block.type}` });
    if (!validContent(block)) ctx.addIssue({ code: 'custom', message: `Invalid content for ${block.type}` });
    if (new TextEncoder().encode(JSON.stringify(block.content)).length > 40_000) {
      ctx.addIssue({ code: 'custom', message: 'A building block exceeds the Matrix content size limit.' });
    }
  });
const dialogSchema = z
  .object({ blocks: z.array(blockSchema).min(1) })
  .refine(
    (dialog) => dialog.blocks.filter((block) => isInquiry(block.type)).length <= 1,
    'Each dialog may contain at most one inquiry.',
  );
const schema = z.compile(
  z.object({
    format: z.literal('neo-agency-case'),
    version: z.literal(1),
    case: z.object({ name: z.string().min(1).max(200), topic: z.string().max(500), encrypted: z.boolean() }),
    dialogs: z.array(dialogSchema),
  }),
);

export type CaseTemplate = z.infer<typeof schema>;

/** Validates an editable export file before creating any Matrix room or event. */
export const parseCaseTemplate = (value: unknown): CaseTemplate => {
  const result = schema.parse(value);
  return { ...result, dialogs: result.dialogs.map((dialog) => ({ blocks: dialog.blocks.map(templatePayload) })) };
};

/** Exports agency envelopes only, oldest first so replay preserves dialog order. */
export const exportCaseTemplate = (metadata: CaseTemplate['case'], dialogs: readonly Dialog[]): CaseTemplate =>
  parseCaseTemplate({
    format: 'neo-agency-case',
    version: 1,
    case: metadata,
    dialogs: [...dialogs]
      .sort((a, b) => a.timestamp - b.timestamp || b.id.localeCompare(a.id))
      .map((dialog) => ({ blocks: dialog.agency.blocks.map(templatePayload) })),
  });
