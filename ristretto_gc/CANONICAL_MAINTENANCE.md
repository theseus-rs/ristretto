# Incremental canonical leaf maintenance

Current scan batching (2026-11-15): the original single-slot scan remains unchanged.
A separate `ScanBatch` operation examines at most eight slots and stops at the first
root/target effect. The internal leaf helper selects it. Cursor and effects retain
the same controller/certificate ordering and storage bounds; failed observation
never advances the cursor. Current validation includes 59 catalog and six pool
models plus 32 fault controls. The measured lifecycle improvement is 18–21%, with
unchanged canonical allocator activity and no full-GC qualification. See
[the batching protocol](CANONICAL_ROOT_BATCH.md).

The optional internal `LeafMaintenance` policy collects the reference-free catalog
using reserved helper participants. It composes with `LeafAllocation` and its typed
roots. Public `Gc` constructors still use legacy registration. This policy does
not yet supply public collector selection or whole-collector qualification. The
[sealed graph adapter](CANONICAL_GRAPH.md) adds catalog-owned edge traversal.

A helper borrows its collector for its lifetime and owns one registered participant.
It rejects catalogs with conditional-reference slots before registering. Its saved
state consists of cursors, copied keys and phase data; it retains no additional
pooled observations outside its participant's existing request. Dropping the helper
uses the existing request cancellation and quota-release ordering. It does not
synchronously finish a collection cycle.

## Polling and cleanup

Each poll performs one release/request transition or at most eight physical
inventory observations, stopping at an occupied or failed observation. The
[batching and publication contract](CANONICAL_SWEEP_BATCH.md) gives the exact bounds.
An observation can also submit a copied command, which only stores private request
state. A pending request retains its preparation across polls. The helper rotates
through four lanes after a terminal result (the edge lane is skipped when E = 0):

| Lane | Work |
| --- | --- |
| Root | Batch confirmed empty slots; publish an announced Prepared external allocation root, remove requested Prepared/Live roots, or release a requested Retired root |
| Edge | Batch confirmed empty slots; remove requested/orphan edges or release Retired slots |
| Object | Batch confirmed empty slots; abandon requested Prepared cells, traverse/acknowledge tickets, retire eligible garbage, or release a Retired slot |
| Phase | Observe/help the controller, scan logical roots, advance an eligible domain, finish a completed sweep, or start the next full collection |

Physical cursors wrap over their declared capacities. Transient discovery failures
rotate the cursor. Stale/conflicting commands finish the lane; permanent exhaustion
and structural failures are reported. Logical root enumeration has its own canonical
cursor, so a physical cleanup scan cannot skip logical tracing work. Trace tickets
and phase work alternate domain priority. Initial domains finish; subsequent full
collections restart domain 0, leaving domain 1 inactive.

For a leaf-only catalog, tracing tickets need no edge traversal because record
eligibility excludes managed references. The graph adapter traverses the complete
source-indexed inventory before acknowledging its ticket. It submits retirement only after
copied membership/phase eligibility; the controller revalidates every mutation.
Root removal retains the canonical deletion barrier. Abandonment applies only to
Prepared membership, so a publication that already won stays helpable.

## Complete sweeps and epoch-bound Finish

A domain at retirement floor 5 may finish only after that helper has visited every
object slot in the same epoch. Finishing as soon as pending work reaches zero can
leave most garbage unvisited.

Each object-cursor advance accumulates a successful visit. The interval receives
sweep credit only when its bounding phase observations report the same active mode,
same epoch and retirement floor. Monotonic epochs/floors certify the intervening
visits even when some phase observations were contended. A changed epoch, mode or
floor resets the corresponding sweep. Failed object discovery resets both sweeps
and accumulated visits: separately counting successful slots could otherwise
repeatedly skip one contended slot while counting others twice. N consecutive
successful cursor advances cover all N physical slots.

The Finish command retains that exact epoch through preparation and retries.
`begin_finish_epoch` rejects a snapshot from another epoch; the bounded facade
reports Conflict. An already prepared attempt still requires the expected-head CAS.
The former unbound Finish could rebase onto a later epoch: a forced schedule
reproduced a delayed helper closing a new, unswept collection. The regression and
epoch-bound command protect this boundary. Trusted raw callers retain their existing
unbound Finish operation; they are responsible for their own policy preconditions.

## Progress and accounting limits

The exercised schedules cover discarded allocation prefixes, live roots, repeated
slot reuse, alternating helpers, helper replacement, retained revoked-root views,
and a stopped allocation origin or payload destructor while another helper reclaims
unrelated storage. A completed later full epoch must have retired eligible original
garbage; physical unlinking may follow. Birth/deletion rules can preserve objects
until a later cycle. These finite schedules do not prove starvation freedom under
an indefinitely adversarial scheduler or unbounded contention.

Catalog calls can execute arbitrary payload `Drop` and host allocator work. Bounding
catalog transitions per poll does not bound wall time, native pauses or Wasm turns.
The policy adds no separate owning catalog share beyond the registered participant;
its wrapper's inline storage is outside the base catalog-only Footprint. The later
[internal runtime owner](CANONICAL_RUNTIME.md) binds helpers to their heap/catalog
and reserves their additional owning storage through the catalog's setup charge.
The [automatic allocation path](CANONICAL_RETRY.md) now preserves payloads across
collisions and full quota; callers still need to drive these helpers.
Public selection, wakeup/scheduling, indefinite capacity
service, graph/conditional-reference integration, callback bounds, admitted callers,
JIT and sustained performance qualification remain open.
