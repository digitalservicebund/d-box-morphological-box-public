// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { TextField } from '../../ui/TextField';
import type { EditorProps } from './types';
import styles from './Editors.module.css';

export const LinkBlockEditor: FC<EditorProps> = ({ block, update, disabled }) => (
  <div className={styles.fields}>
    <TextField label="Link label" value={block.label} onChange={(label) => update({ label })} disabled={disabled} />
    <TextField label="HTTPS URL" type="url" value={block.url} onChange={(url) => update({ url })} disabled={disabled} />
  </div>
);
