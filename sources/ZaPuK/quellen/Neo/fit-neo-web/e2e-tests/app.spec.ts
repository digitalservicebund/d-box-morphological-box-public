// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, test } from '@playwright/test';

test('unauthenticated visit to / redirects to /login', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveURL(/\/login/);
});

test('login page renders the sign-in button', async ({ page }) => {
  await page.goto('/login');
  // Wait for WASM and React to finish initialising before asserting.
  await page.waitForLoadState('networkidle');
  // The login page has exactly one primary action button (the sign-in CTA).
  await expect(page.locator('button.primary')).toBeVisible();
});

test('login page renders without crashing', async ({ page }) => {
  await page.goto('/login');
  // No error boundary fallback should be shown
  await expect(page.locator('body')).not.toContainText('Configuration error');
});

test('login page initializes SDK client for session restoration', async ({
  page,
}) => {
  await page.goto('/login');
  const result = await page.evaluate(async () => {
    const { createSessionClientBuilder } =
      await import('/e2e-tests/fixtures/sessionHarness.ts');
    return createSessionClientBuilder();
  });
  expect(result).toBe(true);
});
