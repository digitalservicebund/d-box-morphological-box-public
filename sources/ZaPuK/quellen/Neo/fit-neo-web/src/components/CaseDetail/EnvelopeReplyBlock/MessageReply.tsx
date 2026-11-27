// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import '../CaseDetail.css';
import '../InquiryForm/InquiryForm.css';

export default function MessageReply({
  block,
  envelopeBlocks,
}: {
  block: NeoBuildingBlock;
  envelopeBlocks: NeoBuildingBlock[];
}) {
  const c = block.content;
  if (!NeoContent.InquiryMessage.instanceOf(c)) return null;

  const reply = envelopeBlocks.find(
    (b) =>
      NeoContent.Message.instanceOf(b.content) &&
      b.content.inner.replyTo === block.id,
  );
  const text =
    reply && NeoContent.Message.instanceOf(reply.content)
      ? (reply.content.inner.text.plain ?? '')
      : '';
  return (
    <div className="case-detail-textarea-wrapper">
      <p className="case-detail-envelope-text">{text}</p>
    </div>
  );
}
