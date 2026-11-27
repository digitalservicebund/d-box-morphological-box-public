// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';

export interface TextFieldProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  type?: string;
  disabled?: boolean;
  required?: boolean;
  error?: string;
  placeholder?: string;
  autoComplete?: string;
  name?: string;
}

export const TextField: FC<TextFieldProps> = ({
  label,
  value,
  onChange,
  type = 'text',
  disabled,
  required,
  error,
  placeholder,
  autoComplete,
  name,
}) => (
  <m3-text-field
    label={label}
    value={value}
    type={type}
    disabled={disabled}
    aria-required={required}
    error={Boolean(error)}
    error-text={error}
    placeholder={placeholder}
    autocomplete={autoComplete}
    name={name}
    variant="outlined"
    onInput={(event) => onChange(event.currentTarget.value)}
  />
);
