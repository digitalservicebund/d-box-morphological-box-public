// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useId, type FC } from 'react';
import styles from './Textarea.module.css';

export type TextareaProps = {
  label: string;
  value: string;
  disabled?: boolean;
  rows?: number;
  monospace?: boolean;
  hideLabel?: boolean;
} & ({ readOnly: true; onChange?: never } | { readOnly?: false; onChange: (value: string) => void });

export const Textarea: FC<TextareaProps> = ({
  label,
  value,
  onChange,
  disabled,
  rows = 4,
  readOnly = false,
  monospace,
  hideLabel,
}) => {
  const id = useId();
  return (
    <div className={styles.field}>
      <label htmlFor={id} className={hideLabel ? styles.hiddenLabel : undefined}>
        {label}
      </label>
      <textarea
        id={id}
        value={value}
        onChange={onChange ? (event) => onChange(event.target.value) : undefined}
        rows={rows}
        disabled={disabled}
        readOnly={readOnly}
        className={monospace ? styles.code : undefined}
        spellCheck={monospace ? false : undefined}
        wrap={monospace ? 'off' : undefined}
      />
    </div>
  );
};
