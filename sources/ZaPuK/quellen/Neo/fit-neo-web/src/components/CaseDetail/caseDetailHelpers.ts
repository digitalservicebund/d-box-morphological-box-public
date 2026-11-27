// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import { NeoContent } from '@matrix-g2x/neo-web-sdk/web';

// ─── Constants ────────────────────────────────────────────────────────────────

export const POLL_CUSTOM_OPTION_ID = 'dev.fitko.neo.poll.custom.v1';

// ─── Shared types ─────────────────────────────────────────────────────────────

export interface InquiryFormState {
  texts: Map<string, string>;
  polls: Map<string, string[]>;
  pollCustom: Map<string, string>;
  files: Map<string, File | null>;
  fileDeclarations: Map<string, boolean>;
  fileComments: Map<string, string>;
}

export type ModalState =
  | { kind: 'idle' }
  | { kind: 'confirm-file'; file: File; blockId: string }
  | {
      kind: 'confirm-declaration';
      blockId: string;
      text: string;
      comment: string;
    }
  | { kind: 'confirm-answer'; previews: { answer: string; comment?: string }[] }
  | { kind: 'confirm-amendment'; text: string }
  | { kind: 'success' };

// ─── Pure helpers ─────────────────────────────────────────────────────────────

export function getSenderInitials(name: string): string {
  const parts = name.split(/[\s._-]/).filter(Boolean);
  if (!parts.length) return '?';
  if (parts.length === 1) return parts[0]![0]!.toUpperCase();
  return (
    parts[0]![0]!.toUpperCase() + parts[parts.length - 1]![0]!.toUpperCase()
  );
}

export function formatSenderName(sender: string): string {
  return sender.includes('@') ? sender.split('@')[0]! : sender;
}

export function formatFileSize(bytes: bigint): string {
  const n = Number(bytes);
  if (n < 1024) return `${n} B`;
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KiB`;
  if (n < 1024 * 1024 * 1024) return `${(n / 1024 / 1024).toFixed(1)} MiB`;
  return `${(n / 1024 / 1024 / 1024).toFixed(1)} GiB`;
}

export function formatLocalFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024)
    return `${(bytes / 1024 / 1024).toFixed(1)} MiB`;
  return `${(bytes / 1024 / 1024 / 1024).toFixed(1)} GiB`;
}

export function extractAttachmentData(c: NeoBuildingBlock['content']): {
  name: string;
  size: bigint | undefined;
  uri: string;
  mimeType?: string;
} | null {
  if (NeoContent.File.instanceOf(c))
    return {
      name: c.inner.title?.plain ?? c.inner.file.name,
      size: c.inner.file.size,
      uri: c.inner.file.uri,
      mimeType: c.inner.file.mimetype,
    };
  if (NeoContent.Image.instanceOf(c))
    return {
      name: c.inner.title?.plain ?? c.inner.file.name,
      size: c.inner.file.size,
      uri: c.inner.file.uri,
      mimeType: c.inner.file.mimetype,
    };
  if (NeoContent.Video.instanceOf(c))
    return {
      name: c.inner.title?.plain ?? c.inner.file.name,
      size: c.inner.file.size,
      uri: c.inner.file.uri,
      mimeType: c.inner.file.mimetype,
    };
  if (NeoContent.Audio.instanceOf(c))
    return {
      name: c.inner.title?.plain ?? c.inner.file.name,
      size: c.inner.file.size,
      uri: c.inner.file.uri,
      mimeType: c.inner.file.mimetype,
    };
  return null;
}
