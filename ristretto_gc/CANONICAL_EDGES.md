# Canonical graph edge inventory

The experimental tagged catalog now represents strong graph edges with exact,
non-owning source/target identities. Edge publication, replacement, removal and
child-root activation use the existing helpable controller. Full-domain tracing
visits the complete fixed edge inventory before acknowledging a source ticket.
This is a catalog protocol and bounded facade, not a selectable graph GC backend.
The concrete GC adapter still admits only leaf payloads. Defaults are unchanged.

An edge slot progresses Prepared → Live(source, optional target) → Retired. Its
source identity is immutable after publication; its target can change. Identities
include object identity and discovery location, so reused slots cannot alias old
objects. Edge slots and versions use prepaid generation-tagged pools. Creation
keeps partial state across misses and performs one selected pool probe or
announcement attempt per poll. Numeric generations never wrap.

Publishing requires a live external source root and, for a non-null target, a live
external target root. Replacement couples the old target's deletion barrier and
the new target's insertion barrier with the exact edge-state change and both
domains' counted work. Same-target replacement produces one proposal per target,
avoiding duplicate work from two effects based on the same old membership.
Fallible ownership acquisition occurs before the irreversible COMMIT decision.
Helpers apply the edge with an exact expected-version CAS before certifying the
controller. A delayed helper cannot overwrite a later edge version.

Reading an observed target identity alone provides no payload authority. The
RootTarget transaction validates the current edge, live source root and exact
Prepared child root against one controller snapshot, then activates that root.
Replacement or source revocation invalidates an older prepared attempt. A child
root that commits first has independent lifetime protection after the edge changes.
Typed runtime child reads and their admission boundary remain to be implemented.

An EdgeHandle is a temporary external creation owner, never a payload field. Its
Drop posts durable removal intent before returning its quota. After successful
publication, release_edge_to_source consumes the handle and returns only an inert
EdgeKey. The source-owned inventory contains no strong Cell or catalog references;
cycles of graph identities therefore do not create Arc ownership cycles. A helper
may remove an orphan edge only after the exact source is Retired or its discovery
identity has been removed/reused. Failed observation is contention, never absence.
Exact retired-slot release cannot clear a later occupant.

Each EdgeTraversal retains one source, domain, epoch, ticket and retention level,
plus a private cursor and at most one preparation/attempt. A poll observes at most
one inventory slot or advances one bounded request stage. Matching targets are
shaded at the source's retention; the cursor advances only after completed shading
or an observation of equal/stronger marking. Failed owning reads do not advance.
Only a full inventory pass can acknowledge the captured ticket. Stronger source
work supersedes the traversal. Insertion/deletion barriers protect changes behind
the cursor. Another helper can restart the same ticket from zero while a scanner
is stopped; there is no exclusively owned shared traversal cursor.

With E > 0, both guarded and numeric bounded acknowledgement commands route through
this traversal. Starting Young or Old domains is rejected; transitive generational
semantics are not implemented here. The raw experiment still exposes numeric
operations for models and is not a safe graph-runtime API. Inventory completeness
for actual payload fields must be enforced by the future sealed runtime adapter.

Traversal currently costs O(E) per source ticket and graph mutations serialize
through the controller. These are explicit performance limitations. Source-indexed
inventory and a measured mutation fast path are required before large-graph claims.
Bounded transition counts do not bound host allocation, arbitrary Record callbacks,
destructors, wall time, scheduler fairness or browser host-turn latency.

## Storage ownership

N is object capacity, M caller-root capacity, Q conditional-reference capacity,
E edge capacity, and K total registered mutator/helper quota. Handles do not expose
raw owning shares; participant requests and handle tokens precede their quota
owners in destruction order. Stopped destructors therefore remain charged.

The previous [reference census](validation/catalog/catalog-tagged-prototype/REFERENCES.md)
bounds two potential private/installed/helped controller families per participant,
plus the canonical head. Edge requests do not add another family: preparation and
attempt are successive states, and traversal owns at most one such request. A
controller still has two stable outcomes and no historical-controller chain.

Distinct edge-slot owners are E discovery entries, E exported/ready handles,
2K+1 potential controller families, K private creations and at most 2K transient
observations: at most 2E+5K+1. States add each retained slot's current version,
two expected/proposed versions per controller family, K private creation states
and 4K transient state observations: at most 2E+14K+3. Copies of a pooled owner
do not add another distinct state. Neither edge state nor slot owns a target Cell.

Source/target/child root checks fit the previous four transient-root observations
per participant. Two direct object effects fit the existing controller allowance.
Traversal additionally retains one source Cell and its current membership; its
temporary target/membership observations are finite and do not escape a poll.
Adding a deliberately loose 4K Cells and 8K memberships to the earlier census
covers these extra owners, including interrupted assignments and destruction.
Reference and edge requests share one participant state rather than accumulating
independent requests.

| Pool | Conservative retained bound | Pre-rounding reserve when E > 0 |
| --- | --- | --- |
| Controls | 7K+3 | 8K+8 |
| Edge slots | 2E+5K+1 | 2E+6K+4 |
| Edge states | 2E+14K+3 | 2E+20K+8 |
| Roots | 2M+5Q+20K+4 | 2M+6Q+24K+16 |
| Cells | N+2M+5Q+33K+6 | N+2M+6Q+40K+24 |
| Memberships | N+2M+5Q+52K+10 | N+2M+6Q+56K+32 |

Reserves round upward to checked powers of two within the tagged-pool limit.
Removing E restores the corresponding root/reference derivation, regardless of
configuration order. Reference pools keep their existing reserves. This census
assumes sealed facade quotas and says nothing about arbitrary owners inside R,
eventual scheduling, selected-slot contention or service beyond finite generations.

Footprint includes actual generic backing, the larger participant/controller
layouts, and E temporary EdgeHandles. Source-owned EdgeKeys require no owning
wrapper reservation. Payload backing, caller containers, allocator bookkeeping and
RSS are separate. The measured 840 native/WASI accounting rows have zero lifecycle
backing requests and exact final refunds. E=0 still incurs larger inline layouts;
enabling even one edge also selects conservative root/cell/membership reserves.
See [the evidence and cost plots](../target/gc-implementation/canonical-edge-REPORT.md).

Validation covers source-owned cyclic destruction, child-root races, failed reads,
stopped helpers, exact slot reuse, cancellation, quota lifetime and a finite native
writer/reader/helper schedule. It does not close the graph-runtime, arbitrary
callback, public backend, VM/JIT, pause, external G1/ZGC or endurance gates.
