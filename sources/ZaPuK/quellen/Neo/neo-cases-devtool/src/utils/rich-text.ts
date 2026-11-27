// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import DOMPurify from 'dompurify';
import { object } from '../domain/neo/relations';

export const sanitizeHtml = (html: string): string =>
  DOMPurify.sanitize(html, {
    ALLOWED_TAGS: [
      'p',
      'br',
      'b',
      'strong',
      'i',
      'em',
      'u',
      's',
      'del',
      'blockquote',
      'code',
      'pre',
      'a',
      'ul',
      'ol',
      'li',
      'hr',
      'h1',
      'h2',
      'h3',
      'h4',
      'h5',
      'h6',
      'table',
      'thead',
      'tbody',
      'tr',
      'th',
      'td',
      'span',
    ],
    ALLOWED_ATTR: ['href', 'title', 'colspan', 'rowspan'],
    ALLOW_DATA_ATTR: false,
  });

export const textRepresentations = (value: unknown): { plain: string; html?: string } => {
  const items = object(value)['m.text'];
  if (!Array.isArray(items)) return { plain: '' };
  const representations = items.map(object);
  const plain = representations.find((item) => !item.mimetype || item.mimetype === 'text/plain')?.body;
  const html = representations.find((item) => item.mimetype === 'text/html')?.body;
  return { plain: typeof plain === 'string' ? plain : '', html: typeof html === 'string' ? html : undefined };
};
