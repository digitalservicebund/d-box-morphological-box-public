<!--
SPDX-FileCopyrightText: 2025, 2026 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Contributing to the Architecture

This repository follows the [arc42 architecture documentation template](../about-arc42.md).
Please follow the guidelines below when contributing.

## General Principles

- This repository is public from the beginning of the conceptual phase.
- We document all content transparently as part of the arc42 structure.
  Publishing incomplete or under development sections early is encouraged, provided
  it contains a notice about the quality of the document.
  - If placeholders are used,
    they must contain a clear note (e.g., <TODO: content pending>)
    to ensure first-time readers can distinguish between
    draft and completed sections.
- All arc42 documentation must be written in **English** to ensure accessibility
  for the entire team and to avoid translation errors later on.

## Repository Structure

- Each arc42 chapter is documented in a separate Markdown file.
- We use directories to group sections that are split due to their size or
  because of dedicated asset files. A Markdown file with the same name as the
  directory has to be included for an overview.

### Architecture Decisions (Chapter 9)

[Chapter 9 (_Architecture Decisions_)](../09_architecture_decisions.md) contains
a set of formal decisions for which we define a more specific process.

- For everything we want to formally decide on as part of the solution architecture,
  we create a new ADR (Architecture Decision Record) as an individual Markdown file.

  For the template and formatting specifics, see
  [ADR-0000: Use Markdown Architectural Decision Records](../architecture_decisions/0000-use-markdown-architectural-decision-records.md).

  - We prefer the full [MADR 4.0.0 template](../architecture_decisions/templates/adr-template.md),
    removing optional sections as applicable and reference existing text
    in favour of repeating ourselves.

- ADRs are numbered sequentially and incremental by design, existing ADRs are only
  updated to indicate them having been amended or replaced. We prefer documenting
  a process, a chain of thoughts,
  rather than trying to achieve a perfect view on the currently valid decisions.

- Chapter 9's overview file contains an index over all ADRs with their respective
  titles.

- See the ADRs of our parent project [ZaPuK](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/tree/main/Zielarchitektur/Architekturentscheidungen)

## Formatting

- See the [GitLab Flavoured Markdown (GLFM) documentation](https://docs.gitlab.com/ee/user/markdown.html)
  for a comprehensive list of formatting options.
- Use **relative links** to connect documents within the arc42 structure.
  This improves consistency and allows for navigation in offline or exported
  formats.

  For example, for linking to a section in Runtime View:

  ```md
  [Consistency Rules](06_runtime-view.md#consistency-rules)
  ```

  Here, the "consistency-rules" slug is automatically generated to refer to
  the existing heading `### Consistency Rules`.

- When using a domain specific German term, we use an English translation
  where possible, alongside the original German term. Optionally an abbreviation
  can also be introduced so following paragraphs can reference the same term.

  For example, for referencing ZaPuK:

  ```md
  Target architecture for mailbox and communication solutions [Zielarchitektur Postfach- und Kommunikationslösungen] (ZaPuK)
  ```

- When explaining concepts using graphics,
  we prefer to extend our [Structurizr](https://structurizr.com/) model
  located in [`structurizr/`](../structurizr/) as a comprehensive representation
  of the project whenever possible,
  and render views of the model to [Mermaid](https://docs.gitlab.com/user/markdown/#mermaid)
  for native inclusion in the Markdown as rendered by GitLab.
  When not possible, we prefer creating Mermaid diagrams directly,
  due to their text based format fitting well with GitLab Markdown rendering and
  version control.
  As fallbacks, image formats can also be used,
  preferring formats that are sources that can be modified easily later
  (e.g. SVG),
  and as a last resort other types of images.

## Proposing changes, Review, History and Version Control

See the [Workflow Documentation](./workflow.md) or contact the architecture team
for support.

### Architecture Review process

- Use **comment threads** in the GitLab **merge request interface** for
  discussions and clarification by directly referencing lines of code:
  <https://docs.gitlab.com/user/project/merge_requests/changes/>
- While small suggestions can be applied and resolved by the author, any
  threads requesting significant changes should be resolved by the reviewer
  requesting the change so they get a chance to check if the change matches
  their proposal.
- PO approval should happen within the GitLab Review Process for this
  solution architecture repository.
  When the PO signals approval, the MR author is responsible for performing the
  merge.
