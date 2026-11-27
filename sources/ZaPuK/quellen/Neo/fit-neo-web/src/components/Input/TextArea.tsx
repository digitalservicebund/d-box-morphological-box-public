// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { ReactNode } from 'react';
import Button from '../Button/Button';
import AlertDiamond from '../Icons/AlertDiamond';
import ArrowRight from '../Icons/ArrowRight';
import './text-area.css';

interface IconButtonAction {
  icon: ReactNode;
  label: string;
  disabled?: boolean;
  onClick?: () => void;
}

interface ActionButton {
  label: string;
  disabled?: boolean;
  onClick?: () => void;
}

interface TextAreaActions {
  iconButtons?:
    | [IconButtonAction | undefined, IconButtonAction | undefined]
    | [];
  button?: ActionButton;
}

interface TextAreaProps {
  name: string;
  id?: string;
  value?: string;
  defaultValue?: string;
  rows?: number;
  disabled?: boolean;
  error?: string;
  actions?: TextAreaActions;
  onChange?: (value: string) => void;
}

export default function TextArea({
  name,
  id,
  value,
  defaultValue,
  rows,
  disabled,
  error,
  actions,
  onChange,
}: TextAreaProps) {
  const textareaId = id ?? name;
  const errorId = `${textareaId}-error`;
  const hasError = Boolean(error);
  const hasActions =
    actions && (actions.iconButtons?.some(Boolean) || actions.button);

  return (
    <div
      className={`text-area${hasError ? ' text-area-invalid' : ''}${disabled ? ' text-area-disabled' : ''}`}
    >
      <div className="text-area-content">
        <div className="text-area-inner">
          <textarea
            className="text-area-control"
            id={textareaId}
            name={name}
            value={value}
            defaultValue={defaultValue}
            rows={rows}
            disabled={disabled}
            aria-invalid={hasError}
            aria-describedby={hasError ? errorId : undefined}
            onChange={(e) => onChange?.(e.target.value)}
          />
          {hasError && (
            <p className="text-area-error" id={errorId} role="alert">
              <AlertDiamond className="text-area-error-icon" />
              {error}
            </p>
          )}
        </div>
        {hasActions && (
          <div className="text-area-actions">
            <div className="text-area-actions-icons">
              {actions.iconButtons?.map((btn, i) =>
                btn ? (
                  <button
                    key={i}
                    className="text-area-icon-button"
                    aria-label={btn.label}
                    disabled={btn.disabled}
                    onClick={btn.onClick}
                  >
                    {btn.icon}
                  </button>
                ) : null,
              )}
            </div>
            {actions.button && (
              <Button
                type="primary"
                disabled={actions.button.disabled}
                onClick={actions.button.onClick}
                icon={<ArrowRight />}
              >
                {actions.button.label}
              </Button>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
