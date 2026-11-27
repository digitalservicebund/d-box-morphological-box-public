// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useTranslation } from 'react-i18next';
import ExternalLinkIndicator from '../../Icons/ExternalLinkIndicator';
import Link from '../../Link/Link';
import './person-header.css';

export interface PersonHeaderProps {
  initials: string;
  name: string;
  orgName?: string;
  websiteHref?: string;
}

export default function PersonHeader({
  initials,
  name,
  orgName,
  websiteHref = '#',
}: PersonHeaderProps) {
  const { t } = useTranslation();

  return (
    <div className="person-header">
      <div className="avatar" aria-hidden="true">
        {initials}
      </div>
      <div className="person-info">
        <div className="person-info-row">
          <span className="person-name">{name}</span>
          {orgName && <span className="divider" aria-hidden="true" />}
          <span className="person-org">{orgName}</span>
          <Link
            href={websiteHref}
            className="website-link"
            icon={<ExternalLinkIndicator />}
          >
            {t('caseDetail.toWebsite', 'Zur Website')}
          </Link>
        </div>
        <span className="person-subtitle">
          {t(
            'caseDetail.openingHoursPlaceholder',
            'Öffnungszeiten: 08:00 - 17:00 (heute)',
          )}
        </span>
      </div>
    </div>
  );
}
