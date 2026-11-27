// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useTranslation } from 'react-i18next';
import Attach from '../Icons/Attach';
import FileImage from './FileImage';
import './InquiryFileField/InquiryFileField.css';

export default function FileThumbnail({
  file,
  size,
}: {
  file: File;
  size?: number;
}) {
  const { t } = useTranslation();
  const isImage = file.type.startsWith('image/');
  const inlineSize = size != null ? { width: size, height: size } : undefined;

  if (isImage) {
    return (
      <FileImage
        file={file}
        className="case-detail-file-thumb-img"
        alt={t(
          'caseDetail.fileThumbnailAlt',
          'Vorschaubild der Datei {{name}}',
          {
            name: file.name,
          },
        )}
        style={inlineSize}
      />
    );
  }
  return (
    <div
      className="case-detail-file-thumb-icon"
      style={inlineSize}
      aria-hidden="true"
    >
      <Attach />
    </div>
  );
}
