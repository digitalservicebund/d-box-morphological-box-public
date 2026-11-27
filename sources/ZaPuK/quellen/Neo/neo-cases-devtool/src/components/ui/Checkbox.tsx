// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useId, useRef, type FC } from 'react';
import styles from './Checkbox.module.css';

export interface CheckboxProps {
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
}

export const Checkbox: FC<CheckboxProps> = ({ label, checked, onChange, disabled }) => {
  const id = useId();
  const ref = useRef<HTMLElementTagNameMap['m3-checkbox']>(null);
  return (
    <div className={styles.row} data-disabled={disabled}>
      <m3-checkbox
        ref={ref}
        id={id}
        checked={checked}
        aria-label={label}
        disabled={disabled}
        onChange={(event) => onChange(event.currentTarget.checked)}
      />
      <label
        htmlFor={id}
        onClick={(event) => {
          // The component does not forward native external-label activation to its inner checkbox.
          event.preventDefault();
          if (disabled) return;
          onChange(!checked);
          ref.current?.shadowRoot?.querySelector<HTMLElement>('[role="checkbox"]')?.focus();
        }}
      >
        {label}
      </label>
    </div>
  );
};
