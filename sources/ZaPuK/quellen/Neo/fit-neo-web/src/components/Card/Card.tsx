// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useTranslation } from 'react-i18next';
import Badge from '../Badge/Badge';
import Button from '../Button/Button';
import ArrowRight from '../Icons/ArrowRight';
import './Card.css';

export type CardProps = {
  title: string;
  text: string;
  badge?: {
    title: string;
    type: 'info' | 'success' | 'warning' | 'error' | 'neutral' | 'received';
    icon?: React.ReactNode;
  };
  onOpen?: () => void;
  icon?: React.ReactNode;
  centered?: boolean;
};

export default function Card({
  icon,
  title,
  text,
  badge,
  onOpen,
  centered,
}: CardProps) {
  const { t } = useTranslation();

  return (
    <div className={`card ${centered ? 'centered' : ''}`}>
      {badge && (
        <div className="status-badge">
          <Badge title={badge.title} type={badge.type} icon={badge.icon} />
        </div>
      )}
      <div className="title-body">
        {icon && <div className="icon">{icon}</div>}
        <div className="text">
          <h3>{title}</h3>
          <p>{text}</p>
        </div>
      </div>
      {onOpen && (
        <div className="action">
          <Button onClick={onOpen} type="primary" icon={<ArrowRight />}>
            {t('card.open', 'Öffnen')}
          </Button>
        </div>
      )}
    </div>
  );
}
