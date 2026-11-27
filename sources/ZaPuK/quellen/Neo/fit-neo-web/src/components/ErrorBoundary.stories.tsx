// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Meta, StoryObj } from '@storybook/react-vite';
import React from 'react';
import { ErrorBoundary } from './ErrorBoundary';

const ThrowOnRender: React.FC = () => {
  throw new Error('Intentional render error for story');
};

const meta = {
  title: 'Components/ErrorBoundary',
  component: ErrorBoundary,
  tags: ['autodocs'],
  args: {
    fallback: (
      <div style={{ padding: 24, color: 'red' }}>
        Something went wrong. Please reload the page.
      </div>
    ),
  },
} satisfies Meta<typeof ErrorBoundary>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Normal: Story = {
  args: {
    children: <div style={{ padding: 24 }}>Content renders normally.</div>,
  },
};

export const WithError: Story = {
  args: {
    children: <ThrowOnRender />,
  },
};
