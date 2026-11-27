// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { Preview } from '@storybook/react-vite';
import '../src/css/fonts.css';
import '../src/css/index.css';
import '../src/css/reset.css';
import '../src/css/theme.css';

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },

    backgrounds: {
      options: {
        light: { name: 'Light', value: '#f2f3f4' },
      },
    },

    // Alllows us to fail on CI
    a11y: { test: 'error' },
  },
  initialGlobals: {
    background: { value: 'light' },
  },
};

export default preview;
