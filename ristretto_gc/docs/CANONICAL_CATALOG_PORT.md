# Bundled canonical catalog and pool

These private modules are included in the `ristretto_gc` package and compiled only
with `experimental-canonical-runtime`. They require no unpublished registry crate
or sibling validation project. Enabling the feature does not change the ordinary
collector or select this pool for its allocations.

The source reference is the frozen Git tree
`4e8b7e5ca04eba08950ab59ac94a2c3870d756bf`, under
`ristretto_gc/validation/catalog/catalog-tagged{,-pool}-prototype`.
This port starts from `origin/main` at
`7b6e1379556d2e92d4639cc4198c845fd5e0b469`.

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
