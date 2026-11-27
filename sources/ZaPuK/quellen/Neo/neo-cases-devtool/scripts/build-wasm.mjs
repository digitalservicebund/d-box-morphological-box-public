// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { resolve } from 'node:path';

const root = resolve(import.meta.dirname, '..');
const manifest = 'rust/matrix-web/Cargo.toml';
const output = 'src/matrix/generated';
const version = '0.2.129';
const run = (command, args) => {
  const result = spawnSync(command, args, { cwd: root, stdio: 'inherit' });
  if (result.error || result.status !== 0) {
    throw result.error ?? new Error(`${command} failed (${result.status})`);
  }
};

// Skip the expensive Rust build when the pinned source and artifacts match.
const fingerprint = createHash('sha256');
for (const file of [manifest, 'rust/matrix-web/Cargo.lock', 'rust/matrix-web/src/lib.rs']) {
  if (existsSync(resolve(root, file))) fingerprint.update(readFileSync(resolve(root, file)));
}
const hash = fingerprint.digest('hex');
const stamp = resolve(root, output, '.source-hash');
const artifacts = ['matrix.js', 'matrix.d.ts', 'matrix_bg.wasm'];
if (
  !existsSync(stamp) ||
  readFileSync(stamp, 'utf8') !== hash ||
  artifacts.some((file) => !existsSync(resolve(root, output, file)))
) {
  const local = resolve(root, '.tools/bin/wasm-bindgen');
  const installed = spawnSync(local, ['--version'], { encoding: 'utf8' });
  if (!installed.stdout?.includes(version)) {
    run('cargo', ['install', 'wasm-bindgen-cli', '--version', version, '--locked', '--root', '.tools']);
  }
  run('cargo', ['build', '--locked', '--manifest-path', manifest, '--target', 'wasm32-unknown-unknown', '--release']);
  mkdirSync(resolve(root, output), { recursive: true });
  run(local, [
    'rust/matrix-web/target/wasm32-unknown-unknown/release/neo_cases_devtool.wasm',
    '--target',
    'web',
    '--out-dir',
    output,
    '--out-name',
    'matrix',
  ]);
  writeFileSync(stamp, hash);
}
