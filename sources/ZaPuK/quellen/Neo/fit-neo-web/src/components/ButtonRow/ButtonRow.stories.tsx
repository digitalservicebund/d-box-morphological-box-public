// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import Button from '../Button/Button';
import ArrowRight from '../Icons/ArrowRight';
import ButtonRow from './ButtonRow';

const meta = {
  title: 'Components/Layout/ButtonRow',
  component: ButtonRow,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
  argTypes: {
    align: {
      control: 'radio',
      options: ['spaced', 'end'],
      description: 'How the buttons are justified within the row',
    },
  },
} satisfies Meta<typeof ButtonRow>;

export default meta;
type Story = StoryObj<typeof meta>;

export const End: Story = {
  args: {
    align: 'end',
    children: (
      <>
        <Button type="secondary" icon={<ArrowRight />}>
          Abbrechen
        </Button>
        <Button type="primary" icon={<ArrowRight />}>
          Senden
        </Button>
      </>
    ),
  },
};

export const Spaced: Story = {
  args: {
    align: 'spaced',
    children: (
      <>
        <Button type="secondary" icon={<ArrowRight />}>
          Abbrechen
        </Button>
        <Button type="primary" icon={<ArrowRight />}>
          Senden
        </Button>
      </>
    ),
  },
};
