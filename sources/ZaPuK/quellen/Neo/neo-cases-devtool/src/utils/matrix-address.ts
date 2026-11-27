// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

/** Returns the Matrix server name, preserving ports and IPv6 literals. */
export const mxidDomain = (mxid: string): string => {
  const separator = mxid.indexOf(':');
  return separator > 1 && mxid.startsWith('@') ? mxid.slice(separator + 1) : '';
};

/** Completes a local recipient using the logged-in MXID, never the homeserver's HTTP hostname. */
export const completeMxid = (input: string, ownMxid: string): string => {
  const value = input.trim();
  if (!value) return '';
  const id = value.startsWith('@') ? value : `@${value}`;
  const separator = id.indexOf(':');
  if (separator >= 0 && separator < id.length - 1) return id;
  const local = separator === id.length - 1 ? id.slice(0, -1) : id;
  const domain = mxidDomain(ownMxid);
  return domain ? `${local}:${domain}` : local;
};

export const homeserverDomain = (url: string): string => new URL(url).host;
