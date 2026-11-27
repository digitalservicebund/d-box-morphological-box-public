// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useTranslation } from 'react-i18next';
import CaseDetail from '../../../components/CaseDetail/CaseDetail';
import Timeline from '../../../components/Timeline/Timeline';
import {
  debugDialog,
  debugOrgName,
  debugUploadMedia,
} from '../../../debugUtils';
import { neoCaseToSections } from '../../caseTimeline';
import '../InnerCase/InnerCase.css';

export default function DebugCase() {
  const { t, i18n } = useTranslation();
  const language = i18n.resolvedLanguage ?? i18n.language;
  const debugDisplayNames = { '@max.mustermann:example.com': 'Max Mustermann' };
  const debugSections = neoCaseToSections(
    { id: '$debug-case-id', dialogs: [debugDialog] },
    null,
    debugDisplayNames,
    t,
    language,
  );

  return (
    <div className="content timeline">
      <aside aria-label={t('case.timeline.label', 'Vorgangsverlauf')}>
        <Timeline
          sections={debugSections}
          selectedId={debugDialog.id}
          onSelect={() => {}}
          isLoadingMore={false}
        />
      </aside>
      <div className="message-content">
        <CaseDetail
          dialog={debugDialog}
          displayNames={debugDisplayNames}
          orgName={debugOrgName}
          onAnswer={async (_blocks) => {
            await new Promise((r) => setTimeout(r, 500));
          }}
          onUploadMedia={debugUploadMedia}
        />
      </div>
    </div>
  );
}
