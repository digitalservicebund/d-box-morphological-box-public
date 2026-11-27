// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import React from 'react';
import AddFile from '../Icons/AddFile';
import AddImage from '../Icons/AddImage';
import TextArea from './TextArea';

const meta = {
  title: 'Components/Input/TextArea',
  component: TextArea,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    name: {
      control: 'text',
      description: 'Name attribute for the textarea',
    },
    id: {
      control: 'text',
      description: 'Optional id — falls back to name if omitted',
    },
    defaultValue: {
      control: 'text',
      description: 'Uncontrolled initial value',
    },
    rows: {
      control: 'number',
      description: 'Number of visible text rows',
    },
    error: {
      control: 'text',
      description: 'Error message — also triggers the error state when set',
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the textarea is disabled',
      table: {
        // @ts-expect-error - Incorrect type definitions
        defaultValue: { summary: false },
      },
    },
    actions: {
      control: false,
      description:
        'Optional actions bar: up to 2 icon buttons on the left and a primary button on the right',
    },
  },
} satisfies Meta<typeof TextArea>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    name: 'text-area-example',
  },
};

export const WithValue: Story = {
  args: {
    name: 'text-area-value-example',
    defaultValue: 'Some textarea value',
  },
};

export const WithIconButtons: Story = {
  args: {
    name: 'text-area-icon-buttons-example',
    defaultValue: 'Some textarea value',
    actions: {
      iconButtons: [
        {
          label: 'Add image',
          icon: React.createElement(AddImage),
          onClick: () => {},
        },
        {
          label: 'Add file',
          icon: React.createElement(AddFile),
          onClick: () => {},
        },
      ],
    },
  },
};

export const WithButton: Story = {
  args: {
    name: 'text-area-button-example',
    defaultValue: 'Some textarea value',
    actions: {
      button: {
        label: 'Submit',
        onClick: () => {},
      },
    },
  },
};

export const WithActions: Story = {
  args: {
    name: 'text-area-actions-example',
    defaultValue: 'Some textarea value',
    actions: {
      iconButtons: [
        {
          label: 'Add image',
          icon: React.createElement(AddImage),
          onClick: () => {},
        },
        {
          label: 'Add file',
          icon: React.createElement(AddFile),
          onClick: () => {},
        },
      ],
      button: {
        label: 'Submit',
        onClick: () => {},
      },
    },
  },
};

export const WithActionsDisabled: Story = {
  args: {
    name: 'text-area-actions-disabled-example',
    defaultValue: 'Some textarea value',
    actions: {
      iconButtons: [
        {
          label: 'Add image',
          icon: React.createElement(AddImage),
          disabled: true,
        },
        {
          label: 'Add file',
          icon: React.createElement(AddFile),
          disabled: true,
        },
      ],
      button: {
        label: 'Submit',
        disabled: true,
      },
    },
  },
};

export const Error: Story = {
  args: {
    name: 'text-area-error-example',
    defaultValue: 'Invalid textarea value',
    error: 'This field contains an error.',
  },
};

export const ErrorWithActions: Story = {
  args: {
    name: 'text-area-error-actions-example',
    defaultValue: 'Invalid textarea value',
    error: 'This field contains an error.',
    actions: {
      iconButtons: [
        {
          label: 'Add image',
          icon: React.createElement(AddImage),
          onClick: () => {},
        },
        {
          label: 'Add file',
          icon: React.createElement(AddFile),
          onClick: () => {},
        },
      ],
      button: {
        label: 'Submit',
        onClick: () => {},
      },
    },
  },
};

export const Disabled: Story = {
  args: {
    name: 'text-area-disabled-example',
    disabled: true,
  },
};

export const DisabledWithValue: Story = {
  args: {
    name: 'text-area-disabled-value-example',
    defaultValue: 'Some textarea value',
    disabled: true,
  },
};
