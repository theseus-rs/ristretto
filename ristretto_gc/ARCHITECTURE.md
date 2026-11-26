# Generational collector with optional payload compaction

All Ristretto Java execution uses this collector. The native worker and
cooperative WASM driver share the same heap, publication, marking, reference
processing and reclamation code. See the [Java preview](../GC_PREVIEW.md) for
the cross-tier functional checks.

Logical reservations transfer to fully constructed metadata before it can become
visible. The descriptor's existing weak collector link carries that charge;
`StorageState::destroy` selects the sole payload destructor, which refunds bytes
after the erased owner and backing are dropped. A constructor unwind after
publication cannot refund a still-owned descriptor. Leases delay both destruction
and refund; stale metadata owners do not delay refund after destruction. No strong
collector cycle or separate charge allocation is introduced. Scoped guards ensure
finalizer/diagnostic unwind reaches payload cleanup and that payload/diagnostic
unwind still refunds the released storage. Normal sweep and deferred reclamation
validate and borrow the collector for refund; private cleanup uses the weak link.
The borrowed path rejects a foreign heap before claiming storage. Initial roots still
belong to the synchronous construction scope.

## Specification contract

The collector preserves Java object identity, reachable instances and arrays, reference-strength ordering, finalization and resurrection, and Java allocation-failure behavior. The JVM permits implementation-specific collection algorithms and noncontiguous heaps; it requires `OutOfMemoryError` when the heap cannot satisfy an allocation. See [JVMS 2.5.3](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.5.3).

