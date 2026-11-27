// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import Badge from './Badge';

const meta = {
  title: 'Components/Information/Badge',
  component: Badge,
  parameters: {
    layout: 'fullscreen',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof Badge>;
export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    title: 'Default badge title',
    type: 'info',
  },
};

export const Info: Story = {
  args: {
    title: 'Info badge title',
    type: 'info',
  },
};
export const Success: Story = {
  args: {
    title: 'Success badge title',
    type: 'success',
  },
};

export const Warning: Story = {
  args: {
    title: 'Warning badge title',
    type: 'warning',
  },
};

export const Error: Story = {
  args: {
    title: 'Error badge title',
    type: 'error',
  },
};

export const Neutral: Story = {
  args: {
    title: 'Neutral badge title',
    type: 'neutral',
  },
};

export const LargeInfo: Story = {
  args: {
    title: 'Large info badge title',
    type: 'info',
    large: {
      text: 'Large info badge text',
      learnMoreLink: '',
    },
  },
};

export const LargeSuccess: Story = {
  args: {
    title: 'Large success badge title',
    type: 'success',
    large: {
      text: 'Large success badge text',
      learnMoreLink: '',
    },
  },
};

export const LargeWarning: Story = {
  args: {
    title: 'Large warning badge title',
    type: 'warning',
    large: {
      text: 'Large warning badge text',
      learnMoreLink: '',
    },
  },
};

export const LargeError: Story = {
  args: {
    title: 'Large error badge title',
    type: 'error',
    large: {
      text: 'Large error badge text',
      learnMoreLink: '',
    },
  },
};

export const LargeNeutral: Story = {
  args: {
    title: 'Large neutral badge title',
    type: 'neutral',
    large: {
      text: 'Large neutral badge text',
      learnMoreLink: '',
    },
  },
};
