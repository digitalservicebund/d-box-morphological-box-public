// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, useEffect, useReducer, useRef, useState, type FC } from 'react';
import { MdClose, MdSave, MdEditNote, MdRefresh } from 'react-icons/md';
import { isInquiry, types, type BlockKind } from '../../domain/neo/identifiers';
import { Button } from '../../components/ui/Button';
import { Checkbox } from '../../components/ui/Checkbox';
import { ComposerBlocks } from './ComposerBlocks';
import type { SendJournal } from '../../matrix/send-envelope';
import { createBlock, draftReducer } from './draft-reducer';
import { AddBlockMenu } from './AddBlockMenu';
import { usePublisher } from './use-publisher';
import styles from './DialogComposer.module.css';

export interface DialogComposerProps {
  roomId: string;
  pending?: SendJournal;
  onPending: (journal: SendJournal) => void;
  onSaved: () => void;
}

export const DialogComposer: FC<DialogComposerProps> = (props) => {
  const [blocks, dispatch] = useReducer(draftReducer, undefined, () => [createBlock('text')]);
  const ref = useRef<HTMLElement>(null);
  const [legacyCompatibility, setLegacyCompatibility] = useState(Boolean(props.pending?.legacyCompatibility));
  const publisher = usePublisher({ ...props, blocks, legacyCompatibility });
  const { busy, journal, progress, error, save, status, connected, closeDraft } = publisher;
  const onAdd = useCallback((kind: BlockKind) => dispatch({ type: 'add', kind }), []);
  useEffect(() => {
    ref.current?.scrollIntoView({ behavior: 'smooth', block: 'start' });
    ref.current?.focus();
    const prevent = (event: BeforeUnloadEvent) => event.preventDefault();
    window.addEventListener('beforeunload', prevent);
    return () => window.removeEventListener('beforeunload', prevent);
  }, []);
  const frozen = busy || Boolean(journal);
  const canSave = connected && !busy && (Boolean(journal) || blocks.length > 0);
  return (
    <section ref={ref} tabIndex={-1} className={styles.composer} aria-label="New dialog composer">
      <m3-card variant="outlined" width="full" className={styles.card}>
        <div slot="header" className={styles.heading}>
          <MdEditNote size={25} aria-hidden="true" />
          <div>
            <h2>New dialog</h2>
            <p>Compose the agency envelope. Sent dialogs are read-only.</p>
          </div>
          <span className={styles.draft}>{journal ? 'PENDING' : 'DRAFT'}</span>
        </div>
        <div className={styles.blocks}>
          <ComposerBlocks
            blocks={blocks}
            pending={props.pending}
            progress={progress}
            frozen={frozen}
            dispatch={dispatch}
          />
          {error && (
            <p className={styles.error} role="alert">
              {error}
            </p>
          )}
          {busy && <p role="status">{status}</p>}
        </div>
        <div slot="actions" className={styles.actions}>
          <div className={styles.draftActions}>
            <Button icon={MdClose} variant="text" onClick={closeDraft} disabled={busy}>
              {journal ? 'Close' : 'Cancel'}
            </Button>
            <Checkbox
              label="Legacy compatibility"
              checked={legacyCompatibility}
              onChange={setLegacyCompatibility}
              disabled={frozen}
            />
          </div>
          <AddBlockMenu
            onAdd={onAdd}
            hasInquiry={blocks.some((block) => isInquiry(types[block.kind]))}
            disabled={frozen}
          />
          <Button icon={journal ? MdRefresh : MdSave} onClick={() => void save()} loading={busy} disabled={!canSave}>
            {journal ? 'Retry save' : 'Save'}
          </Button>
        </div>
      </m3-card>
    </section>
  );
};
