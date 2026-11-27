<!--
SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Architecture Constraints

[[_TOC_]]

In this chapter we describe anything that constrains the **design and implementation decisions** or decisions about **related processes**.

These constraints define the binding scope for our architecture.
They define what must not be exceeded or violated.

Within these limits, the quality requirements defined in [chapter 10](./10_quality_requirements.md) are pursued.
The constraints can influence, restrict, or change the **priority of individual quality objectives**.
Quality objectives determine the direction in which the architecture is optimized within this scope.
In practice, constraints often influence the **weighting** of quality objectives.

_Example_: A constraint such as “must run on embedded hardware XY” could make the quality objective “high performance” more difficult to achieve or change its priority.

## Constraint Sources

The most important constraint sources are:

- Directives such as [OZG](https://www.gesetze-im-internet.de/ozg/)
- [ZaPuK requirements list](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/main/konsolidierte-anforderungen/Konsolidierte-Anforderungen.md)
- [Leistungsbeschreibung](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md)
- [Technical Requirements from BSI (Technische Richtlinien, TRs)](https://www.bsi.bund.de/tr)

A full list of sources for requirements can be found in [chapter 10](./10_quality_requirements.md).

## Relation to Requirements and Architectural Decisions

- **Source of constraints** are the requirements from requirements engineering.
  - This includes functional requirements, quality requirements, and explicit constraints (e.g., "Zero Trust must be implemented").
- In the **constraints chapter**, these requirements are not described again, but only those requirements are **highlighted** that **significantly restrict** the architecture’s solution space.
  - Each constraint refers to the corresponding requirement ID.
- **ADRs (Architecture Decision Records)** then document **how** the architecture deals with these constraints. See [chapter 9 (Architecture Decisions)](09_architecture_decisions.md).

Here's a fictional example of the interplay:

- Leistungsbeschreibung:

  ```text
  LB-4.1.2 - Zero-Trust-Architecture
  ```

- Requirement:

  ```text
  REQ-12 – Zero Trust must be implemented
  ```

- Constraint (Chapter 2):

  ```text
  REQ-12 acts as a constraint, as it restricts technology choices and communication patterns.
  ```

- ADR:

  ```text
  ADR-05 – Decision to use mTLS between services and introduce a policy engine to implement Zero Trust.
  ```

👉 This creates clear traceability:
Requirement → Constraint (chapter 2) → ADR (chapter 9)

**The key constraints of the solution are described below.**

## Main Technical Constraints

### End-to-End Security

#### Description

According to [LB section 4.1](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#41-prinzipien-der-architekturgestaltung), end-to-end security must be implemented according to the current state of the art:

- End-to-end encryption must be provided.
- End-to-end authentication must be ensured.
- Communication between all involved components and users must be encrypted to
  protect against unauthorized access.

#### Rationale

End-to-end security ensures confidentiality, integrity, and authenticity of communication across all system components.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### Zero-Trust Architecture

#### Description

As defined in [LB section 4.1](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#41-prinzipien-der-architekturgestaltung), no user, service, or system is trusted by default.

- Every access to system components or data must be strictly authenticated and authorized, regardless of origin or location.
- Identities must be continuously verified.

#### Rationale

This principle enforces strict access control and prevents implicit trust within the system that may lead to unexpected privilege escalation in the event of a security incident.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### Security by Design

#### Description

In accordance with [LB section 4.1](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#41-prinzipien-der-architekturgestaltung), security requirements must be integrated from the beginning as part of the systems' design:

- Early consideration of all security requirements already in the design process.
- Protection of sensitive data, secure interfaces, and access rights management.
- Regular testing and continuous improvement of security measures.
- Evaluation of software libraries for vulnerabilities.
- Consideration of properties like resistance to side-channel and timing attacks, and formal verification where applicable.

#### Rationale

Security by design reduces risks by addressing potential vulnerabilities early and ensures robustness of the system.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### Separation of Concerns

#### Description

According to [LB section 4.1](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#41-prinzipien-der-architekturgestaltung), components and subcomponents (e.g., SDKs and libraries) must be modularized according to their purpose and security requirements.

- Components should be designed in such a way that they fulfil clearly defined tasks.
- Components can be tested, analysed and formally verified independently from each other.

#### Rationale

Separation of concerns improves maintainability, scalability, and security by enforcing clear boundaries and responsibilities.
This does not exempt us from considering the safety & security of the system as a whole.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### Ability to Deploy without Proprietary Components

#### Description

As defined in [LB section 4.6](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#46-sicherstellung-der-nachnutzbarkeit-und-nachhaltigkeit), the solution must be fully usable in productive operation under a Free Software license, without relying on proprietary third-party components.

- Only Free Software licensed components may be used.
- Demonstration of scalability, performance, and load capacity must be carried out under these conditions.

#### Rationale

This contractual requirement prevents vendor lock-in and ensures long-term sustainability and reusability of the developed solution.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### Developer Experience

#### Description

The solution is built with their users in mind. The system will provide interfaces for external developers to integrate the solution into their third-party software. [LB section 4.2.1](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#421-benutzbarkeit-und-nutzererfahrung) requires that the provided tools and interfaces facilitate smooth integration and continuous development:

- Intuitive APIs
- Helpful error messages
- Clear documentation of processes
- Providing SDKs to support efficient development

#### Rationale

Developer-friendly tools and interfaces reduce onboarding effort, improve productivity, and support sustainable evolution of the solution.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

## Main organizational constraints

### Matrix Spec Changes (MSCs)

#### Description

Any changes or extensions to the Matrix protocol required by this project must be submitted as Matrix Spec Changes (MSCs) through the official Matrix specification process.

Each MSC is subject to public community review and requires **formal approval by the Matrix Spec Core Team** before it can become part of the official
Matrix specification.

Without this approval, such extensions remain project-specific and are not guaranteed to be interoperable with the wider Matrix ecosystem.

#### Rationale

The project relies on the Matrix ecosystem. To ensure interoperability, maintainability, and long-term support, extensions to the Matrix protocol will be submitted to the Matrix standardization process.

#### References

- [Matrix Specification](https://spec.matrix.org)
- [Matrix Spec Proposals Repository](https://github.com/matrix-org/matrix-spec-proposals)

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### LB Open Source Development Best Practices

#### Description

According to [LB section 4.6](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#46-sicherstellung-der-nachnutzbarkeit-und-nachhaltigkeit), the contractor must ensure that development processes follow established Open Source development best practices:

- Use of transparent version control systems (Git).
- Adherence to recognized Open Source development guidelines.
- Provision of clear and understandable documentation.
- Definition of contribution guidelines (CONTRIBUTING.md).
- Provision of a README file.
- Proper dependency management.

#### Rationale

These practices are mandated to ensure transparency, maintainability, and community reusability of the project results.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### Upstream Contributions

#### Description

In accordance with [LB section 4.6](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#46-sicherstellung-der-nachnutzbarkeit-und-nachhaltigkeit), the team shall actively contribute to upstream Open Source projects:

- Submission of bug reports and participation in discussions.
- Contribution of fixes and improvements (e.g., via code patches).
- All modifications to third-party components must be provided back to the respective upstream project under its original license.
- Publication of contributions must occur immediately, and at the latest after each sprint.

#### Rationale

These obligations strengthen the sustainability of the project, reduce maintenance overhead by avoiding forks, and support the Free Software ecosystem.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

## Main regulatory constraints

### Privacy by Design

#### Description

According to [LB section 4.1](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#41-prinzipien-der-architekturgestaltung), privacy-preserving mechanisms must be an integral part of the architecture:

- Implementation of measures to safeguard user privacy.
- Early consideration of Data Protection Impact Assessments (DPIA) in accordance with legal requirements (e.g., GDPR).

#### Rationale

This constraint has both organizational and regulatory aspects:

- As a **regulatory requirement**, compliance with GDPR is mandatory.
- As an **organizational principle**, privacy-preserving measures must be embedded into design and development processes from the beginning.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### LB 4.4.1 Lawfulness of Processing

#### Description

As specified in [LB section 4.4.1](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#44-datenschutz):

- The system must ensure that all data processing is carried out exclusively in accordance with applicable data protection laws.
- Processing of personal data must always be based on one of the legal grounds of Art. 6 GDPR.
- For every data processing activity, a clear legal basis (e.g., consent, legitimate interest, statutory requirement) must be documented and traceable.

#### Rationale

This requirement (LB 4.4.1) enforces GDPR compliance by ensuring that all processing is lawful and based on an explicit legal ground.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### LB 4.4.2 GDPR - Information Obligations

#### Description

According to [LB section 4.4.2](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#44-datenschutz) and Art. 13 GDPR:

- Data subjects must be transparently informed about type, scope, purpose, and legal basis of data processing in a clear privacy policy.
- This information must be accessible during registration and continuously thereafter.
- Information must cover retention periods, rights of data subjects (access, deletion), and contact details of the data protection officer.
- Preparation of the privacy policy itself is not part of the projects deliverables, but all relevant technical information about the system must be made available upon request.

#### Rationale

This requirement (LB 4.4.2) supports GDPR compliance by enforcing transparency towards data subjects and enabling the controller to fulfil Art. 13 obligations.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### LB 4.4.3 GDPR - Right of Access

#### Description

According to [LB section 4.4.3](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#44-datenschutz) and Art. 15 GDPR:

- Data subjects have the right to obtain a copy of their personal data being processed.
- Technical measures must enable automated provision of such data copies.

#### Rationale

This requirement (LB 4.4.3) ensures that the system can technically support data subjects’ right of access under Art. 15 GDPR.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### LB 4.4.4 GDPR - Right to Erasure

#### Description

According to [LB section 4.4.4](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#44-datenschutz) and Art. 17 GDPR:

- Data subjects have the right to request deletion of their personal data.
- Technical measures must enable automated deletion processes.

#### Rationale

This requirement (LB 4.4.4) ensures compliance with the GDPR right to erasure (“right to be forgotten”) and demands technical enforcement in the system.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### LB 4.4.5 GDPR - Processing in Subsystems

#### Description

As required in [LB section 4.4.5](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#44-datenschutz):

- Even indirectly personal data (e.g., server log files) must comply with data protection requirements.
- This includes the principle of data minimization: only the necessary data may be processed.
- Technical and organizational measures for data security must be implemented according to Art. 32 GDPR.

#### Rationale

This requirement (LB 4.4.5) ensures that subsystems and log data processing comply with GDPR principles such as data minimization and Art. 32 security measures.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### LB 4.4.6 GDPR - Data Processing by Third Parties (Processors)

#### Description

According to [LB section 4.4.6](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#44-datenschutz) and Art. 28 GDPR:

- Data processing agreements (Auftragsverarbeitungsverträge) must be concluded with third parties (e.g., hosting providers) engaged in personal data processing (including test/demo environments).
- Systems must be designed so that data subjects’ rights (e.g., portability, deletion) can always be exercised.

#### Rationale

This requirement (LB 4.4.6) ensures compliance with GDPR Art. 28 by enforcing proper contracts with processors and enabling data subjects to exercise their rights.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### LB 4.4.7 GDPR - Data Protection Impact Assessment (DPIA)

#### Description

According to [LB section 4.4.7](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#44-datenschutz) and Art. 35 GDPR:

- Regular Data Protection Impact Assessments (DPIA / German: DSFA) must be carried out.
- Risks must be identified early, and adequate measures for risk minimization must be implemented.

#### Rationale

This requirement (LB 4.4.7) ensures that high-risk processing operations are assessed and mitigated through mandatory GDPR Art. 35 DPIAs.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

### LB 4.7 Licensing of Deliverables

#### Description

According to [LB section 4.7](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#47-regelungen-der-nutzungsrechte-freier-offener-softwareentwicklungsprozess), all project deliverables (including software components, source code, data, documentation, configurations, and other materials) must be licensed under Free and Open Source licenses.

The following binding rules apply:

- Software must be licensed under the European Union Public License (EUPL).
- Upon request of the client, alternative Free Software licenses or Creative Commons Attribution Share Alike 4.0 (CC BY-SA 4.0) may be applied.
- Modifications to third-party Free Software components must be contributed back to the respective upstream project under its original license.
- Documentation and manuals must be licensed under CC BY-SA 4.0.
- Configuration files must be licensed under CC0 1.0.
- All deliverables must be published on agreed public platforms (e.g., openCode or other Git-based repositories).
- License compatibility with third-party components must always be ensured.

#### Rationale

These licensing requirements are binding contractual obligations (LB 4.7).
They ensure openness, reusability, and long-term sustainability of project outcomes.

<!--
#### Architecture Decisions

> [!note]
> References to ADRs or other sections of the solution architecture that describe HOW the constraints are met.
-->

## Relation to other chapters

[Chapter 10](10_quality_requirements.md) describes **quality requirements**.
