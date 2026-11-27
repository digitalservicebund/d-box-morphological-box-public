// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useTranslation } from 'react-i18next';
import Link from '../../components/Link/Link';

export default function NoRequests() {
  const { t } = useTranslation();
  return (
    <div className="no-requests-container">
      <div className="title-body">
        <h2>{t('noRequests.title', 'Keine Vorgänge vorliegend')}</h2>
        <p>
          {t(
            'noRequests.description',
            'Sobald Sie einen aktiven Vorgang bei einer Fachbehörde haben, können Sie unten stehende Vorteile nutzen. Mit dem ersten Vorgang kann es losgehen. Zugang zu den Verwaltungsleistungen von Bund, Ländern und Kommunen finden Sie online.',
          )}
        </p>
        <Link href="https://verwaltung.bund.de/portal/DE">
          https://verwaltung.bund.de/portal/DE
        </Link>
      </div>
      <div className="info-row">
        <div>
          <img src="/communication.svg" alt="" />
          <p>
            {t(
              'noRequests.info1',
              'Einfach, schnelle und sichere Kommunikation über alle ihre aktiven Behördenvorgänge',
            )}
          </p>
        </div>
        <div>
          <img src="/document.svg" alt="" />
          <p>
            {t(
              'noRequests.info2',
              'Unkomplizierte Nachreichung von Dokumenten mit Übersicht über alle ihre Dokumente.',
            )}
          </p>
        </div>
        <div>
          <img src="/multi-documents.svg" alt="" />
          <p>
            {t(
              'noRequests.info3',
              'Viele hilfreiche Informationen und Unterstützungen rund um ihre Vorgänge.',
            )}
          </p>
        </div>
      </div>
    </div>
  );
}
