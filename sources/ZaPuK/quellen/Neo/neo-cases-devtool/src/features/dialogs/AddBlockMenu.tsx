// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { FC } from 'react';
import { MdAdd } from 'react-icons/md';
import { blockKinds, blockLabels, isInquiry, types, type BlockKind } from '../../domain/neo/identifiers';
import { blockIcons } from '../../components/blocks/icons';
import { MenuButton } from '../../components/ui/MenuButton';

export interface AddBlockMenuProps {
  onAdd: (kind: BlockKind) => void;
  hasInquiry: boolean;
  disabled: boolean;
}

export const AddBlockMenu: FC<AddBlockMenuProps> = ({ onAdd, hasInquiry, disabled }) => (
  <MenuButton
    icon={MdAdd}
    label="Add block"
    variant="outlined"
    disabled={disabled}
    options={blockKinds.map((kind) => ({
      value: kind,
      label: blockLabels[kind],
      icon: blockIcons[kind],
      disabled: hasInquiry && isInquiry(types[kind]),
    }))}
    onSelect={(kind) => onAdd(kind as BlockKind)}
  >
    Add block
  </MenuButton>
);
