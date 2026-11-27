// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type {
  NeoBuildingBlock,
  NeoEnvelope,
} from '@matrix-g2x/neo-web-sdk/web';
import { NeoContent } from '@matrix-g2x/neo-web-sdk/web';
import { Fragment, useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { LoadMediaContent } from '../../../hooks/useMediaObjectUrl';
import { formatDateTime } from '../../../lib/formatDateTime';
import Alert from '../../Alert/Alert';
import Badge from '../../Badge/Badge';
import EnvelopeReplyBlock from '../EnvelopeReplyBlock/EnvelopeReplyBlock';
import MessageTextBlock from '../MessageBlocks/MessageTextBlock';
import './EnvelopeView.css';

export default function EnvelopeView({
  envelopes,
  inquiryBlocks,
  loadMediaContent,
}: {
  envelopes: NeoEnvelope[];
  inquiryBlocks: NeoBuildingBlock[];
  loadMediaContent?: LoadMediaContent;
}) {
  const { t, i18n } = useTranslation();
  const [previewImage, setPreviewImage] = useState<{
    url: string;
    name: string;
  } | null>(null);

  const firstEnvelope = envelopes[0];
  const date = firstEnvelope
    ? formatDateTime(
        firstEnvelope.timestamp,
        i18n.resolvedLanguage ?? i18n.language,
      )
    : '';
  const envelopeBlocks = envelopes.flatMap((env) => env.blocks);

  return (
    <>
      {previewImage && (
        <Alert
          open
          onClose={() => setPreviewImage(null)}
          title={t('caseDetail.imagePreview', 'Vorschau')}
        >
          <img
            src={previewImage.url}
            alt={t(
              'caseDetail.imagePreviewAlt',
              'Vollansicht der Datei {{name}}',
              {
                name: previewImage.name,
              },
            )}
            style={{ maxWidth: '100%', maxHeight: '60vh', display: 'block' }}
          />
        </Alert>
      )}
      <section className="case-detail-section case-detail-envelope-section">
        {/* Date + heading + badge row */}
        <div className="case-detail-envelope-row">
          <div className="case-detail-envelope-row-left">
            <span className="case-detail-date">{date}</span>
            <h3 className="case-detail-envelope-title">
              {t('caseDetail.yourAnswer', 'Ihre Antwort')}
            </h3>
          </div>
          <Badge
            type="success"
            title={t('caseDetail.answeredOnTime', 'Fristgerecht geantwortet')}
          />
        </div>

        {/* Readonly inquiry blocks matched to envelope blocks */}
        {inquiryBlocks.map((block) => (
          <EnvelopeReplyBlock
            key={block.id}
            block={block}
            envelopeBlocks={envelopeBlocks}
            loadMediaContent={loadMediaContent}
            onPreviewImage={setPreviewImage}
          />
        ))}
      </section>
      {envelopes.map((envelope) => {
        const amendments = envelope.blocks.filter(
          (block) =>
            NeoContent.Message.instanceOf(block.content) &&
            !block.content.inner.replyTo,
        );
        if (!amendments.length) return null;

        return (
          <Fragment key={envelope.id}>
            <hr className="case-detail-divider" />
            <section className="case-detail-section case-detail-amendment">
              <div className="case-detail-amendment-header">
                <span className="case-detail-date">
                  {formatDateTime(
                    envelope.timestamp,
                    i18n.resolvedLanguage ?? i18n.language,
                  )}
                </span>
                <h3 className="case-detail-amendment-heading">
                  {t('caseDetail.amendmentTitle', 'Ihre Ergänzung')}
                </h3>
              </div>
              {amendments.map((block) => (
                <MessageTextBlock
                  key={block.id}
                  block={block}
                  loadMediaContent={loadMediaContent}
                />
              ))}
            </section>
          </Fragment>
        );
      })}
    </>
  );
}
