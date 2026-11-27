// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdArrowDropDown, MdDns } from 'react-icons/md';
import { config } from '../../app/config';
import { MenuButton } from '../../components/ui/MenuButton';
import styles from './ServerSelect.module.css';

export interface ServerSelectProps {
  value: string;
  onChange: (value: string) => void;
  disabled: boolean;
}

export const ServerSelect: FC<ServerSelectProps> = ({ value, onChange, disabled }) => (
  <div className={styles.field} onKeyDown={(event) => event.stopPropagation()}>
    <span className={styles.label}>Server</span>
    <MenuButton
      icon={MdDns}
      label={`Server: ${value}`}
      options={config.homeservers.map((url) => ({ value: url, label: url }))}
      value={value}
      onSelect={onChange}
      disabled={disabled}
      variant="outlined"
      fullWidth
    >
      <span className={styles.value}>{value}</span>
      <MdArrowDropDown aria-hidden="true" />
    </MenuButton>
  </div>
);
