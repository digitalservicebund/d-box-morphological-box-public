<!--
SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Neo Project Template

## About

This project is a template for creating a new (software) project
within the Matrix G2C communication pilot (Neo).
It is intended to be used as a starting point for new projects,
and is not intended to be used as a production-ready project.

It is made available publicly as open source software in the hope that it may
also be useful for reuse outside of this project.

## What's included

This project template includes a set of default assets that should be part
of new software projects created within the scope of the Neo project.

It includes:

- A [LICENSE](./LICENSE) file and [LICENSES](./LICENSES/) folder which includes
  the software license's full-text under which this project is published
- a minimal, stack-agnostic, [.gitignore](./.gitignore) that new projects
  should extend from.
- A [.gitlab-ci.yaml](./.gitlab-ci.yml) file with some quality gates
  and automation for software releases.
- A [CHANGELOG.md](./CHANGELOG.md) where the release history is kept. The
  contents are [automatically updated](./docs/workflow.md#releasing)
  by the `semantic-release` process.
- A [CONTRIBUTING.md](./CONTRIBUTING.md) file that details how you can
  contribute to this project.
- A [publiccode.yml](./publiccode.yml) file for indexing projects in
  the [EU Open Source Solutions Catalogue](https://interoperable-europe.ec.europa.eu/eu-oss-catalogue).
- A [REUSE.toml](./REUSE.toml) file to annotate licenses and copyright
  information for assets that can't be inline header annotated.
- A [SECURITY.md](./SECURITY.md) file with project-wide security disclosure
  information.
- An [.editorconfig](./.editorconfig) file with defaults for defining and
  maintain consistent coding styles. It follows the `EditorConfig` format
  and you can check if your IDE automatically supports it or if you require a
  plugin on their [official website](https://editorconfig.org).

and assorted configuration files for the included tools.

New projects created from this template include:

- **Automated Releases**: Using [semantic-release](https://semantic-release.gitbook.io/semantic-release/)
  for automated versioning and changelog generation.
- **Conventional Commits**: Enforcing a consistent [commit message format](https://www.conventionalcommits.org/)
  for automated release notes.
- **GitLab CI/CD**: Ready-to-use pipeline configuration for continuous
  integration with basic standard quality gates.
- **License Compliance**: [REUSE](https://reuse.software)-compliant licensing setup.
- **Documentation linting and spell-checking**: includes pre-commit hooks and
  CI steps for linting and spell-checking markdown files.
- **EditorConfig defaults**: for maintaining [consistent coding styles](https://editorconfig.org)
  across multiple IDEs.

## How to use it

You can start a new project with this template by using Copier.

[Copier](https://copier.readthedocs.io/en/stable/) is a tool and library for
defining reusable project templates. It can be used to create a new project
based on this template.

Follow the [installation](https://copier.readthedocs.io/en/stable/#installation)
instructions to set it up locally and then:

```sh
copier copy \
  git@gitlab.opencode.de:fitko/matrix-g2x/matrix-g2c-pilot-project-template.git \
  <new-project-folder>
```

This will create a new project folder with the contents of the template.
It will use the latest release tag or `HEAD` from the default branch.

The new project folder will have all of the template defaults and documentation.

You should then `git init` your project and update any files or documentation
that may not apply.

After some time, if you wish to update your project with any changes made
to this project template, you are able to do it by using:

```sh
copier update
```

### What to change

Once you copy the template files into your repository, you will need to update
some of them with project-specific data:

- [README.md](./README.md): most of the contents of this file should be
  replaced with information that is relevant to your project and not
  this template.
- [publiccode.yml](./publiccode.yml): either remove it if you don't want your
  new project catalogued or update it to have accurate information.
  You must follow the [format schema](https://yml.publiccode.tools) and
  can edit its contents using the
  [official editor / validator](https://publiccode-editor.developers.italia.it).
- [.gitignore](./.gitignore): should include files and folders specific to the
  new project's tech stack.
- [.gitlab-ci.yml](./.gitlab-ci.yml): should be extended to include your build,
  test and deployment workflows.

Also check the existing tooling configuration files if you need to adjust any
settings that are particular to your project's requirements.

The result of this command will be the equivalent of a git rebase, where content
merges will be done where possible and conflicts must be resolved if they appear.

See [updating a project](https://copier.readthedocs.io/en/stable/updating/) for
more details.

### Automatic Dependency Updates

This template includes a minimal [renovate](https://docs.renovatebot.com)
configuration and CI workflow.

To use it in a new project, you will have to do the following steps:

1. Create a new GitLab **Personal Access Token** following [these
  instructions](https://docs.gitlab.com/user/profile/personal_access_tokens/).
  This new token should have the `api` scope.

2. Create a new GitLab **Scheduled pipeline** following [these
  instructions](https://docs.gitlab.com/ci/pipelines/schedules/).

3. Add the token created in **Step 1** as a new variable with the key
  `RENOVATE_TOKEN` and specify the desired recurrence interval pattern.

Once the scheduled pipeline is executed for the first time, a new issue
will be created with a
[Dependency Dashboard](https://docs.renovatebot.com/key-concepts/dashboard/)
and if there are any dependency updates, new Merge Requests will also
be created with the detected changes.

You can further customize the `renovate` process by changing the
[renovate.json5](./renovate.json5) configuration file with custom
behavior that suits better to your project.

### Development Guidelines

For information on how to use the conventions defined in this project, check
out the [expected development workflow](docs/workflow.md).

Please also see [CONTRIBUTING.md](./CONTRIBUTING.md) for details on how
you can contribute to this project.

## Licensing

This project is licensed under the [European Union Public License 1.2](https://spdx.org/licenses/EUPL-1.2.html).

For information on how to manage license and copyright annotations for your
contributions, check our [development workflow](docs/workflow.md) documentation.
