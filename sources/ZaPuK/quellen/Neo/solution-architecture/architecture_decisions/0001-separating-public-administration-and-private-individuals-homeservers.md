---
status: "proposed"
date: 2025-08-21
decision-makers: team::architecture
---

# ADR-0001: Separating Government and Private Individuals' Homeservers

## Context and Problem Statement

The Neo project defines the [Matrix protocol](https://spec.matrix.org)
as core for its communication infrastructure.
A key architectural decision concerns how to structure homeserver deployment:

* Should all users (private individuals and government case workers) share
  **a single centralized homeserver**, or
* Should there be **separate homeservers**:
  one for private individuals and another for government case workers?

The target vision for the project in
[LB 2.5](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md?ref_type=heads#25-zielbild)
defaults to a system using a single Matrix server.
This decision has significant implications for scalability, security,
operational complexity, and compliance.
Particularly, it was identified early in the course of this project
that diverging from the single server concept may be core
to fulfilling multiple of the other requirements.

<!-- This is an optional element. Feel free to remove. -->
## Decision Drivers

* Single-Responsibility Principle (SRP):
  cover needs of different audiences by different services.
* Unlinkability ([LB 2.5.4](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md?ref_type=heads&plain=1#L517))
  as an aspect of Privacy-By-Design:
  by separation of concerns,
  either side can access less of the other's metadata.
  This requires operation of the homeservers by different parties,
  which is explored further in
  [ADR-0004: Separate Homeserver Operators](0004-separate-homeserver-operators.md).
* Countering risk accumulation of centralised infrastructure
  ([BD3](../01_introduction_and_goals.md#bd3-pain-necessitates-for-higher-security-and-privacy-requirements)):
  Reduce the impact of compromises of infrastructure by isolating components
  technically and organisationally.
* See also [ZaPUK ADR-0002](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/main/Zielarchitektur/Architekturentscheidungen/ADR-0002-Infrastrukturtopologie.md?ref_type=heads)

## Considered Options

* [Single Central Homeserver (Monolithic Deployment)](#single-central-homeserver-monolithic-deployment)
* [Two Separated Homeservers (Federated between Private Individuals and Government)](#two-separated-homeservers-private-individuals--government)
* [Tenant-based separation in a single Homeserver (Hybrid)](#tenant-based-separation-in-a-single-homeserver-hybrid)

## Decision Outcome

We deploy **two separate homeservers**,
one dedicated to private individuals' usage and another for government case
workers.

**Rationale:**

* The separation of trust zones is aligned with public sector security
  expectations.
  Private individuals and government case workers have fundamentally different
  access levels, communication patterns, and regulatory requirements.
* Deploying dedicated instances for different target groups simplifies the
  setup per instance regarding authentication requirements.
* Isolation improves control over data governance, audit trails,
  and incident response.
* The service is more resilient against outages,
  as even when there is an incident on one server,
  the other matrix server continues to work.
* Federation between the two domains allows for controlled interaction while
  minimizing the coupling between operational responsibilities.
* Opens up the possibility to introduce cryptographic identities in later ADRs
  to contribute to reaching the goal of unlinkability.

We plan to address the added complexity regarding deletion requirements
in a separate ADR <TODO: add reference>.

<!-- This is an optional element. Feel free to remove. -->
## Pros and Cons of the Options

### Single Central Homeserver (Monolithic Deployment)

* **Pros:**
  * Simpler to maintain and operate
  * Identity and room management in a single place simplifies guarantees,
    e.g. regarding deletion requests
  * Easier analytics and auditability
  * Lower infrastructure footprint
  * Data deduplication, potentially including file attachments via
    [Matrix Media Repo](https://github.com/t2bot/matrix-media-repo)
* **Cons:**
  * Mixed trust zones increase risk of privilege escalation
  * Harder to isolate issues between user groups
  * Harder to separate mixed configuration in terms of authenticating
    private individuals and government agencies
  * Less flexible in terms of scaling and policy enforcement
  * Single point of failure

### Two Separated Homeservers (Private Individuals / Government)

* **Pros:**
  * Clear separation of trust zones (e.g., different compliance policies)
  * Allows tailored operational models per user group
  * Limits blast radius of attacks or outages
  * Facilitates different federation or governance models
  * More Privacy-By-Design by separating concerns where possible
* **Cons:**
  * Requires federation setup between both homeservers
  * Increases operational and monitoring overhead
  * Requires up to twice the hardware and maintenance resources
  * Deletion requests require special attention in respect of the privacy
    requirements.
* **Neutral:**
  * The migration path from the existing infrastructure (ZBP) is not affected

### Tenant-based separation in a single Homeserver (Hybrid)

* **Pros:**
  * Inherits the advantages of the
    [Single Central Homeserver (Monolithic Deployment)](#single-central-homeserver-monolithic-deployment)
* **Cons:**
  * The improvement through tenant-based separation over the option
    [Single Central Homeserver (Monolithic Deployment)](#single-central-homeserver-monolithic-deployment)
    is limited.
    Most importantly this setup still lacks the enforcement boundaries
    required in sensitive deployments,
    nor can it fulfil the privacy requirements.

<!-- This is an optional element. Feel free to remove. -->
## More Information

See also the sibling ADRs

* [ADR-0004: Separate Homeserver Operators](0004-separate-homeserver-operators.md)
* [ADR-0005: Broader Federation](0005-broader-federation.md)

## Next Steps

* Define policies and operational responsibilities per domain
* Evaluate integration implications
  (e.g. with identity providers, logging, audit)
