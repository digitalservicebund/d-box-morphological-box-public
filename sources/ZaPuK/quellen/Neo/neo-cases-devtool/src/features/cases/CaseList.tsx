// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useMemo, useState, type FC } from 'react';
import { MdAdd } from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import { useAuth } from '../auth/auth-context';
import { useWorkspace } from './workspace-context';
import { CreateCaseDialog } from './CreateCaseDialog';
import { CaseListControls } from './CaseListControls';
import { CaseListEntries } from './CaseListEntries';
import { ConnectionFooter } from './ConnectionFooter';
import { filterCases, type CaseListOptions } from './filter-cases';
import styles from './CaseList.module.css';

export const CaseList: FC = () => {
  const [creating, setCreating] = useState(false);
  const [options, setOptions] = useState<CaseListOptions>({ query: '', sortBy: 'title', direction: 'asc' });
  const { rooms, phase, connected } = useAuth();
  const { draftOpen } = useWorkspace();
  const close = useCallback(() => setCreating(false), []);
  const change = useCallback(
    (patch: Partial<CaseListOptions>) => setOptions((current) => ({ ...current, ...patch })),
    [],
  );
  const visible = useMemo(() => filterCases(rooms, options), [rooms, options]);
  return (
    <>
      <aside className={styles.sidebar} inert={draftOpen} aria-label="Cases">
        <div className={styles.controls}>
          <div className={styles.heading}>
            <span>CASES</span>
            {phase === 'loggedIn' && (
              <span>{options.query ? `${visible.length} / ${rooms.length}` : rooms.length}</span>
            )}
          </div>
          <Button
            icon={MdAdd}
            fullWidth
            onClick={() => setCreating(true)}
            disabled={phase !== 'loggedIn' || !connected}
          >
            Add case
          </Button>
          <CaseListControls options={options} onChange={change} disabled={phase !== 'loggedIn'} />
        </div>
        <CaseListEntries rooms={visible} filtered={Boolean(options.query.trim())} resetKey={JSON.stringify(options)} />
        <ConnectionFooter />
      </aside>
      {creating && <CreateCaseDialog onClose={close} />}
    </>
  );
};
