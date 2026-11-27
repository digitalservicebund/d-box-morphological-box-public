// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import { useTranslation } from 'react-i18next';
import '../CaseDetail.css';
import { POLL_CUSTOM_OPTION_ID } from '../caseDetailHelpers';
import '../InquiryPollField/InquiryPollField.css';

export default function PollReply({
  block,
  envelopeBlocks,
}: {
  block: NeoBuildingBlock;
  envelopeBlocks: NeoBuildingBlock[];
}) {
  const { t } = useTranslation();
  const c = block.content;
  if (!NeoContent.InquiryPoll.instanceOf(c)) return null;

  const pollReply = envelopeBlocks.find(
    (b) =>
      NeoContent.Poll.instanceOf(b.content) &&
      b.content.inner.replyTo === block.id,
  );
  const pollReplyContent =
    pollReply && NeoContent.Poll.instanceOf(pollReply.content)
      ? pollReply.content.inner
      : undefined;
  const selections = new Set(pollReplyContent?.selections ?? []);
  const customText = pollReplyContent?.custom;
  return (
    <fieldset
      className="case-detail-poll case-detail-poll-readonly"
      aria-label={c.inner.poll.question.plain ?? undefined}
      disabled
    >
      {c.inner.poll.answers.map((answer) => {
        const isFreeText = answer.id === POLL_CUSTOM_OPTION_ID;
        const isChecked = selections.has(answer.id);
        return (
          <div key={answer.id} className="case-detail-poll-item">
            <label
              className="case-detail-poll-option"
              data-type={c.inner.poll.multiSelect ? 'checkbox' : 'radio'}
            >
              <input
                className="case-detail-poll-input"
                type={c.inner.poll.multiSelect ? 'checkbox' : 'radio'}
                readOnly
                checked={isChecked}
              />
              <span className="case-detail-poll-visual" aria-hidden="true" />
              <span className="case-detail-poll-label">
                {isFreeText
                  ? c.inner.poll.multiSelect
                    ? t(
                        'caseDetail.pollMultiCustomOption',
                        'Weitere Antwort ergänzen',
                      )
                    : t(
                        'caseDetail.pollCustomOption',
                        'Keine der genannten Antworten',
                      )
                  : answer.text.plain}
              </span>
            </label>
            {isFreeText && isChecked && customText && (
              <div className="case-detail-comment-section">
                <strong className="case-detail-comment-label">
                  {t('caseDetail.comment', 'Kommentar:')}
                </strong>
                <p className="case-detail-text">{customText}</p>
              </div>
            )}
          </div>
        );
      })}
    </fieldset>
  );
}
