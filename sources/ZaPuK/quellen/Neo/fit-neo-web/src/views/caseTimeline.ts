// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import {
  NeoContent,
  type NeoBuildingBlock,
  type NeoCase,
  type NeoDialog,
} from '@matrix-g2x/neo-web-sdk/web';
import type {
  TimelineEntry,
  TimelineSection,
} from '../components/Timeline/Timeline';
import { formatDateTime } from '../lib/formatDateTime';

type TFunction = (key: string, defaultValue: string) => string;

function getFirstBlockTitle(blocks: NeoBuildingBlock[], t: TFunction): string {
  const first = blocks[0];
  if (!first) return t('timeline.message', 'Nachricht');

  const c = first.content;
  if (NeoContent.Message.instanceOf(c)) {
    const plain = c.inner.text.plain?.trim();
    if (plain) return plain.length > 60 ? `${plain.slice(0, 60)}…` : plain;
    return t('timeline.message', 'Nachricht');
  }
  if (NeoContent.Infobox.instanceOf(c)) {
    return c.inner.title.plain ?? t('timeline.infobox', 'Infobox');
  }
  if (NeoContent.File.instanceOf(c)) {
    return c.inner.title?.plain ?? c.inner.file.name;
  }
  if (NeoContent.Image.instanceOf(c)) {
    return c.inner.title?.plain ?? c.inner.file.name;
  }
  if (NeoContent.Video.instanceOf(c)) {
    return c.inner.title?.plain ?? c.inner.file.name;
  }
  if (NeoContent.Audio.instanceOf(c)) {
    return c.inner.title?.plain ?? c.inner.file.name;
  }
  if (NeoContent.InquiryMessage.instanceOf(c)) {
    return c.inner.title?.plain ?? t('timeline.inquiry', 'Anfrage');
  }
  if (NeoContent.InquiryFile.instanceOf(c)) {
    return c.inner.title?.plain ?? t('timeline.inquiryFile', 'Dateinanfrage');
  }
  if (NeoContent.InquiryPoll.instanceOf(c)) {
    return t('timeline.inquiryPoll', 'Umfrage');
  }
  if (NeoContent.Poll.instanceOf(c)) {
    return t('timeline.pollResponse', 'Umfrageantwort');
  }
  if (NeoContent.Button.instanceOf(c)) {
    return c.inner.hyperlink.label.plain ?? t('timeline.button', 'Link');
  }
  return t('timeline.message', 'Nachricht');
}

function getFirstBlockBody(blocks: NeoBuildingBlock[]): string | undefined {
  const first = blocks[0];
  if (!first) return undefined;

  const c = first.content;
  if (NeoContent.Infobox.instanceOf(c)) return c.inner.infobox.plain;
  return undefined;
}

function getAttachmentBlocks(blocks: NeoBuildingBlock[]): NeoBuildingBlock[] {
  return blocks.filter(
    (b) =>
      NeoContent.File.instanceOf(b.content) ||
      NeoContent.Image.instanceOf(b.content) ||
      NeoContent.Video.instanceOf(b.content) ||
      NeoContent.Audio.instanceOf(b.content),
  );
}

function getFirstAttachmentName(
  blocks: NeoBuildingBlock[],
): string | undefined {
  const first = getAttachmentBlocks(blocks)[0];
  if (!first) return undefined;
  const c = first.content as
    | InstanceType<typeof NeoContent.File>
    | InstanceType<typeof NeoContent.Image>
    | InstanceType<typeof NeoContent.Video>
    | InstanceType<typeof NeoContent.Audio>;
  return c.inner.title?.plain ?? c.inner.file.name;
}

function dialogToEntry(
  dialog: NeoDialog,
  lastReadTimestamp: bigint | null,
  displayNames: Record<string, string>,
  t: TFunction,
  language: string,
): TimelineEntry {
  const rawSender = dialog.message.blocks[0]?.sender;
  const sender = rawSender
    ? (displayNames[rawSender] ?? rawSender.split('@')[0] ?? rawSender)
    : undefined;
  const dateStr = formatDateTime(dialog.message.timestamp, language);
  const title = getFirstBlockTitle(dialog.message.blocks, t);
  const body = getFirstBlockBody(dialog.message.blocks);
  const attachmentBlocks = getAttachmentBlocks(dialog.message.blocks);
  const attachmentName = getFirstAttachmentName(dialog.message.blocks);
  const extraCount =
    attachmentBlocks.length > 1 ? attachmentBlocks.length - 1 : undefined;

  const latestTimestamp =
    dialog.answers.length > 0
      ? dialog.answers.at(-1)!.timestamp
      : dialog.message.timestamp;
  const isNew =
    lastReadTimestamp !== null && latestTimestamp > lastReadTimestamp;

  const latestEnvelope = dialog.answers.at(-1);
  const reply = latestEnvelope
    ? (() => {
        const rawReplySender = latestEnvelope.blocks[0]?.sender ?? '';
        const replySender =
          displayNames[rawReplySender] ??
          rawReplySender.split('@')[0] ??
          rawReplySender;
        return {
          date: formatDateTime(latestEnvelope.timestamp, language),
          sender: replySender,
          title: getFirstBlockTitle(latestEnvelope.blocks, t),
        };
      })()
    : undefined;

  return {
    id: dialog.id,
    date: dateStr,
    sender,
    title,
    body,
    attachmentName,
    extraCount,
    isNew,
    reply,
    status: (() => {
      if (dialog.answers.length > 0) return 'success';
      const hasInquiry = dialog.message.blocks.some(
        (b) =>
          NeoContent.InquiryMessage.instanceOf(b.content) ||
          NeoContent.InquiryFile.instanceOf(b.content) ||
          NeoContent.InquiryPoll.instanceOf(b.content),
      );
      return hasInquiry ? 'warning' : 'default';
    })(),
  };
}

export function neoCaseToSections(
  neoCase: NeoCase,
  lastReadTimestamp: bigint | null,
  displayNames: Record<string, string>,
  t: TFunction,
  language: string,
): TimelineSection[] {
  if (neoCase.dialogs.length === 0) return [];

  const sectionMap = new Map<string, { label: string; dialogs: NeoDialog[] }>();
  for (const dialog of neoCase.dialogs) {
    const date = new Date(Number(dialog.message.timestamp));
    const key = `${date.getFullYear()}-${date.getMonth()}`;
    const label = date.toLocaleDateString(language, {
      month: 'long',
      year: 'numeric',
    });
    if (!sectionMap.has(key)) sectionMap.set(key, { label, dialogs: [] });
    sectionMap.get(key)!.dialogs.push(dialog);
  }

  return Array.from(sectionMap.values())
    .reverse()
    .map(({ label, dialogs }) => ({
      id: label,
      label,
      entries: [...dialogs]
        .reverse()
        .map((dialog) =>
          dialogToEntry(dialog, lastReadTimestamp, displayNames, t, language),
        ),
    }));
}
