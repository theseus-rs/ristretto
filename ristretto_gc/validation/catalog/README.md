# Catalog and phase protocol experiments

The [phase-only controller optimization](../../CANONICAL_PHASE_DECISIONS.md)
uses one slot for an effect-free proposal and keeps its immutable decision as the
stable head. The bounded facade preserves two local preparation yields for numeric
and root requests. Effect-bearing operations keep the three-slot path. The revised
catalog passes 253 native / 233 WASI tests, 73 corrected transition models and all
60 native fault controls; eight phase controls also pass their WASI negative checks.
Model, facade, memory and finite-progress scopes are documented separately in
[the report](../../../../target/gc-implementation/canonical-phase-REPORT.md).

The [bounded cleanup and helped-publication follow-up](../../CANONICAL_SWEEP_BATCH.md)
lets an allocation recover the same admitted, stably published root after another
helper commits it. A conflict during the object-before-root effect interval remains
pending while the retained root is Prepared or Live; retired roots still reject.
Final catalog tests pass 242 native / 222 WASI. The new native stopped-helper
regression fails when that retry is removed. Reused raw transition models do not
model the bounded facade; memory and finite real-value checks cover it separately.
See [the evidence and cost report](../../../../target/gc-implementation/canonical-sweep-REPORT.md).

The [indexed inventory mode](../../CANONICAL_INDEXED_EDGES.md) now enforces fixed
source ranges at publication and lets each traversal scan only that range. The general
inventory mode is preserved. Final tests pass 236 native / 217 WASI, 68 corrected
catalog models and the affected memory/fault checks. The graph adapter opts in; paired
measurements and remaining cleanup costs are in [the report](../../../../target/gc-implementation/canonical-indexed-REPORT.md).

An internal [graph adapter](../../CANONICAL_GRAPH.md) now uses this inventory with
real typed leaf payloads, coupled mutation, exact child-root activation and cyclic
cleanup. The bounded facade adds stable source/edge observation and local request
discard that preserves won commits. Final catalog tests pass 226 native / 209 WASI;
new fault and memory checks pass. The raw catalog still permits general edge placement,
so traversal scans all edge capacity per source. See [the measured limitations and
validation](../../../../target/gc-implementation/canonical-graph-REPORT.md).

The tagged experiment also has a [strong-edge inventory](../../CANONICAL_EDGES.md):
source-owned non-owning identities, coupled mutation barriers, child-root activation
and complete Full-domain traversal. Its bounded facade prevents direct ticket
acknowledgement from skipping edges and rejects unsupported Young/Old modes.
The real graph runtime and efficient source-indexed traversal remain separate work.
Final tests, fault controls and native/WASI metadata costs are in
[the report](../../../../target/gc-implementation/canonical-edge-REPORT.md).

These packages exercise candidate publication representations. The optional
[private leaf adapter](../../CANONICAL_LEAVES.md) now connects real GC allocations
to the tagged catalog, but public allocation/collection policy does not select it.
No default, whole-GC pause, peer-performance or endurance gate changes.

`catalog-pooled-prototype` extends the selected descriptor direction with fixed
backing for operation and membership states. `catalog-descriptor-prototype` retains
the version using individual system allocations. Their registry makes descriptors
and their work discoverable, while a prepared operation couples one descriptor update with
both domains' pending counts and phase versions. `catalog-prototype` retains the
earlier immutable-tree alternative and its result-recovery fix. The cost package
compares their allocation requests; it does not run a garbage collector.

## Atomic relations

1. A prepared cell receives an identity never reused within its catalog and enters
   one selected slot with a CAS. Failed preparation retains no slot reservation or admission. The
   record remains discoverable when its submitting stack disappears. Production
   integration must transfer metadata, initial child roots and credits into the
   record before this announcement; numeric addresses here own no storage.
