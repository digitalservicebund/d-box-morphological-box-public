// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

/** @type {import("stylelint").Config} */
export default {
  extends: ['stylelint-config-standard'],
  plugins: ['stylelint-no-unsupported-browser-features'],
  rules: {
    'plugin/no-unsupported-browser-features': [
      true,
      {
        browsers: ['baseline widely available'],
        // These seems to cause false positives
        ignore: ['css-nesting', 'intrinsic-width'],
      },
    ],
  },
};
