// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import Infobox from './Infobox';

const meta = {
  title: 'Components/Information/Infobox',
  component: Infobox,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof Infobox>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    title: 'Infobox',
    body: 'Hier gibt es einen Link mit dazugehörigem Erklärungstext.',
    link: {
      href: 'https://example.com',
      label: 'Link',
    },
  },
};

export const WithoutLink: Story = {
  args: {
    title: 'Vorgang abgeschlossen',
    body: 'Melden Sie sich per Email oder Telefon bei Ihrer Behörde, um ihren Vorgang zu besprechen.',
  },
};
