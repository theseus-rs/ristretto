# Helpable publication: design constraints and model evidence

Status: proposed runtime redesign. The collector still uses counted admission or
its legacy gate. No helping protocol is enabled by this document or its tests.
This advances stage D's requirement to establish linearization points before
replacing admission; stage E's graph-retention problem remains separate required work.

## What the current implementation makes atomic by exclusion

`GarbageCollector::register` prepares object, identity and young-index guards,
then enters publication admission. It initializes birth/retention state, inserts
three index memberships, shades the new descriptor and publishes tracing work.
Admission keeps both domain phases stable throughout these effects. A thread
preempted after successful preparation therefore prevents phase closure. The
try-lock preparation fixes waiting before admission, not this admitted interval.

Other operations also depend on this exclusion: owning-root publication/removal,
weak eligibility and upgrade, barriers, promotion, and reference decisions. A
replacement must cover each operation before removing its admission. Keeping a
`MetadataRef` or storage lease does not retain the object's outgoing graph.

## Rejected shortcuts, with executable witnesses

Run `cargo test -p ristretto_gc --test helpable_publication_model -- --nocapture`.
Four tests intentionally expect a specific panic from a rejected design. They
are **counterexamples under removed admission**, not failures of the currently
admitted collector. The first witness also has a preserved red run before its
expected-panic annotation was installed.

| Rejected change | Forced schedule | Consequence |
| --- | --- | --- |
| Check a record's completion bit before replaying existing operations | Helper A sees incomplete and stops. B completes epoch N and acknowledges; the collector begins N+1. A resumes `MarkWord::mark(N, ...)`. | The production mark primitive accepts N and overwrites N+1. Its current admission/epoch precondition cannot be discarded. |
| Replay registry insertion after another helper acknowledges | A sees incomplete and stops. B inserts identity 1; retirement removes it and identity 2 reuses the address. A repeats insertion of identity 1. | An address-keyed last-write-wins index can resurrect or replace an identity. Checking completion separately is insufficient. |
| Infer completion because another helper already set the mark | A wins marking and stops before queue insertion. B sees the mark and acknowledges the record. | Closure can see no tracing task despite untraced outgoing edges. |
| Publish a canonical per-object cell without coordinating phase closure | A reads an open phase and stops. The collector sees no queued cell and closes. A publishes using the old phase. | Atomic object membership alone does not establish a safe announcement/closure handshake. |

An exclusive helper claim is also insufficient: it merely transfers the stalled
owner dependency to whichever helper acquired the claim. Preparation may be
exclusive while it has no externally visible effects, but public completion must
not require that particular thread to resume.

## Candidate atomic relations

The test-only `PublicationCell` uses one CAS to change a generation-tagged record
from Prepared to Queued. Queued represents **both** logical membership and a
persistent traversal requirement; no separate index/mark/queue side effect follows
the CAS. Other transitions are Queued → Traced → Retired, or cancellation from
Prepared → Retired. Reuse changes the generation and refuses numeric overflow.

Seven positive Loom checks cover competing helpers, a stopped original publisher,
late replay after retirement/reuse, cancellation versus commit, generation
exhaustion, and two phase-coupled cases. The latter use a separate, one-record
`CoupledCell` abstraction in which announcement and both domain boundaries share
a CAS word. Announcement either wins and leaves traversal visible, or closure
wins and rejects the old proposal. After traversal, both domains can close while
the original publisher remains stopped; its later CAS cannot repeat an effect.

These models establish only these small atomic relations. `PublicationCell` does
not implement global discovery or phase coordination. `CoupledCell` does not scale
to multiple descriptors, epochs, catalogs or the full reference state machine.
Neither models payload pointers, allocator behavior, trace callbacks, destruction,
roots, graph traversal or physical reclamation. Its Retired state is not evidence
that the current collector reclaimed anything. No runtime memory-safety or
wait-free-progress claim follows from the green model tests.

## Required runtime representation and transition work