2. Private preparation constructs an immutable operation, its expected and proposed
   cell states, and committed/cancelled phase states. A CAS from an exact stable
   controller version installs the operation. The busy controller itself makes
   unfinished work discoverable to other helpers.
3. Helpers share one irreversible commit/cancel decision. A helper applies the
   effect using its exact owning expected cell pointer, certifies completion,
   then replaces the busy controller with the corresponding stable phase state.
   No helper owns an exclusive completion claim. Each replaced controller version
   is unique and is never installed again. Pooled storage may be reused only after
   every owner of the earlier version disappears. A delayed helper can therefore neither
   replay an old effect nor recover an incorrect result after retirement.
4. Acknowledgement validates the exact work ticket and couples its removal with
   the domain count decrement. New work changes that ticket. A domain advances
   only with its pending count zero; a busy controller must be helped first.
   Slot-by-slot enumeration remains authoritative for discovering work.
5. Membership changes irreversibly to Retired. Slot cleanup compares the exact
   retired cell, preserving a later occupant. Canonical abandonment of Prepared
   membership competes with insertion through the same controller. Cancelling an
   individual proposal does not by itself cancel the cell's registration.

The integer phase model has four floors. It deliberately does **not** stand in
for the production collector's root, weak, final, phantom, generation and handoff
rules. A descriptor becoming Live or Retired in this experiment is not evidence
of a real object becoming reachable or being physically reclaimed.

## Executable evidence

Run from the repository root, optionally with an external `CARGO_TARGET_DIR`:

```sh
cargo test --locked --manifest-path ristretto_gc/validation/catalog/catalog-descriptor-prototype/Cargo.toml
cargo test --locked --manifest-path ristretto_gc/validation/catalog/catalog-prototype/Cargo.toml
cargo +nightly miri test --locked --manifest-path ristretto_gc/validation/catalog/catalog-descriptor-prototype/Cargo.toml --lib
cargo check --locked --manifest-path ristretto_gc/validation/catalog/catalog-descriptor-prototype/Cargo.toml --target wasm32-unknown-unknown --lib
```

Set `MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-tree-borrows"` for the recorded Tree
Borrows configuration. WASI tests require a `CARGO_TARGET_WASM32_WASIP2_RUNNER` and
`--target wasm32-wasip2 --lib`. Model tests are native only. The models import the
same transition source and substitute a mutex-backed linearizable owning CAS;
they do not model ArcSwap internals, payload pointers or allocation failures.

The frozen candidate passes 11 native descriptor fixtures, eight descriptor Loom
models, eight WASI fixtures, eight Miri fixtures, eight Tree Borrows fixtures,
three native tree fixtures and three tree models. Native schedules stop helpers
before/after an effect, after its completion certificate, and after controller
release. Other work completes while they remain stopped. Weak probes show that
retained old operations do not retain subsequently created cells after slot
cleanup. Four concurrent publishers preserve both domain counts over 800 cells.
These are scoped protocol checks, not whole-collector progress qualification.

Three frozen mutations fail: comparing a freshly loaded cell state resurrects a
retired cell; moving the completion certificate after controller release returns
Retry for a committed operation; comparing a fresh slot occupant removes its
replacement. The initial tree model also retains its failing result-recovery
schedule and repaired version. All failed outputs remain in the evidence set.

## Allocation screen

```sh
cargo run --release --locked --manifest-path ristretto_gc/validation/catalog/catalog-cost-prototype/Cargo.toml -- descriptor 1024 1024
cargo run --release --locked --manifest-path ristretto_gc/validation/catalog/catalog-cost-prototype/Cargo.toml -- tree 1024 1024
```

Arguments are representation, resident numeric descriptors, and lifecycle count.
A lifecycle prepares/publishes one descriptor, acknowledges each domain once,
retires it, and releases registry ownership. Three process repetitions at each of
five catalog sizes, on native and actual WASI, yield 60 passing processes with
identical repeated counts. The tree fixture inserts into a gap in the identity
range; the descriptor fixture reuses one registry slot with fresh identities.

