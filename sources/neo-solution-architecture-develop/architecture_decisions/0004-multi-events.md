---
status: accepted
date: 2026-03-18
decision-makers: team::architecture
consulted: team::dev
---

# ADR-0004: Multi-Events

## Context and Problem Statement

The system needs to allow complex messages to be flexibly composed
out of different building blocks representing types of content.
For example, a message could consist of text, several files, and a poll.

## Decision Drivers

* Expected implementation effort
* Compatibility,
  e.g. [LB 3.2.3](https://gitlab.opencode.de/fitko/matrix-g2x/gitlab-profile/-/blob/main/Leistungsbeschreibung.md###323-demo-client-zur-darstellung-der-aktivit%C3%A4ten-von-fachbeh%C3%B6rden),
  and being future-proof

## Considered Options

* [Introduce a Multi-Event Nested Message Type](#introduce-a-multi-event-nested-message-type)
* [Composable Multi-Events via Event Relationships](#composable-multi-events-via-event-relationships)

## Decision Outcome

We select the option
"[Composable Multi-Events via Event Relationships](#composable-multi-events-via-event-relationships)"
given there are no significant known disadvantages as presented below.

## Pros and Cons of the Options

### Introduce a Multi-Event Nested Message Type

We create a new `dev.fitko.multi.v1` event to send a set of events atomically.
It MUST contain a `dev.fitko.multi.events.v1` content block.

Event contents SHOULD include a fallback text representation,
e.g. for potential support when using the Element Web client as a "Demo Client"
([LB 3.2.3]).
When sending a Multi-Event,
fallbacks of the nested events SHOULD be concatenated in order and
included in the Multi-Event so clients without Multi-Event support
can interpret them.

#### Multi-Event Content Block

We define the `dev.fitko.multi.events.v1` content block,
which is an array of other events.
It MUST NOT be empty and SHOULD contain at least two entries.
Arrays are inherently ordered,
so a sending client MUST send the content in the intended order
and a receiver also MUST show them in the same order.

The event representations inside the array MUST contain
the child events' `type` and `content`
as the rest of the attributes are redundant through the Multi-Event itself
and SHOULD be ignored by parsing clients.
This is similar to but still a reduced set of the required parameters
for sending a timeline event:
<https://spec.matrix.org/v1.17/client-server-api/#sending-events-to-a-room>.

Multi-Events MUST NOT contain further nested Multi-Events.

```jsonc
"dev.fitko.multi.events.v1" : [
  {
    "type": "$type",
    "content": { ... }
  },
  ...
],
```

#### Example

```jsonc
{
  // irrelevant fields not shown
  "type": "dev.fitko.multi.v1",
  "content": {
    "dev.fitko.multi.events.v1" : [
      event...,
      event...,
      ...
    ],
    "m.text": [ // optional text fallback
      { "body": "<i>Hello world</i>", "mimetype": "text/html" },
      { "body": "Hello world" }
    ]
  }
}
```

* Good, because the message is a single atomic event,
  avoiding issues of completeness and ordering.
* Good, because the design follows the KISS principle.
* Bad, because parsing now needs to consider nested events,
  diverging from regular Matrix significantly.
* Bad, because it violates semantics of Matrix Extensible Event design,
  including fallbacks and redundancy.
* Bad, because it risks running into Matrix size limits early.

### Composable Multi-Events via Event Relationships

The proposed user experience we are solving for is notably comparable to
native Matrix threads within a single room per case,
where each main timeline message is either an informative or inquiring message
sent by the government agency,
with private individual replies sent into threads
which have the inquiry messages as roots.

If we consider multi-part events from either side,
this can also easily map into a thread,
enabling the use of Matrix's relationship and threading APIs
to handle relevant messages efficiently.

Advantages:

* more "Matrix native":
  avoid that the nesting breaks the Extensible Events fallback semantics
* avoid nested parsing in clients
* reaching event size limits later, with a built-in solution for chunking
* can be achieved with current Matrix semantics,
  with a clear roadmap to extensibility for additional features

Disadvantages:

* fewer ordering guarantees, e.g. in cases of sending retries
* atomicity: a message is either received completely or not at all
* Events can only relate to one other event via the generic relationship field,
  and one additional event via rich replies.
  This may lead to clients needing to consider many recursive relationships.
  While there is an API that allows to fetch recursively related events,
  there are few guarantees about the results being the complete set of sought events.
* By default, Matrix will send push notifications for each event.
  This is desirable behaviour in terms of our clients getting notified
  of the need to fetch new message content,
  but at the same time there should only be a single user-facing notification
  for each thread.
* Not being a single event,
  the natural property of limiting the amount of events specifying inquiries
  to just one per thread is lost.

However, the disadvantages can be resolved or mitigated
by the following methods:

* Ordering can be guaranteed by chaining events through a relation:
  a subsequent event needs the previous event to be sent
  (received by the sender's server) to get the event ID it needs to create
  the relationship.
  This is comparable to a reply chain rather than a thread.
  In principle,
  this design can lead to the creation of in-trees (directed rooted trees)
  rather than chains.
  Given the properties of end-to-end encryption guaranteeing event authenticity,
  this would mean that a sender of such events is a bad actor,
  so we discard any such chains detected on the receiving side.
* A client receiving a subsequent event can check if it also has
  the referenced event and use the Matrix APIs around related events to fetch it
  to assure completeness in direction of the message head.
* We can add a "length of multi-part message" field to the first event
    to also assure completeness in direction of the message tail.
  A client could keep fetching related events using
  the Matrix related events APIs until it finds the tail of message
  and consider the message chain invalid otherwise.
* There is no MSC for relationships of Extensible Events yet,
  however there is
  [MSC3051](https://github.com/matrix-org/matrix-spec-proposals/pull/3051)
  proposing a scalable relation format,
  which allows events to relate to an arbitrary amount of other events
  with any mix of relationship types.
  We could leverage this approach to implement:
  * The chain for ordering guarantees,
    using a custom `dev.fitko`-namespaced relationship.
  * Threads, to comfortably fetch all events from a government message,
    including potential answers from private individuals.
  * Specific relations to inquiries to authorise sending answers and follow-ups
    from private individuals,
    similar to
    [poll answers](https://github.com/matrix-org/matrix-spec-proposals/pull/3381).
* Push notifications can be configured via Matrix's flexible
  [push rules](https://spec.matrix.org/v1.17/client-server-api/#push-rules)
  system.
  As events are generally encrypted,
  they will all be of the encrypted event type with opaque content,
  except for consciously unencrypted content,
  for example typically relations,
  allowing only for some filtering.
  Push rules allow configuring by any event content even in encrypted rooms,
  such as event types or other fields depending on event schemata.
  Hence, push rules for encrypted rooms need to be evaluated client side,
  and the client merely receives a wake-up call to start fetching new events
  via push.
  The rules can then be configured to only send one notification to the user
  per thread sent by the other side.
* To an extent, the Inquiry-as-Mixin design faced the same issue,
  however in the thread variant,
  the events are ordered.
  This allows mitigation of the issue by defining any subsequent inquiry events
  in the same thread as invalid.
  In the future,
  this model also allows us to redefine the behaviour so multiple inquiries
  per thread become possible.

The resulting schema is documented in
[chapter 5](/05_building_block_view.md#composing-multi-events).
