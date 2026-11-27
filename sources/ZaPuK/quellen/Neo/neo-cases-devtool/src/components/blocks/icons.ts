// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { IconType } from 'react-icons';
import {
  MdTextFields,
  MdLink,
  MdInfoOutline,
  MdAttachFile,
  MdImage,
  MdVideocam,
  MdAudiotrack,
  MdChatBubbleOutline,
  MdUploadFile,
  MdPoll,
} from 'react-icons/md';
import type { BlockKind } from '../../domain/neo/identifiers';

export const blockIcons: Record<BlockKind, IconType> = {
  text: MdTextFields,
  button: MdLink,
  infobox: MdInfoOutline,
  file: MdAttachFile,
  image: MdImage,
  video: MdVideocam,
  audio: MdAudiotrack,
  textInquiry: MdChatBubbleOutline,
  fileInquiry: MdUploadFile,
  pollInquiry: MdPoll,
};
