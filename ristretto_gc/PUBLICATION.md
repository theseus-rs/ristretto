# Publication and buffered-barrier protocol

This describes both implemented publication policies. The default uses the global
reader/writer gate. Experimental `concurrent_publication(true)` instead uses
counted admission, atomic phase publication, per-reference decisions, and object
shards for retirement. The gated policy remains selectable while the replacement
is qualified. Both policies retain descriptor leases and generation interlocks.

## Concrete leaf ownership experiment

The optional [canonical leaf adapter](CANONICAL_LEAVES.md) transfers a private,
reference-free allocation directly into an owning catalog record. Its exact heap
and record borrow survives helped publication and retained-cell reclamation. It
never wraps legacy registration in a completion flag. Parent roots, construction
protection, opaque tracing, finalizers and heap-owner callbacks are rejected with
the original publication intact. Public constructors, graph handling and the two
implemented publication policies above remain unchanged.

The internal typed leaf view additionally checks a live root, the publication
certificate and exact TypeId. It retains the root/record/heap borrows through
access; revocation prevents later admission without invalidating an existing
view's storage. No additional pooled owner escapes the root's quota. See the
[access protocol](CANONICAL_LEAVES.md#typed-reads-through-canonical-roots).

The internal [leaf allocation transaction](CANONICAL_ALLOCATION.md) now shares
private descriptor preparation with the legacy constructor, then retains canonical
record/root creation, publication and typed result transfer in one participant.
Cancellation posts exact-cell abandonment intent; committed discarded results
request root removal. Existing public constructors still select legacy publication.

The internal [maintenance helper](CANONICAL_MAINTENANCE.md) now performs requested
root cleanup, leaf acknowledgments, retirement and physical release. It requires a
complete same-epoch sweep and retains that epoch in Finish requests, preventing a
delayed helper from closing a newer unswept collection. The subsequent internal
[runtime owner](CANONICAL_RUNTIME.md) binds participants and reads to the exact
heap/catalog and reserves owning-wrapper storage through final backing release.
Public selection, graph/callback integration and whole-GC qualification remain open. The
[automatic allocation follow-up](CANONICAL_RETRY.md) keeps exact private ownership
across slot collisions and full/contended root quota; it does not add scheduling or
qualify arbitrary callbacks.

## Helpable-publication redesign remains required

[The stage-D design checks](validation/HELPABLE_PUBLICATION.md) now contain four
executable counterexamples to early acknowledgement/replay of the existing index,
mark and queue operations, plus seven reduced positive atomic-cell checks. A
completion bit alone permits stale helpers to rewind an epoch or replace a reused
identity; a mark bit alone does not prove queue publication. A canonical membership
cell also needs a phase-coupled announcement boundary. These are design models,
not a replacement runtime protocol. The next representation change must establish
canonical descriptor membership and durable traversal visibility before migrating
registration/root/barrier admission. Current counted/legacy behavior is unchanged.

## Experimental owning identity index

The Cargo feature `ristretto_gc/experimental-identity-index` replaces the
ordinary sparse identity pages. The historical comparison below used the
earlier mutex-backed identity map. Object-address membership remains
authoritative, and handle reconstruction still checks identity, type and logical
liveness under the object shard and existing admission.

The prototype uses 4,096 buckets with owning atomic links. A link combines its
successor with a live/deleted/detached state in one CAS. Logical deletion changes
the exact live state to deleted with the same successor. Physical unlinking CASes
the exact, still-live predecessor state; only a successful unlink may detach the
removed node's successor. Marking a predecessor invalidates every earlier unlink
proposal through it. Insertion now accepts a shared owning intent, as described
below; bucket headers have a separate immutable owning identity.

Each node starts with eight immutable, initialized marker bytes and has explicit
eight-byte alignment. The private `RefCnt` adapter encodes nonempty live/deleted
links as pointers to bytes two/four inside the successor's `Arc` allocation. It
recovers the original data pointer with provenance-preserving pointer arithmetic;
it never dereferences an encoded pointer or constructs an integer-derived pointer.
Each nonempty state owns exactly one strong reference. Null represents an empty
live link; two distinct static markers represent empty deleted/detached links and
own no reference count. This removes separate interior link-state allocations,
including those previously needed for logical deletion. Physical removal of a
bucket head instead allocates the fresh version required by shared insertion.

The adapter was audited against the installed ArcSwap 1.9.4 `RefCnt` and debt
implementation. Its strong-owner tag stays zero, as for ordinary `Arc`; the even
marker offsets cannot alias the weak-owner tag or empty-debt sentinel. Pointer
conversions neither allocate nor panic. Native tests exhaust fast borrow slots,
mix ordinary and marker borrows, and race state changes with unlinking. Miri under
both aliasing models, ASan and TSan provide additional evidence; Loom models the
shared list transitions with linearizable owning links, not ArcSwap internals.
An ArcSwap upgrade requires this adapter contract to be rechecked.

Owning snapshots prevent address-reuse ABA. Non-root successors advance only by
skipping retired nodes and cannot return after removal. The earlier exclusive
insertion prototype allowed the bucket head to return from A to B to A; that
argument is insufficient for shared insertion after B has retired. The current
protocol gives every bucket version, including empty versions, a fresh owning
header. Replacing the header never reinstalls an earlier header, even when its node
is the same. A delayed helper owns its expected header, so neither logical head
return nor allocator address reuse can make an invalidated CAS succeed.

An insertion intent owns an atomic pointer to an immutable attempt: the expected
header, proposed header and privately initialized node/successor. Helpers CAS that
exact expected bucket version to the proposed one. A failed helper may replace the
intent's attempt only after observing that the expected header has permanently
left the bucket. It prepares a new node and frozen successor before CASing the
attempt pointer. Losing proposals have no public effects. No helper rewrites the
successor of another helper's published node.

All attempts for an intent share an irreversible insertion flag. A winner records
success after its bucket CAS. If it stops before that store, another helper can
recognize the proposed header, and **every** bucket modifier stores the previous
head's insertion flag before replacing that header. A helper which observes a
different bucket header rechecks the flag before retargeting. That ordering covers
insertion followed by replacement/removal between its first flag read and header
read. A delayed helper may still attempt its old CAS after acknowledgement; the
unique expected-header condition makes that CAS fail. Merely checking the flag
before a replayable operation would not suffice.

Preparation alone is not membership. Dropping the last intent owner cancels
private proposals; dropping a completed intent leaves membership with the bucket.
Helpers return successful insertion even after the entry has subsequently retired;
this result is historical completion, not proof that its payload is live. The
collector's existing handle validation remains required. Bucket headers do not own
previous headers. An attempt retains only its two headers, while normal node
unlinking detaches retired list history. The forced stopped-helper fixture retains
two nodes across 1,000 later insert/remove pairs and releases them on final drop.
The node counter does not account for headers, attempts or insertion flags.

Sweep retains one traversal cursor per object shard and visits at most one list
node per removal attempt. It acquires the object/young/remembered guards before
logical removal; earlier traversal attempts leave the target identity visible.
A dirty-bucket bitmap drives maintenance: one node or up to 64 empty bitmap words
per unit. Taking bits before traversal preserves later notifications; dropping a
partial cursor returns its active bucket and unvisited bits. A cursor owns no
payload or user callback. Normal unlinking detaches retired history, and whole-list
shutdown uses iterative destruction rather than recursive stack growth.

The dirty notification follows logical deletion. This remains inside the
collector's existing admission: the bitmap is maintenance scheduling, **not** a
phase-closure certificate. Neither lookup nor insertion is wait-free. Lookups can
walk long collision chains and CAS retries can repeat under contention; a bounded
structural visit does not bound allocator, reference-count or host scheduling time.
One help call makes one insertion or retarget attempt, but retargeting allocates;
the synchronous wrapper retries. Index nodes, headers, attempts and flag backing
are not yet part of complete physical-budget accounting.

The list provides idempotent shared insertion. Ordinary registration still
holds the object shard for descriptor insertion and nursery updates. It releases
the shard before assigning birth marks or handing work to the queue. An admitted
publisher can still exclude phase closure.
The list by itself does not provide
canonical catalog membership, discoverable registration records, helped admission,
or the stalled-writer graph-retention protocol required by stages D and E. Current
registration keeps its intent local. Active counted births retain admission
through marks and queue publication.

## Yielding collector-side synchronization

`try_publication_write` returns a phase token on success, including an explicit
empty token for counted mode's gate bypass. Contention returns no token. Cycle
start, marking/reference closure, sweeping and final retirement yield without
advancing the corresponding cursor/phase when the legacy writer is unavailable.
No production collector path calls the legacy writer's blocking `write` method.
An existing reader keeps its original protection. The native recursive lock can
already admit other readers while a writer waits; this change specifically stops
the collector from retaining its coordinator/cursor during that wait.

Mark claims try the injector even when it is the only queue. Producers try their
local queue and injector, then probe other bounded worker queues before publishing
to an owning atomic FIFO when ordinary queues are unavailable.
Each ordinary queue publishes its current count under its mutex. A termination
probe skips a mutex when its count is zero, returns `Some(false)` for linked FIFO
work or observed ordinary work, and returns `None` for a busy nonempty ordinary queue.
Callers yield on `None`. Publication and claim interlocks plus in-flight/pending
checks remain necessary around a successful empty probe. The counts alone are not
an independent termination proof.

Terminal mark/copy claim writers also use try-locks. An unreserved popped claim
continues to exclude closure, but its stalled owner no longer makes a competing
step wait while retaining global coordination. Mutator index operations, explicit
barrier flush/overflow and opaque tracing/handoff callbacks retain waits and
reservations. Queue publication can still wait in the host allocator. Collector
buffer claims now yield as described below. None of these changes allow closure
past an unfinished publisher.

`publication_write_wait` remains in diagnostic reports for compatibility but no
longer records blocking writer acquisition. Failed writer attempts contribute to
`publication_retries`, alongside counted observation/commit retries. A zero writer
wait count is not a zero-STW measurement; actual writer hold time and mutator waits
are separate. Prior reports retain the earlier counter semantics.

## Contended mark publication

After its local queue and injector are unavailable, a producer probes the remaining
worker queues once each in circular order. A count at the worker bound can skip a
probe; an acquired mutex rechecks the bound before insertion. A stale full hint can
miss newly freed capacity without losing publication: the producer still falls back
to the FIFO. There are at most sixteen worker queues, and no acquisition waits or
nests another queue lock. Ordinary queues can still grow their deque storage under
the mutex. This reuses their bounded task capacity; it does not promise allocation-
free publication or make system allocation helpable.

The fallback FIFO links fully initialized nodes through owning `ArcSwapOption`
references. Publication linearizes at the predecessor-link CAS. Its producer need
not advance the advisory tail before another producer or consumer can make progress;
both help advance that tail. Owning references keep every compared pointer alive
and prevent address-reuse ABA. The successful head-CAS claimant alone takes the
successor's payload, using a mutex solely for safe interior mutability. Later
claimants never access that payload. It is removed immediately, so the empty head
retains no allocation metadata or class owner.

A detached predecessor's link becomes a shared non-null tombstone. Clearing it to
null would allow a producer paused before its link CAS to append onto a detached
node. A delayed reader can retain detached nodes but cannot retain the consumed
history through their links. Destruction walks uniquely owned links iteratively
using `Arc::into_inner`, avoiding recursive destruction of an arbitrarily long
chain. A shared outcome consumes its reference without dropping the pointee; using
`try_unwrap(...).ok()` here could race another owner release and recurse.

The FIFO now has an owned `Prepared` append record. Preparation allocates its node,
chooses its first predecessor privately and charges the reservation before sharing
the record. The initial choice needs no CAS; a stale predecessor forces a retry
through its permanently non-null next link. Each `help` call performs
one bounded protocol attempt, without allocating a queue node or locking a payload;
the synchronous `push` API retries these attempts. The record's owning predecessor
link selects a single append location. Helpers change that choice only after its
next link is permanently non-null and belongs to another node. A helper retaining
the old predecessor cannot subsequently insert there, because its null-link CAS
will fail. Multiple helpers can complete the same record without an exclusive
claim. Queue identity is checked before any helping mutation.

Insertion linearizes at the predecessor's null-to-node CAS. The record's node
also carries an irreversible linked bit. Producers observing their own node at
the predecessor link can set it; consumers set it before replacing that link with
the tombstone. A helper observing the tombstone rechecks the bit with Acquire
ordering before changing its predecessor. Thus a consumer may finish before the
original publisher returns, and late helpers cannot reinsert a consumed node.
The completion bit is evidence of this exact immutable link transition; it does
not permit replaying arbitrary index/mark mutations behind a separate done bit.

Last-owner record drop cancels an unlinked intent and refunds its reservation.
It checks both the irreversible bit and predecessor link, then rereads the bit to
cover concurrent consumption. No helper can still borrow a record during its drop.
A linked record's payload/reservation instead belongs to its successful consumer.
An outstanding record owns its node and selected predecessor, plus references held
by active helpers; consumed links do not retain an expanding completed history.
It does not own the queue. Pending, unconsumed work and host-allocated nodes still
need the catalog/budget policy required by stage D.

The synchronous append entry is outlined from ordinary marking. Native queue
headers use 64-byte alignment to separate them from adjacent domain state; Wasm
retains its cooperative layout. These changes have measured costs and regressions,
not a general performance guarantee; see the latest validation and comparison.

The FIFO reservation count increments during preparation and decrements after head
claim or cancellation. Positive counts may include private, unlinked records;
termination inspects the links when the count is nonzero. Private preparation alone
does not prevent closure. Zero does not account for a popped task whose collector
in-flight reservation has not yet been installed.
Existing publication admission/barrier-pending protection spans enqueue, and the
claim reader or coordinator spans dequeue through reservation. Ordinary queue
counts use Release stores under their mutex, before that external protection ends.
Closure uses the existing publication observation/commit protocol, claim exclusion,
and Acquire pending/in-flight checks before accepting zero queue counts. This is
why a held empty injector can be skipped, but an admitted unfinished publisher or
unreserved claim still excludes closure. Advisory queue samples use the same counts
without promising a coherent snapshot. Fallback and ordinary claims alternate while
both have pending work; each claim retains the existing batch and trace-work limits.

Ordinary claims also use zero counts as an advisory reason to skip an empty queue,
avoiding mutex interference with its next producer. A concurrent insertion may be
missed by that attempt, just as a contended mutex can make a claim yield. A claim
with no result never certifies closure: the separate termination protocol above
must still run. Counts are rewritten after actual dequeues only; an empty probe
does not write the queue's count. This changes the eligible probes counted by
`mark_queue_misses`; zero-hint skips do not count as failed mutex acquisitions.

Each generation pays for two fixed nodes (dummy head and tombstone) and shared
node/reservation counters. Each fallback publication allocates one node. Capacity diagnostics
count retained nodes beyond the fixed two, not allocator bytes or these fixed
allocations. Ordinary fast queues still allocate their storage under their mutex;
fallback node allocation also depends on the host allocator. CAS retries are not
wait-free and supply no hard latency bound. A stalled nonempty ordinary queue still
hides its existing work. The collector's current `push` keeps its prepared record
local and retains external admission through enqueue. It does not yet announce
that record in a discoverable catalog or transfer admission to a helper. A stalled
admitted publisher therefore still excludes closure, and opaque callbacks still
protect their graphs. The owned append protocol is a prerequisite for durable
helped work; it does not replace the collector's publication/phase protocol.

## Allocation and reference events

For slot/context roots, page allocation and slot-writer waiting now precede root
admission. A reservation contains no target address and provides no reachability.
Collection reads a separate atomic page view and can finish while preparation
holds a writer mutex. After admission, identity/type and conditional eligibility
are revalidated, then the address is committed and shaded. Error/unwind cancellation
releases the reservation after the caller's admission ends. Owning slot deletion
claims its generation atomically and defers page cleanup until after admission.
Compatibility lookup validates generations through the page view. Tree-root
publication, context registry and metadata/shading waits remain, as does any
admission already owned by an enclosing caller.

The table describes the default gated policy; the counted replacement is specified
below. Both policies share construction ownership, frozen reachability and storage leases.

| Operation | Event and protection |
| --- | --- |
| Construct | `Initialized<Allocation<T>>` owns initialized descriptor storage; a logical reservation remains charged. Initial owning fields protect construction edges. Async initialization retains its owning inputs across suspension. Chunked staging keeps every source element owned until commit; the partial destination owns raw bytes only. |
| Attach | A stack-scoped construction owner retains registrations released by hooks and eligible conditional referents. A per-slot pending count defers conditional decisions through registration. Hooks and system allocation hold no global admission. The constructor invokes `Trace::attach` exclusively, then derives the published value pointer from the original allocation pointer. A pointer reborrowed from the value before attachment cannot be reused after the hook's mutable borrow. |
| Publish | Install the owning root under a short publication reader, then install identity/object metadata under another reader. Until registration, root enumeration finds no descriptor metadata; registration itself applies the current phase barrier. Active marking shades the object; terminal phases mark new allocations black. Transfer initialized storage and logical credit ownership to metadata before registry visibility; returning from registration no longer commits a constructor-local reservation. Other publishers may run concurrently; retirement cannot. |
| Acquire root | Insert the tree/slot/context registration and shade under the publication reader. Existing owning registrations can transfer to a handle without another insertion. Root registration does not validate an unsafe unrooted pointer's lifetime. |
| Release root | Claim the exact slot generation and clear its address under a publication reader, preserving the target through its deletion barrier before releasing occupancy. Owning slot deletion needs no registry lock. Page cleanup follows admission using try-lock/deferred work. Tree and compatibility operations still synchronize. |
| Upgrade weak | Under a publication reader, validate the descriptor identity and logical reference-processing state before creating an owning root. A physically present object is not necessarily eligible for weak upgrade. |
| Clear weak | Under the writer and coordinated reference stage, publish the stage's logical clearing decision and clear applicable slots. Final/phantom retention and queue eligibility remain distinct from weak upgrading. |
| Reserve collector work | A coordinated claim or independent claim reader spans dequeue through incrementing generation in-flight reservations. Tracing and optional relocation closure take their respective claim writers. Continuations/statistics publish before the last independent reservation retires. |
| Retire | Under coordinated sweep and the publication writer, recheck late work and current marks, remove identity/object/young/remembered entries, then release the writer. Close storage-borrow admission, then incrementally clear/destroy when existing leases end. Logical credits stay charged until physical destruction completes. |

Acquire/release epoch loads/stores and AcqRel mark updates publish work within these
critical sections. A metadata Arc alone does not establish a storage lease.
Identity reconstruction and weak upgrade pass their existing collector-specific
publication token into root registration. Validation and insertion remain in one
uninterrupted admission, without recursively admitting the same operation. The
token is checked against the target collector. This reduces nested reader/count
operations; it does not permit closure while the outer publisher is stalled.
The metadata owner allocation happens before registration admission. A validated
typed/weak root keeps its exact registration in a stack guard after admission ends,
then allocates the shared owning record. Cancellation removes that registration;
an allocator delay retains its rooted graph while unrelated cycles can complete.
Initial-root preparation and the reserved bytes stay owned across registration
retries. Both policies validate vacancy and install identity, descriptor and
nursery membership before releasing the shard. The descriptor remains private
while the constructor prepares its marks and traversal. An exact lookup or scan
can atomically record that it passed the private descriptor and move on.

Both policies publish preceding index writes to later phase starts with a CAS
on the packed clock. This orders the lock-free nursery scan as well. An idle
birth can activate with a pending-to-live CAS. An active birth with no queue
work can initialize its private marks and old-only cohort tag, validate the
phase revision and attempt the same CAS. A scanner that has passed a pending
descriptor prevents activation even if it raced after validation. The
publisher erases its speculative marks and cohort tag, rearms the descriptor
and retries against the current clock. Each rearm precedes the next clock
RMW, so a scanner starting a later cycle sees the rearm or a later visibility
state.

A graph born during strong tracing pins the active epoch identities and links a
provisional ticket in each strong domain. It then stamps the phase clock, checks
its pinned identities and attempts activation. A worker visiting a private
ticket prevents that attempt from becoming live. A worker visiting a live ticket
must still own the matching attempt before tracing. A permanent bit distinguishes
the first attempt; later attempts store cancellation before rearming. Epoch
identity also permits a leaf to validate after revision saturation or rebase.
A physical object-shard writer can still delay root enumeration and sweep.
[Forced schedules](src/publication_tests.rs) and [phase models](tests/admission_model.rs).

Taking a pending reference transfers its existing queue root into a stack guard.
Queue waiting, validated-root preparation and shared-owner allocation no longer
share an outer admission. The existing queue root keeps the owner graph alive
until the caller's new root is installed; errors and unwinding remove that root
through the same guard. Typed acquisition still validates/registers under its
own admission. Queue production and root-index operations retain their existing
publication protection.
Queue entries carry the owner's non-reused allocation identity and the exact
tree-root generation. Transfer resolves and validates that identity. Revoking the
queue root cannot make later transfer deliver a replacement allocation, and stale
cleanup cannot erase a newer tree entry after public numeric-ID reuse. Ordinary
owning tree roots and temporary construction roots use the same generation check;
generation exhaustion fails before reuse. Public numeric IDs remain a compatibility
API and are invalid after removal.
Root-guard destruction uses the matching registration's deletion barrier instead
of issuing another barrier from its inner edge. Both allocation identity and
collector must match; replacement edges keep their own barrier. Cloned guards
retain that same registration, and explicit compatibility revocation already
publishes deletion. The registration remains owned until after the inner edge
has been destroyed.
Trace and relocation callbacks explicitly acquire one before accessing descriptor
storage; declared leaves complete their mark claims without accessing storage.
Registry retirement closes admission permanently. The existing graph
publication gate and generation interlocks still supply logical reachability and
reference semantics.

Each domain's existing 64-bit mark word now uses two low bits for strong,
final-retained, and phantom-retained reachability. The strong boundary freezes
weak/soft eligibility; the post-final boundary freezes phantom eligibility.
Later retention keeps storage and descendants alive without changing those earlier
decisions. A successful mark CAS publishes the classification. Cross-generation
tracing, cross-snapshot roots and promotion preserve the source's retention strength,
clamped to the receiving domain's current boundary. A stronger discovery before
closure queues a fresh traversal of the entire object, including already scanned
children. Weaker concurrent traversals cannot downgrade it. Publication and
in-flight exclusion drain all earlier work before advancing a boundary.
New allocations remain black. A new conditional slot created after the strong
boundary records that domain's epoch and defers decisions to its next cycle;
cloning preserves the original slot's decision. This permits new weak references
to resurrected objects without reviving previously cleared references.

Epochs advance by four. Before exhausting the counter, an inactive domain resets
object marks and conditional-slot birth stamps in bounded scan units, keeping its
collection request pending. Other-domain shading checks active state; constructors
initialize unmarked metadata and zero birth stamps during reset. Epoch zero is
reserved and no old stamp can alias the restarted cycle. These rules still rely
on the existing generation and publication interlocks.

## Descriptor storage leases

`StorageState` packs an irreversible retired bit and a bounded borrower count in
one atomic word. A successful AcqRel compare/exchange admits a borrower only while
live. Retirement's AcqRel fetch-or closes admission. The last borrower's AcqRel
decrement publishes readiness; destruction claims the retired/zero state with a
single AcqRel compare/exchange. A lease owns its metadata Arc, which directly owns
erased initialized descriptor storage and its typed destructor. Metadata destruction is a final
fallback for storage retained past collector teardown.

An already admitted lease delays clearing, finalization, and physical destruction
of that allocation; it does not retain outgoing graph edges, create a root, or
make a weak upgrade legal. Borrow admission after logical retirement fails.
Ordinary rooted handle and payload-guard lifetime contracts are unchanged.

Sweep transfers exclusive ownership of an unregistered, unleased descriptor out
of its cycle before executing finalization, clearing or destruction. Neither the
cycle mutex, coordinator nor publication admission spans those callbacks, under
either publication/phase policy. An executing descriptor stays charged as retired
storage; another collector can finish that full cycle and start subsequent cycles.
After returning, a `Continue` resumes in the original sweep only if its epoch and
phase still match and its continuation slot is empty. Otherwise it enters the
retirement queue. `Busy` always enters that queue with its offset and finalized
state intact. The ready queue rotates continuations and preserves exactly-once
destruction. Normal callbacks continue consuming the remaining step budget;
reacquiring a contended coordinator or cursor yields instead of waiting.

A callback that blocks still holds its executing thread; callbacks are not
preemptible. In particular, threadless Wasm requires nonblocking, bounded callback
implementations. This change isolates reclamation callbacks; tracing and relocation
still use their separate graph-protection reservations.
Leased retirees enter an identity-keyed registry; they keep their full logical
charge. The last lease publishes its identity to a ready queue and wakes workers.
Registration rechecks readiness under the registry lock to cover release before
insertion; a queued bit suppresses duplicate notifications. Reclamation takes one
bounded clearing unit on alternating turns with active cycles. A blocked lease
never holds a collection completion ticket open or prevents unrelated garbage
from being reclaimed. Released leases need no new collection request. `stop()`
drains ready clearing work but cannot revoke outstanding leases.

Diagnostics expose delayed `retired_bytes`, `retired_objects`, and `retired_ready`;
these are independent samples. Retired bytes remain included in the finite managed
limit until destruction. This bounds their logical retention without promising
allocation success while storage is borrowed. The internal lease API is not a
replacement for native rooting or an external hazard-pointer API.

`storage_model.rs` compiles the production atomic state machine with Loom atomics.
Its two models cover admission versus retirement, destruction ownership, visibility
of borrowed writes, and exactly one readiness notification from multiple readers.
Forced collector tests cover stalled borrowers, weak invalidation, unrelated
reclamation, incremental clearing, worker wakeup, and collector teardown. These
checks do not establish a gate-free graph or reference-processing protocol.

## Execution-context lifetime

`GarbageCollector::register_context()` attaches an empty context with a checked,
non-reused registry ID and a weak collector reference. `ExecutionContext::with`
installs it for a synchronous call. `scope` installs it for each poll and restores
the previous context before returning `Pending` or `Ready`, and on unwinding.
The VM applies this per-poll scope to Java method execution when context policies
are enabled. Default policies do not create VM execution contexts.

An OS thread is not an owner. A suspended future may move to another thread;
its owning registrations retain their original context slots. Nested scopes,
including scopes for other collectors, restore their predecessors. Unscoped
operations use the configured shared root registry. Multiple callers may share a
context; its slot shards and record mutex serialize conflicting accesses.

Dropping a context handle detaches that owner. Escaped owning roots retain the
context. A nonempty buffer is also owned by the collector's ready registry, so
cancellation transfers its records to collection without requiring another poll.
Only the last Arc, after all roots and records are gone, unregisters the context.
Empty root-page caches are released then; cumulative page creation/release counts
remain observable after detach. `stop()` also trims empty pages in live contexts.
A context alone does not keep a collector alive; owning GC handles retain that
existing responsibility.

Root enumeration reads a separately published immutable registry view. Attachment
publishes its empty context before returning an `ExecutionContext`; roots created
afterward retain their normal insertion barriers. Removal publishes a new view
under the same registry writer. Readers never take that mutex. An expired weak
entry in an older view is skipped; a context inserted behind a cursor still relies
on its roots' publication barriers. Each update clones the registry map, so context
attachment/removal adds allocation and work proportional to the context count.
Reader guards may retain older maps; this cold-path memory/cost is unqualified.

Each generation cursor retains one current context throughout its root traversal,
avoiding registry lookup and weak upgrade for every slot. Its initialization
cleanup and final owner release are returned as work that executes only after
publication admission, cycle and coordinator guards have been released. Those
context boundaries end the current collector step. The cursor can retain a detached
context and its bounded empty-page cache until traversal advances; diagnostics
continue counting that retained owner. This does not retain removed GC roots.

Cache cleanup tries the context mutex. A busy cache keeps its unused ranges and
credits charged, but does not prevent root enumeration or phase completion. A
successful cleanup detaches the unused tails and credits under the mutex, releases
it, returns the ranges, and then refunds the logical credit. Allocation hit/refill
counters remain with the live cache. Other workers can complete collections while
root-scan context cleanup is stalled. Blocking cleanup still occupies its executing
thread. Context destruction from other paths, required-record application, queue
publication and opaque callbacks retain separate waits/reservations.

No scope guard is held across suspension. This is not yet a native-borrow
quiescence protocol: payload guards retain their existing lock/lifetime contract,
and must not prevent cooperative collection from making progress.

## Record publication and drain

`barrier_buffer_size(0)` preserves immediate barriers. Values 1 through 64 enable
fixed-size context buffers during strong tracing; larger configuration values
clamp to 64. Each record contains an address, dirty-owner flag, and the active
young/full epoch tags. Owner mutation versions advance immediately. Array card
bits remain protected by the managed write guard; buffering delays the collector's
owner notification, not the card update itself.

The record lifecycle is:

1. Under publication admission, try the local buffer mutex and coalesce only an
   identical record among the two most recent (same address, both epoch tags,
   and dirty flag). Checking at most two covers alternating owner/edge records
   without scanning the entire buffer. This remains constant-time when full. Its existing pending reservation protects the
   merged work. A popped record still being applied cannot coalesce with a new
   producer's record. Two separate completed-edge cache entries can also satisfy
   an identical edge record after its mark publication completes, as below.
   Before inserting into an empty buffer, also try the ready-registry mutex.
   Busy acquisition leaves the buffer, ownership and pending count unchanged;
   the producer publishes its new record directly. Otherwise, increment the shared
   pending count with AcqRel ordering **before** storing the record. An empty to
   nonempty transition installs a strong context Arc in the ready registry.
2. Collector drains peek at the last record without transferring its reservation,
   then try the required remembered and object shards. They hold the prepared
   object mapping through a matching pop, revalidating the address, both epochs
   and dirty flag. A changed top record cancels preparation without removing work.
   An identical replacement may be claimed using its own pending reservation.
   Claims try the buffer and, for the last record, the ready registry before
   changing either. A busy lock leaves
   the record, its ready owner and pending reservation intact. Removing the last
   buffered record removes the ready entry, but the flusher's Arc and pending
   reservation remain alive. Mutator overflow uses this same prepared claim;
   explicit `flush()` retains its synchronous blocking pop.
3. Publish matching-epoch mark work, or insert the remembered owner. A delayed
   dirty notification also shades that owner for an active young snapshot,
   including an old owner whose remembered enumeration has already passed.
   The remembered shard is acquired before observing young inactivity and held
   through insertion. If a young cycle starts after that observation, its fresh
   remembered cursor cannot pass the owner before insertion; direct shading is
   unnecessary. Pending excludes closure, but does not exclude a generation start.
4. Before releasing the reservation, an edge record may enter the two-entry
   completed cache after verifying Strong retention in each tagged domain. An
   allocation already old is also exempt from young tracing. Age never decreases;
   reuse of its address during that same young epoch publishes the replacement
   Strong before registration admission ends. Dirty-owner records never enter
   this cache. Epoch equality alone is insufficient after numeric epoch rollover,
   so the collector permanently disables this optional cache before resetting
   any marks. New epochs otherwise miss old cache entries naturally. Metadata and
   cache lookups for this optional certificate use try-locks: contention skips
   certification after required publication has already succeeded.
5. Decrement the pending count with AcqRel ordering **after** required publication
   and any successful optional certification. The resulting mark task has its own
   queue/in-flight accounting. Release prepared metadata and the context owner
   afterward, outside index and collector coordination guards.

Completed records do not own roots, payloads or pending reservations. Repeating an
already satisfied edge therefore cannot refill the buffer indefinitely. This
cache does not eliminate every source of new work or guarantee closure under an
arbitrary mutation stream.

Producers reserve records under a publication reader. Explicit and ordinary
collector drains need no publication admission: the existing pending reservation
excludes phase closure and sweep until mark publication finishes. Collector drains
run before acquiring phase, cycle, coordinator or claim guards. A busy ready
registry is only an unavailable scheduling hint, never proof of no pending work.
Each context claim attempt consumes a work unit, including a busy buffer or last
record's ready-registry claim, busy required index or changed preparation hint.
Round-robin selection moves on to other contexts;
the configured work/time budget limits attempts. Drains alternate with ordinary
tracing/phase steps so continually replenished buffers do not take every turn.
Terminal paths yield on pending records instead of draining under their guards.
Known mark work can still be traced while required records remain pending.
The caller's existing pending-count hint skips drain scheduling when zero. A later
publication is handled by subsequent attempts; terminal phases still validate
pending with their publication/claim interlocks. The hint never reports a
collection complete.

After required application, the pending acknowledgement precedes releasing the
context owner. This can invoke final context cleanup, which executes outside the
collector's coordination. Such cleanup can stall its executing thread while other
collectors complete cycles. Collector index preparation now yields without popping
the record, allowing its worker to attempt other work, including ready retirement.
Required publication can still stall in host allocation or after a claim; the
record's reservation excludes closure until marks or remembered membership have
been published. This is not a helpable reservation protocol.
Prepared index guards span the matching claim, including last-ready-entry removal;
remembered insertion also retains its shard. Host allocation/deallocation in those
operations is not bounded by the try-lock protocol.

Collector `barrier_drain` timing starts after a successful peek and includes index
preparation, failed preparation/matching attempts and successful application.
`barrier_flushed` still counts only acknowledged records. Keeping the preparation
helper out of line prevents its code and temporary storage from being inlined into
every ordinary collector step; it does not change the synchronization protocol.

A caller encountering overflow attempts at most 64 prepared record drains, stopping
on busy preparation, then retries insertion once. A full or busy buffer causes direct
publication of the new record. Existing records keep their ownership and pending
reservations until successfully applied; no record is discarded or retried without
bound. A busy buffer/ready registry or another record's index cannot make this
producer wait inside admission. Direct fallback still uses the required target's
metadata/remembered locks, and successful publication can allocate queue/map nodes.
Explicit `flush()` keeps its synchronous contract and blocking pops, with the same
64-record limit; concurrent producers can leave additional work behind.

Before marking closes, reference decisions proceed, sweep retires an allocation,
or compaction retires the snapshot, the collector checks pending reservations
under the selected publication policy's existing protection, in addition to work
accounting: the legacy writer or counted admission/phase observation and commit.
A fast check outside that protection only selects work; it never authorizes retirement.
Empty buffers alone are insufficient because a flusher may have popped a record
without publishing it yet.

Once either generation enters reference processing, mutators use immediate
barriers until both domains leave terminal processing. This prevents a stream of
already-black deletions from continually filling buffers ahead of sweep. Idle
remembered notifications are also immediate, so the next young snapshot cannot
miss a sleeping context's earlier mutations. Epoch mismatch cannot legitimately
strand a required record: pending reservations prevent either snapshot from
retiring while it remains unpublished.

Record storage is bounded by 64 records per registered context. Diagnostics report
current context structure/buffer bytes (excluding map/Arc allocator overhead),
pending records, buffered/flushed counts, and overflow events. Root-page bytes are
reported separately. These are independently sampled observations, not a snapshot
suitable for deciding termination.

## Remembered-set pruning without the publication writer

Mark queues retain exact allocation metadata through dequeue, continuations and
stealing. A callback acquires its storage lease from that metadata, never from a
new lookup of the queued raw address. Retirement therefore invalidates stale work
without allowing it to borrow a replacement allocation at the same address.
Completion passes the original identity into pruning, which validates it together
with the mutation version. Metadata `Arc`s alone do not postpone storage reclamation.

Pruning keeps the object's metadata shard locked from the mutation-version check
through remembered-set removal. Mutators advance that version under the same
shard, then publish the remembered notification. A mutation preceding the check
invalidates the scan; a mutation following the removal reinserts the owner. A
delayed buffered notification can only insert afterward. No collector callback
runs under these two locks; the order is object shard, then remembered set.
This replaces the publication writer for pruning only. Default single-shard
metadata and the remembered registry still have shared mutexes.

A forced regression holds a publication reader open and requires pruning to
complete. A fourth reduced model covers mutation/pruning ordering, including
delayed notifications. Allocation, root publication, reference decisions, cycle
boundaries, and sweep use the selected publication policy described below.

## Lock ordering and verification scope

Producer publication admission precedes context-buffer locking. Buffer operations may take the ready
registry lock; obtaining a ready context releases that registry lock before
locking its buffer. The weak context registry is released before invoking any
context operation or dropping its last upgraded Arc. Final context destruction
never reacquires publication. Collector root-scan cleanup and buffered-drain final
owner release occur outside their phase/cycle/coordinator/admission guards.
Required collector indexes are prepared between the peek and matching pop. Index
guards are released before mark publication; explicit mutator flush retains its
lookup after the blocking pop.

Tests force an explicit flusher to stop after popping a record and before metadata lookup:
the record stays charged, the publication writer remains obtainable, and an
actual collector step cannot close the phase. Pending is checked before queue
emptiness; observing the last release orders mark publication before that check.
Collector-drain tests hold required object/remembered shards: the same worker
yields, keeps pending=1, and reclaims a separately retired object after its storage
lease is released, before unlocking the shard. A forced young start between
inactivity observation and dirty insertion cannot pass the held remembered shard.
Portable context tests reject changed top records and account for identical
replacement reservations separately. Other tests
cover both active epochs, mutation after remembered enumeration, tiny buffers,
detach, escaped roots, cross-thread polling/cancellation, nested scopes, and
unwinding. The native allocation pressure matrix includes context roots and
four-record buffers with both payload backends, both marking policies, prediction
on/off, four mutators, and four workers.

[publication_model.rs](tests/publication_model.rs) uses
[Loom](https://docs.rs/loom/0.7.2/loom/) for reduced handoff models: cancellation
versus closure, popped records versus mark publication, and mark claims versus
closure. It does not model arbitrary Trace implementations, the complete VM,
all reference stages, or a gate-free collector. Miri and ThreadSanitizer provide
additional executed-path checks; address-based APIs still use exposed provenance,
so Miri's pointer coverage is not equivalent to strict-provenance verification.

## Qualification still required

The counted replacement below now covers allocation, roots, weak decisions,
marking boundaries, and sweep. Neither independent marking nor storage leases
alone supplies that protocol. Keep both publication policies selectable until
resource-limited throughput, latency, memory, and endurance qualification passes.
Reduced models do not include the entire runtime or arbitrary callbacks, and the
existing whole-generation terminal interlocks remain necessary.

## Logical recovery observations

An allocation request registers its size-indexed observation before requesting or
joining a full cycle. Its observation outlives waker registration and notification,
so executor migration or a delayed first poll cannot lose evidence of reclaimed
capacity. Refunds use the previous value returned by the logical-budget atomic
subtraction to compute headroom at that subtraction, then mark every eligible
observation. A later competitor's reservation cannot erase this evidence. The
observation mutex protects registration/removal; recording runs no user callbacks.
Collection completion follows its refunds, and acquire/release ticket publication
orders the refund observations before a completed-cycle decision.

An observation is advisory and owns no allocation credit. Publication still needs
a successful atomic reservation. Only adequate headroom preserves the recovery
allowance; smaller refunds do not. Cancellation removes registrations, and observed
entries leave the size index immediately. Ordinary notification separately wakes
requests outside collector locks. No observation is installed on a successful
allocation, and refunds with no pending observations avoid the observer mutex.

## Blocking-operation audit of the current implementation

Conditional edges carry the target's original nonreused ID. Registration,
construction retention, logical-clearing checks and upgrade validate that ID
against the address index; constructing a weak handle from a stale edge cannot
bind it to a replacement at the reused address. Weak handles own no metadata.
Internal metadata ownership uses a strong-only reference counter, while storage
leases and logical reachability remain separate. Admission and the slot decision
still span weak eligibility checks and root registration.

This audit covers both publication policies, independent phases, sharded regions
and old-only passes. Admission means a recursive reader under the default policy
or a counted entry under `concurrent_publication(true)`. A work-unit limit bounds visits
or copied bytes; it does not bound host scheduling, a contended lock, a user hook,
or system allocation/deallocation. No row below establishes a hard time limit.

| Path | Remaining synchronization or synchronous work | Scope and existing mitigation |
| --- | --- | --- |
| Direct payload construction | Region registry and extent locks, system allocation, element initializer | Region backing allocation runs outside the registry lock. Async construction yields every 64 KiB or oversized element and awaits recovery; custom initializers may still block. |
| Descriptor publication | Local cache and descriptor arena; constructor hooks; prepared object/identity/young shards; admission | Failed index preparation releases all guards/admission before waiting and retrying. Construction roots and conditional-owner counts protect the unfinished graph. Metadata owns the charge before visibility and refunds it only after payload destruction. Index insertion can allocate while holding the object shard. An indexed, private descriptor can be skipped while its constructor holds no admission. |
| Owning root acquisition/release | Slot preparation before admission; admission; tree shard; context registration; shading and mark queue | Slot deletion claims a generation atomically, publishes its barrier, then releases occupancy. It pins the page through cleanup after admission. A busy writer receives deferred cleanup; metadata work remains potentially proportional to page count. Tree/map nodes may allocate under locks. Compatibility removal uses generation-validated slot snapshots, but retains outer admission, snapshot allocation and context/tree registry waits. |
| Handle reconstruction | Numeric identity lookup before admission, then admitted object lookup and root installation | Expected identity, type and logical eligibility are revalidated under admission before exposing a payload pointer. Reconstruction is synchronous. |
| Mutation/root barriers | Admission; object shard; context buffer/ready registry; remembered index; mark queue | At most 64 buffered records per context. Overflow flushes bounded batches then falls back to immediate publication. A popped record retains its pending reservation until queue publication. Drains require no admission. |
| Weak creation/clone/upgrade | Admission; weak-slot decision lock; metadata/reference registry; root installation | Slot allocation precedes admission. A failed slot try-lock releases admission before waiting, then retries all decisions under fresh admission. Metadata/reference indexes and root installation still synchronize. Storage leases never authorize weak resurrection. Explicit clearing is atomic. Old-only passes retain conditional referents until precise full processing. |
| Payload reads/writes and native slices | Per-payload read/write lock; write-guard drop runs ownership conversion and barriers | Collector tracing/copying tries these locks and defers on contention. Mutators may wait for a copy chunk or another mutator. Mapped guards retain the parent lock. Guarded cloning and unrestricted bulk operations can span an entire payload. |
| Allocation exhaustion | Waiter/observation indexes and request wake mutex; native task suspension or a cooperative collector step | Async recovery never holds managed guards across an await. A cooperative poll performs at most one configured step. Persistent competition, live-set exhaustion and physical fragmentation are separate from ordinary allocation latency. |
| Legacy synchronous allocation/collection | Full-cycle loop on the calling thread, plus collector synchronization/callbacks | Public synchronous APIs retain their completion semantics. Synchronous JIT helpers bridge async allocation while the invoking compiled frame waits. Experimental resumable allocations return their native frame before awaiting. Sentinel/context setup and other helpers retain synchronous work. |
| Reclamation and phase closure | Coordinator; per-domain cycle/claim locks; queues/shards; default publication writer or counted phase CAS | Independent phases release the coordinator for bounded scans and callbacks. Counted closure checks pending/queued/in-flight work and an unchanged admission revision. Sweep retires under the object shard; leases defer charged storage. Counted paths bypass the legacy writer. |
| Generation conflicts | Per-object scan locks; coordinated strong/reference boundaries | Old-only work permits unrelated young cycles; conflicting callback and remembered scans serialize. With old-only scheduling enabled, precise full tracing drains active young work and defers young starts through reference closure. Mutators continue publishing. |
| Diagnostics and shutdown | Sequential registry/shard/context locks, worker joins, callback destruction | Diagnostic snapshots are independent samples. Shutdown preserves escaped rooted handles and outstanding storage leases. Neither operation is a bounded ordinary mutator path. |

Payload locks also serialize application operations with one another, so every
payload wait cannot be attributed to GC. Diagnostics distinguish publication and
payload waits, inline/awaited recovery, worker/caller steps and deferred retirement;
individual registry-lock and custom-callback durations are not separately measured.
All production legacy-lock accesses are confined to `allocation_guard` and
`try_publication_write`, both bypassed when the counted clock is present. Forced tests
hold the writer throughout counted allocation, rooting, weak operations, full and
old-only collection, and teardown. The clock never blocks entry on another
publisher or asks contexts to park. A stalled synchronous admission can delay
closure while unrelated publishers proceed; sustained publication can force
optimistic phase commits to retry.

Suspended futures retain roots and buffers without admission. Nonempty buffers
retain their contexts through the ready registry, so draining needs no task
acknowledgement. Pending counts acknowledge individual published records. Root
publication shades active snapshots even after enumeration passes a new context.
Native guards retain their rooted graphs; callback leases protect only borrowed
storage. Logical/retired bytes, context storage, cached credits and arena backing
remain observable and charged. A stalled participant can eventually exhaust a
finite heap; allocation progress beyond that limit is not guaranteed.

Counted publication removes the heap-wide writer, but individual GC stalls remain.
Native and threadless cooperative execution have different progress contracts.

## Sharded generation indexes

`metadata_shards` also partitions young membership and remembered owners by the
same descriptor-address hash as object metadata. Membership lookup, insertion,
promotion and removal touch only the address's shard. Scanning releases its set
shard before consulting object metadata or invoking callbacks; it charges an empty
shard as work. Young cross-snapshot, remembered-owner, young sweep and compaction
passes have distinct/reset cursors so finishing one traversal cannot skip another.

Object and membership scans try one shard per unit. Each cursor keeps all shard
positions and an exhaustion mask, so a held shard rotates to another unfinished
shard without losing its own position. A busy shard never counts as exhausted.
After selecting an entry, a failed downstream lock restores that shard's previous
address and rotates; retry performs a fresh lookup, including after address reuse.
Successful entries are not replayed. Tree-root enumeration similarly commits its
root ID only after a successful object lookup. Slot/context root cursors rewind
the selected slot on a busy object lookup; the page stays pinned, while retry reads
its current address so removal/reuse cannot leave a saved stale address. Other
slots are not replayed, and no extra cursor field or allocation is needed.
Cross-domain root publication still
holds admission from the successful root read through shading. Reset, cross-root,
remembered, old-handoff and compaction enumeration yield on busy metadata scans.
Root-scan context cleanup follows the detached-work protocol above. Context writers,
buffer operations, queue publication and reference registry operations still have waits.

Promotion/removal retain the object shard and configured publication protocol.
Promotion tries young and remembered membership before changing age or snapshot
marks. Retirement tries identity, young and remembered shards before removing any
entry. Failure releases all guards and retries the uncommitted entry; an identity
cannot disappear while its retirement is deferred. Queue publication and host
allocation inside successful operations remain potentially blocking. Reclaiming
other shards during a stalled admitted publisher is possible within an already
active sweep, but closing the cycle and starting later cycles still require the
existing admission protocol.

Remembered
pruning retains the object shard across mutation-version validation and set
removal; a new mutation must precede that validation or reinsert afterward.
Buffered notification retains its pending reservation until publication/shading.
New allocations behind a scan cursor are shaded while marking or made black after
strong closure, as before. The same publication regression now runs after both
young and full sweeps, with one/sixteen shards and both marking policies. A forced
native test holds unrelated young/remembered shards while a real mutation barrier
completes. Index sharding is independent of phase distribution and counted publication.

## Evacuation roles and backing lifetime

Cost-aware selection runs after full sweep under the existing coordinated phase
machine. Relocation claims are enabled only after selection completes. Region
roles and the incrementally built candidate index are protected by the registry mutex; each new epoch
uses checked arithmetic. Source and destination roles are disjoint during an epoch,
including when ordinary allocation refills a candidate or a backing reservation
finishes after selection starts. Selection is advisory and cannot retire a live
object or change its identity. Both generations' claim/flight interlocks remain.

Each payload's exclusive guard covers copy commit and the optional role check for
an older partial copy. An incompatible raw destination can be discarded because
only the source owns initialized elements. Empty selected sources are removed from
the registry under its mutex, but the registry's owning Arc is dropped afterward.
The final Region destructor deallocates backing before refunding committed state
and shared credits. A cached zero-length tail can retain that owner and therefore
keeps the backing charged. System deallocation never holds the registry mutex.

Pin history uses a descriptor storage lease across callback and region lookup;
the payload try-lock remains the access guard. Cached IDs can be stale and are
used only for scheduling feedback, never pointer reconstruction or reclamation.
Retry-limit cancellation may itself encounter a guard; its partial destination
remains charged until cancellation, mutation, or destruction can release it.
Neither selection nor the reserve replaces graph publication or weak-reference
ordering. The blocking-operation audit above still applies.

Raw relocation destinations and transferred sources are distinguished from
ordinary returned ranges in recovery accounting. Pure live-data movement cannot
reset physical retry exhaustion. Empty oversized regions retire eagerly, preventing
small subsequent copies from stranding a large released backing allocation.

## Physical allocation offers and guard samples

A direct payload waiter carries a layout and collection ticket, not an object or
range owner. Notification takes the waiter registry, samples the region registry
with try-locks, and removes eligible entries into a local ready list. It releases
all locks before waking or dropping user wakers. Probes are bounded and rotate
among waiters; normal completion wakes every waiter for the completed ticket.
Future cancellation removes both the ticket entry and physical index. A lost offer
re-registers the same owned waiter identity. The real allocator alone grants range
ownership, so samples cannot oversubscribe backing or bypass source exclusion.

Retry accounting retains whether ordinary reclamation was observed within the
current cycle and increments exhaustion only when the completed ticket advances.
Pure copy movement does not reset it. Cooperative waits must advance a step even
when an advisory hint is already true, covering allocator failure within available
arena credits. Direct initialization still runs only after storage reservation.

Guard timing owns shared diagnostic counters rather than a collector or graph
root. The native lock field drops before the timing field, including projection
unwinding. Mapped guards own the parent guard and therefore exactly one sample.
The active count is observational and never participates in marking or retirement.


## Construction ownership outside publication admission

A synchronous constructor installs a borrowed stack scope in thread-local storage.
The Rust borrow keeps its address stable, and the scope cannot migrate or outlive
its stack owner. It restores the previous scope before cleanup, including unwind.
Nested scopes search by collector identity, so another heap cannot adopt a root.
No scope spans an async suspension.

When an owning handle's last Arc drops inside a hook, its existing tree/slot/context
registration transfers to this scope. The registration remains visible throughout
construction; no new root node or metadata lookup is needed on this path. Explicit
root-removal APIs retain a fresh compatibility root under their existing reader
before releasing the old registration. These fresh IDs avoid duplicate cleanup
of an old slot. The scope releases/transfers registrations only after publication,
or while unwinding a failed construction. Normal deletion barriers run on final
removal. Registration, not an OS thread's continued execution, retains the graph.

Ordinary descriptor allocation, strong-edge attachment, metadata sizing/owner
hooks and relocation history lookup no longer retain publication admission. Root installation and
metadata registration retain their short readers. A root for an unpublished
descriptor is harmless: enumeration consults registered metadata, and registration
itself shades or blackens the object for the then-current phase. Construction
roots keep initial edges valid across any intervening collection.

A stalled hook retains the roots it released and their reachable graphs, including
conservative retention of unrelated roots dropped by that hook. Those objects
remain charged to the managed heap limit; their existing root pages/context owners
remain visible. Unrelated garbage can be reclaimed and cycles can complete. This
does not bound arbitrary user callbacks or guarantee an allocation with an
exhausted live set. Conditional-reference owner assignment follows the same rule:
under short admission and the slot decision lock, it validates the referent's
allocation identity, frozen weak eligibility and logical retirement state. An
eligible referent receives a temporary root and active-domain shading before the
slot's unpublished-owner count becomes visible. Repeated assignment in one scope
shares that root/count. An already cleared or logically dead referent receives no
root; attachment cannot reverse a closed decision.

Reference enumeration skips a pending slot before trying its decision lock and
rechecks the count after acquiring that lock. The first check permits progress
while owner binding waits; the second handles a construction starting between
the first check and lock acquisition. Skipping never removes the slot's index. Its rooted
graph survives intervening collections, while unrelated garbage remains reclaimable.
Successful metadata publication releases admission before binding the owner's
exact identity under the slot lock. The owner is already in the object index, so
concurrent owner assignment can resolve its identity. Binding completes before scope exit
decrements its pending count. Cleanup then removes/transfers construction roots with
the ordinary deletion barrier. A later cycle processes the slot and delivers any
eligible notification; failed publication leaves the owner unbound. Nested or
reentrant full collection during attachment is supported. Conservative retention
can postpone reference processing until a subsequent cycle; it does not weaken an
already closed weak decision.

Forced native tests cover a stalled strong-edge attachment with every root
policy and both constructor APIs; nested/reentrant collection and unwind are also
exercised under Miri and on portable targets.

Ordinary weak upgrade, clone and owner assignment try the slot lock after entering
admission. On contention they release admission, wait for and release the slot,
then retry in the same order; they never hold the slot while acquiring the legacy
publication reader. Birth and logical-eligibility decisions are made again under
the successful admission. A held old slot with a waiting operation therefore does
not exclude unrelated young collection. A held slot without pending construction
can still defer reference processing for the generation that contains it, and a
thread paused after successful admission can still defer phase closure.

## Experimental counted publication

`concurrent_publication(true)` replaces every runtime publication reader/writer
acquisition with the following protocol. It does not change payload locking,
metadata shards, native scheduling, or the generation interlocks.

`publication_state.rs` packs both domains' active, marking, and reference-stage
flags, a 20-bit active count, and a 34-bit revision into one 64-bit atomic word.
Ordinary publication entry increments the count and revision in a **single CAS**
before reading phase state or publishing anything. Pure edge barriers that require
admission increment only the count, then advance the revision before exit if they publish work or
change retention. Buffer overflow conservatively counts as publication because
its drain may publish work. A completed no-op barrier can leave an earlier closure
observation valid; an admitted barrier still excludes every phase transition.
Exit subtracts one count unit. These operations use
SeqCst ordering. The count admits at most 1,048,575 simultaneous nested operations;
exhaustion panics before admission rather than silently wrapping.
Every admission keeps both phase states stable until exit; another publisher
can enter without waiting for it. No collector admission bit blocks mutators.

A pure edge barrier first attempts a read-only certificate. It snapshots both
phase flags and the revision, checks exact-epoch Strong retention (or exemption
from young tracing), then verifies the same phase/revision word. An idle snapshot
needs no object lookup. A completed-context cache hit uses a separate cache
try-lock; a miss or busy cache falls back to a metadata reference and releases
the object shard before checking. The cache never holds the required-record buffer.
Neither path accesses payloads, registers roots or holds publication admission.
Retention only strengthens within an epoch. Every phase transition advances the
revision, so a phase cycle cannot masquerade as an unchanged snapshot. Before
numeric revision reuse, the clock permanently disables read-only certification;
ordinary counted publication continues. Failed certificates fall back to admitted
publication and repeat the current-phase work checks. A phase change after a
successful check has the same ordering as a change after an admitted barrier exits.

Certified records can fill the context cache without a pending
reservation: their retention has already been established. Numeric object-epoch
rollover separately disables that cache before resetting marks. Existing work
publication still owns its original admission or pending/in-flight reservation;
certification never substitutes metadata ownership for graph reachability.

Two recent entries precede four sets of four exact records. After 64 consecutive
extended-cache misses, the next 1,024 non-recent lookups bypass the extension;
bounded probing then allows a changed working set to recover. This is only a
lookup heuristic. Eviction, bypass and lock contention always require the normal
metadata check. All cache access and the optional post-drain metadata certification use try-lock,
including insertion after draining
a required record, so a paused cache holder cannot delay its pending acknowledgement.
Dirty-owner records never use these certificates. Fixed context storage includes
the cache; it owns no roots, payload leases or pending reservations.

Direct counted edge barriers that need admission wait for their object shard before entering admission
and release admission before unlocking the shard. The short admitted portion
checks marks and publishes queue work without another index lookup or user
callback. This prevents hot-shard waiters from keeping the active count nonzero
as a convoy. Idle barriers confirm inactivity under admission without indexing the
object. Already Strong targets need no retention update. Legacy publication keeps
its original reader-before-shard ordering. A publisher preempted after admission,
a blocked mark-queue publication or a buffered producer blocked inside admission
can still delay phase closure; this is not a fully helpable publication protocol.

Under the single phase coordinator, `observe` reads the revision and verifies zero
active publishers **before** checking pending records, in-flight reservations, and
queues. `transition` CASes that exact zero-count word to the next flags and advances
the revision, including for a same-flags transition. A failed
observation or CAS defers that collector step. Entry and transition now compete
on one atomic word: an entrant either excludes closure or observes the new phase. The
observation type cannot be copied and is consumed by a transition. Saturation
rebases the revision without changing flags, discards the observation, and requires
all work checks again. Epochs publish while a domain is inactive, before its start
CAS; a failed start preserves the request. Phase flags are authoritative, rather
than mirrors updated after the CAS.

The strong boundary freezes soft/weak eligibility; the post-final boundary freezes
phantom eligibility. Mark words retain those classifications independently of
subsequent final/phantom retention. Reference decisions and upgrades take the
individual slot's mutex after publication admission. Collector decisions use
`try_lock`, deferring the same cursor on contention. Conditional owner assignment
roots eligible referents and marks the slot pending through construction. This is
a per-slot count, not publication admission. Enumeration advances past pending
slots; scope cleanup decrements pending under the slot lock before releasing its
temporary roots. An unpublished owner
cannot be mistaken for a dead Reference or lose its queue notification. Successful
and failed construction are tested separately.

Each conditional slot also records its owner's non-reused identity. Attachment
can assign an address before metadata exists; successful registration binds that
slot to the exact published identity before its construction count is released.
Failed construction leaves an unbound owner. Reference liveness and queue insertion
validate this identity under the object shard. An expired owner or failed attachment
cannot retain a referent or enqueue a replacement allocation at a reused address.

Reference stage 5 publishes logical retirement before sweeping. Numeric handle
reconstruction checks identity, type and logical eligibility together under the
object shard, returning the validated identity. Weak reconstruction carries the
slot's expected identity through that validation. White objects are rejected after
strong closure, except valid retaining upgrades before stage 5. After stage 5,
white graphs cannot be rooted or shaded by dead-edge barriers. New objects publish
black marks before index insertion. Thus sweep can remove white metadata under
its object shard while unrelated publishers continue, then retire storage through
the existing lease protocol. Owning roots remain installed before an allocation's
metadata becomes visible.

Suspended futures retain enumerable roots and hold no admission across await.
Strong and conditional construction hooks retain roots instead of admission. A
preempted short admission can still delay phase closure; it does not stop other
publishers. Retained construction graphs remain charged to the managed heap.
This protocol does not promise bounded allocation latency under exhausted live sets.

Diagnostics report active admissions, successful phase transitions and retries;
these independent samples are not themselves termination tests. A forced native
test holds the legacy writer locked while another thread allocates, roots,
reconstructs identities, creates/clones/upgrades weak references, collects and tears
down. Other forced tests cover pending-owner success/failure, reference ordering,
address reuse, and epoch rollover. Both policies run the context migration,
construction, generation and compaction fixtures and the supported JDK reference
matrix.

`tests/admission_model.rs` compiles the same atomic clock using Loom. Ten models
cover atomic admission, completed-publisher ABA, saturation, stalled admission,
white upgrade versus retirement, preserving the other domain's flags, and ungated
buffer drain reservations, two-domain barriers, final retention, and conditional-owner
publication. The initial observation-order model failed because it
checked admission only after queue inspection; the current protocol checks before
inspection as well. These models still omit the complete two-generation reference
state machine. Sanitizers, forced runtime schedules and stress tests supplement
them; performance and endurance qualification remain separate.

## Young progress during full reclamation

`overlapping_generations(true)` narrows the default terminal-phase interlock only
once the full domain reaches reference stage 5. Full strong tracing still waits
for active young cycles before closing, and strong/reference phases retain their
existing cross-generation reservation checks. Sweeping and relocation can give
new young cycles coordinator turns; at full stage 5 each closing domain checks
its own in-flight reservations. Independent copy queues also yield these turns.

The fourth low-bit mark value records allocation after full tracing/reference
processing. Such an allocation is physically live for full sweeping, but has no
traced-snapshot retention strength. Full sweep does not age or promote it, young
cross-snapshot enumeration skips it, and full relocation excludes it. A newly
created conditional slot records the full birth epoch as usual. Thus a long full
copy phase does not keep every later young allocation alive.

Full relocation is restricted to its traced snapshot. Young cross-snapshot roots
retain those objects and their outgoing graphs, preserving reference strength.
Full completion requires its own queued work and reservations to drain, so this
snapshot cannot disappear while its callback is running. This graph protection is
necessary: descriptor leases alone cannot make a custom callback's child accesses
safe after independent young reclamation. Relocation additionally rejects an
allocation logically retired by either relevant domain before borrowing it. A
new full cycle still cannot close tracing while a young callback's cycle is active.

Reference-owner liveness checks reject full-white owners at stage 5 before applying
the young collector's usual old-object exemption. A dead old Reference cannot
become a queue root or retain a new referent while waiting for physical sweeping.
Only full completion ends the full region-evacuation selection. Young relocation
uses the current region roles and existing destination validity checks; payload
access remains guarded and owned blocks remain charged until their actual release.

Forced schedules hold full clearing or a full copy callback while several young
cycles finish. They cover retained callback graphs after root removal, untraced
births excluded from full copying, new-young reclamation, deferred promotion,
retired Reference owners, persistent-copy fairness, and evacuation ownership.
This policy does not implement old-only tracing or remove the remaining strong
and reference interlocks. Performance/endurance qualification is separate.


## Independent enumeration and reclamation units

`independent_phases(true)` confines the global coordinator to global phase
transitions and their bookkeeping. Root/cross-snapshot/remembered scans retain a
short publication admission over the read-to-shade gap. Their own domain's cycle
mutex excludes its closure; admission excludes the other domain's closure until
cross-publication is visible. Independent reference decisions retain the per-slot
lock plus counted admission (or the legacy writer). Independent retirement likewise
holds admission while updating the exact object and generation indexes. Both
release admission before user clearing/finalization or destruction.

A coordinator claimant never waits on an independently held cycle lock. Work claims
in the coordinated fallback use admission through in-flight reservation when this
policy is enabled; continuations publish before reservations retire. Thus dropping
the coordinator does not introduce an unaccounted dequeue-to-reservation interval.
Phase observations and commits still run under the single coordinator. Tracing and
old-handoff callbacks can still delay their domain's closure or conflicting payload.
Reclamation callbacks transfer descriptor ownership out of the cycle and run without
the coordinator, including when independent phases are disabled. Separate retired ready entries have exclusive popped ownership,
remain charged through callbacks, and can be processed concurrently. Shutdown joins
native workers before cleanup; escaped storage leases preserve their existing
lifetime contract.

Forced tests hold the coordinator while root/reference/sweep units progress, hold a
full clear callback while three young cycles complete, and hold finalization,
clearing and destruction through three subsequent full collections. A clear
continuation returned after those epochs still completes and destroys exactly once.
Deferred callbacks also allow unrelated ready storage and full cycles to finish.
These exercise both publication and phase policies.
Region registries are independently locked under `region_shards`; all backing uses
one shared atomic budget and source selection uses one checked four-region quota.
No path holds two region-registry locks simultaneously. Payload guards, custom
callbacks, system allocation and individual registry contention still permit local
blocking; the experimental counted path does not acquire the legacy publication
writer. Generation strong/reference interlocks follow the selected policy;
old-only passes narrow them as described below.

### Old-only handoff and precise fallback

The selectable old-only pass uses domain one but never shares full completion
semantics. A pending precise full request wins at cycle start; an old pass neither
publishes its ticket nor increments full-collection statistics. Periodic full
requests still occur at the configured young completion interval. An old request
coalesced with a full request is satisfied by the more precise pass.

Publication tags young births with the active old epoch before making the descriptor visible.
The tag is independent of its retention mark: actual old tracing can subsequently
retain a born object's graph without making it part of the initial handoff cursor.
Epoch reset clears the birth tag alongside the domain mark. The handoff takes
publication admission and the object shard while excluding logical retirement,
retains the initial young graph, then releases admission before its callback.
Callback storage has a lease; retention, young marking and remembered scans protect
outgoing graph access. The old-only mark queue accepts only old allocations.
Promotion marks/blackens the old domain before publishing age, then queues the
traversal after the age store, within the same admitted shard operation.

Young closure can ignore old flight reservations only in an old-only pass or the
previously supported post-trace full phase. Each old traced edge is independently
admitted, so the mark-to-cross-publication gap cannot straddle closure. The owning
metadata scan lock still excludes a conflicting young remembered scan, including
when the old callback copied an outgoing edge and a mutator subsequently removed
it. Old copying uses that same lock. Young remembered work rejects an owner already
logically retired by either domain; obtaining its storage lease would otherwise
protect only its descriptor, not its dead outgoing graph.

Every conditional old referent is retained conservatively, including late slot
registration, under publication admission and the slot decision protocol. An old
pass never activates, clears or enqueues a reference. Young reference ordering
remains unchanged. Full reference ordering runs in a precise fallback pass. That
pass drains the current young domain before starting and prevents new young starts
until full tracing/reference closure. Without this handoff, full cross-roots could
import the conservative young cohort repeatedly and falsely exhaust the two-cycle
allocation-recovery allowance even when the heap contained garbage.

These rules permit unrelated young progress during old-only tracing and retain
explicit conflicts and conservative storage. They do not promise progress beyond
the heap/reclamation capacity or remove guarded payload access. Reduced Loom models
cover cross-generation retention strength, promotion publication, and the callback
scan interlock; forced schedules exercise the actual collectors and completion
tickets. They are not a model of every possible user-defined Trace callback.

## Full-start handoff

A requested full pass first drains an already-active young snapshot. While that
request is pending, new young starts yield to it. This avoids repeatedly importing
an earlier young snapshot's floating garbage after mutators have released every
root. Once full tracing starts, ordinary young work can overlap it as before.
With old-only collection enabled, the stronger existing rule still defers young
starts until precise full tracing closes. These are collector-domain scheduling
rules; neither waits for a mutator rendezvous.

## Operation attribution

Optional diagnostics record descriptor publication (including attach and metadata
registration), owning root acquisition/removal, barrier recording, and actual
record drain. Edge-barrier recording includes read-only certification and fallback
admission; owner/root recording retains its respective publication boundary.
These inclusive timings overlap and must not be added together.
Reservation/preparation before descriptor publication and compatibility `add_root`
calls are outside the respective measurements. Disabled recording does not read
a clock; bare Wasm reports counts with no duration. `barrier_buffered` counts new
reservations; `barrier_coalesced` counts events sharing an existing reservation or satisfied by
a completed edge record with matching epochs.
The mechanisms benchmark measures diagnostics on/off separately because timing
and shared counters can materially affect fast operations.

## Experimental compiled-frame root handoff

Direct backend reference methods now transfer root ownership at bytecode boundaries.
The production JIT eligibility allowlist is unchanged; qualified primitive methods
emit no new handoff calls. Backward dataflow now selects reference locals that can
be read on a normal or exceptional successor path. The full J1 lifetime and
performance gates remain open.

1. The caller retains owning argument Values through compiled execution. Generated
   code stores stable object identities for current reference operands and live
   reference locals in an aligned spill buffer. Primitive longs are never roots.
2. `jit_publish_frame_roots` checks the buffer shape. If both the input and current
   owner map contain at most 32 entries and every non-null identity is already
   owned, an unchanged set reuses the map without owner cloning or allocation.
   A subset removes omitted owners into a fixed stack buffer, unlocks the context,
   then releases them. Nulls and duplicates do not keep omitted identities alive.
3. For small sets with new identities, collect distinct missing identities while
   holding the map lock, then validate/root them outside it into bounded inline
   storage. The sole-executor contract keeps these negative lookups valid. Reserve
   room for every new insertion before removing anything: deleted hash buckets
   may remain tombstones, so reserving just the net size increase is insufficient.
   Retain unchanged entries, move omitted owners to inline storage, and insert the
   prepared owners. A disjoint replacement drains the map directly; replacing a
   sole root uses two local owning handles without batch buffers. Release the
   mutex before dropping omitted owners. Validation
   or reservation failure preserves the prior roots and releases prepared owners.
   Larger sets still prepare a replacement map, cloning existing owners or
   validating new identities before the swap and unlocked cleanup. Runs of existing
   owners share one context lock; the lock is released before every collector
   identity lookup/registration and before temporary-owner cleanup on failure.
   Cloning these owning handles only shares an existing registration.
   Collector enumeration uses the ordinary GC root registry and never
   acquires this context mutex. A stalled frame may retain its published graph;
   the mutex itself does not prevent unrelated reclamation.
4. Each helper roots a newly returned reference before returning its raw identity.
   That owner lasts until a later complete frame publication or invocation cleanup.
   The current operand stack includes helper results before the next boundary.
5. Return conversion consumes the completed context and moves an existing frame
   owner into the VM result. An absent identity still undergoes typed validation
   while caller arguments and the context remain owned. This applies to both the
   stack and heap argument-conversion buffers. Pending exceptions share a frame
   owner when present, otherwise they acquire a validated owner. Handler entry
   moves that owner into the helper-result set; uncaught propagation moves it into
   the VM error. Replaced owners are dropped outside the context mutexes. Nested
   compiled invocations have distinct contexts.

The calling activation must be the sole executor publishing or consuming raw
references through its context. Publication's unsafe internal contract requires
that its set include every identity the activation may subsequently use. Existing
owning Rust handles remain independent roots. No Rust unwind crosses generated
frames; runtime helpers retain their guarded FFI contract.

The initial root spill capacity is 4,096 identities (32 KiB of spill storage);
reference methods exceeding it are rejected. Liveness uses dense bits over the
reference-local indices, with an 8 MiB state limit and one million graph-edge limit;
methods exceeding either fall back rather than publish an incomplete root set.
Reference loads generate uses. All local writes kill the prior reference kind on
normal successors, including both slots of a category-two store. Handler uses are
merged after those kills: exception edges conservatively cover every protected
instruction and observe its incoming locals. The worklist follows branches,
switches and backedges until the sets stop changing. Dead locals are released even
when their typed SSA variables still contain an old identity. Class verification
remains responsible for local types, initialization and assignability.

Small snapshots reuse the map and grow it only when insertion capacity requires it;
the larger-set fallback still allocates a replacement map. Membership scans are bounded by 32
input entries and 32 current owners. Every bytecode boundary still calls the
helper. Boundary reduction and whole-invocation measurements remain prerequisites
for performance qualification. Owning argument Values held by the caller remain
independent roots for the duration of execution;
callee-local liveness does not release those external owners.
The allocation-pressure fixtures verify reclamation before return with zero and
one registered native worker under both publication policies. Zero-worker cases
explicitly stop/join the worker started by VM initialization (`threads=0` still
selects one native worker on `start()`); they verify cooperative assists and no
worker steps. Return and exception tests verify owner transfer, post-context
survival and eventual reclamation. These do not establish relocation, mixed-call
or suspension completeness. Existing collector admission/closure waits remain; this
handoff does not remove their global progress limitations.

### Experimental static-call ABI

`invokestatic` uses one aligned buffer sized to the largest runtime request per generated
activation, reused at its call sites. A cell holds one logical primitive value
or stable reference identity. Long/double values still consume two Java local
slots; the descriptor parser enforces the 255-slot Java limit. Float/double cells
preserve their raw bits. No managed payload address is passed to generated code.
The helper receives context, bytecode offset, constant-pool index, buffer and
logical count, and returns raw result bits. An exception must be checked before
the placeholder result enters the operand stack. Invalid buffer shape is rejected
before reading any cell. Readability and initialized contents are an internal
generated-code contract, not something a raw pointer's shape check can establish.

The incoming snapshot already owns all outgoing references and the caller's
remaining live values. Resolution uses the current real frame and existing
method-reference cache. Decoding clones existing owning handles (with typed
validation as a fallback), and those Values remain owned across declaring-class
initialization and the nested dispatcher call. No context lock or payload guard
spans that await. The actual declaring class is initialized only after successful
resolution and invocation-kind validation; merely naming a symbolic subclass
does not initialize it. A returned reference becomes a caller-context owner
before its raw identity is exposed. Exceptions use the existing owning pending
slot and unwind through the dispatcher. Each nested compiled activation has its
own buffer, context and real VM frame.

The synchronous native bridge blocks the calling activation and releases its Tokio
worker. Eligible methods now use the continuations specified below; the
synchronous embedding API retains this bridge. The original
pressure test checks unrelated reclamation while an interpreted or compiled
callee allocates. It does not establish arbitrary native/reentrant callbacks,
cancellation, recursion/stack limits, relocation or every initialization/access
rule. Wasm continues to interpret these calls; a successful Wasm build does not
mean this native ABI executes there. Production call eligibility remains closed.

### Experimental call continuations

For methods without dynamic calls or legacy subroutines, all four ordinary invoke forms have an owning suspension
point. Before returning, generated code saves typed locals, residual operands,
call kind/BCI/constant-pool index and logical arguments. The private call buffer is
sized to the largest call in that method. Its cells contain bits or stable
identities, never managed payload addresses. The pre-instruction frame snapshot
continues to own every argument and remaining live reference. Stale cells outside
that snapshot are not roots and must not be used after their lifetime ends.

`Activation` owns the function and cells. It refuses native resumption while a call
is pending and accepts exactly one completion before resumption. The VM retains
the same owning runtime context, publishes the call-site BCI and yields with no
generated caller frame or borrowed context address active. Resolution and decoded
argument ownership use the shared dispatcher semantics above. Results and pending
exceptions acquire caller-context ownership before completion writes raw result
bits. The restore block joins immediately after invocation, checks the pending
exception and pushes a successful result. It does not replay argument evaluation
or call side effects. The synchronous public API selects a helper branch instead.

Each pending call future owns its nested execution. Native polling and recursive
drop can use temporary stack segments; each segment ends before suspension. Under
an active synchronous compiled frame, polling checks available native capacity
without extending that segment. Logical Java stack charges remain in force.
Cancellation releases frames, roots and method-monitor guards through the normal
execution cleanup. A class-initialization claim now has an owning cancellation
guard: if still initializing when dropped, it marks the class failed and wakes
waiters. Same-thread reentry and waiting attempts do not acquire this guard;
partially executed initialization is never restarted.

Exception-table methods also use this protocol. Their exception dispatch targets
dedicated handler poll edges, separately from normal branch destinations. Each
selected handler consumes one unit of fuel; this includes exception-only loops
without explicit backward branches. At exhaustion, the selected handler's live
locals and single throwable operand become the published root snapshot before
the native frame returns. The pending owner was already transferred to the helper
result set; snapshot replacement retains it before releasing any previous owner.
Resumption enters the handler directly without matching or taking the exception
again. Existing table order and unmatched propagation remain unchanged.

Explicit monitor requests share the saved-state protocol, with a single object
identity retained by the incoming snapshot. No native frame spans monitor
acquisition. Both interpreter and JIT use the logical thread's monitor operations;
null/ownership failures become pending exceptions at the exact instruction BCI.
A successful entry installs an owning monitor/object record before the next await.
The per-thread ledger survives suspension, dead generated locals and OS migration.
Matching exit removes its record; cancelled host scopes release only acquisitions
since their checkpoint, outside the ledger lock. No publication admission or
managed guard spans a wait. This protects monitor objects, not unrelated graphs.
The synchronous embedding helper preserves its blocking completion contract.

Allocation requests use the same owning cells and pending/completed transition.
`new` has no dimension arguments; primitive/reference arrays have one signed i32
encoded in an eight-byte cell; multidimensional arrays retain each dimension in
declaration order. No native stack pointer crosses allocation suspension. Resolution
precedes multidimensional negative-size validation; array resolution does not
initialize its component. `new` owns any class-initialization claim through the
existing cancellation guard. Direct construction and publication await with no
managed guard held, and result/error roots precede completion bits. Cancellation
releases unfinished backing and context roots; completed partial parents become
collectible through ordinary GC. The shared multidimensional builder counts nodes
across recursion levels and yields after 64, including zero-length children.

Field requests retain zero to two raw cells: receiver identity and/or the value's
primitive bits or stable identity. The incoming frame snapshot owns references.
Resolution and static initialization run without a generated frame; instance access
then takes the existing managed guard, with no await while holding it. Reads own
their result before completion; writes commit once, before native resumption.
Both synchronous and resumable helpers use the interpreter's checked descriptor,
access, kind, final-write and receiver rules. Static access initializes the resolved
declaring class, not a symbolic subclass. Class-initialization cancellation uses the
same failed-state/waiter protocol described above. Heap field ownership and barriers
remain in the existing checked storage operations.

Continuation compilation checks the product of resume sites and declared typed
local/operand capacity, rejecting more than 65,536 transfer cells before generating
save/restore code. This resource guard prevents pathological state expansion; it is
not a compiler wall-time bound and rejected methods retain entry-time fallback.

Requests consume the same local fuel as backedges/handlers. The VM retains the
remaining budget across ready monitor/allocation/field requests, yielding at exhaustion;
direct calls yield before dispatch. Refreshing fuel at every ready request was an
unbounded-poll gap in the earlier monitor implementation. The bounded subprocess
now exercises both ready-only allocation and monitor loops. Synchronous embedding
still runs to completion and may use a null opaque context only when its registered
helpers permit it; owning reference continuations require an explicit context.

These paths remain experimental. Other synchronous helpers and native/reentrant
callbacks still need complete suspension qualification. Object wait/notify
cancellation and the full synchronization/Java memory-model matrix remain open. Publication/root handoffs retain the collector's existing admission
and callback progress limitations. Per-call yields and state copies have not been
performance-qualified; this protocol establishes no wall-time or STW bound.

### Experimental constructor and special-call ABI

`invokespecial` shares the static-call cell format and activation-local buffer.
Cell zero is the receiver's stable identity; remaining cells hold logical arguments
in descriptor order. The 255-cell/Java-slot limit includes the receiver. Wide values
still consume one cell and two Java local slots. Generated code publishes the
receiver and any duplicate retained by `new`/`dup` before entering the helper.

The helper resolves using `InvokeKind::Special`, decodes owning Values, checks the
receiver for null and calls the same cached target selector as the interpreter.
Superclass selection uses the caller's hierarchy rather than receiver overrides;
private and interface-default calls retain their special selection rules. This
invocation does not trigger another class initialization. Ordinary `new` and static
initialization rules remain responsible for their existing initialization points.
No payload guard or context-map mutex crosses nested execution. A constructor's
void result contributes no new operand; a duplicated receiver already belongs to
the calling frame. Non-void special results and exceptions use the same owning
transfer as static calls. All paths still use the normal thread dispatcher.

The pressure fixture creates a receiver and stores a newly allocated reference
array in it, then allocates 62.5 MiB before the constructor returns, within a 32 MiB
heap. It checks the receiver edge, reclamation before return and eventual release
under both publication policies, zero/one registered worker, and interpreted or
compiled constructors. Separate fixtures cover exceptional constructor exit,
non-virtual selection, nulls, private wide/reference results and buffer limits.
These checks are not complete access/initialization, recursion, monitor, native
reentrancy, relocation or performance qualification. The production gate remains
unchanged; Wasm continues to interpret these instructions.

### Experimental virtual-call ABI

`invokevirtual` uses the same receiver-first cell buffer and owning argument/result
handoff. It resolves the symbolic method before receiver checks, then calls the
same `select_virtual_target` routine as the interpreter's slow path. Private
methods use their resolved target. Other calls select against the receiver's
actual class, including inherited overrides and most-specific interface defaults.
Only successful selections enter the existing four-entry cache, keyed by class
identity; further receiver classes use normal lookup without growing the cache.
The cache owns class/method metadata, not receiver objects.

Primitive-array receivers require class loading. The helper owns the receiver
through that await and drops its temporary payload guard before loading the class.
Selected declaring-class metadata accompanies the method into normal dispatch.
Return conversion roots any reference before exposing its identity; exceptions
retain their owner while nested frames unwind. The caller's already-published
operand roots remain independent of the nested callee's context.

The pressure matrix checks a caller-only array and an override's local/returned
array while 62.5 MiB is allocated inside a 32 MiB heap, with both publication
policies, zero/one registered worker and interpreted/compiled overrides. Separate
tests check primitive bits, post-return reference lifetime, receiver reclamation
after exceptional exit, cache overflow, private/default/native targets and error
ordering. These do not establish signature-polymorphic calls, all access rules,
recursion, monitors, arbitrary reentrant suspension, relocation or performance.
The production eligibility restriction remains in place.

### Experimental interface-call ABI

`invokeinterface` now shares the receiver-first argument cells, owning conversions
and pending-exception transfer used by the other three experimental call forms.
The instruction's redundant count must equal its receiver plus Java parameter
slots: long/double each contribute two slots but occupy only one eight-byte cell.
The compiler rejects a mismatched count even when invoked directly on unverified
input. Class-file verification independently checks the count and reference kind.

The runtime resolves an interface reference before inspecting the receiver, then
uses `select_interface_target`, extracted from the interpreter's existing path.
The selector checks receiver membership before accepting a target, applies default
and inherited selection, checks selected accessibility before abstractness, and
caches only successful targets in the existing four-entry class-identity cache.
The interpreter's synchronous call fast path is unchanged. Array class loading and
membership awaits retain owning receiver/argument Values, with no payload/context
guard held across suspension. Native Object methods resolved through an interface
use the normal dispatcher and preserve the returned owner's identity.

Eight additional tests cover cache overflow, defaults, linkage/null/membership/
access/abstract errors, wide argument counts and primitive bits, native primitive
arrays, returned reference lifetimes and caught/uncaught throwable ownership.
Both publication policies, actual zero/one worker registrations and interpreted/
compiled callees pass allocation pressure with separate caller and callee arrays
live while discarded allocations are reclaimed before return. Exact exported
classes also run under HotSpot verification and interpretation. This does not
qualify every private/nestmate access case, conflicting defaults, dynamic calls,
recursive exhaustion, reentrancy, monitors, relocation or production coverage.
Production eligibility is unchanged; Wasm continues to use its interpreter.

### Compiled-frame Java stack budget

Compiled frames now reserve the same logical Java slots as interpreted frames:
`max_locals + max_stack + 1`. The dispatcher checks capacity before taking a
pooled frame or moving a synchronized-method monitor guard into the stack. A
rejected method never enters generated code. Successful entry records the charge
on its `StackEntry`; the existing pop/unwind path releases it on both normal and
exceptional return. Emergency throwable construction uses the existing bounded
reserve and releases that reserve before control resumes.

A production-eligible oversized primitive method first reproduced the zero-charge
bug; it now throws before native entry and succeeds at the exact declared budget.
Sixteen policy/mode combinations exercise caught and uncaught overflow across
alternating interpreted/compiled recursive methods. The backtrace contains only
entered frames, with exact call-site byte offsets; argument roots become
reclaimable while returned throwables remain owned. Three synchronized mixed-mode
configurations were intended to verify that a different owner could acquire the
monitor after unwinding. A follow-up exposed that an absent class mirror skipped
those assertions; that original run did not qualify monitor cleanup. The corrected
checks below require a monitor. Monitor bytecodes remain unqualified.

Twelve pressure configurations retain five distinct per-frame arrays while the
leaf allocates 62.5 MiB in a 32 MiB heap. Reclamation advances before return under
both policies, zero/one registered worker and three compiled/interpreted mode
combinations. The final checksum consumes every frame's array. Stack slots and
frames return to zero after each invocation, including overflow and recovery.

This is logical Java stack accounting, not a native stack bound. The synchronous
runtime bridge still consumes Rust/generated stack space on nested calls. Deep
native recursion, generated-frame size bounds and safe host-stack exhaustion
remain open and prevent widening production call eligibility. HotSpot's exact-
class driver checks the same bytecode, including overflow/recovery on its own
stack; it does not equate stack capacities or qualify Ristretto's native stack.


### Native compiled recursion and method monitors

VM entry into generated code runs inside a synchronous, OS-thread-local compiled
nesting guard. An outer entry with insufficient reserve can use a temporary 1 MiB
stack. While a compiled frame is active, Java dispatch rejects available native
stack below 256 KiB before acquiring the next method's monitor or pushing a frame.
A second check covers generated entry itself. The guard restores its previous
nesting state on every return/unwind. It neither holds a managed payload guard nor
changes the Java stack-slot charge.

New throwable construction is polled with a 512 KiB native red zone and a temporary
1 MiB segment when needed. Polling stays on the same OS thread, so GC context TLS
continues to identify the caller. A segment ends before `Pending`; the pinned future
owns the state that survives suspension. Existing throwable owners are propagated
directly, and emergency OOM is checked before boxing the cold construction future.
Native stack segments are outside the configured Java GC heap. Wasm continues with
its interpreter and does not depend on the native stack-switching library.

Static synchronized dispatch now materializes an absent class mirror and uses its
canonical monitor. Mirror creation is a cold, boxed async path; no payload guard
is held across it. This fixes execution without locking when `class.object()` was
absent. Tests require a real monitor, verify contention/cancellation before a
production compiled entry, and check monitor reuse after caught/uncaught overflow.

The isolated native matrix uses Linux x86-64, 1/2 MiB OS stacks, both publication
policies and three mixed execution modes. It checks the native-overflow message,
backtrace depth below the Java slot limit, actual compiled entries, distinct owned
throwables, root reclamation, monitor release and reuse of the same Java thread.
An earlier unguarded release probe aborted. A 128 KiB reserve failed a mixed debug
case; applying a 256 KiB guard to interpreter bootstrap also rejected valid startup.
The final guard is scoped to compiled nesting and is not an interpreter-only
native-reentry guarantee.

These are bounded regressions, not a universal native stack proof. The following
section adds generated-frame size checks. Cold helper worst-case stack use,
unsupported host-bound discovery, additional native backends, native/JNI reentry
and cancellation still require qualification before production calls can be
enabled. No performance gate is satisfied by these checks.

### Generated-frame size checks

Every compiled `Function` records the frame span emitted by Cranelift after register
allocation, plus sixteen bytes for ABI setup on the supported x64/aarch64/riscv64
backends. The pinned backend's `FrameLayout::active_size()` includes outgoing
arguments, fixed stack slots, register spills and saved registers. The compiler
emits neither dynamic stack slots nor tail calls; size extraction rejects those
forms and absent/nonuniform map spans. Other ISAs are rejected at compiler creation.
A failed metadata check frees code allocated before finalization/publication.

A zero-valued eight-byte slot and one user stack map expose that span. Methods
with root handoff attach the map to the first reachable root-publication call;
the slot is initialized before it. Rootless methods use a cold call under an ABI
argument count of `usize::MAX`, which a valid Rust slice of nonzero-sized values
cannot supply. Java bytecode zero has a separate block, so backedges cannot rerun
ABI setup. Normal paths do not execute the cold helper. The extra branch, slot
and frame setup have not been performance-qualified. These maps contain no VM
object identity and are not used by the GC to own references.

Immediately before native entry, `Function::execute` checks the recorded frame plus
the 256 KiB runtime reserve against known current stack capacity. Unknown bounds
are an error, and rejection does not increment the diagnostic execution count.
Both stack and heap argument-buffer paths perform this check. The VM converts
capacity exhaustion into the existing Java overflow path; ordinary frame unwind
continues to release monitors, Java slots and runtime-context roots. The outermost
compiled entry can allocate at least its frame plus 512 KiB; nested entries do not
grow another normal execution segment. Exception construction retains its separate
reserve described above.

The large backend fixture requires 1,048,608 native bytes on the tested host. A
bounded subprocess rejects it on a 512 KiB segment that still exceeds the generic
red zone, verifies zero entries/side effects, and then executes it on a 4 MiB
segment using both argument buffers. At that snapshot, two class-file fixtures
declared 2,050 locals: pure values could be recomputed in a 48-byte frame, whereas
opaque field-helper results required an 8,272-byte frame. The field-continuation
follow-up uses opaque long-array helpers and still checks every result plus a
native frame over 4 KiB, independently of the new continuation expansion limit.
This checks actual register spills and demonstrates why Java slot counts cannot
size native frames; the earlier exact byte counts belong to their original fixture.

This closes the missing generated-frame measurement/check path on the exercised
backend. It does not bound arbitrary helper/native callback frames, qualify all
native reentry/cancellation or host platforms, validate every native ABI input,
or satisfy any GC latency/throughput/endurance gate. Wasm interpreter accounting
and the remaining production eligibility restrictions are unchanged.


## Rooted object-array projections (2026-11-14)

`experimental-trace-projections` additionally permits a scoped writer whose entire
parent has independently owning direct strong fields and needs no other tracing
actions. `RwLockWriteGuard::try_map_trace_rooted` makes this an unsafe lifetime
contract, including every possible mutation through the returned projection.
`Trace::root` alone does not establish eligibility for arbitrary types.

`Value::as_class_vec_mut` is a safe qualifying entry point: ordinary mutation
preparation roots every current Value, and the returned slice cannot replace the
containing Reference variant or access attachment/demotion hooks. Safe construction,
cloning and extraction preserve ownership of replacement fields. A Value targeting
a conditional Java reference owns that separate descriptor; its ordinary scanning
and conditional processing still run. This parent array has no direct conditional
slot or custom trace action to skip. Foreign strong Values retain source ownership.

Successful mapping publishes the existing Release certificate. The original writer
stays boxed through nested projections and revokes before demotion, barriers and
unlock, including unwind. Failed initial mapping certifies nothing. A current Acquire
CAS permits a scan without reading the borrowed payload; root deletion retains its
normal snapshot barrier after a completed scan. The primitive empty-graph API is a
strict subset. No extra per-cell state or relocation bypass is introduced.

Six production-state models include root-publication visibility and composition
with assumed deletion shading in one active epoch. One stock-Loom 0.7.2 model hits
the known RMW/store atomicity defect; an isolated recorded checker patch runs it
unchanged, and intentional ordering/revocation defects still fail. This does not
prove the underlying admission, root-storage or phase protocol. See validation.

Default builds retain ordinary mapping. Whole-value, conditional-parent and opaque
writers and shared-reader contention remain unqualified. Whole-slice rooting and
demotion still take linear synchronous work, so bounded bulk Wasm access and D/E
remain open.

## Scoped certificates for trace-free projections (2026-11-13)

`experimental-trace-projections`, disabled by default, implements a per-managed-cell
empty-graph certificate. The classloader feature enables the collector feature.
Seven typed primitive-array views and the byte view of all eight primitive variants
use it. Default builds retain ordinary mapping; unrelated reference-bearing fields,
object arrays and whole-value writers do not qualify. A cached leaf classification
is insufficient for `Reference`, whose variant can later change.

1. `RwLockWriteGuard::try_map_trace_empty` acquires ordinary mutable ownership before
   its callback, just like `try_map`. A failed callback or callback unwind publishes
   no certificate. The unsafe caller must establish that the entire parent requires
   no tracing action and that every mutation through the projection preserves this
   property until all projected guards are gone.
2. After successful projection, publish the certificate with Release. The original
   writer owns its revocation duty and stays inside the mapped guard's existing box.
   Nested projections transfer that parent; failed nested mapping retains it.
3. Bounded tracing tries normal shared access first. On contention, an Acquire CAS
   from true to true linearizes an empty scan without accessing the exclusively
   borrowed payload. A stale ordinary load or an old success cannot authorize it.
   Direct tracing can also use the current certificate.
4. The original writer revokes with Release before its finish-write hook and unlock,
   including unwind. Subsequent unrestricted writers retain ordinary tracing and
   mutation barriers. Storage remains pinned; the certificate permits no relocation.

Four Loom models compile the production transitions. Forced native and cooperative
fixtures cover full/young/old cycles, stopped native projection, nested mapping,
failure/unwind, and a later reference-bearing variant. Safe primitive entry points
also reject object/reference arrays. Separate mutations demonstrate failed progress
without certification and stale certification without revocation. See
[validation](VALIDATION.md) for the complete checks and cost evidence.

The certificate does not solve arbitrary writers, opaque trace callbacks or stalled
publication admission. It also does not certify a general writer merely because
`Trace::root` ran: every possible later mutation and demotion would need independent
graph coverage. D/E and the full pause/progress gates remain open.

## Whole-value ownership and narrow mutation (2026-11-13)

Unrestricted managed mutable access now invokes `Trace::root` before exposure,
including before successful, failed and unwinding mapping callbacks. The moved
value carries its own strong roots. `Gc`, `Option`, `Vec`, `HeapBuffer`, managed
cells, `Value`, `ArrayElements`, `Object` and `Reference` implement the applicable
restoration. External trace implementations that demote fields must implement
matching restoration. Already owning handles need no new registration; conditional
referents remain conditional. Array storage records the entire demotion range
before restoring fields so unwind preserves cleanup. This does not change cards
or mutation identity merely for borrowing.

The initial whole-value cost is large: every ordinary point operation through an
unrestricted parent acquired all fields. `ReferenceWriteGuardExt` now implements
replacement-only field/array stores and owning one-element projection. Its small
unsafe boundary uses `RwLockWriteGuard::unrooted_mut`: no unrooted value may escape,
including via captures or unwind, and normal graph/barrier obligations still apply.
Setters drop old fields in place; element projection invokes the existing point
root acquisition before returning `&mut Value`. Guard release retains demotion,
owner notification and dirty-card handling. Read-only validation occurs under the
same guard. Whole-value preparation remains available after any narrow operation.

Both ownership failures have frozen negative reproductions. The native/WASI matrices,
Miri, sanitizers and paired cost screens are recorded in
[validation](VALIDATION.md#whole-value-ownership-and-narrow-mutations-2026-11-13).
This supplies no certificate for skipping an inaccessible graph. Unrestricted bulk
borrows still take linear synchronous work; incoming edges from other writers,
conditional reachability, bounded snapshot updates and retained-memory accounting
remain obligations for any general graph-progress protocol.

## Mutable graph ownership investigation (2026-11-09)

An immutable outgoing-edge snapshot cannot safely replace locked tracing unless
edge transfers remain protected independently of their former owner. A concrete
before-fix test moved an array element into a Rust local, released the source, and
completed three full cycles. Its weak probe was cleared even though the extracted
`Value` was still present. The fixture checks liveness before dereferencing, so
the failure does not depend on executing a dangling access.

`ArrayElements` now roots existing values before exposing mutable elements or
slices, and before returning a consuming iterator. `Value::root` reuses the
existing rooted-clone protocol, with publication preceding release of the prior
edge. The original guard's finish hook still unroots values left in dirty slots;
mapped guards, unwind and cancellation retain that hook. New Java stores use a
replacement-only method, which preserves deletion barriers without rooting a
discarded edge. No new atomic state or unsafe implementation is introduced.

At that snapshot only element/iterator boundaries were repaired. The whole-value
boundary is now covered by the later repair above; its acquisition remains linear. The general snapshot design
must cover that transfer, incoming edges from another writer, conditional edges,
publication before root demotion, bounded update/retirement work and retained-memory
accounting. The optional scoped certificate above handles eligible empty-graph projections;
it is not the general solution. Whole-slice rooting has a measured linear cost and cannot be
used to claim bounded Wasm cooperation. Held writers and opaque trace callbacks
continue using the existing graph reservations; D/E remains open.


## Owning generic clones and guard escapes (2026-11-14)

A `Gc` inside a traced payload is an edge. Copying it into a Rust local now creates
an owning registration; copying an already owning handle shares its registration.
`GcRootGuard::clone_gc` shares the matching registration, and guard cloning copies
its inner edge only while retaining that registration alongside it. If the guard
contains a replacement target, cloning follows that target's ownership instead.
`DerefMut` attaches owning protection before returning a mutable handle reference,
including when the caller moves the entire handle out with `mem::replace`.

Edge cloning reserves a root slot before entering publication admission. Inside
admission it checks the descriptor's immutable identity and the same domain
liveness predicates as ordinary rooted address resolution, then commits the slot
and shades the graph. It never dereferences the candidate payload. Failed identity
or liveness validation cancels the reservation after admission is released. An
expired edge cannot root a different allocation at the same address. Raw handles
without collector association cannot be cloned into ownership. Keep `ObjectId` or
`GcWeak` for non-owning identity retention instead.

Custom strong fields need paired root/unroot hooks and complete tracing. Managed
write guards perform the ownership conversion and barriers; an ordinary Rust mutex
does not. Cycle fixtures now use those managed guards and assert reclamation.
These changes still use the existing admission and root registry. Public explicit
demotion/revocation and custom Trace contracts remain caller-enforced, so this
repair does not establish a general writer-root certificate or a safe-API proof.


## Borrowed allocation attachment context (2026-11-14)

Normal and reserved allocation pass `TraceOwner` a borrow of the collector's
immutable self-weak reference. The constructor's collector borrow covers attachment,
including reentrant collection, suspension and unwinding. Retained hooks still clone
their own weak reference; the returned context cannot outlive the collector borrow.
The descriptor address and protected graph remain the unsafe caller's obligations.
This removes one temporary weak clone/drop without changing public ownership,
registration, publication admission or experimental defaults.

## Root guards validate their allocation heap (2026-11-14)

Public guard creation compares the handle's associated collector with the receiver
before registering. A mismatch or unassociated handle panics in `create_root_guard`;
`Gc::as_root` reports `InvalidPointer` before cloning or acquiring a registration.
The check reads borrowed collector pointers and adds no weak-reference operation.
Regular allocation associates its handle before creating the guard, including the
unpublished-construction path. Guard registrations can therefore protect only their
own heap's allocation. Identity mismatch after replacing a guard's inner handle
still roots the replacement independently, including a foreign owning replacement.

This closes the foreign-guard transfer defect recorded in the preceding owner-binding
snapshot. Unsafe raw/demotion/revocation APIs retain their caller lifetime contracts;
existing publication admission and general held-writer limitations are unchanged.

## Owner-aware tracing and nested-cell ownership (2026-11-14)

`Trace` requires an unsafe implementation; its attachment and demotion hooks are
unsafe calls carrying `TraceOwner`. The context borrows the actual collector's
weak reference and stable descriptor address. It grants no independent lifetime.
The caller must protect the graph, exclude conflicting access and publish required
barriers. Complete tracing, truthful capabilities, paired rooting and storage/borrow
lifetime are explicit implementation obligations. Raw `Gc::unroot` requires another
root or same-heap traced ownership for every subsequent payload access.

A strong field demotes only when its source collector matches the owner. Foreign
fields retain owning roots and keep their source collector alive. Option, vector,
tuple, heap-buffer and map-value hooks propagate attachment. A nested managed cell
installs its owner hook before demotion, detaches it before rooting for movement,
and binds the destination when stored again. Guard finish forwards the bound owner;
initial custom attachment is not blindly replayed on every mutation.

Array/instance fields retain dirty-range demotion. Whole-value rooting clears their
owner marker; finishing a moved conditional object binds its reference slot to the
new descriptor. This preserves same-heap queue identity and conditional strength.
Cross-heap queue delivery is not established. Hash keys, ordinary-lock payloads,
embedded root guards and cross-heap cycles retain conservative owning limitations.
A separate foreign root-guard mismatch remains in this snapshot; that boundary
must be repaired before relying on all safe owning-handle construction paths.
The protocol still uses existing admission and cannot skip a graph-bearing writer.

## Manual root removal and explicit revocation (2026-11-14)

Public numeric IDs are not capabilities to revoke owning handles. `add_root`
creates a manual tree registration. `remove_root_by_id` removes only that kind;
submitting an owning/internal registration's numeric ID is a no-op. Exact internal
removal still requires the full non-reused generation token.

Tree entries retain their existing address and 64-bit generation layout. The
high generation bit identifies owning/internal roots, while the low 63 bits are
a checked monotonic sequence shared by both kinds. Every insertion advances that
sequence under the existing tree mutex. It fails closed before entering the kind
bit. A stale owning token cannot remove a newer manual entry, or vice versa,
even when the public numeric ID wraps and is reused. No entry, root token or
registration grows, and no new lock or atomic operation is introduced.

All-registration removal by address is now an unsafe operation. The caller must
keep its borrowed payload argument valid throughout the call and prevent access
through revoked handles once reclamation is possible. Tests that deliberately
revoke roots explicitly establish those lifetime conditions. Dropping a revoked
handle remains permitted; its exact old registration cannot erase a replacement.
A compile-fail example rejects revocation from safe code. This change preserves
the existing admission, SATB deletion barriers and registry locking; it is not
concurrent catalog publication. Explicit demotion and custom Trace contracts still
need enforcement and audit before any general writer certificate.

## Isolated canonical-catalog experiment

The [catalog packages](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/README.md) exercise an alternative in
which a fixed-capacity registry discovers prepared numeric cells and a prepared
controller operation couples one exact cell effect with both domain counts and
phase versions. Another helper can complete a stopped operation, and an old helper
cannot republish a retired cell or remove a reused slot. This remains separate
from runtime registration: no current admission caller has been migrated.

Eleven native fixtures, eight models, WASI and both Miri modes pass. Three deliberate
mutations fail the result-recovery, retirement or slot-reuse assertions. The frozen
60-process allocation-count screen retains both this design and the immutable-tree
alternative. It is not a collector timing comparison. Real metadata/initial-root
ownership, byte budgeting, complete reference floors and the variable work within
ArcSwap remain integration requirements. See the linked experiment for commands,
raw evidence and the six-allocation-per-insertion cost that still needs reduction.

The follow-up [pooled experiment](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/catalog-pooled-prototype/README.md)
retains those logical transitions while using fixed control/membership backing.
Native/WASI, Miri/Tree Borrows, sanitizers and model checks cover the exercised
ownership rules; all negative controls remain evidence. One numeric lifecycle now
uses one system request instead of 18, with increased retained backing. A finite
pool does not itself reserve independent progress for stalled participants, and
this prototype still owns no real payload graph. Production admission is unchanged.

## Descriptor-owned allocation charges (2026-11-14)

Before metadata visibility, `AllocationReservation::transfer` installs the existing
weak collector link and disarms the private reservation without an intervening
fallible operation. Size and collector identity must match. From then on metadata
owns the logical charge through publication, retirement and storage leases.
`run_drop_fn` uses the existing single-winner storage transition, takes the erased
owner, releases its mutex, destroys payload/backing and then refunds the charge.
Sweep and retired-work callers no longer issue an independent byte refund.

The frozen regression injects an unwind after registry publication: the original
constructor refunded 16 bytes although the descriptor remained registered and
undestroyed. The repaired constructor leaves the charge with that descriptor.
Checks also cover private metadata reclamation, destructor unwind, deferred leases,
old metadata ownership and a heap that dies before its storage lease. A dead
collector needs no refund; metadata's weak link cannot keep it alive.

This transfers byte ownership, not initial graph roots or phase admission. The
current destructor and retired-work claims are not a helpable cancellation
protocol. Construction roots, canonical catalog integration, backing budgets and
stalled-callback progress remain separate work. See the
[validation](VALIDATION.md#descriptor-owned-allocation-charges-2026-11-14).

### Refund cleanup and collector borrowing

`DestroyPayload` is armed before the finalizer in private metadata cleanup. Its
scope exit reaches payload destruction even if diagnostics reporting a finalizer
panic also unwind. After the storage claim takes the erased owner, `Refund` stays
armed through payload/backing destruction and diagnostics. This covers both
observed private-cleanup failures without leaving a charge for freed storage.

Normal sweep and deferred reclamation call `run_drop_fn_in`, which checks the
owning collector's pointer before the storage claim and reuses its existing borrow
for the refund. Private cleanup uses the weak fallback. Neither guard allocates or
adds a metadata field. A foreign collector leaves storage and both budgets intact;
a repeated valid cleanup neither destroys nor refunds twice.

The first borrowed-refund candidate passed its eight focused fixtures and broader
checks, but a new finalizer/diagnostic case exposed a remaining charge leak. It was
rejected before timing. The revised nine-fixture implementation and both failing
executables are recorded in [validation](VALIDATION.md#guarded-refunds-and-borrowed-reclamation-2026-11-14).
This does not make finalizers, destructors or retired-work claims helpable/bounded.

### Construction registration transfer

Scope exit restores enclosing TLS, releases its conditional-owner reservations and
adopts strong registrations into the matching enclosing heap as a batch. Ownership
tokens move, including page/context references; an empty parent reuses the vector
backing. Without a matching scope, `remove_detached_root` removes each exact token
without another construction-adoption lookup. It preserves deletion barriers,
publication admission and slot reuse. Explicit helper cleanup can use this path
without accidentally rooting its graph in the helper's unrelated constructor.

This is not a shared publication request. Appending to a populated batch and
cleanup remain linear; admitted removal can still stall phase closure. The original
initial-parent cancellation failure and batch measurements remain in
[validation](VALIDATION.md#construction-root-batches-2026-11-14).

### Ordered initial-root cancellation

Typed preparation now returns an owned `MetadataRef` before installing the initial
parent root. The constructor declares this owner before its private `InitialRoot`
guard and passes a borrow to `register_prepared`. Registration validates the heap
identity before effects and remains a once-only admitted operation. If it unwinds
before publication, reverse local drop order removes the exact initial registration
before private payload destruction. Metadata-hook failures occur before that root
exists. If publication already won, registered metadata retains storage and its
charge until ordinary reclamation.

The private guard suppresses its redundant edge-drop barrier and uses detached
registration removal, which supplies the deletion barrier. It never adopts the
failed parent's root into an outer scope. Child roots and pending conditional-owner
counts keep their existing scope semantics. Success transfers the registration into
the existing `GcRootGuard`; no new metadata/root layout or runtime allocation is added.

The first isolated repair removed the leaked registration but failed cleanup order:
its payload destructor still observed the root. The borrowed-publication repair,
that failing executable, hook/child-protection checks and cost measurements are in
[validation](VALIDATION.md#ordered-initial-root-cleanup-2026-11-14). Metadata ownership
is not graph protection, and this stack guard is not a shared constructor request.
Registration, barriers and root removal still depend on admission.


## Sealed production publication ownership (2026-11-14)

After attachment and metadata hooks, constructors seal their construction scope.
Sealing moves the exact child registrations and conditional-owner reservations,
restores origin TLS, and returns a batch that never adopts cleanup into helper TLS.
`Publication<'gc,T>` owns this batch, prepared metadata, the private typed handle and
its optional initial parent registration. All five constructor paths use it.
Conditional-owner identity binding reads the owned batch, and allocation debt comes
from its metadata. The consumed record returns the existing client root guard.

Cancellation removes the initial parent registration before dropping private
metadata/payload. Child roots and conditional reservations survive payload destruction,
including reentrant collection. Hooks that unwind before sealing retain the existing
synchronous construction adoption behavior. A helper may consume the sealed record
on another scoped thread; its state has no origin-thread TLS borrow.

The shared heap borrow prevents final collector destruction while a private parent
can access children. It adds no strong collector link. This record cannot yet be
stored inside its collector: weak ownership would allow destruction after children
were freed, while strong ownership would create a cycle. Discoverable records need
explicit shutdown ordering and once-only payload extraction before that conversion.
Current registration executes once. Replaying its index, mark or queue side effects
can corrupt the collection. Root cleanup retains its ordinary admission and per-slot
locks, and batch cleanup remains linear. No global-progress gate is closed here.
See [validation](VALIDATION.md#sealed-publication-ownership-2026-11-14) and the
[report](../target/gc-implementation/publication-owner-REPORT.md).


## Rejected inline construction capture (2026-11-14)

The four-registration inline candidate preserves tested exact ownership at capture
sizes zero, four, five and nine, including sealing, context release, cross-thread
cleanup and unrelated TLS. Reference-free scopes must preserve an enclosing scope;
a deliberately broken restoration is detected natively and on WASI. The complete
31-command matrix and five native/WASI fault pairs pass their expected outcomes.

The representation is rejected on measured cost: it increases ordinary and nested
allocation operation time on both targets, despite saving about four allocator
requests per nested operation. Publication's diagnostic type size grows by 120 bytes
native / 104 bytes WASI. See the [full evidence](../target/gc-implementation/capture-inline-REPORT.md).
The follow-up restores vector storage and isolates the TLS bypass. Neither version
makes registration replay-safe, bounds bulk cleanup or removes publication admission.


## Optional reference-free construction scopes (2026-11-14)

The rejected inline representation is removed. Ordinary vector-backed scopes keep
their original layout; Option uses the non-null scope-reference niche. A reference-
free constructor installs no TLS frame, and absent-scope detachment neither extracts
captured registrations nor restores TLS. Reference-bearing scopes keep their normal
sealing/adoption behavior. Hooks continue to execute under the Trace contract.

The 25-command matrix and five native/WASI fault pairs pass. Explicit-leaf timing
improves in the paired screen, with retained WASI allocation/write-control regressions.
See the [complete costs and validation scope](../target/gc-implementation/leaf-scope-REPORT.md).
This preserves the existing private publication borrow, ordered cleanup, once-only
registration and admission. Shared canonical commit and bounded cleanup are still open.

## Shared private publication ownership (2026-11-14)

Prepared resources can move into a shared header whose admitted handles borrow
the matching live collector. The final handle closes admission permanently and
runs ordered private cleanup; retained raw headers preserve neither payload nor
collector lifetime. The private slot lock is released before callbacks. A new
forced-stall fixture confirms logical payload credit remains held while private
destruction is stopped, while unrelated garbage is reclaimed.

All 13 commands pass: native GC 538/562 (one existing ignore), actual WASI 268/279,
eight ASan/TSan shared fixtures, Clippy, bare-Wasm and formatting. Both Miri modes
cover seven byte-identical frozen development fixtures plus the newly appended
stalled-destructor fixture. Four production-state models pass with the frozen
corrected checker; seven distinct mutations yield 11 runtime and three model
negative failures. The initial model witness mismatch and all failures are retained.

Ordinary constructors still use the exclusive stack Publication. A retained handle
prevents exclusive restoration, so stopped-publisher progress and canonical commit
remain open. No performance claim follows; backing/participant accounting, real
graph/reference integration and all qualification gates remain open.

[Report](../target/gc-implementation/shared-publication-REPORT.md) ·
[Verification](../target/gc-implementation/shared-publication-verification.json).

## Borrowed shared descriptor views (2026-11-14)

Admitted shared-publication handles can borrow their exact prepared descriptor
without locking the private result slot. The view lifetime retains the handle's
metadata and construction-graph protection; the raw header gains no payload owner.
One documented pointer dereference is covered by all eleven shared fixtures under
both Miri modes, ASan and TSan. Ordinary constructor paths remain unchanged.

All 13 commands pass: native GC 541/565 (one existing ignore), actual WASI 270/281,
Clippy, bare-Wasm, format and diff. A forced native result-slot stall permits other
helpers' descriptor preparation and unrelated reclamation. A deliberate result-lock
fault is caught on native/WASI, and actual-source compile-fail fixtures reject both
escaping the handle and dropping it while the view is still used. Four unchanged
admission models have exact-hash scoped reuse; they do not model the new pointer.

This supplies a borrowed preparation view, not canonical publication. Announced
work still needs graph-safe cancellation and phase-coupled membership/traversal.
No performance or qualification gate closes; defaults remain unchanged.

[Report](../target/gc-implementation/shared-view-REPORT.md) ·
[Verification](../target/gc-implementation/shared-view-verification.json).

## Effect ownership precedes the commit decision (2026-11-14)

The isolated catalog prepares its fallible effect owner before deciding COMMIT.
Failed preparation remains cancellable; applying the exact-version effect and
recording its certificate then require no further fallible acquisition. A failed
strong effect CAS proves that effect already occurred because canonical controllers
are the only writers and old membership versions are never reinstalled.

A frozen red fixture catches the old irreversible decision before failed preparation.
Eleven prototype checks pass (61 native, 54 WASI, 58 in each Miri mode and 61 in each
sanitizer), including backing accounting. The 22 corrected-checker models include a
forced late-CAS schedule; three variants produce seven expected negative processes.
Integrated Clippy, the added stock-Loom model, format and diff checks also pass.

The catalog still contains only numeric records. Actual graph guards, canonical
payload/phase/reference integration, admitted callers and qualification remain open.
Ordinary runtime paths and defaults are unchanged; no performance claim follows.

[Report](../target/gc-implementation/commit-prefix-REPORT.md) ·
[Verification](../target/gc-implementation/commit-prefix-verification.json).

## Heterogeneous prepared publication headers (2026-11-14)

Erased headers share the existing typed allocation through an Arc trait object.
Their helpers validate the same live heap, use the same admitted count, borrow
descriptor views and dispatch the original typed final cleanup. No strong collector
owner, second header allocation or payload cast is added.

The 13-command matrix passes: native GC 544/568 (one existing ignore), actual WASI
272/283, fourteen shared fixtures in both Miri modes and both sanitizers, Clippy,
bare-Wasm, format and diff. Three faults produce six native/WASI runtime failures;
two compiler checks enforce erased-view borrowing. Four unchanged admission models
have exact-hash scoped reuse. Mixed-type, shared-backing and cross-thread fixtures
exercise the new erased interface.

Ordinary constructors are unchanged. Announced storage/graph lifetime, canonical
commit, phase/reference integration and full qualification remain open. No
performance or default-selection claim follows.

[Report](../target/gc-implementation/erased-publication-REPORT.md) ·
[Verification](../target/gc-implementation/erased-publication-verification.json).

## Announced private descriptor storage (2026-11-14)

Descriptor storage may now outlive cancellation without keeping a private payload
alive. A matching admitted helper is required for every storage view. Announcement
shares the handle-count word and permanently prevents legacy restoration. Final
private cleanup destroys the payload before releasing child roots and conditional
reservations. The interface creates no descriptor storage leases; its unsafe
cancellation contract explicitly excludes outstanding leases.

All thirteen matrix commands pass: native GC 555/579 (one existing ignore), actual
WASI 277/288, all twenty shared fixtures in both Miri modes and sanitizers, Clippy,
bare-Wasm and format. Nine corrected models pass. Nine distinct fault variants
produce seventeen expected runtime/model failures, plus two lifetime compiler
checks. Integration matched 1,494 compiled files before two recorded safety-comment
clarifications; integrated all-features tests and Clippy also pass.

Logical payload credit remains held through a stalled destructor while unrelated
reclamation progresses. Retained metadata/header physical backing is not yet fully
charged. Ordinary constructors do not select this private interface; canonical
publication, graph/phase/reference integration and all qualification remain open.

[Report](../target/gc-implementation/announced-storage-REPORT.md) ·
[Verification](../target/gc-implementation/announced-storage-verification.json).

## Canonical numeric allocation domains (2026-11-14)

The isolated tagged catalog now couples domain activity/scope, generation, births,
retention and exact trace tickets with pending counts. Explicit checked start/finish
replace implicit cycle restart. Promotion preserves young work while publishing the
old handoff. Ordinary discovery cannot revive another domain's logically dead object.

All eleven matrix commands pass: native 74, actual WASI 67, each Miri mode 71,
each sanitizer 74, Clippy, bare-Wasm, allocation/refund audits and format. Twenty-seven
corrected models pass; nine mutants produce four model and eighteen runtime failures.
The original expensive late-helper models remain incomplete evidence; a forced
pre-CAS schedule replaces that one case, with exact unchanged-library evidence.

Ninety-six allocator-count processes retain zero lifecycle allocations and complete
teardown refunds. Requested setup backing grows 8.1–11.8% native / 8.4–12.5% WASI;
16,384 residents add 512 KiB. These are numeric catalog backing costs, not GC timings
or RSS. Actual payload/graph, root and reference integration and all qualification
remain open. Ordinary collector publication and defaults remain unchanged.

[Report](../target/gc-implementation/catalog-domain-REPORT.md) ·
[Backing graph](../target/gc-implementation/catalog-domain-cost-v2/backing-change.svg) ·
[Verification](../target/gc-implementation/catalog-domain-verification.json).

## Guarded retained-record publication (2026-11-14)

The isolated catalog owns generic records. Each helper admits a context-borrowing
record guard before its own decision/effect, then publishes a cleanup-only latch
before certification. Closed/foreign records cancel undecided work; certified head
release needs no graph admission. Numeric insertion methods remain unit-record-only.
Preflight rejection returns the exact non-cloneable record for retry.

V1's eleven-command matrix passes (native80/WASI72, Miri76 and sanitizers80). V2's
record-return change passes full native81/WASI73 and all current retained-record
fixtures under both Miri modes (six) and sanitizers (seven), plus Clippy/bare/audits/
format. Thirty-one corrected models pass; six mutants produce five model and twelve
runtime failures, with three separate compiler checks. Twenty-four numeric accounting
rows are unchanged from the domain-state predecessor. No timing claim follows.

The guard fixture is synthetic. Real heap/identity/graph composition, unique payload
binding, bounded real-record participants, later announcement-failure handling,
canonical roots/references and every qualification gate remain open. Production
collector publication and defaults are unchanged.

[Report](../target/gc-implementation/record-admission-REPORT.md) ·
[Verification](../target/gc-implementation/record-admission-verification.json).

### Bounded retained-record requests

The isolated facade carries its Record through catalog, participant lease, creation,
preparation and attempt state. GuardedCommand is separate from the numeric command
set; sealed monomorphized dispatch cannot expose raw owners to custom commands.
Transient preflight failure returns R into the same request. Busy submit returns R
to its caller. Cancellation or terminal creation failure drops it before quota release;
a losing discovery announcement is terminal. A concrete runtime must bind unique
payload authority and cannot infer permission for another publication from this loss.

The footprint uses actual Cell<R>/Control<R> pool layouts, the outer catalog owner
and the larger supported inline participant form. Indirect record-owned backing is
outside that charge. See [validation](VALIDATION.md#bounded-opaque-record-requests-2026-11-14).
This facade does not make private PreparedStorage into collector-visible ownership.

### Concrete private record identity

PreparedRecord couples exact PreparedStorage with a unique non-cloneable token from
its Core. Reserve uses a checked single CAS before announcement. Enter acquires a
matching-heap handle before binding Core from zero to that token; the same token
remains helpable, other tokens fail, and neither guard nor record drop resets binding.
A private creation that loses before discovery can discard an unbound record and
prepare another from its still-live origin. A discoverable/bound record must retain
its identity across cancellation/retry. Reservation never wraps.

RecordGuard borrows the record and heap and owns the admitted handle. Its view is
restricted to matching descriptor storage. This adds no canonical registration or
cleanup latch: the private graph still closes by cancellation. Real collector/phase/
root authority must exist before published cleanup can select a different path.
See [validation](VALIDATION.md#prepared-payload-record-identity-2026-11-14).

### Canonical roots in the isolated catalog

Root publication and target membership share one controller; both exact effects
precede certification. Removal likewise preserves both domains' deletion work.
Per-domain root enumeration cannot close Strong after a failed owning observation.
Unique token Drop posts a durable request; only canonical retirement plus that
request permits exact slot release. The bounded handle keeps token ownership before
its quota/catalog lease so target destruction cannot refund backing prematurely.
See [root authority and quota derivation](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/catalog-tagged-prototype/ROOTS.md)
and [validation](VALIDATION.md#canonical-root-ownership-2026-11-14).

This is not an actual GcRootGuard or PreparedRecord adapter. Initial private roots,
captured child protection, canonical traversal/conditional decisions and result
transfer need one complete runtime lifetime protocol. A concrete Core that owns a
bounded root handle must break that ownership explicitly; no published cleanup latch
may select legacy registration based solely on this isolated catalog certificate.


## Canonical conditional references (2026-11-14)

The isolated catalog couples conditional decisions, owner publication, construction
custody and exactly-once notification transfer with phase work. Embedded reserved
roots belong to the exact reference object, so late announcements cannot occupy a
replacement's capacity. Non-cloneable reference/construction handles retain their
quota and catalog charge through final payload destruction. Helpers finish stopped
origins, and cancellation preserves an irreversible commit.

All 18 matrix commands and four main-tree checks pass: native/release 175,WASI 161,
full Miri/Tree 170 and both sanitizers 175, Clippy/bare, eight allocator executions
and format. The 48 corrected models are reused only for identical transition inputs;
they exclude facade quotas, real pointer protection, payload destruction and runtime
integration. 36 runtime faults, two model faults and nine external compiler failures
are caught, with a positive API control. All 528 accounting rows pass; reference
lifecycles make zero system requests but add fixed backing and inline storage.

These are isolated protocol and ownership results. The collector still uses its
existing runtime path. Graph/cleanup/root-result integration, admitted operations,
callbacks, JIT and all whole-GC qualification gates remain open. Defaults are unchanged.

[Protocol](https://github.com/theseus-rs/ristretto/blob/0497123e05056546bc526ecc49c081f934f38d64/ristretto_gc/validation/catalog/catalog-tagged-prototype/REFERENCES.md) ·
[Report and cost graph](../target/gc-implementation/canonical-reference-REPORT.md) ·
[Verification](../target/gc-implementation/canonical-reference-verification.json).
