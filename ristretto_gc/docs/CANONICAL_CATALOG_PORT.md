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
