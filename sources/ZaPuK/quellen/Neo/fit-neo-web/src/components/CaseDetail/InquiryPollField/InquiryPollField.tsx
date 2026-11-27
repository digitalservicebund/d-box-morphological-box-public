// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { NeoContent, type NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import { useTranslation } from 'react-i18next';
import {
  POLL_CUSTOM_OPTION_ID,
  type InquiryFormState,
} from '../caseDetailHelpers';
import '../InquiryForm/InquiryForm.css';
import './InquiryPollField.css';

export default function InquiryPollField({
  block,
  state,
  onChange,
}: {
  block: NeoBuildingBlock;
  state: InquiryFormState;
  onChange: (next: InquiryFormState) => void;
}) {
  const { t } = useTranslation();
  const content = block.content;
  if (!NeoContent.InquiryPoll.instanceOf(content)) return null;

  const poll = content.inner.poll;
  const selected = new Set(state.polls.get(block.id) ?? []);
  const customText = state.pollCustom.get(block.id) ?? '';
  const inputType = poll.multiSelect ? 'checkbox' : 'radio';
  return (
    <fieldset
      key={block.id}
      className="case-detail-poll"
      aria-label={poll.question.plain ?? undefined}
    >
      {poll.answers.map((answer) => {
        const isFreeText = answer.id === POLL_CUSTOM_OPTION_ID;
        const isChecked = selected.has(answer.id);
        return (
          <div key={answer.id} className="case-detail-poll-item">
            <label className="case-detail-poll-option" data-type={inputType}>
              <input
                className="case-detail-poll-input"
                type={inputType}
                name={block.id}
                value={answer.id}
                checked={isChecked}
                onChange={(e) => {
                  const polls = new Map(state.polls);
                  if (poll.multiSelect) {
                    const current = polls.get(block.id) ?? [];
                    polls.set(
                      block.id,
                      e.target.checked
                        ? [...current, answer.id]
                        : current.filter((id) => id !== answer.id),
                    );
                  } else {
                    polls.set(block.id, e.target.checked ? [answer.id] : []);
                  }
                  onChange({ ...state, polls });
                }}
              />
              <span className="case-detail-poll-visual" aria-hidden="true" />
              <span className="case-detail-poll-label">
                {isFreeText
                  ? poll.multiSelect
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
            {isFreeText && isChecked && (
              <div className="case-detail-comment-section">
                <strong className="case-detail-comment-label">
                  {t('caseDetail.comment', 'Kommentar:')}
                </strong>
                <div className="case-detail-textarea-wrapper">
                  <textarea
                    className="case-detail-textarea"
                    aria-label={t(
                      'caseDetail.textareaPlaceholder',
                      'Geben Sie Ihre Antwort ein...',
                    )}
                    placeholder={t(
                      'caseDetail.textareaPlaceholder',
                      'Geben Sie Ihre Antwort ein...',
                    )}
                    value={customText}
                    onChange={(e) => {
                      const pollCustom = new Map(state.pollCustom);
                      pollCustom.set(block.id, e.target.value);
                      onChange({ ...state, pollCustom });
                    }}
                  />
                </div>
              </div>
            )}
          </div>
        );
      })}
    </fieldset>
  );
}
