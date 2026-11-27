// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import Input from './Input';

const meta = {
  title: 'Components/Input/Checkbox',
  component: Input,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  args: {
    type: 'checkbox',
  },
  argTypes: {
    type: {
      table: { disable: true },
    },
    label: {
      control: 'text',
      description: 'Visible label for the checkbox',
    },
    name: {
      control: 'text',
      description: 'Name attribute for the checkbox',
    },
    value: {
      control: 'text',
      description: 'Optional value attribute',
    },
    checked: {
      control: 'boolean',
      description: 'Controlled checked state',
    },
    defaultChecked: {
      control: 'boolean',
      description: 'Uncontrolled initial checked state',
      table: {
        // @ts-expect-error - Incorrect type definitions
        defaultValue: { summary: false },
      },
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the checkbox is disabled',
      table: {
        // @ts-expect-error - Incorrect type definitions
        defaultValue: { summary: false },
      },
    },
  },
} satisfies Meta<typeof Input>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    label: 'Checkbox label',
    name: 'checkbox-example',
  },
};

export const Checked: Story = {
  args: {
    label: 'Checked checkbox',
    name: 'checkbox-checked-example',
    defaultChecked: true,
  },
};

export const Disabled: Story = {
  args: {
    label: 'Disabled checkbox',
    name: 'checkbox-disabled-example',
    disabled: true,
  },
};

export const DisabledChecked: Story = {
  args: {
    label: 'Disabled checked checkbox',
    name: 'checkbox-disabled-checked-example',
    defaultChecked: true,
    disabled: true,
  },
};