| Allocation requests | Descriptor registry | Tree at 1 resident | Tree at 16,384 residents |
| --- | ---: | ---: | ---: |
| Record and insertion | 6 | 3 | 18 |
| Complete lifecycle | 18 | 10 | 70 |
| Native bytes per insertion | 376 | 264 | 1,704 |
| WASI bytes per insertion | 264 | 216 | 1,416 |
| Native bytes per lifecycle | 1,216 | 864 | 6,624 |
| WASI bytes per lifecycle | 864 | 704 | 5,504 |

The descriptor counts stay constant across these sizes. Both warmed commit
windows allocate zero in this screen. That observation excludes cold ArcSwap
thread bookkeeping: the installed ArcSwap implementation can allocate debt nodes,
scan its thread-node chain, retry internally and release old reference counts.
Neither its work nor allocator behavior is qualified as a clockless bounded poll.
The tree additionally has linear last-snapshot destruction despite bounded tree
depth. Its snapshots retain old trees, whereas an old descriptor operation retains
only its own cell states and small phase versions.

These numbers exclude catalog initialization, payloads, child roots, resource
budgets, tracing and the production metadata layout. They establish neither
production allocation savings nor GC throughput/latency. Six requests per new
descriptor still need reduction and explicit backing accounting.

See the [allocation figure](../../../target/gc-implementation/catalog-candidate-allocations.svg),
[frozen evidence report](../../../target/gc-implementation/catalog-candidate-REPORT.md),
and [integration requirements](../HELPABLE_PUBLICATION.md). The next implementation
must transfer real ownership and credits, preserve the complete phase rules, and
migrate every operation that currently relies on admission before removing it.

## Preallocated-state follow-up

The [pool implementation](catalog-pool-core-source/README.md) and
[pooled catalog](catalog-pooled-prototype/README.md) retain the same exact-owner
protocol while moving control/membership values into fixed backing. A private
attempt prepares at most three control versions and one membership value. Failure
and cancellation return those private slots. The original system-allocation and
immutable-tree packages remain comparison controls.

```sh
cargo test --locked --manifest-path ristretto_gc/validation/catalog/catalog-pooled-prototype/Cargo.toml
cargo test --locked --manifest-path ristretto_gc/validation/catalog/catalog-pool-core-source/Cargo.toml
python3 benchmarks/gc/catalog_model_check.py --output target/catalog-model-check
cargo run --release --locked --manifest-path ristretto_gc/validation/catalog/catalog-pooled-cost-prototype/Cargo.toml -- pooled 16384 1024
```

