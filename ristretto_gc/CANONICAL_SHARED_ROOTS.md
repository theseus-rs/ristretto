# Shared typed canonical roots

The [experimental caller API](CANONICAL_API.md) now exposes this typed graph
runtime outside the GC crate through transparent wrappers. It preserves the existing
admission/layout bounds and ownership protocol. VM value/layout and execution
integration remain open; no default backend is selected.

`LeafRoot<T>` and `GraphRoot<T>` share one unique bounded catalog RootHandle through
an immutable private `Arc<Storage>`. Clone requires no T: Clone bound and increments
only the shared owner's count. It performs no catalog operation, root reservation,
publication, barrier or host allocation. Every alias calls `Arc::into_inner` through a private Option owner. The final call
frees the Arc backing before returning Storage, then drops the unique token,
posting the existing removal intent before releasing its lease. No external Weak
observers are exposed. Ordinary Arc Drop would refund credit before freeing this
backing when the last alias outlives the runtime; that v1 defect is corrected here. The raw
RootHandle remains non-cloneable. Child reads acquire independent logical roots.

A borrowed typed view remains tied to its exact alias and live heap/runtime. Rust
rejects escaping that view or dropping its owner while borrowed. Other aliases can
leave concurrently. Revocation excludes new reads; an existing view retains its
exact old storage through retirement and slot reuse. An alias can retain storage
and its charge after runtime/heap shutdown; an unrelated runtime cannot read it.

## Preparation and resource bounds

Each allocation or child-read request prepares an empty owner before bounded
polling begins. The private Arc has no aliases or weak observers. Successful typed
handoff initializes its Option<RootHandle> through Arc::get_mut and publishes only
an immutable initialized owner. Null, failed and cancelled requests release their
private backing. No raw-pointer, interior-mutation or new unsafe ownership protocol
is introduced. Exclusive mutator borrows limit simultaneous request owners to M;
root permits limit live logical owners to R. Arbitrarily many copies of one owner
do not consume more logical root permits.

The runtime supplement covers the target's inline wrapper/request representations
and at most R + M Arc backing allocations. It replaces the base catalog's already
charged raw-root representation rather than double charging it. Actual observed
requested backing is 40 native bytes / 32 WASI bytes with this recorded toolchain.
The implementation computes its layout; an independent allocator fixture checks
that claim and the charged amount. A separate populated final-owner audit keeps
external collector/budget owners alive, drops runtime/participants and the original
root, then brackets only the final alias drop. It requires actual destruction,
zero remaining heap bytes, one exact credit refund and the last free before that
refund. Restoring ordinary Arc teardown and omitting pending backing fail their
respective checks on native and WASI. The full logical reservation remains until final
catalog destruction, including retained aliases and stopped payload destructors.

Caller-owned alias variables, frames/stacks, supplied collector/budget owners,
payloads, allocator overhead and RSS require their separate accounting. Arbitrary
payload Drop and host allocation remain outside the bounded catalog-poll contract.
A new logical root performs one additional host allocation, including a child read
that later observes null. This change does not close allocation overhead or pause
qualification. Private lower-level transaction fixtures can still compose a raw
catalog directly; its base Footprint alone does not cover the typed runtime's
supplement. Production runtime constructors enforce the owning census.

## Evidence and remaining work

The [corrected report and graphs](../target/gc-implementation/canonical-shared-root-v2-REPORT.md)
retain the v1 defect, its reproducer, all fresh samples, regressions and provenance
failures. The original v1 archive/report remain historical evidence; their complete
resource-accounting acceptance is superseded. The correction follows the existing
CatalogHandle teardown pattern and adds no unsafe ownership, catalog transition,
layout, quota, generation or retirement-floor change.

679 native GC tests (one existing ignore), 373 WASI tests, Clippy, bare-Wasm builds,
39 Python tests and all 88 affected fixtures under each of four memory tools pass.
Native/WASI and all four memory modes also pass both populated final-owner cases.
The 13 fixture commands plus four memory audit commands give 17 memory commands.
Compiler controls accept non-Clone payload aliases and reject invalid view lifetimes.
Eight intended runtime/audit negative processes and all 90 unchanged finite stress
processes pass. Raw catalog/pool code is unchanged; 73 catalog/six pool models are
reused through exact compiled inputs rather than claimed as new runs.

Across 84 isolated timing processes, owning copy/drop costs about 10.6 ns native
and 3.0 ns WASI versus legacy 23.5 / 12.0 ns. All 18 allocator processes observe no
allocator work in that region. The unchanged full cyclic lifecycle has 30,000 extra
allocation/release requests per 10,000 rounds, and native capacities 8/64/256 regress
about 0.8%/0.7%/1.3% versus phase-v2. Native capacity 2 and WASI capacity 8 are
inconclusive; WASI capacities 2/64/256 improve about 1.7%/2.1%/2.1%. All 224 lifecycle
timing processes and 48 allocator samples are retained. These are component cost
screens and finite correctness evidence, not whole-JVM or sustained qualification.

This remains a feature-gated internal runtime. VM object/array payloads and callers,
general graph layouts, arbitrary callbacks/destruction, public backend selection,
compaction, JIT, native global-pause bounds, G1/ZGC comparisons and all sustainable
capacity, scheduled-tail, paired-hour and 24-hour gates remain open. Defaults are
unchanged. The added root-creation allocation remains an explicit optimization task.
