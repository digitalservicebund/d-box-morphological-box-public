// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { TextField } from '../../ui/TextField';
import { TextBlockEditor } from './TextBlockEditor';
import { LinkBlockEditor } from './LinkBlockEditor';
import type { EditorProps } from './types';
import styles from './Editors.module.css';

export const InfoboxBlockEditor: FC<EditorProps> = (props) => (
  <div className={styles.fields}>
    <TextField
      label="Title (optional)"
      value={props.block.title}
      onChange={(title) => props.update({ title })}
      disabled={props.disabled}
    />
    <TextBlockEditor {...props} />
    <details>
      <summary>Link (optional)</summary>
      <div className={styles.advanced}>
        <LinkBlockEditor {...props} />
      </div>
    </details>
  </div>
);
