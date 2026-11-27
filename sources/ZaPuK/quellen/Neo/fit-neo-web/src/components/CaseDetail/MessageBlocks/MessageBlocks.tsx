// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import type { LoadMediaContent } from '../../../hooks/useMediaObjectUrl';
import ActionButtonBlock from './ActionButtonBlock';
import InfoboxBlock from './InfoboxBlock';
import MessageTextBlock from './MessageTextBlock';

export default function MessageBlocks({
  blocks,
  loadMediaContent,
}: {
  blocks: NeoBuildingBlock[];
  loadMediaContent?: LoadMediaContent;
}) {
  return (
    <>
      {blocks.map((block) => {
        if (NeoContent.Message.instanceOf(block.content)) {
          return (
            <MessageTextBlock
              key={block.id}
              block={block}
              loadMediaContent={loadMediaContent}
            />
          );
        }
        if (NeoContent.Infobox.instanceOf(block.content)) {
          return (
            <InfoboxBlock
              key={block.id}
              block={block}
              loadMediaContent={loadMediaContent}
            />
          );
        }
        if (NeoContent.Button.instanceOf(block.content)) {
          return <ActionButtonBlock key={block.id} block={block} />;
        }
        return null;
      })}
    </>
  );
}
