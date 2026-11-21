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