Runtime prerequisite added (2026-11-09): the fallback tracing FIFO now implements
owned prepared append records with bounded helping attempts and idempotent
completion after consumption. Its predecessor identity and irreversible insertion
evidence replace replay of queue mutations; last-owner cancellation refunds private
preparation. Production-state Loom models and forced native schedules exercise the
actual queue transition implementation. See [the queue protocol](../PUBLICATION.md#contended-mark-publication).
This establishes a reusable durable-work primitive. Registration still does not
announce records, replace canonical membership, or let helpers release admission.
Existing collector calls keep the record local. None of the integration gates
below is completed by this prerequisite.

Runtime experiment added (2026-11-13): the optional owning identity index replaces
numeric lookup and retirement with versioned owning links, exact-key/address removal,
and bounded traversal/maintenance units. Stalled-reader/remover schedules, five
production-state Loom models and a deliberately broken predecessor-CAS variant
exercise its ownership rules. This is **not** the canonical catalog: the object
address map still owns authoritative metadata. Measurements reject this separate
index representation as the default because of allocation, lookup and reclamation
overhead. It remains behind `experimental-identity-index`; default identity maps
are restored. See [the protocol](../PUBLICATION.md#experimental-owning-identity-index).
Embedded interior markers subsequently reduced link-state allocations. Shared
insertion intents now add immutable expected/proposed bucket versions and an
irreversible insertion certificate recorded before every head replacement. Helpers
can retarget invalidated proposals without mutating already published successors;
late CAS attempts fail even when the same node becomes the head again. Eleven
production-state Loom models cover the combined list, including competing helpers,
retargeting, stopped winners and retirement before a late CAS. Separate mutations
that omit the replacement certificate or compare a fresh head fail their forced
schedules. This extends the primitive's completion semantics, not phase admission.
Registration still keeps the intent local; canonical membership, discovery and
closure coupling remain required. Allocation/backing costs must be reduced or
replaced while integrating that membership before a default migration is considered.

The next implementation must make descriptor membership authoritative independently
of a publisher's stack and the three mutable indexes. This is a representation
change, not a new completion flag around `register`.

| Boundary | Required linearization and ownership | Current work still needed |
| --- | --- | --- |
| Prepare | Allocate metadata/record space and reserve logical/physical credits before announcement. The constructor and its initial roots protect children. | Define bounded slots and cancellation ownership without retaining collector locks or admission during allocator/hooks work. |
| Announce | Make a fully initialized immutable request discoverable and coordinate that visibility with both phase boundaries. | Model a scalable catalog/phase handshake. A separate revision increment before or after linking leaves an unsafe gap unless the gap is durably represented and helpable. |
| Commit | Atomically publish canonical membership and all required traversal obligations using the validated request identity/phase. | Replace separate index insertion, mark stores and queue insertion as the logical commit. Helpers may privately prepare alternatives; losing alternatives must have no external effects. |
| Observe | Every collector scan and handle reconstruction validates the canonical identity/state. | Object/identity/young indexes may become accelerators, but must not hide a live descriptor, replace a newer identity or authorize access to a retired payload. Catalog traversal remains authoritative and bounded. |
| Acknowledge | Acknowledge only after effects are durable and a late helper can no longer change them. | Couple acknowledgement/removal to record identity. A separate claimed bit followed by count decrement can itself strand closure; decrement followed by a completion store permits duplicate decrement. |
| Retire | Irreversible transition of that exact canonical identity; stale proposals cannot republish it. | Preserve weak/final/phantom boundaries, graph reachability, descriptor leases and exactly-once deferred destruction. Secondary-index cleanup must not mutate a replacement identity. |
| Reuse/rollover | Old tickets and epoch snapshots cannot match new operations. | Bound retained record/catalog storage without a history chain. Current quiescent mark reset/revision rebase proofs must be revised if acknowledged helpers can still retain old snapshots. |

The production `MarkWord` is intentionally unchanged. Merely rejecting older
epoch numbers would address one witness while leaving index resurrection,
mark/queue publication, epoch reset and announcement/closure races unresolved.
Likewise, swapping the global counter for a record list while preserving the
existing mutations would not satisfy this design.

Records must remain owned and enumerable after their submitting task migrates,
cancels or disappears. An original publisher which resumes after helping must
recover the committed result or the correct cancellation/failure result exactly
once. If commit won a cancellation race, ownership cleanup follows the committed
root/descriptor state; cancellation cannot turn it back into an unpublished value.

A fixed slot cap must not become an unaccounted global wait. Reserve record and
catalog bytes against explicit limits; exhaustion must have a declared retry or
failure path without holding admission. A stalled participant may retain its
record and reachable graph, charged to the heap, but must not retain an expanding
history of unrelated completed records. One poll/help attempt needs a bounded
amount of registry, barrier and queue work on native and clockless Wasm.

## Integration and qualification gates still open

Multi-descriptor experiment (2026-11-14): the isolated
[catalog packages](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/README.md) add fixed-capacity slot discovery and a
prepared controller operation coupling an exact cell update with both domains'
pending counts and phase versions. An installed operation is itself discoverable;
any helper can apply its immutable effect, certify completion and release the
controller. Retirement and slot reuse compare exact owning identities. Eleven
native fixtures and eight models cover stopped helpers, cancellation, late replay,
discovery, reuse and exhaustion; WASI/Miri/Tree Borrows checks pass. Three deliberate
protocol mutations fail. No production collector calls these packages.

This extends the small atomic-relation evidence, including the discovery problem,
without implementing metadata/initial-root ownership, byte reservations, actual
reference floors or all admission callers. The alternative immutable tree incurs
path-copy allocation and unbounded final-snapshot destruction. The descriptor
screen still has six allocations per insertion, and ArcSwap's internal thread
bookkeeping/reader scans are not a bounded-work proof. Both representations and
the complete successful/failed evidence are retained; none closes the gates below.

1. Implement and model scalable discovery/closure coupling, canonical membership,
   and durable traversal publication together. Cover both domains and reference
   floors, early/late helpers, cancellation, slot reuse and epoch rollover.
2. Migrate descriptor registration with full-collector forced suspension at every
   transition. Then migrate roots, barriers, weak decisions, promotion and the
   remaining callers of admission. Verify each operation's graph ownership.
3. Complete E's held-writer and opaque-trace/handoff protocol. Durable publication
   does not make an inaccessible outgoing graph safe to reclaim around.
4. Verify budgets, retained metadata and descriptor storage under repeated reuse,
   stalled participants and teardown; exercise Miri/sanitizers where applicable.
5. Measure publication/root/barrier costs and contention progress with isolated
   frozen builds, then integrate native/Wasm and J0–J5 qualification. No defaults
   or release gate change until the full required matrix passes.

The model suite is a design filter. It does not replace those integration tests,
measurements, the native <1 ms pause gate, or hour/day qualification.

## Preallocated state ownership experiment

The [pooled variant](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/catalog-pooled-prototype/README.md) supplies fixed
backing for the numeric controller and membership values. Owning expected pointers
keep slots unreusable; the final release acquires prior owners, destroys the value,
and only then advertises free storage. Four exact-state pool models, native
concurrent loads, Miri/Tree Borrows and sanitizers exercise that contract. Private
preparation failure and cancellation return their slots; old helpers retain a fixed
set rather than later operations. Reuse/ordering mutations fail.

The paired representation count screen reduces a numeric lifecycle from 18 system
requests to one, while increasing retained backing. It does not charge this backing
to a real heap or enforce a participant bound/reserve sufficient for independent
progress. An undersized pool can refuse preparation; failing closed is not a proof
of reclamation progress. Pool initialization/teardown and ArcSwap internals also
remain outside the bounded-work argument. The next ownership integration must
address these limits together with actual metadata, initial roots and the full
phase rules before any admission migration is safe.

## Runtime byte-ownership prerequisite (2026-11-14)

Logical reservations now transfer into real metadata before publication. A
constructor unwind after registry insertion therefore cannot refund a still-owned
allocation. The sole storage destructor refunds only after payload/backing release;
leases and old metadata references have distinct tested effects on that charge.
See [validation](../VALIDATION.md#descriptor-owned-allocation-charges-2026-11-14).

This does not integrate metadata into the numeric catalog. Initial strong roots,
pending conditional-owner counts and the client's initial root still have separate
construction ownership. A future shared request must preserve nested construction
semantics and define cleanup independently of a helper's ambient construction TLS.
Existing root removal, deletion barriers and slot reuse remain admitted effects;
taking a vector under an exclusive helper claim does not make them helpable.

Construction scope exit now moves whole registration batches and offers explicit
detached cleanup that cannot adopt into a helper's unrelated TLS scope. Empty-parent
adoption reuses vector backing. These are local ownership/cost primitives; cleanup
still retains admission and can be linear. A separate metadata-hook-unwind audit
also finds an initial-parent registration retained after private storage destruction
on both the previous runtime and the batch revision. The subsequent
[ordered cleanup repair](../VALIDATION.md#ordered-initial-root-cleanup-2026-11-14)
finishes metadata hooks before initial-root installation and keeps metadata owned
by the constructor across borrowed registration. Its private guard removes the
root before private storage destruction. This closes the reproduced local
cancellation/order gap; it does not transfer initial graph/client protection into
a shared record or make any admitted effects replayable. The original
[batch evidence](../VALIDATION.md#construction-root-batches-2026-11-14) remains available.

A subsequent [refund repair](../VALIDATION.md#guarded-refunds-and-borrowed-reclamation-2026-11-14)
adds scoped payload cleanup before finalization and scoped credit refund after the
storage claim. It preserves charges through live storage and refunds after release,
including the two reproduced diagnostic-unwind cases. Native reclamation can reuse
a validated collector borrow. These local ownership guards do not publish shared
requests, release phase admission, retain outgoing graphs or complete callback work
on another participant's behalf. Initial-root/canonical-state migration is unchanged.

## Bounded acquisition candidate (2026-11-14)

The isolated [generation-tagged pool](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/catalog-tagged-pool-prototype/README.md)
offers a fixed number of atomic API operations per owning-load/compare attempt.
Slot generation and reference state change atomically, so a delayed token cannot
increment a reused value's count. Busy initialization/destruction excludes both
acquisition and reuse, and generation exhaustion fails closed. Failed proposals
retain ownership for caller cleanup; successful replacement releases the previous
link count while an expected owner still prevents value destruction.

Native/WASI, Miri/Tree Borrows, sanitizer and exact-state model checks pass. Four
deliberate faults fail. These are local ownership checks, not a complete pointer
model. The [tagged catalog follow-up](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/catalog-tagged-prototype/README.md)
now replaces every numeric owning link, including optional discovery and cell
storage. The previous ArcSwap catalog remains a comparison control.

Fallible acquisition adds a required result distinction: Pending retains the same
request because it may still commit, while Retry proves its expected immutable
controller was superseded. A caller cannot discard Pending and assume no effect
occurred. Discovery contention likewise cannot be treated as an empty slot. A
failed post-CAS observation cannot certify the effect; another helper must finish
it. Certification still precedes releasing the Busy controller.

Zero loop allocations do not establish a progress reserve. Public snapshots,
attempts and cell owners can retain unlimited distinct versions. A production API
must enforce per-participant retained-state limits, account for current registry
ownership and held retired cells, and derive charged capacities that leave an
independent helper able to proceed. Eight unsuccessful pool probes also do not prove
exhaustion; resumed selection or reserved slots need a separate progress argument.
Arbitrary last-owner destruction remains outside the atomic-attempt bound. No
production admission, callback or qualification gate changes.

Private control preparation now retains partial successes in a `Preparation`
across one-slot polls. The prior convenience path could discard two successful
allocations on every failed search for its third slot, repeatedly failing even in
a quiescent pool with sufficient reusable slots. A new fixture reproduces this
failure and its repair. The original expected controller remains fixed across
polls, and cancellation returns private outcomes and effects without publication.
See [the evidence](../VALIDATION.md#resumable-private-catalog-preparation-2026-11-14).

This addresses selection during private preparation; it does not enforce quotas.
Progress-sensitive callers must retain the preparation instead of repeatedly using
the finite convenience call. Quiescent progress assumes enough reusable generations
and does not establish contention fairness. Participant admission, reserved helper
capacity, byte accounting and real graph ownership remain required.


## Bounded numeric participants (2026-11-14)

The tagged catalog facade now confines owning snapshots, cell handles, preparations
and attempts to one inline state per non-cloneable participant. Separate mutator
and helper registration counters reserve independent helper slots; request state
drops before its quota can be reused. Returned keys contain only catalog identity,
record identity and registry position. Private cell creation retains membership
backing across selected cell-slot misses. All owning observations require a mutable
idle participant, so a caller cannot accumulate uncharged owning snapshots through
this API.

Conservative role counts derive pool capacities from participant and registry
limits. Forced stops cover private requests, unannounced cells, installed work and
helpers, with an active reserved helper completing/retiring work and reusing slots.
This controls numeric ownership; it does not charge bytes, guarantee per-participant
fairness, handle exhausted generations, publish real transitive graphs or replace
any production admission. The raw validation API still permits unbounded owners;
only instances accessed through the facade satisfy its role assumptions. See the
[report](../../target/gc-implementation/catalog-quota-REPORT.md).

## Charged numeric backing (2026-11-14)

The facade's configured constructor reserves requested backing plus all allowed
inline participant storage from an external Budget. Its opaque handle prevents
ordinary outer-Arc drop: `Arc::into_inner` releases that allocation before the
catalog fields release pool ownership and return credit. Copied keys do not retain
charges; stopped participants do. Native/WASI allocation audits compare calculated
layout with actual requests and require refund after final free. Two deliberate
accounting faults fail on both targets. See the [report](../../target/gc-implementation/catalog-charge-REPORT.md).

This is a scoped logical budget with a frozen, audited Arc-layout assumption.
External budget storage, allocator bookkeeping and RSS are excluded. The private
unlimited convenience constructor does not provide a configured shared heap limit.
The collector still does not use this catalog or accounting object. Finite generation
exhaustion, contention fairness, real metadata/initial graphs/reference floors and
migration of every admitted operation remain required before admission removal.


## Finite generation failure (2026-11-14)

The numeric facade now terminates private preparation after a selected allocation
reports permanent generation exhaustion. The raw pool returns the rejected value
and a detailed reason; ordinary misses still retain partial progress. Existing
owners remain valid at the final generation. Installed requests complete using
prepared backing even after exhaustion is observed. The sticky observation can
conservatively reject after a failed begin operation without proving that every
slot is exhausted. Default generation width is unchanged.

A reduced-width red driver, exact claim race model, native/WASI memory-safety
matrix, actual allocation audits and three new fault variants validate this scope.
See the [report](../../target/gc-implementation/catalog-exhaust-REPORT.md).
This is finite-resource failure handling, not backing rotation, scheduler fairness
or production admission removal. The full reference state machine and real graph
ownership are still required.


## Canonical numeric tracing (2026-11-14)

Tracing membership now couples per-domain epoch, retention and work tickets with
phase pending counts in one controller commit. Stronger discovery supersedes its
old ticket without double counting; acknowledgement compares the exact ticket and
preserves retention. Helpers publish no separate mark store or queue effect.
The six-floor state machine refuses discovery at retirement and refuses retirement
while either domain has work. Exact comparisons protect newer epochs and reused
records from old helpers. See the [evidence](../../target/gc-implementation/catalog-trace-REPORT.md).

TraceInsert publishes white numeric membership, not a real allocation or its initial
roots. AlreadyMarked is a copied-observation rejection, not a root or permission to
access a payload. The two domains currently cover the entire numeric population.
Birth exemptions, young/old handoff, conditional-slot clearing/delivery, graph
snapshots and metadata ownership still require canonical operations. Production
admission cannot be removed on the strength of these numeric fixtures.
