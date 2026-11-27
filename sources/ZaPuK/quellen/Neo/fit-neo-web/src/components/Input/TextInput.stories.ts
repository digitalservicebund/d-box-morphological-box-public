// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import TextInput from './TextInput';

const meta = {
  title: 'Components/Input/TextInput',
  component: TextInput,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    label: {
      control: 'text',
      description: 'Label displayed above the input',
    },
    name: {
      control: 'text',
      description: 'Name attribute for the input',
    },
    id: {
      control: 'text',
      description: 'Optional id — falls back to name if omitted',
    },

    defaultValue: {
      control: 'text',
      description: 'Uncontrolled initial value',
    },
    error: {
      control: 'text',
      description: 'Error message — also triggers the error state when set',
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the input is disabled',
      table: {
        // @ts-expect-error - Incorrect type definitions
        defaultValue: { summary: false },
      },
    },
  },
} satisfies Meta<typeof TextInput>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    label: 'Label',
    name: 'text-input-example',
  },
};

export const WithValue: Story = {
  args: {
    label: 'Label',
    name: 'text-input-value-example',
    defaultValue: 'Some input value',
  },
};

export const Error: Story = {
  args: {
    label: 'Label',
    name: 'text-input-error-example',
    defaultValue: 'Invalid input value',
    error: 'This field contains an error.',
  },
};

export const Disabled: Story = {
  args: {
    label: 'Label',
    name: 'text-input-disabled-example',

    disabled: true,
  },
};

export const DisabledWithValue: Story = {
  args: {
    label: 'Label',
    name: 'text-input-disabled-value-example',
    defaultValue: 'Some input value',
    disabled: true,
  },
};
