// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import {
  type Dispatch,
  type RefObject,
  type SetStateAction,
  useState,
} from 'react';
import { useTranslation } from 'react-i18next';
import Alert from '../../Alert/Alert';
import AlertDiamond from '../../Icons/AlertDiamond';
import FileInfo from '../../Icons/FileInfo';
import Trash from '../../Icons/Trash';
import {
  formatLocalFileSize,
  type InquiryFormState,
} from '../caseDetailHelpers';
import FileThumbnail from '../FileThumbnail';
import { validateSelectedFile } from './fileValidation';
import '../InquiryForm/InquiryForm.css';
import '../InquiryPollField/InquiryPollField.css';
import './InquiryFileField.css';

export default function InquiryFileField({
  block,
  state,
  onChange,
  fileInputRef,
  fileError,
  setFileErrors,
  onPreviewFile,
}: {
  block: NeoBuildingBlock;
  state: InquiryFormState;
  onChange: (next: InquiryFormState) => void;
  fileInputRef: RefObject<HTMLInputElement | null>;
  fileError?: string;
  setFileErrors: Dispatch<SetStateAction<Map<string, string>>>;
  onPreviewFile: (file: File) => void;
}) {
  const { t } = useTranslation();
  const [isConfirmingRemoval, setIsConfirmingRemoval] = useState(false);
  const content = block.content;
  if (!NeoContent.InquiryFile.instanceOf(content)) return null;

  const title =
    content.inner.title?.plain ?? t('caseDetail.inquiryFile', 'Anhang');
  const selectedFile = state.files.get(block.id) ?? null;
  const isDeclaration = state.fileDeclarations.get(block.id) ?? false;
  const comment = state.fileComments.get(block.id) ?? '';

  function removeSelectedFile() {
    if (fileInputRef.current) fileInputRef.current.value = '';
    const files = new Map(state.files);
    files.set(block.id, null);
    onChange({ ...state, files });
    setIsConfirmingRemoval(false);
  }

  return (
    <>
      <Alert
        open={isConfirmingRemoval}
        onClose={() => setIsConfirmingRemoval(false)}
        title={t('caseDetail.confirmRemoveFile', 'Dokument entfernen?')}
        text={t(
          'caseDetail.confirmRemoveFileBody',
          'Möchten Sie das Dokument wirklich entfernen?',
        )}
        secondaryAction={{
          children: t('caseDetail.cancel', 'Abbrechen'),
          onClick: () => setIsConfirmingRemoval(false),
        }}
        primaryAction={{
          children: t('caseDetail.remove', 'Entfernen'),
          onClick: removeSelectedFile,
        }}
      />
      <div
        key={block.id}
        className={`case-detail-file-upload-wrapper${fileError ? ' case-detail-file-upload-wrapper-error' : ''}`}
      >
        <input
          ref={fileInputRef}
          type="file"
          accept=".png,.jpg,.jpeg,.pdf"
          className="case-detail-file-input"
          disabled={isDeclaration}
          aria-hidden="true"
          tabIndex={-1}
          onChange={(e) => {
            const file = e.target.files?.[0] ?? null;
            if (file) {
              const validationError = validateSelectedFile(file);
              if (validationError === 'type') {
                e.target.value = '';
                setFileErrors((prev) =>
                  new Map(prev).set(
                    block.id,
                    t(
                      'caseDetail.fileErrorType',
                      'Ungültiger Dateityp. Nur PNG, JPG und PDF sind erlaubt.',
                    ),
                  ),
                );
                return;
              }
              if (validationError === 'size') {
                e.target.value = '';
                setFileErrors((prev) =>
                  new Map(prev).set(
                    block.id,
                    t(
                      'caseDetail.fileErrorSize',
                      'Die Datei ist zu groß. Maximal 10 MB erlaubt.',
                    ),
                  ),
                );
                return;
              }
            }
            setFileErrors((prev) => {
              const m = new Map(prev);
              m.delete(block.id);
              return m;
            });
            const files = new Map(state.files);
            files.set(block.id, file);
            onChange({ ...state, files });
          }}
        />

        {!isDeclaration && (
          <div className="case-detail-file-upload-form">
            <span className="case-detail-file-upload-title">{title}</span>
            <span className="case-detail-file-upload-hint">
              {t('caseDetail.fileHint', 'PNG, JPG, PDF unter 10 MB')}
            </span>

            <div className="case-detail-file-zone">
              {selectedFile ? (
                <>
                  <div className="case-detail-file-thumb">
                    {selectedFile.type.startsWith('image/') ? (
                      <button
                        type="button"
                        className="case-detail-file-thumb-btn"
                        aria-label={t(
                          'caseDetail.previewImage',
                          'Bild in Vollansicht anzeigen',
                        )}
                        onClick={() => onPreviewFile(selectedFile)}
                      >
                        <FileThumbnail file={selectedFile} size={48} />
                      </button>
                    ) : (
                      <FileThumbnail file={selectedFile} size={48} />
                    )}
                  </div>
                  <div className="case-detail-file-info">
                    <span className="case-detail-file-info-text">
                      {title} | {formatLocalFileSize(selectedFile.size)}
                    </span>
                    <div className="case-detail-file-actions">
                      <button
                        type="button"
                        className="case-detail-file-action-btn case-detail-file-action-btn-info"
                        title={t(
                          'caseDetail.fileHelp',
                          'Hilfe zur Dateiauswahl',
                        )}
                        aria-label={t(
                          'caseDetail.fileHelp',
                          'Hilfe zur Dateiauswahl',
                        )}
                      >
                        <FileInfo />
                      </button>
                      <button
                        type="button"
                        className="case-detail-file-action-btn case-detail-file-action-btn--delete"
                        title={t('caseDetail.removeFile', 'Datei entfernen')}
                        aria-label={t(
                          'caseDetail.removeFile',
                          'Datei entfernen',
                        )}
                        onClick={() => setIsConfirmingRemoval(true)}
                      >
                        <Trash />
                      </button>
                    </div>
                  </div>
                </>
              ) : (
                <>
                  <button
                    type="button"
                    className="case-detail-file-select-btn"
                    onClick={() => fileInputRef.current?.click()}
                  >
                    {t('caseDetail.selectFile', 'Datei auswählen')}
                  </button>
                  <div className="case-detail-file-info">
                    <span className="case-detail-file-upload-status">
                      {t('caseDetail.noSelection', 'Keine Auswahl')}
                    </span>
                    <div className="case-detail-file-actions">
                      <button
                        type="button"
                        className="case-detail-file-action-btn case-detail-file-action-btn-info"
                        title={t(
                          'caseDetail.fileHelp',
                          'Hilfe zur Dateiauswahl',
                        )}
                        aria-label={t(
                          'caseDetail.fileHelp',
                          'Hilfe zur Dateiauswahl',
                        )}
                      >
                        <FileInfo />
                      </button>
                    </div>
                  </div>
                </>
              )}
            </div>
          </div>
        )}
        {fileError && (
          <p className="case-detail-file-error" role="alert">
            <AlertDiamond className="case-detail-file-error-icon" />
            {fileError}
          </p>
        )}

        <label className="case-detail-poll-option" data-type="file-declaration">
          <input
            className="case-detail-poll-input"
            type="checkbox"
            checked={isDeclaration}
            onChange={(e) => {
              const fileDeclarations = new Map(state.fileDeclarations);
              fileDeclarations.set(block.id, e.target.checked);
              const files = new Map(state.files);
              if (e.target.checked) {
                files.set(block.id, null);
                if (fileInputRef.current) fileInputRef.current.value = '';
                setFileErrors((prev) => {
                  const m = new Map(prev);
                  m.delete(block.id);
                  return m;
                });
              }
              onChange({ ...state, files, fileDeclarations });
            }}
          />
          <span className="case-detail-poll-visual" aria-hidden="true" />
          <span className="case-detail-poll-label">
            {t(
              'caseDetail.noFileDeclaration',
              'Ich habe die geforderte Anlage nicht fristgerecht vorliegend.',
            )}
          </span>
        </label>

        {isDeclaration && (
          <div className="case-detail-comment-section">
            <strong className="case-detail-comment-label">
              {t('caseDetail.comment', 'Kommentar:')}
            </strong>
            <div className="case-detail-textarea-wrapper">
              <textarea
                id={`file-comment-${block.id}`}
                className="case-detail-textarea"
                aria-label={
                  content.inner.label?.plain ??
                  t(
                    'caseDetail.textareaPlaceholder',
                    'Geben Sie Ihre Antwort ein...',
                  )
                }
                placeholder={
                  content.inner.label?.plain ??
                  t(
                    'caseDetail.textareaPlaceholder',
                    'Geben Sie Ihre Antwort ein...',
                  )
                }
                value={comment}
                onChange={(e) => {
                  const fileComments = new Map(state.fileComments);
                  fileComments.set(block.id, e.target.value);
                  onChange({ ...state, fileComments });
                }}
              />
            </div>
          </div>
        )}
      </div>
    </>
  );
}
