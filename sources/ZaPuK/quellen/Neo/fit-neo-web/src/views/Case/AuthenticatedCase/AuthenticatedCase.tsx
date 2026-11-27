// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { CaseStore, RoomListStore } from '@matrix-g2x/neo-web-sdk/react';
import { useSyncExternalStore } from 'react';
import { useTranslation } from 'react-i18next';
import Badge from '../../../components/Badge/Badge';
import ArrowRight from '../../../components/Icons/ArrowRight.tsx';
import Info from '../../../components/Icons/Info';
import Link from '../../../components/Link/Link';
import OneOneFiveFooter from '../../../components/OneOneFiveFooter/OneOneFiveFooter';
import TabBar from '../../../components/TabBar/TabBar';
import type { LoadMediaContent } from '../../../hooks/useMediaObjectUrl';
import { formatDateTime } from '../../../lib/formatDateTime';
import AppLayout from '../../AppLayout';
import { MAIN_TABS } from '../../constants';
import './AuthenticatedCase.css';
import InnerCase from '../InnerCase/InnerCase';

interface AuthenticatedCaseProps {
  roomId: string;
  roomListStore: RoomListStore;
  caseStore: CaseStore;
  loadMediaContent: LoadMediaContent;
  onUploadMedia: (data: ArrayBuffer, mimeType: string) => Promise<string>;
}

export default function AuthenticatedCase({
  roomId,
  roomListStore,
  caseStore,
  loadMediaContent,
  onUploadMedia,
}: AuthenticatedCaseProps) {
  const { t, i18n } = useTranslation();
  const language = i18n.resolvedLanguage ?? i18n.language;

  const { rooms } = useSyncExternalStore(
    roomListStore.subscribe,
    roomListStore.getSnapshot,
  );
  const room = rooms.find((r) => r.roomId === roomId);
  const roomTopic = room?.topic;
  const roomName = room?.name ?? roomId;

  const { neoCase } = useSyncExternalStore(
    caseStore.subscribe,
    caseStore.getSnapshot,
  );
  const firstActivity = neoCase?.dialogs.reduce<bigint | undefined>(
    (min, d) =>
      min === undefined || d.message.timestamp < min
        ? d.message.timestamp
        : min,
    undefined,
  );

  return (
    <AppLayout>
      <div className="case">
        <TabBar tabs={MAIN_TABS(t)} />
        <div className="header">
          <Link left href="/">
            {t('case.backLink', 'Vorgänge')}
          </Link>
          <h1>{roomName}</h1>
        </div>
        <div className="case-status-details">
          <div className="case-status">
            <h2>{t('case.status.title', 'Status')}</h2>
            <Badge
              type="received"
              title={t('case.status.received.badge', 'Vorgang eingegangen')}
              large={{
                text: t(
                  'case.status.received.description',
                  'Ihre Behörde wird sich in Kürze bei ihnen melden.',
                ),
                learnMoreLink: '#',
                learnMoreIcon: <ArrowRight />,
              }}
            />
          </div>
          <div className="case-details">
            <div className="case-details-header">
              <h2>{t('case.details.title', 'Details')}</h2>
              <Link href="#" icon={<Info />}>
                {t('case.details.showAll', 'Alle Details anzeigen')}
              </Link>
            </div>
            <table>
              <caption className="visually-hidden">
                {t('case.details.caption', 'Details zum Vorgang')}
              </caption>
              <tbody>
                <tr>
                  <th scope="row">{t('case.details.name', 'Bezeichnung')}</th>
                  <td>{roomName}</td>
                </tr>
                <tr>
                  <th scope="row">
                    {t('case.details.caseNumber', 'Aktenzeichen')}
                  </th>
                  <td>ZZD-2025-274</td>
                </tr>
                <tr>
                  <th scope="row">
                    {t('case.details.submittedAt', 'Antragstellung')}
                  </th>
                  <td>
                    {firstActivity != null
                      ? formatDateTime(firstActivity, language)
                      : '–'}
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
        <InnerCase
          caseStore={caseStore}
          loadMediaContent={loadMediaContent}
          orgName={roomTopic}
          onUploadMedia={onUploadMedia}
        />
        <OneOneFiveFooter />
      </div>
    </AppLayout>
  );
}
