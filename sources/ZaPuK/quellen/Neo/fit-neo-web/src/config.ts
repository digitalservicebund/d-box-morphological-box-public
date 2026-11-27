// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

export interface OidcConfig {
  /** The name shown to users during OIDC authorization. Defaults to 'Meine Digitale Verwaltung'. */
  clientName?: string;
  /** URI of this client, shown during OIDC authorization. Defaults to window.location.origin. */
  clientUri?: string;
}

export interface AppConfig {
  matrixHomeserver: string;
  oidc?: OidcConfig;
}

let appConfig: AppConfig | null = null;

export async function loadConfig(): Promise<void> {
  const res = await fetch('/config.json');
  if (!res.ok) {
    throw new Error(
      `Failed to load config.json: ${res.status} ${res.statusText}`,
    );
  }
  appConfig = await res.json();
}

export function getConfig(): AppConfig {
  if (!appConfig) throw new Error('getConfig() called before loadConfig()');
  return appConfig!;
}
