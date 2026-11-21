# Bundled canonical catalog and pool

These private modules are included in the `ristretto_gc` package and compiled only
with `experimental-canonical-runtime`. They require no unpublished registry crate
or sibling validation project. Enabling the feature does not change the ordinary
collector or select this pool for its allocations.

## C04: exact-generation pool ownership

The first slice carries pool setup, bounded selected-slot allocation, fallible
owning acquisition, non-owning observers and final-release destruction. It keeps
the frozen state transitions and memory orderings. Formatting, documentation and
test-only lint fixes do not change the protocol. The link API is a separate slice.

An observer retains backing but cannot acquire a replacement generation. The
final release holds the slot busy through destruction, including unwinding. A
slot whose generation budget is exhausted cannot wrap. Allocation failure names
the selected slot, not the entire pool. Arbitrary destruction and backing setup
remain outside the bounded attempt contract.

Run the eight native ownership regressions and six exact-state Loom models with:

```sh
cargo test --locked -p ristretto_gc --features experimental-canonical-runtime --lib canonical_pool
cargo clippy --locked -p ristretto_gc --features experimental-canonical-runtime --lib --tests -- -D warnings
```

The five portable regressions also run on `wasm32-wasip2` with Wasmtime. The Loom
models compile the production state source with Loom atomics; they cover state
ordering, not raw-pointer provenance or aggregate pool ownership. `loom` is a
native-only development dependency, absent from the production dependency graph.

## C05: conditional owning links

A link retains one exact-generation pool owner. A failed or foreign proposal
returns its owner; successful replacement releases the old link count while the
expected owner still pins its value. The existing native stopped-reader and
concurrent replacement tests exercise pointer ownership and balanced destruction.

The CAS expression is factored without changing its orderings into
`canonical_pool/link_state.rs`. Two additional Loom models compile that exact
expression with Loom atomics and test competing proposals and stale replacement.
Those models cover numeric selection only; the real pool tests cover the ownership
transfer and foreign-pool checks. The combined pool selector runs 20 native tests
(including eight models) and seven portable WASI tests.

## C06 prerequisite: numeric phase and trace state

The phase/trace slice carries the frozen checked epoch, ticket, work-count and
reachability computations independently of owning publication. Five frozen
numeric regression tests pass on native and WASI, including exhausted counters,
birth versus reachability, and cross-domain retirement. This slice makes no
ownership, publication or collection-progress claim. Run the same commands above
with the `canonical_catalog` selector for its tests.

## C06 prerequisite: retained links and vacancy preference

The catalog wrappers distinguish failed acquisition from an empty optional link.
A stale replacement preserves the installed generation. The frozen vacancy hint
is numeric and expires after a bounded number of competing deferrals; it grants
no slot ownership or publication authority. Three direct regressions accompany
this extraction, passing on native and WASI alongside the five numeric tests.
The private-link assertions reject an internal foreign-pool programming error;
public proposal rejection remains the pool API's fallible result.

## C06 prerequisite: typed catalog backing

The typed backing slice introduces the frozen record/guard contract, cell and
controller storage layouts, and pool setup. It exposes no request installation
or payload access. Controller variants are reserved here so footprint accounting
uses the actual object-controller layout before its operations are enabled.
Two tests verify empty discovery slots, checked backing sizes and release of all
pool owners. Native/WASI catalog selectors now each pass ten tests.

## C06: bounded participant registration and budget ownership

Catalog setup reserves its declared backing, participant wrappers and optional
caller storage before constructing pools. Mutators and helpers have independent
single-CAS admission quotas. A unique lease retains the catalog and releases its
role quota on drop. The final opaque handle releases its outer Arc allocation,
then catalog backing, then its budget charge, matching the frozen ownership order.

At this tip participants have no request storage. Footprint accounting uses the
actual participant type and expands with later request states. There are no
root, conditional-reference or edge token capacities yet. The preview's Q=0
restriction permits the conditional-reference protocol to remain out of scope.

Three portable regressions check quota exhaustion/reuse, budget rejection,
last-participant refund and overflow. One Loom model compiles the production
quota CAS and release expressions and checks competing registration. Catalog
totals are 14 native / 13 WASI tests, with strict Clippy passing.

