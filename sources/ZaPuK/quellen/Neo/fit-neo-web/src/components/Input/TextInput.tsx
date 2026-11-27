// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import AlertDiamond from '../Icons/AlertDiamond';
import './text-input.css';

interface TextInputProps {
  label: string;
  name: string;
  id?: string;
  value?: string;
  defaultValue?: string;

  disabled?: boolean;
  error?: string;
  onChange?: (value: string) => void;
}

export default function TextInput({
  label,
  name,
  id,
  value,
  defaultValue,

  disabled,
  error,
  onChange,
}: TextInputProps) {
  const inputId = id ?? name;
  const errorId = `${inputId}-error`;
  const hasError = Boolean(error);

  return (
    <div
      className={`text-input${hasError ? ' text-input-invalid' : ''}${disabled ? ' text-input-disabled' : ''}`}
    >
      <div className="text-input-inner">
        <label className="text-input-label" htmlFor={inputId}>
          {label}
        </label>
        <input
          className="text-input-control"
          type="text"
          id={inputId}
          name={name}
          value={value}
          defaultValue={defaultValue}
          disabled={disabled}
          aria-invalid={hasError}
          aria-describedby={hasError ? errorId : undefined}
          onChange={(e) => onChange?.(e.target.value)}
        />
        {hasError && (
          <p className="text-input-error" id={errorId} role="alert">
            <AlertDiamond className="text-input-error-icon" />
            {error}
          </p>
        )}
      </div>
    </div>
  );
}
