// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { Textarea } from '../../ui/Textarea';
import type { EditorProps } from './types';
import styles from './Editors.module.css';

export const TextBlockEditor: FC<EditorProps> = ({ block, update, disabled }) => (
  <div className={styles.fields}>
    <Textarea label="Text" value={block.text} onChange={(text) => update({ text })} disabled={disabled} />
    <details>
      <summary>Formatted HTML (optional)</summary>
      <div className={styles.advanced}>
        <Textarea
          label="HTML representation"
          value={block.html}
          onChange={(html) => update({ html })}
          disabled={disabled}
        />
        <small>Supported Matrix HTML only. The text above remains the plain-text fallback.</small>
      </div>
    </details>
  </div>
);
