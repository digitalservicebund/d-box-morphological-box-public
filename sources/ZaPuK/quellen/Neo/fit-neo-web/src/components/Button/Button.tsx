// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { PropsWithChildren, ReactNode } from 'react';
import './button.css';

export interface ButtonProps {
  type: 'primary' | 'secondary' | 'text' | 'danger';
  disabled?: boolean;
  onClick?: () => void;
  icon?: ReactNode;
}

export default function Button({
  children,
  type,
  disabled,
  onClick,
  icon,
}: PropsWithChildren<ButtonProps>) {
  return (
    <button
      className={`button ${type}`}
      aria-disabled={disabled}
      onClick={() => {
        if (!disabled) onClick?.();
      }}
    >
      <span>{children}</span>
      {icon}
    </button>
  );
}
