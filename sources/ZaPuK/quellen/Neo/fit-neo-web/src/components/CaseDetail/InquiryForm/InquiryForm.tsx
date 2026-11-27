// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import { useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import Alert from '../../Alert/Alert';
import type { InquiryFormState } from '../caseDetailHelpers';
import FileImage from '../FileImage';
import InquiryFileField from '../InquiryFileField/InquiryFileField';
import InquiryPollField from '../InquiryPollField/InquiryPollField';
import './InquiryForm.css';

export default function InquiryForm({
  messageBlocks,
  state,
  onChange,
}: {
  messageBlocks: NeoBuildingBlock[];
  state: InquiryFormState;
  onChange: (next: InquiryFormState) => void;
}) {
  const { t } = useTranslation();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [previewFile, setPreviewFile] = useState<File | null>(null);
  const [fileErrors, setFileErrors] = useState<Map<string, string>>(new Map());

  const inquiryBlocks = messageBlocks.filter(
    (b) =>
      NeoContent.InquiryMessage.instanceOf(b.content) ||
      NeoContent.InquiryFile.instanceOf(b.content) ||
      NeoContent.InquiryPoll.instanceOf(b.content),
  );

  if (!inquiryBlocks.length) return null;

  return (
    <>
      {previewFile && (
        <Alert
          open
          onClose={() => setPreviewFile(null)}
          title={previewFile.name}
        >
          <FileImage
            file={previewFile}
            alt={t(
              'caseDetail.imagePreviewAlt',
              'Vollansicht der Datei {{name}}',
              {
                name: previewFile.name,
              },
            )}
            style={{ maxWidth: '100%', maxHeight: '60vh', display: 'block' }}
          />
        </Alert>
      )}
      {inquiryBlocks.map((block) => {
        if (NeoContent.InquiryMessage.instanceOf(block.content)) {
          const placeholder = t(
            'caseDetail.textareaPlaceholder',
            'Geben Sie Ihre Antwort ein...',
          );
          return (
            <div key={block.id} className="case-detail-textarea-wrapper">
              <textarea
                id={`inquiry-${block.id}`}
                className="case-detail-textarea"
                aria-label={placeholder}
                placeholder={placeholder}
                value={state.texts.get(block.id) ?? ''}
                onChange={(e) => {
                  const texts = new Map(state.texts);
                  texts.set(block.id, e.target.value);
                  onChange({ ...state, texts });
                }}
              />
            </div>
          );
        }

        if (NeoContent.InquiryFile.instanceOf(block.content)) {
          return (
            <InquiryFileField
              key={block.id}
              block={block}
              state={state}
              onChange={onChange}
              fileInputRef={fileInputRef}
              fileError={fileErrors.get(block.id)}
              setFileErrors={setFileErrors}
              onPreviewFile={setPreviewFile}
            />
          );
        }

        if (NeoContent.InquiryPoll.instanceOf(block.content)) {
          return (
            <InquiryPollField
              key={block.id}
              block={block}
              state={state}
              onChange={onChange}
            />
          );
        }

        return null;
      })}
    </>
  );
}