Conditional reachability follows [java.lang.ref](https://docs.oracle.com/en/java/javase/25/docs/api/java.base/java/lang/ref/package-summary.html), with finalization governed by [JLS 12.6](https://docs.oracle.com/javase/specs/jls/se25/html/jls-12.html#jls-12.6). The JVM tests exercise JDK 8, 11, 17, 21, and 25 class libraries, including their different reference-handler protocols. These tests are evidence for the covered behavior, not a JVM conformance certification.

## Heap and roots

Every allocation has a stable descriptor and a process-wide 64-bit `ObjectId`. IDs are never reused, including across heaps; exhaustion fails instead of wrapping. Typed runtime handle reconstruction checks the heap's ID registry, then revalidates identity, type and logical liveness together under the object shard and selected publication admission. Java identity hashes, monitors, JIT helper values, class-loader caches, profiling IDs, and native unpacker keys use this identity rather than payload addresses. Legacy unsafe address APIs remain available for pinned Rust allocations.

Identity allocation reserves ranges of 64 from the shared counter. Each OS thread
keeps a 16-byte range without heap/context references or a destructor; Wasm uses
the same local path. Unused IDs are never returned, so IDs can have gaps and do not
encode allocation order across threads. The final partial range is usable, and
exhaustion never invalidates a range already reserved elsewhere. If a host denies
TLS access during teardown, allocation falls back to one checked global ID.

Owning handles share their root registration through `triomphe::Arc` (0.1.17,
`std` only). Root registrations never expose weak ownership, so their counter does
not carry a weak count. Object metadata also uses this strong-only ownership;
conditional edges store the target's nonreused object ID and validate it against
the address index. Cleared weak handles therefore do not retain dead metadata.
Collector ownership still uses standard `Arc`/`Weak`. Weak-reference eligibility
and payload storage leases are unchanged. A prepared
registration protects the graph before allocating this shared owner, and drops the
registration on cancellation.

Mutable Java reference-array element access (`get_mut`, indexing, slices and mutable
iteration) acquires an owning root before exposing an existing value. This allows
`mem::replace` and similar moves to retain the extracted graph after the source
array or execution context disappears. Consuming element iterators own their
unconsumed values too. Managed guard cleanup demotes values remaining in dirty
slots, preserving traced cycles and eventual reclamation. Whole-slice borrowing
roots every exposed reference and has linear, synchronous work; this is not a
bounded cooperative bulk-access protocol.

Managed write guards restore owning strong handles through `Trace::root` before
exposing unrestricted `&mut T` or invoking a mutable mapping callback. This covers
whole `Reference`, `Object` and `ArrayElements` moves, including failed mapping
and unwind. Ownership travels with the value; guard cleanup demotes fields left
inside. Custom `Trace` implementations that demote fields in `unroot` must restore
those fields in `root`. Conditional fields keep their reference-strength semantics.
Whole-value acquisition is synchronous and linear in the strong fields exposed.

`Trace` is an unsafe implementation contract. Unsafe attachment and demotion
receive a borrowed `TraceOwner` naming the actual collector and descriptor. `Gc`
demotes only same-heap handles; foreign strong edges keep their source heap alive.
Option, vector, tuple, heap-buffer and map-value hooks propagate this context.
Nested managed cells install their hook before demotion and detach it before
rooting for movement, then bind to their destination when stored again. Java
instance/array ownership markers reset on whole-value rooting; a moved conditional
object binds its reference slot to the new descriptor without rooting its referent.
These hooks do not provide a general scan of a graph hidden behind a held writer.

Hash-map keys, hash-set elements, ordinary mutex payloads and embedded root guards
retain independent strong ownership. Cycles through those fields, or across
separate collectors, can remain retained. Cross-heap Java reference queue delivery
also remains unqualified; foreign strong ownership does not establish that protocol.

`ReferenceWriteGuardExt` offers field replacement, reference-array replacement and
one-element mutable borrowing while retaining the original managed guard. Replacements
discard the old edge in place, preserving deletion/card barriers without rooting
unrelated fields. Point borrowing roots just the selected value before exposure.
These safe wrappers use `RwLockWriteGuard::unrooted_mut` under its explicit unsafe
no-unrooted-escape contract. Shared access still permits checks under the same
writer; later unrestricted access still invokes whole-value preparation. Interpreter
and experimental JIT field stores, shared `aastore` and multidimensional initialization
use these operations. Java resolution/access/type/bounds checks remain at their
usual call sites. This ownership repair does not permit tracing past a held writer
or establish a complete snapshot protocol. See the
[validation](VALIDATION.md#whole-value-ownership-and-narrow-mutations-2026-11-13).

The optional `experimental-trace-projections` feature permits tracing to finish
while a typed primitive or independently rooted object-array slice is held. The
entire parent must either need no trace actions or preserve every direct strong
field as independently owning without other parent trace actions. A separately
rooted target still runs its own tracing and conditional-reference processing.
Successful mapping publishes with Release; the original boxed writer revokes before
demotion and unlock, including unwind and nested projections. A fresh Acquire CAS
certifies coverage without reading the borrowed payload. Pinning and ordinary
relocation locks remain. Unrestricted writers, arbitrary callbacks and shared-reader
contention do not qualify. Initial/final slice ownership work remains linear.
The feature is disabled by default; its
[protocol](PUBLICATION.md#rooted-object-array-projections-2026-11-14)
and measured overhead are separate from general graph-progress qualification.

`metadata_shards(n)` partitions object-address trees, identity and nursery
directories, and remembered-owner sets. The runtime rounds the count up to a
power of two between 1 and 16. The default is one. Address shards fold aligned
descriptor address bits. Each identity directory holds pages of 64 consecutive
IDs, partitioned by the low bits of the page number.
Young and remembered membership use the same address partitioning as objects.
Each cycle retains separate cursors for cross-generation young roots, remembered
owners and young sweeping. Compaction resets its young cursor before enumeration.
Each scan visits one entry or exhausts one shard per work unit and releases that
shard before any callback. Late publication, promotion and buffered barriers retain
their existing shading rules; sharding does not replace phase closure. Mutation
version validation and remembered removal still hold the object shard together.
Each point lookup, insertion, or removal locks only its selected shard. The number
of shards stays fixed for the heap's lifetime.

Full cross-snapshot, sweep, and compaction traversals retain a position for every
shard and an exhaustion mask. Each work unit tries one shard, visits one entry or
advances past one exhausted shard; it never searches all empty shards in one unit.
A busy shard keeps its position and gives another unfinished shard the next turn.
Failed downstream lookup/removal restores the selected entry's previous position
before rotating. No busy shard can certify completion. The sweep cursor survives reopened marking,
and compaction starts a fresh cursor after sweeping. Young enumeration continues
to use the young-allocation index. Publication behind any cursor still shades new
allocations during marking and makes them black after the strong-mark boundary.
Root/edge barriers and stable identity validation remain in force.

Nursery enumeration retains the current bitmap page. When it runs out of
entries, the scanner reads a fresh owning directory view and checks for a
replacement at the same key before advancing. Only the matching object shard
can change a bit, install a page or unlink an empty page. Holding an old view
keeps bitmap storage alive and gives the reader no reference to a managed
payload. A scan result is an address hint. The collector checks the current
descriptor and its age under the object shard before acting on the hint.

Index selection releases its guard before shading or executing tracing,
relocation, finalizer, or destructor callbacks. Retirement tries the object
and remembered shards, committing removals only after both guards
are held. Promotion acquires the object and remembered shards before changing
age or cross-domain marks. Contention releases acquired guards and retries the
uncommitted entry; other shards can progress. Coordinated removal retains the
coordinator and the default policy retains the publication writer for that attempt.
A rooted constructor installs its initial root while the descriptor is private.
The root registry synchronizes the tree or slot insertion. The allocation owns
the address exclusively and construction roots protect its outgoing edges. If a
root scan has already passed, descriptor registration performs the birth and
traversal work for each active domain under phase admission. Acquiring a root
for an indexed object still validates and shades it under the applicable admission.

An ordinary constructor reserves its exact identity bit before phase admission.
Each bit belongs to one preparer or one published address. Cancellation releases
the preparer's bit. A parked preparer leaves the other IDs on its page available.
The constructor then tries its object shard. Both policies wait for that shard
before acquiring phase admission. Construction roots and reserved bytes
remain owned throughout.

After validating descriptor and nursery vacancy, registration publishes the
address with a release store. Lookup produces a numeric hint. Root reconstruction
checks type, identity and liveness under the object shard. Retirement first
clears the exact address, then releases its bit. Clearing the last bit seals
the page permanently. Sweep queues payload reclamation and releases the cycle
and coordinator guards before unlinking a sealed page from its directory.
Unlink checks both the page identity and the owning directory version. Another
constructor can replace the sealed page while the original worker is parked.
A per-thread weak page hint checks collector identity on every hit. Each sweep
can retain one numeric page and retries a missed address in the current directory.

Registration installs the descriptor, identity address and nursery bit under
the object shard, then releases it. An indexed descriptor begins private.
A scanner that reaches it can record the skip and advance its cursor. The
constructor still owns the initial and child roots at this point. Unwind
removes the exact registry entry and nursery bit before releasing these roots.

Both publication policies order the index writes before the next cycle with a
same-value phase-clock RMW. An idle birth can become live if no scan passed its
pending descriptor. During a collection, a leaf or a graph born after strong
tracing can initialize its marks and old-cohort tag privately and validate the
clock revision. If that revision can repeat, the publisher instead retains and
validates the owning identity of every active domain's epoch.

A graph born during strong tracing also retains those epoch identities. Before
activation it enqueues one exact ticket per strong domain and stamps the
phase clock. A closer that observed an empty queue before this stamp must
retry. At saturation a closer must rebase before obtaining a new observation.
Workers can claim tickets while the publisher is parked. Claiming a still-private
graph records a skip, which defeats activation. Claiming the live graph transfers
its descriptor into a normal marking task. The first ticket is the descriptor
itself. After a retry a permanent visibility bit rejects first-attempt tickets,
and subsequent attempts use distinct cancellation records. A failed publisher
discards its speculative marks and old-cohort tag before rearming. The next
phase-clock RMW orders the rearm before future scans.

Reusable slots have separate free and ready bits. A publisher stopped before
setting its ready bit holds only that slot. Other publishers can wrap into a
linked overflow queue, and workers inspect ready bits and actual overflow links
at closure. Diagnostics count allocated slot pages and linked overflow storage
in the domain’s current epoch, including pages retained after strong closure.
This handoff lets a capped peer trace live births and reclaim garbage
while a constructor remains parked before linking, after certification, or after
activation. The single-shard physical index commit can still delay recovery.
Old-only cycles tag young births with their birth epoch. Post-strong births
receive the retiring domain's mark before peers can see the object.
Metadata `Arc`s alone do not pin storage. Trace and relocation callbacks acquire
explicit storage leases that prevent clearing/destruction through retirement;
generation interlocks and in-flight reservations still protect graph traversal. Statistics visit shards one at a time,
so `live_objects` during concurrent publication is an observation rather than one
atomic index snapshot. Shutdown drains all shards before running finalizers and
destructors. Nursery pages and remembered entries follow the metadata sharding policy. Region
allocation has its own selectable shards and reference registration remains shared.
Sharding is independent of the selectable counted publication protocol.

An explicit Cargo feature, `ristretto_gc/experimental-identity-index`, substitutes
an owning atomic bucket-list index for the default identity pages. The other registry
guards, publication policy and validated handle reconstruction remain as described
above. Its sweep removal and maintenance are resumable, and stalled index readers
retain integer descriptors rather than object payloads or expanding history.
Embedded link markers share each successor's owning allocation. Shared insertion
uses immutable attempts and unique bucket headers; interior deletion/unlinking
allocates no link states, while replacing a head allocates a new header. Another
helper can complete an insertion before or after its original publisher stops,
and delayed CAS attempts cannot replay it after retirement. The feature is off
by default. Its historical comparison used the earlier mutex-backed map. Collection
results differed by target and worker policy. [Identity index protocol and limits](PUBLICATION.md#experimental-owning-identity-index).

With `compact_heap(true)`, Java instance fields, reference-array elements, and all primitive-array elements reside in `HeapBuffer` ranges within a portable `RegionHeap`. Descriptors, locks, class metadata, and arbitrary Rust `Gc<T>` values remain pinned. Default configuration retains host-backed payloads. Region allocation uses aligned disjoint ranges and coalescing free lists. An index of the largest free range in each region narrows allocation candidates before checking alignment. Backing allocation and deallocation run outside the registry mutex, with physical credits reserved until the operation finishes. This requires neither virtual-memory remapping nor pointer tagging.

`HeapBuffer::try_with_fn_in` and `try_with_fn_in_async` initialize directly in the selected backing heap. The interpreter's ordinary instance and array constructors and reflective array construction use the async path. It yields between initialization batches of at most 64 KiB, or one element if larger; the unfinished payload remains unpublished. Each initialization chunk selects its host or region loop once; every successful element commits prefix ownership before the next initializer. Initializer errors, cancellation, and unwinding drop only the initialized prefix. Storage recovery precedes initialization, so a retry does not repeat initializer side effects. Host-backed and synchronous constructors remain available. Preparing an existing buffer from another heap transfers its storage and accounting to the destination heap; async publication yields between preparation batches.

`local_allocation_buffers(true)` enables execution-context caches for small allocations.
Each context retains at most one unused 16 KiB payload range and one descriptor
range, partitioned on 64-byte boundaries, plus at most 4 KiB of unused logical
credit. Requests over 1 KiB or aligned beyond 64 bytes bypass these caches. Failed
batch reservations fall back to exact sizes; failed range refills release cached
ranges before retrying. Descriptor slab failure falls back to host allocation.
Unused logical credits stay included in `bytes_allocated`, including across
suspension. The last user context handle releases caches even if escaped roots
retain its registration; collection also releases one context's caches per work
unit during root enumeration. Shutdown releases all remaining caches.

Pinned descriptor slabs occupy a separate arena from movable payloads. Both arenas
share checked backing credits, refunded only after the last region owner releases
the physical allocation. This cap excludes global descriptor fallbacks, metadata,
and host payloads, so it is not a process RSS limit. Diagnostics expose unused
ranges, unused credit, live-context hits/refills, and descriptor arena occupancy.
The option defaults to false pending performance qualification.

Owning a handle roots the descriptor without pinning its payload. A managed read/write guard (including mapped native guards) pins the payload by excluding relocation's exclusive try-lock. All existing slice access is scoped to these guards. The collector copies at most `relocation_chunk_bytes` per work unit, then publishes the destination under the same exclusive guard. Partial destinations are raw storage, never traced or dropped as live values. A bitmap tracks dirty 1 KiB chunks (or individual oversized elements). Scalar and range writes invalidate only affected chunks, while an unrestricted mutable slice invalidates every chunk before later publication; cancellation and allocation failure leave the source valid. Only the committed buffer owns element destructors.

Default packing moves ranges into free space in earlier regions, preventing relocation ping-pong. Large arrays use contiguous extents and incremental copying; this implementation does not segment Java arrays. Default packing uses existing committed holes. A missing destination, held guard, or repeatedly invalidated copy can defer compaction. Each payload receives a finite retry budget per cycle. Physical region statistics include destinations while copying, separately from logical Java live-byte accounting.

`cost_aware_relocation(true)` adds a bounded selection pass after full sweep. A
collector-built occupancy index visits one registry entry or candidate per work
unit, favoring at most four source regions
with no more than approximately two-thirds occupancy. The quantized ratio estimates
copy cost relative to reclaimable backing; it is a heuristic, not a prediction of
successful evacuation. More than eight observed buffer invalidations since the
previous sample defer a region. A failed payload try-lock defers its cached region
for the following selection. Cold payloads can move while these regions wait. The four-source limit preserves
ordinary allocation holes during evacuation. A retained vector holds cost-ranked
candidates; building it on the collector avoids updating a sorted tree on each
allocation/free. Roles and occupancy are rechecked before selecting a candidate.
Its retained capacity is reported outside the backing cap.
Young collections retain default packing. Both policies retain the same guarded
copy and finite per-payload retry limits.

A selected source cannot receive ordinary allocations or copies during that
selection. Destinations can be earlier, later, or newly allocated regions and cannot
be selected as sources in the same epoch. Partial copies from an older selection
restart if their source/destination roles no longer match. Epoch exhaustion falls
back to default packing without reusing roles. Ending selection, including reopening
marking after late work, restores unmoved source space to ordinary allocation.

`relocation_reserve_bytes(n)` with the cost policy protects up to half the physical
heap cap for new destination backing. Payload and descriptor arenas share that cap
and reserve; ordinary backing cannot consume the reserve. Existing free destination
ranges remain usable. Reserve sizes smaller than a required backing allocation do
not guarantee progress. The effective region size also fits the ordinary allowance
on small heaps. Defaults are policy off and reserve zero; disabling compaction or
the cost policy makes the effective reserve zero.

An empty selected source leaves the registry immediately. Empty oversized backing
also retires immediately so tiny copies cannot strand a dead large array’s region. Physical credits and
committed-byte accounting are refunded only after actual backing deallocation,
including when a zero-length local-cache tail retains the last owner. This allows
one reserve to evacuate several sources without waiting for final trimming. Failed
destination allocation leaves initialized source ownership intact. Gross evacuated
source bytes are reported separately from shared current/peak committed backing,
which includes destinations and descriptor arenas.

Optional per-payload history stores an advisory region ID and the time from the
first failed lock attempt to the next non-busy callback. It is allocated only for
recognized movable payloads when the cost policy is enabled. Diagnostics alone
do not allocate that history.
`Trace::relocation_region` defaults to no history; managed wrappers use a nonblocking
lookup. A concurrent writer can make the cached ID stale, affecting feedback only.
The observed deferral includes scheduling and cooldown, is not exact guard-held
time, and reports resolved samples only. Bare wasm records counts without a clock.
Diagnostics also expose retry ceilings and cancellation attempts that remain busy;
these may retain a charged partial destination until later payload access.

Owning handles register precise roots and retain the collector. Converting a `GcRootGuard` into an owning handle transfers its existing registration; cloned guards share that registration until the final owner drops. Generic `Gc::clone` registers a root when copying a borrowed heap edge; `rooted_clone` has the same semantics. Cloning an owning handle or calling `GcRootGuard::clone_gc` shares the existing registration. Mutable access to a guard's inner handle also shares that registration before exposing it, so moving it out retains ownership. A replaced guard target uses its own root. Edge cloning validates immutable identity and logical liveness under publication admission without reading the payload; raw/unassociated or expired edge cloning panics. Registration reservation is canceled on rejection. Java `Value` clones root objects copied out of fields or arrays into locals, operand stacks, native calls, exceptions, caches, and async futures. Managed cells demote stored handles to traced heap edges before publication and after mutation. Cycles therefore do not become permanent owning roots. Statics, mirrors, and host caches hold owning values until their VM lifetime ends.

Destroying a root guard suppresses its inner edge's deletion barrier only when
the allocation identity and collector match the still-owned registration. The
last registration owner publishes the required root-deletion barrier. Replaced
or foreign edges retain ordinary destruction. Transfer consumes both guard fields
exactly once without running this suppression; failed replacement rooting still
releases the original registration during unwinding.

Public root-guard creation validates the handle's source collector before registering.
`Gc::as_root` returns an error on mismatch; `create_root_guard` panics before acquiring
a registration. This prevents a foreign collector's ineffective registration from
replacing source ownership during guard transfer. The check compares borrowed
collector pointers without a weak clone or extra registration.

Allocation attachment borrows the collector's immutable weak reference for its
`TraceOwner` context. A hook that retains the context still clones its own weak;
the temporary attachment context performs no weak clone/drop. The constructor's
live collector borrow spans the callback, including reentry and unwind.

Each `Gc` stores one ownership mode: either its shared root registration or a weak
collector reference for heap-edge deletion barriers. Owning clones increment only
the registration count. Demotion establishes weak collector access before releasing
the registration, and handle reconstruction registers directly after identity/type
validation. Metadata owns erased initialized storage directly, with a typed drop
function; it no longer needs a separately allocated destructor closure. Destructor
unwinding still releases its backing storage exactly once.

Descriptors and tree roots share immutable tracing operations through a static
per-type table. The per-object trace-step option preserves the leaf fast path;
the descriptor's original value pointer also supplies address lookup and finalizer
dispatch. Type identity and the optional finalizer function share another immutable
per-type table; each descriptor has an atomic finalizer claim. The callback still
runs once outside collector coordination, before its caller destroys storage.
No separate finalizer closure allocation or duplicate descriptor address is needed.
The first metadata revision saved 32 requested bytes per object; the shared type
table saves another 16 in the measured 100,000-object screen. Neither eliminates
the metadata allocation itself.

Typed handle resolution and conditional upgrades reuse one collector-specific
publication token from validation through owning-root insertion. Token reuse
removes nested admission operations while preserving the original protection
interval and per-slot decision lock. A foreign collector's token is rejected.
Metadata-owner allocation precedes admission. An identity/weak conversion keeps
its registered root in a stack guard while admission ends and its shared owner
is allocated; cancellation releases that exact registration.

Types explicitly returning false from `Trace::may_contain_gc_references` need no
trace callback, payload access, scan lock or storage lease to complete an existing
mark claim. Managed and standard lock wrappers propagate this property. A held
write guard on such a leaf cannot delay marking, while relocation still requires
exclusive access and may skip the payload. Clearing budgets and element destructors
are unchanged. The default remains true so custom tracing actions are preserved;
this optimization does not apply to reference-bearing payloads.

Managed write guards always invoke `unroot`. They publish an owner mutation only
when the type can contain edges and `Trace::needs_write_barrier()` conservatively
reports that the current value needs it. The latter query runs under the exclusive
borrow and must be constant time without allocation, payload locks or collector
calls. Java primitive-array variants return false; object and reference-array
variants return true. This does not bypass edge deletion barriers, tracing or
`HeapBuffer` relocation dirtiness, and a later reference-bearing value resumes
ordinary publication. A reference-bearing type's held write guard can still delay
its tracing even when its current value happens to be empty.

`root_storage(RootStorage::Slots)` opts owning handles and guards into reusable
root pages. The default remains `RootStorage::Tree` pending qualification. Sixteen
registries, selected by allocation address, each use 64-slot pages and occupancy
bitmaps. Preparation locks one registry; owning deletion claims its generation
atomically. Reuse normally needs no tree-node allocation. A registration contains
its location and a checked non-reused generation,
so a delayed or duplicate removal cannot erase a replacement slot, even when page
numbers wrap. Generation exhaustion fails instead of reusing an identity.
Root shard selection mixes address bits independently of the metadata index’s
bucket function. Sweep order can group reused descriptors by metadata shard;
using that same grouping for roots concentrated a rolling live set in one shard
and repeatedly allocated/released pages. Independent mixing reduces that measured
churn without enlarging the one-empty-page-per-shard cache. It is deterministic
on each target and does not change a registration’s location during its lifetime.

Slot ownership travels with the owning registration, including across threads and
suspended/cancelled futures. It is independent of OS-thread identity. Guard clones
share one registration, and transfer into an owning handle preserves it.
`RootStorage::Contexts` adds registered task ownership and optional buffered
barriers, as specified in [PUBLICATION.md](PUBLICATION.md).
Tree, slot and context root enumeration try the selected object's metadata shard.
A busy lookup leaves the tree ID uncommitted or rewinds the selected slot before
yielding. Slot retry reads the address again from its pinned page, tolerating
removal and reuse without saving a raw address across the retry. Successful
read-to-shade operations keep the existing phase/admission protection. Context
enumeration reads an immutable registry view, published before attachment returns,
and pins one current context per generation instead of upgrading it for every root.
Initial cache cleanup and final owner release run after publication, cycle and
coordinator guards are released. Busy caches keep unused capacity charged while
collection proceeds. Registry-map publication clones its entries on attachment
and removal; this trades cold-path work/memory for scan independence from writers.
A scan can retain a detached context's empty-page cache until it advances. Queue
publication, buffered records and context cleanup from other paths retain waits.
Explicit `add_root`/`remove_root_by_id` retain their numeric-ID tree under every
policy. Numeric removal accepts only manual registrations returned by `add_root`;
it cannot revoke owning guards, construction roots or queue-transfer roots.
`remove_root` by address supports all stores but is unsafe: callers must preserve
the argument's lifetime during revocation and prevent later access through revoked
handles unless independent reachability protects them.
Tree registrations store a checked, non-reused 63-bit sequence and one owning-kind
bit in their existing 64-bit generation field. Exact internal removal checks the
whole tagged value, preventing a stale owner from deleting a newer registration
after numeric-ID reuse, including across manual/owning kinds. Sequence exhaustion
fails closed before the kind bit. The sequence advances under the existing tree
mutex. Public numeric IDs are invalid after removal. Tree entries store only an
address and tagged generation; tracing always resolves allocation metadata.
This uses 16-byte entries on native and wasm32 and keeps the owning registration
within its existing size. Pending references store the owner's allocation identity
and the exact tree registration, so validated transfer rejects an expired owner
instead of following a replacement at its former address. Tree locks and admitted
map allocation remain; these identity checks do not establish nonblocking progress.

Each generation enumerates slots with its own cursor, visiting at most one root or
one empty-page/shard advance per work unit. Inserting into an already visited slot
or page still shades the target in active snapshots under the selected publication
admission. Removal preserves the same deletion barrier. No slot lock remains held
while shading. The default gate or experimental counted protocol closes marking.
Page memory is allocated/deallocated outside registry locks. Empty pages are released
as roots leave, with at most one cached empty page per registry; nonempty sparse pages
remain enumerable. `stop()` trims cached empty pages while preserving escaped roots.
`root_statistics()` reports occupancy, cached page bytes, reuse, and page retirement;
page bytes exclude registry map nodes and owning-handle metadata.

Slot enumeration now reads an immutable page-membership view through `arc-swap`
and atomic occupancy/address fields, without acquiring the writer mutex. A cursor
owns only its current page across work units. Page membership changes clone a
shard's map; this cost is not constant in that shard's page count. A temporary map
reader can retain that snapshot, while a suspended cursor retains one page.
`resident_pages` and `resident_page_bytes` include retained page payloads, including
pages surviving context teardown; map nodes and Arc headers remain excluded.

Slot acquisition prepares an address-zero reservation before publication admission.
The reservation owns its page and, for context roots, the context. Identity/type
and weak-reference eligibility are checked after preparation. Commit publishes the
address and shades active snapshots under the same admission; cancellation removes
the invisible reservation. `reserved_slots` is computed during the cold statistics
walk rather than by atomic increments on each root operation.

An owning slot registration pins its page. Deletion claims its checked generation,
clears its address and publishes the deletion barrier under admission, then releases
the occupancy bit. A new reservation cannot reuse that bit before the barrier
completes. Stale removers cannot clear a replacement. Page membership cleanup uses
only a try-lock after admission, deferring to a busy writer; scans and unrelated
collections can proceed while that writer is stalled. The designated cache page
can contain live roots, avoiding an additional empty page. The slot index and
nonreused generation share one 64-bit word; exhaustion fails before identity reuse.
Cleanup may scan a shard's pages and clone its map, so it is not bounded work.

Compatibility lookup also uses the page view, validating the generation before
and after its address read. New generations publish with release ordering, so a
reader cannot pair a replacement generation with the prior address. It retains
outer admission and context/tree synchronization. Numeric tree roots, metadata lookup and shading
still have admitted blocking paths. These changes trade higher measured root costs
for demonstrated progress during a stalled slot writer; they have not qualified a
default-policy change.

Production JIT eligibility is limited to static primitive methods, including
integer conditions, narrow/wide jumps and switches. Forward-only bodies need no
execution polls. Bodies with self/backward edges use owning continuations so the
VM can yield between native slices. Reference operations, calls and exception
tables retain their production restrictions.
Runtime helper handles are validated and rooted; admitting additional reference
operations still requires their frame-liveness and semantic qualification.

Pending JIT exceptions carry an owning handle paired with their published identity
under a context-local mutex. Replacement drops the previous owner after unlocking;
extraction moves ownership into the VM error or the handler-result root set.
An already published frame owner is shared instead of registering a second root.
No callback runs under that mutex. Transient helper
results retain one owning handle per non-reused identity. Repeated loads share it;
preparation and replacement cleanup run outside the result-map mutex. Direct
backend reference methods now replace this set at each bytecode boundary with
current operand references and conservatively retained reference locals. Helpers
without frame publication still retain results until invocation return.
Completed contexts move a reference result's existing owner into its returned VM
value; missing identities still require typed validation before context cleanup.
Both argument-conversion buffer paths use the same consuming return handoff.
The separate `execution-profile` build records actual generated-code entries and
interpreter/intrinsic entries, alongside compilation state. Its lifetime totals and
metadata retention are diagnostic; timing comparisons reject instrumented output.

The JIT now retains Java reference kinds separately from machine register types:
both references and longs use I64 registers, but references are category one.
Operand-stack simulation, CFG parameters and helper continuations preserve that
distinction. A worklist checks all incoming stack states, including fallthrough,
and discovers successors reached through backward edges. Handler entry stacks
contain only the exception reference. Switch forwarding blocks preserve operands
below the key and match the full signed Java int domain. Legacy `jsr`/`ret`
lowering is rejected because return-address dataflow is not modeled. These are
compiler correctness prerequisites. The experimental frame transfer below uses
their reference kinds and the reference-local liveness analysis; production eligibility
and complete frame qualification remain open.

Local variables now use separate SSA variables for each Java slot and kind. This
supports legal slot reuse across primitive/reference types and reuse of a wide
argument's second slot; references and longs remain distinct despite both using
I64 registers. This representation does not determine which kind is live at a
given bytecode, so enumerating every declared reference variable would retain
dead values. A backward worklist now selects local references needed by normal or
exceptional successors, accounting for slot reuse, wide stores and backedges.
Already eligible primitive methods can now compile when they reuse local types
instead of falling back after a type-conflict error.

Native continuations own their generated function, marshalled arguments and
aligned cells for all declared typed locals and the largest suspended operand
stack. Selected backward edges consume a positive local counter; exhaustion saves
the state and the destination BCI, then returns. Resumption restores SSA locals
and explicit operands without replaying bytecodes or counting the edge twice.
The default budget uses method instruction count to bound that method's bytecode visits per
slice; copying declared state adds bounded work. Neither bound is a wall-time or
STW guarantee. No collector rendezvous or clock is involved.

The VM retains the Java frame, monitor and owning arguments while its async
dispatcher yields. Each yielding activation returns its native frame before the
dispatcher awaits; cancellation drops the activation and existing
execution-boundary cleanup releases Java stack/monitor
state. The frame publishes the destination BCI at each yield. Each resumed native
entry checks stack capacity and can run on a different
OS thread. The public synchronous `Function::execute` still runs to completion;
embedders needing cooperative execution use `Activation`. Experimental reference,
allocation, direct-call and explicit-monitor methods also have continuations. Their VM invocation retains one owning runtime context,
including published roots and pending exceptions, throughout suspension. Before
yielding, generated code publishes the destination's live local references and
remaining reference operands, releasing consumed conditions and dead roots.
Reference kinds come from Java stack metadata, not the I64 machine width.

Exception-table methods also have continuations. Every selected handler entry
consumes fuel, including forward handlers: backward exception edges can form a
cycle without any explicit backward branch. Exception dispatch uses a separate
destination map so normal branches to the same BCI retain their branch semantics.
The handler's single throwable operand and live local references are published
before a yield. Typed locals and that operand survive resumption; the existing
exception matcher retains table order and moves pending ownership into the handler.
Within the supported finite bytecode body, any cycle crosses either a backward
ordinary branch or exception dispatch, so it consumes fuel. Direct calls return
requests separately. This bounds local bytecode execution, not helper duration.
Diagnostic backedge counts still count explicit branches only; native resumes
include suspension at handler entries.

The private state header contains a resume token, fuel, a runtime-context
address and a call-dispatch mode. `resume_with_context` refreshes that address for each native slice and
clears it before returning; no borrowed context pointer survives an await or
migration. A required missing context fails before entry. VM return/exception
conversion consumes the same context, transferring existing owners before cleanup;
cancellation drops it. Primitive loops need no runtime context.

Eligible direct invocations return `ExecutionStep::Call` after saving typed locals,
residual operands and an argument buffer sized to the method's largest call.
The caller snapshot retains argument references and residual live roots while the
native frame is absent. Call metadata identifies the invoke form, constant-pool
index and call-site BCI. The VM yields, resolves and awaits the ordinary dispatcher,
then writes result bits exactly once. Resumption joins immediately after that call;
pending-exception dispatch and result rooting retain their existing semantics.
The activation refuses resumption until the call is completed. The public
synchronous API selects the existing helper branch instead, preserving its
run-to-completion contract.

Nested call futures use temporary native stack segments while polling and dropping;
no segment survives suspension or migration. If a synchronous generated caller
remains active, polling checks its remaining capacity instead of extending that
bounded segment. Java slot accounting still bounds logical recursion. Cancelling
an owning class initializer marks the class erroneous and wakes waiters, preserving
partial side effects without replay. Reentrant initialization and waiting attempts
do not own that cleanup.

Explicit monitor opcodes return `ExecutionStep::Monitor` with one rooted identity
and the bytecode offset. The VM awaits acquisition after the native frame returns;
exit and completion resume at the exact following instruction, with pending Java
exceptions dispatched through the existing handler path. Synchronous embedding
uses real monitor helpers through the same thread operations. Production monitor
eligibility remains closed.

Interpreter and compiled code share an explicit-entry ownership ledger on the
logical Java thread. Each entry owns its monitor and object root; ordinary exits
remove the most recent matching entry, outside GC/context locks. Execution
boundaries record a non-reused acquisition serial. Cancelling a host invocation
releases its outstanding entries in reverse order, without consuming older outer
entries. Completed Java calls retain the existing non-structured exit policy and
rely on bytecode cleanup handlers; this does not add structured-locking enforcement.
A cancelled acquisition restores runnable thread status. Entries and roots are
released outside the ledger lock, and ownership is independent of OS threads.
Object wait/notify cancellation, arbitrary unbalanced bytecode and native monitor
reentry still require broader qualification. Ledger allocation, retained capacity
and monitor continuation overhead are unmeasured.

Allocation instructions return `ExecutionStep::Allocation` before class resolution,
`new`-triggered initialization, direct construction or publication can await.
Array dimensions are signed integer cells in declaration order; references remain
owned by the incoming frame snapshot. Results and exceptions acquire context owners
before raw bits reach the resume point. Cancellation drops unfinished constructors
and recovery state without replaying allocation or earlier Java side effects.
Array component resolution does not initialize that component. Synchronous helpers
delegate to the same async allocator through their existing bridge.

Every runtime request consumes a fuel unit. The VM carries the remaining fuel
across immediately ready monitor/allocation operations and yields before replacing
an exhausted budget. Direct invocations already yield unconditionally. Embeddings
using `Activation` must honor `remaining_budget()` or yield themselves; refreshing
fuel on every ready request can otherwise monopolize one executor poll. This bounds
local execution work, not runtime-helper duration or pause latency.

Other helpers inside a resumable method remain synchronous, including field access
and its class initialization: the bytecode budget does
not bound a helper's wall time or make it cancellable mid-call. Reference/call
production eligibility, relocation and complete helper/native cancellation
qualification remain open. Wasm uses its existing
cooperative interpreter. Diagnostic native resumes are separate from method-entry
counts, including in checkpoint intervals.

Experimental reference methods spill stable identities before each instruction.
The VM prepares all replacement owners before publishing the set, then releases
discarded owners outside the context lock. Failed validation leaves the old set
installed. A newly returned helper identity is independently owned until the next
snapshot. Collector scans use the ordinary root registry, so holding the frame's
result-map mutex does not block its root observation or unrelated reclamation.
The root spill buffer is capped at 4,096 identities (32 KiB); larger reference frames
are rejected. Snapshots with at most 32 entries/current owners reuse an unchanged
or reduced owner set without allocating or cloning owners. Removed owners move to
a stack buffer and are released after unlocking. Small changing sets prepare only
new owners in inline storage, reserve space for every insertion before committing,
and keep unchanged map entries. Validation/registration and owner cleanup run
outside the frame mutex. Larger sets still prepare a replacement map transactionally;
copies of already-owned handles share a context lock, which is released before
missing-identity validation and cleanup. A failed transfer preserves prior roots
and releases temporary owners. Dead reference locals are omitted. The
liveness matrix is capped at 8 MiB and the graph at one million edges; exceeding
either rejects compilation. Handler edges conservatively include every protected
instruction. Whole-invocation performance remains unqualified. The production allowlist emits no new root calls
and remains unchanged. See [the handoff contract](PUBLICATION.md#experimental-compiled-frame-root-handoff).

Experimental static calls pass logical argument cells to the normal VM dispatcher,
through either a guarded native helper or the call continuation above. The caller snapshot protects references during
resolution; decoded owning Values then protect arguments during declaring-class
initialization and nested execution. Returned references are owned before their
identity reaches generated code, and exceptions use the owning pending slot.
Native lowering reserves one reusable outgoing buffer sized to the largest runtime
request in the method (at least one cell for a zero-argument allocation). Resumable
requests copy only actual arguments into their owning continuation. Java call slots
and multidimensional arity remain capped at 255. Live-frame tests cover nested
interpreted/compiled/intrinsic execution and reclamation during an allocating
callee. The production gate remains closed for calls, and Wasm uses its interpreter.
See [the static-call contract and limits](PUBLICATION.md#experimental-static-call-abi).

The same experimental buffer/ownership protocol now serves constructors and other
`invokespecial` calls, with the receiver in cell zero. Runtime selection reuses the
interpreter's checked special-target cache; generated superclass calls do not use
receiver overrides. Constructor pressure tests preserve a receiver-only reference
edge while collection advances before return. Interface-default simulation now
accepts the required interface-method reference. The production gate is unchanged;
see [the special-call contract](PUBLICATION.md#experimental-constructor-and-special-call-abi).

Experimental virtual calls reuse an extracted interpreter target selector and its
four-entry receiver-class cache. Callee metadata includes its declaring class;
receiver ownership survives primitive-array class loading and nested execution.
Tests cover cached and uncached overrides, private/default/native selection,
exceptional cleanup and reclamation during calls. The production gate remains
closed; see [the virtual-call contract](PUBLICATION.md#experimental-virtual-call-abi).

Experimental interface calls use the same owning bridge and an extracted
interpreter selector for receiver membership, default/inherited targets and
selected-method errors. Redundant bytecode argument counts are checked against
Java slots, independently of the compact runtime cell count. This does not widen
production eligibility; see [the interface-call contract](PUBLICATION.md#experimental-interface-call-abi).

Compiled frames now share the interpreter's declared Java slot budget. Capacity
rejection precedes generated entry, and normal/unwind cleanup releases the frame's
recorded charge. Mixed recursion, throwable ownership, call-site backtraces,
synchronized-method cleanup and five-frame allocation pressure have bounded
regressions. The corrected synchronized test requires a real class monitor. A
compiled-nesting guard now handles the tested native-recursion exhaustion cases;
generated-frame spans now have a backend metadata check before entry, while
helper/reentry bounds and full host coverage remain unqualified. See the
[stack-budget contract](PUBLICATION.md#compiled-frame-java-stack-budget) and
[native recursion contract](PUBLICATION.md#native-compiled-recursion-and-method-monitors)
and [generated-frame bounds](PUBLICATION.md#generated-frame-size-checks).

An allocation-recovery waiter retains whether sufficient headroom was offered to
it, even if another allocator consumes that space before it is polled. Offers are
advisory and do not reserve credits. Logical reservation still uses an atomic
budget update. A lost offer does not spend the two-cycle failure budget; cycles
without an observed or offered opportunity do. Advisory headroom checks recheck
current capacity before returning. Without a usable offer, an unsatisfiable request
still fails after two completed recovery cycles. Offers survive waiter removal and
future migration and are discarded on cancellation without retaining heap bytes.
A size-indexed observation is installed before requesting the cycle, independently
of the first waiter poll. Each refund records the headroom from its atomic budget
update, before competitors can consume it. Only requests that fit that observed
headroom receive an opportunity; a smaller refund cannot prolong a larger request.
Recording invokes no wakers under collector locks. Normal notifications still
wake outside those locks and limit early wakeups to the sampled available capacity.

## Shared state machine

Native and cooperative hosts use these resumable phases:

1. **Roots:** visit context slots, shared slots, and numeric root IDs with bounded cursors, according to the selected storage policy.
2. **Remembered owners:** for young collections, visit owners that might have young children.
3. **Mark:** drain epoch-deduplicated gray work, resuming large objects at their next slot.
4. **References:** process soft/weak, final, and phantom references in order, draining newly shaded work between dependent stages.
5. **Sweep:** traverse the young allocation index or the complete registry, remove logically dead allocations under their object shards (and the default publication writer), and destroy reference-bearing payloads in bounded batches.
6. **Compact:** when enabled, pack surviving payloads in bounded chunks, then inspect and release empty regions incrementally.

Each step has a work budget. A slot scan uses at most 32 slots before returning to the scheduler. Small Java objects and primitive arrays advertise a smaller scan budget, so leaf arrays do not consume 32 units of cooperative work despite having no outgoing references. Root, reference, and sweep traversals retain cursors instead of copying entire registries. Tracing uses try-locks and yields on contention. Large arrays and instance field containers truncate dead values in batches so the final destructor does not repeat a linear walk over cleared slots. Clearing uses the descriptor's cached useful-slot budget, as marking does; leaf arrays do not consume 32 work units for an empty outgoing-edge clear. Native workers pace batches using `worker_yield_time_us` (50 microseconds by default, capped at 1 millisecond), bypassing sleep above 75% logical heap occupancy or while an async collection waiter needs completion; cooperative VM boundaries call the same operation. A native worker that cannot claim the coordinator backs off exponentially up to the configured pacing delay (capped at 1 ms), including under pressure; successful claims halve that delay. The idle handoff also uses a nonblocking coordinator claim and the same backoff; a request-to-active transition cannot strand an idle worker behind a long callback. Setting pacing to zero retains yielding without sleeps.

Young and full-heap cycles have independent epochs, request flags, queues, scan continuations, and traversal cursors. Allocation debt requests young cycles; repeated pending requests are coalesced. Young-cycle completion requests a full cycle at the configured interval, without depending on another allocation arriving. Under the default generation policy, young cycles can complete while an older full trace remains unfinished. Full cycles trace through young objects as required to discover old reachability. The separate experimental old-only policy below replaces intermediate full traversals with conservative old passes and retains precise full collection for recovery and reference processing.

`predictive_collection(true)` opts finite, nonzero heaps into sampled full-cycle
scheduling; it is disabled by default. The model estimates allocation bytes per
second and request/start-to-first-reclamation latency. A full cycle that reclaims
nothing contributes its completion latency instead. Fast-rising, slowly decaying
estimates respond to bursts without immediately discarding recent expensive cycles.
Full-cycle survival and live-byte observations provide a growth cooldown and a small
relocation allowance. The proposed headroom is rate times reclamation latency plus
burst, relocation, and physical-fragmentation allowances, clamped between roughly
3% and 50% of the logical heap. Before timing is available, a 25% base allowance
replaces the rate-times-latency term. These are advisory thresholds, not reservations.

Allocations contribute to a cumulative byte counter. Sampling normally occurs each
heap/64 bytes, bounded to 4 KiB–1 MiB, with shorter intervals near the trigger.
Only samples read a clock or try to lock the model and region registry; busy samples
are deferred. Region occupancy/free-byte totals are cached, and the existing extent
index supplies the largest hole without scanning regions. Dispersed free space
adds pressure only when physical backing headroom is low and the sampled allocation
would exceed the largest hole; alignment and allocation races can still make that
hole unusable. Smaller allocations can still use the other extents.

Prediction shares outstanding full requests with recovery. It coalesces its own
requests and requires fresh allocated bytes after completion, scaled by the observed
retained set, to avoid repeated automatic collections of an unchanged all-live heap.
The ordinary young-debt policy and bounded allocation-failure recovery remain active.
`scheduling_statistics()` exposes the latest estimates independently of diagnostics.
Bare wasm uses the same occupancy/growth policy without reading a host clock or
reporting fabricated rate/latency values; the host must still drive cooperative steps.
This policy does not remove publication locks, reserve evacuation regions, or adapt
the number of active workers. Qualification and measurements are in `VALIDATION.md`.

A cross-snapshot root pass retains objects already marked by the other active snapshot. For a young cycle it visits only the young allocation index. Newly discovered edges and publication/deletion barriers preserve both active snapshots. Promotion hands survivors to an active full snapshot and remembers their outgoing edges. Reference processing, sweeping and compaction are coordinated between generations; full tracing waits for an active young cycle before entering these terminal phases. This avoids competing finalization and weak-reference decisions. By default, marking overlaps while terminal phases serialize between generations. A `collect_full` completion ticket waits for a cycle started for that request, not merely an earlier in-progress cycle.

`overlapping_generations(true)` permits young progress during full sweep,
region selection and relocation. Strong tracing and reference decisions retain
existing interlocks. Allocations born after full reference processing carry a
separate physical-live mark: full sweep preserves them without aging/promoting,
but young cross-snapshot enumeration does not retain them. Full relocation skips
these births and revalidates callback eligibility. Its existing traced snapshot
continues to protect outgoing graphs; a descriptor storage lease alone would not
justify arbitrary callback access to child objects. A logically retired owner
cannot become a young reference-queue root just because it is old.

At full stage 5, closure checks only its own generation's in-flight reservations.
Young cycles can therefore finish while full copy callbacks are held. Pending young
phases receive coordinator turns even with persistent full copy queues, and young
completion leaves the full evacuation selection active. Same-object payload guards,
storage leases and object-shard retirement remain in force. The option defaults to
false; it does not implement old-only tracing or remove strong/reference interlocks.

Native `threads(n)` starts persistent workers. By default, workers claim tracing or relocation tasks under a short coordinator lock, release that lock while executing the task, then publish continuations. Per-object scan locks exclude overlapping scans of the same object; task-local young-edge state prevents workers from corrupting remembered-set decisions. In-flight accounting and publication-gate rechecks prevent termination while work or barriers remain outstanding. Sweeping and reference decisions remain coordinated. Cooperative wasm executes identical tasks through `step`; it never starts OS threads.

`mark_batch_size(n)` optionally claims up to eight tracing tasks together; the
runtime clamps values to 1..=8, including directly constructed configurations. The
default is one. Tasks use a fixed-size stack array and share the existing step work
budget: the sum of their reserved scan budgets cannot exceed the remaining work.
Each task still receives at most 32 scan units. Time is checked between callbacks;
a busy object or elapsed target returns unexecuted tasks with their original scan
offset, mutation version, and young-edge state. Custom callbacks can exceed the soft
target. Relocation continues to claim one task at a time.

With coordinated claims, the entire tracing batch enters its generation’s in-flight count
before releasing the coordinator. Executed and deferred tasks remain counted until
the coordinator is reacquired, then results publish while that lock excludes
termination. This protects tasks held locally but not yet started. Termination
still checks reservations in both generations. The coordinator and selected
object-index shards remain in use. Optional diagnostics report `mark_batches`,
executed `trace_tasks`, busy callbacks, and `trace_deferred` (claimed tasks returned
without execution), allowing claim amortization and wasted reservations to be measured.

Object marking, reclamation, promotion, freed bytes, and reference examinations
accumulate in step-local counters. These merge with the statistics snapshot at step
exit and before publishing cycle completion. Consequently snapshots during an active
step may lag its work; completed-cycle counters are flushed before its completion
ticket is released. This removes the statistics mutex from per-object processing,
while retaining the public snapshot API and cycle/time statistics.

`independent_marking(true)` additionally permits a bounded tracing-only step to
claim and publish work without acquiring the coordinator or a cycle mutex. It is
experimental and disabled by default. Fresh tasks and resumable continuations
use queues separate from phase state; the default is one queue per generation. Independent workers
retain the existing task and step work/time budgets, per-object scan locks, mutation
versions, and young-edge state. Roots, reference decisions, and sweeping
still use the coordinated state machine; relocation has a separate optional policy. Object/identity indexes use the selected
metadata shard count.

Queued tracing retains the exact metadata `Arc` for its allocation, including
continuations and stolen work. Claiming no longer looks up metadata by a raw
address. The callback acquires a storage lease from that metadata; a retired task
is discarded even if its address now indexes a different allocation or type.
Metadata ownership does not keep retired descriptor storage alive. Fresh queue
entries contain one `Arc` instead of a complete type-erased trace descriptor.
Remembered pruning validates both allocation identity and mutation version under
the object shard, so a stale completed task cannot prune a replacement object.

Conditional references retain the non-reused identity of their containing Reference
object as well as their referent. Successful constructor publication binds pending
owner slots before releasing them for processing. Failed attachment leaves an
unbound owner; reference liveness and queue insertion reject expired or unbound
identities even if the saved owner address now belongs to another allocation.

An independent step takes the claim gate for reading, checks published mark
readiness and terminal-phase priority, dequeues tasks, and adds their reservations
to their generation’s in-flight count before releasing the gate. A claim may temporarily
be absent from the queue but not yet counted; its read guard excludes termination
throughout that gap. Mark closure holds locks in the order coordinator, publication
writer, then claim writer. It checks both generations’ in-flight work before
checking every queue in the closing generation and
then disables readiness and advances reference/phase state. Claim readers never
acquire the publication gate or coordinator. Trace callbacks execute after dropping
the claim guard.

Once a step has a reservation, that reservation prevents either generation from
closing marking. It can claim further tasks within its remaining budget without
reacquiring the phase gate. All of the step's reservations, including completed
callbacks, remain outstanding until continuations and step-local statistics have
been published. The final decrement precedes recovery notifications and runs after
releasing every collector lock. This preserves completion-ticket visibility of
statistics. Reservation counts can conservatively include work already executed.

Pending idle generations and active root-enumeration phases get coordinator turns
before independent tracing. Failed independent claims leave the generation-selection
hint unchanged, allowing the fallback to close an empty generation while the other
still has tracing work. Terminal phases retain priority. `independent_mark_steps`
and `mark_gate_misses` in optional diagnostics distinguish accepted independent
steps from unavailable phase claims; accepted-step timing includes the first claim.

### Persistent tracing queues

`work_stealing(true)` selects persistent worker queues under either marking policy.
It is experimental and defaults to false. Each generation has a shared injector
plus up to sixteen worker queues, fixed at heap creation from the configured worker
count; additional workers share queues. Native workers keep a collector-specific
queue assignment across steps. External publication and cooperative callers use
queue zero. Queue ownership does not own application roots or execution contexts.
Wasm retains one queue and its deterministic work budget.

A worker publishes newly discovered edges and continuations to its queue. A local
queue holds at most 256 pending tasks; overflow goes to the injector after releasing
the local lock. A claim probes its local queue, the injector, then other queues
with a rotating victim cursor. Each queue is tried at most once. It skips contended
locks and removes at most the configured batch size from one available source,
charging every task against the existing step budget. Owners prioritize resumable
work from the front; thieves take fresh work, then continuations, from the back.
All continuation offsets, mutation versions and young-edge state travel with a task.
No queue lock spans metadata lookup, callbacks or another queue lock. The single
queue policy also uses nonblocking claims and preserves continuation-first order.
Publication probes available queues before using an owning atomic fallback FIFO.
That FIFO supports prepared append records shared by helpers, with exact-once
linking, cancellation before insertion and safe late retries after consumption.
Each help attempt performs bounded protocol work; retrying synchronous publication
has no hard latency bound. Current collector callers keep these records local and
retain admission, so this does not yet make admitted publication helpable.

An unsuccessful probe does not establish termination. The existing coordinator or
claim guard protects the entire dequeue-to-reservation gap, including stolen tasks.
Closure excludes publication and initial claims, checks both generations’ active
reservations, and validates queue counts and nonempty queue contents. Private FIFO
preparation is charged but is not linked work; announced publication must retain
its existing external protection until durable. A task
that has moved to a stack remains reserved until its results are published.
Queues belong to the heap rather than a worker’s stack: cooperative callers can
finish continuations after a worker departs. Generation interlocks remain
conservative; per-generation counters do not enable old-only reclamation.

Diagnostics report `mark_stolen` (including injector claims), `mark_queue_misses`,
and young/full arrays `queued_mark_tasks`, `in_flight_work`, and
`mark_queue_capacity` (allocated task slots). These are separately sampled
observations, not an atomic termination snapshot. Each local queue’s two backing
deques grow only to their bounded high-water capacities; the injector can grow
with queued heap work. Drained backing allocations remain reusable until `stop()`
trims empty queues, and heap shutdown releases all queue storage. The publication
gate and phase coordinator remain; mutation barriers are immediate by default.
Without independent relocation, copying uses the cycle’s shared queue.

## Concurrent mutation invariants

- By default, allocation/root publication and reclamation share a reader/writer gate. Roots are registered before allocation metadata becomes reclaimable; new allocations are shaded during an active cycle. Root publication shades the allocation even during sweeping. Fresh allocations are gray while tracing and black after the strong-mark boundary; their construction edges are protected by owning roots or reachable owners during publication.
- Dropping an overwritten heap edge shades the old referent during marking. Removing a root also shades its referent. This preserves the snapshot while tracing proceeds concurrently with mutators.
- Managed write guards demote newly stored owning values and publish owner mutation before unlocking. The owner enters the remembered set immediately or through a charged context record. Buffer overflow never discards required work.
- Instance fields and reference arrays maintain 32-slot cards in sparse bitmap words, with 64 cards per word. Minor scans of old owners visit dirty cards, rather than all slots. A card is retained while it contains young references. A card is pruned only when its entire contents were examined under one guard. Partial-card scans conservatively retain it, avoiding cross-worker or cross-generation scan-state races.
- A remembered owner is pruned only if its scan found no young children and its mutation version is unchanged while holding the object's metadata shard through remembered-set removal. Pruning no longer takes the publication writer. Later writes remember it again.
- At mark termination, the default policy takes the publication gate and rechecks gray work, in-flight tasks, and unpublished context records. Completion after compaction also closes publication and drains late gray work before retiring the snapshot. Reclamation also rechecks gray work before removing any allocation. Trace callbacks run outside the publication gate, avoiding an object-lock/publication-lock inversion.

Experimental `concurrent_publication(true)` replaces that gate with counted
admission and a revision/phase CAS. Allocation, root publication and conditional
upgrades never wait for a collector admission lock. Phase closure observes no
active publishers before checking pending/in-flight/queued work and validates the
same revision afterward. Per-slot decision locks protect conditional edges;
unpublished owners retain eligible referents with temporary roots and defer only
their slots, so construction hooks do not hold phase closure open. Stage 5 publishes logical retirement, after
which object shards protect sweep and numeric reconstruction rejects white graphs.
New allocations are black before insertion. Suspended futures retain roots without
admission; a stalled synchronous admission can still delay closure. The default
remains gated, and the experimental policy has not passed performance or endurance
qualification. See [PUBLICATION.md](PUBLICATION.md) for atomic events and limits.

An edge removed during a cycle can keep its target alive until a subsequent cycle. Tests explicitly allow this snapshot retention and verify eventual reclamation.

## Java reference processing and finalization

A Java Reference's referent is represented by a conditional slot instead of an ordinary strong field. Upgrading the slot and deciding its reachability share the selected publication protocol and a per-slot decision lock. Logical weak clearing is published at the strong-mark boundary; incremental physical clearing cannot allow another weak handle to revive an already weakly reachable target. The existing mark words distinguish strong reachability from final and phantom retention, so a later retaining upgrade cannot reverse an earlier decision. Cleared slots remain cleared. Slots created after the boundary defer decisions to the next cycle; clones preserve their source's decision. Slots retain a weak allocation identity as well as an address, preventing allocator address reuse from reviving expired handles. Epoch exhaustion triggers bounded inactive-domain reset scans before counter reuse.

Nonretaining references are cleared even when their Reference owner is not strongly reachable at the clearing boundary: that owner can later become reachable through finalization or another pending reference. Queue eligibility is decided after final-reference tracing. An unreachable Reference owner is never enqueued or resurrected just to deliver a notification. A reachable owner receives a pending root until the handler runs. Retaining final and Java 8 phantom edges depend on owner liveness so dead Reference objects cannot keep entire graphs alive.

Soft references use an eager clearing policy. Soft and weak decisions precede final-reference tracing. Final references keep objects alive while their Java finalizers are pending. Phantom processing follows that tracing. Java 8 phantom references retain their referents until explicitly cleared; later class libraries use clearing phantom semantics. Minor collections maintain a separate index of slots whose referents are young.

Pending Reference objects receive explicit roots until transferred to the Java reference handler. Java 8 uses `tryHandlePending`; later JDKs use the pending-list native methods. Handler tasks sleep on notifications, and Java class-library queues perform enqueueing and Cleaner/finalizer dispatch.

Finalizable instances register after `Object.<init>` completes; clones also register. Java finalizer code executes through Java tasks, not through the collector's Rust destructor callback. A final reference activates once, survives until cleared by the finalizer machinery, and supports resurrection through ordinary owning roots. The separate Rust `Finalize` API retains its restriction against accessing other GC allocations or allocating during sweep.

## Allocation pressure and shutdown

Runtime string conversion uses asynchronous constructors selected from a cached
Java layout. UTF-8 encoding/length scans and UTF-16 Latin-1 scans yield between
64 KiB input batches. Payload initialization writes directly to the selected heap
and yields between 64 KiB output batches. The payload becomes an owning root before
field allocation or final string publication can suspend. Cancellation releases
partial payloads; Java 8 character arrays, compact byte arrays, and lone surrogates
retain their previous representation. Existing synchronous factory APIs remain
available. MUTF-8-to-UTF-16 decoding before the factory and host input preparation
are still synchronous; these batch sizes are work bounds, not hard latency bounds.

String-array conversion, `Thread::object`, primordial-thread fields, module and
class-loader objects, and bootstrap lookup/argument publication now await recovery.
`Thread::object` uses the fallible path, so exhausting its managed heap produces
Java `OutOfMemoryError` instead of a Rust panic. Standard vector bootstrap arguments use chunked async preparation. JIT helpers
drive async allocation through the existing blocking-thread bridge, and standard
native object/array constructors await publication. Explicit synchronous embedding
APIs and custom synchronous preparation hooks retain their completion semantics. No managed read/write guard may span asynchronous allocation recovery.


Managed bytes are reserved before object publication and released after reclamation. Allocation debt separately schedules young collection. Checked array arithmetic and fallible backing allocation cover the issue #940 overflow path. Multidimensional allocation publishes and charges each parent before recursively allocating children. `Object.clone` awaits available managed space before fallibly copying its backing storage. It releases the source read guard before awaiting publication/recovery of the copy. The source handle and copied reference fields retain owning roots across suspension; finalizable clones still register after publication. The guarded clone copy remains synchronous; existing-buffer preparation has a chunked async path.

`Gc::try_new_once` attempts preparation and publication without collection and returns the unpublished value on failure. `Gc::try_new_async` retains the prepared value while awaiting logical credit. It reserves the required bytes atomically before publication and refunds that reservation if publication fails. An unpublished-storage guard destroys initialized data if descriptor setup unwinds, including after temporary root registration. `make_room_async` checks headroom without reserving bytes. Both can resume as soon as sufficient logical space is reclaimed; exhaustion is reported after two recovery-cycle completions without observing or being offered sufficient space. Losing a race to reserve advertised headroom does not consume that cycle budget. Direct physical backing waits can resume on layout-aware offers before completion; generic preparation and host descriptor recovery still await completion. Retry exhaustion counts completed cycles without ordinary reclamation, and backing recovery precedes initialization. An async allocation can therefore pass through separate headroom, backing-storage, and publication recovery stages. Exhaustion becomes Java `OutOfMemoryError`; a preallocated throwable avoids recursively allocating the failure itself.

Synchronous Java object and buffer constructors check logical headroom between GC batches. A returned reservation can satisfy the request while another thread is still tracing. Physical buffer allocation retries as soon as a suitable region becomes available. Both paths share an outstanding recovery ticket, and physical retries finish before the initializer runs. Generic payload preparation and host allocator failures can still require a full cycle. Cooperative VM entry points use the asynchronous APIs to yield between batches.

`collect_full_async` requests a full-cycle completion ticket. With native workers running, the requesting future registers a waker and suspends without spinning or running collection itself. Without workers, each poll performs at most one configured recovery step and yields before continuing. Recovery yields before each new cycle so consecutive failed retries cannot run two recovery steps in one allocation poll. Ordinary cooperative VM scheduling can also perform a step at its existing allocation/interpreter boundaries. These are work budgets, not wall-clock bounds.

Allocation recovery shares an outstanding full-cycle request where possible. Explicit `collect_full`/`collect_full_async` still request a fresh completion ticket. Each waiter has its own registration identity, so sharing a cycle does not overwrite another caller's waker. A second index orders allocation waiters by required bytes. After each step, the collector wakes eligible allocation waiters within the observed headroom and removes their registrations; notifications grant no reservation. The requesting task rechecks availability or atomically reserves bytes when polled. Explicit collection waiters wake only at cycle completion.

Waiter registration precedes headroom, completion, and worker-status rechecks to prevent lost wakeups. Progress notification releases collector locks and task-local tracing context before invoking wakers. Cancellation removes the waiter, drops unpublished data and initialized elements, and releases any reserved physical range; it leaves the requested cycle available to workers and other callers. Unpublished construction edges must retain owning roots across suspension. The final worker exit wakes waiting tasks so they can make cooperative progress. An unwinding worker failure wakes waiters with a collection error; this does not recover a failed collector.

The interpreter's `new`, `newarray`, `anewarray`, `multianewarray`, reflective array allocation, and `Object.clone` use async recovery. Standard runtime/native constructors also await recovery; synchronous embedding APIs and custom synchronous hooks retain inline recovery. Recovery requires released managed guards, and no publication lock spans an await. The managed heap limit does not bound process RSS or make every internal Rust metadata allocation fallible. Unpublished payloads acquire logical credit immediately before publication; region reservations count against the physical region limit while construction is suspended.

VM shutdown cancels spawned tasks, releases VM-owned class roots, and stops the worker. Initialized classes register weak metadata owners even when they have no allocated instances; the mirror registry covers classes that have mirrors without initialization. Task-local owning values remain valid until cancellation drops them. Standalone escaped values keep their collector alive. Wasm tasks use abortable futures; they do not require native thread APIs.

On WASI, a persistent per-VM `LocalSet` preserves Java reference handlers and finalizers across host API calls. Each outer host call drives this scheduler, including immediately ready calls. Shutdown drops the scheduler and its cancelled futures. Hosts must continue driving VM calls or waiting for non-daemon threads for cooperative Java tasks to progress; collecting the heap alone does not execute Java handlers. Bare wasm uses its existing event-loop task executor.

## Comparison with ZGC, G1 and Go

These are architectural comparisons with specific published versions. Performance
requires the paired workload results in [the comparison suite](../benchmarks/gc/README.md).

| Collector | Publication, movement and coordination | Relevant difference here |
| --- | --- | --- |
| ZGC, JDK 25 | Reference metadata and load/store barriers support concurrent relocation and marking. Expensive work is concurrent, with short pauses and required allocation headroom. | This collector uses stable descriptors and explicit owning roots; movable payloads still require managed guards. Counted admission does not implement ZGC's access protocol. |
| G1, JDK 25 | Concurrent marking and remembered sets accompany stop-the-world evacuation, remark and cleanup; full compaction is a fallback under pressure. | Incremental work budgets here do not establish a hard pause limit: publication, payload access and synchronous recovery can still stall. |
| Go 1.26 / Green Tea | Concurrent, non-generational, non-compacting mark/sweep uses write barriers and local allocation areas. Green Tea groups marking/scanning by page. Stop-the-world phase transitions and mutator assists remain. | This collector has generations and optional payload compaction, but pays for explicit root ownership, separate metadata and callback-based tracing. Batching tasks is not the same as scanning objects in page order. |

Sources: [JDK 25 ZGC barrier implementation](https://github.com/openjdk/jdk/blob/jdk-25-ga/src/hotspot/share/gc/z/zBarrier.hpp)
and [ZGC tuning guide](https://docs.oracle.com/en/java/javase/25/gctuning/z-garbage-collector1.html);
[G1 guide](https://docs.oracle.com/en/java/javase/25/gctuning/garbage-first-g1-garbage-collector1.html);
[Go 1.26 collector source](https://github.com/golang/go/blob/go1.26.0/src/runtime/mgc.go),
[Go 1.26 release notes](https://go.dev/doc/go1.26#runtime) and
[Green Tea design](https://go.dev/blog/greenteagc). These sources were checked on
2026-11-08. Their performance claims are not measurements of this implementation.

Our Java comparisons include interpreter/JIT and runtime costs. Collector-only
mechanism measurements isolate narrower operations and cannot establish superiority
over another JVM. Native ZGC/G1 measurements also do not represent Wasm execution;
WASI and browser results have separate cooperative scheduling contracts. Go and
C4 have not been executed by this suite.

## Comparison with C4

The comparison below uses the [2011 C4 paper, sections 2–4](https://lag.net/papers/content/C4-garbage-collector.pdf), not a claim about the current commercial implementation.

| Aspect | Published C4 design | This implementation |
| --- | --- | --- |
| Reference access | A loaded-value barrier enforces marking and relocation invariants and repairs the source reference. | Stable descriptors and object IDs; owning roots, deletion/publication barriers, and managed payload guards. |
| Movement | Concurrent relocation and reference remapping, supported by page and virtual-memory mechanisms. | Optional guarded payload copying in byte-bounded chunks; descriptors and arbitrary Rust values remain pinned. |
| Generations | Old-only marking consumes young-to-old roots from a specially triggered young cycle; further young cycles can continue. | Optional conservative old-only passes use a bounded young handoff and permit unrelated young progress. Conditional referents and cross-generation cycles still require precise full collection. Default full cycles trace the whole heap. |
| Coordination | Collector interlocks coordinate conflicting access and phase changes while mutators run. | The phase coordinator and claim interlocks remain; publication uses either the default writer or experimental counted admission and phase CAS. |

The paper distinguishes a design requiring no global safepoints from its then-current
implementation, which retained short phase-change safepoints. Neither that historical
result nor this collector's work budgets establish a universal pause bound. Counted
publication, distributed phase work and narrower generation interlocks are now
selectable implementations; their combined performance and endurance qualification
remains unfinished. [PLAN.md](PLAN.md) tracks those gates. Payload-lock removal
would additionally require a complete access/write-preservation protocol.

## Performance boundaries

This is a concurrent generational collector with parallel native tracing and relocation, and cooperative execution on threadless hosts. With `compact_heap(true)`, Java field and array payloads in both generations can move; descriptors and arbitrary Rust values remain pinned. Compaction is disabled by default until the endurance and latency release gates in `VALIDATION.md` pass. Held guards, repeated writes, or unavailable destination space can defer compaction.

The default generation policy overlaps young/full marking and serializes terminal phases. Optional generation overlap permits young reclamation during full sweep and relocation; optional old-only passes also permit unrelated young progress during old tracing. Precise full collection still traces the whole heap and coordinates reference decisions. Root population, dirty-card volume, reference processing, allocation rate, survival and conservatively retained snapshots determine total work.

`max_pause_time_us` is a soft scheduling target, not a fixed pause bound. Lock contention, OS scheduling, custom Rust callbacks, and system allocation/deallocation can exceed it. Explicit `collect_full()` and synchronous allocation APIs can complete full cycles inline. Async allocation recovery suspends the requesting task, but its completion latency still includes collection and memory pressure. Latency claims must cite the workload and measurements in `VALIDATION.md`.

Optional `diagnostics(true)` records contended publication/payload lock waits,
inline and awaited allocation recovery, async collection calls, cooperative recovery
assists, early allocation wakeups, registered waiters, caller and worker steps, root registrations,
coordinator misses, worker backoffs, and tracing/relocation outcomes. Unix hosts also report worker
thread CPU during steps. Bare wasm records event counts without clock reads.
`Statistics::total_pause_time_us` retains its legacy name but measures caller
assistance, not an observed global pause. Region statistics distinguish initial
staging copies, relocation copies, recopied bytes, and unavailable destinations.

## Execution contexts and buffered barriers

`RootStorage::Contexts` gives registered execution contexts reusable owning-root
slots. `ExecutionContext::scope` installs a context per future poll; suspended,
migrating, and cancelled tasks preserve roots independently of OS threads. Java
method execution uses these scopes when the policy is enabled. Escaped handles
keep their slots and context alive; unscoped handles use shared slots.

`barrier_buffer_size(1..=64)` buffers snapshot edges and dirty-owner notifications
during strong tracing. The default zero preserves immediate barriers. Nonempty
buffers have collector ownership and remain counted until their records become
mark/remembered work. Collector draining is bounded to one record per work unit,
rotating among contexts. Ordinary collector and explicit context drains need no
publication admission: a record stays charged until its resulting work is visible,
and closure checks pending reservations before queue emptiness. Terminal
reference/sweep phases use immediate barriers.
Producer insertion tries the buffer and, for its first record, the ready registry
before changing pending accounting. Contention falls back to immediate publication.
Overflow makes at most 64 prepared drain attempts, stopping on a busy required
index/claim and retaining that record for a collector. It retries insertion once,
then publishes the new record directly if necessary. These paths do not wait on
buffer/ready locks or on another buffered record's metadata. Direct publication
still synchronizes with the target's indexes and can allocate. Explicit context
`flush()` retains its synchronous contract and blocking pops.
The default publication writer still protects phase closure and retirement;
the counted policy uses its phase/revision protocol instead.
[PUBLICATION.md](PUBLICATION.md) specifies events, ordering, cancellation,
accounting, and the remaining gate-removal obligations.

Completed edge certificates have a separate optional cache in each context: two
recent entries plus four sets of four exact address/epoch records. Sustained
misses temporarily bypass the extension, with bounded probing afterward. Cache
reads and inserts use try-lock and cannot hold the required-record buffer or
delay a drained record's acknowledgement. Misses, eviction and contention fall
back to metadata validation. Dirty-owner notifications cannot use the cache, and
object-epoch rollover disables it. `context_storage_bytes` includes its fixed
storage; cache entries own no graph roots or storage leases.

Descriptor constructors derive their published pointers from the original
allocation after the attachment hook's exclusive borrow ends. This preserves
pointer provenance through attachment and full-allocation destruction, including
the nonzero descriptor marker used for zero-sized values. Constructors now keep
roots released by attachment and metadata hooks in a stack-scoped owner until
registration finishes. Existing owning registrations transfer without removal;
explicit root removals retain a replacement under their normal reader. Hooks and
system allocation execute outside publication admission for ordinary strong-edge
construction. Conditional-reference owner assignment briefly admits validation
and temporary rooting, then retains a per-slot pending count through construction.
Descriptor publication releases admission before owner identity binding; reference
processing skips a pending owner before taking its slot lock. A stalled
strong-edge or conditional-owner hook retains
its transferred roots and charged graph, while unrelated collection can complete.
Nested scope exit restores TLS, releases pending conditional-owner counts, and
moves strong registrations into the matching enclosing heap as a batch. An empty
parent reuses its vector backing. With no matching scope, detached cleanup removes
the exact registrations without consulting construction TLS again; deletion
barriers and admission remain unchanged. Populated-parent append and cleanup remain
linear. Rooted constructors now finish private metadata hooks before installing
their initial parent root. They retain a metadata owner in the calling constructor,
declared before the private root guard, and lend it to registration. On registration
unwind, the guard removes its exact registration without ambient scope adoption
before the caller's private metadata can destroy the payload. Successful construction
converts that registration into the existing client guard. Child roots retain their
independent construction lifetime; unsafe unrooted constructors retain their existing
caller obligations. The [cleanup evidence](VALIDATION.md#ordered-initial-root-cleanup-2026-11-14)
does not establish shared publication, replayable effects or bounded cleanup.
No construction scope spans an async suspension. Root installation and metadata
registration still use short publication readers.

Remembered-set pruning now holds only the object's metadata shard across its
mutation-version check and removal from the remembered set. Mutators advance the
version under that same shard before notification, preventing a newer mutation
from being erased. Pruning no longer takes the publication writer. The experimental counted protocol replaces the other graph-publication dependencies
as described below.

## Deferred descriptor destruction

Trace and relocation callbacks acquire an explicit descriptor storage lease.
Retirement closes admission; existing borrowers delay that object's clearing and
destruction while unrelated collection proceeds. Leases preserve storage, not
logical reachability or weak-reference eligibility. The last release wakes bounded
reclamation without requesting another cycle. Delayed storage stays charged to
`bytes_allocated`; diagnostics report `retired_bytes`, `retired_objects`, and
`retired_ready`. Collection tickets can finish while a borrower is stalled.
`stop()` drains ready work; outstanding leases retain their own storage. See
[PUBLICATION.md](PUBLICATION.md) for atomic ordering, notification handoff, and the
graph-publication requirements under both policies.

## Adaptive native worker pacing

`adaptive_workers(true)` enables advisory native activation. Worker zero samples
queued tracing work, allocation debt, occupancy, and recovery waiters at most every
2 ms. Sampling defers on controller/queue contention. The activation target stays
between one worker and the smaller of configured workers and available CPU
parallelism. Increased demand raises activation; decreases require three low-demand
samples per worker. Excess workers sleep for 1 ms between admission checks. Recovery
pressure removes productive-step delay; failed coordinator claims retain bounded
backoff. `worker_target`, `worker_pacing_us`, and `worker_throttles` expose policy
behavior without implying an exact count of simultaneously executing threads.

The policy changes scheduling only, not termination accounting or work ownership.
It defaults to false pending qualification and is ignored on cooperative wasm.
A coordinated step always admits its first work unit before checking the soft time
deadline; setup or host scheduling delays cannot cause endless zero-work steps.
Subsequent units still obey the configured work/time checks.

## Distributed relocation continuations

`independent_relocation(true)` enables a separate relocation claim reader/writer
lock and persistent queues. With work stealing enabled, native workers use the same
local queue mapping as tracing, capped at sixteen local queues plus an injector;
cooperative wasm uses one queue. Local queues overflow losslessly to the injector.
Enumeration prefetches at most sixteen tasks per configured worker (at most 256
queued tasks before deferring enumeration), with additional already claimed work
still visible in the generation's in-flight count.

A claim reads phase readiness, probes each queue at most once, removes a task, and
increments in-flight work before releasing its claim reader. It acquires a storage
lease and copies one configured chunk per callback. A step batches callbacks within
its work/time budget and publishes each continuation before claiming another task.
One reservation remains outstanding through every dequeue/publish gap until the
batch completes; the first callback always runs despite a soft deadline. Per-object payload guards and retry limits retain their existing
contracts. Root/reference/sweep enumeration stays coordinated. At the end of the
relocation scan, the coordinator takes the claim writer, checks queue counts and
both generations' in-flight work, then disables new claims before trimming regions
and attempting snapshot retirement. Late mark/barrier work retains its existing
publication-gate checks.

`independent_relocation_steps`, `relocation_stolen`, `relocation_queue_misses`,
`queued_relocation_tasks`, and `relocation_queue_capacity` expose the optional
queues. Counts sampled through diagnostics are not termination decisions.
`stop()` releases empty task capacity. Both relocation policies remain available;
this removes coordination from copy continuations, not from graph publication,
reference decisions, or phase transitions. Qualification remains pending.

### Physical recovery under competing allocation

Async payload construction and preparation retain a reclamation observation after
backing allocation fails. Returned payload ranges and shared descriptor/payload
backing refunds advance it, even if another allocator consumes that space before
the waiting task retries. Two consecutive recovery cycles without physical progress
still fail. Continuing reclamation resets that allowance; sustained contention or
unhelpful fragmentation can therefore delay completion. The observation is advisory,
not a reservation or a guarantee that a particular size/alignment fits. Successful
allocation performs no extra progress sampling. A forced test consumes each returned
range before the waiter retries, then holds it live to verify eventual OOM once
reclamation stops.

### JIT allocation recovery

Experimental resumable object/array instructions await direct construction and
publication after returning their generated native frame, using owning allocation
requests. Synchronous embedding and sentinel setup retain the existing
`block_in_place`/runtime bridge. Both paths share the allocator implementation.
Multidimensional arrays share the interpreter's builder, which roots and charges
each parent before children and releases managed guards before awaiting. One budget
counts 64 constructed nodes across recursion levels, including zero-length children;
large backing initialization also retains its byte-based yields. Pending parents
remain rooted until completion or cancellation. These work limits require no clock
and apply to native and Wasm interpreter execution, but do not bound host allocation,
locks or opaque callbacks. Production reference eligibility and global progress
qualification remain open.

### Experimental field requests

Compiled static and instance field operations return owning requests before class
resolution or static initialization can suspend. Their cells preserve primitive
bits and context-owned object identities. Both synchronous embedding and resumable
execution share the interpreter's checked field-reference cache: name and descriptor,
access, static/instance kind, final-write rules, receiver restrictions and narrowing.
Static operations initialize the actual declaring class. The generated path checks
null after resolution, and repeated cached Java field linkage errors preserve their
kind. Reads/writes retain managed payload guards only for the synchronous access;
no guard spans resolution or initialization awaits. Result roots precede completion.

Request fuel is carried across ready operations. Continuation compilation rejects
more than 65,536 declared transfer cells across restore sites before emitting the
save/restore code. This bounds that expansion, not wall time or all compiler work.
Neither change expands production eligibility or establishes a GC pause bound.

### Experimental type-check requests

`checkcast` and `instanceof` return their generated frame before symbolic class
resolution and assignability checks. The owning activation stores the bytecode
location, target constant-pool index and object identity; the runtime context's
published snapshot retains the operand. Resumption restores locals and residual
operands and consumes the shared request fuel. Cast success returns the existing
identity, and failure installs an owned pending throwable before handler dispatch.

The async implementation and synchronous embedding share resolution and checks;
assignability uses the interpreter's helper. Null skips symbolic resolution. Type
checks do not initialize their target. Managed payload guards end before any await.
Loader waits can be cancelled or migrated without retaining a generated frame.
Other synchronous helpers and native bridges remain separate progress work;
typed handler matching uses the owning request described below. Production reference eligibility and performance remain unqualified.

### Experimental reference-constant requests

String and class `ldc`/`ldc_w` instructions return their generated frame before
resolution or allocation. An argument-free request carries the pool index and
exact BCI; the context owns live locals/residual operands while resolution awaits.
The returned string or mirror is retained as a helper result before exposing its
identity. Normal frame publication then transfers that ownership. Requests consume
the same fuel as other runtime operations, including already-interned strings.

Interpreter, owning requests and synchronous embedding share one resolver. String
interning preserves MUTF-8/UTF-16, including unpaired surrogates. Class literals load
and link without target initialization. Native loader `ClassNotFound` becomes
`NoClassDefFoundError`; other resolution errors retain the existing handling.
The synchronous embedding still uses its executor bridge. Integer/float constants
and `ldc2_w` retain inline numeric lowering. Method-type/handle/dynamic constants,
production reference eligibility, broader class-resolution semantics and measured
request overhead remain unfinished.

### Experimental typed exception-match requests

Each typed table entry can return an `ExceptionMatchRequest` carrying its pool index
and throwing BCI. Its private resume token also identifies the table ordinal,
separating duplicate catch types and the throwing bytecode's own runtime request.
Dispatch blocks are shared per throwing BCI. Locals spill through the existing
continuation ABI; the throw's discarded operand stack is not restored. The context
owns the pending throwable and published local roots throughout resolution.
Matching consumes local fuel before returning from generated code, including when
class resolution is already cached. Catch-all handlers still need no type request.

The async matcher retains the exact throwable, resolves the target without class
initialization, acquires its source class with a yielding payload try-read, then
releases that guard before assignability awaits. A result of 1 selects the handler,
0 advances to the next entry, and -1 propagates a newly owned resolution failure
out of the method. Later handlers in that method cannot catch the replacement.
Synchronous embedding uses the same matcher through its existing executor bridge.
Native missing-class resolution is cached as `NoClassDefFoundError`; interpreter
catch lookup also propagates a Java linkage throwable on native missing-class errors.
Broader user class-loader error translation and access verification remain open.

Handler request-site expansion and saved frame state have explicit compiler limits;
excessive expansion rejects compilation before creating those extra resume sites.
This is not a bound on all compiler work. Throwable construction/`athrow` validation,
other native helpers and the collector's admission/held-writer tracing still have
separate progress requirements. Production handler eligibility remains closed.

### Checked reference-array stores

Interpreter `aastore` and synchronous/resumable compiled helpers share one checked
store. Null and bounds errors precede type checking. Non-null stores resolve the
array's actual component definition without initialization and use recursive
component assignability, preserving sibling-loader and multidimensional types.
The array/value remain owned while resolution waits. All payload guards end before
awaits; the commit reacquires the destination and revalidates type identity and bounds
before publishing the edge through the existing managed write barrier.

Experimental compiled stores return owning requests with array, signed index and
value cells before resolution or exception construction. Requests consume the shared
continuation fuel; cancellation releases their context-owned operands. Interpreter
stores now use the async boundary with popped operands owned by the future. The
synchronous embedding retains its bridge. This correctness/progress change does
not qualify production eligibility or
performance. In particular, newly required interpreter checks have unmeasured costs.

### Experimental array-access requests

Primitive/reference array loads, primitive stores and `arraylength` share a typed
request emitter. The activation owns array identity, signed index and value bits;
its context owns the corresponding roots. Each request consumes local fuel and
returns the generated frame before runtime access. A busy payload lock yields the
request without a guard, permitting migration and cancellation. Successful scalar
accesses hold the existing managed guard through the access and its write barrier.
Reference loads acquire an owning result under the array guard, then retain it in
the context after unlocking. Pending errors become Java throwables only after all
array guards end. Float/double cells preserve their raw bits.

Synchronous embedding uses the same checked implementation. Uncontended success
avoids the executor bridge; contention and throwable construction use it after
guards end. Boolean stores apply the JVM low-bit rule, also used by the interpreter.
Other interpreter array accesses retain their existing synchronous guard acquisition.
The production JIT gate remains unchanged.

A held read guard can suspend a store while collection proceeds. A held write guard
on `Gc<RwLock<Reference>>` still excludes collector tracing, even when the current
variant is a primitive array: the enum's type-level tracing contract permits edges.
The request can be cancelled, but this does not supply the missing D/E graph-progress
protocol. Remaining reference-store/field guard acquisition, collector admissions,
exception matching and native helpers also need progress qualification. Request
transfer and contention-retry costs have not been measured.

### Bounded publication preparation and native arguments

`Trace::prepare_step` lets an unpublished value prepare storage in bounded batches.
The compatibility default invokes its existing synchronous `prepare` hook; the
Java `Reference`, field/array storage, and managed-cell implementations forward the
budget to `HeapBuffer`. Async publication yields between 64 KiB batches (at least
one element) and rejects active managed guards before preparation begins. Direct
constructors already in the selected arena need no staging copy.

A partially prepared destination owns raw storage only. The source retains every
initialized element until the final copy commits ownership. Cancellation frees the
destination without invoking element destructors there. Mutation, truncation,
compaction, or changing target heaps cancels incompatible preparation. Truncation
publishes its shorter initialized length before invoking destructors, so unwinding
cannot expose or destroy the removed suffix again.

Native Java object/array publication now awaits recovery. Native field constructors
and module-map buckets initialize directly in managed storage. Environment and
mount/ZIP metadata snapshots finish before awaits; socket result arrays acquire
write guards after allocation. Platform-specific native declarations that allocate
are async in the generated intrinsic registry.

`RustValue::try_to_value_async` provides an object-safe conversion future used by VM
invocation. Scalars and existing custom implementations return an immediate result
without a boxed future. The fourteen built-in vector conversions initialize directly
in chunks and await publication, preserving existing signedness, character truncation,
and array-kind behavior. Custom implementations can override the hook; their existing
synchronous methods retain their completion semantics. Host I/O, decoding, arbitrary
custom callbacks, and guarded cloning can still perform synchronous work.

Physical recovery counts returned ordinary ranges and backing that contained such
reclamation. Raw copy destinations and transferred sources do not reset the retry
allowance merely by moving live bytes between regions. Without this distinction,
repeated evacuation of an unchanged live set could keep an impossible allocation
waiting forever. Destinations become ordinary owned ranges at copy commit, so
later destruction still records real reclamation. Copy/release metrics continue
to report all physical activity separately from this recovery signal.

### Guard timing and early physical recovery

With cost-aware relocation and diagnostics enabled, `payload_guard_holds` records
managed read/write guard lifetimes from acquisition bookkeeping through unlock.
`active_payload_guards` reports unfinished samples separately. Mapped projections
retain the parent sample; failed acquisitions create none. Write-guard mutation
barriers run while the lock remains held and are included. Tracing's managed read
guards are included; relocation's raw exclusive copy locks are excluded. The
measurement stores an inline start time and shared counter ownership, allocating
nothing per guard and retaining no additional heap graph. Bare wasm counts holds
without reading a clock. This differs from `relocation_pin_deferral`, which measures
a failed copy attempt through a subsequent non-busy attempt, including scheduling.

Direct async payload construction now waits for either an advisory physical-space
offer or full-cycle completion. Readiness probes are nonblocking and conservative:
they exclude selected sources, respect ordinary/shared reserve caps, and require
an extent large enough to guarantee alignment. Tight alignment fits may wait for
completion and then retry the actual allocator. Notification examines at most 32
physical waiters per pass with a rotating cursor; cancellation removes the entry.
Wakers run after both registry and waiter locks are released. An offer reserves no
bytes, so a losing allocator re-registers without spending a completed-cycle retry.

Physical retry exhaustion counts completed full cycles without returned ordinary
ranges/backing. Each direct retry yields first. A cooperative physical wait executes
one configured step before accepting an early hint, ensuring OS allocation failure
cannot cause a no-progress loop merely because arena credits appear sufficient.
Generic `Trace::prepare_step` recovery still waits for completion because its failed
allocation's layout is not exposed. Logical-credit waits retain their separate
size-indexed observation protocol. These changes do not remove publication or
payload locks and do not promise allocation success for an exhausted live set.


## Distributed phase work and region registries

`independent_phases(true)` uses per-domain cycle locks for bounded root enumeration,
conditional-slot decisions and sweep units. The global coordinator is acquired for
cycle start, tracing/reference closure, sweep-to-copy handoff and completion.
Coordinator holders try cycle locks rather than waiting for a callback that owns
one. Root read-to-shade work retains publication admission; independent conditional
slot and retirement units retain counted admission or the compatibility writer.
Collector acquisition of that writer is a try-lock; a failed attempt yields the
step without advancing its phase/cursor. Mark-queue termination probes distinguish
contention from emptiness, and terminal mark/copy claims also use try-locks. These
paths release coordination when busy; their unchanged pending/in-flight/admission
requirements can still prevent collection completion.
Callbacks execute after those admissions end. Mark/copy claims become visible in
in-flight accounting before releasing admission, and continuations publish before
reservations retire. Sweep finalization, clearing and destruction move their
descriptor ownership outside both cycle and coordinator locks in every policy.
Blocked reclamation callbacks retain charged storage while unrelated full cycles
continue. A returned clear continuation resumes only in its original sweep epoch
or transfers to the retirement queue. When a storage lease outlives sweep,
the collector charges the descriptor and arms a notification in its storage
state. The sweeper or the last borrower then publishes one owning task. A
reader paused at the queue head leaves other helpers free to claim ready
entries. The queue is allocated on the first deferred handoff. Explicit
tracing generation interlocks and payload guards still apply. On targets that
unwind panics, failure during a retired callback destroys its remaining
payload, refunds the bytes and completes its ready-work accounting before
propagating the panic to the caller.

`region_shards(1..=16)` gives each registry its own available-range index and region
map. A thread preference reuses existing storage without owning ranges or requiring
a thread-exit handoff. Allocation tries available shards first, then rechecks busy
shards before reserving backing or reporting exhaustion. Region IDs encode their
registry; ranges and escaped blocks retain the exact region and its accounting.
Backing allocation/deallocation remains outside registry locks. All registries and
both arenas use the original atomic shared budget, including evacuation reserve.

Evacuation builds per-registry cost indexes in bounded units, compares their next
candidates, and rechecks the selected candidate under its registry. The four-source
limit is shared across the entire payload heap. Destination roles share the cycle
epoch, so copying can cross registries without making destinations current sources.
Trimming retains a bounded cursor across registries. Aggregate statistics are
sequential samples; the shared budget and its high-water mark enforce the cap.
Both new policies remain experimental and disabled by default.

### Experimental old-only passes

`old_collection_interval(N)` with nonzero `N` requests a conservative old-only
pass after every N young completions. The existing precise full interval takes
priority. `collect_old()` requests that pass directly (or a full pass when the
option is disabled). The option enables independent phase units and generation
overlap; counted publication, independent marking and independent relocation remain
separate switches. The default interval is zero.

An old pass enumerates the initial young cohort in bounded handoff units, making
one initial pass over its outgoing old edges. Promotions can require a new traversal.
The old tracing queues then contain only old objects,
including promotions published during the pass. Young objects created after the
old snapshot carry a separate birth epoch and are omitted from the handoff.
Owning-root changes do not turn those births into old snapshot roots. An old
callback that actually traces a young edge retains that graph for subsequent young
snapshots. The initial young cohort and those retained graphs remain charged to
the normal heap limit; a stalled callback can retain its snapshot.

Young collections can finish during an unrelated old tracing callback, sweeping,
and relocation. Tracing and old copying callbacks share their allocation's scan
lock with young remembered-owner scans. A conflicting owner can delay the young
cycle; clean unrelated old callbacks do not hold a global in-flight interlock.
Each traced edge has publication admission. Promotion marks old liveness before
publishing the old age and publishes its old traversal before admission ends.
Old sweeping and copying exclude objects that are still young. A logically retired
owner cannot be queued by another generation's remembered scan.

Conditional old referents are conservative roots of an old pass. Old passes make
no weak/final/phantom decisions and cannot complete a full-collection ticket.
Precise full passes preserve the existing ordered reference processing and reclaim
cross-generation cycles and conservatively retained garbage. With old-only passes
enabled, a precise full pass first drains an active young snapshot, then defers new
young starts through full tracing/reference processing. This prevents conservative
old handoff roots from being imported into consecutive allocation-recovery full
passes. Young collection can resume during full sweep/relocation.

`old_collections`, `old_handoff_objects` and `old_trace_tasks` distinguish this work
from precise full collections. An additional per-object birth epoch costs eight
bytes before structure alignment. This algorithm is experimental; it does not
establish the plan's throughput, latency or endurance targets.

Primitive `Vec` and `HeapBuffer` values no longer spend tracing units walking
non-reference elements. `Trace::may_contain_gc_references()` defaults to true for
custom types; primitives and strings opt out, and supported containers propagate
that fact. Clearing can discard a reference-free buffer in one unit only when its
elements also need no destructor. Elements with destructors still clear in bounded
batches. This removes redundant handoff work without skipping managed edges or
user destruction.

## Archived catalog experiment

The retired catalog used fixed backing for immutable membership and operation
records. Holding an old version delayed reuse of its slot. The allocation
screen reported fewer system allocations and more retained backing.
[Archived pool and measurements](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/catalog-pooled-prototype/README.md).
