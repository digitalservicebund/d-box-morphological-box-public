---
status: "proposed"
date: 2025-08-28
decision-makers: team::architecture
---

# ADR-0002: Separation of Homeserver Operators

## Context and Problem Statement

[ADR-0001](0001-separating-public-administration-and-citizen-homeservers.md)
documents the decision to deploy separate federating Matrix homeservers for
government and private individuals.
While this already allows some separation, yielding advantages including
operational stability and simplification of configuration for the specific user
groups, this is not a complete solution to meet high privacy standards, such as
unlinkability of private individual activities among different government
agencies.

Concepts including further decentralisation, for example operation of
individual homeserver instances for each back-office system of the competent agency
(*Fachverfahren*), is explored in the separate
[ADR-0003](0003-broader-federation.md).

<!-- This is an optional element. Feel free to remove. -->
## Decision Drivers

* Limiting the amount of metadata available to any single party is inherently
  privacy friendly (Privacy-By-Design approach).
* Unlinkability
  ([LB 2.5.4](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md?ref_type=heads&plain=1#L517)):
  Prevent profiling of private individuals
* The operational complexity increases if different organisations are involved
  in providing the whole system. For example, there could be network borders to
  securely bridge, or alignment and common planning for maintenance tasks.
* Countering risk accumulation of centralised infrastructure
  ([BD3](../01_introduction_and_goals.md#bd3-pain-necessitates-for-higher-security-and-privacy-requirements)):
  Reduce the impact of compromises of infrastructure by isolating components
  technically and organisationally.
* [ZaPuK-ADR-0002](https://gitlab.opencode.de/it-planungsrat/fit-ab/zapuk/-/blob/main/Zielarchitektur/Architekturentscheidungen/ADR-0002-Infrastrukturtopologie.md)
  decides in favour of a hybrid approach that favours multi-server setups over
  single-server deployments.
  * ZaPuK has not decided on operation responsibilities.
    Designing for operation by multiple organisations gives us more flexibility
    on how to deploy this in production later.

## Considered Options

* [A single organisation operates all Matrix homeservers](#a-single-organisation-operates-all-matrix-homeservers)
* [Government and Citizen homeservers are operated by separate organisations](#government-and-citizen-homeservers-are-operated-by-separate-organisations)

## Decision Outcome

To achieve unlinkability in the sense of the project requirements, the
homeservers need to be **operated by separate organisations**, and additionally
recipient identities need to be encrypted (ADR-ZZZZ <TODO: update reference>,
"sealed sender/recipient").
As long as ADR-ZZZZ <TODO: update reference> is not implemented, the present
ADR is ineffective.
We thus defer implementation to a later project phase, after ADR-ZZZZ <TODO:
update reference> is implemented.

<!-- This is an optional element. Feel free to remove. -->
### Consequences

* Good, because we improve privacy and take a milestone in achieving true
  unlinkability.
* Bad, because the total operational complexity of the whole system increases
  (operational complexity for each of the two operators might still be lower
  than for a hypothetical single operator that needs to operate two
  homeservers).

<!-- This is an optional element. Feel free to remove. -->
### Confirmation

When this ADR is implemented in combination with ADR-ZZZZ <TODO: update
reference> (sealed sender/recipient), a single homeserver-operating
organisation will be unable to link the communications between a single private
individual and different government agencies to each other.
Note that the scope of this includes also traffic ingress, routing, and
similar - access patterns on the side of either homeserver must not be
accessible to the organisation operating the respective other homeserver beyond
the required federation traffic.
Thus "homeserver" here means potentially the whole Matrix service for a user
group.

Neither will two or more agencies themselves be able to draw such conclusions
by exchanging any metadata available to them.

<!-- This is an optional element. Feel free to remove. -->
## Pros and Cons of the Options

### A single organisation operates all Matrix homeservers

* Good, because a single team can be responsible for operation of all servers,
  reducing the need for coordination and building redundant structures.
* Neutral, because while the need to traverse networks is reduced, the Zero
  Trust principles still apply.
* Bad, because metadata about both government and private individual usage is
  stored in a single place and could be correlated to profile user activities.

### Government and Citizen homeservers are operated by separate organisations

In the context of this project, it could be possible for the involved
realisation partners, e.g. FITKO and BMDS, to fill these roles.

* Good, because homeserver operators will not be able to correlate private
  individual activity with specific government agencies based on metadata, if
  ADR-ZZZZ <TODO: update reference> is also used.
* Good, because government agencies will not be able to correlate private
  individual activity among each other based on metadata, as long as ADR-ZZZZ
  <TODO: update reference> is also used.
* Bad, because operational complexity increases under the circumstances of
  operation needing to be handled by separate organisations.

<!-- This is an optional element. Feel free to remove. -->
## More Information

* [ADR-0001](0001-separating-public-administration-and-citizen-homeservers.md)
* [ADR-0003](0003-broader-federation.md)
* [ADR-ZZZZ](ZZZZ-sealed-sender-recipient.md) <TODO: update reference>
* [Discussion about "sealed sender" in context of the Matrix spec](https://github.com/matrix-org/matrix-spec/issues/549)
* [Workshop minutes day 3, chapter 3 "Multiple accounts/ portable identities & Multiple Matrix servers"](https://nextcloud.fitko.de/f/2053632)
