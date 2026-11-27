// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

const i18NextParserConfig = {
  locales: ['en', 'de'],
  output: 'public/locales/$LOCALE/$NAMESPACE.json',
  sort: true,
  resetDefaultValueLocale: 'en',
  lexers: {
    tsx: [{ lexer: 'JsxLexer', transSupportBasicHtmlNodes: true }],
  },
};

export default i18NextParserConfig;
