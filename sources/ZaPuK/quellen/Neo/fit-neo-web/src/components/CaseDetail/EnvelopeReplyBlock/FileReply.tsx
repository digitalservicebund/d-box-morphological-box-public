// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import { useTranslation } from 'react-i18next';
import {
  useMediaObjectUrl,
  type LoadMediaContent,
} from '../../../hooks/useMediaObjectUrl';
import Attach from '../../Icons/Attach';
import '../CaseDetail.css';
import { formatFileSize } from '../caseDetailHelpers';
import '../InquiryFileField/InquiryFileField.css';

export default function FileReply({
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
  const { t } = useTranslation();
  const c = block.content;
  const fileReply = envelopeBlocks.find(
    (b) =>
      NeoContent.File.instanceOf(b.content) &&
      b.content.inner.replyTo === block.id,
  );
  const msgReply = envelopeBlocks.find(
    (b) =>
      NeoContent.Message.instanceOf(b.content) &&
      b.content.inner.replyTo === block.id,
  );

  const fileContent =
    fileReply && NeoContent.File.instanceOf(fileReply.content)
      ? fileReply.content.inner
      : undefined;
  const isImage = fileContent?.file.mimetype?.startsWith('image/');
  const imageUrl = useMediaObjectUrl(
    isImage ? fileContent?.file.uri : undefined,
    fileContent?.file.mimetype,
    loadMediaContent,
  );
  const messageText =
    msgReply && NeoContent.Message.instanceOf(msgReply.content)
      ? (msgReply.content.inner.text.plain ?? '')
      : '';
  const commentSeparator = messageText.indexOf('\n\n');
  const declarationText =
    commentSeparator >= 0
      ? messageText.slice(0, commentSeparator)
      : messageText;
  const declarationComment =
    commentSeparator >= 0 ? messageText.slice(commentSeparator + 2).trim() : '';

  if (!NeoContent.InquiryFile.instanceOf(c)) return null;

  const title = c.inner.title?.plain ?? t('caseDetail.inquiryFile', 'Anhang');

  if (!fileContent) {
    return (
      <div className="case-detail-file-upload-wrapper">
        {messageText && (
          <>
            <p className="case-detail-text">{declarationText}</p>
            {declarationComment && (
              <div className="case-detail-comment-section">
                <strong className="case-detail-comment-label">
                  {t('caseDetail.comment', 'Kommentar:')}
                </strong>
                <p className="case-detail-text">{declarationComment}</p>
              </div>
            )}
          </>
        )}
      </div>
    );
  }

  return (
    <div className="case-detail-file-upload-wrapper">
      <div className="case-detail-file-upload-form">
        <span className="case-detail-file-upload-title">{title}</span>
        <div className="case-detail-file-zone case-detail-file-zone-readonly">
          <div className="case-detail-file-thumb">
            {isImage && imageUrl ? (
              <button
                type="button"
                className="case-detail-file-thumb-btn"
                aria-label={t(
                  'caseDetail.previewImage',
                  'Bild in Vollansicht anzeigen',
                )}
                onClick={() =>
                  onPreviewImage({
                    url: imageUrl,
                    name: fileContent.file.name,
                  })
                }
              >
                <img
                  className="case-detail-file-thumb-img"
                  src={imageUrl}
                  alt={t(
                    'caseDetail.fileThumbnailAlt',
                    'Vorschaubild der Datei {{name}}',
                    { name: fileContent.file.name },
                  )}
                />
              </button>
            ) : (
              <div className="case-detail-file-thumb-icon" aria-hidden="true">
                <Attach />
              </div>
            )}
          </div>
          <div className="case-detail-file-info">
            <span className="case-detail-file-info-text">
              {title}
              {fileContent.file.size != null &&
                ` | ${formatFileSize(fileContent.file.size)}`}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}
