// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { PropsWithChildren } from 'react';
import './button-row.css';

export interface ButtonRowProps {
  align?: 'spaced' | 'end';
}

export default function ButtonRow({
  align = 'end',
  children,
}: PropsWithChildren<ButtonRowProps>) {
  return <div className={`button-row button-row-${align}`}>{children}</div>;
}
