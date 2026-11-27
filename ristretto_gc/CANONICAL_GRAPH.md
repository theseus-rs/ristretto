# Canonical graph runtime

This historical experiment can be reproduced from the [archived source](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc). The current Java preview uses the [unified JVM heap](../GC_PREVIEW.md).

The [experimental caller API](CANONICAL_API.md) now exposes this typed graph
runtime outside the GC crate through transparent wrappers. It preserves the existing
admission/layout bounds and ownership protocol. VM value/layout and execution
integration remain open; no default backend is selected.

This feature-gated internal runtime combines real typed allocations with catalog
edges, coupled mutation barriers, independent child roots and incremental cleanup.
It does not select the VM's collector backend. The public surface added here is a
hidden diagnostic entry point and two benchmark examples; ordinary GC defaults
are unchanged. See [the evidence](../target/gc-implementation/canonical-graph-REPORT.md)
and [the edge protocol](CANONICAL_EDGES.md).

The [shared-root follow-up](CANONICAL_SHARED_ROOTS.md) permits owning typed aliases
without another catalog root. Child reads retain their independent-root semantics;
only the last alias drops its unique logical token. Root creation now adds prepaid
private backing, measured separately from allocation-free copies.
The corrected final-alias teardown frees that backing through `Arc::into_inner`
before dropping the unique root and refunding catalog credit; a populated final-owner
audit covers both leaf and graph roots. Ordinary Arc teardown in the preceding
shared-root version did not preserve that order.

## Admitted representation

`GraphRuntime` binds one private catalog to one `GarbageCollector`. A node consists
of an eligible record and a fixed range of D catalog edge positions. Each record
stores its usable outgoing edge count K, where 0 ≤ K ≤ D.
D is 1 through 8, E must equal N×D, root capacity R must be positive, and conditional
reference capacity Q must be zero. Both active domains use Full tracing.

T must satisfy the existing leaf admission checks: no hidden managed references,
legacy graph registration, resumable trace callback or finalizer. Its real backing,
heap identity, publication certificate and type remain checked by `LeafRecord`.
Graph edges reside exclusively in the inventory, never in T. An edge contains exact
source and target identities rather than owning cells, so a source-owned cycle does
not keep itself alive through reference counting.

The sealed adapter assigns source slot s the positions `[s×D, (s+1)×D)`. The
[indexed follow-up](CANONICAL_INDEXED_EDGES.md) now enforces this geometry in raw
publication as well as the bounded facade. Graph traversal observes D positions per
source; general catalog callers retain the complete E-position scan. Global root,
object and edge cleanup sweeps remain capacity dependent. Arbitrary Rust `Trace`
graphs and VM object/array payloads are not admitted.

## Allocation, mutation and reads

Each mutator owns one exclusive request. Allocation prepares T once, retains its
record through bounded slot retries, publishes an external root, then creates and
publishes K null edges. Ordinary `allocate` uses K=D. With `allocate_with_edges`,
the caller selects K before publishing the record. Reads and writes recover K
from the live source, including through newly acquired child roots. A target
with K=0 can still be reached from a source with a usable edge. It returns `GraphRoot<T>` only after complete inventory
publication and handoff of the temporary edge owners to the source. The intermediate
source can be traced safely because its new edges are null. Cancellation discards
private request ownership and posts durable root/edge cleanup requests.

