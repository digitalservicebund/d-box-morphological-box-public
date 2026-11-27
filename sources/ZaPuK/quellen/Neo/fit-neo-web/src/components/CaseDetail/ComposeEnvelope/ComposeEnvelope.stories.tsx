// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import ComposeEnvelope from './ComposeEnvelope';

const meta = {
  title: 'Components/CaseDetail/ComposeEnvelope',
  component: ComposeEnvelope,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof ComposeEnvelope>;

export default meta;
type Story = StoryObj<typeof meta>;

const baseState = {
  texts: new Map(),
  polls: new Map(),
  pollCustom: new Map(),
  files: new Map(),
  fileDeclarations: new Map(),
  fileComments: new Map(),
};

export const Default: Story = {
  args: {
    messageBlocks: [],
    state: baseState,
    onChange: () => {},
    showFormulaButton: true,
    onInsertFormula: () => {},
    canSubmit: false,
    onSubmit: () => {},
  },
};

export const ReadyToSubmit: Story = {
  args: {
    messageBlocks: [],
    state: baseState,
    onChange: () => {},
    showFormulaButton: false,
    onInsertFormula: () => {},
    canSubmit: true,
    onSubmit: () => {},
  },
};
