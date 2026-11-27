// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { Textarea } from '../../ui/Textarea';
import { TextField } from '../../ui/TextField';
import type { EditorProps } from './types';
import styles from './Editors.module.css';

export const InquiryBlockEditor: FC<EditorProps> = ({ block, update, disabled }) => (
  <div className={styles.fields}>
    <Textarea label="Prompt" value={block.text} onChange={(text) => update({ text })} disabled={disabled} />
    <TextField
      label={block.kind === 'fileInquiry' ? 'Requested file title (optional)' : 'Response input title (optional)'}
      value={block.title}
      onChange={(title) => update({ title })}
      disabled={disabled}
    />
    <TextField
      label="Text response placeholder (optional)"
      value={block.label}
      onChange={(label) => update({ label })}
      disabled={disabled}
    />
  </div>
);
