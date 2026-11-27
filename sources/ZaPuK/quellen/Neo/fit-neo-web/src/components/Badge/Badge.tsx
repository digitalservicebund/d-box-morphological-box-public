// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useTranslation } from 'react-i18next';
import Error from '../Icons/Badge/Error';
import Info from '../Icons/Badge/Info';
import Neutral from '../Icons/Badge/Neutral';
import Received from '../Icons/Badge/Received';
import Success from '../Icons/Badge/Success';
import Warning from '../Icons/Badge/Warning';
import QuestionMark from '../Icons/QuestionMark';
import Link from '../Link/Link';
import './Badge.css';

export interface BadgeProps {
  title: string;
  type: 'info' | 'success' | 'warning' | 'error' | 'neutral' | 'received';
  icon?: React.ReactNode;
  large?: {
    text: string;
    learnMoreLink: string;
    learnMoreIcon?: React.ReactNode;
  };
}

function BadgeIcon({ type }: { type: BadgeProps['type'] }) {
  switch (type) {
    case 'info':
      return <Info />;
    case 'success':
      return <Success />;
    case 'warning':
      return <Warning />;
    case 'error':
      return <Error />;
    case 'neutral':
      return <Neutral />;
    case 'received':
      return <Received />;
    default:
      return null;
  }
}

export default function Badge({ title, type, icon, large }: BadgeProps) {
  const { text, learnMoreLink, learnMoreIcon } = large ?? {};
  const { t } = useTranslation();

  return (
    <div className={`badge ${type} ${large ? 'large' : ''}`}>
      <div className="header">
        {icon ?? <BadgeIcon type={type} />}
        {/* TODO: Figure out correct semantic */}
        <p className="title">{title}</p>
      </div>

      {large && (
        <div className="body">
          <p>{text}</p>
          <div className="learn-more">
            <Link
              href={learnMoreLink ?? '#'}
              icon={learnMoreIcon ?? <QuestionMark />}
            >
              {t('caseDetail.learnMore', 'Mehr erfahren')}
            </Link>
          </div>
        </div>
      )}
    </div>
  );
}
