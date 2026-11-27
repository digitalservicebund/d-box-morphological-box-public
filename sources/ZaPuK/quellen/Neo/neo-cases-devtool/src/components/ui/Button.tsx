// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useEffect, useRef, type FC, type PropsWithChildren } from 'react';
import type { IconType } from 'react-icons';
import styles from './Button.module.css';

export interface ButtonProps extends PropsWithChildren {
  icon?: IconType;
  onClick?: () => void;
  disabled?: boolean;
  loading?: boolean;
  variant?: 'filled' | 'tonal' | 'outlined' | 'text';
  fullWidth?: boolean;
  type?: 'button' | 'submit';
  label?: string;
  slot?: string;
  form?: string;
  danger?: boolean;
}

export const Button: FC<ButtonProps> = ({
  icon: Icon,
  children,
  onClick,
  disabled,
  loading,
  variant = 'filled',
  fullWidth,
  type = 'button',
  label,
  slot,
  form,
  danger,
}) => {
  const ref = useRef<HTMLElementTagNameMap['m3-button']>(null);
  useEffect(() => {
    // The form-associated custom element exposes a readonly `form` property.
    if (form) ref.current?.setAttribute('form', form);
    else ref.current?.removeAttribute('form');
  }, [form]);
  return (
    <m3-button
      ref={ref}
      className={danger ? styles.danger : undefined}
      variant={variant}
      onClick={onClick}
      disabled={disabled}
      loading={loading}
      full-width={fullWidth}
      type={type}
      aria-label={label}
      title={label}
      icon-only={!children}
      slot={slot}
    >
      {Icon && (
        <span slot="icon">
          <Icon aria-hidden="true" focusable="false" />
        </span>
      )}
      {children}
    </m3-button>
  );
};