The exclusive mutator prepares one private root backing before it polls an
allocation or child read. A null result, error or cancellation leaves this
backing available for the next request. A successful root handoff consumes it.
The backing holds no graph reference until handoff, and the mutator drops it
before returning its participant quota.
[Native and WASI allocation measurements](../benchmarks/gc/CANONICAL_MEASUREMENTS.md#peer-comparison-with-a-persistent-java-heap).

A write borrows live source and optional target roots and uses the catalog's coupled
old-target deletion and new-target insertion barriers. A child read observes a live
source and exact edge under a stable controller revision. A null result linearizes
at that observation. A nonnull result reserves a fresh independent root and activates
it only through `RootTarget`, which revalidates source, edge and exact target in one
controller snapshot. The typed result additionally checks heap admission,
publication and `TypeId`. Replacement before activation forces a retry; an activated
child root remains valid after later unlinking.

When no child root permit can be reserved, a read reobserves the edge. A concurrent
null replacement can therefore finish even while every external root permit remains
occupied. A still-nonnull edge can remain pending until quota is available or the
caller cancels. This is not a public bounded-allocation-failure policy.

Dropping a request calls `discard_request`: it attempts cancellation and clears the
participant's local request. A commit decision that already won remains installed
and helpable. Dropping the originating wrapper does not roll back committed effects.

## Maintenance and retained ownership

The existing helper gains a rotating edge lane between root and object work. It
processes durable removal requests and exact orphan identities, then releases retired
edge slots. Failed discovery remains contention, never proof of absence. The existing
complete retirement sweep, epoch binding and floor of five completed epochs remain
in force. With E=0 the edge lane is skipped, although its cursor/capacity fields still
occupy wrapper storage.

A stopped allocation or child read retains bounded private roots, candidates and
requests. Another participant can complete installed work and reclaim unrelated
objects. Tests stop an allocation at several prefixes while another participant
collects a cycle. The concurrent fixture checks child identities during replacement
and helper activity. These finite fixtures do not establish fairness or sustained-load
latency under arbitrary scheduling.

Host allocation and user destruction retain the limits of the leaf contract. A poll
that finally destroys T may execute its Drop implementation. No wall-time bound or
arbitrary callback isolation is established here. This stage does not prove that the
whole collector has eliminated pauses or blocking.

## Resource census

The base catalog footprint already reserves participants, root owners, temporary
edge owners and all pooled backing. The graph supplement adds:

- One `GraphRuntime` wrapper.
- For each mutator, its wrapper beyond the base participant plus the largest of
  `GraphAllocation`, `GraphRead` and `GraphWrite`; exclusive borrowing prevents
  simultaneous requests on that participant.
- For each helper, its `LeafMaintenance` wrapper beyond the base participant.

`GraphRoot<T>` has the same layout as the charged base root handle. T is not embedded
in these request wrappers. Payload backing is accounted separately by the real heap;
allocator metadata, external owners, host RSS and arbitrary callback allocations are
not included in catalog byte credit. The footprint test checks the complete owning
wrapper census, and a deliberate omitted-request charge fails it.

## Reproducible cost screen

Build `graph_lifecycle` and `graph_lifecycle_counts` with
`--features experimental-canonical-leaves`. Both take `ROUNDS CAPACITY`. The timing
example supports native and WASI clocks; bare Wasm compiles but rejects timing before
setup. The counting example warms up once and records three complete invocations.

Each round creates two real 24-byte values, installs a two-node cycle with two writes,
reads one independently rooted child, drops all three roots, and drives full cleanup.
It checks identities, canaries, exact destruction, object/root/edge release counts and
zero payload credit before the next round. Setup and teardown are separate from the
allocation/mutation/read/cleanup intervals. Counting includes setup, teardown and the
report String, excludes stdout, and reports allocator requests rather than RSS.

`benchmarks/gc/graph_lifecycle.py` freezes the binary, sources, build provenance,
runner and seeded schedule. It validates every raw result and complete schedule,
then produces CSV/JSON and standalone SVG/PNG graphs. Fixed argv0 prevents pathname
length from affecting counting. Poll watchdogs are diagnostic ceilings, not latency
or progress guarantees. This small one-mutator/helper fixture is not a G1/ZGC,
whole-VM, pause, contention, browser-host-turn or endurance qualification.

Source-indexed inventory and [bounded empty-slot cleanup](CANONICAL_SWEEP_BATCH.md)
are implemented. The latter also makes announced publication helpable and fixes
exact-result recovery through partial helper effects. Measured small-capacity,
controller, allocation, root and barrier costs remain optimization targets. VM payloads and callers, callbacks,
compaction, JIT and every full qualification gate remain open.
