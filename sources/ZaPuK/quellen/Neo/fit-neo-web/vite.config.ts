// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

/// <reference types="vitest/config" />
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  optimizeDeps: {
    // Vite's esbuild-based dependency pre-bundler mishandles this
    // package's wasm-bindgen "bundler" target output in dev mode: it
    // throws "Cannot read properties of undefined (reading
    // '__wbindgen_externrefs')" because the pre-bundled chunk's module
    // evaluation order breaks the wasm module's internal `wasm` binding
    // setup. Excluding it from pre-bundling lets Vite serve it as a
    // real ES module instead, which resolves correctly.
    exclude: ['@matrix-g2x/neo-web-sdk'],
  },
  test: {
    exclude: [
      '**/node_modules/**',
      '**/dist/**',
      '**/storybook-static/**',
      '**/e2e-tests/**',
    ],
  },
  build: {
    target: 'esnext',
  },
});
