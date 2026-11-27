// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import './action-button-block.css';

export default function ActionButtonBlock({
  block,
}: {
  block: NeoBuildingBlock;
}) {
  const c = block.content;
  if (!NeoContent.Button.instanceOf(c)) return null;
  return (
    <a
      className="action-button"
      href={c.inner.hyperlink.url}
      target="_blank"
      rel="noreferrer"
    >
      {c.inner.hyperlink.label.plain ?? c.inner.hyperlink.url}
    </a>
  );
}
