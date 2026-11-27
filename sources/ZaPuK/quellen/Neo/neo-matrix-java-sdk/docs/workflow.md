<!--
SPDX-FileCopyrightText: 2025, 2026 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Development Workflow

This project expects you to use a specific workflow that relies on existing
tools and conventions which we consider adequate for open source software
development.

## Tooling

Below we describe how to manage critical aspects of the development workflow,
as a way to streamline the licensing and copyright annotations and the
release of software artifacts, following the
[Semantic Versioning specification](https://semver.org).

### Licensing

We rely on [REUSE](https://reuse.software) to annotate license and copyright
information.

By default, FITKO projects use these licenses:

- [EUPL-1.2](https://interoperable-europe.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
  license for code assets.
- [CC-BY-SA-4.0](https://creativecommons.org/licenses/by-sa/4.0/legalcode)
  license for documentation assets.
- [CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/legalcode) license
  for configuration and auto-generated assets.

A typical REUSE copyright and license header will look like this:

```yaml
# SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)
# SPDX-License-Identifier: EUPL-1.2
```

As you create new files and assets in this repository, they must be either
annotated with a valid licensing comment header, or included in the
[REUSE.toml](../REUSE.toml) file, or a `.license` file must be created
for the corresponding file.

The [REUSE](https://reuse.software) tool will choose the best option for you.
Just run the following command:

```bash
reuse annotate --copyright="FITKO (Föderale IT-Kooperation)" \
  --merge-copyrights \
  --license="EUPL-1.2" <TARGET_FILE>
```

The `reuse-lint` CI step will fail if files are not correctly annotated.
To locally lint the licensing annotations for the repository, you can
[install REUSE](https://reuse.readthedocs.io/en/latest/readme.html#install)
and run:

```bash
reuse lint
```

### Other licenses

If you are adding code with different licenses, please include them in the
headers of the affected files and run:

```bash
reuse download --all
```

so that the [LICENSES](../LICENSES/) folder is updated with the license's
full text.

### Conventional Commits

Commit messages must adhere to the
[Conventional Commit](https://www.conventionalcommits.org/en/v1.0.0/#summary)
standard. Commits that do not adhere to the standard get rejected by the CI
step `commit-lint`.

Commit messages are an essential part of the Release Notes and have
direct impact on the semantic versioning of the repository artifacts.
You can follow [this guide](https://cbea.ms/git-commit/) on how to write
optimized git commit messages.
See below for more information.

### Semantic Releases

This project includes a
[semantic-release](https://semantic-release.gitbook.io/semantic-release/)
setup that automatically:

- Analyzes commit messages to determine the next version number
  (major, minor or patch).
- Generates release notes from conventional commits.
- Creates Git tags and GitLab releases.
- Updates the `CHANGELOG.md` file.

#### Release Types

Based on your commit messages, `semantic-release` will create:

- **Patch releases** (1.0.1): Bug fixes (`fix:` commits)
- **Minor releases** (1.1.0): New features (`feat:` commits)
- **Major releases** (2.0.0): Breaking changes (commits with
  `BREAKING CHANGE:` in footer, or `feat!:` commits)

All commit messages starting with other types, such as  `chore:`, `docs:`,
`ci:`, are ignored and not included in the changelog.

Examples:

- `fix: resolve authentication issue` → Patch release
- `feat: add user management module` → Minor release
- `feat!: redesign API` → Major release
- `fix: update dependency` with `BREAKING CHANGE:` in footer → Major release
- `docs: update README.md` → No release and excluded from next release changelog

The currently enabled types for this repo can be found alongside a short
description about when to use them in the tool's [configuration
file](../commitlint.config.mjs).

## Git Workflow

In this section, we describe the expected Git workflow that should be used
by the developers and which is supported by the release automation.

### Branches

Two protected branches must exist, `main` and `develop`, with `develop`
configured to be the _default_ branch.

When creating new features or adding fixes, follow these steps:

1. Create a new, short-lived, feature branch off `develop`.
2. Do your changes, commit and push them to this new branch.
3. Open a Merge Request from the feature branch to `develop`.
4. If pipelines succeed and the MR is approved (after review), merge
  into `develop`.

This is essentially a `trunk-based` strategy but based on the `develop` branch.

### Branch naming

Branches created from the `develop` branch have to adhere to the following
notation: `<developer_username>`/`<type>`/`<details>`:

- `<developer_username>`: Prefix the branch with your username without the `@` symbol.

- `<type>`: From the list of types used in the conventional commit messages:
  - `chore`, `docs`
  - `fix`
  - `feat`

- `<details>`: A very short note about what this branch is about.

Here is an example: `holz/feat/add-markdown-linting`

### Peer Reviews

To assign fine-grained Editing or Review Tasks within a MR, use GitLab's
built-in annotation feature, mentioning the requested person by username so
they receive a notification:
<https://docs.gitlab.com/user/project/merge_requests/changes/>.

The assignee of a
[ticket](https://gitlab.opencode.de/fitko/matrix-g2x/planning/-/issues/) is
ultimately responsible for ensuring an associated Merge Request gets merged or
otherwise resolved, including reaching out to team members for feedback etc.

If a review requires adjustments via additional commits, you are encouraged to
use [`git commit
--fixup`](https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---fixupamendrewordcommit)
to mark the adjustment's relationship to an original commit.
Once the MR is approved, if not squashing the whole MR, the fixup commits can
be automatically squashed using `git rebase main --interactive --autosquash`.

### Releasing

To trigger a release, just merge to `main`. The following process is expected:

1. The `develop` branch is ready for release.
2. Open a Merge Request from `develop` to `main`.
3. If pipelines succeed and the MR is approved (after review),
  merge into `main`.
4. The `release` CI workflow will be executed and a new release and tag
  will be created in the repository with the notes corresponding to the commit
  log history since the last release.

Then we need to merge `main` back into `develop`, in order to keep them in sync.

1. Open a Merge Request from `main` to `develop`.
2. If pipelines succeed and the MR is approved (after review),
  merge into `develop`.

Now you can start again branching off `develop` for new features.

#### **IMPORTANT**

The automatic release process requires adding a **Personal Access
Token** to the CI/CD variables. Follow
[these instructions](https://docs.gitlab.com/user/profile/personal_access_tokens/)
to create one with the `api` and `write_repository` scopes.

Then follow [this guide](https://docs.gitlab.com/ci/variables/#for-a-project)
to add it using the `GITLAB_TOKEN` key, set it to `Protect variable` and
`Masked and hidden`.

#### Release Configuration

You can adjust the semantic release configuration by editing the
[.releaserc.json](../.releaserc.json) file.

### Merges and commit history

For a clean commit history on all branches, we expect that no merge commits
are created and the merge method is set to `fast-forward merge`.

Squashing merge request commits is allowed but not preferred.

## Standard Quality Gates

We include a minimal set of standard quality gates that address some quality
assurance concerns and are executed as CI pipelines on every Merge Request.

- `reuse-lint` - this step runs the [REUSE](https://reuse.software) tool
  against the repository files. It will fail if there are files which are not
  correctly annotated.

- `commit-lint` - this step uses the [commitlint](https://commitlint.js.org)
  tool to ensure all commit messages for the Merge Request follow the
  conventional commits rules. The configuration can be adjusted
  by editing the [commitlint.config.js](../commitlint.config.js) file.

- `markdown-lint` - this step runs the
  [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2) tool
  against the repository's Markdown files. The configuration can be adjusted
  by editing the [.markdownlint-cl2.jsonc](../.markdownlint-cl2.jsonc) file.

- `markdown-spellcheck` - this step runs the [cspell](https://cspell.org) tool
  to perform spell checking on all markdown files in the repository. The
  configuration can be adjusted by editing the [cspell.json](../cspell.json) file.

The failure of any of these steps will block the merging of the Merge Request.

### Git Hooks

These quality gates are also enforced locally, via
[git-hooks](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks).
Git commits and pushes will fail if the quality gates are not successful.

- `pre-commit`: License linting, markdown linting and spell-checking
  are performed in the `pre-commit` stage.

- `commit-msg`: The conventional commit linter runs in the `commit-msg` stage.

If the required tools are not installed, the hook actions will be skipped.

For instructions on how to install them please refer to their official
websites:

- [REUSE](https://reuse.software) for license linting
- [markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2)
for Markdown linting
- [cspell](https://cspell.org) for markdown spell-checking
- [commitlint](https://commitlint.js.org) for commit linting

## Setting up a local development environment

Please review the following section for additional steps to perform after
checking out this repository:

### Enable Git Hooks

To enable git hooks for this repo, run:

```sh
git config --local core.hooksPath .githooks
```

on the repository root folder.
