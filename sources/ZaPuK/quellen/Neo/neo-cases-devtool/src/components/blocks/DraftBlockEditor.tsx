// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdDeleteOutline } from 'react-icons/md';
import { blockLabels, type BlockKind } from '../../domain/neo/identifiers';
import { Button } from '../ui/Button';
import { blockIcons } from './icons';
import { TextBlockEditor } from './editors/TextBlockEditor';
import { LinkBlockEditor } from './editors/LinkBlockEditor';
import { InfoboxBlockEditor } from './editors/InfoboxBlockEditor';
import { InquiryBlockEditor } from './editors/InquiryBlockEditor';
import { MediaBlockEditor } from './editors/MediaBlockEditor';
import { PollBlockEditor } from './editors/PollBlockEditor';
import type { EditorProps } from './editors/types';
import styles from './DraftBlockEditor.module.css';

export interface DraftBlockEditorProps extends EditorProps {
  remove: () => void;
  index: number;
}

const editors: Record<BlockKind, FC<EditorProps>> = {
  text: TextBlockEditor,
  button: LinkBlockEditor,
  infobox: InfoboxBlockEditor,
  textInquiry: InquiryBlockEditor,
  fileInquiry: InquiryBlockEditor,
  pollInquiry: PollBlockEditor,
  file: MediaBlockEditor,
  image: MediaBlockEditor,
  video: MediaBlockEditor,
  audio: MediaBlockEditor,
};

export const DraftBlockEditor: FC<DraftBlockEditorProps> = ({ remove, index, ...props }) => {
  const Icon = blockIcons[props.block.kind];
  const Editor = editors[props.block.kind];
  return (
    <m3-card variant="outlined" width="full" className={styles.block}>
      <div slot="header" className={styles.header}>
        <Icon aria-hidden="true" />
        <strong>{blockLabels[props.block.kind]}</strong>
        <span>Block {index + 1}</span>
        <Button
          icon={MdDeleteOutline}
          danger
          variant="text"
          label={`Delete block ${index + 1}`}
          onClick={remove}
          disabled={props.disabled}
        />
      </div>
      <Editor {...props} />
    </m3-card>
  );
};
