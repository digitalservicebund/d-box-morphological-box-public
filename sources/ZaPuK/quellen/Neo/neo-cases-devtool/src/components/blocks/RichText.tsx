// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useMemo, type FC } from 'react';
import { sanitizeHtml, textRepresentations } from '../../utils/rich-text';
import styles from './Blocks.module.css';

export interface RichTextProps {
  value: unknown;
  plainOnly?: boolean;
}

export const RichText: FC<RichTextProps> = ({ value, plainOnly }) => {
  const text = textRepresentations(value);
  const html = useMemo(() => (text.html ? sanitizeHtml(text.html) : ''), [text.html]);
  return !plainOnly && html ? (
    <div className={styles.rich} dangerouslySetInnerHTML={{ __html: html }} />
  ) : (
    <p className={styles.text}>{text.plain}</p>
  );
};
