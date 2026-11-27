// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { readFile } from 'node:fs/promises';
import { expect, test } from '@playwright/test';

test('client media requests use a validated MXC source', async ({ page }) => {
  await page.goto('/login');
  const result = await page.evaluate(async () => {
    const { getClientMediaContent } =
      await import('/e2e-tests/fixtures/mediaHarness.tsx');
    return getClientMediaContent('mxc://server/file');
  });
  expect(result).toEqual({
    sourceUrl: 'mxc://server/file',
    bytes: [1, 2, 3],
  });
});

test('attachment downloads use media content bytes', async ({ page }) => {
  await page.goto('/login');
  await page.evaluate(async () => {
    const { renderAttachment } =
      await import('/e2e-tests/fixtures/mediaHarness.tsx');
    renderAttachment();
  });

  const downloadPromise = page.waitForEvent('download');
  await page.getByRole('button', { name: 'report.txt' }).click();
  const download = await downloadPromise;
  expect(download.suggestedFilename()).toBe('report.txt');
  expect(await readFile(await download.path(), 'utf8')).toBe('file contents');
});

test('formatted MXC images use media content bytes', async ({ page }) => {
  await page.goto('/login');
  await page.evaluate(async () => {
    const { renderFormattedImage } =
      await import('/e2e-tests/fixtures/mediaHarness.tsx');
    renderFormattedImage();
  });

  await expect(page.locator('.infobox-body img')).toHaveAttribute(
    'src',
    /^blob:/,
  );
  expect(await page.locator('html').getAttribute('data-media-uri')).toBe(
    'mxc://server/image',
  );
});