## C07 prerequisite: resumable record creation

Private creation retains the exact record through one selected backing attempt
per poll, then announces it through the owning slot CAS. The frozen rotating-slot
and expiring-preference policy is preserved. A failed/foreign observation does
not certify an empty slot or consume a request. Four portable tests exercise
private-stage cancellation, competing announcements, exact drop counts and
retry after failed owning acquisition. Native/WASI catalog totals are 18/17.
Publication into tracing membership remains a later controller operation.

## C07 prerequisite: retained publication preparation

Private preparation captures one expected controller and both possible phase
outcomes. Each poll retains prior successes and makes one selected-slot backing
attempt. It cannot refresh its expected revision after a miss. Two portable
regressions check retained allocations, foreign-catalog rejection and complete
refund after cancellation/drop, without applying membership effects. Native/WASI
catalog totals are 20/19. Controller installation remains separate.

## C07a: helpable object publication

A selected controller retains its exact cell effect and immutable successor.
Helpers acquire fallible effect ownership and the record admission guard before
choosing commit, apply the exact version CAS, publish the completion certificate,
then release the controller. An old successful attempt reports its original
completion without applying the effect twice. Failed acquisition remains Pending.

Four actual catalog tests cover a stopped installer, stopped effect, acquisition
failure and two concurrent helpers. Two Loom models compile the exact selection
CAS and certificate store to check commit/cancel exclusion and effect visibility.
They do not model pointer protection or the record guard; those remain actual
pool/catalog tests. Native/WASI catalog totals are 26/22; strict Clippy passes.

## C07b: participant request custody and cancellation

A participant owns one inline creation/publication request. Its state is dropped
before its lease; no raw owning snapshot escapes the facade. `cancel` distinguishes
an unselected proposal from a commit decision that already won. Dropping an
already selected request leaves the winning operation discoverable and helpable.
Abandoning a Prepared record retires that exact identity, allowing slot reuse.

Seven new portable tests cover private boundaries, installed undecided requests,
every commit/effect/certificate/release boundary, exact payload drop counts, busy
submission returning the same record, quota reuse and refund ordering. The
snapshot API retains the frozen name `possible_phase_outcomes`; it copies possible
outcomes without helping and makes no collection-count claim. Native/WASI catalog
totals are 33/29; strict Clippy passes. The inline state size is included in the
footprint for both numeric and guarded participants. Root/edge operations remain
subsequent prerequisites for the public preview adapters.

## C08 prerequisite: unique root backing

A root token retains its exact target and root-pool owner. Drop posts durable
removal intent; it does not change canonical visibility or authorize slot reuse.
Private creation can be cancelled without publishing a root. The Q=0 extraction
omits conditional-reference root roles and their reserved locations; ordinary
root generations, ownership, announcement CAS and removal-intent ordering remain
unchanged. Two portable tests cover token retention, cancellation and foreign
rejection. Native/WASI catalog totals are 35/31, with strict Clippy passing.

## C08 prerequisite: phase-coupled root publication

Root activation shares the object's publication decision and precedes its
completion certificate. A bare Live root status is insufficient: the retained
root observer helps and verifies a stable controller before reporting live
publication. Prepared-root cancellation and exact late slot release preserve
unique token custody. Two portable regressions and one native competing-helper
case cover these boundaries. Native/WASI catalog totals are 38/33; strict Clippy
passes. Bounded root-token admission follows separately.

## C08 prerequisite: bounded root admission

Root ownership has its own finite token quota and retains the backing charge
through target/token destruction. A participant owns private creation and root
publication until it transfers one completed token. Failed discovery remains
Contended. Four portable tests cover cancellation at each boundary, slot reuse,
retained catalog charges and rejection of currently unsupported backing. Numeric
capacity formulas retain the frozen API; nonzero reference or edge capacity is
rejected until the corresponding supported storage is wired. Native/WASI catalog
totals are 42/37; strict Clippy passes.

## C08 prerequisite: rooted allocation transaction

