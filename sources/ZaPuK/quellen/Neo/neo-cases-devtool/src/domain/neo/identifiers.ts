// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

export const types = {
  text: 'org.matrix.msc1767.message',
  button: 'dev.fitko.neo.button.v1',
  infobox: 'dev.fitko.neo.infobox.v1',
  file: 'dev.fitko.neo.file.v1',
  image: 'dev.fitko.neo.image.v1',
  video: 'dev.fitko.neo.video.v1',
  audio: 'dev.fitko.neo.audio.v1',
  textInquiry: 'dev.fitko.neo.inquiry.message.v1',
  fileInquiry: 'dev.fitko.neo.inquiry.file.v1',
  pollInquiry: 'dev.fitko.neo.inquiry.poll.v1',
  pollResponse: 'dev.fitko.neo.poll.response.v1',
} as const;

export const keys = {
  relations: 'im.nheko.relations.v1.relations',
  chain: 'dev.fitko.neo.relationships.chain.v1',
  length: 'dev.fitko.neo.multi.length.v1',
  pollRelation: 'dev.fitko.neo.relationships.poll.v1',
  customRelation: 'dev.fitko.neo.relationships.poll.custom.v1',
  customAnswer: 'dev.fitko.neo.poll.custom.v1',
  poll: 'dev.fitko.neo.poll.v1',
  selections: 'dev.fitko.neo.selections.v1',
  title: 'dev.fitko.neo.title.v1',
  label: 'dev.fitko.neo.label.v1',
  hyperlink: 'dev.fitko.neo.hyperlink.v1',
  infoboxText: 'dev.fitko.neo.infobox.text.v1',
  encryptedFile: 'dev.fitko.neo.file.encrypted.v1',
} as const;

export type BlockKind = Exclude<keyof typeof types, 'pollResponse'>;
export const blockLabels: Record<BlockKind, string> = {
  text: 'Text',
  button: 'Link button',
  infobox: 'Infobox',
  file: 'File',
  image: 'Image',
  video: 'Video',
  audio: 'Audio',
  textInquiry: 'Text inquiry',
  fileInquiry: 'File inquiry',
  pollInquiry: 'Poll inquiry',
};
export const blockKinds = Object.keys(blockLabels) as BlockKind[];
export const legacyTypes = {
  textInquiry: 'dev.fitko.neo.inquiry.org.matrix.msc1767.message.v1',
  fileInquiry: 'dev.fitko.neo.inquiry.dev.fitko.neo.file.v1.v1',
  pollInquiry: 'dev.fitko.neo.inquiry.dev.fitko.neo.poll.response.v1.v1',
} as const;
const canonicalTypes = new Map<string, string>(
  Object.entries(legacyTypes).map(([kind, type]) => [type, types[kind as keyof typeof legacyTypes]]),
);
export const canonicalType = (type: string): string => canonicalTypes.get(type) ?? type;
export const isInquiry = (type: string): boolean =>
  [types.textInquiry, types.fileInquiry, types.pollInquiry].some((value) => value === canonicalType(type));
