// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import './components/ui/elements';
import { createRoot } from 'react-dom/client';
import { errorMessage } from './utils/errors';

const root = document.getElementById('app');
if (!root) throw new Error('Application root is missing.');
try {
  const { App } = await import('./app/App');
  createRoot(root).render(<App />);
} catch (cause) {
  const message = document.createElement('p');
  message.setAttribute('role', 'alert');
  message.textContent = `Application could not start. Check config.json and reload. ${errorMessage(cause)}`;
  root.append(message);
}
