// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import { MemoryRouter } from 'react-router';
import TabBar from './TabBar';

const DEFAULT_TABS = [
  { to: '/', label: 'Vorgänge' },
  { to: '/documents', label: 'Dokumente' },
  { to: '/profile', label: 'Zugänge und Daten' },
  { to: '/settings', label: 'Kontoaktivitäten' },
];

const meta = {
  title: 'Components/Navigation/TabBar',
  component: TabBar,
  parameters: {
    layout: 'fullscreen',
  },
  tags: ['autodocs'],
  decorators: [
    (Story) => (
      <MemoryRouter initialEntries={['/']}>
        <Story />
      </MemoryRouter>
    ),
  ],
  argTypes: {
    tabs: {
      description: 'List of tab items with a route path and display label',
    },
  },
} satisfies Meta<typeof TabBar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    tabs: DEFAULT_TABS,
  },
};

export const ActiveDocuments: Story = {
  decorators: [
    (Story) => (
      <MemoryRouter initialEntries={['/documents']}>
        <Story />
      </MemoryRouter>
    ),
  ],
  args: {
    tabs: DEFAULT_TABS,
  },
};
