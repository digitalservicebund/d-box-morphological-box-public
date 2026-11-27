// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import Alert from './Alert';

const meta = {
  title: 'Components/Information/Alert',
  component: Alert,
  parameters: {
    layout: 'fullscreen',
  },
  tags: ['autodocs'],
  argTypes: {
    open: {
      control: 'boolean',
      defaultValue: true,
    },
  },
} satisfies Meta<typeof Alert>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    title: 'Default alert title',
    text: 'Default alert text',
    primaryAction: { children: 'Primary Action', onClick: () => {} },
    secondaryAction: { children: 'Secondary Action', onClick: () => {} },
    children: (
      <div
        style={{
          width: '100%',
          border: '1px dashed var(--color-layout-border)',
          borderRadius: 'var(--metric-border-radius-small)',
          display: 'flex',
          flexDirection: 'column',
          gap: 'var(--metric-space-default)',
          justifyContent: 'center',
          alignItems: 'center',
          paddingBlock: '32px',
          paddingInline: '24px',
          backgroundColor: 'var(--layout-background-hued)',
        }}
      >
        Test children
      </div>
    ),
    open: true,
  },
};
