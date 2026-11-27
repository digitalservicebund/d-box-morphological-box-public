// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { ClientProvider, ClientStore } from '@matrix-g2x/neo-web-sdk/react';
import type { Meta, StoryObj } from '@storybook/react-vite';
import { createElement } from 'react';
import BundIdHeader from './BundIdHeader';

const clientStore = new ClientStore({
  matrixHomeserver: 'https://matrix.example.org',
});

const meta = {
  title: 'Components/BundIdHeader',
  component: BundIdHeader,
  parameters: {
    layout: 'fullscreen',
  },
  decorators: [
    (Story) =>
      createElement(ClientProvider, {
        clientStore,
        // oxlint-disable-next-line react/no-children-prop
        children: createElement(Story),
      }),
  ],
  tags: ['autodocs'],
  argTypes: {
    userName: {
      control: 'text',
      description: 'Optional display-name override for the profile control',
    },
    onSearch: {
      action: 'search clicked',
      description: 'Called when the search button is clicked',
    },
    onUser: {
      action: 'user clicked',
      description: 'Called when the user button is clicked',
    },
    onLogout: {
      action: 'logout clicked',
      description: 'Called when Abmelden is selected',
    },
  },
} satisfies Meta<typeof BundIdHeader>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
};

export const LoggedIn: Story = {
  args: {
    userName: 'MAX MUSTERMANN',
  },
};

export const LongUserName: Story = {
  args: {
    userName: 'MAXIMILIAN MUSTERMANN-LÄNGERNACHNAME',
  },
};
