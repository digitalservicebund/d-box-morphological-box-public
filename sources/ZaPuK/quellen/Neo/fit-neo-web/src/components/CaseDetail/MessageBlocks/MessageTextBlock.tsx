// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import { useMatrixHtmlMedia } from '../../../hooks/useMatrixHtmlMedia';
import type { LoadMediaContent } from '../../../hooks/useMediaObjectUrl';
import { sanitizeMatrixHtml } from '../../../lib/sanitizeMatrixHtml';

export default function MessageTextBlock({
  block,
  loadMediaContent,
}: {
  block: NeoBuildingBlock;
  loadMediaContent?: LoadMediaContent;
}) {
  const c = block.content;
  const html = NeoContent.Message.instanceOf(c) ? c.inner.text.html : undefined;
  const sanitizedHtml = html ? sanitizeMatrixHtml(html) : undefined;
  const ref = useMatrixHtmlMedia(sanitizedHtml, loadMediaContent);
  if (!NeoContent.Message.instanceOf(c)) return null;
  const plain = c.inner.text.plain ?? '';
  if (sanitizedHtml) {
    return (
      <p
        ref={ref}
        className="case-detail-text"
        dangerouslySetInnerHTML={{ __html: sanitizedHtml }}
      />
    );
  }
  return <p className="case-detail-text">{plain}</p>;
}
