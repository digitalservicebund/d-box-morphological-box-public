// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { LoadMediaContent } from '../../hooks/useMediaObjectUrl';
import { formatFileSize } from '../CaseDetail/caseDetailHelpers';
import Attach from '../Icons/Attach';
import './attachments-section.css';

export interface AttachmentItem {
  id: string;
  name: string;
  size: bigint | undefined;
  uri: string;
  mimeType?: string;
}

function AttachmentCard({
  attachment,
  loadMediaContent,
}: {
  attachment: AttachmentItem;
  loadMediaContent?: LoadMediaContent;
}) {
  const { t } = useTranslation();
  const [downloading, setDownloading] = useState(false);

  const { name, size, uri, mimeType } = attachment;

  const handleDownload = useCallback(async () => {
    if (!loadMediaContent || downloading) return;
    setDownloading(true);
    try {
      const bytes = await loadMediaContent(uri);
      const blob = new Blob([bytes], {
        type: mimeType ?? 'application/octet-stream',
      });
      const blobUrl = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = blobUrl;
      a.download = name;
      document.body.appendChild(a);
      a.click();
      a.remove();
      window.setTimeout(() => URL.revokeObjectURL(blobUrl), 60_000);
    } catch (err) {
      console.error('Download failed:', err);
    } finally {
      setDownloading(false);
    }
  }, [loadMediaContent, uri, mimeType, name, downloading]);

  const subtitle = size != null ? formatFileSize(size) : undefined;

  const inner = (
    <>
      <div className="document-thumbnail" aria-hidden="true">
        <Attach />
      </div>
      <div className="document-text">
        <span className="document-name">{name}</span>
        {subtitle && <span className="document-subtitle">{subtitle}</span>}
      </div>
    </>
  );

  if (loadMediaContent) {
    return (
      <button
        className="document"
        type="button"
        title={t('caseDetail.download', 'Herunterladen')}
        aria-busy={downloading}
        disabled={downloading}
        onClick={() => {
          void handleDownload();
        }}
      >
        {inner}
      </button>
    );
  }

  return <div className="document">{inner}</div>;
}

export default function AttachmentsSection({
  attachments,
  loadMediaContent,
}: {
  attachments: AttachmentItem[];
  loadMediaContent?: LoadMediaContent;
}) {
  const { t } = useTranslation();

  if (attachments.length === 0) return null;

  return (
    <div className="attachments-section">
      <strong className="attachments-title">
        {t('caseDetail.attachments', 'Anhänge')}
      </strong>
      <div className="attachments-list">
        {attachments.map((attachment) => (
          <AttachmentCard
            key={attachment.id}
            attachment={attachment}
            loadMediaContent={loadMediaContent}
          />
        ))}
      </div>
    </div>
  );
}
