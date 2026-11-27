# Source-indexed canonical edges

This historical experiment can be reproduced from the [archived source](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc). The current Java preview uses the [unified JVM heap](../GC_PREVIEW.md).

The experimental graph runtime now uses catalog-enforced source ranges. A completed
traversal observes D edge positions for its source instead of all E positions in the
catalog. General catalog callers retain the existing complete-inventory mode.
This does not remove the global root/object/edge cleanup sweeps or change GC defaults.
See [the graph contract](CANONICAL_GRAPH.md) and
[the measured results](../target/gc-implementation/canonical-indexed-REPORT.md).

## Setup and publication invariant

Indexed setup requires nonzero D and exactly E=N×D, with checked multiplication.
The bounded constructor rejects an invalid layout before reserving credit. The raw
setup builder requires an unshared owner and rejects reconfiguration. Once a cell,
root, edge, snapshot or traversal owns the catalog identity, geometry cannot be added
or changed through that builder.

For source object slot s, the only permitted edge positions are `[s×D, (s+1)×D)`.
Raw publication validates this condition after establishing exact live source-root
authority. The bounded facade additionally returns `InvalidSlot` for wrong placement,
so a permanently invalid request does not retry as contention. An edge's source
identity is immutable after publication. Replacement validates that exact identity;
reusing the same object slot cannot attach an old source's edges to a new object.

The invariant is enforced below the graph adapter, rather than inferred from an
allocation convention. Private edge creation may reserve an arbitrary valid position,
but it cannot publish that position under a source outside the configured range.
The general mode has no source partition and continues to scan `[0,E)`.

## Traversal and interruption

A traversal obtains its range when it validates the source's exact current ticket,
epoch and retention. It retains the first slot and range length with its private
cursor. `scanned_slots()` remains a count relative to the start of that range.
One poll observes one position or advances a retained preparation/attempt.
Acknowledgement is prepared only after the complete source range is observed.

Failed observations preserve the cursor. An old ticket cannot acknowledge stronger
or later work. Existing insertion/deletion barriers retain targets changed behind
the cursor. Another helper may scan the same range and finish an installed
acknowledgement after the first scanner stops. The range is immutable while any
scanner exists, so private progress does not require a layout handshake.

O(D) describes the number of inventory positions in a successful scan. Contention,
retries, epoch changes, callbacks, allocation and scheduling are not assigned a
wall-time bound by this result. Full-domain-only and leaf-payload-only restrictions
of the graph adapter remain in force.

## Storage and validation

The degree resides in the existing edge registry; first-slot/range-length fields
reside in the existing traversal request. Concrete `size_of`-based catalog and
participant charges include them. No pool reserve or allocator-request count is
changed. Whole-invocation counting measures eight extra requested bytes and eight
extra catalog-credit bytes on native, with no measured WASI increase in the current
layouts. All 48 samples balance allocations, releases and live-byte totals.

The frozen candidate passes 670 native GC tests (one existing ignore), 365 WASI GC
tests, 236 native / 217 WASI catalog tests, native/WASI Clippy, formatting and
bare-Wasm lib/examples. Each memory tool passes the affected 13 graph and 40 edge
tests. All 68 corrected catalog models pass; six unchanged pool models are reused
with exact input hashes. Six selected fault conditions fail as intended on native
and WASI; all 49 checker mutation anchors validate.

New fixtures cover wrong placement, pre-credit configuration rejection, the last
source position, failed reads, stopped scanners, mutation behind the cursor, exact
identity after slot reuse and immutable setup. The real graph fixture exercises
both supported degree endpoints, 1 and 8. The larger stopped-ack model uses the
existing enlarged coroutine-stack helper after a recorded stack overflow in its
first development run; no permutation or preemption cap was added.

Seven paired blocks per target, with two fresh processes per capacity/revision/block,
measure 10,000 cyclic rounds each. At capacity 256, paired lifecycle time falls about
19% native and 11% WASI. The capacity-2 WASI interval includes no change. Cleanup
still accounts for about 94% of runtime at capacity 256. These short fixtures do not
qualify STW/host-turn latency, JVM execution, G1/ZGC comparisons or endurance.

Next attribute and reduce the remaining root/object/edge sweep and controller costs,
then complete VM payload/caller integration, callback handling, compaction, JIT and
all full-plan qualification gates. Defaults remain unchanged.
