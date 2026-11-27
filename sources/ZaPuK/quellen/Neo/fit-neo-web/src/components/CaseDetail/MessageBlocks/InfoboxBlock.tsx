// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import { useTranslation } from 'react-i18next';
import type { LoadMediaContent } from '../../../hooks/useMediaObjectUrl';
import { sanitizeMatrixHtml } from '../../../lib/sanitizeMatrixHtml';
import Infobox from '../../Infobox/Infobox';

export default function InfoboxBlock({
  block,
  loadMediaContent,
}: {
  block: NeoBuildingBlock;
  loadMediaContent?: LoadMediaContent;
}) {
  const { t } = useTranslation();
  const c = block.content;
  if (!NeoContent.Infobox.instanceOf(c)) return null;
  return (
    <Infobox
      title={c.inner.title.plain ?? ''}
      body={c.inner.infobox.plain ?? ''}
      loadMediaContent={loadMediaContent}
      bodyHtml={
        c.inner.infobox.html
          ? sanitizeMatrixHtml(c.inner.infobox.html)
          : undefined
      }
      link={
        c.inner.hyperlink.url
          ? {
              href: c.inner.hyperlink.url,
              label:
                c.inner.hyperlink.label.plain ??
                t('caseDetail.learnMore', 'Mehr erfahren'),
            }
          : undefined
      }
    />
  );
}
