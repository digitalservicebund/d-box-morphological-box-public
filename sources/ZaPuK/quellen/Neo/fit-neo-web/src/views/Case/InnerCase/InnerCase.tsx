// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { CaseStore } from '@matrix-g2x/neo-web-sdk/react';
import { useMemo, useState, useSyncExternalStore } from 'react';
import { useTranslation } from 'react-i18next';
import Card from '../../../components/Card/Card';
import CaseDetail from '../../../components/CaseDetail/CaseDetail';
import Envelope from '../../../components/Icons/Envelope';
import Timeline from '../../../components/Timeline/Timeline';
import { debugCaseDetail } from '../../../debugUtils';
import type { LoadMediaContent } from '../../../hooks/useMediaObjectUrl';
import { neoCaseToSections } from '../../caseTimeline';
import DebugCase from '../DebugCase/DebugCase';
import './InnerCase.css';

interface InnerCaseProps {
  caseStore: CaseStore;
  loadMediaContent: LoadMediaContent;
  orgName: string | undefined;
  onUploadMedia: (data: ArrayBuffer, mimeType: string) => Promise<string>;
}

export default function InnerCase({
  caseStore,
  loadMediaContent,
  orgName,
  onUploadMedia,
}: InnerCaseProps) {
  const { t, i18n } = useTranslation();
  const language = i18n.resolvedLanguage ?? i18n.language;
  const viewState = useSyncExternalStore(
    caseStore.subscribe,
    caseStore.getSnapshot,
  );
  const [userSelectedEntryId, setUserSelectedEntryId] = useState<
    string | undefined
  >();

  const sections = useMemo(
    () =>
      viewState.neoCase
        ? neoCaseToSections(
            viewState.neoCase,
            viewState.lastReadTimestamp,
            viewState.displayNames,
            t,
            language,
          )
        : [],
    [
      viewState.neoCase,
      viewState.lastReadTimestamp,
      viewState.displayNames,
      t,
      language,
    ],
  );

  const mostRecentDialogId = useMemo(() => {
    const dialogs = viewState.neoCase?.dialogs;
    if (!dialogs?.length) return undefined;
    return dialogs.reduce((best, d) =>
      d.message.timestamp > best.message.timestamp ? d : best,
    ).id;
  }, [viewState.neoCase]);

  const selectedEntryId = userSelectedEntryId ?? mostRecentDialogId;
  const selectedDialog = selectedEntryId
    ? viewState.neoCase?.dialogs.find((d) => d.id === selectedEntryId)
    : undefined;

  if (debugCaseDetail) return <DebugCase />;

  if (
    !viewState.loading &&
    (!viewState.neoCase || viewState.neoCase.dialogs.length === 0)
  ) {
    return (
      <div className="content">
        <Card
          title={t('case.noMessages.title', 'Keine Nachrichten Vorliegend')}
          icon={<Envelope />}
          text={t(
            'case.noMessages.description',
            'Sobald Sie eine Nachricht zu ihrem Antrag erhalten haben, können Sie diese hier einsehen.',
          )}
          centered
        />
      </div>
    );
  }

  return (
    <div className="content timeline">
      <aside aria-label={t('case.timeline.label', 'Vorgangsverlauf')}>
        <Timeline
          sections={sections}
          selectedId={selectedEntryId}
          onSelect={setUserSelectedEntryId}
          isLoadingMore={viewState.loading}
        />
      </aside>
      <div className="message-content">
        {selectedDialog ? (
          <CaseDetail
            dialog={selectedDialog}
            displayNames={viewState.displayNames}
            loadMediaContent={loadMediaContent}
            orgName={orgName}
            onAnswer={async (blocks) => {
              await caseStore.sendAnswer(selectedDialog.id, blocks);
              caseStore.reload();
            }}
            onUploadMedia={onUploadMedia}
          />
        ) : (
          <div className="box" />
        )}
      </div>
    </div>
  );
}
