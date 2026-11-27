// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoDialog } from '@matrix-g2x/neo-web-sdk/web';
import { useTranslation } from 'react-i18next';
import type { LoadMediaContent } from '../../hooks/useMediaObjectUrl';
import { formatDateTime } from '../../lib/formatDateTime';
import AttachmentsSection, {
  type AttachmentItem,
} from '../AttachmentsSection/AttachmentsSection';
import AmendmentSection from './AmendmentSection/AmendmentSection';
import './CaseDetail.css';
import {
  extractAttachmentData,
  formatSenderName,
  getSenderInitials,
} from './caseDetailHelpers';
import CaseDetailModals from './CaseDetailModals';
import ComposeEnvelope from './ComposeEnvelope/ComposeEnvelope';
import EnvelopeView from './EnvelopeView/EnvelopeView';
import MessageBlocks from './MessageBlocks/MessageBlocks';
import PersonHeader from './PersonHeader/PersonHeader';
import { useCaseDetailAnswer } from './useCaseDetailAnswer';

// ─── Main component ───────────────────────────────────────────────────────────

export interface CaseDetailProps {
  dialog: NeoDialog;
  displayNames?: Record<string, string>;
  loadMediaContent?: LoadMediaContent;
  orgName?: string;
  onAnswer?: (blocks: NeoContent[]) => Promise<void>;
  onUploadMedia?: (data: ArrayBuffer, mimeType: string) => Promise<string>;
}

export default function CaseDetail({
  dialog,
  displayNames,
  loadMediaContent,
  orgName,
  onAnswer,
  onUploadMedia,
}: CaseDetailProps) {
  const { t, i18n } = useTranslation();
  const {
    formState,
    submitting,
    modalState,
    isAddingAmendment,
    amendmentText,
    submittingAmendment,
    dispatch,
    insertFormula,
    handleSubmitClick,
    submitAnswer,
    canSubmit,
    allowAmendment,
    handleSendAmendment,
  } = useCaseDetailAnswer({ dialog, displayNames, onAnswer, onUploadMedia });

  const sender = dialog.message.blocks[0]?.sender ?? '';
  const dateTime = formatDateTime(
    dialog.message.timestamp,
    i18n.resolvedLanguage ?? i18n.language,
  );
  const resolvedName =
    (sender && displayNames?.[sender]) ?? formatSenderName(sender);
  const initials = getSenderInitials(resolvedName);

  const hasEnvelope = dialog.answers.length > 0;
  const hasInquiry = dialog.message.blocks.some(
    (b) =>
      NeoContent.InquiryMessage.instanceOf(b.content) ||
      NeoContent.InquiryFile.instanceOf(b.content) ||
      NeoContent.InquiryPoll.instanceOf(b.content),
  );
  const hasInquiryMessage = dialog.message.blocks.some((b) =>
    NeoContent.InquiryMessage.instanceOf(b.content),
  );
  const hasInquiryText = dialog.message.blocks
    .filter((b) => NeoContent.InquiryMessage.instanceOf(b.content))
    .some((b) => (formState.texts.get(b.id) ?? '').length > 0);
  const hasInquiryFile = dialog.message.blocks.some((b) =>
    NeoContent.InquiryFile.instanceOf(b.content),
  );

  const messageTitle = hasInquiry
    ? t('caseDetail.inquiryTitle', 'Rückfrage zu Ihrem Vorgang')
    : dialog.message.blocks
        .map((b) => {
          if (NeoContent.Infobox.instanceOf(b.content))
            return b.content.inner.title.plain;
          return undefined;
        })
        .find((title) => title != null);

  const attachments = dialog.message.blocks
    .map((b) => {
      const data = extractAttachmentData(b.content);
      return data ? { id: b.id, ...data } : null;
    })
    .filter((a): a is AttachmentItem => a !== null);

  const inquiryBlocks = dialog.message.blocks.filter(
    (b) =>
      NeoContent.InquiryMessage.instanceOf(b.content) ||
      NeoContent.InquiryFile.instanceOf(b.content) ||
      NeoContent.InquiryPoll.instanceOf(b.content),
  );

  return (
    <div className="case-detail">
      <CaseDetailModals
        modalState={modalState}
        submitting={submitting || submittingAmendment}
        onClose={() => dispatch({ type: 'CLOSE_MODAL' })}
        onSubmit={submitAnswer}
      />

      <section className="case-detail-section">
        {/* Person header */}
        <PersonHeader
          initials={initials}
          name={resolvedName}
          orgName={orgName}
        />

        {/* Date + title header */}
        {messageTitle && (
          <div className="case-detail-message-group">
            <span className="case-detail-date">{dateTime}</span>
            <h2 className="case-detail-message-title">{messageTitle}</h2>
          </div>
        )}

        {/* Inline blocks in event order — attachments and inquiry blocks handled separately */}
        <MessageBlocks
          blocks={dialog.message.blocks}
          loadMediaContent={loadMediaContent}
        />

        {/* Attachments */}
        <AttachmentsSection
          attachments={attachments}
          loadMediaContent={loadMediaContent}
        />
      </section>

      {/* Envelope section */}
      {hasInquiry && (
        <>
          <hr className="case-detail-divider" />
          {hasEnvelope ? (
            <EnvelopeView
              envelopes={dialog.answers}
              inquiryBlocks={inquiryBlocks}
              loadMediaContent={loadMediaContent}
            />
          ) : (
            <section className="case-detail-section">
              <ComposeEnvelope
                messageBlocks={dialog.message.blocks}
                state={formState}
                onChange={(next) => dispatch({ type: 'SET_FORM', value: next })}
                showFormulaButton={
                  hasInquiryMessage && !hasInquiryFile && !hasInquiryText
                }
                onInsertFormula={insertFormula}
                canSubmit={canSubmit}
                onSubmit={handleSubmitClick}
              />
            </section>
          )}
        </>
      )}

      {/* Amendment section — shown only after an answer has been submitted */}
      {hasInquiry && hasEnvelope && (
        <>
          <hr className="case-detail-divider" />
          <section className="case-detail-section">
            <AmendmentSection
              allowAmendment={allowAmendment}
              isAddingAmendment={isAddingAmendment}
              amendmentText={amendmentText}
              submittingAmendment={submittingAmendment}
              onStart={() => dispatch({ type: 'START_AMENDMENT' })}
              onCancel={() => dispatch({ type: 'CANCEL_AMENDMENT' })}
              onTextChange={(text) =>
                dispatch({ type: 'SET_AMENDMENT_TEXT', text })
              }
              onSend={handleSendAmendment}
            />
          </section>
        </>
      )}
    </div>
  );
}
