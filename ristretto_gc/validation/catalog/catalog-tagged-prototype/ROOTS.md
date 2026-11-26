# Canonical roots in the isolated catalog

Current scan batching (2026-11-15): the original single-slot scan remains unchanged.
A separate `ScanBatch` operation examines at most eight slots and stops at the first
root/target effect. The internal leaf helper selects it. Cursor and effects retain
the same controller/certificate ordering and storage bounds; failed observation
never advances the cursor. Current validation includes 59 catalog and six pool
models plus 32 fault controls. The measured lifecycle improvement is 18–21%, with
unchanged canonical allocator activity and no full-GC qualification. See
[the batching protocol](../../../CANONICAL_ROOT_BATCH.md).

The catalog's keys and root-status observations alone grant no payload access.
The optional [leaf adapter](../../../CANONICAL_LEAVES.md) now checks publication,
heap and type before borrowing a typed value through an exact root token. Public
allocation/result transfer, runtime graph traversal, worker wakeups and callback
progress remain separate integration obligations. [REFERENCES.md](REFERENCES.md) extends this
original caller-root protocol with internal construction/notification custody.

Each root owns an exact target cell, a monotonic Prepared/Live/Retired status and a
durable removal-request flag. Creation reserves its unique token before announcing
Prepared. Publication couples object publication and Live in the same controller;
activation/removal couples root status with both domains' required shading. All
fallible owning acquisition precedes this helper's COMMIT decision. Helpers apply
both effects before certification and controller release. A copied intermediate
status is not a completed-operation or access certificate.

Each domain has a canonical root cursor. A scan observes one slot and couples its
required target shading, any requested removal, and cursor advancement. A failed
owning load does not skip a slot. Advance/Finish require a complete scan; domain
Start resets that domain's cursor. Root changes after a cursor passes still supply
their own insertion/deletion shading. Root barriers clamp Strong to the active
floor and preserve old-young births and young-old exclusions. They cannot revive
logically retired targets. Removal may finish without shading an already dead
object. This does not implement conditional-reference upgrades.

RootToken Drop posts a release-store removal intent in the exact root entry.
Helpers can cancel Prepared or remove Live even without an active collection.
For ordinary caller roots, only Retired plus requested removal allows exact
discovery-slot release. Reserved reference roots have the separate release rule
described in REFERENCES.md. Explicit
revocation therefore cannot reuse a slot while its token is still held. The
requested flag itself does not certify deletion: helpers retain the deletion edge
through the canonical operation before releasing the slot. Fair maintenance and
wakeup policy are not supplied by this data structure.

The facade reserves one root quota before private creation. A ready token remains
in its participant until take_root, cancellation or Drop. Noncloneable RootHandle
holds the token before a RootLease field; this lease retains CatalogHandle. Token
and retained-value destruction precede quota release, and catalog backing/outer
allocation destruction precedes logical credit refund. No raw owning root share
escapes this facade. Keys/observations are inert copies. Root commands use their
own sealed dispatch rather than extending the legacy numeric command enum.

Let N be object slots, M root slots/token permits, and K all participant permits.
At most M roots are owned by the root registry and M by outstanding tokens, with
no disjointness assumed (including destructors paused after posting their request).
The previous participant proof permits at most 2K+1 additional root-bearing private
creations, preparations, installed/stale operations or temporary help/observation
owners. Thus 2M+2K+1 root entries is a conservative retained bound. Root entries
hold at most that many additional distinct target cells; the earlier cell bound
was N+2K+1. The resulting N+2M+4K+2 cell bound fits N+2M+6K+4 reserved cells.
Membership ownership adds at most one current membership per extra root-retained
cell to the previous N+6K+3 bound, yielding N+2M+8K+4; N+2M+10K+8 is reserved.
Controls retain the earlier 7K+3 bound and 8K+8 reserve: a root operation has the
same single controller and two outcome records, with a larger inline layout.
Root reserve is 2M+2K+2. All pool counts round up to a checked power of two within
the tagged-pool limit. M=0 preserves the previous pool-count formulas and allocates
no root pool. Finite generation exhaustion and unfair scheduling are not solved by
these reserves; payload destruction is outside the bounded atomic-step claim.

The rooted-allocation transaction now retains one additional exact cell per
participant across record creation, root creation and publication. Its phases are
mutually exclusive. The conservative cell bound becomes N+2M+5K+2 and membership
bound N+2M+9K+4; both fit the existing root-enabled reserves above. Root permits and
controller counts do not increase. The cell's abandonment byte and inline request
layout are included in actual Footprint calculations. See the runtime
[allocation protocol](../../../CANONICAL_ALLOCATION.md) for cancellation ordering
and its remaining maintenance/callback limitations.

Footprint uses the actual generic pool/outer allocation layouts, K maximum inline
participant sizes, and M inline RootHandle sizes. This is a logical backing budget,
not RSS; allocations indirectly owned by R require separate charging. Any concrete
runtime record that owns a root handle must explicitly break that ownership on
result transfer/cleanup; arbitrary ownership cycles are not collected by the facade.

The root_accounting example audits target retention, root publication/drop/removal,
slot reuse, root-held final catalog ownership, zero lifecycle system requests and
refund-after-free. Root-enabled native/WASI accounting and concurrency/memory-tool
results must be reported with their exact source snapshot. None of these tests
qualifies whole-GC pause, throughput, latency or sustained-load behavior.

`RootHandle::borrow_live_record` borrows the exact target retained by its token after
observing Live. It performs no owning membership load or pool share; the record
borrow remains tied to the handle, so it adds no independent owner to the bounds
above. It is not itself a runtime access certificate. The concrete leaf adapter
supplies heap, publication and TypeId checks; general records need their own
admission contract. Existing borrows retain storage after explicit revocation,
while calls observing Retired return no record.
