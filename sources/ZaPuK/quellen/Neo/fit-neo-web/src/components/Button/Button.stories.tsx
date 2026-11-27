// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import ArrowRight from '../Icons/ArrowRight';
import Button from './Button';

const meta = {
  title: 'Components/Select/Button',
  component: Button,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    type: {
      control: 'radio',
      options: ['primary', 'secondary', 'text'],
      description: 'The type of button',
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the button is disabled',
      table: {
        // @ts-expect-error - Incorrect type definitions
        defaultValue: { summary: false },
      },
    },
  },
} satisfies Meta<typeof Button>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    type: 'primary',
    children: 'Button',
    icon: <ArrowRight />,
  },
};

export const Secondary: Story = {
  args: {
    type: 'secondary',
    children: 'Button',
    icon: <ArrowRight />,
  },
};

export const Text: Story = {
  args: {
    type: 'text',
    children: 'Button',
    icon: <ArrowRight />,
  },
};

export const WithoutIcon: Story = {
  args: {
    type: 'primary',
    children: 'Button',
  },
};
