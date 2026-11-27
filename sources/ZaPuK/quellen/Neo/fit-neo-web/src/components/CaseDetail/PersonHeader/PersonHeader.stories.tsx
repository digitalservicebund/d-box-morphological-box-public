// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import PersonHeader from './PersonHeader';

const meta = {
  title: 'Components/CaseDetail/PersonHeader',
  component: PersonHeader,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof PersonHeader>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    initials: 'PM',
    name: 'Peter Müller',
    orgName: 'Zentrale Zulassungsstelle für Drachenhaltung',
    websiteHref: 'https://example.com',
  },
};

export const WithoutOrg: Story = {
  args: {
    initials: 'PM',
    name: 'Peter Müller',
  },
};
