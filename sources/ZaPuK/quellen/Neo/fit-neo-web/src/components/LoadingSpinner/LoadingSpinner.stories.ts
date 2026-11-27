// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import { LoadingSpinner } from './LoadingSpinner';

const meta = {
  title: 'Components/Feedback/LoadingSpinner',
  component: LoadingSpinner,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    text: {
      control: 'text',
      description: 'Accessible label text shown below the spinner',
    },
    inline: {
      control: 'boolean',
      description: 'Display spinner and text side by side',
      table: {
        // @ts-expect-error - Incorrect type definitions
        defaultValue: { summary: false },
      },
    },
  },
} satisfies Meta<typeof LoadingSpinner>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    text: 'Daten werden geladen ...',
  },
};

export const WithText: Story = {
  args: {
    text: 'Daten werden geladen ...',
  },
};

export const Inline: Story = {
  args: {
    text: 'Wird geladen ...',
    inline: true,
  },
};
