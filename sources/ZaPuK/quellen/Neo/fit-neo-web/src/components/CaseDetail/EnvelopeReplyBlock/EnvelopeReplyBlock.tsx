// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import type { LoadMediaContent } from '../../../hooks/useMediaObjectUrl';
import FileReply from './FileReply';
import MessageReply from './MessageReply';
import PollReply from './PollReply';

export default function EnvelopeReplyBlock({
  block,
  envelopeBlocks,
  loadMediaContent,
  onPreviewImage,
}: {
  block: NeoBuildingBlock;
  envelopeBlocks: NeoBuildingBlock[];
  loadMediaContent?: LoadMediaContent;
  onPreviewImage: (image: { url: string; name: string }) => void;
}) {
  const content = block.content;
  if (NeoContent.InquiryMessage.instanceOf(content)) {
    return <MessageReply block={block} envelopeBlocks={envelopeBlocks} />;
  }
  if (NeoContent.InquiryFile.instanceOf(content)) {
    return (
      <FileReply
        block={block}
        envelopeBlocks={envelopeBlocks}
        loadMediaContent={loadMediaContent}
        onPreviewImage={onPreviewImage}
      />
    );
  }
  if (NeoContent.InquiryPoll.instanceOf(content)) {
    return <PollReply block={block} envelopeBlocks={envelopeBlocks} />;
  }
  return null;
}
