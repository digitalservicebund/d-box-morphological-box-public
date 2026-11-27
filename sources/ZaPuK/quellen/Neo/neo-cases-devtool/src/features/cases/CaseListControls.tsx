// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import {
  MdArrowDownward,
  MdArrowUpward,
  MdSearch,
  MdSort,
  MdBusiness,
  MdTitle,
  MdUpdate,
  MdEvent,
} from 'react-icons/md';
import { Button } from '../../components/ui/Button';
import { MenuButton } from '../../components/ui/MenuButton';
import { caseSortLabels, type CaseListOptions, type CaseSortKey } from './filter-cases';
import styles from './CaseListControls.module.css';

export interface CaseListControlsProps {
  options: CaseListOptions;
  onChange: (patch: Partial<CaseListOptions>) => void;
  disabled: boolean;
}
const icons = { agency: MdBusiness, title: MdTitle, updated: MdUpdate, created: MdEvent };
const sortOptions = (Object.keys(caseSortLabels) as CaseSortKey[]).map((key) => ({
  value: key,
  label: caseSortLabels[key],
  icon: icons[key],
}));

export const CaseListControls: FC<CaseListControlsProps> = ({ options, onChange, disabled }) => (
  <div className={styles.controls}>
    <div className={styles.row}>
      <m3-search-bar
        className={styles.search}
        placeholder="Search cases"
        aria-label="Search cases"
        value={options.query}
        onInput={(event) => onChange({ query: event.currentTarget.value })}
        disabled={disabled}
      >
        <span slot="leading">
          <MdSearch aria-hidden="true" />
        </span>
      </m3-search-bar>
      <MenuButton
        icon={MdSort}
        label={`Sort cases by ${caseSortLabels[options.sortBy]}`}
        options={sortOptions}
        value={options.sortBy}
        onSelect={(sortBy) => onChange({ sortBy: sortBy as CaseSortKey })}
        disabled={disabled}
      />
      <Button
        icon={options.direction === 'asc' ? MdArrowUpward : MdArrowDownward}
        variant="text"
        label={options.direction === 'asc' ? 'Switch to descending order' : 'Switch to ascending order'}
        disabled={disabled}
        onClick={() => onChange({ direction: options.direction === 'asc' ? 'desc' : 'asc' })}
      />
    </div>
    <small>
      Sort: {caseSortLabels[options.sortBy]} · {options.direction === 'asc' ? 'Ascending' : 'Descending'}
    </small>
  </div>
);
