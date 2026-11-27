// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import Input from './Input';

const meta = {
  title: 'Components/Input/Radio',
  component: Input,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  args: {
    type: 'radio',
  },
  argTypes: {
    type: {
      table: { disable: true },
    },
    label: {
      control: 'text',
      description: 'Visible label for the radio',
    },
    name: {
      control: 'text',
      description: 'Name attribute, groups radio inputs together',
    },
    value: {
      control: 'text',
      description: 'Value attribute identifying this option',
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
      description: 'Whether the radio is disabled',
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
    label: 'Radio label',
    name: 'radio-example',
    value: 'option-a',
  },
};

export const Checked: Story = {
  args: {
    label: 'Checked radio',
    name: 'radio-checked-example',
    value: 'option-a',
    defaultChecked: true,
  },
};

export const Disabled: Story = {
  args: {
    label: 'Disabled radio',
    name: 'radio-disabled-example',
    value: 'option-a',
    disabled: true,
  },
};

export const DisabledChecked: Story = {
  args: {
    label: 'Disabled checked radio',
    name: 'radio-disabled-checked-example',
    value: 'option-a',
    defaultChecked: true,
    disabled: true,
  },
};
