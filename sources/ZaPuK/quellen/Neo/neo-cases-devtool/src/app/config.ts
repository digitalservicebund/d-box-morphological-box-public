// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

import { loadRuntimeConfig } from './runtime-config';

export const config = await loadRuntimeConfig(`${import.meta.env.BASE_URL}config.json`);
