---
status: "proposed"
date: 2025-09-07
decision-makers: team::architecture
---

# ADR-0003: Broader Federation

## Context and Problem Statement

Federation and decentralization are core aspects of the Matrix protocol.
It is possible to use Matrix infrastructure in a fully centralized manner,
or to achieve different levels of decentralization.
[ADR-0001](0001-separating-public-administration-and-citizen-homeservers.md)
documents the decision to split the Matrix infrastructure into two backends,
one for private individuals and the other for government case workers to
interact with.
This decision is based on operational needs and also contributes to increased
privacy in the whole system.

This ADR examines whether a higher degree of decentralization would improve the
system further.

<!-- This is an optional element. Feel free to remove. -->
## Decision Drivers

* Private individuals' Acceptance
  ([BD4](../01_introduction_and_goals.md#bd4-opportunity-improving-user-experience-for-citizens)):
  great experience and utility for private individuals to maximize uptake
* Government Agency Acceptance
  ([BD5](../01_introduction_and_goals.md#bd5-opportunity-improving-developer-experience-for-government-agencies)):
  first class experience for agencies considering
  implementation to maximize uptake
* Limiting the amount of metadata available to any single party is inherently
  privacy friendly (Privacy-By-Design approach)
* Unlinkability ([LB 2.5.4](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md?ref_type=heads&plain=1#L517)):
  Prevent profiling of users
* Private individuals log into the system via BundID
  ([LB 2.5](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md#25-zielbild))
* See also
  [ZaPuK ADR-0002](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/main/Zielarchitektur/Architekturentscheidungen/ADR-0002-Infrastrukturtopologie.md)
* See also
  [ZaPuK ADR-0004](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/main/Zielarchitektur/Architekturentscheidungen/ADR-0004-Bereitstellung-von-Postfach-Backends.md)

## Considered Options

* [Higher degree of decentralization on the private side](#higher-degree-of-decentralization-on-the-private-side)
* [Higher degree of decentralization on the government side](#higher-degree-of-decentralization-on-the-government-side)
* [Higher degree of decentralization on both sides](#higher-degree-of-decentralization-on-both-sides)
* [Deploy a single homeserver for government and private individuals each](#deploy-a-single-homeserver-for-government-and-private-individuals-each)

## Decision Outcome

We decide to deploy a
**single homeserver for government and private individuals each**,
as a broader federation's small advantage of resilience against outages is
overshadowed by huge increases in operational complexity and, depending on the
method chosen, potential weakening of privacy properties.

Particularly "bring your own homeserver" variants,
both for private individuals and competent agencies,
are not feasible with the privacy requirements of the project in mind.

[ZaPuK ADR-0004](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/main/Zielarchitektur/Architekturentscheidungen/ADR-0004-Bereitstellung-von-Postfach-Backends.md)
discusses letting participants operate their own backends.
While the ADR at hand (**this** document) decides against implementing this
proactively,
the architectural decision does not prohibit additional instances joining the
federated network in the future, should it become desirable.
Further arguments in favour or against this may be discussed in different ADRs
(within Neo or within ZaPuK).

<!-- This is an optional element. Feel free to remove. -->
## Pros and Cons of the Options

### Higher degree of decentralization on the private side

From a technical perspective on the Matrix federation layer,
it would be possible to allow private individuals to (optionally) bring their
own homeservers.
However this would mean solving problems including legal (e.g.
*Zustellungsfiktion*), privacy (e.g. single user homeservers do no longer
obscure the recipient sufficiently, see ADR-ZZZZ <TODO: update reference>),
third-party systems (delegating login to BundID), etc.
This case is thus not practical.

Instead, we regard the case of multiple Matrix servers operated by one or more
governmental organizations.
Clustering by different properties is possible, for example:

* by federal state of residence
* by competent agency the private individual is in contact with
  (multi-account solution)
* by an anonymized hash of the private individual's identity

Choosing this option would have the following effects:

* Good, because the overall system is more resilient against outages:
  Only one shard of private individuals will be affected.
* Neutral, because less metadata is available in a single location.
  However this is less relevant if multiple servers are operated by the same
  organization who can correlate the data across servers, as explained in
  [ADR-0002](0002-separate-homeserver-operators.md).
  Further, clients need to know which server to connect to automatically in
  order to ensure a simple user experience, which removes any anonymizing
  approaches.
* Bad, because the operational overhead and need for coordination increases.
* Bad, because the suggested approach ADR-ZZZZ <TODO: update reference> sealed
  sender/recipient now needs to leak the metadata about the recipient server.

### Higher degree of decentralization on the government side

Competent agencies or their back-office systems (*Fachverfahren*) could be assigned a
homeserver in different ways, for example:

* across an amount of *n* homeservers at random/round robin/etc.
* allowance to operate their own homeserver
* depending on the overall structure of the government

Choosing this option would have the following effects:

* Good, because the overall system is more resilient against outages:
  Only one shard of agencies will be affected.
* Bad, because the operational overhead and need for coordination increases.
* Bad, because the suggested approach ADR-ZZZZ <TODO: update reference> sealed
  sender/recipient now needs to leak the metadata about the recipient server.
  Depending on the degree of decentralization,
  this could affect the unlinkability property of the system
  (the private individuals' homeserver is now able to learn to which shard of
  agencies the private individuals are talking to).

### Higher degree of decentralization on both sides

This solution inherits the largely overlapping arguments from both single-sided
options, amplifying each other.

### Deploy a single homeserver for government and private individuals each

* Good, because the operational complexity is not affected beyond
  [ADR-0002](0004-separate-homeserver-operators.md).
* Good, because we are able to make certain privacy guarantees discussed in
  [ADR-0001](0001-separating-public-administration-and-citizen-homeservers.md)
  and [ADR-0002](0002-separate-homeserver-operators.md).
* Bad, because we limit the sovereignty of participants.
