// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { memo } from 'react';
import './loading-spinner.css';

interface LoadingSpinnerProps {
  text: string;
  inline?: boolean;
}

function LoadingSpinner({ text, inline }: LoadingSpinnerProps) {
  return (
    <div
      className={`loading-spinner${inline ? ' loading-spinner-inline' : ''}`}
      role="status"
      aria-live="polite"
    >
      <span className="loading-spinner-indicator" aria-hidden="true" />
      <span className="loading-spinner-text">{text}</span>
    </div>
  );
}

const MemoizedLoadingSpinner = memo(LoadingSpinner);
export { MemoizedLoadingSpinner as LoadingSpinner };
