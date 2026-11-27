// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useTranslation } from 'react-i18next';
import Button from '../../Button/Button';
import ButtonRow from '../../ButtonRow/ButtonRow';
import Close from '../../Icons/Close';
import Plus from '../../Icons/Plus';
import Send from '../../Icons/Send';
import Infobox from '../../Infobox/Infobox';
import '../InquiryForm/InquiryForm.css';
import './AmendmentSection.css';

export default function AmendmentSection({
  allowAmendment,
  isAddingAmendment,
  amendmentText,
  submittingAmendment,
  onStart,
  onCancel,
  onTextChange,
  onSend,
}: {
  allowAmendment: boolean;
  isAddingAmendment: boolean;
  amendmentText: string;
  submittingAmendment: boolean;
  onStart: () => void;
  onCancel: () => void;
  onTextChange: (text: string) => void;
  onSend: () => void;
}) {
  const { t } = useTranslation();

  if (!allowAmendment) {
    return (
      <Infobox
        title={t('caseDetail.noAmendmentTitle', 'Vorgang abgeschlossen')}
        body={t(
          'caseDetail.noAmendmentBody',
          'Melden Sie sich per Email oder Telefon bei Ihrer Behörde, um ihren Vorgang zu besprechen.',
        )}
      />
    );
  }

  if (isAddingAmendment) {
    return (
      <div className="case-detail-amendment-form">
        <h3 className="case-detail-amendment-title">
          {t('caseDetail.amendmentTitle', 'Ihre Ergänzung')}
        </h3>
        <div className="case-detail-amendment-content">
          <div className="case-detail-textarea-wrapper">
            <textarea
              className="case-detail-textarea"
              aria-label={t('caseDetail.amendmentTitle', 'Ihre Ergänzung')}
              placeholder={t(
                'caseDetail.textareaPlaceholder',
                'Geben Sie Ihre Antwort ein...',
              )}
              value={amendmentText}
              onChange={(e) => onTextChange(e.target.value)}
            />
          </div>
          <ButtonRow align="spaced">
            <Button type="danger" onClick={onCancel} icon={<Close />}>
              {t('caseDetail.cancelAmendment', 'Abbrechen')}
            </Button>
            <Button
              type="primary"
              disabled={!amendmentText.trim() || submittingAmendment}
              onClick={onSend}
              icon={<Send />}
            >
              {submittingAmendment
                ? t('caseDetail.sending', 'Wird gesendet...')
                : t('caseDetail.sendAmendment', 'Ergänzung senden')}
            </Button>
          </ButtonRow>
        </div>
      </div>
    );
  }

  return (
    <ButtonRow align="end">
      <Button type="secondary" onClick={onStart} icon={<Plus />}>
        {t('caseDetail.addMoreAnswer', 'Antwort ergänzen')}
      </Button>
    </ButtonRow>
  );
}
