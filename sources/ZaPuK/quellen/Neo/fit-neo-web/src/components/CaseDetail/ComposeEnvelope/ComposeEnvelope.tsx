// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { NeoBuildingBlock } from '@matrix-g2x/neo-web-sdk/web';
import { useTranslation } from 'react-i18next';
import Button from '../../Button/Button';
import ButtonRow from '../../ButtonRow/ButtonRow';
import Send from '../../Icons/Send';
import SignOff from '../../Icons/SignOff';
import type { InquiryFormState } from '../caseDetailHelpers';
import InquiryForm from '../InquiryForm/InquiryForm';
import './compose-envelope.css';

export interface ComposeEnvelopeProps {
  messageBlocks: NeoBuildingBlock[];
  state: InquiryFormState;
  onChange: (next: InquiryFormState) => void;
  showFormulaButton: boolean;
  onInsertFormula: () => void;
  canSubmit: boolean;
  onSubmit: () => void;
}

export default function ComposeEnvelope({
  messageBlocks,
  state,
  onChange,
  showFormulaButton,
  onInsertFormula,
  canSubmit,
  onSubmit,
}: ComposeEnvelopeProps) {
  const { t } = useTranslation();

  return (
    <>
      <h3 className="case-detail-envelope-title">
        {t('caseDetail.yourAnswer', 'Ihre Antwort')}
      </h3>
      <div className="reply-area">
        <InquiryForm
          messageBlocks={messageBlocks}
          state={state}
          onChange={onChange}
        />
        <ButtonRow align={showFormulaButton ? 'spaced' : 'end'}>
          {showFormulaButton && (
            <Button
              type="secondary"
              onClick={onInsertFormula}
              icon={<SignOff />}
            >
              {t('caseDetail.insertFormula', 'Grußformel einfügen')}
            </Button>
          )}
          <Button
            type="primary"
            disabled={!canSubmit}
            onClick={onSubmit}
            icon={<Send />}
          >
            {t('caseDetail.submit', 'Antwort senden')}
          </Button>
        </ButtonRow>
      </div>
    </>
  );
}
