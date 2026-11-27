// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import AttachmentsSection from './AttachmentsSection';

const meta = {
  title: 'Components/CaseDetail/AttachmentsSection',
  component: AttachmentsSection,
  parameters: {
    layout: 'fullscreen',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof AttachmentsSection>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    attachments: [
      {
        id: '1',
        name: 'bestaetigung.pdf',
        size: BigInt(1_200_000),
        uri: 'mxc://example.com/abc123',
      },
      {
        id: '2',
        name: 'foto.png',
        size: BigInt(300_000),
        uri: 'mxc://example.com/def456',
      },
    ],
  },
};
