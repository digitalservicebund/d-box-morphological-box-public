// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useTranslation } from 'react-i18next';
import Button from '../Button/Button';
import ArrowRight from '../Icons/ArrowRight';
import Link from '../Link/Link';
import './one-one-five-footer.css';

export default function OneOneFiveFooter() {
  const { t } = useTranslation();

  return (
    <div className="oneonefive-footer">
      <img
        src="/115-illustration.svg"
        alt={t(
          'help.illustrationAlt',
          'Zwei Personen besprechen ein Dokument neben einem Fragezeichen.',
        )}
      />
      <div className="inner">
        <div className="title-body">
          <h2>{t('help.title', 'Benötigen Sie Hilfe?')}</h2>
          <p>
            {t(
              'help.description',
              'Unter der 115 beantwortet das Team der Behördennummer von Montag bis Freitag zwischen 08:00 und 18:00 Uhr alle Rückfragen zu Ihren Vorgängen.',
            )}
          </p>
          <Link href="https://www.115.de">https://www.115.de</Link>
        </div>
        <div className="title-body">
          <p>
            {t(
              'help.alternative',
              'Alternativ können Sie auch unseren Bereich für Hilfe nutzen, um nach einer Antwort auf Ihre spezifische Frage zu suchen.',
            )}
          </p>
        </div>
        <div className="horizontal-end-aligned">
          <Button type="secondary" icon={<ArrowRight />}>
            {t('help.openButton', 'Hilfe öffnen')}
          </Button>
        </div>
      </div>
    </div>
  );
}
