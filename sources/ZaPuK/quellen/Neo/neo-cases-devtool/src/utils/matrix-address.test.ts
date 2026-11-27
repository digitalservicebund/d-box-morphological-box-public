// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { expect, it } from 'vite-plus/test';
import { completeMxid, homeserverDomain, mxidDomain } from './matrix-address';
import { recipientSchema } from '../domain/neo/validation';

it('completes both localpart forms using the account domain', () => {
  expect(completeMxid(' citizen ', '@agency:users.example.org')).toBe('@citizen:users.example.org');
  expect(completeMxid('@citizen', '@agency:users.example.org')).toBe('@citizen:users.example.org');
  expect(completeMxid('@citizen:', '@agency:users.example.org')).toBe('@citizen:users.example.org');
  expect(completeMxid('@agency', '@agency:users.example.org')).toBe('@agency:users.example.org');
});

it('preserves explicit domains, ports, and IPv6 literals', () => {
  expect(completeMxid('@citizen:other.example.org', '@agency:local.example.org')).toBe('@citizen:other.example.org');
  expect(completeMxid('citizen:other.example.org:8448', '@agency:local.example.org')).toBe(
    '@citizen:other.example.org:8448',
  );
  expect(mxidDomain('@agency:[::1]:8448')).toBe('[::1]:8448');
  expect(completeMxid('@citizen', '@agency:[::1]:8448')).toBe('@citizen:[::1]:8448');
});

it('does not invent a recipient for empty input or an unknown account domain', () => {
  expect(completeMxid(' ', '@agency:example.org')).toBe('');
  expect(completeMxid('@citizen', '')).toBe('@citizen');
  expect(homeserverDomain('https://synapse.example.org:8448/matrix/')).toBe('synapse.example.org:8448');
});

it('validates completed invitation recipients and rejects empty or malformed input', () => {
  const validate = (input: string) => recipientSchema.safeParse(completeMxid(input, '@agency:example.org'));
  for (const input of ['citizen', '@citizen', '@citizen:other.example.org']) expect(validate(input).success).toBe(true);
  for (const input of ['', '  ', '@', 'two users', '@:example.org']) expect(validate(input).success).toBe(false);
});
