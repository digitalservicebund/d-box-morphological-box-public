<!--
SPDX-FileCopyrightText: 2025-2026 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Neo Solution Architecture

This repository contains documentation of the solution architecture for the
[Matrix-based G2X communication infrastructure pilot](https://gitlab.opencode.de/fitko/g2x-matrix-pilot),
code name "Neo".

## Status

During the first project year (July 2025 - July 2026),
understanding the problem space and context of the system was a primary goal,
to make one of the overarching targets to deploy the system
with successful uptake in the real world achievable.
The high level goals are summarised in the first arc42 chapter,
[Introduction and Goals](./01_introduction_and_goals.md),
and architecture Constraints in the second chapter,
[Architecture Constraints](./02_architecture_constraints.md).

The focus topics were:

1. Leveraging Matrix' federation capability was identified early on
   to benefit operations in practice
   while also essential for achieving the goal of
   [unlinkability](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/54fc5c7a8679ab700358fe63b57091901191dac1/Leistungsbeschreibung.md#254-vermeidung-von-globalen-identifikatoren-zur-unterst%C3%BCtzung-des-schutzziels-der-unlinkability).
2. Private individuals verifying the identity of competent authorities via X.509
   remains an actively developed topic at this time:
   [!29](https://gitlab.opencode.de/fitko/matrix-g2x/solution-architecture/-/merge_requests/29)
   [!37](https://gitlab.opencode.de/fitko/matrix-g2x/solution-architecture/-/merge_requests/37)
3. Building the basis for
   [structured communication](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/54fc5c7a8679ab700358fe63b57091901191dac1/Leistungsbeschreibung.md#251-strukturierte-vorgangskommunikation).
   While Matrix is generally designed to be extensible,
   the regarded use case requires a higher degree of customisation than most
   applications built on Matrix.
   The message formats
   [documented in chapter 5 "Building Block View"](./05_building_block_view.md?ref_type=heads#message-formats)
   of the documentation do not only define custom Matrix message types that fit
   into the upstream Matrix ecosystem,
   but build on a previously proposed system called
   [Extensible Events](https://github.com/matrix-org/matrix-spec-proposals/blob/main/proposals/1767-extensible-events.md)
   with the intention to define the future way of creating Matrix event types.
   Immediate feedback has been contributed to the upstream development of Matrix
   with proper implementations
   and support to finalise specification of Extensible Events planned.
   The chosen approach additionally defines a way to compose complex messages
   that can be freely mixed out of multiple building blocks
   while retaining guarantees for message integrity
   and still remaining compatible with upstream.
4. Leveraging the existing
   [BundID authentication](https://gitlab.opencode.de/fitko/matrix-g2x/solution-architecture/-/merge_requests/15#note_456643)
   infrastructure to create accounts and log into the Matrix subsystem.

Further results of the first year are high level drafts and concepts
in need of further refinement in the future.

## Table of Contents

This documentation follows the [arc42 template](./about-arc42.md).

1. [Introduction and Goals](./01_introduction_and_goals.md)
2. [Architecture Constraints](./02_architecture_constraints.md)
3. [Context and Scope](./03_context_and_scope.md)
4. [Solution Strategy](./04_solution_strategy.md)
5. [Building Block View](./05_building_block_view.md)
6. [Runtime View](./06_runtime_view.md)
7. [Deployment View](./07_deployment_view.md)
8. [Cross-cutting Concepts](./08_concepts.md)
9. [Architecture Decisions](./09_architecture_decisions.md)
10. [Quality Requirements](./10_quality_requirements.md)
11. [Risks and Technical Debts](./11_technical_risks.md)
12. [Glossary](./12_glossary.md)
13. [Methodology](./13_methodology.md)

## Development Guidelines

Please see [CONTRIBUTING.md](./CONTRIBUTING.md) as a starting point for how
you can contribute to this project.

For information on how to use the conventions defined in this project, check
out the [expected development workflow](docs/workflow.md).

This project uses [renovate](https://docs.renovatebot.com) to manage dependency
updates automatically, see the
[Dependency Dashboard](https://docs.renovatebot.com/key-concepts/dashboard/)
issue [!1](https://gitlab.opencode.de/fitko/matrix-g2x/solution-architecture/-/issues/1).

This project is based on the
[Neo Project Template](https://gitlab.opencode.de/fitko/matrix-g2x/matrix-g2c-pilot-project-template/-/tree/main).
Refer to its documentation about how to update the used template version.

## Licensing

This project is licensed under the
[Creative Commons Attribution 4.0 International Public License (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).

For information on how to manage license and copyright annotations for your
contributions, check our [development workflow](docs/workflow.md) documentation.
