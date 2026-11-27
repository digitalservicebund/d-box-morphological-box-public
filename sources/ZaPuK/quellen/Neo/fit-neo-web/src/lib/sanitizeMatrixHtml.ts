// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import DOMPurify from 'dompurify';

// https://spec.matrix.org/v1.18/client-server-api/#mroommessage-msgtypes
const ALLOWED_TAGS = [
  'del',
  'h1',
  'h2',
  'h3',
  'h4',
  'h5',
  'h6',
  'blockquote',
  'p',
  'a',
  'ul',
  'ol',
  'sup',
  'sub',
  'li',
  'b',
  'i',
  'u',
  'strong',
  'em',
  's',
  'code',
  'hr',
  'br',
  'div',
  'table',
  'thead',
  'tbody',
  'tr',
  'th',
  'td',
  'caption',
  'pre',
  'span',
  'img',
  'details',
  'summary',
] as const;

const ELEMENT_ATTRS: Record<string, string[]> = {
  span: [
    'data-mx-bg-color',
    'data-mx-color',
    'data-mx-spoiler',
    'data-mx-maths',
  ],
  a: ['href', 'target'],
  img: ['width', 'height', 'alt', 'title', 'src'],
  ol: ['start'],
  code: ['class'],
  div: ['data-mx-maths'],
};

const ATTR_ELEMENTS: Record<string, string[]> = {};
for (const [el, attrs] of Object.entries(ELEMENT_ATTRS)) {
  for (const attr of attrs) {
    (ATTR_ELEMENTS[attr] ??= []).push(el);
  }
}

const ALLOWED_ATTR_NAMES = Object.keys(ATTR_ELEMENTS);
const HREF_SCHEME = /^(https?|ftp|mailto|magnet|matrix|geo):/i;

function isAllowedAttributeValue(
  tag: string,
  attribute: string,
  value: string,
): boolean {
  if (tag === 'a' && attribute === 'href') return HREF_SCHEME.test(value);
  if (tag === 'img' && attribute === 'src') return value.startsWith('mxc://');
  if (tag === 'code' && attribute === 'class') {
    return value.split(' ').every((part) => part.startsWith('language-'));
  }
  return true;
}

function postProcessDom(root: HTMLDivElement): void {
  for (const a of root.querySelectorAll('a')) {
    const href = a.getAttribute('href');
    if (href !== null && !isAllowedAttributeValue('a', 'href', href)) {
      a.removeAttribute('href');
    }
    if (a.hasAttribute('target')) {
      a.setAttribute('target', '_blank');
      a.setAttribute('rel', 'noopener noreferrer');
    }
  }
  for (const img of root.querySelectorAll('img')) {
    const src = img.getAttribute('src');
    if (src !== null && !isAllowedAttributeValue('img', 'src', src)) {
      img.removeAttribute('src');
    }
  }
  for (const code of root.querySelectorAll('code')) {
    const cls = code.getAttribute('class');
    if (cls !== null && !isAllowedAttributeValue('code', 'class', cls)) {
      code.removeAttribute('class');
    }
  }
}

interface ElementWithSetHTML extends HTMLDivElement {
  setHTML(html: string, options: { sanitizer: Sanitizer | null }): void;
}

// The native sanitizer is used when both browser APIs are available.
const hasSanitizerAPI =
  typeof HTMLDivElement !== 'undefined' &&
  typeof (HTMLDivElement.prototype as ElementWithSetHTML).setHTML ===
    'function' &&
  typeof Sanitizer === 'function';

const sanitizerInstance: Sanitizer | null = hasSanitizerAPI
  ? new Sanitizer({
      elements: ALLOWED_TAGS.map((name) => ({
        name,
        attributes: ELEMENT_ATTRS[name] ?? [],
      })),
      comments: false,
    })
  : null;

function sanitizeWithSanitizerAPI(html: string): string {
  const tmp = document.createElement('div') as ElementWithSetHTML;
  tmp.setHTML(html, {
    sanitizer: sanitizerInstance,
  });
  postProcessDom(tmp);
  return tmp.innerHTML;
}

DOMPurify.addHook('uponSanitizeAttribute', (node, data) => {
  const tag = node.tagName.toLowerCase();
  const allowedElements = ATTR_ELEMENTS[data.attrName];

  if (
    !allowedElements?.includes(tag) ||
    !isAllowedAttributeValue(tag, data.attrName, data.attrValue)
  ) {
    data.keepAttr = false;
    return;
  }

  if (tag === 'a' && data.attrName === 'target') {
    data.attrValue = '_blank';
  }
});

DOMPurify.addHook('afterSanitizeAttributes', (node) => {
  if (node.tagName === 'A' && node.hasAttribute('target')) {
    node.setAttribute('rel', 'noopener noreferrer');
  }
});

function sanitizeWithDOMPurify(html: string): string {
  return DOMPurify.sanitize(html, {
    ALLOWED_TAGS: [...ALLOWED_TAGS],
    ALLOWED_ATTR: ALLOWED_ATTR_NAMES,
    ALLOWED_URI_REGEXP:
      /^(?:(?:(?:f|ht)tps?|mailto|tel|callto|sms|cid|xmpp|matrix|magnet|mxc|geo):|[^a-z]|[a-z+.-]+(?:[^a-z+.-:]|$))/i,
  }) as string;
}

const cache = new Map<string, string>();

export function sanitizeMatrixHtml(html: string): string {
  const cached = cache.get(html);
  if (cached !== undefined) return cached;

  const result = hasSanitizerAPI
    ? sanitizeWithSanitizerAPI(html)
    : sanitizeWithDOMPurify(html);

  cache.set(html, result);
  return result;
}
