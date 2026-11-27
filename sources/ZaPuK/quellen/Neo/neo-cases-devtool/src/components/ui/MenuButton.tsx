// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { useCallback, type FC, type PropsWithChildren } from 'react';
import type { IconType } from 'react-icons';
import { MdCheck } from 'react-icons/md';
import { Button } from './Button';
import { useAnchoredMenu } from './use-anchored-menu';
import styles from './MenuButton.module.css';

export interface MenuOption {
  value: string;
  label: string;
  icon?: IconType;
  disabled?: boolean;
}
export interface MenuButtonProps extends PropsWithChildren {
  icon: IconType;
  label: string;
  options: readonly MenuOption[];
  value?: string;
  onSelect: (value: string) => void;
  disabled?: boolean;
  variant?: 'text' | 'outlined';
  fullWidth?: boolean;
}

export const MenuButton: FC<MenuButtonProps> = ({
  icon,
  label,
  options,
  value,
  onSelect,
  disabled,
  variant = 'text',
  fullWidth,
  children,
}) => {
  const select = useCallback(
    (selection: string) => {
      const option = options.find((item) => item.value === selection);
      if (!disabled && option && !option.disabled) onSelect(selection);
    },
    [options, onSelect, disabled],
  );
  const { menu, popover, anchor, show } = useAnchoredMenu(select);
  return (
    <div ref={anchor} className={styles.anchor}>
      <Button icon={icon} label={label} variant={variant} onClick={show} disabled={disabled} fullWidth={fullWidth}>
        {children}
      </Button>
      <div ref={popover} popover="manual" className={styles.popover}>
        <m3-menu ref={menu}>
          {options.map(({ value: option, label: text, icon: Icon, disabled: inactive }) => (
            <m3-menu-item
              key={option}
              value={option}
              disabled={inactive}
              aria-current={value === option ? 'true' : undefined}
            >
              {Icon && (
                <span slot="leading-icon">
                  <Icon aria-hidden="true" />
                </span>
              )}
              {text}
              {value === option && (
                <span slot="trailing-icon">
                  <MdCheck aria-hidden="true" />
                </span>
              )}
            </m3-menu-item>
          ))}
        </m3-menu>
      </div>
    </div>
  );
};