The model runner applies the already recorded, unmerged
[Loom RMW atomicity patch](https://github.com/tokio-rs/loom/pull/391) in an isolated
checker copy. It checks four pool models, eight protocol models and two expected
negative controls. Production atomics and workspace dependencies are unchanged.
Both stock and corrected-checker results are retained; the latter uses commit
`c673a8d800f3a8163586e8a6177ddafd01f2e665`. The models still abstract pointer protection,
allocator failures and real object graphs.

The [new figure](../../../target/gc-implementation/catalog-pooled-allocations.svg)
shows allocation traffic alongside the extra retained backing. All 90 processes
pass, with identical repeated counts; all 60 earlier tree/unpooled count rows are
reproduced exactly. Record/insertion uses one system request instead of six, and a
complete lifecycle uses one instead of 18. A cell still allocates, and backing after
setup increases about 55% native / 72% WASI at 16,384 resident records. Fixed pools,
thread bookkeeping and tables are included in that setup count. Payloads, child
roots and heap budget enforcement are excluded. There are no throughput, latency
or production memory-saving claims.

The [follow-up evidence report](../../../target/gc-implementation/catalog-pooled-REPORT.md)
records 13 native catalog fixtures, eight protocol models, ten WASI/Miri/Tree Borrows
fixtures, native ASan/TSan, the separate pool checks and all failed mutations. The
pool reserve settings do not yet guarantee progress capacity under an enforced
participant bound. This is the next ownership-integration candidate, not a completed
stage D implementation or a qualified runtime configuration.

The first reproduction runner shared a target directory between variants and was
rejected when its early-reuse negative unexpectedly passed without recompilation.
The final runner retains a separate build directory for each variant; both negative
controls then fail as required. The rejected run, its runner source, and the final
corrected-checker runs are preserved in the evidence report.

## Generation-tagged owning-link experiment

The separate [tagged pool](catalog-tagged-pool-prototype/README.md) tests a replacement
for ArcSwap's ownership acquisition. An indexed token can acquire a reference only
for its exact live generation. Acquisition and strong-owner cloning attempt one CAS
and return contention; initialization and final destruction exclude acquisition and
reuse. Successful link replacement retains the expected owner while releasing the
old link count, and rejected proposals return ownership to the caller.

Seven native, three WASI, seven Miri/Tree Borrows and seven ASan/TSan fixtures pass.
Five exact-state models pass with both stock and corrected Loom, and four mutations
fail. The repository runner also passes its positive and negative checks:

```sh
python3 benchmarks/gc/tagged_pool_model_check.py --output target/tagged-pool-model-check
```

The [evidence](../../../target/gc-implementation/tagged-pool-REPORT.md) retains the
original expected-diagnostic mismatch and Clippy failure as well as the corrected
runs. Those figures describe the original primitive revision. The
[tagged catalog follow-up](catalog-tagged-prototype/README.md) uses it for all numeric
owning links and adds explicit Pending outcomes and contended discovery. The
[cost fixture](catalog-tagged-cost-prototype/README.md) compares it with the pooled
ArcSwap version using separate allocation-count and timing builds.

Bounded atomic attempts do not bound arbitrary value destruction or caller retries,
reserve progress capacity, enforce heap budgets, or establish a collector poll bound.
The tagged representation eliminates loop allocator requests while increasing
backing. The subsequent tagged facade enforces separate participant/helper limits
and derives slot capacities from retained ownership. It preserves partial cell
creation and tests continued helper progress with ordinary participants stopped.
Its byte budget, scheduler fairness, exhaustion policy and integration with real
metadata, initial graphs and phase rules remain open. See the
[participant evidence](../../../target/gc-implementation/catalog-quota-REPORT.md).


The tagged facade subsequently adds configured logical byte reservations for pool
backing and inline participant capacity. Opaque handle teardown refunds after the
outer Arc and pools are freed; a separate allocation/order audit detects early
refund and undercharge. This budget is not integrated with the production heap and
is not RSS accounting. See [charged backing](../../../target/gc-implementation/catalog-charge-REPORT.md).


The current tagged catalog additionally handles finite generation rejection and
canonical per-domain tracing. Epoch, retention and exact work ticket share immutable
membership, committed with pending counts. Native/WASI and memory-tool fixtures,
20 catalog / six pool models, and 27 distinct negative variants validate the
scoped implementation. The new state increases requested backing and measured
numeric lifecycle costs; see the [tracing report](../../../target/gc-implementation/catalog-trace-REPORT.md).
Production metadata, initial graph roots, reference decisions and admission remain
outside the experiment.


The tagged catalog now extends canonical roots with conditional decisions,
construction lifetime custody and exactly-once managed notification transfer.
[REFERENCES.md](catalog-tagged-prototype/REFERENCES.md) describes the protocol and
bounded facade. The [report](../../../target/gc-implementation/canonical-reference-REPORT.md)
records the18-command matrix,48 scoped models, 36 runtime/two model faults, nine
compiler rejections and 528 allocator rows with higher backing costs. This remains an
isolated experiment; production graph/cleanup/admission and qualification are open.
