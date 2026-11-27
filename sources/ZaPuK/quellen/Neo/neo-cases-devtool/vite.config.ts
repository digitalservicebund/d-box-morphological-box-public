// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: CC0-1.0

import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite-plus';

export default defineConfig({
  plugins: [react()],
  build: { target: 'esnext' },
  worker: { format: 'es' },
  fmt: {
    singleQuote: true,
    printWidth: 120,
    trailingComma: 'all',
    semi: true,
    ignorePatterns: [
      'src/matrix/generated/**',
      'rust/**',
      '.tools/**',
      'dist/**',
      'charts/**/templates/**',
      // Keep Copier's tooling defaults formatted as in the upstream template.
      '.gitlab/merge_request_templates/**',
      '.markdownlint-cli2.jsonc',
      '.releaserc.json',
      'commitlint.config.mjs',
      'renovate.json5',
      'cspell.json',
    ],
  },
  lint: {
    ignorePatterns: ['src/matrix/generated/**', 'rust/**', '.tools/**', 'dist/**'],
    jsPlugins: [{ name: 'vite-plus', specifier: 'vite-plus/oxlint-plugin' }],
    rules: { 'vite-plus/prefer-vite-plus-imports': 'error' },
    options: { typeAware: true, typeCheck: true },
  },
  test: { include: ['src/**/*.test.ts'] },
});
