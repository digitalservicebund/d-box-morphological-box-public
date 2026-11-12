// SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: CC0-1.0

export default {
  extends: ["@commitlint/config-conventional"],
  rules: {
    "type-enum": [
      2,
      "always",
      [
        "build", // Changes that affect the build system or external dependencies
        "chore", // Miscellaneous changes that don’t modify src or test files
        "ci", // Changes to CI configuration files and scripts
        "docs", // Documentation only changes
        "feat", // A new feature
        "fix", // A bug fix
        "perf", // A code change that improves performance
        "refactor", // A code change that neither fixes a bug nor adds a feature
        "revert", // Reverts a previous commit
        "style", // Changes that do not affect the meaning of the code (formatting)
        "test", // Adding missing tests or correcting existing tests
      ],
    ],
    "scope-case": [2, "always", "kebab-case"], // e.g., feat(api-auth)
    "subject-case": [
      2,
      "never",
      ["sentence-case", "start-case", "pascal-case", "upper-case"], // lowercase preferred
    ],
    "subject-empty": [2, "never"],
    "subject-full-stop": [2, "never"],
    "header-max-length": [2, "always", 80],
  },
};
