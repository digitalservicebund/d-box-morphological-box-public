// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, test, type Page } from '@playwright/test';

const html = [
  '<a href="https://example.com" target="_self">safe link</a>',
  '<a href="geo:52.5,13.4">location link</a>',
  '<a href="javascript:alert(1)">unsafe link</a>',
  '<img src="mxc://server/id" alt="safe image">',
  '<img src="geo:52.5,13.4" alt="unsafe location image">',
  '<img src="https://example.com/image.png" alt="unsafe image">',
  '<code class="language-ts">safe code</code>',
  '<code class="language-ts other">unsafe class</code>',
  '<span data-mx-color="#abc" data-test="bad">colored text</span>',
  '<script>alert(1)</script>',
].join('');

async function inspectSanitizedHtml(page: Page) {
  return page.evaluate(async (input) => {
    const { sanitizeMatrixHtml } =
      await import('/src/lib/sanitizeMatrixHtml.ts');
    const root = document.createElement('div');
    root.innerHTML = sanitizeMatrixHtml(input);
    return {
      links: Array.from(root.querySelectorAll('a'), (link) => ({
        href: link.getAttribute('href'),
        target: link.getAttribute('target'),
        rel: link.getAttribute('rel'),
      })),
      images: Array.from(root.querySelectorAll('img'), (image) =>
        image.getAttribute('src'),
      ),
      classes: Array.from(root.querySelectorAll('code'), (code) =>
        code.getAttribute('class'),
      ),
      span: {
        color: root.querySelector('span')?.getAttribute('data-mx-color'),
        extra: root.querySelector('span')?.getAttribute('data-test'),
      },
      scripts: root.querySelectorAll('script').length,
    };
  }, html);
}

function expectAllowedValues(
  result: Awaited<ReturnType<typeof inspectSanitizedHtml>>,
  mxcSource: string | null,
) {
  expect(result.links).toEqual([
    {
      href: 'https://example.com',
      target: '_blank',
      rel: 'noopener noreferrer',
    },
    { href: 'geo:52.5,13.4', target: null, rel: null },
    { href: null, target: null, rel: null },
  ]);
  expect(result.images).toEqual([mxcSource, null, null]);
  expect(result.classes).toEqual(['language-ts', null]);
  expect(result.span).toEqual({ color: '#abc', extra: null });
  expect(result.scripts).toBe(0);
}

test('DOMPurify validates Matrix HTML values', async ({ page }) => {
  await page.addInitScript(() => {
    Object.defineProperty(HTMLDivElement.prototype, 'setHTML', {
      configurable: true,
      value: undefined,
    });
  });
  await page.goto('/login');
  expectAllowedValues(await inspectSanitizedHtml(page), 'mxc://server/id');
});

test('native Sanitizer validates Matrix HTML values', async ({
  page,
  browserName,
}) => {
  test.skip(browserName !== 'chromium');
  await page.goto('/login');
  expect(
    await page.evaluate(
      () =>
        typeof Sanitizer === 'function' &&
        'setHTML' in HTMLDivElement.prototype,
    ),
  ).toBe(true);
  expectAllowedValues(await inspectSanitizedHtml(page), 'mxc://server/id');
});
