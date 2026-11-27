// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useTranslation } from 'react-i18next';
import Alert from '../Alert/Alert';
import type { ModalState } from './caseDetailHelpers';
import FileThumbnail from './FileThumbnail';
import './InquiryFileField/InquiryFileField.css';

export default function CaseDetailModals({
  modalState,
  submitting,
  onClose,
  onSubmit,
}: {
  modalState: ModalState;
  submitting: boolean;
  onClose: () => void;
  onSubmit: () => void;
}) {
  const { t } = useTranslation();

  const submitLabel = submitting
    ? t('caseDetail.sending', 'Wird gesendet...')
    : t('caseDetail.submit', 'Antwort senden');

  if (modalState.kind === 'confirm-file') {
    return (
      <Alert
        open
        onClose={onClose}
        title={t('caseDetail.confirmSendFile', 'Anlage senden?')}
        text={t(
          'caseDetail.confirmFileBody',
          'Der Inhalt Ihrer Anlage ist verbindlich. Bitte kontrollieren Sie noch einmal, ob Sie die Anlage senden möchten.',
        )}
        primaryAction={{
          children: submitLabel,
          disabled: submitting,
          onClick: onSubmit,
        }}
        secondaryAction={{
          children: t('caseDetail.checkDetails', 'Angaben prüfen'),
          onClick: onClose,
        }}
      >
        <div className="case-detail-modal-preview">
          <FileThumbnail file={modalState.file} />
        </div>
      </Alert>
    );
  }

  if (modalState.kind === 'confirm-declaration') {
    return (
      <Alert
        open
        onClose={onClose}
        title={t('caseDetail.confirmNoFile', 'Anlage nicht vorliegend?')}
        text={t(
          'caseDetail.confirmNoFileBody',
          'Die Aussage ist verbindlich und unwiderruflich. Ein nachträgliches Übermitteln der Anlage ist nicht möglich. Bitte kontrollieren Sie noch einmal, ob Sie die Aussage so treffen möchten.',
        )}
        primaryAction={{
          children: submitLabel,
          disabled: submitting,
          onClick: onSubmit,
        }}
        secondaryAction={{
          children: t('caseDetail.checkDetails', 'Angaben prüfen'),
          onClick: onClose,
        }}
      >
        <div className="case-detail-modal-declaration">
          <div className="case-detail-modal-declaration-answer">
            <strong>{t('caseDetail.answer', 'Antwort:')} </strong>
            {modalState.text}
          </div>
          {modalState.comment && (
            <div className="case-detail-modal-declaration-comment">
              <strong>{t('caseDetail.comment', 'Kommentar:')} </strong>
              {modalState.comment}
            </div>
          )}
        </div>
      </Alert>
    );
  }

  if (
    modalState.kind === 'confirm-answer' ||
    modalState.kind === 'confirm-amendment'
  ) {
    const previews =
      modalState.kind === 'confirm-amendment'
        ? [{ answer: modalState.text }]
        : modalState.previews;
    return (
      <Alert
        open
        onClose={onClose}
        title={t('caseDetail.confirmAnswer', 'Antwort senden?')}
        text={t(
          'caseDetail.confirmAnswerBody',
          'Ihre Angaben sind verbindlich. Bitte kontrollieren Sie noch einmal, ob Sie die Antwort senden möchten.',
        )}
        primaryAction={{
          children: submitLabel,
          disabled: submitting,
          onClick: onSubmit,
        }}
        secondaryAction={{
          children: t('caseDetail.checkDetails', 'Angaben prüfen'),
          onClick: onClose,
        }}
      >
        <div className="case-detail-modal-declaration">
          {previews.map((preview) => (
            <div
              key={preview.answer}
              className="case-detail-modal-declaration-group"
            >
              <div className="case-detail-modal-declaration-answer">
                <strong>{t('caseDetail.answer', 'Antwort:')} </strong>
                {preview.answer}
              </div>
              {preview.comment && (
                <div className="case-detail-modal-declaration-comment">
                  <strong>{t('caseDetail.comment', 'Kommentar:')} </strong>
                  {preview.comment}
                </div>
              )}
            </div>
          ))}
        </div>
      </Alert>
    );
  }

  if (modalState.kind === 'success') {
    return (
      <Alert
        open
        onClose={onClose}
        title={t('caseDetail.thankYou', 'Danke!')}
        text={t(
          'caseDetail.sentSuccess',
          'Ihre Antwort wurde erfolgreich versendet.',
        )}
        primaryAction={{
          children: t('caseDetail.done', 'Fertig'),
          onClick: onClose,
        }}
      />
    );
  }

  return null;
}
