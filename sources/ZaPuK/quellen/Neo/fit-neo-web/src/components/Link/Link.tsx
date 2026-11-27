// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import type { PropsWithChildren, ReactNode } from 'react';
import { Link as RouterLink } from 'react-router';
import ArrowLeft from '../Icons/ArrowLeft';
import ArrowRight from '../Icons/ArrowRight';
import ExternalLinkIndicator from '../Icons/ExternalLinkIndicator';
import './link.css';

export interface LinkProps {
  left?: boolean;
  href: string;
  icon?: ReactNode;
  className?: string;
}

export default function Link({
  href,
  children,
  left,
  icon,
  className,
}: PropsWithChildren<LinkProps>) {
  const isExternal = href.startsWith('http') || href.startsWith('https');

  const resolvedIcon =
    icon ??
    (isExternal ? (
      <ExternalLinkIndicator />
    ) : left ? (
      <ArrowLeft />
    ) : (
      <ArrowRight />
    ));

  const linkClass = ['link', className].filter(Boolean).join(' ');

  if (isExternal) {
    return (
      <a className={linkClass} href={href} target="_blank" rel="noreferrer">
        {resolvedIcon}
        {children}
      </a>
    );
  }

  return (
    <RouterLink className={linkClass} to={href}>
      {resolvedIcon}
      {children}
    </RouterLink>
  );
}