The guarded participant retains the complete record-to-root transaction, including
private record storage, root quota, resumable publication and one result token.
Cancellation preserves durable abandonment/removal requests; helper-completed
publication is recovered only after guarded admission and stable validation.
Automatic placement keeps the frozen bounded rotating-slot preference. Six
portable frozen regressions cover every private stage, helped results, failed
observations and simultaneous object/root collisions. Native/WASI catalog totals
are 48/43; strict Clippy passes. Root removal/scanning follows separately.

## C09 prerequisite: guarded phase transitions

Frozen tracing transitions couple membership tickets and pending counts. Phase
advancement and finish reject unscanned roots and pending work; `FinishEpoch`
retains the sweep's original epoch. Q=0 requires no conditional-reference scan.
Four portable tests cover stale epochs, outstanding obligations, stale tickets,
and invalid phase input. Native/WASI catalog totals are 52/47; strict Clippy passes.

## C09 prerequisite: root barriers and bounded scanning

Root activation/removal couples the retained target's tracing obligations with
visibility. A scan inspects at most eight slots and commits at most one target or
root effect; any failed owning observation leaves the cursor unchanged. Four
portable regressions cover scan bounds, removal intent, failed observations and
foreign/prepared activation rejection. Native/WASI catalog totals are 56/51;
strict Clippy passes. No callback or scheduler-fairness bound is claimed.

## C09 prerequisite: bounded guarded phase commands

The participant exposes the frozen guarded tracing/phase command set. A failed
private preparation diagnoses copied preconditions without granting access;
still-valid requests retry. A queued `FinishEpoch` never rebases onto a newly
started collection. Three portable tests cover that real supersession, private
phase cancellation, and invalid/foreign rejection. Native/WASI catalog totals
are 59/54; strict Clippy passes.

## C09 prerequisite: bounded root maintenance commands

The participant now supports frozen root activation, requested removal and
single/batched scan commands. Three restored allocation regressions cover a
helper stopped between target/root effects, revoked helper results, and discard
after commit has already won. An additional portable facade test checks scan
batch bounds, cancellation and invalid domains. Native/WASI catalog totals are
63/57; strict Clippy passes. This completes the root/phase API needed by the leaf
maintenance adapter; strong-edge storage is still rejected at setup.

## C10 prerequisite: exact edge backing and geometry

Edge tokens own bounded preparation storage and post removal intent on drop.
Live identities contain source/target keys rather than owning payload links. The
fixed source-degree layout is checked at setup; failed discovery is distinct from
an empty slot. Three portable tests cover private cancellation, competing creators,
foreign rejection, geometry and intent-only drop. Native/WASI catalog totals are
66/60; strict Clippy passes. Bounded edge-capacity admission remains disabled.

## C10 prerequisite: paired membership and edge effects

A frozen edge operation can retain two tracing effects plus one edge replacement
under the same immutable decision. Every fallible proposed-owner acquisition
precedes COMMIT; both memberships and the edge precede the completion certificate.
Three portable tests force late acquisition failure, cancellation and help after
the owner stops at its first effect. Native/WASI catalog totals are 69/63; strict
Clippy passes. The existing decision/order models remain scoped to their actual
atomic helpers; these additional cases execute the real multi-effect controller.

## C10/C11 prerequisite: strong-edge transactions

Frozen publication, replacement, child-root activation and requested/orphan edge
removal validate exact source/target generations under one controller snapshot.
Deletion and insertion barriers share its decision; same-target writes contribute
one ticket, and stale source/target keys cannot attach to reused slots. All 16
native frozen raw-edge regressions (13 portable) are retained. Native/WASI catalog
totals are 85/76; strict Clippy passes. Test helpers use the packaged guarded entry
methods directly; no external catalog dependency is introduced.

## C10 prerequisite: resumable source traversal

Frozen Full-domain traversal retains its exact source generation, epoch, ticket
and strength. It visits the complete admitted source range before acknowledging;
a stronger or superseding ticket requires another complete scan. Stopped private
scanners do not own shared completion. All 13 frozen traversal regressions pass
on both native and WASI, including transitive retention, insertion behind a
cursor, unreachable-cycle reclamation and independent helper completion. Catalog
totals are 98/89; strict Clippy passes. Runtime edge admission remains separate.
