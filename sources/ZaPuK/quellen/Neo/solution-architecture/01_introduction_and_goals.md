<!--
SPDX-FileCopyrightText: 2025 FITKO (Föderale IT-Kooperation)

SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Introduction and Goals

[[_TOC_]]

This document describes a solution implementing government-to-citizen (G2C)
communication developed in the
[piloting a Matrix-based communication channel for G2C communication](https://gitlab.opencode.de/it-planungsrat/fit-ab/vorhaben/-/issues/5
"Pilotierung Matrix-basierter Kommunikationskanal für die G2C-Kommunikation") project
initiated by the
[Federal IT-Architecture Board \[Föderales IT-Architekturboard\]
(FIT-AB)](https://docs.fitko.de/fit/fit-ab/).

The project is led by [FITKO](https://www.fitko.de/) and the
[Federal Ministry for Digital Transformation and State Modernization (BMDS)](https://bmds.bund.de/).

The project aims to evaluate advanced use cases for government to citizen
communication (G2C communication), improving the experience for citizens and
government agencies. The prototype focuses on combining security, privacy,
usability, accessibility as well as providing a future-proof modular and
extensible architecture. The prototype will implement various features that go
beyond the functionality of the existing mailbox, such as **state of the art
end-to-end encryption**, a **mobile interface** and **bidirectional
communication**.

The scope of the project is to develop a **production-grade prototype** for a modern
mailbox component which can be integrated into the existing BundID
infrastructure operated by BMDS. The BundID infrastructure is made available to
all citizens on the legal basis of Article 2 of the [Online Access Act
\[Gesetz zur Verbesserung des Onlinezugangs zu Verwaltungsleistungen\]
(OZG)](https://www.gesetze-im-internet.de/ozg/BJNR313800017.html). It
enables its users to communicate securely with government agencies in the
context of a public service. Secure communication, according to the Online
Access Act, means implementing end-to-end encryption (see
[BD8](#bd8-directive-new-regulatory-requirements-especially-end-to-end-encryption)).

The project fits within the domain of the broader
[target architecture on mailbox and communication solutions
\[Zielarchitektur Postfach- und Kommunikationslösungen\] (ZaPuK)](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/),
also developed by the Federal IT-Architecture Board. While the ZaPuK target
architecture focuses on all kinds of government communication (G2X), this
project focuses specifically on government-to-citizen communication (G2C).

In its
[project proposal](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/raw/main/Steckbrief-F%C3%B6derales-Architekturboard-Messenger-Kommunikation.pdf?ref_type=heads&inline=false),
the Federal IT-Architecture Board decided to use the
[Matrix protocol](https://matrix.org/) for these evaluations. Matrix is an open
standard defining a **set of APIs** which, among other use cases, support
**end-to-end encrypted instant messaging**. From a technical viewpoint, the
mailbox system has similarities to an instant messaging service, with only the
user interface being different to support the relevant usage patterns.

## Drivers

This section describes the WHY of the solution, the drivers.
Everything else must be aligned with this and must not be an end in itself.

### Pain

This section answers the question:
"What specific problems are to be eliminated by the solution?"

#### BD1: \[Pain\] Digital government services as an end-to-end process

In the context of the Online Service Act (Onlinezugangsgesetz, OZG), the scope
of projects realizing **government services** often end with delivering an
application to a competent authority. After the application has been received
digitally, follow-up communication such as status updates are often sent via
physical mail. For citizens this leads to unnecessary efforts and
dissatisfaction with the quality of government services.

#### BD2: \[Pain\] Federal states cannot switch to BundID

According to the first version of the Online Access Act (Onlinezugangsgesetz,
OZG), the federal 16 states started operating their own authentication and
mailbox infrastructure (landesspezifische Nutzendenkonten + Postfächer). With
the Act amending the Online Access Act (OZG-Änderungsgesetz, OZGÄndG),
authentication and follow-up communication takes place in a central mailbox
infrastructure (called BundID, see
[§ 3 OZG](https://www.gesetze-im-internet.de/ozg/__3.html)).
[§ 12 OZG](https://www.gesetze-im-internet.de/ozg/__12.html) now regulates the
transition from the 16 federal authentication and mailbox infrastructures to
BundID:

> (1) Für einen Übergangszeitraum von drei Jahren kann die Identifizierung und
> Authentifizierung sowie die bidirektionale Kommunikation der Nutzer im Sinne
> des § 2 Absatz 4 Nummer 1 für elektronische Verwaltungsleistungen im
> Portalverbund auch über die bisherigen Nutzerkonten und Postfächer der Länder
> oder eines Fachportals erfolgen. Die Frist nach Satz 1 beginnt an dem Tag, an
> dem das Bundesministerium des Innern und für Heimat im Einvernehmen mit dem
> IT-Planungsrat im Bundesgesetzblatt bekanntgibt, dass die Voraussetzungen für
> eine automatisierte Migration der Länderkonten auf das zentrale Bürgerkonto
> vorliegen. \[...\]

To date, BundID does not support bidirectional communication. In contrast, some
federal authentication and mailbox infrastructures already implement
bidirectional communication. Thus, federal states cannot use the BundID mailbox
system although they are interested in doing so.

#### BD3: \[Pain\] Necessitates for higher security and privacy requirements

The switch from decentralized authentication and mailbox systems to BundID (see
above) and
[increased usage of the BundID](https://dashboard.digitale-verwaltung.de/nutzung/identifizierungsmittel)
over the last years necessitate for increased protection against attacks on this
centralized infrastructure (risk accumulation). Sensitive data stored there is
at risk if the service is compromised. An infrastructure as critical as BundID
needs to resist a multitude of outside and inside attack vectors.

### Opportunities

This section answers the question:
"What opportunities should the solution open up?"

#### BD4: \[Opportunity\] Improving user experience for citizens

Interacting with competent authorities offering government services should be
intuitive and transparent. Here, a **well-designed and well-integrated mailbox
system** can do a lot of heavy lifting, as this is where citizens can be kept up
to date with their service requests.

We want to focus on bringing convenience features on par with what citizens are
used to from other services. This includes a mobile app that displays the same
messages as the web interface, real time updates, push notifications and
replying to questions directly from within the app. A sharp focus on
accessibility and seamless integration into back-office systems
[Fachverfahren] (see also BD1) will increase user acceptance and satisfaction.

#### BD5: \[Opportunity\] Improving developer experience for government agencies

Providing tools, documentation and support for government agencies wanting to
provide a rich experience for citizens is key for increasing adoption (relating
to BD1) in a diverse ecosystem of implementers. We want to treat agencies as
first class users and provide an approachable and clear experience by providing
documentation for management, legal and developers.

#### BD6: \[Opportunity\] Synergies with the existing Matrix and free software ecosystem

The community around the [Matrix standard](https://spec.matrix.org/) provides an
ecosystem of available implementations. The protocol already defines ways to
implement extended functionality for certain use-cases which are relevant for
our project, including end-to-end encryption, multi-device functionalities,
optimizations for mobile devices (push notifications) and much more. This
greatly reduces the costs and complexity of developing a new communication
solution.

Matrix has already seen adoption in other government projects, such as
[Tchap](https://www.tchap.gouv.fr) for internal government communication in
France, [Bw Messenger](https://messenger.bwi.de/) for internal communication in
the German military or
[TI-Messenger](https://www.gematik.de/anwendungen/ti-messenger) for internal and
patient communication in the German healthcare sector, as well as in commercial
and community settings.

The project can benefit from the improvements these various parties contribute
to the Matrix standard and their investments to the Matrix software ecosystem.
Shared development on the Matrix specification and implementations following the
open source / FOSS model will help reduce maintenance costs in the future
(compared to an in-house development with the same functionalities).

#### BD7: \[Opportunity\] Backing the target architecture (ZaPuK)

The
[Target Architecture for Mailbox and Communication Solutions \[Zielarchitektur Postfach- und Kommunikationslösungen\]
(ZaPuK)](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/) has
[been decided on by the IT-Planning Council \[IT-Planungsrat\]](https://www.it-planungsrat.de/beschluss/beschluss-2025-28)
as a vision for a unified communication infrastructure for the public sector as
a whole. In contrast to the Neo project, ZaPuK not only considers
government-to-citizen (G2C) communication but describes the whole ecosystem
including government-to-business (G2B) communication and has a more diverse set
of stakeholders. It entails a reference framework to the premise for the Neo
project.

The Neo project has the opportunity to show that an implementation of the target
architecture is viable for the **specific use-case of government to citizen
(G2C) communication**. With the ministry for digital and state modernization
(BMDS) and FITKO as partners, we can evaluate the goals and challenges of the
target architecture in line with its consolidation efforts.

The target architecture describes specific technical aspects (like
authenticating secure channels using eID, Wallet, BundID) with direct impact on
Neo's architecture.

### Directives

This section answers the question:
"Should _new_ laws, guidelines etc. be implemented by the solution?"

This does not refer to _compliance_ with directives and laws, e.g. GDPR, but if
new requirements are the direct driver or if requirements are not yet being
complied with and the solution explicitly serves to achieve compliance.

#### BD8: \[Directive\] New regulatory requirements (especially end-to-end-encryption)

[§ 2 Online Service Act (OZG)](https://www.gesetze-im-internet.de/ozg/__2.html)
defines a mailbox as follows:

> (7) Ein „Postfach“ ist eine IT-Komponente, über die Nutzer **medienbruchfrei,
> barrierefrei und sicher** mit den an den Portalverbund angeschlossenen
> öffentlichen Stellen vorgangsbezogen kommunizieren können sowie elektronische
> Dokumente und Informationen senden **und empfangen** können. Das Postfach ist
> Bestandteil eines Nutzerkontos.

Among other changes, the highlighted part
[was introduced with the Act amending the Online Access Act (OZG-Änderungsgesetz, OZGÄndG)](https://www.buzer.de/gesetz/12765/al198206-0.htm)
in July 2024. Its explanatory memorandum \[Gesetzesbegründung\] states that
"sicher" includes implementing end-to-end encryption:

> Die Kommunikation über das Postfach soll hohen Sicherheitsstandards genügen.
> Bei einer Weiterentwicklung des Nutzerkontopostfachs soll daher nach
> Zurverfügungstellung entsprechender Haushaltsmittel auf eine
> **Ende-zu-Ende-Verschlüsselung** umgestiegen werden. Entsprechende technische
> Anpassungen sind auch bei allen mit dem Nutzerkontopostfach kommunizierenden
> Diensten im Portalverbund erforderlich.

## Business Goals

### BG1: Demonstrate feasibility and validate choice of technology

Within the main scope of the project we want to **evaluate and validate** the
**technology choices** and **proposed high level architecture from the project
mandate** and provide the Federal IT-Architecture Board (FIT-AB) a
**solution architecture (ARC42)** and an
**implementation _of the Minimum Viable Product_** that is
**ready for use in production with minimal efforts _until end of May 2026_**.

These will be developed _in the open_ via the openCode platform to allow
members of the FIT-AB and other stakeholders to stay informed and participate
transparently.

Through the projects learnings and artifacts, FIT-AB should have a strong basis
to discuss further decisions in context of the aforementioned target
architecture (ZaPuK).

**directly supports**: BD7  
**also supports**: BD8, BD1, BD2, BD3, BD4, BD5

### BG2: Improve privacy and security characteristics

Within the main scope of the project we want to provide an **improved
implementation of the BundID mailbox component** with respect to its privacy and
security characteristics.

In order to minimize attack surface on the BundID infrastructure, we want to
**support a zero trust compliant assume-breach scenario for BundID**, fully
maintaining confidentiality and a high level of integrity, only potentially
compromising availability.

Inline with strict regulatory requirements and in order to to allow for minimal
exposure in assume-breach scenarios (like data leaks), we want to implement or
at least lay **significant ground work** for **advanced privacy-preserving
concepts such as advanced metadata privacy**
**_by (a) a concept or b) by the software solution itself or c) by both)_**.

**directly supports**: BD8, BD3

### BG3: Combine great usability, great privacy and great security characteristics

Within the main scope of the project we want to show that implementing the
state-of-the-art security and advanced privacy properties of BO2 do not
inherently conflict with great usability, provided careful considerations and
targeted trade-offs (for example supporting higher security profiles for more
vulnerable users). This is partly in light of BO1 but also a greater audience.
This approach was pioneered by the Signal Messenger, which provides
best-in-class security and privacy properties for their specific use-cases and
proven usability for millions of users.

**directly supports**: BD3, BD4, BD5  
**also supports**: BD8

### BG4: Focus on production-readiness

Within the main scope of the project we want to build a high-quality prototype
that is ready to use in production with minimal effort.

From an early conceptual state we will need to make sure that the solution meets
all legal, technical and functional requirements that are necessary for the
systems' ability to be put to productive use, including those that are defined
in
[section 4 of the service description](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#4-%C3%BCbergreifende-anforderungen)
\[Leistungsbeschreibung\].

Together with various competent authorities from the federal government, members
states and municipalities we can consider real world use cases and feedback.

**directly supports**: BD1, BD2  
**also supports**: BD4, BD5, BD8

### BG5: Reusability of project results

In order to reduce maintenance costs of the project results **_(results:_**
**_concepts, software and infrastructure)_** and increase impact of the Neo
project beyond the project itself, all project results will be published and
openly licensed. Creating forks of existing software components should therefore
be avoided to the extend possible. In favour of project-specific specifications,
the project will also actively contribute to the Matrix specification wherever
reasonable (see
[sections 4.6 and 4.7 of the service description](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#46-sicherstellung-der-nachnutzbarkeit-und-nachhaltigkeit)).

This will also contribute to the sustainability of the whole Matrix ecosystem.
Any work done to improve the stability of the open source upstream components as
well as new features implemented in the Matrix specification will therefore also
benefit other projects in the German public sector and elsewhere in Europe.

**directly supports**: BD6, BD7

### BG6: Organizational and technical scalability

In order to be able to achieve widespread adoption, the solution needs to
massively scale in terms of organizational and technical aspects. Organizational
scalability, in this context, means that competent authorities using the
solution and software vendors working for these authorities need easy onboarding
processes that do involve as little personal interaction with the operations
team of our solution as possible (self-service). Technical scalability, in this
context, means that the solution needs to be able to process follow-up
communication by all citizens.

In order to achieve organizational scalability, we will provide
publicly-accessible management- and developer-oriented documentation for
competent authorities and their software vendors, provide a software development
kit (SDK), keep legal requirements of the competent authorities (such as
security and privacy documentation requirements) in mind as well as think about
synergies with existing infrastructures already used by those authorities (such
as [FIT-Connect](https://docs.fitko.de/fit-connect/)). We will also discuss
opportunities to improve self-service processes for an easy onboarding of
competent authorities.

In order to achieve technical scalability, we will perform extended performance
tests.
The architecture considers scalability in an early stage of the project.
Test automation provides the framework in order to significantly create
workloads on the system landscape.

**directly supports**: BD5, BD1  
**also supports**: BD7

## Quality Requirements

The complete list of quality requirements can be found in chapter
[10. Quality Requirements](10_quality_requirements.md).

## Stakeholders

For the successful development and operation of a system, it is essential to
identify all involved and affected parties. Overlooking relevant stakeholders
can lead to unpleasant surprises later in the process.

Stakeholders significantly influence the scope, level of detail, and outcomes of
architectural work.

This section therefore provides an overview of the system’s stakeholders –
including individuals, roles, or organizations that

- should be familiar with the solution,
- need to be convinced of the benefits of the solution,
- work directly with the solution or its source code,
- rely on the solutions' documentation, or
- make decisions regarding the system or its evolution.

These descriptions give only an overview of the stakeholders. The detailed
description can be found in chapter
[03. Context and Scope](03_context_and_scope.md).

| Role/Name | Expectations |
| --------- | ------------ |
| **Private Individuals** (_Pia Privatperson_) | Complete their request as quickly as possible. Perform tasks from home or on the go (via online forms). Use either a mobile device or PC. Understand the process despite long gaps (6-12 months) between uses. Feel confident and secure when using the technology. Avoid mistakes or delays in the process. Have clear instructions and a way to undo actions if needed. |
| **Administrative Staff** (_Sascha Sachbearbeiter_) | Handle multiple recurring cases daily. Switch between cases frequently. Request additional information or issue decisions. Work from home or office on a PC. Manage interruptions during tasks. Ensure proper documentation and organization of cases. Avoid being overwhelmed by simultaneous cases and interruptions. |
| **Administrative Staff** (_Verena Verwaltung_) | Manage IT-related issues and implement new online services. Have clear and actionable instructions for integration of Neo. Collaborate closely with other administrative staff. Ensure smooth communication with service providers. Avoid being overwhelmed by unclear requirements. |
| **Service Providers** (_Dieter Dienstleister_) | Implement government-to-citizen communication for new and existing online services at ease. Integrate SDKs into existing systems and ensure compatibility. Coordinate with administrative staff and other service providers. Provide clear documentation and training for administrative staff. Avoid hidden costs, unclear requirements, and technical misunderstandings. Ensure minimal impact on existing software and processes. Have access to reliable support and clear communication channels. |
