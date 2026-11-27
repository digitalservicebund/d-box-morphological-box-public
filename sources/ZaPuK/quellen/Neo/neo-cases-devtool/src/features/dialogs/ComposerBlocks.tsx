// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Dispatch, FC } from 'react';
import { DraftBlockEditor } from '../../components/blocks/DraftBlockEditor';
import type { BlockDraft } from '../../domain/neo/types';
import type { SendJournal } from '../../matrix/send-envelope';
import type { DraftAction } from './draft-reducer';
import styles from './DialogComposer.module.css';

export interface ComposerBlocksProps {
  blocks: BlockDraft[];
  pending?: SendJournal;
  progress: number;
  frozen: boolean;
  dispatch: Dispatch<DraftAction>;
}

export const ComposerBlocks: FC<ComposerBlocksProps> = ({ blocks, pending, progress, frozen, dispatch }) => {
  if (pending)
    return (
      <p>
        {pending.blocks.length} blocks · {progress} acknowledged. Retry resumes the immutable publishing snapshot.
      </p>
    );
  if (!blocks.length) return <p className={styles.empty}>Add a building block to start your dialog.</p>;
  return (
    <>
      {blocks.map((block, index) => (
        <DraftBlockEditor
          key={block.id}
          block={block}
          index={index}
          disabled={frozen}
          update={(patch) => dispatch({ type: 'update', id: block.id, patch })}
          remove={() => dispatch({ type: 'remove', id: block.id })}
        />
      ))}
    </>
  );
};
